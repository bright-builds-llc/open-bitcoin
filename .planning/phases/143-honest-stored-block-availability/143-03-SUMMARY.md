---
phase: 143-honest-stored-block-availability
plan: 03
subsystem: network
tags: [block-serving, lookup-unavailable, notfound, unavailable, rust]

requires:
  - phase: 143-honest-stored-block-availability
    provides: Presence facts and has_block probe; LookupUnavailable still copied Available
provides:
  - "LookupUnavailable rewrites status_label to Unavailable and payload_present to false"
  - "Post-gate lookup None returns no Block body and missing_inventory true"
  - "Compact announcement and compact-txn keep cache-first managed_block_serve_input inheritance"
affects:
  - 143-04
  - lookup-unavailable
  - reserved-pruned

tech-stack:
  added: []
  patterns:
    - "LookupUnavailable completion rewrites status and payload_present; TransportFailed and Written still clone eligible_decision"
    - "Missing-payload refuse stays WireNetworkMessage::NotFound with no new wire error"

key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/network/block_serving.rs
    - packages/open-bitcoin-node/src/network/block_serving/tests.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined 143-03 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "LookupUnavailable uses 4-arg missing with rewritten presence instead of mutating a 3-arg result"
  - "Leave HAVL-01 and HAVL-02 Pending until later plans and lifecycle-valid phase verification"

patterns-established:
  - "Pattern 1: Post-gate read failure reports Unavailable and increments unavailable_count, not available_count"
  - "Pattern 2: Compact paths inherit Plan 01 cache-only payload_present through managed_block_serve_input"

requirements-completed: [HAVL-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 143-2026-09-17T01-47-45
generated_at: 2026-09-17T05:17:40Z

duration: 26min
completed: 2026-09-17
---

# Phase 143 Plan 03: Missing-Payload Refuse and LookupUnavailable Honesty Summary

**Post-gate missing bytes now rewrite `status_label` to Unavailable and `payload_present` to false, so NotFound cannot increment available_count.**

## Performance

- **Duration:** 26 min
- **Started:** 2026-09-17T04:51:15Z
- **Completed:** 2026-09-17T05:17:40Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- `ManagedBlockServeIntent::completion(LookupUnavailable)` sets `status_label = Unavailable`, `presence.payload_present = false`, and `missing_inventory = true`.
- `serve_managed_block_request` lookup `None` returns no fabricated `Block` body after a successful gate.
- Compact announcement and compact-txn still call `managed_block_serve_input` with no second `has_block` probe.
- Inbound missing-body and store-error redaction tests still expect `NotFound` and `served_count == 0`.

## Task Commits

Each task was committed atomically:

1. **Task 1 + Task 2: LookupUnavailable honesty tests and rewrite** - `507c9167` (feat)

**Plan metadata:** pending docs commit after this summary.

_Note: Combined RED+GREEN in one hook-passing feat commit because pre-commit runs `verify.sh`, matching Phases 140–142 and 143-01/02._

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/block_serving.rs` - LookupUnavailable completion rewrites Unavailable and payload_present false
- `packages/open-bitcoin-node/src/network/block_serving/tests.rs` - Completion, serve-none, source-scan, and compact-inheritance tests
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC

## Decisions Made

- Combined RED+GREEN because hooks run the workspace verifier; a RED-only commit cannot pass pre-commit.
- Keep the Plan 01 4-arg `missing` helper and pass rewritten presence facts, rather than restoring a 3-arg `missing` and mutating the result.
- Leave HAVL-01 and HAVL-02 Pending. Plan 04 still owns reserved-Pruned docs, and phase verification is later.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] rustfmt wrapping on presence rewrite**
- **Found during:** Task 1+2 first hook-passing commit
- **Issue:** `self.eligible_decision.presence.validated_on_active_chain` wrapping failed rustfmt check.
- **Fix:** Split the chain as `self.eligible_decision.presence.validated_on_active_chain`.
- **Files modified:** `packages/open-bitcoin-node/src/network/block_serving.rs`
- **Verification:** `verify.sh` passed on the successful feat commit
- **Committed in:** `507c9167` (combined feat commit)

***

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Narrow format guard. No scope creep. Combined RED+GREEN was an allowed plan exception, not a deviation.

## Issues Encountered

The first hook-passing commit failed on rustfmt wrapping. The formatted file was recommitted successfully.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 04 can update reserved-Pruned docs and Phase 111 claim copy without changing this completion rewrite.
- Inventory, durable-gate, compact inheritance, and post-gate read failure now report Unavailable + existing NotFound.
- No new operator UI, `getblock`, wire error, or prune-mode product behavior was added.

***
*Phase: 143-honest-stored-block-availability*
*Completed: 2026-09-17*

## Self-Check: PASSED
