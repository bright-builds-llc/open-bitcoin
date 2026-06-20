---
gsd_state_version: 1.0
milestone: v1.7
milestone_name: Full-Sync Soak and Recovery Hardening
status: archived
stopped_at: v1.7 archived; ready for next milestone planning
last_updated: "2026-06-20T14:51:57.000Z"
last_activity: 2026-06-20 -- v1.7 milestone archived and ready for /gsd-new-milestone
progress:
  total_phases: 7
  completed_phases: 7
  total_plans: 37
  completed_plans: 37
  percent: 100
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-06-20)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Planning next milestone

## Current Position

Milestone: v1.7 Full-Sync Soak and Recovery Hardening
Phase: none active
Plan: none active
Status: v1.7 archived; ready for next milestone planning
Last activity: 2026-06-20 -- v1.7 milestone archived and ready for `/gsd-new-milestone`

Progress: [██████████] 100%

## Performance Metrics

**Velocity:**

- Archived milestone plans completed: 37 in v1.7
- Prior milestone plans completed: 27 in v1.6
- Counted v1.7 summary tasks: 65

**By Phase:**

| Phase | Plans | Status |
|-------|------:|--------|
| 75. Multi-Day Soak Runner and Evidence Ledger | 6/6 | Complete |
| 76. Disk and Resource Bound Enforcement | 6/6 | Complete |
| 77. Corruption and Lock Recovery Hardening | 7/7 | Complete |
| 78. Progress Guarantees and Stall Diagnosis | 7/7 | Complete |
| 79. Diagnostics and Support Bundle Forensics | 4/4 | Complete |
| 80. Opt-In Soak UAT and Release Boundaries | 3/3 | Complete |
| 81. Milestone Audit Traceability Closure | 4/4 | Complete |

**Recent Trend:**

- v1.5 completed Phases 60 through 67 and archived with a passed milestone audit.
- v1.6 completed Phases 68 through 74 and shipped explicit opt-in mainnet full-sync completion evidence.
- v1.7 completed Phases 75 through 81 with multi-day soak stability evidence, resource and disk bounds, corruption and lock recovery, progress guarantees, diagnostics, support bundles, opt-in UAT, deterministic release-boundary checks, and audit traceability closure.
- Public-network soak and real service-manager evidence remain opt-in UAT outside default deterministic verification.

## Accumulated Context

### Decisions

Decisions are logged in `PROJECT.md` Key Decisions table. Recent decisions:

- [v1.7]: Scope the milestone to explicit opt-in full-sync soak and recovery hardening, while continuing to defer inbound serving, relay, production-funds wallet, migration apply mode, packaging polish, hosted dashboard, GUI, and broad production-node claims.
- [v1.7]: Keep multi-day public-network soak runs opt-in UAT evidence; default `bash scripts/verify.sh` must remain deterministic, public-network-free, service-manager-free, and free of wall-clock multi-day gates.
- [v1.7]: Archive passed audit evidence in `.planning/milestones/v1.7-MILESTONE-AUDIT.md` and leave raw v1.7 phase histories in `.planning/phases/` for parity and UAT traceability.
- [v1.6]: Scope the milestone to explicit opt-in `open-bitcoind` mainnet full-sync completion before inbound serving, relay, production-funds wallet, migration apply mode, packaging polish, hosted dashboard, GUI, or broad production-node claims.
- [v1.6]: Continue keeping public-network full-sync and service checks opt-in UAT evidence unless a future phase deliberately changes the deterministic verification contract.

### Pending Todos

- Start the next milestone with `/gsd-new-milestone`; that workflow will create fresh requirements.
- Keep raw phase histories in `.planning/phases/` for parity and UAT traceability.
- Keep public-network full-sync and multi-day soak UAT opt-in and outside default deterministic verification unless a future milestone or phase explicitly changes that contract.

### Blockers/Concerns

- No active milestone blockers are recorded.
- Default local verification must remain deterministic; public-network full-sync and multi-day soak checks stay opt-in UAT evidence unless a future milestone or phase explicitly changes that contract.
- `.planning/phases/` retains raw v1.0, v1.3, v1.4, v1.5, v1.6, and v1.7 evidence referenced by parity docs and milestone archives.

## Session Continuity

Last session: 2026-06-20T14:51:57.000Z
Stopped at: v1.7 archived; ready for next milestone planning
Resume file: `.planning/milestones/v1.7-MILESTONE-AUDIT.md`
