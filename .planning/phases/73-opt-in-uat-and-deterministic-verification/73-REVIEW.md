---
phase: 73-opt-in-uat-and-deterministic-verification
reviewed: 2026-06-14T06:35:14Z
depth: standard
files_reviewed: 11
files_reviewed_list:
  - docs/metrics/lines-of-code.md
  - docs/operator/runtime-guide.md
  - docs/parity/README.md
  - docs/parity/catalog/chainstate.md
  - docs/parity/catalog/operator-runtime-release-hardening.md
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - scripts/check-phase73-uat-verification.test.ts
  - scripts/check-phase73-uat-verification.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 2
  info: 1
  total: 3
status: issues_found
---

# Phase 73: Code Review Report

**Reviewed:** 2026-06-14T06:35:14Z
**Depth:** standard
**Files Reviewed:** 11
**Status:** issues_found

## Summary

Reviewed the Phase 73 docs, parity ledger updates, generated LOC report, Bun checker, Bun regression test, and `scripts/verify.sh` wiring. This review applied `AGENTS.md` repo-local guidance, `AGENTS.bright-builds.md`, `standards-overrides.md`, and the canonical Bright Builds architecture, code-shape, verification, testing, and TypeScript/JavaScript standards.

The checker and its Bun regression test pass when run directly, and `docs/parity/index.json` parses as valid JSON. The main concerns are verification scope holes: the default verifier can be redirected through a test-only environment variable, and the machine-readable parity requirements understate two Phase 73 requirement IDs.

## Warnings

### WR-01: Default verification can be redirected to a test fixture repo

**File:** `scripts/check-phase73-uat-verification.ts:5`
**Issue:** The checker trusts `OPEN_BITCOIN_PHASE73_REPO_ROOT` to replace `REPO_ROOT`, and `verifyParityBreadcrumbChecker` skips the real breadcrumb checker whenever that variable is set. Because `scripts/verify.sh:130` invokes the checker without clearing the caller environment, a stale or malicious `OPEN_BITCOIN_PHASE73_REPO_ROOT` can make default verification validate another tree or fixture instead of the current checkout.
**Fix:**
```bash
env -u OPEN_BITCOIN_PHASE73_REPO_ROOT bun run scripts/check-phase73-uat-verification.ts
```

Alternatively, make the override explicitly test-only, for example by requiring `OPEN_BITCOIN_PHASE73_ALLOW_REPO_ROOT_OVERRIDE=1` in the test harness before honoring `OPEN_BITCOIN_PHASE73_REPO_ROOT`.

### WR-02: Phase 73 parity requirements omit VER-02 and VER-03

**File:** `docs/parity/index.json:1026`
**Issue:** The Phase 73 plans and checker require `VER-01`, `VER-02`, `VER-03`, and `VER-04`, but the machine-readable parity row and `docs/parity/checklist.md:34` list only `VER-01` and `VER-04`. Downstream release/readiness consumers can miss that Phase 73 also owns the deterministic coverage map and opt-in UAT matrix requirements.
**Fix:**
```json
"requirements": [
  "VER-01",
  "VER-02",
  "VER-03",
  "VER-04"
]
```

Update the checklist row to match, and consider extending `scripts/check-phase73-uat-verification.ts` to assert the ledger requirements for this surface.

## Info

### IN-01: New Bun regression test is not part of repo-native verification

**File:** `scripts/check-phase73-uat-verification.test.ts:173`
**Issue:** The new test file exercises the Phase 73 checker failure modes, but `scripts/verify.sh:114` through `scripts/verify.sh:130` only runs the checker itself. Future checker refactors could break the regression tests without failing the repo-native verification contract.
**Fix:** Add a focused Bun test invocation near the Phase 73 checker in `scripts/verify.sh`, or document that these script tests are intentionally manual:
```bash
bun test scripts/check-phase73-uat-verification.test.ts
```

## Verification Performed

- `bun run scripts/check-phase73-uat-verification.ts`
- `bun test scripts/check-phase73-uat-verification.test.ts`
- `bun -e 'JSON.parse(await Bun.file("docs/parity/index.json").text()); console.log("valid json")'`
- `git diff --check 55ce8fb^..HEAD -- <reviewed files>`

Full `bash scripts/verify.sh` was not run during this review.

---

_Reviewed: 2026-06-14T06:35:14Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
