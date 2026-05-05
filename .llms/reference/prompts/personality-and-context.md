# Personality And Context

Goblins is a prompt and personality fork as well as a package fork. Runtime
prompt fallbacks live under `prompts/`, remote prompt catalog data can refresh
those prompts, and model metadata preserves Goblins-owned prompt fields when
remote model catalogs are loaded.

## Prompt Contract

The fork contract says the base Goblins prompt lives at `prompts/goblin.md`.
Standalone personality prompts live under `prompts/personalities/`. Selecting a
personality replaces the base prompt for that session; it is not a small style
append layered onto the base prompt.

Source references:

- `README.md:31-35`
- `SPECs.md:22-35`
- `GOBLINS.md:25-32`
- `prompts/goblin.md:1-350`
- `prompts/personalities/friendly.md:1-370`
- `prompts/personalities/pragmatic.md:1-360`
- `codex-rs/protocol/src/config_types.rs:216-220`

## Prompt Catalog And Cache

`PromptPack` models the base and built-in personality prompts. The model manager
applies local prompt overrides to model metadata so remote model refresh does
not erase Goblins prompt fields. Cache logic decides when remote model/catalog
data can be reused, refetched, or treated as stale.

Source references:

- `codex-rs/models-manager/src/prompt_catalog.rs:1-206`
- `codex-rs/models-manager/src/model_info.rs:13-143`
- `codex-rs/models-manager/src/manager.rs:1-490`
- `codex-rs/models-manager/src/cache.rs:14-183`
- `codex-rs/models-manager/src/manager_tests.rs:356-426`
- `codex-rs/models-manager/src/manager_tests.rs:824-847`
- `codex-rs/protocol/src/openai_models.rs:22-23`
- `codex-rs/protocol/src/openai_models.rs:323-406`

## Context Assembly

Core context code assembles current turn input from model metadata,
instructions, repository guidance, tools, memory/context managers, and runtime
state. Prompt/personality work should be treated as one part of the request
context, not as an isolated text file change.

Source references:

- `codex-rs/core/src/context/mod.rs:1-54`
- `codex-rs/core/src/context_manager/history.rs:1-726`
- `codex-rs/core/src/context_manager/normalize.rs:1-345`
- `codex-rs/core/src/context_manager/updates.rs:126-238`
- `codex-rs/core/src/config/mod.rs:2600-2648`
- `codex-rs/core/src/session/mod.rs:539-550`
- `codex-rs/core/src/session/mod.rs:2535-2806`
- `codex-rs/core/src/session/turn.rs:136-356`
- `codex-rs/core/src/session/turn.rs:936-1018`
- `codex-rs/core/src/session/turn_context.rs:48-95`
- `codex-rs/core/src/session/turn_context.rs:327-556`
- `codex-rs/core/src/session/turn_context.rs:667-749`

## Invariants

- `Personality::None` uses the base Goblins prompt.
- Built-in personality IDs currently require protocol/UI support; adding a file
  alone is not enough.
- Base instructions come from explicit config override, resumed session
  metadata, or model default instructions.
- Personality can be baked into model instructions when the template has the
  `{{ personality }}` placeholder; otherwise it is injected as developer
  context.
- Initial turns record full context; later turns record diffs against
  `TurnContextItem`.
- Remote model/catalog refresh must preserve Goblins-owned prompt fields.
- Prompt changes can affect all models because personality is a prompt-layer
  capability, not a model capability.
- Keep system/developer/user instruction hierarchy stronger than prompt style.

## When Changing This

- Update `README.md`, `SPECs.md`, and `GOBLINS.md` if the fork prompt contract
  changes.
- Update prompt catalog, fallback prompt pack, model metadata application, and
  personality tests together.
- Run model-manager prompt/cache tests and relevant TUI personality tests.

Read next:

- [Fork Packaging And Release](../release/fork-packaging-and-release.md)
- [Config And Feature Flags](../config/config-and-feature-flags.md)
- [TUI And App-Server Frontends](../ui/tui-and-app-server-frontends.md)
- `docs/agents_md.md:1-7`
- `docs/config.md:105-119`
