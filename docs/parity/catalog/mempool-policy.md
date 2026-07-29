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
  capacity, relay fee floors, capacity status, and active rolling-fee parity
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
- intentional eviction-order difference: Open Bitcoin keeps descendant-score then
  txid tie-break for pressure victim selection; Knots may also consult
  modified-fee / entry-time multi-index ordering for equal-score packages
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

- `MempoolPressureSummary` exposes fixed capacity and fee-floor evidence. Phase
  103 kept full Knots rolling minimum fee decay deferred; Phase 131 activates
  live bump/decay with `rolling_fee_parity=active`.
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

## Phase 108 durable mempool relay recovery

Phase 108 completes the durable mempool recovery handoff into managed relay
state for the local Open Bitcoin snapshot format. `MempoolSnapshot` replay
still owns recovery classification, while `ManagedMempoolRecoverySummary`
counts `recovered_count`, `dropped_confirmed_count`,
`dropped_duplicate_count`, `dropped_missing_parent_count`,
`dropped_policy_incompatible_count`, and `dropped_evicted_count`.

Accepted recovered records become serveable through the same txid/wtxid
request path as live accepted records. Confirmed, missing-parent,
policy-incompatible, evicted, replaced, and block-connected records clean
through existing shared mempool lifecycle, serving, and fanout cleanup paths.
The evidence roots include `packages/open-bitcoin-node/src/network/recovery.rs`,
`packages/open-bitcoin-node/src/network/tests/recovery_cases.rs`,
`packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs`, and
`packages/open-bitcoin-node/src/storage/mempool_snapshot.rs`.

Phase 108 does not claim Knots `mempool.dat` binary compatibility, public relay
by default, guaranteed public propagation, compact block relay, package relay,
bloom/filter serving, public-network relay CI, production-service operation,
production full-node readiness, production-funds wallet safety/use,
destructive repair, source datadir mutation, compaction, reindexing, store
surgery, or automatic support upload.

## Phase 126 verified compact reconstruction inputs

The independently verified Phase 126 runtime does not change mempool admission,
persistence, replacement, or eviction policy. It hardens the adapter boundary
that supplies compact reconstruction candidates: `ManagedPeerNetwork` takes an
explicit snapshot of the current mempool and bounded `CompactExtraTxnBuffer`
for both managed receive entrypoints, while the generic factless dispatcher
fails with a typed peer-neutral routing error. An explicitly supplied pair of
empty slices remains valid when both live sources are genuinely empty.

This matches the pinned Knots split between live `m_mempool` lookup and
`vExtraTxnForCompact` supply in
`packages/bitcoin-knots/src/net_processing.cpp`, the bounded extra-transaction
limits in `packages/bitcoin-knots/src/net_processing.h`, and
`PartiallyDownloadedBlock::InitData` candidate consumption in
`packages/bitcoin-knots/src/blockencodings.cpp` and
`packages/bitcoin-knots/src/blockencodings.h`. Mempool-assisted reconstruction
behavior remains anchored in
`packages/bitcoin-knots/test/functional/p2p_compactblocks.py`.

Lifecycle-valid verification completed all six Phase 126 requirements, and
Phase 126 remains locally complete at 4/4 plans. The canonical v2.1 integration
audit reports 29/39 requirements complete and routes gap closure through Phases
127–129 before any fresh archive decision. This verified boundary does not add
package relay, public relay default, archive-node behavior, public-network
gate, production service operation, production full-node readiness, or
production-funds wallet claim.

## Phase 130 resource, time, and fee primitives

The `v2-2-resource-time-fee-primitives` surface uniquely owns `FEEP-01` through
`FEEP-05`. Phase 130 establishes deterministic, non-overloaded contracts for
mempool resource accounting, fee roles, acceptance and relay metadata, explicit
operation contexts, committed lifecycle facts, injected retry inputs, and
truthful RPC projection. It does not activate accounted-capacity enforcement,
package execution, complete cross-cache projection, full snapshot schema
evolution, or retry scheduling.

### Version-1 Rust-owned accounting formula

`MEMPOOL_RESOURCE_ACCOUNTING_VERSION = 1` counts only logical state owned by the
Rust mempool (`packages/open-bitcoin-mempool/src/resource.rs`):

