---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "01"
subsystem: consensus
tags: [genesis, sealed-proof, exact-work, basic, pinned-source]
requires:
  - phase: 164-canonical-chain-identity-and-compatibility-gates
    provides: Exact selected canonical genesis bodies and identity-only support facts
  - phase: 165-shared-exact-work-authority
    provides: First-party ProofWork and independent wide arithmetic expectations
provides:
  - Private-field complete-genesis proof with internally derived work for four selectors
  - Independent frozen complete-body, proof and zero-predecessor BASIC expectations
affects: [166-02, 166-05, 166-08, 166-10]
tech-stack:
  added: []
  patterns: [sealed fallible genesis constructor, independent bounded source reproduction]
key-files:
  created:
    - packages/open-bitcoin-consensus/src/chain_spec/bootstrap.rs
    - packages/open-bitcoin-consensus/src/chain_spec/bootstrap/tests.rs
    - scripts/phase166-genesis-vectors.ts
    - scripts/phase166-genesis-vectors.test.ts
    - docs/parity/fixtures/phase166-genesis-vectors.json
  modified:
    - packages/open-bitcoin-consensus/src/chain_spec.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - The sealed proof is distinct from identity-only selection and from future store/coins/runtime authority.
  - Full-body comparison precedes work derivation; no external height, work, flags or acceptance token enters the constructor.
  - Existing narrow BASIC oracle is reused under Bun orchestration; no new Python file or production dependency.
patterns-established:
  - CanonicalGenesisProof owns an immutable clone after exact selected-body comparison.
  - Full-width numerical fixtures remain explicitly arithmetic-only and cannot authorize a chain.
