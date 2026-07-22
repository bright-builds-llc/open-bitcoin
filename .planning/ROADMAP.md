# Roadmap: Open Bitcoin

## Current Status

v2.2 Package Relay and Long-Lived Mempool Policy is planned across Phases 130–138. All 40 requirements are mapped exactly once; Phase 130 is ready for discussion.

## Milestones

- ✅ **v1.0 Headless Parity** — 22 phase entries, including inserted closure phases (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** — Phases 13–34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Full Mainnet Network Syncing** — Phases 35–41 (shipped 2026-05-23). Archive: [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Public Mainnet Sync Proof and Node Hardening** — Phases 42–53 (shipped 2026-06-02). Archive: [v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md)
- ✅ **v1.4 Mainnet IBD Convergence and Peer Compatibility** — Phases 54–59 (shipped 2026-06-05). Archive: [v1.4-ROADMAP.md](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Unattended Mainnet Node Operation Readiness** — Phases 60–67 (shipped 2026-06-10). Archive: [v1.5-ROADMAP.md](milestones/v1.5-ROADMAP.md)
- ✅ **v1.6 Mainnet Full-Sync Completion** — Phases 68–74 (shipped 2026-06-14). Archive: [v1.6-ROADMAP.md](milestones/v1.6-ROADMAP.md)
- ✅ **v1.7 Full-Sync Soak and Recovery Hardening** — Phases 75–81 (shipped 2026-06-20). Archive: [v1.7-ROADMAP.md](milestones/v1.7-ROADMAP.md)
- ✅ **v1.8 Production Full-Node Readiness Boundary** — Phases 82–89 (shipped 2026-06-25). Archive: [v1.8-ROADMAP.md](milestones/v1.8-ROADMAP.md)
- ✅ **v1.9 Inbound Peer Serving and Network Participation Boundary** — Phases 90–99 (shipped 2026-06-29). Archive: [v1.9-ROADMAP.md](milestones/v1.9-ROADMAP.md)
- ✅ **v2.0 Transaction Relay and Mempool Participation Boundary** — Phases 100–109 (shipped 2026-07-03). Archive: [v2.0-ROADMAP.md](milestones/v2.0-ROADMAP.md)
- ✅ **v2.1 Block Serving and Compact Block Relay Boundary** — Phases 110–129 (shipped 2026-07-22). Archive: [v2.1-ROADMAP.md](milestones/v2.1-ROADMAP.md)
- 🚧 **v2.2 Package Relay and Long-Lived Mempool Policy** — Phases 130–138 (planned)

## Active Milestone: v2.2 Package Relay and Long-Lived Mempool Policy

**Milestone Goal:** Extend the bounded v2.0 transaction-relay and mempool foundation with Knots-aligned package admission and opportunistic same-peer 1P1C relay, deterministic long-lived pressure policy, durable recovery, and bounded initial broadcast retry.

**Boundary:** v2.2 does not add a general package wire protocol, arbitrary multi-parent peer assembly, cluster mempool, whole-mempool rebroadcast, public/default relay, guaranteed propagation, public-network default verification, production service operation, production full-node readiness, or production-funds wallet claims.

## Phases

- [ ] **Phase 130: Resource, Time, and Fee Primitives** — Establish unambiguous accounting, fee, time, metadata, and lifecycle outcome contracts for all later policy behavior.
- [ ] **Phase 131: Rolling Fee, Expiry, and Descendant Eviction Core** — Enforce sustained-pressure capacity, eviction, expiry, and block-gated rolling-fee decay deterministically.
- [ ] **Phase 132: Typed Package Vocabulary and Staged Admission** — Deliver dry-run and submission semantics with exact shape validation, partial acceptance, effective-fee grouping, and coherent commits.
- [ ] **Phase 133: Package-Aware Download and Orphan Bridge** — Assemble bounded same-peer 1P1C candidates over ordinary transaction messages and route them to shared package admission.
- [ ] **Phase 134: Authoritative Cross-Cache Lifecycle Integration** — Make one runtime authority and lifecycle delta govern every package and mempool consequence across dependent state.
- [ ] **Phase 135: Snapshot Schema, Checkpointing, and Recovery** — Persist source mempool records and local unbroadcast state, then recover them through policy-aware topological replay.
- [ ] **Phase 136: Receive-Independent Maintenance and Transport Receipts** — Run bounded initial broadcast retry and topological package fanout through existing relay and achieved-effect transport paths.
- [ ] **Phase 137: RPC and Sanitized Operator Evidence** — Expose package, pressure, recovery, checkpoint, and retry truth through stable redacted operator surfaces.
- [ ] **Phase 138: Parity, Adversarial Pressure, Restart, and Release Guardrails** — Prove integrated Knots parity, bounded work, restart safety, and the deliberately narrow v2.2 claim.

## Phase Details

### Phase 130: Resource, Time, and Fee Primitives

**Goal**: Operators and contributors can reason about mempool capacity, fee floors, time-dependent policy, and lifecycle results through explicit, non-overloaded contracts.
**Depends on**: Phase 129
**Requirements**: FEEP-01, FEEP-02, FEEP-03, FEEP-04, FEEP-05
**Success Criteria** (what must be TRUE):

1. Operator evidence reports transaction virtual size, accounted mempool memory usage, and configured capacity as separate values.
2. Operators can distinguish the static relay floor, incremental relay fee, rolling mempool floor, and effective admission floor, and package fees cannot bypass the wrong boundary.
3. Expiry, recovery, and retry outcomes consistently use explicit acceptance time plus typed local-origin and relay-request metadata.
4. Contributors can reproduce admission, replacement, expiry, pressure, block, reorg, and retry decisions from explicit time, block, occupancy, and jitter inputs with stable typed outcomes.

**Plans**: TBD

### Phase 131: Rolling Fee, Expiry, and Descendant Eviction Core

**Goal**: The mempool remains bounded and internally consistent during sustained pressure while its rolling fee follows pinned Knots bump and decay behavior.
**Depends on**: Phase 130
**Requirements**: PRESS-01, PRESS-02, PRESS-03, PRESS-04, PRESS-05
**Success Criteria** (what must be TRUE):

1. Operators see capacity enforced from accounted memory usage while virtual size remains a distinct fee and reporting measure.
2. Pressure removes complete descendant packages in pinned descendant-score order and raises the rolling floor from the package actually evicted.
3. The rolling floor does not decay before a block is connected and then follows the pinned 12-hour, 6-hour, or 3-hour half-life and rounding behavior for current occupancy.
4. Expiry and pressure removal leave no stale descendants or derived indexes and preserve graph and fee-aggregate invariants.
5. Deterministic fill, trim, block, decay, expiry, refill, and reorg scenarios remain within documented resource and performance bounds and agree with recomputation.

**Plans**: TBD

### Phase 132: Typed Package Vocabulary and Staged Admission

**Goal**: Operators can dry-run and submit bounded transaction packages with truthful ordered outcomes, correct fee policy, partial acceptance, and coherent final membership.
**Depends on**: Phase 131
**Requirements**: PACK-01, PACK-02, PACK-03, PACK-04, PACK-05, PACK-06, PACK-07
**Success Criteria** (what must be TRUE):

1. Invalid packages are rejected before expensive work when empty, oversized, duplicated, non-topological, or internally conflicting.
2. Operators can dry-run a package and receive ordered per-transaction results without changing mempool, relay, persistence, or evidence state.
3. Operators can submit a child-with-unconfirmed-parents package and receive package-wide status, ordered final member outcomes, and effective-fee membership.
4. A valid parent can remain accepted when its child fails, while every accepted subpackage commits through one coherent delta and failed preparation leaves no partial mutation.
5. Final results reflect post-trim membership and the scoped replacement, TRUC, ephemeral-dust, witness-identity, reconsiderable-failure, static-floor, and rolling-floor boundaries.

**Plans**: TBD

### Phase 133: Package-Aware Download and Orphan Bridge

**Goal**: Peer-originated reconsiderable transactions can form only the pinned bounded same-peer 1P1C candidate and reuse authoritative package policy without a new wire protocol.
**Depends on**: Phase 132
**Requirements**: PPKG-01, PPKG-02, PPKG-03
**Success Criteria** (what must be TRUE):

1. Peer evidence distinguishes hard rejects from reconsiderable package candidates while retaining only bounded, rotating candidate and reject state.
2. Ordinary transaction messages can assemble only sender-aware same-peer one-parent/one-child candidates with preserved origin and exact package identity.
3. Peer candidates receive the same authoritative package-admission outcomes as local submissions; no package wire message or arbitrary multi-parent assembly is introduced.

**Plans**: TBD

### Phase 134: Authoritative Cross-Cache Lifecycle Integration

**Goal**: Every package or mempool mutation has one authoritative, complete consequence across serving, relay, peer, compact, retry, persistence, and evidence state.
**Depends on**: Phase 133
**Requirements**: MPLIFE-01, MPLIFE-02, MPLIFE-03, MPLIFE-04
**Success Criteria** (what must be TRUE):

1. Package admission, pressure policy, maintenance, snapshots, relay queues, and transport receipts mutate runtime state only through `ManagedNetworkHandle`.
2. One lifecycle delta projects admissions and removals into serving, fanout, peer request/known state, orphan/package candidates, compact inputs, unbroadcast state, persistence dirtiness, and operator evidence.
3. Replacement, pressure eviction, expiry, block connection, reorg, and failed admission leave no stale descendant or accepted-identity entries in dependent caches.
4. Storage and network work occurs after runtime authority is released, and bounded typed receipts are applied through a short follow-up mutation.

**Plans**: TBD

### Phase 135: Snapshot Schema, Checkpointing, and Recovery

**Goal**: Supported restarts recover valid mempool source state and local initial-broadcast intent truthfully without persisting volatile or derived policy state.
**Depends on**: Phase 134
**Requirements**: MPDUR-01, MPDUR-02, MPDUR-03, MPDUR-04
**Success Criteria** (what must be TRUE):

1. Durable snapshots preserve canonical transactions, acceptance times, and surviving locally submitted unbroadcast membership without persisting peer, topology, or rolling-fee derivatives.
2. Recovery validates and topologically replays records against current chainstate and policy, rebuilds indexes, and reports typed recovered and dropped classifications.
3. Restart resets the rolling floor to the pinned baseline while preserving supported entry age and surviving local-unbroadcast semantics.
4. Operators can see checkpoint freshness, dirty generation, persistence strength, and the documented crash-loss window for periodic and clean-shutdown saves, with no runtime-authority lock held across I/O.

**Plans**: TBD

### Phase 136: Receive-Independent Maintenance and Transport Receipts

**Goal**: Idle and active nodes perform bounded initial broadcast retry and ordinary topological package fanout through existing activation, peer-policy, queue, serving, and transport controls.
**Depends on**: Phase 135
**Requirements**: PPKG-04, IBR-01, IBR-02, IBR-03, IBR-04
**Success Criteria** (what must be TRUE):

1. Only bounded, locally submitted, relay-requested, still-present transactions enter initial broadcast retry; the whole mempool never becomes a retry set.
2. Receive-independent maintenance schedules a fresh injected 10-to-15-minute retry cycle and caps decisions and emissions per tick.
3. Surviving package members fan out parent-before-child and retries use the existing relay activation, peer eligibility, txid/wtxid selection, rate, outbox, serving, and successful-receipt path.
4. Unbroadcast membership clears only at the documented eligible serve or successful-write receipt, or on authoritative lifecycle removal, and supported restart behavior never claims guaranteed propagation.

**Plans**: TBD

### Phase 137: RPC and Sanitized Operator Evidence

**Goal**: Operators can inspect and exercise the scoped package and long-lived mempool behavior through one stable, redacted evidence contract.
**Depends on**: Phase 136
**Requirements**: MPOBS-01, MPOBS-02, MPOBS-03
**Success Criteria** (what must be TRUE):

1. RPC and CLI expose package dry-run, package submission, and mempool information with stable errors and per-transaction results matching the authoritative core.
2. Status, dashboard, metrics, logs, and support bundles separately report vsize, accounted usage, capacity, fee floors, pressure/decay, eviction, checkpoint, recovery, and retry outcomes with fixed low-cardinality fields.
3. Shared evidence distinguishes accepted, still-present, eligible, queued, attempted, emitted, requested, served, suppressed, and cleared states without leaking identifiers or per-member details beyond the authenticated direct response.
4. Relay-disabled or peer-policy-suppressed operation can show successful local admission while truthfully showing that no public/default relay or propagation result was achieved.

**Plans**: TBD
**UI hint**: yes

### Phase 138: Parity, Adversarial Pressure, Restart, and Release Guardrails

**Goal**: Contributors and operators have deterministic proof that the integrated v2.2 behavior matches its pinned Knots anchors, remains bounded, and does not broaden release claims.
**Depends on**: Phase 137
**Requirements**: MPVFY-01, MPVFY-02, MPVFY-03, MPVFY-04
**Success Criteria** (what must be TRUE):

1. Pinned fixtures, fake-clock scenarios, randomized graph-oracle tests, and failure injection cover package, rolling-fee, pressure, expiry, recovery, and retry behavior.
2. Package and sustained-pressure benchmarks enforce documented bounded-work and performance expectations without public-network or wall-clock gates in default verification.
3. Parity catalogs, source breadcrumbs, operator docs, and repo-local Cargo and Bazel UAT commands identify exact Knots anchors, intentional differences, and evidence boundaries for every v2.2 surface.
4. Deterministic guardrails require bounded local-package, same-peer 1P1C, ordinary transaction fanout, and initial-broadcast-retry wording while rejecting general package wire, whole-mempool rebroadcast, public/default/production relay, guaranteed propagation, public-network CI, and production-readiness claims.

**Plans**: TBD

## Progress

**Execution Order:** Phases execute in dependency order: 130 → 131 → 132 → 133 → 134 → 135 → 136 → 137 → 138.

| Phase | Plans Complete | Status | Completed |
| --- | ---: | --- | --- |
| 130. Resource, Time, and Fee Primitives | 0/TBD | Not started | - |
| 131. Rolling Fee, Expiry, and Descendant Eviction Core | 0/TBD | Not started | - |
| 132. Typed Package Vocabulary and Staged Admission | 0/TBD | Not started | - |
| 133. Package-Aware Download and Orphan Bridge | 0/TBD | Not started | - |
| 134. Authoritative Cross-Cache Lifecycle Integration | 0/TBD | Not started | - |
| 135. Snapshot Schema, Checkpointing, and Recovery | 0/TBD | Not started | - |
| 136. Receive-Independent Maintenance and Transport Receipts | 0/TBD | Not started | - |
| 137. RPC and Sanitized Operator Evidence | 0/TBD | Not started | - |
| 138. Parity, Adversarial Pressure, Restart, and Release Guardrails | 0/TBD | Not started | - |

***
*Roadmap created: 2026-07-22*
*Granularity: fine*
*Coverage: 40/40 v2.2 requirements mapped exactly once*
