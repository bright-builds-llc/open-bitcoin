---
phase: 137-rpc-and-sanitized-operator-evidence
plan: 08
subsystem: observability
tags: [mempool, metrics, structured-logs, low-cardinality, snapshot-labels]

requires:
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Identifier-free pressure, eviction, checkpoint, recovery, retry, and admission groups on MempoolStatus
provides:
  - Fixed MetricKind values matching Open Bitcoin snapshot labels
  - mempool_policy_metric_samples wired from persist_inbound_metrics_once and DurableSyncRuntime::persist_metrics
  - Allowlisted mempool_policy structured log records with snapshot-matching keys
affects:
  - 137-09 and later operator evidence consumers of the same labels

tech-stack:
  added: []
  patterns:
    - Sample only Available mempool groups; omit unavailable groups instead of recording 0
    - Structured logs use a fixed allowlisted source and key set with no identities

key-files:
  created:
    - packages/open-bitcoin-node/src/metrics/mempool_policy.rs
    - packages/open-bitcoin-node/src/metrics/inbound.rs
    - packages/open-bitcoin-node/src/logging/mempool_policy.rs
  modified:
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-node/src/metrics/tests/contracts.rs
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-node/src/sync/metrics.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs
    - packages/open-bitcoin-node/src/logging.rs
    - packages/open-bitcoin-node/src/logging/tests.rs
    - packages/open-bitcoin-node/src/logging/tests/redaction.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Moved inbound_metric_samples into metrics/inbound.rs so MetricKind growth stays under the 628-line gate."
  - "Derived Ord on MetricKind so mempool_policy_metric_samples can return BTreeMap."
  - "Assembled MempoolStatus from ManagedNetworkOperatorSnapshot because snapshot.mempool() is ManagedMempoolInfo."
  - "Exported mempool_policy_log_record without a new persist hook because relay_mempool_log_record has no production writer."

patterns-established:
  - "Pattern 1: Snapshot-matching metric names stay off the eight-chart dashboard strip."
  - "Pattern 2: Log decay values are allowlisted to half_life_12h / half_life_6h / half_life_3h / not_decaying."

