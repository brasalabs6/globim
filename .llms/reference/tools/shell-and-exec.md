# Shell And Unified Exec

Goblins has multiple model-facing execution tools, but they converge on the same
approval, sandbox, and retry model.

## Model-Facing Tools

`codex-rs/tools/src/local_tool.rs` defines:

- `exec_command`: command string plus optional workdir, shell, TTY, yield timing,
  output-token cap, login mode, and approval fields.
- `write_stdin`: writes to a live unified exec session.
- `shell`: legacy array-argv shell tool.
- `shell_command`: raw script string using configured shell/login behavior.
- `request_permissions`: asks for permission outside a direct command execution
  path.

Approval fields include `sandbox_permissions`, `justification`, `prefix_rule`,
and conditional additional permissions.

Source references:

- `codex-rs/tools/src/local_tool.rs:19-89`
- `codex-rs/tools/src/local_tool.rs:92-134`
- `codex-rs/tools/src/local_tool.rs:136-197`
- `codex-rs/tools/src/local_tool.rs:199-292`
- `codex-rs/tools/src/local_tool.rs:333-377`

## Legacy Shell Handler

`ShellHandler::run_exec_like` parses function/local-shell payloads, resolves
workdirs against turn cwd, applies granted turn permissions, validates
additional permissions, rejects escalation outside `AskForApproval::OnRequest`,
intercepts apply_patch, emits shell events, asks exec policy for approval, and
invokes the shell runtime through `ToolOrchestrator`.

`shell_command` differs by accepting a script string and deriving shell argv from
configured user shell/login mode.

Source references:

- `codex-rs/core/src/tools/handlers/shell.rs:125-169`
- `codex-rs/core/src/tools/handlers/shell.rs:182-395`
- `codex-rs/core/src/tools/handlers/shell.rs:398-596`

## Unified Exec Handler

Unified exec supports long-lived processes. `exec_command` allocates a process
id, derives argv, clamps yield and token limits, applies permission plumbing,
intercepts apply_patch, and delegates to the process manager. `write_stdin`
writes to an existing process and emits terminal interaction events.

Source references:

- `codex-rs/core/src/tools/handlers/unified_exec.rs:45-80`
- `codex-rs/core/src/tools/handlers/unified_exec.rs:179-417`
- `codex-rs/core/src/tools/handlers/unified_exec.rs:428-460`

## Process Manager

The unified exec process manager:

- stores live sessions.
- collects output until deadlines.
- prunes old sessions.
- requires TTY for non-empty stdin writes.
- watches exits.
- supports local PTY and remote exec-server processes.

Source references:

- `codex-rs/core/src/unified_exec/process_manager.rs:331-595`
- `codex-rs/core/src/unified_exec/process_manager.rs:597-739`
- `codex-rs/core/src/unified_exec/process_manager.rs:808-978`
- `codex-rs/core/src/unified_exec/process_manager.rs:1072-1242`
- `codex-rs/core/src/unified_exec/process.rs:35-74`
- `codex-rs/core/src/unified_exec/process.rs:100-155`
- `codex-rs/core/src/unified_exec/process.rs:197-341`
- `codex-rs/core/src/unified_exec/head_tail_buffer.rs:4-179`

## Approval And Sandbox

Shell runtimes implement `Approvable` and `Sandboxable`. `exec_policy` converts
parsed shell commands and policy state into allow/prompt/forbidden decisions and
validates requested prefix-rule amendments. Sandbox override to
`require_escalated` is rejected unless approval policy is `OnRequest`.

Source references:

- `codex-rs/core/src/tools/runtimes/shell.rs:125-240`
- `codex-rs/core/src/tools/runtimes/unified_exec.rs:115-239`
- `codex-rs/core/src/tools/runtimes/unified_exec.rs:241-364`
- `codex-rs/core/src/exec_policy.rs:132-161`
- `codex-rs/core/src/exec_policy.rs:204-330`
- `codex-rs/core/src/exec_policy.rs:585-725`
- `codex-rs/protocol/src/models.rs:29-62`
- `codex-rs/protocol/src/models.rs:1251-1299`

## Do Not Bypass

Low-level `exec()` does not apply sandboxing itself. Callers must pass
sandbox-wrapped commands. New shell-like execution should route through
`exec_policy`, sandbox transforms, network approval, and shell/unified runtimes.

Source references:

- `codex-rs/core/src/exec.rs:292-417`
- `codex-rs/core/src/exec.rs:903-915`
- `codex-rs/core/src/tools/network_approval.rs:42-83`
- `codex-rs/core/src/tools/network_approval.rs:230-762`

## Test Surfaces

- `codex-rs/core/tests/suite/tool_harness.rs:49-204`
- `codex-rs/core/tests/suite/unified_exec.rs:230-380`
- `codex-rs/core/tests/suite/tool_parallelism.rs:72-180`

