# Sandbox, Permissions, And Guardian

Safety decisions are distributed across permission profiles, sandbox policy,
tool orchestration, shell escalation, MCP approval, and Guardian review. Keep
schema-only crates out of policy decisions; runtime gates belong in core or
platform sandbox crates.

## Permission And Sandbox Policy

Effective permissions come from config requirements and runtime updates, then
project into filesystem and network sandbox policies. Tool runtimes use those
profiles to decide whether a call can run directly, needs approval, should run
inside a sandbox, or can retry without sandbox after a denial.

Source references:

- `codex-rs/protocol/src/protocol.rs:941-1011`
- `codex-rs/protocol/src/protocol.rs:1013-1138`
- `codex-rs/protocol/src/models.rs:29-62`
- `codex-rs/protocol/src/models.rs:300-620`
- `codex-rs/protocol/src/permissions.rs:22-130`
- `codex-rs/protocol/src/permissions.rs:221-353`
- `codex-rs/protocol/src/request_permissions.rs:10-80`
- `codex-rs/core/src/config/mod.rs:227-347`
- `codex-rs/core/src/config/permissions.rs:1-230`
- `codex-rs/core/src/session/mod.rs:1966-2300`
- `codex-rs/core/src/tools/orchestrator.rs:126-376`
- `codex-rs/core/src/tools/orchestrator.rs:126-470`
- `codex-rs/core/src/tools/sandboxing.rs:40-117`
- `codex-rs/core/src/tools/sandboxing.rs:40-390`
- `codex-rs/core/src/tools/sandboxing.rs:279-417`
- `codex-rs/core/src/tools/handlers/mod.rs:89-225`
- `codex-rs/sandboxing/src/lib.rs:1-48`

## Managed Network Proxy

`codex-network-proxy` is the managed network policy enforcement layer. It can
run HTTP and SOCKS5 listeners, enforce allow/deny domain policy, apply limited
read-only mode, guard local/private destinations, optionally terminate CONNECT
with MITM support, proxy selected macOS Unix sockets, and emit OTEL audit events
for policy decisions.

Core loads network proxy config from merged config layers plus exec-policy
network rules. Constraints from non-user-controlled layers are enforced before
building runtime proxy state, so user/project/session inputs cannot weaken
trusted network restrictions.

Source references:

- `codex-rs/network-proxy/README.md:3-16`
- `codex-rs/network-proxy/README.md:21-63`
- `codex-rs/network-proxy/README.md:88-103`
- `codex-rs/network-proxy/README.md:136-144`
- `codex-rs/network-proxy/README.md:146-183`
- `codex-rs/network-proxy/README.md:192-219`
- `codex-rs/core/src/network_proxy_loader.rs:34-80`
- `codex-rs/core/src/network_proxy_loader.rs:105-172`
- `codex-rs/core/src/network_proxy_loader.rs:188-221`

## Platform Sandboxes

Linux, Windows, process hardening, exec policy, and shell escalation are
separate crates. Do not flatten them into a single "sandbox" concept; each has
different platform constraints and failure modes.

Source references:

- `codex-rs/linux-sandbox/src/lib.rs:1-27`
- `codex-rs/windows-sandbox-rs/src/lib.rs:1-726`
- `codex-rs/process-hardening/src/lib.rs:1-200`
- `codex-rs/execpolicy/src/lib.rs:1-30`
- `codex-rs/shell-escalation/src/lib.rs:1-35`
- `docs/sandbox.md:1-3`
- `docs/execpolicy.md:1-3`
- `codex-rs/execpolicy/README.md:1-98`
- `codex-rs/shell-escalation/README.md:1-29`

## Guardian Review

Guardian approval wraps higher-risk actions with structured approval requests,
prompts, review sessions, timeouts, and rejection messages. MCP approval can
also route through Guardian based on policy.

Source references:

- `codex-rs/core/src/guardian/mod.rs:1-161`
- `codex-rs/core/src/guardian/mod.rs:1-121`
- `codex-rs/core/src/guardian/approval_request.rs:1-541`
- `codex-rs/core/src/guardian/prompt.rs:1-647`
- `codex-rs/core/src/guardian/prompt.rs:81-190`
- `codex-rs/core/src/guardian/prompt.rs:529-610`
- `codex-rs/core/src/guardian/review.rs:1-772`
- `codex-rs/core/src/guardian/review.rs:141-149`
- `codex-rs/core/src/guardian/review.rs:230-610`
- `codex-rs/core/src/guardian/review_session.rs:1-1568`
- `codex-rs/core/src/mcp_tool_call.rs:899-1203`

## Invariants

- Mutating tools must declare mutability or use a runtime path that gates
  approvals and sandboxing.
- Approval cache keys must include the operation-specific risk surface.
- Guardian and permission hooks must see enough metadata to make a decision.
- Windows filesystem-deny behavior is not equivalent to Linux sandbox behavior.
- Do not weaken managed/cloud permission constraints through runtime updates.
- `require_escalated` bypasses sandbox and managed network; additional
  permissions stay sandboxed and must be non-empty, normalized, and intersected
  with what was requested.
- Network proxy deny rules win over allow rules and over exec-policy decider
  overrides.
- Network proxy listeners clamp unsafe non-loopback binds unless an explicit
  dangerous opt-in is set; Unix-socket proxying also forces loopback listeners.
- Limited network mode only permits read-oriented HTTP methods and cannot inspect
  HTTPS methods through CONNECT unless MITM is enabled.
- Network proxy audit events must not log full URL, path, or query data.
- Session grants cannot set `strict_auto_review`.
- Guardian routes only for `OnRequest`/`Granular` paths with auto-review.

## When Changing This

- Add tests at the runtime layer where approval/sandbox decisions are made.
- Recheck MCP, shell/unified exec, apply_patch, and app-server approval flows.
- Update docs when a platform sandbox has different semantics from another.
- Run narrow tool tests before broader core integration tests.

Read next:

- [Shell And Unified Exec](../tools/shell-and-exec.md)
- [File System And Patch Tools](../tools/file-system-and-patches.md)
- [MCP Tools And Resources](../tools/mcp.md)
- [Secrets And Proxy Boundaries](secrets-and-proxy-boundaries.md)
- [Config And Feature Flags](../config/config-and-feature-flags.md)
- [Analytics, Feedback, And OTEL](../observability/analytics-feedback-otel.md)
- [Testing Matrix](../testing/test-matrix.md)
