---
phase: 73-opt-in-uat-and-deterministic-verification
plan: 01
subsystem: verification
tags: [bun, typescript, deterministic-verification, rust-tests, ver-02]

requires:
  - phase: 68-full-active-chain-validation-and-durable-persistence
    provides: durable active-chain and chainstate persistence anchors
  - phase: 69-tip-tracking-and-stay-current-operation
    provides: durable tip reopen evidence
  - phase: 70-reorg-peer-rotation-and-no-progress-recovery
    provides: reorg, peer failure, and duplicate/no-credit anchors
  - phase: 71-resource-bounds-and-durable-restart-resume
    provides: restart/resume matrix and synthetic resource-bound anchors
provides:
  - Phase 73 VER-02 coverage map in a deterministic Bun checker
  - Focused proof that mapped Rust anchors execute locally
affects: [phase-73, phase-74, deterministic-verification, opt-in-uat]

tech-stack:
  added: []
  patterns: [Bun TypeScript checker with explicit file and test-name anchors]

key-files:
  created:
    - scripts/check-phase73-uat-verification.ts
  modified:
    - docs/metrics/lines-of-code.md

key-decisions:
  - "VER-02 coverage remains checker-only because existing deterministic Rust anchors cover every required behavior."
  - "Crash recovery is represented by durable reopen/recovery tests, not by a process-level crash harness."

patterns-established:
  - "VER-02 coverage map: each behavior key maps to exact repo-local files and test-name needles."
  - "Verification-only task commits may be empty when the plan requires an atomic task commit and the task confirms no source changes are needed."

requirements-completed: [VER-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 73-2026-06-13T22-08-43
generated_at: 2026-06-14T01:31:12Z

duration: 42 min
completed: 2026-06-14
---

# Phase 73 Plan 01: VER-02 Coverage Map Summary

**VER-02 now has an auditable deterministic coverage map that links every required behavior to existing hermetic Rust test anchors.**

## Performance

- **Duration:** 42 min
- **Started:** 2026-06-14T00:48:17Z
- **Completed:** 2026-06-14T01:31:12Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- Created `scripts/check-phase73-uat-verification.ts` with the required `VER02_COVERAGE` map and seven exact behavior keys.
- Mapped VER-02 durable UTXO/undo, connect/disconnect/reorg, header selection, peer failures, crash recovery, duplicate connect prevention, and resource-bound behaviors to existing deterministic Rust anchors.
- Proved representative chainstate and node anchors execute locally with focused Cargo commands.

## Task Commits

Each task was committed atomically:

1. **Task 1: Create the Phase 73 VER-02 coverage checker skeleton** - `55ce8fb` (feat)
2. **Task 2: Prove the mapped Rust anchors are executable** - `b88f2d1` (test, empty verification commit)

## Files Created/Modified

- `scripts/check-phase73-uat-verification.ts` - Phase 73 Bun checker with VER-02 behavior-to-anchor map and hermetic default-verification guard.
- `docs/metrics/lines-of-code.md` - Refreshed by the repo-managed pre-commit hook after adding the checker.

## Decisions Made

- Existing deterministic anchors were sufficient for all VER-02 behaviors, so no new Rust source or test files were added.
- Crash recovery evidence follows Phase 73 decision D-08: durable reopen/recovery tests are the auditable evidence, and no process-level crash harness was added.
- Task 2 used an empty verification commit because the task outcome was proof that no checker remapping or Rust changes were needed.

## Deviations from Plan

None - plan executed as written. The only execution adjustment was an empty Task 2 verification commit to satisfy the explicit per-task commit requirement without introducing a no-op file change.

## Issues Encountered

None.

## Known Stubs

None. Stub scan only found local checker accumulator arrays (`parts` and `failures`) that do not flow to UI rendering or mock data.

## Verification

- `bun --check scripts/check-phase73-uat-verification.ts` passed.
- `bun run scripts/check-phase73-uat-verification.ts` passed and printed `validated Phase 73 opt-in UAT and deterministic verification evidence`.
- `rg -n "const VER02_COVERAGE|durable_utxo_undo_writes|crash_recovery_durable_reopen|phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network" scripts/check-phase73-uat-verification.ts` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate connect_disconnect_and_reorg_preserve_phase_four_outcomes --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node connected_active_chain_progress_survives_runtime_reopen --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node same_datadir_reopen_connects_best_available_branch_when_blocks_are_already_local --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network --all-features` passed.
- Repo-required pre-commit Rust checks and `bash scripts/verify.sh` passed through normal hooks for both task commits.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for `73-02-PLAN.md`. The coverage map gives later Phase 73 plans a single source of truth for VER-02 without expanding default verification into public-network or service-manager UAT.

## Self-Check: PASSED

- Found summary file at `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-01-SUMMARY.md`.
- Found task commit `55ce8fb`.
- Found task commit `b88f2d1`.

---
*Phase: 73-opt-in-uat-and-deterministic-verification*
*Completed: 2026-06-14*
