# Analytics, Feedback, And OTEL

Goblins has three related observability surfaces: product analytics, user
feedback upload, and OpenTelemetry export. They share runtime entry points but
have different privacy, auth, routing, and failure semantics.

## Analytics Events

Analytics is enabled unless config explicitly sets `analytics_enabled =
Some(false)`. Events are posted to the Codex backend only when first-party Codex
auth is available; unsupported providers do not get analytics upload.

Source references:

- `codex-rs/analytics/src/client.rs:116-130`
- `codex-rs/analytics/src/client.rs:226-242`
- `codex-rs/analytics/src/client.rs:354-390`
- `codex-rs/analytics/src/events.rs:49-78`
- `codex-rs/analytics/src/events.rs:426-460`
- `codex-rs/analytics/src/events.rs:612-730`
- `codex-rs/codex-api/src/telemetry.rs:68-98`

## OpenTelemetry

OTEL config controls log, trace, and metric export. The Statsig exporter is
metrics-only and disabled in debug builds. Log and trace filters are intentionally
split so sensitive exports can be controlled independently.

Source references:

- `codex-rs/otel/src/config.rs:6-80`
- `codex-rs/otel/src/lib.rs:18-66`
- `codex-rs/otel/src/provider.rs:46-160`
- `codex-rs/otel/tests/suite/otel_export_routing_policy.rs:95-852`
- `codex-rs/otel/tests/suite/validation.rs:35-91`
- `codex-rs/otel/tests/suite/send.rs:13-217`

## Runtime Wiring

The TUI treats OTEL startup as best-effort and keeps running when export setup
fails. The app-server builds a `codex-app-server` provider and wires JSON logs,
analytics, and feedback handling into its message-processing layer.

Source references:

- `codex-rs/tui/src/lib.rs:970-1011`
- `codex-rs/app-server/src/lib.rs:540-594`
- `codex-rs/app-server/src/message_processor.rs:669-734`

## Feedback Upload

Feedback capture and upload is separate from analytics. App-server exposes
feedback endpoints and the feedback crate handles redaction, attachments,
submission payloads, and retry/error surfaces.

Source references:

- `codex-rs/feedback/src/lib.rs:25-124`
- `codex-rs/feedback/src/lib.rs:160-242`
- `codex-rs/feedback/src/lib.rs:334-526`
- `codex-rs/feedback/src/lib.rs:580-617`
- `codex-rs/app-server/README.md:222-246`

## Persistent Log Data

SQLite log storage is a local observability substrate. It is useful for replay,
thread metadata, debugging, and app-server/TUI state, but it should not be
confused with outbound analytics or OTEL exporters.

Source references:

- `codex-rs/state/src/log_db.rs:190-333`
- `codex-rs/state/src/log_db.rs:548-599`

## Invariants

- Analytics is disabled only when `analytics_enabled == Some(false)`.
- Analytics upload requires Codex backend auth.
- OTEL setup failures should not break the TUI.
- Statsig export is metrics-only and unavailable in debug builds.
- Feedback upload has its own payload and API path; do not route it through
  generic analytics events.
- Local logs are not the same thing as telemetry export.

Read next:

- [Runtime Harness](../architecture/runtime-harness.md)
- [State And Persistence](../architecture/state-persistence.md)
- [Model Providers And Auth](../auth/model-providers-and-auth.md)
