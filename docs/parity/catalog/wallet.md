# Wallet Core And Adapters

This entry tracks the Phase 7 wallet slice implemented in Open Bitcoin. The
behavioral baseline remains Bitcoin Knots `29.3.knots20260210`.

## Coverage

- single-key descriptor parsing for `pkh(...)`, `sh(wpkh(...))`,
  `wpkh(...)`, and `tr(...)`
- optional descriptor checksum suffixes
- WIF private-key parsing plus public-key or x-only public-key descriptor
  inputs
- Base58Check legacy addresses and segwit or taproot bech32 or bech32m address
  derivation
- wallet snapshots, UTXO tracking, and balance views rebuilt from
  `ChainstateSnapshot`
- deterministic effective-value input ordering, change handling, and dust
  folding during transaction construction
- legacy, nested segwit, native segwit, and taproot key-path signing through
  the canonical consensus sighash helpers
- managed wallet snapshot persistence through `open-bitcoin-node`

## Knots sources

- [`packages/bitcoin-knots/doc/descriptors.md`](../../../packages/bitcoin-knots/doc/descriptors.md)
- [`packages/bitcoin-knots/test/functional/rpc_deriveaddresses.py`](../../../packages/bitcoin-knots/test/functional/rpc_deriveaddresses.py)
- [`packages/bitcoin-knots/test/functional/wallet_descriptor.py`](../../../packages/bitcoin-knots/test/functional/wallet_descriptor.py)
- [`packages/bitcoin-knots/test/functional/feature_segwit.py`](../../../packages/bitcoin-knots/test/functional/feature_segwit.py)
- [`packages/bitcoin-knots/test/functional/wallet_gethdkeys.py`](../../../packages/bitcoin-knots/test/functional/wallet_gethdkeys.py)
- [`packages/bitcoin-knots/src/wallet/spend.cpp`](../../../packages/bitcoin-knots/src/wallet/spend.cpp)
- [`packages/bitcoin-knots/src/wallet/coinselection.cpp`](../../../packages/bitcoin-knots/src/wallet/coinselection.cpp)

## Knots behaviors mirrored here

- descriptor addresses for the first single-key wallet surface derive to the
  same legacy, nested segwit, native segwit, and taproot outputs on the
  vendored fixture keys
- wallet signing reuses the same legacy, segwit-v0, and taproot key-path
  sighash semantics as the consensus engine
- change remains explicit through dedicated internal descriptors instead of
  silently reusing receive outputs
- persistence and recovery stay adapter-owned instead of leaking direct I/O
  into the wallet core

## First-party implementation

- [`packages/open-bitcoin-wallet/src/address.rs`](../../../packages/open-bitcoin-wallet/src/address.rs)
- [`packages/open-bitcoin-wallet/src/descriptor.rs`](../../../packages/open-bitcoin-wallet/src/descriptor.rs)
- [`packages/open-bitcoin-wallet/src/wallet.rs`](../../../packages/open-bitcoin-wallet/src/wallet.rs)
- [`packages/open-bitcoin-node/src/wallet.rs`](../../../packages/open-bitcoin-node/src/wallet.rs)

## Known gaps

- ranged descriptors and HD xpub/xprv derivation
- miniscript, multisig, and PSBT flows
- wallet encryption, backup file formats, and migration behavior
- external signer integration
- real-node functional rescans and RPC-facing wallet semantics

## Follow-up triggers

Update this entry when later phases add descriptor ranges, PSBT flows, wallet
RPC or CLI commands, external signers, or broader coin-selection parity.
