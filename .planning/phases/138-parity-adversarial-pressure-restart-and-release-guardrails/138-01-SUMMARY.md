---
phase: 138-parity-adversarial-pressure-restart-and-release-guardrails
plan: 01
subsystem: testing
tags: [restart-composition, press-05, work-count, threshold_free, fake-clock]

# Dependency graph
requires:
  - phase: 135-snapshot-schema-checkpointing-and-recovery
    provides: prepare_mempool_recovery_at, install_mempool_recovery, RecoveryInstallFailureGuard
  - phase: 136-receive-independent-maintenance-and-transport-receipts
    provides: maintenance_tick remint from injected RetryDecisionContext
  - phase: 131-rolling-fee-expiry-and-descendant-eviction-core
    provides: PRESS-05 oracle, N=24 trim cycles, and the Instant smoke gate this plan retires
provides:
  - Named hermetic D-04/D-05 restart × package × unbroadcast × remint × inject composition
  - Instant-free default-smoke PRESS-05 bench pinned to work-count symbols
affects: [138-03 last-gate checker, MPVFY-01 matrix, MPVFY-02 work-count contract]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Restart proof stays fake-clock PolicyTime plus cfg(test) RecoveryInstallFailureGuard
    - Default PRESS-05 smoke is work-count / threshold_free, not Instant latency

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/recovery_cases/restart_composition.rs
  modified:
    - packages/open-bitcoin-node/src/network/tests/recovery_cases.rs
    - docs/parity/source-breadcrumbs.json
    - packages/open-bitcoin-bench/src/cases/mempool.rs
    - scripts/check-phase131-rolling-fee-expiry-pressure.ts
    - scripts/check-phase131-rolling-fee-expiry-pressure.test.ts
    - docs/parity/catalog/mempool-policy.md
    - docs/parity/benchmarks.md

key-decisions:
  - "Place the D-04/D-05 composition in recovery_cases/restart_composition.rs next to staging, not a new mega-harness"
  - "Retire SUSTAINED_PRESSURE_MAX_ELAPSED and pin PRESS-05 to N=24 work-count symbols while Phase 117 remains last-gate until Plan 03"

patterns-established:
  - "Pattern 1: One named hermetic test covers restart install, local-package membership, surviving unbroadcast, retry remint, and injected install failure"
  - "Pattern 2: Default verify enforces PRESS-05 by oracle plus work counts, never Instant wall-clock"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 138-2026-08-22T04-13-58
generated_at: 2026-08-22T05:48:18Z

# Metrics
duration: 32min
completed: 2026-08-22
---

# Phase 138 Plan 01: Restart Composition and Work-Count PRESS-05 Gate Summary

**One hermetic fake-clock restart composition plus Instant-free PRESS-05 default smoke pinned to N=24 work counts**

## Performance

- **Duration:** 32 min
- **Started:** 2026-08-22T05:16:29Z
- **Completed:** 2026-08-22T05:48:18Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Named D-04/D-05 composition proves restart install preserves local-package membership and surviving unbroadcast, resets rolling fee, remints retry timers, and leaves injected install failure inert
- Default smoke no longer uses a 2-second Instant ceiling; PRESS-05 stays closed by the recomputation oracle, N=24 trim cycles, one retained entry, and rolling-fee bump
- Catalog and benchmark docs now say default smoke is work-count / `threshold_free`; timings remain `--full` / UAT only
- Phase 131 last-gate still requires Phase 117; no REQUIREMENTS or `in_progress` surface flips landed

## Task Commits

Each task was committed atomically:

1. **Task 1: Add the hermetic restart × package × remint × inject composition** - `8009a7b1` (test)
2. **Task 2: Retire the Phase 131 2s Instant smoke gate and keep PRESS-05 oracle/work-counts** - `910b67c5` (fix)

**Plan metadata:** docs commit follows this summary

## Files Created/Modified
- `packages/open-bitcoin-node/src/network/tests/recovery_cases/restart_composition.rs` - D-04/D-05 composition test
- `packages/open-bitcoin-node/src/network/tests/recovery_cases.rs` - `mod restart_composition`
- `docs/parity/source-breadcrumbs.json` - recovery topology group lists the new test
- `packages/open-bitcoin-bench/src/cases/mempool.rs` - Instant-free sustained-pressure smoke
- `scripts/check-phase131-rolling-fee-expiry-pressure.ts` - PRESS-05 pin retargeted to work-count symbols
- `scripts/check-phase131-rolling-fee-expiry-pressure.test.ts` - mutation now removes `SUSTAINED_PRESSURE_TRIM_CYCLES`
- `docs/parity/catalog/mempool-policy.md` - PRESS-05 wording is work-count / `threshold_free`
- `docs/parity/benchmarks.md` - default-smoke evidence is work-count, not Instant

## Decisions Made
- Keep the composition as one child of `recovery_cases.rs` and reuse existing prepare/install/inject/maintenance_tick seams; do not rewrite package, recovery, or retry runtime
- Replace `SUSTAINED_PRESSURE_MAX_ELAPSED` with pins for `mempool-policy.sustained-pressure-trim`, `SUSTAINED_PRESSURE_TRIM_CYCLES`, and `const SUSTAINED_PRESSURE_TRIM_CYCLES: usize = 24`

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Leave SUMMARY requirements-completed empty**
- **Found during:** Plan metadata commit
- **Issue:** Copying `MPVFY-01` / `MPVFY-02` into SUMMARY `requirements-completed` made `check-active-milestone-verification-traceability` treat MPVFY-02 as activated without lifecycle-valid verification coverage
- **Fix:** Keep `requirements-completed: []` so REQUIREMENTS and surface status stay unchanged until later plans (D-17)
- **Files modified:** `.planning/phases/138-parity-adversarial-pressure-restart-and-release-guardrails/138-01-SUMMARY.md`
- **Verification:** Plan metadata commit retries with an empty requirements-completed list
- **Committed in:** plan metadata commit

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Prevented a premature requirement flip. Task code is unchanged.

## Issues Encountered
The first metadata commit ran `verify.sh` and failed the active-milestone traceability checker for MPVFY-02. Cleared SUMMARY `requirements-completed` and retried.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
Ready for 138-02. Last-gate, REQUIREMENTS, and `in_progress` surface status remain unchanged for later plans. Phase 117 stays the final `check-phase*` gate until Plan 03.

## Self-Check: PASSED

- `packages/open-bitcoin-node/src/network/tests/recovery_cases/restart_composition.rs` exists
- `git log --oneline --all --grep="138-01"` returns `8009a7b1` and `910b67c5`

---
*Phase: 138-parity-adversarial-pressure-restart-and-release-guardrails*
*Completed: 2026-08-22*
---
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 138-2026-08-22T04-13-58
generated_at: 2026-08-22T06:30:00.000Z
---
