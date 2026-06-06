---
gsd_state_version: 1.0
milestone: v1.5
milestone_name: Unattended Mainnet Node Operation Readiness
status: executing
stopped_at: Phase 61 context gathered
last_updated: "2026-06-06T04:36:00.034Z"
last_activity: 2026-06-06 -- Phase 61 execution started
progress:
  total_phases: 8
  completed_phases: 1
  total_plans: 7
  completed_plans: 1
  percent: 14
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-06-05)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 61 — Resource Bounds and Recovery Taxonomy

## Current Position

Milestone: v1.5 Unattended Mainnet Node Operation Readiness
Phase: 61 (Resource Bounds and Recovery Taxonomy) — EXECUTING
Plan: 1 of 6
Status: Executing Phase 61
Last activity: 2026-06-06 -- Phase 61 execution started

Progress: [#---------] 13%

## Performance Metrics

**Velocity:**

- Current milestone plans completed: 1
- Current milestone plan count: TBD during phase planning
- Prior milestone plans completed: 15 in v1.4

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 60 | 1 | - | - |
| 61 | TBD | - | - |
| 62 | TBD | - | - |
| 63 | TBD | - | - |
| 64 | TBD | - | - |
| 65 | TBD | - | - |
| 66 | TBD | - | - |
| 67 | TBD | - | - |

**Recent Trend:**

- Last 5 completed plans: 59-02, 59-03, 59-04, 59-05, 60-01
- Trend: v1.5 completed Phase 60 and is ready to plan Phase 61.

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- [v1.3]: Keep public-mainnet evidence opt-in and outside the default `bash scripts/verify.sh` gate.
- [v1.3]: Preserve scope boundaries: no inbound serving, transaction relay, production-funds wallet, migration apply mode, packaging, hosted dashboard, GUI, or unattended production-node claim.
- [v1.4]: Scope the milestone to mainnet IBD convergence and public peer compatibility, while continuing to defer inbound serving, transaction relay, production-funds wallet, migration apply mode, packaging, hosted dashboard, GUI, and unattended production-node claims.
- [v1.4]: Skip broad ecosystem research for this milestone and use targeted Knots/protocol comparison during phase planning.
- [v1.5]: Scope the milestone to bounded unattended mainnet node operation readiness, while continuing to defer inbound serving, transaction relay, production-funds wallet, migration apply mode, packaging polish, hosted dashboard, GUI, and broad production-node claims.
- [v1.5]: Continue phase numbering from v1.4; active milestone starts at Phase 60 and runs through Phase 67.
- [v1.5]: Keep public-network long-run and service checks opt-in UAT evidence, not default `bash scripts/verify.sh` checks.
- [v1.5]: Preserve `.planning/phases/` raw histories for v1.0, v1.3, and v1.4 parity and UAT traceability.
- [Phase 55]: Daemon sync records completed outbound handshakes as connected peers and surfaces duplicate-version peers as typed, uncredited compatibility failures.
- [Phase 56]: Header sync records bounded convergence stop reasons, supports optional target header height, and reports first-header-progress evidence in opt-in live smoke output.
- [Phase 57]: Block download/connect progress is bounded, peer-attributed, and visible through durable downloaded/connected height and hash evidence.
- [Phase 58]: Same-datadir restart/resume evidence is captured through deterministic durable-store tests and opt-in two-session live-smoke reporting.
- [Phase 59]: v1.4 release claims remain bounded by operator evidence, support bundle redaction, threat-model roots, parity docs, and deterministic release-boundary checks.

### Pending Todos

- Plan Phase 61 with `/gsd-plan-phase 61`.
- Carry the compatibility harness wrapper through Phase 66.

### Blockers/Concerns

- No active milestone blockers are recorded.
- Default local verification must remain deterministic; public-network checks stay opt-in UAT evidence.
- `.planning/phases/` retains raw v1.0, v1.3, and v1.4 evidence referenced by parity docs and milestone archives.

## Session Continuity

Last session: 2026-06-06T03:46:02.239Z
Stopped at: Phase 61 context gathered
Resume file: .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md
