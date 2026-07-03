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
- typed pressure evidence for transaction count, virtual size, configured
  capacity, relay fee floors, capacity status, and deferred rolling-fee parity
- pure block-connect lifecycle cleanup for confirmed transactions, conflicts,
  and conflict descendants
- managed node cleanup of mempool state and txid/wtxid runtime caches after
  successful block connect
- bounded managed reorg reconsideration of disconnected non-coinbase
  transactions through `MempoolOutcome`
- Open Bitcoin-owned durable accepted-mempool snapshot storage and typed
  recovery replay evidence
- node-side managed wrapper that feeds chainstate snapshots into the pure-core
  mempool engine

## Knots sources

- [`packages/bitcoin-knots/src/txmempool.h`](../../../packages/bitcoin-knots/src/txmempool.h)
- [`packages/bitcoin-knots/src/txmempool.cpp`](../../../packages/bitcoin-knots/src/txmempool.cpp)
- [`packages/bitcoin-knots/src/policy/policy.h`](../../../packages/bitcoin-knots/src/policy/policy.h)
- [`packages/bitcoin-knots/src/policy/rbf.h`](../../../packages/bitcoin-knots/src/policy/rbf.h)
- [`packages/bitcoin-knots/src/validation.cpp`](../../../packages/bitcoin-knots/src/validation.cpp)
- [`packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp`](../../../packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp)
- [`packages/bitcoin-knots/src/node/mempool_persist.cpp`](../../../packages/bitcoin-knots/src/node/mempool_persist.cpp)
- [`packages/bitcoin-knots/src/test/rbf_tests.cpp`](../../../packages/bitcoin-knots/src/test/rbf_tests.cpp)
- [`packages/bitcoin-knots/src/test/txpackage_tests.cpp`](../../../packages/bitcoin-knots/src/test/txpackage_tests.cpp)
- [`packages/bitcoin-knots/test/functional/mempool_limit.py`](../../../packages/bitcoin-knots/test/functional/mempool_limit.py)
- [`packages/bitcoin-knots/test/functional/mempool_reorg.py`](../../../packages/bitcoin-knots/test/functional/mempool_reorg.py)
- [`packages/bitcoin-knots/test/functional/mempool_persist.py`](../../../packages/bitcoin-knots/test/functional/mempool_persist.py)

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
- block-connect cleanup removes confirmed transactions and true conflicts while
  preserving valid descendants whose parents just confirmed
- managed reorg reconsideration replays disconnected non-coinbase transactions
  through the same typed outcome vocabulary used by admission and orphan
  handling
- durable mempool recovery uses typed recovered, confirmed-dropped,
  missing-parent, policy-incompatible, duplicate, and evicted evidence

## First-party implementation

- [`packages/open-bitcoin-mempool/src/pool.rs`](../../../packages/open-bitcoin-mempool/src/pool.rs)
- [`packages/open-bitcoin-mempool/src/pool/lifecycle.rs`](../../../packages/open-bitcoin-mempool/src/pool/lifecycle.rs)
- [`packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs`](../../../packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs)
- [`packages/open-bitcoin-mempool/src/policy.rs`](../../../packages/open-bitcoin-mempool/src/policy.rs)
- [`packages/open-bitcoin-mempool/src/types.rs`](../../../packages/open-bitcoin-mempool/src/types.rs)
- [`packages/open-bitcoin-mempool/tests/parity.rs`](../../../packages/open-bitcoin-mempool/tests/parity.rs)
- [`packages/open-bitcoin-node/src/mempool.rs`](../../../packages/open-bitcoin-node/src/mempool.rs)
- [`packages/open-bitcoin-node/src/network/mempool_lifecycle.rs`](../../../packages/open-bitcoin-node/src/network/mempool_lifecycle.rs)
- [`packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs`](../../../packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs)
- [`packages/open-bitcoin-node/src/storage/mempool_snapshot.rs`](../../../packages/open-bitcoin-node/src/storage/mempool_snapshot.rs)
- [`packages/open-bitcoin-node/src/storage/fjall_store.rs`](../../../packages/open-bitcoin-node/src/storage/fjall_store.rs)
- [`packages/open-bitcoin-node/src/storage/fjall_store/tests.rs`](../../../packages/open-bitcoin-node/src/storage/fjall_store/tests.rs)

## Phase 103 lifecycle and durable recovery

The `v2-0-mempool-chainstate-lifecycle-durable-recovery` surface covers
`MEM-03`, `MEM-04`, `MEM-05`, and `MEM-06`.

- `MempoolPressureSummary` exposes fixed capacity and fee-floor evidence while
  keeping full Knots rolling minimum fee decay explicitly deferred.
- `remove_for_connected_block` and `remove_for_connected_transactions` remove
  confirmed transactions, true conflicts, and conflict descendants through one
  recomputed pure mempool graph path.
