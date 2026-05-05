# Subagents And Orchestration

Subagent orchestration spans model-facing tool schemas, runtime handlers,
`AgentControl`, `AgentRegistry`, mailbox delivery, app-server replay, and TUI
navigation.

## Layers

1. `codex-rs/tools/src/agent_tool.rs` defines the model-facing tool schemas.
2. `codex-rs/tools/src/tool_registry_plan.rs` chooses legacy v1 or MultiAgentV2
   tool surfaces.
3. `codex-rs/core/src/tools/handlers/multi_agents*` parse arguments, emit
   collaboration events, call `AgentControl`, and serialize tool outputs.
4. `codex-rs/core/src/agent/control.rs` and `registry.rs` own the root-thread
   control plane.
5. App-server protocol maps collaboration events to replay items.
6. The TUI hydrates agent metadata and renders navigation/replay.

Source references:

- `codex-rs/tools/src/agent_tool.rs:29-277`
- `codex-rs/tools/src/tool_registry_plan.rs:402-489`
- `codex-rs/core/src/tools/handlers/multi_agents.rs:1-70`
- `codex-rs/core/src/tools/handlers/multi_agents_v2.rs:1-42`
- `codex-rs/core/src/agent/control.rs:129-142`
- `codex-rs/core/src/agent/registry.rs:16-33`
- `codex-rs/app-server-protocol/src/protocol/thread_history.rs:600-835`
- `codex-rs/tui/src/app/agent_navigation.rs:29-197`

## Legacy V1 Tool Surface

V1 tools are exposed when collaboration is enabled and MultiAgentV2 is disabled.

- `spawn_agent`: optional message/items, optional role, model/reasoning
  overrides, and `fork_context`; returns `agent_id` and nickname.
- `send_input`: targets an agent id, accepts message/items, optional interrupt.
- `resume_agent`: resumes a closed agent by id from rollout.
- `wait_agent`: waits for target ids to reach final status.
- `close_agent`: closes an agent id and live descendants.

Source references:

- `codex-rs/tools/src/agent_tool.rs:29-52`
- `codex-rs/tools/src/agent_tool.rs:90-122`
- `codex-rs/tools/src/agent_tool.rs:182-198`
- `codex-rs/tools/src/agent_tool.rs:200-211`
- `codex-rs/tools/src/agent_tool.rs:245-257`
- `codex-rs/core/src/tools/handlers/multi_agents/spawn.rs:34-180`
- `codex-rs/core/src/tools/handlers/multi_agents/send_input.rs:27-82`
- `codex-rs/core/src/tools/handlers/multi_agents/resume_agent.rs:28-172`
- `codex-rs/core/src/tools/handlers/multi_agents/wait.rs:38-183`
- `codex-rs/core/src/tools/handlers/multi_agents/close_agent.rs:26-91`

## MultiAgentV2 Tool Surface

MultiAgentV2 is task-path based.

- `spawn_agent`: requires `task_name` and message. `fork_turns` replaces
  `fork_context` and accepts `none`, `all`, or a positive integer string.
- `send_message`: queues text to a relative or canonical task name and does not
  trigger a turn.
- `followup_task`: sends text and triggers a turn on a non-root target.
- `wait_agent`: waits on mailbox sequence/pending mail, not final statuses.
- `close_agent`: accepts id or task name, rejects root, closes descendants.
- `list_agents`: lists live agents, optionally filtered by path prefix.
- `resume_agent` is not wired in v2.

Source references:

- `codex-rs/tools/src/agent_tool.rs:54-88`
- `codex-rs/tools/src/agent_tool.rs:124-180`
- `codex-rs/tools/src/agent_tool.rs:212-243`
- `codex-rs/tools/src/agent_tool.rs:224-277`
- `codex-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs:35-260`
- `codex-rs/core/src/tools/handlers/multi_agents_v2/message_tool.rs:10-123`
- `codex-rs/core/src/tools/handlers/multi_agents_v2/wait.rs:30-108`
- `codex-rs/core/src/tools/handlers/multi_agents_v2/close_agent.rs:26-132`
- `codex-rs/core/src/tools/handlers/multi_agents_v2/list_agents.rs:17-37`

## Runtime Data Flow

### Spawn

Handlers parse arguments, build child config from active turn state, apply role
config where allowed, create `SessionSource::SubAgent(ThreadSpawn)`, then call
`AgentControl::spawn_agent_with_metadata`.

`AgentControl` reserves spawn slots, metadata, path/nickname, inherited shell
snapshot/exec policy, creates or forks a thread through `ThreadManagerState`,
registers metadata, emits analytics, notifies app-server listeners, persists
open spawn edges, and sends the initial op.

Source references:

- `codex-rs/core/src/tools/handlers/multi_agents_common.rs:138-162`
- `codex-rs/core/src/tools/handlers/multi_agents_common.rs:198-279`
- `codex-rs/core/src/agent/control.rs:183-337`
- `codex-rs/core/src/agent/control.rs:339-449`
- `codex-rs/core/src/agent/control.rs:1015-1053`
- `codex-rs/core/src/agent/control.rs:1154-1176`

