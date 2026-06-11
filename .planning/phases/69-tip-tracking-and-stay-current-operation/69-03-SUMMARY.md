---
phase: 69-tip-tracking-and-stay-current-operation
plan: 03
subsystem: node-sync-runtime
tags: [rust, sync-runtime, stay-current, operator-guidance]

requires:
  - phase: 69-tip-tracking-and-stay-current-operation
    plan: 01
    provides: Typed best-known tip and stay-current status DTOs.
  - phase: 69-tip-tracking-and-stay-current-operation
    plan: 02
    provides: Runtime best-known tip and stay-current status projection.
provides:
  - Bounded stay-current next-action operator guidance
  - Current-at-best-known-tip idle stop reason
  - Fresh idle-at-tip runtime regression coverage
affects: [sync-runtime, durable-status, operator-guidance, phase-69]

tech-stack:
  added: []
  patterns: [bounded-status-guidance, evidence-gated-idle-stop, additive-status-contract]

key-files:
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tip.rs
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/types/projection.rs
    - packages/open-bitcoin-node/src/sync/types/recovery.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/sync_state.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/runtime/support.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Expose stay_current_next_action as an additive SyncStatus field with a legacy-safe unavailable default."
  - "Keep Recovering without a next-action string so storage, peer, and lifecycle recovery guidance remains owned by existing recovery fields."
  - "Only report current_at_best_known_tip as an idle stop reason when fresh best-tip evidence and connected active-chain evidence agree."

patterns-established:
  - "Stay-current operator guidance should be selected from a bounded enum-derived helper instead of ad hoc runtime strings."
  - "Idle-at-tip success can be distinguished from no-progress diagnosis without public network access by using persisted header and active-chain evidence."

requirements-completed: [TIP-06, TIP-07]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 69-2026-06-11T15-13-14
generated_at: 2026-06-11T18:25:49Z

duration: 33min
completed: 2026-06-11
---

# Phase 69-03: Stay-Current Next Action And Idle-At-Tip Stop Reason

**Added bounded operator guidance for stay-current states and taught idle sync cycles to stop as current when fresh validated tip evidence matches the connected active chain.**

## Performance

- **Duration:** 33 min
- **Completed:** 2026-06-11T18:25:49Z
- **Tasks:** 4
- **Files modified:** 17

## Accomplishments

- Added `SyncStatus.stay_current_next_action` with legacy-compatible serde defaults and status fixture coverage.
- Added a bounded `stay_current_next_action` helper with the four planned operator strings and no recovery override.
- Added `SyncStopReason::CurrentAtBestKnownTip` with stable label, message, health signal, recovery mapping, and phase projection.
- Updated `sync_until_idle` to report `current_at_best_known_tip` instead of `no_progress` when fresh best-known tip evidence equals connected active-chain progress.
- Added a fresh idle-at-tip scripted runtime regression test that verifies persisted status, next action, best-known tip freshness, latest stop reason, and health signal evidence.
- Refreshed the tracked LOC report through the repo-managed commit hook.

## Task Commits

1. **Tasks 1-4: Add bounded next-action guidance and idle-at-tip stop reason** - `321af94` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/status.rs` - Adds the additive `stay_current_next_action` status contract field and default.
- `packages/open-bitcoin-node/src/sync/tip.rs` - Adds bounded next-action selection and current-at-best-known-tip stop-reason helpers.
- `packages/open-bitcoin-node/src/sync.rs` - Uses fresh tip evidence to distinguish idle-at-tip success from no-progress idle stops.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Projects available next-action guidance with stay-current status.
- `packages/open-bitcoin-node/src/sync/types.rs` - Adds the informational `CurrentAtBestKnownTip` stop reason.
- `packages/open-bitcoin-node/src/sync/types/projection.rs` - Projects the new stop reason as `current_at_best_known_tip`.
- `packages/open-bitcoin-node/src/sync/types/recovery.rs` - Keeps the success stop reason out of recovery categories.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Adds default unavailable status for summary projections.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Adds the fresh idle-at-tip regression test and updates serialization assertions.
- CLI, RPC, and status test fixtures - Carry the additive status field through existing constructors.
- `docs/metrics/lines-of-code.md` - Refreshed by repo-managed hooks.

## Decisions Made

The new stop reason is intentionally evidence-gated through `classify_stay_current`. A node with no useful fresh best-tip evidence still reports no-progress or stale-tip guidance; only a fresh best-known header tip matching connected active-chain progress can become `current_at_best_known_tip`.

## Deviations from Plan

None.

## Issues Encountered

None.

## User Setup Required

None - all new coverage uses local durable-store and scripted transport fixtures. Public-network smoke remains opt-in and ignored by default.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase69_stay_current_next_action --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase69_fresh_idle_cycle_reports_current_at_best_known_tip --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase69_sync_status --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_recovery_category_maps_operator_stop_reasons --all-features`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- Commit hook ran `bash scripts/verify.sh` successfully before `321af94` and completed in `5m 29.042s`.

## Next Phase Readiness

Plan 69-04 can now reuse the bounded stay-current status and next-action projection when adding daemon cadence and no-network default behavior.

## Self-Check: PASSED

---
*Phase: 69-tip-tracking-and-stay-current-operation*
*Completed: 2026-06-11*
