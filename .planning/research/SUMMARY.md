# Project Research Summary

**Project:** Open Bitcoin
**Milestone:** v2.2 Package Relay and Long-Lived Mempool Policy
**Domain:** Knots-compatible Bitcoin package admission, transaction relay, and sustained mempool policy
**Researched:** 2026-07-22
**Confidence:** HIGH

## Executive Summary

Open Bitcoin v2.2 is a policy-and-runtime milestone, not a new peer protocol. Experts build this capability by keeping package validation, fee policy, eviction, and lifecycle decisions in a deterministic mempool core; keeping peer eligibility, orphan/reconsiderable state, fanout, and serving in the network core; and letting a thin daemon shell supply time, randomness, storage, sockets, and operator projections. The pinned Bitcoin Knots `29.3.knots20260210` baseline supports broad local package evaluation but only opportunistic same-peer 1-parent/1-child package handling over ordinary `inv`/`getdata`/`tx` messages. v2.2 should match that boundary exactly rather than introduce package wire messages or imply general package relay.

The recommended implementation adds no external production dependency. Extend the first-party Rust crates, existing Fjall mempool snapshot, Tokio daemon shell, v2.0 transaction relay machinery, and v2.1 authoritative transport. Build pressure accounting, descendant-package eviction, and the rolling fee state machine before package admission because package feerate decisions depend on the active dynamic floor and final trimming. Route every mutation, maintenance tick, snapshot, RPC command, and transport receipt through the sole `ManagedNetworkHandle` authority; persist source records, acceptance times, and the local unbroadcast set, while rebuilding derived indexes and resetting the rolling fee on restart for Knots parity.

The largest risks are false atomicity, graph/cache divergence, virtual-size-only pressure claims, premature delivery evidence, split runtime authority, and scope creep. Mitigate them with typed package shapes, staged subpackage commits, one lifecycle delta across all caches, deterministic accounted-memory and rolling-fee fixtures, bounded receive-independent maintenance, serve/write-backed receipts, sanitized low-cardinality evidence, and positive/negative release claim checks. Default verification must remain deterministic and hermetic; public-network review stays explicit and opt-in, and public/default/production relay claims remain deferred.

## Key Findings

### Scope and Key Decisions

- Implement local package dry-run and child-with-unconfirmed-parents submission, including ordered per-wtxid outcomes, individual-first partial acceptance, effective-fee groups, post-trim result rewriting, and the pinned package policy boundaries.
- Implement peer-originated package handling only as bounded, sender-aware same-peer 1P1C assembly over ordinary transaction messages. Do not add a package inventory type or wire command.
- Relay each accepted and still-present package member in topological order through existing txid/wtxid fanout, peer policy, queue, rate, serving, and activation controls. Package admission never promises atomic or guaranteed network propagation.
- Separate the configured static relay floor from the rolling mempool floor. Package aggregate feerate may satisfy only the eligible dynamic-floor rule; it must not silently waive `minrelaytxfee`.
- Treat capacity as accounted dynamic memory, not transaction vsize. Preserve vsize as a separate fee and reporting measure.
- Name the retry feature **initial broadcast retry**. Retry only bounded, locally submitted, relay-requested transactions on fresh randomized 10–15 minute cycles and clear them only at the documented eligible serve/write or lifecycle-removal boundary.
- Persist transactions, acceptance times, and surviving local unbroadcast membership. Rebuild topology and volatile peer state, and reset—not persist—the rolling minimum fee on restart unless a documented intentional deviation is approved.
- Keep public relay defaults, general package wire relay, guaranteed propagation, cluster mempool, public-network default verification, production full-node readiness, and production-funds wallet use out of scope.

### Recommended Stack

No stack replacement or package installation is needed. The milestone should reuse the existing workspace and make narrowly bounded first-party additions described in [STACK.md](./STACK.md).

**Core technologies:**

- Rust `1.94.1`, edition 2024 — typed package, fee, pressure, lifecycle, and scheduling transitions in the existing first-party crates.
- Bitcoin Knots `29.3.knots20260210` — pinned behavioral and fixture authority; do not import behavior from newer Core or Knots versions.
- Standard-library collections plus first-party SHA-256 — canonical mempool graph, deterministic indexes, bounded queues/filters, and exact package identity without a graph or hashing dependency.
- Fjall `3.1.4` with serde/serde_json — extend the existing versioned mempool snapshot instead of creating a second store or rebroadcast journal.
- Tokio `1.52.1` and existing `getrandom` in the shell — receive-independent wakeups and fresh jitter; never put wall-clock, randomness, or Tokio into the pure mempool/network policy crates.
- Existing `open-bitcoin-bench` and `bash scripts/verify.sh` — deterministic pressure/performance evidence and the unchanged repository verification contract.

