---
phase: 24-wallet-aware-live-status-and-build-provenance
plan: "02"
subsystem: build-provenance
requirements-completed: [OBS-01, OBS-02, DASH-01]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 24-2026-04-28T17-43-00
generated_at: 2026-04-28T19:18:00.000Z
completed: 2026-04-28
---

# Phase 24 Plan 02 Summary

## One-Liner

Shared operator status snapshots now carry real compile-time build provenance
when available, and both status and dashboard surfaces render that metadata
instead of hardcoded unavailable placeholders.

## What Was Built

- Added `packages/open-bitcoin-cli/build.rs` to emit compile-time commit, build
  time, target, and profile metadata for Cargo builds while keeping the values
  optional.
- Replaced hardcoded `BuildProvenance::unavailable()` calls in the status
  collector with a helper that reads compile-time env vars through `option_env!`
  and preserves unavailable reasons when a build system does not provide them.
- Updated human status rendering to show `build_time`, `target`, and `profile`
  alongside version and commit.
- Updated the dashboard build summary to project the same shared build
  provenance fields instead of only version plus commit state.
- Added focused tests for the build-provenance helper and for live status JSON
  output carrying populated build metadata fields.

## Task Commits

1. **Task 1: populate and render real build provenance through shared operator
   status** — Pending the wrapper-owned Phase 24 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status::tests -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture`

## Deviations from Plan

- Build provenance remains intentionally graceful under non-Cargo builds such as
  Bazel smoke compilation: the shared snapshot now prefers real metadata when
  available but still emits explicit unavailable reasons when those env vars are
  absent.

## Self-Check: PASSED

- Status JSON and human output now surface real build metadata for the Cargo
  test build.
- Dashboard build rows consume the same repaired shared status model instead of
  a bespoke metadata path.
- The phase keeps the shared `open-bitcoin-node` status contract data-only by
  injecting build metadata from the CLI shell boundary.
