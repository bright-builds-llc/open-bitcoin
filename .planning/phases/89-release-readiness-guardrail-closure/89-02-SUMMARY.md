---
phase: 89-release-readiness-guardrail-closure
plan: 02
subsystem: deterministic-claim-guardrails
tags:
  - release-readiness
  - parity
  - verification
  - bun
requires:
  - phase: 89-release-readiness-guardrail-closure
    plan: 01
    provides: Expanded release-readiness rows and Phase 87 enforcement.
  - phase: 88-deterministic-claim-guardrails
    provides: Deterministic claim-guardrail checker and fixture pattern.
provides:
  - Expanded Phase 88 curated corpus for upgrade, runbook, and service policy docs.
  - Fixture coverage for deferred-surface promotion in each newly covered policy doc.
  - Policy-doc pointers and parity-root evidence for the expanded guardrail corpus.
affects:
  - scripts/check-phase88-deterministic-claim-guardrails.ts
  - scripts/check-phase88-deterministic-claim-guardrails.test.ts
  - docs/parity/index.json
  - docs/parity/checklist.md
  - docs/parity/upgrade-and-rollback-policy.md
  - docs/parity/operator-runbooks.md
  - docs/parity/service-operation-expectations.md
tech-stack:
  added: []
  patterns:
    - Fixed curated corpus lists instead of recursive docs traversal.
    - Temp-dir fixture tests for focused claim-guardrail behavior.
key-files:
  created:
    - .planning/phases/89-release-readiness-guardrail-closure/89-02-SUMMARY.md
  modified:
    - scripts/check-phase88-deterministic-claim-guardrails.ts
    - scripts/check-phase88-deterministic-claim-guardrails.test.ts
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/upgrade-and-rollback-policy.md
    - docs/parity/operator-runbooks.md
    - docs/parity/service-operation-expectations.md
key-decisions:
  - "Added only the three canonical policy docs named by GAP-02 to the Phase 88 corpus; `.planning/`, milestone archives, and recursive docs traversal remain excluded."
  - "Kept the existing deferred-surface, promotion-predicate, and scoped-allowance vocabulary because the real docs passed without broadening it."
  - "Used the same scoped guardrail pointer sentence in all three new policy docs."
requirements-completed:
  - REL-02
  - REL-03
  - REL-04
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 89-2026-06-24T20-03-26
generated_at: 2026-06-24T20:50:00Z
duration: focused
completed: 2026-06-24
---

# Phase 89 Plan 02: Expanded Deterministic Claim Corpus Summary

**The Phase 88 deterministic claim guardrails now scan the upgrade policy, operator runbooks, and service operation expectations docs.**

## Accomplishments

- Added `docs/parity/upgrade-and-rollback-policy.md`, `docs/parity/operator-runbooks.md`, and `docs/parity/service-operation-expectations.md` to the Phase 88 checker `TARGET_FILES`, `POINTER_FILES`, and `REQUIRED_EVIDENCE`.
- Added fixture tests proving unscoped promotion fails in each new policy doc:
  - migration apply mode as production-ready
  - inbound serving as fully supported
  - Windows service integration as production-grade
- Extended scoped wording tests so allowed deferred, outside-default-verification, and unsupported phrasing still passes.
- Registered the expanded Phase 88 evidence roots in `docs/parity/index.json` and `docs/parity/checklist.md`.
- Added the exact v1.8 deterministic claim-guardrail pointer sentence to each newly scanned policy doc.

## Task Commits

No task commit was created for Plan 02. The yolo wrapper defers commit and push until the phase verification gate is clean.

## Deviations from Plan

None.

## Verification

- `bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts` - passed with 7 tests and 24 assertions.
- `bun run scripts/check-phase88-deterministic-claim-guardrails.ts` - passed.
- IDE diagnostics for edited TypeScript files - no linter errors found.

## Default Verification Boundary

Plan 02 preserved the curated local corpus and did not add public-network, real service-manager, package-manager service, support-upload, destructive-repair, recursive docs, planning archive, or multi-day default verification.

## Residual Risks

- Future docs outside the curated corpus can still drift unless later phases add scoped evidence or deliberately expand the corpus.
- Production full-node readiness remains future-scoped.
