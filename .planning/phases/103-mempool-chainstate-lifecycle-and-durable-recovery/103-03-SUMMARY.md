---
phase: 103-mempool-chainstate-lifecycle-and-durable-recovery
plan: 03
subsystem: storage
tags: [rust, mempool, storage, recovery, fjall]
requirements-completed: [MEM-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 103-2026-07-01T12-38-00
generated_at: 2026-07-01T12:38:00.304Z
completed: 2026-07-01
---

# Phase 103 Plan 03: Durable Mempool Snapshot Summary

Added an Open Bitcoin-owned durable accepted-mempool snapshot and recovery evidence path.

## Accomplishments

- Added `StorageNamespace::Mempool` and a dedicated Fjall keyspace.
- Added `MempoolSnapshot` and `MempoolSnapshotRecord` for accepted mempool records with txid, wtxid, encoded transaction, fee, and virtual size.
- Added versioned encode/decode support using the existing snapshot schema pattern and consensus transaction codec.
- Added Fjall save, load, and clear APIs for mempool snapshots.
- Added recovery replay evidence for recovered, confirmed-dropped, missing-parent, policy-incompatible, duplicate, and evicted records.
- Added codec and Fjall tests for round-trip, reopen, clear, schema mismatch, and corruption behavior.

## Key Files

- `packages/open-bitcoin-node/src/storage.rs`
- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs`
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs`
- `packages/open-bitcoin-node/src/storage/fjall_store.rs`
- `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs`
- `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs`
- `docs/parity/source-breadcrumbs.json`

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::mempool_snapshot` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node mempool_snapshot` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node fjall_mempool_snapshot` passed.

## Boundaries

The durable format is an Open Bitcoin-owned versioned snapshot. This does not claim binary compatibility with Knots `mempool.dat`, destructive repair, operator repair UI, or unattended public-mainnet production recovery.
