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

## Phase 70 branch and reorg recovery claim

Phase 70 keeps branch replacement deterministic by selecting candidate branches
by cumulative work, then height, then hash for the stable final tie-breaker.
The sync runtime waits for replacement branch block bodies before changing the
active chain, reuses `Chainstate::reorg` through the managed chainstate adapter,
persists the resulting active-chain snapshot, and exposes bounded latest
evidence through `sync.latest_reorg`.

That bounded latest evidence includes common ancestor height/hash, disconnected
count, connected count, final active height/hash, and whether the transition was
fully persisted. Missing active-chain block bodies, missing undo data, malformed
stored chainstate, or storage persistence failures remain storage recovery
blockers rather than peer retry claims.

## Phase 71 resource and storage-pressure claim

Phase 71 extends the local restart/resume evidence with storage-pressure
classification. Low-disk backend failures surface through
`StorageRecoveryAction::FreeDisk`, map to
`SyncRecoveryCategory::ResourceExhaustion`, and tell the operator:
`Free disk space for the selected datadir, then retry sync.` The claim remains
diagnostic and bounded; it does not add automatic chainstate repair,
block serving, production-funds wallet claims, migration apply mode, signed
packaging, Windows service support, GUI, hosted dashboards, or broad
production-node readiness.

## Phase 72 active-chain evidence claim

Phase 72 adds observability/support evidence only. Connected and validated
active-chain height, hash, and work now flow into operator status, support
evidence, live-smoke summaries, metrics, and structured logs so reviewers can
distinguish downloaded block bodies from durably persisted active-chain
progress.

This evidence does not add inbound serving, address relay, block serving,
transaction relay, compact block relay, production-funds wallet claims,
migration apply mode, signed packaging, Windows service support, GUI, hosted
dashboards, or broad production-node readiness.

## VER-02 deterministic coverage map

Phase 73 makes the deterministic chainstate coverage boundary auditable through
existing local tests and checker anchors. The VER-02 map covers UTXO/undo
persistence, block connect/disconnect/reorg across restart, best-chain header
selection, peer response failures, crash recovery as durable reopen, duplicate
connect prevention, and resource bounds.

This coverage map is local verification evidence only. It does not add block
serving, transaction relay, compact block relay, production-funds wallet
claims, migration apply mode, signed packaging, Windows service support, GUI,
hosted dashboards, or broad production-node readiness.

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
