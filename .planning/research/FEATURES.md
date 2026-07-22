# Feature Research

**Domain:** Knots-compatible package admission/relay and long-lived mempool policy
**Milestone:** Open Bitcoin v2.2 Package Relay and Long-Lived Mempool Policy
**Baseline:** Bitcoin Knots `29.3.knots20260210`
**Researched:** 2026-07-22
**Confidence:** HIGH — recommendations are grounded in the pinned source tree and its functional/unit tests, plus the shipped Open Bitcoin v2.0/v2.1 contracts.

## Scope Decision

v2.2 should implement the package and long-lived mempool behavior that the pinned Knots baseline actually exposes. That means two related but different package surfaces:

1. Local RPC/package admission can evaluate as many as 25 transactions, with `submitpackage` restricted to one child and its independent unconfirmed parents.
2. P2P package propagation has no new package wire message. Knots relays ordinary transactions and opportunistically reconstructs only a 1-parent-1-child (1p1c) package when a low-fee parent and its orphan child can be reconsidered together.

The milestone should not call this BIP331-style general package relay, cluster mempool, or guaranteed propagation. It should retain the v2.0 explicit relay activation boundary and use the authoritative v2.1 peer transport/status instance.

The long-lived policy boundary should also match Knots precisely: pressure raises a rolling admission floor; decay is block-gated and occupancy-sensitive; the rolling floor resets on process restart rather than being serialized; accepted mempool entries and the locally submitted unbroadcast set are the durable recovery surfaces. “Periodic rebroadcast” should mean best-effort retry of the local unbroadcast set, not periodic reannouncement of the whole mempool.

## Feature Landscape

### Table Stakes (Users and Operators Expect These)

