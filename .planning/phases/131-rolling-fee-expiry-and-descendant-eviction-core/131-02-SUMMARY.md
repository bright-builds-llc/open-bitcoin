---
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 131-2026-07-24T22-07-47
generated_at: 2026-07-25T04:09:00.000Z
requirements-completed: []
phase: 131-rolling-fee-expiry-and-descendant-eviction-core
plan: 02
subsystem: mempool
tags: [mempool, pressure, rolling-fee, getminfee, block-gated-decay, PRESS-03]

requires:
  - phase: 131-rolling-fee-expiry-and-descendant-eviction-core
    provides: RollingFeeState bump + accounted trim from Plan 01
provides:
  - Block-gated GetMinFee-shaped rolling decay with 12h/6h/3h half-lives
  - Connected-block lifecycle opens decay gate via BlockLifecycleContext.connected_at
  - PRESS-03 hermetic decay fixtures (gate, 10s skip, incremental/2 zero)
affects:
  - 131-03 expiry cleanup
  - 131-04 rolling fee parity label flip
  - Admission/materialize call sites that need injected PolicyTime for live decay

tech-stack:
  added: []
  patterns:
    - Injected PolicyTime decay_toward with occupancy-adjusted pow(2, dt/halflife)
    - removeForBlock-equivalent gate open even when connect removes nothing

key-files:
  created:
    - packages/open-bitcoin-mempool/src/pool/tests/rolling_fee_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/fee/rolling.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
    - packages/open-bitcoin-mempool/src/lib.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Decay APIs take only PolicyTime + accounted usage/capacity/incremental — no SystemTime"
  - "External rolling uses f64::round (llround-equivalent for positive rates); effective stays max(static, rolling)"
  - "Empty-pool occupancy shortens half-life; fixtures use capacity 0 when asserting the default 12h path"

patterns-established:
  - "Bump clears block_since gate; connect sets gate true and last_update = connected_at"
  - "Mempool::materialize_rolling_fee_rate(now) applies GetMinFee-shaped decay for admission/evidence"

requirements-addressed: [PRESS-03]

duration: 84min
completed: 2026-07-25
---

# Phase 131 Plan 02: Block-Gated Rolling Decay Summary

**Knots GetMinFee block-gated 12h/6h/3h rolling decay with injected PolicyTime and connected-block gate open (PRESS-03)**

## Performance

- **Duration:** 84 min
- **Started:** 2026-07-25T02:39:16Z
- **Completed:** 2026-07-25T04:03:00Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments

- Implemented `RollingFeeState::decay_toward` with Knots half-life shortening, 10s update gate, and incremental/2 zeroing
- Wired `remove_for_connected_block_transition` to open the decay gate from `BlockLifecycleContext.connected_at` even when the block removes nothing
- Landed PRESS-03 fixtures plus a node lifecycle test proving bump→time does not decay and bump→connect→time does

## Task Commits

Each task was committed atomically:

1. **Tasks 1–2: PRESS-03 decay fixtures + GetMinFee port + connect gate** - `935310d4` (feat)

**Plan metadata:** _(docs commit follows)_

_Note: Pre-commit `verify.sh` rejects intentionally failing RED tests, so Task 1 RED could not land as a separate commit. Fixtures and implementation shipped together once green._

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/fee/rolling.rs` - Constants, `open_decay_gate_after_block`, `decay_toward`
- `packages/open-bitcoin-mempool/src/pool/tests/rolling_fee_cases.rs` - Gate, half-life, 10s skip, zero, lifecycle fixtures
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Opens decay gate on connected-block transition
- `packages/open-bitcoin-mempool/src/pool.rs` - `track_package_removed` + `materialize_rolling_fee_rate`
- `packages/open-bitcoin-node/src/network.rs` - Shell helpers for hermetic bump/materialize
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs` - Connect-gated decay regression

## Decisions Made

- Keep raw rolling separate from effective admission (`max(static, rolling)`); never raise mid-decay returns to incremental
- Apply decay on materialize with injected time after the gate opens rather than sampling clocks in core
- Use `mempool_capacity: 0` in empty-pool lifecycle fixtures so occupancy shorteners do not turn a 12h assertion into a 3h path

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Combined Task 1 RED commit into the green feat commit**
- **Found during:** Task 1 commit
- **Issue:** Pre-commit runs full `verify.sh`, which fails on intentionally red PRESS-03 fixtures
- **Fix:** Implemented Tasks 1–2 then committed once green
- **Files modified:** all plan files
- **Verification:** `cargo test -p open-bitcoin-mempool --lib rolling_fee_` and `cargo test -p open-bitcoin-node --lib mempool_lifecycle`
- **Committed in:** `935310d4`

**2. [Rule 1 - Bug] Empty-pool occupancy shortened half-life in lifecycle fixtures**
- **Found during:** Task 1/2 green
- **Issue:** `usage == 0` with default capacity triggers Knots `/4` half-life (3h), so a 12h advance yielded `/16` not `/2`
- **Fix:** Lifecycle fixtures use `MempoolCapacity::new(0)` so `/4` and `/2` shorteners stay inactive
- **Files modified:** `rolling_fee_cases.rs`, `mempool_lifecycle_cases.rs`
- **Verification:** rolling_fee_decay and node lifecycle tests green
- **Committed in:** `935310d4`

**3. [Rule 1 - Bug] Zero-threshold effective assertion used equal static and incremental**
- **Found during:** Task 1 green
- **Issue:** `assert_ne!(effective, incremental)` failed when both were 1000 sat/kvB
- **Fix:** Raise static floor to 2000 so effective ≠ incremental after rolling zeros
- **Files modified:** `rolling_fee_cases.rs`
- **Verification:** `rolling_fee_decay_zeros_below_incremental_half` green
- **Committed in:** `935310d4`

---

**Total deviations:** 3 auto-fixed (1 blocking, 2 bugs)
**Impact on plan:** Required for hooks and Knots-accurate occupancy/fee-role fixtures. No scope creep into expiry or parity-label flip.

## Issues Encountered

- First commit attempt failed rustfmt check; reformatted and recommitted successfully as `935310d4`

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 03 can add PolicyTime-driven expiry cleanup on top of the live rolling state machine
- Plan 04 can flip `RollingFeeParityStatus` once bump+decay evidence is complete

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-mempool/src/fee/rolling.rs`
- FOUND: `packages/open-bitcoin-mempool/src/pool/tests/rolling_fee_cases.rs`
- FOUND: commit `935310d4`

---
*Phase: 131-rolling-fee-expiry-and-descendant-eviction-core*
*Completed: 2026-07-25*

<!-- gsd-lifecycle-trailer -->
---
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 131-2026-07-24T22-07-47
generated_at: 2026-07-25T04:09:00.000Z
---
