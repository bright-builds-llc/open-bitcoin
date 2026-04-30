---
phase: 16-metrics-logs-and-sync-telemetry
verified: 2026-04-26T23:46:26Z
status: passed
score: 13/13 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 16-2026-04-26T21-50-05
generated_at: 2026-04-26T23:46:26Z
lifecycle_validated: true
overrides_applied: 0
deferred:
  - truth: "Actual dashboard panel rendering for SYNC-06 is downstream work, not a Phase 16 deliverable."
    addressed_in: "Phase 19"
    evidence: "Phase 19 goal: Provide a useful local terminal dashboard for live node operation, sync progress, metrics, logs, wallet summary, and safe actions."
  - truth: "Live service-manager restart collection is downstream work; Phase 16 provides the bounded metric kind and storage path."
    addressed_in: "Phase 18"
    evidence: "Phase 18 goal: Let operators manage Open Bitcoin as a supervised daemon on macOS and Linux without losing inspectability or safety."
---

# Phase 16: Metrics, Logs, and Sync Telemetry Verification Report

**Phase Goal:** Give operators and later dashboard work durable, bounded, and explainable runtime evidence.
**Verified:** 2026-04-26T23:46:26Z
**Status:** passed
**Re-verification:** No - initial verification. No prior `16-VERIFICATION.md` existed.

## Goal Achievement

Phase 16 achieved the runtime evidence layer promised by the roadmap. The implementation provides bounded, restart-persistent metrics history, managed structured JSONL logs with deterministic retention, status-facing recent warning/error queries, and sync telemetry projected through shared metrics/log/status/health contracts. Review warnings WR-01 through WR-04 were verified closed by commit `145154d`.

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | The node records bounded historical metrics for sync height, header height, peer counts, mempool size, wallet summary, disk usage, RPC health, and service restarts. | VERIFIED | `MetricKind::ALL` defines all 8 canonical series in `metrics.rs:23`; `append_and_prune_metric_samples` enforces max age, interval buckets, per-series caps, and deterministic ordering in `metrics.rs:86`; Fjall appends bounded snapshots in `fjall_store.rs:200`. Live service restart collection is deferred to Phase 18, per Phase 16 context. |
| 2 | Structured logs rotate with documented retention and status-visible paths. | VERIFIED | `LogRetentionPolicy` defaults to daily/14 files/14 days/268435456 bytes in `logging.rs:124`; writer uses `open-bitcoin-runtime-<unix_day>.jsonl` in `logging/writer.rs:57`; retention planner enforces max age, count, and byte caps in `logging/prune.rs:41`; docs record the same path contract in `operator-observability.md:13`. |
| 3 | Recent warnings/errors can be queried through status-facing APIs without opening raw log files manually. | VERIFIED | `load_log_status` returns `LogStatus` with bounded `recent_signals` in `logging/writer.rs:40`; `recent_log_signals_from_records` filters warn/error records and bounds the result in `logging.rs:182`; `health_signals_from_recent_logs` maps them into shared `HealthSignal` values in `logging.rs:204`. |
| 4 | Sync bottlenecks and health signals are visible through metrics and logs without changing consensus or network behavior. | VERIFIED | `SyncRunSummary::metric_samples`, `structured_log_records`, and `sync_status` project sync evidence in `sync/types.rs:238`, `sync/types.rs:264`, and `sync/types.rs:284`; runtime writes through shared adapters in `sync.rs:271` and `sync.rs:287`; retry/stall/storage/network health evidence is covered by sync tests. |
| 5 | The node shell records bounded metrics history for every canonical `MetricKind` series. | VERIFIED | Pure retention iterates `MetricKind::ALL` at `metrics.rs:96`; tests cover all series, caps, age, ordering, and interval buckets in `metrics.rs:248` through `metrics.rs:371`. |
| 6 | Metrics history survives store reopen instead of being overwritten by each sync progress write. | VERIFIED | `FjallNodeStore::append_metric_samples` loads existing samples before saving in `fjall_store.rs:207`; `metrics_history_appends_across_reopen` proves restart persistence in `fjall_store/tests.rs:252`; sync runtime uses append, not direct overwrite, in `sync.rs:277`. |
| 7 | Status-facing metrics metadata exposes retention and enabled-series information when live collectors are unavailable. | VERIFIED | `MetricsStatus::available` and `unavailable` preserve retention and `MetricKind::ALL` in `metrics.rs:148`; `load_metrics_status` reports explicit unavailable status for missing snapshots in `fjall_store.rs:226`; tests assert this in `fjall_store/tests.rs:387`. |
| 8 | Runtime log records are structured Open Bitcoin-owned data with level, message, timestamp, and source. | VERIFIED | `StructuredLogRecord` has `level`, `source`, `message`, and `timestamp_unix_seconds` in `logging.rs:28`; serialization test is at `logging/tests.rs:80`. |
| 9 | Managed structured logs rotate daily and prune deterministically by max files, max age, and total retained bytes. | VERIFIED | Writer buckets by Unix day in `logging/writer.rs:57`; planner ignores unmanaged files and applies all three caps in `logging/prune.rs:41`; tests cover max files, max age, and byte cap in `logging/tests.rs:232`, `logging/tests.rs:255`, and `logging/tests.rs:277`. |
| 10 | Status-facing callers can query bounded recent warnings and errors without parsing raw log files themselves. | VERIFIED | `load_log_status` reads managed files and returns `LogStatus.recent_signals` in `logging/writer.rs:40`; bounded query test passes at `logging/tests.rs:181`. |
| 11 | Sync progress and bottlenecks are visible through shared metrics, logs, status, and health signal contracts. | VERIFIED | `SyncProgress` includes message/header/block counters in `status.rs:72`; summary projections feed metrics/log/status in `sync/types.rs:238`; runtime adapter calls are in `sync.rs:277` and `sync.rs:316`. |
| 12 | Sync telemetry records attempted, connected, failed, retried, and stalled peer evidence without changing consensus or network behavior. | VERIFIED | `SyncRunSummary` records attempted/connected/failed counters and outcomes in `sync/types.rs:208`; retry attempts are preserved in `sync.rs:136`; stalled peers add warning health evidence in `sync.rs:218`; tests cover retry and stall behavior in `sync/tests.rs:562` and `sync/tests.rs:595`. |
| 13 | Default sync telemetry tests remain hermetic. | VERIFIED | Tests use `ScriptedTransport`, isolated temp stores, deterministic timestamps, and temp log dirs in `sync/tests.rs:38`, `sync/tests.rs:112`, and telemetry tests; live smoke remains ignored at `sync/tests.rs:960`. |

