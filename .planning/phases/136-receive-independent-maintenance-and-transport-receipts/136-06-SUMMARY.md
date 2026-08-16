---
phase: 136-receive-independent-maintenance-and-transport-receipts
plan: "06"
subsystem: network
tags: [maintenance-tick, first-hop-inv, retry-worker, unbroadcast, fake-clock]

# Dependency graph
requires:
  - phase: 136-receive-independent-maintenance-and-transport-receipts
    provides: Plan 01 leftover cursor, Plan 03 TX INV PeerEmission, Plan 04 TransportWritten receipts, Plan 05 retry enqueue
provides:
  - Handle-owned maintenance_tick walking only unbroadcast_members
  - First-hop TX INV drain without the receive loop
  - Shutdown-aware open-bitcoind retry worker with injected now and jitter
affects: [phase-137, IBR-01, IBR-02, IBR-03, IBR-04, D-05, D-06, D-07, D-16, D-17, D-18, D-19]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Shell-owned timer copies checkpoint wait/now injection without importing checkpoint types
    - Walk cursor is last prepared identity so leftovers are not starved when inspect covers the set
    - getrandom failure is JitterUnavailable, not a silent 0 or 300 jitter

key-files:
  created:
    - packages/open-bitcoin-node/src/network/runtime_authority/maintenance.rs
    - packages/open-bitcoin-node/src/network/tests/maintenance_tick_cases.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/retry.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/retry.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Store last prepared identity as the walk cursor so leftovers progress when N < inspect 256."
  - "Drain first-hop INV after accept; abort unused write capabilities until a later socket write path exists."
  - "Always start the retry worker from open-bitcoind main beside the checkpoint worker, never from DurableSyncRuntime."
  - "Keep IBR-01 through IBR-04 Pending until lifecycle-valid phase verification."

patterns-established:
  - "maintenance_tick is receive-independent and walks only unbroadcast_members with production 256/32 budgets."
  - "Clocks and CSPRNG stay in the open-bitcoind shell; node tests inject RetryDecisionContext."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 136-2026-08-15T21-24-17
generated_at: 2026-08-16T05:33:22Z

# Metrics
duration: 72min
completed: 2026-08-16
---

# Phase 136 Plan 06: Maintenance Tick and Transport Receipts Summary

**Receive-independent `maintenance_tick` walks only the local unbroadcast set, remints an injected 10-to-15-minute due time, and an open-bitcoind checkpoint-style worker wakes it without DurableSync or the receive loop.**

## Performance

- **Duration:** 72 min
- **Started:** 2026-08-16T04:20:51Z
- **Completed:** 2026-08-16T05:33:22Z
- **Tasks:** 2
- **Files modified:** 21

## Accomplishments

- `ManagedNetworkHandle::maintenance_tick` walks `unbroadcast_members` only via `select_maintenance_identities` with production inspect 256 / prepare 32, calls `enqueue_retry_admissions`, remints due time from injected `RetryDecisionContext`, and returns owned `PeerEmission` values.
- `drain_tx_fanout_emissions` prepares first-hop TX INV after local accept without advancing ten minutes and without `receive_message`.
- Recovery install and handle construction remint `maybe_retry_due_at_unix_seconds` and `maybe_unbroadcast_walk_cursor` to `None`. Those fields are not persisted.
- `network/tests.rs` now registers `getdata_tx_receipt_cases`, `package_fanout_cases`, and `maintenance_tick_cases`.
- `start_initial_broadcast_retry_worker` always starts beside the checkpoint worker. Fake-clock tests prove elapsed ticks, shutdown-before-tick, and no silent constant jitter.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add maintenance_tick and first-hop TX INV drain on the handle** - `779fd8aa` (feat)
2. **Task 2: Add the open-bitcoind shutdown-aware retry worker** - `ef6206ee` (feat)

**Plan metadata:** pending docs commit

