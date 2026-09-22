# Roadmap: Open Bitcoin

## Current Status

v2.4 Prune-Mode Product Behavior is active across Phases 146–151. Phase numbering continues at 146 after v2.3 Phase 145. All 17 requirements are mapped exactly once. Historical `.planning/phases/` directories stay tracked.

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
- 🚧 **v2.4 Prune-Mode Product Behavior** — Phases 146–151 (in progress)

## Active Milestone: v2.4 Prune-Mode Product Behavior

**Milestone Goal:** Add Knots-aligned prune product behavior for the single active chainstate, so the node can delete old block payloads inside a height window and still tell the truth about what remains.

**Boundary:** v2.4 maps Knots prune behavior onto height-selected Fjall block and undo key deletion. It does not add a `blk`/`rev` flat-file store, archive-node serving, assumeutxo or a second chainstate, BIP37 or compact-filter serving, public serving or relay defaults, public-network CI as a release gate, production full-node readiness, or production-funds wallet claims. Functional-core crates stay I/O-free. Historical `.planning/phases/` directories stay tracked.

## Phases

- [ ] **Phase 146: Wallet Leftover-Snapshot Cutover** — Cut wallet rescan off leftover snapshot bytes so prune cannot resurrect snapshot-as-truth.
- [ ] **Phase 147: Pure Prune Policy and Lock Windows** — Encode height windows, 550 MiB target, 288-block keep, and 10-block lock buffer as I/O-free decisions.
- [ ] **Phase 148: Fjall Payload Unlink and Have-Pruned** — Delete paired block and undo keys durably, record have-pruned only after success, and fail closed on interrupted prune.
- [ ] **Phase 149: Limited Serving and Honest Pruned Labels** — Advertise `NODE_NETWORK_LIMITED`, refuse out-of-window and removed payloads, and emit `Pruned` only when earned.
- [ ] **Phase 150: Operator Prune Surfaces and Evidence** — Expose prune status, manual prune, prune locks, and sanitized support evidence on operator surfaces.
- [ ] **Phase 151: Parity Roots and No-Claim Guardrails** — Cite Knots prune anchors including the Fjall-versus-`blk`/`rev` difference and keep deferred claims out.

## Phase Details

### Phase 146: Wallet Leftover-Snapshot Cutover
**Goal**: Wallet rescan treats durable coins and payload-present blocks as chain truth, never leftover snapshot bytes.
**Depends on**: Phase 145
**Requirements**: SNAP-01
**Success Criteria** (what must be TRUE):
  1. Wallet rescan reads coins best-block and payload-present blocks; leftover snapshot blobs are non-authoritative on the wallet path.
  2. After same-datadir reopen with a leftover snapshot present, rescan does not rebuild balances or history from that snapshot.
  3. Contributors can observe that unlink-ready prune work has not yet deleted payloads; cutover alone does not invent have-pruned.
**Plans:** 2/3 plans executed

Plans:
- [x] 146-01-PLAN.md — Cut WalletRescanRuntime to coins + has_block scan authority
- [x] 146-02-PLAN.md — Cut durable RPC rescan seed off leftover snapshot load
- [ ] 146-03-PLAN.md — Parity breadcrumbs and no-prune / leftover-write guardrails

### Phase 147: Pure Prune Policy and Lock Windows
**Goal**: Operators and later unlink get Knots-aligned prune mode, height-window, and lock-buffer decisions without any disk I/O in core.
**Depends on**: Phase 146
**Requirements**: PRUN-01, PRUN-02, PRUN-03, LOCK-01
**Success Criteria** (what must be TRUE):
  1. Operator can disable prune, select manual-only prune, or set an automatic target of at least 550 MiB through typed config/policy shapes.
  2. Automatic prune plans keep the last 288 blocks and do not start before the network prune-after height.
  3. Manual prune refuses a target height inside the 288-block keep window.
  4. A prune lock forbids deleting the locked height range plus a 10-block buffer, decided in `open-bitcoin-chainstate` without Fjall or filesystem access.
**Plans**: TBD

### Phase 148: Fjall Payload Unlink and Have-Pruned
**Goal**: Eligible heights lose paired block and undo payloads on disk, and have-pruned is recorded only after that delete is durable.
**Depends on**: Phase 147
**Requirements**: UNLK-01, UNLK-02, UNLK-03
**Success Criteria** (what must be TRUE):
  1. Pruning a height removes that height's Fjall block payload and undo together; no `blk`/`rev` flat-file store is introduced.
  2. The node records have-pruned only after a non-empty durable delete batch succeeds, never from prune config alone.
  3. Restart after an interrupted prune finishes the partial delete or refuses closed without inventing blocks or triggering reindex.
  4. Heights covered by the Phase 147 lock buffer remain present after a prune attempt that would otherwise delete them.
