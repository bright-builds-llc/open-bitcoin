# Roadmap: Open Bitcoin

## Current Status

v2.5 Prune-Aware Compact-Filter Serving (BIP157/158) is in progress: Phases 154–155 completed eight plans and five requirements after full native verification on 2026-10-04. Two of nine phases are complete; 17 requirements remain pending, and Phase 156 is ready for context. BASIC construction, recoverable immutable storage and pre-prune startup protection are implemented; activation and serving remain planned scope, and v2.5 is not a shipped capability. v2.4 remains archived with three nonblocking advisories, and all historical phase directories remain tracked for verifier evidence.

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
- ✅ **v2.4 Prune-Mode Product Behavior** — Phases 146–153 (shipped 2026-10-03). Archive: [v2.4-ROADMAP.md](milestones/v2.4-ROADMAP.md)
- 📋 **v2.5 Prune-Aware Compact-Filter Serving (BIP157/158)** — Phases 154–162 (planning; started 2026-10-03).

## Active Milestone: v2.5 Prune-Aware Compact-Filter Serving (BIP157/158)

Let lightweight clients fetch Knots-compatible BASIC compact filters from an explicitly enabled node, with durable indexing that stays correct across pruning, restart and reorgs on the existing single active Fjall chainstate. Exact commitments precede recoverable storage and protection; concrete manual/automatic prune safety precedes activation and catch-up; branch correctness precedes authenticated RPC and bounded peer serving. Existing operator consumers and a continuous validated-chain proof close the scope.

Material guidance: repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, and architecture, verification and local-guidance standards. Research inputs: [summary](research/SUMMARY.md), [stack](research/STACK.md), [features](research/FEATURES.md), [architecture](research/ARCHITECTURE.md) and [pitfalls](research/PITFALLS.md). Extend existing first-party crates, pure-core/shell boundaries and the serialized chainstate/prune authority; add no production crate or Rust Bitcoin dependency.

## Phases

Integer phases continue from archived Phase 153; decimal phases remain available for urgent insertions. Fine granularity preserves nine coherent delivery boundaries. All requirements are Pending until implementation and lifecycle-valid verification are committed.

- [x] **Phase 154: BASIC Generation and Commitment Parity** — Produce exact BASIC bytes and ancestry-dependent commitments from validated script history. (completed 2026-10-04)
- [x] **Phase 155: Recoverable Index and Pre-Prune Startup Protection** — Recover concrete Fjall records and validate protection before interrupted pruning can resume. (completed 2026-10-04)
- [ ] **Phase 156: Index-Owned Manual and Automatic Prune Coordination** — Make required input protection enforceable through the actual deletion and lock owners.
- [ ] **Phase 157: Safe Activation and Scheduled Index Catch-Up** — Enable indexing only with retained inputs, then advance through real startup and ordinary maintenance.
- [ ] **Phase 158: Validated Reorg and Retained Branch Identity** — Replace active commitments while retaining indexed displaced blocks and refusing lost reorg inputs.
- [ ] **Phase 159: Authenticated BASIC Filter and Index RPCs** — Expose pinned lookup results and initial-synchronization semantics through the shared durable authority.
- [ ] **Phase 160: Explicit Bounded Peer Filter Serving** — Serve all three branch-aware BIP157 families on real inbound and outbound paths with achieved-write evidence.
- [ ] **Phase 161: Shared Operator Index and Retention Evidence** — Present configured capability, progress, failures and separate retained-index growth through existing consumers.
- [ ] **Phase 162: Real Prune Retention and Integrated Parity Proof** — Prove retained client responses across continuous validated-chain pruning, recovery, forks and both transports.

## Phase Details

### Phase 154: BASIC Generation and Commitment Parity

**Goal**: Clients can obtain pinned BASIC/type 0 filter bytes, hashes and headers derived from validated blocks and historical spent scripts.
**Depends on**: Phase 153 (archived v2.4 foundation)
**Requirements**: CFIL-01, CFIL-02
**Success Criteria** (what must be TRUE):

1. A contributor can compare generated BASIC bytes against independent pinned Knots vectors for genesis, empty filters, duplicate raw scripts, mapped collisions, OP_RETURN exclusions and historical spent-output scripts. (CFIL-01)
1. A validated spend fixture, including same-block-created-and-spent outputs, contributes the required historical scripts; missing non-genesis spent-script evidence never becomes an invented empty input set. (CFIL-01)
1. A client obtains the exact filter hash and header with a zero genesis predecessor, correct raw-byte ordering and replacement ancestry commitments. (CFIL-02)

**Plans**: 4/4 complete; full native verification passed

- `154-01-PLAN.md` — pure BASIC codec, crypto and commitments
- `154-02-PLAN.md` — validated historical inputs and actual spend proof
- `154-03-PLAN.md` — independent parity vectors, documentation and native wiring
- `154-04-PLAN.md` — explicit Bun test-file execution in the verifier

