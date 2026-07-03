---
phase: 108-durable-mempool-relay-state-recovery
plan: 03
subsystem: operator-evidence
tags:
  - relay-evidence
  - metrics
  - logs
  - support-bundle
requires:
  - phase: 108-durable-mempool-relay-state-recovery
    provides: Plan 108-01 managed recovery summary
provides:
  - Shared `RelayRecoveryCounters`
  - CLI, dashboard, and support rendering for `Relay recovery`
  - Fixed recovery metric kinds and structured log keys
  - Support redaction for sensitive recovery reasons
affects:
  - packages/open-bitcoin-node/src/status/relay_evidence.rs
  - packages/open-bitcoin-node/src/metrics.rs
  - packages/open-bitcoin-node/src/logging.rs
  - packages/open-bitcoin-cli/src/operator/status/render/relay.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs
  - packages/open-bitcoin-cli/src/operator/support/render/relay.rs
  - packages/open-bitcoin-cli/src/operator/support/redaction.rs
tech-stack:
  added: []
  patterns:
    - Fixed low-cardinality recovery counters
    - Shared status projection reused by CLI, dashboard, support, metrics, and logs
key-files:
  modified:
    - packages/open-bitcoin-node/src/status/relay_evidence.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-node/src/metrics/tests.rs
    - packages/open-bitcoin-node/src/logging.rs
    - packages/open-bitcoin-node/src/logging/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render/relay.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/render/relay.rs
    - packages/open-bitcoin-cli/src/operator/support/redaction.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
key-decisions:
  - "Recovery evidence is fixed aggregate counters, not raw transaction, peer, endpoint, permission, or storage text."
  - "Baseline-compatible RPC response structs remain unchanged; Open Bitcoin status/support surfaces carry recovery evidence."
  - "Unavailable recovery reasons are redacted through the existing relay/mempool support sanitizer."
requirements-completed:
  - MEM-06
  - REL-01
  - REL-02
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 108-2026-07-03T14-09-06
generated_at: 2026-07-03T15:32:37Z
completed: 2026-07-03
---

# Phase 108 Plan 03 Summary

Recovered relay state now appears in operator evidence as sanitized fixed counters.

## Accomplishments

- Added `RelayRecoveryCounters` to `RelayEvidenceStatus` with implemented zero defaults.
- Projected latest managed recovery summaries into relay evidence, and marked recovery counters unavailable when storage recovery evidence exists.
- Rendered `Relay recovery` in operator status, dashboard rows, and support bundle Markdown.
- Added fixed metrics for all six recovery counters.
- Added fixed structured log fields for recovery counts.
- Extended support redaction so hostile recovery reasons become `redacted_relay_mempool_evidence`.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node relay_evidence_status_projects_recovery_counters -- --nocapture` - passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node relay_metric_kinds_are_low_cardinality_counters -- --nocapture` - passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node relay_mempool_log_record_uses_fixed_outcome_counts -- --nocapture` - passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli relay_evidence -- --nocapture` - passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support_bundle -- --nocapture` - passed.

## Deviations

- The generated plan mentioned a baseline RPC omission test. The implementation leaves baseline RPC structs untouched and validates recovery evidence through shared status, CLI, metrics, logging, and support tests.

## Residual Boundaries

No dynamic metric labels, raw transaction hex, txids, wtxids, peer ids, endpoints, permission strings, credentials, public propagation proof, production-readiness proof, or destructive repair command was added.
