---
phase: 132-typed-package-vocabulary-and-staged-admission
reviewed: 2026-07-26T13:15:10Z
depth: standard
files_reviewed: 51
files_reviewed_list:
  - README.md
  - docs/metrics/lines-of-code.md
  - docs/parity/catalog/mempool-policy.md
  - docs/parity/source-breadcrumbs.json
  - packages/README.md
  - packages/open-bitcoin-consensus/src/classify.rs
  - packages/open-bitcoin-consensus/src/script/witness.rs
  - packages/open-bitcoin-mempool/src/fee.rs
  - packages/open-bitcoin-mempool/src/lib.rs
  - packages/open-bitcoin-mempool/src/package.rs
  - packages/open-bitcoin-mempool/src/package/report.rs
  - packages/open-bitcoin-mempool/src/package/shape.rs
  - packages/open-bitcoin-mempool/src/package/tests.rs
  - packages/open-bitcoin-mempool/src/policy.rs
  - packages/open-bitcoin-mempool/src/policy/ephemeral.rs
  - packages/open-bitcoin-mempool/src/policy/ephemeral/tests.rs
  - packages/open-bitcoin-mempool/src/policy/output.rs
  - packages/open-bitcoin-mempool/src/policy/replacement.rs
  - packages/open-bitcoin-mempool/src/policy/replacement/diagram.rs
  - packages/open-bitcoin-mempool/src/policy/truc.rs
  - packages/open-bitcoin-mempool/src/policy/truc/tests.rs
  - packages/open-bitcoin-mempool/src/pool.rs
  - packages/open-bitcoin-mempool/src/pool/admission.rs
  - packages/open-bitcoin-mempool/src/pool/candidate.rs
  - packages/open-bitcoin-mempool/src/pool/expiry.rs
  - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
  - packages/open-bitcoin-mempool/src/pool/oracle.rs
  - packages/open-bitcoin-mempool/src/pool/package_admission.rs
  - packages/open-bitcoin-mempool/src/pool/package_admission/finalization.rs
  - packages/open-bitcoin-mempool/src/pool/package_admission/residual.rs
  - packages/open-bitcoin-mempool/src/pool/package_admission/test_support.rs
  - packages/open-bitcoin-mempool/src/pool/patch.rs
  - packages/open-bitcoin-mempool/src/pool/patch/graph.rs
  - packages/open-bitcoin-mempool/src/pool/pressure.rs
  - packages/open-bitcoin-mempool/src/pool/prospective.rs
  - packages/open-bitcoin-mempool/src/pool/prospective/limits.rs
  - packages/open-bitcoin-mempool/src/pool/tests.rs
  - packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs
  - packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs
  - packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs
  - packages/open-bitcoin-mempool/src/pool/tests/pressure_internal_cases.rs
  - packages/open-bitcoin-mempool/src/pool/tests/prospective_failure_cases.rs
  - packages/open-bitcoin-mempool/src/pool/tests/prospective_oracle_cases.rs
  - packages/open-bitcoin-mempool/src/pool/tests/revision_cases.rs
  - packages/open-bitcoin-mempool/src/resource.rs
  - packages/open-bitcoin-mempool/src/types.rs
  - packages/open-bitcoin-node/src/network.rs
  - scripts/check-phase131-rolling-fee-expiry-pressure.test.ts
  - scripts/check-phase132-typed-package-staged-admission.test.ts
  - scripts/check-phase132-typed-package-staged-admission.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
status: issues_found
---

# Phase 132: Code Review Report

**Reviewed:** 2026-07-26T13:15:10Z
**Depth:** standard
**Files Reviewed:** 51
**Status:** issues_found

## Summary

The exact 51-file Phase 132 scope was re-reviewed after the iteration-three
fixes. The four prior findings are resolved: null-data classification now
requires a push-only suffix and permits valued unspendable outputs,
transaction-wide null-data/dust/bare-datacarrier limits are enforced, package
removals emit identity-matched lifecycle retry clears, and the stale parity
catalog entries were removed. The earlier exact dust-threshold, checked fee
arithmetic, and local sentence/clause claim-check fixes also remain sound.

The review applied the repository's Bright Builds architecture, code-shape,
verification, testing, Rust, and TypeScript guidance and compared policy
behavior with the pinned Bitcoin Knots baseline. Targeted verification passed:
the Phase 132 checker passed all 27 mutation tests, the checker itself passed,
all 343 `open-bitcoin-mempool` unit tests, five integration tests, and five
doctests passed, and crate-scoped Clippy passed with warnings denied. Two
actionable policy/proof-checker gaps remain.

## Warnings

### WR-01: Bare-anchor-only transactions remain accepted by default

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-mempool/src/policy/output.rs:49-75`

**Issue:** Transaction-wide output enforcement rejects null-data-only
transactions when bare data carriers are disabled, but does not apply Knots'
corresponding `bare-anchor` rule when there are no monetary or null-data
outputs. Because permitted dust outputs are marked non-monetary, a transaction
whose only output is a zero-valued pay-to-anchor output currently passes the
default Open Bitcoin policy. Pinned Knots rejects that transaction as
`bare-anchor` unless `permitbareanchor` is enabled. The same omission also
affects any transaction containing only permitted dust outputs and no
null-data output.

**Fix:** Add a typed `permit_bare_anchor` policy option with the pinned default
of `false`, then enforce the missing transaction-wide branch:

```rust
if self.monetary_outputs == 0
    && self.null_data_outputs == 0
    && !config.permit_bare_anchor
{
    return Err(PolicyError::non_standard(
        "bare-anchor transactions are disabled",
    ));
}
```

Add tests proving default rejection of pay-to-anchor-only and ordinary
dust-only transactions, configurable acceptance when the option is enabled,
and unchanged handling of a data output paired with an anchor.

### WR-02: Local double negation bypasses the package-scope claim checker

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase132-typed-package-staged-admission.ts:649-663`

**Issue:** The checker skips a forbidden package-scope claim whenever its local
sentence or clause contains any negation marker. That correctly fixed the
earlier paragraph-level false exemption, but it also exempts affirmative claims
formed by negating a boundary word. In a temporary mutation fixture, appending
`General package wire support is not deferred.` caused
`checkPhase132TypedPackageStagedAdmission` to return no failures. The sentence
claims an explicitly out-of-scope wire surface, so the proof checker can still
certify documentation that contradicts the Phase 132 boundary.

**Fix:** Recognize explicit allowed boundary forms rather than treating any
local marker as sufficient negation. For example, require the forbidden claim
to be the object of a supported form such as `is not supported`, `is deferred`,
or `is outside scope`, and reject double-negative forms including `not
deferred`, `not unsupported`, and `without deferring`. Add the exact
`General package wire support is not deferred.` mutation as a regression test.

***

_Reviewed: 2026-07-26T13:15:10Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
