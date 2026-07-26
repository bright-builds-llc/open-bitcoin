---
phase: 132-typed-package-vocabulary-and-staged-admission
fixed_at: 2026-07-26T13:39:59Z
review_path: .planning/phases/132-typed-package-vocabulary-and-staged-admission/132-REVIEW.md
iteration: 3
findings_in_scope: 2
fixed: 2
skipped: 0
status: all_fixed
---

# Phase 132: Code Review Fix Report

**Fixed at:** 2026-07-26T13:39:59Z
**Source review:** `.planning/phases/132-typed-package-vocabulary-and-staged-admission/132-REVIEW.md`
**Iteration:** 3

**Summary:**

- Findings in scope: 2
- Fixed: 2
- Skipped: 0

## Fixed Issues

### WR-01: Bare-anchor-only transactions remain accepted by default

**Status:** fixed: requires human verification
**Files modified:** `packages/open-bitcoin-mempool/src/types.rs`, `packages/open-bitcoin-mempool/src/policy/output.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs`, `docs/metrics/lines-of-code.md`
**Commit:** `f63a2ee8`
**Applied fix:** Added the typed `permit_bare_anchor` policy option with the pinned default of `false`. Transaction-wide output facts now reject transactions with neither monetary nor null-data outputs unless that option is enabled, matching the Knots `bare-anchor` branch.
**Verification:** The focused regression rejects default P2A-only and permitted ordinary-dust-only transactions, accepts both with the typed override, accepts anchor/dust outputs with monetary companions, and preserves data-plus-anchor handling through `permit_bare_datacarrier`. The normal commit hook passed the complete `bash scripts/verify.sh` contract.

### WR-02: Narrow-claim checker treats double negation as a safe boundary

**Status:** fixed: requires human verification
**Files modified:** `scripts/check-phase132-typed-package-staged-admission.ts`, `scripts/check-phase132-typed-package-staged-admission.test.ts`, `docs/metrics/lines-of-code.md`
**Commit:** `711e4c89`
**Applied fix:** Replaced the generic any-negation exemption with explicit claim-boundary forms tied to the forbidden claim. The checker now rejects `not deferred`, `not unsupported`, and `without deferring` while retaining supported forms such as `does not add`, `remains deferred`, `is not supported`, and `remains outside ... scope`.
**Verification:** All 35 checker mutation and control cases passed, including the exact `General package wire support is not deferred.` regression and five legitimate boundary controls. The live Phase 132 checker passed, and the normal commit hook passed the complete `bash scripts/verify.sh` contract.

***

_Fixed: 2026-07-26T13:39:59Z_
_Fixer: the agent (gsd-code-fixer)_
_Iteration: 3_
