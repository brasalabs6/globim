#!/usr/bin/env bash
set -euo pipefail

mode="${1:?usage: detect-source-change.sh <bazel|ci|sdk>}"

if [[ "${GITHUB_EVENT_NAME:-}" == "workflow_dispatch" ]]; then
  echo "source=true" >> "$GITHUB_OUTPUT"
  exit 0
fi

if [[ "${GITHUB_EVENT_NAME:-}" == "pull_request" ]]; then
  base_sha="${PR_BASE_SHA:?PR_BASE_SHA is required for pull_request events}"
  head_sha="${PR_HEAD_SHA:?PR_HEAD_SHA is required for pull_request events}"
elif [[ "${GITHUB_EVENT_NAME:-}" == "push" ]]; then
  base_sha="${PUSH_BEFORE_SHA:-}"
  head_sha="${GITHUB_SHA:?GITHUB_SHA is required for push events}"
  if [[ -z "$base_sha" || "$base_sha" =~ ^0+$ ]]; then
    echo "source=true" >> "$GITHUB_OUTPUT"
    exit 0
  fi
else
  echo "source=true" >> "$GITHUB_OUTPUT"
  exit 0
fi

mapfile -t files < <(git diff --name-only --no-renames "$base_sha" "$head_sha")

is_common_source_file() {
  local file="$1"
  [[ "$file" == codex-rs/* ]] && return 0
  [[ "$file" == codex-cli/* ]] && return 0
  [[ "$file" == scripts/* ]] && return 0
  [[ "$file" == tools/* ]] && return 0
  [[ "$file" == third_party/* ]] && return 0
  [[ "$file" == sdk/* ]] && return 0
  case "$file" in
    BUILD.bazel|*/BUILD.bazel|*.bzl|MODULE.bazel|MODULE.bazel.lock|.bazelrc|.bazelversion)
      return 0
      ;;
    Cargo.toml|Cargo.lock|justfile|package.json|pnpm-lock.yaml|pnpm-workspace.yaml)
      return 0
      ;;
    workspace_root_test_launcher.sh.tpl|workspace_root_test_launcher.bat.tpl)
      return 0
      ;;
  esac
  return 1
}

is_mode_source_file() {
  local file="$1"
  is_common_source_file "$file" && return 0
  return 1
}

source=false
for file in "${files[@]}"; do
  echo "changed: $file"
  if is_mode_source_file "$file"; then
    source=true
  fi
done

echo "source=$source" >> "$GITHUB_OUTPUT"
