---
phase: 09-parity-harnesses-and-fuzzing
verified: 2026-04-24T10:34:00Z
status: passed
score: "8/8 phase truths verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 9-2026-04-24T10-06-16
generated_at: 2026-04-24T10:34:00Z
lifecycle_validated: true
---

# Phase 09 Verification: Parity Harnesses And Fuzzing

## Verdict

Phase 9 passed verification. The implementation adds reusable parity harness
infrastructure, RPC black-box parity coverage, deterministic property-style
tests, CI report collection, and parity documentation without broadening runtime
behavior outside the planned verification surface.

## Verified Truths

1. A first-party `open-bitcoin-test-harness` crate exists and exposes reusable
   functional case, target, isolation, and report APIs.
2. The harness supports Open Bitcoin and Knots-compatible JSON-RPC targets using
   the same case definitions.
3. RPC parity tests cover the current supported method surface and emit reports
   through `OPEN_BITCOIN_PARITY_REPORT_DIR`.
4. Optional Knots execution is explicitly gated by
   `OPEN_BITCOIN_KNOTS_RPC_ADDR`, `OPEN_BITCOIN_KNOTS_RPC_USER`, and
   `OPEN_BITCOIN_KNOTS_RPC_PASSWORD`; absent configuration records a skipped
   suite instead of failing local verification.
5. Integration helpers reserve ports, allocate sandbox directories, and guard
   child processes for future daemon-backed parity tests.
6. Codec property tests exercise compact-size integer and byte-vector round
   trips, canonical encodings, and truncation rejection.
7. Network property tests exercise message header and version-message round
   trips plus checksum corruption rejection.
8. Repo-native verification and CI collect parity reports while preserving
   `bash scripts/verify.sh` as the blocking contract.

## Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-test-harness --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --test black_box_parity`
- `OPEN_BITCOIN_PARITY_REPORT_DIR=$PWD/packages/target/parity-reports cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --test black_box_parity`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec --all-features --test properties`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features --test properties`
- `cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `bash scripts/verify.sh`

## Residual Risk

The full Bitcoin Knots daemon lifecycle is intentionally deferred until the
pinned baseline process path and fixture strategy are added. This phase records
the target contract and report format so future Knots-backed tests can reuse the
same case list without changing the default local verification contract.
