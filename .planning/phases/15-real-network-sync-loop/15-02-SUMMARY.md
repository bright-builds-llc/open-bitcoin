---
phase: 15-real-network-sync-loop
plan: "02"
subsystem: storage
tags: [sync, storage, blocks, fjall, sync-02, sync-03]
provides:
  - Durable raw block persistence helpers
  - Block round-trip coverage in isolated temp stores
affects: [SYNC-02, SYNC-03, storage, node]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 15-2026-04-26T21-05-56
generated_at: 2026-04-26T21:08:01.619Z
completed: 2026-04-26
---

# Phase 15 Plan 02: Durable Downloaded Blocks

## Accomplishments

- Added `FjallNodeStore::save_block` and `FjallNodeStore::load_block`.
- Stored downloaded blocks in the block-index namespace under canonical block-hash keys.
- Covered block round-trip and missing-block behavior in the existing restart-focused storage test.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features` passed.

## Notes

The raw block storage is intentionally a shell-layer helper. Pure block, consensus, and chainstate crates remain database-free.
