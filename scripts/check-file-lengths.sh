#!/usr/bin/env bash
set -euo pipefail

# Delegate to the managed Bright Builds file-length scan so local verify.sh
# matches `.github/workflows/bright-builds-checks.yml`. Do not reimplement
# extensions, exclusions, physical-line counting, or the 628-line limit here.

if ! command -v bun >/dev/null 2>&1; then
  echo "error: bun is required" >&2
  exit 1
fi

if ! command -v git >/dev/null 2>&1; then
  echo "error: git is required" >&2
  exit 1
fi

repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "error: must be run from within a git repository" >&2
  exit 1
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
checker="${script_dir}/bright-builds-check.ts"
if [[ ! -f "$checker" ]]; then
  echo "error: missing ${checker}" >&2
  exit 1
fi

cd "$repo_root"
exec bun "$checker" file-lengths
