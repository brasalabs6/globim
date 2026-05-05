# Remote Control And Device Keys

Remote control lets an app-server connect to a backend-mediated controller
channel. Device keys bind local clients to signed remote-control payloads.
Together they are a transport/auth boundary, not a general-purpose thread API.

## Remote-Control Transport

Remote control lives under the app-server transport layer. It tracks client
connections, enrollment, protocol messages, WebSocket behavior, and status
notifications. Initialization and app-server auth still apply before remote
control can expose useful thread operations.

Source references:

- `codex-rs/features/src/lib.rs:211-212`
- `codex-rs/features/src/lib.rs:1017-1022`
- `codex-rs/app-server/src/transport/remote_control/mod.rs:1-125`
- `codex-rs/app-server/src/transport/remote_control/mod.rs:37-122`
- `codex-rs/app-server/src/transport/remote_control/client_tracker.rs:1-570`
- `codex-rs/app-server/src/transport/remote_control/client_tracker.rs:23-328`
- `codex-rs/app-server/src/transport/remote_control/enroll.rs:1-514`
- `codex-rs/app-server/src/transport/remote_control/enroll.rs:23-159`
- `codex-rs/app-server/src/transport/remote_control/enroll.rs:193-256`
- `codex-rs/app-server/src/transport/remote_control/protocol.rs:1-252`
- `codex-rs/app-server/src/transport/remote_control/websocket.rs:1-1951`
- `codex-rs/app-server/src/transport/remote_control/websocket.rs:52-115`
- `codex-rs/app-server/src/transport/remote_control/websocket.rs:241-453`
- `codex-rs/app-server/src/transport/remote_control/websocket.rs:486-603`
- `codex-rs/app-server/src/transport/remote_control/websocket.rs:631-834`
- `codex-rs/app-server/src/transport/remote_control/websocket.rs:836-1000`
- `codex-rs/app-server/src/transport/auth.rs:1-751`

## Device-Key API

App-server v2 exposes `device/key/create`, `device/key/public`, and
`device/key/sign`. Protocol DTOs model key algorithms, protection classes,
protection policy, and remote-control-specific sign payloads. The app-server
maps those DTOs into `codex-device-key` requests and stores bindings through
state.

Source references:

- `codex-rs/app-server-protocol/src/protocol/common.rs:619-632`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:2990-3148`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:2978-3152`
- `codex-rs/app-server/src/message_processor.rs:720-740`
- `codex-rs/app-server/src/message_processor.rs:843-863`
- `codex-rs/app-server/src/message_processor.rs:1140-1207`
- `codex-rs/app-server/src/device_key_api.rs:37-101`
- `codex-rs/app-server/src/device_key_api.rs:103-170`
- `codex-rs/app-server/src/device_key_api.rs:174-314`
- `codex-rs/device-key/src/lib.rs:29-170`
- `codex-rs/device-key/src/lib.rs:171-1495`
- `codex-rs/device-key/src/platform.rs:9-49`

## Persistence

Remote-control enrollment state uses a dedicated SQLite migration. Device-key
bindings are persisted through state runtime records, not ad hoc files.

Source references:

- `codex-rs/state/migrations/0024_remote_control_enrollments.sql:1-10`
- `codex-rs/state/src/runtime/remote_control.rs:3-120`
- `codex-rs/state/src/runtime.rs:1-86`
- `codex-rs/app-server/tests/suite/v2/device_key.rs:1-119`

## Invariants

- Device-key signatures must be bound to structured payloads and expected
  audiences/scopes.
- Hardware-backed keys are preferred; degraded OS-protected keys require an
  explicit policy that allows them.
- Remote-control status is client-visible through `remoteControl/status/changed`.
- Keep enrollment challenge handling distinct from normal connection signing.
- Do not bypass app-server auth/initialization through remote-control code.
- Remote control requires feature enablement, SQLite state, and ChatGPT auth
  with account id; API-key auth is rejected.
- Enrollment cache key is WebSocket URL plus account id plus app-server client
  name.
- Remote-control WebSocket streams are keyed by `(client_id, stream_id)` with
  sequence ids, stream-scoped ACKs, buffered replay, and subscribe cursors.
- Remote inbound streams become local app-server connections only after
  `initialize`.
- Device-key RPCs are local-only and are not served over remote transport.

## When Changing This

- Update v2 DTOs, schema fixtures, and device-key tests together.
- Add remote-control transport tests for connection, enrollment, reconnect, and
  status notification changes.
- Recheck [App-Server Protocol](../protocol/app-server.md) for method naming
  and experimental gating.

Read next:

- [Model Providers And Auth](../auth/model-providers-and-auth.md)
- [State And Persistence](../architecture/state-persistence.md)
- [Safety](../safety/sandbox-permissions-guardian.md)
