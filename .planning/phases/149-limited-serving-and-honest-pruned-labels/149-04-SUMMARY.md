---
phase: 149-limited-serving-and-honest-pruned-labels
plan: "04"
subsystem: networking
tags: [pruned-label, have-pruned, block-serving, status-counters]

requires:
  - phase: 149-03
    provides: production Pruned only for a have-pruned active-chain payload gap
provides:
  - Earned-label comments on both Pruned variants
  - Status counter tests for pruned_count versus unavailable_count
affects:
  - 150 operator prune surfaces
  - 151 parity and no-claim closeout

tech-stack:
  added: []
  patterns:
    - "pruned_count moves only for have_pruned && !payload_present on the active chain"
    - "node-limited-serve cites blockstorage.cpp IsBlockPruned"

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/pruned_label.rs
  modified:
    - packages/open-bitcoin-network/src/block_serving.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Leave LABL-01 Pending until lifecycle-valid Phase 149 verification"
  - "Keep operator prune fields out; pruned_count is the only projection"
  - "Cite blockstorage.cpp on the whole node-limited-serve breadcrumb group"

patterns-established:
  - "A missing payload with have_pruned false stays Unavailable even in manual prune mode"
  - "A present payload stays Available when have_pruned is already true"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 149-2026-09-27T19-48-11
generated_at: 2026-09-28T04:08:23Z

duration: 1h 1m
completed: 2026-09-28
---

# Phase 149 Plan 04: Honest Pruned Label Projection Summary

**`pruned_count` increments only for an active-chain gap whose durable have_pruned flag is set, and the reserved Pruned comments now say that rule**

## Performance

- **Duration:** 1h 1m
- **Started:** 2026-09-28T03:08:07Z
- **Completed:** 2026-09-28T04:08:23Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments

- Both `Pruned` variants describe the earned label: durable `have_pruned` is true and the payload is absent. A missing payload without that flag stays `Unavailable`.
- Active-chain getdata after a payload gap increments `pruned_count` only when `serving_have_pruned` is true. The same gap with the flag false, manual prune mode alone, a present payload, and an unknown hash leave `pruned_count` at 0.
- Durability JSON still forbids the key `pruned`. `getblockchaininfo` shape is unchanged. No RPC, CLI, dashboard, or support-bundle prune fields were added.

## Task Commits

Each task was committed atomically:

1. **Task 1: Rewrite reserved Pruned comments** - `5952d96e` (docs)
2. **Task 2: Project pruned_count only for an earned gap** - `fddea08e` (test)

**Plan metadata:** docs commit for this summary

_Note: Task 2 is TDD coverage of the Plan 03 production label. The tests passed on the first run, so there is no separate failing RED commit. Pre-commit runs `bash scripts/verify.sh`._

## Files Created/Modified

- `packages/open-bitcoin-network/src/block_serving.rs` — earned-label comments on both `Pruned` variants
- `packages/open-bitcoin-node/src/network/tests/pruned_label.rs` — counter tests for unavailable, pruned, available, manual mode, and unknown hash
- `packages/open-bitcoin-node/src/network/tests.rs` — `mod pruned_label`
- `docs/parity/source-breadcrumbs.json` — `pruned_label.rs` and `blockstorage.cpp` on `node-limited-serve`
- `packages/open-bitcoin-node/src/network/limited_serve.rs` — matching breadcrumb header
- `packages/open-bitcoin-node/src/network/tests/limited_serve_advertisement.rs` — matching breadcrumb header
- `packages/open-bitcoin-node/src/network/tests/block_serving/limited_window.rs` — matching breadcrumb header
- `docs/metrics/lines-of-code.md` — verifier freshness

## Decisions Made

- LABL-01 stays Pending. This plan forbids flipping it, and active-milestone verification traceability rejects Complete before `149-VERIFICATION.md` exists.
- The projection is the existing `pruned_count` increment. Phase 150 owns operator prune fields.
- `node-limited-serve` cites `packages/bitcoin-knots/src/node/blockstorage.cpp` because Knots `IsBlockPruned` is `m_have_pruned` plus missing block data. Every file in that group carries the same header.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Group breadcrumb headers include blockstorage.cpp**
- **Found during:** Task 2 (pruned counter tests)
- **Issue:** Adding `blockstorage.cpp` to `node-limited-serve` makes the other three files' breadcrumb headers stale. The checker requires each file's header to match the group list exactly.
- **Fix:** Appended that Knots path to `limited_serve.rs`, `limited_serve_advertisement.rs`, and `limited_window.rs`.
- **Files modified:** those three headers plus `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun scripts/check-parity-breadcrumbs.ts` exited 0.
- **Committed in:** `fddea08e` (task commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The header sync is required for the breadcrumb checker. No operator prune surface was added.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 149 plan work is complete. `Pruned` is counted only for an earned active-chain gap. Phase 150 owns operator prune fields. Phase 151 owns the parity-doc pass. LABL-01 stays Pending until lifecycle-valid phase verification.

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-node/src/network/tests/pruned_label.rs
- FOUND: 5952d96e
- FOUND: fddea08e

---
*Phase: 149-limited-serving-and-honest-pruned-labels*
*Completed: 2026-09-28*
