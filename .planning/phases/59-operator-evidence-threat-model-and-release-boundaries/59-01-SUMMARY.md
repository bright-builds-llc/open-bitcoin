---
phase: 59-operator-evidence-threat-model-and-release-boundaries
plan: 01
subsystem: operator-evidence
tags: [rust, sync-status, operator-cli, rpc, live-smoke, observability]

requires:
  - phase: 56-header-ibd-convergence
    provides: header progress and first-header evidence semantics
  - phase: 57-block-download-and-connect-progress
    provides: downloaded-versus-connected block progress semantics
  - phase: 58-same-datadir-restart-and-resume-evidence
    provides: restart/resume evidence and schema v2 live-smoke fields
provides:
  - OBS-01 deterministic consistency assertions across sync summary, metrics, logs, RPC, status, dashboard, and live-smoke fixtures
  - Latest sync peer error projection from SyncRunSummary status evidence
  - Explicit dashboard peer-count wording for inbound and outbound counts
affects: [operator-status, operator-dashboard, rpc-getblockchaininfo, live-smoke-fixtures, sync-summary]

tech-stack:
  added: []
  patterns:
    - Fixture-driven cross-surface assertions over shared sync/status data
    - Downloaded block height remains distinct from connected block height

key-files:
  created:
    - .planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-01-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - scripts/test-run-live-mainnet-smoke.sh

key-decisions:
  - "SyncRunSummary::sync_status now projects the latest peer error so summary-derived status, logs, and metrics do not diverge on OBS-01 latest-error evidence."
  - "Dashboard peer counts now spell out inbound/outbound labels while continuing to read from OpenBitcoinStatusSnapshot."
  - "Task commits were deferred to the final strict yolo push gate per wrapper instructions."

patterns-established:
  - "Cross-surface operator evidence tests use one deterministic fixture with headers=840100, downloaded=840006, connected=840004."
  - "Renderer tests assert shared snapshot values instead of adding renderer-local aggregation."

