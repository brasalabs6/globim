# Architecture Overview

Goblins keeps the upstream Codex Rust architecture and changes fork identity at
the package, command, prompt, and release layers. The public entrypoint is the
Node package `@brasalabs/goblins` and command `goblin`; the staged native binary
inside platform packages remains `codex`.

## Runtime Flow

Current high-level flow:

```text
@brasalabs/goblins/bin/goblin.js
  -> vendor/<target>/codex/codex
  -> codex-rs/cli
  -> codex-rs/tui by default
  -> embedded or remote codex-app-server
  -> app-server JSON-RPC-lite protocol
  -> CodexMessageProcessor
  -> ThreadManager and CodexThread
  -> Codex session submission loop
  -> model turn loop
  -> ToolRouter and ToolOrchestrator
  -> tool runtimes, MCP clients, state/log/thread persistence
```

Source references:

- `codex-cli/bin/goblin.js:15-98`
- `codex-cli/bin/goblin.js:141-209`
- `codex-rs/cli/src/main.rs:70-176`
- `codex-rs/cli/src/main.rs:733-890`
- `codex-rs/tui/src/lib.rs:449-496`
- `codex-rs/app-server/src/lib.rs:126-134`
- `codex-rs/core/src/thread_manager.rs:563-587`
- `codex-rs/core/src/thread_manager.rs:1058-1169`
- `codex-rs/core/src/session/mod.rs:419-674`
- `codex-rs/core/src/session/turn.rs:120-358`
- `codex-rs/core/src/tools/router.rs:39-101`
- `codex-rs/core/src/tools/orchestrator.rs:1-8`

## Major Boundaries

### CLI and packaging

The JavaScript launcher handles platform detection, optional package resolution,
local vendor resolution, helper binary `PATH` setup, signal forwarding, and
child exit mirroring. It deliberately resolves an internal executable named
`codex` even though users invoke `goblin`.

Source references:

- `codex-cli/package.json:1-21`
- `codex-cli/bin/goblin.js:56-95`
- `codex-cli/bin/goblin.js:141-209`
- `codex-rs/cli/Cargo.toml:1-80`

### TUI

The TUI is the default interactive UI. It can start an embedded app-server or
connect to a remote app-server. The current TUI boundary is transitional:
`app_server_adapter.rs` says the TUI still drains direct core event streams while
flows move behind the app-server boundary.

Source references:

- `codex-rs/tui/src/lib.rs:1-186`
- `codex-rs/tui/src/lib.rs:678-920`
- `codex-rs/tui/src/app/app_server_adapter.rs:1-12`
- `codex-rs/tui/src/app/app_server_adapter.rs:125-236`
- `codex-rs/tui/src/app_server_session.rs:146-255`

### App-server

The app-server is the integration boundary for external clients. It supports
stdio, Unix socket, WebSocket, in-process, and disabled transports. It separates
incoming processing from outbound writes, gates requests on initialization and
experimental capability flags, and forwards core thread/model/auth/MCP/plugin
work into `CodexMessageProcessor`.

Source references:

- `codex-rs/app-server/src/main.rs:19-81`
- `codex-rs/app-server/src/transport/mod.rs:31-188`
- `codex-rs/app-server/src/lib.rs:390-780`
- `codex-rs/app-server/src/message_processor.rs:162-178`
- `codex-rs/app-server/src/message_processor.rs:582-860`
- `codex-rs/app-server/src/codex_message_processor.rs:971-1348`

### Core runtime

`codex-core` owns the session, turn, thread, tool, MCP, plugin, skill, sandbox,
and state bridge runtime. `ThreadManager` maintains active `CodexThread`
handles. `CodexThread` wraps a session with submit/event channels. `Codex::spawn`
loads configuration, instructions, plugins, skills, MCP, execution policy, model
provider state, and starts the submission loop.

Source references:

- `codex-rs/core/src/lib.rs:1-202`
- `codex-rs/core/src/thread_manager.rs:206-252`
- `codex-rs/core/src/codex_thread.rs:90-118`
- `codex-rs/core/src/codex_thread.rs:186-274`
- `codex-rs/core/src/session/mod.rs:419-674`
- `codex-rs/core/src/session/handlers.rs:123-295`
- `codex-rs/core/src/session/turn.rs:120-358`

### Tools

Tool exposure is split across `codex-tools` and `codex-core`. `codex-tools`
defines model-facing tool schemas and registry plans. `codex-core` turns those
plans into concrete handlers, executes tools, runs hooks, applies approval and
sandbox policy, emits events, and records goal accounting.

Source references:

- `codex-rs/tools/README.md:3-9`
- `codex-rs/tools/README.md:60-66`
- `codex-rs/tools/src/tool_registry_plan.rs:72-77`
- `codex-rs/tools/src/tool_registry_plan_types.rs:11-43`
- `codex-rs/core/src/tools/spec.rs:71-299`
- `codex-rs/core/src/tools/registry.rs:38-92`
- `codex-rs/core/src/tools/registry.rs:265-512`

## Extension Rules

- Add user-facing CLI/package changes in `codex-cli` and release scripts, not in
  Rust core.
- Add new app-server APIs in v2 protocol and update generated schemas, fixtures,
  TypeScript exports, docs, and tests.
- Add new tool schemas in `codex-tools`, then register concrete handlers in
  `codex-core`.
- Keep runtime execution, approval, sandbox, session, and persistence logic out
  of `codex-tools`.
- Treat TUI/app-server adapter code as current migration code, not as the target
  architecture for new app-server-backed features.

Read next:

- [Repository Map](repository-map.md) for ownership boundaries.
- [Runtime Harness](runtime-harness.md) for request-to-turn flow.
- [State And Persistence](state-persistence.md) for rollout, SQLite, and thread
  storage.
- [App-Server Protocol](../protocol/app-server.md) for external client APIs.
- [Tools Overview](../tools/overview.md) for tool schema/runtime boundaries.
