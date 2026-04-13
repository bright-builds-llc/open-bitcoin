#!/usr/bin/env bash
set -euo pipefail

require_command() {
  local command_name="$1"
  local maybe_install_hint="${2:-}"

  if command -v "$command_name" >/dev/null 2>&1; then
    return
  fi

  echo "error: ${command_name} is required" >&2
  if [[ -n "$maybe_install_hint" ]]; then
    echo "$maybe_install_hint" >&2
  fi
  exit 1
}

require_command cargo
require_command cargo-llvm-cov "install it with: cargo install cargo-llvm-cov --locked"
require_command bazel "install Bazelisk or Bazel and ensure \`bazel\` is on PATH"
require_command grep
require_command node

bash scripts/check-pure-core-deps.sh
cargo fmt --manifest-path packages/Cargo.toml --all --check
cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
cargo test --manifest-path packages/Cargo.toml --workspace --all-features
bazel build //:core //:node

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
if grep -q "^Uncovered Lines:" "$coverage_report"; then
  sed -n '/Uncovered Lines:/,$p' "$coverage_report" >&2
  exit 1
fi
