# Tools Overview

The tool system is split between model-facing definitions and runtime execution.

- `codex-rs/tools`: shared schema builders, Responses API tool specs, registry
  plan construction, and tool-shape tests.
- `codex-rs/core`: concrete handlers, turn/session state, hooks, approvals,
  sandboxing, runtime execution, MCP calls, events, and accounting.

This separation is deliberate. Do not move approval flow, sandboxing, session
state, or process execution into `codex-tools`.

Source references:

- `codex-rs/tools/README.md:3-9`
- `codex-rs/tools/README.md:60-66`
- `codex-rs/tools/src/tool_spec.rs:18-58`
- `codex-rs/tools/src/tool_registry_plan_types.rs:11-43`
- `codex-rs/core/src/tools/spec.rs:71-299`
- `codex-rs/core/src/tools/registry.rs:38-92`

## Current Active Tool Families

- Shell and host execution:
  - `shell`
  - `local_shell`
  - `shell_command`
  - `exec_command`
  - `write_stdin`
  - `request_permissions`
- Patch editing:
  - `apply_patch` as freeform custom tool or JSON function tool.
- Planning and goal tools:
  - `update_plan`
  - `get_goal`
  - `create_goal`
  - `update_goal`
- MCP:
  - namespaced MCP tools such as `mcp__server__tool`
  - `list_mcp_resources`
  - `list_mcp_resource_templates`
  - `read_mcp_resource`
  - `tool_search` for deferred MCP/discoverable tools.
- Local utility:
  - `list_dir`
  - `view_image`
  - `test_sync_tool` in test scenarios.
- Collaboration:
  - legacy v1 agent tools.
  - MultiAgentV2 task-path tools.

Source references:

- `codex-rs/tools/src/tool_registry_plan.rs:138-192`
- `codex-rs/tools/src/tool_registry_plan.rs:194-260`
- `codex-rs/tools/src/tool_registry_plan.rs:274-356`
- `codex-rs/tools/src/tool_registry_plan.rs:371-489`

## Runtime Dispatch

The runtime path is:

```text
Responses API output item
  -> ToolRouter converts item to ToolCall
  -> ToolRegistry selects handler
  -> pre-tool hooks
  -> mutating tool gate
  -> handler parses arguments and builds runtime request
  -> ToolOrchestrator or direct runtime path
  -> post-tool hooks
  -> model-visible output item
```

Source references:

- `codex-rs/core/src/tools/router.rs:175-297`
- `codex-rs/core/src/tools/registry.rs:265-512`
- `codex-rs/core/src/tools/context.rs:58-112`
- `codex-rs/core/src/tools/context.rs:139-204`
- `codex-rs/core/src/tools/context.rs:247-335`
- `codex-rs/core/src/tools/context.rs:374-480`

## Safety Boundary

Mutating tools must declare mutability or otherwise reuse runtime gates. The
orchestrator centralizes:

- approval requirement.
- sandbox selection.
- first execution attempt.
- sandbox denial analysis.
- optional escalation approval.
- retry without sandbox.

Source references:

- `codex-rs/core/src/tools/registry.rs:372-393`
- `codex-rs/core/src/tools/orchestrator.rs:126-376`
- `codex-rs/core/src/tools/sandboxing.rs:40-117`
- `codex-rs/core/src/tools/sandboxing.rs:279-417`

## Parallelism

Parallel execution is opt-in per call. Parallel-capable calls take a shared lock;
serial calls take an exclusive lock. This means non-parallel calls exclude each
other and exclude parallel calls.

Source references:

- `codex-rs/core/src/tools/parallel.rs:27-49`
- `codex-rs/core/src/tools/parallel.rs:83-143`

## Extension Rule

For any new model-visible tool:

1. Add the schema to `codex-rs/tools`.
2. Add a handler kind or reuse an existing one in the registry plan.
3. Register a concrete handler in core spec construction.
4. Implement runtime behavior in `codex-rs/core`.
5. Add hooks, mutability, approval, sandbox, and output shape tests.
6. Add integration tests through the core tool harness if the tool has side
   effects.