**Dependency rule:** Add no external production dependency unless implementation or profiling demonstrates a concrete capability gap, followed by maintenance/security review and Cargo/Bazel integration evidence.

### Expected Features

The detailed feature contract is in [FEATURES.md](./FEATURES.md).

**Must have (table stakes):**

- Context-free package validation at exact pinned limits: non-empty, at most 25 transactions, at most 404,000 weight units, unique identities, topological order, and no internal input conflicts.
- `testmempoolaccept`-equivalent dry-run and `submitpackage`-equivalent child-with-unconfirmed-parents outcomes, or an explicitly narrower RPC claim until those endpoints ship.
- Staged package admission with correct individual-first partial results, package feerate groups, package RBF/TRUC/ephemeral-dust scope, one coherent subpackage commit, and accurate post-trim outcomes.
- Bounded same-peer P2P 1P1C reconstruction from reconsiderable parent and orphan-child evidence, followed by ordinary per-transaction relay.
- Deterministic dynamic-memory accounting, descendant-score package eviction, rolling fee bump/block gate/12–6–3-hour decay, expiry, and complete descendant cleanup.
- Dependency-ordered snapshot recovery with acceptance time and unbroadcast membership, plus rolling-floor restart reset.
- Bounded randomized initial-broadcast retry through the existing relay and authoritative transport path.
- Shared RPC/CLI/dashboard/status, metrics, logs, and support evidence with stable low-cardinality outcomes, redaction, and achieved-effect lineage.
- Deterministic Knots fixtures, virtual-time sustained-pressure scenarios, parity breadcrumbs, repo-local UAT commands, and release no-claim checks.

**Should have (competitive and auditability differentiators):**

- Typed package-wide and per-wtxid decision models that make partial acceptance and effective-fee membership reviewable.
- A pure pressure state machine with explicit time, block, occupancy, and fee inputs.
- One package-to-admission-to-fanout-to-successful-write evidence lineage without leaking transaction or peer identities.
- Typed recovery/drop evidence and checkpoint freshness rather than silent replay loss.
- A deterministic adversarial harness spanning package bursts, eviction, decay, expiry, reconnect, retry, checkpoint failure, restart, and reorg.
- An explicit claim taxonomy distinguishing local package admission, opportunistic P2P 1P1C, ordinary transaction fanout, and initial broadcast retry.

**Defer beyond v2.2:**

- BIP331/general package wire relay, arbitrary multi-parent P2P reconstruction, cluster mempool, Erlay, bloom/filter expansion, and Knots `mempool.dat` import/export.
- Whole-mempool or wallet-wide rebroadcast as part of node relay policy.
- Public/default relay, public-network CI, production-scale or guaranteed-propagation claims, production service operation, production full-node readiness, and production-funds wallet use.

### Observable Operator Outcomes

- A valid fee-bumping parent/child package reports ordered per-wtxid results, enters the mempool coherently, and queues each surviving member through ordinary relay when relay is explicitly active.
- A valid parent plus invalid child can leave the parent accepted and relay-eligible while returning a non-success package result and a precise child rejection.
- Mempool pressure reports vsize, accounted usage, capacity, the static relay floor, effective `mempoolminfee`, occupancy/decay state, and descendant-package eviction counts as distinct concepts.
- A post-bump clock advance without a connected block does not decay the floor; after a block, decay follows the pinned occupancy-sensitive half-life and rounding rules.
- Restart topologically replays durable entries, restores only surviving local unbroadcast markers, reports dropped/recovered classifications and checkpoint freshness, and starts from the pinned rolling-fee restart baseline.
- Initial broadcast retry distinguishes due, eligible, queued, attempted, emitted, requested, served, suppressed, and cleared. Admission or queueing alone is never reported as public propagation.
- Relay-disabled or peer-policy-suppressed operation may still admit locally while truthfully reporting that no public/default relay claim was achieved.

### Architecture Approach

The architecture in [ARCHITECTURE.md](./ARCHITECTURE.md) extends the existing functional core/imperative shell and forbids parallel authorities. All stateful decisions remain inside `ManagedPeerNetwork` behind cloned references to the same `ManagedNetworkHandle`; effectful adapters capture owned commands or snapshots under the authority, release the lock, perform storage/network work, then feed typed receipts back through a short mutation. One staged `MempoolDelta` must drive mempool graph changes and every relay, serving, orphan/reject, compact reconstruction, unbroadcast, persistence, and evidence consequence.

