---
phase: 97-inbound-metrics-sample-production
plan: 03
subsystem: operator-surfaces
tags: [rust, dashboard, status, support, docs]
requires:
  - phase: 97-inbound-metrics-sample-production
    plan: 01
    provides: Fixed inbound MetricKind sample mapping.
  - phase: 97-inbound-metrics-sample-production
    plan: 02
    provides: Retained inbound metrics history.
provides:
  - Bounded dashboard chart selection for retained inbound samples.
  - Live status projection of RPC-retained metric samples from `openbitcoinnetworkstatus`.
  - Status/support proof that retained inbound samples preserve kind/value/timestamp only.
affects: [dashboard, status-json, support-bundle, operator-docs]
tech-stack:
  added: []
  patterns: [bounded-dashboard-selector, retained-metric-evidence, no-new-ui]
key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs
  modified:
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - docs/architecture/operator-observability.md
    - docs/operator/runtime-guide.md
key-decisions:
  - "Keep the live dashboard row capped at MAX_DASHBOARD_CHARTS = 8."
  - "Substitute retained inbound series into optional dashboard tail slots only when samples exist."
  - "Document retained inbound metrics as local evidence only, with no relay or production claims."
requirements-completed: [INB-05, DOS-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 97-2026-06-28T16-11-36
generated_at: 2026-06-28T17:15:16Z
duration: 24min
completed: 2026-06-28
---

# Phase 97 Plan 03: Operator Evidence Surface Summary

**Retained inbound samples now show up through existing dashboard, status, and support evidence paths without adding a new UI surface.**

## Accomplishments

- Added a bounded dashboard selector in `dashboard/model/metrics.rs` that keeps eight charts and swaps retained inbound samples into optional tail slots.
- Added dashboard coverage for retained inbound admits, resource pressure, and reconnect suppression samples.
- Added status JSON coverage for inbound metric sample kind/value/timestamp without dynamic labels.
- Extended `openbitcoinnetworkstatus` and live CLI status collection so retained metrics come from the daemon RPC context when available.
- Added support store-health coverage proving retained inbound samples survive in support evidence.
- Updated operator docs with the exact closed flow from `InboundPeerServingStatus` to retained history.

## Task Commits

Deferred until the wrapper-level clean verification gate.

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Adds bounded inbound chart selection.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs` - Keeps dashboard metric selection focused and below the file-length gate.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Adds retained inbound chart coverage.
- `packages/open-bitcoin-cli/src/operator/status.rs` and `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Map retained metrics from `openbitcoinnetworkstatus` into live status snapshots.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Adds support metrics history coverage.
- `packages/open-bitcoin-node/src/status/tests.rs` - Adds status JSON retained sample coverage.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` and `packages/open-bitcoin-rpc/src/method/node.rs` - Include retained metrics in the Open Bitcoin network status RPC response.
- `docs/architecture/operator-observability.md` and `docs/operator/runtime-guide.md` - Document bounded retained inbound metric history.

## Deviations from Plan

None.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_charts_render_retained_inbound_metric_samples_without_expanding_row -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli fake_live_rpc_maps_metrics_from_open_bitcoin_network_status -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support_bundle_preserves_retained_inbound_metric_samples -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node status_metrics_json_preserves_retained_inbound_samples_without_dynamic_labels -- --nocapture`

## User Setup Required

None.
