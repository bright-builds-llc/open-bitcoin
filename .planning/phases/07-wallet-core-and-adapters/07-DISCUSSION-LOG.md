---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 07-2026-04-18T00-33-56
generated_at: 2026-04-18T00:33:56Z
---

# Phase 7: Wallet Core and Adapters - Discussion Log

**Mode:** Yolo  
**Completed:** 2026-04-17

## Auto-Selected Gray Areas

1. Pure-core versus shell boundary
2. Descriptor families included in the first wallet slice
3. Wallet state source of truth for balances and UTXOs
4. Deterministic coin-selection and change behavior
5. Signing surface and parity verification strategy

## Synthesized Decisions

### 1. Pure-core versus shell boundary
- Auto-selected: a pure-core `open-bitcoin-wallet` crate plus a managed wallet
  adapter in `open-bitcoin-node`
- Rationale: matches the repo's functional-core boundary and keeps persistence
  concerns outside the wallet domain logic

### 2. Descriptor families included in the first wallet slice
- Auto-selected: single-key `pkh`, `sh(wpkh)`, `wpkh`, and `tr`
- Rationale: this covers the first headless wallet surface with address,
  transaction-building, and signing behavior while avoiding premature HD or
  miniscript complexity

### 3. Wallet state source of truth
- Auto-selected: derive wallet UTXOs and balances from `ChainstateSnapshot`
- Rationale: keeps the wallet core deterministic and adapter-friendly instead
  of coupling it directly to storage or node runtime state

### 4. Deterministic coin-selection and change behavior
- Auto-selected: effective-value ordering with explicit internal change
  descriptors and dust-sized change folded into the fee
- Rationale: simple, auditable, and sufficient for the targeted fixture surface

### 5. Signing surface and verification strategy
- Auto-selected: reuse consensus sighash helpers and canonical script
  verification, then anchor address fixtures to vendored Knots references
- Rationale: avoids a second signing model and keeps parity evidence honest

## Deferred Ideas Captured

- xpub/xprv and ranged descriptors
- multisig, miniscript, and PSBT flows
- encryption, backup files, and external signer support

---

*Phase: 07-wallet-core-and-adapters*  
*Discussion logged: 2026-04-17*
