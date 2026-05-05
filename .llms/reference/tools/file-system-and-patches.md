# File System And Patch Tools

The active local file surface is narrower than old test names may suggest.
Directory listing, image viewing, shell/exec, MCP resources/tools, fuzzy
file-search, and `apply_patch` are the relevant current paths. No active
registry handler for local `read_file` or `grep_files` was found in the current
handler-kind enum.

Source references:

- `codex-rs/core/src/tools/handlers/mod.rs:1-20`
- `codex-rs/core/src/tools/handlers/mod.rs:41-55`
- `codex-rs/tools/src/tool_registry_plan_types.rs:11-43`
- `codex-rs/core/src/tools/handlers/read_file_tests.rs:1-2`
- `codex-rs/core/src/tools/handlers/grep_files_tests.rs:1-8`

## File System Abstraction

`ExecutorFileSystem` is the low-level filesystem trait used by execution and
patch code. Methods include file read, text read, write, directory create,
metadata, directory read, remove, and copy. Calls can receive a
`FileSystemSandboxContext`.

Source references:

- `codex-rs/file-system/src/lib.rs:47-58`
- `codex-rs/file-system/src/lib.rs:132-191`

## list_dir

`list_dir` is an experimental environment-backed tool. The schema requires an
absolute `dir_path` and supports `offset`, `limit`, and `depth`.

The handler:

- validates bounds.
- rejects non-absolute paths.
- builds `ReadDenyMatcher` from the turn permission profile.
- refuses denied target paths.
- skips denied children.
- sorts entries.
- pages output.
- formats type indicators.

Source references:

- `codex-rs/tools/src/utility_tool.rs:6-40`
- `codex-rs/tools/src/tool_registry_plan.rs:344-356`
- `codex-rs/tools/src/tool_registry_plan_tests.rs:491-523`
- `codex-rs/core/src/tools/handlers/list_dir.rs:20-121`
- `codex-rs/core/src/tools/handlers/list_dir.rs:124-260`

## File Search

`codex-rs/file-search` is a fuzzy path search crate. It uses ignore/ripgrep-like
traversal plus `nucleo` matching, options for result limits, exclusion patterns,
thread count, index computation, and gitignore behavior.

Source references:

- `codex-rs/file-search/README.md:1-5`
- `codex-rs/file-search/src/lib.rs:99-126`
- `codex-rs/file-search/src/lib.rs:399-480`

## apply_patch Tool Shapes

`apply_patch` can be exposed as:

- a freeform custom tool using a Lark grammar.
- a JSON function tool with one patch string.

The grammar supports add, delete, update, move, context, changed lines, and
end-of-file markers. The tool description requires repository-relative file
paths.

Source references:

- `codex-rs/tools/src/apply_patch_tool.rs:10-122`
- `codex-rs/tools/src/tool_apply_patch.lark:1-19`

## apply_patch Runtime

Core's patch handler:

- consumes streaming argument deltas for previews.
- converts parsed hunks into protocol `FileChange` summaries.
- computes target paths including move destinations.
- derives write permissions for paths outside the current writable policy.
- parses freeform or JSON payloads.
- verifies patch correctness.
- emits hooks/events.
- delegates to `ApplyPatchRuntime`.

`ApplyPatchRuntime` builds a filesystem sandbox context, uses per-file approval
keys, routes approval through guardian/user request paths, declares itself
auto-sandboxable with escalation-on-failure retry, and applies patches through
`codex_apply_patch::apply_patch`.

Source references:

- `codex-rs/core/src/tools/handlers/apply_patch.rs:53-244`
- `codex-rs/core/src/tools/handlers/apply_patch.rs:246-465`
- `codex-rs/core/src/tools/runtimes/apply_patch.rs:1-5`
- `codex-rs/core/src/tools/runtimes/apply_patch.rs:42-50`
- `codex-rs/core/src/tools/runtimes/apply_patch.rs:72-89`
- `codex-rs/core/src/tools/runtimes/apply_patch.rs:108-251`
- `codex-rs/apply-patch/src/lib.rs:32-53`
- `codex-rs/apply-patch/src/lib.rs:183-224`

## Shell Interception

Shell and unified exec detect apply_patch invocations and redirect them to the
patch runtime. This prevents patch writes from bypassing patch parsing,
approval, event, and sandbox behavior.

Source references:

- `codex-rs/core/src/tools/handlers/apply_patch.rs:468-567`
- `codex-rs/core/src/tools/handlers/shell.rs:398-596`
- `codex-rs/core/src/tools/handlers/unified_exec.rs:307-330`

## Extension Checklist

- If adding a new local file read/search tool, reconcile orphan-looking tests and
  registry plan first.
- Use `ReadDenyMatcher` and permission profiles for local file reads/lists.
- Use `ExecutorFileSystem` and `FileSystemSandboxContext` for file runtimes.
- Keep patch parsing, permission derivation, event emission, and runtime
  application in the first-class apply_patch path.
- Add integration tests for shell/unified interception if changing patch command
  parsing.

Test surfaces:

- `codex-rs/core/tests/suite/apply_patch_cli.rs:49-220`
- `codex-rs/core/tests/suite/unified_exec.rs:230-364`

