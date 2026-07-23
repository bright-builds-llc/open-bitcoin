---
phase: 130-resource-time-and-fee-primitives
plan: "10"
subsystem: network
tags: [rust, transaction-relay, retry, explicit-inputs, parity]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Canonical retry eligibility metadata and typed lifecycle clearing facts from Plans 130-03 and 130-04
provides:
  - Validated 0-to-300-second retry jitter value with a fixed typed error
  - Immutable retry decision context carrying exact injected Unix time and jitter
  - Crate-root retry input exports for later shell and policy consumers
affects: [phase-136, transaction-relay, initial-broadcast-retry]
tech-stack:
  added: []
  patterns:
    - Shell-sampled time and randomness represented as immutable pure-core inputs
    - Fallible bounded newtype construction with fixed low-cardinality errors
key-files:
  created:
    - packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs
  modified:
    - packages/open-bitcoin-network/src/peer/transaction_relay.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/lib.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Represent only the variable 0-to-300-second portion of the pinned retry cycle; Phase 136 owns base cadence, scheduling, fanout, receipts, and clearing."
  - "Keep retry observation time as the exact injected i64 Unix-seconds value and expose jitter only after fallible bounded construction."
patterns-established:
  - "Pure retry contracts retain caller-supplied facts without reading clocks, randomness, timers, transport, or scheduler state."
requirements-completed: []
requirements-addressed: [FEEP-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-23T23:37:11Z
duration: 21 min
completed: 2026-07-23
---

# Phase 130 Plan 10: Injected Network Retry Inputs Summary

**Bounded retry jitter and exact observation time now cross the network pure-core boundary as validated immutable inputs without hidden clock, randomness, timer, scheduler, or transport effects**

## Performance

- **Duration:** 21 min
- **Started:** 2026-07-23T23:16:36Z
- **Completed:** 2026-07-23T23:37:11Z
- **Tasks:** 1
- **Files modified:** 6

## Accomplishments

- Added `RetryJitterSeconds` with an inclusive `0..=300` contract and typed fixed-label range failure.
- Added `RetryDecisionContext` that retains exact injected Unix seconds and validated jitter.
- Re-exported the retry vocabulary through transaction relay, peer, and crate-root boundaries for later Phase 136 consumers.
- Registered a focused `net_processing.cpp` parity breadcrumb and proved three deterministic retry tests are discoverable.
- Audited workspace retry/jitter/context contracts and public exports across libraries, binaries, benchmarks, integration tests, and doctests; the exact timed all-target workspace gate passed.

## Task Commits

1. **Task 1: Define injected retry time and jitter inputs** - `091b5041` (feat)

The TDD RED run failed on the intentionally absent retry types. The completed GREEN contract and inline tests were committed atomically after focused tests, parity validation, the timed all-target compile gate, and the full repository hook passed.

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs` - Validated jitter, typed error, immutable decision context, and three inline tests.
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` - Retry module registration and transaction-relay re-exports.
- `packages/open-bitcoin-network/src/peer.rs` - Required public peer-layer re-export bridge.
- `packages/open-bitcoin-network/src/lib.rs` - Crate-root retry contract exports.
- `docs/parity/source-breadcrumbs.json` - Dedicated initial-broadcast retry input group anchored to pinned `net_processing.cpp`.
- `docs/metrics/lines-of-code.md` - Hook-managed LOC freshness.

## Decisions Made

- The new contract models only injected inputs. It does not decide a fixed schedule, scan candidates, emit fanout, mutate unbroadcast state, or interpret transport receipts.
- Jitter uses a `u64` newtype with a fallible constructor; invalid values cannot enter a `RetryDecisionContext`.
- The range error is a typed unit value with the fixed `retry_jitter_out_of_range` label, avoiding dynamic or sensitive shared evidence.

## ASVS Mitigations

- **ASVS-130-V1/V13:** Audited same-named workspace contracts and public callers, added the required peer re-export bridge, and passed `cargo check --workspace --all-targets`.
- **ASVS-130-V5/V7/V11:** Enforced the inclusive jitter bound through a fallible newtype and covered boundary acceptance, rejection, fixed error formatting, and exact context retention.
- **ASVS-130-V9/V10 boundaries:** Added no scheduler, transport behavior, timer, randomness source, dependency, or dynamic execution path.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added the peer-layer public re-export bridge**
- **Found during:** Task 1 verification
- **Issue:** The plan named transaction-relay and crate-root wiring, but the private `peer` module requires an intermediate re-export before crate-root exports are accessible.
- **Fix:** Re-exported all three retry contract types from `peer.rs`.
- **Files modified:** `packages/open-bitcoin-network/src/peer.rs`
- **Verification:** Focused tests and the exact timed workspace all-target compile gate passed.
- **Committed in:** `091b5041`

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The required intermediary preserves the planned public API and introduces no scheduling or transport behavior.

## Issues Encountered

- The first commit attempt exposed a parity-group mismatch because the focused retry file had been placed in the broader transaction-relay breadcrumb group. A dedicated retry-input group now matches the file's exact `net_processing.cpp` breadcrumb.
- The next full-hook run identified uncovered `Display` lines for the typed range error. The rejection test now verifies the fixed display label, and the final hook passed with zero uncovered lines.
- The metadata hook correctly rejected early FEEP-04 completion because Phase 130 has no lifecycle-valid `VERIFICATION.md` yet. FEEP-04 remains addressed and pending phase verification.

## Authentication Gates

None.

## Known Stubs

None.

## Threat Flags

None - the change adds pure immutable values and no network endpoint, authentication path, file-access pattern, persistence schema, scheduler, or transport surface.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 136 can sample time and fresh jitter in its shell and pass them into this contract without changing the pure-core boundary.
- Scheduling, candidate scans, bounded emissions, transport receipts, and retry clearing remain explicitly deferred to Phase 136.
- No blockers remain.

## Self-Check: PASSED

- Summary and retry source files exist.
- Task commit `091b5041` exists.
- Three named retry tests, focused parity registration, the exact timed workspace all-target compile gate, and the full repository verifier pass.
- Lifecycle ID, yolo mode, FEEP-04 ownership, and no-effects boundary match the committed implementation.

---
*Phase: 130-resource-time-and-fee-primitives*
*Completed: 2026-07-23*
