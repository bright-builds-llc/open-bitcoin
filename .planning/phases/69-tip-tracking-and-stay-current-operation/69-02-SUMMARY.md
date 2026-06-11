---
phase: 69-tip-tracking-and-stay-current-operation
plan: 02
subsystem: node-sync-runtime
tags: [rust, sync-runtime, tip-tracking, stay-current]

requires:
  - phase: 69-tip-tracking-and-stay-current-operation
    plan: 01
    provides: Typed best-known tip and stay-current status DTOs.
  - phase: 68-full-active-chain-validation-and-durable-persistence
    provides: Durable header store and validated active-chain progress evidence.
provides:
  - Best-known validated tip runtime projection
  - Stay-current classification from typed sync evidence
  - Per-peer terminal-header tip observations
  - Peer agreement classification against the best-known tip
  - Tip freshness threshold runtime configuration
affects: [sync-runtime, durable-status, peer-outcomes, parity-breadcrumbs, phase-69]

tech-stack:
  added: []
  patterns: [pure-status-projection, terminal-header-peer-evidence, additive-runtime-config]

key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tip.rs
  modified:
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/progress.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Record the peer's accepted terminal header from the received Headers message; do not infer peer evidence from the global best header-store tip."
  - "Classify missing best-known tip evidence as NoProgress unless useful progress occurred; missing evidence is not stale evidence."
  - "Keep stay-current and best-tip derivation in a private pure helper module so runtime_state only gathers inputs and assigns status fields."
  - "Keep sync.rs below the production line limit by moving tip formatting/observation helpers into sync::tip and peer error classification into sync::progress."

patterns-established:
  - "PeerSyncOutcome can carry optional terminal tip evidence while failed, waiting, and no-evidence peers explicitly remain None."
  - "Durable status projection should derive best-known tip fields from the header store and active-chain state at the same observed timestamp."

requirements-completed: [TIP-03, TIP-04, TIP-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 69-2026-06-11T15-13-14
generated_at: 2026-06-11T17:52:13Z

duration: 56min
completed: 2026-06-11
---

# Phase 69-02: Tip Evidence And Stay-Current Projection

**Derived best-known tip, peer agreement, and stay-current status from durable sync runtime evidence.**

## Performance

- **Duration:** 56 min
- **Completed:** 2026-06-11T17:52:13Z
- **Tasks:** 4
- **Files modified:** 9

## Accomplishments

- Added `sync::tip` as a pure projection helper for best-known tip status, peer agreement rows, freshness, and stay-current classification.
- Added optional peer terminal-tip evidence fields to `PeerSyncOutcome` and `PeerProgress`.
- Recorded each peer's accepted terminal header after successful `Headers` processing, preserving the peer's own observed tip even when the global header-store tip is ahead.
- Projected `SyncStatus.best_known_tip` and `SyncStatus.stay_current` from the durable header store, connected active-chain tip, peer outcomes, lifecycle, and progress evidence.
- Added `SyncRuntimeConfig.tip_freshness_threshold_seconds` with a default of `1_200`.
- Added parity breadcrumb coverage and refreshed the tracked LOC report.

## Task Commits

1. **Tasks 1-4: Derive best-known tip evidence and peer agreement** - `d56206b` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/tip.rs` - New pure helper for tip evidence, peer agreement, freshness, stay-current classification, and sync-local hash formatting.
- `packages/open-bitcoin-node/src/sync.rs` - Captures accepted terminal-header evidence from peer `Headers` messages and delegates tip helpers.
- `packages/open-bitcoin-node/src/sync/progress.rs` - Carries optional peer tip evidence and owns peer-error classification.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Populates `best_known_tip` and `stay_current` during durable status projection.
- `packages/open-bitcoin-node/src/sync/types.rs` - Adds tip freshness threshold config and optional peer tip evidence fields.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Updates direct peer outcome fixtures for no-evidence defaults.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Adds peer agreement and runtime terminal-header observation tests.
- `docs/parity/source-breadcrumbs.json` - Adds the `node-sync-tip-tracking` breadcrumb group.
- `docs/metrics/lines-of-code.md` - Refreshed by repo-managed hooks.

## Decisions Made

`BestKnownTipSource::HeaderStore` is used for runtime projection because the durable header store remains the authoritative validated best-known tip. Per-peer observations are retained as agreement rows rather than promoted to the best-known source unless future phases add a different evidence source.

## Deviations from Plan

### Auto-fixed Issues

**1. [Hook] Breadcrumb mapping initially made `sync/tip.rs` stale**
- **Found during:** First commit-hook attempt.
- **Issue:** `sync/tip.rs` was temporarily added to the broader `node-sync-runtime` mapping, whose breadcrumbs include `net.cpp`; the file header intentionally used the four Plan 69-02 anchors.
- **Fix:** Added a dedicated `node-sync-tip-tracking` mapping group with the exact anchors for `sync/tip.rs`.
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 233 Rust files.

**2. [Hook] `sync.rs` exceeded the production file-length limit**
- **Found during:** Second commit-hook attempt.
- **Issue:** `sync.rs` reached 652 lines, above the 628-line production limit.
- **Fix:** Moved sync-local hash formatting and terminal-tip recording into `sync::tip`; moved peer-error classification into `sync::progress`.
- **Verification:** Commit hook reported `Production Rust file-length check passed: 184 file(s) checked, limit 628 lines.`

**Total deviations:** 2 auto-fixed.
**Impact on plan:** No behavior expansion; both fixes improved structure while preserving the planned evidence semantics.

## Issues Encountered

The initial focused test run exposed one import path issue after adding `sync::tip`; `BlockProgressPoint` lives in `runtime_state`. The import was corrected before full verification.

## User Setup Required

None - all tests use local scripted sync/runtime fixtures. Public-network smoke remains explicitly opt-in and ignored by default.

## Verification

- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node tip:: --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase69_tip --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase69_peer_agreement_classifies_agrees_behind_disagrees_and_no_evidence --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase69_peer_tip_observation_uses_peer_terminal_header_not_global_best --all-features`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- Commit hook ran `bash scripts/verify.sh` successfully before `d56206b` and completed in `5m 35.630s`.

## Next Phase Readiness

Plan 69-03 can now build bounded stay-current next-action decisions on top of populated best-known tip and stay-current status fields.

## Self-Check: PASSED

---
*Phase: 69-tip-tracking-and-stay-current-operation*
*Completed: 2026-06-11*
