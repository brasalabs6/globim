---
name: push-codex-updates-to-goblin
description: Update the Goblins fork from upstream OpenAI Codex CLI to the latest stable release or to a requested Codex CLI version. Use when asked to bring Goblins forward to a new rust-v* Codex release, rebase fork commits, resolve upstream conflicts, preserve Goblins package/CLI/prompt/release features, open or update the upgrade PR, or explain the upgrade procedure.
---

# Push Codex Updates To Goblin

## Objective

Move `brasalabs6/globim` forward from its current upstream Codex base to a requested or latest stable Codex CLI release while preserving the Goblins fork contract and leaving a reviewable PR with validation evidence.

Use this skill for both:

- "Update Goblins to latest Codex CLI."
- "Update Goblins to Codex CLI `<version>`."

## Non-Negotiables

- Verify "latest" before assuming it. Check upstream GitHub release/tag and npm `@openai/codex` for the target version.
- Preserve current Goblins public surface: package `@brasalabs/goblins`, command `goblin`, native packages `@brasalabs/goblins-*`, repository slug `brasalabs6/globim`, and internal Rust binary `codex`.
- Preserve Goblins prompt/personality behavior, especially model-manager prompt overrides that prevent remote `/models` metadata from replacing Goblins-owned prompts.
- Use a dedicated worktree and branch. Do not rebase or resolve conflicts in the user's active checkout.
- Make destructive Git operations visible before running them. Prefer `--force-with-lease` only for the dedicated upgrade branch after an intentional rebase.
- Record validation and residual risks in the PR body and session artifacts.

## Release Resolution

For an explicit version:

```bash
TARGET_VERSION=0.128.0
TARGET_TAG=rust-v${TARGET_VERSION}
git fetch upstream --tags --prune
git rev-parse "${TARGET_TAG}^{}"
npm view @openai/codex@"${TARGET_VERSION}" version
```

For "latest":

```bash
git fetch upstream --tags --prune
gh release view --repo openai/codex --json tagName,name,isLatest,publishedAt
npm view @openai/codex version
```

If GitHub and npm disagree, stop and explain the mismatch. Do not guess.

## Setup

1. Read repo contracts first: `SPECs.md`, `AGENTS.md`, `GOBLINS.md` when present, and recent session artifacts for prior upgrade decisions.
2. Fetch remotes:

```bash
git fetch origin --prune
git fetch upstream --tags --prune
```

3. Identify the current fork base. Prefer explicit release docs/tags; otherwise inspect history:

```bash
git log --oneline --decorate --graph --max-count=30
git describe --tags --match 'rust-v*' --abbrev=0 <current-fork-branch-or-sha>
```

4. Create a dedicated worktree and branch:

```bash
BRANCH=upgrade/codex-${TARGET_VERSION}
WORKTREE=/home/guilherme/brainstorm/worktrees/globim-upgrade-codex-${TARGET_VERSION}
BASE_BRANCH=origin/globimling

git worktree add -b "${BRANCH}" "${WORKTREE}" "${BASE_BRANCH}"
git -C "${WORKTREE}" push -u origin "${BRANCH}"
```

Use the repository's principal/default Goblins branch as the PR base. In this fork that is normally `globimling`, even when `main` temporarily contains an upstream mirror or an integration exception. If the current repo state contradicts the docs, record the tension instead of hiding it.

## Rebase Procedure

Replay the fork commits from the old upstream base onto the new release tag:

```bash
OLD_TAG=rust-v0.125.0
git -C "${WORKTREE}" rebase --onto "${TARGET_TAG}" "${OLD_TAG}"
```

During conflicts, preserve Goblins features over upstream defaults:

- Workflows: keep Goblins package staging, scoped npm package names, Linux/Windows release set, and `NPM_TOKEN` publication flow.
- `codex-cli/package.json`: keep `@brasalabs/goblins`, version equal to `TARGET_VERSION`, and bin `goblin`.
- `codex-cli/bin/goblin.js`: keep scoped native package names and reinstall guidance for `@brasalabs/goblins`.
- Cargo workspace: align workspace package versions to `TARGET_VERSION`.
- Prompt files and model-manager code: keep Goblins identity and prompt-preservation behavior.
- Docs: update release base and current target; do not reintroduce old `Globim`/`GLOBIM` branding except lowercase repo/branch slugs.

