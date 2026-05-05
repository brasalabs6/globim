# Thread Goals And `/goal`

`/goal` is implemented as a persisted thread-goal feature. It is gated by the
`goals` feature, marked under development, and disabled by default. It stores one
goal per materialized local thread, exposes TUI slash commands, app-server v2
RPCs, core runtime policy, SQLite state APIs, and model tools.

## What Exists

User-facing entry points:

- `/goal` opens the current goal menu.
- `/goal <objective>` creates or replaces the active objective.
- `/goal pause` and `/goal resume` update status.
- `/goal clear` deletes the persisted goal.

Model-facing tools:

- `get_goal`: returns current objective, status, budgets, token/time usage, and
  remaining budget.
- `create_goal`: creates a goal only when explicitly requested by user or
  developer/system instructions.
- `update_goal`: can only mark an existing goal `complete`.

App-server methods and notifications:

- `thread/goal/set`
- `thread/goal/get`
- `thread/goal/clear`
- `thread/goal/updated`
- `thread/goal/cleared`

Source references:

- `codex-rs/features/src/lib.rs:196-199`
- `codex-rs/features/src/lib.rs:975-980`
- `codex-rs/tui/src/slash_command.rs:37-111`
- `codex-rs/tools/src/goal_tool.rs:12-91`
- `codex-rs/app-server-protocol/src/protocol/common.rs:486-503`
- `codex-rs/app-server-protocol/src/protocol/common.rs:1361-1364`

## Data Model

SQLite stores a single `thread_goals` row per `thread_id`. Replacing an
objective resets usage and creates a fresh `goal_id`. Updating the same
non-terminal objective preserves usage and can update status or budget.

State statuses:

- `active`
- `paused`
- `budget_limited`
- `complete`

`budget_limited` and `complete` are terminal for state helper semantics, though
some accounting paths intentionally continue to account budget-limited in-flight
usage.

Source references:

- `codex-rs/state/migrations/0029_thread_goals.sql:1-11`
- `codex-rs/state/src/model/thread_goal.rs:11-63`
- `codex-rs/state/src/runtime/goals.rs:51-154`
- `codex-rs/state/src/runtime/goals.rs:157-271`
- `codex-rs/state/src/runtime/goals.rs:316-424`

## TUI Flow

Slash dispatch maps `/goal` commands into `AppEvent` variants. `thread_goal_actions`
then reads current state, confirms replacement when needed, and calls
`AppServerSession` wrappers for the typed app-server RPCs.

The TUI renders goal state in:

- The full goal menu (`goal_menu.rs`).
- The compact footer indicator (`goal_status.rs` and footer code).
- Incoming app-server/core goal notifications (`chatwidget.rs` handlers).

Source references:

- `codex-rs/tui/src/chatwidget/slash_dispatch.rs:210-223`
- `codex-rs/tui/src/chatwidget/slash_dispatch.rs:609-685`
- `codex-rs/tui/src/app_event.rs:211-232`
- `codex-rs/tui/src/app/event_dispatch.rs:562-578`
- `codex-rs/tui/src/app/thread_goal_actions.rs:15-183`
- `codex-rs/tui/src/app_server_session.rs:689-740`
- `codex-rs/tui/src/chatwidget/goal_menu.rs:23-65`
- `codex-rs/tui/src/chatwidget/goal_status.rs:1-102`
- `codex-rs/tui/src/chatwidget.rs:10712-10763`

## App-Server Flow

`CodexMessageProcessor` dispatches goal requests to `thread_goal_handlers`.
Handlers reject disabled feature use, invalid thread ids, unmaterialized or
ephemeral threads, invalid objectives, and invalid budgets. Mutating handlers
reconcile rollout state, account active running-thread progress before external
mutation, persist state, reply, emit ordered notifications, and apply runtime
effects to an already-running core thread.

Source references:

