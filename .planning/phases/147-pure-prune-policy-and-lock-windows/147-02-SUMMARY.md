---
phase: 147-pure-prune-policy-and-lock-windows
plan: "02"
subsystem: chainstate
tags: [prune, plan_automatic_prune, keep-window, byte-budget, prune-lock]

requires:
  - phase: 147-01
    provides: PruneMode, last_prunable_height, automatic_prune_allowed, height_forbidden_by_any_lock
provides:
  - Pure plan_automatic_prune returning PrunePlan from injected sizes/locks
  - Automatic prune-after empty success, 288 keep-window, oldest-first MiB byte budget, lock omit
affects:
  - 147-03 manual planner
  - 148 Fjall unlink consumers

tech-stack:
  added: []
  patterns:
    - "Automatic planner returns PrunePlan (possibly empty) never Err for prune-after/under-target/Disabled"
    - "Injected BTreeMap height_sizes + current_usage_bytes; no Fjall/fs walk"

key-files:
  created:
    - packages/open-bitcoin-chainstate/src/prune/plan.rs
    - packages/open-bitcoin-chainstate/src/prune/tests/automatic.rs
  modified:
    - packages/open-bitcoin-chainstate/src/prune.rs
    - packages/open-bitcoin-chainstate/src/prune/tests.rs
    - packages/open-bitcoin-chainstate/src/lib.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Stop automatic selection when remaining_usage <= target_bytes after subtracting a candidate"
  - "checked_mul overflow on target_mib → empty plan (no panic)"
  - "Leave PRUN-02 and LOCK-01 Pending until lifecycle-valid Phase 147 VERIFICATION"

patterns-established:
  - "plan.rs owns PrunePlan + AutomaticPruneInput; manual planner deferred to Plan 03"
  - "Coverage tests for overflow and height > last_prunable early break"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 147-2026-09-22T12-05-43
generated_at: 2026-09-22T16:59:07.000Z

duration: 55 min
completed: 2026-09-22
---

# Phase 147 Plan 02: Automatic Prune Planner Summary

**Pure `plan_automatic_prune` with 288 keep-window, prune-after empty success, oldest-first MiB byte budget from injected sizes, and lock-buffer omission**

## Performance

- **Duration:** 55 min
- **Started:** 2026-09-22T16:03:17Z
- **Completed:** 2026-09-22T16:59:07Z
- **Tasks:** 1
- **Files modified:** 7

## Accomplishments

- Added `PrunePlan`, `AutomaticPruneInput`, and `plan_automatic_prune` in `prune/plan.rs` with no disk/Fjall I/O
- Wired crate-root re-exports; automatic path returns empty plans for Disabled/ManualOnly, tip ≤ prune-after, and under-target usage
- Covered keep-window (tip=1000 never plans ≥713), MiB-scale byte budget (`[700,701]` omits 702), lock omit without plan failure, plus overflow / past-last coverage

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement plan_automatic_prune with keep-window, prune-after, byte budget, and lock omit** - `9eaa7293` (feat)

**Plan metadata:** (pending final docs commit)

_Note: Combined RED+GREEN in one hook-passing feat commit (plan allows this; breadcrumbs required before verify)._

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/prune/plan.rs` — automatic planner algorithm
- `packages/open-bitcoin-chainstate/src/prune/tests/automatic.rs` — 8 Arrange/Act/Assert tests
- `packages/open-bitcoin-chainstate/src/prune.rs` — `mod plan` + re-exports
- `packages/open-bitcoin-chainstate/src/prune/tests.rs` — `mod automatic`
- `packages/open-bitcoin-chainstate/src/lib.rs` — crate-root re-exports
- `docs/parity/source-breadcrumbs.json` — `plan.rs` + `tests/automatic.rs` in `chainstate-prune`
- `docs/metrics/lines-of-code.md` — hook-regenerated LOC freshness

## Decisions Made

- Stop when `remaining_usage <= target_bytes` after selecting a candidate (research Open Question 1)
- `target_mib.checked_mul(1024*1024)` overflow yields empty plan rather than panic
- Do not implement `plan_manual_prune` here (Plan 03)
- Leave requirement Complete flips for Phase 147 VERIFICATION (same as 147-01)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Added coverage tests for overflow and height > last break**
- **Found during:** Task 1 (pre-commit verify coverage gate)
- **Issue:** Uncovered lines 51 (`checked_mul` overflow arm) and 64 (`height > last` break) failed verify.sh
- **Fix:** Tests `plan_automatic_empty_when_target_mib_overflows_byte_conversion` and `plan_automatic_stops_before_heights_past_last_prunable`
- **Files modified:** `packages/open-bitcoin-chainstate/src/prune/tests/automatic.rs`
- **Verification:** Focused automatic tests pass; verify.sh via commit hooks exits 0
- **Committed in:** `9eaa7293`

**2. [Rule 3 - Blocking] Included LOC artifact in feat commit**
- **Found during:** Task 1 (first commit attempts)
- **Issue:** Pre-commit regenerates `docs/metrics/lines-of-code.md`; omitting it left hooks without a clean commit path
- **Fix:** Stage regenerated LOC with the feat commit
- **Files modified:** `docs/metrics/lines-of-code.md`
- **Verification:** Commit `9eaa7293` succeeded with hooks
- **Committed in:** `9eaa7293`

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** Required for hook-passing coverage and LOC freshness; no scope creep (no manual planner, unlink, RPC, or FlushMode)

## Issues Encountered

- First two commit attempts failed after ~16–20 minute verify runs (coverage gaps, then gsd-tools reporting `nothing_to_commit` while staged). Fixed coverage and retried successfully.

## User Setup Required

None - no external service configuration required.

## Requirements Status

Left `requirements-completed` empty and PRUN-02/LOCK-01 Pending until lifecycle-valid Phase 147 VERIFICATION exists (active-milestone verification traceability rejects early Complete flips).

## Next Phase Readiness

- Ready for Plan 147-03 `plan_manual_prune` to reuse `PrunePlan` and lock/keep helpers
- Automatic candidate heights are ready for Phase 148 unlink to consume without inventing disk walks in core

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-chainstate/src/prune/plan.rs`
- FOUND: `packages/open-bitcoin-chainstate/src/prune/tests/automatic.rs`
- FOUND: commit `9eaa7293`

---
*Phase: 147-pure-prune-policy-and-lock-windows*
*Completed: 2026-09-22*
