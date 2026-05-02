#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "error: must be run from within a git repository" >&2
  exit 1
}

cd "$repo_root"

if [[ ! -d ".githooks" ]]; then
  echo "error: missing .githooks directory at ${repo_root}/.githooks" >&2
  exit 1
fi

if [[ ! -x ".githooks/pre-commit" ]]; then
  echo "error: missing required executable hook: .githooks/pre-commit" >&2
  exit 1
fi

current_hooks_path="$(git config --local --get core.hooksPath || true)"
if [[ "$current_hooks_path" == ".githooks" ]]; then
  echo "Git hooks already configured: core.hooksPath=.githooks"
  exit 0
fi

git config --local core.hooksPath .githooks
echo "Configured git hooks: core.hooksPath=.githooks"
