---
phase: 61-resource-bounds-and-recovery-taxonomy
plan: 03
subsystem: node-sync-runtime
tags: [rust, sync, storage, structured-logs, recovery-taxonomy, resource-bounds]

# Dependency graph
requires:
  - phase: 61-resource-bounds-and-recovery-taxonomy/61-02
    provides: shared SyncRecoveryCategory mapping helpers and storage-first taxonomy inputs
  - phase: 60-unattended-sync-loop-control
    provides: sync stop reasons, durable runtime state, and bounded sync loop contracts
provides:
  - summary-derived sync status recovery category projection
  - structured sync progress logs with recovery_category labels
  - durable runtime recovery category projection with storage-first precedence
  - deterministic repeated-cycle RR-01 resource pressure and retention regression
affects: [phase-61, sync-runtime, operator-status, structured-logs, support-bundle, phase-62-truth-surfaces]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - summary status derives recovery category from stop reason before latest peer category
    - durable sync state applies storage metadata before last-error, stop-reason, peer, and shutdown categories
    - deterministic sync fixtures prove resource bounds without public-network paths

key-files:
  created:
    - .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-03-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/types/recovery.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Summary-derived status uses stop reason before latest peer recovery category; broad last-error parsing stays in durable runtime state."
  - "Durable runtime recovery category precedence is storage metadata, last-error detail, stop reason, latest peer category, then clean or unclean shutdown metadata."
  - "Structured sync progress logs carry recovery_category while preserving bounded message length through a 192-character summary-record cap."
  - "The sync recovery helper module is visible to the parent sync runtime so durable status uses the shared classifier instead of duplicating string logic."

patterns-established:
  - "Status, durable state, and structured logs project the same shared recovery category labels from typed sync/runtime facts."
  - "Repeated-cycle sync tests assert configured resource pressure, endpoint-keyed retry state, synchronous durable writes, and retention defaults together."

