# Cloud Tasks And Requirements

Cloud task code is separate from the normal local TUI/thread runtime. It uses a
backend client, ChatGPT auth, environment detection, cloud requirements, and in
some paths remote thread-store behavior.

## Cloud Tasks UI And CLI

`codex cloud` initializes a backend client, verifies ChatGPT/Codex backend auth,
lists tasks, creates tasks, shows status/diff, and applies task output. The TUI
portion has its own app state and rendering separate from the main chat TUI.

Source references:

- `codex-rs/cloud-tasks/src/cli.rs:15-120`
- `codex-rs/cloud-tasks/src/lib.rs:43-104`
- `codex-rs/cloud-tasks/src/lib.rs:43-107`
- `codex-rs/cloud-tasks/src/lib.rs:164-220`
- `codex-rs/cloud-tasks/src/lib.rs:157-273`
- `codex-rs/cloud-tasks/src/lib.rs:297-599`
- `codex-rs/cloud-tasks/src/lib.rs:509-603`
- `codex-rs/cloud-tasks/src/lib.rs:730-978`
- `codex-rs/cloud-tasks/src/app.rs:43-138`
- `codex-rs/cloud-tasks/src/ui.rs:24-268`
- `codex-rs/cloud-tasks/src/ui.rs:788-841`

## Backend Client

`codex-cloud-tasks-client` abstracts backend task operations. Lower-level
backend client code owns HTTP request construction, typed response handling, and
backend API models used by cloud tasks and other remote paths.

Source references:

- `codex-rs/cloud-tasks-client/src/lib.rs:1-19`
- `codex-rs/cloud-tasks-client/src/api.rs:20-170`
- `codex-rs/cloud-tasks-client/src/http.rs:319-380`
- `codex-rs/cloud-tasks-client/src/http.rs:430-561`

## Cloud Requirements

Cloud requirements can constrain config and permissions after local config
loading. They must fail closed when they cannot be loaded and must not be
overwritten by lower-precedence user/system inputs.

Source references:

- `codex-rs/cloud-requirements/src/lib.rs:1-90`
- `codex-rs/cloud-requirements/src/lib.rs:90-195`
- `codex-rs/cloud-requirements/src/lib.rs:196-360`
- `codex-rs/cloud-requirements/src/lib.rs:368-532`
- `codex-rs/cloud-requirements/src/lib.rs:536-750`
- `codex-rs/config/src/loader/mod.rs:58-132`
- `codex-rs/config/src/loader/mod.rs:379-420`
- `codex-rs/config/src/loader/mod.rs:548-602`
- `codex-rs/config/src/config_requirements.rs:22-130`
- `codex-rs/config/src/config_requirements.rs:253-420`
- `codex-rs/config/src/config_requirements.rs:637-810`
- `codex-rs/config/src/config_requirements.rs:918-1165`
- `codex-rs/config/src/requirements_exec_policy.rs:48-188`
- `codex-rs/core/src/config/config_loader_tests.rs:754-1268`
- `codex-rs/app-server/src/config_api.rs:140-150`
- `codex-rs/app-server/src/config_api.rs:257-295`
- `codex-rs/app-server-protocol/src/protocol/common.rs:911-915`
- `codex-rs/app-server-protocol/src/protocol/v2.rs:949-1078`
- `codex-rs/app-server/src/codex_message_processor.rs:8825-8860`
- `codex-rs/app-server/src/config_manager.rs:33-96`
- `codex-rs/app-server/src/config_manager.rs:207-240`

## Remote Thread Store

Remote thread-store support is present but still work in progress. Treat remote
storage as a different implementation of the `ThreadStore` boundary, not as a
drop-in clone of local rollout behavior.

Source references:

- `codex-rs/thread-store/src/remote/mod.rs:27-262`
- `codex-rs/thread-store/src/remote/helpers.rs:1-443`
- `codex-rs/thread-store/src/remote/list_threads.rs:1-280`
- `codex-rs/app-server/tests/suite/v2/remote_thread_store.rs:1-259`
- `.llms/reference/architecture/state-persistence.md:1-84`

## Invariants

- Cloud task auth expects ChatGPT/Codex backend auth, not arbitrary provider
  auth.
- Cloud requirements can pin behavior above local config.
- Remote thread-store unsupported paths must remain explicit and test-covered.
- Cloud task URLs depend on backend base URL normalization.
- Environment resolution accepts exact ids or a unique case-insensitive label.
- `codex cloud exec` prompt text comes from an argument or stdin.
- Best-of-N must be between 1 and 4 and is included in metadata only when
  greater than 1.
- Apply accepts unified git diffs; preflight is non-mutating and CLI apply
  exits nonzero unless apply succeeds.
- Cloud requirements apply only to eligible Business/Enterprise Codex-backend
  auth and fail closed for eligible accounts.
- Requirements precedence fills unset fields cloud/admin/system/legacy; lower
  layers do not override stronger constraints, and `rules` cannot contain
  `allow`.

## When Changing This

- Add tests for auth failure, environment selection, task list/create/apply, and
  requirement precedence.
- Update config docs when cloud requirements affect visible config.
- Update thread-store docs if remote thread-store parity changes.

Read next:

- [Config And Feature Flags](../config/config-and-feature-flags.md)
- [State And Persistence](../architecture/state-persistence.md)
- [Model Providers And Auth](../auth/model-providers-and-auth.md)
