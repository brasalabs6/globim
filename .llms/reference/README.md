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
- Use [Config And Feature Flags](config/config-and-feature-flags.md) before
  changing profiles, feature gates, managed requirements, or config layering.
- Use [Personality And Context](prompts/personality-and-context.md) before
  changing prompt injection, developer context, or Goblins persona defaults.
- Use [Tools Overview](tools/overview.md) before the specific tool subsystem
  references:
  - [Tool Registry](tools/tool-registry.md)
  - [Shell And Unified Exec](tools/shell-and-exec.md)
  - [File System And Patch Tools](tools/file-system-and-patches.md)
  - [MCP Tools And Resources](tools/mcp.md)
  - [Validation And Testing](tools/validation-and-testing.md)
- Use [Safety, Sandbox, Permissions, And Guardian](safety/sandbox-permissions-guardian.md)
  before touching approval, sandbox, permission request, or auto-review paths.
- Use [Subagents And Orchestration](agents/subagents.md) for multi-agent work,
  and [Agent Jobs](agents/agent-jobs.md) for CSV fanout worker jobs.
- Use [App-Server Protocol](protocol/app-server.md) for client/server API work.
- Use [TUI And App-Server Frontends](ui/tui-and-app-server-frontends.md) for
  frontend/runtime boundary changes.
- Use [SDK And Protocol Generation](sdk/sdk-and-protocol-generation.md) for
  schema fixture, generated TypeScript, Python SDK, and SDK drift work.
- Use [Testing Matrix](testing/test-matrix.md) when choosing validation.
- Use [Extension Guide](extension-guide.md) as a change checklist.
- Use [User Docs And Fork Drift](docs/user-docs-and-fork-drift.md) when changing
  public docs, install instructions, package identity, or fork branding.
- Use [Glossary](glossary.md) for shared terms.

## Area Index

- Agents:
  - [Subagents And Orchestration](agents/subagents.md)
  - [Agent Jobs](agents/agent-jobs.md)
  - [External Agent Migration And Sessions](agents/external-agent-migration-and-sessions.md)
- Architecture:
  - [Architecture Overview](architecture/overview.md)
  - [Repository Map](architecture/repository-map.md)
  - [Runtime Harness](architecture/runtime-harness.md)
  - [State And Persistence](architecture/state-persistence.md)
- Auth and safety:
  - [Model Providers And Auth](auth/model-providers-and-auth.md)
  - [Safety, Sandbox, Permissions, And Guardian](safety/sandbox-permissions-guardian.md)
  - [Secrets And Proxy Boundaries](safety/secrets-and-proxy-boundaries.md)
- Config, prompts, and extensions:
  - [Config And Feature Flags](config/config-and-feature-flags.md)
  - [Personality And Context](prompts/personality-and-context.md)
  - [Skills, Plugins, Apps, And Hooks](extensions/skills-plugins-apps-hooks.md)
- Cloud, remote, and observability:
  - [Cloud Tasks And Requirements](cloud/cloud-tasks-and-requirements.md)
  - [Remote Control And Device Keys](runtime/remote-control-and-device-keys.md)
  - [Analytics, Feedback, And OTEL](observability/analytics-feedback-otel.md)
- Frontends and protocol:
  - [App-Server Protocol](protocol/app-server.md)
  - [TUI And App-Server Frontends](ui/tui-and-app-server-frontends.md)
  - [SDK And Protocol Generation](sdk/sdk-and-protocol-generation.md)
- Runtime features:
  - [Feature Overview](features/overview.md)
  - [Thread Goals](features/goal.md)
  - [Turn Context, Compaction, Realtime, And Review](runtime/turn-context-compaction-realtime-review.md)
- Tools and validation:
  - [Tools Overview](tools/overview.md)
  - [Tool Registry](tools/tool-registry.md)
  - [Shell And Unified Exec](tools/shell-and-exec.md)
  - [File System And Patch Tools](tools/file-system-and-patches.md)
  - [MCP Tools And Resources](tools/mcp.md)
  - [Validation And Testing](tools/validation-and-testing.md)
  - [Testing Matrix](testing/test-matrix.md)
- Release and docs:
  - [Fork Packaging And Release](release/fork-packaging-and-release.md)
  - [User Docs And Fork Drift](docs/user-docs-and-fork-drift.md)

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
