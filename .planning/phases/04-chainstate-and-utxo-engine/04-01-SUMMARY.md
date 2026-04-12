---
phase: 04-chainstate-and-utxo-engine
plan: 01
subsystem: chainstate-foundation
tags: [chainstate, utxo, workspace, pure-core]
provides:
  - new pure-core open-bitcoin-chainstate crate
  - typed coin, undo, chain-position, and snapshot models
  - workspace and open-bitcoin-core wiring for chainstate
affects: [chainstate, workspace, architecture]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 04-2026-04-12T23-38-43
generated_at: 2026-04-12T23:57:50.795Z
duration: ongoing
completed: 2026-04-12
---

# Phase 4 Plan 01 Summary

Established the pure-core chainstate foundation and wired it through the
workspace.

## Accomplishments

- Added `open-bitcoin-chainstate` as a first-party pure-core crate under
  `packages/` and wired it into Cargo, Bazel, `open-bitcoin-core`, and the
  repo purity checks.
- Defined typed `Coin`, `TxUndo`, `BlockUndo`, `ChainPosition`,
  `ChainstateSnapshot`, and transition helper types so later block mutations do
  not rely on untyped maps or runtime-owned state.
- Added unit coverage for the new model types and kept the workspace build
  green before any block-mutation logic landed.
