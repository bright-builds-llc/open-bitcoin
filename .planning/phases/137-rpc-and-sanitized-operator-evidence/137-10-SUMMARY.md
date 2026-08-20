---
phase: 137-rpc-and-sanitized-operator-evidence
plan: 10
subsystem: rpc
tags: [sendrawtransaction, dual-state, relay_disabled, openbitcoinpackage, status-redaction]

# Dependency graph
requires:
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Plan 06 typed openbitcoinpackage dual-state projector
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Plan 05 identifier-free mempool snapshot groups
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Plan 07 collector copy without last-package tables
provides:
  - Regression lock that sendrawtransaction success JSON stays txid_hex/replaced_txids/evicted_txids
  - Relay-disabled accept/still-present proof on openbitcoinpackage
  - Status and operator JSON proofs that last-package members and propagation claims stay off shared evidence
affects:
  - 137-11 remaining submit fanout receipts
  - Phase 138 Knots object-vs-hex sendrawtransaction closeout

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Dual-state lives only on the Open Bitcoin extension and identifier-free aggregates
    - D-18 forbids claim keys on extension/aggregates; Phase 105 relay.public_relay stays a non-claim capability slot

key-files:
  created:
    - packages/open-bitcoin-rpc/src/dispatch/tests/dual_state.rs
  modified:
    - packages/open-bitcoin-rpc/src/dispatch/tests/transaction_methods.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/tests/mempool_policy.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Keep SendRawTransactionResponse as the three-field txid object; do not add dual-state or rewrite it to a Knots hex string"
  - "Prove D-15 on openbitcoinpackage plus mempool.admission/retry aggregates, not on BaselineParity sendrawtransaction"
  - "Scope D-18 claim-key bans to the typed package report and mempool aggregates so Phase 105 relay.public_relay is not deleted"

patterns-established:
  - "Pattern 1: Freeze tests deserialize success JSON as a map and lock the exact key set"
  - "Pattern 2: EligibleServe/served is a relay token only; admission stays accepted/still-present/cleared"

requirements-completed: [MPOBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-19T23:58:40Z

# Metrics
duration: 5min
completed: 2026-08-19
---

# Phase 137 Plan 10: Dual-State Axes And sendrawtransaction Freeze Summary

**Regression tests lock BaselineParity `sendrawtransaction` to the three-field txid object and prove relay-disabled local accept as dual-state without echoing last-package members or adding propagation claims.**

## Performance

- **Duration:** 5 min
- **Started:** 2026-08-19T23:53:40Z
- **Completed:** 2026-08-19T23:58:40Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Froze `sendrawtransaction` success JSON and `SendRawTransactionResponse` to `txid_hex`, `replaced_txids`, and `evicted_txids` with no admission, relay, or propagation keys.
- Proved a relay-disabled `openbitcoinpackage` submit can be `accepted` then `still-present` while member `relay` is `relay_disabled`, and that shared admission/retry counts move without a member table.
- Locked `served` as a relay token that does not label admission `cleared`.
- Added `json_status_has_no_last_package_members` so operator JSON status stays identifier-free.

## Task Commits

Each task was committed atomically:

1. **Task 1: Freeze sendrawtransaction key set** - `dc26a471` (test)
2. **Task 2: Prove relay-disabled dual-state and status redaction** - `ba9f7fa1` (test)

**Plan metadata:** `docs(137-10)` commit for this SUMMARY only; STATE.md and ROADMAP.md were not updated.

_Note: TDD RED was already green because Plans 03/06/07 already implemented the contracts. Commits are regression locks, not new production fields._

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/dispatch/tests/transaction_methods.rs` - Exact key-set and struct-field freeze tests
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Import `SendRawTransactionResponse` / `Wtxid` and wire `mod dual_state`
- `packages/open-bitcoin-rpc/src/dispatch/tests/dual_state.rs` - Relay-disabled accept, status-echo, D-18, and served≠cleared proofs
- `packages/open-bitcoin-cli/src/operator/status/tests/mempool_policy.rs` - Named last-package JSON lock
- `docs/parity/source-breadcrumbs.json` - Register `dual_state.rs`

## Decisions Made

- Production types did not change. `SendRawTransactionResponse` already had the required three fields.
- D-17 is proven at the typed projector: `EligibleServe`/`served` facts yield `PackageRelayState::Served` while `FinallyPresent` stays `accepted`.
- D-18 is enforced on the typed package report and `mempool.admission` / `mempool.retry`. Phase 105 `relay.public_relay` remains the existing intentionally-different capability slot.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Scoped D-18 away from Phase 105 `relay.public_relay`**
- **Found during:** Task 2 (dual-state JSON)
- **Issue:** A recursive key scan of `openbitcoinnetworkstatus` failed because Phase 105 already serializes `relay.public_relay` as a non-claim capability field. Deleting that field would be an architectural change outside this test-only plan.
- **Fix:** Assert D-18 on the typed `openbitcoinpackage` report and on mempool admission/retry aggregates, matching T-137-10-04 claim-field intent.
- **Files modified:** `packages/open-bitcoin-rpc/src/dispatch/tests/dual_state.rs`, `packages/open-bitcoin-cli/src/operator/status/tests/mempool_policy.rs`
- **Verification:** `dual_state::` and `json_status_has_no_last_package` pass
- **Committed in:** `ba9f7fa1`

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Required to lock D-18 without ripping out Phase 105 relay evidence. No production fields were added.

## Issues Encountered

- Task 1 named tests passed on first run because D-16 was already implemented. They remain the required freeze names.
- No dispatch fixture emits `EligibleServe` without `TransportWritten` / `LifecycleRemoval`. `served_is_not_cleared` uses the public projector ladder, as the plan allows.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 11 can finish remaining submit fanout receipts on the existing ladder.
- `sendrawtransaction` object-vs-hex-string remains Phase 138.
- No blockers for those plans.

## Self-Check: PASSED

---
*Phase: 137-rpc-and-sanitized-operator-evidence*
*Completed: 2026-08-19*
---
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-20T00:10:00Z
---
