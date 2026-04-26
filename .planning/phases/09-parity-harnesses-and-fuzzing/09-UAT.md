---
status: complete
phase: 09-parity-harnesses-and-fuzzing
source:
  - .planning/phases/09-parity-harnesses-and-fuzzing/09-01-SUMMARY.md
  - .planning/phases/09-parity-harnesses-and-fuzzing/09-02-SUMMARY.md
  - .planning/phases/09-parity-harnesses-and-fuzzing/09-03-SUMMARY.md
  - .planning/phases/09-parity-harnesses-and-fuzzing/09-04-SUMMARY.md
started: 2026-04-26T13:04:41Z
updated: 2026-04-26T13:09:27Z
---

## Current Test

[testing complete]

## Tests

### 1. Cross-Implementation RPC Harness
expected: From the repo root, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-test-harness --all-features` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --test black_box_parity` pass. The same functional RPC case list targets Open Bitcoin by default, and the Knots-compatible target is skipped rather than failing when `OPEN_BITCOIN_KNOTS_RPC_ADDR`, `OPEN_BITCOIN_KNOTS_RPC_USER`, and `OPEN_BITCOIN_KNOTS_RPC_PASSWORD` are not configured.
result: pass

### 2. Parity Report Emission
expected: Running `OPEN_BITCOIN_PARITY_REPORT_DIR=$PWD/packages/target/parity-reports cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --test black_box_parity` passes and writes stable JSON and Markdown suite reports under `packages/target/parity-reports`, including Open Bitcoin results and the optional Knots target skip state when no Knots RPC endpoint is configured.
result: pass

### 3. Integration Isolation Helpers
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-test-harness --all-features` passes and includes coverage for distinct sibling sandboxes, distinct sibling port reservations, Basic auth header construction, and JSON/Markdown report writing. These helpers are available for future daemon-backed parity runs without hard-coded ports, shared data directories, or orphaned child processes.
result: pass

### 4. Codec Property-Style Coverage
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec --all-features --test properties` passes, proving generated transaction, compact-size, message-header, and truncation cases round-trip or reject malformed input deterministically.
result: pass

### 5. Network Property-Style Coverage
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features --test properties` passes, proving generated wire messages round-trip through the protocol codec and checksum mutations are rejected with the expected invalid-checksum behavior.
result: pass

### 6. Verify Path and Parity Catalog
expected: `bash scripts/verify.sh` passes from the repo root, defaults `OPEN_BITCOIN_PARITY_REPORT_DIR` to `packages/target/parity-reports`, includes `//:test_harness` in the Bazel smoke build, and leaves the parity catalog entry at `docs/parity/catalog/verification-harnesses.md` registered in `docs/parity/index.json`.
result: pass

## Summary

total: 6
passed: 6
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
