use codex_protocol::config_types::ReasoningSummary;
use codex_protocol::openai_models::ConfigShellToolType;
use codex_protocol::openai_models::ModelInfo;
use codex_protocol::openai_models::ModelInstructionsVariables;
use codex_protocol::openai_models::ModelMessages;
use codex_protocol::openai_models::ModelVisibility;
use codex_protocol::openai_models::TruncationMode;
use codex_protocol::openai_models::TruncationPolicyConfig;
use codex_protocol::openai_models::WebSearchToolType;
use codex_protocol::openai_models::default_input_modalities;

use crate::config::ModelsManagerConfig;
use crate::prompt_catalog::PromptPack;
use codex_utils_output_truncation::approx_bytes_for_tokens;
use tracing::warn;

const PERSONALITY_PLACEHOLDER: &str = "{{ personality }}";

pub fn with_config_overrides(mut model: ModelInfo, config: &ModelsManagerConfig) -> ModelInfo {
    if let Some(supports_reasoning_summaries) = config.model_supports_reasoning_summaries
        && supports_reasoning_summaries
    {
        model.supports_reasoning_summaries = true;
    }
    if let Some(context_window) = config.model_context_window {
        model.context_window = Some(
            model
                .max_context_window
                .map_or(context_window, |max_context_window| {
                    context_window.min(max_context_window)
                }),
        );
    }
    if let Some(auto_compact_token_limit) = config.model_auto_compact_token_limit {
        model.auto_compact_token_limit = Some(auto_compact_token_limit);
    }
    if let Some(token_limit) = config.tool_output_token_limit {
        model.truncation_policy = match model.truncation_policy.mode {
            TruncationMode::Bytes => {
                let byte_limit =
                    i64::try_from(approx_bytes_for_tokens(token_limit)).unwrap_or(i64::MAX);
                TruncationPolicyConfig::bytes(byte_limit)
            }
            TruncationMode::Tokens => {
                let limit = i64::try_from(token_limit).unwrap_or(i64::MAX);
                TruncationPolicyConfig::tokens(limit)
            }
        };
    }

    if let Some(base_instructions) = &config.base_instructions {
        model.base_instructions = base_instructions.clone();
        model.model_messages = None;
    } else if !config.personality_enabled {
        model.model_messages = None;
    }

    model
}

/// Preserve Goblins-owned prompt fields while allowing remote model metadata to
/// update capabilities, limits, and availability around them.
pub(crate) fn apply_local_prompt_overrides(
    mut model: ModelInfo,
    local_model: Option<&ModelInfo>,
) -> ModelInfo {
    if let Some(local_model) = local_model {
        model.base_instructions = local_model.base_instructions.clone();
        model.model_messages = local_model.model_messages.clone();
    } else {
        apply_prompt_pack_override(&mut model, &PromptPack::fallback());
    }

    model
}

pub(crate) fn apply_prompt_pack_overrides(models: &mut [ModelInfo], prompt_pack: &PromptPack) {
    for model in models {
        apply_prompt_pack_override(model, prompt_pack);
    }
}

pub(crate) fn apply_prompt_pack_override(model: &mut ModelInfo, prompt_pack: &PromptPack) {
    model.base_instructions = prompt_pack.goblin.clone();
    model.model_messages = Some(local_personality_messages(prompt_pack));
}

pub fn apply_fallback_prompt_override(model: &mut ModelInfo) {
    apply_prompt_pack_override(model, &PromptPack::fallback());
}

/// Build a minimal fallback model descriptor for missing/unknown slugs.
pub fn model_info_from_slug(slug: &str) -> ModelInfo {
    warn!("Unknown model {slug} is used. This will use fallback model metadata.");
    ModelInfo {
        slug: slug.to_string(),
        display_name: slug.to_string(),
        description: None,
        default_reasoning_level: None,
        supported_reasoning_levels: Vec::new(),
        shell_type: ConfigShellToolType::Default,
        visibility: ModelVisibility::None,
        supported_in_api: true,
        priority: 99,
        additional_speed_tiers: Vec::new(),
        availability_nux: None,
        upgrade: None,
        base_instructions: PromptPack::fallback().goblin,
        model_messages: Some(local_personality_messages(&PromptPack::fallback())),
        supports_reasoning_summaries: false,
        default_reasoning_summary: ReasoningSummary::Auto,
        support_verbosity: false,
        default_verbosity: None,
        apply_patch_tool_type: None,
        web_search_tool_type: WebSearchToolType::Text,
        truncation_policy: TruncationPolicyConfig::bytes(/*limit*/ 10_000),
        supports_parallel_tool_calls: false,
        supports_image_detail_original: false,
        context_window: Some(272_000),
        max_context_window: Some(272_000),
        auto_compact_token_limit: None,
        effective_context_window_percent: 95,
        experimental_supported_tools: Vec::new(),
        input_modalities: default_input_modalities(),
        used_fallback_model_metadata: true, // this is the fallback model metadata
        supports_search_tool: false,
    }
}

fn local_personality_messages(prompt_pack: &PromptPack) -> ModelMessages {
    ModelMessages {
        instructions_template: Some(PERSONALITY_PLACEHOLDER.to_string()),
        instructions_variables: Some(ModelInstructionsVariables {
            personality_default: Some(prompt_pack.goblin.clone()),
            personality_friendly: Some(prompt_pack.friendly.clone()),
            personality_pragmatic: Some(prompt_pack.pragmatic.clone()),
        }),
    }
}

#[cfg(test)]
#[path = "model_info_tests.rs"]
mod tests;
