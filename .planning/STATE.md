---
gsd_state_version: 1.0
milestone: v1.7
milestone_name: Full-Sync Soak and Recovery Hardening
status: executing
stopped_at: Completed 78-05 deterministic progress guarantee tests
last_updated: "2026-06-17T10:41:21.878Z"
last_activity: 2026-06-17
progress:
  total_phases: 6
  completed_phases: 3
  total_plans: 26
  completed_plans: 25
  percent: 96
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-06-14)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 78 — progress-guarantees-and-stall-diagnosis

## Current Position

Milestone: v1.7 Full-Sync Soak and Recovery Hardening
Phase: 78 (progress-guarantees-and-stall-diagnosis) — EXECUTING
Plan: 7 of 7
Status: Ready to execute
Last activity: 2026-06-17

Progress: [########--] 81%

## Performance Metrics

**Velocity:**

- Current milestone plans completed: 21
- Current milestone plan count: 26
- Prior milestone plans completed: 27 in v1.6

**By Phase:**

| Phase | Plans | Status |
|-------|------:|--------|
| 75. Multi-Day Soak Runner and Evidence Ledger | 6/6 | Complete |
| 76. Disk and Resource Bound Enforcement | 6/6 | Complete |
| 77. Corruption and Lock Recovery Hardening | 7/7 | Complete |
| 78. Progress Guarantees and Stall Diagnosis | 2/7 | In Progress |
| 79. Diagnostics and Support Bundle Forensics | 0/0 | Pending |
| 80. Opt-In Soak UAT and Release Boundaries | 0/0 | Pending |

**Recent Trend:**

- v1.5 completed Phases 60 through 67 and archived with a passed milestone audit.
- v1.6 completed Phases 68 through 74 and shipped explicit opt-in mainnet full-sync completion evidence.
- v1.7 starts with a focus on multi-day soak stability, resource and disk bounds, corruption and lock recovery, progress guarantees, diagnostics, and support bundles.
- Public-network soak and real service-manager evidence remain opt-in UAT outside default deterministic verification.

## Accumulated Context

### Decisions

Decisions are logged in `PROJECT.md` Key Decisions table. Recent decisions:

- [v1.7]: Scope the milestone to explicit opt-in full-sync soak and recovery hardening, while continuing to defer inbound serving, relay, production-funds wallet, migration apply mode, packaging polish, hosted dashboard, GUI, and broad production-node claims.
- [v1.7]: Keep multi-day public-network soak runs opt-in UAT evidence; default `bash scripts/verify.sh` must remain deterministic, public-network-free, service-manager-free, and free of wall-clock multi-day gates.
- [v1.7]: Continue phase numbering from v1.6; active milestone starts at Phase 75 and runs through Phase 80.
- [v1.7]: Map every active v1.7 requirement to exactly one roadmap phase with Pending traceability before execution starts.
- [v1.6]: Scope the milestone to explicit opt-in `open-bitcoind` mainnet full-sync completion before inbound serving, relay, production-funds wallet, migration apply mode, packaging polish, hosted dashboard, GUI, or broad production-node claims.
- [v1.6]: Continue keeping public-network full-sync and service checks opt-in UAT evidence unless a future phase deliberately changes the deterministic verification contract.
- [Phase 74-release-boundaries-parity-and-documentation]: v1.6 is documented as source-built, explicit opt-in full-sync completion evidence, not broad production-node readiness.
- [Phase 74-release-boundaries-parity-and-documentation]: The deterministic v1.6 checker guards parity roots, all 26 requirement ids, README/runtime-guide wording, deferred-scope docs, and default-verification exclusions.

### Pending Todos

- Execute remaining Phase 78 progress guarantees and stall diagnosis plans.
- Keep raw phase histories in `.planning/phases/` for parity and UAT traceability.
- Keep public-network full-sync and multi-day soak UAT opt-in and outside default deterministic verification unless a future phase deliberately changes that contract.

### Blockers/Concerns

- No active milestone blockers are recorded.
- Default local verification must remain deterministic; public-network full-sync and multi-day soak checks stay opt-in UAT evidence unless a future milestone or phase explicitly changes that contract.
- `.planning/phases/` retains raw v1.0, v1.3, v1.4, v1.5, and v1.6 evidence referenced by parity docs and milestone archives.

## Session Continuity

Last session: 2026-06-17T10:41:21.875Z
Stopped at: Completed 78-05 deterministic progress guarantee tests
Resume file: .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-05-SUMMARY.md
