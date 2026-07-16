# Roadmap: Open Bitcoin

## Current Status

v2.1 Block Serving and Compact Block Relay Boundary is the active milestone after v2.0. Phases 110–121 completed the milestone definition of done; the milestone audit found six non-critical debt items, so approved hardening and closeout Phases 122–124 remain before archive.

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
- 🚧 **v2.1 Block Serving and Compact Block Relay Boundary** - Phases 110 through 124 (110–121 complete; 122–124 optional hardening and closeout from [v2.1-MILESTONE-AUDIT.md](v2.1-MILESTONE-AUDIT.md)).

## Active Milestone: v2.1 Block Serving and Compact Block Relay Boundary

**Milestone Goal:** Add bounded, opt-in block serving and compact-block relay behavior with Knots parity evidence, deterministic local verification, sanitized operator evidence, and explicit no-claim guardrails for public defaults, package relay, filter serving, production full-node readiness, and production-funds wallet use.

### Phases

- [x] **Phase 110: Block Serving Activation and Eligibility Boundary** - Establish default-off serving activation, peer eligibility, safe block status classification, and resource-bound policy before any storage read. (completed 2026-07-04)
- [x] **Phase 111: Full Block Serving Request Path** - Serve eligible full and witness block requests from validated local block data with bounded request handling and historical/pruned safeguards. (completed 2026-07-04)
- [x] **Phase 112: BIP152 Wire Codec and Message Semantics** - Add first-party `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn` payload support with Knots-aligned malformed-input behavior. (completed 2026-07-04)
- [x] **Phase 113: Compact Relay Negotiation and Announcement Policy** - Track per-peer compact-block negotiation and decide when compact block announcements are allowed. (completed 2026-07-05)
- [x] **Phase 114: Compact Block Reconstruction from Mempool State** - Reconstruct compact blocks from mempool and bounded extra transaction inputs with collision and missing-transaction evidence. (completed 2026-07-05)
- [x] **Phase 115: Missing Transaction Round Trip, Fallback, and Validation Handoff** - Complete `getblocktxn`/`blocktxn`, fallback, volatile-state cleanup, and validation/connect integration. (completed 2026-07-06)
- [x] **Phase 116: Operator Evidence, Metrics, Logs, and Support Boundary** - Project block-serving and compact-relay truth through shared RPC, CLI, dashboard, metrics, logs, and support surfaces. (completed 2026-07-10)
- [x] **Phase 117: Parity Traceability, UAT, and Release Guardrails** - Close parity, UAT, docs, and deterministic no-claim guardrails for the bounded v2.1 release boundary. (completed 2026-07-10)
- [x] **Phase 118: Outbound Compact Block Announcement Wiring** - Honor compact announcement decisions on the wire by emitting `cmpctblock` (or headers/inventory fallback) without false-positive announce evidence. (completed 2026-07-11)
- [x] **Phase 119: Compact Receive Mempool Candidate Injection** - Feed live mempool and bounded extra candidates into compact-block receive, and hook mempool-remove lifecycle into partial state. (completed 2026-07-13)
- [x] **Phase 120: Compact Download Timeout and Misbehavior Runtime Bridge** - Schedule compact-download timeout expiration from the node runtime and escalate typed compact misbehavior beyond silent suppress. (completed 2026-07-14)
- [x] **Phase 121: Block Relay Metrics and Log Runtime Projection** - Project block-relay metric samples and structured log records through the sync runtime persist/logging path. (completed 2026-07-14)
- [x] **Phase 122: Compact Relay Peer Completion** - Serve eligible inbound `getblocktxn` requests after local compact announcements and align the protocol-path test vocabulary. (completed 2026-07-15)
- [x] **Phase 123: Runtime Timing and Evidence Integrity** - Make timeout scheduling independent of receives and derive relay evidence from actual runtime emissions and the authoritative network instance. (completed 2026-07-16)
- [ ] **Phase 124: Milestone Closeout Reconciliation** - Reconcile milestone metadata, re-audit the completed hardening work, and establish archive readiness.

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

**Plans:** 4/4 plans complete

Plans:

- [x] 110-01-PLAN.md — Activation settings and peer eligibility policy
- [x] 110-02-PLAN.md — Block status classification and safe outcome labels
- [x] 110-03-PLAN.md — Resource-governance and in-flight cleanup coverage
- [x] 110-04-PLAN.md — Docs, parity evidence, and default-off guardrails

