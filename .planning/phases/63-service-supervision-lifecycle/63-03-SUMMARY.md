---
phase: 63-service-supervision-lifecycle
plan: 03
subsystem: operator-status
tags: [rust, service-lifecycle, status, dashboard, phase62-sync-truth]
requires:
  - phase: 63-service-supervision-lifecycle
    provides: service install/action wiring from plans 63-01 and 63-02
provides:
  - Shared `ServiceLifecycleStatus` contract with exact Phase 63 labels
  - Service manager snapshot projection into richer shared `ServiceStatus`
  - Consistent service lifecycle rendering across direct service status, human status, JSON status, and dashboard rows
affects: [operator-status, operator-dashboard, service-supervision, support-bundle-status]
tech-stack:
  added: []
  patterns:
    - Shared typed lifecycle enum with serde kebab-case labels
    - Pure service snapshot to status-contract projection
    - Explicit `FieldAvailability` reasons for missing service evidence
key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/status/service_status.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/service.rs
    - packages/open-bitcoin-cli/src/operator/service/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Use one shared service lifecycle vocabulary for direct service status, shared status JSON, human status, and dashboard rows."
  - "Treat service manager status inspection errors as typed unavailable-manager state instead of dropping the whole status snapshot."
  - "Keep Phase 62 sync truth fields independent from service lifecycle rendering."
patterns-established:
  - "Service status renderers consume shared `ServiceStatus` fields instead of inventing surface-specific lifecycle names."
  - "Service evidence that is missing or uninspected remains `Unavailable: {reason}` rather than false, zero, or empty."
