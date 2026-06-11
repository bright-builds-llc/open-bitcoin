---
phase: 68-full-active-chain-validation-and-durable-persistence
plan: 02
subsystem: sync-status
tags: [status, active-chain, chain-work, sync, rust]
requires:
  - phase: 68-full-active-chain-validation-and-durable-persistence
    plan: 01
    provides: durable connected-progress proof
provides:
  - Additive active-chain status fields
  - Runtime projection from connected chainstate to validated active-chain evidence
  - Focused status and sync-progress regression coverage
affects: [phase-68, status, sync-runtime, operator-evidence]
tech-stack:
  added: []
  patterns: [serde-compatible-status-extension, decimal-chain-work-string]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
key-decisions:
  - "Expose cumulative work as a decimal string so JSON consumers avoid integer precision ambiguity."
  - "Keep `block_height` as a compatibility alias for connected chainstate height while adding explicit validated active-chain fields."
patterns-established:
  - "Additive status fields use serde defaults and are populated only from connected chainstate evidence."
requirements-completed: [SYNC-01, SYNC-02, SYNC-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 68-2026-06-11T11-56-49
generated_at: 2026-06-11T12:47:39Z
duration: 22min
completed: 2026-06-11
---

# Phase 68 Plan 02 Summary

**Self-Check: PASSED**

## Accomplishments

- Added `validated_active_chain_height`, `maybe_validated_active_chain_hash`, and `maybe_validated_active_chain_work` to `SyncProgress`.
- Carried `chain_work` through sync runtime progress points and `SyncRunSummary`.
- Updated runtime status projection so active-chain fields mirror connected chainstate, not downloaded-only bodies.
- Updated status fixtures and sync progress tests for connected, downloaded-only, and unavailable states.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_progress --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync:: --all-features` passed with 75 passed and 1 ignored live-network smoke.

## Residual Risks

- Metrics remain height-focused in this phase; richer cross-surface observability is still Phase 72 scope.
