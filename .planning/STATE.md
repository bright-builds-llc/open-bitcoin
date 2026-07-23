---
gsd_state_version: 1.0
milestone: v2.2
milestone_name: Package Relay and Long-Lived Mempool Policy
status: ready_to_execute
stopped_at: Phase 130 planned — 13 plans ready
last_updated: "2026-07-23T17:05:13.364Z"
last_activity: 2026-07-23 — Revised Phase 130 planning blockers; 13 plans ready to execute.
progress:
  total_phases: 9
  completed_phases: 0
  total_plans: 13
  completed_plans: 0
  percent: 0
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-07-22 after starting milestone v2.2).

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 130 — Resource, Time, and Fee Primitives

## Current Position

Milestone: v2.2 Package Relay and Long-Lived Mempool Policy
Phase: 130 of 138 (1 of 9 milestone phases)
Plan: 0 of 13
Status: Ready to execute
Last activity: 2026-07-23 -- Phase 130 planning complete

Progress: [░░░░░░░░░░] 0%

Next action: Run /gsd-execute-phase 130

## Performance Metrics

**Current milestone:**

- Total plans completed: 0
- Average duration: —
- Total execution time: 0 hours

| Phase | Plans | Total | Avg/Plan |
| --- | ---: | ---: | ---: |
| 130–138 | 0 | TBD | — |

## Accumulated Context

### Decisions

- [v2.2 milestone]: Initialized the new milestone through `/gsd-new-milestone` after the archived v2.1 closeout.
- [v2.2 roadmap]: Use the research-backed nine-phase dependency order across Phases 130–138 at fine granularity.
- [v2.2 roadmap]: Assign PPKG-04 to Phase 136, where parent-before-child package fanout becomes an achieved transport behavior after the peer bridge and lifecycle authority exist.
- [v2.2 roadmap]: Keep package handling to local package APIs and bounded same-peer 1P1C assembly over ordinary transaction messages; add no general package wire protocol.
- [v2.2 roadmap]: Persist canonical entries, acceptance times, and surviving local unbroadcast membership, but rebuild derived state and reset the rolling fee on restart.
- [v2.2 roadmap]: Keep default verification deterministic and hermetic; public/default/production relay and guaranteed-propagation claims remain deferred.

### Pending Todos

- Keep historical `.planning/phases/` directories tracked because repository verifiers consume selected evidence.
- Keep repo-local Cargo and Bazel command forms in UAT guidance.
- Preserve existing explicit relay activation and public-network opt-in boundaries.

### Blockers/Concerns

- Phase 131 planning must define the deterministic Rust accounted-memory model and parity tolerance.
- Phase 132 planning must confirm scoped package RBF, TRUC, and ephemeral-dust prerequisites or narrow unsupported outcomes explicitly.
- Phase 135 planning must choose mempool-local snapshot compatibility, checkpoint cadence/strength, and the advertised crash-loss window.
- Phase 136 planning must specify the exact eligible-serve or successful-write receipt that clears unbroadcast membership.

## Latest Milestone Archive

- Roadmap: `.planning/milestones/v2.1-ROADMAP.md`
- Requirements: `.planning/milestones/v2.1-REQUIREMENTS.md`
- Audit: `.planning/milestones/v2.1-MILESTONE-AUDIT.md`

## Session Continuity

Last session: 2026-07-23T14:48:49.163Z
Stopped at: Phase 130 context gathered
Resume file: .planning/phases/130-resource-time-and-fee-primitives/130-CONTEXT.md
