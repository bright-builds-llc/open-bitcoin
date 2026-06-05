---
gsd_state_version: 1.0
milestone: v1.4
milestone_name: Mainnet IBD Convergence and Peer Compatibility
status: complete
stopped_at: v1.4 milestone completed and archived
last_updated: "2026-06-05T22:32:09.384Z"
last_activity: 2026-06-05
progress:
  total_phases: 6
  completed_phases: 6
  total_plans: 15
  completed_plans: 15
  percent: 100
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-06-05)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Planning next milestone

## Current Position

Milestone: v1.4 Mainnet IBD Convergence and Peer Compatibility
Plan: Not started
Status: Milestone complete
Last activity: 2026-06-05

Progress: [##########] 100%

## Performance Metrics

**Velocity:**

- Total plans completed: 15
- Average duration: -
- Total execution time: -

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 54 | 1 | - | - |
| 55 | 1 | - | - |
| 56 | 1 | - | - |
| 57 | 4 | - | - |
| 58 | 3 | - | - |
| 59 | 5 | - | - |

**Recent Trend:**

- Last 5 plans: 59-01, 59-02, 59-03, 59-04, 59-05
- Trend: v1.4 closed with operator truth-surface consistency, redacted support evidence, repo-local docs, threat/release boundaries, and deterministic release-boundary checks.

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- [v1.3]: Keep public-mainnet evidence opt-in and outside the default `bash scripts/verify.sh` gate.
- [v1.3]: Preserve scope boundaries: no inbound serving, transaction relay, production-funds wallet, migration apply mode, packaging, hosted dashboard, GUI, or unattended production-node claim.
- [v1.4]: Scope the milestone to mainnet IBD convergence and public peer compatibility, while continuing to defer inbound serving, transaction relay, production-funds wallet, migration apply mode, packaging, hosted dashboard, GUI, and unattended production-node claims.
- [v1.4]: Continue phase numbering from v1.3; active milestone starts at Phase 54.
- [v1.4]: Skip broad ecosystem research for this milestone and use targeted Knots/protocol comparison during phase planning.
- [Phase 55]: Daemon sync records completed outbound handshakes as connected peers and surfaces duplicate-version peers as typed, uncredited compatibility failures.
- [Phase 56]: Header sync records bounded convergence stop reasons, supports optional target header height, and reports first-header-progress evidence in opt-in live smoke output.
- [Phase 57]: Block download/connect progress is bounded, peer-attributed, and visible through durable downloaded/connected height and hash evidence.
- [Phase 58]: Same-datadir restart/resume evidence is captured through deterministic durable-store tests and opt-in two-session live-smoke reporting.
- [Phase 59]: v1.4 release claims are bounded by operator evidence, support bundle redaction, threat-model roots, parity docs, and deterministic release-boundary checks.

### Pending Todos

- Start the next milestone with `/gsd-new-milestone`.
- Consider a future operator-facing CLI or script wrapper around the Phase 54 compatibility harness so contributors do not have to invoke the Rust harness path directly.

### Blockers/Concerns

- No active milestone blockers are recorded.
- Default local verification must remain deterministic; public-network checks stay opt-in UAT evidence.
- `.planning/phases/` retains raw v1.0, v1.3, and v1.4 evidence referenced by parity docs and milestone archives.

## Session Continuity

Last session: 2026-06-05T22:32:09.384Z
Stopped at: v1.4 milestone completed and archived
Resume file: .planning/MILESTONES.md
