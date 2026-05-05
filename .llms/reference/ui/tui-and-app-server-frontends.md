# TUI And App-Server Frontends

The terminal UI is the default frontend, but many flows are moving through the
app-server boundary. Treat the current state as transitional: some events still
drain directly from core while app-server-backed flows grow.

## TUI Runtime

The TUI starts from `codex-rs/tui`, creates or connects to an app-server
session, renders chat widgets, bottom panes, overlays, status indicators, goal
state, and agent navigation. User-visible UI changes usually need snapshots.

Source references:

- `codex-rs/cli/src/main.rs:70-176`
- `codex-rs/cli/src/app_cmd.rs:4-24`
- `codex-rs/tui/src/lib.rs:259-496`
- `codex-rs/tui/src/lib.rs:449-920`
- `codex-rs/tui/src/lib.rs:678-1117`
- `codex-rs/tui/src/app_server_session.rs:125-181`
- `codex-rs/tui/src/app_server_session.rs:225-409`
- `codex-rs/tui/src/app_server_session.rs:541-634`
- `codex-rs/tui/src/app_server_session.rs:771-995`
- `codex-rs/tui/src/chatwidget.rs:1-12181`
- `codex-rs/tui/src/slash_command.rs:1-269`
- `codex-rs/tui/src/app/agent_navigation.rs:29-197`
- `AGENTS.md:109-134`

## App-Server Adapter

The adapter documents the migration state directly: the TUI still drains direct
core streams while features are being moved behind app-server messages. Do not
assume a feature is purely app-server-backed until the relevant UI path confirms
it.

Source references:

- `codex-rs/tui/src/app/app_server_adapter.rs:1-12`
- `codex-rs/tui/src/app/app_server_adapter.rs:125-464`
- `codex-rs/tui/src/app_server_session.rs:689-740`
- `codex-rs/app-server-client/src/lib.rs:1-16`
- `codex-rs/app-server-client/src/lib.rs:156-210`
- `codex-rs/app-server-client/src/lib.rs:441-590`
- `codex-rs/app-server-client/src/remote.rs:1-9`
- `codex-rs/app-server-client/src/remote.rs:94-208`
- `codex-rs/app-server/src/lib.rs:126-134`
- `codex-rs/app-server/src/lib.rs:605-758`
- `codex-rs/app-server/src/lib.rs:797-940`
- `codex-rs/app-server/src/message_processor.rs:582-740`
- `codex-rs/app-server/src/codex_message_processor.rs:1141-1154`
- `codex-rs/app-server/src/codex_message_processor.rs:6532-6746`

## User Commands

Slash commands, app events, and app-server session wrappers form the usual path
for user-triggered frontend behavior. New user-facing features need command
parsing, app events, app-server calls, rendering, and snapshot coverage.

Source references:

- `codex-rs/tui/src/chatwidget/slash_dispatch.rs:210-223`
- `codex-rs/tui/src/chatwidget/slash_dispatch.rs:609-685`
- `codex-rs/tui/src/app_event.rs:211-232`
- `codex-rs/tui/src/app/event_dispatch.rs:562-578`
- `codex-rs/tui/src/app_server_session.rs:1-2119`

## Invariants

- Do not update only the TUI for a feature that app-server clients can observe.
- Snapshot tests are required for intentional terminal UI rendering changes.
- Metadata hydration matters for agents, goals, resumed threads, and replay.
- App-server response ordering can matter for resume/start snapshots.
- Embedded app-server is the default path; remote app-server uses WebSocket.
- Bearer tokens are accepted only on `wss://` or loopback `ws://`.
- Clients initialize once per connection; experimental API availability is
  connection-scoped.
- Lossless events block; best-effort events may lag or drop.

## When Changing This

- Update app-server wrappers and protocol docs for new frontend-visible APIs.
- Run narrow TUI tests and `cargo insta pending-snapshots -p codex-tui`.
- Recheck app-server integration tests if the feature also has remote clients.

Read next:

- [App-Server Protocol](../protocol/app-server.md)
- [Subagents And Orchestration](../agents/subagents.md)
- [Thread Goals](../features/goal.md)
- `codex-rs/app-server/README.md:22-73`
- `docs/tui-chat-composer.md:1-348`
- `docs/tui-alternate-screen.md:1-130`
