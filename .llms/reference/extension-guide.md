# Extension Guide

Use this checklist before implementing new Goblins features or modifying agents,
tools, MCP, goals, app-server protocol, or runtime state.

## Pick The Right Layer

- Public command/package/release identity: `codex-cli`, `scripts`, workflows.
- Terminal UI: `codex-rs/tui`.
- External API: `codex-rs/app-server` and `codex-rs/app-server-protocol`.
- Session and turn runtime: `codex-rs/core/src/session`, `tasks`,
  `thread_manager`, `codex_thread`.
- Model-facing tool schema: `codex-rs/tools`.
- Tool execution: `codex-rs/core/src/tools`.
- MCP lifecycle/naming/resources: `codex-rs/codex-mcp`.
- MCP approvals/call metadata: `codex-rs/core/src/mcp_tool_call.rs`.
- Persistent state: `codex-rs/state`.
- Thread history boundary: `codex-rs/thread-store`.
- Core shared protocol: `codex-rs/protocol`.

## App-Server API Changes

- Prefer v2.
- Add request/response/notification DTOs.
- Add macro registrations and serialization scope.
- Preserve initialize and experimental gates.
- Update schema fixtures and generated TypeScript exports.
- Update docs and integration tests.
- Add TUI app-server wrappers when needed.

Read first:

- [App-Server Protocol](protocol/app-server.md)
- `AGENTS.md:174-218`

## Tool Changes

- Add schema in `codex-rs/tools`.
- Add or reuse a `ToolHandlerKind`.
- Register specs and handlers in core spec construction.
- Implement a `ToolHandler`.
- Mark mutating behavior correctly.
- Add hook payloads if policy or observability should see the call.
- Use `ToolOrchestrator` for approval/sandbox/retry when mutating or sandboxable.
- Add output schema and model-readable errors.
- Add core integration tests.

Read first:

- [Tools Overview](tools/overview.md)
- [Tool Registry](tools/tool-registry.md)
- [Shell And Unified Exec](tools/shell-and-exec.md)

## MCP Changes

- Prefer `McpConnectionManager` for startup, listing, resources, filtering,
  naming, and calls.
- Keep approval, metadata, sandbox-state augmentation, and OpenAI file argument
  handling in core MCP call paths.
- Preserve `mcp__server__tool` naming.
- Update deferred/tool_search exposure tests if exposure changes.

Read first:

- [MCP Tools And Resources](tools/mcp.md)

## File Or Patch Changes

- Use `ExecutorFileSystem` and sandbox contexts for file runtimes.
- Use `ReadDenyMatcher` for local read/list behavior.
- Keep `apply_patch` first-class; do not let shell/unified exec bypass patch
  verification or approval.
- If adding local read/search tools, first reconcile inactive-looking
  `read_file` and `grep_files` tests with current registry state.

Read first:

- [File System And Patch Tools](tools/file-system-and-patches.md)

## Goal Changes

- Goals are under-development and default-off.
- Preserve materialized local-thread requirement.
- Preserve one-goal-per-thread unless the schema and API are redesigned.
- Keep model tools limited unless deliberately changing the safety contract.
- Update state, core protocol, app-server v2, TUI, tool schemas, runtime policy,
  and tests together.

Read first:

- [Thread Goals And `/goal`](features/goal.md)

## Subagent Changes

- Keep `AgentControl` root-tree scope.
- Preserve `AgentRegistry` reservation and release semantics.
- Preserve v1 id routing vs v2 path routing.
- Preserve mailbox delivery phases.
- Update app-server replay and TUI hydration when event shape changes.

Read first:

- [Subagents And Orchestration](agents/subagents.md)

## Validation Ladder

1. Run formatting or diff checks for the touched files.
2. Run narrow unit tests for the changed crate/module.
3. Run integration tests for changed runtime flows.
4. Run TUI snapshots for UI changes.
5. Run package/release smoke checks for package identity or release changes.

Read first:

- [Validation And Testing](tools/validation-and-testing.md)

