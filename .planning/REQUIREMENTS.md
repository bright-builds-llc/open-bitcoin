# Requirements: Open Bitcoin v2.0

**Defined:** 2026-06-29
**Milestone:** v2.0 Transaction Relay and Mempool Participation Boundary
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v2.0 Requirements

Requirements for the current milestone. Each requirement maps to exactly one roadmap phase.

### Relay Activation And Permission Boundary

- [x] **ACT-01**: Operator can enable transaction relay only through explicit relay activation settings that keep public relay off by default.
- [x] **ACT-02**: Node classifies peer relay eligibility across outbound, inbound, manual, protected, and permissioned peers without changing service bits or public defaults accidentally.
- [x] **ACT-03**: Permission effects for `relay`, `forcerelay`, and `mempool` activate only the scoped behavior documented for v2.0.
- [x] **ACT-04**: Bloom/filter permissions, compact-block behavior, and unrelated peer permissions remain inactive unless a later requirement explicitly activates them.

### Transaction Inventory And Identity

- [ ] **INV-01**: Node handles transaction `inv`, `getdata`, `tx`, and `notfound` messages with typed txid and wtxid identity.
- [ ] **INV-02**: Node tracks per-peer txid/wtxid negotiation, already-have state, request state, and received-transaction cleanup deterministically.
- [ ] **INV-03**: Node handles duplicate announcements, identity mismatches, `notfound`, timeout, and disconnect cleanup without stale request state.
- [ ] **INV-04**: Relay decisions emit stable typed actions for announcements, requests, suppressions, fallbacks, and peer cleanup.

### Bounded Transaction Download

- [ ] **DL-01**: Node schedules transaction downloads with bounded in-flight request caps, expiry, peer fallback, and retry evidence.
- [ ] **DL-02**: Node suppresses redundant transaction requests through already-have, recent-reject, in-flight, and mempool-state checks.
- [x] **DL-03**: Node stages missing-parent transactions in a bounded orphan or candidate state and requests eligible parents.
- [x] **DL-04**: Node reconsiders staged missing-parent transactions after parent acceptance and expires or evicts them with evidence when limits are reached.
- [x] **DL-05**: Transaction download behavior preserves v1.9 queue, request, timeout, churn, and resource-governance limits under adversarial bursts.

### Mempool Admission And Lifecycle

- [x] **MEM-01**: Peer and local transaction submissions flow through one stable mempool outcome contract for accepted, rejected, duplicate, replaced, orphaned, evicted, and expired states.
- [x] **MEM-02**: Mempool admission tests cover standardness, fees, RBF, ancestor/descendant limits, duplicate handling, and no partial mutation on rejection.
- [ ] **MEM-03**: Mempool pressure and trimming behavior produce truthful relay-facing eviction, fee-floor, and capacity evidence, or explicitly document any deferred Knots parity gap.
- [ ] **MEM-04**: Block connect removes confirmed and conflicting transactions from mempool and relay-serving caches.
- [ ] **MEM-05**: Block disconnect or reorg handling reconsiders eligible disconnected transactions within the documented v2.0 boundary.
- [ ] **MEM-06**: Durable mempool persistence saves accepted transaction state and recovers or repairs stale, corrupt, or incompatible records safely on restart.

### Relay Serving And Fanout

- [ ] **REL-01**: Node serves only relay-eligible transactions in response to peer `getdata` requests and reports unknown, stale, confirmed, rejected, or evicted transactions correctly.
- [ ] **REL-02**: Node announces accepted transactions to eligible peers using negotiated txid or wtxid identity, per-peer queues, rate limits, and suppression rules.
- [ ] **REL-03**: Local `sendrawtransaction` submissions enter mempool admission and queued relay evidence without guaranteeing public propagation.
- [ ] **REL-04**: Rebroadcast behavior is either implemented with bounded scheduling and evidence or explicitly marked deferred across docs, status, and tests.

### Operator, RPC, Metrics, Logs, And Support Evidence

- [ ] **OBS-01**: RPC surfaces such as `sendrawtransaction`, `getmempoolinfo`, `getnetworkinfo`, and Open Bitcoin network status report relay and mempool participation truthfully.
- [ ] **OBS-02**: CLI and dashboard surfaces render relay and mempool state from the shared status contract without raw transaction, peer, permission, or credential leakage.
- [ ] **OBS-03**: Metrics and structured logs use fixed low-cardinality relay outcomes for accepted, rejected, orphaned, requested, served, announced, suppressed, evicted, and expired events.
- [ ] **OBS-04**: Support bundles sanitize relay and mempool evidence, including raw transaction hex, disallowed txids or wtxids, peer endpoints, permission strings, dynamic labels, and credentials.

### Parity, UAT, And Release Boundary

