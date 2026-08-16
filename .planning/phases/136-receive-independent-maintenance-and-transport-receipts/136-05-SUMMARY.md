---
phase: 136-receive-independent-maintenance-and-transport-receipts
plan: "05"
subsystem: network
tags: [package-fanout, parent-before-child, retry-enqueue, phase104-queue]

# Dependency graph
requires:
  - phase: 136-receive-independent-maintenance-and-transport-receipts
    provides: Plan 01 leftover cursor and Plan 04 TransportWritten receipt path
provides:
  - Parent-before-child package FIFO on the existing TxFanoutQueue
  - enqueue_retry_admissions through record_prepared_admission
affects: [phase-136-06, PPKG-04, IBR-03, D-13, D-14, D-15, D-19]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Co-admitted members enqueue from facts.final_present() in admitted topological order
    - AlreadyPresent and independently admitted parents are not re-enqueued
    - Retry identities enter TxFanoutQueue::enqueue_admission in caller-supplied order only

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/package_fanout_cases.rs
  modified:
    - packages/open-bitcoin-node/src/network/relay_fanout.rs
    - packages/open-bitcoin-node/src/network/relay_fanout/lifecycle.rs
    - packages/open-bitcoin-node/src/network/relay_fanout/action_info.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs
    - packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs
    - packages/open-bitcoin-node/src/network/tests/relay_local_submission_cases.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Keep the existing final_present() fanout loop; AlreadyPresent parents are not in that list."
  - "enqueue_retry_admissions iterates the caller slice only and reuses record_prepared_admission."
  - "rebroadcast_deferred means the first hop is recorded and the retry cycle has not yet run."
  - "Keep PPKG-04 and IBR-03 Pending until lifecycle-valid phase verification."

patterns-established:
  - "Package fanout is ordinary INV/TX on the Phase 104 FIFO; no second announcer or package wire."
  - "Retry prepare is enqueue_retry_admissions, not a parallel queue or rebroadcast.rs."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 136-2026-08-15T21-24-17
generated_at: 2026-08-16T04:10:20Z

# Metrics
duration: 74min
completed: 2026-08-16
---

# Phase 136 Plan 05: Package FIFO and Retry Enqueue Summary

**Co-admitted package members enqueue parent-before-child on the existing Phase 104 FIFO, and retry identities enter that same queue through `enqueue_retry_admissions`.**

## Performance

- **Duration:** 74 min
- **Started:** 2026-08-16T02:56:37Z
- **Completed:** 2026-08-16T04:10:20Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments

- `prepare_fanout_projection` keeps admitted `final_present()` order. AlreadyPresent parents are not in that list and are not re-enqueued.
- Independently admitted children use ordinary single-tx enqueue and do not re-announce a present parent.
- `package_fanout_cases.rs` proves parent-before-child FIFO, no parent re-enqueue, and existing txid/wtxid peer mode. The module stays unregistered until Plan 06.
- `enqueue_retry_admissions` builds `TxFanoutAdmission { Accepted }` per caller identity and calls `record_prepared_admission` / `TxFanoutQueue::enqueue_admission`. Unpassed identities stay unattempted.
- Relay-disabled or ineligible peers produce the existing suppress actions and no new queue.
- `rebroadcast_deferred` comments now say the first hop is recorded and the receive-independent retry cycle has not yet run.

## Task Commits

Each task was committed atomically:

1. **Task 1: Prove co-admitted parent-before-child FIFO and no parent re-announce** - `10942dd3` (feat)
2. **Task 2: Add enqueue_retry_admissions on the existing Phase 104 path** - `cd4ac096` (feat)

**Plan metadata:** pending docs commit

_Note: TDD RED commits were not created because `.githooks/pre-commit` runs `bash scripts/verify.sh`, which requires a green tree. Tests were written first, then production code landed in the same feat commit._

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/tests/package_fanout_cases.rs` - Four named FIFO and peer-mode tests (module unregistered until Plan 06)
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` - `enqueue_retry_admissions` plus test queue inspector
- `packages/open-bitcoin-node/src/network/relay_fanout/lifecycle.rs` - D-13/D-14 comment; `record_prepared_admission` is `pub(in crate::network)`
- `packages/open-bitcoin-node/src/network/relay_fanout/action_info.rs` - Extracted `implemented_capability` and `translate_fanout_action`
- `packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs` - `queued_relay_ids` FIFO inspector
- `packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs` - Three named retry-enqueue tests
- `packages/open-bitcoin-node/src/network/tests/relay_local_submission_cases.rs` - Truthful `rebroadcast_deferred` rustdoc
- `docs/parity/source-breadcrumbs.json` - `node-initial-broadcast-retry` group for `package_fanout_cases.rs`
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC report

