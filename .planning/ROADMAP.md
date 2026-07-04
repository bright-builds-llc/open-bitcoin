# Roadmap: Open Bitcoin

## Current Status: v2.1 Ready To Start

v2.1 Block Serving and Compact Block Relay Boundary is the active milestone after v2.0. The roadmap continues phase numbering after Phase 109 and maps all 34 approved v2.1 requirements exactly once.

## Latest Completed Milestone: v2.0 Transaction Relay and Mempool Participation Boundary

**Delivered:** Bounded transaction relay and mempool participation through explicit activation, permission-aware txid/wtxid download, orphan and admission outcomes, durable mempool recovery, relay serving/fanout, sanitized operator evidence, and deterministic no-claim guardrails.

**Boundary:** v2.0 does not claim compact block relay, bloom/filter serving, broad package relay, public transaction relay by default, public-network relay CI, production full-node readiness, production service operation, or production-funds wallet safety.

**Phases completed:** Phases 100 through 109.

**Archive:**

- [v2.0-ROADMAP.md](milestones/v2.0-ROADMAP.md)
- [v2.0-REQUIREMENTS.md](milestones/v2.0-REQUIREMENTS.md)
- [v2.0-MILESTONE-AUDIT.md](milestones/v2.0-MILESTONE-AUDIT.md)

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
- ✅ **v2.0 Transaction Relay and Mempool Participation Boundary** - Phases 100 through 109 (shipped 2026-07-03). Archive: [v2.0-ROADMAP.md](milestones/v2.0-ROADMAP.md)
- 🚧 **v2.1 Block Serving and Compact Block Relay Boundary** - Phases 110 through 117 (ready to start).

## Active Milestone: v2.1 Block Serving and Compact Block Relay Boundary

**Milestone Goal:** Add bounded, opt-in block serving and compact-block relay behavior with Knots parity evidence, deterministic local verification, sanitized operator evidence, and explicit no-claim guardrails for public defaults, package relay, filter serving, production full-node readiness, and production-funds wallet use.

### Phases

- [ ] **Phase 110: Block Serving Activation and Eligibility Boundary** - Establish default-off serving activation, peer eligibility, safe block status classification, and resource-bound policy before any storage read.
- [ ] **Phase 111: Full Block Serving Request Path** - Serve eligible full and witness block requests from validated local block data with bounded request handling and historical/pruned safeguards.
- [ ] **Phase 112: BIP152 Wire Codec and Message Semantics** - Add first-party `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn` payload support with Knots-aligned malformed-input behavior.
- [ ] **Phase 113: Compact Relay Negotiation and Announcement Policy** - Track per-peer compact-block negotiation and decide when compact block announcements are allowed.
- [ ] **Phase 114: Compact Block Reconstruction from Mempool State** - Reconstruct compact blocks from mempool and bounded extra transaction inputs with collision and missing-transaction evidence.
- [ ] **Phase 115: Missing Transaction Round Trip, Fallback, and Validation Handoff** - Complete `getblocktxn`/`blocktxn`, fallback, volatile-state cleanup, and validation/connect integration.
- [ ] **Phase 116: Operator Evidence, Metrics, Logs, and Support Boundary** - Project block-serving and compact-relay truth through shared RPC, CLI, dashboard, metrics, logs, and support surfaces.
- [ ] **Phase 117: Parity Traceability, UAT, and Release Guardrails** - Close parity, UAT, docs, and deterministic no-claim guardrails for the bounded v2.1 release boundary.

### Phase Details

#### Phase 110: Block Serving Activation and Eligibility Boundary

**Goal:** Create the pure activation, peer eligibility, block status, and resource-governance boundary that every v2.1 block-serving effect must pass.
**Depends on:** Phase 109
**Requirements:** BSRV-01, BSRV-02, BSRV-03, BSRV-05, BSRV-06
**Success Criteria** (what must be TRUE):

1. Operator can see that block serving and compact relay are disabled by default and activate only through explicit settings.
2. Peer eligibility is deterministic across outbound, inbound, manual, protected, and permissioned peers without changing service bits or public defaults.
3. Block status classification distinguishes validated, available, stale, side-chain, pruned, unavailable, unvalidated, unknown, and suppressed outcomes before storage reads.
4. Resource-governance tests prove request caps, backpressure, timeouts, churn, ban/discourage, and in-flight cleanup remain active under adversarial block-serving requests.

**Plans:** 3 plans

Plans:

- [ ] 110-01: Activation settings and peer eligibility policy
- [ ] 110-02: Block status classification and safe outcome labels
- [ ] 110-03: Resource-governance integration and default-off guardrails

#### Phase 111: Full Block Serving Request Path

**Goal:** Add the node-shell path that serves eligible full and witness block requests from validated local block data without broad historical or archive-node claims.
**Depends on:** Phase 110
**Requirements:** BSRV-04, GOV-01, GOV-05
**Success Criteria** (what must be TRUE):

