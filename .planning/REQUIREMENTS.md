# Requirements: Open Bitcoin v1.7 Full-Sync Soak and Recovery Hardening

**Defined:** 2026-06-14
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v1.7 Requirements

Requirements for v1.7 Full-Sync Soak and Recovery Hardening. Each requirement maps to exactly one roadmap phase.

### Multi-Day Soak Stability

- [ ] **SOAK-01**: Operator can run an explicit opt-in full-sync soak for multiple days with durable run identity, start and end checkpoints, and resumable report state.
- [ ] **SOAK-02**: Operator can bound a soak by elapsed time, target height, datadir, network, peer policy, disk budget, and stop condition without changing default verification.
- [ ] **SOAK-03**: Operator can distinguish clean completion, diagnosed blocker, operator stop, resource stop, recovery stop, and unexpected termination in soak evidence.
- [ ] **SOAK-04**: Contributor can replay deterministic synthetic soak scenarios that exercise long-run control flow without public-network access or wall-clock multi-day tests.

### Disk And Resource Bounds

- [ ] **RES-05**: Operator can see disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle bounds before starting a long soak.
- [ ] **RES-06**: Operator can receive typed low-disk, disk-growth, compaction, log-retention, metrics-retention, and support-bundle size guidance during and after a soak.
- [ ] **RES-07**: Operator can stop or pause a soak before unsafe storage pressure while preserving durable progress and an actionable next step.
- [ ] **RES-08**: Contributor can verify resource-bound behavior with deterministic fixtures that do not require a public peer, real service manager, or large local disk allocation.

### Corruption And Lock Recovery

- [x] **REC-05**: Operator can detect lock contention, stale lock evidence, and concurrent datadir use with no hidden mutation of the source datadir.
- [x] **REC-06**: Operator can detect corruption markers, schema mismatches, partial writes, and unreadable runtime stores with typed recovery categories.
- [x] **REC-07**: Operator can generate recovery evidence that separates safe retry, read-only inspection, backup-then-rebuild, and stop-and-escalate guidance.
- [x] **REC-08**: Contributor can run deterministic recovery tests for lock contention, stale lock, corruption marker, schema mismatch, partial write, and storage-open failure paths.

### Progress Guarantees And Stall Diagnosis

- [ ] **PROG-01**: Operator can trust that reported soak progress is credited only after validated, durably connected active-chain progress or explicit stay-current evidence.
- [ ] **PROG-02**: Operator can see expected progress windows, last useful work, last peer contribution, stalled subsystem, and no-progress threshold evidence.
- [ ] **PROG-03**: Operator can distinguish public-network reachability issues, incompatible peers, slow peers, stalled validation, storage pressure, at-tip waiting, and local shutdown.
- [ ] **PROG-04**: Contributor can verify progress-guarantee logic with deterministic tests for false progress, stale in-flight work, peer rotation, at-tip waiting, and validation stalls.

### Diagnostics And Support Bundles

- [ ] **DIAG-01**: Operator can generate a redacted "what happened" support bundle that includes the soak timeline, checkpoint chain, resource pressure, recovery events, peer outcomes, and final verdict.
- [ ] **DIAG-02**: Operator can compare CLI status, dashboard status, RPC status, metrics, structured logs, live-smoke reports, and support bundles against one shared diagnostic contract.
- [ ] **DIAG-03**: Operator can read concise failure narratives that identify the likely cause, evidence basis, next action, and whether the run proved soak stability, diagnosed a blocker, or stopped inconclusively.
- [ ] **DIAG-04**: Contributor can verify support-bundle redaction, size bounds, timeline ordering, and cross-surface consistency through deterministic checks.

### Opt-In UAT, Verification, And Release Boundaries

- [ ] **VER-05**: Contributor can run `bash scripts/verify.sh` without internet access, public peers, real service managers, multi-day sleeps, current-tip timing, or large disk consumption.
- [ ] **VER-06**: Operator can run copy-pasteable repo-local Cargo and Bazel commands for opt-in multi-day soak, bounded recovery drills, support-bundle generation, and post-failure diagnosis.
- [ ] **VER-07**: Contributor can audit parity breadcrumbs, fixtures, support bundle schemas, deterministic checkers, and operator docs for every new v1.7 source, test, and evidence surface.
- [ ] **REL-04**: Contributor can verify v1.7 docs and status surfaces describe only explicit opt-in soak and recovery hardening, not broad production-node readiness.

