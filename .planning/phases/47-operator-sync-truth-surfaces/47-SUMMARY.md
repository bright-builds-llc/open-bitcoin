---
phase: 47
phase_name: Operator Sync Truth Surfaces
phase_lifecycle_id: 47-2026-05-26T21-36-05
lifecycle_mode: yolo
generated_at: 2026-05-26T21:57:42Z
generated_by: gsd-execute-phase
status: complete
requirements:
  - OBS-01
  - OBS-02
---

# Phase 47 Summary

## Completed

- Added a shared `SyncProgressSignal` status field and explicit last successful progress timestamp to `SyncStatus`, keeping unavailable fields visible when durable truth is absent.
- Projected progress signal, last progress time, peer status, latest error, and recovery guidance from durable sync summaries without changing validation or peer-credit semantics.
- Split operator progress metrics into header height, downloaded block height, connected block height, and the existing compatibility sync height.
- Updated structured sync summary logs with header/downloaded/connected heights, progress signal, and last progress time.
- Updated CLI status, dashboard, and RPC fallback projections so operator surfaces remain conservative about connected chainstate and never imply full sync before validation.
- Refreshed operator and architecture docs plus the tracked LOC and parity breadcrumb artifacts.

## Tests Added Or Updated

- Added sync summary coverage for progress signal and last successful progress timestamp.
- Updated sync metric/log projection tests for the new downloaded and connected block dimensions.
- Updated status serialization, CLI status/dashboard rendering, RPC `getblockchaininfo`, and offline runtime fixtures for the expanded shared status contract.

## Simplification Pass

The implementation keeps the shared status contract as the only operator truth model. `block_height` remains the connected-height compatibility alias, while new fields add machine-readable detail instead of requiring renderer-local interpretation or duplicate status DTOs.

## Residual Risk

Live mainnet smoke was not run because Phase 47 verification is deterministic and live public-network proof remains explicitly opt-in for Phase 50. RPC fallback status can infer only limited progress from `getblockchaininfo`, so it reports last successful progress as unavailable outside durable Open Bitcoin sync state.
