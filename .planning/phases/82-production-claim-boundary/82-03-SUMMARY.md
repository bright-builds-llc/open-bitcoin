---
phase: 82-production-claim-boundary
plan: 03
subsystem: docs
tags: [readme, runtime-guide, parity-catalogs]
requires:
  - phase: 82-01
    provides: Canonical production claim boundary
provides:
  - README and runtime-guide v1.8 boundary pointers
  - Catalog deferral pointers for operator runtime, P2P, and chainstate
affects: [readme, operator-docs, parity-catalog]
tech-stack:
  added: []
  patterns:
    - Entry documents link to the canonical matrix instead of duplicating it
key-files:
  created: []
  modified:
    - README.md
    - docs/operator/runtime-guide.md
    - docs/parity/catalog/operator-runtime-release-hardening.md
    - docs/parity/catalog/p2p.md
    - docs/parity/catalog/chainstate.md
key-decisions:
  - "Kept operator commands unchanged and repo-local while updating only production-boundary pointer text."
  - "Catalog updates are compact deferral pointers, not duplicate matrices."
patterns-established:
  - "Public/operator entrypoints state the v1.8 boundary and v1.7 historical evidence in one place before command examples."
requirements-completed: [PROD-01, PROD-02, PROD-03, PROD-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 82-2026-06-21T12-38-13
generated_at: 2026-06-21T13:29:02Z
duration: 16min
completed: 2026-06-21
---

# Phase 82-03 Summary: Entrypoint Boundary Pointers

**README, runtime guide, and parity catalogs now point to the current v1.8 production boundary without expanding claims**

## Accomplishments

- Updated README status, parity-at-a-glance, and operator preview text to make v1.8 the current production claim boundary and v1.7 historical evidence.
- Updated the runtime guide opening and limitations pointers with the exact five support terms and the canonical boundary link while preserving Phase 80 UAT commands.
- Added compact Phase 82 pointers to operator-runtime, P2P, and chainstate parity catalogs.

## Task Commits

Task commits were deferred by the strict wrapper git gate. The wrapper will create one final phase commit only after clean verification and lifecycle validation.

## Verification

- Focused `rg` checks for README, runtime guide, catalog pointers, support terms, command preservation, and deferred-surface wording passed.
- Full `bash scripts/verify.sh` is recorded in the final Phase 82 verification artifact.

## User Setup Required

None.

## Next Phase Readiness

Operators and contributors now enter the current v1.8 boundary from README, runtime guide, and relevant catalogs without reading stale v1.7 wording as current scope.
