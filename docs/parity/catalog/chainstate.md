# Chainstate And UTXO Engine

This entry tracks the Phase 4 chainstate slice implemented in Open Bitcoin.
The behavioral baseline remains Bitcoin Knots `29.3.knots20260210`.

## Coverage

- explicit UTXO entries carrying output, coinbase, creation-height, and
  creation-median-time-past metadata
- pure-core active-chain snapshots and per-block undo payloads
- direct block connect using the existing consensus validators plus derived
  spend contexts from the current UTXO view
- direct tip disconnect that removes created outputs, restores spent inputs in
  reverse order, and rewinds the active tip
- explicit reorg application over disconnect and reconnect paths
- deterministic best-tip preference by cumulative work, then height, then block
  hash for repo-owned fixtures
- node-side in-memory snapshot persistence that keeps storage outside the pure
  chainstate core

## Knots sources

- [`packages/bitcoin-knots/src/coins.h`](../../../packages/bitcoin-knots/src/coins.h)
- [`packages/bitcoin-knots/src/coins.cpp`](../../../packages/bitcoin-knots/src/coins.cpp)
- [`packages/bitcoin-knots/src/validation.cpp`](../../../packages/bitcoin-knots/src/validation.cpp)
- [`packages/bitcoin-knots/src/node/blockstorage.cpp`](../../../packages/bitcoin-knots/src/node/blockstorage.cpp)

## Knots behaviors mirrored here

- unspendable outputs do not enter the spendable UTXO view
- connect spends inputs before it adds outputs at the connected height
- disconnect removes created outputs before replaying undo in reverse order
- connect rejects BIP30-style output overwrites instead of silently replacing
  live coins
- best-chain preference is work-first even though Open Bitcoin uses a stable
  hash tie-break for deterministic fixtures instead of Knots' pointer-identity
  fallback

## First-party implementation

- [`packages/open-bitcoin-chainstate/src/engine.rs`](../../../packages/open-bitcoin-chainstate/src/engine.rs)
- [`packages/open-bitcoin-chainstate/src/types.rs`](../../../packages/open-bitcoin-chainstate/src/types.rs)
- [`packages/open-bitcoin-chainstate/tests/parity.rs`](../../../packages/open-bitcoin-chainstate/tests/parity.rs)
- [`packages/open-bitcoin-node/src/chainstate.rs`](../../../packages/open-bitcoin-node/src/chainstate.rs)

## Known gaps

- disk-backed coins databases, cache-flush policy, and assumeutxo flows
- mempool repair and disconnected-transaction pools during reorg
- header-chain validation and full node chainstate-manager behavior beyond this
  phase's active-chain slice

## Follow-up triggers

Update this entry when later phases add mempool-coupled spend views,
header-chain work calculation, or disk-backed persistence that materially
changes the external chainstate behavior.
