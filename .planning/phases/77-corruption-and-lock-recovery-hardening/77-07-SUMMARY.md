---
phase: 77-corruption-and-lock-recovery-hardening
plan: 07
subsystem: verification
tags: [checker, verification, recovery-evidence, phase-gate]

requires:
  - phase: 77-corruption-and-lock-recovery-hardening
    provides: Plans 77-01 through 77-06 implementation, docs, and parity roots
provides:
  - Deterministic Phase 77 corruption and lock recovery checker
  - Repo-native verifier wiring for Phase 77
  - Phase 77 verification evidence report
  - Compatibility repairs required by the final verifier
affects: [verification, operator-support, operator-soak, docs-metrics, prior-phase-checkers]

tech-stack:
  added: []
  patterns:
    - Keep default verification hermetic and public-network-free.
    - Prefer checker anchors for release boundaries and deterministic recovery evidence.

key-files:
  created:
    - scripts/check-phase77-corruption-lock-recovery.ts
    - scripts/check-phase77-corruption-lock-recovery.test.ts
    - .planning/phases/77-corruption-and-lock-recovery-hardening/77-VERIFICATION.md
    - .planning/phases/77-corruption-and-lock-recovery-hardening/77-07-SUMMARY.md
  modified:
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-bench/src/cases/operator_runtime.rs
    - packages/open-bitcoin-cli/src/operator/soak/runtime.rs
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/resource_bounds.rs
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-cli/src/operator/support/evidence.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
    - scripts/check-phase64-service-restart-resume.ts
    - scripts/check-phase71-resource-restart.ts
    - scripts/check-phase72-observability-evidence.ts
    - scripts/check-phase75-soak-runner.test.ts
    - scripts/check-phase75-soak-runner.ts

key-decisions:
  - "Wire Phase 77 verification immediately after the Phase 76 checker so resource-bound and recovery-bound contracts are checked in dependency order."
  - "Adjust prior phase checkers and operator evidence tests to accept Phase 77 probe-only status behavior instead of forcing status/support paths to open Fjall stores."
  - "Keep soak runtime enrichment explicit to preserve resource-bound evidence for the soak runner without changing probe-only status/support inspection."

patterns-established:
  - "Phase checkers validate exact docs, source anchors, test anchors, parity roots, and default-verification exclusions."
  - "Verification fallout is committed with the verification report rather than hidden as generated or incidental churn."

requirements-completed: [REC-05, REC-06, REC-07, REC-08]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 77-2026-06-15T18-33-03
generated_at: 2026-06-16T02:08:59Z

duration: 2h+
completed: 2026-06-16
***

# Phase 77 Plan 07: Deterministic Verification Summary

**Phase 77 recovery hardening now has a deterministic checker, repo-native verifier wiring, and recorded verification evidence.**

## Performance

- **Completed:** 2026-06-16T02:08:59Z
- **Tasks:** 2
- **Files created:** 4
- **Files modified:** 14

## Accomplishments

- Added `scripts/check-phase77-corruption-lock-recovery.ts` to validate REC-05 through REC-08 plan coverage, recovery source anchors, probe-only status/support boundaries, support/live-smoke/soak projections, docs/parity roots, deterministic tests, and forbidden default-verification strings.
- Added fixture-root checker tests for missing requirements, missing source anchors, direct status/support Fjall opens, live-smoke anchor gaps, and incorrect verifier ordering.
- Wired the checker test and checker into `scripts/verify.sh` immediately after Phase 76.
- Recorded `.planning/phases/77-corruption-and-lock-recovery-hardening/77-VERIFICATION.md` with `status: passed` after the executor's full `bash scripts/verify.sh` pass.
- Reconciled final verifier fallout from Phase 77's probe-only status behavior across support evidence, soak runtime resource-bound enrichment, prior phase checkers, operator tests, benchmark smoke expectations, and LOC freshness.

## Task Commits

Each task/fix was committed atomically:

1. **Task 1 failing tests** - `f916b45` (test)
2. **Task 1 checker and verifier wiring** - `a8a82c4` (feat)
3. **Task 2 verification fallout and evidence report** - `d138f1c` (fix/docs)

