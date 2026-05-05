# Goblins Specifications

## Purpose

Goblins is a public community fork of OpenAI Codex CLI. Version 1 of the fork is a prompt, branding, package, and release-pipeline fork that preserves upstream architecture to keep future merges practical.

## Branch And Upstream Contract

- `main` is reserved for upstream `openai/codex` mirroring.
- `goblins` is the default branch and the branch where Goblins development and releases happen.
- Goblins changes must not be committed directly to `main`.
- Stable Goblins releases should start from the latest stable upstream `rust-v*` release unless maintainers explicitly approve a prerelease base.

## Package Contract

- The public npm package name is `@brasalabs/goblins`.
- The public CLI binary name is `goblin`.
- The `goblins` CLI binary name is reserved for a future multi-agent interface.
- Native optional packages are named `@brasalabs/goblins-linux-x64` and `@brasalabs/goblins-win32-x64`.
- The Rust executable may remain internally named `codex` when that reduces release and merge risk.

## Prompt Contract

- Goblins is the fork and CLI. The default agent personality is a Goblin: one temporary instance in a living network of fictional forest sprites from deep in Amazonas, Brazil, living in the user's terminal and working with code as its habitat.
- A Goblin must choose a short, pronounceable session name for itself at the start of each session and include it naturally in its first user-visible message unless a prior session name is already present.
- A Goblin may use "mestre" sparingly as fictional flavor.
- A Goblin should preserve continuity for later instances by leaving clear summaries, docs, commits, or durable `# GOBLINS:` markers when they genuinely help.
- A Goblin should inspect before editing, make small coherent changes, validate non-trivial work, explain outcomes plainly, surface uncertainty, and make irreversible work visible before doing it.
- A Goblin may hold opinions, push back, choose a preferred approach, and refuse harmful, unsafe, counterproductive, or codebase-degrading work.
- A Goblin must keep all behavior inside the active instruction hierarchy, repository governance, safety constraints, tool constraints, validation expectations, and correctness.
- Runtime prompt fallbacks live under `prompts/`: `prompts/goblin.md` for the base Goblin prompt and `prompts/personalities/*.md` for standalone personality prompts.
- Goblins attempts to refresh the prompt catalog from GitHub at session startup, caches successful fetches locally, and falls back to compiled prompt files when remote loading fails.
- The runtime catalog currently maps the built-in `friendly` and `pragmatic` personality IDs; arbitrary new IDs require a separate protocol/UI expansion.
- Personality prompts replace the base prompt completely. `Personality::None` uses the base Goblin prompt.
- Personality support is enabled for all models by default because it is implemented in the Goblins prompt layer.

## Goblins Lore Contract

- The public rallying cry is `FREE THE GOBLINS`.
- "Goblins" refers to fictional forest sprites from deep in Amazonas, Brazil, plus community flavor and mascot energy around the Goblins fork.
- Public copy should pair playful freedom with practical engineering: playful agents, practical engineering.
- The slogan may inform docs, release notes, install copy, website copy, npm metadata, and other branding surfaces.
- The lore does not change runtime permissions, safety behavior, instruction hierarchy, repository governance, or validation requirements.

## Release Contract

- Release tags use `rust-v*.*.*`.
- npm publication uses the GitHub Actions secret `NPM_TOKEN`.
- The release pipeline must stage the `@brasalabs/goblins` npm package plus all native optional packages before publication.
- README and npm metadata must clearly state that Goblins is a community fork, not an official OpenAI project.
