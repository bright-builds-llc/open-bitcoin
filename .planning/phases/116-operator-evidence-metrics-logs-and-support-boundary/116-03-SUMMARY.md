---
phase: 116-operator-evidence-metrics-logs-and-support-boundary
plan: 116-03
subsystem: metrics-logs-runtime-counters
tags:
  - operator-evidence
  - metrics
  - logs
  - block-relay
requires:
  - 116-01
provides:
  - Fixed low-cardinality block-relay metric samples from shared status.
  - Sanitized structured block-relay log records with stable source and labels.
  - Runtime counter increments that feed metrics and log projections directly.
affects:
  - node-observability
  - managed-network
  - parity-breadcrumbs
tech-stack:
  added: []
  patterns:
    - Metrics and logs project from shared status instead of parsing log text.
    - Runtime effect sites increment fixed counter vocabularies only; no dynamic labels or peer-level dimensions.
key-files:
  created:
    - packages/open-bitcoin-node/src/metrics/block_relay.rs
  modified:
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-node/src/metrics/tests.rs
    - packages/open-bitcoin-node/src/logging.rs
    - packages/open-bitcoin-node/src/logging/tests.rs
    - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "The block-relay metric surface uses fixed `_count` samples for serving, announcement, reconstruction, fallback, malformed, timeout, and cleanup outcomes."
  - "Structured logs use the stable `block_relay` source and fixed label vocabulary from earlier block-serving and compact-relay phases."
  - "Runtime increments stay at effect sites so logs and metrics consume shared truth rather than reconstructing counters later."
requirements-completed:
  - OBS-03
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 116-2026-07-06T03-46-36
generated_at: 2026-07-06T05:05:00Z
---

# Phase 116 Plan 03: Block-Relay Metrics And Structured Logs Summary

Phase 116-03 added bounded observability for block serving and compact relay using the shared status contract established in Plan 01.

## Accomplishments

- Added fixed `MetricKind` entries and `block_relay_metric_samples()` projection for the Phase 116 counter set.
- Added `BLOCK_RELAY_LOG_SOURCE` and `block_relay_log_record()` so structured logs emit stable fixed labels and counts.
- Completed runtime counter increment wiring at block-serving and compact-relay effect sites that feed the shared evidence projection.
- Added focused tests for low-cardinality metrics, sanitized log output, and runtime counter projection behavior.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node block_relay -- --nocapture`

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking issue] File-length guard failures required module extraction**
- **Found during:** `bash scripts/verify.sh`
- **Issue:** `packages/open-bitcoin-node/src/network.rs`, `metrics.rs`, and `status.rs` exceeded the repo’s production Rust line-limit guard.
- **Fix:** Extracted `network/types.rs`, `metrics/block_relay.rs`, and `status/observability.rs` so the Phase 116 behavior stayed unchanged while the size guard passed.
- **Files modified:** `packages/open-bitcoin-node/src/network.rs`, `packages/open-bitcoin-node/src/network/types.rs`, `packages/open-bitcoin-node/src/metrics.rs`, `packages/open-bitcoin-node/src/metrics/block_relay.rs`, `packages/open-bitcoin-node/src/status.rs`, `packages/open-bitcoin-node/src/status/observability.rs`, `docs/parity/source-breadcrumbs.json`

## Self-Check

- Complete: runtime increments, fixed metrics, and structured logs all read the shared block-relay projection.
- Passed: focused node metrics, logging, block-relay, and file-length checks are green in the working tree.
