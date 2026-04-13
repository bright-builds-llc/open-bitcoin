# Mempool Policy

This entry tracks the Phase 5 mempool and node-policy slice implemented in
Open Bitcoin. The behavioral baseline remains Bitcoin Knots
`29.3.knots20260210`.

## Coverage

- pure-core mempool entry state with txid/wtxid identity, fee, virtual size,
  and explicit parent or child relationships
- admission against the active chainstate snapshot plus already-accepted
  mempool parents
- standardness checks for relay-fee, weight, scriptSig push-only behavior,
  non-standard script forms, and dust thresholds
- conflict detection plus targeted RBF replacement requiring higher absolute
  fee, higher feerate, and an incremental relay bump
- deterministic ancestor or descendant accounting and limit enforcement
- size-limit trimming that removes the lowest descendant-score package
- node-side managed wrapper that feeds chainstate snapshots into the pure-core
  mempool engine

## Knots sources

- [`packages/bitcoin-knots/src/txmempool.h`](../../../packages/bitcoin-knots/src/txmempool.h)
- [`packages/bitcoin-knots/src/txmempool.cpp`](../../../packages/bitcoin-knots/src/txmempool.cpp)
- [`packages/bitcoin-knots/src/policy/policy.h`](../../../packages/bitcoin-knots/src/policy/policy.h)
- [`packages/bitcoin-knots/src/policy/rbf.h`](../../../packages/bitcoin-knots/src/policy/rbf.h)
- [`packages/bitcoin-knots/src/test/rbf_tests.cpp`](../../../packages/bitcoin-knots/src/test/rbf_tests.cpp)
- [`packages/bitcoin-knots/src/test/txpackage_tests.cpp`](../../../packages/bitcoin-knots/src/test/txpackage_tests.cpp)

## Knots behaviors mirrored here

- relay policy extends the existing consensus validator rather than duplicating
  fee, lock-time, or maturity rules
- non-standard outputs and underpriced transactions fail admission before the
  mempool mutates
- conflicts can replace existing transactions only when the configured RBF
  policy and fee-bump rules are satisfied
- ancestor or descendant metrics are visible through entry state and drive
  deterministic limit checks
- size-limit trimming removes the weakest descendant-score package instead of
  silently allowing unbounded growth

## First-party implementation

- [`packages/open-bitcoin-mempool/src/pool.rs`](../../../packages/open-bitcoin-mempool/src/pool.rs)
- [`packages/open-bitcoin-mempool/src/policy.rs`](../../../packages/open-bitcoin-mempool/src/policy.rs)
- [`packages/open-bitcoin-mempool/src/types.rs`](../../../packages/open-bitcoin-mempool/src/types.rs)
- [`packages/open-bitcoin-mempool/tests/parity.rs`](../../../packages/open-bitcoin-mempool/tests/parity.rs)
- [`packages/open-bitcoin-node/src/mempool.rs`](../../../packages/open-bitcoin-node/src/mempool.rs)

## Known gaps

- package relay beyond single-transaction admission
- rolling minimum-fee decay and long-lived relay-fee state
- reorg-driven mempool repair and disconnected-transaction staging
- networking, RPC, and CLI surfaces over the mempool

## Follow-up triggers

Update this entry when later phases add package relay, dynamic rolling-min-fee
behavior, reorg repair, or operator-facing mempool interfaces that materially
change the externally visible policy surface.