**Major components:**

1. `open-bitcoin-mempool` — package types and admission, static/dynamic fee separation, accounted usage, rolling fee, expiry, topology, descendant eviction, and coherent lifecycle deltas.
2. `open-bitcoin-network` — hard versus reconsiderable rejection, bounded same-peer 1P1C candidates, ordinary topological transaction fanout, serving, per-peer limits, and pure retry decisions.
3. `open-bitcoin-node` and `ManagedNetworkHandle` — sole mutation authority, cross-cache delta application, snapshot capture/recovery, maintenance commands, transport receipt application, and shared operator snapshot.
4. `open-bitcoin-rpc`/`open-bitcoind` shell — package RPC parsing/projection, injected clocks and jitter, bounded receive-independent scheduling, Fjall I/O, live transport, and shutdown-aware checkpoint coordination.
5. Operator and verification adapters — RPC/CLI/dashboard/status, fixed-cardinality metrics/logs/support bundles, parity catalogs, deterministic fixtures, benchmarks, UAT, and claim guardrails.

### Critical Pitfalls

1. **Treating the whole package call as atomic** — preserve individual-first results, stage only the matching subpackage, commit once, trim afterward, and rewrite outcomes for members removed by final pressure.
2. **Letting topology or dependent caches diverge** — maintain canonical entry, spent-outpoint, adjacency, aggregate, eviction, serving, fanout, compact, orphan, unbroadcast, persistence, and evidence changes through one tested lifecycle delta.
3. **Collapsing fee floors or claiming vsize-only pressure parity** — use distinct static, incremental, rolling, and effective fee concepts plus an auditable accounted-memory model and deterministic Knots vectors.
4. **Persisting the wrong state or lacking a real checkpoint path** — persist source records, entry time, and unbroadcast membership; topologically replay; rebuild derived state; reset rolling fee; prove periodic and clean-shutdown behavior without a broad global schema break.
5. **Whole-mempool retry or premature delivery completion** — retry only bounded local unbroadcast entries with fresh jitter and ordinary peer limits, and clear only on the specified serve/write receipt or lifecycle removal.
6. **Recreating split authority** — forbid mempool, rolling-fee, package-candidate, or unbroadcast ownership in RPC/background workers; all commands and snapshots pass through the existing `ManagedNetworkHandle`.
7. **Overbroad protocol, observability, or product claims** — no new package wire messages, unbounded state, dynamic metric labels, identifier leakage, queued-as-achieved evidence, live default verification, or public/default/production relay language.

## Implications for Roadmap

Based on the shared dependency analysis, use nine phases. This is the smallest ordering that gives each high-risk invariant a clear completion and verification boundary.

### Phase 1: Resource, Time, and Fee Primitives

**Rationale:** Every later admission, pressure, expiry, persistence, and operator decision depends on unambiguous primitives.
**Delivers:** Separate static/incremental/rolling/effective fee types; accounted usage separate from vsize; entry acceptance time; monotonic/injected time handling; typed removal reasons and evidence enums.
**Addresses:** Dynamic-memory pressure, operator fee truth, expiry/recovery prerequisites.
**Avoids:** One overloaded fee field, wall-clock reads in pure code, and a vsize value mislabeled as memory usage.

### Phase 2: Rolling Fee, Expiry, and Descendant Eviction Core

**Rationale:** Package admission must evaluate against the real dynamic floor and survive the final pressure policy.
**Delivers:** Pure bump/block-gate/decay state machine; 12/6/3-hour occupancy behavior; strict time and rounding fixtures; accounted-cap enforcement; deterministic descendant-score eviction; invariant oracle and pressure benchmarks.
**Addresses:** Rolling minimum fee, sustained pressure, expiry, bounded memory.
**Avoids:** Immediate decay, wrong victim/floor calculations, repeated whole-map scans, and incomplete descendant removal.

### Phase 3: Typed Package Vocabulary and Staged Admission

**Rationale:** With pressure semantics stable, package evaluation can return truthful final membership and fee results.
**Delivers:** General and child-with-unconfirmed-parents package types; exact shape limits; dry-run; individual-first evaluation; staged subpackage commit; effective-fee groups; replacement/TRUC/ephemeral-dust boundary; ordered per-wtxid results and post-trim rewriting.
**Addresses:** Local package validation/submission and correct partial acceptance.
**Avoids:** Repeated single-transaction mutation, global all-or-nothing semantics, topology drift, and package feerate bypass of the static floor.

### Phase 4: Package-Aware Download and Orphan Bridge

