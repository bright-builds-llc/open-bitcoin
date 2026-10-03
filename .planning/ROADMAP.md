# Roadmap: Open Bitcoin

## Current Status

v2.4 Prune-Mode Product Behavior is active across Phases 146–153. Phases 146–151 delivered their original plans; the 2026-10-02 audit reopened three requirements for gap closure. Phase 152 now closes SNAP-01 with passing runtime and lifecycle evidence. All 17 requirements are mapped exactly once: 15 Complete, two Pending in Phase 153. Historical `.planning/phases/` directories stay tracked.

## Latest Completed Milestone: v2.3 Chainstate Durability and Historical Serving

**Delivered:** Disk-backed per-outpoint coins, typed cache-flush policy, fuller chainstate-manager behavior for the single active chainstate, and honest stored-block availability that serves or reports a stored block only when the payload bytes are present.

**Boundary:** v2.3 is storage-first. It does not add prune or archive product modes, assumeutxo / assumevalid / IBD snapshot shortcuts, compact-filter or BIP37 serving, LevelDB or rust-bitcoin, public serving or relay defaults, public-network CI as a release gate, production full-node readiness, or production-funds wallet claims. Functional-core crates stay I/O-free. Historical `.planning/phases/` directories stay tracked.

**Phases completed:** Phases 139 through 145 (29 plans).

**Archive:**

- [v2.3-ROADMAP.md](milestones/v2.3-ROADMAP.md)
- [v2.3-REQUIREMENTS.md](milestones/v2.3-REQUIREMENTS.md)
- [v2.3-MILESTONE-AUDIT.md](milestones/v2.3-MILESTONE-AUDIT.md)

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
- ✅ **v2.2 Package Relay and Long-Lived Mempool Policy** — Phases 130–138, including inserted 133.1 (shipped 2026-08-22). Archive: [v2.2-ROADMAP.md](milestones/v2.2-ROADMAP.md)
- ✅ **v2.3 Chainstate Durability and Historical Serving** — Phases 139–145 (shipped 2026-09-20). Archive: [v2.3-ROADMAP.md](milestones/v2.3-ROADMAP.md)
- 🚧 **v2.4 Prune-Mode Product Behavior** — Phases 146–153 (gap closure in progress)

## Active Milestone: v2.4 Prune-Mode Product Behavior

**Milestone Goal:** Add Knots-aligned prune product behavior for the single active chainstate, so the node can delete old block payloads inside a height window and still tell the truth about what remains.

**Boundary:** v2.4 maps Knots prune behavior onto height-selected Fjall block and undo key deletion. It does not add a `blk`/`rev` flat-file store, archive-node serving, assumeutxo or a second chainstate, BIP37 or compact-filter serving, public serving or relay defaults, public-network CI as a release gate, production full-node readiness, or production-funds wallet claims. Functional-core crates stay I/O-free. Historical `.planning/phases/` directories stay tracked.

## Phases

- [x] **Phase 146: Wallet Leftover-Snapshot Cutover** — Cut wallet rescan off leftover snapshot bytes so prune cannot resurrect snapshot-as-truth. (completed 2026-09-22)
- [x] **Phase 147: Pure Prune Policy and Lock Windows** — Encode height windows, 550 MiB target, 288-block keep, and 10-block lock buffer as I/O-free decisions. (completed 2026-09-22)
- [x] **Phase 148: Fjall Payload Unlink and Have-Pruned** — Delete paired block and undo keys durably, record have-pruned only after success, and fail closed on interrupted prune. (completed 2026-09-27)
- [x] **Phase 149: Limited Serving and Honest Pruned Labels** — Advertise `NODE_NETWORK_LIMITED`, refuse out-of-window and removed payloads, and emit `Pruned` only when earned. (completed 2026-09-28)
- [x] **Phase 150: Operator Prune Surfaces and Evidence** — Expose prune status, manual prune, prune locks, and sanitized support evidence on operator surfaces. (completed 2026-09-29)
- [x] **Phase 151: Parity Roots and No-Claim Guardrails** — Cite Knots prune anchors including the Fjall-versus-`blk`/`rev` difference and keep deferred claims out. (completed 2026-09-30)
- [x] **Phase 152: Post-Prune Wallet Rescan Eligibility** — Enforce creating-payload eligibility after real pruning in both scan adapters. (completed 2026-10-03)
- [ ] **Phase 153: Automatic Prune Retention Integration** — Wire measured retained payloads and the automatic target through the existing planner and durable owner.

## Phase Details

