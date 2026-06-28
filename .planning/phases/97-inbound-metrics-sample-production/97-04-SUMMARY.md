---
phase: 97-inbound-metrics-sample-production
plan: 04
subsystem: verification
tags: [typescript, bun, verifier, docs, generated-artifacts]
requires:
  - phase: 97-inbound-metrics-sample-production
    plan: 01
    provides: Status-derived inbound metric mapper.
  - phase: 97-inbound-metrics-sample-production
    plan: 02
    provides: Runtime metrics append path.
  - phase: 97-inbound-metrics-sample-production
    plan: 03
    provides: Dashboard/status/support proof.
provides:
  - Deterministic Phase 97 checker and mutation tests.
  - Structural guard for RPC/live-status retained metrics and sync-disabled inbound metrics worker wiring.
  - Default verifier wiring after Phase 96.
affects: [verify-sh, phase-checkers, generated-loc]
tech-stack:
  added: []
  patterns: [bun-structural-checker, mutation-fixtures, verifier-ordering]
key-files:
  created:
    - scripts/check-phase97-inbound-metrics.ts
    - scripts/check-phase97-inbound-metrics.test.ts
  modified:
    - scripts/verify.sh
key-decisions:
  - "Check the mapper, runtime append, dashboard selector, docs, and verifier wiring as one deterministic structural contract."
  - "Run Phase 97 immediately after Phase 96 in the default verifier."
requirements-completed: [INB-05, DOS-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 97-2026-06-28T16-11-36
generated_at: 2026-06-28T17:15:16Z
duration: 19min
completed: 2026-06-28
---

# Phase 97 Plan 04: Verification Summary

**A deterministic Phase 97 checker now locks the retained inbound metrics contract into the default verifier.**

## Accomplishments

- Added `scripts/check-phase97-inbound-metrics.ts`.
- Added mutation fixture tests for missing metric variants, label-count mapping, runtime append omissions, dashboard expansion, docs claim creep, and verifier ordering.
- Extended the checker after review to require the sync-disabled inbound metrics worker, RPC `MetricsStatus` response field, dispatch projection, and live CLI metrics test.
- Updated the checker after the file-length split so it follows `sync/metrics.rs`, `open_bitcoind/inbound_metrics.rs`, `context/inbound_status.rs`, and `dashboard/model/metrics.rs`.
- Wired Phase 97 checker tests and checker execution into `scripts/verify.sh` immediately after Phase 96.

## Task Commits

Deferred until the wrapper-level clean verification gate.

## Files Created/Modified

- `scripts/check-phase97-inbound-metrics.ts` - Adds the Phase 97 structural checker.
- `scripts/check-phase97-inbound-metrics.test.ts` - Adds deterministic checker tests.
- `scripts/verify.sh` - Runs Phase 97 after Phase 96.
- `docs/parity/source-breadcrumbs.json` - Tracks parity breadcrumbs for new first-party Rust modules.

## Deviations from Plan

None.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all -- --check`
- `bun test scripts/check-phase97-inbound-metrics.test.ts`
- `bun run scripts/check-phase97-inbound-metrics.ts`
- `bash scripts/check-file-lengths.sh`
- `bash scripts/verify.sh`

## User Setup Required

None.
