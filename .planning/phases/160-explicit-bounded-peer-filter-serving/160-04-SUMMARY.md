---
phase: 160-explicit-bounded-peer-filter-serving
plan: "04"
subsystem: consensus
tags: [proof-work, fixed-width-arithmetic, stale-serving, parity]
requires: []
provides:
  - Exact target-derived 256-bit ProofWork with modular accumulation and ordering
  - Signed best-header proof-equivalent time with modular multiplication and pinned saturation
affects: [160-05, 160-11, 160-12]
tech-stack:
  added: []
  patterns: [private fixed limbs, bounded pure arithmetic, typed fail-closed inputs]
key-files:
  created:
    - packages/open-bitcoin-consensus/src/chain_proof.rs
    - packages/open-bitcoin-consensus/src/chain_proof/tests.rs
  modified:
    - packages/open-bitcoin-consensus/src/lib.rs
key-decisions:
  - ProofWork exposes ZERO and modular domain operations without a public raw-limb constructor or validation-provenance assertion.
  - Keep compact-target decoding and existing fork-choice scoring unchanged; reject decoded-zero targets locally.
  - Use bounded four-u64 arithmetic with no floating-point, u128 proxy, dependency, or I/O.
requirements-addressed: [CFNET-05]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-10T19:08:33Z"
duration: 5min
completed: 2026-10-10
---

# Phase 160 Plan 04: Exact Block Proof and Signed Equivalent Time Summary

**Exact 256-bit target proof and signed best-header equivalent time now preserve Knots modular overflow, floor division and signed INT64_MAX saturation.**

## Performance

- Started: 2026-10-10T19:03:50Z
- Completed: 2026-10-10T19:08:33Z
- Tasks: 2/2 implemented and locally verified
- First-party Rust files created/modified: 3

## Accomplishments

- Added private four-u64 work with exact carry, borrow, modular multiplication, numeric ordering and fixed 256-step restoring division.
- Reused compact-target decoding while rejecting zero values produced by truncated compact mantissas. Negative and overflowing targets fail closed.
- Implemented signed differences, best-header proof denominator, caller-supplied positive consensus spacing, floor division after 256-bit modular multiplication and signed saturation above 63 bits.
- Preserved existing HeaderEntry u128 scoring and fork-choice behavior; this pure module establishes arithmetic only, with header/branch provenance owned by Plan 05.

## Task Commits

Both tasks and this summary await the outer consolidated commit. No staging, commits, push, branch changes, worktrees, hook bypass or shared lifecycle-state updates were performed by this executor. Requirement completion remains pending lifecycle-valid phase verification.

## Verification and TDD Evidence

All Cargo executions used pinned Bun 1.3.9 through `scripts/command-timings.ts`, stable key `phase160-chain-proof`, and the checkout cooperative build lock. Cargo commands ran sequentially in this executor.

- Task 1 RED: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus phase160_proof -- --nocapture` executed 6 tests; 4 failed and 2 passed before implementation. Task 1 GREEN: all 6 passed after implementation.
- Task 2 RED: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus phase160_proof_time -- --nocapture` executed 11 tests; 9 failed and 2 passed before implementation. Final Task 2 GREEN: 14 executed and passed after adding error, negative-floor and maximum-spacing cases.
- Final combined `phase160_proof` run: 20 executed, 20 passed, zero ignored. Nonmatching existing integration targets correctly ran zero matching tests; the required unit-test count is nonzero.
- Crate-scoped `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --all-targets --all-features -- -D warnings`: passed.
- Targeted `cargo llvm-cov --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --lib --lcov --output-path /tmp/open-bitcoin-phase160-proof.lcov -- phase160_proof --nocapture`: 20 executed and passed; new production module has 105/105 covered lines and 12/12 covered functions. Branch instrumentation was not enabled; no measured branch-coverage percentage is claimed.
- Scoped rustfmt check and `git diff --check`: passed. Reviewed all new source plus the two-line lib.rs export diff for scope and side effects.
- Independent known vectors come from the pinned `GetBlockProof` formula, including regtest, mainnet, nonuniform compact target, unit target, target 2, target 65535 and maximum compact target. The nonuniform equivalent-time expectations use independent integer calculations, rather than calling the Rust function to construct expected results.
- Full native verification, independent source/security review, serial breadcrumb registration and final requirement completion remain owned by the parent workflow.

## Acceptance Criteria and Threat Mitigations

- [x] No floating-point/u128 approximation or new dependency.
- [x] Fixed arithmetic loop bounds, no unchecked shifts outside limb width, no allocation, unsafe code, unwrap/expect or panic in production.
- [x] Age sign, best-proof denominator, multiplication width and saturation have distinct focused tests.
- [x] Invalid/zero/overflow targets and unusable proof/spacing fail closed.
- [x] All executable production lines/functions are covered; decision alternatives have explicit sign, equality, invalid-input, floor, overflow and saturation cases.
- [x] T-160-10 exact modular proof arithmetic, T-160-11 target rejection and T-160-12 fixed division/multiplication ceilings are implemented with passing unit evidence.

## Simplification Review

Removed an unnecessary explicit overflow flag from restoring division: each remainder fits the already-consumed numerator prefix, so its left shift remains within the at-most-256-bit input width. Reused one shift helper for division and modular multiplication. Production remains 165 lines and focused tests 277 lines.

## Breadcrumb Registration Handoff

Parent must serially register both exact new paths below using their existing in-source ordered anchor list:

- `packages/open-bitcoin-consensus/src/chain_proof.rs`
- `packages/open-bitcoin-consensus/src/chain_proof/tests.rs`

Ordered anchors for each: `packages/bitcoin-knots/src/chain.cpp`, then `packages/bitcoin-knots/src/arith_uint256.h`. Existing lib.rs infrastructure breadcrumb remains unchanged. This executor did not edit the shared breadcrumb manifest.

## Deviations from Plan

None in implementation scope. The outer strict gate intentionally defers commits and state/requirement updates as specified by this plan. Decoded-zero handling is an explicit target-validation requirement, implemented locally without changing pre-existing decoder behavior.

## Issues and Next Steps

No known stubs, authentication gates or introduced trust-boundary surfaces. Plan 05 must bind this arithmetic to genuinely admitted/recovered header facts and scripts-valid stale provenance. Local arithmetic verification does not claim peer-serving integration or phase-wide completion.

## Self-Check: PASSED

All three declared Rust files and this summary exist. Source exports, nonzero passing test counts, covered production lines/functions and clean whitespace were verified. Commit existence is inapplicable because commits are explicitly deferred to the outer gate; no commit hashes are fabricated.
