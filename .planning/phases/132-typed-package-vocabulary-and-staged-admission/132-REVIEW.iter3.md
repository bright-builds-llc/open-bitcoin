---
phase: 132-typed-package-vocabulary-and-staged-admission
reviewed: 2026-07-26T11:29:21Z
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
  warning: 3
  info: 1
  total: 4
status: issues_found
---

# Phase 132: Code Review Report

**Reviewed:** 2026-07-26T11:29:21Z
**Depth:** standard
**Files Reviewed:** 51
**Status:** issues_found

## Summary

The exact 51-file Phase 132 scope was re-reviewed after fix commits `08838db2`,
`7dee61cb`, and `10c9ed82`. The three findings from the prior review are
resolved: dust thresholds now use the exact serialized output size and Knots
spend sizes, fee rounding clamps checked multiplication and adjustment
overflow, and forbidden documentation claims are evaluated with local
sentence/clause negation.

The package vocabulary, prospective overlay, replacement/TRUC/ephemeral
ordering, final lifecycle assembly, parity documentation, and checker mutations
were reviewed against the pinned Knots sources. Targeted verification passed:
the Phase 132 checker passed its 27 mutation tests, and all 346
`open-bitcoin-mempool` unit/integration tests plus five doctests passed.
`bash scripts/verify.sh --fast` also completed successfully. Three actionable
correctness/parity gaps and one stale documentation item remain.

## Warnings

### WR-01: Null-data recognition accepts non-push scripts and rejects valid valued outputs

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-mempool/src/policy/output.rs:94-115`

**Issue:** Every script beginning with `OP_RETURN` is treated as standard
null-data, even when the suffix is not push-only. For example,
`OP_RETURN OP_CHECKSIG` bypasses `classify_script_pubkey` and is accepted. The
same branch rejects every nonzero-valued null-data output. Pinned Knots
`solver.cpp` classifies null-data only when the bytes after `OP_RETURN` are
push-only, while its standardness path does not require a zero value because an
unspendable output has a zero dust threshold. This produces both false
acceptance and false rejection at ordinary and package admission boundaries.

**Fix:** Add a `NullData` script classification, or validate the suffix with the
existing push-only parser before treating the output as data carrier. Route
malformed `OP_RETURN` scripts to `NonStandard`, retain the carrier-size and
enablement checks, and remove the zero-value rejection. Add parity tests for
`OP_RETURN OP_CHECKSIG`, truncated pushdata, a normal pushed payload, and a
nonzero-valued valid null-data output.

### WR-02: Output policy omits transaction-wide Knots data and dust limits

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-mempool/src/policy.rs:176-178`

**Issue:** Standardness validates outputs independently and never aggregates
their classifications. Pinned Knots rejects more than one null-data output,
rejects more than `MAX_DUST_OUTPUTS_PER_TX` (currently one) permitted dust
output, and by default rejects a transaction whose only output class is
null-data (`permitbaredatacarrier=false`). Open Bitcoin currently accepts all
three shapes when the corresponding per-output checks pass. This is especially
material to Phase 132 because enabling ephemeral dust allows a transaction to
create arbitrarily many dust outputs even though the pinned policy limit
remains one.

**Fix:** Have output validation return typed facts such as `is_null_data`,
`is_dust`, and `is_monetary`, aggregate them in
`validate_standard_transaction`, and enforce the pinned one-data-output and
one-dust-output limits plus the bare-datacarrier rule. Add a typed
`permit_bare_datacarrier` option if that Knots flag is intended to be
configurable. Add transaction-level tests for two valid `OP_RETURN` outputs,
two permitted dust outputs, data-only output, and a data output paired with a
monetary output.

### WR-03: Package removals omit committed lifecycle retry-clear facts

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-mempool/src/pool/package_admission/finalization.rs:84-101`

**Issue:** Package finalization records a removal and final absence for each
base member removed by replacement or final pressure trim, but it does not
record `MempoolRetryClearCause::LifecycleRemoval`. Single-transaction
admission, expiry, and connected-block removal all emit that retry-clear fact
for the same committed lifecycle event. A package replacement or pressure
eviction therefore returns an incomplete `MempoolLifecycleDelta`; downstream
cross-cache/retry projection can retain stale retry or unbroadcast state for a
member that is no longer in the mempool.

**Fix:**

```rust
builder
    .record_retry_clear(MempoolRetryClear {
        member,
        cause: MempoolRetryClearCause::LifecycleRemoval,
    })
    .map_err(lifecycle_invariant_error)?;
```

Record this beside each base removal and add package replacement and
pressure-trim assertions proving one identity-matched lifecycle retry clear per
removed member.

## Info

### IN-01: The parity catalog still lists completed Phase 132 package execution as a gap

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/catalog/mempool-policy.md:484-503`

**Issue:** The catalog's Phase 132 section says `PACK-01` through `PACK-07` are
closed, but `Known gaps` still lists package execution and pinned TRUC
exceptions as future Phase 132 work, and the follow-up trigger still says to
update the entry when package execution is added.

**Fix:** Remove the completed Phase 132 gap and package-execution trigger while
retaining the genuine later-phase package wire, projection, durability, retry,
and adapter boundaries.

***

_Reviewed: 2026-07-26T11:29:21Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