| Feature | Why Expected | Complexity | Baseline Semantics and Observable Outcome |
| --- | --- | --- | --- |
| Context-free package shape validation | Invalid or adversarial batches must fail before expensive validation or mutation | MEDIUM | Reject empty/oversized, duplicate, internally conflicting, overweight, or dependency-unsorted packages. Knots permits at most 25 transactions and 404,000 total weight for multi-transaction packages. Operators receive stable package-wide errors such as `package-too-many-transactions`, `package-too-large`, `package-not-sorted`, or `conflict-in-package`. |
| Dry-run package validation | Contributors and callers need to know whether a package would be accepted without mutating the mempool | HIGH | `testmempoolaccept` evaluates topologically sorted, internally non-conflicting packages of up to 25 transactions. Results remain input-ordered and report `txid`, `wtxid`, optional `allowed`, rejection details, vsize, base fee, effective feerate, and the wtxids included in that effective calculation. One early failure may leave later results unevaluated. |
| Child-with-unconfirmed-parents submission | CPFP is the principal externally visible reason to submit a package | HIGH | `submitpackage` accepts a single transaction or a topologically sorted child-with-parents tree: the last transaction is the child, every other transaction is its direct parent, parents do not depend on each other, and every unconfirmed parent needed by the child is included. A child may raise an ordinary parent over the dynamic mempool floor, but package feerate does not replace the static `minrelaytxfee` requirement; apply only the narrow TRUC/package exceptions present in the baseline. |
| Correct partial-submission contract | Operators need accurate outcomes when some package members are independently acceptable and others are not | HIGH | Knots first accepts transactions that pass individually, then evaluates only reconsiderable/missing-input members as a subpackage. A failed subpackage must not partially mutate, but previously accepted individual transactions may remain and be broadcast. `package_msg=success` means every transaction is accepted or already present; any weaker result must identify per-wtxid outcomes. |
| Package-context replacement and policy checks | Package admission cannot bypass RBF, ancestor/descendant, TRUC, ephemeral-dust, or standardness policy | HIGH | Match Knots’ limited 1p1c package-RBF topology and fee-diagram improvement checks, replacement-count and incremental-fee rules, package ancestor/descendant accounting, disabled CPFP carve-out in package context, TRUC inheritance/topology and sibling eviction, and ephemeral-dust spend requirements. If any are deliberately deferred, narrow the public claim and record the parity gap. |
| Package-safe mutation and post-submit trimming | Failed validation or pressure must not leave an internally inconsistent graph | HIGH | Stage additions/removals, validate the subpackage, apply in topological order, then enforce the mempool cap. Package evaluation may temporarily exceed the cap; after trimming, any accepted member no longer present must be reported as `mempool full`. Parent/child links, spent-outpoint state, and replacement sets must remain coherent. |
| Opportunistic P2P 1p1c acceptance | Knots nodes can propagate a fee-bumped parent/child pair without a package wire protocol | HIGH | Use existing `inv`/`getdata`/`tx` flow. Classify low-fee or pressure failures as reconsiderable, retain bounded reject/orphan state, request the missing parent, and evaluate a parent plus orphan child together. Only one reconsiderable parent is supported for this P2P path, and candidate children are selected from the same peer’s orphan evidence to resist censorship/blame ambiguity. |
| Ordinary per-transaction relay after package acceptance | Existing peers understand transactions, not a package object | HIGH | Relay every accepted/still-present transaction through the v2.0 txid/wtxid inventory, serving, queue, rate-limit, known-set, origin-suppression, and peer-feefilter paths. A low-fee parent may propagate because a relayed child causes the recipient to request its parent. No package inventory type is introduced. |
| Dynamic-memory pressure accounting | Knots’ `-maxmempool` is a memory bound, not a sum-of-vsize bound | HIGH | Track both transaction virtual bytes and estimated dynamic memory usage. Capacity enforcement uses dynamic memory against `maxmempool`; operator surfaces distinguish `bytes` (sum of vsize), `usage` (dynamic memory), and `maxmempool`. Open Bitcoin’s current `max_mempool_virtual_size`-only trimming is insufficient for a parity claim. |
| Descendant-score package eviction | A bounded mempool must preserve CPFP economics and graph consistency under pressure | HIGH | Repeatedly remove the lowest descendant-score entry together with all in-mempool descendants until usage is below the cap. Clean relay-serving, fanout, orphan/reject, compact-reconstruction, and operator-evidence state for every removal. Never leave a child whose required mempool parent was evicted. |
| Rolling minimum admission fee | Recently evicted low-fee traffic should not immediately churn back into a full mempool | HIGH | For each size-limit eviction package, raise the rolling floor to that package’s descendant feerate plus `incrementalrelayfee`. Admission compares transaction or effective package fee against the current floor. `getmempoolinfo.mempoolminfee` reports the maximum of the dynamic floor and configured `minrelaytxfee`. |
| Knots-aligned fee-floor decay | Operators expect fee pressure to recover after blocks create room | HIGH | Do not decay until a block has connected after the last pressure bump. Then use a 12-hour half-life, shortened to 6 hours below half capacity and 3 hours below quarter capacity; update after more than 10 seconds have elapsed. Drop the rolling value to zero below half the incremental relay fee. The clock and occupancy inputs must be explicit and deterministic in core tests. |
| Restart/recovery boundary | Long-running behavior must recover without silently inventing stronger persistence than Knots | HIGH | Persist accepted transactions and local unbroadcast membership through Open Bitcoin’s owned snapshot. Replay in dependency order through current policy, classifying confirmed, duplicate, missing-parent, policy-incompatible, and pressure-evicted drops. Match Knots by resetting the rolling minimum fee on restart; if replay exceeds the current cap, trimming may establish a new floor. |
| Bounded initial-broadcast retry | A locally submitted transaction should not be stranded when no peer requested it on first announcement | MEDIUM | Add only locally submitted, relay-requested transactions to an unbroadcast set. Retry that set every randomized 10–15 minutes, through normal peer eligibility/queue/feefilter policy, until a peer requests the transaction. Persist the set across restart and remove entries when acknowledged, confirmed, replaced, expired, evicted, or absent from the mempool. |
| Expiry and sustained-pressure lifecycle | Long-lived pools must not retain stale packages indefinitely | MEDIUM | Expire transactions older than the configured `mempoolexpiry` (Knots default: 336 hours) and remove their descendants. Block connect, reorg reconsideration, replacement, expiry, and size eviction must all use the same typed lifecycle and cleanup contract. |
| Operator-visible package and pressure truth | Operators must be able to distinguish policy pressure, relay inactivity, and deferred/public boundaries | MEDIUM | Expose package accept/reject/partial outcomes; current/min/incremental fee floors; vsize, dynamic usage and cap; eviction/expiry counts; unbroadcast count; retry due/success/suppression outcomes; and recovery classifications through the shared RPC/CLI/dashboard/status contract. Metrics/logs use fixed low-cardinality labels and support bundles exclude raw transactions, transaction IDs, peer endpoints, permissions, and credentials. |
| Deterministic parity and release guardrails | The project’s core value requires auditable behavior and scoped claims | MEDIUM | Add pinned source/test roots, package/pressure fixtures, deterministic clocks and jitter, repo-local Cargo/Bazel UAT commands, and no-claim checks. Default `bash scripts/verify.sh` must remain free of live public-network, wall-clock soak, service-manager, and production-deployment requirements. |

