---
phase: 132-typed-package-vocabulary-and-staged-admission
fixed_at: 2026-07-26T13:06:59Z
review_path: .planning/phases/132-typed-package-vocabulary-and-staged-admission/132-REVIEW.md
iteration: 2
findings_in_scope: 4
fixed: 4
skipped: 0
status: all_fixed
---

# Phase 132: Code Review Fix Report

**Fixed at:** 2026-07-26T13:06:59Z
**Source review:** `.planning/phases/132-typed-package-vocabulary-and-staged-admission/132-REVIEW.md`
**Iteration:** 2

**Summary:**

- Findings in scope: 4
- Fixed: 4
- Skipped: 0

## Fixed Issues

### WR-01: Null-data recognition accepts non-push scripts and rejects valid valued outputs

**Status:** fixed: requires human verification
**Files modified:** `packages/open-bitcoin-mempool/src/policy/output.rs`, `packages/open-bitcoin-mempool/src/policy.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs`, `packages/open-bitcoin-node/src/network/tests.rs`, `packages/open-bitcoin-rpc/src/dispatch/tests.rs`, `docs/metrics/lines-of-code.md`
**Commit:** `617f22c5`
**Applied fix:** Null-data classification now requires `OP_RETURN` followed by a push-only suffix, routes malformed or truncated push scripts through non-standard classification, and permits valid null-data outputs to carry value. Corrected node and RPC fixtures that had encoded decimal `80` (`OP_RESERVED`) as a direct push instead of the exact `OP_PUSHDATA1 80` carrier.
**Verification:** Public transaction-standardness tests cover empty scripts, `OP_RETURN OP_CHECKSIG`, truncated pushdata, a valid pushed payload, and a valued valid payload. The corrected node and RPC mempool-info regressions passed, targeted LLVM coverage reported no uncovered `policy/output.rs` lines, and the final normal commit hook passed the complete `bash scripts/verify.sh` contract.

### WR-02: Output policy omits transaction-wide Knots data and dust limits

**Status:** fixed: requires human verification
**Files modified:** `packages/open-bitcoin-mempool/src/policy/output.rs`, `packages/open-bitcoin-mempool/src/policy.rs`, `packages/open-bitcoin-mempool/src/types.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs`, `docs/metrics/lines-of-code.md`
**Commit:** `7531bc52`
**Applied fix:** Per-output validation now returns typed null-data, dust, and monetary facts that a typed transaction aggregate enforces after all outputs pass. Added the pinned one-null-data and one-dust limits plus the default-disabled `permit_bare_datacarrier` policy option.
**Verification:** Transaction-level tests reject two valid null-data outputs, reject two otherwise permitted dust outputs, reject a data-only transaction by default, accept data paired with a monetary output, and accept data-only when explicitly permitted. Targeted LLVM coverage reported no uncovered WR-02 production lines, and the normal commit hook passed the complete `bash scripts/verify.sh` contract.

### WR-03: Package removals omit committed lifecycle retry-clear facts

**Status:** fixed: requires human verification
**Files modified:** `packages/open-bitcoin-mempool/src/pool/package_admission/finalization.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs`, `docs/metrics/lines-of-code.md`
**Commit:** `911db6e6`
**Applied fix:** Package finalization now records a `MempoolRetryClear` with `LifecycleRemoval` beside every committed base-member removal before recording final absence.
**Verification:** Existing package replacement and final pressure-trim tests now prove retry-clear cardinality matches committed removals and each clear matches the exact removed txid/wtxid identity, including witness-alias requests. The normal commit hook passed the complete `bash scripts/verify.sh` contract.

### IN-01: The parity catalog still lists completed Phase 132 package execution as a gap

**Status:** fixed
**Files modified:** `docs/parity/catalog/mempool-policy.md`
**Commit:** `54177200`
**Applied fix:** Removed the completed Phase 132 package-execution and TRUC gap, removed its stale execution trigger, and retained only genuine later package-wire, cross-cache projection, durability, retry, adapter, and import/export boundaries with locally deferred claim wording.
**Verification:** All 27 Phase 132 checker mutation tests passed, the live PACK-01 through PACK-07 checker passed, and the normal commit hook passed the complete `bash scripts/verify.sh` contract.

## Skipped Issues

None — all findings in scope were fixed.

***

_Fixed: 2026-07-26T13:06:59Z_
_Fixer: the agent (gsd-code-fixer)_
_Iteration: 2_
