---
phase: 07-wallet-core-and-adapters
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 07-2026-04-18T00-33-56
generated_at: 2026-04-18T00:33:56Z
---

# Phase 7: Wallet Core and Adapters - Research

**Researched:** 2026-04-17  
**Domain:** descriptor wallets, address derivation, spend construction, signing,
and adapter-owned wallet recovery  
**Confidence:** HIGH

## Summary

- Phase 7 should land as a new pure-core `open-bitcoin-wallet` crate plus a
  managed snapshot store in `open-bitcoin-node`, following the same boundary
  pattern already used for chainstate. [VERIFIED: `.planning/PROJECT.md`,
  `packages/open-bitcoin-node/src/chainstate.rs`]
- The first descriptor slice can stay narrow and still satisfy the roadmap:
  single-key `pkh`, `sh(wpkh)`, `wpkh`, and `tr` descriptors with direct WIF or
  public-key inputs, optional checksum suffixes, and explicit deferral of HD,
  multisig, miniscript, and PSBT behavior. [VERIFIED:
  `packages/bitcoin-knots/doc/descriptors.md`,
  `packages/bitcoin-knots/test/functional/wallet_descriptor.py`,
  `.planning/phases/07-wallet-core-and-adapters/07-CONTEXT.md`]
- Reusing `legacy_sighash`, `segwit_v0_sighash`, `taproot_sighash`,
  `TransactionValidationContext`, and the canonical verifier makes wallet
  signatures auditable and keeps the signing logic aligned with consensus.
  [VERIFIED: `packages/open-bitcoin-consensus/src/sighash.rs`,
  `packages/open-bitcoin-consensus/src/context.rs`]
- `FeeRate`, `dust_threshold_sats`, and transaction virtual-size helpers from
  `open-bitcoin-mempool` are sufficient for deterministic first-pass coin
  selection without porting Knots' full BnB or knapsack machinery yet. That
  broader algorithmic surface should stay documented as deferred parity work.
  [VERIFIED: `packages/open-bitcoin-mempool/src/types.rs`,
  `packages/open-bitcoin-mempool/src/policy.rs`,
  `packages/bitcoin-knots/src/wallet/coinselection.cpp`]

## Minimum Phase Truths

1. The workspace exposes a pure-core wallet crate that owns descriptor parsing,
   address derivation, wallet balances, UTXO tracking, and transaction signing.
2. Managed wallet recovery loads and saves snapshots through an adapter-owned
   store in `open-bitcoin-node`.
3. Repo-owned tests prove legacy, nested segwit, native segwit, and taproot
   key-path coverage, and parity docs mark the wallet surface done with
   explicit deferrals.

## Known Deferrals

- xpub/xprv and ranged descriptor imports
- multisig and miniscript descriptors
- PSBT flows and external signer support
- wallet encryption, backup, and on-disk file compatibility

---

*Phase: 07-wallet-core-and-adapters*  
*Research captured: 2026-04-17*
