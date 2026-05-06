#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd -- "${script_dir}/.." && pwd)"

maybe_git_commit=""
if git_commit="$(git -C "${repo_root}" rev-parse HEAD 2>/dev/null)"; then
  maybe_git_commit="${git_commit%%$'\n'*}"
fi
build_time="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

if [[ -n "${maybe_git_commit}" ]]; then
  printf 'STABLE_OPEN_BITCOIN_BUILD_COMMIT %s\n' "${maybe_git_commit}"
fi
printf 'OPEN_BITCOIN_BUILD_TIME %s\n' "${build_time}"
