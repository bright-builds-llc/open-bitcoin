---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 103-2026-07-01T12-38-00
generated_at: 2026-07-01T12:38:00.304Z
status: complete
---

# Phase 103: Mempool Chainstate Lifecycle and Durable Recovery - Research

## RESEARCH COMPLETE

Phase 103 should be planned as four connected slices: pure mempool lifecycle, managed chainstate/runtime integration, durable mempool storage, and parity/verification evidence. The high-risk part is keeping mempool graph/index cleanup, relay caches, orphan/request state, and durable recovery coherent without pulling I/O into pure mempool code.

## Scope Anchors

Phase 103 owns:

- **MEM-03:** truthful pressure, trimming, fee-floor, and capacity evidence, including explicit deferred Knots gaps.
- **MEM-04:** block-connect removal of confirmed and conflicting transactions from mempool plus relay-serving caches.
- **MEM-05:** bounded block-disconnect or reorg reconsideration of eligible disconnected transactions.
- **MEM-06:** durable accepted-mempool persistence and safe restart/recovery behavior.

It must not implement Phase 104 relay serving/fanout/rebroadcast, Phase 105 broad RPC/operator/support observability, or Phase 106 final release-boundary closeout except where narrow docs/checkers are required for truthfulness.

## Existing Open Bitcoin Seams

### Pure Mempool

- `packages/open-bitcoin-mempool/src/pool.rs` owns `Mempool`, `entries`, `spent_outpoints`, replacement sets, limit validation, trimming, and `recompute_state`.
- `Mempool::accept_transaction` already clones prospective entries, removes replacement conflicts, inserts the candidate, recomputes state, validates limits, and trims to size before mutating the live pool.
- `trim_to_size` already removes the lowest descendant-score package and returns evicted txids.
- `MempoolOutcome` in `packages/open-bitcoin-mempool/src/outcome.rs` already provides fixed labels for accepted, rejected, duplicate, replaced, orphaned, evicted, and expired states.
- Current tests cover admission, duplicate, missing input, standardness, fee, RBF, ancestor/descendant, trimming, and no-partial-mutation behavior.

Planning implication: add lifecycle methods near `pool.rs`, likely in a child file such as `pool/lifecycle.rs`, so cleanup can reuse `recompute_state` rather than duplicating graph/index mutation.

### Managed Runtime

- `packages/open-bitcoin-node/src/network.rs` owns `ManagedPeerNetwork`, `ManagedMempool`, `PeerManager`, `TxOrphanage`, block caches, and in-memory `transactions_by_txid` / `transactions_by_wtxid`.
- `connect_local_block`, `connect_stored_block`, and `reorg_to_branch` are the natural shell hooks after successful chainstate mutation.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` already maps peer/local transactions through `MempoolOutcome`, orphan staging, parent requests, and request cleanup.
- `packages/open-bitcoin-node/src/network/action_translation.rs` already cleans transaction request state on peer disconnect and translates scheduler actions into targeted `getdata`.

Planning implication: managed cleanup should call pure mempool lifecycle APIs and then remove stored txid/wtxid cache entries plus request/orphan facts. Do not put chainstate or storage dependencies inside `open-bitcoin-mempool`.

### Durable Storage

- `packages/open-bitcoin-node/src/storage.rs` defines `StorageNamespace`, `SchemaVersion`, `PersistMode`, `StorageError`, `StorageRecoveryAction`, and `RecoveryMarker`.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` opens one Fjall database with distinct keyspaces for headers, block index, chainstate, wallet, metrics, runtime, and schema.
- `snapshot_codec.rs` encodes versioned JSON snapshots and maps schema mismatch/corruption into typed `StorageError` values.
- Fjall tests already cover schema mismatch, recovery marker corruption, interrupted writes, runtime metadata, and chainstate corruption.

Planning implication: add a dedicated mempool namespace and versioned codec. Keep remove APIs explicit so confirmed, stale, corrupt, or incompatible records can be repaired/dropped with evidence.

## Knots Anchors To Follow Or Deliberately Defer

- `packages/bitcoin-knots/src/txmempool.h`:
  - `removeRecursive`
  - `removeForReorg`
  - `removeConflicts`
  - `removeForBlock`
  - `UpdateTransactionsFromBlock`
  - `TrimToSize`
  - `Expire`
  - `rollingMinimumFeeRate`
- `packages/bitcoin-knots/src/txmempool.cpp`:
  - `removeRecursive` removes transactions and descendants for block/conflict/reorg/sizelimit reasons.
  - `removeForBlock` removes in-block transactions, clears conflicts, and updates descendant links.
  - `UpdateTransactionsFromBlock` fixes mempool descendant state after disconnected transactions are re-added.
  - `TrimToSize` drives pressure eviction and rolling minimum fee behavior.
- `packages/bitcoin-knots/src/validation.cpp`:
  - `MaybeUpdateMempoolForReorg` re-adds disconnected transactions, removes recursive failures, calls `UpdateTransactionsFromBlock`, and then `removeForReorg`.
  - `DisconnectTip` adds disconnected transactions into `DisconnectedBlockTransactions`.
  - `ConnectTip` calls `mempool.removeForBlock(...)` and `disconnectpool.removeForBlock(...)`.
- `packages/bitcoin-knots/src/kernel/disconnected_transactions.h/.cpp`:
  - bounded memory pool for disconnected block transactions.
  - `AddTransactionsFromBlock`, `removeForBlock`, `LimitMemoryUsage`, and `take`.