requirements-completed: [SVC-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 63-2026-06-07T14-20-10
generated_at: 2026-06-07T18:06:12Z
duration: 72min
completed: 2026-06-07
---

# Phase 63 Plan 03: Service Lifecycle Status Contract Summary

**Shared service lifecycle contract with exact Phase 63 labels, explicit unavailable reasons, and consistent status/dashboard rendering**

## Performance

- **Duration:** 72 min
- **Started:** 2026-06-07T16:54:33Z
- **Completed:** 2026-06-07T18:06:12Z
- **Tasks:** 3/3
- **Files modified:** 12

## Accomplishments

- Added `ServiceLifecycleStatus` with exact labels: `unmanaged`, `installed-stopped`, `running`, `failed`, `disabled`, and `unavailable-manager`.
- Extended `ServiceStatus` with `lifecycle`, `service_file_path`, `log_path`, and `diagnostics` while preserving older JSON through explicit serde defaults.
- Projected service manager snapshots and manager errors into the shared status contract without removing Phase 62 sync configured targets, counters, stop reasons, recovery category, or block progress evidence.
- Rendered the same service lifecycle contract in direct `open-bitcoin service status`, human `open-bitcoin status`, JSON status, and dashboard service rows.

## Task Commits

1. **Task 1: Add shared service lifecycle status contract** - `8979c51` (`feat`)
2. **Task 2: Project service manager snapshots into lifecycle status** - `58fbb25` (`feat`)
3. **Task 3: Render lifecycle labels in service status, status, JSON, and dashboard** - `d712c69` (`feat`)

## Files Created/Modified

- `packages/open-bitcoin-node/src/status.rs` - Added `ServiceLifecycleStatus`, `as_str()`, richer `ServiceStatus`, and serde defaults.
- `packages/open-bitcoin-node/src/status/tests.rs` - Covered lifecycle label serialization and legacy JSON defaults.
- `packages/open-bitcoin-node/src/lib.rs` - Re-exported the lifecycle enum.
- `packages/open-bitcoin-cli/src/operator/status/service_status.rs` - Added pure service snapshot projection and manager-error status handling.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Covered lifecycle mapping, manager evidence, unavailable-manager behavior, and Phase 62 sync truth preservation.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Rendered lifecycle, manager, booleans, service file, logs, and diagnostics in stable order.
- `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - Covered human status service lifecycle rendering and Phase 62 sync evidence.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Added ordered service rows for lifecycle, manager, installed, enabled, running, service file, logs, and diagnostics.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Covered dashboard service row order and unavailable-manager rendering.
- `packages/open-bitcoin-cli/src/operator/service.rs` - Rendered direct service status with Phase 63 labels and typed unavailable-manager inspection failures.
- `packages/open-bitcoin-cli/src/operator/service/tests.rs` - Covered direct service status labels and unavailable-manager output.
- `docs/parity/source-breadcrumbs.json` - Added breadcrumbs for new first-party Rust files.
- `docs/metrics/lines-of-code.md` - Refreshed by repo hooks.

## Decisions Made

- Used `ServiceLifecycleStatus::as_str()` as the single label source for renderer-facing lifecycle strings.
- Kept `enabled` unavailable when the manager omits enablement instead of inferring from process state.
- Returned successful direct service status output for status-inspection manager errors, with first line `service: unavailable-manager` and diagnostics containing the manager error.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - AGENTS Pre-Commit Compliance] Kept TDD RED failures uncommitted**
- **Found during:** Tasks 1, 2, and 3
- **Issue:** The plan requested TDD RED commits, but repo rules require all Rust pre-commit checks to pass before every commit.
- **Fix:** Wrote and ran failing RED tests, then committed only after GREEN implementation and required verification passed.
- **Files modified:** Rust test and implementation files listed above.
- **Verification:** Each RED run failed for the expected missing behavior before the GREEN implementation passed focused tests.
- **Committed in:** `8979c51`, `58fbb25`, `d712c69`

**2. [Rule 3 - Blocking] Split status projection and render tests to satisfy repo file-length policy**
- **Found during:** Task 1
- **Issue:** Initial in-place changes pushed production Rust files past the repo-managed file-length hook limit.
- **Fix:** Moved service status projection into `packages/open-bitcoin-cli/src/operator/status/service_status.rs` and render tests into `packages/open-bitcoin-cli/src/operator/status/render/tests.rs`.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/status/service_status.rs`, `packages/open-bitcoin-cli/src/operator/status/render/tests.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `scripts/verify.sh` production file-length check passed in task commit hooks.
- **Committed in:** `8979c51`

**3. [Rule 2 - Missing Critical Functionality] Treated direct service status manager errors as typed unavailable-manager**
- **Found during:** Task 3
- **Issue:** The direct `open-bitcoin service status` path still failed the command on manager status errors, which contradicted the Phase 63 status-inspection contract.
- **Fix:** Rendered `service: unavailable-manager` with explicit unavailable reasons and diagnostics for status inspection errors.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/service.rs`, `packages/open-bitcoin-cli/src/operator/service/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase63_service_lifecycle_rendering --all-features`
- **Committed in:** `d712c69`

**Total deviations:** 3 auto-fixed (2 Rule 2, 1 Rule 3)
**Impact on plan:** The deviations were required for repo compliance and status-contract correctness. No scope beyond Phase 63 service status surfaces was added.

## Issues Encountered

- Task 2 and Task 3 commits ran the full repo hooks successfully, including Cargo, benchmark smoke, Bazel smoke, coverage, parity, and file-length checks.
- The first Task 3 RED run exposed test scaffolding mistakes; those were corrected before the meaningful RED failures were recorded.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase63_service_lifecycle_status_contract --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase63_service_lifecycle_projection --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli collect_status_snapshot_with_error_manager --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase63_service_lifecycle_rendering --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_sections_surface_service_lifecycle --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status_render_includes_sync_progress_and_peer_evidence --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_sections_surface_sync_progress_and_peer_counts --all-features`
- `cargo fmt --manifest-path packages/Cargo.toml --all --check`
- `cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --workspace --all-features`

All verification commands passed. Normal git hooks also ran for each task commit.

## Known Stubs

None. Stub scan found no TODO/FIXME/placeholder/coming soon/not available markers in files changed by this plan.

## Auth Gates

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 63-04 can build on a stable service lifecycle status contract. Status consumers should now use the shared `ServiceStatus` fields and lifecycle enum instead of deriving labels from manager-specific state names.

## Self-Check: PASSED

- Summary file exists: `.planning/phases/63-service-supervision-lifecycle/63-03-SUMMARY.md`
- Task commits found: `8979c51`, `58fbb25`, `d712c69`
- Stub scan completed with no known stubs.

---
*Phase: 63-service-supervision-lifecycle*
*Completed: 2026-06-07*