### Messaging And Mailbox

V1 sends `Op::UserInput` directly to the target thread and may interrupt first.
V2 resolves an `AgentPath`, sends `Op::InterAgentCommunication`, enqueues it in
the target session mailbox, and optionally triggers a turn.

The mailbox is an unbounded channel plus monotonic watch sequence. `wait_agent`
v2 subscribes to the sequence, short-circuits if pending mail exists, or waits
until timeout. During sampling, pending mailbox mail can preempt after reasoning
and commentary so the turn follows up promptly.

Source references:

- `codex-rs/core/src/agent/control.rs:632-691`
- `codex-rs/core/src/agent/mailbox.rs:11-71`
- `codex-rs/core/src/session/session.rs:3103-3218`
- `codex-rs/core/src/session/handlers.rs:319-332`
- `codex-rs/core/src/session/handlers.rs:1068-1070`
- `codex-rs/core/src/session/turn.rs:1944-1983`
- `codex-rs/protocol/src/protocol.rs:831-855`

### Resume, Close, List

V1 resume reads persisted spawn edges, resumes the requested thread and open
descendants, re-registers metadata, and reattaches listeners. Close marks spawn
edges closed, shuts down live descendants, removes thread manager entries, and
releases registry entries. List resolves optional path prefix and returns live
agent metadata.

Source references:

- `codex-rs/core/src/agent/control.rs:451-630`
- `codex-rs/core/src/agent/control.rs:712-758`
- `codex-rs/core/src/agent/control.rs:808-827`
- `codex-rs/core/src/agent/control.rs:861-933`
- `codex-rs/core/src/agent/control.rs:1178-1202`

## Feature Gates And Limits

- `multi_agent` / `Feature::Collab`: stable, default-enabled, gates
  collaboration tools.
- `multi_agent_v2` / `Feature::MultiAgentV2`: under development, default
  disabled, switches to task-path tools.
- `enable_fanout` / `Feature::SpawnCsv`: under development, default disabled,
  implies Collab and exposes CSV agent-job tools separately.
- With MultiAgentV2 enabled, `agents.max_threads` is invalid.
- `max_concurrent_threads_per_session` includes the root, so derived
  `agent_max_threads` is one less.
- `min_wait_timeout_ms` must be `1..=3_600_000`.
- `agents.max_depth` must be at least 1.
- V1 explicitly rejects depth overflow; v2 computes child depth but inspected
  paths do not perform the same handler-side explicit rejection.

Source references:

- `codex-rs/features/src/lib.rs:143-148`
- `codex-rs/features/src/lib.rs:472-475`
- `codex-rs/features/src/lib.rs:834-849`
- `codex-rs/features/src/feature_configs.rs:6-27`
- `codex-rs/core/src/config/mod.rs:790-811`
- `codex-rs/core/src/config/mod.rs:1798-1847`
- `codex-rs/core/src/config/mod.rs:2445-2498`
- `codex-rs/core/src/tools/handlers/multi_agents_common.rs:281-286`

## Invariants

- One `AgentControl` instance is shared by all descendants in a root thread tree.
- `AgentRegistry` reservations must be committed or released so spawn slots,
  paths, and nicknames do not leak.
- `AgentPath` task names must be lowercase ASCII letters, digits, or underscores.
- Full-history fork rejects role/model/reasoning overrides.
- Child config must inherit live runtime provider, model, reasoning, approvals,
  sandbox, permission profile, cwd, shell policy, and base instructions.
- Mailbox delivery must not leak into a turn after the answer boundary unless
  current-turn delivery is explicitly accepted.
- TUI/app metadata hydration must be preserved because replay items may only
  contain ids/status maps and need cached or `thread/read` metadata.

Source references:

- `codex-rs/core/src/agent/control.rs:129-142`
- `codex-rs/core/src/agent/registry.rs:79-97`
- `codex-rs/core/src/agent/registry.rs:183-259`
- `codex-rs/core/src/agent/registry.rs:331-339`
- `codex-rs/protocol/src/agent_path.rs:17-72`
- `codex-rs/protocol/src/agent_path.rs:125-180`
- `codex-rs/core/src/tools/handlers/multi_agents_common.rs:241-279`
- `codex-rs/tui/src/app/thread_routing.rs:874-964`
- `codex-rs/tui/src/app/session_lifecycle.rs:520-616`

## Extension Checklist

- Change model schemas in `codex-rs/tools/src/agent_tool.rs`.
- Wire schemas/handlers in `codex-rs/tools/src/tool_registry_plan.rs`.
- Change runtime handlers under `codex-rs/core/src/tools/handlers/multi_agents*`.
- Preserve `AgentControl` and `AgentRegistry` root-tree semantics.
- Update app-server replay and TUI hydration/rendering if events or output shape
  changes.
- Add tests in `codex-rs/core/src/tools/handlers/multi_agents_tests.rs` and
  agent control tests for spawn/resume/close/list/mailbox behavior.

