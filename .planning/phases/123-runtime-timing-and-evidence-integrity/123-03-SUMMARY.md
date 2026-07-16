---
phase: 123-runtime-timing-and-evidence-integrity
plan: 03
subsystem: sync-runtime-evidence
tags: [rust, block-serving, achieved-effects, partial-batch]

requires:
  - phase: 123-runtime-timing-and-evidence-integrity
    plan: 01
    provides: Typed SyncPeerReceiveOutcome for scripted sync sessions
  - phase: 123-runtime-timing-and-evidence-integrity
    plan: 02
    provides: Private authoritative served-write counter and typed acknowledgement API
provides:
  - Per-message send-then-ack ordering at the durable sync transport boundary
  - Exact successful-prefix accounting when a later batch send fails
  - Focused success, failure, non-block, and partial-batch regression coverage
affects: [123-04, 123-05, HARD-03, block-relay-metrics]

tech-stack:
  added: []
  patterns:
    - Achieved-effect evidence advances immediately after each successful typed wire write
    - Batch evidence is committed per message so later failures cannot erase successful prefixes

key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs
  modified:
    - packages/open-bitcoin-node/src/sync/session.rs
    - packages/open-bitcoin-node/src/sync/tests.rs

key-decisions:
  - "Keep send_all as the only sync-runtime acknowledgement seam and call the existing managed-network API immediately after session.send succeeds."
  - "Use an exact 1-based scripted send failure so partial-batch tests distinguish successful Block prefixes from later Block or non-block failures."

patterns-established:
  - "Pattern: retain typed WireNetworkMessage identity through send completion, then acknowledge exactly once."
  - "Pattern: failed and non-block writes never mutate private served-block evidence."

requirements-completed:
  - HARD-03
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 123-2026-07-15T18-12-00
generated_at: 2026-07-16T02:19:11Z

duration: 9min
completed: 2026-07-16
---

# Phase 123 Plan 03: Sync Write Evidence Summary

**Durable sync writes now advance private served-block evidence only after each successful typed Block send, preserving every successful prefix across later batch failures.**

## Performance

- **Duration:** 9 min
- **Started:** 2026-07-16T02:10:00Z
- **Completed:** 2026-07-16T02:19:11Z
- **Tasks:** 1 TDD task
- **Files changed:** 3

## Accomplishments

- Made `DurableSyncRuntime::send_all` mutable and ordered the authoritative acknowledgement immediately after each successful `session.send`.
- Added five focused Arrange/Act/Assert tests for successful Block, failed Block, successful non-block, one-Block partial prefix, and two-Block partial prefix behavior.
- Kept `SyncPeerReceiveOutcome` in the scripted fixture and read the private runtime counter instead of public status, metrics, or logs.

## Task Commits

1. **RED: failing sync write evidence coverage** - `c79b8310` (test)
2. **GREEN: successful sync block-write acknowledgement** - `ecf6a848` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/session.rs` - Acknowledges each typed message immediately after transport success.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Registers the focused Phase 123 child module after runtime timing coverage.
- `packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs` - Proves exact successful-write and partial-batch counts.

## Verification Results

- RED timing-wrapped suite: 3 expected failures with observed `0` versus expected `1`, `1`, and `2`; failed-send and non-block controls passed.
- Exact timing-wrapped GREEN suite: 5 passed, 0 failed.
- Timing-wrapped `cargo clippy -p open-bitcoin-node --all-targets --all-features -- -D warnings`: passed.
- Timing-wrapped `cargo build -p open-bitcoin-node --all-targets --all-features`: passed.
- `cargo fmt --all`, all acceptance searches, source-order inspection, and `git diff --check`: passed.

## Decisions Made

- The acknowledgement remains inside the existing loop rather than using end-of-batch accounting, a receipt abstraction, retry state, or transport-local counter.
- The two partial-batch fixtures intentionally fail different third messages: a Block after one successful Block plus non-block, and a non-block after two successful Blocks.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- The GREEN test binary paused in macOS `_dyld_start` while another Cargo workload was active. Process sampling confirmed it had not entered test logic; it resumed without intervention and all five tests completed in 1.93 seconds.

## User Setup Required

None - no external service configuration required.

## Residual Risks

- The orchestrator owns post-wave breadcrumb manifest registration for the new Rust test file and the centralized repository hook.
- Plan 123-04 still owns the inbound listener half of HARD-03; Plan 123-05 will consume the private count in authoritative projections.

## Next Phase Readiness

- The sync-session half of successful block-emission evidence is complete and ready for inbound write acknowledgement and authoritative metric/log projection.
- No blockers remain.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-node/src/sync/session.rs` with send-before-ack ordering and `&mut self`.
- FOUND: `packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs` with both Knots breadcrumbs and all five exact tests.
- FOUND: `packages/open-bitcoin-node/src/sync/tests.rs` with the child module registered after Plan 01 timing coverage.
- FOUND COMMITS: `c79b8310`, `ecf6a848`.

***

*Phase: 123-runtime-timing-and-evidence-integrity*
*Completed: 2026-07-16*