### Phase 146: Wallet Leftover-Snapshot Cutover
**Goal**: Wallet rescan treats durable coins and payload-present blocks as chain truth, never leftover snapshot bytes.
**Depends on**: Phase 145
**Historical Requirements**: SNAP-01; current closure ownership is Phase 152.
**Success Criteria** (what must be TRUE):
  1. Wallet rescan reads coins best-block and payload-present blocks; leftover snapshot blobs are non-authoritative on the wallet path.
  2. After same-datadir reopen with a leftover snapshot present, rescan does not rebuild balances or history from that snapshot.
  3. Contributors can observe that unlink-ready prune work has not yet deleted payloads; cutover alone does not invent have-pruned.
**Plans:** 3/3 plans complete

Plans:
- [x] 146-01-PLAN.md — Cut WalletRescanRuntime to coins + has_block scan authority
- [x] 146-02-PLAN.md — Cut durable RPC rescan seed off leftover snapshot load
- [x] 146-03-PLAN.md — Parity breadcrumbs and no-prune / leftover-write guardrails

### Phase 147: Pure Prune Policy and Lock Windows
**Goal**: Operators and later unlink get Knots-aligned prune mode, height-window, and lock-buffer decisions without any disk I/O in core.
**Depends on**: Phase 146
**Requirements**: PRUN-03, LOCK-01
**Historical Requirements**: PRUN-01, PRUN-02; current closure ownership is Phase 153.
**Success Criteria** (what must be TRUE):
  1. Operator can disable prune, select manual-only prune, or set an automatic target of at least 550 MiB through typed config/policy shapes.
  2. Automatic prune plans keep the last 288 blocks and do not start before the network prune-after height.
  3. Manual prune refuses a target height inside the 288-block keep window.
  4. A prune lock forbids deleting the locked height range plus a 10-block buffer, decided in `open-bitcoin-chainstate` without Fjall or filesystem access.
**Plans:** 3/3 plans complete
Plans:
- [x] 147-01-PLAN.md — Typed PruneMode parse, keep-window helpers, lock-buffer predicates, chainstate-prune breadcrumbs
- [x] 147-02-PLAN.md — Automatic prune planner (288 keep, prune-after empty plan, injected byte budget, lock omit)
- [x] 147-03-PLAN.md — Manual prune planner with keep-window typed refusal (no clamp) and lock omit

### Phase 148: Fjall Payload Unlink and Have-Pruned
**Goal**: Eligible heights lose paired block and undo payloads on disk, and have-pruned is recorded only after that delete is durable.
**Depends on**: Phase 147
**Requirements**: UNLK-01, UNLK-02, UNLK-03
**Success Criteria** (what must be TRUE):
  1. Pruning a height removes that height's Fjall block payload and undo together; no `blk`/`rev` flat-file store is introduced.
  2. The node records have-pruned only after a non-empty durable delete batch succeeds, never from prune config alone.
  3. Restart after an interrupted prune finishes the partial delete or refuses closed without inventing blocks or triggering reindex.
  4. Heights covered by the Phase 147 lock buffer remain present after a prune attempt that would otherwise delete them.
**Plans:** 5/5 plans complete

Plans:
- [x] 148-01-PLAN.md — Paired Fjall delete and have-pruned marker
- [x] 148-02-PLAN.md — Flush owner consumes the prune plan
- [x] 148-03-PLAN.md — Finish or refuse an interrupted prune
- [x] 148-04-PLAN.md — Drop deleted hashes from the block cache
- [x] 148-05-PLAN.md — Evict cache hashes when a committed unlink is followed by a flush error

### Phase 149: Limited Serving and Honest Pruned Labels
**Goal**: Peers and status see limited-network serving and honest `Pruned` versus `Unavailable` labels only after real deletes.
**Depends on**: Phase 148
**Requirements**: SERV-01, SERV-02, SERV-03, LABL-01
**Success Criteria** (what must be TRUE):
  1. In prune mode the node advertises `NODE_NETWORK_LIMITED` and does not advertise full `NODE_NETWORK`.
  2. A peer request for a block older than the limited serve window is refused.
  3. A peer request for a block whose payload prune removed is not served.
  4. Status and RPC report `Pruned` only when have-pruned is set and the payload is gone; a missing payload without prune stays `Unavailable`.
**Plans:** 4/4 plans complete

Plans:
- [x] 149-01-PLAN.md — Pure NETWORK_LIMITED bit and limited-serve window predicates
- [x] 149-02-PLAN.md — Advertise limited service from PruneMode on the version message
- [x] 149-03-PLAN.md — Refuse out-of-window and pruned block bodies
- [x] 149-04-PLAN.md — Project honest Pruned status only after have-pruned

