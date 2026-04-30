## deferred-20-03-out-of-scope-verification | 2026-04-27T11:48:21Z

- `packages/open-bitcoin-node/src/wallet_registry.rs` fails `bun run scripts/check-parity-breadcrumbs.ts --check` with `breadcrumb block is missing or stale; run with --write`.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc -p open-bitcoin-cli --all-features` fails unrelated `operator::detect` tests in dirty files outside Plan 20-03's write set:
  - `open-bitcoin-cli/src/operator/detect/tests.rs::detects_linux_core_knots_candidates_read_only`
  - `open-bitcoin-cli/src/operator/detect/tests.rs::detects_macos_core_knots_candidates_read_only`
- `bash scripts/verify.sh` fails on the pre-existing stale LOC report at `docs/metrics/lines-of-code.md`.
