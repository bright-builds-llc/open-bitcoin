#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo is required" >&2
  exit 1
fi

if ! command -v cargo-llvm-cov >/dev/null 2>&1; then
  echo "error: cargo-llvm-cov is required" >&2
  echo "install it with: cargo +stable install cargo-llvm-cov --locked" >&2
  exit 1
fi

bash scripts/check-pure-core-deps.sh
cargo fmt --manifest-path packages/Cargo.toml --all --check
cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
cargo test --manifest-path packages/Cargo.toml --workspace --all-features

pure_core_crates=()
while IFS= read -r crate_name; do
  [[ -z "$crate_name" ]] && continue
  pure_core_crates+=("$crate_name")
done < scripts/pure-core-crates.txt
llvm_cov_args=()
for crate_name in "${pure_core_crates[@]}"; do
  llvm_cov_args+=(--package "$crate_name")
done

cargo llvm-cov clean --manifest-path packages/Cargo.toml --workspace
coverage_report="$(mktemp)"
trap 'rm -f "$coverage_report"' EXIT

cargo llvm-cov --manifest-path packages/Cargo.toml "${llvm_cov_args[@]}" --show-missing-lines --text >"$coverage_report"
if rg -q "^Uncovered Lines:" "$coverage_report"; then
  sed -n '/Uncovered Lines:/,$p' "$coverage_report" >&2
  exit 1
fi
