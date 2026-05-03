---
status: complete
phase: 16-metrics-logs-and-sync-telemetry
source:
  - 16-01-SUMMARY.md
  - 16-02-SUMMARY.md
  - 16-03-SUMMARY.md
started: 2026-05-03T13:08:15Z
updated: 2026-05-03T13:09:48Z
---

## Current Test

[testing complete]

## Tests

### 1. Phase 16 verification report
expected: Open `.planning/milestones/v1.1-phases/16-metrics-logs-and-sync-telemetry/16-VERIFICATION.md`. It should show `status: passed` with `score: 13/13 must-haves verified`, and the verified truths should cover bounded restart-persistent metrics history, managed structured log rotation/retention, status-facing recent warning/error queries, and sync telemetry plus health evidence. The deferred items should be limited to dashboard rendering in Phase 19 and live service-manager restart collection in Phase 18.
result: pass

### 2. Metrics history survives store reopen
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::metrics_history_appends_across_reopen -- --exact`. It should pass and prove metrics history appends across store reopen instead of being overwritten on the next write.
result: pass

### 3. Missing metrics snapshot reports explicit unavailable status
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::missing_metrics_snapshot_reports_unavailable_status -- --exact`. It should pass and prove `load_metrics_status` returns explicit unavailable metadata and guidance when no metrics snapshot has been recorded yet.
result: pass

### 4. Structured log retention matches the operator contract
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::default_log_retention_matches_operator_contract -- --exact` and open `docs/architecture/operator-observability.md`. The test and doc should agree on daily rotation, `14` files, `14` days, `268435456` bytes, and the managed `open-bitcoin-runtime-<unix_day>.jsonl` naming scheme.
result: pass

### 5. Recent warnings and errors are queryable through log status
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::load_log_status_reads_bounded_recent_signals -- --exact`. It should pass and prove recent warning/error signals can be loaded through `LogStatus` without manually parsing raw JSONL files.
result: pass

### 6. Sync status and structured logs expose progress counters
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::sync_status_and_log_records_include_message_header_block_counters -- --exact`. It should pass and prove sync telemetry exposes message, header, and block counters through shared status and structured log records.
result: pass

### 7. Stalled peers surface warning health evidence
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::stalled_peer_emits_warning_health_signal_and_log_record -- --exact`. It should pass and prove a stalled peer produces both a warning health signal and a structured log record instead of silently disappearing from operator-visible telemetry.
result: pass

## Summary

total: 7
passed: 7
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
