---
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 131-2026-07-24T22-07-47
generated_at: 2026-07-25T10:04:48.000Z
requirements-completed: [PRESS-01, PRESS-02, PRESS-03, PRESS-04, PRESS-05]
phase: 131-rolling-fee-expiry-and-descendant-eviction-core
plan: 05
subsystem: mempool
tags: [mempool, sustained-pressure, rolling-fee, expiry, PRESS-05, open-bitcoin-bench, verifier]

requires:
  - phase: 131-rolling-fee-expiry-and-descendant-eviction-core
    provides: Accounted trim/bump, block-gated decay, expiry, and accounted_memory/active evidence (01–04)
provides:
  - Hermetic sustained-pressure oracle (fill→trim→block→decay→expiry→refill→reorg)
  - Restart-zero rolling fee contract (D-15)
  - open-bitcoin-bench mempool-policy.sustained-pressure-trim threshold (N=24 / 2s)
  - scripts/check-phase131-rolling-fee-expiry-pressure.ts wired into verify.sh
affects:
  - Phase 131 verification / PRESS-05 closeout
  - Default verifier ownership of rolling-fee/expiry/pressure surfaces

tech-stack:
  added: []
  patterns:
    - Hermetic Instant-bounded Pure bench cases for pressure blowup detection
    - Phase checker dual wiring (VERIFY_COMMAND_ORDER heredoc + run_step) after Phase 130

key-files:
  created:
    - packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs
    - scripts/check-phase131-rolling-fee-expiry-pressure.ts
    - scripts/check-phase131-rolling-fee-expiry-pressure.test.ts
  modified:
    - packages/open-bitcoin-mempool/src/pool/tests.rs
    - packages/open-bitcoin-bench/src/cases/mempool.rs
    - packages/open-bitcoin-bench/src/cases.rs
    - packages/open-bitcoin-bench/src/fixtures.rs
    - packages/open-bitcoin-bench/src/registry.rs
    - docs/parity/source-breadcrumbs.json
    - docs/parity/catalog/mempool-policy.md
    - scripts/verify.sh
    - scripts/check-phase130-resource-time-fee-primitives.test.ts

key-decisions:
  - "Sustained-pressure bench uses N=24 admit/trim cycles with a 2s wall-time ceiling"
  - "Phase 131 checker runs between Phase 130 and the Phase 117 final gate"
  - "Rolling fee restart baseline remains zero (non-durable) for this phase"

patterns-established:
  - "PRESS-05 oracle asserts recompute_resource_ledger + rolling expectation after every committed transition"
  - "Hermetic Pure bench thresholds live in the case module and fail default verify when exceeded"

requirements-addressed: [PRESS-05]

duration: 58min
completed: 2026-07-25
---

# Phase 131 Plan 05: Sustained Oracle/Perf + Breadcrumbs + Phase 131 Verifier Summary

**PRESS-05 locked with hermetic multi-step oracle, restart-zero contract, 2s Pure bench threshold, and verify.sh Phase 131 checker**

## Performance

- **Duration:** 58 min
- **Started:** 2026-07-25T08:40:48Z
- **Completed:** 2026-07-25T09:39:04Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Added `sustained_pressure_cases.rs` covering fill→trim→block→decay→expiry→refill→reorg with `recompute_resource_ledger` and rolling-fee equality after each step, plus D-15 restart-zero
- Added Pure `mempool-policy.sustained-pressure-trim` bench case (N=24 cycles, 2s max elapsed) and documented bounds/non-durability in `mempool-policy.md`
- Created `check-phase131-rolling-fee-expiry-pressure.ts` asserting PRESS-01..05 surfaces, breadcrumbs, evidence labels, and no public-network soak claims; wired into both verify.sh command-order blocks after Phase 130

## Task Commits

Each task was committed atomically:

1. **Task 1: Sustained oracle scenario + restart-zero assertion** (+ Rule 3 breadcrumb registration for new test module) - `d7434246` (feat)
2. **Task 2: Hermetic bench threshold + breadcrumbs + Phase 131 verifier** (+ Rule 3 Phase 130 test needle update for Phase 131 insertion) - `850bf959` (feat)

