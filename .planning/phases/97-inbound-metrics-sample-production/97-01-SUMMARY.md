---
phase: 97-inbound-metrics-sample-production
plan: 01
subsystem: metrics
tags: [rust, inbound, metrics, status, config]
requires:
  - phase: 90-inbound-listener-admission
    provides: Inbound admission counters in shared status.
  - phase: 91-peer-permissions
    provides: Permission class and effect evidence in shared inbound status.
  - phase: 94-dos-and-resource-governance
    provides: Inbound resource-governance counters in shared status.
provides:
  - Pure inbound status to MetricSample mapping.
  - Numeric status aggregates for inactive permission observations and permission validation failures.
affects: [inbound-metrics, status-contract, runtime-config]
tech-stack:
  added: []
  patterns: [status-derived-metrics, low-cardinality-counters, pure-mapper]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-node/src/metrics/tests.rs
    - packages/open-bitcoin-node/src/status/inbound.rs
    - packages/open-bitcoin-node/src/status/inbound/tests.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/config/loader.rs
    - packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
key-decisions:
  - "Keep inbound metric production pure and source it only from FieldAvailability<InboundPeerServingStatus>."
  - "Treat unavailable inbound status as an empty inbound sample set."
  - "Use inactive_permission_effect_observations instead of inactive permission label count."
requirements-completed: [INB-05, DOS-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 97-2026-06-28T16-11-36
generated_at: 2026-06-28T17:15:16Z
duration: 41min
completed: 2026-06-28
---

# Phase 97 Plan 01: Inbound Metric Mapping Summary

**Inbound status aggregates now map to fixed low-cardinality metric samples without adding labels or side effects.**

## Accomplishments

- Added `inbound_metric_samples` for all 23 inbound `MetricKind` variants.
- Added serde-defaulted `inactive_permission_effect_observations` and `permission_validation_failures` to `InboundPeerServingStatus`.
- Threaded inactive permission observation counts from managed admission info into the shared inbound status projection.
- Added a config validation aggregate helper for permission class parse and duplicate-address failures.

## Task Commits

Deferred until the wrapper-level clean verification gate. The user-invoked wrapper requires no commit or push before final verification is clean.

## Files Created/Modified

- `packages/open-bitcoin-node/src/metrics.rs` - Adds pure inbound sample mapping.
- `packages/open-bitcoin-node/src/metrics/tests.rs` - Pins all inbound sample mappings and unavailable-status behavior.
- `packages/open-bitcoin-node/src/status/inbound.rs` - Adds canonical numeric permission aggregates.
- `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` - Adds validation aggregate helper and unit test.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Projects admission/config aggregates into shared inbound status.

## Deviations from Plan

- The permission validation aggregate test lives inline in `config/loader/open_bitcoin_runtime.rs` instead of `config/tests.rs`, so it can directly exercise the private helper without widening production API.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node inbound_status_maps_to_each_fixed_inbound_metric_kind -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node inactive_permission_metric_uses_observation_count_not_label_count -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_permission_validation_failure_count_is_config_validation_aggregate -- --nocapture`

## User Setup Required

None.