- each entry-map `Txid` key and fixed `MempoolEntry` value;
- fixed transaction input and output elements;
- scriptSig, scriptPubKey, and witness payload bytes;
- one Rust `Vec<u8>` header for every witness item;
- direct parent and child `Txid` identities; and
- each spent-outpoint `OutPoint` key and `Txid` value.

`TransactionVirtualSize`, `AccountedMempoolMemory`, and `MempoolCapacity` remain
compile-time distinct. The cached ledger and independent recomputation oracle
must agree; overflow maps to a typed invariant error.

### Intentional eviction-order difference

Pressure victim selection uses Knots-compatible descendant-score ordering with an
intentional txid tie-break. Knots may also consult modified-fee / entry-time
multi-index ordering when scores collide. This difference is recorded so parity
reviewers do not treat equal-score eviction order as a silent mismatch.

### Intentional difference from Knots allocator estimates

Knots `DynamicUsage` estimates C++ allocator and container behavior. Open
Bitcoin intentionally uses a deterministic Rust-owned logical formula instead of
imitating allocator capacity, hash-table bucket slack, C++ pointer estimates, or
network/node caches. Observable RPC meanings stay Knots-compatible while the
byte totals may differ from Knots `usage` on identical transaction sets.

### Fee roles

Four compile-time-distinct fee roles are required at mempool policy boundaries
(`packages/open-bitcoin-mempool/src/fee.rs`):

- `StaticRelayFeeRate` — ordinary per-transaction anti-free-relay floor;
- `IncrementalRelayFeeRate` — replacement and pressure-bump input only;
- `RollingMempoolFeeRate` — pressure-driven rolling floor (raw state; Phase 131
  owns bump/decay mechanics);
- `EffectiveAdmissionFeeRate` — derived `max(static, rolling)` at decision and
  summary boundaries, never an independent mutable store.

Eligible package aggregates may satisfy the rolling floor while ordinary members
must still satisfy the static floor individually. Incremental never contaminates
ordinary admission or `mempoolminfee`.

### Metadata, recovery compatibility, and explicit contexts

Canonical entries carry typed acceptance time plus origin and relay-intent
metadata (`packages/open-bitcoin-mempool/src/context.rs`). Missing legacy
metadata classifies only as `LegacyUnknown`, `RecoveryUnknown`, and
`NotRequested`; recovery never invents local origin or current time. Known
capture and recovery pass metadata through `AdmissionContext::recovery` without
substituting restart time.

Operation-specific immutable contexts cover admission, pressure, block, reorg,
and retry decisions. Pure mempool and network policy never read wall-clock time
or randomness; shells inject exact values.

### Lifecycle delta invariants

`MempoolLifecycleDelta` is committed-fact vocabulary separate from
`MempoolOutcome` attempt results (`packages/open-bitcoin-mempool/src/pool/lifecycle.rs`).
Deltas record admitted members, final post-transition membership, and typed
removals that keep cause independent from direct-versus-descendant role. Retry
clears resolve with `LifecycleRemoval` > `TransportWritten` > `EligibleServe`
precedence. Stable enum-derived labels are the only shared evidence projection;
transaction identities stay on authenticated direct responses.

### RPC resource and fee mappings

Authoritative `getmempoolinfo` projection preserves Knots field meanings and
exposes Open Bitcoin extensions without redefining baseline fields:

| Field | Meaning |
| --- | --- |
| `bytes` | total `TransactionVirtualSize` |
| `usage` | `AccountedMempoolMemory` |
| `maxmempool` | configured `MempoolCapacity` |
| `mempoolminfee` | derived `EffectiveAdmissionFeeRate` |
| `minrelaytxfee` | static relay floor |
| `incrementalrelayfee` | incremental replacement/pressure bump |
| `rollingmempoolfee` | raw rolling floor (extension) |
| `effectiveadmissionfee` | explicit effective admission (extension) |
| `capacityenforcement` | `accounted_memory` |

Phase 131 enforces accounted-memory trim against `MempoolCapacity` and exposes
live rolling-fee bump/decay with `rolling_fee_parity=active`. Historically,
Phase 130 shipped the transitional RPC label fixed `legacy_vsize` during Phase 130
before Plan 04 retired `PolicyConfig.legacy_vsize_trim_limit` and flipped live
evidence to `accounted_memory`.

### Sustained-pressure bounds (PRESS-05)

