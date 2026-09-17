---
phase: 144-operator-flush-and-availability-evidence
plan: 01
subsystem: observability
tags: [chainstate-durability, flush-lifecycle, have-bytes, csobs, status-snapshot, rust]

requires:
  - phase: 140-coins-cache-flush-policy
    provides: decide_flush, FlushMode::None occupancy classification, LastFlushReason
  - phase: 142-manager-flush-lifecycle-and-restart
    provides: FlushLifecycle initialize/execute_flush and estimated_cache_bytes occupancy
  - phase: 143-honest-stored-block-availability
    provides: BlockServingPresenceFacts payload_present / index_known / validated_on_active_chain
provides:
  - "Dedicated chainstate_durability FieldAvailability field on OpenBitcoinStatusSnapshot"
  - "Retained last real FlushDecision and coins recovery outcome on FlushLifecycle"
  - "Sibling HaveBytesAccumulator last labels plus three D-12 counters"
  - "ManagedNetworkOperatorSnapshot copies the shared field without extending block_relay"
affects:
  - 144-02
  - 144-03
  - cli-dashboard-render
  - metrics-logs-support

tech-stack:
  added: []
  patterns:
    - "Current cache_size from decide_flush(FlushMode::None); last_flush_reason/write_kind from last real execute_flush"
    - "HaveBytesEvidence lives on the status contract; the network accumulator only records last labels and three counters"

key-files:
  created:
    - packages/open-bitcoin-node/src/status/chainstate_durability.rs
    - packages/open-bitcoin-node/src/status/chainstate_durability/tests.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/durability.rs
    - packages/open-bitcoin-node/src/network/chainstate_durability_evidence.rs
    - packages/open-bitcoin-node/src/network/chainstate_durability_evidence/tests.rs
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
    - packages/open-bitcoin-node/src/network/operator_snapshot.rs
    - packages/open-bitcoin-node/src/network/types.rs
    - packages/open-bitcoin-cli/src/operator/status.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined each task RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "HaveBytesEvidence is defined on the status contract so project_chainstate_durability stays I/O-free"
  - "Dedicated breadcrumb group for HAVL files cites blockstorage.cpp HaveBlockData and validation.cpp"
  - "Leave CSOBS-01 and CSOBS-02 Pending until Plan 02/03 renderers and lifecycle-valid phase verification"

patterns-established:
  - "Pattern 1: One dedicated chainstate_durability field; never fold CSOBS into recovery_evidence or block_relay"
  - "Pattern 2: Fail-closed and interrupted-without-B publish no coins tip; never substitute interrupted H hashes"
  - "Pattern 3: Sibling HAVL accumulator stores last labels plus available/unavailable/index_known_without_payload counts only"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 144-2026-09-17T11-23-17
generated_at: 2026-09-17T13:41:10Z

duration: 71min
completed: 2026-09-17
---

# Phase 144 Plan 01: Shared Contract And Runtime Retention Summary

**OpenBitcoinStatusSnapshot now carries a dedicated `chainstate_durability` field backed by retained last flush/recovery facts and a sibling have-bytes accumulator, without inventing tips or extending `block_relay`.**

## Performance

- **Duration:** 71 min
- **Started:** 2026-09-17T12:30:00Z
- **Completed:** 2026-09-17T13:41:10Z
- **Tasks:** 3
- **Files modified:** 25

## Accomplishments

- Shared `ChainstateDurabilityEvidence` contract serializes the locked snake_case CSOBS shape. Stopped or unprojected runtimes stay `Unavailable` with `CHAINSTATE_DURABILITY_UNAVAILABLE_REASON`.
- `FlushLifecycle` retains the last real `execute_flush` decision and coins-marker recovery. Current `cache_size` is `decide_flush(FlushMode::None)` occupancy; `last_flush_reason` / `write_kind` are never overwritten by that None classify.
- Fail-closed and interrupted-without-B leave both coins-best-block fields `None`. Replay missing-body errors include `fail_closed` (and `interrupted` when two heads could not finish).
- Sibling `HaveBytesAccumulator` records last serving labels plus `available_count`, `unavailable_count`, and `index_known_without_payload_count`. Payload-absent decisions are unavailable. `ManagedNetworkOperatorSnapshot` copies the shared field.

## Task Commits

Each task was committed atomically:

1. **Task 1: Define ChainstateDurabilityEvidence and snapshot field** - `4c05b71f` (feat)
2. **Task 2: Retain last FlushExecution and recovery facts** - `4b5aefec` (feat)
3. **Task 3: Sibling HAVL accumulator and managed snapshot copy** - `2499e636` (feat)

**Plan metadata:** pending docs commit after this summary.

_Note: Combined RED+GREEN in one hook-passing feat commit per task because pre-commit runs `verify.sh`, matching Phases 140–143._