**UI hint**: yes

### Phase 155: Recoverable Index and Pre-Prune Startup Protection

**Goal**: Operators can reopen a durable index without trusting progress beyond recovered chainstate or losing required history to startup prune recovery.
**Depends on**: Phase 154
**Requirements**: CFIX-02, CFIX-04, CFPR-03
**Success Criteria** (what must be TRUE):

1. An operator reopens real Fjall after successful or interrupted index writes and recovers integrity-checked records plus safe progress, or receives a fail-closed diagnostic without erasing a valid prefix or advancing a phantom cursor. (CFIX-02)
1. When filter writes precede a successful coins/chain-metadata flush, restart retains valid immutable records but reconciles active projection and resume cursor to the recovered branch and durable chainstate checkpoint. (CFIX-04)
1. A production durable-runtime reopen with a live interrupted prune intent validates index-owned protection before initialize can call resume_prune_intent; required bodies and undo survive, or startup refuses before unsafe deletion. (CFPR-03)
1. Faults in record, checkpoint or protection persistence leave conservative protection and explicit failure evidence; stored filter records alone never authorize a cursor ahead of recoverable chainstate. (CFIX-02, CFIX-04, CFPR-03)

**Plans**: 4/4 complete; full native verification passed

Plans:
- [x] `155-01-PLAN.md` — Bounded BASIC validation and pure fenced recovery/protection contracts
- [x] `155-02-PLAN.md` — Additive immutable Fjall records, atomic publication and reopen fault proof
- [x] `155-03-PLAN.md` — Mandatory initialize pre-prune guard and production durable-runtime reopen
- [x] `155-04-PLAN.md` — Cross-boundary recovery matrix, parity/docs and native guard wiring

### Phase 156: Index-Owned Manual and Automatic Prune Coordination

**Goal**: An active index keeps required block and undo inputs protected until a safe durable checkpoint permits manual or ordinary automatic deletion.
**Depends on**: Phase 155
**Requirements**: CFPR-01
**Success Criteria** (what must be TRUE):

1. Manual pruning and ordinary automatic retention both reload and enforce index protection at actual application; a stalled or failed index retains every required body/undo input even when the soft target cannot be reached. (CFPR-01)
1. Authenticated operator lock set/clear cannot replace, remove or weaken reserved index-owned protection; authoritative CRUD and deletion paths enforce ownership. (CFPR-01)
1. Protection advances only after the corresponding filter records and chainstate-fenced durable checkpoint are recoverable; injected checkpoint/protection faults retain extra history rather than release unsafe inputs. (CFPR-01)
1. An explicit disable transition stops catch-up and invalidates in-flight work before releasing owned protection; re-enable restores protection before work or deletion can proceed. (CFPR-01)

**Plans**: TBD

### Phase 157: Safe Activation and Scheduled Index Catch-Up

**Goal**: Operators can explicitly activate BASIC indexing and observe bounded, ordered catch-up and ongoing validated-connect progress without enabling networking.
**Depends on**: Phase 156
**Requirements**: CFAC-01, CFAC-02, CFIX-01
**Success Criteria** (what must be TRUE):

1. An operator selects BASIC indexing through bare blockfilterindex, 1 or basic; omission/0 stays disabled, repeated/mixed forms have tested documented semantics, and explicit activation chooses durable storage without opening listeners or enabling networking. (CFAC-01)
1. Fresh activation or resumption after actual historical body/non-genesis undo loss refuses without source-history mutation, prefix deletion, skipped heights, current-coins reconstruction or implicit repair/download. (CFAC-02)
1. Actual daemon startup and a bounded scheduled maintenance caller advance retained-history catch-up without another peer message, yield between turns and never label incomplete initial work complete. (CFIX-01)
1. Ordinary validated connects hand complete historical script facts to the same ordered index owner; accepted-state changes and persistence failures cannot be missed by success-only callbacks or publish unsafe durable progress. (CFIX-01)

**Plans**: TBD

### Phase 158: Validated Reorg and Retained Branch Identity

**Goal**: Clients obtain branch-correct replacement filters after validated reorgs while previously indexed displaced blocks remain addressable by hash.
**Depends on**: Phase 157
**Requirements**: CFIX-03
**Success Criteria** (what must be TRUE):

1. A continuous validated local fork rewinds the active index to its common ancestor and derives replacement filters and headers from that ancestor, including equal-height replacements. (CFIX-03)
1. After reorg and real Fjall reopen, previously indexed displaced blocks remain retrievable by block hash while the active height projection identifies the replacement branch. (CFIX-03)
1. A reorg requiring missing retained body/undo inputs refuses explicitly and preserves conservative progress/protection; it never substitutes current coins, a different branch or hidden redownload. (CFIX-03)

