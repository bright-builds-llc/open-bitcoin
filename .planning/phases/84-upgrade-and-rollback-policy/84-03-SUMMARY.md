---
phase: 84-upgrade-and-rollback-policy
plan: 03
subsystem: operator-docs
tags: [upgrade-policy, rollback, operator-entrypoints, parity-catalogs]

requires:
  - phase: 84-01
    provides: Canonical upgrade-and-rollback-policy.md policy text.
  - phase: 84-02
    provides: Parity roots and release-boundary pointers for the upgrade policy.
  - phase: 82-production-claim-boundary
    provides: Production claim boundary and deferred-surface vocabulary.
  - phase: 83-support-matrix-and-issue-evidence
    provides: Support matrix and issue-evidence boundaries.
provides:
  - README and runtime-guide pointers to the canonical upgrade policy.
  - Catalog pointers preserving upgrade, rollback, wallet, migration, chainstate, and destructive-repair boundaries.
  - Focused verification evidence for Phase 84 operator entrypoint links.
affects: [operator-runbooks, service-expectations, release-readiness, deterministic-claim-guardrails]

tech-stack:
  added: []
  patterns:
    - Compact entrypoint and catalog links to a canonical docs/parity policy.
    - Deferred mutation boundaries preserved in local catalog prose.

key-files:
  created:
    - .planning/phases/84-upgrade-and-rollback-policy/84-03-SUMMARY.md
  modified:
    - README.md
    - docs/operator/runtime-guide.md
    - docs/parity/catalog/operator-runtime-release-hardening.md
    - docs/parity/catalog/drop-in-audit-and-migration.md
    - docs/parity/catalog/wallet.md
    - docs/parity/catalog/chainstate.md

key-decisions:
  - "Keep README and runtime-guide pointers compact, with the canonical policy as the source of truth."
  - "Keep catalog updates as boundary notes rather than duplicated policy tables."
  - "Skip STATE.md and ROADMAP.md updates because the orchestrator owns shared planning-state writes after execution waves."

patterns-established:
  - "Operator entrypoints link to upgrade-and-rollback-policy.md before source-built command workflows."
  - "Catalog pages name deferred mutation surfaces where rollback guidance could otherwise be overread."

