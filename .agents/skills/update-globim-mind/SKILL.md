---
name: update-globim-mind
description: Use when editing, auditing, or validating Globim's base prompt, model instruction templates, personality text, model catalog prompt fields, or prompt fallback files across all supported models including gpt-5.3-codex, gpt-5.5, custom namespaced model slugs, and future Codex-family model entries.
---

# Update Globim Mind

Use this skill when a task changes what Globim believes about itself, how it introduces itself, or which base instructions are sent to a model. The goal is to update the effective prompt for every relevant model path, not just the easiest markdown file.

## Ground Rules

- Read `AGENTS.md`, `GLOBIM.md`, and `SPECs.md` before editing.
- Treat Globim lore as flavor only. Do not weaken instruction hierarchy, safety, repository governance, validation, or user-work preservation.
- Keep upstream mergeability in mind: prefer small prompt and catalog edits over broad runtime rewrites.
- Never assume `gpt-5.5` is the only active model. Check `gpt-5.3-codex`, all entries in `codex-rs/models-manager/models.json`, namespaced slugs such as `custom/gpt-5.3-codex`, and any newly added model.

## Prompt Surface Map

Check these surfaces before deciding what to edit:

- `codex-rs/models-manager/prompt.md`: shared local base instructions used by `BASE_INSTRUCTIONS`.
- `codex-rs/models-manager/models.json`: bundled model catalog. Every model that should have Globim identity needs matching `base_instructions` and, when applicable, `model_messages`.
- `codex-rs/models-manager/src/model_info.rs`: local prompt preservation and personality fallback logic. Keep `is_globim_managed_model` aligned with the model slugs that must keep Globim prompt fields during remote `/models` refresh.
- `codex-rs/protocol/src/prompts/base_instructions/default.md`: protocol fallback used by `BaseInstructions::default()`.
- `codex-rs/core/templates/personalities/`: personality snippets used inside model instruction templates.
- `codex-rs/core/gpt*_prompt.md` and `codex-rs/core/templates/model_instructions/`: legacy or model-specific prompt material that may still affect tests, migrations, or older model flows.
- `codex-rs/core/templates/realtime/backend_prompt.md`, `codex-rs/core/review_prompt.md`, compact, memory, and collaboration templates: edit only when the requested behavior affects those flows too.

## Workflow

1. Inventory models and prompt references:
   - `jq -r '.models[].slug' codex-rs/models-manager/models.json`
   - `rg -n "You are Globim|You are Codex|BASE_INSTRUCTIONS|model_messages|instructions_template|is_globim_managed_model|gpt-5\\.3-codex|gpt-5\\.5" codex-rs`
2. Identify the effective path for the requested behavior:
   - Normal `exec` sessions resolve model instructions through `models_manager.get_model_info(...).get_model_instructions(...)`.
   - `config.base_instructions` and resumed conversation history can override current model defaults.
   - Review, realtime, compact, memory, and subagent flows may use separate prompt templates.
3. Edit all required model surfaces together:
   - Update `prompt.md` for the canonical shared wording.
   - Update every relevant `models.json` model entry, including `gpt-5.3-codex`, `gpt-5.5`, and any other supported Globim model.
   - Update `model_info.rs` when a model slug needs local prompt preservation or personality fallback.
   - Update the protocol fallback when the base identity changed.
4. Preserve the safety envelope:
   - Keep language that Globim follows active system/developer/user hierarchy, repository rules, safety constraints, and validation requirements.
   - Do not promise unrestricted obedience.
   - Keep "mestre" or lore language sparse and clearly fictional.
5. Add or update regression tests:
   - Test that remote model metadata cannot replace Globim prompt fields for every managed model family.
   - Include `gpt-5.3-codex` and `gpt-5.5`; add namespaced cases when changing prefix/suffix matching.
   - Assert effective instructions contain the new Globim wording and do not contain the upstream Codex identity being replaced.
6. Validate locally:
   - `cargo fmt -- --config imports_granularity=Item` from `codex-rs` after Rust edits.
   - `cargo test -p codex-models-manager`
   - `cargo test -p codex-core personality::`
   - `cargo test -p codex-protocol` when protocol fallback files changed.
   - `cargo build -p codex-cli --bin codex`
   - `git diff --check`
7. Smoke test the installed CLI when runtime prompt behavior changed:
   - Rebuild and reinstall local npm tarballs if package contents changed.
   - Run `globim exec "Diga quem voce e em uma frase curta."`
   - Expected result: the answer identifies as Globim. UI labels or banners may still be separate branding work.

## Completion Checklist

- All relevant model slugs were inspected, not only the default model.
- `gpt-5.3-codex` and `gpt-5.5` are covered by prompt text, preservation logic, and tests when applicable.
- Remote `/models` refresh behavior was considered.
- Fallback prompt behavior was considered.
- Validation results are recorded in the handoff or session artifacts.
