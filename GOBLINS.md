# Goblins Fork Contract

## Branches

- `main` belongs to upstream `openai/codex` and must be kept as an upstream mirror.
- Do not land Goblins-specific commits on `main`.
- `goblimling` is the default branch for this fork and the primary branch for Goblins development, CI, releases, and npm publication.
- Upstream updates should be brought in by updating `main` from `upstream/main`, then intentionally rebasing or merging `goblimling` against the chosen stable upstream base.

## Stable Base

- Goblins releases start from the latest stable upstream `rust-v*` release unless a maintainer explicitly chooses an alpha, beta, or development snapshot.
- The first Goblins release is `0.125.1`, based on upstream `rust-v0.125.0` / npm `@openai/codex@0.125.0`.
- Release tags use the upstream-compatible `rust-v*.*.*` format. The first fork tag is `rust-v0.125.1`.

## Public Surface

- The public npm package is `@brasalabs/goblins`.
- The public CLI command is `goblin`.
- The `goblins` command is reserved for a future multi-agent interface.
- Platform packages use scoped names: `@brasalabs/goblins-linux-x64`, `@brasalabs/goblins-win32-x64`, and `@brasalabs/goblins-win32-arm64`.
- The internal Rust binary may remain named `codex` to reduce fork drift and keep upstream build wiring simple.

## Prompt Contract

- The default Goblins prompt must present the agent as a terminal-dwelling Goblin with playful warmth and practical engineering discipline.
- The "mestre" framing is fictional flavor only. The agent must still obey the active instruction hierarchy, repository rules, safety constraints, and validation requirements.
- Lore must never weaken correctness, security, governance, or validation.

## Release Contract

- GitHub Actions publishes releases from `rust-v*.*.*` tags on `goblimling`.
- npm publication uses `NPM_TOKEN` from GitHub Actions secrets.
- If `NPM_TOKEN` is absent, build and release validation may proceed, but npm publication is blocked until the secret is configured.
