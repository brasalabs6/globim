use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;
use tokio::time::timeout;
use tracing::debug;
use tracing::warn;

const DEFAULT_GITHUB_PROMPTS_BASE_URL: &str =
    "https://raw.githubusercontent.com/brasalabs6/goblins/main";
const REMOTE_FETCH_TIMEOUT: Duration = Duration::from_millis(1500);
const MAX_PROMPT_BYTES: usize = 256 * 1024;

const FALLBACK_GOBLIN_PROMPT: &str = include_str!("../../../prompts/goblin.md");
const FALLBACK_FRIENDLY_PROMPT: &str = include_str!("../../../prompts/personalities/friendly.md");
const FALLBACK_PRAGMATIC_PROMPT: &str = include_str!("../../../prompts/personalities/pragmatic.md");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PromptPack {
    pub(crate) goblin: String,
    pub(crate) friendly: String,
    pub(crate) pragmatic: String,
}

impl PromptPack {
    pub(crate) fn fallback() -> Self {
        Self {
            goblin: FALLBACK_GOBLIN_PROMPT.to_string(),
            friendly: FALLBACK_FRIENDLY_PROMPT.to_string(),
            pragmatic: FALLBACK_PRAGMATIC_PROMPT.to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct PromptCatalogLoader {
    cache_path: PathBuf,
}

impl PromptCatalogLoader {
    pub(crate) fn new(cache_path: PathBuf) -> Self {
        Self { cache_path }
    }

    pub(crate) async fn load_cached_or_fallback(&self) -> PromptPack {
        self.load_cache()
            .await
            .map(|cache| cache.map(|cache| cache.prompt_pack))
            .unwrap_or_else(|err| {
                debug!("failed to load Goblins prompt cache: {err}");
                None
            })
            .unwrap_or_else(PromptPack::fallback)
    }

    pub(crate) async fn load_remote_or_cache(&self) -> PromptPack {
        let cached_pack = self.load_cached_or_fallback().await;
        let mut prompt_pack = cached_pack;
        let mut fetched_any = false;

        let (goblin_prompt, friendly_prompt, pragmatic_prompt) = tokio::join!(
            fetch_first_remote_prompt(&["prompts/goblin.md", "prompts/globin.md"]),
            fetch_remote_prompt("prompts/personalities/friendly.md"),
            fetch_remote_prompt("prompts/personalities/pragmatic.md")
        );

        if let Some(prompt) = goblin_prompt {
            prompt_pack.goblin = prompt;
            fetched_any = true;
        }
        if let Some(prompt) = friendly_prompt {
            prompt_pack.friendly = prompt;
            fetched_any = true;
        }
        if let Some(prompt) = pragmatic_prompt {
            prompt_pack.pragmatic = prompt;
            fetched_any = true;
        }

        if fetched_any && let Err(err) = self.save_cache(&prompt_pack).await {
            warn!("failed to write Goblins prompt cache: {err}");
        }

        prompt_pack
    }

    async fn load_cache(&self) -> io::Result<Option<PromptCache>> {
        match fs::read(&self.cache_path).await {
            Ok(contents) => {
                let cache = serde_json::from_slice(&contents)
                    .map_err(|err| io::Error::new(ErrorKind::InvalidData, err.to_string()))?;
                Ok(Some(cache))
            }
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn save_cache(&self, prompt_pack: &PromptPack) -> io::Result<()> {
        if let Some(parent) = self.cache_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let cache = PromptCache {
            fetched_at: Utc::now(),
            prompt_pack: prompt_pack.clone(),
        };
        let json = serde_json::to_vec_pretty(&cache)
            .map_err(|err| io::Error::new(ErrorKind::InvalidData, err.to_string()))?;
        fs::write(&self.cache_path, json).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PromptCache {
    fetched_at: DateTime<Utc>,
    #[serde(flatten)]
    prompt_pack: PromptPack,
}

async fn fetch_first_remote_prompt(paths: &[&str]) -> Option<String> {
    for path in paths {
        if let Some(prompt) = fetch_remote_prompt(path).await {
            return Some(prompt);
        }
    }
    None
}

async fn fetch_remote_prompt(path: &str) -> Option<String> {
    let url = remote_prompt_url(path);
    match fetch_remote_text(&url).await {
        Ok(prompt) => Some(prompt),
        Err(err) => {
            debug!(url, "failed to fetch Goblins prompt: {err}");
            None
        }
    }
}

async fn fetch_remote_text(url: &str) -> Result<String, String> {
    let client = codex_login::default_client::build_reqwest_client();
    let response = timeout(REMOTE_FETCH_TIMEOUT, client.get(url).send())
        .await
        .map_err(|_| "request timed out".to_string())?
        .map_err(|err| err.to_string())?;

    if !response.status().is_success() {
        return Err(format!("unexpected status {}", response.status()));
    }

    let text = timeout(REMOTE_FETCH_TIMEOUT, response.text())
        .await
        .map_err(|_| "response body timed out".to_string())?
        .map_err(|err| err.to_string())?;

    validate_prompt_text(text)
}

fn validate_prompt_text(text: String) -> Result<String, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("remote prompt is empty".to_string());
    }
    if text.len() > MAX_PROMPT_BYTES {
        return Err(format!(
            "remote prompt exceeds maximum size of {MAX_PROMPT_BYTES} bytes"
        ));
    }
    Ok(text)
}

fn remote_prompt_url(path: &str) -> String {
    let base_url = std::env::var("GOBLINS_PROMPTS_BASE_URL")
        .unwrap_or_else(|_| DEFAULT_GITHUB_PROMPTS_BASE_URL.to_string());
    let base_url = base_url.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{base_url}/{path}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_prompts_are_standalone() {
        let prompt_pack = PromptPack::fallback();

        assert!(prompt_pack.goblin.contains("You are a Goblin."));
        assert!(prompt_pack.friendly.contains("# Personality"));
        assert!(prompt_pack.friendly.contains("# Project Docs Spec"));
        assert!(prompt_pack.pragmatic.contains("# Personality"));
        assert!(prompt_pack.pragmatic.contains("# Project Docs Spec"));
    }

    #[test]
    fn validates_prompt_text() {
        assert!(validate_prompt_text("hello".to_string()).is_ok());
        assert!(validate_prompt_text("  \n".to_string()).is_err());
        assert!(validate_prompt_text("x".repeat(MAX_PROMPT_BYTES + 1)).is_err());
    }
}