**Plans**: TBD

### Phase 159: Authenticated BASIC Filter and Index RPCs

**Goal**: Authenticated clients can query BASIC filters and index summaries with pinned Knots result shapes, selection and error ordering.
**Depends on**: Phase 158
**Requirements**: CFRP-01, CFRP-02
**Success Criteria** (what must be TRUE):

1. An authenticated getblockfilter request defaults to BASIC and returns exact filter/header hex for indexed active, retained stale and pruned blocks through the shared durable authority, including available records during catch-up. (CFRP-01)
1. Disabled index, unknown type/block, never-connected block, indexing absence and corruption produce the pinned BASIC error ordering and codes; backend faults never become empty successful filters. (CFRP-01)
1. Authenticated getindexinfo returns basic block filter index with only the pinned synced and best_block_height fields; exact-name selection, unknown selection and disabled absence return the expected objects. (CFRP-02)
1. getindexinfo synced retains Knots' initial-synchronization meaning when later tip movement creates lag; it is neither a configured-capability flag nor an instantaneous tip-equality claim. (CFRP-02)

**Plans**: TBD
**UI hint**: yes

### Phase 160: Explicit Bounded Peer Filter Serving

**Goal**: Explicitly permitted peers receive complete indexed BASIC ranges and checkpoints through bounded runtime transport with Knots validation behavior.
**Depends on**: Phase 159
**Requirements**: CFNET-01, CFNET-02, CFNET-03, CFNET-04, CFNET-05, CFNET-06
**Success Criteria** (what must be TRUE):

1. General BASIC serving and explicit per-peer blockfilters permission require BASIC indexing; observed version messages advertise NODE_COMPACT_FILTERS according to configured/per-peer capability before initial catch-up completes, coexist truthfully with NODE_NETWORK_LIMITED and do not activate public listeners/defaults. (CFNET-01)
1. Peers receive ordered cfilter bytes for complete inclusive ranges up to 1,000 blocks and cfheaders with the correct preceding header plus ordered hashes for ranges up to 2,000 blocks; stop-hash ancestry selects active or permitted stale branches, including during catch-up. (CFNET-02, CFNET-03)
1. Peers receive cfcheckpt headers at every positive 1,000-block interval through the permitted stop block, excluding genesis and following that branch's ancestry; checkpoint resource accounting does not invent a 1,000/2,000-block range cap. (CFNET-04, CFNET-06)
1. Unsupported types, invalid/disallowed stop hashes, reversed or oversized ranges produce pinned disconnect/validation outcomes; a valid request crossing an absent record yields no fabricated empty filter or shortened successful response. (CFNET-05)
1. Real inbound and outbound runtime callers read persisted records under explicit work/byte/queue bounds, write outside the authority lock and earn completion only from achieved transport writes; queue pressure and partial-write failure abort the unwritten suffix without crediting a full-range success. (CFNET-06)

**Plans**: TBD
**UI hint**: yes

### Phase 161: Shared Operator Index and Retention Evidence

**Goal**: Operators can distinguish index capability, initial synchronization, current lag, safe durable progress and failures consistently across existing status, CLI, dashboard and support evidence.
**Depends on**: Phase 160
**Requirements**: CFOP-01
**Success Criteria** (what must be TRUE):

1. Existing status RPC, repo-local CLI and terminal dashboard show consistent configured capability, initial catch-up, current lag and safe durable progress from one authoritative projection. (CFOP-01)
1. Missing history, backend/index failure and retained filter availability have distinct actionable categories; redacted support evidence agrees with live operator facts without exposing raw scripts, paths or peer identifiers. (CFOP-01)
1. Operators see retained filter growth separately from the soft block/undo prune target, including protection-induced retention; useful filters never imply old block bodies or wallet eligibility have been restored. (CFOP-01)

**Plans**: TBD
**UI hint**: yes

### Phase 162: Real Prune Retention and Integrated Parity Proof

**Goal**: Contributors can reproduce retained filter service after real prune/reopen and audit the complete scoped lifecycle without broadening milestone claims.
**Depends on**: Phase 161
**Requirements**: CFPR-02, CFGR-01, CFGR-02
**Success Criteria** (what must be TRUE):

