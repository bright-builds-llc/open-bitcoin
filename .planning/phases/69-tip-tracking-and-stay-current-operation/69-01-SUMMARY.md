---
phase: 69-tip-tracking-and-stay-current-operation
plan: 01
subsystem: node-status
tags: [rust, sync-status, serde, tip-tracking]

requires:
  - phase: 68-full-active-chain-validation-and-durable-persistence
    provides: Validated active-chain progress fields consumed by later Phase 69 runtime projection.
provides:
  - Typed best-known tip status DTOs
  - Typed stay-current status DTO
  - Additive SyncStatus serde defaults for Phase 69 fields
  - Serialization/default coverage for legacy and new status payloads
affects: [sync-status, operator-status, rpc-status, dashboard-status, phase-69]

tech-stack:
  added: []
  patterns: [additive-serde-status-fields, field-availability-defaults]

key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/status.rs
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
  - "Use additive FieldAvailability defaults so old SyncStatus JSON decodes with Phase 69 fields unavailable."
  - "Keep runtime-derived best-tip and stay-current classification out of Plan 69-01; summary constructors expose unavailable placeholders until Plan 69-02."

patterns-established:
  - "Status contract fields added to SyncStatus must be carried through CLI/RPC fixture constructors immediately so downstream crates compile."

requirements-completed: [TIP-01, TIP-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 69-2026-06-11T15-13-14
generated_at: 2026-06-11T16:55:00Z

duration: 31min
completed: 2026-06-11
---

# Phase 69-01: Shared Tip And Stay-Current Status Contract

**Additive typed SyncStatus fields for best-known tip evidence and stay-current state, with legacy JSON defaults.**

## Performance

- **Duration:** 31 min
- **Started:** 2026-06-11T16:24:00Z
- **Completed:** 2026-06-11T16:55:00Z
- **Tasks:** 3
- **Files modified:** 11

## Accomplishments

- Added `BestKnownTipSource`, `TipFreshnessStatus`, `PeerTipAgreementStatus`, `PeerTipAgreement`, `BestKnownTipStatus`, and `StayCurrentStatus`.
- Added `SyncStatus.best_known_tip` and `SyncStatus.stay_current` with serde defaults for older runtime/status JSON.
- Added deterministic Phase 69 serialization/default tests and updated downstream CLI/RPC fixtures for additive compile compatibility.

## Task Commits

1. **Tasks 1-3: Add typed Phase 69 status DTOs, defaults, and tests** - `ec67e25` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/status.rs` - New typed best-known tip and stay-current contracts.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Summary status constructor now fills Phase 69 fields as unavailable placeholders.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Added Phase 69 default and serialization tests.
- `packages/open-bitcoin-node/src/status/tests.rs` - Existing status fixtures assert legacy defaults.
- `packages/open-bitcoin-cli/src/operator/status/sync_state.rs` - CLI fallback/RPC-derived sync status fills unavailable Phase 69 fields.
- `packages/open-bitcoin-cli/src/operator/**/tests.rs` and `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Downstream fixtures updated for additive fields.
- `docs/metrics/lines-of-code.md` - Refreshed by repo-managed hooks.

## Decisions Made

Runtime derivation stays out of this plan. The new fields exist as shared typed contract and default to unavailable until Plan 69-02 derives real best-known tip and stay-current evidence.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Downstream SyncStatus literals needed additive fields**
- **Found during:** Plan 69-01 pre-commit Cargo sequence.
- **Issue:** CLI and RPC tests/constructors had explicit `SyncStatus` literals missing `best_known_tip` and `stay_current`.
- **Fix:** Added unavailable Phase 69 defaults to downstream CLI/RPC status constructors and fixtures.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/status/sync_state.rs`, CLI status/dashboard/runtime test fixtures, and `packages/open-bitcoin-rpc/src/dispatch/tests.rs`.
- **Verification:** Full Cargo pre-commit sequence and commit hook verification passed.
- **Committed in:** `ec67e25`

**Total deviations:** 1 auto-fixed (Rule 3).
**Impact on plan:** Required compile compatibility for the additive shared status contract; no scope expansion beyond carrying unavailable defaults.

## Issues Encountered

`cargo fmt --check` initially reported formatting changes and `cargo clippy` exposed downstream `SyncStatus` literals. Both were fixed before commit.

## User Setup Required

None - no external service configuration required.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all --check`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_summary_status_projections_include_counters --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase69_sync_status --all-features`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- Commit hook ran `bash scripts/verify.sh` successfully before `ec67e25`.

## Next Phase Readiness

Plan 69-02 can now derive real best-known tip evidence and stay-current classification into the typed fields.

## Self-Check: PASSED

---
*Phase: 69-tip-tracking-and-stay-current-operation*
*Completed: 2026-06-11*
