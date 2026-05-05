# Goblins Reference Index

This directory is a durable LLM-facing reference for modifying Goblins safely.
It documents the current codebase shape, important invariants, extension points,
and feature surfaces that are easy to break when changing agents, tools, MCP,
app-server protocol, or `/goal`.

The references are intentionally hierarchical:

- Start with [Architecture Overview](architecture/overview.md).
- Use [Repository Map](architecture/repository-map.md) to find ownership.
- Use [Runtime Harness](architecture/runtime-harness.md) to understand how a user
  request moves from CLI/TUI/app-server into core, model turns, tools, and state.
- Use [State And Persistence](architecture/state-persistence.md) for rollout,
  SQLite state, logs, and thread-store boundaries.
- Use [Feature Overview](features/overview.md) before reading individual feature
  docs such as [Thread Goals](features/goal.md).
- Use [Tools Overview](tools/overview.md) before the specific tool subsystem
  references:
  - [Tool Registry](tools/tool-registry.md)
  - [Shell And Unified Exec](tools/shell-and-exec.md)
  - [File System And Patch Tools](tools/file-system-and-patches.md)
  - [MCP Tools And Resources](tools/mcp.md)
  - [Validation And Testing](tools/validation-and-testing.md)
- Use [Subagents And Orchestration](agents/subagents.md) for multi-agent work.
- Use [App-Server Protocol](protocol/app-server.md) for client/server API work.
- Use [Extension Guide](extension-guide.md) as a change checklist.
- Use [Glossary](glossary.md) for shared terms.

## Repository Contract

Goblins is a community fork of OpenAI Codex CLI. The first-version fork contract
keeps upstream Rust architecture intact while changing the public package,
command, prompt defaults, README surface, and release pipeline identity.

Key naming rules:

- Public npm package: `@brasalabs/goblins`.
- Public command: `goblin`.
- Internal Rust executable and native payload path: `codex`.
- Development branch for the fork: `goblins`.
- Root docs and release scripts are fork-facing; some `codex-rs` docs still use
  upstream Codex naming and must be read with that caveat.

Primary source references:

- `SPECs.md:3-50`
- `GOBLINS.md:3-40`
- `README.md:1-35`
- `codex-cli/package.json:1-21`
- `codex-cli/bin/goblin.js:15-95`

## Reading Discipline

Each reference file uses repository-relative source paths with line numbers.
When implementing a change, inspect the cited source before editing. Some docs
describe under-development features and transitional architecture; treat those
sections as current-state documentation, not as a promise that the shape is final.

Important repo constraints to remember:

- Do not assume `codex-core` is the right home for new code. The contributor
  guide explicitly warns against adding code there by default.
- New app-server APIs should be v2 APIs and must update protocol types, schema
  fixtures, TypeScript surfaces, docs, and tests together.
- TUI snapshots are required for user-visible terminal UI changes.
- Avoid code changes related to `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` and
  `CODEX_SANDBOX_ENV_VAR` unless the task explicitly targets that area.

Source references:

- `AGENTS.md:7-10`
- `AGENTS.md:64-75`
- `AGENTS.md:109-134`
- `AGENTS.md:174-218`

