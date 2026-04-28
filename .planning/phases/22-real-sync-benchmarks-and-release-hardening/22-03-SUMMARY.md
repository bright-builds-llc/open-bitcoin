---
phase: 22-real-sync-benchmarks-and-release-hardening
plan: "03"
subsystem: parity-ledger-and-release-closeout
requirements-completed: [MIG-05, VER-05, VER-08]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 22-2026-04-28T01-24-15
generated_at: 2026-04-28T01:57:12Z
tags:
  - parity
  - verification
  - benchmarks
  - release
  - docs
key_files:
  created:
    - scripts/check-benchmark-report.ts
    - docs/parity/catalog/operator-runtime-release-hardening.md
  modified:
    - scripts/verify.sh
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/release-readiness.md
    - docs/parity/deviations-and-unknowns.md
    - docs/parity/catalog/README.md
    - docs/metrics/lines-of-code.md
metrics:
  completed_date: "2026-04-27"
  files_created: 2
  files_modified: 7
---

# Phase 22 Plan 03 Summary

## One-Liner

Phase 22 now closes with a repo-owned benchmark-report validator and a refreshed
parity ledger that treats real-sync benchmarks and operator-runtime
release-hardening as first-class audit surfaces instead of Phase 10 leftovers.

## What Was Built

- Added `scripts/check-benchmark-report.ts` so the generated smoke report is
  structurally validated for:
  - schema version
  - required benchmark groups
  - required Phase 22 runtime case ids
  - allowed durability metadata
  - expected smoke profile behavior
- Wired that validator into `bash scripts/verify.sh` immediately after the smoke
  benchmark run so the repo-native verification contract proves both report
  generation and report integrity.
- Added `docs/parity/catalog/operator-runtime-release-hardening.md` as the Phase
  22 audit matrix for verification, benchmark evidence, operator docs,
  migration boundaries, and release-ledger closeout.
- Split the machine-readable and human-readable parity ledger into clearer
  Phase 22 surfaces:
  - `real-sync-benchmarks`
  - `operator-runtime-release-hardening`
- Rewrote `docs/parity/release-readiness.md` from the older headless-v1 audit
  wording into the current v1.1 operator-runtime handoff surface.
- Refreshed the deviations register and catalog index so packaged installs,
  Windows service support, hosted dashboards, migration apply mode, public
  network default verification, and timing-threshold release gates remain
  visible non-claims.

## Task Commits

1. **Task 1 and Task 2: harden benchmark verification and refresh the release ledger** — Pending the final wrapper-owned Phase 22 closeout commit.

## Verification

Passed:

- `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports`
- `bun scripts/check-benchmark-report.ts --report=packages/target/benchmark-reports/open-bitcoin-bench-smoke.json`
- `bash scripts/verify.sh`

## Deviations from Plan

- Repo-native verification surfaced closeout-only maintenance work: the LOC
  report needed a refresh, the new Rust files needed to be tracked before the
  parity-breadcrumb checker would see them, and Rust formatting had to be
  normalized before the final verify pass.
- None of those closeout steps changed the shipped behavior; they only brought
  the phase into compliance with the repo's verification contract.

## Self-Check: PASSED

- The parity ledger now distinguishes shipped v1.1 runtime claims from deferred
  or out-of-scope surfaces in both machine-readable and human-readable form.
- The default repo verification path proves benchmark smoke output exists and is
  structurally correct.
