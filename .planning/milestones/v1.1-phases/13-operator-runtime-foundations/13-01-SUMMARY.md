---
phase: 13-operator-runtime-foundations
plan: "01"
subsystem: storage
tags: [storage, adr, contracts, db-01]
provides:
  - DB-01 storage decision ADR
  - Adapter-facing storage namespace, schema, persistence, and recovery contracts
affects: [DB-01, storage, node]
requirements-completed: [DB-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 13-2026-04-26T18-50-22
generated_at: 2026-04-26T18:58:37.416Z
completed: 2026-04-26
---

# Phase 13 Plan 01: Storage Decision and Contracts

## Accomplishments

- Added `docs/architecture/storage-decision.md` with `Decision: fjall`, explicit `redb` and `rocksdb` comparisons, and Phase 14 recovery obligations.
- Added `packages/open-bitcoin-node/src/storage.rs` with storage namespaces, schema versions, persist modes, recovery actions, and typed storage errors.
- Exported the storage contracts from `open-bitcoin-node` and added parity breadcrumb coverage.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::` passed.

## Notes

No database crate, filesystem access, storage adapter, or persistence side effect was added in this plan.
