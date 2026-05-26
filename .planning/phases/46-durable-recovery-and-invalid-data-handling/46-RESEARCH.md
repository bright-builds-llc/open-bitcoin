---
phase: 46
phase_name: Durable Recovery and Invalid Data Handling
phase_lifecycle_id: 46-2026-05-26T17-16-33
lifecycle_mode: yolo
generated_at: 2026-05-26T17:16:33Z
generated_by: gsd-yolo-discuss-plan-execute-commit-and-push
requirements:
  - NODE-02
  - NODE-03
  - NODE-05
---

# Phase 46 Research

## Runtime Findings

- `DurableSyncRuntime::open` restores chainstate snapshots and header stores. It does not hydrate all durable block bodies into the in-memory network, but `block_reconcile::reconcile_best_chain` connects persisted best-chain blocks before the runtime asks peers for missing bodies.
- `sync_once_with_resolver` records accepted header and block contribution only after `ManagedPeerNetwork::receive_sync_message` succeeds. Block bodies are saved only after that validation path succeeds.
- `record_outcome` currently captures peer failures, peer attribution, and health signals, but active durable sync state is persisted without the peer failure as `last_error`.
- `SyncProgress` currently exposes validated header height and connected block height, plus per-run message/header/block counters. It does not expose a distinct durable downloaded block height.
- `SyncRuntimeError::from(NetworkError)` already maps invalid headers to `InvalidData`. `SyncRuntimeError::from(ManagedNetworkError)` maps chainstate validation failures to a generic network error, so invalid block bodies need a narrower classification.

## Status Findings

- `SyncStatus` already has `lifecycle`, `phase`, `last_error`, `recovery_action`, `lag`, and `resource_pressure`.
- Storage metadata has recovery actions for incompatible schema, corruption, unavailable namespace, interrupted writes, and backend failures.
- Operator docs already describe runtime status and resource pressure; they need the new durable progress field semantics and recovery guidance taxonomy.

## Implementation Notes

- Add `downloaded_block_height` and `connected_block_height` to `SyncProgress`.
- Add `downloaded_block_height` to `SyncRunSummary` and refresh it from the durable best chain after reconcile, peer success, and peer failure with partial progress.
- Derive durable `last_error` and `recovery_action` from the latest peer failure when no explicit error or storage recovery action is present.
- Map chainstate `BlockValidation`, `TransactionValidation`, and invalid tip-extension errors from sync block processing to `SyncRuntimeError::InvalidData`.
- Add tests that assert invalid block bodies are rejected with `PeerFailureReason::InvalidData`, no block contribution, no durable saved block, and no active-chain advancement.
