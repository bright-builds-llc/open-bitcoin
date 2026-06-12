---
phase: 70-reorg-peer-rotation-and-no-progress-recovery
plan: 03
subsystem: sync-runtime
tags: [sync, peer-rotation, retry-backoff, stale-inflight, deterministic-tests]

requires: [70-02]
provides:
  - No-credit block responses release stale in-flight work
  - Typed peer failure attribution for notfound, malformed, invalid, duplicate, disconnected, and non-extending block responses
  - Endpoint-keyed retry/backoff for no-credit, stalled, incompatible, and network-error peers
  - Deterministic peer rotation without production peer banning or reputation scope
affects: [phase-70, sync-runtime, peer-status, block-download]

tech-stack:
  added: []
  patterns:
    - PeerProgress decides whether an outcome counts as a successful outbound slot
    - PeerFailureReason classifies no-credit block responses for retry/backoff
    - Phase 70 peer regressions live under a phase70_peer test module for focused execution

key-files:
  created:
    - .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-03-SUMMARY.md
  modified:
    - .planning/ROADMAP.md
    - .planning/STATE.md
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/progress.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/sync/types.rs

key-decisions:
  - "No-credit block responses remain typed peer outcomes but no longer satisfy the target outbound slot."
  - "No-credit, stalled, incompatible, and network-error peers use the existing endpoint-keyed retry/backoff map."
  - "Keep Phase 70 peer recovery scoped to outbound sync retries; do not add banning, reputation, addrman, relay, or compact-block policy."

patterns-established:
  - "PeerProgress::is_successful_outbound_slot guards connected peer slot accounting."
  - "PeerProgress::should_retry_with_backoff centralizes retryable no-credit and stalled outcomes."
  - "Phase 70 peer tests assert stale in-flight release, exact failure labels, backoff keys, and replacement peer attempts."

requirements-completed: [REC-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 70-2026-06-12T14-56-46
generated_at: 2026-06-12T20:49:33Z

duration: 32min
completed: 2026-06-12
---

# Phase 70 Plan 03: Peer Attribution, Stale In-Flight Release, and Rotation Summary

**Peer failures and no-credit block responses now release stale work, keep exact typed attribution, enter endpoint-keyed backoff, and rotate to eligible replacement peers.**

## Performance

- **Duration:** 32 min
- **Started:** 2026-06-12T20:17:45Z
- **Completed:** 2026-06-12T20:49:33Z
- **Tasks:** 2
- **Files modified:** 6
- **Files created:** 1

## Accomplishments

- Added `PeerFailureReason::is_no_credit_block_response` for the retryable no-credit block labels.
- Added `PeerProgress::is_successful_outbound_slot` so connected outcomes with no-credit failure reasons do not consume the configured target slot.
- Added `PeerProgress::should_retry_with_backoff` and reused the existing endpoint-keyed `peer_backoff` map for no-credit and stalled outcomes.
- Preserved existing release behavior for requested `Block` and `NotFound` messages before attribution.
- Added focused Phase 70 peer tests for `notfound`, malformed, invalid, duplicate, disconnected, non-extending, stalled, incompatible, and network-error peers.
- Proved replacement peer attempts remain bounded and deterministic under the local scripted transport.
- Regenerated the tracked LOC report after adding the regression coverage.

## Task Commits

Implementation and summary are in the pending `70-03` commit prepared after verification.

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/types.rs` - Added no-credit block response classification.
- `packages/open-bitcoin-node/src/sync/progress.rs` - Added peer progress predicates for successful slots and retry/backoff outcomes.
- `packages/open-bitcoin-node/src/sync.rs` - Counted only successful connected outcomes toward the outbound slot target and marked no-credit outcomes for backoff.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Added the `phase70_peer` deterministic regression module.
- `docs/metrics/lines-of-code.md` - Regenerated the tracked LOC report.
- `.planning/ROADMAP.md` and `.planning/STATE.md` - Advanced Phase 70 progress to plan 04.

## Decisions Made

- Kept no-credit block outcomes as normal peer outcomes with typed failure reasons instead of adding a new error type.
- Counted `connected_peers` as successful outbound slots for status pressure so no-credit peers do not inflate healthy outbound evidence.
- Used the existing retry/backoff machinery rather than introducing peer bans, eviction, reputation, address-manager changes, compact-block fallback, or relay policy.

## Deviations from Plan

None.

## Issues Encountered

- Running clippy and build concurrently caused Cargo to serialize on its package/build locks. Both commands completed successfully after the lock cleared.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase70_peer --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync:: --all-features`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- Acceptance `rg` checks for required Phase 70 peer test names, typed failure labels, endpoint-keyed backoff, connected-slot accounting, and default-verification exclusions.

## User Setup Required

None - all coverage is deterministic and local.

## Next Phase Readiness

Phase 70-04 can build the no-progress status contract on top of typed peer retry/backoff and stale in-flight release evidence.

## Self-Check: PASSED

- Summary file exists.
- Required Phase 70 peer tests pass under the `phase70_peer` filter.
- No production peer governance or public-network verification scope was added.

*Phase: 70-reorg-peer-rotation-and-no-progress-recovery*
*Completed: 2026-06-12*