### Differentiators (Competitive Advantage)

These improve Open Bitcoin’s safety and auditability without changing externally observable Knots behavior.

| Feature | Value Proposition | Complexity | Notes |
| --- | --- | --- | --- |
| Typed package decision model | Makes partial acceptance, reconsiderable failure, replacement, eviction, and relay consequences reviewable without parsing strings | HIGH | Model package-wide state separately from per-wtxid outcomes; encode valid topology and effective-fee groups in domain types where practical. |
| Pure pressure state machine | Makes rolling-floor and eviction decisions deterministic and cheap to test | HIGH | Feed explicit time, block-connected events, usage, configured cap, and fee rates into pure transitions; keep scheduling, storage, RPC, and socket effects in adapters. |
| Auditable package-to-relay lineage | Lets contributors prove which accepted package members became successfully emitted inventory | MEDIUM | Preserve the v2.1 achieved-effect rule: distinguish eligible/queued/attempted from successful wire emission. Aggregate evidence links package outcome → mempool membership → relay queue → successful write without exposing identities. |
| Recovery with typed drop evidence | Makes restart behavior diagnosable instead of silently changing mempool contents | MEDIUM | Reuse v2.0 recovery classifications and add dependency-order, unbroadcast-restored, floor-reset, and replay-trim summaries. Report aggregate counts only. |
| Deterministic adversarial pressure harness | Finds graph and cleanup bugs without public-network tests | HIGH | Simulate clock advances, block cadence, occupancy bands, package bursts, reconnects, restart replay, partial package failure, replacement, eviction, and retry acknowledgements under fixed seeds. |
| Explicit claim taxonomy | Prevents “package relay” from being mistaken for a new wire protocol or public production relay | LOW | Status/docs distinguish `local_package_rpc`, `opportunistic_p2p_1p1c`, `initial_broadcast_retry`, and deferred `general_package_wire_relay`/`public_relay_default`. |

### Anti-Features (Commonly Requested, Often Problematic)

