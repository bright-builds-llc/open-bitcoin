---
phase: 132-typed-package-vocabulary-and-staged-admission
fixed_at: 2026-07-26T11:13:04Z
review_path: .planning/phases/132-typed-package-vocabulary-and-staged-admission/132-REVIEW.md
iteration: 1
findings_in_scope: 3
fixed: 3
skipped: 0
status: all_fixed
---

# Phase 132: Code Review Fix Report

**Fixed at:** 2026-07-26T11:13:04Z
**Source review:** `.planning/phases/132-typed-package-vocabulary-and-staged-admission/132-REVIEW.md`
**Iteration:** 1

**Summary:**

- Findings in scope: 3
- Fixed: 3
- Skipped: 0

## Fixed Issues

### WR-01: Dust thresholds ignore the serialized output size

**Status:** fixed: requires human verification
**Files modified:** `packages/open-bitcoin-mempool/src/policy/output.rs`, `packages/open-bitcoin-mempool/src/policy.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs`, `docs/metrics/lines-of-code.md`
**Commit:** `08838db2`
**Applied fix:** Replaced fixed witness and legacy spend-size thresholds with the exact Knots calculation: serialized transaction-output size plus 67 witness/P2A bytes or 148 legacy bytes. Added public policy regressions for P2WPKH, P2SH, and P2A outputs and CompactSize boundary tests for variable-length scripts.
**Verification:** Focused dust-threshold and P2A policy tests passed, targeted LLVM coverage passed, and the normal commit hook completed the full `bash scripts/verify.sh` contract.

### WR-02: Fee rounding can overflow after saturating multiplication

**Status:** fixed: requires human verification
**Files modified:** `packages/open-bitcoin-mempool/src/fee.rs`, `packages/open-bitcoin-mempool/src/types.rs`, `docs/metrics/lines-of-code.md`
**Commit:** `7dee61cb`
**Applied fix:** Moved fee multiplication and rounding into checked `i128` arithmetic, documented and applied conservative clamping at the public `i64` boundary, and preserved Knots-style nonzero signed minimum fees when division would otherwise round to zero. Added arithmetic and public API boundary coverage for positive and negative extremes, zero rates and sizes, one-vbyte rates, and maximum virtual size.
**Verification:** Focused fee arithmetic and fee-rate tests passed, and the normal commit hook completed the full `bash scripts/verify.sh` contract.

### WR-03: Any paragraph-level negation suppresses all forbidden-claim checks

**Status:** fixed: requires human verification
**Files modified:** `scripts/check-phase132-typed-package-staged-admission.ts`, `scripts/check-phase132-typed-package-staged-admission.test.ts`, `docs/parity/catalog/mempool-policy.md`, `docs/metrics/lines-of-code.md`
**Commit:** `10c9ed82`
**Applied fix:** Scoped forbidden-claim negation to sentence- and bounded-clause-local text, including Markdown list boundaries, instead of accepting any negation elsewhere in the paragraph. Added a mutation with an affirmative general-package-wire claim followed by an unrelated negative sentence, and made the existing parity catalog's deferred claim explicit in its own list item.
**Verification:** The checker mutation suite passed with 27 tests, the live Phase 132 checker passed, and the normal commit hook completed the full `bash scripts/verify.sh` contract.

## Skipped Issues

None — all findings in scope were fixed.

***

_Fixed: 2026-07-26T11:13:04Z_
_Fixer: the agent (gsd-code-fixer)_
_Iteration: 1_
