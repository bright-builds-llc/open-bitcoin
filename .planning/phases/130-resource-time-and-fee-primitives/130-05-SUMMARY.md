---
phase: 130-resource-time-and-fee-primitives
plan: "05"
subsystem: node-admission
tags: [rust, mempool, metadata, lifecycle-deltas, runtime-authority]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Canonical metadata and deterministic committed lifecycle deltas from Plans 130-03 and 130-04
provides:
  - Exact peer receive/reconsideration metadata through managed admission
  - Explicit-time local admission with resolved relay intent through ManagedNetworkHandle
  - Bridge-owned cache consequences driven by semantic lifecycle deltas and final membership
  - Deprecated fail-closed no-time compatibility adapters with named migration/removal owners
affects: [phase-130-plan-06, phase-130-plan-11, phase-134, node-admission, rpc-admission]
tech-stack:
  added: []
  patterns:
    - Shell-supplied typed admission contexts through the sole runtime authority
    - Attempt outcomes retained for direct callers while cache effects consume committed deltas
key-files:
  created:
    - packages/open-bitcoin-node/src/network/runtime_authority/tests.rs
  modified:
    - packages/open-bitcoin-mempool/src/context.rs
    - packages/open-bitcoin-node/src/mempool.rs
    - packages/open-bitcoin-node/src/network/admission_bridge.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs
    - packages/open-bitcoin-node/src/network/relay_fanout.rs
key-decisions:
  - "Peer admission always constructs known-time Peer/NotRequested metadata; peer IDs never become canonical origin metadata."
  - "Explicit local admission accepts shell-sampled time and resolved relay intent, while no-time adapters remain LegacyUnknown/RecoveryUnknown/NotRequested."
  - "Bridge-owned removal, compact-extra, peer cleanup, and admitted-body storage consume lifecycle cause, identity, and final-membership facts without outcome-vector reclassification."
patterns-established:
  - "ManagedMempool exposes context-aware transition admission while ManagedNetworkHandle preserves the single mutation authority."
  - "RelayIntent::NotRequested suppresses local fanout even when relay activation and an eligible peer are present."
