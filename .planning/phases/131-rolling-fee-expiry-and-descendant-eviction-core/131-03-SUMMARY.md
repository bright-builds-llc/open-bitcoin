---
phase: 131-rolling-fee-expiry-and-descendant-eviction-core
plan: 03
subsystem: mempool
tags: [mempool, expiry, PolicyTime, ManagedNetworkHandle, PRESS-04, LegacyUnknown]

requires:
  - phase: 131-rolling-fee-expiry-and-descendant-eviction-core
    provides: Accounted trim/bump (01) and block-gated rolling decay (02)
provides:
  - Pure Mempool::expire(PolicyTime) with Knots-shaped age cutoff
  - Expiry Direct/Descendant lifecycle removals and ledger recompute cleanup
  - ManagedNetworkHandle::expire_mempool authority hook with serving projection
affects:
  - 131-04 evidence/seam retirement
  - 131-05 sustained-pressure oracle scenarios
  - Phase 136 receive-independent maintenance timers

tech-stack:
  added: []
  patterns:
    - Injected PolicyTime expiry cutoff; skip LegacyUnknown without inventing times
    - Authority-only expire mutate with existing Expiry → serving status maps

key-files:
  created:
    - packages/open-bitcoin-mempool/src/pool/expiry.rs
    - packages/open-bitcoin-mempool/src/pool/tests/expiry_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/types.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-mempool/src/lib.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/tests.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Aged Known roots are Direct; collected children are Descendant even if also aged"
  - "LegacyUnknown entries are skipped (A1/D-12) so mixed recovery pools remain operable"
  - "No Phase 136 timers — API + ManagedNetworkHandle hook + hermetic tests only"

patterns-established:
  - "expire uses collect_descendants + single recompute_state like pressure trim"
  - "Shell samples PolicyTime and mutates solely through ManagedNetworkHandle"

requirements-completed: []
requirements-addressed: [PRESS-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 131-2026-07-24T22-07-47
generated_at: 2026-07-25T05:51:40.000Z

duration: 90min
completed: 2026-07-25
---

# Phase 131 Plan 03: Pure Expiry API + Authority Hook Summary

**Knots Expire-shaped pure mempool cleanup with injected PolicyTime, LegacyUnknown skip, and ManagedNetworkHandle hook (PRESS-04)**

## Performance

- **Duration:** 90 min
- **Started:** 2026-07-25T04:21:56Z
- **Completed:** 2026-07-25T05:51:40Z
- **Tasks:** 2
- **Files modified:** 24

## Accomplishments

- Added `PolicyConfig.mempool_expiry_hours` (default 336) and pure `Mempool::expire(now)` that stages aged `Known` roots, removes descendant packages, and emits `MempoolRemovalCause::Expiry` with Direct/Descendant roles
- Skipped `LegacyUnknown` acceptance times without inventing clocks or sampling `SystemTime` in core
- Wired `ManagedNetworkHandle::expire_mempool` / `ManagedPeerNetwork::expire_mempool` through the sole mutate authority with existing Expiry → serving cleanup

## Task Commits

Each task was committed atomically:

1. **Tasks 1–2: PRESS-04 expiry fixtures + pure expire + authority hook** - `58a62e85` (feat)

**Plan metadata:** _(docs commit follows)_

_Note: Pre-commit `verify.sh` rejects intentionally failing RED tests, so Task 1 RED could not land as a separate commit. Fixtures and implementation shipped together once green._

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/pool/expiry.rs` - Expire-shaped pure API
- `packages/open-bitcoin-mempool/src/pool/tests/expiry_cases.rs` - Age, descendant, LegacyUnknown, cause, overflow fixtures
- `packages/open-bitcoin-mempool/src/types.rs` - `DEFAULT_MEMPOOL_EXPIRY_HOURS` + `mempool_expiry_hours`
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` - Network expire + serving projection
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - Authority `expire_mempool` hook
- `docs/parity/source-breadcrumbs.json` - Expiry sources under mempool-policy

## Decisions Made

- Prefer skip over fail-closed for `LegacyUnknown` during sweeps (RESEARCH A1 / D-12)
- Keep expire separate from admission trim; no Phase 136 periodic maintenance loop
- Clamp overflowing `mempool_expiry_hours` to `i64::MAX` so cutoff math stays defined

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Combined Task 1 RED commit into the green feat commit**
- **Found during:** Task 1 commit
- **Issue:** Pre-commit runs full `verify.sh`, which fails on intentionally red PRESS-04 fixtures
- **Fix:** Implemented Tasks 1–2 then committed once green
- **Files modified:** all plan files
- **Verification:** `cargo test -p open-bitcoin-mempool --lib expiry_` and `cargo test -p open-bitcoin-node --lib expire_mempool`
- **Committed in:** `58a62e85`

**2. [Rule 1 - Bug] Coverage/deny-warnings on unused DEFAULT re-export and unreachable try_from Err arm**
- **Found during:** Task 2 commit verify
- **Issue:** Private-module `pub use DEFAULT_MEMPOOL_EXPIRY_HOURS` failed `-D unused-imports`; prior `Err(_)` fallback was llvm-cov uncovered
- **Fix:** Drop unused re-export; clamp with `unwrap_or(i64::MAX)` and add overflow + constant fixtures
- **Files modified:** `pool/expiry.rs`, `pool/tests/expiry_cases.rs`
- **Verification:** full pre-commit `verify.sh` via gsd-tools commit
- **Committed in:** `58a62e85`

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Required for hooks and coverage. No scope creep into evidence flip or Phase 136 timers.

## Issues Encountered

- First commit attempts failed verify on coverage then unused-import under `-D warnings`; fixed and recommitted successfully as `58a62e85`

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 04 can flip capacity-enforcement / rolling-fee evidence labels and retire the legacy vsize seam
- Plan 05 can include expiry steps in sustained fill/trim/block/decay/expiry oracle scenarios

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-mempool/src/pool/expiry.rs`
- FOUND: `packages/open-bitcoin-mempool/src/pool/tests/expiry_cases.rs`
- FOUND: commit `58a62e85`

---
*Phase: 131-rolling-fee-expiry-and-descendant-eviction-core*
*Completed: 2026-07-25*
