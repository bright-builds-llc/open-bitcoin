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
cargo llvm-cov --manifest-path packages/Cargo.toml --package open-bitcoin-core --fail-under-lines 100 --summary-only