Hermetic fill→trim→block→decay→expiry→refill→reorg scenarios must agree with
`recompute_resource_ledger` and the rolling-fee state machine after each
committed transition. Performance bounds for accounted-capacity trim loops are
enforced by the Pure `open-bitcoin-bench` case
`mempool-policy.sustained-pressure-trim` (N=24 admit/trim cycles, 2s wall-time
ceiling under the default verifier). Phase 131 does not add public-network or
non-deterministic soak gates.

### Non-durable rolling fee (D-15)

`RollingMempoolFeeRate` remains non-durable in this phase: a restarted mempool
baseline is zero unless a later durability phase (MPDUR / Phase 135) redesigns
persistence. Knots dump/load similarly does not restore the rolling minimum.

### Injected retry inputs

`RetryJitterSeconds` (`0..=300`) and `RetryDecisionContext` carry exact injected
Unix seconds and validated jitter
(`packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs`). Phase 130
models only those inputs; it does not schedule retries, fan out, or clear
unbroadcast membership from transport receipts.

### Exact later-phase boundaries

Without weakening Phase 130:

- **Phase 131** owns accounted-memory enforcement, trimming against accounted
  capacity, rolling-fee bump/decay mechanics, expiry, and hermetic PRESS
  oracle/perf coverage (not Phase 138 adversarial soak).
- **Phase 132** owns package execution and pinned package-policy exceptions
  (including TRUC) beyond the fee-role vocabulary already fixed here.
- **Phase 134** owns complete cross-cache projection of lifecycle deltas through
  the runtime authority.
- **Phase 135** owns full snapshot/checkpoint/recovery schema evolution beyond
  the Phase 130 optional metadata fields on the existing schema version.
- **Phase 136** owns retry scheduling, fanout, receipts, and clearing.

Phase 130 does not claim general package wire relay, whole-mempool rebroadcast,
public or default relay, guaranteed propagation, public-network default CI,
production readiness, or production-funds wallet use.

## Typed Package Vocabulary and Staged Admission

Phase 132 closes `PACK-01` through `PACK-07` as a bounded local pure-core
surface. It does not add peer package assembly, a general package wire, an RPC
package adapter, public or default relay, guaranteed propagation,
public-network CI, or production readiness.

- **PACK-01 — checked shape and identity.** `WellFormedPackage` rejects empty
  requests, more than 25 transactions, more than 404,000 weight units,
  duplicate txid or wtxid identities, non-topological request order, and
  internal input conflicts before expensive work. Private request-aligned
  storage preserves ordered responses, while the package fingerprint sorts
  wtxids independently so identity does not redefine request order.
- **PACK-02 — non-mutating dry run.** `DryRunPackageCommand` runs the complete
  local policy pipeline and returns the same ordered result vocabulary as
  submission. It has no commit capability and leaves entries, resource and
  rolling-fee state, relay, persistence, and evidence unchanged.
- **PACK-03 — checked submission and report.** `SubmissionPackage` is an opaque
  refinement constructed only by `try_from_package`; it proves a singleton or
  child-with-unconfirmed-parents capability. `PackageReport::try_new` enforces
  one input-index-aligned result per member, identity order, derived
  complete/partial/failed status, and checked effective-fee membership.
- **PACK-04 — individual-first partial acceptance.** Members are tried in input
  order. Successful singletons stay in the prospective view, and only eligible
  reconsiderable members enter residual package evaluation. A valid parent may
  therefore remain finally present when its child is rejected.
- **PACK-05 — coherent staged commit.** A crate-private sparse overlay records
  entry, spent-index, topology, resource, rolling-fee, and lifecycle changes.
  Its `MempoolPatch` is bound to the exact base revision; stale apply rejects
  before mutation. Validation, replacement, limits, script, trim, and patch
  preparation failures discard the prospective state rather than partially
  mutating the live mempool.
- **PACK-06 — separated fee roles and groups.** Every ordinary member still
  meets the static relay floor independently. A non-empty checked effective-fee
  group may meet the active rolling floor; incremental relay fee remains only a
  replacement or pressure input. Groups validate unique ordered wtxid
  membership, aggregate size, and effective-rate consistency.
