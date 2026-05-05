# Feature Overview

Goblins inherits the Codex feature-flag system and adds fork-specific product
identity through package, prompt, and release surfaces. Feature docs in this
directory focus on behavior that crosses UI, app-server, core, state, and model
tools.

## Feature Flag Discipline

Features may be stable/default-on or under-development/default-off. Tool
surfaces, protocol methods, TUI commands, and runtime policies often branch on
feature availability. A feature change must be checked at all of these layers:

- Feature declaration and stage.
- Config schema and profile overrides.
- Tool registry gating.
- App-server experimental method/notification gating.
- TUI command visibility.
- Core runtime behavior.
- Tests and snapshots.

Source references:

- `codex-rs/features/src/lib.rs:143-148`
- `codex-rs/features/src/lib.rs:834-849`
- `codex-rs/features/src/lib.rs:975-980`
- `codex-rs/core/config.schema.json:442-444`

## Documented Features

- [Thread Goals](goal.md): persisted `/goal`, model goal tools, app-server
  `thread/goal/*` APIs, SQLite state, accounting, budget limiting, pause/resume,
  and idle continuation.
- [Subagents And Orchestration](../agents/subagents.md): collaboration tools,
  legacy v1 agent ids, MultiAgentV2 task paths, `AgentControl`, registry,
  mailbox, and app/TUI metadata.
- [Tools Overview](../tools/overview.md): shell, unified exec, patch, MCP,
  resources, list_dir, parallelism, and validation harnesses.

## Current High-Risk Feature Areas

- Goals are under development and disabled by default.
- MultiAgentV2 is under development and disabled by default.
- Spawn CSV/fanout is under development and separate from the core collaboration
  surface.
- TUI/app-server flow is transitional, so features may still have both core
  event and app-server notification paths.

Source references:

- `codex-rs/features/src/lib.rs:143-148`
- `codex-rs/features/src/lib.rs:834-849`
- `codex-rs/features/src/lib.rs:975-980`
- `codex-rs/tui/src/app/app_server_adapter.rs:1-12`