| Feature | Why Requested | Why Problematic | Alternative |
| --- | --- | --- | --- |
| New package inventory/message protocol | Sounds like the direct way to relay packages | The pinned Knots baseline does not use one for this behavior; it would create an unsupported protocol and overclaim BIP331/general package relay | Use ordinary transaction inventory plus opportunistic same-peer 1p1c reconstruction. |
| “Broadcast every mempool transaction periodically” | Appears to maximize propagation | Knots retries only locally submitted transactions still awaiting evidence of initial delivery; whole-pool rebroadcast leaks timing, wastes bandwidth, and fingerprints nodes | Maintain a bounded durable unbroadcast set and stop after peer `getdata` acknowledgement or lifecycle removal. |
| Deterministic fixed retry interval | Simplifies tests and scheduling | A fixed public cadence is fingerprintable and diverges from Knots’ randomized 10–15 minute cycle | Inject a seeded jitter source for tests and cryptographically appropriate randomness in the runtime shell. |
| Persist the rolling fee floor verbatim across restart | Seems safer under pressure | Knots does not serialize this state; persisting it would reject transactions Knots accepts after restart and creates stale-clock recovery hazards | Reset the rolling floor on restart, replay accepted entries, enforce current policy/cap, and report the reset. |
| Virtual-size-only mempool cap | Reuses the current Open Bitcoin model | It does not represent actual memory pressure and diverges from Knots’ `DynamicMemoryUsage()`-based `maxmempool` enforcement | Track vsize for fee policy and dynamic usage for capacity enforcement; expose both. |
| Atomic all-or-nothing `submitpackage` | Sounds simpler for callers | Knots intentionally accepts and broadcasts individually valid members even when another member fails; changing this is observable and can harm propagation | Make each staged multi-transaction subpackage atomic while preserving package-level partial submission and per-transaction results. |
| General multi-parent P2P package reconstruction | Extends CPFP reach | The baseline opportunistic network path handles only 1p1c and rejects cases with more than one reconsiderable parent; broader behavior changes DoS and blame semantics | Keep broad child-with-parents admission local to RPC and scope P2P to parity-backed 1p1c. |
| Unbounded package/reject/orphan caches | Avoids missing a later fee-bump opportunity | Creates memory/CPU DoS and can let attackers crowd out honest candidates | Reuse v2.0 orphan, request, timeout, per-peer, and recent-reject bounds; cache rejected package hashes with bounded rolling structures. |
| Immediate rolling-floor decay after eviction | Returns to low fees quickly | Knots deliberately waits for a connected block; time alone does not prove pressure has cleared | Gate decay on a post-bump block and then use occupancy-adjusted half-lives. |
| Public relay enabled by default | Makes the feature easy to demonstrate | Violates the shipped activation boundary and implies abuse, support, packaging, firewall, and production evidence that v2.2 does not provide | Keep transaction/package relay explicit, bounded, observable, and opt-in. |
| Public-network CI or wall-clock soak as a default gate | Offers realistic coverage | Makes verification nondeterministic and conflicts with the repo contract | Use hermetic fixtures and virtual time by default; retain explicit opt-in UAT for live review. |
| Knots `mempool.dat` binary compatibility | Sounds like seamless migration | v2.0 intentionally owns a typed snapshot format; binary compatibility materially expands migration, corruption, and support scope | Preserve the Open Bitcoin snapshot and defer import/export compatibility to a dedicated migration milestone. |
| Cluster mempool/Erlay/bloom/filter expansion | Related to relay and batching | These are distinct protocol/policy surfaces and obscure the v2.2 parity boundary | Record them as future scoped milestones with their own threat model and evidence. |

## Observable Operator and User Outcomes

