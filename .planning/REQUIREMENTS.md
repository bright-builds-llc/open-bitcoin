# Requirements: Open Bitcoin v2.1

**Defined:** 2026-07-03
**Milestone:** v2.1 Block Serving and Compact Block Relay Boundary
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v2.1 Requirements

Requirements for the current milestone. Each requirement maps to exactly one roadmap phase after roadmap approval.

### Block Serving Activation And Eligibility

- [x] **BSRV-01**: Operator can enable block serving and compact-block relay only through explicit activation settings that keep public serving off by default.
- [x] **BSRV-02**: Node classifies block-serving eligibility across outbound, inbound, manual, protected, and permissioned peers without changing service bits or public defaults accidentally.
- [x] **BSRV-03**: Node serves only validated and available blocks inside the documented active-chain or recent-valid boundary.
- [x] **BSRV-04**: Node handles block, witness block, and compact block `getdata` requests with bounded request caps, queue backpressure, and peer cleanup.
- [x] **BSRV-05**: Node reports unknown, stale, side-chain, pruned, unavailable, unvalidated, and suppressed block-serving outcomes without leaking prune height or raw peer details.
- [x] **BSRV-06**: Block serving preserves existing block download, inbound resource-governance, timeout, churn, ban, discourage, and in-flight cleanup limits under adversarial request bursts.

### BIP152 Wire Messages And Negotiation

- [x] **CMP-01**: Node encodes, decodes, and validates `sendcmpct` messages with version 2 semantics and documented handling for unsupported versions.
- [x] **CMP-02**: Node encodes, decodes, and validates `cmpctblock` payloads with header, nonce, six-byte short IDs, and prefilled transaction differential indexes.
- [x] **CMP-03**: Node encodes, decodes, and validates `getblocktxn` and `blocktxn` payloads with differential indexes and witness transaction serialization.
- [ ] **CMP-04**: Node tracks per-peer compact-block capability, high-bandwidth preference, low-bandwidth preference, and compact-block announcement eligibility deterministically.
- [ ] **CMP-05**: Node announces compact blocks only when activation, peer negotiation, header state, block availability, and resource limits permit it.
- [x] **CMP-06**: Compact-block negotiation remains independent from transaction relay, package relay, bloom/filter permissions, compact filters, and public serving defaults.

### Compact Block Reconstruction And Fallback

- [x] **RCN-01**: Node validates compact block headers, transaction counts, prefilled ordering, null transactions, short ID bounds, and malformed payloads before accepting partial state.
- [x] **RCN-02**: Node reconstructs compact blocks from current mempool state plus bounded extra or recent block transaction inputs using witness-hash short IDs.
- [x] **RCN-03**: Node detects short ID collisions, duplicate matches, missing transactions, and reconstruction failures with stable typed outcomes.
- [x] **RCN-04**: Node requests missing compact-block transactions with bounded `getblocktxn` indexes only when the peer and in-flight state are eligible.
- [x] **RCN-05**: Node accepts `blocktxn` responses only for expected in-flight partial compact blocks from the matching peer and rejects duplicate, unexpected, out-of-bounds, or mismatched responses.
- [x] **RCN-06**: Reconstructed blocks enter the existing block validation and connect path without mutating chainstate from partial compact-block state.
- [x] **RCN-07**: Node falls back to full block fetch or suppression when reconstruction fails, responses timeout, blocks are old or far from the active tip, or peer/resource state becomes ineligible.

### Resource Governance And Runtime Integration

- [x] **GOV-01**: Full block serving, compact block serving, partial compact-block state, missing transaction requests, and fallback all participate in existing request, queue, and in-flight resource limits.
- [x] **GOV-02**: Malformed compact blocks, invalid compact-block headers, duplicate `blocktxn`, unexpected `blocktxn`, and out-of-bounds indexes produce Knots-aligned misbehavior, disconnect, or suppression decisions.
- [x] **GOV-03**: Restart, reconnect, disconnect, timeout, and reorg cleanup remove volatile compact-relay state without deleting validated chainstate or durable block data.
- [x] **GOV-04**: Compact block relay integrates with mempool lifecycle, transaction relay, and block connect/disconnect events without activating package relay or filter serving.
- [x] **GOV-05**: Historical, pruned, stale, side-chain, and unavailable block serving remains bounded by documented eligibility rules and does not imply archive-node behavior.

### Operator, RPC, Metrics, Logs, And Support Evidence

- [ ] **OBS-01**: RPC and shared network status report block-serving activation, serving eligibility, compact negotiation, reconstruction, fallback, and in-flight compact-block state truthfully.
- [x] **OBS-02**: CLI and dashboard surfaces render block-serving and compact-block relay state from the shared status contract without raw peer, permission, credential, or transaction payload leakage.
- [ ] **OBS-03**: Metrics and structured logs use fixed low-cardinality labels for served, suppressed, compact-announced, reconstructed, missing-requested, fallback, malformed, timeout, and cleanup outcomes.
- [x] **OBS-04**: Support bundles sanitize block-serving and compact-relay evidence, including raw transaction lists, raw peer endpoints, permission strings, credentials, and dynamic labels.
- [x] **OBS-05**: Operator docs and UAT guidance provide copy-pasteable repo-local Cargo and Bazel commands for block-serving and compact-relay workflows.

### Parity, UAT, And Release Boundary

