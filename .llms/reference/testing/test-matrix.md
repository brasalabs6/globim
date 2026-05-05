# Test Matrix

Use this matrix to choose the smallest useful validation set for Goblins
changes. Prefer scoped tests first, then widen only when the touched surface
crosses crate, protocol, UI, SDK, or release boundaries.

## Command Entry Points

The `justfile` is the command index for common validation. `AGENTS.md` adds the
repo-specific rules: run targeted Rust tests, keep TUI snapshots intentional,
and regenerate protocol artifacts when app-server protocol changes.

Source references:

- `justfile:1-119`
- `AGENTS.md:57-62`
- `AGENTS.md:111-153`
- `AGENTS.md:181-219`

## Rust Runtime

Run crate-level tests near the changed code before full workspace tests. Common
scoped commands:

```bash
cd codex-rs
cargo test -p codex-tools tool_registry_plan
cargo test -p codex-core --test all unified_exec
cargo test -p codex-mcp
cargo test -p codex-state goals
cargo test -p codex-app-server --test all v2
cargo test -p codex-app-server-protocol
cargo test -p codex-tui
cargo insta pending-snapshots -p codex-tui
```

Source references:

- `codex-rs/app-server/tests/suite/v2/mod.rs:1-66`
- `codex-rs/app-server/tests/suite/v2/initialize.rs:29-120`
- `codex-rs/app-server/tests/suite/v2/compaction.rs:1-220`
- `codex-rs/app-server/tests/suite/v2/realtime_conversation.rs:1-220`
- `codex-rs/app-server/tests/suite/v2/review.rs:36-180`
- `codex-rs/core/tests/suite/mod.rs:1-113`
- `codex-rs/core/tests/suite/compact.rs:1-220`
- `codex-rs/core/tests/suite/compact_remote.rs:1-220`
- `codex-rs/core/tests/suite/realtime_conversation.rs:1-220`
- `codex-rs/core/tests/suite/review.rs:36-180`

## Protocol And SDKs

App-server protocol changes must keep Rust DTOs, JSON schema, generated
TypeScript protocol fixtures, app-server docs, and Python SDK consumers aligned.
The TypeScript SDK is separate: it is a hand-written wrapper around Codex CLI
JSONL event streams and needs updates only when that CLI event surface or SDK
wrapper behavior changes.

Common commands:

```bash
just write-app-server-schema
(cd codex-rs && cargo test -p codex-app-server-protocol)
(cd sdk/typescript && pnpm test)
(cd sdk/python && uv run pytest)
```

Source references:

- `sdk/typescript/package.json:34-45`
- `sdk/typescript/jest.config.cjs:1-18`
- `sdk/python/pyproject.toml:60-62`
- `sdk/python/tests/test_contract_generation.py:1-52`
- `sdk/python/tests/test_real_app_server_integration.py:1-545`

## Lints And Generated Artifacts

Run formatting and lint commands for touched languages. The argument-comment
lint catches Rust call-site clarity issues that standard Rust tooling will not.
Schema commands update checked-in artifacts and should be committed with the
source change that required them.

Source references:

- `tools/argument-comment-lint/README.md:57-88`
- `tools/argument-comment-lint/README.md:131-144`
- `AGENTS.md:181-219`

## Docs-Only Validation

For `.llms/reference` changes, validate whitespace, file inventory, and stale
placeholder absence before widening to code tests. These commands are a smoke
check; use a dedicated Markdown-link/source-reference checker when exact link
and line-range hygiene matters.

```bash
git diff --check -- .llms/reference
find .llms/reference -type f -name '*.md' | sort
! rg -n 'TO''DO|TB''D|FIX''ME|line_''unavailable' .llms/reference
```

Source references:

- `docs/install.md:27-49`
- `docs/contributing.md:35-58`

## Selection Rules

- For tool registry or handler changes, run `codex-tools` registry tests and
  core integration tests for the handler path.
- For approval, sandbox, hooks, or permissions changes, include tests covering
  the orchestrator and denial/escalation behavior.
- For app-server API changes, regenerate schema fixtures and run both protocol
  and app-server v2 suites.
- For TUI-visible changes, run focused TUI tests and inspect pending snapshots
  rather than accepting them blindly.
- For package, release, or launcher changes, include npm staging/release smoke
  checks and fork identity docs.

Read next:

- [Validation And Testing](../tools/validation-and-testing.md)
- [App-Server Protocol](../protocol/app-server.md)
- [SDK And Protocol Generation](../sdk/sdk-and-protocol-generation.md)