**Plans**: TBD

### Phase 149: Limited Serving and Honest Pruned Labels
**Goal**: Peers and status see limited-network serving and honest `Pruned` versus `Unavailable` labels only after real deletes.
**Depends on**: Phase 148
**Requirements**: SERV-01, SERV-02, SERV-03, LABL-01
**Success Criteria** (what must be TRUE):
  1. In prune mode the node advertises `NODE_NETWORK_LIMITED` and does not advertise full `NODE_NETWORK`.
  2. A peer request for a block older than the limited serve window is refused.
  3. A peer request for a block whose payload prune removed is not served.
  4. Status and RPC report `Pruned` only when have-pruned is set and the payload is gone; a missing payload without prune stays `Unavailable`.
**Plans**: TBD

### Phase 150: Operator Prune Surfaces and Evidence
**Goal**: Operators can inspect prune state, request manual prune, manage prune locks, and read sanitized support evidence.
**Depends on**: Phase 149
**Requirements**: OPER-01, OPER-02, OPER-03, LOCK-02
**Success Criteria** (what must be TRUE):
  1. Operator can read pruned, prune height, automatic pruning, and prune target from RPC, CLI, and dashboard.
  2. Operator can request a manual prune when prune mode is on and observe the refusal or height outcome.
  3. Operator can list and set prune locks through the operator surface.
  4. Support evidence reports prune counts and last prune height without raw storage paths.
**Plans**: TBD
**UI hint**: yes

### Phase 151: Parity Roots and No-Claim Guardrails
**Goal**: Parity evidence is auditable and the v2.4 claim cannot be read as archive, assumeutxo, BIP37, public defaults, or production readiness.
**Depends on**: Phase 150
**Requirements**: GRD-01
**Success Criteria** (what must be TRUE):
  1. Parity docs cite pinned Knots prune anchors and document the intentional Fjall key versus `blk`/`rev` file difference.
  2. Deterministic checkers still reject archive serving, assumeutxo, BIP37, public defaults, and production-readiness claims.
  3. Default `bash scripts/verify.sh` stays deterministic, and historical `.planning/phases/` directories remain tracked.
**Plans**: TBD

## Requirement Coverage

| Requirement | Phase | Status |
| --- | --- | --- |
| SNAP-01 | Phase 146 | Pending |
| PRUN-01 | Phase 147 | Pending |
| PRUN-02 | Phase 147 | Pending |
| PRUN-03 | Phase 147 | Pending |
| LOCK-01 | Phase 147 | Pending |
| UNLK-01 | Phase 148 | Pending |
| UNLK-02 | Phase 148 | Pending |
| UNLK-03 | Phase 148 | Pending |
| SERV-01 | Phase 149 | Pending |
| SERV-02 | Phase 149 | Pending |
| SERV-03 | Phase 149 | Pending |
| LABL-01 | Phase 149 | Pending |
| OPER-01 | Phase 150 | Pending |
| OPER-02 | Phase 150 | Pending |
| OPER-03 | Phase 150 | Pending |
| LOCK-02 | Phase 150 | Pending |
| GRD-01 | Phase 151 | Pending |

**Coverage:** 17/17 v2.4 requirements mapped. No orphans. No duplicates.

## Research Flags for Planning

These are planning inputs, not extra phases:

- **Phase 148:** Exact Fjall delete-batch atomicity, height-to-key candidate indexing, and crash seams after index clear / after unlink / after coins.
- **Phase 149:** Precise inventory and block-serve fact wiring for limited-only advertisement, including Knots getdata edge cases.

## Progress

**Execution Order:**
Phases execute in numeric order: 146 → 147 → 148 → 149 → 150 → 151

| Phase | Plans Complete | Status | Completed |
| --- | --- | --- | --- |
| 146. Wallet Leftover-Snapshot Cutover | 2/3 | In Progress|  |
| 147. Pure Prune Policy and Lock Windows | 0/TBD | Not started | - |
| 148. Fjall Payload Unlink and Have-Pruned | 0/TBD | Not started | - |
| 149. Limited Serving and Honest Pruned Labels | 0/TBD | Not started | - |
| 150. Operator Prune Surfaces and Evidence | 0/TBD | Not started | - |
| 151. Parity Roots and No-Claim Guardrails | 0/TBD | Not started | - |

## Next Step

Run `/gsd-discuss-phase 146` to discuss wallet leftover-snapshot cutover before planning.

---
*Roadmap created: 2026-09-21 for milestone v2.4. Phase numbering continues from v2.3 Phase 145.*
