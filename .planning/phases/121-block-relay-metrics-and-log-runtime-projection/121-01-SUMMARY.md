---
phase: 121-block-relay-metrics-and-log-runtime-projection
plan: 01
subsystem: observability
tags: [block-relay, metrics, structured-logs, DurableSyncRuntime, FieldAvailability, OBS-03]

requires:
  - phase: 116-operator-evidence-metrics-logs-and-support-boundary
    provides: block_relay_metric_samples + block_relay_log_record helpers
  - phase: 97-inbound-metrics-sample-production
    provides: DurableSyncRuntime inbound provider + persist_metrics extension pattern
provides:
  - set_block_relay_metric_status_provider with Available-gated persist_metrics append
  - write_block_relay_log on the same sync tick as persist_metrics / write_summary_logs
  - DurableSyncRuntime tests for available/unavailable/unset metrics and log leakage
affects:
  - 121-02 open-bitcoind production wiring and Phase 121 checker

tech-stack:
  added: []
  patterns:
    - Shared maybe_block_relay_metric_status_provider for metrics and logs
    - Call-site FieldAvailability::Available gate before block_relay_metric_samples (asymmetric vs inbound helper)

key-files:
  created:
    - packages/open-bitcoin-node/src/sync/waiting.rs
    - packages/open-bitcoin-node/src/sync/runtime_state/helpers.rs
  modified:
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/metrics.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "One shared provider field for metrics and structured logs (no twin setter)"
  - "Gate block_relay_metric_samples at Available match only — never call on Unavailable / default_unavailable wrap"
  - "write_block_relay_log runs on the same progress tick after write_summary_logs"

patterns-established:
  - "Pattern: Available-gated family append in persist_metrics for helpers that always emit N samples"
  - "Pattern: Shared provider drives both metric samples and structured-log emission"

requirements-completed: []  # OBS-03 closeout owned by 121-02 (production wiring + checker)
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 121-2026-07-14T04-25-57
generated_at: 2026-07-14T06:11:09Z

duration: 3min
completed: 2026-07-14
---

# Phase 121 Plan 01: Block Relay Runtime Projection Summary

**DurableSyncRuntime now persists block-relay MetricKind samples and emits sanitized `block_relay` structured logs when a shared provider returns Available status.**

## Performance

- **Duration:** 3 min
- **Started:** 2026-07-14T06:07:51Z
- **Completed:** 2026-07-14T06:11:09Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added `set_block_relay_metric_status_provider` and Available-gated `persist_metrics` append of nine block-relay MetricKinds alongside sync samples.
- Added `write_block_relay_log` reusing `block_relay_log_record` + `append_structured_record`, wired on the same sync tick as `persist_metrics` / `write_summary_logs`.
- Runtime tests cover Available append, Unavailable/unset omission, log emit/omit, and sensitive-marker leakage.

## Task Commits

Each task was committed atomically:

1. **Task 1: Provider setter + Available-gated persist_metrics** - (combined feat commit with Task 2)
2. **Task 2: write_block_relay_log + tick wiring + leakage tests** - (combined feat commit with Task 1)

**Plan metadata:** included in the same gsd-tools commit as code + SUMMARY

_Note: User-requested gsd-tools commit batches code + SUMMARY to avoid duplicate full verify hook runs._

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync.rs` — provider field + tick call to `write_block_relay_log`
- `packages/open-bitcoin-node/src/sync/metrics.rs` — setter + Available-gated persist append
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` — `write_block_relay_log`
- `packages/open-bitcoin-node/src/lib.rs` — crate-root re-export of `block_relay_metric_samples`
- `packages/open-bitcoin-node/src/sync/tests.rs` — six runtime proofs (3 metrics + 3 logs)

## Decisions Made

- Shared provider for metrics and logs (discretion from CONTEXT).
- Call-site Available gate required because `block_relay_metric_samples` always emits nine samples (D-04 / research Pitfall 1).
- Helpers and MetricKinds left unchanged (D-05/D-08).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Critical] Deferred OBS-03 REQUIREMENTS closeout to Plan 02**
- **Found during:** State update after Plan 01
- **Issue:** Auto `requirements mark-complete OBS-03` would mark the requirement Complete before open-bitcoind wiring and Phase 121 checker land
- **Fix:** Reverted REQUIREMENTS.md OBS-03 to Pending; Plan 02 owns closeout
- **Files modified:** `.planning/REQUIREMENTS.md` (reverted)
- **Commit:** N/A (not committed as Complete)

**2. [Rule 3 - Blocking] Split sync modules to satisfy 628-line production file limit**
- **Found during:** Pre-commit verify after feat commit attempt
- **Issue:** `sync.rs` (632) and `runtime_state.rs` (628) exceeded the production Rust line limit after Plan 01 additions
- **Fix:** Extracted `record_waiting_outcome` to `sync/waiting.rs` and helpers to `sync/runtime_state/helpers.rs`
- **Files modified:** `sync.rs`, `sync/waiting.rs`, `sync/runtime_state.rs`, `sync/runtime_state/helpers.rs`
- **Commit:** same as Plan 01 feat commit

## Verification Results

```text
cargo test -p open-bitcoin-node persist_metrics_appends_block_relay → 1 passed
cargo test -p open-bitcoin-node persist_metrics_omits_block_relay → 2 passed
cargo test -p open-bitcoin-node write_block_relay_log → 3 passed
```

## Known Stubs

None.

## Threat Flags

None — no new network endpoints, auth paths, or trust-boundary schemas beyond the plan threat model (provider → persist/log gate already mitigated).

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-node/src/sync/metrics.rs (`set_block_relay_metric_status_provider`)
- FOUND: packages/open-bitcoin-node/src/sync/runtime_state.rs (`write_block_relay_log`)
- FOUND: .planning/phases/121-block-relay-metrics-and-log-runtime-projection/121-01-SUMMARY.md
