#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

git config core.hooksPath .githooks

echo "Configured core.hooksPath to .githooks"
echo "Git will now run .githooks/pre-commit before each commit."
