# Runtime Harness

The harness is the end-to-end execution path that turns a user command or client
request into a model turn, tool calls, events, and persisted state.

## CLI To UI Or Server

`codex-rs/cli/src/main.rs` is a Clap multitool. With no subcommand it starts the
interactive TUI. Other subcommands route to exec/review, login/logout, MCP
server operations, app-server, schema generation, sandbox helpers, apply,
resume/fork, cloud, responses proxy, exec server, and feature listing.

Source references:

- `codex-rs/cli/src/main.rs:70-176`
- `codex-rs/cli/src/main.rs:733-890`
- `codex-rs/cli/src/main.rs:903-955`

## TUI Runtime

`run_main` normalizes remote WebSocket vs embedded app-server mode, validates
auth token transport, maps dangerous bypass flags into approval/sandbox config,
loads config, runs personality migration, handles cloud/OSS model behavior, and
launches UI state.

Embedded TUI mode builds an `InProcessClientStartArgs` with config, feedback,
log DB, environment manager, client name `codex-tui`, experimental API enabled,
and CLI session source.

Source references:

- `codex-rs/tui/src/lib.rs:449-496`
- `codex-rs/tui/src/lib.rs:678-920`
- `codex-rs/tui/src/app_server_session.rs:146-255`

## App-Server Request Harness

The app-server starts config management, personality migration, auth/cloud
requirements, telemetry, state/log DBs, transport acceptors, optional remote
control, outbound routing, and `MessageProcessor`.

`initialize` is special. It stores per-connection experimental flags, client
info, originator/user-agent metadata, and platform/home-dir data. Non-initialize
requests require an initialized connection and may require experimental
capability.

Source references:

- `codex-rs/app-server/src/lib.rs:390-780`
- `codex-rs/app-server/src/lib.rs:574-674`
- `codex-rs/app-server/src/message_processor.rs:582-776`
- `codex-rs/app-server/src/message_processor.rs:793-860`

## Thread Startup Harness

Thread start validates mutually exclusive sandbox and permission settings,
builds config overrides, resolves instructions, dynamic tool state, environment
state, and calls `ThreadManager.start_thread_with_options`. The first event from
`Codex::spawn` must be `SessionConfigured` before the thread is inserted in the
manager.

Source references:

- `codex-rs/app-server/src/codex_message_processor.rs:2426-2535`
- `codex-rs/app-server/src/codex_message_processor.rs:2649-2905`
- `codex-rs/core/src/thread_manager.rs:563-587`
- `codex-rs/core/src/thread_manager.rs:1058-1169`

## Session And Turn Harness

`Codex::spawn` creates bounded submission and unbounded event channels, validates
environments, loads plugins/skills/AGENTS instructions, builds execution policy,
loads model/prompt catalogs, builds base instructions, configures dynamic tools
and collaboration mode, and starts the submission loop.

User input operations update session settings and create turn context. A turn
loads context, plugin instructions, MCP tools, skills, hook prompts, and user
prompt data before sampling. The turn repeatedly samples the model and executes
function/tool calls until the assistant message completes the turn.

Source references:

- `codex-rs/core/src/session/mod.rs:419-674`
- `codex-rs/core/src/session/handlers.rs:123-295`
- `codex-rs/core/src/session/turn.rs:120-130`
- `codex-rs/core/src/session/turn.rs:136-358`

## Tool Harness

`ToolRouter` holds visible tool specs and the concrete `ToolRegistry`.
`ToolRegistry` resolves a handler, runs pre hooks, gates mutating tools on the
turn tool-call gate, invokes the handler, runs post hooks, records dispatch
traces, and returns typed output.

The `ToolOrchestrator` is the central approval/sandbox/retry harness for
sandboxable runtimes. It asks whether approval is required, chooses an initial
sandbox, runs the attempt, analyzes sandbox denial, optionally requests
escalation, then retries without sandbox when approved.

Source references:

- `codex-rs/core/src/tools/router.rs:39-100`
- `codex-rs/core/src/tools/router.rs:175-297`
- `codex-rs/core/src/tools/registry.rs:38-92`
- `codex-rs/core/src/tools/registry.rs:265-512`
- `codex-rs/core/src/tools/orchestrator.rs:1-8`
- `codex-rs/core/src/tools/orchestrator.rs:126-376`

## Harness Invariants

- Do not bypass `ThreadManager` when creating or resuming threads.
- Do not bypass `ToolRegistry` for model-visible tool calls.
- Do not bypass `ToolOrchestrator`, `exec_policy`, or sandbox runtimes for
  mutating/shell-like tools.
- Do not call low-level `exec()` as if it applies sandboxing; callers must pass
  sandbox-wrapped commands.
- Keep app-server request serialization scopes and experimental gates intact.
- Preserve event ordering when app-server listeners need ordered thread
  notifications.

Source references:

- `codex-rs/core/src/exec.rs:903-915`
- `codex-rs/app-server-protocol/src/protocol/common.rs:147-212`
- `codex-rs/app-server/src/codex_message_processor/thread_goal_handlers.rs:389-438`

