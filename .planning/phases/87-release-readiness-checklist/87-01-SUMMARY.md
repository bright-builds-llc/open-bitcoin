---
phase: 87-release-readiness-checklist
plan: 01
subsystem: docs
tags:
  - release-readiness
  - parity
  - verification
  - bun
requires:
  - phase: 82-production-claim-boundary
    provides: Production-readiness vocabulary, evidence gates, deferred surfaces, and no-claim boundary.
  - phase: 83-support-matrix-and-issue-evidence
    provides: Support matrix, issue-evidence posture, and residual-risk vocabulary.
  - phase: 84-upgrade-and-rollback-policy
    provides: Source-built upgrade, rollback, backup, and hidden-mutation boundaries.
  - phase: 85-operator-runbooks
    provides: Operator preflight, long-run, recovery, support-bundle, and escalation evidence guidance.
  - phase: 86-service-operation-expectations
    provides: Service operation expectation docs and adjacent verifier wiring pattern.
provides:
  - v1.8 release-readiness checklist covering all current v1.8 requirement ids.
  - Explicit no-claim review for production full-node readiness and deferred production-adjacent surfaces.
  - Parity root and compact entrypoint links for the release-readiness checklist.
  - Deterministic Phase 87 Bun checker and fixture tests wired into repo-native verification.
affects:
  - docs/parity/release-readiness.md
  - docs/parity/index.json
  - docs/parity/checklist.md
  - docs/parity/README.md
  - README.md
  - scripts/verify.sh
  - Phase 88 deterministic claim guardrail planning
tech-stack:
  added: []
  patterns:
    - Narrow Bun/TypeScript release-boundary checker with fixture tests.
    - Compact parity entrypoint links backed by deterministic checker coverage.
key-files:
  created:
    - scripts/check-phase87-release-readiness.ts
    - scripts/check-phase87-release-readiness.test.ts
    - .planning/phases/87-release-readiness-checklist/87-01-SUMMARY.md
  modified:
    - docs/parity/release-readiness.md
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/README.md
    - README.md
    - docs/parity/production-claim-boundary.md
    - docs/parity/support-matrix.md
    - docs/parity/upgrade-and-rollback-policy.md
    - docs/parity/operator-runbooks.md
    - docs/parity/service-operation-expectations.md
    - docs/parity/deviations-and-unknowns.md
    - docs/parity/catalog/operator-runtime-release-hardening.md
    - docs/metrics/lines-of-code.md
    - MODULE.bazel.lock
    - scripts/verify.sh
key-decisions:
  - "Phase 87 extends `docs/parity/release-readiness.md` as the canonical release-review checklist instead of creating a parallel checklist document."
  - "The checklist maps all current v1.8 requirement ids to canonical evidence, default verification, UAT/manual posture, residual risk, and no-claim or next-gate status."
  - "Default verification remains deterministic and excludes public-network, real service-manager, multi-day, package-manager service, Windows service, and automatic support-upload checks."
  - "Phase 88 retains ownership of broad all-doc deterministic claim guardrails for REL-02, REL-03, and REL-04."
patterns-established:
  - "Release-readiness rows require field-based evidence and named verification roots rather than artifact existence by itself."
  - "Parity entrypoints link to the canonical checklist compactly while avoiding duplicate release matrices."
requirements-completed:
  - PROD-01
  - PROD-02
  - PROD-03
  - PROD-04
  - SUP-01
  - SUP-02
  - SUP-03
  - SUP-04
  - UPG-01
  - UPG-02
  - UPG-03
  - UPG-04
  - RUN-01
  - RUN-02
  - RUN-03
  - SVC-01
  - SVC-02
  - REL-01
  - REL-05
  - REL-06
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 87-2026-06-23T01-49-01
generated_at: 2026-06-23T04:43:38Z
duration: 43min
completed: 2026-06-23
---

# Phase 87 Plan 01: Release Readiness Checklist Summary

**v1.8 release-readiness checklist, parity entrypoint links, and a deterministic Bun checker that preserve the production full-node no-claim boundary.**

## Performance

- **Duration:** 43 min
- **Started:** 2026-06-23T04:00:49Z
- **Completed:** 2026-06-23T04:43:39Z
- **Tasks:** 4
- **Files modified:** 21

## Accomplishments

- Added the `v1-8-release-readiness-checklist` section and no-claim review to `docs/parity/release-readiness.md`.
- Registered the checklist in human and machine parity roots, README entrypoints, v1.8 boundary docs, and the operator runtime release-hardening catalog.
- Added `scripts/check-phase87-release-readiness.ts` plus fixture tests for required rows, no-claim wording, parity roots, entrypoint links, verify order, and default-verification boundaries.
- Wired the Phase 87 checker and tests into `bash scripts/verify.sh` after the Phase 86 service-operation checks.
- Refreshed generated LOC metrics and retained Bazel lock freshness from verifier-side dependency metadata updates.

