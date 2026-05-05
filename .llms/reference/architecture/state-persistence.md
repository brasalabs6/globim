# State And Persistence

Persistence is split across rollout/thread history, SQLite state, SQLite logs,
and the `ThreadStore` abstraction.

## SQLite State Runtime

`codex-state` is SQLite-backed state for rollout metadata. It mirrors selected
JSONL rollout metadata into SQLite and keeps the state surface intentionally
small.

`StateRuntime` initializes state and log databases under Codex home, runs
migrations, computes max thread update timestamps, and starts log maintenance.
SQLite uses create-if-missing, WAL, synchronous normal, busy timeout, and
migrations.

Source references:

- `codex-rs/state/src/lib.rs:1-6`
- `codex-rs/state/src/runtime.rs:86-156`
- `codex-rs/state/src/runtime.rs:164-228`

## Logs

The log DB tracing layer uses a bounded queue and background batch insertion into
a separate logs database. Keep logs separate from user-visible rollout state.

Source references:

- `codex-rs/state/src/log_db.rs:1-7`

## Thread Store Boundary

`ThreadStore` abstracts thread creation, resume, append, flush, shutdown,
history load, read, list, metadata update, archive, and unarchive. Core and
app-server should depend on the trait boundary instead of assuming local file
layout.

`LocalThreadStore` is filesystem/SQLite-backed and supports rollout-path reads
for legacy local code. `RemoteThreadStore` is gRPC-backed and explicitly work in
progress; do not imply local feature parity without checking unsupported paths.

Source references:

- `codex-rs/thread-store/src/lib.rs:1-44`
- `codex-rs/thread-store/src/store.rs:19-84`
- `codex-rs/thread-store/src/local/mod.rs:41-97`
- `codex-rs/thread-store/src/remote/mod.rs:27-32`

## Goal State

Thread goals use a dedicated SQLite table with one row per thread. `thread_id`
is the primary key and cascades on thread deletion. The row stores `goal_id`,
objective, status, optional token budget, tokens used, time used, and
millisecond timestamps.

Source references:

- `codex-rs/state/migrations/0029_thread_goals.sql:1-11`
- `codex-rs/state/src/model/thread_goal.rs:11-63`
- `codex-rs/state/src/runtime/goals.rs:23-105`
- `codex-rs/state/src/runtime/goals.rs:157-424`

## Dynamic Tool And Remote Control State

The migration set also includes thread dynamic tools and remote-control
enrollments. When adding persisted features, prefer explicit migrations and
small state models rather than storing opaque runtime blobs.

Source references:

- `codex-rs/state/migrations/0001_threads.sql:1-25`
- `codex-rs/state/migrations/0004_thread_dynamic_tools.sql:1-11`
- `codex-rs/state/migrations/0024_remote_control_enrollments.sql:1-10`

## Extension Checklist

- Add a migration for schema changes.
- Add typed model/row conversion in `codex-rs/state/src/model`.
- Add state runtime APIs in `codex-rs/state/src/runtime`.
- Preserve local and remote thread-store expectations.
- Add tests for replacement/update/delete/accounting semantics.
- Update app-server/core protocol conversion if state becomes visible outside
  the state crate.