## Files Created/Modified

- `scripts/check-phase77-corruption-lock-recovery.ts` - New deterministic checker for Phase 77 recovery hardening boundaries.
- `scripts/check-phase77-corruption-lock-recovery.test.ts` - Fixture-root checker regression tests.
- `scripts/verify.sh` - Runs Phase 77 checker test and checker after Phase 76.
- `.planning/phases/77-corruption-and-lock-recovery-hardening/77-VERIFICATION.md` - Verification evidence with `status: passed`.
- `docs/metrics/lines-of-code.md` - Refreshed generated LOC report after new checker and test files.
- `packages/open-bitcoin-cli/src/operator/support.rs` and `support/evidence.rs` - Kept support evidence compatible with probe-only status and live-smoke final-status evidence.
- `packages/open-bitcoin-cli/src/operator/soak/runtime.rs` - Preserved soak resource-bound evidence when the soak runner can read its own durable store.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Updated operator binary expectations for Phase 77 probe-only evidence.
- `packages/open-bitcoin-bench/src/cases/operator_runtime.rs` - Updated smoke expectations for probe-only metrics history.
- `scripts/check-phase64-service-restart-resume.ts`, `scripts/check-phase71-resource-restart.ts`, `scripts/check-phase72-observability-evidence.ts`, `scripts/check-phase75-soak-runner.test.ts`, and `scripts/check-phase75-soak-runner.ts` - Updated checker anchors to align with the Phase 77 recovery/support layout.

## Verification

- `bun test scripts/check-phase77-corruption-lock-recovery.test.ts` passed.
- `bun run scripts/check-phase77-corruption-lock-recovery.ts` passed.
- `bun test scripts/check-phase75-soak-runner.test.ts` passed.
- `bun run scripts/check-phase64-service-restart-resume.ts && bun run scripts/check-phase71-resource-restart.ts && bun run scripts/check-phase72-observability-evidence.ts && bun run scripts/check-phase75-soak-runner.ts` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_support_bundle_includes_phase72_full_sync_evidence_and_typed_verdict --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_support_bundle_summarizes_phase61_resource_recovery_evidence --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_soak --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib support::tests::support_recovery_evidence_collection_preserves_probe_only_store_health --all-features` passed.
- Executor-recorded full verifier: `bash scripts/verify.sh` passed in 46m 46.019s and is recorded in `77-VERIFICATION.md`.

## Deviations from Plan

- The 77-07 executor became unavailable after writing `77-VERIFICATION.md`; the orchestrator preserved the valid checker/report work, restored accidental summary frontmatter churn, reran focused checks, and committed the remaining verification fallout.
- The verification-fallout commit touched additional operator support, soak, benchmark, and prior-checker files because full verification exposed stale assumptions from earlier phases after Phase 77 made status/support inspection probe-only.

## Issues Encountered

- Prior phase checkers still required legacy service/status anchors that Phase 77 intentionally replaced with probe-only evidence. The checkers now accept the new probe-only anchors while preserving their original safety claims.
- Operator support and soak evidence had stale assumptions about runtime metadata and resource-bound availability after status collection stopped opening Fjall stores for probe-only inspection.

## Known Stubs

None.

## Threat Flags

None. The checker explicitly guards against public-network, service-manager, long-sleep, process-scan, and opt-in live-smoke commands entering default verification.

## User Setup Required

None.

## Next Phase Readiness

Ready for orchestrator-level code review, lifecycle validation, final repo verification, and phase completion bookkeeping.

---
*Phase: 77-corruption-and-lock-recovery-hardening*
*Completed: 2026-06-16*

## Self-Check: PASSED

- Found summary file at `.planning/phases/77-corruption-and-lock-recovery-hardening/77-07-SUMMARY.md`.
- Verified task commits exist: `f916b45`, `a8a82c4`, `d138f1c`.
- Confirmed `77-VERIFICATION.md` contains `status: passed`.
- Reran focused checker, prior-checker, support, and soak tests after taking over from the unavailable executor.