requirements-completed: [CBOT-02, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-18T23:06:41Z"
commits: false
duration: 5 min recorded execution window, initial reading excluded
completed: 2026-10-18
---

# Phase 166 Plan 01: Canonical Genesis Facts Summary

**Complete selected genesis bodies now produce a sealed proof with internally derived work; four independent frozen vectors include real genesis BASIC inputs and zero-predecessor commitments.**

## Performance

- Evidence recording began at 2026-10-18T23:01:53Z; preceding instruction/source reading is excluded from the recorded window.
- Tasks: 2/2. Owned implementation/evidence files: 7, plus this summary.
- No staging, commits, pushes, branch/worktree creation or hook bypass. Shared state, roadmap, requirements and task progress remain root-owned.

## Accomplishments and API

`chain_spec::bootstrap::CanonicalGenesisProof` is also re-exported through `chain_spec`. Its only constructor is `prove(ChainSpec, &Block) -> Result<Self, RootIdentityError>`. Private fields hold the selected spec, an owned complete block and derived `ProofWork`. Read-only accessors are `spec()`, `chain()`, `block()`, `header()`, `hash()` and `work()`.

The constructor reuses exact selected header/full-body checks before calling first-party `block_proof`. Tests cover all four selectors, changed header/non-genesis root, witness, output script, eight transaction/body mutations and all twelve foreign-selector pairs. Selection remains `IdentityOnly`; ordinary history still refuses, including missing Signet challenge validation. Genesis output exclusion is described as special UTXO handling, rather than script unspendability.

The Bun reproducer reads pinned Knots source constants and independently serializes/hashes all four complete blocks and transactions. It refuses an incorrect submodule revision or source bytes differing from `git show PIN:path`; frozen metadata binds seven exact source files by SHA-256. It invokes the existing narrow Python oracle for pinned protocol/SipHash helpers and GCS formulas, without changing that oracle. Each vector contains header/body, raw/display hashes and txid, serialized output outpoint, full output script/value, proof hex/LE and BASIC key/bytes/hash/header with zero predecessor. Four carry/wrap cases and four wide encoding cases remain a separate `arithmetic-only-no-chain-authority` section.

## Automated Evidence and RED/GREEN

All Bun/native commands used the pinned Bun 1.3.9 PATH. Cargo commands used `scripts/command-timings.ts` and ran sequentially. Actual logs are retained under ignored `.local/open-bitcoin-dev/phase166/plan01/`.

| Check | Actual result | Log |
| --- | --- | --- |
| Task 1 RED, planned consensus `phase166` filter | Exit 101; eight discovered, three passed/five failed against intentionally refusing constructor | `task1-red.log` |
| Task 1 GREEN | Exit 0; eight passed, zero failed | `task1-green.log` |
| Task 1 final after rustfmt | Exit 0; eight passed, zero failed; other integration harnesses discovered zero matching tests and are not counted | `task1-final.log` |
| Task 2 RED | Exit 1; five discovered/failed; fixture initially absent and reproduction intentionally unimplemented | `task2-red.log` |
| Fixture reproduction | Exit 0; four genesis rows generated independently of production Rust | `task2-write.log` |
| Task 2 GREEN | Exit 0; five passed, 33 assertions | `task2-green.log` |
| Task 2 final after exact pinned-source checking | Exit 0; five passed, 33 assertions | `task2-final.log` |
| `--check` | Exit 0; recomputation matches frozen values | `task2-check.log` |
| Final `--check` preservation | Exit 0; exact fixture bytes unchanged; SHA-256 `2d45750fd7c262fe1426bbff3a8ed6f20c3ebb0ae230095f3c7a8305f1a884d3` | `task2-check-readonly.log` |
| Scoped consensus Clippy all-targets/all-features with warnings denied | Exit 0 | `clippy.log` |
| Breadcrumb supported `--check` mode after reading `--help` | Exit 0; 1206 Rust files verified | `breadcrumbs.log` |

Rustfmt check on all three touched/new Rust files and `git diff --check` passed. The existing oracle and Phase 165 arithmetic fixture have no diff. No Phase 154 synthetic corpus or prior receipt was changed. The source-derived BASIC output retains the full genesis P2PK script even though genesis outputs are excluded from spendable coins. No native daemon was invoked or qualified.

## Task Commits

Deferred to the strict wrapper after complete phase acceptance; `commits: false` is intentional. No commit or hook result is claimed.

## Decisions Made

Followed the checked plan. Parent confirmed that the sealed proof is the distinct genesis evidence type and does not itself confer store, coins or runtime authority. No global selection-qualified evidence variant was added.

## Deviations from Plan

None. Additional scoped Clippy and fixture preservation checks substantiate the planned verification without broadening implementation scope.

## Issues Encountered

The first Cargo invocation briefly waited for its artifact-directory lock. The parent observed active rustc children after the wait released; execution completed normally without termination. Deliberate RED failures were retained and resolved by the implementations. An initial apply-patch request using delete-and-add for the same script was rejected without applying; it was retried as an update. No unresolved implementation failure remains.

## Review and Limits

The simplification pass kept production behavior to exact comparison, internal proof derivation and immutable data access. Owned source/fixture diffs were reviewed; no placeholder or stub remains. The impossible invalid-target branch maps to the existing fixed `NonCanonicalHeader` refusal rather than panicking. No new product filesystem/network/authentication surface was introduced beyond the planned test-only source reproduction.

Requirement IDs above record this plan's contribution; root owns complete requirement/lifecycle closure. Durable empty coins, store publication/reopen, runtime binding and BASIC publication are later plans. Full default native verification, historical ownership renewal, generated LOC, independent reviews and normal-hook Git saving remain scheduled for their declared owners. Ordinary history, networking activation and actual pinned-daemon differential qualification remain outside this plan.

## User Setup Required

None.

## Next Plan Readiness

Plan 166-02 can consume `CanonicalGenesisProof` through the concrete API above; no blocker remains for that handoff.

## Self-Check: PASSED

All five created implementation/evidence files and both modified owned paths exist. The new Rust files are registered in the breadcrumb manifest. All thirteen focused tests passed in final runs, scoped Clippy/breadcrumbs/format/diff checks passed, and the final fixture checker preserved bytes. Commit-existence checks are intentionally inapplicable under the strict-wrapper no-commit instruction.
