---
phase: 145-parity-roots-and-no-claim-guardrails
plan: 01
subsystem: docs
tags: [parity-roots, checklist, breadcrumbs, CheckBlockDataAvailability, CanFlushToDisk]

# Dependency graph
requires:
  - phase: 138-parity-adversarial-pressure-restart-and-release-guardrails
    provides: Closeout surface schema with id/title/status/requirements/evidence/upstream/known_gaps
  - phase: 139-coins-view-cache-contract-and-engine-apply
    provides: CACHE-01 coins-view/cache evidence
  - phase: 140-pure-flush-policy-and-typed-decisions
    provides: FLUSH-01 and MGR-03 typed flush/recovery evidence
  - phase: 141-durable-fjall-coins-adapter
    provides: COIN-01 and CSOBS-03 Fjall coins evidence
  - phase: 142-manager-flush-lifecycle-and-restart
    provides: MGR-01, MGR-02, and FLUSH-02 manager/restart evidence
  - phase: 143-honest-stored-block-availability
    provides: HAVL-01 through HAVL-03 payload-byte availability evidence
  - phase: 144-operator-flush-and-availability-evidence
    provides: CSOBS-01 and CSOBS-02 chainstate_durability contract
provides:
  - Seven in_progress v2.3 index/checklist surfaces with exactly-once ownership of all 15 requirement IDs
  - Current v2.3 chainstate catalog claim with leftover snapshots labeled historical and non-authoritative
  - Dedicated node-stored-block-presence breadcrumb citing node/blockstorage.cpp
affects: [145-02 last-gate checker, 145-03 UAT and claim copy, 145-04 leftover-Pending flip]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - v2.3 surfaces copy the Phase 138 closeout object shape and stay in_progress until Plan 04
    - HaveBlockData is discussion-name only; pinned serve-path symbol is CheckBlockDataAvailability

key-files:
  created: []
  modified:
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/catalog/chainstate.md
    - docs/parity/catalog/p2p.md
    - docs/parity/source-breadcrumbs.json
    - packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/block_presence.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Keep all seven v2.3 surfaces in_progress; the closeout surface owns only CSVFY-01 and CSVFY-02"
  - "Cite CheckBlockDataAvailability as the pinned serve-path symbol; HaveBlockData is the locked discussion name only"
  - "Split has_block files onto node-stored-block-presence citing node/blockstorage.cpp"
  - "Leave requirements-completed empty so CSVFY-01 is not activated before the last-gate checker exists"

patterns-established:
  - "Pattern 1: Distinct Phase 139-144 surfaces plus one CSVFY closeout row, not a competing v2.3-closeout.md"
  - "Pattern 2: Catalog current-claim section above labeled historical Phase 4 snapshot coverage"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 145-2026-09-18T03-36-29
generated_at: 2026-09-18T05:28:30Z

# Metrics
duration: 35min
completed: 2026-09-18
---

# Phase 145 Plan 01: Inventory and Parity-Root Backfill Summary

**Seven in_progress v2.3 parity surfaces with exactly-once owners, a current disk-backed coins catalog claim, and a CheckBlockDataAvailability has_block breadcrumb**

## Performance

- **Duration:** 35 min
- **Started:** 2026-09-18T04:52:19Z
- **Completed:** 2026-09-18T05:28:30Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Every v2.3 requirement ID now has exactly one machine-readable owner in `docs/parity/index.json` and `docs/parity/checklist.md`
- Phase 145 closeout owns only CSVFY-01 and CSVFY-02; CACHE/FLUSH/COIN/MGR/HAVL/CSOBS stay on their Phase 139-144 surfaces
- `catalog/chainstate.md` presents disk-backed coins, flush, manager, and honest availability as the current claim and leftover snapshots as historical non-authoritative blobs
- Serve-path roots name `CheckBlockDataAvailability`; `HaveBlockData` is discussion-name only
- `has_block` files sit on `node-stored-block-presence` citing `node/blockstorage.cpp`

## Task Commits

Each task was committed atomically:

1. **Task 1: Backfill six Phase 139-144 owners and the CSVFY closeout surface** - `67a3d31a` (docs)
2. **Task 2: Refresh chainstate catalog, serve-path honesty, and has_block breadcrumb split** - `6061684d` (docs)

**Plan metadata:** pending final docs commit

## Files Created/Modified
- `docs/parity/index.json` - Seven in_progress v2.3 surfaces after the v2.2 closeout; closeout lists D-06 sources and D-07 differences
- `docs/parity/checklist.md` - Matching human rows for the same seven surface IDs
- `docs/parity/catalog/chainstate.md` - Current v2.3 claim, historical Phase 4 coverage, deferred FUT-21 assumeutxo
- `docs/parity/catalog/p2p.md` - Serve-path sentence naming `CheckBlockDataAvailability` and `blockmanager_tests.cpp`
- `docs/parity/source-breadcrumbs.json` - New `node-stored-block-presence` group; remaining `node-storage-contract` stays explicit `none`
- `packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs` - Breadcrumb comment now cites `blockstorage.cpp`
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/block_presence.rs` - Matching breadcrumb comment
- `docs/metrics/lines-of-code.md` - Hook-refreshed LOC artifact from the Task 2 commit

## Decisions Made
- Kept all seven new surfaces `in_progress` and did not flip REQUIREMENTS checkboxes (D-04 / D-22 / T-145-05)
- Named `CheckBlockDataAvailability` as the pinned Knots serve-path symbol and `CanFlushToDisk` as the manager readiness symbol
- Split only `blocks.rs` and `block_presence.rs` out of the explicit-`none` storage group
- Left `requirements-completed` empty so CSVFY-01 is not activated before Plans 02-04 land the checker and UAT package

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated Rust breadcrumb comments to match the new group**
- **Found during:** Task 2 (catalog, serve-path, breadcrumb split)
- **Issue:** `check-parity-breadcrumbs.ts` requires source breadcrumb blocks to match JSON group breadcrumbs. Moving `has_block` files onto `node-stored-block-presence` would fail check mode if the files kept `none`.
- **Fix:** Replaced the explicit-`none` comments in `blocks.rs` and `block_presence.rs` with `packages/bitcoin-knots/src/node/blockstorage.cpp`. No runtime behavior change.
- **Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/tests/block_presence.rs`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts` exits 0
- **Committed in:** `6061684d` (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Required for the Task 2 breadcrumb checker. No scope creep and no runtime coins/flush/manager/serve change.

## Issues Encountered
None

## Authentication Gates
None

## Known Stubs
Intentional named-now closeout evidence that later plans must create:

- `docs/parity/index.json` and `docs/parity/checklist.md` name `.planning/phases/145-parity-roots-and-no-claim-guardrails/145-UAT.md` (lands in Plan 03)
- `docs/parity/index.json` and `docs/parity/checklist.md` name `scripts/check-phase145-parity-uat-release-boundary.ts` (lands in Plan 02)

These stubs do not prevent Plan 01's inventory/catalog/breadcrumb goal.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Ready for `145-02-PLAN.md` last-gate checker and mutation fixtures
- Surfaces stay `in_progress`; CSVFY/CACHE/MGR rows stay Pending until Plan 04
- Do not archive v2.3 from later plans

---
*Phase: 145-parity-roots-and-no-claim-guardrails*
*Completed: 2026-09-18*

## Self-Check: PASSED
