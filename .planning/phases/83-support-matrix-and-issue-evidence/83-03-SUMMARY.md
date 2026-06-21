---
phase: 83-support-matrix-and-issue-evidence
plan: 03
subsystem: parity-docs
tags: [support-matrix, operator-docs, parity-catalogs, issue-evidence]

requires:
  - phase: 83-01
    provides: Canonical support matrix and issue-evidence policy.
  - phase: 83-02
    provides: Parity root links and release-review pointers for the support matrix.
  - phase: 82-production-claim-boundary
    provides: Phase 82 support vocabulary and deferred production-adjacent boundary.
provides:
  - README and runtime-guide entrypoints to the canonical support matrix.
  - Operator issue-evidence pointer preserving local Cargo and Bazel support-bundle forms.
  - Compact catalog pointers for operator-runtime, P2P, chainstate, wallet, and migration boundaries.
affects: [phase-83, parity-docs, operator-docs, support-policy]

tech-stack:
  added: []
  patterns:
    - Link to docs/parity/support-matrix.md from entrypoints instead of duplicating the support matrix.
    - Keep catalog support-policy additions as compact pointers to the canonical matrix.

key-files:
  created:
    - .planning/phases/83-support-matrix-and-issue-evidence/83-03-SUMMARY.md
  modified:
    - README.md
    - docs/operator/runtime-guide.md
    - docs/parity/catalog/operator-runtime-release-hardening.md
    - docs/parity/catalog/p2p.md
    - docs/parity/catalog/chainstate.md
    - docs/parity/catalog/wallet.md
    - docs/parity/catalog/drop-in-audit-and-migration.md

key-decisions:
  - "Kept docs/parity/support-matrix.md as the only full support matrix source of truth."
  - "Preserved Phase 82 support terms and deferred-surface boundaries in README, runtime guide, and catalog prose."
  - "Followed the strict wrapper git gate: no commits, staging, pushes, STATE.md updates, or ROADMAP.md updates."

patterns-established:
  - "README/runtime-guide support pointers should name the support matrix without copying matrix rows."
  - "Catalog pages should route support classification to the canonical matrix and restate only local deferred-surface boundaries."

requirements-completed: [SUP-01, SUP-02, SUP-03, SUP-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 83-2026-06-21T16-02-39
generated_at: 2026-06-21T17:07:08Z

duration: 4 min
completed: 2026-06-21
---

# Phase 83 Plan 03: Support Matrix Entry Points Summary

**Support matrix linked from README, runtime guide, and focused parity catalogs without duplicating the canonical table**

## Performance

- **Duration:** 4 min
- **Started:** 2026-06-21T17:03:21Z
- **Completed:** 2026-06-21T17:07:08Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments

- Added top-level README pointers to `docs/parity/support-matrix.md` beside the production-claim boundary and parity entrypoints.
- Updated `docs/operator/runtime-guide.md` to point release reviewers and issue reporters to the support matrix while preserving repo-local Cargo and Bazel support-bundle commands.
- Added compact Phase 83 support-matrix pointers to the operator-runtime, P2P, chainstate, wallet, and migration catalogs without copying the canonical matrix.

## Task Commits

Task commits were not created because this wrapper run has a strict git gate: no commits, no staging, and no pushes until after phase verification passes.

1. **Task 1: Update README and runtime guide support pointers** - deferred by strict wrapper git gate
2. **Task 2: Add compact support-matrix pointers to relevant catalogs** - deferred by strict wrapper git gate

**Plan metadata:** deferred by strict wrapper git gate

## Files Created/Modified

- `README.md` - Added support-matrix pointers in the status/parity and operator-preview entrypoints.
- `docs/operator/runtime-guide.md` - Added support-matrix and issue-evidence pointers near release-boundary review and support-bundle collection.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - Added the Phase 83 catalog row for `v1-8-support-matrix-issue-evidence` and SUP-01 through SUP-04.
- `docs/parity/catalog/p2p.md` - Linked P2P deferred relay/inbound boundaries to the support matrix.
- `docs/parity/catalog/chainstate.md` - Linked scoped chainstate evidence limits to the support matrix.
- `docs/parity/catalog/wallet.md` - Linked production-funds wallet deferrals to the support matrix.
- `docs/parity/catalog/drop-in-audit-and-migration.md` - Linked dry-run migration scope and migration-apply deferrals to the support matrix.
- `.planning/phases/83-support-matrix-and-issue-evidence/83-03-SUMMARY.md` - Execution summary.

## Decisions Made

- Followed `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `standards/core/code-shape.md`, and `standards/core/verification.md` for this doc-only execution path.
- Kept every entrypoint as a pointer to `docs/parity/support-matrix.md` rather than creating alternate support registries.
- Preserved public-network, real service-manager, multi-day, and release-blocking checks as opt-in unless future scoped gates change the contract.
- Left `.planning/STATE.md` and `.planning/ROADMAP.md` untouched because the orchestrator owns shared GSD state updates for this wrapper run.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None. Existing uncommitted Phase 83/shared-state artifacts were observed and left untouched.

## Known Stubs

None introduced. The stub-pattern scan matched only pre-existing catalog prose saying prior service/status/dashboard docs were not placeholders; no TODO, FIXME, placeholder implementation, mock data, or hardcoded empty UI path was added by this plan.

## Threat Flags

None. The changes are documentation-only and stay within the plan threat model for operator entrypoints, issue-evidence disclosure boundaries, catalog drift prevention, deferred-surface preservation, and default verification wording.

## Verification

Passed the focused Plan 83-03 verification:

- README `rg` checks for `docs/parity/support-matrix.md`, the required environment-family sentence, and the production full-node readiness non-claim.
- Runtime-guide `rg` checks for `../parity/support-matrix.md`, smallest useful redacted evidence guidance, `Unavailable: <reason>`, repo-local Cargo and Bazel support-bundle command forms, and opt-in public-network/service-manager/multi-day/release-blocking wording.
- Catalog `rg` checks for the operator-runtime Phase 83 row, `v1-8-support-matrix-issue-evidence`, SUP-01 through SUP-04, and targeted P2P, chainstate, wallet, and migration support-boundary phrases.
- Negative `rg` checks confirming README, runtime guide, and catalog files do not duplicate the full support-matrix header.
- `git diff --check -- README.md docs/operator/runtime-guide.md docs/parity/catalog/operator-runtime-release-hardening.md docs/parity/catalog/p2p.md docs/parity/catalog/chainstate.md docs/parity/catalog/wallet.md docs/parity/catalog/drop-in-audit-and-migration.md`.

Full `bash scripts/verify.sh` was not run in this wrapper plan because Plan 83-03 scopes verification to focused `rg` checks, while Plan 83-04 owns the deterministic checker and final Phase 83 verification path.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 83-04. Normal README, runtime-guide, and relevant catalog entrypoints now route readers to the canonical support matrix and issue-evidence checklist while preserving Phase 82 support terms and deferred production-adjacent surfaces.

## Self-Check: PASSED

- Found `.planning/phases/83-support-matrix-and-issue-evidence/83-03-SUMMARY.md`.
- Found all README, runtime-guide, and catalog files modified by this plan.
- Confirmed no files are staged.
- Confirmed the summary records task commits as deferred by the strict wrapper git gate.
- Confirmed `.planning/STATE.md` and `.planning/ROADMAP.md` were not updated by this plan.

---
*Phase: 83-support-matrix-and-issue-evidence*
*Completed: 2026-06-21*
