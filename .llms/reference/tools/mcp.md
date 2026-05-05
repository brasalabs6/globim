# MCP Tools And Resources

MCP integration is split between `codex-mcp` and `codex-core`.

- `codex-mcp` owns server lifecycle, RMCP clients, startup status, tool/resource
  listing, canonical tool naming, filtering, transport origin tracking, and
  call routing.
- `codex-core` owns model tool-call handling, approvals, hooks, metadata,
  sandbox-state augmentation, OpenAI file argument handling, and result
  sanitization.

Source references:

- `codex-rs/codex-mcp/src/connection_manager.rs:1-7`
- `codex-rs/codex-mcp/src/connection_manager.rs:69-129`
- `codex-rs/core/src/tools/handlers/mcp.rs:18-100`
- `codex-rs/core/src/mcp_tool_call.rs:85-189`

## Runtime Environment

MCP runtime state can include sandbox metadata for capable servers:

- permission profile.
- sandbox policy.
- Codex Linux sandbox executable path.
- sandbox cwd.
- legacy Landlock flag.

`McpRuntimeEnvironment` describes where servers should run for the current
caller and provides fallback cwd for stdio servers that omit cwd.

Source references:

- `codex-rs/codex-mcp/src/runtime.rs:1-65`

## Connection Manager

`McpConnectionManager` starts enabled servers, emits startup update/complete
events, tracks origins, stores clients by server name, aggregates tools,
resources, and resource templates, and routes tool calls to the right client.

It also exposes whether a server supports sandbox-state metadata.

Source references:

- `codex-rs/codex-mcp/src/connection_manager.rs:69-129`
- `codex-rs/codex-mcp/src/connection_manager.rs:131-268`
- `codex-rs/codex-mcp/src/connection_manager.rs:318-380`
- `codex-rs/codex-mcp/src/connection_manager.rs:383-565`

## Tool Naming And Filtering

MCP `ToolInfo` preserves raw server/tool metadata plus callable namespace/name.
`qualify_tools` sanitizes and deduplicates names using the
`mcp__{server}__{tool}` shape. RMCP client startup validates server names,
starts transports, snapshots/cache tools for Codex Apps, filters enabled/disabled
tools, masks file-path params for model-visible schemas, and annotates plugin or
connector provenance.

Source references:

- `codex-rs/codex-mcp/src/tools.rs:28-53`
- `codex-rs/codex-mcp/src/tools.rs:108-130`
- `codex-rs/codex-mcp/src/tools.rs:133-228`
- `codex-rs/codex-mcp/src/rmcp_client.rs:84-119`
- `codex-rs/codex-mcp/src/rmcp_client.rs:131-214`
- `codex-rs/codex-mcp/src/rmcp_client.rs:238-300`

## MCP Tool Schema Conversion

MCP tool schemas are converted to Responses API tool definitions. If an MCP
input schema omits `properties` or sets it to null, Codex inserts an empty
object because OpenAI models require the field.

Source references:

- `codex-rs/tools/src/mcp_tool.rs:6-63`

## MCP Resource Tools

`codex-tools` defines:

- `list_mcp_resources`
- `list_mcp_resource_templates`
- `read_mcp_resource`

Core handles them through `McpResourceHandler`. Listing all servers disallows
cursors and delegates through `session.services.mcp_connection_manager`.

Source references:

- `codex-rs/tools/src/mcp_resource_tool.rs:6-94`
- `codex-rs/core/src/tools/handlers/mcp_resource.rs:180-349`

## Naming And Approval Invariants

- MCP server names are validated before startup and must match the supported
  alphanumeric, underscore, and dash pattern.
- Model-visible names are sanitized and qualified as `mcp__server__tool`; raw
  server/tool metadata is preserved for routing.
- Colliding or too-long model-visible names are disambiguated with SHA-1-derived
  suffixes and capped at the Responses API tool-name limit.
- Approval policy belongs in core MCP call handling, not in the connection
  manager.

Source references:

- `codex-rs/codex-mcp/src/rmcp_client.rs:442-446`
- `codex-rs/codex-mcp/src/tools.rs:29-53`
- `codex-rs/codex-mcp/src/tools.rs:136-228`
- `codex-rs/codex-mcp/src/tools.rs:240-350`
- `codex-rs/core/src/mcp_tool_call.rs:726-1095`

