# Roadmap: Open Bitcoin

## Milestones

- ✅ **v1.0 Headless Parity** - 22 phase entries, including inserted 3.x and 7.x closure phases (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** - Phases 13 through 34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Full Mainnet Network Syncing** - Phases 35 through 41 (shipped 2026-05-23). Archive: [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Public Mainnet Sync Proof and Node Hardening** - Phases 42 through 53 (shipped 2026-06-02). Archive: [v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md)
- ✅ **v1.4 Mainnet IBD Convergence and Peer Compatibility** - Phases 54 through 59 (shipped 2026-06-05). Archive: [v1.4-ROADMAP.md](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Unattended Mainnet Node Operation Readiness** - Phases 60 through 67 (shipped 2026-06-10). Archive: [v1.5-ROADMAP.md](milestones/v1.5-ROADMAP.md)
- 🚧 **v1.6 Mainnet Full-Sync Completion** - Phases 68 through 74 (active). Requirements: 26

## Current Focus

v1.6 Mainnet Full-Sync Completion turns the shipped v1.5 unattended
operator-review loop into a truthful explicit opt-in `open-bitcoind`
sync-to-tip claim. The milestone focuses on full active-chain validation,
durable chainstate and UTXO/undo persistence, tip tracking, stay-current
operation, reorg and peer recovery, bounded resources, coherent operator
evidence, opt-in UAT, and deterministic release-boundary checks.

The milestone does not include inbound serving, address relay, block serving,
transaction relay, compact block relay, production-funds wallet claims,
migration apply mode, signed packaging, Windows service support, GUI work,
hosted dashboards, public-network checks in `bash scripts/verify.sh`, or broad
production-node claims.

Raw v1.0, v1.3, v1.4, and v1.5 phase histories remain in
[.planning/phases/](phases/) for parity and UAT traceability; do not move or
delete those phase directories.

## Phases

- [x] **Phase 68: Full Active-Chain Validation and Durable Persistence** - Sync to the best-known validated peer tip only through consensus-validated, durably connected active-chain progress.
- [x] **Phase 69: Tip Tracking and Stay-Current Operation** - Define best-known tip evidence and keep the daemon current after initial catch-up.
- [x] **Phase 70: Reorg, Peer Rotation, and No-Progress Recovery** - Make branch competition, reorgs, peer failures, and no-progress causes deterministic and operator-visible.
- [x] **Phase 71: Resource Bounds and Durable Restart/Resume** - Prove long-sync resource bounds and safe recovery across shutdown, interruption, and storage-pressure cases.
- [ ] **Phase 72: Operator Observability and Support Evidence** - Align every operator surface around one full-sync truth contract and redacted support evidence.
- [ ] **Phase 73: Opt-In UAT and Deterministic Verification** - Keep default verification hermetic while adding deterministic coverage and repo-local public-mainnet UAT commands.
- [ ] **Phase 74: Release Boundaries, Parity, and Documentation** - Close v1.6 with scoped parity roots, release-readiness docs, operator guidance, and claim-boundary checks.

## Phase Details

### Phase 68: Full Active-Chain Validation and Durable Persistence

**Goal**: Operators can run explicit opt-in `open-bitcoind` mainnet sync until the active chain reaches the best-known validated peer tip, with progress credited only after consensus validation and durable connection.
**Depends on**: Phase 67
**Requirements**: SYNC-01, SYNC-02, SYNC-03, SYNC-04
**Plans**: 3/3 complete

**Success Criteria**:
1. Operator can run explicit opt-in mainnet sync until the active chain reaches the best-known validated peer tip or returns a typed blocker.
2. Status evidence distinguishes header height, downloaded block height, connected block height, validated active-chain height, cumulative work, and tip freshness.
3. Same-datadir restart recovers durable active-chain, UTXO, undo, block-index, and runtime metadata needed to continue validation safely.
4. Block progress is credited only after consensus validation and durable active-chain connection, never after headers-only or downloaded-only progress.

### Phase 69: Tip Tracking and Stay-Current Operation

**Goal**: Operators can understand best-known tip evidence and keep `open-bitcoind` caught up after initial sync.
**Depends on**: Phase 68
**Requirements**: TIP-01, TIP-02, TIP-03
**Plans**: 5/5 complete

**Success Criteria**:
1. Operator can inspect best-known mainnet tip source, height, hash, work, timestamp, freshness, and peer agreement evidence.
2. Status surfaces distinguish initial catch-up, current-at-best-known-tip, stale-tip, recovering, and no-progress states without renderer-specific interpretation.
3. After catch-up, the daemon detects, validates, connects, and reports new headers and blocks as stay-current progress.
4. Tip freshness and peer agreement evidence remain coherent across restart and peer rotation.

### Phase 70: Reorg, Peer Rotation, and No-Progress Recovery

**Goal**: Operators can survive branch competition, reorgs, stale in-flight work, and peer failures with deterministic outcomes and actionable diagnosis.
**Depends on**: Phase 69
**Requirements**: REC-01, REC-02, REC-03, REC-04
**Plans**: 6/6 complete

Plans:
- [x] 70-01-PLAN.md - Reorg and reconcile status contract
- [x] 70-02-PLAN.md - Branch/reorg runtime and storage blockers
- [x] 70-03-PLAN.md - Peer attribution, stale in-flight release, and rotation
- [x] 70-04-PLAN.md - No-progress status contract
- [x] 70-05-PLAN.md - Shared no-progress diagnosis and rendering
- [x] 70-06-PLAN.md - Operator docs, README relevance, and deterministic verification closeout

**Success Criteria**:
1. Competing header branches resolve through cumulative-work selection with deterministic active-chain outcomes.
2. Reorg handling durably disconnects and reconnects blocks with bounded undo evidence.
3. Stale, slow, incompatible, malformed, invalid, disconnecting, and `notfound` peers receive typed attribution, retry/backoff, and rotation behavior.
4. Operator-facing status explains whether no progress means behind, stalled, at tip, recovering, or clearing stale in-flight work, with next actions.

### Phase 71: Resource Bounds and Durable Restart/Resume

**Goal**: Operators can run long full-sync attempts within documented resource bounds and recover safely after interruptions or storage pressure.
**Depends on**: Phase 70
**Requirements**: RES-01, RES-02, RES-03, RES-04
**Plans**: Pending

**Success Criteria**:
1. Bounds are documented and tested for peers, in-flight blocks, queues, caches, storage writes, logs, metrics, and support evidence.
2. Same-datadir resume is safe after clean shutdown, unclean shutdown, mid-download interruption, mid-connect interruption, and stale in-flight work.
3. Recovery guidance distinguishes schema mismatch, corruption markers, lock contention, low disk, and storage pressure without hidden data mutation.
4. Deterministic synthetic long-chain tests exercise resource bounds without public-network access.

### Phase 72: Operator Observability and Support Evidence

**Goal**: Operators can inspect and share one coherent full-sync truth contract across CLI, dashboard, RPC, metrics, logs, live-smoke reports, and support bundles.
**Depends on**: Phase 71
**Requirements**: OBS-01, OBS-02, OBS-03, OBS-04
**Plans**: 4 plans

Plans:
- [x] 72-01-PLAN.md - Align CLI, dashboard, and RPC status surfaces
- [x] 72-02-PLAN.md - Add support evidence and typed verdicts
- [x] 72-03-PLAN.md - Extend metrics, logs, and live-smoke projections
- [x] 72-04-PLAN.md - Document guidance and wire deterministic verification

**Success Criteria**:
1. CLI status, dashboard, RPC, metrics, structured logs, live-smoke reports, and support bundles share one full-sync truth contract.
2. Redacted support evidence includes initial and final tip, connected height/hash/work, restart/resume checkpoints, stay-current window, peer contribution, no-progress or reorg events, resource pressure, and final verdict.
3. Cross-surface comparison confirms agreement on connected chain progress, tip freshness, recovery category, peer health, and next action.
4. Operator guidance explains whether evidence proves sync-to-tip, stay-current behavior, diagnosed blocker, or deferred production-node scope.

### Phase 73: Opt-In UAT and Deterministic Verification

**Goal**: Contributors keep default verification deterministic while operators get repo-local opt-in commands for public-mainnet full-sync review.
**Depends on**: Phase 72
**Requirements**: VER-01, VER-02, VER-03, VER-04
**Plans**: 4 plans

Plans:
- [x] 73-01-PLAN.md - Map VER-02 deterministic coverage to existing hermetic anchors
- [x] 73-02-PLAN.md - Add the central Phase 73 opt-in public-mainnet UAT matrix
- [ ] 73-03-PLAN.md - Wire the Phase 73 deterministic checker into default verification
- [ ] 73-04-PLAN.md - Close parity, breadcrumb, and evidence auditability

**Success Criteria**:
1. `bash scripts/verify.sh` runs without internet access, public peers, real service managers, long-running sync, or current-tip timing.
2. Deterministic tests cover durable UTXO/undo writes, block connect/disconnect/reorg across restart, best-chain header selection, peer response failures, crash recovery, duplicate connect prevention, and resource bounds.
3. Operator docs provide copy-pasteable repo-local Cargo and Bazel commands for opt-in public-mainnet full-sync, stay-current, restart/resume, and support-bundle UAT.
4. Parity breadcrumbs, fixtures, compatibility harness reports, and deterministic checkers cover every new v1.6 source, test, and operator-evidence surface.

### Phase 74: Release Boundaries, Parity, and Documentation

**Goal**: Reviewers can audit that v1.6 claims only explicit opt-in full-sync completion and preserves all deferred scope boundaries.
**Depends on**: Phase 73
**Requirements**: REL-01, REL-02, REL-03
**Plans**: Pending

**Success Criteria**:
1. v1.6 parity roots, threat model, release-readiness matrix, README, and operator docs describe only the explicit opt-in full-sync completion claim.
2. Deterministic release-boundary checks fail if docs or status surfaces imply inbound serving, relay, production-wallet, migration-apply, packaging, GUI, hosted-dashboard, or broad production-node claims.
3. Operator docs explain shipped sync-to-tip evidence, opt-in UAT commands, support evidence locations, failure interpretation, and deferred scope.
4. Final milestone traceability shows all 26 v1.6 requirements mapped, verified, and ready for archive.

## Progress

**Execution Order:** Phase 68 -> 69 -> 70 -> 71 -> 72 -> 73 -> 74

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 68. Full Active-Chain Validation and Durable Persistence | 3/3 | Complete    | 2026-06-11 |
| 69. Tip Tracking and Stay-Current Operation | 5/5 | Complete    | 2026-06-12 |
| 70. Reorg, Peer Rotation, and No-Progress Recovery | 6/6 | Complete | 2026-06-12 |
| 71. Resource Bounds and Durable Restart/Resume | 4/4 | Complete    | 2026-06-13 |
| 72. Operator Observability and Support Evidence | 4/4 | Complete   | 2026-06-13 |
| 73. Opt-In UAT and Deterministic Verification | 2/4 | In Progress|  |
| 74. Release Boundaries, Parity, and Documentation | 0/0 | Pending | — |

## Completed Milestone Summaries

<details>
<summary>✅ v1.5 Unattended Mainnet Node Operation Readiness (Phases 60-67) - SHIPPED 2026-06-10</summary>

- [x] Phase 60: Unattended Sync Loop Control (1/1 plans) - completed 2026-06-06
- [x] Phase 61: Resource Bounds and Recovery Taxonomy (6/6 plans) - completed 2026-06-06
- [x] Phase 62: Long-Run Sync Truth Surfaces (4/4 plans) - completed 2026-06-07
- [x] Phase 63: Service Supervision Lifecycle (4/4 plans) - completed 2026-06-07
- [x] Phase 64: Service Restart and Same-Datadir Resume Evidence (3/3 plans) - completed 2026-06-07
- [x] Phase 65: Support Bundle and Operator Review Docs (2/2 plans) - completed 2026-06-08
- [x] Phase 66: Compatibility Harness Operator Wrapper (1/1 plans) - completed 2026-06-08
- [x] Phase 67: Release Boundaries and Deterministic Verification (1/1 plans) - completed 2026-06-09

Detailed phase requirements, success criteria, and plan links are archived in
[milestones/v1.5-ROADMAP.md](milestones/v1.5-ROADMAP.md). Raw v1.5 phase
execution artifacts remain in [.planning/phases/](phases/) for parity and UAT
traceability.

</details>

<details>
<summary>✅ v1.4 Mainnet IBD Convergence and Peer Compatibility (Phases 54-59) - SHIPPED 2026-06-05</summary>

- [x] Phase 54: Peer Compatibility Baseline and Diagnostic Harness (1/1 plans) - completed 2026-06-02
- [x] Phase 55: Outbound Handshake Compatibility Fixes (1/1 plans) - completed 2026-06-03
- [x] Phase 56: Header IBD Convergence (1/1 plans) - completed 2026-06-03
- [x] Phase 57: Block Download and Connect Progress (4/4 plans) - completed 2026-06-04
- [x] Phase 58: Same-Datadir Restart and Resume Evidence (3/3 plans) - completed 2026-06-05
- [x] Phase 59: Operator Evidence, Threat Model, and Release Boundaries (5/5 plans) - completed 2026-06-05

Detailed phase requirements, success criteria, and plan links are archived in
[milestones/v1.4-ROADMAP.md](milestones/v1.4-ROADMAP.md). Raw v1.4 phase
execution artifacts remain in [.planning/phases/](phases/) for parity and UAT
traceability.

</details>

## Milestone History

| Milestone | Phases | Plans | Status | Shipped |
| --- | ---: | ---: | --- | --- |
| v1.0 Headless Parity | 22 | 80 | Shipped | 2026-04-26 |
| v1.1 Operator Runtime and Real-Network Sync | 22 | 69 | Shipped | 2026-04-30 |
| v1.2 Full Mainnet Network Syncing | 7 | 13 | Shipped | 2026-05-23 |
| v1.3 Public Mainnet Sync Proof and Node Hardening | 12 | 13 | Shipped | 2026-06-02 |
| v1.4 Mainnet IBD Convergence and Peer Compatibility | 6 | 15 | Shipped | 2026-06-05 |
| v1.5 Unattended Mainnet Node Operation Readiness | 8 | 22 | Shipped | 2026-06-10 |
| v1.6 Mainnet Full-Sync Completion | 7 | 14 | Active | — |

## Next Step

Begin Phase 72 planning:

```bash
/gsd-discuss-phase 72
```