#### Phase 111: Full Block Serving Request Path

**Goal:** Add the node-shell path that serves eligible full and witness block requests from validated local block data without broad historical or archive-node claims.
**Depends on:** Phase 110
**Requirements:** BSRV-04, GOV-01, GOV-05
**Success Criteria** (what must be TRUE):

1. Eligible peers can request block and witness block inventory and receive the correct validated block serialization.
2. Unknown, stale, side-chain, pruned, unavailable, and ineligible block requests produce deterministic suppress or unavailable evidence.
3. Full block serving participates in existing queue, request, and in-flight limits.
4. Historical and pruned block behavior stays bounded by documented eligibility rules and does not imply archive-node availability.

**Plans:** 4/4 plans complete

Plans:

- [x] 111-01-PLAN.md — Full block and witness block `getdata` handling
- [x] 111-02-PLAN.md — Node-shell block read, serve, suppress, and unavailable outcomes
- [x] 111-03-PLAN.md — Historical, pruned, and request-pressure test matrix
- [x] 111-04-PLAN.md — Gap closure for checker evidence roots and no-claim guardrails

#### Phase 112: BIP152 Wire Codec and Message Semantics

**Goal:** Add first-party BIP152 payload support and malformed-input semantics before compact relay runtime behavior depends on it.
**Depends on:** Phase 111
**Requirements:** CMP-01, CMP-02, CMP-03, RCN-01
**Success Criteria** (what must be TRUE):

1. `sendcmpct` version 2 payloads round-trip and unsupported versions follow the documented Knots-compatible boundary.
2. `cmpctblock` payloads encode and decode headers, nonces, six-byte short IDs, and prefilled transaction differential indexes.
3. `getblocktxn` and `blocktxn` payloads encode and decode differential indexes and witness transaction serialization.
4. Malformed compact-block payloads are rejected before partial reconstruction state is accepted.

**Plans:** 3/3 plans complete

Plans:

- [x] 112-01: `sendcmpct` and compact-block message enum support
- [x] 112-02: `cmpctblock` codec, short IDs, and prefilled transaction fixtures
- [x] 112-03: `getblocktxn`/`blocktxn` codec and malformed-payload tests

#### Phase 113: Compact Relay Negotiation and Announcement Policy

**Goal:** Track per-peer compact-block capability and decide when compact block announcements are allowed without coupling compact relay to transaction relay or public defaults.
**Depends on:** Phase 112
**Requirements:** CMP-04, CMP-05, CMP-06
**Success Criteria** (what must be TRUE):

1. Peer state records compact-block capability, high-bandwidth preference, low-bandwidth preference, and compact announcement eligibility.
2. New valid blocks are announced as compact blocks only when activation, negotiation, header state, block availability, and resource limits permit it.
3. Compact-block negotiation remains independent from transaction relay, package relay, bloom/filter permissions, compact filters, and public serving defaults.
4. Tests cover unsupported versions, toggled high-bandwidth preference, headers fallback, and default-disabled behavior.

**Plans:** 3/3 plans complete

Plans:

- [x] 113-01: Per-peer `sendcmpct` negotiation state
- [x] 113-02: Compact block announcement decision policy
- [x] 113-03: Header/inventory fallback and scope-isolation guardrails

#### Phase 114: Compact Block Reconstruction from Mempool State

**Goal:** Reconstruct compact blocks from current mempool state and bounded extra transaction inputs while producing stable outcomes for collision, duplicate, missing, and failure cases.
**Depends on:** Phase 113
**Requirements:** RCN-02, RCN-03, GOV-04
**Success Criteria** (what must be TRUE):

1. Compact reconstruction uses witness-hash short IDs from current mempool state plus bounded extra or recent block transaction inputs.
2. Short ID collisions, duplicate matches, missing transactions, malformed inputs, and reconstruction failures produce stable typed outcomes.
3. Compact reconstruction integrates with mempool lifecycle, transaction relay, and block connect/disconnect events without activating package relay or filter serving.
4. Partial compact-block state remains volatile and bounded.

**Plans:** 3/3 plans complete

Plans:

