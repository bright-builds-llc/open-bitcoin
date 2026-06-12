---
gsd_state_version: 1.0
milestone: v1.6
milestone_name: Mainnet Full-Sync Completion
status: executing
stopped_at: Phase 70 plan 03 complete
last_updated: "2026-06-12T20:49:33.000Z"
last_activity: 2026-06-12
progress:
  total_phases: 7
  completed_phases: 2
  total_plans: 14
  completed_plans: 11
  percent: 78
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-06-11)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 70 — Reorg, Peer Rotation, and No-Progress Recovery

## Current Position

Milestone: v1.6 Mainnet Full-Sync Completion
Phase: 70 (Reorg, Peer Rotation, and No-Progress Recovery) — EXECUTING
Plan: 4 of 6
Status: Ready to execute
Last activity: 2026-06-12

Progress: [#####-----] 50%

## Performance Metrics

**Velocity:**

- Current milestone plans completed: 11
- Current milestone plan count: 14
- Prior milestone plans completed: 22 in v1.5

**By Phase:**

| Phase | Plans | Status |
|-------|------:|--------|
| 68. Full Active-Chain Validation and Durable Persistence | 3/3 | Complete |
| 69. Tip Tracking and Stay-Current Operation | 5/5 | Complete |
| 70. Reorg, Peer Rotation, and No-Progress Recovery | 3/6 | In Progress |
| 71. Resource Bounds and Durable Restart/Resume | 0/0 | Pending |
| 72. Operator Observability and Support Evidence | 0/0 | Pending |
| 73. Opt-In UAT and Deterministic Verification | 0/0 | Pending |
| 74. Release Boundaries, Parity, and Documentation | 0/0 | Pending |

**Recent Trend:**

- v1.5 completed Phases 60 through 67 and archived with a passed milestone audit.
- v1.6 started with a focus on full mainnet sync-to-tip completion.
- v1.6 roadmap covers 26 requirements across Phases 68 through 74.
- Phase 68 completed full active-chain validation and durable persistence.
- Phase 69 completed tip tracking, stay-current status, post-catch-up coverage, and documentation/checker closeout.
- Phase 70 plans 01 through 03 completed reorg status contracts, typed branch/reorg runtime outcomes, durable latest reorg projection, deterministic storage-blocker coverage, and peer attribution/backoff/rotation coverage.
- Next workflow: execute Phase 70 plan 04.

## Accumulated Context

### Decisions

Decisions are logged in `PROJECT.md` Key Decisions table. Recent v1.5 decisions:

- [v1.6]: Scope the milestone to explicit opt-in `open-bitcoind` mainnet full-sync completion before inbound serving, relay, production-funds wallet, migration apply mode, packaging polish, hosted dashboard, GUI, or broad production-node claims.
- [v1.6]: Continue keeping public-network full-sync and service checks opt-in UAT evidence unless a future phase deliberately changes the deterministic verification contract.
- [v1.6]: Continue phase numbering from v1.5; active milestone starts at Phase 68 and runs through Phase 74.
- [v1.6]: Map every active v1.6 requirement to exactly one roadmap phase with Pending traceability before execution starts.
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

- Execute Phase 70 plan 04 for the no-progress status contract.
- Keep public-network full-sync UAT opt-in and outside default deterministic verification.

### Blockers/Concerns

- No active milestone blockers are recorded.
- Default local verification must remain deterministic; public-network full-sync checks stay opt-in UAT evidence unless a future milestone or phase explicitly changes that contract.
- `.planning/phases/` retains raw v1.0, v1.3, v1.4, and v1.5 evidence referenced by parity docs and milestone archives.

## Session Continuity

Last session: 2026-06-12T20:49:33.000Z
Stopped at: Phase 70 plan 03 complete
Resume file: .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-04-PLAN.md
