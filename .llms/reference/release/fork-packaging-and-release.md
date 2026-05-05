# Fork Packaging And Release

Goblins is public as `@brasalabs/goblins` and command `goblin`, while the native
payload remains named `codex`. Release docs must preserve that split because it
is central to keeping upstream architecture mergeable.

## Public Identity

The root package is private workspace tooling. The shipped CLI package is
`codex-cli/package.json`, renamed for the fork. The Node launcher is
`codex-cli/bin/goblin.js`; it resolves platform/native packages and ultimately
executes an internal binary named `codex`.

Source references:

- `README.md:1-39`
- `SPECs.md:3-50`
- `GOBLINS.md:3-40`
- `codex-cli/package.json:1-22`
- `codex-rs/Cargo.toml:111-118`
- `codex-rs/cli/Cargo.toml:1-14`
- `codex-cli/bin/goblin.js:15-98`
- `codex-cli/bin/goblin.js:141-209`

## Native Package Staging

Build scripts stage root and platform npm packages, install native payloads,
and preserve package metadata. Release identity changes must be applied in
`codex-cli`, package staging scripts, and workflow artifacts together.

Source references:

- `codex-cli/scripts/build_npm_package.py:17-67`
- `codex-cli/scripts/build_npm_package.py:115-187`
- `codex-cli/scripts/build_npm_package.py:208-305`
- `codex-cli/scripts/build_npm_package.py:334-418`
- `codex-cli/scripts/install_native_deps.py:21-72`
- `codex-cli/scripts/install_native_deps.py:146-183`
- `codex-cli/scripts/install_native_deps.py:186-251`
- `codex-cli/scripts/install_native_deps.py:254-329`
- `codex-cli/scripts/install_native_deps.py:332-461`
- `scripts/stage_npm_packages.py:16-213`
- `.github/dotslash-config.json:1-81`

## CI And Release Workflows

CI validates the fork on PRs/manual runs and pushes to the `goblins` branch.
Rust release publishes from `rust-v*.*.*` tags and builds/stages native
artifacts before npm publication.

Source references:

- `.github/workflows/ci.yml:1-108`
- `.github/workflows/rust-release.yml:1-40`
- `.github/workflows/rust-release.yml:41-217`
- `.github/workflows/rust-release.yml:219-317`
- `.github/workflows/rust-release.yml:318-421`
- `.github/workflows/rust-release-windows.yml:1-240`
- `.github/actions/linux-code-sign/action.yml:1-49`
- `.github/actions/macos-code-sign/action.yml:1-140`
- `.github/actions/windows-code-sign/action.yml:1-73`

## Invariants

- Public command: `goblin`.
- Reserved future command: `goblins`.
- Internal Rust executable/native payload path: `codex`.
- Release branch contract: fork work happens on `goblins`; upstream mirror work
  belongs to `main`.
- Version/package changes must update npm staging and release workflows, not
  only `package.json`.
- Release tags use `rust-v*.*.*`, must point at `goblins`, and must match Cargo
  plus `codex-cli/package.json` versions.
- `NPM_TOKEN` is mandatory only for npm publication.

## When Changing This

- Update root fork docs, package metadata, launcher, native staging, and release
  workflow checks together.
- Validate package staging before tagging.
- Recheck prompt/personality fallbacks because release packages include the
  compiled prompt assets.
- Keep upstream `codex-rs` docs caveat visible when they still use Codex names.

Read next:

- [Repository Map](../architecture/repository-map.md)
- [Personality And Context](../prompts/personality-and-context.md)
- [Validation And Testing](../tools/validation-and-testing.md)