requirements-completed: [RR-01, RR-02, RR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 61-2026-06-06T03-43-41
generated_at: 2026-06-06T15:33:19Z

# Metrics
duration: 33m 34s
completed: 2026-06-06
---

# Phase 61 Plan 03: Runtime Projection and Resource Bounds Summary

**Durable sync state, summary status, and structured logs now project shared recovery categories, with a deterministic repeated-cycle test proving RR-01 resource bounds.**

## Performance

- **Duration:** 33m 34s
- **Started:** 2026-06-06T14:59:45Z
- **Completed:** 2026-06-06T15:33:19Z
- **Tasks:** 3
- **Files modified:** 6 implementation/generated files, plus this summary

## Accomplishments

- Added summary/status helpers so sync status exposes a shared recovery category from operator stop reasons or the latest peer failure category.
- Added `recovery_category=<label>` to structured sync progress records, including `recovery_category=unavailable` when no category exists.
- Applied durable runtime recovery precedence in storage-first order: storage metadata, last-error detail, stop reason, peer category, then shutdown metadata.
- Added deterministic repeated-cycle sync coverage for resource pressure caps, endpoint-keyed retry backoff, synchronous durable metadata writes, and metric/log retention defaults.

## Task Commits

Each completed task was committed atomically with normal hooks:

1. **Task 1: Project recovery categories in summaries, status, and structured logs** - `e3c0483` (`feat`)
2. **Task 2: Apply storage-first recovery category precedence in durable runtime state** - `a330ebb` (`feat`)
3. **Task 3: Prove resource pressure and retention stay bounded in deterministic repeated cycles** - `bb1ba81` (`test`)

TDD RED failure evidence was captured before GREEN changes, but failing RED commits were not created because this run required normal hooks and no `--no-verify`.

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Adds latest recovery category projection and structured log recovery category fields.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Persists durable sync recovery category using storage-first precedence.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Adds summary projection, durable precedence, and repeated-cycle RR-01 resource-bound tests.
- `packages/open-bitcoin-node/src/sync/types.rs` - Exposes the sync recovery helper module to the parent sync runtime.
- `packages/open-bitcoin-node/src/sync/types/recovery.rs` - Keeps the shared error-detail classifier production-visible for durable runtime projection.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC report through normal verification hooks.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-03-SUMMARY.md` - Records plan completion and verification.

## Decisions Made

- Summary-derived status does not parse broad last-error text; it uses stop reasons first, then the latest peer outcome category, leaving text-detail fallback to durable runtime state.
- Durable runtime state treats storage recovery metadata as authoritative before last-error, stop-reason, peer, and shutdown-derived categories.
- The structured progress record remains bounded by shortening the message prefix and using a 192-character cap for summary records that now include recovery category evidence.
- The existing shared recovery classifier is reused by durable runtime state through `pub(super) mod recovery` instead of duplicating string classification in `runtime_state.rs`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Temporarily scoped detail parsing to tests during Task 1**
- **Found during:** Task 1 (summary recovery category projection)
- **Issue:** Removing broad last-error parsing from summary-derived status made `recovery_category_from_error_detail` production-unused until Task 2 consumed it from durable runtime state, and clippy rejects dead code.
- **Fix:** Temporarily gated the helper to tests in Task 1, then made it production-visible again in Task 2 when durable runtime state used the shared classifier.
- **Files modified:** `packages/open-bitcoin-node/src/sync/types/recovery.rs`, `packages/open-bitcoin-node/src/sync/runtime_state.rs`
- **Verification:** `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`; `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node recovery_category --all-features`
- **Committed in:** `e3c0483`, `a330ebb`

**2. [Rule 1 - Bug] Preserved bounded structured log messages after adding recovery_category**
- **Found during:** Task 1 (full test suite)
- **Issue:** Adding `recovery_category` to summary progress records exceeded the existing 160-character assertion for structured progress messages.
- **Fix:** Removed the redundant `sync progress` prefix and raised the summary-record cap to 192 while preserving field coverage and boundedness.
- **Files modified:** `packages/open-bitcoin-node/src/sync/types/summary.rs`, `packages/open-bitcoin-node/src/sync/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node recovery_category --all-features`; `cargo test --manifest-path packages/Cargo.toml --all-features`
- **Committed in:** `e3c0483`

**3. [Rule 3 - Blocking] Made the shared recovery classifier visible to durable runtime state**
- **Found during:** Task 2 (storage-first durable projection)
- **Issue:** `runtime_state.rs` needed the existing error-detail classifier but the recovery module was private to `sync::types`.
- **Fix:** Changed the module visibility to `pub(super)` and imported `types::recovery::recovery_category_from_error_detail`.
- **Files modified:** `packages/open-bitcoin-node/src/sync/types.rs`, `packages/open-bitcoin-node/src/sync/runtime_state.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node durable_sync_state_ --all-features`; `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- **Committed in:** `a330ebb`

**4. [Rule 3 - Blocking] Regenerated tracked LOC report required by repo verification**
- **Found during:** All task commits
- **Issue:** Normal hooks refresh the tracked `docs/metrics/lines-of-code.md` artifact after Rust source/test changes.
- **Fix:** Included the regenerated LOC report in the relevant task commits.
- **Files modified:** `docs/metrics/lines-of-code.md`
- **Verification:** Normal commit hooks ran `bash scripts/verify.sh` and passed for all task commits.
- **Committed in:** `e3c0483`, `a330ebb`, `bb1ba81`

---

**Total deviations:** 4 auto-fixed (1 Rule 1, 3 Rule 3)
**Impact on plan:** All deviations were required for correctness or repo verification. No scope beyond runtime recovery projection, structured log evidence, and deterministic resource-bound tests was added.

## Issues Encountered

- TDD RED states were run locally and failed as expected, but were not committed because normal hooks reject intentionally failing tests.
- No authentication gates, manual setup blockers, or architectural decisions occurred.

## Verification

Focused task checks:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_summary_projects_consistent_operator_evidence_fields --all-features` - failed in RED, then passed after Task 1.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node recovery_category --all-features` - passed after Task 1 and in final verification.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node durable_sync_state_projects_storage_lock_category_from_last_error --all-features` - failed in RED, then passed after Task 2.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node durable_sync_state_ --all-features` - passed after Task 2.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node bounded_unattended_cycles_preserve_resource_pressure_and_retention --all-features` - failed in RED, then passed after Task 3 and in final verification.
- `rg -n "bounded_unattended_cycles_preserve_resource_pressure_and_retention|peer retry state is keyed by resolved endpoint|durable storage writes are synchronous adapter calls with no queued write backlog|MetricRetentionPolicy::default|LogRetentionPolicy::default|max_blocks_in_flight_total: 4" packages/open-bitcoin-node/src/sync/tests.rs` - passed.
- `if rg -n "run-live-mainnet-smoke|--manual-peer|--restart-after-progress" scripts/verify.sh; then exit 1; fi` - passed.

Repo verification:

- `cargo fmt --manifest-path packages/Cargo.toml --all` - passed before each task commit.
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` - passed before each task commit.
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` - passed before each task commit.
- `cargo test --manifest-path packages/Cargo.toml --all-features` - passed before each task commit.
- `bash scripts/verify.sh` - passed in normal hooks for all three task commits and passed again as final verification.

## Stub Scan

No blocking stubs found. The scan found no `TODO`, `FIXME`, `placeholder`, `coming soon`, `not available`, or hardcoded empty-value patterns that flow to UI rendering in the files changed by this plan. Existing unavailable-field reasons are intentional status availability contracts, not stubs.

## Threat Flags

None. The changes use existing sync status, durable metadata, and structured log surfaces; no new network endpoints, auth paths, file access patterns, schema changes, or trust-boundary surfaces were introduced.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 61 now has runtime-level recovery category projection plus deterministic RR-01 evidence for resource bounds and retention. Plan 61-06 can document the taxonomy and validation matrix with status/log/durable-state behavior already backed by focused tests.

---
*Phase: 61-resource-bounds-and-recovery-taxonomy*
*Completed: 2026-06-06*

## Self-Check: PASSED

- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-03-SUMMARY.md` exists.
- Task commit `e3c0483` is reachable.
- Task commit `a330ebb` is reachable.
- Task commit `bb1ba81` is reachable.
- `git diff --check` passed for the planning summary diff.
