# Requirements: Open Bitcoin

**Defined:** 2026-07-22
**Milestone:** v2.2 Package Relay and Long-Lived Mempool Policy
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v2.2 Requirements

Requirements for package admission and relay, rolling minimum-fee behavior, initial broadcast retry, and sustained mempool pressure. Each requirement maps to exactly one roadmap phase.

### Resource, Time, and Fee Primitives

- [x] **FEEP-01**: Operator evidence distinguishes transaction virtual size from accounted mempool memory usage and its configured capacity.
- [x] **FEEP-02**: Node policy distinguishes the configured static relay floor, incremental relay fee, rolling mempool floor, and effective admission floor without allowing package fees to bypass the wrong boundary.
- [x] **FEEP-03**: Mempool entries carry explicit acceptance time and typed local-origin and relay-request metadata needed by expiry, recovery, and initial broadcast retry.
- [x] **FEEP-04**: Pure mempool and network policy accepts explicit time, block, occupancy, and jitter inputs without reading wall-clock time or randomness directly.
- [x] **FEEP-05**: Admission, replacement, expiry, pressure eviction, block connection, reorg, and retry clearing use stable typed outcomes suitable for deterministic lifecycle and operator evidence.

### Rolling Fee and Sustained Pressure

- [ ] **PRESS-01**: Node enforces the configured mempool capacity using deterministic accounted-memory usage while retaining virtual size as a separate fee and reporting measure.
- [ ] **PRESS-02**: Pressure trimming selects and removes complete descendant packages by the pinned Knots descendant-score ordering and raises the rolling minimum fee from the actual evicted package.
- [ ] **PRESS-03**: Rolling minimum-fee decay is block-gated, occupancy-sensitive, clock-safe, and matches pinned 12-hour, 6-hour, and 3-hour half-life behavior and rounding boundaries.
- [ ] **PRESS-04**: Expiry and pressure removals clean every affected descendant and derived index while preserving mempool graph and fee-aggregate invariants.
- [ ] **PRESS-05**: Sustained fill, trim, block, decay, expiry, refill, and reorg sequences remain bounded and agree with a deterministic recomputation oracle and performance thresholds.

### Package Admission

- [ ] **PACK-01**: Node validates package shape before expensive work, including non-empty input, the pinned 25-transaction and 404,000-weight limits, unique identities, topological order, and no internal input conflicts.
- [ ] **PACK-02**: Operator can dry-run package admission and receive ordered per-transaction results without mutating mempool, relay, persistence, or evidence state.
- [ ] **PACK-03**: Operator can submit a child-with-unconfirmed-parents package and receive package-wide status plus ordered final per-transaction outcomes and effective-fee membership.
- [ ] **PACK-04**: Package admission preserves pinned individual-first partial-acceptance behavior instead of treating the entire call as globally atomic.
- [ ] **PACK-05**: Each accepted subpackage is staged and committed through one coherent mempool delta, with no partial mutation when validation, replacement, limits, or commit preparation fails.
- [ ] **PACK-06**: Package fee evaluation applies the pinned effective-fee grouping rules while preserving the static relay floor and evaluating the active rolling floor correctly.
- [ ] **PACK-07**: Package outcomes reflect final post-trim membership and match the pinned replacement, TRUC, ephemeral-dust, same-txid/different-witness, and reconsiderable-failure boundaries selected for the scoped surface.

### Opportunistic Peer Package Relay

- [ ] **PPKG-01**: Node distinguishes hard rejects from reconsiderable package candidates and retains only bounded, rotating candidate and reject evidence.
- [ ] **PPKG-02**: Node assembles only sender-aware same-peer one-parent/one-child candidates over ordinary transaction messages, preserving member origin and exact pinned package identity.
- [ ] **PPKG-03**: Peer-originated package candidates reuse the authoritative package admission engine rather than reimplementing package policy in the network or RPC layers.
- [ ] **PPKG-04**: Accepted and still-present package members enter existing transaction serving and relay fanout in parent-before-child order under existing activation, peer-policy, queue, rate, and txid/wtxid controls.

### Authoritative Lifecycle Integration

