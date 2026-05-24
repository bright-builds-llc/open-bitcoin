---
gsd_state_version: 1.0
milestone: v1.3
milestone_name: Public Mainnet Sync Proof and Node Hardening
status: planning
stopped_at: Phase 42 complete; ready for Phase 43
last_updated: "2026-05-24T14:18:50.544Z"
last_activity: 2026-05-24 -- Phase 42 verified live-smoke preflight implementation
progress:
  total_phases: 9
  completed_phases: 1
  total_plans: 1
  completed_plans: 1
  percent: 11
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-05-24)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** v1.3 Public Mainnet Sync Proof and Node Hardening

## Current Position

Phase: 42 of 50 (1 of 9 in v1.3) - Live Smoke Entry and Network Preflight
Plan: 42-01 complete
Status: Complete; ready for Phase 43 planning
Last activity: 2026-05-24 -- Phase 42 verified live-smoke preflight implementation

Progress: [#---------] 11%

## Performance Metrics

**Velocity:**

- Total plans completed: 1
- Average duration: -
- Total execution time: -

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 42 | 1 | complete | - |
| 43-50 | TBD | - | - |

**Recent Trend:**

- Last 5 plans: 42-01
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

### Pending Todos

- 0 pending.

### Blockers/Concerns

- Public-network proof depends on operator environment and may close with diagnosed network/environment blockers if live progress cannot be observed.
- Default local verification must remain deterministic; public-network checks stay opt-in.
- `.planning/phases/` remains retained for v1.0 evidence referenced by parity docs.

## Session Continuity

Last session: 2026-05-24T13:45:27.122Z
Stopped at: Phase 42 complete; ready for Phase 43
Resume file: .planning/phases/42-live-smoke-entry-and-network-preflight/42-01-SUMMARY.md