_Note: TDD RED commits were not created because `.githooks/pre-commit` runs `bash scripts/verify.sh`, which requires a green tree. Tests were written first, then production code landed in the same feat commit._

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/runtime_authority/maintenance.rs` - Handle-owned tick and TX INV drain
- `packages/open-bitcoin-node/src/network/tests/maintenance_tick_cases.rs` - Seven named fake-clock tick tests
- `packages/open-bitcoin-node/src/network.rs` - Timer fields and `MaintenanceTick*` re-exports
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - `mod maintenance` and error mapping
- `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs` - Recovery remint of due time and cursor
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Construction remint
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` - txid/wtxid lookup for drain
- `packages/open-bitcoin-node/src/network/announcement_transport.rs` - `is_transaction_inventory`
- `packages/open-bitcoin-node/src/network/tests.rs` - Registers Plan 04/05/06 test modules
- `packages/open-bitcoin-rpc/src/context/network.rs` - Drain after successful local accept
- `packages/open-bitcoin-rpc/src/dispatch.rs` - `MaintenanceTick` RPC error mapping
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/retry.rs` - Shutdown-aware retry worker
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/retry.rs` - Four named worker tests
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Start/shutdown beside checkpoint
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs` - `mod retry`
- `docs/parity/source-breadcrumbs.json` - `node-initial-broadcast-retry` group including Plan 04/05/06 sources
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC report

## Decisions Made

- Store the last prepared identity as the walk cursor. Plan 01's `maybe_next_after` is last inspected; when N < 256 that wraps to the head and starves leftovers.
- `submit_local_transaction_outcome_at` stays enqueue-only. The RPC wrapper drains after accept and aborts unused write capabilities so first-hop prepare is proven without a socket write path.
- The worker samples now and jitter in the shell, never in `open-bitcoin-node`. `getrandom` failure is `RetryWait::JitterUnavailable` and repeats the last interval (600s only as the initial wait).
- Do not start the worker from `DurableSyncRuntime` or `start_daemon_sync_worker`.
- Keep IBR-01 through IBR-04 Pending until Phase 136 has lifecycle-valid VERIFICATION.md.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Last-prepared walk cursor**
- **Found during:** Task 1 leftover-cursor test
- **Issue:** Storing Plan 01 `maybe_next_after` (last inspected) when N=40 < inspect 256 wraps to the head and never prepares leftovers.
- **Fix:** Store `selection.prepare.last()` as the cursor, falling back to `maybe_next_after` when prepare is empty.
- **Files modified:** `packages/open-bitcoin-node/src/network/runtime_authority/maintenance.rs`, `packages/open-bitcoin-node/src/network/tests/maintenance_tick_cases.rs`
- **Verification:** `maintenance_tick_respects_inspect_256_and_prepare_32_with_leftover_cursor` passes
- **Committed in:** `779fd8aa`

**2. [Rule 2 - Missing Critical] Abort unused peer-effect capabilities**
- **Found during:** Task 1 drain wiring
- **Issue:** Drain produces owned `PeerEmission` values. Dropping them leaks pending write reservations. The planned worker and RPC wrapper do not write sockets.
- **Fix:** Abort unused capabilities after drain in the RPC wrapper and after each worker tick.
- **Files modified:** `packages/open-bitcoin-rpc/src/context/network.rs`, `packages/open-bitcoin-rpc/src/bin/open_bitcoind/retry.rs`
- **Verification:** First-hop test still sees TX INV emissions; worker tests pass
- **Committed in:** `779fd8aa` (RPC), `ef6206ee` (worker)

**3. [Rule 3 - Blocking] Clippy collapsible_if / single_match / unnecessary_min**
- **Found during:** Task 1 commit (`verify.sh`)
- **Issue:** Nested due-time `if`, `match` on `Option`, and `40.min(256)` failed `-D warnings`.
- **Fix:** Collapse the due-time guard, use `if let Some`, assert `inspected_count == 40`.
- **Files modified:** `packages/open-bitcoin-node/src/network/runtime_authority/maintenance.rs`, `packages/open-bitcoin-node/src/network/tests/maintenance_tick_cases.rs`
- **Verification:** Subsequent `verify.sh` compiled the node lib
- **Committed in:** `779fd8aa`

### Other Deviations

**4. TDD RED commits omitted**
- Pre-commit runs full `bash scripts/verify.sh`. Failing tests cannot be committed. Tests were written first, then production code landed in the same feat commit.

**5. [Rule 3 - Blocking] Left IBR-01 through IBR-04 Pending**
- Marking them Complete fails `check-active-milestone-verification-traceability` because Phase 136 has no lifecycle-valid VERIFICATION.md yet. Same pattern as Plans 01–05.

***

**Total deviations:** 3 auto-fixed (cursor, capability abort, clippy) plus 2 process notes
**Impact on plan:** Wakeup, remint, first-hop drain, and shell ownership match D-05 through D-08 and D-16 through D-19. No public/default relay or guaranteed-propagation claims.

## Issues Encountered

- Task 2 sources in the working tree are compiled by Task 1 `verify.sh`. `RetryWait::JitterUnavailable` had to be constructed on the production jitter-failure path before Task 1 could commit.
- `submit_local_transaction_with_relay_evidence_at` aborts drained emissions rather than writing them. Socket write remains a later path; first-hop prepare is still proven on the handle.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- IBR-02 has a receive-independent timer. IBR-01 walk set is `unbroadcast_members`. IBR-03 drain reuses TX INV `PeerEmission`. IBR-04 rustdoc forbids public/default relay and guaranteed propagation.
- Phase 137 can add RPC/dashboard retry counters. Phase 138 can close claim-guardrail wording.
- Keep IBR requirements Pending until phase verification.

***
*Phase: 136-receive-independent-maintenance-and-transport-receipts*
*Completed: 2026-08-16*

## Self-Check: PASSED