- [ ] **MPLIFE-01**: `ManagedNetworkHandle` remains the sole runtime mutation authority for package admission, pressure policy, maintenance, persistence snapshots, relay queues, and transport receipts.
- [ ] **MPLIFE-02**: One lifecycle delta projects every package admission and removal into serving, fanout, peer request/known state, orphan/package candidates, compact reconstruction inputs, unbroadcast state, persistence dirtiness, and operator evidence.
- [ ] **MPLIFE-03**: Replacement, pressure eviction, expiry, block connection, reorg, and failed admission cannot leave stale descendants or accepted identities in any dependent cache.
- [ ] **MPLIFE-04**: Runtime adapters capture owned commands or snapshots under authority, release the lock before storage or network I/O, and apply bounded typed receipts in a short follow-up mutation.

### Durable Mempool Policy Recovery

- [ ] **MPDUR-01**: Durable mempool snapshots preserve canonical transactions, acceptance times, and surviving locally submitted unbroadcast membership without persisting derived peer, topology, or rolling-fee state.
- [ ] **MPDUR-02**: Recovery validates and topologically replays durable records against current chainstate and policy, rebuilds derived indexes, and reports typed recovered and dropped classifications.
- [ ] **MPDUR-03**: Rolling minimum-fee state resets to the pinned restart baseline while restored entries retain supported age and local-unbroadcast semantics.
- [ ] **MPDUR-04**: Coalesced periodic and clean-shutdown checkpoint paths expose freshness, dirty generation, persistence strength, and the documented crash-loss window without holding runtime authority across I/O.

### Initial Broadcast Retry

- [ ] **IBR-01**: Node tracks only bounded locally submitted, relay-requested, still-present transactions for initial broadcast retry; it never treats the whole mempool as a rebroadcast set.
- [ ] **IBR-02**: Receive-independent maintenance schedules fresh randomized 10-to-15-minute retry cycles from injected inputs and caps work and emissions per tick.
- [ ] **IBR-03**: Retry uses existing relay activation, peer eligibility, txid/wtxid selection, rate limits, bounded outboxes, serving paths, and successful transport receipts rather than a parallel fanout path.
- [ ] **IBR-04**: Unbroadcast membership clears only at the documented eligible serve or successful-write receipt, or on authoritative lifecycle removal, and survives supported restart boundaries without claiming guaranteed propagation.

### RPC and Operator Evidence

- [ ] **MPOBS-01**: RPC and CLI expose the scoped package dry-run, submission, and mempool-info behavior with stable errors and per-transaction results that match the authoritative core.
- [ ] **MPOBS-02**: Status, dashboard, metrics, logs, and support bundles distinguish vsize, accounted usage, capacity, static and rolling fee floors, pressure and decay state, eviction, checkpoint, recovery, and retry outcomes using fixed low-cardinality fields.
- [ ] **MPOBS-03**: Shared evidence is redacted and distinguishes accepted, still-present, eligible, queued, attempted, emitted, requested, served, suppressed, and cleared states; identifiers and detailed per-member results remain confined to the authenticated direct response that supplied them.

### Parity, Verification, and Release Boundaries

- [ ] **MPVFY-01**: Package, rolling-fee, pressure, expiry, recovery, and retry behavior has deterministic pinned-Knots fixtures, fake-clock scenarios, randomized graph-oracle tests, and failure-injection coverage.
- [ ] **MPVFY-02**: Package and sustained-pressure benchmarks enforce documented bounded-work and performance expectations without adding public-network or wall-clock gates to default verification.
- [ ] **MPVFY-03**: Parity catalogs, breadcrumbs, operator docs, and repo-local Cargo and Bazel UAT commands identify exact Knots anchors, intentional differences, and evidence boundaries for every v2.2 surface.
- [ ] **MPVFY-04**: Deterministic claim guardrails require the bounded local-package, same-peer 1P1C, ordinary transaction fanout, and initial-broadcast-retry wording while rejecting general package wire, whole-mempool rebroadcast, public/default/production relay, guaranteed propagation, public-network CI, and production-readiness claims.

## Future Requirements

Deferred beyond v2.2 and not mapped to the current roadmap.

### Broader Package and Relay Protocols

