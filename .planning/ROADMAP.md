# Roadmap: Open Bitcoin

## Current Milestone: v2.0 Transaction Relay and Mempool Participation Boundary

**Goal:** Let Open Bitcoin validate, store, announce, request, and relay unconfirmed transactions through bounded mempool participation while preserving Knots-compatible externally observable behavior and avoiding broader production-readiness or public-default relay claims.

**Boundary:** v2.0 is a bounded transaction relay and mempool participation milestone. It does not claim compact block relay, bloom/filter serving, broad package relay, public transaction relay by default, public-network relay CI, production full-node readiness, production service operation, or production-funds wallet safety.

**Phase numbering:** v2.0 continues after the archived v1.9 Phase 99 work. New phase directories should start at Phase 100.

## Phase Plan

| Phase | Name | Requirements | Status |
| --- | --- | --- | --- |
| 100 | Relay Activation Boundary and Permission Semantics | ACT-03, ACT-04 | Complete |
| 101 | Transaction Inventory Identity and Download Scheduling | INV-01, INV-04 | Complete |
| 102 | Orphan Handling and Admission Outcome Bridge | DL-03, DL-04, DL-05, MEM-01, MEM-02 | Complete |
| 103 | Mempool Chainstate Lifecycle and Durable Recovery | MEM-03 | Complete |
| 104 | Relay Serving, Fanout, and Rebroadcast Policy | REL-04 | Complete |
| 105 | Operator, RPC, Metrics, Logs, and Support Evidence | OBS-01, OBS-02, OBS-03, OBS-04 | Complete |
| 106 | Parity Traceability, UAT, and Release Boundary Guardrails | BOUND-01, BOUND-02, BOUND-03, BOUND-04, BOUND-05 | Complete |
| 107 | Runtime Relay Activation and Download Eligibility Integration | ACT-01, ACT-02, INV-02, INV-03, DL-01, DL-02, REL-03 | Complete |
| 108 | Durable Mempool Relay State Recovery | MEM-04, MEM-05, MEM-06, REL-01, REL-02 | Complete |

## Phase Details

### Phase 100: Relay Activation Boundary and Permission Semantics

**Purpose:** Define the explicit activation contract before relay behavior reaches sockets, mempool admission, or public-facing status.

**Scope:**

- Add default-off relay activation semantics and peer eligibility policy.
- Turn v1.9 inert `relay`, `forcerelay`, and `mempool` labels into scoped v2.0 effects only where requirements authorize them.
- Preserve inactive bloom/filter, compact-block, and unrelated permission behavior.

**Success criteria:**

- Activation matrix tests cover default config, outbound peers, inbound peers, manual peers, protected slots, and permissioned peers.
- Service-bit, public-default, and production-readiness behavior remains unchanged unless explicitly scoped.
- Docs explain exactly which permission effects are active in v2.0 and which remain labels.

**Verification:** Pure policy tests, config/parser tests, no-claim checker fixtures, and `bash scripts/verify.sh`.

**Plans:** 3 plans

Plans:
- [x] 100-01-PLAN.md — Pure relay activation and eligibility policy in `open-bitcoin-network`.
- [x] 100-02-PLAN.md — Open Bitcoin JSONC/CLI relay activation config and parser wiring.
- [x] 100-03-PLAN.md — Docs, parity roots, no-claim checker, verifier wiring, and phase verification.

### Phase 101: Transaction Inventory Identity and Download Scheduling

**Purpose:** Build the txid/wtxid-aware relay identity and request scheduler before mempool admission side effects are wired in.

**Scope:**

- Handle transaction `inv`, `getdata`, `tx`, and `notfound` decisions with typed txid/wtxid identity.
- Track per-peer negotiation, already-have state, in-flight requests, duplicate announcements, mismatches, timeouts, and disconnect cleanup.
- Add bounded download scheduling with peer fallback and recent-reject suppression.

**Success criteria:**

- Unit tests cover txid and wtxid paths separately.
- Duplicate, mismatch, timeout, `notfound`, and disconnect cases leave no stale request state.
- Scheduler decisions stay pure and emit typed peer actions rather than performing socket I/O.

