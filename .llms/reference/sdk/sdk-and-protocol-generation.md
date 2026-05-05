# SDK And Protocol Generation

The app-server protocol drives checked-in JSON schema fixtures, generated
TypeScript protocol files, and the Python SDK. The TypeScript SDK is separate:
it wraps `codex exec --experimental-json` and follows CLI JSONL event shapes.
Protocol changes are not complete until generated artifacts and schema-driven
SDK consumers are reconciled.

## Protocol Source

`codex-app-server-protocol` defines JSON-RPC-lite wrappers, client/server
request enums, notification enums, v2 DTOs, experimental filtering, schema
fixture generation, and TypeScript exports.

Source references:

- `codex-rs/app-server-protocol/src/lib.rs:1-49`
- `codex-rs/app-server-protocol/src/jsonrpc_lite.rs:1-87`
- `codex-rs/app-server-protocol/src/protocol/common.rs:147-212`
- `codex-rs/app-server-protocol/src/protocol/common.rs:480-660`
- `codex-rs/app-server-protocol/src/protocol/common.rs:1116-1210`
- `codex-rs/app-server-protocol/src/protocol/common.rs:1360-1436`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:1-80`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:3540-3668`

## Schema Fixtures

Schema artifacts live under `codex-rs/app-server-protocol/schema/json` and
`schema/typescript`. Regenerate them with `just write-app-server-schema`; include
`--experimental` when experimental surface changes. Tests compare checked-in
fixtures to fresh generation.

Source references:

- `AGENTS.md:174-218`
- `justfile:96-98`
- `codex-rs/app-server-protocol/src/export.rs:44-120`
- `codex-rs/app-server-protocol/src/export.rs:186-246`
- `codex-rs/app-server-protocol/src/export.rs:291-545`
- `codex-rs/app-server-protocol/src/export.rs:980-1323`
- `codex-rs/app-server-protocol/tests/schema_fixtures.rs:1-143`
- `codex-rs/app-server-protocol/src/bin/export.rs:1-34`
- `codex-rs/app-server-protocol/src/bin/write_schema_fixtures.rs:1-42`

## TypeScript SDK

The TypeScript SDK wraps the `codex` CLI from npm, runs
`codex exec --experimental-json`, and exchanges JSONL events over stdio. Its
event types are based on `codex-rs/exec/src/exec_events.rs`, not generated from
the app-server v2 schema.

Source references:

- `sdk/typescript/README.md:1-13`
- `sdk/typescript/package.json:34-45`
- `sdk/typescript/src/index.ts:1-40`
- `sdk/typescript/src/codex.ts:1-39`
- `sdk/typescript/src/exec.ts:1-220`
- `sdk/typescript/src/exec.ts:350-389`
- `sdk/typescript/src/thread.ts:1-120`
- `sdk/typescript/src/events.ts:1-82`
- `sdk/typescript/src/outputSchemaFile.ts:1-40`
- `sdk/typescript/tests/run.test.ts:1-808`
- `sdk/typescript/tests/exec.test.ts:1-145`

## Python SDK

The Python SDK consumes the v2 schema bundle, generates model surfaces, stages
SDK packages, and tests public API behavior plus real app-server integration.

Source references:

- `sdk/python/README.md:1-110`
- `sdk/python/scripts/update_sdk_artifacts.py:20-74`
- `sdk/python/scripts/update_sdk_artifacts.py:236-528`
- `sdk/python/scripts/update_sdk_artifacts.py:531-603`
- `sdk/python/scripts/update_sdk_artifacts.py:909-981`
- `sdk/python/scripts/update_sdk_artifacts.py:984-1114`
- `sdk/python/src/codex_app_server/client.py:1-542`
- `sdk/python/src/codex_app_server/async_client.py:1-208`
- `sdk/python/src/codex_app_server/api.py:1-791`
- `sdk/python/tests/test_client_rpc_methods.py:1-130`
- `sdk/python/tests/test_contract_generation.py:1-52`
- `sdk/python/tests/test_real_app_server_integration.py:1-545`

## Invariants

- New app-server APIs should be v2.
- V2 DTOs need `#[ts(export_to = "v2/")]`.
- Keep serde and TypeScript renames aligned.
- Do not use `skip_serializing_if` on v2 response/notification fields unless
  explicitly required.
- Experimental APIs must be filtered correctly in schema and TS outputs.
- Python SDK consumes `codex_app_server_protocol.v2.schemas.json`; TypeScript
  SDK is hand-written around `codex exec --experimental-json` and is not
  schema-generated automatically.

## When Changing This

- Run `just write-app-server-schema` and, when needed,
  `just write-app-server-schema --experimental`.
- Run `cargo test -p codex-app-server-protocol`.
- Run TypeScript SDK build/test when SDK surfaces are affected.
- Run Python SDK artifact/update tests when schema feeds Python generation.

Read next:

- [App-Server Protocol](../protocol/app-server.md)
- [Testing Matrix](../testing/test-matrix.md)
