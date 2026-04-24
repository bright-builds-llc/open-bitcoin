---
phase: 09-parity-harnesses-and-fuzzing
plan: 04
subsystem: ci-parity-reporting
tags: [ci, docs, parity-reports, verification]
requires:
  - phase: 09-01
    provides: "Reusable Rust parity harness crate and RPC suite"
  - phase: 09-02
    provides: "Harness isolation and reports"
  - phase: 09-03
    provides: "Deterministic property-style tests"
provides:
  - "Verify-path parity report directory"
  - "CI artifact upload for parity reports"
  - "Parity catalog entry for harness and property coverage"
  - "Phase 9 closeout verification evidence"
affects: [ci, docs, verification]
key-files:
  modified:
    - scripts/verify.sh
    - .github/workflows/ci.yml
    - docs/parity/catalog/README.md
    - docs/parity/catalog/verification-harnesses.md
    - docs/parity/index.json
    - .planning/phases/09-parity-harnesses-and-fuzzing/09-04-SUMMARY.md
    - .planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md
requirements-completed: [VER-03, VER-04, PAR-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 9-2026-04-24T10-06-16
generated_at: 2026-04-24T10:31:00Z
duration: completed-session
completed: 2026-04-24
---

# Phase 09 Plan 04: CI Reporting And Closeout Summary

## Accomplishments

- Updated `scripts/verify.sh` to default
  `OPEN_BITCOIN_PARITY_REPORT_DIR` to `$PWD/packages/target/parity-reports`.
- Added `//:test_harness` to the repo-native Bazel smoke build.
- Updated CI to upload generated parity reports as an artifact while keeping
  `bash scripts/verify.sh` as the blocking command.
- Added `docs/parity/catalog/verification-harnesses.md` and registered it in
  `docs/parity/index.json`.
- Recorded the optional Knots target environment contract and the deferred
  full Knots process lifecycle in parity docs.

## Targeted Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-test-harness --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --test black_box_parity`
- `OPEN_BITCOIN_PARITY_REPORT_DIR=$PWD/packages/target/parity-reports cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --test black_box_parity`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-codec --all-features --test properties`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features --test properties`

## Full Verification

- passed: `bash scripts/verify.sh`
- passed final lifecycle gate:
  `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 9 --require-plans --require-verification --raw`

## Closeout

- Phase 9 implements the roadmap's four planned slices without expanding node,
  wallet, RPC, or CLI behavior beyond earlier supported surfaces.