**Score:** 13/13 truths verified

### Deferred Items

| # | Item | Addressed In | Evidence |
|---|------|-------------|----------|
| 1 | Actual dashboard panel rendering for SYNC-06 | Phase 19 | Phase 19 goal explicitly covers terminal dashboard sync progress, metrics, and logs. Phase 16 supplies the shared evidence layer consumed by that work. |
| 2 | Live service-manager restart collection | Phase 18 | Phase 16 context defers launchd/systemd restart collection to service lifecycle work; Phase 16 keeps `MetricKind::ServiceRestarts` and bounded storage ready. |

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `packages/open-bitcoin-node/src/metrics.rs` | Pure metrics kinds, retention, append/prune, status constructors | VERIFIED | Exists, substantive, pure; no filesystem/Fjall dependency found. |
| `packages/open-bitcoin-node/src/storage/fjall_store.rs` | Fjall append/load metrics history APIs | VERIFIED | `append_metric_samples` calls pure retention and persists `MetricsStorageSnapshot`; clean-shutdown ordering fixed. |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs` | Metrics restart/retention tests | VERIFIED | Covers reopen, per-series cap, max age, unavailable status, and recovery marker clearing. |
| `packages/open-bitcoin-node/src/logging.rs` | Structured records, recent signals, status mapping | VERIFIED | Owns `StructuredLogRecord`, `RecentLogSignal`, `LogStatus`, and health mapping. |
| `packages/open-bitcoin-node/src/logging/writer.rs` | Managed JSONL append/read adapter | VERIFIED | Writes JSONL, prunes after append, reads managed files into status. |
| `packages/open-bitcoin-node/src/logging/prune.rs` | Pure retention planner | VERIFIED | Selects only managed files and enforces max files, age, and bytes. |
| `packages/open-bitcoin-node/src/logging/tests.rs` | Structured logging and retention tests | VERIFIED | Covers serialization, bounded recent queries, missing log dirs, and retention caps. |
| `packages/open-bitcoin-node/src/status.rs` | Shared status counters | VERIFIED | `SyncProgress` exposes messages, headers, and blocks. |
| `packages/open-bitcoin-node/src/sync/types.rs` | Sync projections into shared contracts | VERIFIED | Provides metric samples, structured records, status, peer status, and health signals. |
| `packages/open-bitcoin-node/src/sync.rs` | Runtime metrics/log wiring | VERIFIED | Persists metrics through append API and writes optional structured logs. |
| `packages/open-bitcoin-node/src/sync/tests.rs` | Hermetic telemetry regression tests | VERIFIED | Covers metrics history, logs, stalls, retries, storage/network health, idle detection. |
| `docs/architecture/operator-observability.md` | Operator-facing retention/path contract | VERIFIED | Documents default metrics/log retention and `open-bitcoin-runtime-<unix_day>.jsonl`. |
| `docs/parity/source-breadcrumbs.json` | Breadcrumb coverage | VERIFIED | New logging files listed under `node-observability-contracts`. |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `fjall_store.rs` | `metrics.rs` | `append_and_prune_metric_samples` | VERIFIED | gsd-tools key-link check passed; manual trace at `fjall_store.rs:211`. |
| `fjall_store.rs` | `snapshot_codec.rs` | `MetricsStorageSnapshot` | VERIFIED | Persisted through existing metrics snapshot codec. |
| `logging/writer.rs` | `logging/prune.rs` | `plan_log_retention` after append | VERIFIED | gsd-tools key-link check passed; manual trace at `logging/writer.rs:75`. |
| `logging.rs` | `status.rs` | `HealthSignal` mapping | VERIFIED | `health_signals_from_recent_logs` maps recent log signals into shared status health signals. |
| `sync.rs` | `fjall_store.rs` | `append_metric_samples` | VERIFIED | Runtime persists `summary.metric_samples(timestamp)` through bounded Fjall append. |
| `sync.rs` | `logging/writer.rs` | `append_structured_log_record` | VERIFIED | Runtime writes optional structured sync logs when `maybe_log_dir` is configured. |
| `sync/types.rs` | `status.rs` | `sync_status` and `health_signal` | VERIFIED | Summary projections use shared `SyncStatus`, `PeerStatus`, and `HealthSignal`, with no `SyncTelemetry` DTO. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| `metrics.rs` / `fjall_store.rs` | `MetricSample` history | Runtime samples or caller samples passed to `append_metric_samples` | Yes | FLOWING - pure retention preserves bounded real samples and Fjall stores/reloads them. |
| `sync.rs` | `summary.metric_samples(timestamp)` | `SyncRunSummary` populated by scripted/real peer sync outcomes | Yes | FLOWING - sync runtime appends header height, sync height, and peer count metrics. |
| `logging/writer.rs` | `LogStatus.recent_signals` | Managed JSONL `StructuredLogRecord` files | Yes | FLOWING - reader deserializes managed files and returns bounded warning/error signals. |
| `sync/types.rs` | `structured_log_records` | `SyncRunSummary` outcomes, counters, and health signals | Yes | FLOWING - records include counters, stalls, failures, retries, storage/network health. |
| `status.rs` | `SyncProgress` counters | `SyncRunSummary::sync_status` | Yes | FLOWING - status exposes best heights, progress ratio, messages, headers, and blocks. |
| `fjall_store.rs` | `MetricsStatus` metadata | Presence/absence of persisted metrics snapshot | Yes | FLOWING - status explicitly reports available/unavailable with retention and enabled series. |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Metrics interval buckets are enforced | `CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics::tests::append_and_prune_metric_samples_enforces_sample_interval_buckets` | 1 passed | PASS |
| Recent warnings/errors load through `LogStatus` | `CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::load_log_status_reads_bounded_recent_signals` | 1 passed | PASS |
| Sync counters appear in status and structured logs | `CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::sync_status_and_log_records_include_message_header_block_counters` | 1 passed | PASS |
| Sync idle detection uses progress, not equal message count | `CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::sync_until_idle_continues_equal_message_rounds_when_heights_advance` | 1 passed | PASS |
| Clean shutdown clears recovery marker | `CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::recovery_marker_round_trips_and_clean_shutdown_clears_it` | 1 passed | PASS |

Full post-fix verification is also recorded in `16-REVIEW-FIX.md`: `CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 bash scripts/verify.sh` completed successfully after commit `145154d`.

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| OBS-03 | 16-01, 16-03 | Bounded historical metrics for canonical node/runtime series | SATISFIED | `MetricKind::ALL`, interval bucket retention, Fjall append/reopen tests, and sync runtime metric appends. |
| OBS-04 | 16-02 | Structured logs with rotation, retention, and status-visible locations | SATISFIED | JSONL managed writer, daily Unix-day bucket, documented path, deterministic retention planner. |
| OBS-05 | 16-02, 16-03 | Operators can inspect recent warnings/errors without manually opening raw logs | SATISFIED | `load_log_status`, bounded `recent_signals`, health signal mapping, sync warning/error log tests. |
| SYNC-06 | 16-03 | Sync progress and bottlenecks visible through status, metrics history, logs, and dashboard-ready contracts | SATISFIED | Sync counters/status, metrics history, structured logs, retry/stall/storage/network health evidence; dashboard rendering deferred to Phase 19. |

No orphaned Phase 16 requirements were found in `.planning/REQUIREMENTS.md`; the phase maps to OBS-03, OBS-04, OBS-05, and SYNC-06.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---|---|---|---|
| - | - | None blocking | - | Stub scans found no TODO/FIXME/placeholders, no console-only implementation, and no hollow status/rendering paths. Benign empty match arms were only in tests or exhaustive enum handling. |

### Human Verification Required

None. The phase delivers library/runtime contracts with deterministic tests and no visual, external-service, or live-network behavior required for default verification.

### Gaps Summary

No blocking gaps found. The two non-Phase-16 surfaces identified during goal-backward verification are explicitly downstream: actual dashboard panel rendering belongs to Phase 19, and live service-manager restart event collection belongs to Phase 18. Phase 16 provides the durable, bounded, status-facing evidence contracts those later phases need.

---

_Verified: 2026-04-26T23:46:26Z_
_Verifier: the agent (gsd-verifier)_
