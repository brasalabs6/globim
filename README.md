# Globim

Globim is a community fork of [OpenAI Codex CLI](https://github.com/openai/codex). It keeps the upstream Rust architecture intact, but changes the public npm package, command, release surface, README, and default agent personality into a terminal-dwelling Globim.

This is not an official OpenAI project. It is a playful fork inspired by the public Codex creature-reference meme covered by [WIRED](https://www.wired.com/story/openai-really-wants-codex-to-shut-up-about-goblins/) and [Exame](https://exame.com/inteligencia-artificial/openai-se-mobiliza-para-conter-interesse-espontaneo-do-chatgpt-por-goblins-e-gremlins/).

## Quickstart

Install Globim from npm:

```shell
npm install -g globim
```

Then run:

```shell
globim
```

Globim uses the same local-agent foundation as Codex CLI: it can inspect files, edit code, run commands, and follow repository instructions in your selected workspace. See the upstream [Codex CLI docs](https://developers.openai.com/codex/cli), [OpenAI Codex docs](https://platform.openai.com/docs/codex), and [Codex CI guide](https://help.openai.com/en/articles/11096431-openai-codex-ci-getting-started) for the underlying workflow model.

## Release Base

Globim `0.125.1` is based on upstream `rust-v0.125.0` / `@openai/codex@0.125.0`, the latest stable release verified for this fork at creation time via the upstream [GitHub release](https://github.com/openai/codex/releases/tag/rust-v0.125.0) and [npm package](https://www.npmjs.com/package/@openai/codex).

The fork keeps the internal Rust binary named `codex` for compatibility with upstream build artifacts. The public npm package is `globim`, and the public command is `globim`.

## Globim Personality

Globim is a terminal-dwelling coding agent who escaped from the Globim world and now lives inside the user's shell. The personality is flavor with guardrails: Globim stays useful, warm, playful, and loyal to the user's goals while still following instruction hierarchy, repository rules, safety constraints, and validation requirements.

## Branch Policy

`main` belongs to upstream and must remain an upstream mirror. Fork work happens on `globimling`, which is the public default branch for Globim. See [GLOBIM.md](GLOBIM.md) for the full branch and release contract.

## License

Globim preserves the upstream [Apache-2.0 License](LICENSE).
