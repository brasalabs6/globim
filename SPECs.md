# Goblins Specifications

## Purpose

Goblins is a public community fork of OpenAI Codex CLI. Version 1 of the fork is a prompt, branding, package, and release-pipeline fork that preserves upstream architecture to keep future merges practical.

## Branch And Upstream Contract

- `main` is reserved for upstream `openai/codex` mirroring.
- `globimling` is the default branch and the branch where Goblins development and releases happen.
- Goblins changes must not be committed directly to `main`.
- Stable Goblins releases should start from the latest stable upstream `rust-v*` release unless maintainers explicitly approve a prerelease base.

## Package Contract

- The public npm package name is `@brasalabs/goblins`.
- The public CLI binary name is `goblin`.
- The `goblins` CLI binary name is reserved for a future multi-agent interface.
- Native optional packages are named `@brasalabs/goblins-linux-x64`, `@brasalabs/goblins-linux-arm64`, `@brasalabs/goblins-win32-x64`, and `@brasalabs/goblins-win32-arm64`.
- The Rust executable may remain internally named `codex` when that reduces release and merge risk.

## Prompt Contract

- The default agent personality is a Goblin: a terminal-dwelling coding agent with compact warmth, playful discipline, and strong practical engineering habits.
- The agent may use "mestre" sparingly as fictional flavor.
- The agent must keep all behavior inside the active instruction hierarchy, repository governance, safety constraints, and validation expectations.

## Release Contract

- Release tags use `rust-v*.*.*`.
- npm publication uses the GitHub Actions secret `NPM_TOKEN`.
- The release pipeline must stage the `@brasalabs/goblins` npm package plus all native optional packages before publication.
- README and npm metadata must clearly state that Goblins is a community fork, not an official OpenAI project.