1. Eligible peers can request block and witness block inventory and receive the correct validated block serialization.
2. Unknown, stale, side-chain, pruned, unavailable, and ineligible block requests produce deterministic suppress or unavailable evidence.
3. Full block serving participates in existing queue, request, and in-flight limits.
4. Historical and pruned block behavior stays bounded by documented eligibility rules and does not imply archive-node availability.

**Plans:** 3 plans

Plans:

- [ ] 111-01: Full block and witness block `getdata` handling
- [ ] 111-02: Node-shell block read, serve, suppress, and unavailable outcomes
- [ ] 111-03: Historical, pruned, and request-pressure test matrix

#### Phase 112: BIP152 Wire Codec and Message Semantics

**Goal:** Add first-party BIP152 payload support and malformed-input semantics before compact relay runtime behavior depends on it.
**Depends on:** Phase 111
**Requirements:** CMP-01, CMP-02, CMP-03, RCN-01
**Success Criteria** (what must be TRUE):

1. `sendcmpct` version 2 payloads round-trip and unsupported versions follow the documented Knots-compatible boundary.
2. `cmpctblock` payloads encode and decode headers, nonces, six-byte short IDs, and prefilled transaction differential indexes.
3. `getblocktxn` and `blocktxn` payloads encode and decode differential indexes and witness transaction serialization.
4. Malformed compact-block payloads are rejected before partial reconstruction state is accepted.

**Plans:** 3 plans

Plans:

- [ ] 112-01: `sendcmpct` and compact-block message enum support
- [ ] 112-02: `cmpctblock` codec, short IDs, and prefilled transaction fixtures
- [ ] 112-03: `getblocktxn`/`blocktxn` codec and malformed-payload tests

#### Phase 113: Compact Relay Negotiation and Announcement Policy

**Goal:** Track per-peer compact-block capability and decide when compact block announcements are allowed without coupling compact relay to transaction relay or public defaults.
**Depends on:** Phase 112
**Requirements:** CMP-04, CMP-05, CMP-06
**Success Criteria** (what must be TRUE):

1. Peer state records compact-block capability, high-bandwidth preference, low-bandwidth preference, and compact announcement eligibility.
2. New valid blocks are announced as compact blocks only when activation, negotiation, header state, block availability, and resource limits permit it.
3. Compact-block negotiation remains independent from transaction relay, package relay, bloom/filter permissions, compact filters, and public serving defaults.
4. Tests cover unsupported versions, toggled high-bandwidth preference, headers fallback, and default-disabled behavior.

**Plans:** 3 plans

Plans:

- [ ] 113-01: Per-peer `sendcmpct` negotiation state
- [ ] 113-02: Compact block announcement decision policy
- [ ] 113-03: Header/inventory fallback and scope-isolation guardrails

#### Phase 114: Compact Block Reconstruction from Mempool State

**Goal:** Reconstruct compact blocks from current mempool state and bounded extra transaction inputs while producing stable outcomes for collision, duplicate, missing, and failure cases.
**Depends on:** Phase 113
**Requirements:** RCN-02, RCN-03, GOV-04
**Success Criteria** (what must be TRUE):

1. Compact reconstruction uses witness-hash short IDs from current mempool state plus bounded extra or recent block transaction inputs.
2. Short ID collisions, duplicate matches, missing transactions, malformed inputs, and reconstruction failures produce stable typed outcomes.
3. Compact reconstruction integrates with mempool lifecycle, transaction relay, and block connect/disconnect events without activating package relay or filter serving.
4. Partial compact-block state remains volatile and bounded.

**Plans:** 3 plans

Plans:

- [ ] 114-01: BIP152 short-ID helper and reconstruction state model
- [ ] 114-02: Mempool and extra-transaction reconstruction inputs
- [ ] 114-03: Collision, duplicate, missing, and lifecycle integration tests

#### Phase 115: Missing Transaction Round Trip, Fallback, and Validation Handoff

**Goal:** Complete compact-block download by requesting missing transactions, processing `blocktxn`, falling back safely, and handing complete blocks to existing validation/connect logic.
**Depends on:** Phase 114
**Requirements:** RCN-04, RCN-05, RCN-06, RCN-07, GOV-02, GOV-03
**Success Criteria** (what must be TRUE):

1. Node sends bounded `getblocktxn` requests only for eligible peers and expected in-flight partial compact blocks.
2. `blocktxn` responses complete only matching peer/block partial state and reject duplicate, unexpected, out-of-bounds, or mismatched responses.
3. Reconstructed blocks enter the existing validation/connect path with no chainstate mutation from partial compact state.
4. Full-block fallback or suppression handles reconstruction failure, timeout, old/far blocks, peer/resource ineligibility, malformed compact blocks, invalid headers, and cleanup events.
5. Restart, reconnect, disconnect, timeout, and reorg cleanup remove volatile compact-relay state without deleting validated chainstate or durable block data.

