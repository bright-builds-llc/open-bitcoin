---
phase: 14-durable-storage-and-recovery
plan: "02"
subsystem: storage-codec
tags: [storage, dto, headers, wallet, chainstate, db-02, db-05]
provides:
  - Schema-versioned JSON snapshot DTOs
  - Header/block-index export and rebuild helpers
  - Parity breadcrumb manifest coverage for new Rust files
affects: [DB-02, DB-05, storage, parity]
requirements-completed: [DB-02, DB-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 14-2026-04-26T20-34-48
generated_at: 2026-04-26T20:45:57Z
completed: 2026-04-26
---

# Phase 14 Plan 02: Snapshot DTOs

## Accomplishments

- Added node-owned snapshot codecs for chainstate active chain, UTXOs, undo data, headers, block-index metadata, wallet descriptors/UTXOs, runtime metadata, recovery markers, and metrics placeholders.
- Added `HeaderStore::from_entries` and `HeaderStore::entries` so persisted header entries can rebuild the in-memory sync-state projection.
- Registered `storage/fjall_store.rs` and `storage/snapshot_codec.rs` in `docs/parity/source-breadcrumbs.json`.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network header_store::` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.

## Notes

The DTO layer intentionally keeps serialization and database concerns in `open-bitcoin-node`; pure chainstate, wallet, network, primitive, codec, and consensus crates remain storage-engine free.
