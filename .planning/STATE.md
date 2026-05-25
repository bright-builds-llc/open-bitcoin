---
gsd_state_version: 1.0
milestone: v1.3
milestone_name: Public Mainnet Sync Proof and Node Hardening
status: planning
stopped_at: Phase 44 complete; ready for Phase 45
last_updated: "2026-05-25T17:07:25.113Z"
last_activity: 2026-05-25 -- Phase 44 completed peer contribution attribution
progress:
  total_phases: 9
  completed_phases: 3
  total_plans: 3
  completed_plans: 3
  percent: 33
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-05-24)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 45 — Runtime Resource Bounds and Store Coordination

## Current Position

Phase: 45 of 50 (4 of 9 in v1.3) - Runtime Resource Bounds and Store Coordination
Plan: Not started
Status: Ready to plan Phase 45
Last activity: 2026-05-25 -- Phase 44 completed peer contribution attribution

Progress: [###-------] 33%

## Performance Metrics

**Velocity:**

- Total plans completed: 3
- Average duration: -
- Total execution time: -

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 42 | 1 | complete | - |
| 43 | 1 | complete | - |
| 44 | 1 | complete | - |
| 45-50 | TBD | - | - |

**Recent Trend:**

- Last 5 plans: 42-01, 43-01, 44-01
- Trend: Started

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- [v1.2]: Phase 41 found no `needs-phase` security follow-up before archive; future production surfaces still need fresh threat models when scoped.
- [v1.3]: Continue phase numbering from v1.2; active milestone starts at Phase 42.
- [v1.3]: Keep public-mainnet evidence opt-in and outside the default `bash scripts/verify.sh` gate.
- [v1.3]: Preserve scope boundaries: no inbound serving, transaction relay, production-funds wallet, migration apply mode, packaging, hosted dashboard, GUI, or unattended production-node claim.
- [phase 42]: The live-mainnet smoke runner is still opt-in, but now accepts repeatable `--manual-peer=HOST[:PORT]`, generates a review-local JSONC config for manual-peer runs, records endpoint outcomes, and preserves operator cancellation as a distinct report status.
- [phase 43]: Sync summaries now preserve configured outbound peer targets separately from observed peer counts, report retry-backoff skips as `waiting` with `retry_backoff`, and rotate mixed failures to replacement peers without advancing bad durable progress.
- [phase 44]: Sync peer outcomes now distinguish activity from useful contribution; headers and blocks count only after accepted sync handling, while idle, stalled, waiting, and failed peers remain visible without useful-progress credit.

### Pending Todos

- 0 pending.

### Blockers/Concerns

- Public-network proof depends on operator environment and may close with diagnosed network/environment blockers if live progress cannot be observed.
- Default local verification must remain deterministic; public-network checks stay opt-in.
- `.planning/phases/` remains retained for v1.0 evidence referenced by parity docs.

## Session Continuity

Last session: 2026-05-25T17:07:25.113Z
Stopped at: Phase 44 complete; ready for Phase 45
Resume file: .planning/phases/44-peer-contribution-attribution/44-VERIFICATION.md
