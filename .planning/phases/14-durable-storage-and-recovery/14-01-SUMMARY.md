---
phase: 14-durable-storage-and-recovery
plan: "01"
subsystem: storage
tags: [storage, fjall, recovery, db-02, db-03, db-04, db-05]
provides:
  - Fjall-backed node storage adapter
  - Keyspace schema initialization and compatibility checks
  - Runtime recovery marker persistence
affects: [DB-02, DB-03, DB-04, DB-05, storage, node]
requirements-completed: [DB-02, DB-03, DB-04, DB-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 14-2026-04-26T20-34-48
generated_at: 2026-04-26T20:45:57Z
completed: 2026-04-26
---

# Phase 14 Plan 01: Fjall Storage Adapter

## Accomplishments

- Added `fjall` to the node crate and Bazel target without adding database dependencies to pure crates.
- Added `FjallNodeStore` with separate keyspaces for headers, block index, chainstate, wallet, metrics, runtime, and schema metadata.
- Implemented schema initialization, schema mismatch detection, persist-mode mapping, namespace reads/writes, runtime metadata persistence, and recovery markers.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` passed.
- `bash scripts/check-pure-core-deps.sh` passed.

## Notes

The adapter persists current snapshot-shaped state only. Real peer sync integration and hot-path storage layout remain deferred to later phases.