**Rationale:** P2P behavior should consume the proven shared admission engine while transport remains unchanged.
**Delivers:** Hard versus reconsiderable rejection, bounded sender-aware same-peer 1P1C construction, exact package hash, rotating reject state, and typed admission commands.
**Addresses:** Opportunistic pinned P2P package acceptance.
**Avoids:** New package wire commands, arbitrary multi-parent assembly, lost sender attribution, and unbounded reject caches.

### Phase 5: Authoritative Cross-Cache Lifecycle Integration

**Rationale:** Persistence and retry are unsafe until every admission and removal reason has one coherent system-wide consequence.
**Delivers:** `ManagedNetworkHandle` package commands; one delta for admission, replacement, pressure, expiry, block connect, and reorg; synchronized serving, fanout, peer, compact, orphan/package, unbroadcast, persistence-dirty, and evidence state.
**Addresses:** Package-safe mutation, ordinary topological fanout preparation, and complete sustained-pressure cleanup.
**Avoids:** Split authority, stale compact/relay state, async or storage work under the authority lock, and partial cross-cache commits.

### Phase 6: Snapshot Schema, Checkpointing, and Recovery

**Rationale:** Durable retry and long-lived policy require source-of-truth recovery before the scheduler can act safely after restart.
**Delivers:** Acceptance time and unbroadcast fields; mempool-scoped compatibility strategy; owned snapshot capture; coalesced periodic and clean-shutdown checkpoints; topological replay; typed recovery/drop evidence; restored surviving unbroadcast state; rebuilt indexes and rolling-floor reset.
**Addresses:** Restart/recovery and durability truth.
**Avoids:** Txid-order replay, stale derived state, global schema collateral damage, missing production save calls, and undocumented crash-loss guarantees.

### Phase 7: Receive-Independent Maintenance and Transport Receipts

**Rationale:** Long-lived behavior must progress on idle nodes, and success evidence must come from the authoritative transport.
**Delivers:** Bounded maintenance ticks; fresh 10–15 minute jitter; initial-broadcast retry through existing fanout and peer policy; generalized v2.1 outboxes; request/serve or successful-write receipts; lifecycle clearing and restart behavior.
**Addresses:** Expiry/decay scheduling, ordinary per-transaction package fanout, and bounded local initial-broadcast retry.
**Avoids:** Message-driven-only maintenance, fixed cadence, whole-mempool rebroadcast, queueing-as-success, and retry paths that bypass bounds.

### Phase 8: RPC and Sanitized Operator Evidence

**Rationale:** Expose only semantics that the completed authority and transport paths can prove.
**Delivers:** `testmempoolaccept`/`submitpackage` and mempool-info projections; CLI/dashboard/status fields; fixed-cardinality metrics and logs; redacted support evidence; package, fee, pressure, recovery, checkpoint, and retry outcome lineage.
**Addresses:** Operator-visible package and long-lived mempool truth.
**Avoids:** One package success flag, dynamic labels, identifier/peer leakage, stale independent snapshots, and eligible/queued actions described as achieved effects.

### Phase 9: Parity, Adversarial Pressure, Restart, and Release Guardrails

**Rationale:** The milestone claim requires integrated proof across pure policy, runtime authority, persistence, transport, and documentation.
**Delivers:** Pinned differential vectors; virtual-time long-run scenarios; randomized graph oracle tests; pressure and package benchmarks; restart/failure injection; authority provenance checks; exact parity roots; repo-local Cargo/Bazel UAT; positive and negative claim fixtures.
**Addresses:** Deterministic verification, auditability, performance evidence, and scoped release language.
**Avoids:** Public-network/default test dependencies, production-scale claims from short tests, missing parity evidence, and silent expansion to general/public/production relay.

### Phase Ordering Rationale

- Resource and fee semantics precede package admission because the current rolling floor and post-submit trim determine package acceptance and final per-member results.
- The shared package engine precedes the P2P bridge, and the P2P bridge deliberately precedes transport changes, preventing protocol invention from driving core policy.
- Cross-cache lifecycle integration precedes persistence and retry so neither recovered nor scheduled transactions can bypass removal invariants.
- Recovery precedes the scheduler because retry eligibility depends on durable local-origin state and current recovered membership.
- Operator projection follows achieved-effect transport wiring so status cannot confuse eligibility, queueing, emission, request, and service.
- Integrated adversarial and claim verification closes the milestone only after all state owners and evidence paths are final.

### Research Flags

**Phases requiring targeted deeper research during planning:**

