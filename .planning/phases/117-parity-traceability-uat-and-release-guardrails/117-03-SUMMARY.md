---
phase: 117-parity-traceability-uat-and-release-guardrails
plan: "03"
subsystem: release-documentation
tags: [v2.1, parity, operator-docs, release-boundary, block-relay]
requires:
  - phase: 117-02
    provides: Deterministic Phase 117 claim and verifier guardrails
provides:
  - One canonical bounded/default-off v2.1 release narrative
  - Exact Cargo and Bazel operator inspection commands
  - Aggregate-only block-relay evidence and optional public-network UAT boundary
affects: [readme, release-review, operator-uat, support-policy]
tech-stack:
  added: []
  patterns:
    - Canonical release handoff links instead of a competing changelog
    - Aggregate fixed evidence with explicit redaction and deferred-surface lists
key-files:
  created: []
  modified:
    - README.md
    - docs/parity/README.md
    - docs/parity/release-readiness.md
    - docs/parity/production-claim-boundary.md
    - docs/parity/deviations-and-unknowns.md
    - docs/parity/support-matrix.md
    - docs/operator/runtime-guide.md
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
key-decisions:
  - "Bounded/default-off block serving and compact-block relay are preview evidence; public-default, archive-scale, service, and production forms remain deferred."
  - "Public-network review may be recorded as not run and is never part of pre-commit, default CI, release verification, or scripts/verify.sh."
requirements-completed: [BOUND-03, BOUND-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 117-2026-07-10T05-06-19
generated_at: 2026-07-10T05:55:43.000Z
duration: 6min
completed: 2026-07-10
---

# Phase 117 Plan 03: README, Operator Docs, Runtime Docs, and Release Notes Summary

**Contributor, parity, support, release, runtime, status, and observability docs now present one bounded/default-off v2.1 contract with exact local review commands and unchanged broader deferrals**

## Performance

- **Duration:** 6 min
- **Completed:** 2026-07-10T05:55:43.000Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Made `docs/parity/release-readiness.md` the canonical v2.1 handoff and linked Phase 110 through 117 evidence, the checker pair, full verifier, and UAT package.
- Added exact Cargo and Bazel status, RPC, and support-bundle commands plus focused and full verification commands to the runtime guide.
- Reclassified bounded/default-off block serving and compact-block relay as preview evidence while preserving package/filter/public/archive/service/production/wallet/packaging/GUI/migration/repair/upload deferrals.
- Documented aggregate-only `block_relay` evidence and redaction of raw payloads, hashes, peer/endpoints, permission data, credentials, secrets, and dynamic labels.

## Task Commits

The parent verification-first wrapper reserves git mutation until Phase 117 passes. Task changes are pending the final phase commit.

## Files Created/Modified

- `README.md` and `docs/parity/README.md` — Concise current v2.1 claim and canonical handoff pointer.
- `docs/parity/release-readiness.md` — v2.1 release-review handoff and evidence roots.
- Production boundary, deviations, and support matrix — Scoped preview terms plus broader deferrals.
- Runtime guide, status snapshot, and operator observability — Exact commands, aggregate evidence, redaction, and optional-UAT boundaries.
- Phase 117 checker/test — More robust Markdown table/no-claim parsing exercised by the real historical corpus.

## Decisions Made

- Kept historical milestone sections intact while adding a current v2.1 statement that supersedes their earlier all-deferred block-relay wording.
- Treated Markdown table rows as semantic units so a deferred support term qualifies the statement in the same row without making the checker whitespace-dependent.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Historical table rows and support-bundle nouns caused claim-checker false positives**

- **Found during:** Repository-mode checker verification.
- **Issue:** Cell-by-cell parsing separated a deferred support term from its statement, while `support` in `support-bundle` was interpreted as a positive verb.
- **Fix:** Made Markdown rows atomic, narrowed the positive verb to `supports`, and added explicit grammatical no-claim markers with a regression fixture.
- **Verification:** Fixture suite passes 14/14 and repository mode validates.
- **Committed in:** Pending final wrapper commit.

**Total deviations:** 1 auto-fixed blocking issue.
**Impact on plan:** Stronger structural parsing; forbidden positive claims remain covered by negative fixtures.

## Issues Encountered

None after the checker parsing correction.

## User Setup Required

None - public-network review is optional and no external service configuration is required.

## Next Phase Readiness

- All five BOUND requirements are represented in completed plans.
- The final UAT package and full repository verification can now run against the finished corpus.

***

## Self-Check: PASSED

- `bun test scripts/check-phase117-parity-uat-release-boundary.test.ts`: 14 pass, 0 fail.
- `bun run scripts/check-phase117-parity-uat-release-boundary.ts`: validated.
- `git diff --check` passes.

*Phase: 117-parity-traceability-uat-and-release-guardrails*
*Completed: 2026-07-10*
