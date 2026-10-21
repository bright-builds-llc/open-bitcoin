#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# A failed regression check stops the ordinary guard before classification.
bun test "${script_dir}/check-panic-sites.test.ts"
exec bun "${script_dir}/check-panic-sites.ts" "${1:-scripts/panic-sites.allowlist}"
