# Roadmap: Open Bitcoin

## Milestones

- ✅ **v1.0 Headless Parity** - 22 phase entries, including inserted 3.x and 7.x closure phases (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** - Phases 13 through 34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Full Mainnet Network Syncing** - Phases 35 through 41 (shipped 2026-05-23). Archive: [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Public Mainnet Sync Proof and Node Hardening** - Phases 42 through 53 (shipped 2026-06-02). Archive: [v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md)
- ✅ **v1.4 Mainnet IBD Convergence and Peer Compatibility** - Phases 54 through 59 (shipped 2026-06-05). Archive: [v1.4-ROADMAP.md](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Unattended Mainnet Node Operation Readiness** - Phases 60 through 67 (shipped 2026-06-10). Archive: [v1.5-ROADMAP.md](milestones/v1.5-ROADMAP.md)
- ✅ **v1.6 Mainnet Full-Sync Completion** - Phases 68 through 74 (shipped 2026-06-14). Archive: [v1.6-ROADMAP.md](milestones/v1.6-ROADMAP.md)
- ✅ **v1.7 Full-Sync Soak and Recovery Hardening** - Phases 75 through 81 (shipped 2026-06-20). Archive: [v1.7-ROADMAP.md](milestones/v1.7-ROADMAP.md)
- ✅ **v1.8 Production Full-Node Readiness Boundary** - Phases 82 through 89 (shipped 2026-06-25). Archive: [v1.8-ROADMAP.md](milestones/v1.8-ROADMAP.md)
- 🚧 **v1.9 Inbound Peer Serving and Network Participation Boundary** - Phases 90 through 95 (active).

## Current Focus

v1.9 Inbound Peer Serving and Network Participation Boundary started on 2026-06-25.

**Goal:** Let Open Bitcoin accept and serve inbound peers under explicit admission, permission, address, eviction/ban, and resource-governance rules while keeping relay and production participation claims deferred.

**Current state:** Phase 95 Plan 01 is complete; Plans 02 through 04 remain for parity roots, UAT/public boundary wording, and aggregate checker wiring.

**Boundary:** v1.9 does not claim transaction relay, compact block relay, mempool propagation, public inbound serving by default, production service operation, or production full-node readiness.

## Phases

| Phase | Name | Goal | Requirements | Status |
|-------|------|------|--------------|--------|
| 90 | Inbound Listener and Admission Policy | 10/10 | Complete   | 2026-06-25 |
| 91 | Peer Permissions and Connection Classes | 10/10 | Complete | 2026-06-25 |
| 92 | Address Advertisement and Discovery Boundaries | 9/9 | Complete    | 2026-06-26 |
| 93 | Eviction, Ban, and Misbehavior Policy | 3/3 | Complete    | 2026-06-26 |
| 94 | DoS and Resource Governance | 8/8 | Complete    | 2026-06-27 |
| 95 | Network Participation Evidence and Release Boundary | 4/4 | Complete    | 2026-06-27 |

## Phase Details

### Phase 90: Inbound Listener and Admission Policy

**Goal:** Introduce an explicit opt-in inbound listener and admission path that creates typed inbound peers, performs the handshake lifecycle, enforces caps, and exposes operator evidence without regressing outbound sync.

**Requirements:** INB-01, INB-02, INB-03, INB-04, INB-05

**Plans:** 10/10 plans complete

Plans:
- [x] 90-01-PLAN.md - Pure inbound listener and admission contracts
- [x] 90-02-PLAN.md - Open Bitcoin-owned inbound config and CLI controls
- [x] 90-03-PLAN.md - Peer manager inbound state, handshake, and counters
- [x] 90-04-PLAN.md - Runtime listener adapter, daemon startup, and loopback integration
- [x] 90-05-PLAN.md - Shared inbound status and metrics contract
- [x] 90-06-PLAN.md - RPC-facing inbound status without changing getnetworkinfo shape
- [x] 90-07-PLAN.md - Operator status collection and rendering
- [x] 90-08-PLAN.md - Support bundle inbound evidence and redaction
- [x] 90-09-PLAN.md - Operator docs, parity roots, breadcrumbs, and UAT commands
- [x] 90-10-PLAN.md - Deterministic Phase 90 checker and final verification (completed 2026-06-25)

**Success criteria:**
1. Operators can enable/disable inbound serving through explicit config or CLI controls, and the disabled path cannot bind a listener.
2. Listener preflight returns deterministic diagnostics for disabled, invalid, unsafe, unavailable, or already-bound endpoints.
3. Inbound peer admission creates typed connection records with handshake state, duplicate/self-connection protections, and accurate inbound/outbound counters.
4. Inbound caps and protected slots are enforced without starving outbound sync or changing existing full-sync defaults.
5. Status, metrics, logs, RPC-facing status, and support evidence expose inbound admission and handshake outcomes.

### Phase 91: Peer Permissions and Connection Classes

**Goal:** Model Knots-aligned permission concepts and connection classes while keeping v1.9 permission effects bounded to admission, eviction, address, download-serving, and diagnostics behavior.

**Requirements:** PERM-01, PERM-02, PERM-03, PERM-04

**Plans:** 10/10 plans complete

Plans:
- [x] 91-01-PLAN.md — Pure permission vocabulary and connection-class domain model
- [x] 91-02-PLAN.md — Open Bitcoin JSONC and CLI permission-class config
- [x] 91-03-PLAN.md — Permission evidence in admission records and managed counters
- [x] 91-04-PLAN.md — Runtime listener permission-aware admission wiring
- [x] 91-05-PLAN.md — Shared status, RPC, and metrics permission evidence
- [x] 91-06-PLAN.md — Operator status permission rendering
- [x] 91-07-PLAN.md — Support-bundle permission evidence and redaction
- [x] 91-08-PLAN.md — Relay, mempool, filter, and compact-block negative safeguards
- [x] 91-09-PLAN.md — Operator docs, parity roots, and UAT commands
- [x] 91-10-PLAN.md — Deterministic Phase 91 checker and verifier wiring (completed 2026-06-25)

**Success criteria:**
1. Config parsing accepts only explicit, documented peer permission classes and returns stable validation errors for unsupported combinations.
2. Permission effects are observable in admission, eviction, address-response, download-serving, and diagnostic paths.
3. Relay, mempool, force-relay, and compact-block-style permissions cannot silently enable deferred relay behavior.
4. Status and support evidence explain permission decisions without leaking secrets.

### Phase 92: Address Advertisement and Discovery Boundaries

**Goal:** Add privacy-aware listener advertisement and bounded address request/management behavior without claiming full address relay or broader public-network discovery parity.

**Requirements:** ADDR-01, ADDR-02, ADDR-03, ADDR-04

**Plans:** 9/9 plans complete

Plans:
- [x] 92-01-PLAN.md — Pure local address advertisement contracts
- [x] 92-02-PLAN.md — Bounded getaddr/addr wire support and version sender gating
- [x] 92-03-PLAN.md — Learned-address contract and getaddr response policy
- [x] 92-04-PLAN.md — PeerManager address intake and permission-aware getaddr handling
- [x] 92-05-PLAN.md — Shared node status address-boundary evidence
- [x] 92-06-PLAN.md — Runtime listener evidence and RPC status projection
- [x] 92-07-PLAN.md — CLI status and support rendering for address evidence
- [x] 92-08-PLAN.md — Operator docs, parity metadata, and source breadcrumbs
- [x] 92-09-PLAN.md — Deterministic Phase 92 checker and verifier wiring (completed 2026-06-26)

**Success criteria:**
1. Local address candidate selection respects configured listener addresses, routability, reachability, and privacy-network boundaries.
2. Bounded `getaddr` response behavior is deterministic, permission-aware, and capped by count, age, cache, and source policy.
3. Learned addresses enter a typed address-management contract with routability, source, freshness, and persistence evidence.
4. Documentation and release checks distinguish local listener advertisement, address request responses, peer discovery, and full address relay.

### Phase 93: Eviction, Ban, and Misbehavior Policy

**Goal:** Add deterministic peer eviction, disconnect, discourage, ban, expiry, unban, and misbehavior handling with Knots anchors and operator-visible reasons.

**Requirements:** EVICT-01, EVICT-02, EVICT-03, EVICT-04

**Success criteria:**
1. Eviction scoring uses explicit, deterministic criteria for connection class, handshake progress, diversity, activity, and permissions.
2. Admission caps and abuse policy can evict or disconnect peers while preserving reason codes, metrics, logs, and support evidence.
3. Discourage/ban state is durable, expiry-aware, scoped to address/subnet, manually reversible, and never hidden behind broad implicit bans.
4. Misbehavior accounting applies bounded responses and respects protected peer classes.

### Phase 94: DoS and Resource Governance

**Goal:** Bound inbound sockets, parsing, queues, requests, timeouts, churn, and reconnect behavior while making resource pressure visible and deterministically testable.

**Requirements:** DOS-01, DOS-02, DOS-03, DOS-04, DOS-05

**Plans:** 8/8 plans complete

Plans:
- [x] 94-01-PLAN.md - Pure message-envelope resource gate
- [x] 94-02-PLAN.md - Pure queue, request, timeout, churn, and reconnect policy
- [x] 94-03-PLAN.md - Runtime listener resource-envelope and timeout wiring
- [x] 94-04-PLAN.md - Peer request-cap enforcement and resource-limit disconnects
- [x] 94-05-PLAN.md - Shared status and fixed metrics projection
- [x] 94-06-PLAN.md - Operator status and support rendering
- [x] 94-07-PLAN.md - Operator, architecture, and parity documentation
- [x] 94-08-PLAN.md - Deterministic Phase 94 checker and verifier wiring (completed 2026-06-26)

**Success criteria:**
1. Inbound message parsing rejects invalid magic, malformed headers, oversized payloads, unsupported commands, and malformed payloads before unbounded allocation.
2. Per-peer and aggregate read/write queues, inventory/request bounds, header/block/transaction request caps, and backpressure behavior are enforced.
3. Slow handshakes, idle peers, connection churn, repeated failures, and banned/discouraged reconnect attempts have deterministic limits and tests.
4. Metrics, structured logs, support bundles, and status output expose resource pressure and next actions.
5. Default `bash scripts/verify.sh` remains public-network-free while proving inbound resource policy through synthetic and loopback-safe checks.

### Phase 95: Network Participation Evidence and Release Boundary

**Goal:** Close v1.9 by proving parity roots, non-regression, UAT guidance, support redaction, and deterministic release-boundary checks that keep deferred network participation claims out of scope.

**Requirements:** BOUND-01, BOUND-02, BOUND-03, BOUND-04, BOUND-05, BOUND-06

**Plans:** 3/4 plans executed

Plans:
- [x] 95-01-PLAN.md — Support resource-governance redaction and regression tests
- [x] 95-02-PLAN.md — Parity closeout roots and release-readiness traceability
- [x] 95-03-PLAN.md — Operator UAT commands and public boundary wording
- [ ] 95-04-PLAN.md — Aggregate Phase 95 checker and verifier wiring

**Success criteria:**
1. Release and parity docs cite Knots anchors or record intentional deviations for inbound serving, permissions, address handling, eviction/ban, and resource governance.
2. Deterministic checkers reject transaction relay, compact block relay, mempool propagation, public inbound default, production-service, and production-readiness claims for v1.9.
3. Existing outbound sync, full-sync, soak, support-bundle, production no-claim, and release-boundary behavior remains verified and non-regressed.
4. Operator UAT includes copy-pasteable repo-local Cargo and Bazel commands for loopback or synthetic inbound review.
5. Support bundles preserve useful inbound serving diagnosis while redacting peer addresses where needed.
6. Requirements, roadmap, summaries, verification, and audit artifacts maintain 28/28 requirement traceability.

## Milestone History

| Milestone | Phases | Plans | Status | Shipped |
| --- | ---: | ---: | --- | --- |
| v1.0 Headless Parity | 22 | 80 | Shipped | 2026-04-26 |
| v1.1 Operator Runtime and Real-Network Sync | 22 | 69 | Shipped | 2026-04-30 |
| v1.2 Full Mainnet Network Syncing | 7 | 13 | Shipped | 2026-05-23 |
| v1.3 Public Mainnet Sync Proof and Node Hardening | 12 | 13 | Shipped | 2026-06-02 |
| v1.4 Mainnet IBD Convergence and Peer Compatibility | 6 | 15 | Shipped | 2026-06-05 |
| v1.5 Unattended Mainnet Node Operation Readiness | 8 | 22 | Shipped | 2026-06-10 |
| v1.6 Mainnet Full-Sync Completion | 7 | 27 | Shipped | 2026-06-14 |
| v1.7 Full-Sync Soak and Recovery Hardening | 7 | 37 | Shipped | 2026-06-20 |
| v1.8 Production Full-Node Readiness Boundary | 8 | 26 | Shipped | 2026-06-25 |
| v1.9 Inbound Peer Serving and Network Participation Boundary | 6 | 20 | Active | — |

## Traceability

**Coverage:** 28/28 v1.9 requirements mapped, 0 unmapped.

| Phase | Requirements | Count |
|-------|--------------|------:|
| Phase 90 | INB-01, INB-02, INB-03, INB-04, INB-05 | 5 |
| Phase 91 | PERM-01, PERM-02, PERM-03, PERM-04 | 4 |
| Phase 92 | ADDR-01, ADDR-02, ADDR-03, ADDR-04 | 4 |
| Phase 93 | EVICT-01, EVICT-02, EVICT-03, EVICT-04 | 4 |
| Phase 94 | DOS-01, DOS-02, DOS-03, DOS-04, DOS-05 | 5 |
| Phase 95 | BOUND-01, BOUND-02, BOUND-03, BOUND-04, BOUND-05, BOUND-06 | 6 |

## Next Step

Run the milestone audit or completion flow for v1.9:

```bash
/gsd-audit-milestone
```

Also available:

```bash
/gsd-progress
```