| Scenario | Expected External Result |
| --- | --- |
| Submit a valid parent plus fee-bumping child | Both enter the mempool, results are keyed by wtxid, and both are queued for ordinary transaction relay when relay is active. Effective-fee fields identify the exact members used in the calculation. |
| Submit a valid parent plus non-standard child | The valid parent may remain accepted and be broadcast; the child reports its rejection and `package_msg` is not `success`. |
| Receive child before low-fee parent from a relay peer | Child becomes bounded orphan/candidate state; parent is requested by txid; the same-peer 1p1c pair is evaluated when the parent arrives. |
| Receive a package needing two reconsiderable parents over P2P | The opportunistic 1p1c path does not attempt general package evaluation; state remains bounded and the node does not claim general package wire relay. |
| Mempool crosses its dynamic-memory cap | Lowest descendant-score packages are removed until within the cap, the rolling floor rises to the strongest removed-package rate plus increment, and all relay/cache state for removals is cleaned. |
| Time advances with no block after a fee-floor bump | The floor does not decay. |
| A block connects after a fee-floor bump | Decay becomes eligible; the rate then follows the 12/6/3-hour half-life selected by current occupancy. |
| Daemon restarts | Accepted snapshot records are replayed and local unbroadcast membership is restored; the old rolling floor is reset, and replay-time policy/cap failures are counted. |
| Locally submitted transaction gets no `getdata` | It remains in the unbroadcast set and is retried on a randomized 10–15 minute schedule while relay stays active and eligible. |
| A peer requests the locally submitted transaction | The node serves it through existing eligibility rules and removes it from the unbroadcast set; future cycles do not reannounce it merely because it remains in the mempool. |
| Relay is disabled or peer policy suppresses emission | Package admission may still succeed locally, but relay status reports suppressed/disabled outcomes and never promises public propagation. |

## Feature Dependencies

```text
v2.0 single-transaction admission + topology + typed outcomes
├──requires──> Package shape/domain model
│               ├──requires──> Package-context validation and staged mutation
│               └──requires──> submitpackage/testmempoolaccept-compatible outcomes
├──requires──> Dynamic-memory pressure model
│               ├──requires──> Descendant-score eviction and graph cleanup
│               └──requires──> Rolling minimum-fee bump/decay
├──requires──> Bounded orphan + reconsiderable-reject state
│               └──requires──> Opportunistic same-peer P2P 1p1c acceptance
└──requires──> Durable mempool snapshot
                ├──requires──> Dependency-ordered recovery
                └──requires──> Durable local unbroadcast set

v2.0 txid/wtxid relay + serving + fanout
└──requires──> Per-transaction relay of accepted package members
                └──requires──> Bounded initial-broadcast retry

v2.1 authoritative peer transport + successful-write evidence
└──requires──> Runtime package relay/retry scheduling and truthful observability

Package admission ──depends-on──> Current rolling floor
Package mutation ──depends-on──> Post-submit pressure trimming
Eviction/expiry/block cleanup ──requires──> Relay, unbroadcast, orphan and compact-input cleanup
Public/default relay claims ──conflict-with──> v2.2 scoped activation boundary
General package wire relay ──conflict-with──> Pinned Knots opportunistic 1p1c claim
```

### Dependency Notes

- **Package admission requires pressure policy first:** effective package feerate is compared with the current rolling floor, and a successful staged package must survive post-submit trimming.
- **P2P 1p1c requires v2.0 orphan/reject distinctions:** a low-fee parent must be remembered as reconsiderable, not permanently rejected, while truly invalid transactions remain suppressed.
- **Package relay requires v2.0 txid/wtxid fanout and v2.1 transport authority:** accepted members are announced as ordinary transactions, and success evidence must come from the network instance that actually emitted the wire message.
- **Initial-broadcast retry requires durable local-origin state:** normal peer-relayed transactions must never be added to the retry set; the set must survive restart and clean with every mempool removal path.
- **Pressure eviction affects compact reconstruction:** v2.1 reconstruction candidates must use the current post-eviction mempool snapshot, with no stale removed transactions presented as live candidates.
- **Recovery requires topological ordering:** v2.0 snapshots are currently txid-sorted; package ancestors must be replayed before descendants or recovery will falsely classify valid children as missing-parent drops.

## v2.2 Minimum Shippable Boundary

### Launch With (P1)

