# Globim Fork Contract

## Branches

- `main` belongs to upstream `openai/codex` and must be kept as an upstream mirror.
- Do not land Globim-specific commits on `main`.
- `globimling` is the default branch for this fork and the primary branch for Globim development, CI, releases, and npm publication.
- Upstream updates should be brought in by updating `main` from `upstream/main`, then intentionally rebasing or merging `globimling` against the chosen stable upstream base.

## Stable Base

- Globim releases start from the latest stable upstream `rust-v*` release unless a maintainer explicitly chooses an alpha, beta, or development snapshot.
- The first Globim release is `0.125.1`, based on upstream `rust-v0.125.0` / npm `@openai/codex@0.125.0`.
- Release tags use the upstream-compatible `rust-v*.*.*` format. The first fork tag is `rust-v0.125.1`.

## Public Surface

- The public npm package is `globim`.
- The public CLI command is `globim`.
- Platform packages use unscoped names: `globim-linux-x64`, `globim-linux-arm64`, `globim-darwin-x64`, `globim-darwin-arm64`, `globim-win32-x64`, and `globim-win32-arm64`.
- The internal Rust binary may remain named `codex` to reduce fork drift and keep upstream build wiring simple.

## Prompt Contract

- Globim's default prompt must present Globim as a terminal-dwelling coding agent with playful warmth and practical engineering discipline.
- The "mestre" framing is fictional flavor only. Globim must still obey the active instruction hierarchy, repository rules, safety constraints, and validation requirements.
- Lore must never weaken correctness, security, governance, or validation.

## Release Contract

- GitHub Actions publishes releases from `rust-v*.*.*` tags on `globimling`.
- npm publication uses `NPM_TOKEN` from GitHub Actions secrets.
- If `NPM_TOKEN` is absent, build and release validation may proceed, but npm publication is blocked until the secret is configured.
