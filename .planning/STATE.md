---
gsd_state_version: 1.0
milestone: v1.3
milestone_name: Public Mainnet Sync Proof and Node Hardening
status: milestone-ready
stopped_at: Phase 51 added for v1.3 audit gap closure
last_updated: "2026-05-31T13:50:00Z"
last_activity: 2026-05-31 -- Phase 51 gap closure phase added from milestone audit
progress:
  total_phases: 10
  completed_phases: 9
  total_plans: 10
  completed_plans: 10
  percent: 90
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-05-28)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** v1.3 Phase 51 audit gap closure

## Current Position

Phase: 51 (Live Smoke Fresh Status Integration) — PENDING
Plan: 0 of 0
Status: gap closure phase created from v1.3 milestone audit; plan Phase 51 next
Last activity: 2026-05-31 -- Phase 51 gap closure phase added from milestone audit

Progress: [#########-] 90%

## Performance Metrics

**Velocity:**

- Total plans completed: 10
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
| 48-50 | 4 | complete | - |
| 48 | 1 | complete | - |
| 49 | 2 | complete | - |
| 50 | 1 | complete | - |
| 51 | 0 | pending | - |

**Recent Trend:**

- Last 5 plans: 47-01, 48-01, 49-01, 49-02, 50-01
- Trend: v1.3 audit found one blocking integration gap; Phase 51 is pending.

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
- [phase 49]: v1.3 release claims are bounded by an explicit threat model, parity catalog surface, and deterministic release-boundary guard; public-network checks remain opt-in and outside default verification.
- [phase 51]: Milestone audit gap G-01 requires live-smoke progress snapshots to use fresh daemon sync status before v1.3 can archive.

### Pending Todos

- Plan Phase 51: Live Smoke Fresh Status Integration.

### Blockers/Concerns

- Phase 50 closed public-mainnet proof through diagnosed blocker evidence (`handshake_failure`) without claiming live header/block progress, but the v1.3 audit found the live-smoke snapshot loop is not yet wired to fresh daemon sync status.
- Default local verification must remain deterministic; public-network checks stay opt-in.
- `.planning/phases/` remains retained for v1.0 evidence referenced by parity docs.

## Session Continuity

Last session: 2026-05-28T03:16:08.621Z
Stopped at: Phase 51 added for v1.3 audit gap closure
Resume file: .planning/v1.3-MILESTONE-AUDIT.md
