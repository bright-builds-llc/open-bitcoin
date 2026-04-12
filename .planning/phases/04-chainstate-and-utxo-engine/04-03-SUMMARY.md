---
phase: 04-chainstate-and-utxo-engine
plan: 03
subsystem: chainstate-adapters-and-parity
tags: [chainstate, node, parity, docs, verification]
provides:
  - node-side snapshot store and managed chainstate adapter
  - targeted chainstate parity integration tests
  - parity catalog promotion of chainstate to done
affects: [chainstate, node, parity, verification]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 04-2026-04-12T23-38-43
generated_at: 2026-04-12T23:57:50.795Z
duration: ongoing
completed: 2026-04-12
---

# Phase 4 Plan 03 Summary

Closed the chainstate phase with adapter-owned persistence and auditable parity
artifacts.

## Accomplishments

- Added `ManagedChainstate`, `ChainstateStore`, and `MemoryChainstateStore` in
  `open-bitcoin-node` so persistence stays outside the pure-core chainstate
  crate.
- Added repo-owned parity integration tests that cover connect, disconnect,
  reorg, unspendable outputs, and BIP30-style overwrite rejection through the
  public chainstate API.
- Added `docs/parity/catalog/chainstate.md`, updated `docs/parity/index.json`,
  and kept `bash scripts/verify.sh` green with the new pure-core coverage
  surface in place.
