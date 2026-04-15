---
phase: 06-p2p-networking-and-sync
plan: 03
subsystem: managed-node-network-adapter
tags: [node, chainstate, mempool, relay, wtxidrelay]
provides:
  - managed peer network wrapper
  - block application through managed chainstate
  - transaction relay through managed mempool
affects: [networking, node, mempool, chainstate]
requirements_completed: [P2P-01, P2P-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 06-2026-04-15T00-28-26
generated_at: 2026-04-15T01:00:09Z
completed: 2026-04-14
---

# Phase 6 Plan 03 Summary

Connected the pure-core peer manager to the node shell without breaking the
functional-core boundary.

## Accomplishments

- Added `ManagedPeerNetwork` in `open-bitcoin-node` so the shell can host
  peer-manager state, local block or transaction stores, and encoded-message
  exchange over managed chainstate and mempool adapters.
- Routed received blocks through `ManagedChainstate` and received transactions
  through `ManagedMempool`, keeping sync and relay behavior explicit and
  reusable.
- Added node-level tests proving `wtxidrelay` request selection and in-memory
  multi-node block or tx flow through the managed adapter.
