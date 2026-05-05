# Repository Map

This map identifies the main directories and ownership boundaries for future
changes.

## Top Level

- `README.md`: fork-facing overview, installation, release baseline, public
  package and command names.
- `SPECs.md`: fork contract, branch policy, prompt catalog expectations, release
  requirements.
- `GOBLINS.md`: working contract for package names, prompts, branching, and
  release naming.
- `AGENTS.md`: contributor guidance for Rust, tests, protocol changes, MCP, and
  TUI snapshots.
- `package.json`: private monorepo tooling package.
- `pnpm-workspace.yaml`: JavaScript package workspace membership.
- `.github/workflows/`: CI and release workflow surfaces.
- `prompts/`: local prompt fallbacks for Goblins personalities.
- `codex-cli/`: shipped npm package and Node launcher.
- `codex-rs/`: primary Rust workspace.
- `sdk/`: TypeScript SDK workspace.
- `docs/`: user-facing docs inherited from or adapted from upstream.
- `.llms/reference/`: LLM-facing implementation reference docs for high-risk
  code paths, feature surfaces, and fork-specific invariants.

Source references:

- `README.md:1-35`
- `SPECs.md:3-50`
- `GOBLINS.md:3-40`
- `AGENTS.md:1-218`
- `package.json:1-35`
- `pnpm-workspace.yaml:1-16`
- `prompts/goblin.md:1-89`

## Rust Workspace

`codex-rs/Cargo.toml` defines the Rust workspace, package version `0.128.1`,
edition 2024, Apache-2.0 license, workspace members, and internal `codex-*`
dependencies.

Important crates:

- `cli`: `codex-cli` package, native binary named `codex`, dispatches interactive
  TUI, exec, review, login/logout, MCP, app-server, schema, sandbox, apply,
  resume/fork, cloud, responses proxy, exec server, and feature listing.
- `tui`: terminal UI, app-server client/session adapter, chat widget, bottom
  pane, snapshots, goal display, agent navigation.
- `app-server`: JSON-RPC-lite server, transports, connection processing,
  thread/model/auth/MCP/plugin/skill/fs/device APIs.
- `app-server-protocol`: wire protocol, v2 DTOs, JSON-RPC-lite structs,
  experimental API metadata, schema/TypeScript generation.
- `core`: session runtime, threads, model turn loop, tools, MCP calls, plugins,
  skills, goals, sandboxing, execution policy, agent control.
- `protocol`: core operation/event/item protocol and shared DTOs.
- `state`: SQLite state/log runtime and migrations.
- `thread-store`: storage-neutral thread persistence boundary with local and
  remote implementations.
- `tools`: model-facing tool schema builders and registry-plan construction.
- `codex-mcp`: MCP connection manager, RMCP clients, tool/resource qualification,
  runtime environment.
- `file-system`, `file-search`, `apply-patch`, `exec`, `sandboxing`,
  `shell-command`, `shell-escalation`: supporting execution and file tooling.
- `analytics`, `otel`, `feedback`: telemetry, feedback, and observability
  surfaces.
- `cloud-tasks`, `cloud-tasks-client`, `cloud-requirements`: cloud task CLI/API
  flows and managed requirements.
- `external-agent-migration`, `external-agent-sessions`: import paths for
  compatible external-agent config and sessions.
- `login`, `model-provider`, `model-provider-info`: auth state, provider
  definitions, and provider-specific auth behavior.

Source references:

- `codex-rs/Cargo.toml:1-118`
- `codex-rs/Cargo.toml:120-227`
- `codex-rs/cli/Cargo.toml:1-80`
- `codex-rs/tui/Cargo.toml:1-155`
- `codex-rs/app-server/Cargo.toml:1-127`
- `codex-rs/app-server-protocol/Cargo.toml:1-43`
- `codex-rs/core/Cargo.toml:1-163`
- `codex-rs/protocol/Cargo.toml:1-63`
- `codex-rs/state/Cargo.toml:1-29`
- `codex-rs/thread-store/Cargo.toml:1-40`

## Ownership Heuristics

- Package identity and native staging: `codex-cli` and `scripts/`.
- Terminal UI behavior: `codex-rs/tui`.
- External client API: `codex-rs/app-server` plus `codex-rs/app-server-protocol`.
- Session/turn behavior: `codex-rs/core/src/session`, `tasks`, `thread_manager`,
  and `codex_thread`.
- Tool schemas: `codex-rs/tools`.
- Tool execution: `codex-rs/core/src/tools`.
- Shell/unified execution: `codex-rs/core/src/tools/handlers`,
  `codex-rs/core/src/tools/runtimes`, `codex-rs/core/src/unified_exec`, and
  `codex-rs/core/src/exec_policy.rs`.
- MCP startup/list/call/resource naming: `codex-rs/codex-mcp`.
- MCP runtime approvals and model-call handling: `codex-rs/core/src/mcp_tool_call.rs`
  and `codex-rs/core/src/tools/handlers/mcp*.rs`.
- Persistent thread metadata and logs: `codex-rs/state` and `codex-rs/thread-store`.
- Core protocol objects: `codex-rs/protocol`.
- Config layering and feature flags: `codex-rs/config`, `codex-rs/features`,
  and `codex-rs/core/src/config`.
- Auth and model providers: `codex-rs/login`, `codex-rs/model-provider`, and
  `codex-rs/model-provider-info`.
- Extensions: `codex-rs/core-skills`, `codex-rs/core-plugins`,
  `codex-rs/plugin`, `codex-rs/hooks`, and app connector paths.
- Cloud and remote runtime: `codex-rs/cloud-*`, app-server remote-control
  transport, `codex-rs/device-key`, and state migrations.
- Observability: `codex-rs/analytics`, `codex-rs/otel`, `codex-rs/feedback`,
  and `codex-rs/state/src/log_db.rs`.
- SDK/protocol generation: `codex-rs/app-server-protocol`, `sdk/python`, and
  `sdk/typescript`.

## Known Documentation Risks

- `codex-rs/README.md` still contains upstream package naming in places, while
  root docs are fork-facing.
- The TUI/app-server split is transitional.
- App-server protocol exports v1 and v2, but new API work should be v2.
- `codex-core` is large and tempting; contributor guidance says not to place new
  code there by default.
- Public and internal binary names intentionally differ.
- `RemoteThreadStore` exists but is explicitly work in progress.
- User docs under `docs/` may drift from fork identity unless checked against
  root fork contract docs.

Source references:

- `codex-rs/README.md:1-27`
- `codex-rs/tui/src/app/app_server_adapter.rs:1-12`
- `codex-rs/app-server-protocol/src/lib.rs:1-49`
- `AGENTS.md:64-75`
- `codex-cli/bin/goblin.js:56-65`
- `codex-rs/thread-store/src/remote/mod.rs:27-32`
