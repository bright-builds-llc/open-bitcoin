---
phase: 16-metrics-logs-and-sync-telemetry
reviewed: 2026-04-26T23:30:18Z
depth: standard
files_reviewed: 13
files_reviewed_list:
  - docs/architecture/operator-observability.md
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-node/src/logging.rs
  - packages/open-bitcoin-node/src/logging/prune.rs
  - packages/open-bitcoin-node/src/logging/tests.rs
  - packages/open-bitcoin-node/src/logging/writer.rs
  - packages/open-bitcoin-node/src/metrics.rs
  - packages/open-bitcoin-node/src/status.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-node/src/sync/types.rs
findings:
  critical: 0
  warning: 4
  info: 0
  total: 4
status: issues_found
---

# Phase 16: Code Review Report

**Reviewed:** 2026-04-26T23:30:18Z
**Depth:** standard
**Files Reviewed:** 13
**Status:** issues_found

## Summary

Reviewed the listed Phase 16 observability, metrics, logging, storage, and sync files at standard depth. The review was informed by `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and the pinned Bright Builds standards pages for architecture, code shape, verification, testing, and Rust.

No critical security issues were found. The main risks are behavioral: metric retention does not enforce the advertised sample interval, `sync_until_idle` can stop while equal-sized progress continues, the public log rotation policy exposes an unsupported hourly state, and clean-shutdown metadata can become inconsistent with a surviving recovery marker after a partial failure.

## Warnings

### WR-01: Metric sample interval is advertised but not enforced

**File:** `packages/open-bitcoin-node/src/metrics.rs:84`
**Issue:** `MetricRetentionPolicy` exposes `sample_interval_seconds`, and the operator contract documents a 30 second sampling interval for a 24 hour / 2880 sample window. `append_and_prune_metric_samples` only filters by age and count; it never buckets, coalesces, or rejects samples that arrive more frequently than the policy interval. If the runtime records samples every second, the default 2880-sample cap retains about 48 minutes per series instead of the documented day-scale window.
**Fix:**
```rust
// Enforce interval buckets per kind before applying the count cap.
let bucket = sample.timestamp_unix_seconds / policy.sample_interval_seconds.max(1);
// Keep one sample per (kind, bucket), preferably the newest sample in that bucket,
// then apply max_age_seconds and max_samples_per_series.
```
Add a regression test that appends same-kind samples inside one interval and verifies only one sample for that interval is retained, plus a test that 2880 default buckets cover the intended 24 hour window.

### WR-02: `sync_until_idle` can stop while sync is still progressing

**File:** `packages/open-bitcoin-node/src/sync.rs:120`
**Issue:** The idle loop breaks when the current round's `messages_processed` equals the previous round's count. Equal message counts do not mean no progress: a peer can deliver one new header or block per round with the same number of messages each time. In that case the loop exits after the second equal-sized productive round and leaves additional queued progress unprocessed.
**Fix:**
```rust
let mut last_summary = self.sync_once(transport, timestamp)?;
for _ in 1..self.config.max_rounds {
    let previous_progress = (
        last_summary.best_header_height,
        last_summary.best_block_height,
    );
    last_summary = self.sync_once(transport, timestamp)?;
    let current_progress = (
        last_summary.best_header_height,
        last_summary.best_block_height,
    );
    if current_progress == previous_progress {
        break;
    }
}
```
Add a scripted transport test with three rounds that each process the same number of messages while advancing the header height, and assert that `sync_until_idle` does not stop after the second round.

### WR-03: Hourly log rotation is representable but silently ignored

**File:** `packages/open-bitcoin-node/src/logging.rs:111`
**Issue:** `LogRetentionPolicy` includes `LogRotation::Hourly`, but the writer and retention planner always use the daily `open-bitcoin-runtime-<unix_day>.jsonl` scheme. A caller can request hourly rotation and receive daily files with day-based retention, which violates the repo standard to avoid representable unsupported states and can mislead operators relying on tighter rotation.
**Fix:**
```rust
// If only daily rotation is supported in Phase 16, remove LogRotation::Hourly
// or reject it before writing logs. If hourly is intended, make file naming,
// managed-file parsing, age calculations, and tests rotation-aware.
```
Add coverage that either rejects `LogRotation::Hourly` explicitly or verifies hourly file buckets and retention behavior.

### WR-04: Clean-shutdown marker update is not atomic

**File:** `packages/open-bitcoin-node/src/storage/fjall_store.rs:297`
**Issue:** `mark_clean_shutdown` persists `last_clean_shutdown = true` before clearing the recovery marker. If metadata persistence succeeds but marker removal or durability fails, the store can reopen with metadata claiming a clean shutdown while `recovery_marker` still requests repair/reindex. That creates conflicting lifecycle signals for operators and recovery automation.
**Fix:**
```rust
// Persist the metadata update and recovery-marker removal in one runtime batch
// with the requested durability, or clear the marker first and only mark clean
// after the marker removal has succeeded.
```
Add a regression test around the intended invariant: after any successful clean shutdown, metadata is clean and no marker exists; after failure, the store must not present a clean shutdown alongside an uncleared marker.

---

_Reviewed: 2026-04-26T23:30:18Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