**Plan metadata:** _(docs commit follows)_

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs` - PRESS-05 oracle + restart-zero tests
- `packages/open-bitcoin-mempool/src/pool/tests.rs` - Module registration
- `packages/open-bitcoin-bench/src/cases/mempool.rs` - Sustained-pressure trim bench + threshold
- `packages/open-bitcoin-bench/src/cases.rs` - Run all MempoolPolicy cases in repeatability test
- `packages/open-bitcoin-bench/src/fixtures.rs` - pub(crate) helpers for pressure fixtures
- `packages/open-bitcoin-bench/src/registry.rs` - Expanded mempool Knots mapping notes
- `docs/parity/source-breadcrumbs.json` - `sustained_pressure_cases` under mempool-policy
- `docs/parity/catalog/mempool-policy.md` - PRESS-05 bounds + D-15 non-durable rolling fee
- `scripts/check-phase131-rolling-fee-expiry-pressure.ts` / `.test.ts` - Phase 131 ownership checker
- `scripts/verify.sh` - Dual wiring after Phase 130
- `scripts/check-phase130-resource-time-fee-primitives.test.ts` - Heredoc mutation needle updated for Phase 131 adjacency

## Decisions Made

- Threshold chosen as N=24 / 2s: tight enough for quadratic blowups, loose enough for CI noise
- Keep Phase 117 as the final phase checker; insert Phase 131 between Phase 130 and Phase 117
- Leave rolling fee non-durable (restart zero) per D-15 / Phase 135 boundary

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Register sustained_pressure_cases breadcrumb with Task 1**
- **Found during:** Task 1 commit
- **Issue:** Pre-commit parity breadcrumb check rejects new Rust test modules without `source-breadcrumbs.json` mapping
- **Fix:** Added path under `mempool-policy` group in the same feat commit
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** full pre-commit `verify.sh` via gsd-tools commit
- **Committed in:** `d7434246`

**2. [Rule 3 - Blocking] Update Phase 130 heredoc mutation needle for Phase 131 insertion**
- **Found during:** Task 2 verify wiring
- **Issue:** Phase 130 checker test expected Phase 130 run immediately followed by Phase 117 test
- **Fix:** Updated mutation needle to remove Phase 130 run while leaving Phase 131/117 order intact
- **Files modified:** `scripts/check-phase130-resource-time-fee-primitives.test.ts`
- **Verification:** `bun test scripts/check-phase130-resource-time-fee-primitives.test.ts` + Task 2 commit verify
- **Committed in:** `850bf959`

**3. [Rule 3 - Blocking] Keep PRESS-05 in requirements-addressed, not requirements-completed**
- **Found during:** SUMMARY docs commit
- **Issue:** Active-milestone traceability treats `requirements-completed: [PRESS-05]` as activated without a lifecycle-valid VERIFICATION artifact
- **Fix:** Mirror Plans 01–04: `requirements-completed: []` with `requirements-addressed: [PRESS-05]` until phase verification
- **Files modified:** `131-05-SUMMARY.md`
- **Verification:** re-run docs commit verify
- **Committed in:** `b58435fe`

---

**Total deviations:** 3 auto-fixed (all blocking for hooks/adjacency/traceability)
**Impact on plan:** Required for green verify and correct dual wiring. No STATE/ROADMAP updates (per executor prompt).

## Issues Encountered

- First Task 1 commit attempt failed rustfmt; fixed with `cargo fmt` before successful commit
- Phase 131 mutation tests initially used `_renamed` suffixes that still contained original needles; switched to non-substring-preserving replacements

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- PRESS-05 is complete; Phase 131 core pressure/rolling/expiry/evidence surfaces are verifier-owned
- Later phases (132+) can rely on hermetic PRESS oracles without public soak gates

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs` contains `sustained_pressure_oracle` and `recompute_resource_ledger`
- FOUND: `packages/open-bitcoin-bench/src/cases/mempool.rs` contains `mempool-policy.sustained-pressure-trim`
- FOUND: `scripts/check-phase131-rolling-fee-expiry-pressure.ts` contains `PRESS-05`
- FOUND: `rg check-phase131-rolling-fee-expiry-pressure scripts/verify.sh` returns ≥2 matches
- FOUND: commit `d7434246`
- FOUND: commit `850bf959`

---
*Phase: 131-rolling-fee-expiry-and-descendant-eviction-core*
*Completed: 2026-07-25*

<!-- gsd-lifecycle-trailer -->
---
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 131-2026-07-24T22-07-47
generated_at: 2026-07-25T10:04:48.000Z
---
