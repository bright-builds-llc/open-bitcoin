---
phase: 138-parity-adversarial-pressure-restart-and-release-guardrails
reviewed: 2026-08-22T10:40:00Z
depth: standard
files_reviewed: 40
files_reviewed_list:
  - README.md
  - docs/operator/runtime-guide.md
  - docs/parity/benchmarks.md
  - docs/parity/catalog/mempool-policy.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/production-claim-boundary.md
  - docs/parity/release-readiness.md
  - docs/parity/source-breadcrumbs.json
  - docs/parity/support-matrix.md
  - packages/open-bitcoin-bench/src/cases/mempool.rs
  - packages/open-bitcoin-node/src/network/tests/recovery_cases.rs
  - packages/open-bitcoin-node/src/network/tests/recovery_cases/restart_composition.rs
  - scripts/check-current-documentation-reconciliation.test.ts
  - scripts/check-current-documentation-reconciliation.ts
  - scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts
  - scripts/check-phase124-milestone-closeout-reconciliation.fixtures/base.ts
  - scripts/check-phase124-milestone-closeout-reconciliation.fixtures/phase125.ts
  - scripts/check-phase124-milestone-closeout-reconciliation.test/scenarios-3.ts
  - scripts/check-phase124-milestone-closeout-reconciliation.test/setup.ts
  - scripts/check-phase124-milestone-closeout-reconciliation.ts
  - scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts
  - scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts
  - scripts/check-phase130-resource-time-fee-primitives.test.ts
  - scripts/check-phase130-resource-time-fee-primitives.ts
  - scripts/check-phase131-rolling-fee-expiry-pressure.test.ts
  - scripts/check-phase131-rolling-fee-expiry-pressure.ts
  - scripts/check-phase134-authoritative-lifecycle.test/scope-claims.ts
  - scripts/check-phase134-authoritative-lifecycle/scope.ts
  - scripts/check-phase135-snapshot-recovery.test.ts
  - scripts/check-phase135-snapshot-recovery.ts
  - scripts/check-phase138-parity-uat-release-boundary.test.ts
  - scripts/check-phase138-parity-uat-release-boundary.ts
  - scripts/check-phase138-parity-uat-release-boundary/checks.ts
  - scripts/check-phase138-parity-uat-release-boundary/claims.ts
  - scripts/check-phase138-parity-uat-release-boundary/constants.ts
  - scripts/check-phase138-parity-uat-release-boundary/matrix.ts
  - scripts/check-phase138-parity-uat-release-boundary/test-fixtures.ts
  - scripts/check-phase138-parity-uat-release-boundary/verifier.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 1
  info: 3
  total: 4
status: issues
---

# Phase 138: Code Review Report

**Reviewed:** 2026-08-22T10:40:00Z
**Depth:** standard
**Files Reviewed:** 40
**Status:** issues

## Summary

Phase 138 source from `8009a7b1` through HEAD was reviewed at standard depth against the restart-composition, PRESS-05 work-count, last-gate, and D-21/D-22 claim contracts. Production and bench Rust in this range do not use `unwrap()` or `Instant` wall-clock gates. Default `verify.sh` stays hermetic: Phase 117 remains present, Phase 138 is the last `check-phase*` command, current-documentation reconciliation follows immediately, and no `run_step` mentions `public-network`, `wall-clock`, or `run-live-mainnet-smoke`. The last-gate requires the verbatim D-21 sentence on README and the runtime guide, keeps `package relay` on the deny list rather than an allow list, and fails a positive `supports package relay` clause on both the Phase 138 and Phase 117 checkers.

The remaining defect is claim drift after Plan 04 promotion: `docs/parity/release-readiness.md` still says the closeout surface remains `in_progress` after index/checklist status is `done`.

## Warnings

### WR-01: Closeout surface still claimed in_progress on a live claim doc

**File:** `docs/parity/release-readiness.md:113`
**Issue:** After Plan 04, `docs/parity/index.json` and `docs/parity/checklist.md` mark `v2-2-parity-uat-release-boundary` `done`, and `scripts/check-phase138-parity-uat-release-boundary/checks.ts` requires that status. The live claim corpus file still says `The closeout surface \`v2-2-parity-uat-release-boundary\` remains in_progress.` The last-gate requires the D-21 sentence only on README and the runtime guide, so this leftover Plan 02 sentence can survive `bun run scripts/check-phase138-parity-uat-release-boundary.ts`. Operators reading release-readiness get the pre-closeout status.
**Fix:** Replace the leftover status clause with done/closed wording that keeps the verbatim D-21 sentence and does not introduce unscoped `package relay`. Optionally pin CLAIM_FILES so the closeout surface id cannot appear beside `remains in_progress`:

```markdown
Active v2.2 work uses this scoped wording: bounded local-package APIs, same-peer 1P1C assembly over ordinary transaction messages, ordinary transaction fanout, and initial-broadcast-retry of locally submitted unbroadcast members. Companion allowed wording includes persist canonical entries, acceptance times, and surviving local unbroadcast, rebuild derived state and reset rolling fee on restart, and hermetic default verification. The closeout surface `v2-2-parity-uat-release-boundary` is done.
```

## Info

### IN-01: Verifier comments and reconciliation errors still name Phase 117 as the final gate

**File:** `scripts/verify.sh:305` and `scripts/check-current-documentation-reconciliation.ts:334-337`
**Issue:** The executable order is 117 then 138 then reconciliation, and the checks enforce that sequence. The heredoc comment still says Phase 129 precedes the final Phase 117 gate, and reconciliation failures still say the order must immediately follow Phase 117.
**Fix:** Retarget the comment and failure strings to `117 then 138 then reconciliation` so a later last-gate move is diagnosed against the live contract.

### IN-02: Snapshot checklist-id ternary is a no-op

**File:** `scripts/check-phase138-parity-uat-release-boundary/checks.ts:109`
**Issue:** `surfaceId === SNAPSHOT_CHECKLIST_ID ? SNAPSHOT_CHECKLIST_ID : surfaceId` always equals `surfaceId`. Harmless, but it looks like a leftover mapping.
**Fix:** Use `surfaceId` directly, or map the human top-level snapshot name here if that was the intended branch.

### IN-03: Done Phase 136 surface still describes itself as an in_progress owner

**File:** `docs/parity/index.json:3600-3601`
**Issue:** `v2-2-receive-independent-maintenance-and-transport-receipts` is `done`, but `suspected_unknowns` still says Phase 138 review may tighten wording beyond this `in_progress owner`.
**Fix:** Drop or rewrite that unknown so it does not re-open a promoted surface.

---

_Reviewed: 2026-08-22T10:40:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
