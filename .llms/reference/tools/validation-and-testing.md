# Validation And Testing

This file maps the main validation harnesses for changes to tools, app-server,
TUI, state, goals, and subagents.

## CI And Release Signals

CI runs on pull requests, manual dispatch, and pushes to the `goblins` branch.
It validates Cargo workspace manifest inheritance, TUI/core boundary checks,
Bazel clippy parity, native build, personality path, npm staging, README checks,
and prettier.

Release runs on `rust-v*.*.*` tags, validates tag/version/branch contract, builds
Linux musl and Windows native artifacts, stages npm packages, creates GitHub
Release artifacts, and publishes npm tarballs when appropriate.

Source references:

- `.github/workflows/ci.yml:1-108`
- `.github/workflows/rust-release.yml:1-421`
- `scripts/stage_npm_packages.py:16-213`
- `codex-cli/scripts/build_npm_package.py:12-419`
- `codex-cli/scripts/install_native_deps.py:21-183`

## Core Tool Harness

Core integration tests are aggregated in one binary and install aliases so the
test binary can dispatch as `apply_patch` and `codex-linux-sandbox`.

The shared responses harness captures `/responses` requests and exposes helpers
for structured request JSON, input items, tool outputs, headers, path/query
checks, and call-output lookup. Repo guidance recommends these helpers over
manual JSON digging.

Source references:

- `codex-rs/core/tests/all.rs:1-5`
- `codex-rs/core/tests/suite/mod.rs:1-113`
- `codex-rs/core/tests/common/lib.rs:34-45`
- `codex-rs/core/tests/common/responses.rs:40-228`
- `AGENTS.md:147-156`

## Test Environment Builders

`TestCodexBuilder` and `TestEnv` create local or remote-aware test environments,
temporary homes/cwds, user shells, model configs, workspace setup, and remote
exec-server settings. Core test support exposes deterministic process ids and
thread-manager test switches.

Source references:

- `codex-rs/core/tests/common/test_codex.rs:71-143`
- `codex-rs/core/tests/common/test_codex.rs:187-227`
- `codex-rs/core/src/test_support.rs:1-6`
- `codex-rs/core/src/test_support.rs:40-46`

## Tool Tests

Important tool-related suites:

- `tool_harness.rs`: shell execution, plan updates, malformed payload handling,
  output shape, event emission.
- `tool_parallelism.rs`: test-sync behavior and shell parallelism.
- `apply_patch_cli.rs`: freeform/function/shell/heredoc patch calls.
- `unified_exec.rs`: apply_patch interception, begin/end events, write_stdin,
  metadata, output-token clamping.
- `search_tool.rs`: tool_search and deferred MCP workflows.
- `rmcp_client.rs`, `hooks_mcp.rs`, `openai_file_mcp.rs`: MCP lifecycle, hooks,
  file param upload/rewrite.

Source references:

- `codex-rs/core/tests/suite/tool_harness.rs:49-204`
- `codex-rs/core/tests/suite/tool_parallelism.rs:72-180`
- `codex-rs/core/tests/suite/apply_patch_cli.rs:49-220`
- `codex-rs/core/tests/suite/unified_exec.rs:230-380`
- `codex-rs/core/tests/suite/search_tool.rs:141-245`
- `codex-rs/core/tests/suite/rmcp_client.rs:20-260`
- `codex-rs/core/tests/suite/hooks_mcp.rs:129-220`
- `codex-rs/core/tests/suite/openai_file_mcp.rs:93-220`

## Goal Tests

Goal test coverage spans state, app-server integration, TUI snapshots/status, and
core session handlers.

Source references:

- `codex-rs/state/src/runtime/goals.rs:516-1180`
- `codex-rs/app-server/tests/suite/v2/thread_resume.rs:180-712`
- `codex-rs/tui/src/chatwidget/tests/slash_commands.rs:698-843`
- `codex-rs/tui/src/chatwidget/tests/status_and_layout.rs:1692-1907`
- `codex-rs/tui/src/chatwidget/tests/goal_menu.rs:10-50`
- `codex-rs/tui/src/chatwidget/tests/review_mode.rs:1325-1436`
- `codex-rs/core/src/session/tests.rs:7867-8038`

## TUI Snapshot Rule

For UI changes, run the specific snapshot tests and update snapshots only when
the UI change is intentional. The contributor guide has explicit TUI snapshot
expectations.

Source references:

- `AGENTS.md:109-134`

## Documentation-Only Validation

For `.llms/reference` docs, minimum smoke validation is:

```bash
git diff --check -- .llms/reference
find .llms/reference -type f -name '*.md' | sort
! rg -n 'TO''DO|TB''D|FIX''ME|line_''unavailable' .llms/reference
```

The smoke commands do not validate Markdown links or source-reference line
ranges. Run a dedicated checker for those when a doc pass changes many
repository-relative references.

For code changes, run the narrowest relevant tests first, then widen according
to blast radius.

## Common Commands

Run commands from `codex-rs` unless the command says otherwise:

```bash
cargo test -p codex-tools tool_registry_plan
cargo test -p codex-core --test all unified_exec
cargo test -p codex-mcp
cargo test -p codex-state goals
cargo test -p codex-app-server --test all v2
cargo test -p codex-app-server-protocol
cargo test -p codex-tui
cargo insta pending-snapshots -p codex-tui
```

For package or release identity changes, validate from the repository root with
the release/package commands documented in
[Fork Packaging And Release](../release/fork-packaging-and-release.md).

## Read Next

- [Testing Matrix](../testing/test-matrix.md) for subsystem-specific commands.
- [App-Server Protocol](../protocol/app-server.md) for API codegen rules.
