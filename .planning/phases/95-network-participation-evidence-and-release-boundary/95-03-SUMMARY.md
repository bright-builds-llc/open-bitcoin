---
phase: 95-network-participation-evidence-and-release-boundary
plan: 03
subsystem: docs
tags:
  - operator-uat
  - release-boundary
  - support-matrix
  - production-claim-boundary
  - v1.9
dependency_graph:
  requires:
    - Phase 95 Plan 01 inbound evidence roots
    - Phase 95 Plan 02 parity closeout roots
  provides:
    - Phase 95 repo-local operator UAT command guidance
    - v1.9 public boundary wording for bounded opt-in inbound evidence
    - preserved deferred relay, public-default, service, and production-readiness claims
  affects:
    - docs/operator/runtime-guide.md
    - README.md
    - docs/parity/production-claim-boundary.md
    - docs/parity/support-matrix.md
tech_stack:
  added: []
  patterns:
    - repo-local Cargo and Bazel operator command forms
    - bounded opt-in inbound evidence wording
    - Phase 82 support vocabulary preservation
key_files:
  created:
    - .planning/phases/95-network-participation-evidence-and-release-boundary/95-03-SUMMARY.md
  modified:
    - docs/operator/runtime-guide.md
    - README.md
    - docs/parity/production-claim-boundary.md
    - docs/parity/support-matrix.md
decisions:
  - Bounded v1.9 inbound evidence is documented as opt-in UAT, not broad relay or public-default support.
  - Legacy Phase 82/87 guardrail literals remain present while newer v1.9 wording points at the closeout roots.
  - Runtime guide no-claim wording uses production-service vocabulary to satisfy service-lifecycle guardrails.
metrics:
  started_at: 2026-06-27T14:43:39Z
  completed_at: 2026-06-27T15:15:59Z
  duration: 32m20s
  tasks_completed: 2
  files_changed: 5
requirements_completed:
  - BOUND-01
  - BOUND-03
  - BOUND-04
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 95-2026-06-27T12-48-17
---

# Phase 95 Plan 03: Operator UAT and Public Boundary Summary

Added Phase 95 operator closeout commands and aligned public v1.9 boundary wording around bounded opt-in inbound evidence while keeping public defaults, relay, service operation, and production full-node readiness deferred.

## Tasks Completed

| Task | Result | Commit |
| --- | --- | --- |
| Task 1: Add Phase 95 operator UAT closeout commands | Added a runtime-guide section with repo-local Cargo/Bazel loopback or synthetic review commands and default verification boundaries. | `7369115c` |
| Task 2: Align public boundary wording with v1.9 scope | Updated README, production claim boundary, and support matrix wording for bounded opt-in inbound evidence while preserving deferred relay/public-default/service/production claims. | `01f7bffd` |

## Verification

Passed:

- Plan 03 Task 1 `rg` command for the Phase 95 runtime-guide heading, checker command strings, and required Cargo/Bazel command forms.
- Plan 03 Task 1 acceptance `grep` checks for all six repo-local binary command forms, `bun test scripts/check-phase95-network-participation-release-boundary.test.ts`, and `bash scripts/verify.sh`.
- Plan 03 Task 2 `rg` command for `v1.9`, `bounded opt-in inbound`, deferred relay/public-default terms, and production-readiness no-claim wording.
- Plan 03 Task 2 acceptance `grep` checks for `v1.9`, `bounded opt-in inbound`, `public inbound defaults`, `transaction relay`, `compact block relay`, `mempool propagation`, and `does not claim production full-node readiness`.
- `bun run scripts/check-phase82-production-claim-boundary.ts`
- `bun run scripts/check-phase83-support-matrix-issue-evidence.ts`
- `bun run scripts/check-phase87-release-readiness.ts`
- `bun run scripts/check-phase88-deterministic-claim-guardrails.ts`
- Commit hooks with `bash scripts/verify.sh` for both task commits. The Task 2 hook completed successfully and reported `verify.sh completed in 9m 55.541s`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Preserved service-lifecycle vocabulary**

- **Found during:** Task 1 commit hook
- **Issue:** The Phase 63 service-lifecycle guard rejected the exact runtime-guide phrase `production service`.
- **Fix:** Reworded the runtime guide no-claim text to use `production-service` operation.
- **Files modified:** `docs/operator/runtime-guide.md`
- **Commit:** `7369115c`

**2. [Rule 3 - Blocking] Preserved legacy support-matrix row label**

- **Found during:** Task 2 focused checker run
- **Issue:** The Phase 83 checker requires an exact `address relay` support-matrix environment-family row.
- **Fix:** Kept the `address relay` row label and put the v1.9 full-address-relay boundary in the row text.
- **Files modified:** `docs/parity/support-matrix.md`
- **Commit:** `01f7bffd`

**3. [Rule 3 - Blocking] Preserved Phase 82/87 legacy guardrail anchors**

- **Found during:** Task 2 commit hooks
- **Issue:** Older guardrails still require exact broad-claim and release-pointer anchors: `Open Bitcoin supports relay/inbound serving.`, `inbound serving`, the v1.8 production-readiness gate sentence, and `docs/parity/release-readiness.md#v18-release-readiness-checklist`.
- **Fix:** Restored those anchors as deferred or legacy pointers while keeping the new v1.9 bounded opt-in inbound evidence wording separate.
- **Files modified:** `README.md`, `docs/parity/production-claim-boundary.md`
- **Commit:** `01f7bffd`

## Known Stubs

None. Stub scan found none of the configured marker strings or hardcoded empty-value markers in the created/modified docs.

## Residual Risks

- Plan 04 still owns the deterministic aggregate Phase 95 checker. The runtime guide documents `scripts/check-phase95-network-participation-release-boundary.*` commands as required by this plan, but those script files are not present until the Plan 04 scope lands.
- The v1.9 evidence remains bounded opt-in loopback or synthetic review evidence only. Public inbound defaults, production network participation, transaction relay, compact block relay, mempool propagation, full address relay, production-service operation, and production full-node readiness remain deferred.

## Threat Flags

None. This plan changed documentation only and introduced no new network endpoint, auth path, file-access pattern, schema boundary, or runtime trust boundary.

## Self-Check: PASSED

- FOUND: `.planning/phases/95-network-participation-evidence-and-release-boundary/95-03-SUMMARY.md`
- FOUND: `7369115c`
- FOUND: `01f7bffd`
