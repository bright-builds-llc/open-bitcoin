---
phase: 09-parity-harnesses-and-fuzzing
plan: 01
subsystem: parity-harness
tags: [parity, rpc, harness, black-box]
requires: []
provides:
  - "Reusable Rust parity harness crate"
  - "Target-neutral functional RPC case model"
  - "Default Open Bitcoin RPC black-box parity suite"
  - "Optional Knots-compatible RPC target"
affects: [verification, rpc, ci]
key-files:
  modified:
    - packages/Cargo.toml
    - BUILD.bazel
    - packages/open-bitcoin-test-harness/Cargo.toml
    - packages/open-bitcoin-test-harness/BUILD.bazel
    - packages/open-bitcoin-test-harness/src/lib.rs
    - packages/open-bitcoin-test-harness/src/case.rs
    - packages/open-bitcoin-test-harness/src/target.rs
    - packages/open-bitcoin-rpc/Cargo.toml
    - packages/open-bitcoin-rpc/tests/black_box_parity.rs
requirements-completed: [VER-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 9-2026-04-24T10-06-16
generated_at: 2026-04-24T10:28:00Z
duration: in-progress-session
completed: 2026-04-24
---

# Phase 09 Plan 01: Cross-Implementation Black-Box Harness Summary

## Accomplishments

- Added `open-bitcoin-test-harness` as a first-party workspace crate with a
  target-neutral `FunctionalCase` suite model.
- Added `HarnessTarget` and `RpcHttpTarget` so suites exercise authenticated
  JSON-RPC HTTP behavior instead of calling dispatch helpers directly.
- Added `packages/open-bitcoin-rpc/tests/black_box_parity.rs`, where the same
  `functional_cases()` list targets Open Bitcoin by default and an external
  Knots-compatible endpoint when `OPEN_BITCOIN_KNOTS_RPC_*` variables are set.
- Exposed the harness crate through Bazel as `//:test_harness`.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-test-harness --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --test black_box_parity`

## Handoff

- Plan 09-02 extends the harness with reusable isolation and report helpers.

