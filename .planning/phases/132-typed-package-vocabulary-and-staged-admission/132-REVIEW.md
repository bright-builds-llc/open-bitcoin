---
phase: 132-typed-package-vocabulary-and-staged-admission
reviewed: 2026-07-26T10:09:20Z
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
  info: 0
  total: 3
status: issues_found
---

# Phase 132: Code Review Report

**Reviewed:** 2026-07-26T10:09:20Z
**Depth:** standard
**Files Reviewed:** 51
**Status:** issues_found

## Summary

The exact requested Phase 132 scope was reviewed with the repository's local
guidance and the Bright Builds architecture, code-shape, verification, testing,
Rust, and TypeScript standards. The package types, prospective overlay,
revision guard, replacement/TRUC/ephemeral ordering, final-membership rewrite,
lifecycle construction, adapter boundary, documentation claims, and checker
mutations were traced in context.

No critical issue was found. Three correctness warnings remain: dust thresholds
do not match the pinned Knots formula, fee rounding can overflow after a
saturating multiplication, and the documentation-claim checker can be bypassed
by an unrelated negation elsewhere in the same paragraph.

## Warnings

### WR-01: Dust thresholds ignore the serialized output size

**File:** `packages/open-bitcoin-mempool/src/policy/output.rs:34-43`

**Issue:** `dust_threshold_sats_at_rate` assigns every recognized witness output
the same 110-vbyte size and every other spendable output the same 182-vbyte
size. The pinned Knots `GetDustThreshold` calculation instead adds the actual
serialized output size to a 67-vbyte discounted witness-input estimate or a
148-byte legacy-input estimate. At the default 3,000 sat/kvB rate, the current
code returns 330 sats for P2WPKH instead of 294, 330 sats for the 4-byte P2A
script instead of 240, and 546 sats for a standard P2SH output instead of 540.
That changes which outputs are dust and therefore changes standardness,
ephemeral permissions, zero-fee checks, and complete-spend enforcement. The
test at `pool/tests/package_policy_cases.rs:203-209` currently locks in two of
the incorrect values.

**Fix:** Compute the exact serialized `TransactionOutput` size (amount,
CompactSize script length, and script bytes), add the appropriate Knots spend
estimate, and apply the configured dust relay rate with checked rounding. Update
the P2WPKH/P2SH expectations and add explicit P2A and variable-script-length
vectors against the pinned Knots formula.

### WR-02: Fee rounding can overflow after saturating multiplication

**File:** `packages/open-bitcoin-mempool/src/fee.rs:53-55,70-72`

**Issue:** Both rounding expressions perform an ordinary addition after
`saturating_mul`. For example,
`FeeRate::from_sats_per_kvb(i64::MAX).fee_for_virtual_size(1)` first saturates
the product to `i64::MAX` and then adds 999. That panics in checked/debug builds
and wraps in optimized builds, potentially producing a negative required fee.
`from_fee_sats_and_vbytes` has the analogous `+ virtual_size - 1` boundary.
The public constructors accept the full `i64` range, and Phase 132 now routes
static, rolling, replacement, and dust calculations through these helpers, so
the behavior is neither fail-closed nor consistently checked.

**Fix:** Perform the product and rounding addition in checked `i128` arithmetic,
then return a checked/fallible conversion (or a deliberately documented clamp)
to `i64`. Prefer fallible constructors for policy-role rates if negative or
unrepresentable values are invalid. Add `i64::MAX`, negative-rate, zero-size,
and maximum-size boundary tests.

### WR-03: Any paragraph-level negation suppresses all forbidden-claim checks

**File:** `scripts/check-phase132-typed-package-staged-admission.ts:649-659`

**Issue:** Once a paragraph contains any marker such as `not`, `without`, or
`deferred`, every forbidden claim in that paragraph is accepted. A paragraph
such as “Open Bitcoin supports a general package wire. Guaranteed propagation
is not supported.” passes because the second sentence contains `not`, even
though the first sentence makes an out-of-scope affirmative claim. The existing
mutation test only appends a wholly affirmative paragraph, so it does not
exercise this bypass.

**Fix:** Evaluate each forbidden-claim occurrence in its own sentence or bounded
clause and require a negation marker local to that occurrence. Add a mutation
test containing an affirmative forbidden claim and an unrelated negative
sentence in the same paragraph.

***

_Reviewed: 2026-07-26T10:09:20Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
