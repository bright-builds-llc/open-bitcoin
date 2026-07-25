---
phase: 131-rolling-fee-expiry-and-descendant-eviction-core
plan: 01
subsystem: mempool
tags: [mempool, pressure, rolling-fee, accounted-capacity, trim, PRESS-01, PRESS-02]

requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: MempoolCapacity, accounted resource ledger, fee-role wrappers, Pressure lifecycle causes
provides:
  - Accounted-memory trim_to_size against MempoolCapacity
  - RollingFeeState::track_package_removed package+incremental bump
  - PRESS-01/02 hermetic pressure fixtures
affects:
  - 131-02 block-gated rolling decay
  - 131-04 evidence/seam retirement (legacy_vsize field + capacityenforcement label)

tech-stack:
  added: []
  patterns:
    - Accounted-capacity pressure loop with prospective rolling clone on admit
    - Knots trackPackageRemoved strict-greater bump clearing block-since gate

key-files:
  created:
    - packages/open-bitcoin-mempool/src/fee/rolling.rs
    - packages/open-bitcoin-mempool/src/pool/pressure.rs
    - packages/open-bitcoin-mempool/src/pool/tests/pressure_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/fee.rs
    - packages/open-bitcoin-mempool/src/lib.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-mempool/src/pool/admission.rs
    - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
    - packages/open-bitcoin-mempool/src/pool/tests.rs
    - packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs
    - packages/open-bitcoin-mempool/src/pool/tests/lifecycle_delta_cases.rs
    - packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs
    - packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs
    - packages/open-bitcoin-mempool/tests/parity.rs
    - packages/open-bitcoin-node/src/network/tests/recovery_cases.rs
    - scripts/check-phase130-resource-time-fee-primitives.ts
    - docs/parity/catalog/mempool-policy.md
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Trim limiter is accounted_memory vs MempoolCapacity; total_virtual_size never drives eviction"
  - "Prospective admit clones RollingFeeState so CandidateEvicted discards bump side effects"
  - "PolicyConfig.legacy_vsize_trim_limit retained until Plan 04; Phase 130 checker now asserts the field on types.rs"

patterns-established:
  - "Pressure package feerate = descendant aggregate fee/vsize + incremental sats/kvB"
  - "Effective admission remains max(static, rolling); incremental is bump/replacement input only"

requirements-completed: []
requirements-addressed: [PRESS-01, PRESS-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 131-2026-07-24T22-07-47
generated_at: 2026-07-25T02:04:37.000Z

duration: 102min
completed: 2026-07-25
---

# Phase 131 Plan 01: Accounted Trim + Package Rolling Bump Summary

**Accounted-memory mempool trim with Knots `trackPackageRemoved` package+incremental rolling bumps (PRESS-01/02)**

## Performance

- **Duration:** 102 min
- **Started:** 2026-07-25T00:22:34Z
- **Completed:** 2026-07-25T02:04:37Z
- **Tasks:** 3
- **Files modified:** 19

## Accomplishments

- Switched `trim_to_size` to `accounted_memory > mempool_capacity` and moved it into `pool/pressure.rs`
- Added `RollingFeeState` with strict-greater `track_package_removed` that clears the block-since bump gate
- Landed PRESS-01/02 hermetic fixtures and migrated legacy vsize trim drivers to tiny `mempool_capacity`

## Task Commits

Each task was committed atomically:

1. **Tasks 1–3: PRESS fixtures + accounted trim + fixture migration** - `c4720a12` (feat)

**Plan metadata:** _(docs commit follows)_

_Note: Pre-commit `verify.sh` rejects intentionally failing RED tests, so Task 1 RED could not land as a separate commit. Implementation, green fixtures, and migrations shipped together._

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/fee/rolling.rs` - RollingFeeState bump/inject API
- `packages/open-bitcoin-mempool/src/pool/pressure.rs` - Accounted-capacity trim + package selection
- `packages/open-bitcoin-mempool/src/pool/tests/pressure_cases.rs` - PRESS-01/02 trim and bump oracles
- `packages/open-bitcoin-mempool/src/pool/admission.rs` - Prospective rolling clone + commit wiring
- `scripts/check-phase130-resource-time-fee-primitives.ts` - Legacy seam check points at PolicyConfig field

## Decisions Made

- Prospective admission clones rolling state before trim so failed `CandidateEvicted` paths do not retain bumps
- Left `legacy_vsize_trim_limit` on `PolicyConfig` and `MempoolCapacityEnforcement::LegacyVsize` for Plan 04
- Updated Phase 130 checker to require the transitional field on `types.rs` after pool trim stopped naming it

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Combined Task 1 RED commit into the green feat commit**
- **Found during:** Task 1 commit
- **Issue:** Pre-commit runs full `verify.sh`, which fails on intentionally red PRESS fixtures
- **Fix:** Implemented Tasks 1–3 then committed once green
- **Files modified:** all plan files
- **Verification:** `cargo test -p open-bitcoin-mempool --lib` and full verify via gsd-tools commit
- **Committed in:** `c4720a12`

**2. [Rule 3 - Blocking] Phase 130 checker required `legacy_vsize_trim_limit` in `pool.rs`**
- **Found during:** Task 3 / commit verify
- **Issue:** Checker treated removal of the active pool trim limiter as loss of the Phase 130 seam
- **Fix:** Assert transitional field on `types.rs`; document Plan 01 accounted trim in mempool-policy catalog
- **Files modified:** `scripts/check-phase130-resource-time-fee-primitives.ts`, `docs/parity/catalog/mempool-policy.md`
- **Verification:** `bun test scripts/check-phase130-resource-time-fee-primitives.test.ts`
- **Committed in:** `c4720a12`

**3. [Rule 2 - Missing Critical] Prospective rolling clone on admit**
- **Found during:** Task 2
- **Issue:** Mutating mempool rolling during prospective trim would retain bumps after `CandidateEvicted`
- **Fix:** Clone `RollingFeeState` for trim; assign only after successful commit
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/admission.rs`
- **Verification:** `no_partial_mutation_for_candidate_evicted` + capacity-zero eviction fixtures
- **Committed in:** `c4720a12`

---

**Total deviations:** 3 auto-fixed (2 blocking, 1 missing critical)
**Impact on plan:** Required for hooks, Phase 130 continuity, and bump correctness. No scope creep into decay/expiry/evidence flip.

## Issues Encountered

- Coverage and rustfmt/`-Dwarnings` dead-code findings on new helpers required iterative verify-commit cycles before hooks accepted the feat commit

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 02 can add block-gated decay on `RollingFeeState` (`block_since_last_rolling_fee_bump`, `last_rolling_fee_update`)
- Plan 04 can delete `legacy_vsize_trim_limit` and flip `capacityenforcement` / rolling parity labels

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-mempool/src/fee/rolling.rs`
- FOUND: `packages/open-bitcoin-mempool/src/pool/pressure.rs`
- FOUND: `packages/open-bitcoin-mempool/src/pool/tests/pressure_cases.rs`
- FOUND: commit `c4720a12`

---
*Phase: 131-rolling-fee-expiry-and-descendant-eviction-core*
*Completed: 2026-07-25*
