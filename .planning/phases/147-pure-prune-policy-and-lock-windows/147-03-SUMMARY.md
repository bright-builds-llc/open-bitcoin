---
phase: 147-pure-prune-policy-and-lock-windows
plan: "03"
subsystem: chainstate
tags: [prune, plan_manual_prune, keep-window, ManualPruneRefusal, prune-lock]

requires:
  - phase: 147-02
    provides: PrunePlan, last_prunable_height, height_inside_keep_window, height_forbidden_by_any_lock
provides:
  - Pure plan_manual_prune returning Result<PrunePlan, ManualPruneRefusal>
  - Keep-window refusal (no Knots RPC clamp), lock omit, no byte-budget walk
affects:
  - 148 Fjall unlink consumers
  - 150 operator manual prune surfaces

tech-stack:
  added: []
  patterns:
    - "Manual planner refuses keep-window targets with ManualPruneRefusal::TargetInsideKeepWindow"
    - "ManualOnly and Automatic allow manual planning; only Disabled refuses"
    - "Optional maybe_present_heights BTreeSet filters candidates without storage I/O"

key-files:
  created:
    - packages/open-bitcoin-chainstate/src/prune/tests/manual.rs
  modified:
    - packages/open-bitcoin-chainstate/src/prune/plan.rs
    - packages/open-bitcoin-chainstate/src/prune.rs
    - packages/open-bitcoin-chainstate/src/lib.rs
    - packages/open-bitcoin-chainstate/src/prune/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Refuse keep-window targets; do not clamp like Knots RPC (D-07 / PRUN-03)"
  - "tip == prune_after_height is allowed for manual; tip < prune_after is ChainTooShort"
  - "Leave PRUN-03 and LOCK-01 Pending until lifecycle-valid Phase 147 VERIFICATION"

patterns-established:
  - "plan.rs owns both automatic and manual planners; ManualPruneInput has no byte-budget fields"
  - "Coverage tests exercise maybe_present_heights Some path including lock omit and past-end break"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 147-2026-09-22T12-05-43
generated_at: 2026-09-22T18:20:12.000Z

duration: 62 min
completed: 2026-09-22
---

# Phase 147 Plan 03: Manual Prune Planner Summary

**Pure `plan_manual_prune` with typed keep-window refusal (no Knots RPC clamp), lock-buffer omit, and no automatic byte-budget walk**

## Performance

- **Duration:** 62 min
- **Started:** 2026-09-22T17:17:49Z
- **Completed:** 2026-09-22T18:20:12Z
- **Tasks:** 1
- **Files modified:** 7

## Accomplishments

- Added `ManualPruneInput`, `ManualPruneRefusal`, and `plan_manual_prune` extending `prune/plan.rs` without changing `plan_automatic_prune` behavior
- Refuses Disabled, ChainTooShort (`tip < prune_after`), TargetAboveTip, and TargetInsideKeepWindow; plans legal heights with lock omit
- Covered tip=1000/target=713 refusal vs target=712 success, present-filter path, Automatic mode allow, and no byte-budget fields

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement plan_manual_prune with keep-window refusal and lock omit** - `d135e8f6` (feat)

**Plan metadata:** (pending final docs commit)

_Note: Combined RED+GREEN in one hook-passing feat commit (plan allows this)._

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/prune/plan.rs` — manual planner + refusal enum
- `packages/open-bitcoin-chainstate/src/prune/tests/manual.rs` — 10 Arrange/Act/Assert tests
- `packages/open-bitcoin-chainstate/src/prune.rs` — re-exports
- `packages/open-bitcoin-chainstate/src/prune/tests.rs` — `mod manual`
- `packages/open-bitcoin-chainstate/src/lib.rs` — crate-root re-exports
- `docs/parity/source-breadcrumbs.json` — `tests/manual.rs` in `chainstate-prune`
- `docs/metrics/lines-of-code.md` — hook-regenerated LOC freshness

## Decisions Made

- Keep-window illegal targets refuse with `TargetInsideKeepWindow` rather than Knots RPC clamp to `tip-288`
- Manual `tip == prune_after_height` is allowed (asymmetric vs automatic empty when `tip <= prune_after`)
- Do not implement FlushMode / have_pruned / RPC here
- Leave requirement Complete flips for Phase 147 VERIFICATION (same as 147-01/02)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Added present-heights coverage test for verify coverage gate**
- **Found during:** Task 1 (pre-commit verify coverage gate)
- **Issue:** Uncovered lines 151–159 (`maybe_present_heights: Some` arm) failed verify.sh
- **Fix:** Test `plan_manual_filters_present_heights_and_omits_locks` covering filter, lock omit, and past-`effective_end` break
- **Files modified:** `packages/open-bitcoin-chainstate/src/prune/tests/manual.rs`
- **Verification:** Focused manual tests pass; verify.sh via commit hooks exits 0
- **Committed in:** `d135e8f6`

**2. [Rule 3 - Blocking] Applied cargo fmt before successful commit**
- **Found during:** Task 1 (earlier commit attempts)
- **Issue:** rustfmt/import-order diffs blocked verify.sh
- **Fix:** `cargo fmt -p open-bitcoin-chainstate` and restage
- **Files modified:** `plan.rs`, `tests/manual.rs`
- **Verification:** Commit `d135e8f6` succeeded with hooks
- **Committed in:** `d135e8f6`

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** Required for hook-passing coverage and format; no scope creep (no unlink, RPC, FlushMode, or automatic planner behavior change)

## Issues Encountered

- Three commit attempts failed before coverage + fmt were both clean (~11–20 minute verify runs each). Final attempt succeeded.

## User Setup Required

None - no external service configuration required.

## Requirements Status

Left `requirements-completed` empty and PRUN-03/LOCK-01 Pending until lifecycle-valid Phase 147 VERIFICATION exists (active-milestone verification traceability rejects early Complete flips). Plan frontmatter requirements were `PRUN-03`, `LOCK-01`.

## Next Phase Readiness

- Phase 147 decision surface complete: mode, automatic planner, manual planner
- Ready for Phase 148 unlink to consume `PrunePlan` heights and for Phase 150 to map operator refusals

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-chainstate/src/prune/plan.rs` (`plan_manual_prune`)
- FOUND: `packages/open-bitcoin-chainstate/src/prune/tests/manual.rs`
- FOUND: commit `d135e8f6`

---
*Phase: 147-pure-prune-policy-and-lock-windows*
*Completed: 2026-09-22*
