---
phase: 68-full-active-chain-validation-and-durable-persistence
plan: 01
subsystem: sync-runtime
tags: [sync, chainstate, durable-persistence, restart-resume, rust]
requires:
  - phase: 67-release-boundaries-and-deterministic-verification
    provides: deterministic verification and v1.5 release-boundary closeout
provides:
  - Same-datadir durable active-chain reconnect proof
  - Regression coverage for validated connected progress surviving runtime reopen
affects: [phase-68, sync-runtime, durable-store, status]
tech-stack:
  added: []
  patterns: [durable-runtime-reopen-test, connected-chainstate-progress-proof]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/sync/tests.rs
key-decisions:
  - "Treat connected chainstate height/hash/work as the Phase 68 active-chain progress proof."
patterns-established:
  - "Runtime restart tests assert both status projection and persisted chainstate snapshot state."
requirements-completed: [SYNC-01, SYNC-03, SYNC-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 68-2026-06-11T11-56-49
generated_at: 2026-06-11T12:47:39Z
duration: 18min
completed: 2026-06-11
---

# Phase 68 Plan 01 Summary

**Self-Check: PASSED**

## Accomplishments

- Added `connected_active_chain_progress_survives_runtime_reopen`.
- Proved a requested best-chain child block persists as a downloaded body and connected active-chain tip.
- Reopened a fresh `DurableSyncRuntime` on the same store and verified connected height, hash, and cumulative work survived.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node connected_active_chain_progress_survives_runtime_reopen --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync:: --all-features` passed with 75 passed and 1 ignored live-network smoke.

## Residual Risks

- Public-mainnet sync duration and stay-current behavior remain deferred to later v1.6 phases and opt-in UAT.