- [ ] **BOUND-01**: Parity docs, source breadcrumbs, and index entries cite concrete Bitcoin Knots anchors for transaction relay, transaction download, mempool admission, validation, and policy behavior.
- [ ] **BOUND-02**: Deterministic checkers prevent compact block relay, bloom/filter serving, package relay, public-relay-default, production-readiness, and production-funds claims from entering v2.0 artifacts.
- [ ] **BOUND-03**: UAT guidance provides copy-pasteable repo-local Cargo and Bazel commands and keeps public-network relay review opt-in.
- [ ] **BOUND-04**: README, operator docs, runtime docs, and release notes describe the bounded v2.0 relay claim and list deferred surfaces clearly.
- [ ] **BOUND-05**: The default `bash scripts/verify.sh` contract remains deterministic and free of public-network, wall-clock soak, service-manager, and production-deployment gates.

## Deferred Requirements

Deferred to future milestones. These are acknowledged but not part of the v2.0 roadmap.

### Relay Expansion

- **FUT-01**: Node participates in compact block relay with `cmpctblock`, `getblocktxn`, and `blocktxn` parity.
- **FUT-02**: Node supports broad package relay, cluster mempool policy, and package orphan behavior.
- **FUT-03**: Node enables public transaction relay by default with production-ready abuse, support, service, packaging, and firewall guidance.
- **FUT-04**: Node runs public-network relay UAT as a CI or release-blocking default gate.

### Filters And Additional Protocol Surfaces

- **FUT-05**: Node serves BIP37 bloom-filter behavior.
- **FUT-06**: Node serves compact filters and related block-filter behavior.
- **FUT-07**: Node expands address relay beyond the v1.9 bounded address advertisement and discovery claim.

### Product And Operations

- **FUT-08**: Node claims production full-node readiness under the v1.8 production-readiness gate set.
- **FUT-09**: Wallet behavior is approved for production-funds use.
- **FUT-10**: Migration apply mode mutates source datadirs, services, configs, or wallets.
- **FUT-11**: GUI, hosted dashboard, packaging, installer, and managed service deployment are provided.

## Out Of Scope

Explicitly excluded from v2.0 to prevent scope creep.

| Feature | Reason |
| --- | --- |
| Compact block relay | Separate protocol and peer-negotiation milestone; not required to prove transaction relay boundary. |
| Bloom filters and compact filters | Separate privacy and filter-serving surfaces with different abuse and support risks. |
| Package relay and cluster mempool | Requires package policy, orphanage, and parity work beyond single-transaction relay. |
| Public relay by default | Requires production abuse, service, packaging, firewall, and support readiness that v2.0 does not claim. |
| Public-network relay CI by default | Would violate the deterministic local verification contract. |
| Production full-node readiness | Governed by v1.8 readiness gates, not established by relay alone. |
| Production-funds wallet safety | Wallet safety and support evidence are separate milestones. |
| GUI, hosted dashboard, packaging, installer, and service deployment | Product and distribution surfaces are deferred while relay internals remain the focus. |
| Migration apply mode | Migration remains dry-run-first and must not mutate source datadirs, services, configs, or wallets in v2.0. |

## Traceability

| Requirement | Phase | Status |
| --- | --- | --- |
| ACT-01 | Phase 100 | Complete |
| ACT-02 | Phase 100 | Complete |
| ACT-03 | Phase 100 | Complete |
| ACT-04 | Phase 100 | Complete |
| INV-01 | Phase 101 | Pending |
| INV-02 | Phase 101 | Pending |
| INV-03 | Phase 101 | Pending |
| INV-04 | Phase 101 | Pending |
| DL-01 | Phase 101 | Pending |
| DL-02 | Phase 101 | Pending |
| DL-03 | Phase 102 | Complete |
| DL-04 | Phase 102 | Complete |
| DL-05 | Phase 102 | Complete |
| MEM-01 | Phase 102 | Complete |
| MEM-02 | Phase 102 | Complete |
| MEM-03 | Phase 103 | Pending |
| MEM-04 | Phase 103 | Pending |
| MEM-05 | Phase 103 | Pending |
| MEM-06 | Phase 103 | Pending |
| REL-01 | Phase 104 | Pending |
| REL-02 | Phase 104 | Pending |
| REL-03 | Phase 104 | Pending |
| REL-04 | Phase 104 | Pending |
| OBS-01 | Phase 105 | Pending |
| OBS-02 | Phase 105 | Pending |
| OBS-03 | Phase 105 | Pending |
| OBS-04 | Phase 105 | Pending |
| BOUND-01 | Phase 106 | Pending |
| BOUND-02 | Phase 106 | Pending |
| BOUND-03 | Phase 106 | Pending |
| BOUND-04 | Phase 106 | Pending |
| BOUND-05 | Phase 106 | Pending |

**Coverage:**

- v2.0 requirements: 32 total
- Mapped to phases: 32
- Unmapped: 0

*Requirements defined: 2026-06-29*
*Last updated: 2026-06-29 after Phase 100 completion*
