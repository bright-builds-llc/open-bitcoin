---
phase: 151-parity-roots-and-no-claim-guardrails
plan: "01"
subsystem: parity
tags: [parity, knots, prune, fjall, breadcrumbs]

requires:
  - phase: 150-operator-prune-surfaces-and-evidence
    provides: Shipped operator prune surfaces whose evidence this plan records without changing runtime prune behavior
provides:
  - Six done v2.4 parity surfaces with exactly-once requirement owners
  - Current v2.4 catalog claims for height-window Fjall prune, limited serving, and the operator quartet
  - cli-operator-prune breadcrumbs that cite rpc/blockchain.cpp beside bitcoin-cli.cpp
affects: [151-02, 151-03, 151-04, GRD-01]

tech-stack:
  added: []
  patterns:
    - "v2.4 requirement ownership lives only in docs/parity/index.json and docs/parity/checklist.md"
    - "The frozen Fjall-versus-blk/rev sentence sits beside UnlinkPrunedFiles as a documented difference"

key-files:
  created: []
  modified:
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/catalog/chainstate.md
    - docs/parity/catalog/p2p.md
    - docs/parity/catalog/rpc-cli-config.md
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Ship the six surfaces and the catalog claim refresh in one hook-passing commit because pre-commit runs verify.sh."
  - "Refresh the cli-operator-prune source comments so they match the new rpc/blockchain.cpp breadcrumb."
  - "Leave requirements-completed empty so GRD-01 stays unchecked until a later plan's checker can name the evidence."

patterns-established:
  - "Each v2.4 requirement ID has one checklist owner, and the closeout surface owns only GRD-01."
  - "Current v2.4 prune sentences stay separate from the retained v2.3 foundation sentence."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 151-2026-09-29T21-06-33
generated_at: 2026-09-30T01:28:41Z

duration: 53 min
completed: 2026-09-30
---

# Phase 151 Plan 01: Backfill v2.4 Parity Roots Summary

**Six done v2.4 parity surfaces, current height-window Fjall prune claims, and an operator breadcrumb that cites `rpc/blockchain.cpp` beside `bitcoin-cli.cpp`**

## Performance

- **Duration:** 53 min
- **Started:** 2026-09-30T00:35:38Z
- **Completed:** 2026-09-30T01:28:41Z
- **Tasks:** 3
- **Files modified:** 9

## Accomplishments

- Every v2.4 requirement now has one machine-readable owner. The closeout surface owns only GRD-01.
- The chainstate, P2P, and RPC catalogs state height-window Fjall prune, `NODE_NETWORK_LIMITED` serving, and the `getblockchaininfo` prune quartet beside the frozen Knots difference.
- `cli-operator-prune` cites `packages/bitcoin-knots/src/rpc/blockchain.cpp` as well as `packages/bitcoin-knots/src/bitcoin-cli.cpp`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add six done v2.4 surfaces with exactly-once owners** - `14d4418c` (feat)
2. **Task 2: Record the v2.4 prune claim and frozen Knots symbols in the existing catalogs** - `14d4418c` (feat)
3. **Task 3: Cite rpc/blockchain.cpp from the operator prune breadcrumb group** - `a1366f0c` (feat)

**Plan metadata:** recorded in the docs commit for this summary.

## Files Created/Modified

- `docs/parity/index.json` - Six `done` v2.4 names and checklist objects. GRD-01 is the only closeout requirement.
- `docs/parity/checklist.md` - Matching human rows after the v2.3 closeout row.
- `docs/parity/catalog/chainstate.md` - Past-tense v2.3 reserved-label note, plus `## Current v2.4 claim` with `UnlinkPrunedFiles` and the frozen difference sentence.
- `docs/parity/catalog/p2p.md` - `## Current v2.4 limited serving`, and `block_status_pruned` only after have-pruned plus a missing payload.
- `docs/parity/catalog/rpc-cli-config.md` - `## Current v2.4 operator prune` naming the quartet, lock methods, and the `clearprunelock` extension.
- `docs/parity/source-breadcrumbs.json` - `cli-operator-prune` now includes `rpc/blockchain.cpp`.
- `packages/open-bitcoin-cli/src/operator/prune.rs` - Source breadcrumb comment matches the JSON group.
- `packages/open-bitcoin-cli/src/operator/tests/routing/prune.rs` - Source breadcrumb comment matches the JSON group.
- `docs/metrics/lines-of-code.md` - Pre-commit refreshed the tracked line-count report.

## Decisions Made

- Tasks 1 and 2 share `14d4418c` because the plan requires the catalog edit in the same hook-passing commit as the new surfaces.
- In-source breadcrumb comments for the two `cli-operator-prune` Rust files were updated with the JSON group. The checker treats a group change as a stale comment.
- `requirements-completed` stays empty. `.planning/REQUIREMENTS.md` still has `- [ ] **GRD-01**`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Refresh cli-operator-prune source comments**

- **Found during:** Task 3 (Cite rpc/blockchain.cpp from the operator prune breadcrumb group)
- **Issue:** Adding `packages/bitcoin-knots/src/rpc/blockchain.cpp` to the JSON group made the existing comments in `operator/prune.rs` and `operator/tests/routing/prune.rs` stale. `scripts/check-parity-breadcrumbs.ts` failed verify.sh.
- **Fix:** Appended that path to both breadcrumb comment blocks. The `files` array and the other prune groups were left unchanged.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/prune.rs`, `packages/open-bitcoin-cli/src/operator/tests/routing/prune.rs`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check` reported 906 Rust files verified, then the hook-passing commit succeeded.
- **Committed in:** `a1366f0c` (Task 3 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The comment refresh is the checker counterpart of the planned JSON breadcrumb. Tasks 1 and 2 sharing a commit follows the plan's same-commit instruction.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 02 can add the last-gate checker against these surfaces, catalog sentences, and the `cli-operator-prune` breadcrumb. GRD-01 remains unchecked. The closeout evidence list already names `151-UAT.md` and `scripts/check-phase151-parity-uat-release-boundary.ts`; later plans create those files. No `blk`/`rev` store, runtime prune change, or milestone archive was added.

Ready for 151-02.

## Self-Check: PASSED

- FOUND: docs/parity/index.json, checklist.md, catalog pages, source-breadcrumbs.json, and 151-01-SUMMARY.md
- FOUND: 14d4418cf15b6ed20873410090b452cb995467b4
- FOUND: a1366f0c783428f21c1a19f5a604a19ac99506a0
- FOUND: GRD-01 still unchecked
- FOUND: no docs/parity/v2.4-closeout.md

---
*Phase: 151-parity-roots-and-no-claim-guardrails*
*Completed: 2026-09-30*
