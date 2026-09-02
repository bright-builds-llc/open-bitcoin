# Roadmap: Open Bitcoin

## Current Status

v2.3 Chainstate Durability and Historical Serving is planned across Phases 139–145. All 15 requirements are mapped exactly once.

## Latest Completed Milestone: v2.2 Package Relay and Long-Lived Mempool Policy

**Delivered:** Knots-aligned package admission and opportunistic same-peer 1P1C relay, deterministic long-lived pressure policy, durable mempool recovery, bounded initial broadcast retry, sanitized operator evidence, and last-gate release guardrails.

**Boundary:** v2.2 does not add a general package wire protocol, arbitrary multi-parent peer assembly, cluster mempool, whole-mempool rebroadcast, public/default relay, guaranteed propagation, public-network default verification, production service operation, production full-node readiness, or production-funds wallet claims.

**Phases completed:** Phases 130 through 138, including inserted 133.1 (95 plans).

**Archive:**

- [v2.2-ROADMAP.md](milestones/v2.2-ROADMAP.md)
- [v2.2-REQUIREMENTS.md](milestones/v2.2-REQUIREMENTS.md)
- [v2.2-MILESTONE-AUDIT.md](milestones/v2.2-MILESTONE-AUDIT.md)

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
- 🚧 **v2.3 Chainstate Durability and Historical Serving** — Phases 139–145 (planned)

## Active Milestone: v2.3 Chainstate Durability and Historical Serving

**Milestone Goal:** Replace snapshot-style coin persistence with disk-backed coins, cache-flush policy, and fuller chainstate-manager behavior, while keeping block-serving honest about what is actually stored.

**Boundary:** v2.3 is storage-first. It does not add prune or archive product modes, assumeutxo / assumevalid / IBD snapshot shortcuts, compact-filter or BIP37 serving, LevelDB or rust-bitcoin, public serving or relay defaults, public-network CI as a release gate, production full-node readiness, or production-funds wallet claims. Functional-core crates stay I/O-free. Historical `.planning/phases/` directories stay tracked.

## Phases

- [x] **Phase 139: Coins-View, Cache Contract, and Engine Apply** — Overlay a typed DIRTY/FRESH coins cache so connect and disconnect stop cloning or rewriting the whole UTXO set. (completed 2026-08-31)
- [ ] **Phase 140: Pure Flush Policy and Typed Decisions** — Decide IfNeeded, Periodic, Always, and disk-space refusal from injected facts in an I/O-free state machine.
- [ ] **Phase 141: Durable Fjall Coins Adapter** — Persist per-outpoint coins, best-block, and interrupted-flush markers; fail closed on coins disk-read errors.
- [ ] **Phase 142: Manager Flush Lifecycle and Restart** — One manager owns coins init, ordered flush, interrupted-flush recovery, and restart from coins best-block.
- [ ] **Phase 143: Honest Stored-Block Availability** — Serve or report Available only when payload bytes are present; refuse cleanly when they are not.
- [ ] **Phase 144: Operator Flush and Availability Evidence** — Expose sanitized flush, recovery, cache-size, and have-bytes vs do-not facts on operator surfaces.
- [ ] **Phase 145: Parity Roots and No-Claim Guardrails** — Cite Knots coins/flush/manager/serve-path anchors and keep deferred claims out of v2.3.

## Phase Details

### Phase 139: Coins-View, Cache Contract, and Engine Apply
**Goal**: Connect and disconnect mutate an in-memory DIRTY/FRESH coins overlay instead of cloning or rewriting the whole UTXO set.
**Depends on**: Phase 138
**Requirements**: CACHE-01
**Success Criteria** (what must be TRUE):
  1. Contributors can apply connect and disconnect against a typed `CoinsView` / `CoinsCache` overlay with DIRTY/FRESH transitions, without cloning the full UTXO map.
  2. A cache hit is distinguishable from parent-view truth; HaveCoin and in-cache occupancy remain separate facts.
  3. Prepare/commit uses a child cache flushed into the parent cache, and a failed prepare leaves the parent view unchanged.
  4. The chainstate core remains I/O-free: no Fjall, filesystem, or clock appears in `open-bitcoin-chainstate`.
