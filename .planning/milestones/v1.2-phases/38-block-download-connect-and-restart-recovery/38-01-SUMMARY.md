---
phase: 38
phase_name: "Block Download, Connect, and Restart Recovery"
plan_id: "38-01"
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: "38-2026-05-02T01-10-10"
generated_at: "2026-05-02T04:00:21Z"
status: completed
---

# Summary 38-01: Durable Block Reconciliation And Replay-Safe Chain Advancement

## Completed

- Added bounded block in-flight limits to the durable sync runtime and runtime-owned missing-block request top-ups that work both after fresh header import and after restart.
- Extended `ManagedPeerNetwork` and `PeerManager` with the smallest helper surface needed for best-chain block planning, tracked block requests, and local block-hash reconciliation.
- Added durable best-chain reconciliation logic that reconnects already-stored blocks on reopen and uses the undo-backed chainstate reorg path when a locally available better branch overtakes the active chain.
- Split the new runtime reconciliation logic into [`block_reconcile.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/block_reconcile.rs) so production files stay within the repo’s file-size gate.
- Refreshed README and roadmap status text for the new block-download/restart-recovery foundation and regenerated the tracked LOC report required by repo-native verification.

## Tests Added

- Restart reconnects persisted blocks before re-requesting them from peers.
- Restart can reorg to a better durable branch once the replacement blocks are already local.
- Header-store best-chain entry ordering is covered directly.
- Peer request helpers now cover tracked requested-block reads, capacity stop behavior, and the skip-only `None` path when capacity remains but no new block request is valid.

## Residual Risks

- `open-bitcoind` still only exposes the preflight startup path from Phase 35; richer operator controls and broader daemon-owned runtime surfaces remain later v1.2 work.
- Header-chain tip selection still uses the current synthetic per-header work model in `HeaderStore`; later parity evidence may require higher-fidelity cumulative work accounting.