- [x] 114-01-PLAN.md — BIP152 short-ID helper and reconstruction state model
- [x] 114-02-PLAN.md — Mempool and extra-transaction reconstruction inputs
- [x] 114-03-PLAN.md — Collision, duplicate, missing, and lifecycle integration tests

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

**Plans:** 4/4 plans complete

Plans:

- [x] 115-01-PLAN.md — Missing transaction request scheduler and in-flight matching
- [x] 115-02-PLAN.md — `blocktxn` response handling and misbehavior outcomes
- [x] 115-03-PLAN.md — Validation/connect handoff and full-block fallback
- [x] 115-04-PLAN.md — Restart, reconnect, timeout, reorg, and duplicate cleanup matrix

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

**Plans:** 4/4 plans complete

Plans:

- [x] 116-01-PLAN.md — Shared block-relay status contract and RPC projection (OBS-01)
- [x] 116-02-PLAN.md — CLI and dashboard block-relay rendering (OBS-02)
- [x] 116-03-PLAN.md — Metrics and structured log labels (OBS-03)
- [x] 116-04-PLAN.md — Support redaction, checker, docs, and closeout (OBS-04, OBS-05)

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

**Plans:** 4/4 plans complete

Plans:

- [x] 117-01: Parity roots, breadcrumbs, and Knots anchor index
- [x] 117-02: Deterministic no-claim and verifier-boundary checkers
- [x] 117-03: README, operator docs, runtime docs, and release notes
- [x] 117-04: UAT package and milestone release-boundary closure

#### Phase 118: Outbound Compact Block Announcement Wiring

**Goal:** Close the CMP-05 runtime seam so compact announcement decisions produce real outbound `cmpctblock` (or headers/inventory fallback) without false-positive announce evidence.
**Depends on:** Phase 117
**Requirements:** CMP-05
**Gap Closure:** Closes gaps from [v2.1-MILESTONE-AUDIT.md](v2.1-MILESTONE-AUDIT.md) (outbound announce wiring)
**Success Criteria** (what must be TRUE):

1. `ManagedPeerNetwork::announce_block` honors `CompactAnnouncementAction` instead of always emitting Headers/Inv.
2. `AnnounceCompactBlock` builds and sends `WireNetworkMessage::CompactBlock` from the validated local block.
3. Compact-announced evidence increments only when a compact payload is actually sent.
4. Fallback and suppression paths still emit Headers/Inv or no message with stable reasons.

**Plans:** 3/3 plans complete

Plans:

- [x] 118-01-PLAN.md — Pure Block→CompactBlockPayload builder (coinbase-only Knots shape) + unit tests
- [x] 118-02-PLAN.md — PeerManager::announce_block_with_action CompactBlock/Headers/Inv/None emission
- [x] 118-03-PLAN.md — ManagedPeerNetwork action honor + evidence-after-emit (close false-positive CompactAnnounced)

#### Phase 119: Compact Receive Mempool Candidate Injection

**Goal:** Feed live mempool and bounded extra candidates into compact-block receive so reconstruction outcomes and mempool lifecycle hooks work on the runtime path.
**Depends on:** Phase 118
**Requirements:** RCN-02, RCN-03, GOV-04
**Gap Closure:** Closes gaps from [v2.1-MILESTONE-AUDIT.md](v2.1-MILESTONE-AUDIT.md) (mempool candidate injection)
**Success Criteria** (what must be TRUE):

1. Inbound `CompactBlock` dispatch no longer always uses empty `CompactBlockReceiveFacts::default()`.
2. Live receive supplies mempool candidates and bounded extras into `handle_compact_block_download`.
3. `on_mempool_transaction_removed` is hooked from mempool lifecycle without activating package relay or filters.
4. Runtime tests cover reconstruction, collision, duplicate, missing, and lifecycle cleanup outcomes.

**Plans:** 3/3 plans complete

Plans:

- [x] 119-01-PLAN.md — PeerManager mempool-removal forwarder + CompactExtraTxnBuffer helpers
- [x] 119-02-PLAN.md — Shell CompactBlock intercept with mempool/extra facts + admission feeds
- [x] 119-03-PLAN.md — Lifecycle hooks + runtime injected-path tests + parity breadcrumbs

#### Phase 120: Compact Download Timeout and Misbehavior Runtime Bridge

