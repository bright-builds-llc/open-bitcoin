---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "18"
subsystem: network-lifecycle
tags: [rust, peer-identity, orphan-cursors, reconciliation, unbroadcast]
requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    provides: "Plan 13 authoritative lifecycle projections and exact read-only reconciliation"
provides:
  - "Stored txid+wtxid orphan identities carried through exact peer teardown"
  - "Symmetric peer cursor reconciliation across txid and wtxid identity domains"
  - "Symmetric retry-eligible canonical versus actual unbroadcast reconciliation"
affects: [phase-134-verification, peer-transaction-lifecycle, lifecycle-reconciliation]
tech-stack:
  added: []
  patterns:
    - "Resolve stored complete identities before teardown rather than reconstructing aliases from canonical inputs"
    - "Audit expected and actual bounded identity sets through symmetric difference without repair"
key-files:
  created:
    - packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_cases/identity_aliases.rs
  modified:
    - packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs
    - packages/open-bitcoin-network/src/peer/transaction_lifecycle/reconciliation.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs
    - packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/reconciliation.rs
    - scripts/check-phase133-package-aware-download-orphan-bridge.ts
key-decisions:
  - "Candidate cursors retain complete child txid+wtxid identities, still without retaining child transaction bodies."
  - "Expected unbroadcast membership is the retry-eligible subset of canonical mempool identities."
  - "MPLIFE-01 through MPLIFE-04 remain pending until phase re-verification."
patterns-established:
  - "Identity teardown: carry the exact stored identity into every request, known, orphan, and cursor cleanup domain."
  - "Read-only audit: derive an expected bounded set and count its symmetric difference with actual state."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-29T11:10:14Z
duration: 1h 52m
completed: 2026-07-29
---

# Phase 134 Plan 18: Identity Alias and Symmetric Reconciliation Summary

**Stored orphan aliases now terminate every peer identity cursor, while unbroadcast audits detect both missing and unexpected retry-eligible members**

## Performance

- **Duration:** 1h 52m
- **Started:** 2026-07-29T09:18:00Z
- **Completed:** 2026-07-29T11:10:14Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Carried each removed orphan's stored txid and wtxid through peer teardown so canonical inputs cannot leave alias-keyed request, known, orphan, or candidate-cursor residue.
- Retained complete child identities in bounded candidate cursors and made reconciliation compare both txid and wtxid domains.
- Added same-txid/different-wtxid and multiple-alias regressions with auditable Bitcoin Knots breadcrumbs.
- Derived retry-eligible canonical unbroadcast membership and counted its symmetric difference with actual state, including missing-only and equal-cardinality swap corruption.

## Task Commits

Each task was committed atomically:

