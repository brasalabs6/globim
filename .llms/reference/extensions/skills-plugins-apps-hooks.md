# Skills, Plugins, Apps, And Hooks

Extensions span skills, plugins, apps/connectors, MCP app tools, and lifecycle
hooks. The common rule is to keep discovery/configuration separate from runtime
execution and to route public client operations through app-server v2.

## Skills

Skills are local instruction bundles discovered from configured skill roots and
plugin-provided skill directories. Core-skills owns skill config rules, prompt
rendering, context-budget behavior, implicit invocation detection, and explicit
mention extraction.

Source references:

- `codex-rs/config/src/skills_config.rs:12-49`
- `codex-rs/skills/src/lib.rs:1-169`
- `codex-rs/core-skills/src/manager.rs:27-197`
- `codex-rs/core-skills/src/manager.rs:246-299`
- `codex-rs/core-skills/src/loader.rs:105-219`
- `codex-rs/core-skills/src/loader.rs:221-380`
- `codex-rs/core-skills/src/config_rules.rs:30-128`
- `codex-rs/core-skills/src/injection.rs:1-511`
- `codex-rs/core-skills/src/render.rs:21-1511`
- `codex-rs/core-skills/src/invocation_utils.rs:8-124`

## Plugins And Marketplaces

Plugins have stable `<plugin>@<marketplace>` ids, plugin roots, data roots,
optional skills, hooks, MCP servers, and app connector metadata. `codex-core`
plugin management loads installed marketplaces/plugins and turns them into
runtime extension inputs. App-server APIs expose marketplace add/remove/upgrade,
plugin list/read, plugin install/uninstall, and plugin MCP OAuth paths.

Source references:

- `codex-rs/plugin/src/lib.rs:1-66`
- `codex-rs/plugin/src/plugin_id.rs:1-64`
- `codex-rs/plugin/src/load_outcome.rs:12-168`
- `codex-rs/core-plugins/src/lib.rs:1-7`
- `codex-rs/core-plugins/src/loader.rs:110-151`
- `codex-rs/core-plugins/src/loader.rs:496-609`
- `codex-rs/core-plugins/src/loader.rs:648-814`
- `codex-rs/core-plugins/src/manifest.rs:12-56`
- `codex-rs/core-plugins/src/manifest.rs:137-286`
- `codex-rs/core-plugins/src/manifest.rs:392-430`
- `codex-rs/core-plugins/src/toggles.rs:4-88`
- `codex-rs/core/src/plugins/manager.rs:1-1916`
- `codex-rs/app-server/src/codex_message_processor/plugins.rs:1-823`
- `codex-rs/app-server/src/codex_message_processor/plugin_mcp_oauth.rs:1-95`

## Apps And Connectors

Apps are exposed through connector metadata and may surface MCP-backed tools.
The app-server owns `app/list` and app list notifications. The MCP layer owns
model-visible tool qualification and Codex Apps tool filtering/cache behavior.

Source references:

- `codex-rs/connectors/src/lib.rs:1-616`
- `codex-rs/core/src/connectors.rs:104-140`
- `codex-rs/core/src/connectors.rs:192-354`
- `codex-rs/core/src/connectors.rs:514-723`
- `codex-rs/core/src/apps/mod.rs:1-2`
- `codex-rs/core/src/context/apps_instructions.rs:11-29`
- `codex-rs/core/src/tools/handlers/tool_suggest.rs:35-168`
- `codex-rs/app-server/src/codex_message_processor/apps_list_helpers.rs:1-66`
- `codex-rs/app-server/src/codex_message_processor/plugin_app_helpers.rs:1-149`
- `codex-rs/app-server-protocol/src/protocol/common.rs:579-614`
- `codex-rs/app-server-protocol/src/protocol/common.rs:1359-1393`
- `codex-rs/codex-mcp/src/rmcp_client.rs:19-37`

## Hooks

Hooks are loaded from config and plugin bundles, then run around tool and
runtime events where core has enough policy context. Do not run hook execution
from schema-only crates.

Source references:

- `codex-rs/hooks/src/lib.rs:1-62`
- `codex-rs/hooks/src/engine/dispatcher.rs:25-138`
- `codex-rs/core/src/hook_runtime.rs:103-279`
- `codex-rs/core/src/hook_runtime.rs:363-510`
- `codex-rs/core/src/tools/registry.rs:357-460`
- `codex-rs/core/src/tools/registry.rs:646-700`
- `codex-rs/core/src/tools/orchestrator.rs:382-420`
- `codex-rs/app-server-protocol/src/protocol/common.rs:1367-1369`
- `codex-rs/core/tests/suite/hooks_mcp.rs:129-220`

## Invariants

- Extension descriptions are model-visible; keep rendering prompt-safe and
  budget-aware.
- Plugin ids and marketplace names are validated path segments.
- Skill mentions can be text, structured input, or linked paths; ambiguity and
  disabled paths must be respected.
- App tool exposure goes through MCP naming/filtering rules.
- App-server config and plugin writes serialize under the global config scope.
- Plugin component paths must start with `./` and remain under the plugin root.
- Plugin hooks require both `Feature::Plugins` and `Feature::PluginHooks`.

## When Changing This

- Update feature gates, config loader behavior, app-server DTOs, and tests when
  an extension becomes client-visible.
- Update skill render/injection tests for any model-visible skill text change.
- Update plugin manager and app-server plugin tests together for marketplace or
  plugin lifecycle changes.
- Recheck MCP naming and tool-search docs if apps expose new tool surfaces.

Read next:

- [MCP Tools And Resources](../tools/mcp.md)
- [Config And Feature Flags](../config/config-and-feature-flags.md)
- [App-Server Protocol](../protocol/app-server.md)
