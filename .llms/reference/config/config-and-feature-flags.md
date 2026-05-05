# Config And Feature Flags

Goblins uses the upstream Codex config loader and feature system. Features are
declared in `codex-rs/features`, resolved into effective runtime config in
`codex-rs/core/src/config`, and exposed to clients through app-server config
APIs.

## Feature Model

Each feature has a stable key, stage, default-enabled value, and optional typed
configuration. Stable/default-on features are treated as normal runtime
capability; under-development/default-off features must be gated at every
surface that exposes behavior.

High-risk feature keys:

- `multi_agent`: stable collaboration tools.
- `multi_agent_v2`: under-development task-path agents.
- `enable_fanout`: under-development CSV agent jobs.
- `goals`: under-development persisted thread goals.
- `plugins`, `plugin_hooks`, `apps`, `enable_mcp_apps`: extension surfaces.
- `guardian_approval`, `remote_control`, `realtime_conversation`: policy and
  transport-sensitive flows.

Source references:

- `codex-rs/config/src/config_toml.rs:68-421`
- `codex-rs/config/src/config_toml.rs:644-783`
- `codex-rs/features/src/lib.rs:71-220`
- `codex-rs/features/src/lib.rs:830-990`
- `codex-rs/features/src/feature_configs.rs:1-48`

## Config Resolution

Config loading merges user config, project config, profiles, CLI/runtime
overrides, managed config, and cloud requirements. Effective config should be
read from `Config`, not reconstructed from raw TOML. Relative paths are resolved
against the config base directory during deserialization.

Managed and cloud requirements can pin or constrain features, permission
profiles, hooks, network policy, and auth behavior. Runtime feature enablement
from the app-server is applied after load and must not override stronger
managed/cloud pins.

Source references:

- `codex-rs/config/src/loader/mod.rs:54-318`
- `codex-rs/config/src/loader/mod.rs:330-602`
- `codex-rs/config/src/state.rs:153-388`
- `codex-rs/core/src/config/mod.rs:218-347`
- `codex-rs/core/src/config/mod.rs:849-1111`
- `codex-rs/core/src/config/mod.rs:1169-1176`
- `codex-rs/core/src/config/mod.rs:1759-1877`
- `codex-rs/core/src/config/mod.rs:1798-1868`
- `codex-rs/core/src/config/mod.rs:2445-2485`
- `codex-rs/core/src/config/managed_features.rs:20-410`
- `codex-rs/core/src/config/config_loader_tests.rs:249-317`
- `codex-rs/core/src/config/config_loader_tests.rs:754-1268`

## App-Server Config APIs

The app-server config boundary provides read/write APIs, config layer metadata,
requirements readback, and runtime feature enablement. Config RPC payloads are
the explicit exception to the usual app-server camelCase rule: they mirror
`config.toml` snake_case keys where needed.

Source references:

- `codex-rs/app-server/src/config_api.rs:113-181`
- `codex-rs/app-server/src/config_api.rs:456-874`
- `codex-rs/app-server/src/config_manager.rs:33-96`
- `codex-rs/app-server/src/config_manager.rs:149-240`
- `codex-rs/app-server/src/config_manager.rs:304-324`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:578-1036`

## Invariants

- Do not enable under-development features only in one layer; check CLI/TUI,
  app-server, core runtime, tool registry, state, tests, and schema fixtures.
- Do not let user config weaken managed or cloud requirements.
- Keep feature dependencies normalized. `enable_fanout` implies collaboration,
  but collaboration does not imply agent jobs.
- `multi_agent_v2` counts the root thread in its concurrent-thread cap and
  conflicts with legacy `agents.max_threads`.
- Treat config read/write APIs as public client surface.
- Project/cwd/repo config layers are disabled when the workspace is untrusted.
- `PermissionProfile` is the source of truth for effective runtime permissions;
  legacy approval/sandbox keys project into it.

## When Changing This

- Add or update the feature spec and typed config shape.
- Update `core/config.schema.json` through the repo's schema workflow.
- Add config loader tests for base/profile/managed/cloud precedence.
- Update app-server config DTOs and tests when the value is client-visible.
- Cross-link the behavior doc for the feature that the flag gates.

Read next:

- [Feature Overview](../features/overview.md)
- [Subagents And Orchestration](../agents/subagents.md)
- [Agent Jobs](../agents/agent-jobs.md)
- [Thread Goals](../features/goal.md)
- [Sandbox, Permissions, And Guardian](../safety/sandbox-permissions-guardian.md)
  for managed network proxy and permission-profile safety constraints.
- `docs/config.md:1-121`
- `codex-rs/core/src/config/schema.md:1-11`
