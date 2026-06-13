---
phase: 72-operator-observability-and-support-evidence
plan: 04
subsystem: operator-observability
tags: [docs, verification, parity, support-evidence, full-sync-evidence]

requires: [72-CONTEXT, 72-RESEARCH, 72-01-SUMMARY, 72-02-SUMMARY, 72-03-SUMMARY]
provides:
  - Operator guidance for Phase 72 full-sync evidence verdicts
  - Architecture and parity documentation for cross-surface evidence boundaries
  - Deterministic Phase 72 observability/support checker wired into default verification
  - Refactored support and sync-summary test modules that preserve file-length gates
  - Code-review warning fixes for verdict escalation, partial Markdown evidence, live-smoke validated-height absence, and checker verdict coverage
affects: [phase-72, docs, verification, support-bundle, sync-summary]

tech-stack:
  added: []
  patterns:
    - Default verification remains deterministic and public-network-free
    - Phase-specific checkers assert exact anchors across docs, source, tests, and verification wiring
    - Large Rust implementation/test modules are split into nearby child modules with parity breadcrumbs

key-files:
  created:
    - .planning/phases/72-operator-observability-and-support-evidence/72-04-SUMMARY.md
    - .planning/phases/72-operator-observability-and-support-evidence/72-REVIEW-FIX.md
    - scripts/check-phase72-observability-evidence.ts
    - packages/open-bitcoin-cli/src/operator/support/evidence.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-node/src/sync/types/summary/tests.rs
  modified:
    - docs/operator/runtime-guide.md
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
    - docs/parity/catalog/p2p.md
    - docs/parity/catalog/chainstate.md
    - docs/parity/catalog/operator-runtime-release-hardening.md
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - scripts/check-phase71-resource-restart.ts
    - scripts/verify.sh

key-decisions:
  - "Documented Phase 72 verdicts as evidence interpretation, not as broad production-node readiness."
  - "Wired a deterministic checker into `scripts/verify.sh` after Phase 71 while keeping live mainnet smoke, service managers, and long-sync operations outside default verification."
  - "Split support evidence and sync summary tests into child modules instead of loosening repository file-length gates."

patterns-established:
  - "Phase 72 checkers now assert plan requirement coverage, source/test anchors, cross-surface evidence values, unavailable-reason wording, support redaction, docs claims, and verify-script boundaries."
  - "Parity breadcrumbs are required for every new first-party Rust source/test file introduced by phase execution."

