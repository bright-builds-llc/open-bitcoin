---
phase: 131-rolling-fee-expiry-and-descendant-eviction-core
plan: 04
subsystem: mempool
tags: [mempool, evidence, accounted_memory, rolling-fee, PRESS-01, PRESS-03, legacy_vsize]

requires:
  - phase: 131-rolling-fee-expiry-and-descendant-eviction-core
    provides: Accounted trim/bump (01), block-gated decay (02), and expiry (03)
provides:
  - MempoolCapacityEnforcement::AccountedMemory / "accounted_memory"
  - RollingFeeParityStatus::Active / "active"
  - Deleted PolicyConfig.legacy_vsize_trim_limit live seam
  - Historicalized Phase 130 checker + intentional eviction tie-break deviation
affects:
  - 131-05 sustained-pressure oracle scenarios
  - Operator getmempoolinfo capacityenforcement / rolling_fee_parity surfaces

tech-stack:
  added: []
  patterns:
    - Low-cardinality live evidence enums with sole AccountedMemory / Active variants
    - Phase checkers retain historical seams via SUMMARY/catalog archive wording

key-files:
  created: []
  modified:
    - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
    - packages/open-bitcoin-mempool/src/types.rs
    - packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs
    - packages/open-bitcoin-mempool/src/pool/tests/pressure_cases.rs
    - packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs
    - packages/open-bitcoin-mempool/tests/parity.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - scripts/check-phase130-resource-time-fee-primitives.ts
    - scripts/check-phase130-resource-time-fee-primitives.test.ts
    - docs/parity/catalog/mempool-policy.md
    - docs/parity/index.json
    - docs/parity/deviations-and-unknowns.md

key-decisions:
  - "Delete LegacyVsize / Deferred live variants entirely (no dual labels)"
  - "Phase 130 checker asserts historical SUMMARY/catalog wording, not live pool.rs seam"
  - "Record descendant-score + txid tie-break as intentional Knots difference (D-04)"

patterns-established:
  - "Evidence flip lands with historical checker rewrite so verify cannot resurrect dual limiters"
  - "Parity catalog keeps fixed `legacy_vsize` during Phase 130 only as historical prose"

requirements-completed: []
requirements-addressed: [PRESS-01, PRESS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 131-2026-07-24T22-07-47
generated_at: 2026-07-25T08:02:08.000Z

duration: 112min
completed: 2026-07-25
---

# Phase 131 Plan 04: Evidence Flip + Delete legacy_vsize Seam Summary

**Live operator evidence is accounted_memory / active; PolicyConfig.legacy_vsize_trim_limit deleted and Phase 130 checker historicalized (PRESS-01/03)**

## Performance

- **Duration:** 112 min
- **Started:** 2026-07-25T06:09:39Z
- **Completed:** 2026-07-25T08:02:08Z
- **Tasks:** 2
- **Files modified:** 19

## Accomplishments

- Flipped `MempoolCapacityEnforcement` to sole `AccountedMemory` (`accounted_memory`) and `RollingFeeParityStatus` to sole `Active` (`active`) across pressure summaries, node, RPC, and CLI fixtures
- Deleted `PolicyConfig.legacy_vsize_trim_limit` / `DEFAULT_LEGACY_VSIZE_TRIM_LIMIT` so no fixture can re-enable vsize trim
- Updated parity catalog/index for Phase 131 live enforcement and recorded intentional eviction-order txid tie-break deviation
- Rewrote Phase 130 `checkLegacyEnforcementSeam` to require historical SUMMARY/catalog wording without live `legacy_vsize_trim_limit`

## Task Commits

Each task was committed atomically:

1. **Task 1: Flip enums, delete legacy_vsize field, update call sites** (+ Rule 3 checker/catalog historicalization required for green verify) - `51f137e1` (feat)
2. **Task 2: Update parity docs and Phase 130 checker historical seam** (remaining index/deviations intentional-difference entry) - `fd03ffca` (docs)

**Plan metadata:** _(docs commit follows)_

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - AccountedMemory / Active evidence enums and pressure_summary
- `packages/open-bitcoin-mempool/src/types.rs` - Removed legacy_vsize_trim_limit from PolicyConfig
- `packages/open-bitcoin-mempool/src/pool/tests/{lifecycle,pressure,resource}_cases.rs` - Updated assertions and fixtures
- `packages/open-bitcoin-node` / `open-bitcoin-rpc` / CLI / bench fixtures - Operator evidence strings
- `scripts/check-phase130-resource-time-fee-primitives.{ts,test.ts}` - Historical seam lock
- `docs/parity/catalog/mempool-policy.md` - accounted_memory + historical Phase 130 note + txid tie-break
- `docs/parity/index.json` / `deviations-and-unknowns.md` - Intentional difference + Phase 131 ownership wording

## Decisions Made

- No dual live labels: LegacyVsize and Deferred variants are gone
- Keep Phase 130 checker historically true via 130-09 SUMMARY + catalog archive sentence
- Leave live accounted_memory asserts for Plan 05 Phase 131 checker rather than expanding Phase 130

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Landed Phase 130 checker/catalog historicalization with Task 1 feat commit**
- **Found during:** Task 1 commit
- **Issue:** Pre-commit `verify.sh` fails while `checkLegacyEnforcementSeam` still requires live `legacy_vsize_trim_limit` / `LegacyVsize`
- **Fix:** Historicalized checker + matching bun mutation test and catalog prose in the same feat commit so seam deletion can land green
- **Files modified:** `scripts/check-phase130-resource-time-fee-primitives.ts`, `.test.ts`, `docs/parity/catalog/mempool-policy.md`
- **Verification:** full pre-commit `verify.sh` via gsd-tools commit
- **Committed in:** `51f137e1`

**2. [Rule 2 - Correctness] Updated CLI/bench capacityenforcement fixtures beyond plan file list**
- **Found during:** Task 1 call-site sweep
- **Issue:** Stale `"legacy_vsize"` strings in CLI/bench fixtures would diverge from live RPC evidence
- **Fix:** Set fixtures to `"accounted_memory"`
- **Files modified:** `packages/open-bitcoin-cli/**`, `packages/open-bitcoin-bench/src/cases/operator_runtime.rs`
- **Verification:** workspace verify via Task 1 commit
- **Committed in:** `51f137e1`

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 correctness)
**Impact on plan:** Required for hooks and operator-truth consistency. No STATE/ROADMAP updates (per executor prompt).

## Issues Encountered

- First Task 1-only commit attempt failed verify on Phase 130 live-seam check; resolved by bundling checker historicalization into the feat commit

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 05 can assert live accounted_memory / active labels in sustained-pressure oracle scenarios and a dedicated Phase 131 checker
- Operators now see truthful capacity enforcement and rolling-fee parity after bump/decay/expiry

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` contains `AccountedMemory` / `accounted_memory` / `Active`
- FOUND: `rg legacy_vsize_trim_limit packages --glob '*.rs'` returns 0
- FOUND: commit `51f137e1`
- FOUND: commit `fd03ffca`
- FOUND: `docs/parity/catalog/mempool-policy.md` contains `accounted_memory` and historical `fixed \`legacy_vsize\` during Phase 130`
- FOUND: `docs/parity/index.json` contains `mempool-eviction-txid-tie-break`

---
*Phase: 131-rolling-fee-expiry-and-descendant-eviction-core*
*Completed: 2026-07-25*
