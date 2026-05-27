---
gsd_state_version: 1.0
milestone: v1.3
milestone_name: Public Mainnet Sync Proof and Node Hardening
status: planning
stopped_at: Phase 48 context gathered
last_updated: "2026-05-27T13:30:05.999Z"
last_activity: 2026-05-26 -- Phase 47 completed operator sync truth surfaces
progress:
  total_phases: 9
  completed_phases: 6
  total_plans: 6
  completed_plans: 6
  percent: 100
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-05-24)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 48 — Support Evidence and Operator Runbooks

## Current Position

Phase: 48 of 50 (7 of 9 in v1.3) - Support Evidence and Operator Runbooks
Plan: Not started
Status: Ready to plan Phase 48
Last activity: 2026-05-26 -- Phase 47 completed operator sync truth surfaces

Progress: [#######---] 67%

## Performance Metrics

**Velocity:**

- Total plans completed: 6
- Average duration: -
- Total execution time: -

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 42 | 1 | complete | - |
| 43 | 1 | complete | - |
| 44 | 1 | complete | - |
| 45 | 1 | complete | - |
| 46 | 1 | complete | - |
| 47 | 1 | complete | - |
| 48-50 | TBD | - | - |

**Recent Trend:**

- Last 5 plans: 43-01, 44-01, 45-01, 46-01, 47-01
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
- [phase 47]: Operator sync status now exposes progress signal and last successful progress; status, dashboard, metrics, logs, and RPC-facing blockchain info keep header, downloaded, and connected progress distinct.

### Pending Todos

- 0 pending.

### Blockers/Concerns

- Public-network proof depends on operator environment and may close with diagnosed network/environment blockers if live progress cannot be observed.
- Default local verification must remain deterministic; public-network checks stay opt-in.
- `.planning/phases/` remains retained for v1.0 evidence referenced by parity docs.

## Session Continuity

Last session: 2026-05-27T13:30:05.995Z
Stopped at: Phase 48 context gathered
Resume file: .planning/phases/48-support-evidence-and-operator-runbooks/48-CONTEXT.md
