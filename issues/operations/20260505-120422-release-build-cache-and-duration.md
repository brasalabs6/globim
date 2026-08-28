# Release builds are too slow without effective Rust cache

Status: OPEN
Type: follow-up
Category: operations
Created: 2026-05-05T12:04:22-03:00
Severity: medium
Owner/action type: operations
Evidence quality: direct, partial while `rust-v0.128.1` is still running

## Summary

- The `rust-release` workflow for `rust-v0.128.1` is taking roughly an hour or more before npm publication can happen.
- The release path intentionally waits for native Linux and Windows artifacts before creating the GitHub Release and publishing npm packages.
- The Linux musl build completed, but Windows was still compiling at issue creation time.
- The repo has GitHub Issues disabled, so this local issue artifact captures the follow-up until remote issues are enabled or another tracker is chosen.

## Impact

- Slow release feedback delays confirmation that `@brasalabs/goblins` was published to npm.
- Long release runs make failures expensive because a broken Windows/Linux build may appear only after tens of minutes.
- High GitHub Actions runtime cost is likely because release runners compile optimized Rust targets from mostly cold state.
- The user experience during release is poor: the tag is pushed, but there is a long opaque wait before npm availability changes.

## Evidence

- Files:
  - `.github/workflows/rust-release.yml` builds Linux musl with `cargo build --target x86_64-unknown-linux-musl --release --bin codex`.
  - `.github/workflows/rust-release.yml` builds Windows with `cargo build --target x86_64-pc-windows-msvc --release --bin codex --bin codex-windows-sandbox-setup --bin codex-command-runner`.
  - `.github/workflows/rust-release.yml` only runs the `release` and `publish-npm` jobs after both native build jobs complete.
- Commits:
  - Release tag `rust-v0.128.1` points at `6be0042db974434870495ad0da46d78462ad9f2a`.
  - PR `#10` merged the CI release-readiness changes before the tag was pushed.
- PRs/issues/checks:
  - `rust-v0.128.1` workflow run: `https://github.com/brasalabs6/goblins/actions/runs/25380397535`.
  - `tag-check` passed in 13s: `https://github.com/brasalabs6/goblins/actions/runs/25380397535/job/74426816409`.
  - Linux musl build passed in 47m10s: `https://github.com/brasalabs6/goblins/actions/runs/25380397535/job/74426876037`.
  - Windows build was still in `Cargo build` after about one hour at issue creation time: `https://github.com/brasalabs6/goblins/actions/runs/25380397535/job/74426876178`.
  - Prior successful `rust-v0.128.0` release took about 1h27m17s, so this appears systemic rather than a one-off delay.
- Reproduction notes:
  - At issue creation time, `npm view @brasalabs/goblins versions --json` returned only `0.125.1` and `0.128.0`.
  - At issue creation time, `npm view @brasalabs/goblins dist-tags --json` returned `latest: 0.128.0`.

## Proposed Action

- Add Rust build caching to `.github/workflows/rust-release.yml` for both `build-unix` and `build-windows`.
- Prefer `sccache` with the GitHub Actions backend if it works reliably for this repository; set `RUSTC_WRAPPER=sccache` only after preserving the existing Linux UBSan wrapper behavior.
- Add Cargo dependency/target caching keyed by OS, Rust version, target triple, release profile, `Cargo.lock`, and relevant build scripts.
- Verify the Linux musl path separately because it currently uses a hermetic `CARGO_HOME`, Zig, musl tools, UBSan wrapper setup, and `rusty_v8` musl artifact configuration.
- Verify the Windows path separately because it builds three release binaries and may benefit differently from target-cache versus compiler-cache.
- Add lightweight timing output around dependency restore, Cargo build, artifact staging, release creation, and npm publish steps.
- Keep release correctness higher priority than speed: do not reuse caches across incompatible targets or stale lockfiles.

## Acceptance Criteria

- Release workflow still publishes the same artifacts and npm packages as before.
- Cold-cache release remains correct.
- Warm-cache release is measurably faster for both Linux musl and Windows.
- Build logs show cache hit/miss information and per-phase duration.
- The change does not re-enable heavy Bazel/Rust SDK workflows on pull requests.

## Related

- Userflows: N/A
- Changes: N/A
- Sessions: release session for `rust-v0.128.1` on 2026-05-05
- Replacement: N/A

## Closure

- Final status: OPEN
- Closed at: N/A
- Reason: N/A
- Validation: N/A
- Replacement: N/A
