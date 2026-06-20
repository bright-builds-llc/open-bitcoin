---
gsd_state_version: 1.0
milestone: v1.8
milestone_name: Production Full-Node Readiness Boundary
status: roadmap_ready
stopped_at: v1.8 roadmap created; ready to plan Phase 82
last_updated: "2026-06-20T18:22:16.000Z"
last_activity: 2026-06-20 -- v1.8 roadmap created with Phases 82 through 88 and 23/23 requirements mapped
progress:
  total_phases: 7
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
  percent: 0
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-06-20)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** v1.8 Production Full-Node Readiness Boundary

## Current Position

Milestone: v1.8 Production Full-Node Readiness Boundary
Phase: Phase 82 - Production Claim Boundary (not started)
Plan: none active
Status: Roadmap ready
Last activity: 2026-06-20 -- v1.8 roadmap created with seven phases covering support, upgrade, runbook, service, release-readiness, and claim-guardrail boundaries

Progress: [----------] 0%

## Performance Metrics

**Velocity:**

- Archived milestone plans completed: 37 in v1.7
- Prior milestone plans completed: 27 in v1.6
- Counted v1.7 summary tasks: 65

**By Phase:**

| Phase | Name | Requirements | Status |
|-------|------|--------------|--------|
| 82 | Production Claim Boundary | PROD-01, PROD-02, PROD-03, PROD-04 | Not started |
| 83 | Support Matrix and Issue Evidence | SUP-01, SUP-02, SUP-03, SUP-04 | Not started |
| 84 | Upgrade and Rollback Policy | UPG-01, UPG-02, UPG-03, UPG-04 | Not started |
| 85 | Operator Runbooks | RUN-01, RUN-02, RUN-03 | Not started |
| 86 | Service Operation Expectations | SVC-01, SVC-02 | Not started |
| 87 | Release Readiness Checklist | REL-01, REL-05, REL-06 | Not started |
| 88 | Deterministic Claim Guardrails | REL-02, REL-03, REL-04 | Not started |

**Recent Trend:**

- v1.5 completed Phases 60 through 67 and archived with a passed milestone audit.
- v1.6 completed Phases 68 through 74 and shipped explicit opt-in mainnet full-sync completion evidence.
- v1.7 completed Phases 75 through 81 with multi-day soak stability evidence, resource and disk bounds, corruption and lock recovery, progress guarantees, diagnostics, support bundles, opt-in UAT, deterministic release-boundary checks, and audit traceability closure.
- v1.8 is planned as Phases 82 through 88, covering 23/23 requirements for production-readiness terminology, support boundaries, upgrade policy, operator runbooks, service expectations, release-readiness documentation, and deterministic claim guardrails.
- Public-network soak and real service-manager evidence remain opt-in UAT outside default deterministic verification.

## Accumulated Context

### Decisions

Decisions are logged in `PROJECT.md` Key Decisions table. Recent decisions:

- [v1.8]: Structure the milestone as seven continuation phases, starting at Phase 82 after the completed v1.7 Phase 81 archive.
- [v1.8]: Keep the milestone boundary-setting only; v1.8 defines evidence gates and guardrails before any production full-node readiness claim is allowed.
- [v1.8]: Keep default verification deterministic and public-network-free while adding release-boundary checks that fail overbroad production-readiness language.
- [v1.7]: Scope the milestone to explicit opt-in full-sync soak and recovery hardening, while continuing to defer inbound serving, relay, production-funds wallet, migration apply mode, packaging polish, hosted dashboard, GUI, and broad production-node claims.
- [v1.7]: Keep multi-day public-network soak runs opt-in UAT evidence; default `bash scripts/verify.sh` must remain deterministic, public-network-free, service-manager-free, and free of wall-clock multi-day gates.
- [v1.7]: Archive passed audit evidence in `.planning/milestones/v1.7-MILESTONE-AUDIT.md` and leave raw v1.7 phase histories in `.planning/phases/` for parity and UAT traceability.
- [v1.6]: Continue keeping public-network full-sync and service checks opt-in UAT evidence unless a future phase deliberately changes the deterministic verification contract.

### Pending Todos

- Plan Phase 82 with `/gsd-plan-phase 82`.
- Keep raw phase histories in `.planning/phases/` for parity and UAT traceability.
- Keep public-network full-sync and multi-day soak UAT opt-in and outside default deterministic verification unless a future milestone or phase explicitly changes that contract.

### Blockers/Concerns

- No active milestone blockers are recorded.
- Default local verification must remain deterministic; public-network full-sync and multi-day soak checks stay opt-in UAT evidence unless a future milestone or phase explicitly changes that contract.
- `.planning/phases/` retains raw v1.0, v1.3, v1.4, v1.5, v1.6, and v1.7 evidence referenced by parity docs and milestone archives.

## Session Continuity

Last session: 2026-06-20T18:22:16.000Z
Stopped at: v1.8 roadmap created; ready to plan Phase 82
Resume file: `.planning/ROADMAP.md`
