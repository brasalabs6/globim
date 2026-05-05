# User Docs And Fork Drift

Goblins keeps upstream Codex architecture while changing the public package,
command, prompt identity, README surface, and release branch. User-facing docs
must distinguish fork-owned identity from inherited engine documentation.

## Canonical Fork Surface

The root docs are the canonical source for fork identity. They define that
development work lands on `goblins`, upstream mirroring remains on `main`, the
public package is `@brasalabs/goblins`, and the public command is `goblin`.

Source references:

- `README.md:1-43`
- `SPECs.md:1-50`
- `GOBLINS.md:1-40`

## Package And Runtime Identity

The npm package and Node launcher expose Goblins publicly while preserving the
internal native binary name and payload path as `codex`. This is intentional:
the wrapper changes the user command without forcing a broad Rust binary rename.

Source references:

- `codex-cli/package.json:1-22`
- `package.json:1-8`
- `codex-cli/bin/goblin.js:1-95`
- `codex-cli/bin/goblin.js:117-153`
- `prompts/goblin.md:1-5`
- `prompts/goblin.md:95-102`
- `prompts/goblin.md:335-340`

## Drift Candidates

Some docs still describe upstream Codex behavior or naming. Treat them as engine
references unless they have been explicitly fork-adapted. When editing user docs,
keep `README.md`, `SPECs.md`, and `GOBLINS.md` authoritative, then update
downstream docs to avoid contradicting them.

Source references:

- `docs/authentication.md:1-3`
- `docs/getting-started.md:1-3`
- `docs/contributing.md:5-58`
- `docs/install.md:13-64`
- `docs/open-source-fund.md:1-8`
- `docs/CLA.md:1-49`

## Invariants

- `main` mirrors upstream OpenAI Codex CLI.
- Fork feature, package, prompt, and release work lands on `goblins`.
- Public package name is `@brasalabs/goblins`.
- Public command is `goblin`.
- Internal Rust binary can remain `codex`.
- Prompt and personality files are fork-owned product surface.
- Upstream docs under `docs/` may remain useful, but fork-facing docs must not
  imply users install or invoke the wrong package.

## Change Checklist

- For install docs, verify package name, command name, platform package names,
  and examples.
- For contribution docs, preserve branch policy and release branch expectations.
- For prompt/personality docs, check both prompt source and runtime injection
  behavior.
- For release docs, keep npm package identity, tag naming, and GitHub workflow
  assumptions aligned.
- For inherited docs, label or rewrite upstream-specific language that conflicts
  with the fork contract.

Read next:

- [Fork Packaging And Release](../release/fork-packaging-and-release.md)
- [Personality And Context](../prompts/personality-and-context.md)
- [Repository Map](../architecture/repository-map.md)