requirements-completed: [UPG-01, UPG-02, UPG-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 84-2026-06-21T21-33-46
generated_at: 2026-06-22T00:40:39Z

duration: 4 min
completed: 2026-06-22
---

# Phase 84 Plan 03: Operator Entrypoint And Catalog Pointers Summary

**Canonical upgrade policy links from README, runtime guide, and parity catalogs with mutation and rollback boundaries intact**

## Performance

- **Duration:** 4 min
- **Started:** 2026-06-22T00:36:51Z
- **Completed:** 2026-06-22T00:40:39Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Added `./docs/parity/upgrade-and-rollback-policy.md` to README parity and operator-preview entrypoints.
- Added `../parity/upgrade-and-rollback-policy.md` to the runtime guide opening boundary block and release-boundary review area.
- Added the Phase 84 upgrade-policy catalog row with `v1-8-upgrade-rollback-policy` and UPG-01 through UPG-04 references.
- Added compact migration, wallet, and chainstate catalog pointers preserving deferred mutation, wallet, and destructive-repair boundaries.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add README and runtime-guide policy pointers** - `e0536d8` (docs)
2. **Task 2: Add compact relevant catalog pointers** - `b2d0a4d` (docs)

**Plan metadata:** committed separately after summary self-check.

## Files Created/Modified

- `README.md` - Top-level operator and parity pointers to the upgrade policy.
- `docs/operator/runtime-guide.md` - Practical source-built upgrade-policy pointer while preserving repo-local Cargo and Bazel command forms.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - Phase 84 audit row and deferred release-channel/repair boundaries.
- `docs/parity/catalog/drop-in-audit-and-migration.md` - Migration pointer preserving deferred apply, source service mutation, source datadir rewrite, and external wallet rewrite.
- `docs/parity/catalog/wallet.md` - Wallet pointer preserving production-funds, external rewrite, raw copy, and hidden mutation boundaries.
- `docs/parity/catalog/chainstate.md` - Chainstate pointer requiring field-level schema/storage evidence and no destructive repair implication.
- `.planning/phases/84-upgrade-and-rollback-policy/84-03-SUMMARY.md` - Execution summary.

## Decisions Made

- Followed `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant Bright Builds standards for docs-only execution.
- Used compact links and local boundary notes instead of duplicating the canonical upgrade-policy checklist or decision tables.
- Used `--no-verify` task commits per the wave executor instruction; focused acceptance checks passed and full repo verification remains with the orchestrator.

## Deviations from Plan

None - plan executed exactly as written.

## Known Stubs

None. Stub scanning found one pre-existing use of "placeholders" in `docs/parity/catalog/operator-runtime-release-hardening.md` describing docs that are not placeholders; it is not a stub.

## Threat Flags

None. This plan changed documentation links and catalog boundary prose only; it introduced no new network endpoint, auth path, file access behavior, or schema trust boundary.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Verification

Passed focused Task 1 verification:

- `rg` checks for `./docs/parity/upgrade-and-rollback-policy.md`, `source-built upgrade, rollback, backup, and compatibility decisions`, and `it does not claim production full-node readiness` in `README.md`.
- `rg` checks for `../parity/upgrade-and-rollback-policy.md`, `pre-upgrade checklist`, `failed-upgrade guidance`, `rollback guidance`, `no hidden source datadir, wallet, service, or config mutation`, and existing repo-local Cargo and Bazel command forms in `docs/operator/runtime-guide.md`.
- Negative `rg` check confirmed README and runtime guide did not copy the policy table header.
- `git diff --check -- README.md docs/operator/runtime-guide.md`.

Passed focused Task 2 verification:

- `rg` checks for `Phase 84 upgrade and rollback policy`, `v1-8-upgrade-rollback-policy`, UPG-01 through UPG-04, `upgrade-and-rollback-policy.md`, release-channel deferrals, and destructive-repair boundary in `docs/parity/catalog/operator-runtime-release-hardening.md`.
- `rg` checks for migration, wallet, and chainstate catalog policy links plus required deferred-boundary phrases.
- Negative `rg` check confirmed the four catalog files did not copy the policy table header.
- `git diff --check -- docs/parity/catalog/operator-runtime-release-hardening.md docs/parity/catalog/drop-in-audit-and-migration.md docs/parity/catalog/wallet.md docs/parity/catalog/chainstate.md`.

Passed plan-level verification:

- `rg -n "upgrade-and-rollback-policy.md" README.md docs/operator/runtime-guide.md docs/parity/catalog/operator-runtime-release-hardening.md docs/parity/catalog/drop-in-audit-and-migration.md docs/parity/catalog/wallet.md docs/parity/catalog/chainstate.md`
- `git diff --check HEAD~2..HEAD -- README.md docs/operator/runtime-guide.md docs/parity/catalog/operator-runtime-release-hardening.md docs/parity/catalog/drop-in-audit-and-migration.md docs/parity/catalog/wallet.md docs/parity/catalog/chainstate.md`
- `git diff --check -- README.md docs/operator/runtime-guide.md docs/parity/catalog/operator-runtime-release-hardening.md docs/parity/catalog/drop-in-audit-and-migration.md docs/parity/catalog/wallet.md docs/parity/catalog/chainstate.md`

Full `bash scripts/verify.sh` was not run for this plan because the orchestrator will run full repo verification after execution waves.

## Next Phase Readiness

Ready for 84-04 to add deterministic checker coverage and verifier wiring for the Phase 84 policy surface.

## Self-Check: PASSED

- Found `.planning/phases/84-upgrade-and-rollback-policy/84-03-SUMMARY.md`.
- Found task commits `e0536d8` and `b2d0a4d`.
- `git diff --check -- .planning/phases/84-upgrade-and-rollback-policy/84-03-SUMMARY.md` passed.
- `git status --short` showed only the intended new summary artifact before the metadata commit.

---
*Phase: 84-upgrade-and-rollback-policy*
*Completed: 2026-06-22*
