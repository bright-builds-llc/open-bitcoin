---
phase: 58-same-datadir-restart-and-resume-evidence
plan: 01
subsystem: sync
tags: [rust, durable-sync, same-datadir, restart, fjall]
requires:
  - phase: 57-block-download-and-connect-progress
    provides: separated downloaded and connected block progress evidence
provides:
  - deterministic same-datadir reopen tests for durable header and block status
  - no-duplicate getdata/connect evidence after reopening the same Fjall datadir
affects: [sync-runtime, live-smoke-restart-evidence, resume-evidence]
tech-stack:
  added: []
  patterns: [real Fjall reopen regression tests, ScriptedTransport assertions]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/sync/tests.rs
key-decisions:
  - "Tightened existing restart tests instead of adding a separate test module."
  - "Used ScriptedTransport and runtime summaries as observable evidence rather than direct Fjall key inspection."
patterns-established:
  - "same_datadir test names mark durable resume evidence for future grep-based audits."
requirements-completed: [RESUME-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 58-2026-06-05T12-58-05
generated_at: 2026-06-05T13:27:35Z
duration: 10min
completed: 2026-06-05
---

# Phase 58: Plan 01 Summary

**Deterministic same-datadir reopen coverage for durable headers, downloaded/connected block hashes, and no duplicate block requests.**

## Performance

- **Duration:** 10 min
- **Started:** 2026-06-05T13:17:00Z
- **Completed:** 2026-06-05T13:27:35Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments

- Renamed restart-focused sync tests with `same_datadir` evidence labels.
- Asserted reopened runtime summaries preserve durable downloaded and connected block hashes.
- Asserted already connected blocks are absent from post-reopen `getdata` requests and do not count as new received blocks.
- Preserved deterministic, public-network-free verification.

## Task Commits

Task commits are deferred to the final strict yolo push gate for this run. No code is committed until phase verification and repo verification pass.

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/tests.rs` - Tightened same-datadir durable reopen tests and no-duplicate request assertions.

## Decisions Made

- Reused the existing real-Fjall restart tests because they already exercise `FjallNodeStore::open(&path)` before and after reopening the same datadir.
- Kept evidence at the sync-runtime observable surface: `snapshot_summary`, `SyncProgress`, `getdata_block_hashes`, and `blocks_received`.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node same_datadir --all-features` - passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node restart --all-features` - passed.
- `rg -n "same_datadir.*header|same_datadir.*downloaded|maybe_downloaded_block_hash|maybe_connected_block_hash|getdata_block_hashes\\(&transport\\.sent_messages\\(\\)\\)|same_datadir.*duplicate|blocks_received, 0|best_available_branch" packages/open-bitcoin-node/src/sync/tests.rs` - found the required assertions.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 58-02 can build restart-resume smoke evidence on deterministic same-datadir resume tests. No blockers.

---
*Phase: 58-same-datadir-restart-and-resume-evidence*
*Completed: 2026-06-05*
