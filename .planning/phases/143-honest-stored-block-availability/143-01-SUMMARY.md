---
phase: 143-honest-stored-block-availability
plan: 01
subsystem: network
tags: [block-serving, presence-facts, pruned-reserved, payload-present, unavailable, rust]

requires:
  - phase: 111-full-block-serving-request-path
    provides: I/O-free classifier, Phase 111 missing-body test, NotFound refuse path
  - phase: 142-manager-flush-lifecycle-and-restart
    provides: Deferred honest payload-present availability to Phase 143
provides:
  - "BlockServingPresenceFacts on ManagedBlockServeInput and ManagedBlockServeDecision"
  - "Production data_availability Available only when payload_present"
  - "Missing payload classified Unavailable, not Pruned"
  - "Phase 111 missing-body test renamed unavailable_notfound"
affects:
  - 143-02
  - 143-03
  - durable-payload-probe
  - reserved-pruned

tech-stack:
  added: []
  patterns:
    - "Presence facts sit beside BlockServingStatusFacts on the shell report seam"
    - "Available is authorized only by payload_present; index_known and validated_on_active_chain are report-only"

key-files:
  created:
    - packages/open-bitcoin-node/src/network/block_serving/tests.rs
  modified:
    - packages/open-bitcoin-node/src/network/block_serving.rs
    - packages/open-bitcoin-node/src/network/inventory.rs
    - packages/open-bitcoin-node/src/network/tests/block_serving.rs
    - packages/open-bitcoin-network/src/block_serving.rs
    - scripts/check-phase111-full-block-serving-request-path.ts
    - scripts/check-phase111-full-block-serving-request-path.test.ts
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined 143-01 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "BlockServingPresenceFacts sits beside BlockServingStatusFacts on the shell report seam"
  - "SideChain chain_position still uses cache_present, not durable_payload_present"
  - "Leave HAVL-02 and HAVL-03 Pending until later plans and lifecycle-valid phase verification"

patterns-established:
  - "Pattern 1: Shell assembles payload_present, index_known, and validated_on_active_chain; the classifier stays I/O-free"
  - "Pattern 2: Production missing payload is Unavailable; Pruned remains a reserved unused mapping"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 143-2026-09-17T01-47-45
generated_at: 2026-09-17T03:54:20Z

duration: 23min
completed: 2026-09-17
---

# Phase 143 Plan 01: Presence Facts and Reserved Pruned Summary

**Managed serve decisions now expose distinguishable `payload_present`, `index_known`, and `validated_on_active_chain` facts, and production missing payload is Unavailable instead of invented Pruned.**

## Performance

- **Duration:** 23 min
- **Started:** 2026-09-17T03:31:19Z
- **Completed:** 2026-09-17T03:54:20Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- `BlockServingPresenceFacts` is copied onto every `ManagedBlockServeInput` and `ManagedBlockServeDecision`. `Available` is authorized only by `payload_present`.
- Production `managed_block_serve_input` no longer injects `BlockServingDataAvailability::Pruned`. Missing cache plus `durable_payload_present=false` is `Unavailable`.
- The Phase 111 missing-body test and checker now require `phase111_active_chain_non_tip_missing_local_block_returns_unavailable_notfound`. Reserved `Pruned` rustdoc and classifier mapping stay.
- Adapter unit tests moved to `block_serving/tests.rs` so the production file stays under the 628-line trigger.

## Task Commits

Each task was committed atomically:

1. **Task 1 + Task 2: Presence-fact tests and honest availability** - `9cde0089` (feat)

**Plan metadata:** pending docs commit after this summary.

_Note: Combined RED+GREEN in one hook-passing feat commit because pre-commit runs `verify.sh`, matching Phases 140–142._

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/block_serving.rs` - Presence facts on input/decision; tests extracted
- `packages/open-bitcoin-node/src/network/block_serving/tests.rs` - Moved adapter unit tests with eligible presence defaults
- `packages/open-bitcoin-node/src/network/inventory.rs` - Honest `durable_payload_present` assembly without Pruned
- `packages/open-bitcoin-node/src/network/tests/block_serving.rs` - Unavailable-not-Pruned and presence-fact tests
- `packages/open-bitcoin-network/src/block_serving.rs` - Reserved-for-future-prune-delete rustdoc
- `scripts/check-phase111-full-block-serving-request-path.ts` - Required test name flip
- `scripts/check-phase111-full-block-serving-request-path.test.ts` - Fixture name flip
- `docs/parity/source-breadcrumbs.json` - New adapter test path
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC

## Decisions Made

- Combined RED+GREEN because hooks run the workspace verifier; a RED-only commit cannot pass pre-commit.
- Sit the three bools beside `BlockServingStatusFacts` rather than extending the I/O-free classifier input.
- Keep `SideChain` chain position on cache presence so a durable-only probe cannot reclassify an unknown hash as a side-chain body.
- Leave HAVL-02 and HAVL-03 Pending. Plan 02 still owns the durable probe, and Plan 03 still owns `LookupUnavailable` status honesty.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] SideChain stays cache-only**
- **Found during:** Task 2
- **Issue:** Using `payload_present` (cache OR durable) for `SideChain` would let a durable-only probe reclassify an off-active hash as a side-chain body.
- **Fix:** Keep `cache_present` for chain-position SideChain; `payload_present` still authorizes `Available`.
- **Files modified:** `packages/open-bitcoin-node/src/network/inventory.rs`
- **Verification:** Node `network::` tests passed, including side-chain NotFound coverage
- **Committed in:** `9cde0089` (combined feat commit)

---

**Total deviations:** 1 auto-fixed (1 missing critical)
**Impact on plan:** Narrow correctness guard. No scope creep. Combined RED+GREEN was an allowed plan exception, not a deviation.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 02 can replace `gate_inventory_for_durable_serving(..., true)` with a real `has_block` probe result.
- Plan 03 can rewrite `LookupUnavailable` `status_label` to Unavailable without changing this fact-assembly seam.
- No new operator UI, `getblock`, or prune-mode product behavior was added.

---
*Phase: 143-honest-stored-block-availability*
*Completed: 2026-09-17*

## Self-Check: PASSED