**Verification:** Pure `open-bitcoin-network` tests, deterministic fake-clock request expiry tests, and `bash scripts/verify.sh`.

**Plans:** 3/3 plans complete

Plans:
- [x] 101-01-PLAN.md — Pure typed transaction relay identity and download scheduler in `open-bitcoin-network`.
- [x] 101-02-PLAN.md — PeerManager and managed-network scheduler integration without mempool admission side effects.
- [x] 101-03-PLAN.md — Parity documentation, deterministic checker, verifier wiring, and phase verification evidence.

### Phase 102: Orphan Handling and Admission Outcome Bridge

**Purpose:** Connect transaction download to mempool admission through a typed outcome boundary without letting peer socket code mutate mempool state directly.

**Scope:**

- Add bounded missing-parent staging and parent request behavior.
- Reconsider staged transactions after parent acceptance and expire or evict them with evidence.
- Expose stable mempool outcomes for accepted, rejected, duplicate, replaced, orphaned, evicted, and expired states.
- Preserve v1.9 resource-governance limits under transaction download pressure.

**Success criteria:**

- Missing-parent tests prove bounded orphan state, parent requests, reconsideration, cap eviction, and expiry.
- Admission tests cover standardness, fee, RBF, ancestor/descendant, duplicate, and no-partial-mutation cases.
- Managed runtime tests prove peer transactions pass through the relay/download boundary before mempool admission.

**Verification:** Pure mempool tests, relay/admission bridge tests, managed in-memory integration tests, and `bash scripts/verify.sh`.

**Plans:** 4/4 plans complete

Plans:
- [x] 102-01-PLAN.md — Stable mempool outcome contract and no-partial-mutation admission tests.
- [x] 102-02-PLAN.md — Pure bounded orphan staging, parent requests, reconsideration, expiry, and eviction.
- [x] 102-03-PLAN.md — Managed runtime bridge from transaction download to mempool outcomes and orphan reconsideration.
- [x] 102-04-PLAN.md — Parity roots, deterministic checker, verifier wiring, and phase verification.

### Phase 103: Mempool Chainstate Lifecycle and Durable Recovery

**Purpose:** Make mempool participation coherent across blocks, reorg boundaries, trimming, persistence, and restart.

**Scope:**

- Report mempool pressure, trimming, fee-floor, and capacity evidence truthfully, including documented parity gaps if any Knots behavior remains deferred.
- Remove confirmed and conflicting transactions from mempool and relay-serving caches on block connect.
- Reconsider eligible disconnected transactions after block disconnect or reorg within the documented v2.0 boundary.
- Persist accepted mempool transaction state and recover or repair stale, corrupt, or incompatible records safely.

**Success criteria:**

- Block connect, conflict cleanup, replacement, eviction, and reorg tests prove relay cache coherence.
- Durable storage tests cover save, load, remove, restart, schema mismatch, stale record, and corruption behavior.
- Operator-visible state distinguishes implemented pressure behavior from deferred parity.

**Verification:** Mempool lifecycle tests, Fjall adapter tests, restart/recovery tests, parity docs, and `bash scripts/verify.sh`.

**Plans:** 4/4 plans complete

Plans:
- [x] 103-01-PLAN.md — Pure mempool lifecycle pressure, block-connect cleanup, and conflict/descendant cleanup.
- [x] 103-02-PLAN.md — Managed chainstate lifecycle hooks and bounded reorg reconsideration.
- [x] 103-03-PLAN.md — Durable mempool snapshot persistence, codec, Fjall storage, and replay evidence.
- [x] 103-04-PLAN.md — Parity roots, deterministic checker, verifier wiring, and phase verification.

### Phase 104: Relay Serving, Fanout, and Rebroadcast Policy

**Purpose:** Allow eligible peers to request and hear about eligible transactions without over-serving stale data or implying guaranteed propagation.

**Scope:**

- Serve only relay-eligible transactions for peer `getdata` requests.
- Report unknown, stale, confirmed, rejected, or evicted transactions correctly.
- Announce accepted transactions to eligible peers using negotiated txid or wtxid identity, queue limits, rate limits, and suppression rules.
- Route local `sendrawtransaction` submissions through admission and queued relay evidence.
- Implement bounded rebroadcast or explicitly mark rebroadcast deferred across docs, status, and tests.

