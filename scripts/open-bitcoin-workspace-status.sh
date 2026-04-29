#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd -- "${script_dir}/.." && pwd)"

maybe_git_commit="$(git -C "${repo_root}" rev-parse HEAD 2>/dev/null || true)"
maybe_git_commit="${maybe_git_commit%%$'\n'*}"

if [[ -n "${maybe_git_commit}" ]]; then
  printf 'STABLE_OPEN_BITCOIN_BUILD_COMMIT %s\n' "${maybe_git_commit}"
fi
