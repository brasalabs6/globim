# Urgent Next Actions

- Timestamp UTC: `2026-05-13T04:18:12Z`
- Repository: `/home/guilherme/brainstorm/goblins`

1. Preserve current worktree state; do not revert unrelated changes.
2. Read README and primary manifests to identify exact purpose and commands.
3. Run safe static checks only after confirming they do not mutate unrelated files.
4. Inspect tests/CI/deploy paths and record blockers in this directory.
5. If another agent already added timestamped notes, merge findings conceptually rather than overwriting files.

## Immediate Owner Notes

- Use `git status --short` before editing.
- Keep any additional urgent notes in `.llms/urgent/<agent>-<timestamp>.md`.
- Do not paste secrets, tokens, private key material, cookies, or full credential file contents into documentation.
