---
gsd_state_version: 1.0
milestone: v2.2
milestone_name: Package Relay and Long-Lived Mempool Policy
status: executing
stopped_at: Completed 130-01-PLAN.md
last_updated: "2026-07-23T18:28:54.353Z"
last_activity: 2026-07-23 -- Completed Plan 130-01 resource accounting primitives
progress:
  total_phases: 9
  completed_phases: 0
  total_plans: 13
  completed_plans: 1
  percent: 8
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-07-22 after starting milestone v2.2).

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 130 — Resource, Time, and Fee Primitives

## Current Position

Milestone: v2.2 Package Relay and Long-Lived Mempool Policy
Phase: 130 (Resource, Time, and Fee Primitives) — EXECUTING
Plan: 2 of 13
Status: Ready to execute Plan 130-02
Last activity: 2026-07-23 -- Completed Plan 130-01 resource accounting primitives

Progress: [█░░░░░░░░░] 8%

Next action: Execute Plan 130-02

## Performance Metrics

**Current milestone:**

- Total plans completed: 1
- Average duration: 50 min
- Total execution time: 50 min

| Phase | Plans | Total | Avg/Plan |
| --- | ---: | ---: | ---: |
| 130–138 | 1 | 50 min | 50 min |

## Accumulated Context

### Decisions

- [v2.2 milestone]: Initialized the new milestone through `/gsd-new-milestone` after the archived v2.1 closeout.
- [v2.2 roadmap]: Use the research-backed nine-phase dependency order across Phases 130–138 at fine granularity.
- [v2.2 roadmap]: Assign PPKG-04 to Phase 136, where parent-before-child package fanout becomes an achieved transport behavior after the peer bridge and lifecycle authority exist.
- [v2.2 roadmap]: Keep package handling to local package APIs and bounded same-peer 1P1C assembly over ordinary transaction messages; add no general package wire protocol.
- [v2.2 roadmap]: Persist canonical entries, acceptance times, and surviving local unbroadcast membership, but rebuild derived state and reset the rolling fee on restart.
- [v2.2 roadmap]: Keep default verification deterministic and hermetic; public/default/production relay and guaranteed-propagation claims remain deferred.
- [Phase 130]: Use deterministic Rust-owned logical mempool accounting rather than C++ allocator estimates.
- [Phase 130]: Keep Phase 130 trimming exclusively on legacy vsize while reporting distinct accounted usage and capacity.
- [Phase 130]: Map resource arithmetic failures to MempoolError::InternalInvariant at mutation boundaries.

### Pending Todos

- Keep historical `.planning/phases/` directories tracked because repository verifiers consume selected evidence.
- Keep repo-local Cargo and Bazel command forms in UAT guidance.
- Preserve existing explicit relay activation and public-network opt-in boundaries.

### Blockers/Concerns

- Phase 131 planning must define accounted-memory enforcement and parity tolerance against the Plan 130 ledger.
- Phase 132 planning must confirm scoped package RBF, TRUC, and ephemeral-dust prerequisites or narrow unsupported outcomes explicitly.
- Phase 135 planning must choose mempool-local snapshot compatibility, checkpoint cadence/strength, and the advertised crash-loss window.
- Phase 136 planning must specify the exact eligible-serve or successful-write receipt that clears unbroadcast membership.

## Latest Milestone Archive

- Roadmap: `.planning/milestones/v2.1-ROADMAP.md`
- Requirements: `.planning/milestones/v2.1-REQUIREMENTS.md`
- Audit: `.planning/milestones/v2.1-MILESTONE-AUDIT.md`

## Session Continuity

Last session: 2026-07-23T18:28:54.351Z
Stopped at: Completed 130-01-PLAN.md
Resume file: None