## MCP Call Flow

Core MCP calls:

1. Parse JSON arguments.
2. Emit MCP begin/end events.
3. Determine approval mode.
4. Block disabled Codex Apps tools.
5. Add turn/thread metadata.
6. Optionally upload OpenAI file arguments.
7. Add sandbox-state metadata for capable servers.
8. Call the connection manager.
9. Sanitize image outputs for models without image input.

Approval can auto-approve based on policy/profile, require annotation or prompt
approval, use cached keys, run permission hooks, route through guardian review,
or use MCP elicitation/user input.

Source references:

- `codex-rs/core/src/mcp_tool_call.rs:85-189`
- `codex-rs/core/src/mcp_tool_call.rs:291-384`
- `codex-rs/core/src/mcp_tool_call.rs:530-606`
- `codex-rs/core/src/mcp_tool_call.rs:620-649`
- `codex-rs/core/src/mcp_tool_call.rs:726-829`
- `codex-rs/core/src/mcp_tool_call.rs:899-1095`
- `codex-rs/protocol/src/protocol.rs:2344-2376`

## Config And Plugin Surfaces

Global and plugin MCP servers are loaded from config, filtered by requirements,
and may include per-tool approval overrides. Prefer existing config edit helpers
for MCP config mutation.

Source references:

- `codex-rs/config/src/config_toml.rs:178-179`
- `codex-rs/config/src/mcp_edit.rs:20-121`
- `codex-rs/core/src/mcp.rs:9-34`
- `codex-rs/core/src/plugins/manager.rs:1309`
- `codex-rs/core/src/config/config_tests.rs:3395-4436`

## First-Party MCP Server

`codex-rs/mcp-server` is the first-party stdio MCP server. It loads Codex
config, initializes exec-server environment management, sets app residency
requirements, wires OTEL, reads JSON-RPC messages from stdin, writes responses to
stdout, and exposes `codex` plus `codex-reply` tools that start or continue
Codex sessions.

Approval requests are sent back to the MCP client through `elicitation/create`
for exec and patch approvals, then converted into Codex `Op::ExecApproval` or
`Op::PatchApproval` submissions. Keep this client-facing approval loop distinct
from model-visible MCP tool calls handled by `codex-core`.

Source references:

- `codex-rs/mcp-server/src/lib.rs:59-110`
- `codex-rs/mcp-server/src/lib.rs:112-191`
- `codex-rs/mcp-server/src/message_processor.rs:47-76`
- `codex-rs/mcp-server/src/message_processor.rs:187-273`
- `codex-rs/mcp-server/src/message_processor.rs:308-348`
- `codex-rs/mcp-server/src/message_processor.rs:350-418`
- `codex-rs/mcp-server/src/codex_tool_config.rs:20-65`
- `codex-rs/mcp-server/src/codex_tool_config.rs:109-198`
- `codex-rs/mcp-server/src/codex_tool_runner.rs:56-148`
- `codex-rs/mcp-server/src/exec_approval.rs:17-48`
- `codex-rs/mcp-server/src/exec_approval.rs:50-147`
- `codex-rs/mcp-server/src/patch_approval.rs:20-41`
- `codex-rs/mcp-server/src/patch_approval.rs:43-142`

## Extension Checklist

- Prefer `McpConnectionManager` for listing, naming, filtering, resources, and
  call routing.
- Keep approval and sandbox-state augmentation in core MCP call handling.
- Keep the first-party MCP server's client-elicitation approval flow aligned
  with Codex approval ops when changing MCP-hosted Codex sessions.
- Preserve deterministic `mcp__server__tool` naming and collision behavior.
- Update resource tools and handler if adding resource/template behavior.
- Update tool_search/deferred exposure tests when changing MCP exposure.
- Preserve OpenAI file parameter rewrite behavior for file-taking MCP tools.

Test surfaces:

- `codex-rs/core/tests/suite/rmcp_client.rs:20-260`
- `codex-rs/core/tests/suite/hooks_mcp.rs:129-220`
- `codex-rs/core/tests/suite/openai_file_mcp.rs:93-220`
- `codex-rs/core/tests/suite/search_tool.rs:141-245`
- `codex-rs/codex-mcp/src/connection_manager_tests.rs:90-234`