**Goal:** Schedule compact-download timeout expiration from the node runtime and escalate typed compact misbehavior beyond silent suppress.
**Depends on:** Phase 119
**Requirements:** RCN-07, GOV-02, GOV-03
**Gap Closure:** Closes gaps from [v2.1-MILESTONE-AUDIT.md](v2.1-MILESTONE-AUDIT.md) (timeout tick and misbehavior bridge)
**Success Criteria** (what must be TRUE):

1. `expire_compact_download_timeouts` is called from the node/sync runtime on a deterministic tick.
2. Timeout expiration produces full-block fallback or suppression `PeerAction`s on the live path.
3. Disconnect/timeout/reorg cleanup still remove only volatile compact-relay state.
4. Typed compact misbehavior maps to Knots-aligned disconnect, score, or suppression decisions rather than empty-action silence only.

**Plans:** 3/3 plans complete

Plans:

- [x] 120-01-PLAN.md — Timeout tick forwarder + receive_* piggyback + Timeout cleanup evidence (RCN-07, GOV-03)
- [x] 120-02-PLAN.md — Misbehavior escalation beyond silent suppress (GOV-02)
- [x] 120-03-PLAN.md — ReceivedBlock volatile cleanup + runtime proofs + parity breadcrumbs (GOV-03)

#### Phase 121: Block Relay Metrics and Log Runtime Projection

**Goal:** Project block-relay metric samples and structured log records through the sync runtime persist and logging path.
**Depends on:** Phase 120
**Requirements:** OBS-03
**Gap Closure:** Closes gaps from [v2.1-MILESTONE-AUDIT.md](v2.1-MILESTONE-AUDIT.md) (metrics/log runtime projection)
**Success Criteria** (what must be TRUE):

1. `DurableSyncRuntime::persist_metrics` appends `block_relay_metric_samples` when block-relay status is available.
2. Structured sync/logging emits `block_relay_log_record` with fixed low-cardinality labels.
3. Runtime tests prove projection beyond helper-only unit coverage.
4. No raw peer, permission, credential, or transaction payload leakage is introduced.

**Plans:** 2/2 plans complete

Plans:

- [x] 121-01-PLAN.md — Runtime provider + persist_metrics + structured log emission + Rust tests
- [x] 121-02-PLAN.md — open-bitcoind wiring + Phase 121 Bun checker + verify.sh + OBS-03 closeout

#### Phase 122: Compact Relay Peer Completion

**Goal:** Complete the peer-facing BIP152 request/response symmetry so a remote peer can request missing transactions after a locally originated compact announcement.
**Depends on:** Phase 121
**Requirements:** HARD-01
**Gap Closure:** Closes non-critical debt from [v2.1-MILESTONE-AUDIT.md](v2.1-MILESTONE-AUDIT.md) (inbound `GetBlockTxn` serving and stale Phase 112 test naming)
**Success Criteria** (what must be TRUE):

1. Inbound `GetBlockTxn` reaches a bounded live serving path instead of remaining a peer-dispatch no-op.
2. The node serves only eligible, validated, available transactions for the matching locally announced compact block.
3. Invalid, unavailable, or ineligible requests produce stable suppression or misbehavior outcomes without leaking sensitive peer or transaction data.
4. Protocol-path tests and parity evidence cover the new response path, and stale no-op terminology is removed.

**Plans:** 1/1 plans complete

Plans:

- [ ] Pending `/gsd-plan-phase 122`

#### Phase 123: Runtime Timing and Evidence Integrity

**Goal:** Make compact-relay timing and operator evidence reflect authoritative live runtime events rather than receive activity or proxy counts.
**Depends on:** Phase 122
**Requirements:** HARD-02, HARD-03, HARD-04
**Gap Closure:** Closes non-critical debt from [v2.1-MILESTONE-AUDIT.md](v2.1-MILESTONE-AUDIT.md) (idle timeout scheduling, successful block-emission counting, and authoritative runtime metric projection)
**Success Criteria** (what must be TRUE):

1. Compact-download timeouts expire on a deterministic runtime schedule even when the peer connection is otherwise idle.
2. `BlockServedCount` increments only after a successful `WireNetworkMessage::Block` emission.
3. Block-relay metric and log projection samples the same authoritative network instance used by `DurableSyncRuntime`.
4. Focused runtime tests prove idle expiry, post-emission counting, and projection of sync-runtime compact activity.