**Success criteria:**

- `getdata` tests cover eligible, unknown, stale, confirmed, rejected, replaced, and evicted transactions.
- Fanout tests prove per-peer queue bounds, identity negotiation, rate limiting, and suppression.
- Local submission evidence is truthful: accepted and queued does not mean public propagation is guaranteed.

**Verification:** Managed peer network tests, fake-clock fanout tests, RPC submission tests, status/doc checks, and `bash scripts/verify.sh`.

**Plans:** 4/4 plans complete

Plans:
- [x] 104-01-PLAN.md — Pure relay serving/fanout/rebroadcast-deferred policy in `open-bitcoin-network`.
- [x] 104-02-PLAN.md — Managed transaction serving, fanout queues, inv/tx/notfound translation, and lifecycle cleanup.
- [x] 104-03-PLAN.md — Local `sendrawtransaction` relay evidence without public propagation guarantees.
- [x] 104-04-PLAN.md — Parity roots, deterministic checker, verifier wiring, and phase verification.

### Phase 105: Operator, RPC, Metrics, Logs, and Support Evidence

**Purpose:** Project relay and mempool participation through one shared evidence contract without leaking sensitive transaction or peer material.

**Scope:**

- Align `sendrawtransaction`, `getmempoolinfo`, `getnetworkinfo`, and Open Bitcoin network status with implemented relay state.
- Render CLI and dashboard relay/mempool state from shared status.
- Add fixed low-cardinality metrics and structured log outcomes.
- Sanitize support bundle relay and mempool evidence.

**Success criteria:**

- RPC, CLI, dashboard, metrics, logs, and support surfaces classify every relay field as implemented, unavailable, deferred, or intentionally different.
- Sanitizer tests reject raw transaction hex, disallowed txids or wtxids, peer endpoints, permission strings, credentials, and dynamic metric labels.
- All surfaces avoid public relay, compact-block, and production-readiness claims.

**Verification:** RPC dispatch tests, CLI renderer tests, dashboard/status tests, metrics/log tests, support redaction tests, and `bash scripts/verify.sh`.

**Plans:** 4/4 plans complete

Plans:
- [x] 105-01-PLAN.md — Shared relay evidence status contract and RPC projection.
- [x] 105-02-PLAN.md — Operator status, dashboard, metrics, and structured logs.
- [x] 105-03-PLAN.md — Support bundle rendering and relay/mempool sanitization.
- [x] 105-04-PLAN.md — Docs, parity roots, deterministic checker, and phase closeout.

### Phase 106: Parity Traceability, UAT, and Release Boundary Guardrails

**Purpose:** Close the milestone with auditable Knots anchors, deterministic no-claim checks, and operator UAT guidance.

**Scope:**

- Update parity docs, source breadcrumbs, and index entries for transaction relay, transaction download, mempool admission, validation, and policy behavior.
- Add deterministic checkers for compact-block, bloom/filter, package-relay, public-default, production-readiness, and production-funds no-claim boundaries.
- Document repo-local Cargo and Bazel UAT commands.
- Refresh README, operator docs, runtime docs, and release notes around the bounded v2.0 claim.
- Keep default verification deterministic and public-network-free.

**Success criteria:**

- All 32 v2.0 requirements have exactly one roadmap owner and concrete evidence roots.
- UAT commands use repo-local Cargo and Bazel forms and mark public-network relay review opt-in.
- `bash scripts/verify.sh` remains the default verification contract and includes the v2.0 guardrails.

**Verification:** Parity breadcrumb checks, release-boundary checker tests, docs/UAT command checks, roadmap/requirements traceability audit, and `bash scripts/verify.sh`.

### Phase 107: Runtime Relay Activation and Download Eligibility Integration

**Purpose:** Close the broken live relay integration path by carrying explicit runtime relay activation into the managed network and requiring relay eligibility in transaction download scheduling.

**Scope:**