- **PACK-07 — pinned package-policy exceptions and final truth.** Evaluation
  keeps static-floor, TRUC, rolling-floor, ancestor/descendant limits, limited
  replacement, ephemeral-dust, and late-script stages explicit. Limited RBF
  conservatively sums pre-union descendant counts against the 100-candidate
  bound and evaluates TRUC direct conflicts and sibling-eviction intent against
  pre-replacement facts. P2A uses witness program bytes `0x4e 0x73`; the dust
  rate is 3000 sat/kvB; permissions default to `anchor=true`, `send=false`,
  `dust=false`; dusty parents require zero base and modified fee; and a child
  must spend all permitted ephemeral outputs. Same-txid/different-witness
  aliases, hard and reconsiderable failures, and effective groups remain typed.
  Admission performs one final trim and then rewrites every initially
  successful result from authoritative post-trim membership before producing
  lifecycle facts.

The intentional Rust differences are internal safety mechanisms rather than
external policy divergence: opaque refinements replace caller booleans,
request-aligned vectors own ordering, the prospective mempool is a sparse
overlay instead of a full clone, and deterministic Rust-owned accounting
replaces C++ allocator estimates.

### Phase 132 pinned Knots anchors

- [`packages/bitcoin-knots/doc/policy/packages.md`](../../../packages/bitcoin-knots/doc/policy/packages.md)
- [`packages/bitcoin-knots/src/policy/packages.cpp`](../../../packages/bitcoin-knots/src/policy/packages.cpp)
- [`packages/bitcoin-knots/src/validation.h`](../../../packages/bitcoin-knots/src/validation.h)
- [`packages/bitcoin-knots/src/validation.cpp`](../../../packages/bitcoin-knots/src/validation.cpp)
- [`packages/bitcoin-knots/src/test/txpackage_tests.cpp`](../../../packages/bitcoin-knots/src/test/txpackage_tests.cpp)
- [`packages/bitcoin-knots/src/test/txvalidation_tests.cpp`](../../../packages/bitcoin-knots/src/test/txvalidation_tests.cpp)
- [`packages/bitcoin-knots/test/functional/mempool_package_rbf.py`](../../../packages/bitcoin-knots/test/functional/mempool_package_rbf.py)
- [`packages/bitcoin-knots/test/functional/mempool_truc.py`](../../../packages/bitcoin-knots/test/functional/mempool_truc.py)
- [`packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py`](../../../packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py)

The bounded local core emits only reports and `MempoolLifecycleDelta` facts.
Phase 133 owns same-peer candidate assembly, Phase 134 owns authoritative
cross-cache projection, Phase 135 owns durable recovery, Phase 136 owns relay
fanout and retry, and Phase 137 owns RPC/operator adapters. General package wire
protocols, arbitrary multi-parent peer assembly, public/default relay,
guaranteed propagation, public-network gates, and production readiness remain
deferred.

## Package-Aware Download and Orphan Bridge

Phase 133 closes `PPKG-01` through `PPKG-03` with one narrow peer-download
bridge over the existing ordinary `inv`/`getdata`/`tx` flow.

- **PPKG-01 — bounded reject evidence.** Two independent node-global rolling
  filters retain hard-rejected `Wtxid` evidence and reconsiderable `Wtxid` or
  `PackageFingerprint` evidence. Each filter is locked to 120,000 recent
  insertions at a target false-positive rate of 0.000001, uses fixed memory
  under sustained unique input, and resets on an authoritative active-tip
  change. Evidence suppresses redundant work only; it never causes peer
  punishment or disconnection. The intentional scoped difference from Knots is
  that Open Bitcoin keys these domains by wtxid and package fingerprint rather
  than also retaining selected txid keys.
- **PPKG-02 — bounded same-peer candidate proof.** Orphan transaction bodies
  are stored once while announcer provenance is capped independently. A
  reconsiderable parent can select the newest eligible child only when that
  child has exactly one missing parent and retained provenance includes the
  parent-delivering peer. Traversal, global orphan count, per-peer orphan count,
  and announcer count remain bounded; disconnect, expiry, rejection, and
  eviction clean every index coherently. The resulting candidate keeps its two
  transaction bodies, origins, and provenances private and consumable, with
  request order exactly `[parent, child]` and aligned same-peer origins.
