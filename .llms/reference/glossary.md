# Glossary

## AgentPath

Task-path identifier used by MultiAgentV2. Paths are rooted at `/root`, and task
names use lowercase ASCII letters, digits, or underscores.

Source references:

- `codex-rs/protocol/src/agent_path.rs:17-72`
- `codex-rs/protocol/src/agent_path.rs:125-180`

## App-Server

The JSON-RPC-lite server used by TUI and external clients. It exposes thread,
turn, model, auth, MCP, plugin, skill, goal, and agent-control operations.

Source references:

- `codex-rs/app-server/src/lib.rs:126-134`
- `codex-rs/app-server/src/message_processor.rs:162-178`

## CodexThread

Core wrapper around a `Codex` session. It exposes submit/event channels and
tracks source, rollout path, elicitation count, and shutdown state.

Source references:

- `codex-rs/core/src/codex_thread.rs:90-118`
- `codex-rs/core/src/codex_thread.rs:186-274`

## Goal

A persisted thread objective managed by the `goals` feature. Stored in SQLite,
visible through TUI `/goal`, app-server `thread/goal/*`, and model tools.

Source references:

- `codex-rs/state/migrations/0029_thread_goals.sql:1-11`
- `codex-rs/tools/src/goal_tool.rs:12-91`

## MCP

Model Context Protocol integration. `codex-mcp` owns server lifecycle and
connection management; core owns model tool-call execution policy.

Source references:

- `codex-rs/codex-mcp/src/connection_manager.rs:1-7`
- `codex-rs/core/src/mcp_tool_call.rs:85-189`

## Rollout

Persisted thread history/log form used by local thread storage and resume flows.
SQLite state mirrors selected metadata.

Source references:

- `codex-rs/state/src/lib.rs:1-6`
- `codex-rs/thread-store/src/local/mod.rs:41-97`

## ThreadStore

Storage-neutral trait for creating, reading, appending, resuming, archiving, and
listing thread data.

Source references:

- `codex-rs/thread-store/src/store.rs:19-84`

## ToolOrchestrator

Central approval, sandbox selection, retry, and escalation harness for
sandboxable tool runtimes.

Source references:

- `codex-rs/core/src/tools/orchestrator.rs:1-8`
- `codex-rs/core/src/tools/orchestrator.rs:126-376`

## ToolRegistryPlan

The `codex-tools` plan that maps model-visible specs and handler names to core
handler kinds.

Source references:

- `codex-rs/tools/src/tool_registry_plan_types.rs:51-119`

## Unified Exec

Long-lived process execution surface behind `exec_command` and `write_stdin`.

Source references:

- `codex-rs/tools/src/local_tool.rs:19-134`
- `codex-rs/core/src/tools/handlers/unified_exec.rs:179-417`

