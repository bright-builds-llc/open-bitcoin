---
phase: 22-real-sync-benchmarks-and-release-hardening
plan: "01"
subsystem: runtime-benchmark-harness
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 22-2026-04-28T01-24-15
generated_at: 2026-04-28T01:57:12Z
tags:
  - benchmark
  - sync
  - storage
  - operator
  - wallet
key_files:
  created:
    - packages/open-bitcoin-bench/src/runtime_fixtures.rs
    - packages/open-bitcoin-bench/src/cases/sync_runtime.rs
    - packages/open-bitcoin-bench/src/cases/storage_recovery.rs
    - packages/open-bitcoin-bench/src/cases/operator_runtime.rs
    - packages/open-bitcoin-bench/src/cases/wallet_rescan.rs
  modified:
    - packages/open-bitcoin-bench/Cargo.toml
    - packages/open-bitcoin-bench/BUILD.bazel
    - packages/open-bitcoin-bench/src/cases.rs
    - packages/open-bitcoin-bench/src/registry.rs
    - packages/open-bitcoin-bench/src/report.rs
    - packages/open-bitcoin-bench/src/runner.rs
    - packages/open-bitcoin-bench/src/main.rs
    - docs/parity/source-breadcrumbs.json
metrics:
  completed_date: "2026-04-27"
  files_created: 5
  files_modified: 17
---

# Phase 22 Plan 01 Summary

## One-Liner

Open Bitcoin now ships deterministic runtime-backed benchmark coverage for sync,
storage, operator-runtime, and wallet-rescan behavior, plus richer report
metadata that makes smoke and full runs auditable without turning elapsed time
into a release gate.

## What Was Built

- Extended `open-bitcoin-bench` with four new runtime-hardening groups:
  - `sync-runtime`
  - `storage-recovery`
  - `operator-runtime`
  - `wallet-rescan`
- Added shared runtime fixtures for scripted transports, temp storage,
  deterministic blocks or headers, status snapshots, and funded wallet or
  chainstate setup so the new cases stay hermetic and locally reproducible.
- Upgraded the benchmark registry and report schema to capture:
  - benchmark group identity for the new Phase 22 surfaces
  - per-case measurement focus, fixture type, and durability metadata
  - run-profile metadata such as `debug` vs. `release`
  - richer Markdown and JSON report output for reviewer inspection
- Updated the benchmark wrapper so smoke runs keep the `debug` profile while
  full runs use a `release` build and report that distinction explicitly.
- Hardened the benchmark crate tests and tempdir handling so the new runtime
  cases stay stable under parallel test execution.

## Task Commits

1. **Task 1 and Task 2: expand runtime-backed benchmark coverage and report metadata** — Pending the final wrapper-owned Phase 22 closeout commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench -- --nocapture`
- `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports`
- `bash scripts/verify.sh`

## Deviations from Plan

- The final implementation needed a dedicated `runtime_fixtures.rs` module to
  keep the new runtime-backed cases focused and reusable.
- Parallel benchmark tests uncovered tempdir collisions, so the closeout pass
  added unique tempdir ids to keep the suite deterministic under repo-native
  verification.

## Self-Check: PASSED

- The benchmark harness now proves the required runtime-hardening scenarios
  without depending on public-network access.
- The emitted benchmark reports make the scenario intent and binary profile
  explicit enough for audit and release-readiness review.