requirements-completed: []
requirements-addressed: [FEEP-03, FEEP-04, FEEP-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-23T23:04:33Z
duration: 54 min
completed: 2026-07-23
---

# Phase 130 Plan 05: Managed Admission Authority Integration Summary

**Managed peer and local admission now preserves canonical metadata and projects bridge-owned cache effects exclusively from committed lifecycle deltas**

## Performance

- **Duration:** 54 min
- **Started:** 2026-07-23T22:10:21Z
- **Completed:** 2026-07-23T23:04:33Z
- **Tasks:** 1
- **Files modified:** 18

## Accomplishments

- Added exact known receive/reconsideration time, peer origin, and non-requested relay metadata for live peer admission.
- Added explicit-time local admission through `ManagedNetworkHandle`, including requested/not-requested relay intent and duplicate metadata immutability.
- Replaced admission-side outcome-vector cache cleanup with lifecycle delta cause, role, identity, and final-membership consumption.
- Kept detailed outcomes available to direct authenticated callers while shared evidence remains fixed-label and identity-free.
- Retained only deprecated fail-closed no-time adapters, with Plan 130-06 owning node-caller migration and Plan 130-11 owning RPC migration and final removal.
- Expanded the historical Phase 102 guard to accept and mutation-test transition-based admission without weakening its ordering or peer-boundary checks.

## Task Commits

1. **Task 1: Migrate managed admission to typed contexts and deltas** - `10c5b493` (feat)

The TDD RED runs failed on the missing relay-intent API and on unintended fanout for `NotRequested`; the completed GREEN implementation passed the focused suites, exact timed all-target workspace gate, and full normal-hook verifier.

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/context.rs` - Trusted peer/local admission constructors and explicit Unix-time conversion.
- `packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs` - Direct constructor mapping and coverage regressions.
- `packages/open-bitcoin-node/src/mempool.rs` - Context-aware result and transition submission methods plus documented fail-closed adapters.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` - Canonical peer/local contexts, explicit-time local API, and delta-driven cache projection.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - Explicit-time local submission through the sole managed authority and deprecated compatibility methods.
- `packages/open-bitcoin-node/src/network/runtime_authority/tests.rs` - Extracted authority tests including the explicit local admission path.
- `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs` - Exact peer/local/reconsideration metadata, duplicate, fail-closed, and lifecycle-delta regressions.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` - Relay-intent-aware local fanout suppression.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Narrow Plan 130-11 compatibility allowances.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Fail-closed intermediate RPC relay-evidence assertion.
- `scripts/check-phase102-orphan-admission-bridge.ts` - Transition-aware historical admission ordering guard.
- `scripts/check-phase102-orphan-admission-bridge.test.ts` - Mutation coverage for the transition admission command.
- `docs/parity/source-breadcrumbs.json` - Registered the extracted runtime-authority test file.
- `docs/metrics/lines-of-code.md` - Hook-managed LOC freshness.

## Decisions Made

- Canonical peer metadata is always `Known(receive_or_reconsideration_time)`, `Peer`, and `NotRequested`.
- Canonical local metadata is constructed only by the explicit-time API and uses the caller's resolved relay intent.
- No-time compatibility calls cannot queue initial relay or deferred rebroadcast; they fail closed until Plan 130-11 migrates the RPC shell.
- Current bridge-owned serving, compact-extra, and peer-removal effects use delta facts; complete Phase 134 cross-cache projection remains deferred.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Suppressed fanout for explicit NotRequested relay intent**
- **Found during:** Task 1 TDD integration
- **Issue:** The existing local fanout helper still queued to eligible peers when canonical relay intent was `NotRequested`.
- **Fix:** Passed typed relay intent into fanout projection and supplied no eligible peer inputs for `NotRequested`.
- **Files modified:** `packages/open-bitcoin-node/src/network/relay_fanout.rs`, `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs`
- **Verification:** The regression first failed with queued count 1, then passed with zero queued transactions.
- **Committed in:** `10c5b493`

**2. [Rule 3 - Blocking] Migrated the historical Phase 102 admission guard**
- **Found during:** Task 1 normal-hook verification
- **Issue:** The guard required the removed `submit_transaction_outcome` command in the managed bridge and rejected the planned transition API.
- **Fix:** Allowed either the historical outcome command or the canonical transition command at the same ordered bridge position, prohibited both from peer/socket code, and added a transition-removal mutation.
- **Files modified:** `scripts/check-phase102-orphan-admission-bridge.ts`, `scripts/check-phase102-orphan-admission-bridge.test.ts`
- **Verification:** The focused checker suite passed 10/10 tests and the live checker passed.
- **Committed in:** `10c5b493`

**3. [Rule 3 - Blocking] Extracted runtime-authority tests below the production file limit**
- **Found during:** Task 1 normal-hook verification
- **Issue:** Adding the explicit authority API and inline test pushed `runtime_authority.rs` above the enforced 628-line production limit.
- **Fix:** Moved all inline tests into the required `runtime_authority/tests.rs` child module and registered its parity breadcrumb.
- **Files modified:** `packages/open-bitcoin-node/src/network/runtime_authority.rs`, `packages/open-bitcoin-node/src/network/runtime_authority/tests.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** The production file-length guard and parity breadcrumb checker passed in the full verifier.
- **Committed in:** `10c5b493`

---

**Total deviations:** 3 auto-fixed (1 bug, 2 blocking)
**Impact on plan:** All fixes were required for privacy correctness and repository compatibility; no Phase 131 pressure policy or Phase 134 complete cross-cache projection was added.

## Issues Encountered

- The normal hook surfaced the historical guard, production file-length, RPC compatibility expectation, formatting, and direct constructor coverage gaps in sequence. Each was resolved without bypassing hooks.
- The metadata hook continues to reject premature FEEP completion before Phase 130 has lifecycle-valid verification. FEEP-03, FEEP-04, and FEEP-05 remain addressed and pending phase verification.

## Authentication Gates

None.

## Known Stubs

None.

## Threat Flags

None - the change preserves the existing authority and authenticated RPC boundary; it adds no network endpoint, authentication path, file-access pattern, durable schema, or dependency.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 130-06 can migrate remaining node-owned compatibility callers to explicit time and relay intent.
- Plan 130-11 retains the named RPC shell sampling and compatibility-removal seam.
- Phase 134 retains complete cross-cache lifecycle projection ownership.
- No blockers remain.

## Self-Check: PASSED

- Summary and extracted runtime-authority test file exist.
- Task commit `10c5b493` exists.
- Lifecycle mode, lifecycle ID, addressed requirements, verification claims, and changed-file metrics match the committed work.

---
*Phase: 130-resource-time-and-fee-primitives*
*Completed: 2026-07-23*