**Plans**: 4 plans

Plans:
- [x] 139-01-PLAN.md — CoinsView contract, MemoryCoinsView, and four lookup facts
- [x] 139-02-PLAN.md — DIRTY/FRESH AddCoin/SpendCoin/BatchWrite/Flush/Sync algebra
- [x] 139-03-PLAN.md — Engine apply on child overlay without next_utxos clone
- [x] 139-04-PLAN.md — Manager prepare/commit overlay isolation and leftover persist

### Phase 140: Pure Flush Policy and Typed Decisions
**Goal**: Flush and recovery decisions are a typed, injectable policy the later manager can execute without clocks or I/O in core.
**Depends on**: Phase 139
**Requirements**: FLUSH-01, MGR-03
**Success Criteria** (what must be TRUE):
  1. Given injected cache-size, time, disk-space, and mode facts, the node decides None, IfNeeded, Periodic, or Always without reading a clock or disk itself.
  2. Disk-space refusal is a first-class decision outcome, not a later adapter surprise.
  3. Cache-size state is classified as OK, LARGE, or CRITICAL from injected occupancy facts.
  4. Flush and Sync remain distinct decisions, and FlushForPrune is not implemented.
**Plans**: 3 plans

Plans:
- [x] 140-01-PLAN.md — Types, cache-size classification, and decide_flush skeleton
- [ ] 140-02-PLAN.md — Knots Flush/Sync/refusal decision matrix
- [ ] 140-03-PLAN.md — RecoveryDecision sketch, crate exports, leftover-persist guard

### Phase 141: Durable Fjall Coins Adapter
**Goal**: Spendable UTXOs live as per-outpoint Fjall records with best-block and interrupted-flush markers; snapshot blobs are no longer live coin truth.
**Depends on**: Phase 140
**Requirements**: COIN-01, CSOBS-03
**Success Criteria** (what must be TRUE):
  1. The node stores and loads coins as per-outpoint durable records with coins best-block and two-element `head_blocks` markers.
  2. A leftover snapshot blob is non-authoritative after the explicit one-way migration; reopen does not treat it as UTXO truth.
  3. A coins disk-read error fails closed as a typed storage or recovery error, never as spent or missing.
  4. Schema mismatch fails closed, and LevelDB or rust-bitcoin are not introduced.
**Plans**: TBD

### Phase 142: Manager Flush Lifecycle and Restart
**Goal**: One manager owns coins init, flush points, interrupted-flush recovery, and restart from durable coins best-block.
**Depends on**: Phase 141
**Requirements**: MGR-01, MGR-02, FLUSH-02
**Success Criteria** (what must be TRUE):
  1. One manager initializes the coins database, health-check, and cache, and reports CanFlush-style readiness only after that sequence for the single active chainstate.
  2. After same-datadir restart, tip and UTXO view come from durable coins best-block, not a leftover snapshot blob.
  3. The node flushes IfNeeded after connect/reorg, Periodic on injected ticks, and Always on shutdown, writing block, undo, and index before coins.
  4. A mid-flush crash is recovered by interrupted-flush replay from stored undo and block bodies, or fails closed without inventing a consistent tip.
  5. Progress credit and `persist_progress` no longer rewrite the full UTXO snapshot as live truth.
**Plans**: TBD

### Phase 143: Honest Stored-Block Availability
**Goal**: The node serves or reports a stored block only when the payload bytes are present and refuses cleanly when they are not.
**Depends on**: Phase 142
**Requirements**: HAVL-01, HAVL-02, HAVL-03
**Success Criteria** (what must be TRUE):
  1. Inventory, serve, and RPC classify a block as Available only after a payload-byte probe succeeds.
  2. When the payload is absent, the node refuses cleanly as Unavailable and does not emit Pruned unless prune mode actually deleted files.
  3. Operator-visible facts distinguish `payload_present`, `index_known`, and `validated_on_active_chain`; coins tip or header index alone cannot authorize a serve.
  4. Missing-payload refuse does not change public-default serving or claim archive-node behavior.
**Plans**: TBD

