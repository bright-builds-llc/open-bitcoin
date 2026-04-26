---
gsd_state_version: 1.0
milestone: v1.1
milestone_name: Operator Runtime and Real-Network Sync
status: planning
stopped_at: Phase 16 context gathered
last_updated: "2026-04-26T21:53:01.475Z"
last_activity: 2026-04-26
progress:
  total_phases: 10
  completed_phases: 3
  total_plans: 13
  completed_plans: 13
  percent: 100
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-26)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** v1.1 Operator Runtime and Real-Network Sync - Phase 16 metrics, logs, and sync telemetry.

## Current Position

Phase: 16
Plan: Not started
Status: Ready to plan
Last activity: 2026-04-26

Progress: 3/10 phases complete

## Performance Metrics

**Velocity:**

- Total plans completed in v1.1: 13
- Average duration: not available yet
- Total execution time: not available yet

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 13 | 5 | - | - |
| 14 | 4 | - | - |
| 15 | 4 | - | - |
| 16 | TBD | - | - |
| 17 | TBD | - | - |
| 18 | TBD | - | - |
| 19 | TBD | - | - |
| 20 | TBD | - | - |
| 21 | TBD | - | - |
| 22 | TBD | - | - |

**Recent Trend:**

- Phase 13 completed 5 operator-runtime foundation plans on 2026-04-26.
- Phase 14 completed 4 durable-storage and recovery plans on 2026-04-26.
- Phase 15 completed 4 real-network sync loop plans on 2026-04-26.

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- [v1.0]: Use Bitcoin Knots `29.3.knots20260210` as the pinned behavioral baseline.
- [v1.0]: Keep the initial milestone headless and defer GUI work.
- [v1.0]: Use functional core / imperative shell boundaries with explicit I/O adapters.
- [v1.0]: Use Bazelisk and Bazel/Bzlmod for first-party workspace builds.
- [v1.0]: Treat reachable production panic-like sites as typed-error work instead of allowlisting them.
- [v1.0]: Guard first-party production Rust under `packages/open-bitcoin-*/src` in `bash scripts/verify.sh`, while excluding vendored Knots, build output, `tests.rs`, and inline `#[cfg(test)]` sections.
- [v1.0]: Preserve external Bitcoin, RPC, CLI, wallet, mempool, networking, and consensus behavior while replacing internal crashes with crate-local typed errors.
- [v1.1]: Build a terminal dashboard and rich status output before any desktop GUI work.
- [v1.1]: Treat migration from Bitcoin Core or Bitcoin Knots as explicit, dry-run-first, and backup-aware.
- [v1.1]: Keep macOS service lifecycle higher priority than Linux, while designing the interface so Linux systemd support fits the same command surface.

### Roadmap Evolution

- v1.0 Headless Parity archived on 2026-04-26.
- v1.1 Operator Runtime and Real-Network Sync starts at Phase 13, continuing phase numbering after the archived v1.0 milestone.

### Pending Todos

- 0 pending.

### Blockers/Concerns

- Durable database choice must be made deliberately before real-network sync relies on it.
- Migration flows must not mutate existing Core/Knots data until detection, explanation, backup, and dry-run behavior are implemented.
- The TUI dashboard depends on stable status, metrics, and sync-state projections; avoid building a decorative dashboard before those data contracts exist.

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 260424-7ow | Improve GitHub CI caching for Rust and Bazel verification | 2026-04-24 | c8a16bd | [260424-7ow-improve-github-ci-caching-for-rust-and-b](./quick/260424-7ow-improve-github-ci-caching-for-rust-and-b/) |
| 260424-jnn | Partial Clippy Panic-Lint Enforcement | 2026-04-24 | a547acd | [260424-jnn-partial-clippy-panic-lint-enforcement](./quick/260424-jnn-partial-clippy-panic-lint-enforcement/) |
| 260425-avx | Add deterministic LOC report generator and wire it into pre-commit/verify | 2026-04-25 | 9b09df8 | [260425-avx-add-deterministic-loc-report-generator-a](./quick/260425-avx-add-deterministic-loc-report-generator-a/) |
| 260425-c8x | Migrate LOC generator to TypeScript and Bun | 2026-04-25 | e8d1055 | [260425-c8x-migrate-loc-generator-to-typescript-and-](./quick/260425-c8x-migrate-loc-generator-to-typescript-and-/) |
| 260425-csn | Refresh README parity status and docs freshness guidance | 2026-04-25 | 37202da | [260425-csn-refresh-readme-parity-status-and-docs-fr](./quick/260425-csn-refresh-readme-parity-status-and-docs-fr/) |
| 260425-e1c | Fix CI Bun provisioning | 2026-04-25 | e51c539 | [260425-e1c-fix-ci-bun-provisioning](./quick/260425-e1c-fix-ci-bun-provisioning/) |
| 260425-kao | Add parity breadcrumb source anchors to first-party Rust files | 2026-04-25 | d2b67e3 | [260425-kao-add-parity-breadcrumb-source-anchors-to-](./quick/260425-kao-add-parity-breadcrumb-source-anchors-to-/) |
| 260425-kzh | Add repo rule requiring parity breadcrumbs for new Rust files | 2026-04-25 | d4e136d | [260425-kzh-add-repo-rule-requiring-parity-breadcrum](./quick/260425-kzh-add-repo-rule-requiring-parity-breadcrum/) |

## Session Continuity

Last session: 2026-04-26T21:53:01.470Z
Stopped at: Phase 16 context gathered
Resume file: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md