- [ ] Package shape types and context-free validation with exact Knots limits and errors.
- [ ] Deterministic dynamic-memory accounting, descendant-score trimming, rolling-floor bump/decay, and restart-reset semantics.
- [ ] Package-context dry-run and staged admission, including CPFP effective-fee groups, partial submission, replacement, TRUC, ephemeral-dust, and post-submit trim outcomes.
- [ ] `testmempoolaccept`/`submitpackage`-equivalent RPC outcomes or an explicitly documented narrower RPC compatibility boundary if endpoint work is separately phased.
- [ ] Opportunistic same-peer P2P 1p1c acceptance using the existing bounded orphan/download system.
- [ ] Per-transaction fanout/serving for accepted package members through existing relay activation and peer policy.
- [ ] Durable local unbroadcast state and randomized 10–15 minute initial-broadcast retry.
- [ ] Dependency-ordered mempool recovery plus relay/unbroadcast/compact-input cleanup for every removal reason.
- [ ] Shared operator status, metrics, logs, sanitized support evidence, parity roots, deterministic tests, UAT, and no-claim guardrails.

### Add After Core Validation (P2)

- [ ] Long-duration virtual-time pressure scenarios across multiple block/decay/restart cycles — add once unit transitions and runtime wiring agree.
- [ ] Opt-in public-network package propagation UAT — add only after deterministic 1p1c and retry evidence passes; never make it a default verification gate.
- [ ] Performance baselines for large valid/invalid 25-transaction packages and adversarial pressure churn — add before any performance or production-scale claim.

### Future Consideration (P3 / Out of v2.2)

- [ ] General package wire relay/BIP331 — defer because it is not the pinned Knots behavior activated here.
- [ ] Cluster mempool and broad cluster-linearization policy — defer to a dedicated policy milestone.
- [ ] Erlay/transaction reconciliation — separate negotiation, privacy, and reconciliation surface.
- [ ] Knots `mempool.dat` import/export — separate migration compatibility surface.
- [ ] Public relay defaults, production service operation, production full-node readiness, and production-funds wallet use — retain existing release gates.

## Feature Prioritization Matrix

| Feature | Operator/User Value | Implementation Cost | Priority |
| --- | --- | --- | --- |
| Package model and shape validation | HIGH | MEDIUM | P1 |
| Dynamic-memory pressure accounting | HIGH | HIGH | P1 |
| Rolling fee bump/decay/reset | HIGH | HIGH | P1 |
| Package-context staged admission | HIGH | HIGH | P1 |
| RPC dry-run/submission outcomes | HIGH | HIGH | P1 |
| Opportunistic P2P 1p1c | HIGH | HIGH | P1 |
| Per-transaction package fanout | HIGH | MEDIUM | P1 |
| Durable initial-broadcast retry | HIGH | MEDIUM | P1 |
| Dependency-ordered recovery and cleanup | HIGH | HIGH | P1 |
| Shared sanitized observability | HIGH | MEDIUM | P1 |
| Virtual-time long-run pressure harness | MEDIUM | HIGH | P2 |
| Opt-in live network UAT | MEDIUM | MEDIUM | P2 |
| General package wire relay | LOW for this milestone | HIGH | P3 |
| Cluster mempool/Erlay | LOW for this milestone | HIGH | P3 |

**Priority key:** P1 is required for the v2.2 claim; P2 strengthens evidence after the deterministic core is complete; P3 is explicitly deferred.

## Baseline Feature Analysis

