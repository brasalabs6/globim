# Tool Registry

Tool exposure is planned in `codex-rs/tools` and executed in `codex-rs/core`.
The registry plan is the contract between those layers.

## Plan Types

`ToolHandlerKind` enumerates handler families such as shell, unified exec, MCP,
MCP resources, apply_patch, list_dir, goal, plan, request input, tool search,
dynamic tool, view image, and collaboration tools.

`ToolRegistryPlan` contains:

- model-visible tool specs.
- handler-name to handler-kind registrations.

Source references:

- `codex-rs/tools/src/tool_registry_plan_types.rs:11-43`
- `codex-rs/tools/src/tool_registry_plan_types.rs:51-119`

## Plan Construction

`build_tool_registry_plan` chooses tools based on `ToolsConfig`:

- Shell tool shape from `config.shell_type`.
- MCP resource tools when MCP resources are enabled.
- `update_plan` always in supported tool configs.
- Goal tools when `config.goal_tools` is true.
- `request_user_input` in Plan mode/tool-enabled contexts.
- `request_permissions` when permission request tool is enabled.
- `tool_search` and deferred MCP tools for discoverable tool loading.
- `tool_suggest` for installable plugins/connectors.
- `apply_patch` when configured and environment-backed.
- `list_dir` when experimental and environment-backed.
- web/image/view image when enabled.
- collaboration tools depending on v1 vs MultiAgentV2 config.

Source references:

- `codex-rs/tools/src/tool_registry_plan.rs:72-77`
- `codex-rs/tools/src/tool_registry_plan.rs:138-192`
- `codex-rs/tools/src/tool_registry_plan.rs:194-260`
- `codex-rs/tools/src/tool_registry_plan.rs:274-356`
- `codex-rs/tools/src/tool_registry_plan.rs:371-489`
- `codex-rs/tools/src/tool_registry_plan.rs:509-599`

## Core Spec Construction

Core turns the plan into a `ToolRouter`. Handler kinds become concrete handler
instances such as shell, unified exec, MCP, MCP resource, apply_patch, goal,
plan, request input, request permissions, list_dir, view_image, collaboration,
and test-sync handlers.

Source references:

- `codex-rs/core/src/tools/spec.rs:71-78`
- `codex-rs/core/src/tools/spec.rs:112-180`
- `codex-rs/core/src/tools/spec.rs:187-299`

## Handler Trait

`ToolHandler` declares:

- `kind()`
- `matches_kind()`
- `is_mutating()`
- pre-tool hook payload.
- post-tool hook payload.
- optional diff consumer.
- async `handle()`.

The generic registry wraps these typed handlers behind `AnyToolHandler`.

Source references:

- `codex-rs/core/src/tools/registry.rs:38-92`
- `codex-rs/core/src/tools/registry.rs:162-204`
- `codex-rs/core/src/tools/registry.rs:215-239`

## Dispatch Lifecycle

During dispatch the registry:

1. Resolves handler by model-visible tool name.
2. Checks payload kind.
3. Records origin/metrics.
4. Runs pre-tool hooks.
5. Gates mutating tools.
6. Calls `handle_any`.
7. Runs post-tool hooks.
8. Records traces.
9. Returns a typed output.

Source references:

- `codex-rs/core/src/tools/registry.rs:265-512`

## Output Contract

All handler outputs implement `ToolOutput`, which converts runtime results into:

- model response items.
- post-tool hook responses.
- code-mode result JSON.

Important output wrappers:

- `McpToolOutput`
- `FunctionToolOutput`
- `ApplyPatchToolOutput`
- `ExecCommandToolOutput`

Source references:

- `codex-rs/core/src/tools/context.rs:91-112`
- `codex-rs/core/src/tools/context.rs:139-204`
- `codex-rs/core/src/tools/context.rs:247-335`
- `codex-rs/core/src/tools/context.rs:374-480`

## Change Checklist

- Keep tool schema, handler kind, core handler registration, runtime behavior,
  and tests in sync.
- Add parallel support deliberately; it affects global tool-call locking.
- Add mutability detection for tools that change files, process state, network
  state, MCP state, or session state.
- Provide hook payloads when external policy/observability should see the call.
- Keep unsupported/deferred/dynamic tool error paths model-readable.

