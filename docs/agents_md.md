# AGENTS.md and GLOBIM.md

For information about AGENTS.md, see [this documentation](https://developers.openai.com/codex/guides/agents-md).

Globim automatically loads `GLOBIM.md` files alongside `AGENTS.md`. In each directory, `AGENTS.override.md` is preferred over `AGENTS.md`, configured fallback filenames are used only when no primary AGENTS file exists, and `GLOBIM.md` is appended after the selected primary doc.

## Hierarchical agents message

When the `child_agents_md` feature flag is enabled (via `[features]` in `config.toml`), Codex appends additional guidance about AGENTS.md and GLOBIM.md scope and precedence to the user instructions message and emits that message even when no AGENTS.md or GLOBIM.md is present.
