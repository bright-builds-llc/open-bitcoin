#!/usr/bin/env bash
set -euo pipefail

# The repo's file-size trigger is intentionally derived from floor(tau * 100).
readonly pi="3.141592653589793"
readonly tau="$(awk -v pi="$pi" 'BEGIN { printf "%.15f", 2 * pi }')"
readonly default_max_file_lines="$(awk -v tau="$tau" 'BEGIN { printf "%d", int(tau * 100) }')"
readonly max_file_lines="${OPEN_BITCOIN_MAX_FILE_LINES:-$default_max_file_lines}"

if ! [[ "$max_file_lines" =~ ^[0-9]+$ ]] || [[ "$max_file_lines" -le 0 ]]; then
  echo "error: OPEN_BITCOIN_MAX_FILE_LINES must be a positive integer" >&2
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

collect_rust_files() {
  git -C "$repo_root" ls-files packages \
    | grep -E '\.rs$' \
    | grep -Ev '^packages/(bitcoin-knots|target)/|/tests\.rs$|/tests/' \
    | LC_ALL=C sort
}

production_rust_report="$(mktemp)"
trap 'rm -f "$production_rust_report"' EXIT

checked_file_count=0
while IFS= read -r relative_path; do
  [[ -z "$relative_path" ]] && continue
  checked_file_count=$((checked_file_count + 1))

  line_count="$(wc -l <"$repo_root/$relative_path")"
  line_count="${line_count//[[:space:]]/}"

  if [[ "$line_count" -ge "$max_file_lines" ]]; then
    over_by="$((line_count - max_file_lines))"
    printf '%s\t%s\t%s\n' "$line_count" "$over_by" "$relative_path" >>"$production_rust_report"
  fi
done < <(collect_rust_files)

if [[ ! -s "$production_rust_report" ]]; then
  echo "Production Rust file-length check passed: ${checked_file_count} file(s) checked, limit ${max_file_lines} lines."
  exit 0
fi

echo "error: production Rust files must stay below ${max_file_lines} lines" >&2
echo >&2

offender_count=0
while IFS=$'\t' read -r line_count over_by relative_path; do
  offender_count=$((offender_count + 1))
  echo "- ${relative_path}: ${line_count} lines (${over_by} over)" >&2
  echo "  suggestions:" >&2
  echo "    * Move inline tests into a sibling tests.rs file." >&2
  echo "    * Split the file into foo.rs plus foo/ child modules." >&2
  echo "    * Pull low-level helpers, constants, parsers, or encoding logic into child modules." >&2
  echo "    * Extract distinct responsibilities into narrower modules." >&2
  echo "    * Keep the root file as a thin public entrypoint with delegation." >&2
  echo >&2
done < <(sort -t $'\t' -k1,1nr -k3,3 "$production_rust_report")

echo "Remediation summary: ${offender_count} file(s) exceeded the ${max_file_lines}-line limit across ${checked_file_count} checked production Rust file(s)." >&2
echo "Start by moving inline tests out, then split remaining responsibilities until each file is below the threshold." >&2
exit 1