- **FUT-12**: Node supports BIP331 or another general package wire protocol with explicitly negotiated package inventory and transfer messages.
- **FUT-13**: Node supports arbitrary multi-parent peer package reconstruction or cluster-mempool policy beyond the pinned same-peer 1P1C boundary.
- **FUT-14**: Node supports Erlay, bloom/filter expansion, or Knots `mempool.dat` import/export.

### Broader Operation Claims

- **FUT-15**: Node enables public transaction or package relay by default with production-scale support evidence.
- **FUT-16**: Default CI or release blocking depends on public-network package-relay or sustained-pressure runs.
- **FUT-17**: Node claims production full-node readiness, production service operation, guaranteed propagation, or production-funds wallet safety.

## Out of Scope

| Feature | Reason |
| --- | --- |
| New package inventory or wire messages | The pinned baseline's scoped P2P behavior is opportunistic same-peer 1P1C over ordinary transaction messages. |
| Arbitrary multi-parent P2P package assembly | It exceeds the bounded pinned peer behavior and materially expands memory, attribution, and DoS risk. |
| Cluster mempool | It is a distinct policy architecture and is not required to close the selected v2.2 gaps. |
| Whole-mempool or wallet-wide periodic rebroadcast | v2.2 retries only bounded locally submitted unbroadcast transactions to preserve privacy and resource limits. |
| Guaranteed public propagation | Admission, eligibility, queueing, and even a successful write do not prove network-wide propagation. |
| Public/default relay promotion | Existing activation and claim boundaries remain in force until a later production-readiness milestone deliberately changes them. |
| Live public-network default verification | Default verification remains hermetic and deterministic; any public-network review stays explicit opt-in UAT. |
| Production readiness or production-funds wallet claims | Package and long-lived mempool policy do not independently satisfy the existing production gates. |

## Traceability

Every v2.2 requirement maps to exactly one roadmap phase.

| Requirement | Phase | Status |
| --- | --- | --- |
| FEEP-01 | Phase 130 | Complete |
| FEEP-02 | Phase 130 | Complete |
| FEEP-03 | Phase 130 | Complete |
| FEEP-04 | Phase 130 | Complete |
| FEEP-05 | Phase 130 | Complete |
| PRESS-01 | Phase 131 | Pending |
| PRESS-02 | Phase 131 | Pending |
| PRESS-03 | Phase 131 | Pending |
| PRESS-04 | Phase 131 | Pending |
| PRESS-05 | Phase 131 | Pending |
| PACK-01 | Phase 132 | Pending |
| PACK-02 | Phase 132 | Pending |
| PACK-03 | Phase 132 | Pending |
| PACK-04 | Phase 132 | Pending |
| PACK-05 | Phase 132 | Pending |
| PACK-06 | Phase 132 | Pending |
| PACK-07 | Phase 132 | Pending |
| PPKG-01 | Phase 133 | Pending |
| PPKG-02 | Phase 133 | Pending |
| PPKG-03 | Phase 133 | Pending |
| PPKG-04 | Phase 136 | Pending |
| MPLIFE-01 | Phase 134 | Pending |
| MPLIFE-02 | Phase 134 | Pending |
| MPLIFE-03 | Phase 134 | Pending |
| MPLIFE-04 | Phase 134 | Pending |
| MPDUR-01 | Phase 135 | Pending |
| MPDUR-02 | Phase 135 | Pending |
| MPDUR-03 | Phase 135 | Pending |
| MPDUR-04 | Phase 135 | Pending |
| IBR-01 | Phase 136 | Pending |
| IBR-02 | Phase 136 | Pending |
| IBR-03 | Phase 136 | Pending |
| IBR-04 | Phase 136 | Pending |
| MPOBS-01 | Phase 137 | Pending |
| MPOBS-02 | Phase 137 | Pending |
| MPOBS-03 | Phase 137 | Pending |
| MPVFY-01 | Phase 138 | Pending |
| MPVFY-02 | Phase 138 | Pending |
| MPVFY-03 | Phase 138 | Pending |
| MPVFY-04 | Phase 138 | Pending |

**Coverage:**
- v2.2 requirements: 40 total
- Mapped to phases: 40
- Unmapped: 0 ✓
- Duplicate mappings: 0 ✓

***
*Requirements defined: 2026-07-22*
*Last updated: 2026-07-22 after v2.2 roadmap creation*
