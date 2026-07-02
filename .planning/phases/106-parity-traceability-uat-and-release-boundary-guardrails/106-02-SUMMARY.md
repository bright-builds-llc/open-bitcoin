---
phase: 106-parity-traceability-uat-and-release-boundary-guardrails
plan: 106-02
subsystem: deterministic-phase-checker
tags:
  - typescript
  - verification
  - parity
  - release-boundary
requires:
  - 106-01
provides:
  - Phase 106 deterministic checker and regression fixtures.
  - scripts/verify.sh wiring after Phase 105 and before pure-core checks.
  - Current LOC metrics after adding checker scripts.
affects:
  - verifier
  - phase-checkers
  - metrics-docs
tech-stack:
  added: []
  patterns:
    - Fixed file corpus checker with JSON parsing and no-claim paragraph scan.
    - Fixture tests for traceability, UAT commands, Knots anchors, verifier order, external-gate drift, and forbidden positive claims.
key-files:
  created:
    - scripts/check-phase106-parity-uat-release-boundary.ts
    - scripts/check-phase106-parity-uat-release-boundary.test.ts
  modified:
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md
key-decisions:
  - "The checker validates the richer docs/parity/index.json checklist surfaces as the canonical v2.0 requirement ownership map."
  - "The top-level surface list is also checked so Phase 104 through Phase 106 remain visible in the summary status map."
  - "Bullet list items are paragraph boundaries for no-claim scanning so deferral lists do not inherit unrelated positive verbs."
patterns-established:
  - "v2.0 requirements across Phase 100-106 must appear exactly once in the parity checklist surface map."
requirements-completed:
  - BOUND-02
  - BOUND-04
  - BOUND-05
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 106-2026-07-02T03-46-34
generated_at: 2026-07-02T04:46:00Z
duration: 34m
completed: 2026-07-02
---

# Phase 106 Plan 02: Deterministic Guardrail Checker and Verification Wiring Summary

Phase 106 now has a deterministic checker and fixture coverage wired into the repo-native verifier.

## Accomplishments

- Added `scripts/check-phase106-parity-uat-release-boundary.ts`.
- Added fixture tests for missing `BOUND-*` requirements, duplicate v2.0 ownership, missing UAT commands, missing Knots anchors, missing verifier wiring, default public-network gate drift, and positive unsupported production/public relay claims.
- Wired the Phase 106 test and checker into `scripts/verify.sh` immediately after Phase 105 in both the visible command-order block and executable `run_step` sequence.
- Regenerated `docs/metrics/lines-of-code.md` after adding the TypeScript scripts.

## Verification

- `bun test scripts/check-phase106-parity-uat-release-boundary.test.ts` passed.
- `bun run scripts/check-phase106-parity-uat-release-boundary.ts` passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` passed.
- `git diff --check` passed.

## Residual Risks

- The checker is deterministic and local. It does not run public-network relay review, service-manager UAT, wall-clock soak, production deployment, or production-funds wallet tests.
- Full repo verification is recorded separately because the local environment previously hung during the Cargo test phase of `bash scripts/verify.sh`.