### Phase 150: Operator Prune Surfaces and Evidence
**Goal**: Operators can inspect prune state, request manual prune, manage prune locks, and read sanitized support evidence.
**Depends on**: Phase 149
**Requirements**: OPER-01, OPER-02, OPER-03, LOCK-02
**Success Criteria** (what must be TRUE):
  1. Operator can read pruned, prune height, automatic pruning, and prune target from RPC, CLI, and dashboard.
  2. Operator can request a manual prune when prune mode is on and observe the refusal or height outcome.
  3. Operator can list and set prune locks through the operator surface.
  4. Support evidence reports prune counts and last prune height without raw storage paths.
**Plans:** 8/8 plans complete

Plans:
- [x] 150-01-PLAN.md — Pure GetPruneHeight, quartet projection, and timestamp pre-step
- [x] 150-02-PLAN.md — JSONC prune integer and startup prune mode
- [x] 150-03-PLAN.md — Durable prune locks, support counters, and resume
- [x] 150-04-PLAN.md — getblockchaininfo quartet and status snapshot facts
- [x] 150-05-PLAN.md — pruneblockchain and lock RPC
- [x] 150-06-PLAN.md — Read-only dashboard and CLI status
- [x] 150-07-PLAN.md — CLI prune run and prune lock routing
- [x] 150-08-PLAN.md — Sanitized support prune evidence

**UI hint**: yes

### Phase 151: Parity Roots and No-Claim Guardrails
**Goal**: Parity evidence is auditable and the v2.4 claim cannot be read as archive, assumeutxo, BIP37, public defaults, or production readiness.
**Depends on**: Phase 150
**Requirements**: GRD-01
**Success Criteria** (what must be TRUE):
  1. Parity docs cite pinned Knots prune anchors and document the intentional Fjall key versus `blk`/`rev` file difference.
  2. Deterministic checkers still reject archive serving, assumeutxo, BIP37, public defaults, and production-readiness claims.
  3. Default `bash scripts/verify.sh` stays deterministic, and historical `.planning/phases/` directories remain tracked.
**Plans:** 4/4 plans complete

Plans:
- [x] 151-01-PLAN.md — Backfill v2.4 parity surfaces and catalog anchors
- [x] 151-02-PLAN.md — Add the Phase 151 no-claim checker and fixtures
- [x] 151-03-PLAN.md — Publish the scoped claim, UAT package, and verifier wiring
- [x] 151-04-PLAN.md — Flip leftover Pending rows without archiving v2.4

### Phase 152: Post-Prune Wallet Rescan Eligibility
**Goal**: Node and durable RPC wallet rescans admit entries only when their creating payloads are present, including midrange scans and chunk resume after actual pruning; leftover snapshots remain non-authoritative.
**Depends on**: Phase 151
**Requirements**: SNAP-01
**Gap Closure**: INT-02; closes the real-prune → midrange-rescan flow in [the v2.4 audit](v2.4-MILESTONE-AUDIT.md).
**Success Criteria** (what must be TRUE):
  1. Both scan paths enforce eligibility for every entry admitted to a full replacement, including creating heights before the requested start.
  2. A post-prune midrange scan cannot silently replace the wallet with entries whose creating payload is absent; refusal preserves prior wallet state.
  3. Chunk resume and same-datadir reopen preserve the same rule and reject leftover snapshot authority.
  4. Runtime regressions using real paired deletion and the default verifier pass; source-string assertions alone do not prove the flow.
**Planning Tasks**:
  1. Define one full-replacement eligibility contract grounded in Phase 146 D-04/D-08, keeping I/O in the shell.
  2. Enforce it in node chunk/resume and durable RPC range adapters, including durable refusal evidence.
  3. Cover deleted creating heights outside the requested range, resume/reopen, missing in-range payloads, and retained-payload controls.
  4. Record lifecycle-valid SNAP-01 evidence, parity breadcrumbs, and relevant docs after repo-native verification.
**Plans:** 3/3 plans complete

Plans:
- [x] 152-01-PLAN.md — Shared full-replacement eligibility, node authority and real prune/resume regressions
- [x] 152-02-PLAN.md — Durable RPC integration and real prune/reopen regressions
- [x] 152-03-PLAN.md — Scoped parity, README status and closure evidence
**UI hint**: no

