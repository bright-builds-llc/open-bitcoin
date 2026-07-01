---
phase: 103-mempool-chainstate-lifecycle-and-durable-recovery
plan: 02
subsystem: node-network
tags: [rust, mempool, chainstate, reorg, testing]
requirements-completed: [MEM-04, MEM-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 103-2026-07-01T12-38-00
generated_at: 2026-07-01T12:38:00.304Z
completed: 2026-07-01
---

# Phase 103 Plan 02: Managed Mempool Lifecycle Summary

Wired pure mempool lifecycle cleanup into managed block connect and reorg transitions.

## Accomplishments

- Added `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` as the node shell bridge.
- `connect_local_block` and connected `connect_stored_block` now apply mempool lifecycle cleanup only after successful chainstate connection.
- Runtime `transactions_by_txid` and `transactions_by_wtxid` caches are cleared for confirmed, conflicting, and descendant removals.
- `reorg_to_branch` removes transactions confirmed by the replacement branch and reconsiders non-coinbase transactions from disconnected blocks through `MempoolOutcome`.
- Managed mempool info now exposes typed capacity status and rolling-fee parity status from the pure pressure summary.

## Key Files

- `packages/open-bitcoin-node/src/network.rs`
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs`
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs`
- `packages/open-bitcoin-node/src/mempool.rs`
- `packages/open-bitcoin-node/src/network/admission_bridge.rs`
- `docs/parity/source-breadcrumbs.json`

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node mempool_lifecycle_cases` passed.

## Boundaries

Reorg reconsideration is bounded to disconnected block transactions and does not add public relay serving, fanout, rebroadcast, sleeps, public-network behavior, compact block relay, or package relay.
