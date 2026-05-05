# App-Server Protocol

The app-server protocol is the external client boundary. New API work should
target v2 unless there is an explicit compatibility reason to touch v1.

## JSON-RPC-Lite Shape

The protocol uses a JSON-RPC-lite shape and does not require the `"jsonrpc":
"2.0"` field. Requests, notifications, responses, and errors are serialized
through tagged method enums and untagged request-id/response structs.

Source references:

- `codex-rs/app-server-protocol/src/jsonrpc_lite.rs:1-87`
- `codex-rs/app-server-protocol/src/protocol/common.rs:147-212`
- `codex-rs/app-server-protocol/src/protocol/common.rs:1116-1165`
- `codex-rs/app-server-protocol/src/lib.rs:1-49`

## Initialization And Experimental Gating

`initialize` sets per-connection experimental API flags, client info,
originator/user-agent data, platform information, and home-dir information.
Requests after initialization may require experimental capability.

Experimental goal methods and notifications are marked in protocol metadata.

Source references:

- `codex-rs/app-server/src/message_processor.rs:582-776`
- `codex-rs/app-server-protocol/src/protocol/common.rs:486-503`
- `codex-rs/app-server-protocol/src/protocol/common.rs:1361-1364`
- `codex-rs/app-server-protocol/src/protocol/common.rs:2768-2835`

## Request Serialization Scopes

Client requests can have serialization scopes. This matters for operations that
must be ordered by `thread_id`, such as persisted goal mutations and other
thread-local state changes.

Source references:

- `codex-rs/app-server-protocol/src/protocol/common.rs:147-212`
- `codex-rs/app-server-protocol/src/protocol/common.rs:1650-1662`

## Common V2 Surfaces

The v2 protocol defines thread start/resume/fork/list/read, turn start, model
list, thread items, session sources, and collab replay items.

Source references:

- `codex-rs/app-server-protocol/src/protocol/common.rs:424-660`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:3540-3668`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:4406-4421`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:5440-5572`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:5868-5893`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:6442-6575`

## Thread Goal Protocol

Goal protocol objects mirror the core/state goal model with app-server v2 DTOs:

- `ThreadGoalStatus`
- `ThreadGoal`
- `ThreadGoalSetParams`
- `ThreadGoalSetResponse`
- `ThreadGoalGetParams`
- `ThreadGoalGetResponse`
- `ThreadGoalClearParams`
- `ThreadGoalClearResponse`
- `ThreadGoalUpdatedNotification`
- `ThreadGoalClearedNotification`

Source references:

- `codex-rs/app-server-protocol/src/protocol/v2.rs:3982-4077`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:6658-6672`
- `codex-rs/protocol/src/protocol.rs:3644-3693`

## Collab Replay Protocol

App-server v2 replay currently serializes collab agent tool calls using tool
names such as `spawnAgent`, `sendInput`, `resumeAgent`, `wait`, and `closeAgent`.
MultiAgentV2 runtime maps newer `send_message` and `followup_task` semantics to
existing send/input display paths rather than adding separate replay enum
variants.

Source references:

- `codex-rs/app-server-protocol/src/protocol/thread_history.rs:600-835`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:5868-5893`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:6442-6575`
- `codex-rs/tui/src/chatwidget.rs:4282-4475`

## New API Checklist

When adding or changing an app-server API:

- Prefer v2.
- Use lower-case slash-separated RPC method names.
- Keep Rust DTO names explicit and stable.
- Use camelCase wire fields.
- Add request/response structs and notification structs as needed.
- Add protocol macro entries and serialization scope if needed.
- Update schema fixtures and generated TypeScript exports.
- Update app-server docs and integration tests.
- Preserve initialization and experimental gating rules.

Source references:

- `AGENTS.md:174-218`
- `codex-rs/app-server-protocol/src/lib.rs:1-49`
- `codex-rs/app-server/src/message_processor.rs:582-860`

