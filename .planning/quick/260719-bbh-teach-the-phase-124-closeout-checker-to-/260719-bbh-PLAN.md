# Quick Task 260719-bbh: Teach the Phase 124 closeout checker to accept approved v2.1 gap planning

**Date:** 2026-07-19
**Status:** Complete

## Objective

Extend the Phase 124 milestone-closeout reconciliation checker with one exact,
fail-closed post-audit gap-planning state so the approved Phases 127–129
planning corpus passes without weakening any earlier Phase 124/126 lifecycle
state or the bounded v2.1 no-claim guardrails. Then run the focused and full
repository verification contracts and finish the pending milestone-planning
commit.

## Tasks

1. Add the exact post-audit gap-planning state and regression coverage.
   - Files:
     `scripts/check-phase124-milestone-closeout-reconciliation.ts`,
     `scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts`,
     `scripts/check-phase124-milestone-closeout-reconciliation.test.ts`
   - Action: Add a narrowly typed, additive state for the approved corpus only:
     the audit is `gaps_found`; Phases 110–126 remain complete; Phases 127,
     128, and 129 are pending in dependency order with their approved
     requirement assignments; exactly 29 of 39 requirements are complete; and
     the ten pending requirements have exactly these single owners:
     `BSRV-03`, `BSRV-04`, `OBS-02`, and `OBS-04` → Phase 127;
     `CMP-04`, `CMP-05`, and `OBS-03` → Phase 128; and `OBS-01`,
     `BOUND-02`, and `HARD-05` → Phase 129. Require the canonical coverage
     counts, `gaps_found` audit scores/routing, and `/gsd-plan-phase 127`
     next action while rejecting archive or execution routes. Select this state
     only when its full discriminator is present; leave the existing Phase
     124 and Phase 126 lifecycle parsers, artifact/frontmatter checks, legacy
     legal states, verifier ordering, and no-claim scan unchanged and active.
     Extend the existing fixture model instead of coupling mutation tests to
     the live worktree. Add focused Arrange/Act/Assert tests for the accepted
     state and for wrong ownership, wrong 29/39 or 10-pending counts, malformed
     Phase 127–129 topology, non-`gaps_found` audit state, stale routing, and
     positive deferred-scope claims. Keep the real-repository corpus test as
     the integration proof and retain all existing lifecycle mutation tests as
     regression guards.
   - Verify:
     `bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts`
     and
     `bun run scripts/check-phase124-milestone-closeout-reconciliation.ts`.
   - Done: The approved post-audit Phase 127–129 planning corpus passes; every
     specified mutation fails with a focused diagnostic; all previous
     Phase 124/126 legal-state, lifecycle-provenance, verifier-order, and
     no-claim tests still pass.

2. Run the repository contract and finish the pending milestone-planning commit.
   - Files: the three checker/test files above plus the already pending
     `.planning/MILESTONES.md`, `.planning/PROJECT.md`,
     `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`,
     `.planning/STATE.md`, and `.planning/v2.1-MILESTONE-AUDIT.md` changes;
     include `docs/metrics/lines-of-code.md` only if repo-managed verification
     regenerates it as a required freshness update.
   - Action: Review the complete diff and confirm it contains only the approved
     v2.1 post-audit planning projection, the checker’s additive legal state,
     its tests/fixtures, and any required generated LOC freshness. Do not edit
     production runtime code, implement Phases 127–129, weaken no-claim
     boundaries, rewrite prior lifecycle evidence, or revert any existing
     user/orchestrator planning changes. After all checks pass, create the
     pending atomic commit with a message such as
     `fix(v2.1): accept approved gap-planning state`.
   - Verify:
     `git diff --check`;
     `bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts`;
     `bun run scripts/check-phase124-milestone-closeout-reconciliation.ts`;
     and the full repo-native contract `bash scripts/verify.sh`.
   - Done: Focused and full verification pass, the final diff has no unintended
     files or behavior, and the approved milestone gap-planning changes plus
     their checker support are committed atomically.

## Guidance Applied

This plan follows the repo-local Bun scripting and strict
`bash scripts/verify.sh` contract from `AGENTS.md`, the Bright Builds workflow
sidecar, `standards/core/verification.md`,
`standards/core/testing.md`, and the TypeScript guidance for typed legal states,
functional validation, and focused Arrange/Act/Assert tests. No active local
override applies.
