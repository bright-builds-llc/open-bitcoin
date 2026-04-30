---
phase: 13-operator-runtime-foundations
plan: "02"
subsystem: observability
tags: [metrics, logging, retention, observability]
provides:
  - Bounded metrics retention contracts
  - Structured log retention contracts
affects: [OBS-03, OBS-04, node]
requirements-completed: [OBS-03, OBS-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 13-2026-04-26T18-50-22
generated_at: 2026-04-26T18:58:37.416Z
completed: 2026-04-26
---

# Phase 13 Plan 02: Metrics and Log Retention Contracts

## Accomplishments

- Added `docs/architecture/operator-observability.md` with exact metrics and log retention defaults.
- Added `packages/open-bitcoin-node/src/metrics.rs` with serializable metric kinds, samples, retention policy, and metrics status.
- Added `packages/open-bitcoin-node/src/logging.rs` with serializable log level, rotation, retention, path, and log status contracts.
- Wired serde support for the node crate and breadcrumb coverage for new observability contract files.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::` passed.

## Notes

No metrics writer, tracing subscriber, log appender, file pruning logic, dashboard renderer, or public-network dependency was added.