requirements-completed: [OBS-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 59-2026-06-05T15-10-59
generated_at: 2026-06-05T16:00:38Z

duration: 12min
completed: 2026-06-05
---

# Phase 59 Plan 01: Operator Evidence Consistency Summary

**Deterministic OBS-01 fixture coverage now keeps header, downloaded block, connected block, peer, signal, and latest-error evidence aligned across operator surfaces.**

## Performance

- **Duration:** 12 min
- **Started:** 2026-06-05T15:48:20Z
- **Completed:** 2026-06-05T16:00:38Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added `sync_summary_projects_consistent_operator_evidence_fields` to assert `SyncRunSummary` status, peer status, metrics, and structured logs agree on `840100/840006/840004`, `block_progress`, peer count, and latest error.
- Tightened RPC `getblockchaininfo` coverage so `blocks` stays the connected-height alias (`840004`), not the downloaded-only height (`840006`).
- Added status and dashboard renderer assertions proving `OpenBitcoinStatusSnapshot` values render unchanged, including `awaiting_blocks`, latest error text, and peer state/source evidence.
- Extended deterministic live-smoke fixture checks for nested schema v2 `firstHeaderProgress`, `firstBlockProgress`, and `restartResumeEvidence.recoveryDiagnosis` fields.

## Task Commits

Task commits were deferred to the final strict yolo push gate per wrapper instructions. No staging, commits, or pushes were performed by this executor.

1. **Task 1: Assert sync summary, metrics, logs, RPC, and live-smoke consistency** - deferred
2. **Task 2: Assert status and dashboard renderers use the shared snapshot** - deferred

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/types/summary.rs` - added OBS-01 projection test and projected latest peer error into summary status.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - renamed/tightened durable `getblockchaininfo` regression around connected height.
- `scripts/test-run-live-mainnet-smoke.sh` - added nested schema v2 fixture checks for first progress and restart diagnosis fields.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - added human renderer fixture test for shared sync truth fields.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - added dashboard fixture test and explicit inbound/outbound peer-count wording.
- `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-01-SUMMARY.md` - created this execution summary.

## Decisions Made

- Used the existing shared status and sync summary structures as the source of truth; no new runtime aggregation service was added.
- Kept public-network live smoke out of default verification; only deterministic fixture checks changed.
- Did not update `.planning/STATE.md`, `.planning/ROADMAP.md`, or requirements files because the wrapper constrained this executor to the plan file set plus summary.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Projected latest peer error from sync summary status**
- **Found during:** Task 1
- **Issue:** The RED test showed `SyncRunSummary::sync_status` left `last_error` unavailable even when peer outcomes carried `peer stalled before block connect`.
- **Fix:** `sync_status` now uses `latest_error_message()` for `last_error` while preserving the unavailable reason when no error exists.
- **Files modified:** `packages/open-bitcoin-node/src/sync/types/summary.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_summary_projects_consistent_operator_evidence_fields --all-features`
- **Committed in:** deferred to final strict yolo push gate

**2. [Rule 2 - Missing Critical] Made dashboard peer-count evidence explicit**
- **Found during:** Task 2
- **Issue:** The dashboard projection rendered compact `in=0 out=2`, which was less explicit than the plan's outbound peer-count evidence requirement.
- **Fix:** Dashboard peer-count rows now render `inbound=0 outbound=2`, still sourced directly from `OpenBitcoinStatusSnapshot`.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_projection_preserves_shared_sync_truth_fields --all-features`
- **Committed in:** deferred to final strict yolo push gate

***

**Total deviations:** 2 auto-fixed (2 missing critical)
**Impact on plan:** Both fixes keep OBS-01 evidence truthful without broadening sync capability, adding runtime aggregation, or changing public-network verification.

## Issues Encountered

- Initial RED test runs were parallelized and contended on Cargo locks. They still produced the expected RED failures, and all final verification was rerun sequentially.
- `bash scripts/verify.sh` was not run because this wrapper owns only the plan file set plus summary, while the repo aggregate verification can refresh repo-wide generated artifacts. Targeted plan verification, clippy, and build checks were run instead.

## Verification

Passed:

- `cargo fmt --all --manifest-path packages/Cargo.toml`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_summary_projects_consistent_operator_evidence_fields --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc getblockchaininfo_uses_durable_connected_block_height_not_downloaded_height --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status_human_render_preserves_shared_sync_truth_fields --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_projection_preserves_shared_sync_truth_fields --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator --all-features`
- `bash scripts/test-run-live-mainnet-smoke.sh`
- `bash -lc '! rg -n "run-live-mainnet-smoke|--restart-after-progress" scripts/verify.sh'`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node -p open-bitcoin-rpc -p open-bitcoin-cli --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-node -p open-bitcoin-rpc -p open-bitcoin-cli --all-targets --all-features`
- `git diff --check`

Acceptance scans passed for all plan-specified `rg` checks.

## Known Stubs

None. Stub-pattern scan only found existing shell empty-string guards in deterministic fixture helpers.

## Threat Flags

None. Changes are deterministic tests/fixture assertions plus projection formatting; no new network endpoint, auth path, file-access boundary, or schema boundary was introduced.

## User Setup Required

None.

## Next Phase Readiness

OBS-01 is ready for the later Phase 59 support, docs, threat-model, and release-boundary plans. Remaining Phase 59 work should continue treating live public-network evidence as opt-in and deterministic verification as local-only.

## Self-Check: PASSED

- Found summary file at `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-01-SUMMARY.md`.
- Found all five changed plan-owned implementation files.
- Commit self-check intentionally skipped because the wrapper requires no staging, commits, or pushes in this executor; task and metadata commits are deferred to the final strict yolo push gate.

***
*Phase: 59-operator-evidence-threat-model-and-release-boundaries*
*Completed: 2026-06-05*
