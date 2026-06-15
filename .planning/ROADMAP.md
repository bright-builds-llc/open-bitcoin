# Roadmap: Open Bitcoin

## Milestones

- ✅ **v1.0 Headless Parity** - 22 phase entries, including inserted 3.x and 7.x closure phases (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** - Phases 13 through 34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Full Mainnet Network Syncing** - Phases 35 through 41 (shipped 2026-05-23). Archive: [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Public Mainnet Sync Proof and Node Hardening** - Phases 42 through 53 (shipped 2026-06-02). Archive: [v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md)
- ✅ **v1.4 Mainnet IBD Convergence and Peer Compatibility** - Phases 54 through 59 (shipped 2026-06-05). Archive: [v1.4-ROADMAP.md](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Unattended Mainnet Node Operation Readiness** - Phases 60 through 67 (shipped 2026-06-10). Archive: [v1.5-ROADMAP.md](milestones/v1.5-ROADMAP.md)
- ✅ **v1.6 Mainnet Full-Sync Completion** - Phases 68 through 74 (shipped 2026-06-14). Archive: [v1.6-ROADMAP.md](milestones/v1.6-ROADMAP.md)
- **v1.7 Full-Sync Soak and Recovery Hardening** - Phases 75 through 80 (active). Requirements: 24

## Current Milestone: v1.7 Full-Sync Soak and Recovery Hardening

v1.7 Full-Sync Soak and Recovery Hardening turns the v1.6 explicit opt-in
sync-to-tip claim into a more durable long-run operator-review workflow. The
milestone focuses on multi-day soak stability, disk and resource bounds,
corruption and lock recovery, progress guarantees, stall diagnosis, richer
diagnostics, and redacted "what happened" support bundles for failed or
degraded long runs.

The milestone does not include inbound serving, address relay, block serving,
transaction relay, compact block relay, production-funds wallet claims,
migration apply mode, destructive repair, automatic support-bundle upload,
signed packaging, Windows service support, GUI work, hosted dashboards,
public-network checks in `bash scripts/verify.sh`, multi-day wall-clock tests
as default gates, or broad production-node claims.

Raw v1.0, v1.3, v1.4, v1.5, and v1.6 phase histories remain in
[.planning/phases/](phases/) for parity and UAT traceability; do not move or
delete those phase directories.

## Phases

- [x] **Phase 75: Multi-Day Soak Runner and Evidence Ledger** - Give operators a bounded, resumable, explicit opt-in soak workflow with durable run identity and deterministic synthetic coverage.
- [x] **Phase 76: Disk and Resource Bound Enforcement** - Make long-run disk, cache, queue, log, metric, and support-evidence bounds visible, enforceable, and testable.
- [ ] **Phase 77: Corruption and Lock Recovery Hardening** - Diagnose lock contention, stale locks, corruption markers, schema mismatches, partial writes, and storage-open failures without hidden mutation.
- [ ] **Phase 78: Progress Guarantees and Stall Diagnosis** - Ensure long-run progress is credited only for validated durable work and stalled paths produce actionable typed diagnosis.
- [ ] **Phase 79: Diagnostics and Support Bundle Forensics** - Produce redacted "what happened" support evidence that reconstructs timeline, resource pressure, peer outcomes, recovery events, and final verdict.
- [ ] **Phase 80: Opt-In Soak UAT and Release Boundaries** - Keep default verification deterministic while documenting opt-in multi-day soak commands and guarding the scoped v1.7 claim.

## Phase Details

### Phase 75: Multi-Day Soak Runner and Evidence Ledger

**Goal**: Operators can run bounded multi-day full-sync soaks with durable run identity, resumable reports, typed stop reasons, and deterministic synthetic soak coverage.
**Depends on**: Phase 74
**Requirements**: SOAK-01, SOAK-02, SOAK-03, SOAK-04
**Plans**: 6 plans

Plans:
- [x] 75-01-PLAN.md — Soak domain contracts, durable ledger, report projection, and outcome taxonomy.
- [x] 75-02-PLAN.md — `open-bitcoin soak` operator command parsing, dispatch, run/resume/stop/report behavior, and binary tests.
- [x] 75-03-PLAN.md — Deterministic synthetic long-run and ledger replay coverage without public-network or multi-day waits.
- [x] 75-04-PLAN.md — Compact redacted soak summary projection for support bundles.
- [x] 75-05-PLAN.md — Operator docs, architecture notes, parity roots, and scoped README wording.
- [x] 75-06-PLAN.md — Phase 75 checker, default verifier wiring, and generated LOC freshness.

**Success Criteria**:
1. Operator can start an explicit opt-in soak with bounded elapsed time, target height, datadir, network, peer policy, disk budget, and stop conditions.
2. Soak evidence persists durable run identity, start and end checkpoints, resume metadata, and final verdict across clean or interrupted runs.
3. Stop reasons distinguish clean completion, diagnosed blocker, operator stop, resource stop, recovery stop, and unexpected termination.
4. Deterministic synthetic soak tests exercise long-run control flow without public-network access or wall-clock multi-day waits.

### Phase 76: Disk and Resource Bound Enforcement

**Goal**: Operators can understand and enforce long-run resource limits before storage pressure turns a soak into an unsafe or opaque failure.
**Depends on**: Phase 75
**Requirements**: RES-05, RES-06, RES-07, RES-08
**Plans**: 6 plans

Plans:
- [x] 76-01-PLAN.md — Shared resource-bound status contracts, thresholds, and status schema field.
- [x] 76-02-PLAN.md — Status resource-bound collection and human rendering.
- [x] 76-03-PLAN.md — Soak preflight and runtime resource-stop enforcement.
- [x] 76-04-PLAN.md — Support bundle and dashboard resource-bound projections.
- [x] 76-05-PLAN.md — Operator docs, architecture docs, README updates, and parity records.
- [x] 76-06-PLAN.md — Phase 76 deterministic checker, verifier wiring, and LOC freshness.

**Success Criteria**:
1. Preflight and status surfaces expose disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle bounds for long soaks.
2. Runtime guidance classifies low disk, disk growth, compaction, log retention, metrics retention, and support-bundle size pressure.
3. Operators can pause or stop before unsafe storage pressure while preserving durable progress and a clear next action.
4. Deterministic fixtures verify resource-bound behavior without public peers, real service managers, or large local disk allocations.

### Phase 77: Corruption and Lock Recovery Hardening

**Goal**: Operators can diagnose store locks and corruption-style failures safely, with guidance that separates retryable, inspectable, rebuild-required, and escalation cases.
**Depends on**: Phase 76
**Requirements**: REC-05, REC-06, REC-07, REC-08
**Plans**: 7 plans

Plans:
- [ ] 77-01-PLAN.md — Shared recovery evidence contracts and pure classifier.
- [ ] 77-02-PLAN.md — Probe-only Fjall lock evidence and storage recovery fixtures.
- [ ] 77-03-PLAN.md — Non-mutating operator status recovery evidence.
- [ ] 77-04-PLAN.md — Support and dashboard recovery evidence projection.
- [ ] 77-05-PLAN.md — Soak checkpoint and report recovery evidence.
- [ ] 77-06-PLAN.md — Operator docs, architecture docs, and parity roots.
- [ ] 77-07-PLAN.md — Deterministic Phase 77 checker, verifier wiring, and verification evidence.

**Success Criteria**:
1. Lock contention, stale lock evidence, and concurrent datadir use are detected without hidden mutation.
2. Corruption markers, schema mismatches, partial writes, and unreadable runtime stores map to typed recovery categories.
3. Recovery evidence separates safe retry, read-only inspection, backup-then-rebuild, and stop-and-escalate guidance.
4. Deterministic tests cover lock contention, stale locks, corruption markers, schema mismatch, partial writes, and storage-open failures.

### Phase 78: Progress Guarantees and Stall Diagnosis

**Goal**: Operators can trust reported long-run progress and understand exactly which subsystem or external condition prevented useful work.
**Depends on**: Phase 77
**Requirements**: PROG-01, PROG-02, PROG-03, PROG-04
**Plans**: Pending

**Success Criteria**:
1. Soak progress is credited only after validated, durably connected active-chain progress or explicit stay-current evidence.
2. Status evidence includes expected progress windows, last useful work, last peer contribution, stalled subsystem, and no-progress threshold state.
3. Diagnosis distinguishes public-network reachability, incompatible peers, slow peers, stalled validation, storage pressure, at-tip waiting, and local shutdown.
4. Deterministic tests prove false-progress prevention, stale in-flight cleanup, peer rotation, at-tip waiting, and validation-stall classification.

### Phase 79: Diagnostics and Support Bundle Forensics

**Goal**: Operators can generate redacted support evidence that answers what happened during a failed or degraded long run.
**Depends on**: Phase 78
**Requirements**: DIAG-01, DIAG-02, DIAG-03, DIAG-04
**Plans**: Pending

**Success Criteria**:
1. Support bundles include a redacted soak timeline, checkpoint chain, resource pressure, recovery events, peer outcomes, and final verdict.
2. CLI status, dashboard status, RPC status, metrics, structured logs, live-smoke reports, and support bundles share one diagnostic contract.
3. Failure narratives identify likely cause, evidence basis, next action, and whether the run proved soak stability, diagnosed a blocker, or stopped inconclusively.
4. Deterministic checks verify support-bundle redaction, size bounds, timeline ordering, and cross-surface consistency.

### Phase 80: Opt-In Soak UAT and Release Boundaries

**Goal**: Contributors keep default verification deterministic while operators get copy-pasteable long-run UAT commands and reviewers can audit the scoped v1.7 claim.
**Depends on**: Phase 79
**Requirements**: VER-05, VER-06, VER-07, REL-04
**Plans**: Pending

**Success Criteria**:
1. `bash scripts/verify.sh` runs without internet access, public peers, real service managers, multi-day sleeps, current-tip timing, or large disk consumption.
2. Operator docs provide repo-local Cargo and Bazel commands for opt-in multi-day soak, bounded recovery drills, support-bundle generation, and post-failure diagnosis.
3. Parity breadcrumbs, fixtures, support-bundle schemas, deterministic checkers, and operator docs cover every new v1.7 source, test, and evidence surface.
4. v1.7 docs and status surfaces describe only explicit opt-in soak and recovery hardening, not broad production-node readiness.

## Progress

**Execution Order:** Phase 75 -> 76 -> 77 -> 78 -> 79 -> 80

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 75. Multi-Day Soak Runner and Evidence Ledger | 6/6 | Complete    | 2026-06-15 |
| 76. Disk and Resource Bound Enforcement | 6/6 | Complete | 2026-06-15 |
| 77. Corruption and Lock Recovery Hardening | 0/7 | Pending | - |
| 78. Progress Guarantees and Stall Diagnosis | 0/0 | Pending | - |
| 79. Diagnostics and Support Bundle Forensics | 0/0 | Pending | - |
| 80. Opt-In Soak UAT and Release Boundaries | 0/0 | Pending | - |

## Completed Milestone Summaries

<details>
<summary>✅ v1.6 Mainnet Full-Sync Completion (Phases 68-74) - SHIPPED 2026-06-14</summary>

- [x] Phase 68: Full Active-Chain Validation and Durable Persistence (3/3 plans) - completed 2026-06-11
- [x] Phase 69: Tip Tracking and Stay-Current Operation (5/5 plans) - completed 2026-06-12
- [x] Phase 70: Reorg, Peer Rotation, and No-Progress Recovery (6/6 plans) - completed 2026-06-12
- [x] Phase 71: Resource Bounds and Durable Restart/Resume (4/4 plans) - completed 2026-06-13
- [x] Phase 72: Operator Observability and Support Evidence (4/4 plans) - completed 2026-06-13
- [x] Phase 73: Opt-In UAT and Deterministic Verification (4/4 plans) - completed 2026-06-14
- [x] Phase 74: Release Boundaries, Parity, and Documentation (1/1 plans) - completed 2026-06-14

Detailed phase requirements, success criteria, and plan links are archived in
[milestones/v1.6-ROADMAP.md](milestones/v1.6-ROADMAP.md). Raw v1.6 phase
execution artifacts remain in [.planning/phases/](phases/) for parity and UAT
traceability.

</details>

<details>
<summary>✅ v1.5 Unattended Mainnet Node Operation Readiness (Phases 60-67) - SHIPPED 2026-06-10</summary>

Detailed phase requirements, success criteria, and plan links are archived in
[milestones/v1.5-ROADMAP.md](milestones/v1.5-ROADMAP.md). Raw v1.5 phase
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
| v1.6 Mainnet Full-Sync Completion | 7 | 27 | Shipped | 2026-06-14 |
| v1.7 Full-Sync Soak and Recovery Hardening | 6 | 19 | Active | - |

## Next Step

Plan Phase 77:

```bash
/gsd-plan-phase 77
```