- [x] **BOUND-01**: Parity docs, source breadcrumbs, and index entries cite concrete Bitcoin Knots anchors for block serving, BIP152 messages, reconstruction, fallback, peer state, and resource governance.
- [ ] **BOUND-02**: Deterministic checkers prevent package relay, bloom/filter serving, compact filter serving, public-serving-default, production-readiness, and production-funds claims from entering v2.1 artifacts.
- [x] **BOUND-03**: README, operator docs, runtime docs, and release notes describe the bounded v2.1 block-serving and compact-relay claim and list deferred surfaces clearly.
- [x] **BOUND-04**: The default `bash scripts/verify.sh` contract remains deterministic and free of public-network, wall-clock soak, service-manager, and production-deployment gates.
- [x] **BOUND-05**: Public-network block-serving or compact-relay review remains opt-in UAT evidence and is never required for pre-commit, default CI, or release-boundary verification.

### Milestone Hardening And Closeout

- [x] **HARD-01**: Node serves eligible inbound `getblocktxn` requests for locally announced compact blocks through a bounded, parity-auditable peer path.
- [x] **HARD-02**: Compact-download timeout expiration advances on a deterministic runtime schedule even when no further peer message is received.
- [x] **HARD-03**: Served-block evidence derives from successful `WireNetworkMessage::Block` emission rather than eligible-peer proxy counts.
- [x] **HARD-04**: Runtime block-relay metrics and logs sample the authoritative network instance used by `DurableSyncRuntime`.
- [ ] **HARD-05**: Roadmap, requirement coverage, phase status, and the final milestone audit agree and route v2.1 directly to archival.

## Deferred Requirements

Deferred to future milestones. These are acknowledged but not part of the v2.1 roadmap.

### Relay Expansion

- **FUT-01**: Node supports broad package relay, cluster mempool policy, and package orphan behavior.
- **FUT-02**: Node enables public block serving, compact block relay, and transaction relay by default with production-ready abuse, support, service, packaging, and firewall guidance.
- **FUT-03**: Node runs public-network relay UAT as a CI or release-blocking default gate.
- **FUT-04**: Node claims archive-node or broad historical block-serving behavior.

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

Explicitly excluded from v2.1 to prevent scope creep.

| Feature | Reason |
| --- | --- |
| Package relay and cluster mempool | Requires package policy, orphanage, and parity work beyond compact block relay. |
| Bloom filters and compact filters | Separate privacy and filter-serving surfaces with different abuse and support risks. |
| Public serving by default | Requires production abuse, service, packaging, firewall, and support readiness that v2.1 does not claim. |
| Archive-node historical serving | v2.1 proves bounded block serving, not unbounded historical availability. |
| Public-network relay CI by default | Would violate the deterministic local verification contract. |
| Production full-node readiness | Governed by v1.8 readiness gates, not established by compact block relay alone. |
| Production-funds wallet safety | Wallet safety and support evidence are separate milestones. |
| GUI, hosted dashboard, packaging, installer, and service deployment | Product and distribution surfaces are deferred while relay internals remain the focus. |
| Migration apply mode | Migration remains dry-run-first and must not mutate source datadirs, services, configs, or wallets in v2.1. |

## Traceability

Traceability is populated by the v2.1 roadmap. Each active requirement maps to exactly one phase.

| Requirement | Phase | Status |
| --- | --- | --- |
| BSRV-01 | Phase 110 | Complete |
| BSRV-02 | Phase 110 | Complete |
| BSRV-03 | Phase 127 | Complete |
| BSRV-04 | Phase 127 | Complete |
| BSRV-05 | Phase 110 | Complete |
| BSRV-06 | Phase 110 | Complete |
| CMP-01 | Phase 112 | Complete |
| CMP-02 | Phase 112 | Complete |
| CMP-03 | Phase 112 | Complete |
| CMP-04 | Phase 128 | Pending |
| CMP-05 | Phase 128 | Pending |
| CMP-06 | Phase 113 | Complete |
| RCN-01 | Phase 112 | Complete |
| RCN-02 | Phase 126 | Complete |
| RCN-03 | Phase 126 | Complete |
| RCN-04 | Phase 125 | Complete |
| RCN-05 | Phase 125 | Complete |
| RCN-06 | Phase 125 | Complete |
| RCN-07 | Phase 120 | Complete |
| GOV-01 | Phase 111 | Complete |
| GOV-02 | Phase 120 | Complete |
| GOV-03 | Phase 120 | Complete |
| GOV-04 | Phase 126 | Complete |
| GOV-05 | Phase 111 | Complete |
| OBS-01 | Phase 129 | Pending |
| OBS-02 | Phase 127 | Complete |
| OBS-03 | Phase 128 | Pending |
| OBS-04 | Phase 127 | Complete |
| OBS-05 | Phase 116 | Complete |
| BOUND-01 | Phase 126 | Complete |
| BOUND-02 | Phase 129 | Pending |
| BOUND-03 | Phase 117 | Complete |
| BOUND-04 | Phase 117 | Complete |
| BOUND-05 | Phase 117 | Complete |
| HARD-01 | Phase 122 | Complete |
| HARD-02 | Phase 123 | Complete |
| HARD-03 | Phase 123 | Complete |
| HARD-04 | Phase 123 | Complete |
| HARD-05 | Phase 129 | Pending |

**Coverage:**

- v2.1 requirements: 39 total
- Mapped to phases: 39
- Complete: 33
- Pending integration gap closure: 6
- Unmapped: 0

*Requirements defined: 2026-07-03*
*Last updated: 2026-07-20 after Phase 127 authoritative-state verification*
