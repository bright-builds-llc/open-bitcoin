---
gsd_state_version: 1.0
milestone: v1.3
milestone_name: Public Mainnet Sync Proof and Node Hardening
status: planning
stopped_at: Phase 49 complete; ready for Phase 50 context
last_updated: "2026-05-27T22:30:12Z"
last_activity: 2026-05-27
progress:
  total_phases: 9
  completed_phases: 8
  total_plans: 9
  completed_plans: 9
  percent: 89
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-05-24)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 50 — Public Mainnet Progress Evidence Closeout

## Current Position

Phase: 50
Plan: Not started
Status: Ready to gather Phase 50 context
Last activity: 2026-05-27

Progress: [#########-] 89%

## Performance Metrics

**Velocity:**

- Total plans completed: 9
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
| 48 | 1 | - | - |
| 49 | 2 | - | - |

**Recent Trend:**

- Last 5 plans: 46-01, 47-01, 48-01, 49-01, 49-02
- Trend: Phase 49 complete; Phase 50 remains.

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

### Pending Todos

- 0 pending.

### Blockers/Concerns

- Public-network proof depends on operator environment and may close with diagnosed network/environment blockers if live progress cannot be observed.
- Default local verification must remain deterministic; public-network checks stay opt-in.
- `.planning/phases/` remains retained for v1.0 evidence referenced by parity docs.

## Session Continuity

Last session: 2026-05-27T22:30:12Z
Stopped at: Phase 49 complete; ready for Phase 50 context
Resume file: .planning/ROADMAP.md
