---
phase: 69-tip-tracking-and-stay-current-operation
plan: 04
subsystem: node-sync-runtime
tags: [rust, sync-runtime, stay-current, deterministic-tests]

requires:
  - phase: 69-tip-tracking-and-stay-current-operation
    plan: 03
    provides: Bounded stay-current next-action guidance and idle-at-tip stop reason.
  - phase: 68-full-active-chain-validation-and-durable-persistence
    provides: Durable active-chain validation and persistence path.
provides:
  - Post-catch-up stay-current progress regression coverage
  - Headers-only non-current regression coverage
  - Stale-tip next-action regression coverage
  - Runtime reopen coherence regression coverage
affects: [sync-runtime-tests, durable-status, phase-69]

tech-stack:
  added: []
  patterns: [scripted-sync-fixtures, durable-reopen-verification, public-network-free-uat]

key-files:
  modified:
    - packages/open-bitcoin-node/src/sync/tests.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Keep 69-04 implementation test-only because the 69-03 runtime/status path already satisfies the planned semantics."
  - "Use existing scripted transport plus durable store fixtures so post-catch-up tests exercise header validation, block request, block connection, and persisted chainstate state."
  - "Assert headers-only progress as InitialCatchUp, not CurrentAtBestKnownTip, because validated active-chain progress must reach the best-known header tip before current status is true."

patterns-established:
  - "Post-catch-up stay-current coverage should assert both status projection and the stored chainstate active tip."
  - "Stale-tip coverage should use fixed timestamps and the configured freshness threshold; no public-network clock or peer dependency is needed."

requirements-completed: [TIP-02, TIP-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 69-2026-06-11T15-13-14
generated_at: 2026-06-11T18:52:08Z

duration: 27min
completed: 2026-06-11
---

# Phase 69-04: Post-Catch-Up Stay-Current Semantics

**Added deterministic tests proving new work after catch-up, headers-only non-current behavior, stale-tip guidance, and reopen coherence.**

## Performance

- **Duration:** 27 min
- **Completed:** 2026-06-11T18:52:08Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- Added `phase69_post_catch_up_new_headers_connect_and_report_stay_current_progress`.
  - Starts from a connected height-1 active chain.
  - Observes a height-2 header, requests the block, accepts the block, connects it through the existing validation path, and persists the active-chain tip at height 2.
  - Asserts durable status reports `validated_active_chain_height = 2`, best-known tip height/hash/work, fresh evidence, peer agreement, and `CurrentAtBestKnownTip`.
- Added `phase69_headers_only_tip_does_not_report_current`.
  - Proves a header-only height advance leaves validated active-chain height at 1 and reports `InitialCatchUp`, not `CurrentAtBestKnownTip`.
- Added `phase69_stale_tip_is_distinct_from_no_progress`.
  - Uses a fixed timestamp and `tip_freshness_threshold_seconds = 1_200` to assert `StaleTip`, `TipFreshnessStatus::Stale`, and the planned stale-tip next-action text.
- Added `phase69_tip_evidence_survives_runtime_reopen`.
  - Persists current-at-tip status, reopens the same store, and proves best-known tip and stay-current evidence can still be loaded and re-derived.
- Refreshed the tracked LOC report through the repo-managed commit hook.

## Task Commits

1. **Tasks 1-2: Add post-catch-up, headers-only, stale-tip, and reopen tests** - `aeb7140` (test)

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/tests.rs` - Adds four deterministic Phase 69 tests covering post-catch-up progress, headers-only non-current behavior, stale-tip guidance, and reopen semantics.
- `docs/metrics/lines-of-code.md` - Refreshed by repo-managed hooks.

## Decisions Made

No production runtime changes were needed. The existing 69-03 status path already required fresh best-known tip evidence and matching connected active-chain progress before reporting current; 69-04 locked that behavior down with deterministic UAT-style tests.

## Deviations from Plan

None.

## Issues Encountered

None.

## User Setup Required

None - all tests use local durable stores and scripted sync transport. Public-network smoke remains explicitly opt-in and ignored by default.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase69_post_catch_up_new_headers_connect_and_report_stay_current_progress --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase69_ --all-features`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- Commit hook ran `bash scripts/verify.sh` successfully before `aeb7140` and completed in `5m 38.730s`.

## Next Phase Readiness

Plan 69-05 can finalize phase documentation and operator-facing guidance with deterministic proof that current-at-tip status depends on validated active-chain progress, not headers-only progress.

## Self-Check: PASSED

---
*Phase: 69-tip-tracking-and-stay-current-operation*
*Completed: 2026-06-11*
