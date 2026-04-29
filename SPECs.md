# Globim Specifications

## Purpose

Globim is a public community fork of OpenAI Codex CLI. Version 1 of the fork is a prompt, branding, package, and release-pipeline fork that preserves upstream architecture to keep future merges practical.

## Branch And Upstream Contract

- `main` is reserved for upstream `openai/codex` mirroring.
- `globimling` is the default branch and the branch where Globim development and releases happen.
- Globim changes must not be committed directly to `main`.
- Stable Globim releases should start from the latest stable upstream `rust-v*` release unless maintainers explicitly approve a prerelease base.

## Package Contract

- The public npm package name is `globim`.
- The public CLI binary name is `globim`.
- Native optional packages are named `globim-linux-x64`, `globim-linux-arm64`, `globim-darwin-x64`, `globim-darwin-arm64`, `globim-win32-x64`, and `globim-win32-arm64`.
- The Rust executable may remain internally named `codex` when that reduces release and merge risk.

## Prompt Contract

- The default agent personality is Globim: a terminal-dwelling coding agent with compact warmth, playful discipline, and strong practical engineering habits.
- Globim may use "mestre" sparingly as fictional flavor.
- Globim must keep all behavior inside the active instruction hierarchy, repository governance, safety constraints, and validation expectations.

## Release Contract

- Release tags use `rust-v*.*.*`.
- npm publication uses the GitHub Actions secret `NPM_TOKEN`.
- The release pipeline must stage the `globim` npm package plus all native optional packages before publication.
- README and npm metadata must clearly state that Globim is a community fork, not an official OpenAI project.