| Capability | Pinned Knots `29.3.knots20260210` | Open Bitcoin Through v2.1 | Recommended v2.2 Approach |
| --- | --- | --- | --- |
| Package dry-run | Up to 25 topologically sorted, internally non-conflicting transactions with per-tx results | Single-transaction admission only | Implement package context and stable per-wtxid outcome groups. |
| Local package submission | Child with independent unconfirmed parents; partial acceptance possible; accepted members broadcast individually | No package submission surface | Match topology, staged mutation, partial-result, and broadcast behavior. |
| P2P package propagation | Opportunistic same-peer 1p1c using ordinary tx messages and orphan/reconsiderable caches | Bounded orphan parent requests, but package relay deferred | Extend the existing flow; do not add a package wire message. |
| Pressure cap | Dynamic memory usage under `maxmempool` | Total virtual size under `max_mempool_virtual_size` | Add dynamic usage accounting while retaining vsize as a separate fee/reporting measure. |
| Rolling minimum fee | Evicted descendant-package rate + increment; block-gated 12/6/3-hour decay | Fixed min/incremental fee evidence; rolling parity explicitly `deferred` | Implement an explicit-time pure state machine and surface current floor. |
| Restart floor behavior | Dynamic floor resets; transactions and unbroadcast set may persist | Typed transaction snapshot persists; no rolling floor exists | Preserve baseline reset, replay topology, and restore local unbroadcast membership. |
| Initial broadcast retry | Local unbroadcast set, randomized 10–15 minute retries until peer request | `rebroadcast_deferred` evidence only, no timer | Replace deferred marker with bounded scheduled outcomes through the authoritative transport. |
| Sustained-pressure cleanup | Descendant eviction/expiry and cross-cache cleanup | Basic descendant trim and lifecycle cleanup exist | Make every package/pressure removal propagate through relay, recovery, observability, and compact-input state. |

## Sources

### Pinned Knots Package Admission and Relay

- [`doc/policy/packages.md`](../../packages/bitcoin-knots/doc/policy/packages.md), [`policy/packages.h`](../../packages/bitcoin-knots/src/policy/packages.h), and [`policy/packages.cpp`](../../packages/bitcoin-knots/src/policy/packages.cpp) — package limits, ordering, consistency, child-with-parents shape, effective-fee policy, and limited package-RBF scope.
- [`validation.cpp`](../../packages/bitcoin-knots/src/validation.cpp) — `AcceptMultipleTransactions`, `AcceptPackage`, staged subpackage mutation, effective feerates, replacement, partial submission, and post-submit limiting.
- [`rpc/mempool.cpp`](../../packages/bitcoin-knots/src/rpc/mempool.cpp) — `testmempoolaccept`, `submitpackage`, RPC topology, result, max-fee, and broadcast contracts.
- [`node/txdownloadman.h`](../../packages/bitcoin-knots/src/node/txdownloadman.h), [`node/txdownloadman_impl.cpp`](../../packages/bitcoin-knots/src/node/txdownloadman_impl.cpp), and [`net_processing.cpp`](../../packages/bitcoin-knots/src/net_processing.cpp) — reconsiderable rejection, same-peer opportunistic 1p1c, ordinary per-transaction relay, and rejected-package caching.
- [`policy/truc_policy.cpp`](../../packages/bitcoin-knots/src/policy/truc_policy.cpp), [`policy/ephemeral_policy.cpp`](../../packages/bitcoin-knots/src/policy/ephemeral_policy.cpp), and [`policy/rbf.cpp`](../../packages/bitcoin-knots/src/policy/rbf.cpp) — package-context topology, ephemeral dust, sibling eviction, and replacement policy.
- [`rpc_packages.py`](../../packages/bitcoin-knots/test/functional/rpc_packages.py), [`p2p_opportunistic_1p1c.py`](../../packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py), [`p2p_1p1c_network.py`](../../packages/bitcoin-knots/test/functional/p2p_1p1c_network.py), [`mempool_truc.py`](../../packages/bitcoin-knots/test/functional/mempool_truc.py), and [`mempool_ephemeral_dust.py`](../../packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py) — externally observable package behavior.

### Pinned Knots Pressure, Retry, and Recovery

