#!/usr/bin/env bash
set -euo pipefail

workspace_manifest="packages/Cargo.toml"
pure_core_list="scripts/pure-core-crates.txt"

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo is required" >&2
  exit 1
fi

if ! command -v rg >/dev/null 2>&1; then
  echo "error: rg is required" >&2
  exit 1
fi

if [[ ! -f "$workspace_manifest" ]]; then
  echo "error: missing workspace manifest: $workspace_manifest" >&2
  exit 1
fi

if [[ ! -f "$pure_core_list" ]]; then
  echo "error: missing pure-core allowlist: $pure_core_list" >&2
  exit 1
fi

metadata_json="$(cargo metadata --manifest-path "$workspace_manifest" --format-version 1 --no-deps)"
forbidden_crates=(tokio reqwest rustls rand)
forbidden_imports='std::fs|std::net|std::env|std::process|std::thread|tokio|reqwest|rustls|rand'

while IFS= read -r crate_name; do
  [[ -z "$crate_name" ]] && continue

  deps="$(
    node -e '
const metadata = JSON.parse(process.argv[1]);
const crateName = process.argv[2];
const pkg = metadata.packages.find((entry) => entry.name === crateName);
if (!pkg) {
  console.error(`missing crate: ${crateName}`);
  process.exit(2);
}
for (const dependency of pkg.dependencies) {
  console.log(dependency.name);
}
' "$metadata_json" "$crate_name"
  )"

  for forbidden_crate in "${forbidden_crates[@]}"; do
    if printf '%s\n' "$deps" | rg -x "$forbidden_crate" >/dev/null 2>&1; then
      echo "error: pure-core crate $crate_name depends on forbidden crate $forbidden_crate" >&2
      exit 1
    fi
  done

  crate_src_dir="packages/${crate_name}/src"
  if [[ ! -d "$crate_src_dir" ]]; then
    echo "error: missing source directory for pure-core crate $crate_name: $crate_src_dir" >&2
    exit 1
  fi

  if rg -n "$forbidden_imports" "$crate_src_dir" >/tmp/open-bitcoin-forbidden-imports.txt 2>/dev/null; then
    echo "error: forbidden imports found in pure-core crate $crate_name" >&2
    cat /tmp/open-bitcoin-forbidden-imports.txt >&2
    rm -f /tmp/open-bitcoin-forbidden-imports.txt
    exit 1
  fi
done <"$pure_core_list"

echo "Pure-core dependency and import checks passed."