### Phase 144: Operator Flush and Availability Evidence
**Goal**: Operators can see flush, recovery, cache-size, and have-bytes versus do-not through sanitized surfaces.
**Depends on**: Phase 143
**Requirements**: CSOBS-01, CSOBS-02
**Success Criteria** (what must be TRUE):
  1. Status, RPC, CLI, dashboard, metrics, logs, and support expose flush, recovery, and have-bytes versus do-not using sanitized low-cardinality fields.
  2. Operator evidence reports cache-size state (OK / LARGE / CRITICAL) and last flush reason.
  3. Coins best-block, interrupted-flush or replay outcome, and per-request availability labels are visible without peer identifiers or raw coin dumps.
**Plans**: TBD
**UI hint**: yes

### Phase 145: Parity Roots and No-Claim Guardrails
**Goal**: Parity evidence is auditable and the v2.3 claim cannot be read as prune, assumeutxo, archive, or production readiness.
**Depends on**: Phase 144
**Requirements**: CSVFY-01, CSVFY-02
**Success Criteria** (what must be TRUE):
  1. Parity roots cite pinned Knots coins, flush, manager, and serve-path anchors, or document intentional differences.
  2. Deterministic no-claim guardrails keep prune/archive modes, assumeutxo, compact filters, public defaults, and production readiness out of the v2.3 claim.
  3. Default `bash scripts/verify.sh` stays deterministic, and historical `.planning/phases/` directories remain tracked.
**Plans**: TBD

## Requirement Coverage

| Requirement | Phase | Status |
| --- | --- | --- |
| CACHE-01 | Phase 139 | Pending |
| FLUSH-01 | Phase 140 | Pending |
| MGR-03 | Phase 140 | Pending |
| COIN-01 | Phase 141 | Pending |
| CSOBS-03 | Phase 141 | Pending |
| MGR-01 | Phase 142 | Pending |
| MGR-02 | Phase 142 | Pending |
| FLUSH-02 | Phase 142 | Pending |
| HAVL-01 | Phase 143 | Pending |
| HAVL-02 | Phase 143 | Pending |
| HAVL-03 | Phase 143 | Pending |
| CSOBS-01 | Phase 144 | Pending |
| CSOBS-02 | Phase 144 | Pending |
| CSVFY-01 | Phase 145 | Pending |
| CSVFY-02 | Phase 145 | Pending |

**Coverage:** 15/15 v2.3 requirements mapped. No orphans. No duplicates.

## Research Flags for Planning

These are planning inputs, not extra phases:

- **Phase 141:** Exact per-coin key schema, dedicated `coins` keyspace, 64 MiB dirty-batch accounting, `head_blocks` encoding, compact codec home, undo-as-own-record versus leftover snapshot DTO, and schema-bump versus multi-namespace migration.
- **Phase 142:** Allowed crash-loss window (periodic plus clean shutdown versus every-connect), Knots cache-byte defaults versus first-party accounting, and interrupted-flush replay versus fail-closed when bodies or undo are missing.
- **Phase 143:** Shape of `durable_availability` and the reserved `Pruned` label so help text cannot be read as prune-mode.

## Progress

**Execution Order:**
Phases execute in numeric order: 139 → 140 → 141 → 142 → 143 → 144 → 145

| Phase | Plans Complete | Status | Completed |
| --- | --- | --- | --- |
| 139. Coins-View, Cache Contract, and Engine Apply | 4/4 | Complete    | 2026-08-31 |
| 140. Pure Flush Policy and Typed Decisions | 1/3 | In Progress|  |
| 141. Durable Fjall Coins Adapter | 0/TBD | Not started | - |
| 142. Manager Flush Lifecycle and Restart | 0/TBD | Not started | - |
| 143. Honest Stored-Block Availability | 0/TBD | Not started | - |
| 144. Operator Flush and Availability Evidence | 0/TBD | Not started | - |
| 145. Parity Roots and No-Claim Guardrails | 0/TBD | Not started | - |

## Next Step

Run `/gsd-execute-phase 140` to implement the pure flush-policy and typed-decision plans.

---
*Roadmap created: 2026-08-29 for milestone v2.3. Phase numbering continues from v2.2 Phase 138.*
