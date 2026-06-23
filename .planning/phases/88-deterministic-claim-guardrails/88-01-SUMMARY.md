---
phase: 88-deterministic-claim-guardrails
plan: 01
subsystem: verification
tags:
  - release-readiness
  - parity
  - verification
  - bun
requires:
  - phase: 87-release-readiness-checklist
    provides: Release-readiness checker style, parity root pattern, and verifier-order parsing.
provides:
  - Deterministic Phase 88 claim-guardrail checker.
  - Fixture coverage for production-readiness overclaims, deferred-surface promotion, scoped no-claim wording, heredoc-only verifier drift, and default-verifier drift.
  - Fixed release/operator corpus scanning without recursive docs traversal.
affects:
  - scripts/check-phase88-deterministic-claim-guardrails.ts
  - scripts/check-phase88-deterministic-claim-guardrails.test.ts
tech-stack:
  added: []
  patterns:
    - Narrow Bun/TypeScript guardrail checker with temp-dir fixture tests.
    - Context-unit Markdown scanning that preserves table rows and scoped no-claim language.
key-files:
  created:
    - scripts/check-phase88-deterministic-claim-guardrails.ts
    - scripts/check-phase88-deterministic-claim-guardrails.test.ts
    - .planning/phases/88-deterministic-claim-guardrails/88-01-SUMMARY.md
  modified: []
key-decisions:
  - "Kept the scanner on the curated release/operator corpus from the Phase 88 plan instead of scanning all docs or planning archives."
  - "Validated executable `scripts/verify.sh` text after stripping `VERIFY_COMMAND_ORDER`, so heredoc-only Phase 88 commands are failures."
  - "Allowed scoped no-claim and deferred wording while failing unscoped exact production-readiness claims and positive promotions of Phase 82 deferred surfaces."
requirements-completed:
  - REL-02
  - REL-03
  - REL-04
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 88-2026-06-23T20-39-38
generated_at: 2026-06-23T21:19:46Z
duration: 20min
completed: 2026-06-23
---

# Phase 88 Plan 01: Deterministic Claim Guardrail Checker Summary

**Phase 88 now has a deterministic Bun checker and fixture suite for production-readiness and deferred-surface claim guardrails.**

## Performance

- **Duration:** 20 min
- **Completed:** 2026-06-23T21:19:46Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Added `scripts/check-phase88-deterministic-claim-guardrails.ts` with the exported `checkPhase88DeterministicClaimGuardrails` interface and `OPEN_BITCOIN_PHASE88_REPO_ROOT` fixture override.
- Added fixed-file reads for the curated public release/operator corpus, parity-root parsing for `v1-8-deterministic-claim-guardrails`, and executable verifier-order checks after Phase 87.
- Added context-unit scanning that preserves Markdown table rows, rejects unscoped production-readiness smoke claims, and rejects deferred-surface promotion with positive predicates.
- Added six fixture tests covering the success path, production-readiness overclaim failure, deferred-surface promotion failure, scoped wording allowance, heredoc-only verifier failure, and default-verifier drift failure.

## Task Commits

No task commit was created for Plan 01. The Phase 88 wrapper keeps all changes uncommitted until final focused and full verification pass, then creates one phase-scoped finalization commit.

## Files Created/Modified

- `scripts/check-phase88-deterministic-claim-guardrails.ts` - Deterministic Phase 88 checker.
- `scripts/check-phase88-deterministic-claim-guardrails.test.ts` - Fixture tests for checker behavior.
- `.planning/phases/88-deterministic-claim-guardrails/88-01-SUMMARY.md` - This plan summary.

## Decisions Made

- Kept Phase 88 in TypeScript/Bun to match the adjacent release-boundary checkers and avoid Rust source churn.
- Kept `.planning/`, milestone archives, and recursive directory traversal out of the checker corpus.
- Treated existing negative release-boundary phrasing such as `without internet access`, `no public-network`, and `remain outside` as scoped no-claim context to avoid false positives while preserving positive-claim failures.

## Deviations from Plan

The implementation added a few extra scoped negative phrases beyond the minimum required list so current release-readiness rows remain valid no-claim contexts.

**Total deviations:** 1 intentional refinement.
**Impact on plan:** No scope change.

## Issues Encountered

The first real-doc dry run showed that `release-blocking live sync` contains the promotion predicate `release-blocking`. The checker now avoids treating the deferred surface name itself as a promotion predicate and still fails standalone `release-blocking` promotions.

## Verification

- `bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts` - passed with 6 tests and 18 assertions.
- `bun run scripts/check-phase88-deterministic-claim-guardrails.ts` - intentionally failed against the real repo because Plan 02 parity roots, pointers, and verifier wiring were not yet registered.

## Default Verification Boundary

Plan 01 added only local deterministic checker code and fixture tests. It did not add public-network, service-manager, package-manager service, support-upload, destructive-repair, or multi-day default verification.

## User Setup Required

None.

## Residual Risks

- Plan 02 still needs to register Phase 88 in parity roots, add public/operator pointers, wire `scripts/verify.sh`, refresh LOC, and run full verification.
- Production full-node readiness remains future-scoped.

## Next Phase Readiness

Ready for Plan 02 docs, parity-root, verifier, metrics, and verification closeout work.
