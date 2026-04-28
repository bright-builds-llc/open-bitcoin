---
phase: 21-drop-in-parity-audit-and-migration
plan: "02"
subsystem: migration-proof-and-notice-sync
requirements-completed: [CLI-07, WAL-08, MIG-03, MIG-04, MIG-05]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 21-2026-04-27T23-11-20
generated_at: 2026-04-28T00:35:35Z
tags:
  - migration
  - binary-test
  - parity
  - notices
  - verification
key_files:
  created: []
  modified:
    - packages/open-bitcoin-cli/src/operator/migration.rs
    - packages/open-bitcoin-cli/src/operator/migration/planning.rs
    - packages/open-bitcoin-cli/src/operator/migration/tests.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
    - packages/open-bitcoin-cli/tests/operator_flows.rs
metrics:
  completed_date: "2026-04-27"
  files_created: 0
  files_modified: 5
---

# Phase 21 Plan 02 Summary

## One-Liner

Phase 21 now proves the migration planner end to end with a sandboxed binary test and keeps runtime migration notices synchronized with the machine-readable parity ledger.

## What Was Built

- Added an end-to-end `open-bitcoin migrate plan` binary test that:
  - seeds a realistic detected source install in a temp sandbox
  - verifies the explanation-first dry-run output shape
  - checks that wallet/config/service evidence is surfaced
  - confirms source files remain unchanged after the run
  - rejects secret leakage such as cookie contents
- Added runtime migration notice coverage so the planner surfaces only the
  relevant intentional differences for the selected migration surface.
- Added a repo-root sync guard in `operator_flows` that asserts the runtime
  migration notice IDs and summaries remain aligned with `docs/parity/index.json`
  without forcing installed binaries to read repo docs at runtime.
- Hardened the final operator-binary verification path so the migration test and
  the surrounding operator test suite stay reliable under the heavier
  `bash scripts/verify.sh` run.

## Task Commits

1. **Task 1 and Task 2: add binary proof and parity-notice sync coverage** — `4a2087f` `feat(21): add migration dry-run planner and parity audit`

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_flows -- --nocapture`
- `bash scripts/verify.sh`

## Deviations from Plan

- None in shipped behavior. The final implementation follows the planned
  self-contained runtime-notice model and proves it with a repo-root sync test
  rather than a runtime doc parser.

## Self-Check: PASSED

- The migration binary dry-run remains read-only and secret-safe in isolated
  sandbox tests.
- Runtime migration notices stay auditable against the parity ledger through the
  dedicated sync guard.
