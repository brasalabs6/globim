# External Agent Migration And Sessions

External-agent migration imports configuration, extension assets, and session
records from compatible tools into Goblins. It is item-based: detection returns
typed migration items, and import applies only the selected items.

## Migration Item Detection

Detection identifies config, skills, `AGENTS.md`, plugins, MCP servers,
subagents, hooks, commands, and sessions. The app-server exposes this through v2
external-agent config APIs so UI clients can present choices before importing.

Source references:

- `codex-rs/external-agent-migration/src/lib.rs:1-124`
- `codex-rs/external-agent-migration/src/lib.rs:228-271`
- `codex-rs/external-agent-migration/src/lib.rs:505-534`
- `codex-rs/external-agent-migration/src/lib.rs:863-884`
- `codex-rs/external-agent-migration/src/lib.rs:1078-1110`
- `codex-rs/external-agent-migration/src/lib.rs:1357-1376`
- `codex-rs/app-server/src/external_agent_config_api.rs:32-93`
- `codex-rs/app-server/src/external_agent_config_api.rs:141-200`
- `codex-rs/app-server/src/external_agent_config_api.rs:205-317`

## Session Imports

Session import requires detected source paths. Import skips already-imported
current content and records ledger entries so repeated imports do not duplicate
the same session data.

Source references:

- `codex-rs/external-agent-sessions/src/lib.rs:1-132`
- `codex-rs/external-agent-sessions/src/lib.rs:178-210`
- `codex-rs/external-agent-sessions/src/detect.rs:19-82`
- `codex-rs/external-agent-sessions/src/export.rs:26-121`
- `codex-rs/external-agent-sessions/src/records.rs:16-82`
- `codex-rs/external-agent-sessions/src/records.rs:329-377`

## App-Server And TUI Surface

The app-server protocol carries external-agent config detection/import requests,
while the TUI adapter exposes migration actions through the app-server session
facade. Current UI plumbing is still part of the transitional TUI/app-server
architecture.

Source references:

- `codex-rs/app-server-protocol/src/protocol/common.rs:888-897`
- `codex-rs/app-server-protocol/src/protocol/common.rs:1395-1395`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:1082-1224`
- `codex-rs/app-server/src/config/external_agent_config.rs:43-258`
- `codex-rs/app-server/src/config/external_agent_config.rs:261-565`
- `codex-rs/app-server/src/config/external_agent_config.rs:616-745`
- `codex-rs/app-server/src/config/external_agent_config.rs:773-998`
- `codex-rs/tui/src/app_server_session.rs:326-349`
- `codex-rs/tui/src/app/app_server_adapter.rs:192-203`
- `codex-rs/tui/src/app/app_server_adapter.rs:435-448`
- `codex-rs/tui/src/external_agent_config_migration.rs:95-135`
- `codex-rs/tui/src/external_agent_config_migration.rs:635-703`
- `codex-rs/tui/src/external_agent_config_migration.rs:872-883`

## Tests

The app-server v2 suite covers detection, import, config migration, hooks, and
session-related behavior. Add tests there when changing the client-visible
migration contract.

Source references:

- `codex-rs/app-server/tests/suite/v2/external_agent_config.rs:1-974`
- `codex-rs/app-server/README.md:1438-1510`

## Invariants

- Detection must be typed enough for clients to select exact items.
- Import must be item-based, not all-or-nothing.
- Session imports require source paths and must avoid duplicate current content.
- Hook import honors disable-all behavior and must not overwrite existing hook
  scripts accidentally.
- App-server API changes need protocol, docs, schema fixtures, and v2 tests.

Read next:

- [Subagents And Orchestration](subagents.md)
- [App-Server Protocol](../protocol/app-server.md)
- [TUI And App-Server Frontends](../ui/tui-and-app-server-frontends.md)