**Plans:** 7/7 plans complete

Plans:

- [ ] Pending `/gsd-plan-phase 123`

#### Phase 124: Milestone Closeout Reconciliation

**Goal:** Reconcile planning metadata with completed implementation evidence and produce an archive-ready v2.1 audit.
**Depends on:** Phase 123
**Requirements:** HARD-05
**Gap Closure:** Closes non-critical debt from [v2.1-MILESTONE-AUDIT.md](v2.1-MILESTONE-AUDIT.md) (stale roadmap and requirements coverage plus final hardening re-audit)
**Success Criteria** (what must be TRUE):

1. ROADMAP and REQUIREMENTS status, traceability, and coverage agree with the completed phase artifacts.
2. The milestone audit is rerun after Phases 122–123 and records no unresolved approved hardening item.
3. The default deterministic verifier and changed-path milestone checks pass without weakening no-claim guardrails.
4. The active milestone points directly to archival through `/gsd-complete-milestone v2.1`.

**Plans:** TBD

Plans:

- [ ] Pending `/gsd-plan-phase 124`

## Progress

**Execution Order:** 110 -> 111 -> 112 -> 113 -> 114 -> 115 -> 116 -> 117 -> 118 -> 119 -> 120 -> 121 -> 122 -> 123 -> 124

| Phase | Milestone | Plans Complete | Status | Completed |
| --- | --- | ---: | --- | --- |
| 110. Block Serving Activation and Eligibility Boundary | v2.1 | 4/4 | Complete    | 2026-07-04 |
| 111. Full Block Serving Request Path | v2.1 | 4/4 | Complete    | 2026-07-04 |
| 112. BIP152 Wire Codec and Message Semantics | v2.1 | 3/3 | Complete    | 2026-07-04 |
| 113. Compact Relay Negotiation and Announcement Policy | v2.1 | 3/3 | Complete    | 2026-07-05 |
| 114. Compact Block Reconstruction from Mempool State | v2.1 | 3/3 | Complete    | 2026-07-05 |
| 115. Missing Transaction Round Trip, Fallback, and Validation Handoff | v2.1 | 4/4 | Complete    | 2026-07-06 |
| 116. Operator Evidence, Metrics, Logs, and Support Boundary | v2.1 | 4/4 | Complete   | 2026-07-10 |
| 117. Parity Traceability, UAT, and Release Guardrails | v2.1 | 4/4 | Complete    | 2026-07-10 |
| 118. Outbound Compact Block Announcement Wiring | v2.1 | 3/3 | Complete   | 2026-07-11 |
| 119. Compact Receive Mempool Candidate Injection | v2.1 | 3/3 | Complete   | 2026-07-13 |
| 120. Compact Download Timeout and Misbehavior Runtime Bridge | v2.1 | 3/3 | Complete    | 2026-07-14 |
| 121. Block Relay Metrics and Log Runtime Projection | v2.1 | 2/2 | Complete   | 2026-07-14 |
| 122. Compact Relay Peer Completion | v2.1 | 1/1 | Complete    | 2026-07-15 |
| 123. Runtime Timing and Evidence Integrity | v2.1 | 7/7 | Complete    | 2026-07-16 |
| 124. Milestone Closeout Reconciliation | v2.1 | 0/TBD | Not started | - |

## Traceability

- Active requirements: [REQUIREMENTS.md](REQUIREMENTS.md)
- v2.1 research summary: [SUMMARY.md](research/SUMMARY.md)
- Latest requirements archive: [v2.0-REQUIREMENTS.md](milestones/v2.0-REQUIREMENTS.md)
- Active milestone audit: [v2.1-MILESTONE-AUDIT.md](v2.1-MILESTONE-AUDIT.md)
- Latest archived milestone audit: [v2.0-MILESTONE-AUDIT.md](milestones/v2.0-MILESTONE-AUDIT.md)

**Coverage:**

- v2.1 requirements: 39 total
- Mapped to phases: 39
- Satisfied: 34
- Pending hardening and closeout: 5
- Unmapped: 0

## Next Step

Plan Phase 122 with `/gsd-plan-phase 122`, then execute Phases 122–124 and re-audit before archiving v2.1.
