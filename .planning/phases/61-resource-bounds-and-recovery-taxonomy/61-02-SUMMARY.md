---
phase: 61-resource-bounds-and-recovery-taxonomy
plan: 02
subsystem: node-sync-recovery
tags: [rust, sync, storage, recovery-taxonomy, parity-breadcrumbs]

requires:
  - phase: 61-resource-bounds-and-recovery-taxonomy/61-01
    provides: shared SyncRecoveryCategory status contract
  - phase: 60-unattended-sync-loop-control
    provides: sync stop reasons and durable runtime state facts
provides:
  - storage action and StorageError recovery category mapping
  - sync peer, stop, runtime, and detail recovery category helpers
  - sync summary and runtime-state projection of typed recovery categories
  - parity breadcrumb coverage for the new sync recovery child module
affects: [phase-61, sync-runtime, storage, operator-status, structured-logs, parity]

tech-stack:
  added: []
  patterns:
    - boundary-aware lock-contention classification
    - storage-first recovery category precedence for sync status and runtime state

key-files:
  created:
    - packages/open-bitcoin-node/src/sync/types/recovery.rs
    - .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-02-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/storage.rs
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Storage action and StorageError categories take precedence over peer and network retry guidance."
  - "Lock-contention classification uses word-boundary matching so unrelated words such as block do not match lock."
  - "The new sync recovery Rust module was committed with its parity breadcrumb because AGENTS.md requires breadcrumbs for first-party Rust source files."

patterns-established:
  - "Pure recovery mapping helpers live beside sync types, while durable runtime projection consumes typed categories instead of renderer-local strings."
  - "Storage category helpers are reusable by runtime errors, summary status, and durable recovery-action metadata."

