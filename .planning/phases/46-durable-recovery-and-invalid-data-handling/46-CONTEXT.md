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

# Phase 46 Context

## Goal

Durable sync must recover from partial work and invalid peer data without losing validated progress, duplicating block connects, or advancing a bad chain.

## Recommended Answers

1. Scope the phase to the existing durable sync runtime, status projection, operator documentation, and deterministic tests. Do not add a new durable queue or a long-running mainnet daemon loop in this phase.
2. Preserve the existing `block_height` status field as the connected active-chain height for compatibility. Add explicit progress fields so operators can distinguish validated headers, durable downloaded blocks, and connected blocks.
3. Treat the durable store and header store as the restart source of truth. Reopened runtimes should reconcile already persisted blocks before requesting them again.
4. Reject invalid peer headers or blocks before they receive contribution credit. Rejections must be attributed to the peer and must not persist or connect the invalid block body.
5. Surface recovery guidance from storage metadata first. When the latest issue is peer/network related, derive actionable guidance from the peer failure reason so transient network failures, incompatible stores, corrupt stores, resource limits, invalid data, and cancellation-oriented operator guidance remain distinguishable.
6. Keep verification deterministic: unit/runtime tests first, then repo-native verification. Live mainnet smoke is not required for this phase.

## Decisions

- D-01: `SyncProgress.block_height` remains a connected block height alias to avoid breaking existing RPC/CLI consumers.
- D-02: New durable progress fields will be additive and serialized through existing status snapshots.
- D-03: Invalid block handling will classify chainstate block validation failures as peer invalid data at the sync runtime boundary.
- D-04: Recovery guidance will be stored in durable sync state whenever a peer failure is present and no storage recovery action overrides it.
- D-05: Documentation will explain the durable recovery fields and include repo-local Cargo/Bazel operator commands.

## Success Criteria Mapping

- SC-01 / NODE-02: Restart tests prove persisted blocks are reconciled without duplicated requests or lost connected progress.
- SC-02 / NODE-03: Invalid header and block tests prove peer attribution, no contribution credit, no block persistence, and no active-chain advancement.
- SC-03 / NODE-05: Status and docs distinguish peer/network, invalid data, storage, resource limit, and cancellation guidance.
- SC-04 / NODE-02/NODE-05: Durable status separates header, downloaded block, connected block, and error/recovery state.