If Git drops commits as already upstream/applied, record which commits were dropped and why.

## Metadata Alignment

After the rebase, audit and align version surfaces:

```bash
node -p "require('./codex-cli/package.json').name + ' ' + require('./codex-cli/package.json').version"
grep -m1 '^version' codex-rs/Cargo.toml
rg -n '0\\.125|@openai/codex|Globim|GLOBIM|globim-linux|bin/globim\\.js' README.md GOBLINS.md codex-cli codex-rs .github scripts package.json
cargo update --workspace --offline
```

Use `rg -i globim` carefully: lowercase `globim` is still expected in repository URLs and `globimling`.

## Validation

Run the narrow checks first:

```bash
git diff --check
rg -n '^(<{7}|>{7}|={7}$)' . || true
rg -n 'Globim|GLOBIM' . || true
python3 .github/scripts/verify_cargo_workspace_manifests.py
python3 .github/scripts/verify_tui_core_boundary.py
python3 .github/scripts/verify_bazel_clippy_lints.py
pnpm install --frozen-lockfile
pnpm run format
just fmt
```

Run focused Rust checks for the surfaces touched. For the 0.128.0 upgrade, these were the useful minimum:

```bash
CARGO_TARGET_DIR=/home/guilherme/brainstorm/globim/codex-rs/target cargo test -p codex-models-manager refresh_available_models_preserves_goblins_prompt_fields
CARGO_TARGET_DIR=/home/guilherme/brainstorm/globim/codex-rs/target cargo test -p codex-core personality
CARGO_TARGET_DIR=/home/guilherme/brainstorm/globim/codex-rs/target cargo build -p codex-cli --bin codex
```

If disk fills during Rust builds, remove only regenerable build/temp artifacts after narrating what will be deleted. Then rerun the failed check.

Validate npm package staging:

```bash
python3 codex-cli/scripts/build_npm_package.py \
  --release-version "${TARGET_VERSION}" \
  --package @brasalabs/goblins \
  --staging-dir /tmp/goblins-npm-stage-"${TARGET_VERSION}" \
  --pack-output /tmp/goblins-npm-"${TARGET_VERSION}".tgz
```

For native package smoke checks, stage a platform package with a local `vendor-src` containing the compiled `codex` binary and `rg`, then link it under the meta package and run:

```bash
node /tmp/goblins-npm-stage-"${TARGET_VERSION}"/bin/goblin.js --version
```

Expected version output should include the target Codex CLI version.

## PR And Handoff

Push the rebased branch:

```bash
git push --force-with-lease origin "${BRANCH}"
```

Open or update a Draft PR with:

- target version and upstream tag
- base branch and worktree path
- conflict decisions
- dropped commits
- validation commands and outcomes
- known risks, including CI not yet completed
- session artifact path

Before handoff, verify mergeability against the intended base:

```bash
git merge-tree "${BASE_BRANCH}" HEAD >/tmp/goblins-upgrade-merge-tree.txt
rg -n '^(<{7}|>{7}|={7}$)|CONFLICT' /tmp/goblins-upgrade-merge-tree.txt || true
gh pr view <number> --repo brasalabs6/globim --json baseRefName,headRefName,state,isDraft,mergeStateStatus,statusCheckRollup
```

If GitHub reports conflicts, compare the intended default branch, the active release branch, and any upstream-mirror branch before changing the PR base. Change the PR base only after recording the reasoning.

## Completion Checklist

- Target release verified from upstream sources.
- Dedicated branch/worktree exists and is pushed.
- Rebase completed on top of `rust-v*` target tag.
- Goblins package, command, native packages, release workflow, and prompts are preserved.
- Versions and docs are aligned to the target version.
- No conflict markers or uppercase legacy typo regressions remain.
- Focused Rust, package, and workflow validations are recorded.
- Draft PR body contains evidence and residual risks.
- Session artifacts are closed and pushed.
