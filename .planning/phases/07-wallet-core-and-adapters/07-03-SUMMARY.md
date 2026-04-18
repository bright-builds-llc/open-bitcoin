---
phase: 07-wallet-core-and-adapters
plan: 03
subsystem: signing-and-managed-recovery
tags: [wallet, signing, recovery, node]
provides:
  - legacy, segwit, and taproot wallet signing
  - managed wallet snapshot store
  - adapter-owned wallet recovery surface
affects: [wallet, node]
requirements_completed: [WAL-02, WAL-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 07-2026-04-18T00-33-56
generated_at: 2026-04-18T00:33:56Z
completed: 2026-04-17
---

# Phase 7 Plan 03 Summary

Closed the wallet core or shell boundary with signing and managed snapshot
recovery.

## Accomplishments

- Reused consensus sighash helpers to sign legacy, nested segwit, native
  segwit, and taproot key-path spends inside the wallet core.
- Added `WalletStore`, `MemoryWalletStore`, and `ManagedWallet` in
  `open-bitcoin-node` so persistence stays adapter-owned.
- Added tests proving managed snapshot persistence and the remaining signing
  descriptor paths.
