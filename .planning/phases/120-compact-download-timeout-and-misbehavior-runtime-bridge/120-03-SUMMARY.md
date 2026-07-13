---
phase: 120-compact-download-timeout-and-misbehavior-runtime-bridge
plan: 03
subsystem: network
tags: [compact-blocks, received-block, volatile-cleanup, gov-03, parity-breadcrumbs]

requires:
  - phase: 120-compact-download-timeout-and-misbehavior-runtime-bridge
    provides: Timeout tick (Plan 01) and misbehavior escalation (Plan 02)
  - phase: 115-missing-transaction-round-trip-fallback-and-validation-handoff
    provides: on_compact_download_block_connected and Phase 115 cleanup proofs
provides:
  - ReceivedBlock path calls on_compact_download_block_connected before connect_stored_block
  - BlockConnected cleanup evidence when multi-peer matching in_flight slots are removed
  - GOV-03 ManagedPeerNetwork proofs for disconnect/timeout/reorg-restart/ReceivedBlock
  - node-compact-cleanup-volatile parity breadcrumb registry entry
affects:
  - Phase 121 DurableSyncRuntime metrics (explicitly untouched)
  - Phase 120 closeout / verification

tech-stack:
  added: []
  patterns:
    - Clear volatile compact in_flight across all peers on ReceivedBlock before connect
    - PeerManager on_compact_download_block_connected returns removed peer count for evidence

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs
  modified:
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-network/src/peer/compact_download_state.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Clear volatile compact slots before connect_stored_block so connect failures cannot leave multi-peer stale in_flight"
  - "on_compact_download_block_connected returns removed peer count for BlockConnected evidence"
  - "Phase 121 persist_metrics / block_relay_log_record remain untouched; package/filter defaults stay off"

patterns-established:
  - "Pattern: ReceivedBlock shell wiring mirrors inventory handle_block multi-peer clear"
  - "Pattern: GOV-03 proofs assert durable chain tip/block map unchanged after volatile cleanup"

requirements-completed: [GOV-03, RCN-07, GOV-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 120-2026-07-13T20-01-34
generated_at: 2026-07-13T23:55:00Z

duration: 41min
completed: 2026-07-13
---

# Phase 120 Plan 03: Compact ReceivedBlock Cleanup and GOV-03 Proofs Summary

**Wired `PeerAction::ReceivedBlock` to clear matching volatile compact in_flight across all peers, recorded BlockConnected cleanup evidence, and proved GOV-03 volatile-only cleanup with Phase 121 / package-filter isolation.**

## Performance

- **Duration:** 41 min
- **Started:** 2026-07-13T23:14:31Z
- **Completed:** 2026-07-13T23:55:00Z
- **Tasks:** 2/2
- **Files modified:** 5

## Accomplishments

- Wired `process_actions` `ReceivedBlock` to call `on_compact_download_block_connected` before `connect_stored_block` and record `CompactDownloadCleanupCause::BlockConnected` when slots are removed.
- Returned removed peer count from `PeerManager::on_compact_download_block_connected` for accurate evidence.
- Added `compact_cleanup_cases` proving multi-peer ReceivedBlock clear, disconnect/timeout durable-store preservation, reorg/restart volatile-only cleanup, Phase 115 regression wrapper, and package/filter/Phase 121 isolation.
- Registered `node-compact-cleanup-volatile` parity breadcrumbs for Plans 01–03 coverage sweep.

## Task Commits

Each task was committed atomically:

1. **Task 1: Wire ReceivedBlock → on_compact_download_block_connected** - `928e34f5` (feat)
2. **Task 2: GOV-03 runtime proofs + parity breadcrumbs + Phase 121 isolation** - `9b30abec` (feat)

**Plan metadata:** pending final docs commit

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/action_translation.rs` - ReceivedBlock multi-peer clear + BlockConnected evidence
- `packages/open-bitcoin-network/src/peer/compact_download_state.rs` - return removed count from block-connected cleanup
- `packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs` - GOV-03 ManagedPeerNetwork proofs
- `packages/open-bitcoin-node/src/network/tests.rs` - module include
- `docs/parity/source-breadcrumbs.json` - `node-compact-cleanup-volatile` group

## Decisions Made

- Prefer clear-before-connect so a connect failure cannot leave other peers' matching compact slots.
- Extend `on_compact_download_block_connected` to return `usize` rather than re-scanning `known_peers` for evidence counts.
- Keep `docs/parity/index.json` unchanged — no new intentional OB-vs-Knots Invalid→Disconnect note beyond Plan 02.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Critical] Return removed count from PeerManager cleanup**
- **Found during:** Task 1
- **Issue:** Counting matching in_flight via shell-side peer iteration was less accurate than walking `compact_download_states` directly.
- **Fix:** `on_compact_download_block_connected` now returns the number of peers that lost a matching slot.
- **Files modified:** `packages/open-bitcoin-network/src/peer/compact_download_state.rs`, `packages/open-bitcoin-node/src/network/action_translation.rs`
- **Verification:** Phase 115 + compact_cleanup focused tests
- **Committed in:** `928e34f5`

**2. [Rule 3 - Blocking] TDD RED/GREEN collapsed**
- **Found during:** Task 1–2
- **Issue:** Pre-commit full `verify.sh` (~12m) and breadcrumb scope requiring tracked files made a pure failing RED commit impractical without orphan mapping failures.
- **Fix:** Implemented wiring then proofs in two feat commits; behavior matches `<behavior>` criteria; focused tests written and green before Task 2 commit.
- **Impact:** No behavior gap; commit history is feat→feat rather than test→feat pairs.

**3. [Rule 1 - Bug] BlockTxn connect timestamp and self-referential Phase 121 assertion**
- **Found during:** Task 2 (pre-commit discovery via untracked module)
- **Issue:** Completing compact with timestamp `1001` failed `time-too-new`; `include_str` negative check matched its own `persist_metrics(` / `block_relay_log_record(` literals.
- **Fix:** Use `i64::from(announced.header.time)` for connect; split Phase 121 markers with `concat!`.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs`
- **Verification:** `cargo test -p open-bitcoin-node compact_cleanup`
- **Committed in:** `9b30abec`

---

**Total deviations:** 3 auto-fixed (Rules 1–3)
**Impact on plan:** Necessary for evidence accuracy, commit/breadcrumb hygiene, and green GOV-03 proofs. No Phase 121 scope creep.

## Issues Encountered

- Pre-commit full `verify.sh` is long-running (~12m per commit).
- Breadcrumb registry groups must match tracked in-scope Rust files; untracked mapped files fail `check-parity-breadcrumbs.ts`.

## User Setup Required

None.

## Next Phase Readiness

- Phase 120 GOV-03 ReceivedBlock multi-peer volatile clear is closed on the ManagedPeerNetwork live path.
- RCN-07/GOV-02 proofs from Plans 01–02 remain green.
- DurableSyncRuntime `persist_metrics` / `block_relay_log_record` remain untouched (Phase 121).
- Orchestrator owns STATE.md / ROADMAP.md updates for this plan.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs`
- FOUND: `on_compact_download_block_connected` in ReceivedBlock arm of `action_translation.rs`
- FOUND: `CompactDownloadCleanupCause::BlockConnected` evidence recording
- FOUND: commit `928e34f5`
- FOUND: commit `9b30abec`
- FOUND: focused tests `compact_cleanup`, `compact_timeout`, `compact_misbehavior` passed
- FOUND: `bun scripts/check-parity-breadcrumbs.ts` green under Task 2 commit
- FOUND: `git diff` does not list Phase 121 `persist_metrics` / `block_relay_log_record` surfaces