- **PPKG-03 — one authoritative admission bridge.** The network layer remains
  admission-neutral and does not depend on mempool policy. The node shell
  constructs the Phase 132 refinement, caches its package fingerprint, performs
  exactly one authoritative package-admission call per eligible candidate, and
  returns the exact ordered report and lifecycle delta. Exhaustive typed
  feedback records hard member evidence, reconsiderable member evidence, or a
  failed package fingerprint in the matching domain.

Phase 133 intentionally does not project package results into relay-serving,
fanout, receipts, persistence, or operator surfaces. Phase 134 owns
authoritative lifecycle projection, Phase 136 owns fanout/receipts, and Phase
137 owns RPC/operator adapters. General package wire protocols, arbitrary
multi-parent peer assembly, public/default relay, guaranteed propagation,
public-network gates, and production readiness remain deferred.

### Phase 133 pinned Knots anchors

- [`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp)
- [`packages/bitcoin-knots/src/node/txdownloadman_impl.cpp`](../../../packages/bitcoin-knots/src/node/txdownloadman_impl.cpp)
- [`packages/bitcoin-knots/src/txorphanage.cpp`](../../../packages/bitcoin-knots/src/txorphanage.cpp)
- [`packages/bitcoin-knots/test/functional/p2p_orphan_handling.py`](../../../packages/bitcoin-knots/test/functional/p2p_orphan_handling.py)
- [`packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py`](../../../packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py)
- [`packages/bitcoin-knots/test/functional/p2p_tx_download.py`](../../../packages/bitcoin-knots/test/functional/p2p_tx_download.py)

## Authoritative Cross-Cache Lifecycle Integration

Phase 134 implements one deterministic, I/O-free lifecycle projector behind
`ManagedNetworkHandle`. A validated prepared transition commits canonical
mempool state and then applies compact, serving, ordinary fanout, peer,
unbroadcast, persistence, and bounded evidence projections in one fixed
infallible order. `final_membership` decides which admitted members enter
accepted projections; admission stays parent-before-child, teardown stays
descendant-before-ancestor, and reorg transitions are prepared and committed
sequentially.

The normal mutation path is incremental. Seven-label bounded reconciliation is
read-only audit, recovery, randomized-test, and failure-injection evidence; it
is not a normal-path cache rebuild. Peer writes and current-schema Fjall
snapshots use separate bounded, affine prepare → execute → complete families.
Their I/O occurs after authority is released, and only an achieved effect can
mint a receipt. Completion distinguishes `Applied`, `AchievedButStale`, and
`AlreadyApplied` without clearing newer authoritative truth.

### D-01 through D-17 traceability

| Decisions | Exact Open Bitcoin source | Scenario, oracle, or checker evidence | Pinned Knots anchors |
| --- | --- | --- | --- |
| D-01, D-02, D-03, D-04 | `network/runtime_authority.rs::ManagedNetworkHandle`, `network/runtime_authority/lifecycle.rs::apply_lifecycle_command`, `network/lifecycle_projection.rs::LifecycleCommand`, `open-bitcoin-mempool/src/pool/prepared_lifecycle.rs::{PreparedMempoolTransition, PreparedLifecycleFacts}` | `network/tests/lifecycle_projection_cases.rs::assert_complete_projection`, `scripts/check-phase134-authoritative-lifecycle.ts` authority and dispatcher contracts | `validation.cpp::{AcceptToMemoryPool, ProcessNewPackage}`, `net_processing.cpp::{ProcessPackageResult, RelayTransaction}` |
| D-05 | `network/lifecycle_projection.rs::LifecycleProjectionPlan`, `network/lifecycle_projection/authority.rs::{validate_prepared_lifecycle, apply_prepared_lifecycle}`, `open-bitcoin-network/src/peer/transaction_lifecycle.rs::{prepare_transaction_lifecycle, apply_prepared_transaction_lifecycle}` | `network/tests/lifecycle_projection_cases/oracle.rs::every_injected_preflight_failure_preserves_the_complete_aggregate`, `scripts/check-phase134-apply-boundaries.ts` | `validation.cpp::MemPoolAccept::FinalizeSubpackage`, `txmempool.cpp::RemoveStaged` |
| D-06, D-07, D-08, D-09, D-10 | `network/lifecycle_projection/authority.rs`, `network/lifecycle_projection/reconciliation.rs::reconcile_lifecycle_projection`, `network/relay_serving.rs`, `network/relay_fanout/lifecycle.rs`, `network/compact_receive_candidates.rs`, `open-bitcoin-network/src/peer/transaction_lifecycle.rs` | `network/tests/lifecycle_projection_cases/admission.rs::full_package_projects_parent_first_final_membership_across_every_target`, `admission/partial_package.rs::{partial_package_projects_only_the_parent_survivor, replacement_package_tears_down_both_victim_aliases_and_fingerprint, pressure_eviction_tears_down_descendant_before_ancestor_across_every_projection}`, `maintenance.rs::{expiry_removes_descendants_from_every_projection_and_advances_once, connected_block_conflict_removes_descendants_from_every_projection, reorg_steps_apply_sequentially_and_reconcile_each_generation}`, `oracle.rs::fixed_seed_generated_oracle_detects_each_corrupted_target_exactly` | `txmempool.cpp::{removeRecursive, RemoveStaged, Expire, TrimToSize}`, `validation.cpp::{MemPoolAccept::SubmitPackage, ProcessNewPackage}`, `node/txdownloadman_impl.cpp::MempoolRejectedTx`, `txorphanage.cpp::{EraseTx, EraseForPeer, AddChildrenToWorkSet}` |
| D-11, D-12, D-13, D-14, D-15 | `network/lifecycle_effects.rs::{PeerEffectCapability, PeerEffectReceipt, PreparedSnapshotWrite, SnapshotWriteReceipt, EffectCompletion}`, `network/announcement_transport.rs::PeerEmission::acknowledge_write`, `sync/session.rs`, `open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs`, `storage/fjall_store/mempool.rs::execute_prepared_mempool_snapshot_write` | `network/tests/lifecycle_projection_cases/effects.rs::{stale_snapshot_completion_records_truth_without_clearing_newer_dirty_state, duplicate_peer_completion_precedes_stale_session_detection}`, `effects/contracts.rs`, `open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs::phase134_rpc_successful_prefix_write_failure_stops_before_third_command`, `storage/fjall_store/tests/snapshot_persistence.rs` | `net_processing.cpp::RelayTransaction`, `node/mempool_persist.cpp::{DumpMempool, LoadMempool}` |
| D-16 | `network/tests/lifecycle_projection_cases/{admission.rs,admission/partial_package.rs,maintenance.rs,effects.rs,oracle.rs}` and `open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs` | Eleven deterministic full/partial/replacement/pressure/expiry/block/reorg/failed/stale/duplicate/partial-I/O scenarios plus the fixed-seed independent oracle | `validation.cpp`, `txmempool.cpp`, `net_processing.cpp`, `node/txdownloadman_impl.cpp`, `txorphanage.cpp`, `node/mempool_persist.cpp` |
| D-17 | `scripts/check-phase134-authoritative-lifecycle.ts`, `scripts/check-phase134-apply-boundaries.ts`, and their exported root-aware contracts | `scripts/check-phase134-authoritative-lifecycle.test.ts` independently mutates authority, targets, effects, scenarios, evidence, scope, and verifier ordering; the apply checker rejects fallibility, derivation, codec, async, and I/O work in all seven applies plus aggregate commit | The same six pinned source files above define the reviewed mutation and effect boundaries. |

### Phase 134 review finding repair evidence

The initial Phase 134 review recorded one critical and eleven warning findings.
The following final-tree evidence maps every finding to its implementation and
an executable regression surface. These rows document the implemented repairs;
they do not promote the pending MPLIFE requirements or replace independent
phase-level re-verification.

| Finding | Concrete implementation path | Regression command or evidence path |
| --- | --- | --- |
| `CR-01` foreign receipt consumption | `packages/open-bitcoin-node/src/network.rs` installs a unique live-handle authority incarnation; `packages/open-bitcoin-node/src/network/lifecycle_effects.rs` keys pending/completed work by complete family binding; `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` consumes only exact bindings | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node lifecycle_projection_cases::effects` (`packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts.rs`) |
| `WR-01` accepted-package count bound | `packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs` caps raw accepted-package work before preprocessing and caps deduplicated retained state | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network transaction_lifecycle_cases::bounded_packages` |
| `WR-02` txid/wtxid cursor alias | `packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs` carries stored txid+wtxid identities and `packages/open-bitcoin-network/src/peer/transaction_lifecycle/reconciliation.rs` compares both identity domains | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network transaction_lifecycle_cases::identity_aliases` |
| `WR-03` retirement before capacity | `packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs` retires same-transition fingerprints before validating the prospective final map | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network transaction_lifecycle_cases::bounded_packages` |
| `WR-04` stale validated capability | `packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs::commit_prepared_mempool_transition` checks revision and consumes the patch in one mutable call; the obsolete split API is removed | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool prepared_lifecycle_cases` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node lifecycle_projection_cases` |
| `WR-05` atomic peer completion/evidence | `packages/open-bitcoin-node/src/network/lifecycle_projection.rs::CompletePeerEmission` and `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` validate, classify, account, and record evidence under one authority guard | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node lifecycle_projection_cases::effects::contracts::peer_sessions` |
| `WR-06` per-peer session freshness | `packages/open-bitcoin-node/src/network.rs` retains bounded target-peer session generations; unrelated peer churn cannot stale another peer's receipt | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node lifecycle_projection_cases::effects::contracts::peer_sessions` |
| `WR-07` complete-or-abort terminal paths | `packages/open-bitcoin-node/src/network/lifecycle_effects.rs` provides exact peer/snapshot abort; `packages/open-bitcoin-node/src/sync/session/emission_terminal.rs`, RPC fanout, and Fjall snapshot execution terminate every owned capability | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node lifecycle_projection_cases::effects`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc announcement_successful_prefix`, and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node snapshot_persistence` |
| `WR-08` symmetric unbroadcast reconciliation | `packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs` derives retry-eligible expected membership and counts the symmetric difference against actual state | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node lifecycle_projection_cases::reconciliation` |
| `WR-09` transitive helper guard | `scripts/check-phase134-apply-boundaries.ts` traverses exact fully qualified repo-owned calls and rejects unresolved or unclassified helpers | `bun test scripts/check-phase134-authoritative-lifecycle.test.ts scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts` plus `bun scripts/check-phase134-apply-boundaries.ts` |
| `WR-10` premature done status | `docs/parity/checklist.md`, both Phase 134 records in `docs/parity/index.json`, and the structured status parser remain `in_progress` while MPLIFE or recorded gaps are pending | `bun test scripts/check-phase134-authoritative-lifecycle.test.ts scripts/check-phase134-authoritative-lifecycle.test/scope-claims.ts` plus `bun scripts/check-phase134-authoritative-lifecycle.ts` |
| `WR-11` canonical claim surfaces and variants | `scripts/check-phase134-authoritative-lifecycle/scope.ts` normalizes and guards README, package README, catalog, checklist, and machine index claims | `bun test scripts/check-phase134-authoritative-lifecycle.test.ts scripts/check-phase134-authoritative-lifecycle.test/scope-claims.ts` plus `bun scripts/check-phase134-authoritative-lifecycle.ts` |

### MPLIFE requirement evidence

| Requirement | Exact implementation and test evidence | Pinned Knots anchors |
| --- | --- | --- |
| `MPLIFE-01` | `ManagedNetworkHandle::apply_lifecycle_command`, every admission/maintenance/effect facade under `network/runtime_authority/`, and the authority/dispatcher/direct-mutation mutations in `check-phase134-authoritative-lifecycle.test.ts` | `validation.cpp`, `net_processing.cpp` |
| `MPLIFE-02` | `LifecycleProjectionPlan`, `apply_prepared_lifecycle`, all seven target applies, `assert_complete_projection`, and full/partial package scenario coverage | `validation.cpp`, `txmempool.cpp`, `net_processing.cpp`, `node/txdownloadman_impl.cpp`, `txorphanage.cpp` |
| `MPLIFE-03` | Prepared descendant-first teardown, exact txid/wtxid and package-fingerprint cleanup, replacement/pressure/expiry/block/reorg/failed-admission scenarios, and the independent seven-target reconciliation oracle | `txmempool.cpp`, `validation.cpp`, `node/txdownloadman_impl.cpp`, `txorphanage.cpp` |
| `MPLIFE-04` | Family-specific affine peer/snapshot capabilities, outside-authority node/RPC/Fjall executors, successful-prefix tests, and current/stale/duplicate completion tests | `net_processing.cpp`, `node/mempool_persist.cpp` |

The `MPLIFE-01` through `MPLIFE-04` requirement checkboxes remain pending until
the phase-level verifier records its final result. This catalog records the
implemented lineage without pre-approving that later verification gate.

### Phase 134 scope boundary

D-18 remains unchanged: default verification is deterministic and hermetic.
Phase 134 does not implement the Phase 135 snapshot schema, checkpoint
coordinator, crash-loss window, or recovery contract; Phase 136
receive-independent retry scheduling or package fanout; Phase 137 broad
RPC/operator surfaces; or Phase 138 release proof. It also does not add a
general package wire, whole-mempool rebroadcast, public or default relay,
guaranteed propagation, public-network CI, or a production-readiness claim.

### Knots sources for this surface

- [`packages/bitcoin-knots/src/txmempool.h`](../../../packages/bitcoin-knots/src/txmempool.h)
- [`packages/bitcoin-knots/src/txmempool.cpp`](../../../packages/bitcoin-knots/src/txmempool.cpp)
- [`packages/bitcoin-knots/src/rpc/mempool.cpp`](../../../packages/bitcoin-knots/src/rpc/mempool.cpp)
- [`packages/bitcoin-knots/src/kernel/mempool_entry.h`](../../../packages/bitcoin-knots/src/kernel/mempool_entry.h)
- [`packages/bitcoin-knots/src/kernel/mempool_removal_reason.h`](../../../packages/bitcoin-knots/src/kernel/mempool_removal_reason.h)
- [`packages/bitcoin-knots/src/node/mempool_persist.cpp`](../../../packages/bitcoin-knots/src/node/mempool_persist.cpp)
- [`packages/bitcoin-knots/src/validation.cpp`](../../../packages/bitcoin-knots/src/validation.cpp)
- [`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp)
- [`packages/bitcoin-knots/src/policy/policy.h`](../../../packages/bitcoin-knots/src/policy/policy.h)
- [`packages/bitcoin-knots/doc/policy/packages.md`](../../../packages/bitcoin-knots/doc/policy/packages.md)

### First-party evidence roots

- [`packages/open-bitcoin-mempool/src/resource.rs`](../../../packages/open-bitcoin-mempool/src/resource.rs)
- [`packages/open-bitcoin-mempool/src/fee.rs`](../../../packages/open-bitcoin-mempool/src/fee.rs)
- [`packages/open-bitcoin-mempool/src/context.rs`](../../../packages/open-bitcoin-mempool/src/context.rs)
- [`packages/open-bitcoin-mempool/src/pool/lifecycle.rs`](../../../packages/open-bitcoin-mempool/src/pool/lifecycle.rs)
- [`packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs`](../../../packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs)
- [`packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs`](../../../packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs)
- [`packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs`](../../../packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs)
- [`packages/open-bitcoin-mempool/src/pool/tests/lifecycle_delta_cases.rs`](../../../packages/open-bitcoin-mempool/src/pool/tests/lifecycle_delta_cases.rs)
- [`packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs`](../../../packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs)
- [`packages/open-bitcoin-node/src/storage/snapshot_codec.rs`](../../../packages/open-bitcoin-node/src/storage/snapshot_codec.rs)
- [`packages/open-bitcoin-rpc/src/dispatch/node.rs`](../../../packages/open-bitcoin-rpc/src/dispatch/node.rs)
- [`.planning/phases/130-resource-time-and-fee-primitives/`](../../../.planning/phases/130-resource-time-and-fee-primitives/)

## Known gaps

- Phase 138 adversarial soak and broader public-network pressure validation remain
  outside Phase 131's hermetic PRESS oracle/perf coverage
- full snapshot/checkpoint/recovery schema beyond optional metadata fields
  (Phase 135)
- retry scheduling, fanout, receipts, and unbroadcast clearing (Phase 136)
- RPC and operator package adapters (Phase 137)
- Knots `mempool.dat` binary compatibility
- general package wire relay, whole-mempool rebroadcast, public/default relay,
  guaranteed propagation, public-network default CI, production readiness, and
  production-funds wallet use remain deferred

## Follow-up triggers

Revisit this entry when the currently deferred general package wire relay
boundary changes, or when later phases add checkpoint/recovery schema changes,
retry scheduling, broad operator-facing mempool interfaces, or Knots-compatible
mempool file import/export that materially changes the externally visible
policy surface.