- **Phase 2:** Define and benchmark the Rust accounted-memory model and the observable tolerance relative to Knots `DynamicMemoryUsage()` before making pressure-parity claims.
- **Phase 3:** Inventory current single-transaction prerequisites for package RBF, TRUC, and ephemeral-dust policy. Include the pinned rules or emit typed unsupported/deferred outcomes and narrow the claim.
- **Phase 6:** Choose a mempool-scoped snapshot compatibility design, periodic checkpoint cadence, clean-shutdown strength, and stated sudden-crash loss window without casually bumping the global schema.
- **Phase 7:** Specify the exact unbroadcast completion receipt. It must be no earlier than eligible transaction service and should align the pinned behavior with v2.1's stronger achieved-effect evidence.

**Phases with well-documented patterns; skip broad research-phase:**

- **Phase 1:** Types, entry time, explicit clocks, and fee separation are directly specified by current code and pinned sources.
- **Phase 4:** The pinned same-peer 1P1C flow and ordinary-message boundary are explicit in Knots source/tests; planning needs mapping, not ecosystem discovery.
- **Phase 5:** The repository already established `ManagedNetworkHandle`, lifecycle bridges, and authoritative transport patterns in v2.0/v2.1.
- **Phase 8:** Existing shared status, metric, logging, support-bundle, and redaction contracts provide the implementation pattern.
- **Phase 9:** Repo-native verification, parity breadcrumbs, UAT forms, and no-claim checkers are established; this phase needs scenario design rather than technology research.

## Confidence Assessment

| Area | Confidence | Notes |
| --- | --- | --- |
| Stack | HIGH | Exact workspace/tool versions, dependencies, current seams, and Knots baseline were verified from local primary sources; no new dependency is needed. |
| Features | HIGH | Package, RPC, P2P 1P1C, rolling fee, eviction, persistence, and unbroadcast behavior agree across pinned implementation and tests. |
| Architecture | HIGH overall; MEDIUM for scheduler details | Sole-authority and pure-core boundaries are established; exact checkpoint cadence and final transport receipt require requirements decisions. |
| Pitfalls | HIGH overall; MEDIUM for memory/durability tolerances | Failure modes are grounded in current code gaps and pinned behavior; Rust memory-accounting tolerance and crash guarantees remain design decisions. |

**Overall confidence:** HIGH

### Gaps to Address

- **Rust memory accounting:** Define a deterministic owned estimator, operator definitions, benchmark behavior, and the exact claim tolerance; byte-for-byte C++ allocator parity is not realistic.
- **Package RBF/TRUC/ephemeral-dust scope:** Confirm required prerequisites before roadmap estimation. The milestone must not claim full package admission parity while silently omitting them.
- **Snapshot compatibility and crash durability:** Decide mempool-local versioning, checkpoint cadence, sync strength, failure retry, clean-shutdown behavior, and the advertised crash-loss window.
- **Unbroadcast acknowledgement:** Choose eligible enqueue/serve versus successful wire write based on the existing v2.1 receipt model; never clear on inventory queueing.
- **RPC sequencing:** Ship exact package RPC behavior before the broad v2.2 package-admission claim, even if the adapters are planned after the core.
- **Performance thresholds:** Establish benchmark baselines for 25-member packages, long/wide graphs, repeated pressure churn, maintenance scans, and checkpoint work before any sustained-scale statement.

## Sources

### Primary (HIGH confidence)

- [STACK.md](./STACK.md) — current dependency seams, exact versions, and no-new-dependency recommendation.
- [FEATURES.md](./FEATURES.md) — pinned package/pressure/retry behavior, operator outcomes, and minimum shippable boundary.
- [ARCHITECTURE.md](./ARCHITECTURE.md) — sole-authority component model, data flows, integration seams, and dependency-aware build order.
- [PITFALLS.md](./PITFALLS.md) — critical failure modes, phase mapping, recovery strategies, and claim risks.
- `packages/bitcoin-knots` at tag `v29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878` — authoritative package policy, validation, transaction download/relay, mempool pressure, persistence, RPC, and functional/unit fixtures.
- Current `open-bitcoin-mempool`, `open-bitcoin-network`, `open-bitcoin-node`, `open-bitcoin-rpc`, and `open-bitcoin-bench` sources — existing admission, relay, persistence, authority, transport, observability, and verification seams.
- [.planning/PROJECT.md](../PROJECT.md), v2.0/v2.1 milestone requirements, parity catalogs, and repository standards — product, activation, architecture, evidence, and release boundaries.

### Secondary and Tertiary

No secondary or tertiary sources materially drive the recommendations. Open questions are implementation-contract decisions to validate against the local primary sources, not unresolved ecosystem claims.

***
*Research completed: 2026-07-22*
*Ready for roadmap: yes*