- `ManagedPeerNetwork` applies lifecycle cleanup after successful block connect
  and clears txid/wtxid runtime caches for removed transactions.
- Managed reorg reconsideration is bounded to disconnected block transactions
  and returns typed `MempoolOutcome` values.
- `StorageNamespace::Mempool`, `MempoolSnapshot`, and the Fjall mempool
  snapshot APIs persist Open Bitcoin-owned accepted mempool records and replay
  them with typed recovery evidence.
- The Phase 103 checker and summaries keep this surface auditable through
  deterministic local verification.

## Phase 104 relay cache boundary

The `v2-0-relay-serving-fanout-rebroadcast-policy` surface extends the Phase
103 lifecycle work only where relay serving and fanout need coherent mempool
state. `RelayServingCache` and `ManagedRelayFanoutState` consume
`MempoolOutcome` and lifecycle cleanup evidence so accepted transactions can be
served or queued, while confirmed, replaced, evicted, and expired transactions
are removed from serving and fanout state. `LocalRelaySubmissionEvidence`
records `sendrawtransaction` outcomes with fixed labels such as `queued`,
`suppressed`, `relay_disabled`, `not_eligible`, and `rebroadcast_deferred`.

This Phase 104 bridge does not add periodic rebroadcast scheduling, compact
block relay, package relay, bloom/filter serving, public relay defaults,
internet-connected relay CI, Phase 105 operator/RPC/metrics/log/support
presentation, Phase 106 release-boundary closeout, production service
operation, production full-node readiness, or production-funds wallet use.

## Phase 105 operator relay evidence

The `v2-0-operator-rpc-metrics-logs-support-evidence` surface documents the
operator-facing mempool and relay evidence that Phase 105 projects from the
shared status contract. The mempool policy surface now exposes fixed aggregate
relay outcome counters through status, RPC extension status, metrics, logs, and
support bundles without adding new mempool acceptance rules.

The fixed counter vocabulary is `accepted_count`, `rejected_count`,
`orphaned_count`, `requested_count`, `served_count`, `announced_count`,
`suppressed_count`, `evicted_count`, `expired_count`, and
`rebroadcast_deferred_count`. Capability fields are classified as
`implemented`, `unavailable`, `deferred`, or `intentionally_different` so
operators can distinguish local mempool evidence from deferred public relay
readiness.

This Phase 105 bridge does not add Knots-complete mempool policy, compact block
relay, package relay, bloom/filter serving, public relay defaults,
public-network relay CI, production service operation, production full-node
readiness, or production-funds wallet use.

## Phase 106 release boundary guardrails

The `v2-0-parity-uat-release-boundary` surface closes the v2.0 mempool and
relay evidence boundary by linking the Phase 100 through Phase 105 evidence
roots to deterministic claim checks and repo-local UAT commands. It does not
change mempool admission, replacement, eviction, rolling-fee, or persistence
behavior.

The Phase 106 checker keeps the mempool-policy claim bounded to local relay and
mempool evidence. It rejects unsupported positive claims for compact block
relay, package relay, bloom/filter serving, public relay defaults,
public-network relay CI, production service operation, production full-node
readiness, production-service proof, production full-node readiness proof,
production-funds wallet use, and production-funds wallet safety proof.

## Phase 107 runtime activation and download eligibility bridge

The `v2-0-runtime-relay-activation-download-eligibility` surface does not
change mempool admission or persistence behavior. It documents the integration
repair that ensures resolved relay activation reaches managed network
construction and transaction download eligibility is checked before requests
are scheduled. `sendrawtransaction` success does not guarantee public
propagation; local admission and queued relay evidence remain bounded by the
same status and support redaction contracts from Phase 105.

Public/operator evidence for this bridge is aggregate and sanitized:
`RelayActivationEvidence` and `RelayDownloadEligibilityCounters` use fixed
labels and numeric counters only. Docs, status, and support evidence must not
copy peer ids, endpoints, permission strings, class names, txids, wtxids, raw
transaction hex, credentials, or dynamic labels.

Phase 107 does not claim compact block relay, package relay, bloom/filter
serving, public relay by default, public-network relay CI, production service
operation, production full-node readiness, production-funds wallet safety,
production-funds wallet use, or durable mempool recovery.

## Known gaps

- package relay beyond single-transaction admission
- rolling minimum-fee decay and long-lived relay-fee state
- Knots `mempool.dat` binary compatibility
- periodic rebroadcast scheduling beyond `rebroadcast_deferred` evidence
- public-network relay readiness evidence beyond the bounded Phase 105 operator
  presentation and Phase 106 deterministic closeout surfaces

## Follow-up triggers

Update this entry when later phases add package relay, dynamic rolling-min-fee
behavior, rebroadcast scheduling, broad operator-facing mempool interfaces, or
Knots-compatible mempool file import/export that materially changes the
externally visible policy surface.