## Future Requirements

Deferred to a future milestone, not part of v1.7.

### Production Node Expansion

- **PNODE-01**: Operator can accept inbound peers and advertise service capability.
- **PNODE-02**: Operator can serve blocks and participate in address relay.
- **PNODE-03**: Operator can participate in mempool transaction relay and compact block relay.
- **PNODE-04**: Operator can rely on signed packages and broader distribution polish for production deployment.

### Wallet And Migration Expansion

- **WALLET-01**: Operator can use wallets with production funds under an audited safety claim.
- **MIGR-01**: Operator can perform explicit migration apply mode from Bitcoin Core or Bitcoin Knots with backup-aware mutation.

### Product Surface Expansion

- **GUI-01**: Operator can use a desktop GUI or hosted dashboard.
- **WIN-01**: Operator can install and manage a Windows service.

### Long-Run Automation Expansion

- **SOAK-05**: Maintainer can run scheduled public-network soak monitors outside local development machines.
- **SOAK-06**: Maintainer can publish signed, externally comparable soak result artifacts for release candidates.

## Out of Scope

Explicitly excluded from v1.7 to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Inbound serving, address advertisement, block serving, mempool relay, transaction relay, and compact block relay | v1.7 hardens explicit opt-in outbound full-sync soak behavior before expanding node-serving scope. |
| Broad production full-node readiness claims | v1.7 proves soak stability and recovery evidence for the scoped workflow, not every production-node surface. |
| Production-funds wallet safety claims | Wallet production safety requires a separate audit and threat model. |
| Migration apply mode or source datadir mutation | Existing Core or Knots datadirs and wallets remain high-value data; mutation remains deferred. |
| Automatic destructive repair of corrupted stores | v1.7 may diagnose and guide recovery, but hidden destructive mutation remains out of scope. |
| Automatic upload of support bundles or operator telemetry | Support evidence must remain local and redacted unless an operator explicitly shares it. |
| Public-network checks inside `bash scripts/verify.sh` | Default verification must remain deterministic and public-network-free. |
| Multi-day wall-clock tests as default commit or CI gates | Long-run soak remains opt-in UAT; deterministic synthetic coverage guards default verification. |
| Centralized trusted peers, hidden tip oracles, pruning, assumeutxo, assumevalid, or snapshot bootstrap | v1.7 should harden first-party full-sync behavior without shortcutting the audited validation claim. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| SOAK-01 | Phase 75 | Pending |
| SOAK-02 | Phase 75 | Pending |
| SOAK-03 | Phase 75 | Pending |
| SOAK-04 | Phase 75 | Pending |
| RES-05 | Phase 76 | Pending |
| RES-06 | Phase 76 | Pending |
| RES-07 | Phase 76 | Pending |
| RES-08 | Phase 76 | Pending |
| REC-05 | Phase 77 | Complete |
| REC-06 | Phase 77 | Complete |
| REC-07 | Phase 77 | Complete |
| REC-08 | Phase 77 | Complete |
| PROG-01 | Phase 78 | Pending |
| PROG-02 | Phase 78 | Pending |
| PROG-03 | Phase 78 | Pending |
| PROG-04 | Phase 78 | Pending |
| DIAG-01 | Phase 79 | Pending |
| DIAG-02 | Phase 79 | Pending |
| DIAG-03 | Phase 79 | Pending |
| DIAG-04 | Phase 79 | Pending |
| VER-05 | Phase 80 | Pending |
| VER-06 | Phase 80 | Pending |
| VER-07 | Phase 80 | Pending |
| REL-04 | Phase 80 | Pending |

**Coverage:**
- v1.7 requirements: 24 total
- Mapped to phases: 24
- Unmapped: 0

---
*Requirements defined: 2026-06-14*
*Last updated: 2026-06-14 after v1.7 milestone initialization*
