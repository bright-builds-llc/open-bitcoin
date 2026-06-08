---
gsd_state_version: 1.0
milestone: v1.5
milestone_name: Unattended Mainnet Node Operation Readiness
status: executing
stopped_at: Phase 64 verified
last_updated: "2026-06-08T03:43:38.403Z"
last_activity: 2026-06-07
progress:
  total_phases: 8
  completed_phases: 5
  total_plans: 18
  completed_plans: 18
  percent: 100
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-06-05)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 65 — Support Bundle and Operator Review Docs

## Current Position

Milestone: v1.5 Unattended Mainnet Node Operation Readiness
Phase: 65
Plan: Not started
Status: Ready to plan Phase 65
Last activity: 2026-06-07

Progress: [##########] 100%

## Performance Metrics

**Velocity:**

- Current milestone plans completed: 18
- Current milestone plan count: 18
- Prior milestone plans completed: 15 in v1.4

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 60 | 1 | - | - |
| 61 | 6 | - | - |
| 62 | 4 | - | - |
| 63 | 4 | - | - |
| 64 | 3 | - | - |
| 65 | TBD | - | - |
| 66 | TBD | - | - |
| 67 | TBD | - | - |

**Recent Trend:**

- Last 5 completed plans: 59-02, 59-03, 59-04, 59-05, 60-01
- Trend: v1.5 completed Phases 60, 61, 62, 63, and 64; ready to plan Phase 65.

| Phase 61 P01 | 20m 23s | 2 tasks | 12 files |
| Phase 61-resource-bounds-and-recovery-taxonomy P02 | 23min | 3 tasks | 7 files |
| Phase 61-resource-bounds-and-recovery-taxonomy P04 | 31m 15s | 2 tasks | 7 files |
| Phase 61-resource-bounds-and-recovery-taxonomy P05 | 17m 54s | 2 tasks | 6 files |
| Phase 61-resource-bounds-and-recovery-taxonomy P03 | 33m 34s | 3 tasks | 7 files |
| Phase 61-resource-bounds-and-recovery-taxonomy P06 | 13m 4s | 3 tasks | 6 files |

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
- [Phase 64]: Service restart/resume status exposes selected datadir, clean versus unclean prior shutdown, durable progress, stale in-flight verdict, recovery category, and next-action guidance.
- [Phase 64]: Keep real service-manager restarts and public-network restart smoke as opt-in UAT evidence outside `bash scripts/verify.sh`.
- [Phase 59]: v1.4 release claims remain bounded by operator evidence, support bundle redaction, threat-model roots, parity docs, and deterministic release-boundary checks.
- [Phase 61]: Expose recovery categories as a typed serde enum while keeping recovery_action as separate human guidance.
- [Phase 61]: Default missing persisted recovery_category values to unavailable so older RuntimeMetadata remains readable.
- [Phase 61]: Register the new Rust status child module in parity breadcrumbs before committing to satisfy repo rules.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Storage action and StorageError categories take precedence over peer and network retry guidance.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Lock-contention classification uses word-boundary matching so unrelated words such as block do not match lock.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: The sync recovery Rust module is committed with its parity breadcrumb because AGENTS.md requires breadcrumbs for first-party Rust source files.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Use the ten Phase 61 recovery labels in live-smoke diagnosis output instead of the older v1.4 category set.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Expose support bundle resource pressure through explicit allowlisted keys only, leaving raw peer/report material omitted.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Keep public-network live smoke opt-in and absent from scripts/verify.sh.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Render SyncRecoveryCategory::as_str() directly in status, dashboard, and RPC warning surfaces.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Keep machine recovery category output separate from human recovery_action guidance.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Rename targeted renderer and RPC tests so plan acceptance filters execute real assertions.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Summary-derived status uses stop reason before latest peer recovery category; broad last-error parsing stays in durable runtime state.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Durable runtime recovery category precedence is storage metadata, last-error detail, stop reason, latest peer category, then clean or unclean shutdown metadata.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Structured sync progress logs carry recovery_category while preserving bounded message length through a 192-character summary-record cap.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: The sync recovery helper module is visible to the parent sync runtime so durable status uses the shared classifier instead of duplicating string logic.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Document sync.recovery_category as the stable machine label and keep sync.recovery_action as human next-action guidance.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Use a Bun deterministic checker to guard Phase 61 recovery labels, resource-pressure fields, RR-01 bound statements, repo-local commands, and public-network exclusion from default verification.
- [Phase 61-resource-bounds-and-recovery-taxonomy]: Keep public-network live-smoke, manual-peer, and restart-after-progress commands documented only as opt-in UAT, not default verification.

### Pending Todos

- Plan Phase 65 with `/gsd-plan-phase 65`.
- Carry the compatibility harness wrapper through Phase 66.

### Blockers/Concerns

- No active milestone blockers are recorded.
- Default local verification must remain deterministic; public-network checks stay opt-in UAT evidence.
- `.planning/phases/` retains raw v1.0, v1.3, and v1.4 evidence referenced by parity docs and milestone archives.

## Session Continuity

Last session: 2026-06-08T03:43:38.403Z
Stopped at: Phase 64 verified
Resume file: .planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-VERIFICATION.md
