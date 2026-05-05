# Turn Context, Compaction, Realtime, And Review

Turn execution is more than a single model call. Core prepares turn context,
manages history and compaction, handles realtime conversation paths, and runs
review-mode flows through separate templates and task code.

## Turn Context

Turn context binds model info, prompt/instructions, tool configuration, state
DB, rollout/thread metadata, permission profile, and current user input. Tool
registration, goal tools, MCP resources, and environment-backed tools are
decided per turn.

Source references:

- `codex-rs/protocol/src/protocol.rs:400-592`
- `codex-rs/protocol/src/protocol.rs:1318-1344`
- `codex-rs/protocol/src/protocol.rs:2022-2235`
- `codex-rs/protocol/src/protocol.rs:2807-2884`
- `codex-rs/core/src/state/session.rs:19-124`
- `codex-rs/core/src/session/handlers.rs:123-230`
- `codex-rs/core/src/session/handlers.rs:968-1158`
- `codex-rs/core/src/session/turn_context.rs:1-793`
- `codex-rs/core/src/session/turn_context.rs:48-127`
- `codex-rs/core/src/session/turn_context.rs:327-348`
- `codex-rs/core/src/session/turn_context.rs:741-745`
- `codex-rs/core/src/session/turn.rs:120-358`
- `codex-rs/core/src/session/turn.rs:1944-1983`
- `codex-rs/core/src/tasks/mod.rs:333-341`
- `codex-rs/core/src/tasks/mod.rs:491-500`

## Compaction

Context compaction is driven by context-manager history code and compact
templates. Tests cover manual/automatic compaction, history normalization, and
preserving enough context for follow-up turns.

Source references:

- `codex-rs/core/src/context_manager/history.rs:1-726`
- `codex-rs/core/src/context_manager/normalize.rs:1-345`
- `codex-rs/core/src/compact.rs:42-116`
- `codex-rs/core/src/compact.rs:151-286`
- `codex-rs/core/src/compact.rs:305-355`
- `codex-rs/core/src/compact.rs:402-520`
- `codex-rs/core/src/compact_remote.rs:35-77`
- `codex-rs/core/src/compact_remote.rs:113-260`
- `codex-rs/protocol/src/items.rs:25-37`
- `codex-rs/protocol/src/items.rs:130-150`
- `codex-rs/protocol/src/items.rs:384-408`
- `codex-rs/core/templates/compact/prompt.md:1-9`
- `codex-rs/core/templates/compact/summary_prefix.md:1-1`
- `codex-rs/core/tests/suite/compact.rs:1-3368`

## Realtime

Realtime conversation has backend prompt templates, a core realtime
conversation runtime, and WebRTC support. It also includes fanout task handling
inside realtime state that is unrelated to Spawn CSV agent jobs.

Source references:

- `codex-rs/core/templates/realtime/backend_prompt.md:1-151`
- `codex-rs/core/src/realtime_context.rs:58-123`
- `codex-rs/core/src/realtime_context.rs:282-305`
- `codex-rs/core/src/realtime_context.rs:452-470`
- `codex-rs/core/src/realtime_conversation.rs:590-856`
- `codex-rs/core/src/realtime_conversation.rs:1419-1438`
- `codex-rs/core/src/realtime_conversation.rs:1-1445`
- `codex-rs/realtime-webrtc/src/lib.rs:1-90`
- `codex-rs/realtime-webrtc/src/native.rs:1-227`
- `codex-rs/core/tests/suite/realtime_conversation.rs:1-3384`

## Review

Review mode uses dedicated templates and task code. Keep review-specific model
steering separate from normal turn and goal continuation behavior.

Source references:

- `codex-rs/protocol/src/protocol.rs:2977-3062`
- `codex-rs/core/templates/review/history_message_completed.md:1-8`
- `codex-rs/core/templates/review/history_message_interrupted.md:1-8`
- `codex-rs/core/src/session/review.rs:5-178`
- `codex-rs/core/src/tasks/review.rs:56-180`
- `codex-rs/core/src/tasks/review.rs:213-280`
- `codex-rs/core/tests/suite/review.rs:1-947`

## Memories

Memory read/write crates provide prompt helpers, citation behavior, usage
tracking, workspace selection, storage, and phased write runtime. Memory changes
can affect prompt context and token budget.

Source references:

- `codex-rs/memories/read/src/lib.rs:1-20`
- `codex-rs/memories/read/src/prompts.rs:1-56`
- `codex-rs/memories/write/src/lib.rs:1-136`
- `codex-rs/memories/write/src/runtime.rs:1-295`
- `codex-rs/memories/write/src/phase1.rs:1-793`
- `codex-rs/memories/write/src/phase2.rs:1-569`

## Invariants

- Turn context is per-turn; do not assume static tool exposure across modes.
- Every real turn should persist a durable `TurnContextItem`.
- Manual and pre-turn compaction clear reference context so the next turn
  reinjects.
- Mid-turn compaction reinjects initial context before the last real user or
  summary item.
- Remote compaction drops stale developer/non-content user wrappers before
  installing replacement history.
- Plan mode and goal continuation have different runtime behavior.
- Realtime fanout is not the same feature as Spawn CSV.
- Review steering should stay review-specific.
- Memory changes can affect prompt size and compaction behavior.

## When Changing This

- Add core integration tests for turn/context/compaction changes.
- Update prompt templates and snapshots/tests together.
- Recheck goals, tool registry, MCP, and TUI docs when turn context changes
  model-visible behavior.

Read next:

- [Personality And Context](../prompts/personality-and-context.md)
- [Thread Goals](../features/goal.md)
- [Tool Registry](../tools/tool-registry.md)