requirements-completed: [OBS-01, OBS-02, OBS-03, OBS-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 72-2026-06-13T16-25-04
generated_at: 2026-06-13T20:20:26Z

duration: 88min
completed: 2026-06-13
---

# Phase 72 Plan 04: Docs and Verification Closure Summary

**Phase 72 now has operator-facing evidence guidance and a deterministic repository checker that proves the new observability/support contract stays wired into default verification.**

## Performance

- **Duration:** 88 min
- **Started:** 2026-06-13T18:52:00Z
- **Completed:** 2026-06-13T20:20:26Z
- **Tasks:** 3
- **Files modified:** 15
- **Files created:** 6

## Accomplishments

- Added operator guidance for `sync_to_tip_proven`, `stay_current_proven`, `diagnosed_blocker`, and `inconclusive` support verdicts.
- Documented Phase 72 comparison fields, repo-local Cargo/Bazel support-bundle commands, README impact review, and deferred production-scope boundaries.
- Added `scripts/check-phase72-observability-evidence.ts` to verify plan coverage, source/test anchors, cross-surface value agreement, unavailable reasons, support redaction, docs wording, and default-verification boundaries.
- Wired the Phase 72 checker into `scripts/verify.sh` immediately after the Phase 71 checker and refreshed the tracked LOC report.
- Split support evidence derivation and tests into child modules, split sync-summary tests into a child module, and added the required parity breadcrumbs.
- Updated adjacent checker/source expectations so moved tests and new evidence modules remain covered by existing Phase 71 and parity gates.
- Fixed all four Phase 72 code-review warnings and recorded the fix summary in `72-REVIEW-FIX.md`.

## Task Commits

Task commits are pending the wrapper-owned final commit after full phase verification.

1. **Task 1: Document Phase 72 evidence interpretation and deferred scope** - `pending final wrapper commit`
2. **Task 2: Add deterministic Phase 72 cross-surface checker** - `pending final wrapper commit`
3. **Task 3: Wire Phase 72 checker into default verification and refresh LOC** - `pending final wrapper commit`

## Files Created/Modified

- `docs/operator/runtime-guide.md` - Adds Phase 72 verdict interpretation, support-bundle commands, README impact review, and deferred-proof boundaries.
- `docs/architecture/status-snapshot.md` - Records the cross-surface evidence comparison field set.
- `docs/architecture/operator-observability.md` - Aligns operator observability docs with the shared Phase 72 fields, metrics, and logs.
- `docs/parity/catalog/p2p.md`, `docs/parity/catalog/chainstate.md`, `docs/parity/catalog/operator-runtime-release-hardening.md` - Record Phase 72 parity/evidence boundaries and deferred production-node scope.
- `scripts/check-phase72-observability-evidence.ts` - Adds deterministic Phase 72 source/docs/test/default-verification checks.
- `scripts/verify.sh` - Runs the Phase 72 checker after the Phase 71 checker.
- `docs/metrics/lines-of-code.md` - Refreshes tracked generated LOC output.
- `packages/open-bitcoin-cli/src/operator/support/evidence.rs` - Holds full-sync evidence and typed verdict derivation.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Holds support evidence and redaction tests split from `support.rs`.
- `packages/open-bitcoin-node/src/sync/types/summary/tests.rs` - Holds sync-summary metric/log projection tests split from `summary.rs`.
- `docs/parity/source-breadcrumbs.json` - Adds breadcrumbs for new first-party Rust source/test files.

## Decisions Made

- Kept `scripts/verify.sh` deterministic: the checker asserts live-smoke fixture coverage but does not run public peers, service-manager commands, long sync, or current-tip timing checks.
- Kept docs explicit that Phase 72 evidence improves diagnosis/proof surfaces without claiming inbound serving, transaction relay, GUI, signed packaging, Windows service support, or broad production-node readiness.
- Used module splits rather than disabling file-length checks, preserving local ownership and targeted tests.

## Deviations from Plan

- Added Rust child modules during closeout because the accumulated support and sync-summary changes exceeded repository file-length limits.
- Updated the Phase 71 checker source list so moved support redaction tests remain part of the existing Phase 71 resource/restart enforcement.

## Issues Encountered

- Full verification surfaced stale expectations in dashboard metric labels, RPC Phase 72 fixture hashes, node metric samples, and structured log record sizing. These were fixed with targeted test/source updates before the final `scripts/verify.sh` run passed.
- The standard code review found four warnings in support verdict escalation, support Markdown, live-smoke validated-height projection, and checker verdict coverage. All four were fixed and re-verified before phase verification.

## Verification

- `bun --check scripts/check-phase72-observability-evidence.ts`
- `bun run scripts/check-phase72-observability-evidence.ts`
- `bun run scripts/check-phase71-resource-restart.ts`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `bash scripts/check-file-lengths.sh`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase72_ --all-features -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc phase72 --all-features -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_summary --all-features -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase72_summary_metrics_and_logs_carry_full_sync_truth_dimensions --all-features -- --nocapture`
- `bash scripts/test-run-live-mainnet-smoke.sh`
- `bash scripts/verify.sh`
- `72-REVIEW-FIX.md` focused review-fix verification commands all passed.

## User Setup Required

None - all Plan 04 checks are deterministic and local.

## Next Phase Readiness

Phase 72 implementation and code-review fixes are complete; the remaining closeout is phase verification, lifecycle validation, and the wrapper-owned final commit/push.

## Self-Check: PASSED

- Summary file exists.
- Phase 72 checker passes directly and through `scripts/verify.sh`.
- Full repo verification passed.
- Phase 72 code-review warnings were fixed and re-verified with focused checks plus full `scripts/verify.sh`.
- New Rust child modules have parity breadcrumbs.
- Default verification remains public-network-free.

*Phase: 72-operator-observability-and-support-evidence*
*Completed: 2026-06-13*