- Pass resolved `RuntimeConfig.relay` settings into `ManagedPeerNetwork` instead of relying on default relay activation.
- Extend transaction announcement/download scheduling so Phase 100 relay eligibility and activation state participate before requests are scheduled.
- Prove enabled, disabled, eligible, ineligible, outbound, inbound, manual, protected, and permissioned peer behavior without changing service bits or public defaults.
- Refresh UAT, runtime docs, and deterministic guardrails so this wiring drift is covered before milestone archive.

**Success criteria:**

- Daemon/runtime construction preserves explicit relay activation config through managed network serving and fanout state.
- Transaction download scheduling suppresses ineligible or inactive relay peers with stable typed evidence and no stale request state.
- RPC/status/UAT evidence distinguishes default-off relay, explicitly enabled relay, and eligible peer classes truthfully.
- Deterministic verification catches dropped activation config and missing download eligibility gates.

**Verification:** Runtime config tests, managed peer network tests, transaction relay scheduler tests, RPC/status projection tests, docs/checker fixtures, and `bash scripts/verify.sh`.

**Plans:** 6/6 plans complete

Plans:
- [x] 107-01-PLAN.md — Pure transaction-download eligibility gate and PeerManager wiring.
- [x] 107-02-PLAN.md — Runtime relay activation propagation into managed network and RPC context construction.
- [x] 107-03-PLAN.md — Sanitized activation and download eligibility status evidence.
- [x] 107-04-PLAN.md — Docs, parity roots, and operator UAT guidance.
- [x] 107-05-PLAN.md — Deterministic Phase 107 checker and verifier wiring.
- [x] 107-06-PLAN.md — Phase verification evidence and planning closeout.

### Phase 108: Durable Mempool Relay State Recovery

**Purpose:** Make recovered accepted mempool state rejoin relay serving, fanout, and operator evidence consistently with live accepted transactions.

**Scope:**

- Replay durable accepted mempool records through a managed recovery path that repopulates relay-serving indexes and evidence without performing socket I/O.
- Keep confirmed, conflicting, replaced, evicted, expired, and reorg-reconsidered transactions coherent across restart and relay cache lifecycle boundaries.
- Project recovered relay and mempool evidence through RPC, CLI/dashboard, metrics, logs, and support bundle surfaces without leaking raw transaction or peer material.
- Add deterministic restart and recovery tests for snapshot replay, stale records, corruption repair, lifecycle cleanup, and evidence projection.

**Success criteria:**

- Recovered accepted transactions are serveable or suppressible through the same typed relay-serving policy as live accepted transactions.
- Restart recovery does not create false public propagation guarantees, duplicate fanout, stale serving cache entries, or raw-data leaks.
- Lifecycle cleanup after block connect, conflict removal, replacement, eviction, expiry, and reorg remains coherent after recovery.
- Operator surfaces and support evidence report recovered relay state with fixed low-cardinality outcomes.

**Verification:** Durable mempool snapshot tests, managed recovery integration tests, relay serving/fanout lifecycle tests, RPC/operator evidence tests, support redaction tests, and `bash scripts/verify.sh`.

**Plans:** 5/5 plans complete

Plans:
- [x] 108-01-PLAN.md — Managed recovery replay and relay cache/fanout identity rehydration without startup socket I/O.
- [x] 108-02-PLAN.md — Restart lifecycle cleanup for recovered confirmed, conflicting, replaced, evicted, expired, and reorg-reconsidered records.
- [x] 108-03-PLAN.md — Sanitized recovered relay evidence across RPC, CLI/dashboard, metrics, logs, and support surfaces.
- [x] 108-04-PLAN.md — Phase 108 docs, parity roots, source breadcrumbs, deterministic checker, and verifier wiring.
- [x] 108-05-PLAN.md — Final verification evidence and Phase 108 requirements/roadmap/state closeout.

## Dependencies

