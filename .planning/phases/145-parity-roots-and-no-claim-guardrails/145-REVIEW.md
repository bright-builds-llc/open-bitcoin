---
phase: 145-parity-roots-and-no-claim-guardrails
reviewed: 2026-09-18T08:50:00Z
depth: standard
files_reviewed: 23
files_reviewed_list:
  - scripts/check-phase145-parity-uat-release-boundary.ts
  - scripts/check-phase145-parity-uat-release-boundary.test.ts
  - scripts/check-phase145-parity-uat-release-boundary/checks.ts
  - scripts/check-phase145-parity-uat-release-boundary/constants.ts
  - scripts/check-phase145-parity-uat-release-boundary/claims.ts
  - scripts/check-phase145-parity-uat-release-boundary/verifier.ts
  - scripts/check-phase145-parity-uat-release-boundary/test-fixtures.ts
  - scripts/verify.sh
  - README.md
  - docs/operator/runtime-guide.md
  - docs/parity/index.json
  - docs/parity/checklist.md
  - docs/parity/catalog/chainstate.md
  - docs/parity/catalog/p2p.md
  - docs/parity/source-breadcrumbs.json
  - docs/parity/release-readiness.md
  - docs/parity/production-claim-boundary.md
  - docs/parity/support-matrix.md
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/PROJECT.md
  - .planning/STATE.md
  - .planning/phases/145-parity-roots-and-no-claim-guardrails/145-UAT.md
findings:
  critical: 0
  warning: 1
  info: 1
  total: 2
status: issues
---

# Phase 145: Code Review Report

**Reviewed:** 2026-09-18T08:50:00Z
**Depth:** standard
**Files Reviewed:** 23
**Status:** issues

## Summary

Phase 145 is a closeout and no-claim guardrail phase. The seven `v2-3-*` surfaces own the 15 requirement IDs exactly once, leftover REQUIREMENTS rows are `[x]`, `verify.sh` is 144 then 145 then 121 with Phase 138 still file-final, and the D-14 sentence is verbatim on README and the runtime guide. The last-gate checker is filesystem-only and fixture-tested, but its D-16 substring match misses the live hyphenated and backtick spellings used in the claim corpus, so a positive overclaim written the way those docs already write the phrases would pass.

## Warnings

### WR-01: D-16 deny tokens miss live hyphenated and backtick spellings

**File:** `scripts/check-phase145-parity-uat-release-boundary/constants.ts:125-141`
**Issue:** `checkClaims` lowercases clauses and then requires `lower.includes(topic)`. The deny list uses `production service operation` and `leveldb chainstate`, but the live claim corpus writes `production-service operation` (required by the Phase 63 runtime-guide rule) and `LevelDB \`chainstate/\``. Those spellings do not contain the deny tokens, so a positive sentence such as `Open Bitcoin provides production-service operation.` or `Open Bitcoin provides LevelDB \`chainstate/\` live import.` would not fail. Current docs pass only because they wrap those phrases in `does not` / `deferred` markers. `coins repair` from D-16 is also absent as its own token.
**Fix:** Normalize punctuation before matching, and add a mutation for the live spellings:

```typescript
function normalizeClaimText(text: string): string {
  return text.toLowerCase().replaceAll("`", "").replaceAll("-", " ");
}

// In checkClaims, compare normalizeClaimText(clause) against
// normalizeClaimText(topic) so "production-service operation" and
// "LevelDB `chainstate/`" match the D-16 topics.

export const DENIED_OVERCLAIMS = [
  // existing tokens...
  "coins repair",
] as const;
```

Add fixture tests that append the hyphenated and backtick-positive sentences to `README.md` and expect those topics in the failure list.

## Info

### IN-01: Closeout UAT evidence still says Plan 04 owns leftover flips

**File:** `.planning/phases/145-parity-roots-and-no-claim-guardrails/145-UAT.md:26`
**Issue:** Test 1 evidence still says `Plan 04 still owns leftover Pending flips and done promotions` after Plan 04 already flipped those rows and promoted the seven surfaces. `result: pending` can stay until phase verification; the evidence sentence is now factually stale on a named closeout evidence root.
**Fix:** Replace that sentence with the Plan 04 outcome (leftover Pending flipped, surfaces `done`) and keep `result: pending` only if verification has not yet recorded pass/fail.

---

_Reviewed: 2026-09-18T08:50:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
