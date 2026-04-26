---
phase: 16-metrics-logs-and-sync-telemetry
plan: "02"
subsystem: observability
tags: [logging, jsonl, retention, status, health]

requires:
  - phase: 13-operator-runtime-foundations
    provides: Metrics, logs, and status contracts for operator-facing runtime evidence
  - phase: 16-metrics-logs-and-sync-telemetry
    provides: Bounded metrics/status evidence model from Plan 01
provides:
  - Structured runtime log records with source-scoped recent warning and error queries
  - Managed JSONL writer and reader using Unix-day daily buckets
  - Pure deterministic log retention planner for max files, max age, and total bytes
affects: [OBS-04, OBS-05, logs, status, dashboard]

tech-stack:
  added: []
  patterns:
    - Repo-owned JSONL log adapter without tracing_appender or subscriber dependencies
    - Pure retention planner with a thin filesystem shell for append, load, and prune
    - Status-facing recent warning and error signals derived from structured records

key-files:
  created:
    - .planning/phases/16-metrics-logs-and-sync-telemetry/16-02-SUMMARY.md
    - packages/open-bitcoin-node/src/logging/prune.rs
    - packages/open-bitcoin-node/src/logging/writer.rs
    - packages/open-bitcoin-node/src/logging/tests.rs
  modified:
    - docs/architecture/operator-observability.md
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-node/src/logging.rs

key-decisions:
  - "Runtime logs use repo-owned line-delimited JSON with Unix-day filenames instead of tracing/appender dependencies."
  - "Log retention stays pure-planned and adapter-executed so pruning never selects unmanaged files."
  - "Recent warning and error access lives in open-bitcoin-node status contracts, not CLI/dashboard raw-file parsing."

patterns-established:
  - "plan_log_retention enforces max files, max age, and total bytes over managed file metadata."
  - "append_structured_log_record writes one JSON record per line and prunes after append."
  - "load_log_status returns bounded source-scoped warning and error signals."

requirements-completed: [OBS-04, OBS-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 16-2026-04-26T21-50-05
generated_at: 2026-04-26T22:56:45Z

duration: 18 min
completed: 2026-04-26
---

# Phase 16 Plan 02: Structured Log Writing, Retention, and Recent Signal Queries Summary

**Structured JSONL runtime logs with deterministic retention pruning and source-scoped recent warning/error status signals**

## Performance

- **Duration:** 18 min
- **Started:** 2026-04-26T22:38:32Z
- **Completed:** 2026-04-26T22:56:45Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Added `StructuredLogRecord` and source-scoped recent warning/error projection into `LogStatus`.
- Added managed JSONL append and load APIs using `open-bitcoin-runtime-<unix_day>.jsonl` daily buckets.
- Added pure retention planning for max files, max age, and total byte budgets while ignoring unmanaged files.
- Documented the runtime log path contract and registered required parity breadcrumbs for new Rust files.

## Task Commits

1. **Task 1: Add structured records and status mapping** - `b7f5d2b` (feat)
2. **Task 2: Add JSONL writer, reader, and retention planner** - `5ce96ae` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/logging.rs` - Structured record type, status signal projection, and module wiring.
- `packages/open-bitcoin-node/src/logging/tests.rs` - Structured record, status mapping, writer, reader, and retention tests.
- `packages/open-bitcoin-node/src/logging/writer.rs` - Filesystem JSONL append/load shell and retention application.
- `packages/open-bitcoin-node/src/logging/prune.rs` - Pure managed-log retention planner.
- `docs/architecture/operator-observability.md` - Runtime JSONL file contract for operators and downstream consumers.
- `docs/parity/source-breadcrumbs.json` - Breadcrumb entries for new first-party Rust files.
- `docs/metrics/lines-of-code.md` - Hook-managed LOC report updated by the repo pre-commit hook.

## Decisions Made

- Kept structured logging inside `open-bitcoin-node` with repo-owned JSONL serialization so the runtime path remains dependency-light and auditable.
- Planned retention over metadata first, then applied deletions from the filesystem shell, which keeps retention deterministic and prevents unmanaged files from being selected.
- Returned unavailable log status for missing log directories instead of synthesizing fake warning/error records.

## Deviations from Plan

### Process Adjustments

**1. [AGENTS.md Precedence] Deferred RED commits until GREEN**
- **Found during:** Task 1 and Task 2 TDD execution
- **Issue:** The plan requested TDD RED commits, but repo instructions require all Rust checks to pass before any commit.
- **Fix:** Wrote failing tests and verified RED failures, then committed only after implementation and verification were green.
- **Files modified:** No extra files beyond the planned task files.
- **Verification:** RED failures were observed before implementation; final task commits passed hook-enabled verification.
- **Committed in:** `b7f5d2b`, `5ce96ae`

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added required parity breadcrumb coverage**
- **Found during:** Task 1
- **Issue:** New first-party Rust test/source files require `docs/parity/source-breadcrumbs.json` entries under repo instructions.
- **Fix:** Registered the new logging test, writer, and prune files under the observability contract breadcrumb.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- **Committed in:** `b7f5d2b`, `5ce96ae`

**2. [Rule 1 - Bug] Fixed package clippy failure in retention planner**
- **Found during:** Task 2 verification
- **Issue:** `cargo clippy` reported a redundant closure in `logging/prune.rs` with `-D warnings`.
- **Fix:** Replaced the closure with the function item so clippy passed cleanly.
- **Files modified:** `packages/open-bitcoin-node/src/logging/prune.rs`
- **Verification:** `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` passed.
- **Committed in:** `5ce96ae`

**3. [Rule 1 - Cleanup] Removed stale placeholder wording before summary**
- **Found during:** Pre-summary stub scan
- **Issue:** A doc comment still described recent log signals as a placeholder even though the implementation was complete.
- **Fix:** Updated the comment to describe the actual warning/error signal contract.
- **Files modified:** `packages/open-bitcoin-node/src/logging.rs`
- **Verification:** Full Rust pre-commit verification and final plan checks passed after the cleanup.
- **Committed in:** `5ce96ae`

---

**Total deviations:** 4 handled (1 AGENTS.md process adjustment, 1 required breadcrumb adjustment, 2 verification/stub-scan fixes)
**Impact on plan:** No scope change. The log contract, retention behavior, and verification requirements were preserved.

## Issues Encountered

- The Task 2 RED run failed as expected because `logging::writer` and `logging::prune` did not exist yet.
- The first Task 2 clippy pass caught a redundant closure; it was fixed before committing.
- The pre-summary stub scan found stale comment wording only; no runtime stub or unwired UI/data path remained.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- `CARGO_BUILD_JOBS=1 cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` passed.
- Full Rust pre-commit sequence passed before task commits: `cargo fmt --manifest-path packages/Cargo.toml --all`, `CARGO_BUILD_JOBS=1 cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`, `CARGO_BUILD_JOBS=1 cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`, and `CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 cargo test --manifest-path packages/Cargo.toml --all-features`.
- Repo hook verification ran `scripts/verify.sh` successfully during both task commits.

## Known Stubs

None.

## Self-Check: PASSED

- Found summary file and all created Rust source/test files.
- Found task commits `b7f5d2b` and `5ce96ae` in git history.
- Stub-pattern scan found no remaining placeholder/TODO/FIXME or empty-value rendering stubs in the plan's changed implementation files.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 16 Plan 03 can consume structured status-facing log evidence without parsing unmanaged log files. Metrics history and recent log signals now provide bounded local evidence for operator status and dashboard surfaces.

---
*Phase: 16-metrics-logs-and-sync-telemetry*
*Completed: 2026-04-26*