**Plans:** 4 plans

Plans:

- [ ] 115-01: Missing transaction request scheduler and in-flight matching
- [ ] 115-02: `blocktxn` response handling and misbehavior outcomes
- [ ] 115-03: Validation/connect handoff and full-block fallback
- [ ] 115-04: Restart, reconnect, timeout, reorg, and duplicate cleanup matrix

#### Phase 116: Operator Evidence, Metrics, Logs, and Support Boundary

**Goal:** Expose truthful block-serving and compact-relay evidence through shared operator surfaces with fixed low-cardinality labels and redaction.
**Depends on:** Phase 115
**Requirements:** OBS-01, OBS-02, OBS-03, OBS-04, OBS-05
**Success Criteria** (what must be TRUE):

1. RPC and shared network status report activation, eligibility, compact negotiation, reconstruction, fallback, and in-flight compact-block state truthfully.
2. CLI and dashboard render block-serving and compact-relay status from the shared contract.
3. Metrics and structured logs use fixed labels for served, suppressed, compact-announced, reconstructed, missing-requested, fallback, malformed, timeout, and cleanup outcomes.
4. Support bundles redact raw peer, permission, credential, transaction payload, and dynamic-label material.
5. Operator UAT docs include repo-local Cargo and Bazel command forms.

**Plans:** 4 plans

Plans:

- [ ] 116-01: Shared block-relay status contract and RPC projection
- [ ] 116-02: CLI and dashboard rendering
- [ ] 116-03: Metrics and structured log labels
- [ ] 116-04: Support redaction and repo-local UAT command docs

#### Phase 117: Parity Traceability, UAT, and Release Guardrails

**Goal:** Close the milestone with Knots source anchors, deterministic checkers, docs, UAT guidance, and no-claim release boundary verification.
**Depends on:** Phase 116
**Requirements:** BOUND-01, BOUND-02, BOUND-03, BOUND-04, BOUND-05
**Success Criteria** (what must be TRUE):

1. Parity docs, source breadcrumbs, and index entries cite Knots anchors for block serving, BIP152 messages, reconstruction, fallback, peer state, and resource governance.
2. Deterministic checkers prevent package relay, bloom/filter serving, compact filter serving, public-serving-default, production-readiness, and production-funds claims from entering v2.1 artifacts.
3. README, operator docs, runtime docs, and release notes describe the bounded v2.1 claim and deferred surfaces clearly.
4. `bash scripts/verify.sh` remains deterministic and free of public-network, wall-clock soak, service-manager, and production-deployment gates.
5. Public-network block-serving or compact-relay review remains opt-in UAT evidence and is not required for pre-commit, default CI, or release-boundary verification.

**Plans:** 4 plans

Plans:

- [ ] 117-01: Parity roots, breadcrumbs, and Knots anchor index
- [ ] 117-02: Deterministic no-claim and verifier-boundary checkers
- [ ] 117-03: README, operator docs, runtime docs, and release notes
- [ ] 117-04: UAT package and milestone release-boundary closure

## Progress

**Execution Order:** 110 -> 111 -> 112 -> 113 -> 114 -> 115 -> 116 -> 117

| Phase | Milestone | Plans Complete | Status | Completed |
| --- | --- | ---: | --- | --- |
| 110. Block Serving Activation and Eligibility Boundary | v2.1 | 0/3 | Not started | - |
| 111. Full Block Serving Request Path | v2.1 | 0/3 | Not started | - |
| 112. BIP152 Wire Codec and Message Semantics | v2.1 | 0/3 | Not started | - |
| 113. Compact Relay Negotiation and Announcement Policy | v2.1 | 0/3 | Not started | - |
| 114. Compact Block Reconstruction from Mempool State | v2.1 | 0/3 | Not started | - |
| 115. Missing Transaction Round Trip, Fallback, and Validation Handoff | v2.1 | 0/4 | Not started | - |
| 116. Operator Evidence, Metrics, Logs, and Support Boundary | v2.1 | 0/4 | Not started | - |
| 117. Parity Traceability, UAT, and Release Guardrails | v2.1 | 0/4 | Not started | - |

## Traceability

- Active requirements: [REQUIREMENTS.md](REQUIREMENTS.md)
- v2.1 research summary: [SUMMARY.md](research/SUMMARY.md)
- Latest requirements archive: [v2.0-REQUIREMENTS.md](milestones/v2.0-REQUIREMENTS.md)
- Latest milestone audit: [v2.0-MILESTONE-AUDIT.md](milestones/v2.0-MILESTONE-AUDIT.md)

**Coverage:**

- v2.1 requirements: 34 total
- Mapped to phases: 34
- Unmapped: 0

## Next Step

Begin Phase 110 with `/gsd-discuss-phase 110` or `/gsd-yolo-discuss-plan-execute-commit-and-push 110`.