## Files Created/Modified

- `packages/open-bitcoin-node/src/status/chainstate_durability.rs` - Shared CSOBS contract, labels, `HaveBytesEvidence`, and I/O-free projector
- `packages/open-bitcoin-node/src/status/chainstate_durability/tests.rs` - Locked JSON, fail-closed tip omission, and `as_str` coverage
- `packages/open-bitcoin-node/src/status.rs` - Dedicated `chainstate_durability` field after `block_relay`
- `packages/open-bitcoin-node/src/lib.rs` - Re-export `ChainstateDurabilityEvidence`
- `packages/open-bitcoin-cli/src/operator/status.rs` - Compile-fix snapshot literals with `default_unavailable()`
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` - Retained write decision and recovery outcome; `project_chainstate_durability`
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/durability.rs` - Retention, None-mode occupancy, and fail-closed recovery tests
- `packages/open-bitcoin-node/src/network/chainstate_durability_evidence.rs` - Sibling last/aggregate HAVL accumulator
- `packages/open-bitcoin-node/src/network/chainstate_durability_evidence/tests.rs` - Accumulator rules and managed snapshot copy
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` - `record_block_serving_evidence` also records have-bytes
- `packages/open-bitcoin-node/src/network/operator_snapshot.rs` - Projects retained flush and HAVL facts
- `packages/open-bitcoin-node/src/network/types.rs` - `chainstate_durability()` getter
- `docs/parity/source-breadcrumbs.json` - Contract, flush, HAVL, and Plan 02/03 pre-registration
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC

## Decisions Made

- Combined RED+GREEN because hooks run the workspace verifier; a RED-only commit cannot pass pre-commit.
- Define `HaveBytesEvidence` on the status contract so later tasks do not deadlock on missing types and the projector stays I/O-free.
- Keep current `cache_size` on `decide_flush(FlushMode::None)` and last write reason/kind on the last real `execute_flush`.
- Give HAVL files their own breadcrumb group citing `blockstorage.cpp` HaveBlockData and `validation.cpp` instead of the compact-relay group.
- Leave CSOBS-01 and CSOBS-02 Pending. Plan 02 still owns CLI/dashboard human lines; Plan 03 still owns metrics, logs, and support.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Extract flush durability tests from oversized tests.rs**
- **Found during:** Task 2
- **Issue:** Adding Task 2 tests to `flush_lifecycle/tests.rs` would exceed the 1000-line test-file trigger.
- **Fix:** Extracted durability coverage to `flush_lifecycle/tests/durability.rs` with the chainstate-adapter breadcrumbs.
- **Files modified:** `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests.rs`, `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/durability.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `cargo test -p open-bitcoin-node --lib durability` and pre-commit `verify.sh`
- **Committed in:** `4b5aefec` (Task 2)

**2. [Rule 3 - Blocking] Dedicated HAVL breadcrumb group**
- **Found during:** Task 3
- **Issue:** Task 1 pre-registered the new files under the compact-relay evidence group, but Task 3 requires `blockstorage.cpp` / `validation.cpp` file-header cites. Sharing the compact-relay group would rewrite `block_relay_evidence.rs` headers.
- **Fix:** Moved the two HAVL files into `node-network-chainstate-durability-evidence` with the HaveBlockData cites.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts` via pre-commit
- **Committed in:** `2499e636` (Task 3)

**3. [Rule 1 - Bug] Clippy `manual_unwrap_or_default` on coins tip lookup**
- **Found during:** Task 3 commit
- **Issue:** `match coins_best_block() { Ok(v) => v, Err(_) => None }` failed clippy `-D warnings`.
- **Fix:** Use `.ok().flatten()` so a coins-view error publishes no tip without `unwrap`.
- **Files modified:** `packages/open-bitcoin-node/src/network/operator_snapshot.rs`
- **Verification:** pre-commit `verify.sh`
- **Committed in:** `2499e636` (Task 3)

---

**Total deviations:** 3 auto-fixed (2 blocking, 1 bug)
**Impact on plan:** All auto-fixes necessary for file-length, breadcrumb, and clippy gates. No scope creep. Combined RED+GREEN was an allowed plan exception, not a deviation.

## Issues Encountered

- First Task 3 commit failed clippy on the coins-best-block match; fixed with `.ok().flatten()` and recommitted. No amend.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 02 can render human `OK` / `LARGE` / `CRITICAL` and last-flush tokens from the shared field without reopening Fjall or reclassifying occupancy.
- Plan 03 can log `fail_closed` from `StorageError` Display and emit metrics from the same sanitized labels.
- No `getblock`, ninth chart, or Plan 02/03 renderer work landed beyond compile-fixing snapshot literals.

---
*Phase: 144-operator-flush-and-availability-evidence*
*Completed: 2026-09-17*

## Self-Check: PASSED
