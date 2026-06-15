---
phase: 77-corruption-and-lock-recovery-hardening
plan: 05
subsystem: operator-soak
tags: [rust, soak, recovery-evidence, reports]

requires:
  - phase: 77-corruption-and-lock-recovery-hardening
    provides: Plan 77-04 shared status recovery evidence for support, live-smoke, and dashboard surfaces
provides:
  - Soak checkpoint recovery action class, cause, and next-action fields
  - Soak outcome projection that prefers top-level recovery evidence category with sync fallback
  - Soak Markdown report rendering for recovery action class, cause, and next action
affects: [operator-soak, support-evidence, recovery-reports]

tech-stack:
  added: []
  patterns:
    - Soak checkpoints consume `status.recovery_evidence` as the recovery source of truth.
    - Soak reports render compact stable labels and next action only.

key-files:
  created:
    - .planning/phases/77-corruption-and-lock-recovery-hardening/77-05-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator/soak/ledger.rs
    - packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs
    - packages/open-bitcoin-cli/src/operator/soak/report.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs

key-decisions:
  - "Preserve existing soak outcome vocabulary while letting top-level recovery evidence provide the category used by soak outcome classification."
  - "Keep report recovery evidence compact: category, action class, cause, and next action, with forbidden raw material covered by tests."

patterns-established:
  - "Checkpoint recovery category falls back to legacy sync recovery category only when top-level recovery evidence is unavailable."
  - "Soak report Markdown places recovery action class, cause, and next action beside the recovery category line."

requirements-completed: [REC-06, REC-07, REC-08]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 77-2026-06-15T18-33-03
generated_at: 2026-06-15T23:55:09Z

duration: 12min
completed: 2026-06-15
---

# Phase 77 Plan 05: Soak Recovery Evidence Summary

**Soak checkpoints and reports now carry compact Phase 77 recovery action class, cause, and next-action evidence without changing Phase 75 outcome labels.**

## Performance

- **Duration:** 12min
- **Started:** 2026-06-15T23:43:37Z
- **Completed:** 2026-06-15T23:55:09Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added optional recovery action class, cause, and next-action fields to `SoakCheckpointStatus`.
- Updated soak checkpoint and outcome projection to prefer `snapshot.recovery_evidence` while preserving legacy `sync.recovery_category` fallback.
- Rendered recovery action class, cause, and next action in soak Markdown reports, while JSON keeps the serialized checkpoint status field names.
- Added focused TDD coverage for checkpoint projection, outcome taxonomy, report rendering, and forbidden raw material exclusion.

## Task Commits

Each task was committed atomically:

1. **Task 1 RED: Add failing soak recovery checkpoint tests** - `098f056` (test)
2. **Task 1 GREEN: Extend soak checkpoint recovery evidence** - `6eac9ef` (feat)
3. **Task 2 RED: Add failing soak recovery report tests** - `0bb4af1` (test)
4. **Task 2 GREEN: Render soak recovery report evidence** - `c569167` (feat)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` - Added recovery action class, cause, and next-action checkpoint fields.
- `packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs` - Projects top-level recovery evidence into checkpoints and outcome evidence with sync fallback.
- `packages/open-bitcoin-cli/src/operator/soak/report.rs` - Renders Markdown recovery action class, cause, and next action beside recovery category.
- `packages/open-bitcoin-cli/src/operator/soak/tests.rs` - Covers outcome taxonomy, report JSON, Markdown, and forbidden-material exclusion.
- `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs` - Covers checkpoint projection from available/unavailable top-level recovery evidence.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Updated an adjacent soak checkpoint fixture for the expanded status struct.

## Decisions Made

- Top-level `snapshot.recovery_evidence.value.category` now drives soak outcome classification when available; `snapshot.sync.recovery_category` remains the compatibility fallback.
- JSON report projection continues to serialize `SoakCheckpointStatus` directly, preserving field names such as `maybe_recovery_action_class_label`.
- Markdown report output shows only compact recovery labels and next action, not backend details or credentials.

## Verification

- RED Task 1: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_recovery_evidence_checkpoint_ --all-features` failed on missing checkpoint recovery fields before implementation.
- GREEN Task 1: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_recovery_evidence_checkpoint_ --all-features` passed: 3 tests.
- GREEN Task 1: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_outcome_classifies_recovery_and_resource_evidence --all-features` passed: 1 test.
- RED Task 2: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_recovery_evidence_report_ --all-features` failed on the missing Markdown `Recovery action class` line before implementation.
- GREEN Task 2: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_recovery_evidence_report_ --all-features` passed: 3 tests.
- Final focused verification reran all three required Cargo commands and passed.
- All six plan acceptance `rg` commands passed.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated adjacent support soak fixture for expanded checkpoint status**
- **Found during:** Task 1 (Extend soak checkpoint recovery fields)
- **Issue:** `packages/open-bitcoin-cli/src/operator/support/tests.rs` constructed `SoakCheckpointStatus`, so adding required fields broke test compilation outside the explicit task file list.
- **Fix:** Added `None` values for the new optional recovery fields in that existing fixture.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support/tests.rs`
- **Verification:** Focused soak checkpoint, outcome, and report tests passed.
- **Committed in:** `6eac9ef`

---

**Total deviations:** 1 auto-fixed (Rule 3)
**Impact on plan:** The deviation was a directly required adjacent test-helper update caused by the expanded checkpoint struct. It did not change production behavior outside the planned soak surface.

## Issues Encountered

None.

## Known Stubs

None. The stub scan found only ordinary format strings in existing resource label code, not placeholder data or unwired UI/report sources.

## Threat Flags

None - the changed trust boundary was already covered by T-77-13 through T-77-15 in the plan, and no new endpoint, file access pattern, auth path, or schema boundary was introduced.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 77-06 to build on compact soak recovery evidence without redefining `recovery_stop` or `resource_stop` semantics.

---
*Phase: 77-corruption-and-lock-recovery-hardening*
*Completed: 2026-06-15*

## Self-Check: PASSED

- Found summary file at `.planning/phases/77-corruption-and-lock-recovery-hardening/77-05-SUMMARY.md`.
- Verified task commits exist: `098f056`, `6eac9ef`, `0bb4af1`, `c569167`.
- Confirmed final focused Cargo verification and acceptance scans passed.
