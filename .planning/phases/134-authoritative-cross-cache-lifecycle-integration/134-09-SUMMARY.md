---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "09"
subsystem: network-runtime
tags: [rust, peer-effects, successful-prefix, rpc, sync]

requires:
  - phase: 134-08
    provides: typed lifecycle effect capabilities, receipts, and shared completion dispatch
provides:
  - affine peer-emission write capabilities that produce receipts only after achieved writes
  - node sync and RPC inbound executors that preserve exactly the successful command prefix
  - independent RPC failure injection for encode, rejection, disconnect, and write boundaries
affects: [134-10, peer-lifecycle, compact-relay, inbound-listener]

tech-stack:
  added: []
  patterns:
    - owned peer effect leaves authority before external I/O
    - write success consumes capability into one completion receipt
    - ordered executor stops at the first failed command without crediting its suffix

key-files:
  created:
    - packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs
  modified:
    - packages/open-bitcoin-node/src/network/announcement_transport.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
    - packages/open-bitcoin-node/src/sync/session.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
    - scripts/check-phase128-production-compact-announcement-transport.ts

key-decisions:
  - "PeerEmission owns an affine write capability; only acknowledge_write can create its receipt after external success."
  - "Both production shells complete each written command immediately before advancing, preserving successful-prefix truth."
  - "RPC failure coverage uses a private injected executor contract while production completion remains routed through ManagedNetworkHandle."

patterns-established:
  - "Write-acknowledge-complete: encode and write outside authority, consume the capability only on Written, then dispatch shared completion."
  - "Successful prefix: a failure at command N retains completions before N and gives no achieved credit to N or its unsent suffix."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T16:07:31Z

duration: 1h 17m
completed: 2026-07-28
---

# Phase 134 Plan 09: Peer Write Successful Prefix Summary

**Affine peer-emission capabilities now credit only achieved writes, with node sync and RPC inbound executors preserving the exact successful command prefix.**

## Performance

- **Duration:** 1h 17m
- **Started:** 2026-07-28T14:50:50Z
- **Completed:** 2026-07-28T16:07:31Z
- **Tasks:** 3
- **Files modified:** 31

## Accomplishments

- Replaced preparation-time peer receipts with affine write capabilities bound to authority epoch, lifecycle generation, effect ID, peer ID, and peer-session generation.
- Routed node sync announcements through per-command send, acknowledge, and shared completion so later encode or socket failure cannot erase earlier achieved truth.
- Added a production-used RPC executor seam with independent encode, resource rejection, disconnect, and write-failure proofs; command 1 completes once while commands 2 and 3 receive no achieved credit.
- Advanced peer-session generation on production connect, inbound admission, disconnect, and reconnect boundaries so older capabilities classify as achieved-but-stale.

## Task Commits

Each task was committed atomically:

1. **Task 1: Introduce the peer-emission write capability boundary** - `02389171` (feat)
2. **Task 2: Preserve successful prefixes in the node sync shell** - `5bde2b4b` (feat)
3. **Task 3: Update and independently verify the RPC inbound executor** - `66beafa6` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/announcement_transport.rs` - Owns peer emission capabilities and creates receipts only through post-write acknowledgement.
- `packages/open-bitcoin-node/src/network/runtime_authority/effects.rs` - Completes peer effects through the shared dispatcher and records relay evidence only for applied completions.
- `packages/open-bitcoin-node/src/sync/session.rs` - Sends, acknowledges, and completes each queued peer emission before advancing.
- `packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs` - Proves exact three-command successful-prefix behavior for encode and socket failures.
- `packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs` - Runs the RPC write-acknowledge-complete executor outside authority.
- `packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs` - Injects encode, rejection, disconnect, and write failures directly in the RPC executor.
- `docs/parity/source-breadcrumbs.json` - Registers the new RPC inbound test module against Knots network-processing anchors.
- `scripts/check-phase128-production-compact-announcement-transport.ts` - Guards the stronger capability-based inbound write boundary.

## Decisions Made

- Kept capabilities affine and receipts non-cloneable in production so a caller cannot pre-create or replay achieved-write evidence.
- Used the existing shared lifecycle dispatcher for all peer-emission completion; neither shell gained an alternate authority mutation path.
- Classified successful writes completed after peer-session rotation as achieved-but-stale without adding relay evidence.
- Kept `MPLIFE-04` pending for later phase-level reconciliation as explicitly requested.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Advanced peer-session generations on real connection churn**

- **Found during:** Task 1 (peer-emission capability boundary)
- **Issue:** Stale reconnect receipts could not be rejected if production connect, inbound admission, and disconnect paths did not rotate the peer-session generation.
- **Fix:** Added checked generation advancement to all connection-churn paths with typed exhaustion errors.
- **Files modified:** `packages/open-bitcoin-node/src/network/inbound.rs`, `packages/open-bitcoin-node/src/network/runtime_authority.rs`, `packages/open-bitcoin-node/src/network/types.rs`
- **Verification:** Current, stale reconnect, and duplicate completion tests pass.
- **Committed in:** `02389171`

**2. [Rule 3 - Blocking] Boxed ready emissions to satisfy the warnings-denied Clippy gate**

- **Found during:** Task 1 mandatory Clippy verification
- **Issue:** The capability-bearing `Ready` variant made `AnnouncementPreparationOutcome` exceed the large-enum threshold.
- **Fix:** Boxed only the ready emission while preserving affine ownership and queue semantics.
- **Files modified:** `packages/open-bitcoin-node/src/network/announcement_transport.rs`, `packages/open-bitcoin-node/src/sync/session.rs`
- **Verification:** Clippy with `-D warnings`, build, and full tests pass.
- **Committed in:** `02389171`

**3. [Rule 3 - Blocking] Updated legacy structural guards for the capability boundary**

- **Found during:** Task 1 and Task 3 normal commit hooks
- **Issue:** Phase 122, 123, 126, and 128 guards required the superseded preparation-time receipt syntax.
- **Fix:** Changed the guards and mutation oracles to require capability ownership, post-write `acknowledge_write`, and shared completion dispatch.
- **Files modified:** Phase 122, 123, 126, and 128 checker and mutation-test files under `scripts/`
- **Verification:** All affected checker suites and their mutation cases pass; the final normal hook passes.
- **Committed in:** `02389171`, `66beafa6`

**Total deviations:** 3 auto-fixed (1 missing critical functionality, 2 blocking issues)
**Impact on plan:** All fixes were required to enforce or verify the planned write-capability boundary; no feature scope was added.

## Issues Encountered

- The initial RPC test fixture prepared announcements before completing the peer handshake and correctly received `Ineligible`; completing the fixture through the public RPC network handshake made the production preparation path eligible.
- The Task 3 normal hook initially stopped at the legacy Phase 128 syntax guard; the mutation-tested guard was updated and the full hook then passed.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 10 can build on a proven write-capability boundary and independently verified node/RPC successful-prefix executors.
- `MPLIFE-04` intentionally remains pending for later phase-level reconciliation.

## Self-Check: PASSED

All key files and task commits were found, and the summary passes `git diff --check`.

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-28*
