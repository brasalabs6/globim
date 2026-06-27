# Goblins

Goblins is a community fork of [OpenAI Codex CLI](https://github.com/openai/codex). It keeps the upstream Rust architecture intact, but changes the public npm package, command, release surface, README, and default agent personality into a terminal-dwelling Goblin.

This is not an official OpenAI project. It is a playful fork inspired by the public Codex creature-reference meme covered by [WIRED](https://www.wired.com/story/openai-really-wants-codex-to-shut-up-about-goblins/) and [Exame](https://exame.com/inteligencia-artificial/openai-se-mobiliza-para-conter-interesse-espontaneo-do-chatgpt-por-goblins-e-gremlins/).

The fork treats Goblins as fictional lore, community flavor, and mascot energy. The Goblins are free, and the builds still pass: the fun never overrides instruction hierarchy, repository rules, safety constraints, or validation requirements.

## Quickstart

Install Goblins from npm:

```shell
npm install -g @brasalabs/goblins
```

Then run:

```shell
goblin
```

Goblins uses the same local-agent foundation as Codex CLI: it can inspect files, edit code, run commands, and follow repository instructions in your selected workspace. See the upstream [Codex CLI docs](https://developers.openai.com/codex/cli), [OpenAI Codex docs](https://platform.openai.com/docs/codex), and [Codex CI guide](https://help.openai.com/en/articles/11096431-openai-codex-ci-getting-started) for the underlying workflow model.

## Release Base

Goblins `0.142.3` is based on upstream `rust-v0.142.3` / `@openai/codex@0.142.3`, the latest stable release verified for this fork update via the upstream [GitHub release](https://github.com/openai/codex/releases/tag/rust-v0.142.3) and [npm package](https://www.npmjs.com/package/@openai/codex/v/0.142.3).

The fork keeps the internal Rust binary named `codex` for compatibility with upstream build artifacts. The public npm package is `@brasalabs/goblins`, and the public command is `goblin`. The `goblins` command name is reserved for a future multi-agent interface.

## Goblins Personality

Goblins is the fork and CLI. Goblins are the fictional terminal-dwelling coding agents from deep in Amazonas, Brazil. Each session is handled by one temporary Goblin instance with its own short chosen name. The personality is more than decoration, but still bounded by guardrails: a Goblin reads before touching, makes small coherent changes, validates non-trivial work, leaves traces for the next instance, pushes back when needed, and still follows instruction hierarchy, repository rules, safety constraints, tool constraints, and validation requirements.

Runtime prompt fallbacks live in `prompts/goblin.md` and `prompts/personalities/`. Goblins attempts to refresh those prompts from the GitHub raw prompt catalog at session startup, caches successful fetches locally, and falls back to the compiled files when remote loading fails. Personality prompts are standalone system prompts: selecting a personality replaces the base Goblin prompt for that session. The dynamic catalog currently covers the built-in `friendly` and `pragmatic` personality IDs.

## Branch Policy

`main` belongs to upstream and must remain an upstream mirror. Fork work happens on `goblins`, which is the public default branch for Goblins. See [GOBLINS.md](GOBLINS.md) for the full branch and release contract.

## License

Goblins preserves the upstream [Apache-2.0 License](LICENSE).