requirements-completed: [RR-02, RR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 61-2026-06-06T03-43-41
generated_at: 2026-06-06T05:32:04Z

duration: 23min
completed: 2026-06-06
---

# Phase 61 Plan 02: Recovery Taxonomy Mapping Summary

**Storage, peer, stop, runtime, and detail facts now map into shared typed recovery categories with storage-first precedence.**

## Performance

- **Duration:** 23 min
- **Started:** 2026-06-06T05:09:23Z
- **Completed:** 2026-06-06T05:32:04Z
- **Tasks:** 3
- **Files modified:** 7

## Accomplishments

- Added `StorageRecoveryAction::recovery_category` and `StorageError::recovery_category` so schema, corruption, lock contention, and backend failures project typed `SyncRecoveryCategory` values.
- Added `sync/types/recovery.rs` with pure mappings for peer failures, operator stop reasons, runtime errors, and text detail facts.
- Wired typed recovery categories through sync summaries, durable runtime state, and runtime error logs so renderers do not need ad hoc string classification.
- Registered the new sync recovery module in `docs/parity/source-breadcrumbs.json`.

## Task Commits

Each task was committed atomically where feasible under the repo hooks:

1. **Task 1: Map storage actions and errors into recovery categories** - `b4d4872` (`feat`)
2. **Task 2: Map sync peer, stop, runtime, and detail facts into recovery categories** - `a9c8678` (`feat`)
3. **Task 3: Register parity breadcrumbs for the sync recovery mapping module** - `a9c8678` (`feat`, combined with Task 2 because the new Rust module required its breadcrumb before hooks could pass)

Task 1 and Task 2 followed TDD RED/GREEN locally. The failing RED states were not committed because normal hooks would reject intentionally failing tests.

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage.rs` - Adds typed storage recovery category helpers and regression tests for schema, corruption, lock contention, backend, and interrupted-write failures.
- `packages/open-bitcoin-node/src/sync/types.rs` - Registers the `recovery` child module.
- `packages/open-bitcoin-node/src/sync/types/recovery.rs` - Implements pure sync recovery category mapping helpers with parity breadcrumbs and focused tests.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Projects typed recovery categories into sync status using storage-first precedence.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Persists recovery category precedence through durable runtime state and runtime error logs.
- `docs/parity/source-breadcrumbs.json` - Adds breadcrumb coverage for the new sync recovery module.
- `docs/metrics/lines-of-code.md` - Hook-regenerated tracked LOC artifact.

## Decisions Made

- Storage incompatibility, corruption, lock, and backend categories are authoritative when a storage action or storage runtime error is present.
- Lock classification is boundary-aware, because substring matching misclassifies words like `block` as lock contention.
- Runtime projection now consumes typed mapping helpers directly so operator-facing status and logs share one taxonomy source.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Combined Task 2 and Task 3 for parity breadcrumb compliance**
- **Found during:** Task 2 (sync recovery module creation)
- **Issue:** AGENTS.md requires new first-party Rust source files to be represented in `docs/parity/source-breadcrumbs.json` before commit.
- **Fix:** Added the recovery module breadcrumb in the same commit that created the module.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check`
- **Committed in:** `a9c8678`

**2. [Rule 1 - Bug] Avoided false lock-contention matches inside unrelated words**
- **Found during:** Task 2 (focused sync recovery tests)
- **Issue:** A naive `contains("lock")` check would classify details such as `block flush failed` as `StorageLockContention`.
- **Fix:** Added ASCII word-boundary helpers and regression coverage in storage and sync recovery mapping tests.
- **Files modified:** `packages/open-bitcoin-node/src/storage.rs`, `packages/open-bitcoin-node/src/sync/types/recovery.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage_recovery_category --all-features`; `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_recovery_category --all-features`
- **Committed in:** `a9c8678`

**3. [Rule 3 - Blocking] Wired recovery helpers into production projections**
- **Found during:** Task 2 (pre-commit clippy)
- **Issue:** `cargo clippy --all-targets --all-features -- -D warnings` rejected helpers that were only referenced by tests.
- **Fix:** Projected `SyncRunSummary::recovery_category()` into sync status and durable runtime state, and included typed categories in runtime error logs.
- **Files modified:** `packages/open-bitcoin-node/src/sync/types/summary.rs`, `packages/open-bitcoin-node/src/sync/runtime_state.rs`
- **Verification:** `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`; `cargo test --manifest-path packages/Cargo.toml --all-features`
- **Committed in:** `a9c8678`

**4. [Rule 3 - Blocking] Applied rustfmt after hook feedback**
- **Found during:** Task 2/3 commit attempt
- **Issue:** The normal commit hook found a rustfmt diff in `sync/types/summary.rs`.
- **Fix:** Ran `cargo fmt --manifest-path packages/Cargo.toml --all`, restaged the task files, and retried the normal hooked commit.
- **Files modified:** `packages/open-bitcoin-node/src/sync/types/summary.rs`
- **Verification:** Normal commit hooks completed `bash scripts/verify.sh` successfully.
- **Committed in:** `a9c8678`

***

**Total deviations:** 4 auto-fixed (1 Rule 1, 1 Rule 2, 2 Rule 3)
**Impact on plan:** All deviations were required for correctness or repo policy compliance. No scope beyond the recovery taxonomy mapping was added.

## Issues Encountered

- TDD RED commits were not created because the user required normal hooks and the repo hooks reject intentionally failing tests.
- The Task 2/3 commit initially stopped on formatting; rustfmt resolved the issue and the normal hooks then passed.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage_recovery_category --all-features` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_recovery_category --all-features` - passed
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed
- `git diff --check` - passed
- `cargo fmt --manifest-path packages/Cargo.toml --all` - passed before commits
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` - passed before commits
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` - passed before commits
- `cargo test --manifest-path packages/Cargo.toml --all-features` - passed before commits
- Normal commit hooks completed `bash scripts/verify.sh` successfully for both task commits.

## Stub Scan

No stubs found. The required scan found only Rust format strings such as `recovery_category={}` and `messages_processed={}`, not placeholder data or unwired mock values.

## Threat Flags

None. The changed files only map and project existing storage, peer, stop, runtime, and detail facts already covered by the plan threat model.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

The shared recovery taxonomy now has typed input mappings and durable/runtime projection points ready for follow-up operator guidance and recovery-surface work. Remaining plans can consume `SyncRecoveryCategory` instead of deriving recovery labels from renderer-local strings.

***
*Phase: 61-resource-bounds-and-recovery-taxonomy*
*Completed: 2026-06-06*

## Self-Check: PASSED

- `packages/open-bitcoin-node/src/sync/types/recovery.rs` exists.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-02-SUMMARY.md` exists.
- Commit `b4d4872` is reachable.
- Commit `a9c8678` is reachable.