- [`txmempool.h`](../../packages/bitcoin-knots/src/txmempool.h) and [`txmempool.cpp`](../../packages/bitcoin-knots/src/txmempool.cpp) — dynamic-memory trimming, descendant-score eviction, rolling fee bump/decay, expiry, unbroadcast state, and block-gated decay.
- [`kernel/mempool_options.h`](../../packages/bitcoin-knots/src/kernel/mempool_options.h) and [`node/mempool_args.cpp`](../../packages/bitcoin-knots/src/node/mempool_args.cpp) — `maxmempool`, 336-hour default expiry, min/incremental relay fee, and configuration semantics.
- [`node/transaction.cpp`](../../packages/bitcoin-knots/src/node/transaction.cpp), [`node/mempool_persist.cpp`](../../packages/bitcoin-knots/src/node/mempool_persist.cpp), and [`net_processing.cpp`](../../packages/bitcoin-knots/src/net_processing.cpp) — local unbroadcast insertion, persistence, acknowledgement cleanup, and randomized 10–15 minute retry.
- [`mempool_limit.py`](../../packages/bitcoin-knots/test/functional/mempool_limit.py), [`mempool_expiry.py`](../../packages/bitcoin-knots/test/functional/mempool_expiry.py), [`mempool_unbroadcast.py`](../../packages/bitcoin-knots/test/functional/mempool_unbroadcast.py), and [`mempool_persist.py`](../../packages/bitcoin-knots/test/functional/mempool_persist.py) — pressure, expiry, retry, and restart outcomes.

### Existing Open Bitcoin Foundations

- [`docs/parity/catalog/mempool-policy.md`](../../docs/parity/catalog/mempool-policy.md) — v2.0 capability and explicit rolling-fee/package/rebroadcast gaps.
- [`open-bitcoin-mempool/src/pool.rs`](../../packages/open-bitcoin-mempool/src/pool.rs), [`pool/lifecycle.rs`](../../packages/open-bitcoin-mempool/src/pool/lifecycle.rs), and [`types.rs`](../../packages/open-bitcoin-mempool/src/types.rs) — current single-transaction topology, vsize trim, pressure summary, and deferred rolling-fee marker.
- [`transaction_relay/fanout.rs`](../../packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs), [`orphanage.rs`](../../packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs), and [`scheduler.rs`](../../packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs) — current bounded fanout, deferred rebroadcast marker, orphan handling, and transaction download scheduling.
- [`storage/mempool_snapshot.rs`](../../packages/open-bitcoin-node/src/storage/mempool_snapshot.rs) and [`network/mempool_lifecycle.rs`](../../packages/open-bitcoin-node/src/network/mempool_lifecycle.rs) — current typed recovery and cross-runtime cleanup foundations.
- [v2.0 requirements](../milestones/v2.0-REQUIREMENTS.md) and [v2.1 requirements](../milestones/v2.1-REQUIREMENTS.md) — shipped activation, resource, transport, observability, and release boundaries that v2.2 must preserve.

## Confidence and Open Questions

| Area | Confidence | Remaining Decision |
| --- | --- | --- |
| Package limits/topology/RPC outcomes | HIGH | Decide whether exact RPC endpoint parity ships in the same phase as the core model or immediately after it; the milestone claim should not precede it. |
| Opportunistic P2P 1p1c | HIGH | Decide exact bounded cache sizes/expiry by mapping current Open Bitcoin limits to Knots’ rolling filters and orphanage without weakening v2.0 governance. |
| Rolling fee and pressure | HIGH | Define an auditable dynamic-memory estimator for Rust allocations; exact allocator byte identity is unrealistic, but the admission/eviction ordering and external fields must be stable and documented. |
| Restart semantics | HIGH | Confirm the milestone requirement wording says the rolling floor resets at restart while transaction/unbroadcast state persists; “fee-floor persistence” would be a deliberate Knots deviation. |
| TRUC/ephemeral-dust package policy | HIGH for baseline existence, MEDIUM for current Open Bitcoin prerequisites | These rules materially expand package policy; phase planning should inventory which underlying single-transaction checks already exist before estimating implementation. |
| Public/default relay boundary | HIGH | No open product decision inside v2.2: keep default/public/production claims deferred. |

***
*Feature research for: Open Bitcoin v2.2 Package Relay and Long-Lived Mempool Policy*
*Researched: 2026-07-22*