## Task Commits

Plan outputs from the prior autonomous run were committed in one phase-scoped finalization commit:

1. **Tasks 1-4: Release-readiness checklist, parity links, checker/tests, and closeout verification** - `4af193c` (`chore(87): finalize autonomous phase 87`)

This summary reconciles the missing GSD plan summary after that already-pushed finalization commit.

## Files Created/Modified

- `docs/parity/release-readiness.md` - Canonical v1.8 release-readiness checklist and no-claim review.
- `docs/parity/index.json` - Machine-readable parity root for `v1-8-release-readiness-checklist`.
- `docs/parity/checklist.md` - Human parity checklist entry for the Phase 87 surface.
- `docs/parity/README.md` - Compact parity entrypoint pointer to the checklist.
- `README.md` - Compact operator/contributor pointer to the v1.8 release-readiness checklist.
- `docs/parity/production-claim-boundary.md` - Release-readiness pointer while preserving the no-claim vocabulary.
- `docs/parity/support-matrix.md` - Support matrix pointer to release-readiness evidence.
- `docs/parity/upgrade-and-rollback-policy.md` - Upgrade and rollback pointer to release-readiness evidence.
- `docs/parity/operator-runbooks.md` - Runbook pointer to release-readiness evidence.
- `docs/parity/service-operation-expectations.md` - Service expectation pointer to release-readiness evidence.
- `docs/parity/deviations-and-unknowns.md` - Deferred surface and residual-risk pointer updates.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - Phase 87 release-review catalog surface.
- `scripts/check-phase87-release-readiness.ts` - Deterministic Phase 87 checker.
- `scripts/check-phase87-release-readiness.test.ts` - Fixture tests for checker success and failure modes.
- `scripts/verify.sh` - Phase 87 checker and test wiring after Phase 86.
- `docs/metrics/lines-of-code.md` - Fresh generated LOC report.
- `MODULE.bazel.lock` - Bazel module lock freshness from verifier-side metadata updates.
- `.planning/phases/87-release-readiness-checklist/87-VERIFICATION.md` - Phase verification evidence.
- `.planning/phases/87-release-readiness-checklist/87-01-SUMMARY.md` - This closeout summary.

## Decisions Made

- Kept Phase 87 documentation and Bun automation only; no first-party Rust source or test files changed, so parity source breadcrumbs did not need updates.
- Required every current v1.8 requirement id to appear in the release-readiness checklist rather than limiting the checklist to REL-01, REL-05, and REL-06.
- Kept release-readiness proof tied to canonical evidence roots and deterministic checker output rather than artifact existence, daemon startup, raw logs, or elapsed time alone.
- Preserved Phase 88 ownership for broad documentation claim scanning and overbroad production-readiness guardrails.

## Deviations from Plan

None - plan content was implemented as specified.

**Total deviations:** 0 auto-fixed.
**Impact on plan:** No scope change.

## Issues Encountered

The prior autonomous run landed all Phase 87 task outputs in a single finalization commit rather than per-task commits. No code or documentation rework was needed; this pass restored the missing GSD summary and shared tracking metadata before final verification.

## Verification

- `bun test scripts/check-phase87-release-readiness.test.ts` - passed with 6 tests and 17 assertions.
- `bun --check scripts/check-phase87-release-readiness.ts` - passed.
- `bun run scripts/check-phase87-release-readiness.ts` - passed.
- `bun run scripts/check-phase86-service-operation-expectations.ts` - passed.
- `bun -e 'JSON.parse(await Bun.file("docs/parity/index.json").text()); console.log("index ok")'` - passed.
- `git diff --check` - passed.
- `bash scripts/check-file-lengths.sh` - passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` - passed and refreshed `docs/metrics/lines-of-code.md`.
- `bash scripts/verify.sh` - passed in 30m 12.275s.

## Default Verification Boundary

Default verification remained public-network-free, real-service-manager-free, package-manager-service-free, Windows-service-free, support-upload-free, and multi-day-free. Phase 87 added only deterministic release-readiness checks.

## User Setup Required

None - no external service configuration required.

## Residual Risks

- Phase 87 checks release-readiness checklist coverage and linked roots only.
- Phase 88 still owns REL-02, REL-03, and REL-04 broad deterministic claim guardrails across documentation.

## Next Phase Readiness

Ready for lifecycle validation, roadmap/state completion updates, and wrapper finalization after full repo verification passes again with the new GSD closeout artifacts.

---
*Phase: 87-release-readiness-checklist*
*Completed: 2026-06-23*
