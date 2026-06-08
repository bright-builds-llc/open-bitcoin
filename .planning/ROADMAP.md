# Roadmap: Open Bitcoin

## Milestones

- ✅ **v1.0 Headless Parity** - 22 phase entries, including inserted 3.x and 7.x closure phases (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** - Phases 13 through 34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Full Mainnet Network Syncing** - Phases 35 through 41 (shipped 2026-05-23). Archive: [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Public Mainnet Sync Proof and Node Hardening** - Phases 42 through 53 (shipped 2026-06-02). Archive: [v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md)
- ✅ **v1.4 Mainnet IBD Convergence and Peer Compatibility** - Phases 54 through 59 (shipped 2026-06-05). Archive: [v1.4-ROADMAP.md](milestones/v1.4-ROADMAP.md)
- 🚧 **v1.5 Unattended Mainnet Node Operation Readiness** - Phases 60 through 67 (active)

## Current Focus

v1.5 Unattended Mainnet Node Operation Readiness is ready for Phase 65 planning.
Next action: `/gsd-plan-phase 65`

## Phases

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

### 🚧 v1.5 Unattended Mainnet Node Operation Readiness (Active)

**Milestone Goal:** Make the opt-in `open-bitcoind` mainnet sync workflow bounded, restart-safe, and observable enough for extended unattended operator review without broadening the production-node claim.

- [x] **Phase 60: Unattended Sync Loop Control** - Operators can opt into repeated daemon sync cycles with bounded stop, retry, pause, resume, and shutdown behavior. (completed 2026-06-06)
- [x] **Phase 61: Resource Bounds and Recovery Taxonomy** - Operators can trust unattended sync bounds, recovery states, and next-action guidance across long runs. (completed 2026-06-06)
- [x] **Phase 62: Long-Run Sync Truth Surfaces** - Operators see consistent bounded sync truth across status, dashboard, RPC, metrics, logs, and live-smoke snapshots. (completed 2026-06-07)
- [x] **Phase 63: Service Supervision Lifecycle** - Operators can manage launchd or systemd supervision for the opt-in unattended workflow with truthful lifecycle state. (completed 2026-06-07)
- [x] **Phase 64: Service Restart and Same-Datadir Resume Evidence** - Operators can prove service-supervised restarts reopen durable state and resume sync safely. (completed 2026-06-07)
- [ ] **Phase 65: Support Bundle and Operator Review Docs** - Operators can collect redacted v1.5 evidence and follow repo-local deterministic and opt-in review commands.
- [ ] **Phase 66: Compatibility Harness Operator Wrapper** - Operators can run the Phase 54 public-peer compatibility harness through a documented CLI or repo script wrapper.
- [ ] **Phase 67: Release Boundaries and Deterministic Verification** - Reviewers can audit v1.5 claims, deferred scopes, and default verification boundaries.

## Phase Details

### Phase 60: Unattended Sync Loop Control
**Goal**: Operators can opt into repeated `open-bitcoind` mainnet sync cycles with bounded stop, retry, pause, resume, and shutdown behavior.
**Depends on**: Phase 59
**Requirements**: LOOP-01, LOOP-02, LOOP-03, LOOP-04
**Success Criteria** (what must be TRUE):
  1. Operator can start `open-bitcoind` with an explicit mainnet sync setting and observe repeated sync cycles after RPC binds without issuing another interactive command.
  2. Operator can see the documented stop reason when the loop stops for configured targets, pause or shutdown, sustained no progress, resource exhaustion, storage failure, or incompatible peer exhaustion.
  3. Peer, network, and protocol failures use bounded retry and backoff without hot-looping, unbounded peer creation, or crediting failed peers with useful progress.
  4. Operator can pause, resume, and cleanly shut down the loop while durable state and next-action guidance are preserved.
**Plans**: 1/1 complete ([60-01](phases/60-unattended-sync-loop-control/60-01-PLAN.md))

### Phase 61: Resource Bounds and Recovery Taxonomy
**Goal**: Operators can trust unattended sync bounds, recovery states, and next-action guidance across long runs.
**Depends on**: Phase 60
**Requirements**: RR-01, RR-02, RR-04
**Success Criteria** (what must be TRUE):
  1. Operator can inspect documented bounds for outbound peers, in-flight headers or blocks, retry queues, storage writes, metrics samples, structured logs, and support evidence size.
  2. Recovery handling distinguishes clean shutdown, unclean shutdown, incompatible schema, store corruption, storage lock contention, resource exhaustion, invalid peer data, public-network unreachability, and operator cancellation.
  3. Operator-visible errors and recovery guidance use consistent typed states across status, logs, support bundles, and docs.
  4. Extended unattended runs preserve the documented bounds without unbounded growth or silent loss of recovery evidence.
**Plans**: 6 plans
Plans:
- [ ] [61-01](phases/61-resource-bounds-and-recovery-taxonomy/61-01-PLAN.md) - Shared recovery category status contract
- [ ] [61-02](phases/61-resource-bounds-and-recovery-taxonomy/61-02-PLAN.md) - Storage and sync recovery category mappings
- [ ] [61-03](phases/61-resource-bounds-and-recovery-taxonomy/61-03-PLAN.md) - Runtime projection and deterministic resource-bound tests
- [ ] [61-04](phases/61-resource-bounds-and-recovery-taxonomy/61-04-PLAN.md) - Live-smoke and support evidence recovery/resource summaries
- [ ] [61-05](phases/61-resource-bounds-and-recovery-taxonomy/61-05-PLAN.md) - Status, dashboard, and RPC recovery category rendering
- [ ] [61-06](phases/61-resource-bounds-and-recovery-taxonomy/61-06-PLAN.md) - Operator docs and deterministic boundary checker

### Phase 62: Long-Run Sync Truth Surfaces
**Goal**: Operators see consistent bounded sync truth across status, dashboard, RPC, metrics, logs, and live-smoke snapshots.
**Depends on**: Phase 61
**Requirements**: OBS-01, OBS-02
**Success Criteria** (what must be TRUE):
  1. Status, dashboard, RPC sync status, metrics, structured logs, and live-smoke snapshots agree on loop phase, configured targets, attempt counters, latest progress, latest stop reason, peer health, and downloaded or connected block evidence.
  2. Metrics and structured logs retain bounded long-run samples and cycle summaries without unbounded growth.
  3. Operator can distinguish progress, waiting, retry, stop, and recovery states the same way across every truth surface.
  4. Repeated long-run snapshot output stays compact enough for operator review while preserving diagnosis evidence.
**Plans**: 4/4 complete
Plans:
- [x] [62-01](phases/62-long-run-sync-truth-surfaces/62-01-PLAN.md) - Canonical Rust sync truth contract and durable projection
- [x] [62-02](phases/62-long-run-sync-truth-surfaces/62-02-PLAN.md) - Rust status, dashboard, sync-status, and RPC truth rendering
- [x] [62-03](phases/62-long-run-sync-truth-surfaces/62-03-PLAN.md) - Compact live-smoke JSON and Markdown truth snapshots
- [x] [62-04](phases/62-long-run-sync-truth-surfaces/62-04-PLAN.md) - Operator docs and deterministic cross-surface contract checker

### Phase 63: Service Supervision Lifecycle
**Goal**: Operators can manage launchd or systemd supervision for the opt-in unattended workflow with truthful lifecycle state.
**Depends on**: Phase 62
**Requirements**: SVC-01, SVC-02, SVC-04
**Success Criteria** (what must be TRUE):
  1. Operator can preview, install, start, stop, restart, and inspect launchd or systemd supervision for the opt-in unattended daemon workflow.
  2. Service status distinguishes unmanaged, installed-stopped, running, failed, disabled, and unavailable-manager states while preserving shared sync truth fields.
  3. Service runbooks explain log locations, config paths, safe shutdown, restart review, and recovery actions for launchd and systemd operators.
  4. Service commands and docs keep the workflow framed as opt-in extended operator review, not a broad production-node claim.
**Plans**: 4 plans
Plans:
- [x] [63-01](phases/63-service-supervision-lifecycle/63-01-PLAN.md) - Service preview command and open-bitcoind service target
- [x] [63-02](phases/63-service-supervision-lifecycle/63-02-PLAN.md) - Start, stop, and restart service actions
- [x] [63-03](phases/63-service-supervision-lifecycle/63-03-PLAN.md) - Shared lifecycle status and rendering
- [x] [63-04](phases/63-service-supervision-lifecycle/63-04-PLAN.md) - Operator runbook and deterministic lifecycle checker

### Phase 64: Service Restart and Same-Datadir Resume Evidence
**Goal**: Operators can prove service-supervised restarts reopen durable state and resume sync safely.
**Depends on**: Phase 63
**Requirements**: SVC-03, RR-03
**Success Criteria** (what must be TRUE):
  1. Daemon restart under service supervision reopens durable sync state and reports clean versus unclean prior shutdown.
  2. Same-datadir restart tests cover extended loop recovery without duplicate block requests, duplicate block connects, corrupted active chainstate, or lost progress counters.
  3. Restart and resume status gives explicit next-action guidance and resumes bounded sync work without stale in-flight requests.
  4. Restart and resume evidence is available through deterministic tests and opt-in UAT reports without making public-network checks part of default verification.
**Plans**: 3/3 complete
Plans:
- [x] [64-01](phases/64-service-restart-and-same-datadir-resume-evidence/64-01-PLAN.md) - Shared service restart/resume status contract
- [x] [64-02](phases/64-service-restart-and-same-datadir-resume-evidence/64-02-PLAN.md) - Status, dashboard, and service restart evidence rendering
- [x] [64-03](phases/64-service-restart-and-same-datadir-resume-evidence/64-03-PLAN.md) - Operator docs and deterministic restart/resume checker

### Phase 65: Support Bundle and Operator Review Docs
**Goal**: Operators can collect redacted v1.5 evidence and follow repo-local deterministic and opt-in review commands.
**Depends on**: Phase 64
**Requirements**: OBS-03, OBS-04
**Success Criteria** (what must be TRUE):
  1. Operator can generate a redacted v1.5 support bundle summarizing long-run sync cycles, service state, restart and recovery evidence, peer outcomes, progress counters, stop reasons, metrics, logs, and config sources.
  2. Support bundles exclude credentials and raw local report artifacts while preserving fields needed to diagnose unattended operation.
  3. Operator docs provide copy-pasteable repo-local Cargo and Bazel commands for deterministic checks, opt-in long-run review, service-based review, support bundle collection, and pass/fail interpretation.
  4. Docs make public-network long-run and service checks explicit opt-in UAT evidence rather than default `bash scripts/verify.sh` checks.
**Plans**: TBD

### Phase 66: Compatibility Harness Operator Wrapper
**Goal**: Operators can run the Phase 54 public-peer compatibility harness through a documented CLI or repo script wrapper with reports that align with daemon behavior.
**Depends on**: Phase 65
**Requirements**: COMPAT-01, COMPAT-02, COMPAT-03
**Success Criteria** (what must be TRUE):
  1. Operator can run the Phase 54 public-peer compatibility harness through a documented CLI or repo script wrapper instead of invoking the Rust harness path directly.
  2. Compatibility wrapper output includes stable JSON and Markdown reports with peer endpoint, network, negotiated capabilities, failing step, diagnosis, transcript summary, and redaction boundaries.
  3. Wrapper diagnostics cover version rejection, network mismatch, service-bit mismatch, unsupported message order, timeout, peer disconnect, malformed payload, and local configuration failure.
  4. Wrapper diagnoses align with daemon peer-replacement behavior and release-boundary docs.
**Plans**: TBD

### Phase 67: Release Boundaries and Deterministic Verification
**Goal**: Reviewers can audit v1.5 claims, deferred scopes, and default verification boundaries.
**Depends on**: Phase 66
**Requirements**: REL-01, REL-02, REL-03, REL-04
**Success Criteria** (what must be TRUE):
  1. Reviewer can inspect refreshed v1.5 threat-model and release-readiness docs covering unattended sync loop behavior, service supervision, long-run evidence, resource bounds, recovery states, support redaction, and compatibility wrapper output.
  2. Parity docs distinguish v1.5 extended operator-review readiness from deferred inbound serving, transaction relay, compact block relay, production-funds wallet use, migration apply mode, packaging distribution, hosted dashboard, GUI work, and broad production-node claims.
  3. Default repo verification remains deterministic; public-network long-run and service checks stay opt-in UAT evidence rather than part of `bash scripts/verify.sh`.
  4. Release-boundary checks fail deterministically when v1.5 docs or parity roots omit the unattended-operation claim boundaries.
**Plans**: TBD

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 60. Unattended Sync Loop Control | 1/1 | Complete    | 2026-06-06 |
| 61. Resource Bounds and Recovery Taxonomy | 6/6 | Complete   | 2026-06-06 |
| 62. Long-Run Sync Truth Surfaces | 4/4 | Complete    | 2026-06-07 |
| 63. Service Supervision Lifecycle | 4/4 | Complete    | 2026-06-07 |
| 64. Service Restart and Same-Datadir Resume Evidence | 3/3 | Complete | 2026-06-07 |
| 65. Support Bundle and Operator Review Docs | 0/TBD | Not started | - |
| 66. Compatibility Harness Operator Wrapper | 0/TBD | Not started | - |
| 67. Release Boundaries and Deterministic Verification | 0/TBD | Not started | - |

## Milestone History

| Milestone | Phases | Plans | Status | Shipped |
| --- | ---: | ---: | --- | --- |
| v1.0 Headless Parity | 22 | 80 | Shipped | 2026-04-26 |
| v1.1 Operator Runtime and Real-Network Sync | 22 | 69 | Shipped | 2026-04-30 |
| v1.2 Full Mainnet Network Syncing | 7 | 13 | Shipped | 2026-05-23 |
| v1.3 Public Mainnet Sync Proof and Node Hardening | 12 | 13 | Shipped | 2026-06-02 |
| v1.4 Mainnet IBD Convergence and Peer Compatibility | 6 | 15 | Shipped | 2026-06-05 |
| v1.5 Unattended Mainnet Node Operation Readiness | 8 | TBD | Active | - |

## Next Step

Plan Phase 65 support bundle and operator review docs:

```bash
/gsd-plan-phase 65
```
