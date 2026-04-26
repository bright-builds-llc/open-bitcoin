---
phase: 16-metrics-logs-and-sync-telemetry
plan: "01"
subsystem: observability
tags: [metrics, fjall, storage, retention, telemetry]

requires:
  - phase: 13-operator-runtime-foundations
    provides: Metrics retention and status contracts
  - phase: 14-durable-storage-and-recovery
    provides: FjallNodeStore metrics snapshot persistence
  - phase: 15-real-network-sync-loop
    provides: Runtime metrics producers for sync progress
provides:
  - Pure bounded metrics append-and-prune history helper
  - Fjall-backed metrics append API that preserves bounded history across restart
  - Metrics status availability constructors and storage status projection
affects: [OBS-03, metrics, storage, status, dashboard]

tech-stack:
  added: []
  patterns:
    - Functional-core metrics retention with thin Fjall persistence adapter
    - Snapshot-backed bounded metrics history in the existing metrics namespace

key-files:
  created:
    - .planning/phases/16-metrics-logs-and-sync-telemetry/16-01-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Metrics history remains snapshot-backed in the existing Fjall metrics namespace instead of introducing per-sample keys."
  - "Unavailable metrics history is reported through MetricsStatus with MetricKind::ALL metadata, not fake numeric samples."

patterns-established:
  - "append_and_prune_metric_samples combines existing and new samples before enforcing max-age and per-series caps."
  - "FjallNodeStore append APIs load, prune, save, and return a typed MetricsStorageSnapshot."

requirements-completed: [OBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 16-2026-04-26T21-50-05
generated_at: 2026-04-26T22:35:15Z

duration: 12 min
completed: 2026-04-26
---

# Phase 16 Plan 01: Bounded Fjall-Backed Metrics History Summary

**Bounded metrics history with pure retention logic and Fjall-backed restart persistence**

## Performance

- **Duration:** 12 min
- **Started:** 2026-04-26T22:23:36Z
- **Completed:** 2026-04-26T22:35:15Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Added `append_and_prune_metric_samples` with deterministic MetricKind ordering, max-age pruning, and per-series sample caps.
- Added `MetricsStatus::available` and `MetricsStatus::unavailable` constructors that preserve retention policy and `MetricKind::ALL`.
- Added `FjallNodeStore::append_metric_samples` and `FjallNodeStore::load_metrics_status` over the existing metrics snapshot namespace.
- Added restart, per-series cap, age-pruning, and unavailable-status tests for durable metrics history.

## Task Commits

1. **Task 1: Add pure bounded metrics history helpers** - `8bc8449` (feat)
2. **Task 2: Append and load bounded metrics through FjallNodeStore** - `dd80184` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/metrics.rs` - Pure metrics retention helper, status constructors, and unit tests.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Fjall append/load status APIs for bounded metrics history.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs` - Durable metrics history tests using isolated temp stores and deterministic timestamps.
- `docs/metrics/lines-of-code.md` - Hook-managed LOC report updated by the repo pre-commit hook.

## Decisions Made

- Kept metrics history snapshot-backed because the bounded maximum remains small and the existing metrics namespace already has a versioned snapshot codec.
- Represented missing metrics history through `MetricsStatus::unavailable` while preserving enabled-series metadata, avoiding fake samples for unavailable collectors.

## Deviations from Plan

### Process Adjustments

**1. [AGENTS.md Precedence] Deferred RED commits until GREEN**
- **Found during:** Task 1 and Task 2 TDD execution
- **Issue:** The plan requested TDD RED commits, but repo instructions require all Rust checks to pass before any commit.
- **Fix:** Wrote failing tests and verified RED failures, then committed only after implementation and verification were green.
- **Files modified:** No extra files beyond the planned task files.
- **Verification:** RED failures were observed before implementation; final task commits passed hook-enabled verification.
- **Committed in:** `8bc8449`, `dd80184`

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Constrained hook-time Cargo verification concurrency**
- **Found during:** Task 1 commit
- **Issue:** Two hook-enabled commit attempts hit transient `Resource temporarily unavailable` errors while `cargo test` listed test binaries.
- **Fix:** Retried normal hook-enabled commits with `CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1` so verification stayed enabled while reducing local resource pressure.
- **Files modified:** None.
- **Verification:** Hook-managed `scripts/verify.sh` completed successfully for both task commits.
- **Committed in:** `8bc8449`, `dd80184`

---

**Total deviations:** 2 handled (1 AGENTS.md process adjustment, 1 blocking verification fix)
**Impact on plan:** No scope change. The implementation and verification requirements were preserved.

## Issues Encountered

- Initial Task 1 commit attempts failed in the pre-commit hook due transient OS resource pressure during Cargo test listing; resolved without bypassing hooks.
- One targeted verification batch was accidentally started with parallel Cargo commands, causing file-lock waits; all commands completed successfully and subsequent verification ran sequentially.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` passed.
- Repo hook verification ran `scripts/verify.sh` successfully during both task commits.

## Known Stubs

None.

## Self-Check: PASSED

- Found summary file and all modified source/test files.
- Found task commits `8bc8449` and `dd80184` in git history.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 16 Plan 02 can build structured log writing and retention on top of the same status-facing evidence model. Metrics history is now durable and bounded for later status and dashboard consumers.

*Phase: 16-metrics-logs-and-sync-telemetry*
*Completed: 2026-04-26*
