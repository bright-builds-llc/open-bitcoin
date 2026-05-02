#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

bash scripts/ensure-git-hooks.sh >/dev/null

echo "Git hooks are configured from .githooks"
echo "Re-running this installer is safe and idempotent."
echo "Git will now run the repo-managed hooks before commit."
