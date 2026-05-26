---
phase: 46
phase_name: Durable Recovery and Invalid Data Handling
phase_lifecycle_id: 46-2026-05-26T17-16-33
lifecycle_mode: yolo
generated_at: 2026-05-26T17:33:21Z
generated_by: gsd-execute-phase
status: complete
requirements:
  - NODE-02
  - NODE-03
  - NODE-05
---

# Phase 46 Summary

## Completed

- Added explicit durable sync progress separation with `downloaded_block_height` and `connected_block_height` while preserving `block_height` as the connected-height compatibility alias.
- Refreshed sync summaries from runtime state so returned summaries and durable sync metadata report validated header height, durable downloaded height, and connected chainstate height after restart/recovery.
- Persisted latest peer failure error and peer-derived recovery guidance in active durable sync state when no storage recovery action takes priority.
- Classified sync-time chainstate validation failures for peer-provided blocks as invalid peer data, so invalid blocks are attributed to peers without being saved or connected.
- Updated CLI human status rendering, RPC/status fixtures, operator docs, status architecture docs, and the tracked LOC report.

## Tests Added Or Updated

- Added a restart recovery test that persists headers and partial durable block bodies, reopens the runtime, reconnects available blocks, requests only the missing block, and asserts separated downloaded/connected status.
- Added an invalid block body test that verifies peer attribution, `InvalidData` classification, no block contribution credit, no durable block save, no active-chain advancement, and durable last-error/recovery guidance.
- Updated status, RPC, and CLI fixtures for the expanded additive `SyncProgress` contract.

## Simplification Pass

The implementation uses the existing header store, durable block store, and best-chain reconciliation path as the recovery source of truth. No new durable queue, background worker, status DTO, or dependency was introduced.

## Residual Risk

Live mainnet smoke was not run because Phase 46 verification is deterministic and live-network review remains explicitly opt-in. Intentional cancellation is documented through the existing live-smoke report behavior; no new daemon cancellation API was added in this phase.