requirements-completed: [MPOBS-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-19T23:44:41Z

duration: 7min
completed: 2026-08-19
---

# Phase 137 Plan 08: Mempool Policy Metrics and Logs Summary

**Fixed low-cardinality MetricKind values and an allowlisted `mempool_policy` log builder now use the same snapshot labels as status, without dashboard chart binding or identifier cardinality.**

## Performance

- **Duration:** 7 min
- **Started:** 2026-08-19T23:37:47Z
- **Completed:** 2026-08-19T23:44:41Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Added 14 Open Bitcoin `MetricKind` values for resources, fee floors, pressure, checkpoint overdue, recovery recovered count, retry, and admission.
- Sampled only Available `MempoolStatus` groups and wired `mempool_policy_metric_samples` into both existing persist collectors.
- Added `MEMPOOL_POLICY_LOG_SOURCE` plus snapshot-matching `key=value` logs with no txids, Knots aliases, or propagation words.
- Left `MAX_DASHBOARD_CHARTS` at 8 and kept Phase 105 `Relay*` kinds unchanged.

## Task Commits

Each task was committed atomically:

1. **Task 1 RED: failing metric-kind tests** - `0f520ec6` (test)
2. **Task 1 GREEN: implement MetricKind variants and sampling** - `57c6b2ad` (feat)
3. **Task 2 RED: failing structured-log tests** - `e6286bb1` (test)
4. **Task 2 GREEN: implement mempool_policy log records** - `20111436` (feat)

**Plan metadata:** pending docs commit for this SUMMARY

_Note: TDD tasks produced RED then GREEN commits; no refactor commit was needed._

## Files Created/Modified

- `packages/open-bitcoin-node/src/metrics.rs` - New MetricKind variants, ALL length 74, Ord derive, re-exports
- `packages/open-bitcoin-node/src/metrics/mempool_policy.rs` - Available-group sampler and operator-snapshot assembler
- `packages/open-bitcoin-node/src/metrics/inbound.rs` - Extracted inbound samples to keep metrics.rs under 628 lines
- `packages/open-bitcoin-node/src/metrics/tests/contracts.rs` - Named metric-kind and skip-unavailable tests
- `packages/open-bitcoin-node/src/lib.rs` - Re-export sampler next to `relay_metric_samples`
- `packages/open-bitcoin-node/src/sync/metrics.rs` - `DurableSyncRuntime::persist_metrics` extend
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs` - `persist_inbound_metrics_once` extend
- `packages/open-bitcoin-node/src/logging.rs` - `MEMPOOL_POLICY_LOG_SOURCE` and re-export
- `packages/open-bitcoin-node/src/logging/mempool_policy.rs` - Allowlisted log record builder
- `packages/open-bitcoin-node/src/logging/tests.rs` - Import new source and builder
- `packages/open-bitcoin-node/src/logging/tests/redaction.rs` - Named source, key, hex, and alias tests
- `docs/parity/source-breadcrumbs.json` - Register new first-party files

## Decisions Made

- New mempool-policy sampling lives in `metrics/mempool_policy.rs`; inbound samples moved out so `metrics.rs` stays at 516 lines.
- `MetricKind` now implements `Ord` so the planned `BTreeMap<MetricKind, u64>` return type compiles.
- Persist sites assemble `MempoolStatus` through `mempool_status_from_operator_snapshot` because `snapshot.mempool()` is still `ManagedMempoolInfo`.
- The log builder is exported only. `relay_mempool_log_record` has no production caller, so no second log filesystem or invented persist hook was added.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Extracted inbound metric samples from metrics.rs**
- **Found during:** Task 1 (GREEN)
- **Issue:** Adding 14 kinds would push `metrics.rs` past the 628-line gate.
- **Fix:** Moved `inbound_metric_samples` to `metrics/inbound.rs`.
- **Files modified:** `packages/open-bitcoin-node/src/metrics.rs`, `packages/open-bitcoin-node/src/metrics/inbound.rs`
- **Verification:** named `mempool_policy_metric` tests pass; `metrics.rs` is 516 lines
- **Committed in:** `57c6b2ad`

**2. [Rule 3 - Blocking] Derived Ord on MetricKind**
- **Found during:** Task 1 (GREEN)
- **Issue:** Planned `BTreeMap<MetricKind, u64>` requires `Ord`.
- **Fix:** Added `PartialOrd, Ord` to the `MetricKind` derive list.
- **Files modified:** `packages/open-bitcoin-node/src/metrics.rs`
- **Verification:** sampler compiles and skip-unavailable test passes
- **Committed in:** `57c6b2ad`

**3. [Rule 3 - Blocking] Assembled MempoolStatus from the operator snapshot**
- **Found during:** Task 1 (GREEN)
- **Issue:** Plan 04/05 left `snapshot.mempool()` as `ManagedMempoolInfo`; groups live on `ManagedNetworkOperatorSnapshot`.
- **Fix:** Added `mempool_status_from_operator_snapshot` and used it at both persist sites.
- **Files modified:** `packages/open-bitcoin-node/src/metrics/mempool_policy.rs`, `packages/open-bitcoin-node/src/sync/metrics.rs`, `packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs`
- **Verification:** both persist sites call `mempool_policy_metric_samples`
- **Committed in:** `57c6b2ad`

**4. [Rule 2 - Missing Critical] Registered new first-party files in the breadcrumb map**
- **Found during:** Task 1 and Task 2
- **Issue:** New Rust sources require a parity-breadcrumb mapping.
- **Fix:** Added inbound, mempool-policy metrics, and mempool-policy logging paths to `node-observability-contracts`.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** mappings list the new paths
- **Committed in:** `57c6b2ad`, `20111436`

---

**Total deviations:** 4 auto-fixed (3 blocking, 1 missing critical)
**Impact on plan:** Required for compile, file-length policy, persist wiring, and breadcrumb policy. No scope creep.

## Issues Encountered

- `relay_mempool_log_record` is test-only today, so Task 2 exported the builder instead of adding a sibling persist write.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Metrics and logs share the Plan 05 snapshot vocabulary.
- Dashboard sparkline binding stays at eight charts for later plans.
- A future persist hook can call `mempool_policy_log_record` if/when `relay_mempool` logs gain a production writer.

## Self-Check: PASSED

---
*Phase: 137-rpc-and-sanitized-operator-evidence*
*Completed: 2026-08-19*
---
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-20T00:10:00Z
---
