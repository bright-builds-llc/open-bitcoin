#!/usr/bin/env bash
set -euo pipefail

allowlist_path="${1:-scripts/panic-sites.allowlist}"
scan_output="$(mktemp)"
violations_output="$(mktemp)"

cleanup() {
  rm -f "$scan_output" "$violations_output"
}
trap cleanup EXIT

is_allowed() {
  local finding="$1"
  local finding_path="${finding%%:*}"
  local finding_text="${finding#*:*:}"
  local allowed_path=""
  local needle=""
  local allowlist_rationale=""

  [[ -f "$allowlist_path" ]] || return 1

  while IFS='|' read -r allowed_path needle allowlist_rationale; do
    [[ -z "$allowed_path" || "$allowed_path" == \#* ]] && continue
    if [[ "$finding_path" == "$allowed_path" && "$finding_text" == *"$needle"* ]]; then
      return 0
    fi
  done <"$allowlist_path"

  return 1
}

while IFS= read -r file_path; do
  [[ "$file_path" == */tests.rs ]] && continue
  awk -v file_path="$file_path" '
    /^[[:space:]]*#\[cfg\(test\)\]/ { exit }
    /^[[:space:]]*\/\// { next }
    /(^|[^[:alnum:]_])(unwrap|expect)[[:space:]]*\(/ ||
    /(panic|unreachable|todo|unimplemented)![[:space:]]*\(/ {
      print file_path ":" FNR ":" $0
    }
  ' "$file_path"
done < <(find packages -type f -path 'packages/open-bitcoin-*/src/*.rs' | sort) >"$scan_output"

while IFS= read -r finding; do
  if ! is_allowed "$finding"; then
    printf '%s\n' "$finding" >>"$violations_output"
  fi
done <"$scan_output"

if [[ -s "$violations_output" ]]; then
  {
    echo "Unclassified production panic-like sites found."
    echo
    cat "$violations_output"
    echo
    echo "Either replace the crash path with typed control flow or add a narrow entry to ${allowlist_path}:"
    echo "path|needle|rationale"
  } >&2
  exit 1
fi

echo "check-panic-sites.sh: no unclassified production panic-like sites"
