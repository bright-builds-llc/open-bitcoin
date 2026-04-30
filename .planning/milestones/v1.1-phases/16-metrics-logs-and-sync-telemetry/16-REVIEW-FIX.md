---
phase: 16-metrics-logs-and-sync-telemetry
source_review: .planning/phases/16-metrics-logs-and-sync-telemetry/16-REVIEW.md
status: resolved
findings_addressed: [WR-01, WR-02, WR-03, WR-04]
generated_at: 2026-04-26T23:45:00Z
---

# Phase 16 Code Review Fix Summary

## Findings Addressed

- WR-01: `append_and_prune_metric_samples` now enforces `sample_interval_seconds` buckets before the per-series cap, preserving the newest sample per interval bucket.
- WR-02: `sync_until_idle` now compares chain progress by best header and block heights, so equal-sized productive rounds continue until height progress stops or `max_rounds` is reached.
- WR-03: `LogRotation::Hourly` was removed because Phase 16 only supports daily managed JSONL buckets.
- WR-04: `mark_clean_shutdown` now clears the recovery marker before persisting clean-shutdown metadata, avoiding a successful metadata claim while a marker remains uncleared.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::sync_until_idle_continues_equal_message_rounds_when_heights_advance` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::metrics_history_appends_across_reopen` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::recovery_marker_round_trips_and_clean_shutdown_clears_it` passed.
- `CARGO_BUILD_JOBS=1 cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` passed.
