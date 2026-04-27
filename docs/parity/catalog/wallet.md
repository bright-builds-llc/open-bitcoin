# Wallet Core And Adapters

This entry tracks the shipped wallet slice through Phase 20. The behavioral
baseline remains Bitcoin Knots `29.3.knots20260210`.

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
- durable named-wallet registry selection for the current wallet subset
- one active external and one active internal ranged single-key descriptor per
  wallet, including persisted `range` and `next_index` state
- wallet-scoped RPC or CLI routing through `-rpcwallet` and `/wallet/<name>` for
  the supported method subset
- `sendtoaddress`-style wallet sends for the current slice, plus the
  Open Bitcoin-owned operator preview or confirm wrapper under `open-bitcoin
  wallet send`
- operator-visible wallet freshness reporting through the shared status snapshot
- resumable rescan progress and freshness metadata in `getwalletinfo` and the
  shared status surface
- Open Bitcoin-owned managed-wallet backup exports that stay one-way and reject
  detected external wallet destinations
- read-only external wallet inspection with wallet-format, chain-scope, and
  product-confidence hints for backup and migration planning

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
- the operator send flow uses a preview and explicit confirmation wrapper, but
  the final mutation still goes through the wallet-scoped `sendtoaddress` path
- wallet-scoped selection follows the `/wallet/<name>` or `-rpcwallet` pattern
  for the implemented subset without claiming full `loadwallet` parity
- wallet operators can inspect existing Core/Knots wallet candidates without
  mutating wallet files, metadata, permissions, or timestamps
- backup export is deliberately Open Bitcoin-owned JSON rather than a
  Core-compatible `wallet.dat` copy

## First-party implementation

- [`packages/open-bitcoin-wallet/src/address.rs`](../../../packages/open-bitcoin-wallet/src/address.rs)
- [`packages/open-bitcoin-wallet/src/descriptor.rs`](../../../packages/open-bitcoin-wallet/src/descriptor.rs)
- [`packages/open-bitcoin-wallet/src/wallet.rs`](../../../packages/open-bitcoin-wallet/src/wallet.rs)
- [`packages/open-bitcoin-node/src/wallet.rs`](../../../packages/open-bitcoin-node/src/wallet.rs)
- [`packages/open-bitcoin-node/src/wallet_registry.rs`](../../../packages/open-bitcoin-node/src/wallet_registry.rs)
- [`packages/open-bitcoin-rpc/src/dispatch.rs`](../../../packages/open-bitcoin-rpc/src/dispatch.rs)
- [`packages/open-bitcoin-cli/src/operator/wallet.rs`](../../../packages/open-bitcoin-cli/src/operator/wallet.rs)

## Known gaps

- miniscript, multisig, and PSBT flows
- wallet encryption and restore or import compatibility with external wallet
  formats
- external signer integration
- full multiwallet lifecycle parity such as `loadwallet`, `unloadwallet`, and
  `listwallets`
- richer `send` RPC ergonomics beyond the shipped `sendtoaddress`-style path
- Phase 21 still owns any external-wallet mutation, backup export, restore,
  import, copy, or migration execution flow

## Follow-up triggers

Update this entry when later phases add wallet lifecycle parity, richer send
surfaces, PSBT flows, external signers, restore or import behavior, or broader
coin-selection parity.