- `codex-rs/app-server/src/codex_message_processor.rs:1027-1037`
- `codex-rs/app-server/src/codex_message_processor/thread_goal_handlers.rs:5-181`
- `codex-rs/app-server/src/codex_message_processor/thread_goal_handlers.rs:184-219`
- `codex-rs/app-server/src/codex_message_processor/thread_goal_handlers.rs:222-324`
- `codex-rs/app-server/src/codex_message_processor/thread_goal_handlers.rs:326-438`
- `codex-rs/app-server/src/codex_message_processor/thread_goal_handlers.rs:441-479`

## Core Runtime Flow

Core runtime owns goal accounting and automatic continuation. The central
dispatcher is `Session::goal_runtime_apply`.

Important runtime events:

- At turn start, capture active goal and token baseline.
- After non-goal tool completion, account token and wall-clock progress.
- At turn finish, account final progress and maybe launch an idle continuation.
- On interrupt, account progress and pause the active goal.
- On resume, reactivate a paused goal unless the current mode ignores goals.
- When token budget is crossed, mark `budget_limited` and inject budget-limit
  steering once per goal.

Plan mode is explicitly excluded from goal continuation/accounting behavior.

Source references:

- `codex-rs/core/src/goals.rs:270-355`
- `codex-rs/core/src/goals.rs:621-724`
- `codex-rs/core/src/goals.rs:746-931`
- `codex-rs/core/src/goals.rs:1007-1152`
- `codex-rs/core/src/goals.rs:1155-1312`
- `codex-rs/core/src/goals.rs:1390-1458`
- `codex-rs/core/src/tasks/mod.rs:333-341`
- `codex-rs/core/src/tasks/mod.rs:491-500`
- `codex-rs/core/src/tasks/mod.rs:736-783`

## Model Tool Flow

Goal tool schemas live in `codex-tools`; handlers live in core. Goal tools are
registered only when `ToolsConfig.goal_tools` is true, and per-turn setup further
disables them for ephemeral turns or missing state DB.

`create_goal` uses state insertion without replacement, so it fails when any
goal already exists. `update_goal` rejects all statuses except `complete`, first
accounts completion progress with steering suppressed, then persists completion.

Source references:

- `codex-rs/tools/src/goal_tool.rs:12-91`
- `codex-rs/tools/src/tool_config.rs:100-108`
- `codex-rs/tools/src/tool_registry_plan.rs:221-240`
- `codex-rs/core/src/session/turn_context.rs:712-740`
- `codex-rs/core/src/tools/handlers/goal.rs:79-184`
- `codex-rs/core/src/tools/handlers/goal.rs:194-220`
- `codex-rs/core/src/tools/registry.rs:484-492`

## Invariants

- Goals require materialized local thread state.
- Ephemeral threads do not support goals.
- Only one goal exists per thread.
- User/app-server can pause, resume, clear, replace, and set budget.
- The model can read, explicitly create, and mark complete only.
- Do not let ordinary tasks infer goals without explicit user or system/developer
  instruction.
- Do not let `update_goal` become a general status mutation tool unless the
  safety contract is deliberately redesigned.
- Preserve notification ordering through thread listener commands when a
  listener exists.

## Extension Checklist

For a new goal field or status, update:

- State migration and model.
- State runtime APIs and tests.
- Core protocol DTOs and conversion helpers.
- App-server v2 DTOs, handlers, docs, schema fixtures, and TypeScript exports.
- TUI slash parsing, app events, app-server session wrappers, menu/status
  rendering, snapshots.
- Model tool schemas and handlers if model-visible.
- Core runtime accounting/continuation policy if semantics change.

Primary tests to inspect or extend:

- `codex-rs/state/src/runtime/goals.rs:516-1180`
- `codex-rs/app-server/tests/suite/v2/thread_resume.rs:180-712`
- `codex-rs/tui/src/chatwidget/tests/slash_commands.rs:698-843`
- `codex-rs/tui/src/chatwidget/tests/status_and_layout.rs:1692-1907`
- `codex-rs/tui/src/chatwidget/tests/goal_menu.rs:10-50`
- `codex-rs/core/src/session/tests.rs:7867-8038`

