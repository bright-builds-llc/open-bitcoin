# Roadmap: Open Bitcoin

## Milestones

- ✅ **v1.0 Headless Parity** - 22 phase entries, including inserted 3.x and 7.x closure phases (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** - Phases 13 through 34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Full Mainnet Network Syncing** - Phases 35 through 41 (shipped 2026-05-23). Archive: [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Public Mainnet Sync Proof and Node Hardening** - Phases 42 through 53 (shipped 2026-06-02). Archive: [v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md)
- ✅ **v1.4 Mainnet IBD Convergence and Peer Compatibility** - Phases 54 through 59 (shipped 2026-06-05). Archive: [v1.4-ROADMAP.md](milestones/v1.4-ROADMAP.md)
- ✅ **v1.5 Unattended Mainnet Node Operation Readiness** - Phases 60 through 67 (shipped 2026-06-10). Archive: [v1.5-ROADMAP.md](milestones/v1.5-ROADMAP.md)
- ✅ **v1.6 Mainnet Full-Sync Completion** - Phases 68 through 74 (shipped 2026-06-14). Archive: [v1.6-ROADMAP.md](milestones/v1.6-ROADMAP.md)
- ✅ **v1.7 Full-Sync Soak and Recovery Hardening** - Phases 75 through 81 (shipped 2026-06-20). Archive: [v1.7-ROADMAP.md](milestones/v1.7-ROADMAP.md)
- 🚧 **v1.8 Production Full-Node Readiness Boundary** - Phases 82 through 89 (gap closure active).

## Current Focus

v1.8 Production Full-Node Readiness Boundary is in audit gap closure.

**Goal:** Define and enforce the support, upgrade, service, runbook, release-readiness, and evidence boundaries required before Open Bitcoin may truthfully claim production full-node readiness.

**Granularity:** fine
**Coverage:** 23/23 v1.8 requirements mapped; REL-01 through REL-04 routed to Phase 89 gap closure.

## Phases

- [x] **Phase 82: Production Claim Boundary** - Define production-readiness terminology, support levels, deferred surfaces, and evidence gates.
- [x] **Phase 83: Support Matrix and Issue Evidence** - Make supported environments, issue-report evidence, residual risks, and support-matrix update boundaries explicit.
- [x] **Phase 84: Upgrade and Rollback Policy** - Document source-built upgrade, rollback, backup, and state/schema compatibility expectations.
- [x] **Phase 85: Operator Runbooks** - Provide preflight, long-run operation, diagnosis, recovery, support-bundle, and escalation runbooks.
- [x] **Phase 86: Service Operation Expectations** - Bound source-built daemon and service-supervision expectations with repo-local verification commands. (completed 2026-06-22)
- [x] **Phase 87: Release Readiness Checklist** - Map all v1.8 requirements to release evidence, docs, UAT, residual risk, and the no-claim boundary. (completed 2026-06-23)
- [x] **Phase 88: Deterministic Claim Guardrails** - Add default verification checks that fail overbroad production-readiness or deferred-surface claims. (completed 2026-06-23)
- [ ] **Phase 89: Release Readiness Guardrail Closure** - Close v1.8 milestone audit gaps by wiring Phase 88 guardrail evidence into the release checklist and expanding the deterministic claim-guardrail corpus. (gap closure)

## Phase Details

### Phase 82: Production Claim Boundary
**Goal**: Operators and release reviewers can understand exactly what production full-node readiness means, what evidence gates are required, and which shipped surfaces remain outside the claim.
**Depends on**: Nothing (first v1.8 phase)
**Requirements**: PROD-01, PROD-02, PROD-03, PROD-04
**Success Criteria** (what must be TRUE):
  1. Operator can read one production-readiness definition that distinguishes supported, preview, opt-in UAT, unsupported, and deferred surfaces.
  2. Release reviewer can trace each allowed production-related statement to an evidence gate, current status, and verification source.
  3. Contributor can identify the full evidence set required before a future production full-node readiness claim is allowed.
  4. Operator-facing release language preserves the deferred-surface inventory without implying production support.
**Plans**: 4 plans

Plans:
- [x] 82-01-PLAN.md - Canonical production claim boundary and release-readiness handoff.
- [x] 82-02-PLAN.md - Parity metadata roots and deferred-surface register.
- [x] 82-03-PLAN.md - README, runtime guide, and catalog boundary pointers.
- [x] 82-04-PLAN.md - Narrow Phase 82 checker, verifier wiring, and closeout evidence.

### Phase 83: Support Matrix and Issue Evidence
**Goal**: Operators, contributors, and release reviewers can use the v1.8 support matrix without accidentally broadening support or hiding carried-forward risks.
**Depends on**: Phase 82
**Requirements**: SUP-01, SUP-02, SUP-03, SUP-04
**Success Criteria** (what must be TRUE):
  1. Operator can classify source-built install, runtime, network, storage, and service-supervision environments by support level.
  2. Operator can identify the support information expected for issue reports, including redacted bundles, logs, config summaries, service state, resource evidence, and sync evidence.
  3. Contributor can update the support matrix while preserving the production-boundary and deferred-surface limits.
  4. Release reviewer can see residual risks and manual validation surfaces carried forward from v1.1 through v1.7.
**Plans**: 4/4 plans complete

### Phase 84: Upgrade and Rollback Policy
**Goal**: Operators can make source-built upgrade, rollback, backup, and state/schema decisions without hidden datadir, wallet, service, or config mutation.
**Depends on**: Phase 82
**Requirements**: UPG-01, UPG-02, UPG-03, UPG-04
**Success Criteria** (what must be TRUE):
  1. Operator can follow a pre-upgrade checklist covering backups, source-built binaries, config files, datadir ownership, service state, and current sync evidence.
  2. Operator can distinguish upgrade, retry, rollback, backup-then-rebuild, and stop-and-escalate guidance for state and schema compatibility outcomes.
  3. Operator can follow failed-upgrade and rollback guidance that avoids hidden mutation of source datadirs, wallets, services, and configs.
  4. Contributor can run deterministic checks that fail when upgrade policy, rollback boundaries, or backup expectations drift from the release-readiness contract.
**Plans**: 4 plans

Plans:
- [x] 84-01-PLAN.md - Canonical source-built upgrade and rollback policy.
- [x] 84-02-PLAN.md - Parity roots and release-boundary links.
- [x] 84-03-PLAN.md - README, runtime guide, and catalog entrypoint links.
- [x] 84-04-PLAN.md - Phase 84 checker, verifier wiring, and closeout evidence.

### Phase 85: Operator Runbooks
**Goal**: Operators can run, monitor, diagnose, recover, and escalate long-running source-built node operation using the existing v1.3 through v1.7 evidence surfaces.
**Depends on**: Phase 83, Phase 84
**Requirements**: RUN-01, RUN-02, RUN-03
**Success Criteria** (what must be TRUE):
  1. Operator can complete a production-boundary preflight before long-running source-built node operation.
  2. Operator can follow long-run monitoring, no-progress diagnosis, recovery, and escalation runbooks using shipped evidence surfaces.
  3. Operator can collect a redacted support-bundle timeline and identify when evidence is sufficient for support triage.
**Plans**: 4 plans

Plans:
- [x] 85-01-PLAN.md - Canonical operator runbook for preflight, monitoring, diagnosis, recovery, support timeline, and escalation.
- [x] 85-02-PLAN.md - Parity roots, release-boundary links, README, runtime guide, and operator-runtime catalog pointers.
- [x] 85-03-PLAN.md - Narrow deterministic Bun checker, fixture tests, and verifier wiring for Phase 85 runbooks.
- [x] 85-04-PLAN.md - Generated docs freshness, focused checker runs, and full repo-native verification closeout.

### Phase 86: Service Operation Expectations
**Goal**: Operators can distinguish source-built daemon operation, service supervision, distribution limits, and production-service claim boundaries, then verify expectations with repo-local commands.
**Depends on**: Phase 83, Phase 85
**Requirements**: SVC-01, SVC-02
**Success Criteria** (what must be TRUE):
  1. Operator can distinguish source-built daemon operation from service supervision, packaged-service distribution, service-manager availability, and unsupported production-service claims.
  2. Operator can verify service lifecycle, restart/resume, log, metric, resource-bound, and recovery expectations through repo-local Cargo command forms.
  3. Operator can verify the same service expectation evidence through repo-local Bazel command forms where applicable.
**Plans**: 4/4 plans complete

### Phase 87: Release Readiness Checklist
**Goal**: Release reviewers can decide whether v1.8 release language is truthful by following one checklist that maps requirements to evidence, docs, UAT, checks, residual risk, and the no-claim boundary.
**Depends on**: Phase 82, Phase 83, Phase 84, Phase 85, Phase 86
**Requirements**: REL-01, REL-05, REL-06
**Success Criteria** (what must be TRUE):
  1. Release reviewer can map every v1.8 requirement to documentation, UAT, deterministic checks, and residual-risk status.
  2. Contributor-facing README and parity docs point to the v1.8 boundary docs, support policy, upgrade policy, runbooks, and release-readiness checklist.
  3. Release reviewer can verify that v1.8 ends with a truthful no-claim boundary unless all production readiness gates are satisfied by a future milestone.
**Plans**: 1/1 plans complete

### Phase 88: Deterministic Claim Guardrails
**Goal**: Default verification prevents overbroad production-readiness language while keeping public-network, real service-manager, and multi-day checks opt-in.
**Depends on**: Phase 87
**Requirements**: REL-02, REL-03, REL-04
**Success Criteria** (what must be TRUE):
  1. Deterministic verification fails if release docs claim production full-node readiness without the required v1.8 evidence gates.
  2. Deterministic verification fails if release docs imply the deferred-surface inventory is production-ready.
  3. Default `bash scripts/verify.sh` runs the v1.8 release-boundary checker without requiring public-network, real service-manager, or multi-day checks.
**Plans**: 2/2 plans complete

### Phase 89: Release Readiness Guardrail Closure
**Goal**: Release reviewers can audit Phase 88 guardrails from the canonical v1.8 checklist, and deterministic claim guardrails cover every canonical v1.8 policy document that could otherwise promote deferred production-adjacent surfaces.
**Depends on**: Phase 88
**Requirements**: REL-01, REL-02, REL-03, REL-04
**Gap Closure**: Closes `.planning/v1.8-MILESTONE-AUDIT.md` GAP-01 and GAP-02, plus the release-readiness reviewer flow and deterministic claim-guardrail flow gaps.
**Success Criteria** (what must be TRUE):
  1. Release reviewer can find REL-02, REL-03, and REL-04 rows in the canonical v1.8 release-readiness checklist with evidence, default verification, UAT/manual posture, residual risk, and no-claim or next-gate status.
  2. Phase 88 deterministic claim-guardrail scanning covers the canonical upgrade policy, operator runbooks, and service expectation docs in addition to the existing release/operator corpus.
  3. Fixture coverage proves deferred-surface promotion in those canonical policy docs fails deterministically while scoped no-claim wording still passes.
  4. Gap-closure closeout records stale planning metadata refresh or explicitly routes any remaining planning hygiene to milestone closeout.
**Plans**: 3 plans

Plans:
- [ ] 89-01-PLAN.md - Release-readiness checklist rows and Phase 87 enforcement for REL-02 through REL-04.
- [ ] 89-02-PLAN.md - Phase 88 curated policy-doc corpus expansion, fixtures, and parity roots.
- [ ] 89-03-PLAN.md - Focused verification, LOC freshness, final verifier, and metadata-hygiene closeout.

## Progress Table

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 82. Production Claim Boundary | 4/4 | Complete    | 2026-06-21 |
| 83. Support Matrix and Issue Evidence | 4/4 | Complete    | 2026-06-21 |
| 84. Upgrade and Rollback Policy | 4/4 | Complete    | 2026-06-22 |
| 85. Operator Runbooks | 4/4 | Complete    | 2026-06-22 |
| 86. Service Operation Expectations | 4/4 | Complete    | 2026-06-22 |
| 87. Release Readiness Checklist | 1/1 | Complete    | 2026-06-23 |
| 88. Deterministic Claim Guardrails | 2/2 | Complete   | 2026-06-23 |
| 89. Release Readiness Guardrail Closure | 0/3 | Pending | - |

## Completed Milestone Summaries

<details>
<summary>✅ v1.7 Full-Sync Soak and Recovery Hardening (Phases 75-81) - SHIPPED 2026-06-20</summary>

Detailed phase requirements, success criteria, and plan links are archived in
[milestones/v1.7-ROADMAP.md](milestones/v1.7-ROADMAP.md). Requirements are
archived in [milestones/v1.7-REQUIREMENTS.md](milestones/v1.7-REQUIREMENTS.md),
and the passed milestone audit is archived in
[milestones/v1.7-MILESTONE-AUDIT.md](milestones/v1.7-MILESTONE-AUDIT.md).
Raw v1.7 phase execution artifacts remain in [.planning/phases/](phases/) for
parity and UAT traceability.

</details>

<details>
<summary>✅ v1.6 Mainnet Full-Sync Completion (Phases 68-74) - SHIPPED 2026-06-14</summary>

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
| v1.7 Full-Sync Soak and Recovery Hardening | 7 | 37 | Shipped | 2026-06-20 |
| v1.8 Production Full-Node Readiness Boundary | 8 | 26 | Gap closure | - |

## Next Step

Plan the v1.8 gap closure phase:

```bash
/gsd-plan-phase 89
```
