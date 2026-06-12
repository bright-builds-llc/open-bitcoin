---
phase: 70-reorg-peer-rotation-and-no-progress-recovery
plan: 05
subsystem: shared-status
tags: [sync, no-progress, status-projection, cli-status, recovery-guidance]

requires: [70-02, 70-03, 70-04]
provides:
  - Pure no-progress diagnosis classifier
  - Shared no-progress next-action mapping
  - Durable sync status projection from runtime evidence
  - CLI rendering of shared no-progress fields
affects: [phase-70, node-sync, shared-status, cli-status]

tech-stack:
  added: []
  patterns:
    - Pure classifier maps typed sync evidence to shared status variants
    - Storage/resource blockers outrank peer guidance
    - CLI renderer consumes shared status fields without local reclassification

key-files:
  created:
    - .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-05-SUMMARY.md
  modified:
    - .planning/ROADMAP.md
    - .planning/STATE.md
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-node/src/sync/progress.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs

key-decisions:
  - "Keep no-progress classification in sync progress code as a pure helper."
  - "Project diagnosis and guidance from runtime state so all consumers share the same contract."
  - "Render the serialized shared diagnosis label instead of adding renderer-local diagnosis wording."

patterns-established:
  - "NoProgressInput carries stay-current, progress signal, recovery category, in-flight work, reconcile progress, and peer outcomes."
  - "classify_no_progress uses deterministic precedence across storage/resource, recovery, branch competition, at-tip, block-body, stale in-flight, peer, and behind-header cases."
  - "no_progress_next_action is the single source for bounded operator guidance strings."

requirements-completed: [REC-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 70-2026-06-12T14-56-46
generated_at: 2026-06-12T22:52:10Z

duration: 57min
completed: 2026-06-12
---

# Phase 70 Plan 05: Shared No-Progress Diagnosis and Rendering Summary

**No-progress status now classifies typed sync evidence into shared diagnosis and next-action fields, and the CLI renders those fields directly.**

## Performance

- **Duration:** 57 min
- **Started:** 2026-06-12T21:55:02Z
- **Completed:** 2026-06-12T22:52:10Z
- **Tasks:** 2
- **Files modified:** 8
- **Files created:** 1

## Accomplishments

- Added `NoProgressInput`, `classify_no_progress`, and `no_progress_next_action` in sync progress code.
- Implemented precedence for current-at-tip, behind-awaiting-headers, awaiting block bodies, stale in-flight cleanup, peer backoff, peer stalls, exhausted peer failures, branch competition awaiting bodies, recovery, and storage/resource blockers.
- Projected available `SyncStatus.no_progress_diagnosis` and `SyncStatus.no_progress_next_action` from `DurableSyncRuntime` status summaries.
- Updated CLI human status rendering to display the shared no-progress diagnosis and action fields.
- Added deterministic node projection tests for at-tip, branch competition, peer backoff, stale in-flight cleanup, and storage/resource blockers.
- Added CLI rendering coverage proving the displayed diagnosis/action come from shared `SyncStatus` fields.
- Regenerated the tracked LOC report after source and test changes.

## Task Commits

Implementation and summary are included in the `70-05` commit created after verification.

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/progress.rs` - Added the pure classifier, next-action helper, storage/resource precedence, and focused classifier tests.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Projected no-progress diagnosis and next-action fields from runtime status evidence.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Added deterministic status projection coverage for the required no-progress causes.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Rendered shared no-progress fields in the sync status section.
- `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - Added CLI coverage for shared diagnosis/action rendering and unavailable fallback text.
- `docs/metrics/lines-of-code.md` - Regenerated the tracked LOC report.
- `.planning/ROADMAP.md` and `.planning/STATE.md` - Advanced Phase 70 progress to plan 06.

## Decisions Made

- Kept the classifier pure and local to sync progress code so runtime projection can pass typed evidence without CLI/RPC duplication.
- Let storage and resource blockers take precedence over peer advice, preserving the plan's denial-of-service guidance order.
- Used serde serialization for the human diagnosis label so the renderer consumes the shared enum label without adding another status-string mapping.

## Deviations from Plan

- No behavior deviation. The renderer uses serde label projection for the shared enum instead of adding a dedicated `as_str` helper to `status.rs`, keeping the shared status file below the repo file-length gate.

## Issues Encountered

- A first full verifier attempt exposed the production file-length gate in `packages/open-bitcoin-node/src/status.rs` after adding an enum label helper. The helper was removed, CLI rendering now derives the shared label through serde, LOC was regenerated, and `bash scripts/verify.sh` passed.
- Focused Cargo tests were accidentally started in parallel once and waited on Cargo's package/build locks. Subsequent full verification was run sequentially and passed.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase70_no_progress --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli no_progress --all-features`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bash scripts/verify.sh`
- Acceptance `rg` checks for classifier APIs, exact next-action strings, runtime projection, CLI rendering, required projection tests, CLI shared-field assertions, and absence of prohibited default-verification additions or broad production wording.

## User Setup Required

None - all coverage is deterministic and local.

## Next Phase Readiness

Phase 70-06 can close operator docs, README relevance, deterministic verification, and phase-level verification status.

## Self-Check: PASSED

- Summary file exists.
- No-progress diagnosis is typed, shared, precedence-ordered, and renderer-independent.
- Runtime status projection and CLI rendering consume the same shared fields.
- Full repo verification passed after the file-length fix.

*Phase: 70-reorg-peer-rotation-and-no-progress-recovery*
*Completed: 2026-06-12*
