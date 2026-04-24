# Quick Task 260424-jnn: Partial Clippy Panic-Lint Enforcement - Summary

**Completed:** 2026-04-24
**Status:** Complete

## Changes

- Added production-only Clippy deny attributes for panic-related lints to selected runtime and domain crate roots.
- Added the same deny attributes to the `open-bitcoin-cli` and `open-bitcoind` production binary roots.
- Left benchmark and test-harness crates out of this Clippy layer; they remain covered by `scripts/check-panic-sites.sh`.

## Verification

- `cargo clippy --manifest-path packages/Cargo.toml --workspace --lib --all-features -- -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::unreachable -D clippy::todo -D clippy::unimplemented -D clippy::panic_in_result_fn`
- `cargo clippy --manifest-path packages/Cargo.toml --workspace --bins --all-features -- -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::unreachable -D clippy::todo -D clippy::unimplemented -D clippy::panic_in_result_fn`
- `bash scripts/check-panic-sites.sh`
- `bash scripts/verify.sh`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --workspace --all-features`