### Phase 153: Automatic Prune Retention Integration
**Goal**: A configured automatic target drives retention in the running durable lifecycle through the existing pure planner and paired-unlink owner, preserving keep-window, prune-after, locks, recovery, and cache consistency.
**Depends on**: Phase 152
**Requirements**: PRUN-01, PRUN-02
**Gap Closure**: INT-01; closes the automatic-target → ongoing-retention flow in [the v2.4 audit](v2.4-MILESTONE-AUDIT.md).
**Success Criteria** (what must be TRUE):
  1. An automatic target of at least 550 MiB has a production planner caller using measured retained block/undo payload facts.
  2. Ordinary lifecycle activity can durably delete eligible paired payloads above the target without a manual RPC request.
  3. The 288-block window, network prune-after height, durable lock buffers, and disabled/manual-only behavior remain protected.
  4. Real-delete labels, cache/undo consistency, restart finish-or-refuse, and Phase 152 wallet eligibility remain correct under runtime tests and full verification.
**Planning Tasks**:
  1. Assemble authoritative per-height usage, target, chain parameters, tip, and current durable locks in the shell.
  2. Invoke the existing automatic planner from production retention/flush and apply through the durable owner with callback-based cache/undo cleanup.
  3. Cover target exceedance, no-op gates, lock/window protection, later-flush error, and reopen using real storage and the production lifecycle.
  4. Verify operator/support/wallet regressions, refresh parity/product evidence, and re-audit the milestone after closure.
**Plans:** 0 plans; run `/gsd-plan-phase 153` after Phase 152.
**UI hint**: no

## Requirement Coverage

| Requirement | Phase | Status |
| --- | --- | --- |
| SNAP-01 | Phase 152 | Complete |
| PRUN-01 | Phase 153 | Pending |
| PRUN-02 | Phase 153 | Pending |
| PRUN-03 | Phase 147 | Complete |
| LOCK-01 | Phase 147 | Complete |
| UNLK-01 | Phase 148 | Complete |
| UNLK-02 | Phase 148 | Complete |
| UNLK-03 | Phase 148 | Complete |
| SERV-01 | Phase 149 | Complete |
| SERV-02 | Phase 149 | Complete |
| SERV-03 | Phase 149 | Complete |
| LABL-01 | Phase 149 | Complete |
| OPER-01 | Phase 150 | Complete |
| OPER-02 | Phase 150 | Complete |
| OPER-03 | Phase 150 | Complete |
| LOCK-02 | Phase 150 | Complete |
| GRD-01 | Phase 151 | Complete |

**Coverage:** 17/17 v2.4 requirements mapped; 15 Complete, two Pending. No orphans. No duplicates.

## Research Flags for Planning

These are planning inputs, not extra phases:

- **Phase 148:** Exact Fjall delete-batch atomicity, height-to-key candidate indexing, and crash seams after index clear / after unlink / after coins.
- **Phase 149:** Precise inventory and block-serve fact wiring for limited-only advertisement, including Knots getdata edge cases.
- **Phase 152:** Full replacement versus incremental eligibility, probes outside requested ranges, and refusal without changing prior wallet state.
- **Phase 153:** Fjall block/undo byte accounting and retention cadence; reuse the owner and keep the temporary IBD target deferred.

## Progress

**Execution Order:**
Phases execute in numeric order: 146 → 147 → 148 → 149 → 150 → 151 → 152 → 153. Wallet eligibility closes before routine automatic deletion is enabled.

| Phase | Plans Complete | Status | Completed |
| --- | --- | --- | --- |
| 146. Wallet Leftover-Snapshot Cutover | 3/3 | Complete    | 2026-09-22 |
| 147. Pure Prune Policy and Lock Windows | 3/3 | Complete    | 2026-09-22 |
| 148. Fjall Payload Unlink and Have-Pruned | 5/5 | Complete    | 2026-09-27 |
| 149. Limited Serving and Honest Pruned Labels | 4/4 | Complete    | 2026-09-28 |
| 150. Operator Prune Surfaces and Evidence | 8/8 | Complete    | 2026-09-29 |
| 151. Parity Roots and No-Claim Guardrails | 4/4 | Complete   | 2026-09-30 |
| 152. Post-Prune Wallet Rescan Eligibility | 3/3 | Complete    | 2026-10-03 |
| 153. Automatic Prune Retention Integration | 0/0 | Pending planning | - |

## Next Step

Run `/gsd-discuss-phase 153` or `/gsd-plan-phase 153` for automatic retention. Phase 152 passed; after Phase 153 passes, rerun `/gsd-audit-milestone`. Milestone archival remains `/gsd-complete-milestone v2.4` only after re-audit establishes closure.

---
*Roadmap created: 2026-09-21 for milestone v2.4. Phase numbering continues from v2.3 Phase 145.*
