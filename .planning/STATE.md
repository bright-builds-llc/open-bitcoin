---
gsd_state_version: 1.0
milestone: v1.5
milestone_name: Unattended Mainnet Node Operation Readiness
status: shipped
stopped_at: v1.5 milestone archived; ready for next milestone definition
last_updated: "2026-06-10T20:42:02.509Z"
last_activity: 2026-06-10 -- v1.5 milestone archived
progress:
  total_phases: 8
  completed_phases: 8
  total_plans: 22
  completed_plans: 22
  percent: 100
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-06-10)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** v1.5 shipped; define the next milestone.

## Current Position

Milestone: v1.5 Unattended Mainnet Node Operation Readiness — SHIPPED
Phase: none active
Plan: none active
Status: Ready for next milestone definition
Last activity: 2026-06-10 -- v1.5 milestone archived

Progress: [##########] 100%

## Performance Metrics

**Velocity:**

- Current milestone plans completed: 22
- Current milestone plan count: 22
- Prior milestone plans completed: 15 in v1.4

**By Phase:**

| Phase | Plans | Status |
|-------|------:|--------|
| 60 | 1/1 | Complete |
| 61 | 6/6 | Complete |
| 62 | 4/4 | Complete |
| 63 | 4/4 | Complete |
| 64 | 3/3 | Complete |
| 65 | 2/2 | Complete |
| 66 | 1/1 | Complete |
| 67 | 1/1 | Complete |

**Recent Trend:**

- v1.5 completed Phases 60 through 67 and archived with a passed milestone audit.
- Next workflow: `/gsd-new-milestone`.

## Accumulated Context

### Decisions

Decisions are logged in `PROJECT.md` Key Decisions table. Recent v1.5 decisions:

- [v1.5]: Scope the milestone to bounded unattended mainnet node operation readiness, while continuing to defer inbound serving, transaction relay, production-funds wallet, migration apply mode, packaging polish, hosted dashboard, GUI, and broad production-node claims.
- [v1.5]: Continue phase numbering from v1.4; active milestone starts at Phase 60 and runs through Phase 67.
- [v1.5]: Keep public-network long-run and service checks opt-in UAT evidence, not default `bash scripts/verify.sh` checks.
- [v1.5]: Preserve `.planning/phases/` raw histories for v1.0, v1.3, v1.4, and v1.5 parity and UAT traceability.
- [Phase 64]: Service restart/resume status exposes selected datadir, clean versus unclean prior shutdown, durable progress, stale in-flight verdict, recovery category, and next-action guidance.
- [Phase 64]: Keep real service-manager restarts and public-network restart smoke as opt-in UAT evidence outside `bash scripts/verify.sh`.
- [Phase 65]: Redacted support bundles include bounded service lifecycle, restart/resume, log path, metrics availability, and live-smoke summary interpretation evidence.
- [Phase 65]: Default verification includes a deterministic support review checker and continues to exclude public-network and real service-manager commands.
- [Phase 66]: The operator CLI exposes deterministic compatibility harness reports through `open-bitcoin compatibility harness`.
- [Phase 66]: Compatibility wrapper diagnosis delegates to `open-bitcoin-network::evaluate_transcript`; CLI code only constructs scenarios and renders local reports.
- [Phase 66]: Default verification includes a deterministic compatibility wrapper checker and continues to exclude public-network peer probing.
- [Phase 67]: v1.5 is documented as source-built, explicit opt-in unattended mainnet operator-review readiness, not production-node readiness.
- [Phase 67]: The deterministic checker guards v1.5 parity roots and default-verification exclusions.

### Pending Todos

- Define the next milestone with `/gsd-new-milestone`.

### Blockers/Concerns

- No active milestone blockers are recorded.
- Default local verification must remain deterministic; public-network checks stay opt-in UAT evidence unless a future milestone explicitly changes that contract.
- `.planning/phases/` retains raw v1.0, v1.3, v1.4, and v1.5 evidence referenced by parity docs and milestone archives.

## Session Continuity

Last session: 2026-06-10T20:42:02.509Z
Stopped at: v1.5 milestone archived; ready for next milestone definition
Resume file: `.planning/milestones/v1.5-ROADMAP.md`
