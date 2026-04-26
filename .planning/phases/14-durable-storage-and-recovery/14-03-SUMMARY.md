---
phase: 14-durable-storage-and-recovery
plan: "03"
subsystem: storage-recovery
tags: [storage, restart, schema, corruption, recovery, db-02, db-03, db-04]
provides:
  - Restart persistence tests
  - Schema mismatch and corruption recovery tests
  - Interrupted-write and clean-shutdown marker tests
affects: [DB-02, DB-03, DB-04, storage, recovery]
requirements-completed: [DB-02, DB-03, DB-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 14-2026-04-26T20-34-48
generated_at: 2026-04-26T20:45:57Z
completed: 2026-04-26
---

# Phase 14 Plan 03: Recovery Evidence

## Accomplishments

- Added isolated temp-directory tests that close and reopen the fjall store before loading persisted chainstate, headers, block-index entries, wallet, metrics, and runtime metadata.
- Added schema mismatch and malformed-record tests that return typed `StorageError` variants with recovery guidance.
- Added recovery marker tests for interrupted writes, reindex guidance, repair guidance, and clean-shutdown marker clearing.
- Updated the storage ADR with Phase 14 implementation evidence.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` passed.

## Notes

Recovery helpers expose operator guidance and test behavior without mutating real datadirs or service-manager state.