1. A client retrieves previously indexed filter bytes/headers through authenticated RPC and filters, headers and checkpoints through all three peer families after actual paired Fjall body/undo deletion and production datadir reopen; ordinary block serving and wallet eligibility still obey the existing pruned-body restrictions. (CFPR-02)
1. A hermetic continuous consensus-validated chain with real historical spends runs through actual startup, receive-independent scheduled catch-up, ordinary connect, shallow reorg/stale lookup, manual and automatic pruning, reopen, authenticated RPC and both inbound/outbound transport callers. Sparse codec-valid and MemoryCoinsView-only fixtures cannot satisfy this proof. (CFGR-02)
1. Concrete durable-coins/Fjall fault and refusal scenarios cover fresh/lagged missing history, pending prune intent, cursor ahead of recovered chainstate, index/protection/checkpoint write failure, corrupted records and partial transport writes without fabricated progress or successful receipts. (CFGR-02)
1. A contributor audits BASIC generation/protocol/RPC behavior through pinned source breadcrumbs, parity roots and deterministic native claim checks; Fjall layout, BASIC-only V0 exclusion, earlier undo refusal and local resource-policy differences are explicit. (CFGR-01)
1. Copy-pasteable repo-local Cargo/Bazel UAT and the native verification contract reproduce the scoped evidence while preserving BIP37/V0, assumeutxo, archive-scale, public-default/network-CI, implicit repair and production/funds exclusions. (CFGR-01, CFGR-02)

**Plans**: TBD
**UI hint**: yes

## Dependency and Evidence Gates

- Phase 155 must prove the actual pre-resume startup path with real Fjall/durable-runtime fixtures. Installing protection after initialize is insufficient. Reserved ownership is established at this foundation seam and enforced across every concrete mutation/deletion path in Phase 156.
- Phase 157 may expose the BASIC activation forms only after Phases 155–156 establish recovery, chainstate-fenced resume progress and non-overridable protection. Fresh or lagged missing-history activation always refuses without mutation or acquisition.
- Build continuous validated spend/fork fixtures from Phase 154 onward and use them for ordinary lifecycle evidence in Phases 157–158. Final integration expands these fixtures; it must not discover their feasibility only at closeout.
- Phase 160 advertises configured BASIC capability independently of initial synchronization, serves complete indexed ranges during catch-up and uses branch-aware persisted lookup. A transport enqueue or prepared plan is not an achieved write receipt.
- Phase 162 owns CFPR-02's complete post-prune client proof because RPC and all peer families must already exist. Earlier phases prove their own safety behavior; final proof traverses their ordinary production callers with concrete durable coins and Fjall.
- Preserve the v2.4 permitted finish-or-Repair refusal, generic-sink advisory and deletion-summary undercount limits. Those counters/defaults cannot authorize index progress; unrelated advisory cleanup is excluded.

UI hints follow the roadmap keyword contract, which also matches protocol header terminology. Only Phase 161 changes an existing terminal dashboard; no GUI or new frontend is planned.

## Research Focus for Planning

| Phase | Decision to settle before implementation |
| -- | -- |
| 155 | Additive schema/reopen compatibility; recoverable chain checkpoint; immutable extra records versus active projection; atomic filter batches and early protection before prune resume. |
| 156 | Reserved CRUD ownership; apply-time manual/automatic checks; cached measurement invalidation; safe disable and protection-release ordering. |
| 157 | Mixed/repeated option semantics; validated undo access; accepted-state/persistence-error seams; measured bounded offline scheduler budgets and required prefix/suffix boundary. |
| 158 | Common-ancestor rewind and retained stale identity under real faults; explicit deep-reorg missing-input refusal. |
| 159–160 | BASIC-only known V0 outcomes; exact RPC error ordering; implicit/all peer permissions; stale eligibility; checkpoint byte/work/queue budgets and transport cleanup. |

## Progress

Execution order: 154 → 155 → 156 → 157 → 158 → 159 → 160 → 161 → 162.

| Phase | Plans Complete | Status | Completed |
| -- | -- | -- | -- |
| 154. BASIC Generation and Commitment Parity | 4/4 | Complete | 2026-10-04 |
| 155. Recoverable Index and Pre-Prune Startup Protection | 4/4 | Complete | 2026-10-04 |
| 156. Index-Owned Manual and Automatic Prune Coordination | 0/TBD | Not started | - |
| 157. Safe Activation and Scheduled Index Catch-Up | 0/TBD | Not started | - |
| 158. Validated Reorg and Retained Branch Identity | 0/TBD | Not started | - |
| 159. Authenticated BASIC Filter and Index RPCs | 0/TBD | Not started | - |
| 160. Explicit Bounded Peer Filter Serving | 0/TBD | Not started | - |
| 161. Shared Operator Index and Retention Evidence | 0/TBD | Not started | - |
| 162. Real Prune Retention and Integrated Parity Proof | 0/TBD | Not started | - |

## Coverage

22/22 current v2.5 requirements map to exactly one owning phase; no orphans or duplicate owners. Requirement lists in Phase Details are authoritative and match [REQUIREMENTS.md](REQUIREMENTS.md#traceability). All nine phases have 2–5 observable success criteria; active milestone completion is 2/9 verified phases and 8/8 created plans. CFIL-01/02, CFIX-02/04 and CFPR-03 are Complete; the remaining 17 requirements are Pending.
