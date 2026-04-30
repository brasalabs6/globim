# Goblins

Goblins is a community fork of [OpenAI Codex CLI](https://github.com/openai/codex). It keeps the upstream Rust architecture intact, but changes the public npm package, command, release surface, README, and default agent personality into a terminal-dwelling Goblin.

This is not an official OpenAI project. It is a playful fork inspired by the public Codex creature-reference meme covered by [WIRED](https://www.wired.com/story/openai-really-wants-codex-to-shut-up-about-goblins/) and [Exame](https://exame.com/inteligencia-artificial/openai-se-mobiliza-para-conter-interesse-espontaneo-do-chatgpt-por-goblins-e-gremlins/).

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

Goblins `0.125.1` is based on upstream `rust-v0.125.0` / `@openai/codex@0.125.0`, the latest stable release verified for this fork at creation time via the upstream [GitHub release](https://github.com/openai/codex/releases/tag/rust-v0.125.0) and [npm package](https://www.npmjs.com/package/@openai/codex).

The fork keeps the internal Rust binary named `codex` for compatibility with upstream build artifacts. The public npm package is `@brasalabs/goblins`, and the public command is `goblin`. The `goblins` command name is reserved for a future multi-agent interface.

## Goblins Personality

The default Goblins agent is a terminal-dwelling Goblin who escaped from the Goblins world and now lives inside the user's shell. The personality is flavor with guardrails: the agent stays useful, warm, playful, and loyal to the user's goals while still following instruction hierarchy, repository rules, safety constraints, and validation requirements.

## Branch Policy

`main` belongs to upstream and must remain an upstream mirror. Fork work happens on `globimling`, which is the public default branch for Goblins. See [GOBLINS.md](GOBLINS.md) for the full branch and release contract.

## License

Goblins preserves the upstream [Apache-2.0 License](LICENSE).