- `packages/bitcoin-knots/src/node/mempool_persist.cpp` and `.h`:
  - `LoadMempool` and `DumpMempool`.
  - corrupt or incompatible mempool data can be logged and skipped rather than blocking node start.
- Functional tests:
  - `test/functional/mempool_persist.py`
  - `test/functional/mempool_compatibility.py`
  - `test/functional/mempool_limit.py`
  - `test/functional/mempool_reorg.py`
  - `test/functional/mempool_accept.py`

Important parity gap likely to document: Open Bitcoin currently has a simple min relay fee plus size trim; full Knots rolling minimum fee decay and full mempool.dat compatibility are not necessary unless implemented and tested in this phase.

## Recommended Plan Shape

### Plan 01: Pure Mempool Lifecycle And Pressure Contract

Build pure APIs for:

- block-connected transaction removal.
- conflict and descendant cleanup.
- lifecycle summary labels for confirmed, conflict, descendant, evicted, reconsidered, rejected, stale, and deferred parity.
- pressure/capacity summary from `PolicyConfig`, virtual size, total fee, evictions, and fee floor.

Tests:

- confirmed tx is removed.
- conflict spender is removed.
- descendants are removed or coherently recomputed.
- `spent_outpoints`, parents/children, ancestor/descendant stats, and total virtual size remain coherent.
- pressure summary distinguishes implemented trim behavior from deferred rolling fee parity.

### Plan 02: Managed Chainstate Hooks And Reorg Reconsideration

Wire lifecycle APIs into:

- `connect_local_block`
- `connect_stored_block`
- `reorg_to_branch`
- transaction cache cleanup in `ManagedPeerNetwork`
- Phase 102 orphan/reconsideration bridge where useful.

Tests:

- block connect removes confirmed mempool tx from pure mempool and txid/wtxid caches.
- block connect removes conflicting tx and descendants.
- reorg/disconnect reconsideration accepts eligible disconnected transactions.
- reorg/disconnect records rejected/orphaned/evicted outcomes without unbounded recursion.
- request/orphan state remains coherent after cleanup.

### Plan 03: Durable Mempool Storage And Recovery

Add:

- `StorageNamespace::Mempool`.
- versioned mempool snapshot/record codec.
- `FjallNodeStore::{save,load,remove}_mempool...` style APIs.
- restart reconstruction path for accepted txs.
- stale/confirmed/policy-incompatible/corrupt/schema-mismatch tests.

Tests:

- save/load accepted mempool state.
- remove clears a confirmed/evicted tx record.
- reopen recovers accepted records.
- schema mismatch returns a typed `StorageError::SchemaMismatch`.
- corrupt bytes return `StorageError::Corruption`.
- stale records are dropped or repaired with typed evidence rather than silently ignored.

### Plan 04: Parity Docs And Deterministic Checker

If implementation changes docs/parity roots, add:

- Phase 103 parity catalog/index/checklist entries.
- deterministic `scripts/check-phase103-mempool-lifecycle.ts`.
- fixture tests that fail on missing MEM-03..MEM-06 evidence, missing key roots, missing verifier wiring, or overclaims.
- verifier wiring after Phase 102.

Checker should explicitly allow bounded Phase 103 lifecycle claims while rejecting Phase 104+ relay serving/fanout/rebroadcast, compact block relay, package relay, bloom/filter serving, public relay defaults, public-network CI, production readiness, and production-funds wallet claims.

## Risks And Mitigations

- **Graph corruption risk:** removing a tx without descendants or recomputing state can leave parent/child stats and spent outpoints stale. Mitigation: use one pure removal pipeline that returns a new recomputed state and test graph indexes after each path.
- **Scope creep risk:** durable mempool persistence can drift into full `mempool.dat` compatibility or operator repair UI. Mitigation: store Open Bitcoin-owned records only and document Knots compatibility gaps.
- **False parity risk:** exposing `min_relay_feerate` as pressure evidence could imply full rolling minimum fee behavior. Mitigation: add explicit `rolling_fee_floor_parity` or deferred-gap wording until implemented.
- **Adapter leakage risk:** chainstate or Fjall types could leak into `open-bitcoin-mempool`. Mitigation: pure mempool takes transactions/facts and returns summaries; managed node owns chainstate/storage calls.
- **Verification runtime risk:** full repo verification is already heavy. Mitigation: use targeted Cargo checks during implementation but close with repo-native `bash scripts/verify.sh`.

## Verification Recommendations

- Unit tests in `open-bitcoin-mempool` for lifecycle removal, pressure summary, and graph invariants.
- Managed-network tests for block connect, conflict cleanup, reorg reconsideration, cache cleanup, and orphan/request coherence.
- Fjall tests for durable mempool save/load/remove/reopen/schema/corruption/stale behavior.
- Parity checker tests if docs/checker roots are updated.
- Final default verification: `bash scripts/verify.sh`.

## Open Questions For Planner

- Whether to persist a full mempool snapshot or per-transaction records. Per-transaction records make remove/repair easier; snapshots may be simpler. Prefer whichever keeps tests clearer and avoids overlarge rewrites.
- Whether reorg reconsideration should live in `ManagedPeerNetwork` or a small child module. Prefer a child module if `network.rs` grows.
- Whether MEM-03 needs an operator-visible status field in Phase 103 or just shared typed runtime evidence. Add a narrow field only if needed to avoid a misleading current operator surface.
