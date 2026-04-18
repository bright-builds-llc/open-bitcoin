---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 07-2026-04-18T00-33-56
generated_at: 2026-04-18T00:33:56Z
---

# Phase 7: Wallet Core and Adapters - Context

**Gathered:** 2026-04-17  
**Status:** Ready for planning and execution  
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 7 owns the first headless wallet slice: single-key descriptor handling,
address derivation, balance and UTXO tracking from chainstate snapshots,
deterministic transaction building, signing, and adapter-owned recovery.
Ranged descriptors, xpub/xprv derivation, miniscript, multisig, PSBT, wallet
encryption, external signers, rescans against a real node process, and RPC or
CLI surfaces stay out of scope for this phase.

</domain>

<decisions>
## Implementation Decisions

### Wallet boundary
- **D-01:** Add a dedicated pure-core `open-bitcoin-wallet` crate and re-export
  it through `open-bitcoin-core` rather than hiding wallet behavior in
  `open-bitcoin-node`.
- **D-02:** Keep persistence adapter-owned in `open-bitcoin-node` via a managed
  wrapper that loads and saves wallet snapshots without introducing direct I/O
  into the pure core.

### Descriptor and address scope
- **D-03:** The first descriptor slice covers single-key `pkh(...)`,
  `sh(wpkh(...))`, `wpkh(...)`, and `tr(...)` descriptors with direct private
  or public key material and optional checksum suffixes.
- **D-04:** Descriptor ranges (`*`), xpub/xprv derivation, miniscript,
  multisig, `combo`, and other descriptor families are explicitly deferred to
  later wallet or interface phases.

### Wallet state model
- **D-05:** Wallet balances and spendable UTXOs are derived from
  `ChainstateSnapshot` rather than by embedding chainstate or persistence logic
  into the wallet core.
- **D-06:** Coin selection stays deterministic and explainable: order inputs by
  effective value, require an internal change descriptor when the leftover is a
  real change output, and drop dust-sized change back into the fee.

### Signing and verification
- **D-07:** Signing should reuse the existing consensus sighash helpers and the
  canonical script verifier rather than inventing a second wallet-specific
  script engine.
- **D-08:** Phase 7 must prove legacy, nested segwit, native segwit, and
  taproot key-path coverage through repo-owned tests, plus upstream-derived
  address fixtures for the first descriptor surface.

</decisions>

<specifics>
## Specific Ideas

- Use WIF or raw public-key inputs for the first descriptor parser so the pure
  core does not need randomness, seed generation, or HD derivation yet.
- Persist wallet snapshots through a store trait in `open-bitcoin-node` in the
  same style as managed chainstate.
- Promote the wallet parity surface to `done` only if the parity doc calls out
  the intentionally deferred wallet features instead of implying full Bitcoin
  Core wallet coverage.

</specifics>

<canonical_refs>
## Canonical References

### Project and phase scope
- `.planning/PROJECT.md`
- `.planning/REQUIREMENTS.md` — `WAL-01`, `WAL-02`, `WAL-03`
- `.planning/ROADMAP.md` § Phase 7
- `.planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md`
- `.planning/phases/05-mempool-and-node-policy/05-CONTEXT.md`

### Existing Rust implementation
- `packages/open-bitcoin-consensus/src/sighash.rs`
- `packages/open-bitcoin-consensus/src/crypto.rs`
- `packages/open-bitcoin-consensus/src/context.rs`
- `packages/open-bitcoin-mempool/src/policy.rs`
- `packages/open-bitcoin-node/src/chainstate.rs`

### Knots wallet baseline
- `packages/bitcoin-knots/doc/descriptors.md`
- `packages/bitcoin-knots/doc/psbt.md`
- `packages/bitcoin-knots/test/functional/rpc_deriveaddresses.py`
- `packages/bitcoin-knots/test/functional/wallet_descriptor.py`
- `packages/bitcoin-knots/test/functional/feature_segwit.py`
- `packages/bitcoin-knots/test/functional/wallet_gethdkeys.py`
- `packages/bitcoin-knots/src/wallet/spend.cpp`
- `packages/bitcoin-knots/src/wallet/coinselection.cpp`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `open-bitcoin-consensus` already exposes legacy, segwit-v0, and taproot
  sighash helpers plus canonical script verification.
- `open-bitcoin-mempool` already exposes fee-rate helpers, dust thresholds, and
  transaction weight or virtual-size calculation.
- `ManagedChainstate` already demonstrates the adapter pattern this phase
  should mirror for wallet persistence and recovery.

### Established Patterns
- New pure-core subsystems land as dedicated crates under `packages/`.
- `open-bitcoin-core` re-exports new pure-core crates for downstream users.
- Shell crates own persistence or orchestration while pure-core crates own
  deterministic domain state and validation.

</code_context>

<deferred>
## Deferred Ideas

- ranged descriptors and HD key derivation
- xpub/xprv parsing and descriptor import parity
- miniscript or multisig descriptors
- PSBT construction and update flows
- wallet encryption, keypool behavior, and backup file formats
- external signer integration

</deferred>

---

*Phase: 07-wallet-core-and-adapters*  
*Context gathered: 2026-04-17*