| Phase | Depends On | Reason |
| --- | --- | --- |
| 100 | None | Activation policy must exist before behavior can safely expand. |
| 101 | Phase 100 | Inventory/request behavior depends on knowing which peers are relay eligible. |
| 102 | Phase 101 | Orphan and admission bridges need stable relay identity and download state. |
| 103 | Phase 102 | Lifecycle and recovery need typed mempool outcomes first. |
| 104 | Phase 103 | Serving and fanout need coherent mempool and relay-cache lifecycle. |
| 105 | Phase 104 | Operator surfaces need implemented relay serving and fanout state. |
| 106 | Phase 105 | Closeout guardrails need all implementation and surface evidence. |
| 107 | Phase 106 | Gap closure starts from the completed v2.0 implementation and audit evidence. |
| 108 | Phase 107 | Recovery relay state depends on runtime activation and eligibility wiring being coherent first. |

## Deferred Scope

- Compact block relay and related `cmpctblock`, `getblocktxn`, or `blocktxn` behavior.
- BIP37 bloom filters, compact filters, and full filter serving.
- Broad package relay, cluster mempool policy, and package orphan behavior.
- Public transaction relay by default.
- Public-network relay UAT as a default CI or pre-commit gate.
- Production full-node readiness, production service operation, and production-funds wallet safety.
- GUI, hosted dashboards, packaging, installer, migration apply mode, and destructive migration behavior.

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
- ✅ **v1.9 Inbound Peer Serving and Network Participation Boundary** - Phases 90 through 99 (shipped 2026-06-29). Archive: [v1.9-ROADMAP.md](milestones/v1.9-ROADMAP.md)
- 🔄 **v2.0 Transaction Relay and Mempool Participation Boundary** - Phases 100 through 108 (active; ready for milestone audit/archive).

## Milestone History

| Milestone | Phases | Plans | Status | Shipped | Archive |
| --- | ---: | ---: | --- | --- | --- |
| v1.0 Headless Parity | 22 | 80 | Shipped | 2026-04-26 | [roadmap](milestones/v1.0-ROADMAP.md) |
| v1.1 Operator Runtime and Real-Network Sync | 22 | 69 | Shipped | 2026-04-30 | [roadmap](milestones/v1.1-ROADMAP.md) |
| v1.2 Full Mainnet Network Syncing | 7 | 13 | Shipped | 2026-05-23 | [roadmap](milestones/v1.2-ROADMAP.md) |
| v1.3 Public Mainnet Sync Proof and Node Hardening | 12 | 13 | Shipped | 2026-06-02 | [roadmap](milestones/v1.3-ROADMAP.md) |
| v1.4 Mainnet IBD Convergence and Peer Compatibility | 6 | 15 | Shipped | 2026-06-05 | [roadmap](milestones/v1.4-ROADMAP.md) |
| v1.5 Unattended Mainnet Node Operation Readiness | 8 | 22 | Shipped | 2026-06-10 | [roadmap](milestones/v1.5-ROADMAP.md) |
| v1.6 Mainnet Full-Sync Completion | 7 | 27 | Shipped | 2026-06-14 | [roadmap](milestones/v1.6-ROADMAP.md) |
| v1.7 Full-Sync Soak and Recovery Hardening | 7 | 37 | Shipped | 2026-06-20 | [roadmap](milestones/v1.7-ROADMAP.md) |
| v1.8 Production Full-Node Readiness Boundary | 8 | 26 | Shipped | 2026-06-25 | [roadmap](milestones/v1.8-ROADMAP.md) |
| v1.9 Inbound Peer Serving and Network Participation Boundary | 10 | 56 | Shipped | 2026-06-29 | [roadmap](milestones/v1.9-ROADMAP.md) |
| v2.0 Transaction Relay and Mempool Participation Boundary | 9 | 35 | Active | - | active |

## Traceability

- Active requirements: [REQUIREMENTS.md](REQUIREMENTS.md)
- v2.0 research summary: [SUMMARY.md](research/SUMMARY.md)
- v1.9 requirements archive: [v1.9-REQUIREMENTS.md](milestones/v1.9-REQUIREMENTS.md)
- v1.9 milestone audit: [v1.9-MILESTONE-AUDIT.md](milestones/v1.9-MILESTONE-AUDIT.md)

## Next Step

Run the v2.0 milestone audit, then archive the milestone if the audit passes:

```bash
/gsd-audit-milestone
```
