---
phase: 143-honest-stored-block-availability
plan: 04
subsystem: docs
tags: [block-serving, reserved-pruned, phase-111-claims, unavailable, documentation]

requires:
  - phase: 143-honest-stored-block-availability
    provides: Presence facts, has_block probe, LookupUnavailable honesty, renamed unavailable_notfound test
provides:
  - "Claim-scanned Phase 111 docs describe missing payload as Unavailable"
  - "block_status_pruned remains reserved for a future prune-mode delete"
  - "Phase 111 checker and no-claim scan stay green without getblock or archive-node claims"
affects:
  - 144-operator-evidence
  - reserved-pruned
  - phase-111-claim-copy

tech-stack:
  added: []
  patterns:
    - "Missing payload copy is Unavailable plus NotFound; reserved Pruned is future prune-mode delete only"
    - "Phase 111 REQUIRED_TERMS keep block_status_pruned as reserved vocabulary"

key-files:
  created: []
  modified:
    - docs/architecture/status-snapshot.md
    - docs/operator/runtime-guide.md
    - docs/parity/catalog/p2p.md
    - docs/parity/index.json

key-decisions:
  - "Leave HAVL-02 Pending until lifecycle-valid phase verification"
  - "Task 2 needed no checker fixture edits because fixtures already used unavailable_notfound"

patterns-established:
  - "Pattern 1: Claim-scanned Phase 111 copy says reserved for a future prune-mode delete, not this node pruned historical files"
  - "Pattern 2: Keep existing does not add archive-node behavior / public block serving by default sentences"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 143-2026-09-17T01-47-45
generated_at: 2026-09-17T06:11:50Z

duration: 14min
completed: 2026-09-17
---

# Phase 143 Plan 04: Reserved-Pruned Docs and Phase 111 Claim Copy Summary

**Claim-scanned Phase 111 docs now describe missing payload as Unavailable and keep `block_status_pruned` reserved for a future prune-mode delete.**

## Performance

- **Duration:** 14 min
- **Started:** 2026-09-17T05:57:31Z
- **Completed:** 2026-09-17T06:11:50Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Replaced `pruned active non-tip blocks` in the four claim-scanned Phase 111 docs with reserved-Pruned wording.
- Missing payload is documented as `WireNetworkMessage::NotFound` with `block_status_unavailable`. `block_status_pruned` remains reserved vocabulary, not a prune-mode product signal.
- Existing "does not add ... archive-node behavior ... public block serving by default" sentences and the three repo-local runtime-guide commands stayed verbatim.
- Phase 111 checker tests and the live no-claim scan passed without adding `getblock`, archive-node honesty, or public-default serving claims.

## Task Commits

Each task was committed atomically:

1. **Task 1: Rewrite pruned-as-prune-mode copy in the four claim-scanned docs** - `0baf0b7f` (docs)
2. **Task 2: Confirm Phase 111 checker and no-claim scan stay green** - no commit (verify-only; fixtures already matched)

**Plan metadata:** pending docs commit after this summary.

## Files Created/Modified

- `docs/architecture/status-snapshot.md` - Missing payload is Unavailable; reserved Pruned wording
- `docs/operator/runtime-guide.md` - Same reserved-Pruned wording; repo-local Cargo/Bazel/verify commands unchanged
- `docs/parity/catalog/p2p.md` - Request-path paragraph no longer reads as prune-mode
- `docs/parity/index.json` - `v2-1-full-block-serving-request-path` rationale updated; status/evidence/upstream unchanged

## Decisions Made

- Leave HAVL-02 Pending and keep `requirements-completed` empty until lifecycle-valid phase verification exists. Activating HAVL-02 in this summary would fail the active-milestone traceability checker before `143-VERIFICATION.md`.
- Task 2 needed no checker or fixture edits. `REQUIRED_TERMS` still includes `block_status_pruned`, and fixtures already used `phase111_active_chain_non_tip_missing_local_block_returns_unavailable_notfound`.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Operators and the Phase 111 checker can see reserved `Pruned` without reading current missing-payload behavior as prune-mode.
- Phase 144 can add operator evidence surfaces without inheriting prune-mode claim copy from these four docs.
- No new operator UI, `getblock`, archive-node honesty, or public-default serving claim was added.

***
*Phase: 143-honest-stored-block-availability*
*Completed: 2026-09-17*

## Self-Check: PASSED