1. **Task 1: Carry actual orphan identities through cursor teardown** - `cdebcc54` (fix)
2. **Task 2: Make unbroadcast reconciliation symmetric** - `cd149487` (fix)

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_cases/identity_aliases.rs` - Same-txid/different-wtxid, multiple-alias, and clean reconciliation regressions.
- `packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_cases.rs` - Registers the identity-alias regression module.
- `packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs` - Carries stored orphan identities through exact teardown.
- `packages/open-bitcoin-network/src/peer/transaction_lifecycle/reconciliation.rs` - Reconciles cursor identity overlap in both txid and wtxid domains.
- `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs` - Applies complete child identities during ordinary cursor cleanup.
- `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs` - Retains bounded complete child identities without child transaction bodies.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs` - Derives retry-eligible expected unbroadcast membership and audits symmetric differences.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/reconciliation.rs` - Covers extra-only, missing-only, swapped, clean, and exact mismatch identities.
- `scripts/check-phase133-package-aware-download-orphan-bridge.ts` - Keeps the identity-only cursor evidence contract current.
- `scripts/check-phase133-package-aware-download-orphan-bridge.test.ts` - Mutates the new cursor shape while preserving the no-child-body guard.
- `docs/parity/source-breadcrumbs.json` - Registers the new Rust regression file's Knots anchors.
- `docs/metrics/lines-of-code.md` - Hook-refreshed tracked source metrics.

## Decisions Made

- Stored orphan identity is authoritative for teardown; incoming canonical teardown identities are supplemental aliases, not a source from which stored wtxids may be reconstructed.
- Candidate cursors retain both txid and wtxid because either domain can be the only surviving reconciliation signal, while retaining child transaction bodies remains forbidden.
- Unbroadcast expectation is derived only from canonical entries whose metadata is retry eligible, and reconciliation remains a deterministic read-only audit.
- MPLIFE-01 through MPLIFE-04 remain pending for the phase verifier; this gap plan does not claim requirement completion.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Retained complete identities in candidate cursors**

- **Found during:** Task 1 (Carry actual orphan identities through cursor teardown)
- **Issue:** The planned lifecycle files could not reconcile a cursor after its orphan record was removed because the cursor retained only a wtxid and no longer had a txid-side identity.
- **Fix:** Added a bounded private `CandidateChildIdentity` pair and updated cursor cleanup and retained-byte accounting without retaining child bodies.
- **Files modified:** `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs`
- **Verification:** Alias regressions, the broader peer lifecycle suite, warnings-denied Clippy, full workspace tests, and the normal repository verifier passed.
- **Committed in:** `cdebcc54`

**2. [Rule 3 - Blocking] Updated the Phase 133 identity-only cursor evidence guard**

- **Found during:** Task 1 normal commit hook
- **Issue:** The Phase 133 static checker hard-coded the superseded `child_wtxids` field and rejected the complete-identity cursor even though it still retained no child bodies.
- **Fix:** Made the checker require `CandidateChildIdentity` storage, forbid transaction bodies in both the cursor and identity record, and updated its child-body mutation.
- **Files modified:** `scripts/check-phase133-package-aware-download-orphan-bridge.ts`, `scripts/check-phase133-package-aware-download-orphan-bridge.test.ts`
- **Verification:** All 30 Phase 133 checker tests passed, including the child-body-retention mutation, before the normal hook passed.
- **Committed in:** `cdebcc54`

**3. [Rule 1 - Bug] Corrected inherited STATE plan position**

- **Found during:** Plan metadata update
- **Issue:** Although Plan 17 already had a completed summary, STATE still pointed at Plan 17, so the required first advance routed to the now-completed Plan 18.
- **Fix:** Applied one corrective advance after Plan 18's summary existed so the next action truthfully routes to Plan 19.
- **Files modified:** `.planning/STATE.md`
- **Verification:** STATE now reports Plan 19 of 24 and `Next action: Execute 134-19-PLAN.md`.
- **Committed in:** Plan metadata commit

**Total deviations:** 3 auto-fixed (2 blocking, 1 bug)
**Impact on plan:** The implementation deviations preserve bounded identity-only semantics and the evidence contract; the metadata correction restores truthful routing. No feature scope was added.

## Issues Encountered

- Both TDD RED stages were executed and observed, but separate failing-test commits were omitted because repository instructions require formatting, warnings-denied Clippy, all-target build, and all-feature tests to pass before every Rust commit. Each GREEN implementation was committed atomically after the full gate.
- Task 1's first normal hook correctly exposed the stale Phase 133 field-shape checker; the checker and its mutation test were updated before retrying the commit.
- Task 2's all-feature test run exceeded its advisory threshold during macOS binary startup. Process sampling showed the RPC test binary entirely in `_dyld_start`; the suite was not interrupted and ultimately passed.

## Threat Model Closure

- Stored complete identities mitigate alias tampering and stale cursor disclosure across txid and wtxid boundaries.
- Symmetric set differences make missing and unexpected state equally attributable, including equal-cardinality swaps.
- Production reconciliation remains read-only; no repair path, network endpoint, authentication path, file-access boundary, or schema change was introduced.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Peer alias teardown and unbroadcast membership audits now cover both identity directions without mutating runtime state.
- Phase 134 may continue with Plan 19; MPLIFE-01 through MPLIFE-04 remain pending until formal re-verification.

## Self-Check: PASSED

- Summary and the new identity-alias regression file exist.
- Task commits `cdebcc54` and `cd149487` exist in repository history.
- MPLIFE-01 through MPLIFE-04 remain pending in both the checklist and traceability table.

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-29*
