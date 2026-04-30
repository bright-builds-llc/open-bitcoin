---
phase: 30-configless-live-status-and-dashboard-bootstrap
plan: 01
status: completed
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 30-2026-04-29T16-19-20
generated_at: 2026-04-29T16:27:07.002Z
---

# Phase 30 Plan 01 Summary

## One-Liner

`open-bitcoin status` and `open-bitcoin dashboard` now keep the shared live-RPC
bootstrap alive when the implicit `bitcoin.conf` is absent by passing datadir
and chain defaults through the normal startup resolver instead of bailing out
early.

## What Was Built

- Added `maybe_chain_name` to `CliStartupArgs` so operator-owned internal
  startup resolution can pass `-chain=...` into the existing runtime config
  loader without inventing a second parser.
- Updated `startup_config_for_status()` in
  `packages/open-bitcoin-cli/src/operator/runtime.rs` to build shared startup
  args from operator config evidence, include `-conf` only when `bitcoin.conf`
  exists, and keep status/dashboard on one bootstrap path.
- Added operator runtime regression tests covering configless `status`,
  configless `dashboard`, and the no-credentials fallback behavior.

## Task Commits

1. **Task 1: repair shared live-RPC bootstrap** — Pending the wrapper-owned
   Phase 30 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture`

## Deviations from Plan

- None. The existing public operator execution path was expressive enough to
  verify both status and dashboard behavior, so no new runtime-only helper
  surface was needed.

## Self-Check: PASSED

- The shared bootstrap no longer depends on an on-disk implicit `bitcoin.conf`
  to decide whether live RPC should be attempted.
- `status` and `dashboard` now fail in the same truthful way for a configless
  local workflow: `unreachable` when live RPC is attempted but the daemon is
  not available, `stopped` when credentials cannot be bootstrapped at all.
