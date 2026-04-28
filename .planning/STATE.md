---
gsd_state_version: 1.0
milestone: v1.1
milestone_name: Operator Runtime and Real-Network Sync
status: milestone_complete
stopped_at: Phase 22 verified complete
last_updated: "2026-04-28T01:57:12Z"
last_activity: 2026-04-27 -- Phase 22 completed and verified
progress:
  total_phases: 10
  completed_phases: 10
  total_plans: 38
  completed_plans: 38
  percent: 100
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-26)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Milestone closeout after Phase 22 verification

## Current Position

Phase: 22 (real-sync-benchmarks-and-release-hardening) — VERIFIED COMPLETE
Plan: 22-03 complete; 3/3 plans complete
Status: Milestone ready for `/gsd-verify-work`, `/gsd-secure-phase`, and `/gsd-audit-milestone`
Last activity: 2026-04-27 -- Phase 22 completed and verified

Progress: 10/10 phases complete

## Performance Metrics

**Velocity:**

- Total plans completed in v1.1: 38
- Average duration: not available yet
- Total execution time: not available yet

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 13 | 5 | - | - |
| 14 | 4 | - | - |
| 15 | 4 | - | - |
| 16 | 3 | - | - |
| 17 | 5 | - | - |
| 18 | 3 | - | - |
| 19 | 3 | - | - |
| 20 | 5 | - | - |
| 21 | 3 | - | - |
| 22 | 3 | - | - |

**Recent Trend:**

- Phase 13 completed 5 operator-runtime foundation plans on 2026-04-26.
- Phase 14 completed 4 durable-storage and recovery plans on 2026-04-26.
- Phase 15 completed 4 real-network sync loop plans on 2026-04-26.
- Phase 17 completed 5 CLI status and first-run onboarding plans on 2026-04-27.
- Phase 18 completed 3 service lifecycle integration plans on 2026-04-27.
- Phase 19 completed 3 ratatui dashboard plans on 2026-04-27.
- Phase 20 completed 5 wallet runtime expansion plans on 2026-04-27.
- Phase 21 completed 3 drop-in parity audit and migration plans on 2026-04-27.
- Phase 22 completed 3 real-sync benchmark and release-hardening plans on 2026-04-27.

| Phase 16 P01 | 12 min | 2 tasks | 4 files |
| Phase 16 P02 | 18 min | 2 tasks | 7 files |
| Phase 16-metrics-logs-and-sync-telemetry P03 | 19 min | 2 tasks | 5 files |
| Phase 20-wallet-runtime-expansion P01 | 9 min | 2 tasks | 6 files |
| Phase 20-wallet-runtime-expansion P02 | 17m | 2 tasks | 12 files |
| Phase 20-wallet-runtime-expansion P03 | 24m | 2 tasks | 12 files |
| Phase 20-wallet-runtime-expansion P04 | - | 2 tasks | 13 files |
| Phase 20-wallet-runtime-expansion P05 | - | 2 tasks | 12 files |

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
- [Phase 16]: Metrics history remains snapshot-backed in the existing Fjall metrics namespace instead of introducing per-sample keys.
- [Phase 16]: Unavailable metrics history is reported through MetricsStatus with MetricKind::ALL metadata, not fake numeric samples.
- [Phase 16]: Runtime logs use repo-owned line-delimited JSON with Unix-day filenames instead of tracing/appender dependencies.
- [Phase 16]: Log retention stays pure-planned and adapter-executed so pruning never selects unmanaged files.
- [Phase 16]: Recent warning and error access lives in open-bitcoin-node status contracts, not CLI/dashboard raw-file parsing.
- [Phase 16-metrics-logs-and-sync-telemetry]: Sync telemetry uses shared MetricSample, StructuredLogRecord, SyncStatus, PeerStatus, and HealthSignal contracts instead of a sync-only DTO. — Keeps status and dashboard consumers on the Phase 16 observability surface while satisfying SYNC-06.
- [Phase 16-metrics-logs-and-sync-telemetry]: Sync runtime appends final metric samples through FjallNodeStore::append_metric_samples with default retention. — Append history preserves bounded time-series evidence and avoids overwriting snapshots after Plan 16-01.
- [Phase 16-metrics-logs-and-sync-telemetry]: Sync structured log writing is optional via SyncRuntimeConfig::maybe_log_dir and log write failures become warning health signals. — Keeps default hermetic sync behavior unchanged while surfacing operator-visible logging failures.
- [Phase 20-wallet-runtime-expansion]: Persist ranged descriptor range/cursor state inside SingleKeyDescriptor and mirror it into DescriptorRecord.original_text to preserve node snapshot DTO compatibility.
- [Phase 20-wallet-runtime-expansion]: Recover ranged child indexes by matching derived scripts during rescan and signing instead of widening WalletUtxo or WalletSnapshot outside the plan write set.
- [Phase 20-wallet-runtime-expansion]: Persist wallet registry membership, selected-wallet metadata, and rescan checkpoints as separate records in the existing Fjall wallet namespace.
- [Phase 20-wallet-runtime-expansion]: Resume wallet rescans by replaying bounded height windows from durable chainstate snapshots and checkpoint after each chunk.
- [Phase 20-wallet-runtime-expansion]: Normalize stored #ob:: ranged-descriptor metadata during node snapshot decode so Plan 20-01 snapshots remain reloadable without expanding the plan write set.
- [Phase 20-wallet-runtime-expansion]: Keep wallet selection in transport metadata and URI routing instead of request JSON payloads.
- [Phase 20-wallet-runtime-expansion]: Preserve the typed GetWalletInfoResponse shape for downstream callers and append Phase 20 freshness metadata at JSON serialization time.
- [Phase 20-wallet-runtime-expansion]: Resolve conf_target and estimate_mode in the RPC shell into deterministic fee rates before reusing the shared build-and-sign spend path.
- [Phase 21-drop-in-parity-audit-and-migration]: Keep migration as a separate operator-owned `open-bitcoin migrate plan` surface instead of extending onboarding. — Preserves the target-setup wizard boundary while giving source-install review its own dry-run-first contract.
- [Phase 21-drop-in-parity-audit-and-migration]: Keep migration dry-run only in Phase 21 and require explicit `--source-datadir` selection when detection is ambiguous. — Protects existing Core/Knots datadirs, services, and wallets from implicit mutation or guessed source selection.
- [Phase 21-drop-in-parity-audit-and-migration]: Store migration-relevant deviations in `docs/parity/index.json` and surface only the relevant notices at runtime. — Keeps the CLI and parity ledger on one auditable machine-readable source of truth.

### Roadmap Evolution

- v1.0 Headless Parity archived on 2026-04-26.
- v1.1 Operator Runtime and Real-Network Sync starts at Phase 13, continuing phase numbering after the archived v1.0 milestone.

### Pending Todos

- 0 pending.

### Blockers/Concerns

- Durable database choice must be made deliberately before real-network sync relies on it.
- Automatic or destructive migration remains out of scope after Phase 21; later phases must preserve the current dry-run-first safety boundary until an apply-mode design is explicitly planned.
- The TUI dashboard depends on stable status, metrics, and sync-state projections; avoid building a decorative dashboard before those data contracts exist.
- `bash scripts/verify.sh` passes after the Phase 22 benchmark, parity-ledger, and release-hardening closeout run.

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

Last session: 2026-04-28T01:57:12Z
Stopped at: Phase 22 verified complete
Resume file: .planning/ROADMAP.md
