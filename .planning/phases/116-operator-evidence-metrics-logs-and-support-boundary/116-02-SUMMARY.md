---
phase: 116-operator-evidence-metrics-logs-and-support-boundary
plan: 116-02
subsystem: cli-dashboard-rendering
tags:
  - operator-evidence
  - cli
  - dashboard
  - block-relay
requires:
  - 116-01
provides:
  - CLI status collection and rendering for shared block-relay evidence.
  - Dashboard rows for block-serving and compact-relay evidence from the same snapshot contract.
  - Sensitive-material regression coverage for human, JSON, and dashboard surfaces.
affects:
  - cli-status
  - dashboard
  - parity-breadcrumbs
tech-stack:
  added: []
  patterns:
    - Human and dashboard output read the shared RPC/status contract instead of re-deriving runtime truth.
    - Unavailable operator evidence still renders explicit reason text rather than disappearing from output.
key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/status/render/block_relay.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/block_relay.rs
  modified:
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "CLI JSON mode preserves the shared `block_relay` shape unchanged so support and automation see the same schema as RPC."
  - "Human and dashboard output use fixed counter groups for activation, eligibility, reconstruction, fallback, in-flight, and cleanup evidence."
  - "Unavailable activation remains a first-class rendered state with stable non-sensitive reason text."
requirements-completed:
  - OBS-02
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 116-2026-07-06T03-46-36
generated_at: 2026-07-06T05:04:00Z
---

# Phase 116 Plan 02: CLI And Dashboard Block-Relay Rendering Summary

Phase 116-02 surfaced the shared block-relay contract through operator-facing CLI and dashboard views without introducing renderer-local heuristics or sensitive data leakage.

## Accomplishments

- Extended operator status collection so `block_relay` flows from RPC snapshots and offline fallback uses the same default-unavailable contract.
- Added `Block relay evidence` human rendering plus unchanged JSON projection from the shared contract.
- Added dashboard model rows for activation, serving eligibility, compact announcements, reconstruction and fallback outcomes, in-flight counts, and cleanup counters.
- Added tests that assert cross-surface field presence and the absence of endpoints, hashes, credentials, payloads, and other sensitive strings.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator_status -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_model -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli block_relay -- --nocapture`

## Deviations from Plan

None observed beyond standard implementation wiring.

## Self-Check

- Complete: CLI and dashboard both read the shared `block_relay` snapshot contract.
- Passed: focused CLI, dashboard, and block-relay rendering tests are green in the working tree.
