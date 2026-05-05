# Agent Jobs

Agent jobs are the Spawn CSV/fanout feature. They expose `spawn_agents_on_csv`
to the main agent and `report_agent_job_result` to worker agents. The feature is
under development, default-off, and gated by `enable_fanout` / `Feature::SpawnCsv`.

## Tool Surface

`spawn_agents_on_csv` reads a CSV, treats the instruction as a row-template,
spawns one worker per row, waits for completion, and exports an output CSV.
`report_agent_job_result` is worker-only and accepts a JSON result payload.

Source references:

- `codex-rs/features/src/lib.rs:143-148`
- `codex-rs/features/src/lib.rs:472-475`
- `codex-rs/features/src/lib.rs:833-850`
- `codex-rs/tools/src/tool_config.rs:85-119`
- `codex-rs/tools/src/tool_config.rs:196-201`
- `codex-rs/tools/src/agent_job_tool.rs:6-95`
- `codex-rs/tools/src/tool_config.rs:149-150`
- `codex-rs/tools/src/tool_registry_plan.rs:492-507`
- `codex-rs/tools/src/tool_registry_plan_tests.rs:389-419`
- `codex-rs/tools/src/tool_registry_plan_tests.rs:529-561`

## Runtime Flow

The core handler branches by tool name. Spawn reads and parses CSV input,
validates headers/rows/id column, creates persisted job/items, transitions the
job to running, applies effective concurrency, spawns worker subagents, waits
for reports or failures, exports a CSV snapshot, and returns progress/failure
summary JSON.

Workers receive item-specific instructions that require exactly one
`report_agent_job_result` call. Missing reports are treated as failures.

Source references:

- `codex-rs/core/src/tools/spec.rs:197-201`
- `codex-rs/core/src/tools/handlers/agent_jobs.rs:39-180`
- `codex-rs/core/src/tools/handlers/agent_jobs.rs:49-215`
- `codex-rs/core/src/tools/handlers/agent_jobs.rs:221-463`
- `codex-rs/core/src/tools/handlers/agent_jobs.rs:470-509`
- `codex-rs/core/src/tools/handlers/agent_jobs.rs:574-823`
- `codex-rs/core/src/tools/handlers/agent_jobs.rs:949-1044`
- `codex-rs/core/src/tools/handlers/agent_jobs.rs:584-808`
- `codex-rs/core/src/tools/handlers/agent_jobs.rs:992-1046`
- `codex-rs/core/src/tools/handlers/agent_jobs.rs:1083-1232`

## Persistence

Agent jobs have job and item models, persisted statuses, timestamps, input CSV
metadata, output CSV path, attempts, result JSON, and error messages. State
runtime APIs create jobs/items, claim pending items, record results/failures,
cancel jobs, and compute progress.

Source references:

- `codex-rs/state/migrations/0014_agent_jobs.sql:1-38`
- `codex-rs/state/migrations/0015_agent_jobs_max_runtime_seconds.sql:1-2`
- `codex-rs/state/src/model/agent_job.rs:7-143`
- `codex-rs/state/src/model/agent_job.rs:161-235`
- `codex-rs/state/src/runtime/agent_jobs.rs:7-127`
- `codex-rs/state/src/runtime/agent_jobs.rs:133-311`
- `codex-rs/state/src/runtime/agent_jobs.rs:332-558`
- `codex-rs/state/src/runtime/agent_jobs.rs:584-679`
- `codex-rs/state/src/runtime/agent_jobs.rs:615-683`

## Invariants

- `enable_fanout` implies collaboration, but normal collaboration does not
  imply agent jobs.
- Child/subagent contexts explicitly disable `Feature::SpawnCsv` in several
  spawn paths so workers do not recursively expose main-agent fanout tools.
- The report tool should be visible only to workers that need it.
- Worker sessions are labeled `agent_job:{job_id}`.
- Only the assigned running worker thread can report its item result
  atomically.
- `result` must be a JSON object.
- Effective concurrency defaults to 16, caps at 64 and `agent_max_threads`.
- `max_runtime_seconds` must be at least 1.
- `stop=true` cancels the job and exports a snapshot.
- CSV parsing must preserve quoted commas and escaping.
- Exported CSV should include original row data plus job/item/status/result
  columns.
- Timeouts, cancellation, and missing worker reports must produce model-readable
  failures.

Source references:

- `codex-rs/features/src/lib.rs:845-849`
- `codex-rs/features/src/tests.rs:270-281`
- `codex-rs/core/src/session/mod.rs:495`
- `codex-rs/core/src/agent/control.rs:538`
- `codex-rs/core/src/tools/handlers/multi_agents_common.rs:281-286`
- `codex-rs/core/src/tools/handlers/agent_jobs_tests.rs:6-60`
- `codex-rs/core/tests/suite/agent_jobs.rs:221-448`

## When Changing This

- Update feature gates, tool schema tests, core handler tests, state tests, and
  subagent config inheritance together.
- Recheck collaboration docs because this is separate from the main v1/v2 agent
  tool surface.
- Recheck permissions if CSV input/output paths move through a different file
  runtime.

Read next:

- [Subagents And Orchestration](subagents.md)
- [Config And Feature Flags](../config/config-and-feature-flags.md)
- [State And Persistence](../architecture/state-persistence.md)
