---
phase: 09-parity-harnesses-and-fuzzing
plan: 02
subsystem: integration-isolation
tags: [parity, isolation, reports, ci]
requires:
  - phase: 09-01
    provides: "Reusable Rust parity harness crate and RPC suite"
provides:
  - "Unique sandbox and port reservation helpers"
  - "Drop-owned process cleanup guard"
  - "JSON and Markdown parity report writers"
affects: [verification, ci, test-support]
key-files:
  modified:
    - packages/open-bitcoin-test-harness/src/lib.rs
    - packages/open-bitcoin-test-harness/src/isolation.rs
    - packages/open-bitcoin-test-harness/src/report.rs
    - packages/open-bitcoin-rpc/tests/black_box_parity.rs
requirements-completed: [VER-03, VER-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 9-2026-04-24T10-06-16
generated_at: 2026-04-24T10:29:00Z
duration: in-progress-session
completed: 2026-04-24
---

# Phase 09 Plan 02: Integration Isolation And Reports Summary

## Accomplishments

- Added `Sandbox`, `PortReservation`, and `ProcessGuard` to keep integration
  runs isolated from hard-coded ports, shared data directories, and orphaned
  child processes.
- Added harness tests proving sibling sandboxes and sibling port reservations
  are distinct.
- Added report helpers that emit stable JSON and Markdown suite reports.
- Wired the RPC black-box parity suite to write reports when
  `OPEN_BITCOIN_PARITY_REPORT_DIR` is set.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-test-harness --all-features`
- `OPEN_BITCOIN_PARITY_REPORT_DIR=$PWD/packages/target/parity-reports cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --test black_box_parity`

## Handoff

- Plan 09-03 adds deterministic property-style coverage for parser,
  serialization, and protocol boundaries.