## Decisions Made

- Keep iterating `facts.final_present()`; it is already admitted topological order and excludes AlreadyPresent parents (D-13, D-14).
- `enqueue_retry_admissions` walks only the supplied slice. It does not call `mempool.entries()` or create a second queue (D-12, D-19, T-136-05-01).
- `rebroadcast_deferred` remains a truthful first-hop label: retry is not yet due, not that Phase 104 shipped a timer and not that retry was cancelled.
- Keep PPKG-04 and IBR-03 Pending until Phase 136 has lifecycle-valid VERIFICATION.md.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Breadcrumb mapping for package_fanout_cases.rs**
- **Found during:** Task 1 commit (`verify.sh`)
- **Issue:** The checker scans every in-scope Rust file on disk. The new test file cannot exist without a mapping, even though Plan 06 owns shared registration.
- **Fix:** Add `node-initial-broadcast-retry` with `net_processing.cpp` and `mempool_unbroadcast.py`. Leave `mod package_fanout_cases` out of `tests.rs`.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check`
- **Committed in:** `10942dd3`

**2. [Rule 3 - Blocking] Cover queued_relay_ids in the registered fanout suite**
- **Found during:** Task 1 commit (coverage)
- **Issue:** `package_fanout_cases.rs` is unregistered, so the new `TxFanoutQueue::queued_relay_ids` inspector had no live coverage.
- **Fix:** Assert FIFO identities in the existing network `fanout_cases` enqueue test.
- **Files modified:** `packages/open-bitcoin-network/src/peer/transaction_relay/tests/fanout_cases.rs`
- **Verification:** Coverage passed on the Task 1 commit
- **Committed in:** `10942dd3`

**3. [Rule 3 - Blocking] allow(dead_code) on enqueue_retry_admissions**
- **Found during:** Task 2 commit (`verify.sh` production lib)
- **Issue:** Plan 06 owns the shell timer that will call the helper. Production has no caller yet.
- **Fix:** `#[allow(dead_code)]` with a Plan 06 comment.
- **Files modified:** `packages/open-bitcoin-node/src/network/relay_fanout.rs`
- **Verification:** Production lib compiles under `-D warnings`
- **Committed in:** `cd4ac096`

**4. [Rule 3 - Blocking] Extract helpers to stay under the 628-line production gate**
- **Found during:** Task 2
- **Issue:** Adding `enqueue_retry_admissions` pushed `relay_fanout.rs` to the file-length limit.
- **Fix:** Move `implemented_capability` and `translate_fanout_action` into `action_info.rs`.
- **Files modified:** `packages/open-bitcoin-node/src/network/relay_fanout.rs`, `packages/open-bitcoin-node/src/network/relay_fanout/action_info.rs`
- **Verification:** Production file-length check passed (618 lines)
- **Committed in:** `cd4ac096`

### Other Deviations

**5. TDD RED commits omitted**
- Pre-commit runs full `bash scripts/verify.sh`. Failing tests cannot be committed. Tests were written first, then production code landed in the same feat commit.

**6. [Rule 3 - Blocking] Left PPKG-04 and IBR-03 Pending**
- Marking them Complete fails `check-active-milestone-verification-traceability` because Phase 136 has no lifecycle-valid VERIFICATION.md yet. Same pattern as Plans 01–04.

***

**Total deviations:** 4 auto-fixed (breadcrumb, coverage, dead_code, file length) plus 2 process notes
**Impact on plan:** Package FIFO and retry enqueue match D-13, D-14, D-15, and D-19. No second announcer or package wire.

## Issues Encountered

- Task 1 tests had to be temporarily registered to run, then unregistered before commit so Plan 06 still owns `tests.rs`.
- Task 2 needed a file-length extract and a dead_code allow because the production caller is Plan 06.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 06 can register `mod package_fanout_cases` and re-run the `package_fanout` filter; the four named tests already exist.
- Plan 06 can start the shell timer and call `enqueue_retry_admissions` with unbroadcast members only.
- PPKG-04 and IBR-03 remain Pending until phase verification.
- No blockers.

***
*Phase: 136-receive-independent-maintenance-and-transport-receipts*
*Completed: 2026-08-16*

## Self-Check: PASSED
