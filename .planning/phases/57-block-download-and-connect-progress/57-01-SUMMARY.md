---
phase: 57-block-download-and-connect-progress
plan: 01
subsystem: sync
tags: [rust, sync, block-download, peer-manager, tests]

requires:
  - phase: 56-header-ibd-convergence
    provides: header-first durable sync loop and best-chain header persistence
provides:
  - bounded best-chain-only block request regression coverage
  - no-credit in-flight cleanup regression coverage
  - invalid block retry eligibility after validation failure
affects: [sync, peer-manager, block-download-progress, BLK-01, BLK-04]

tech-stack:
  added: []
  patterns:
    - deterministic scripted sync transport tests
    - peer-manager requested-state cleanup before durable progress credit

key-files:
  created:
    - .planning/phases/57-block-download-and-connect-progress/57-01-SUMMARY.md
  modified:
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-node/src/sync/tests.rs

key-decisions:
  - "Keep block body scheduling driven only by DurableSyncRuntime best-chain reconciliation, not raw inventory announcements."
  - "Treat a received block body as locally known only after managed validation/storage accepts it."
  - "Use end-of-peer-session cleanup for malformed receive errors that cannot expose a parsable block hash."

patterns-established:
  - "Scripted two-peer sync tests assert retry eligibility by counting repeated getdata requests for the same best-chain block hash."
  - "PeerManager clears requested block inventory on block receipt while ManagedPeerNetwork remains responsible for marking accepted blocks known."

requirements-completed: [BLK-01, BLK-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 57-2026-06-03T13-56-54
generated_at: 2026-06-03T20:58:00Z

duration: ~1h
completed: 2026-06-03
---

# Phase 57 Plan 01: Block Download Request Bounds Summary

**Best-chain-only block request tests with retry-safe in-flight cleanup for notfound, disconnect, invalid, and malformed peer paths**

## Performance

- **Duration:** ~1h
- **Started:** 2026-06-03T20:20:00Z (approximate; exact executor start was not preserved across context compaction)
- **Completed:** 2026-06-03T20:58:00Z
- **Tasks:** 2
- **Files created/modified:** 4

## Accomplishments

- Added deterministic tests proving block body requests are scheduled only from validated best-chain headers and respect both per-peer and total in-flight caps.
- Added no-credit cleanup tests for `notfound`, disconnect, invalid block data, and malformed receive errors.
- Fixed invalid block retry suppression by preventing raw `block` receipt from marking the hash locally known before validation/storage succeeds.

## Task Commits

1. **Task 1: Prove best-chain-only bounded block scheduling** - `6ee1a12` (test)
2. **Task 2: Prove in-flight release on no-credit cleanup paths** - `9d8c0aa` (fix)

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/tests.rs` - Added bounded request tests, two-peer cleanup tests, and scripted malformed receive transport coverage.
- `packages/open-bitcoin-network/src/peer.rs` - Stopped marking received block bodies as known before managed validation/storage accepts them.
- `docs/metrics/lines-of-code.md` - Refreshed by the repo hook after Rust source/test changes.
- `.planning/phases/57-block-download-and-connect-progress/57-01-SUMMARY.md` - Execution summary and self-check artifact.

## Decisions Made

- Kept production request scheduling unchanged for Task 1 because the existing `block_reconcile::request_missing_blocks` path already used `best_chain_entries()`, skipped active/local/in-flight hashes, and enforced total caps.
- Left malformed payload hash attribution on the end-of-session cleanup path because the test transport models an error after request emission without a parsable block message.
- Moved local-known authority to the managed network validation/storage path instead of adding a broader rollback API.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Invalid block bodies suppressed later retries**
- **Found during:** Task 2 (Prove in-flight release on no-credit cleanup paths)
- **Issue:** `PeerManager::handle_block` marked a block hash as locally known before `ManagedPeerNetwork` validated or stored the block. An invalid body cleared requested state but left the hash known, so later eligible peers skipped the same missing best-chain block.
- **Fix:** Removed the premature `known_blocks` insertion from raw block receipt. Successful block storage/connection already records local block hashes through managed network paths.
- **Files modified:** `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-node/src/sync/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node block_inflight --all-features`
- **Committed in:** `9d8c0aa`

---

**Total deviations:** 1 auto-fixed (Rule 1)
**Impact on plan:** The fix was required for BLK-04 correctness and stayed within the block request cleanup surface.

## Issues Encountered

- Task 1 TDD tests passed without production changes because bounded best-chain scheduling was already implemented.
- The first Task 2 focused run failed `block_inflight_invalid_block_releases_runtime_and_peer_inflight_for_retry`, confirming the stale known-block suppression bug. The production fix resolved the failure.

## Verification

- `cargo fmt --all --manifest-path packages/Cargo.toml` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node bounded_block_requests --all-features` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node block_inflight --all-features` - passed
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed
- Pre-commit hook `bash scripts/verify.sh` - passed for both task commits; Task 2 hook completed in 1m46s

## Known Stubs

None - touched files were scanned for placeholder/TODO/FIXME and hardcoded empty UI-flow stubs.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 57 can continue with progress projection and operator-surface work knowing BLK-01 and the request-cleanup portion of BLK-04 have deterministic regression coverage.

## Self-Check: PASSED

- `FOUND:.planning/phases/57-block-download-and-connect-progress/57-01-SUMMARY.md`
- `FOUND:6ee1a12`
- `FOUND:9d8c0aa`
- Worktree scope confirmed: only this summary plus pre-existing orchestrator-owned `.planning/STATE.md` and `.planning/ROADMAP.md` remain uncommitted.

---
*Phase: 57-block-download-and-connect-progress*
*Completed: 2026-06-03*
