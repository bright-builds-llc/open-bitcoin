---
phase: 132-typed-package-vocabulary-and-staged-admission
verified: 2026-07-26T14:37:18Z
status: passed
score: 5/5 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 132-2026-07-25T18-13-00
generated_at: 2026-07-26T14:37:18Z
lifecycle_validated: true
overrides_applied: 0
re_verification:
  previous_result: gaps_found
  previous_score: 4/5
  gaps_closed:
    - "Pinned bare-anchor default now matches Bitcoin Knots, with explicit override behavior and independent mutation-tested checker coverage"
  gaps_remaining: []
  regressions: []
---

# Phase 132: Typed Package Vocabulary and Staged Admission Verification Report

**Phase Goal:** Operators can dry-run and submit bounded transaction packages with truthful ordered outcomes, correct fee policy, partial acceptance, and coherent final membership.
**Verified:** 2026-07-26T14:37:18Z
**Status:** passed
**Re-verification:** Yes — after closing the review-fix bare-anchor default and structural-proof gap

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Invalid packages are rejected before expensive work when empty, oversized, duplicated, non-topological, or internally conflicting. | ✓ VERIFIED | `WellFormedPackage::try_from` performs empty/count checks before identity and weight work, then duplicate txid/wtxid, checked cumulative weight, topology, and internal-conflict checks (`package/shape.rs:132-188`). Exact 25/404,000 boundaries, fingerprint order, and identity cases pass in the 344-test mempool suite and Phase 132 mutation checker. |
| 2 | Dry-run returns ordered per-transaction results without changing mempool, relay, persistence, or evidence state. | ✓ VERIFIED | `Mempool::dry_run_package` takes `&self`, runs the shared prospective evaluator, and discards its prepared transition (`pool/package_admission.rs:69-92`). Complete-snapshot and dry-run/submit equality regressions pass. No network, node, RPC, persistence, relay, or evidence adapter enters the pure package core. |
| 3 | Submitting a child with unconfirmed parents returns package-wide status, ordered final member outcomes, and effective-fee membership. | ✓ VERIFIED | The private `SubmissionPackage` refinement is constructible only through `try_from_package`; private `PackageReport`/`EffectiveFeeGroup` fields are validated by checked constructors. Five external compile-fail doctests, ordered partial-acceptance cases, and effective-fee group invariants pass. |
| 4 | A valid parent can remain when its child fails; accepted subpackages form one coherent delta; failed preparation causes no partial mutation. | ✓ VERIFIED | Individual-first evaluation stages successes in the sparse prospective overlay and applies submit once through the revision-bound `apply_prepared` seam. Valid-parent/invalid-child, residual rollback, replacement rollback, stale revision, and sparse recomputation regressions pass. Review fix `911db6e6` now also emits one identity-matched lifecycle retry clear for every committed package removal. |
| 5 | Final results reflect post-trim membership and the scoped replacement, TRUC, ephemeral-dust, witness-identity, reconsiderable, static-floor, and rolling-floor boundaries. | ✓ VERIFIED | Replacement/TRUC/ephemeral/order/finalization behavior passes, including exact dust formulas, checked fee rounding, null-data rules, transaction-wide output limits, lifecycle clears, one trim, and final aliases. `PolicyConfig::default().permit_bare_anchor` is now `true`, matching Knots `DEFAULT_PERMITBAREANCHOR{true}` and its kernel mempool option initialization; the focused regression proves default acceptance, explicit false rejection, and companion behavior. |

**Score:** 5/5 truths verified

## Review-Fix Verification

| Fix | Live Evidence | Regression Evidence | Status |
| --- | --- | --- | --- |
| `08838db2` exact Knots dust thresholds | Output serialized size plus 67-vB witness/P2A or 148-byte legacy spend size feeds the independent dust rate. | P2WPKH 294, P2SH 540, P2A 240, and CompactSize 252/253 cases pass. | ✓ VERIFIED |
| `7dee61cb` checked fee rounding | Multiplication and ceiling adjustment use checked `i128`, then explicitly clamp at the public `i64` boundary. | Positive/negative extreme, zero-size/rate, and signed-minimum cases pass without panic or wrap. | ✓ VERIFIED |
| `10c9ed82` local claim negation | Forbidden claims are evaluated by sentence/bounded clause rather than paragraph-wide negation. | Unrelated-sentence negation mutation fails as intended. | ✓ VERIFIED |
| `617f22c5` null-data validation | `OP_RETURN` requires a push-only suffix; malformed/truncated forms route to non-standard while valid valued null-data remains allowed. | Focused malformed, truncated, pushed-payload, and valued-payload tests pass. Direct Knots `solver.cpp` comparison agrees. | ✓ VERIFIED |
| `7531bc52` transaction output facts | Typed null-data/dust/monetary facts enforce one null-data output, one permitted dust output, and configurable bare-datacarrier policy. | Multi-data, multi-dust, data-only default, paired monetary, and explicit override tests pass. Direct Knots `policy.cpp:166-238` comparison agrees. | ✓ VERIFIED |
| `911db6e6` package removal retry clears | Finalization records `LifecycleRemoval` retry-clear facts beside immutable-base removals. | Replacement and final-pressure-trim tests assert cardinality, cause, txid, and wtxid identity. | ✓ VERIFIED |
| `54177200` parity catalog closure | Completed Phase 132 package execution/TRUC entries were removed from known gaps; later-phase boundaries remain. | Live checker and documentation reconciliation pass. | ✓ VERIFIED |
| `f63a2ee8` bare-anchor transaction policy plus `da871c84` closure | The configurable transaction-wide branch matches Knots, and `PolicyConfig::default()` now matches the pinned true default. | `transaction_output_facts_enforce_bare_anchor_toggle_and_companions` proves default acceptance, explicit false rejection, explicit true acceptance, and data/dust/monetary companions. | ✓ VERIFIED |
| `711e4c89` explicit claim boundaries | Checker accepts only explicit supported/deferred/out-of-scope forms near each forbidden claim. | Three double-negation mutations fail and five legitimate boundary controls pass within the 36/36 checker suite. | ✓ VERIFIED |
| `da871c84` structural-proof closure | `EphemeralPolicy` and `PolicyConfig` default bodies are sliced and checked independently. | Separate mutations of `anchor: true` and `permit_bare_anchor: true` both fail; all 36 checker mutation/control tests pass. | ✓ VERIFIED |

## Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-mempool/src/package.rs` | Bounded opaque package vocabulary and distinct dry-run/submit contracts | ✓ VERIFIED | Private invariant-bearing fields, cached identities, bounded constants, read-only accessors, and compile-fail privacy proof remain substantive and wired. |
| `packages/open-bitcoin-mempool/src/package/shape.rs` | Cheap-first validation and checked submission refinement | ✓ VERIFIED | Constructor ordering and child-with-unconfirmed-parents refinement remain intact. |
| `packages/open-bitcoin-mempool/src/package/report.rs` | Checked ordered reports and effective-fee groups | ✓ VERIFIED | Cardinality, order, status, membership, non-empty/unique group, and rate invariants pass. |
| `packages/open-bitcoin-mempool/src/pool/candidate.rs`, `pool/patch.rs`, and `pool.rs` | Shared pre-script preparation and revision-bound sparse apply | ✓ VERIFIED | Live application checks the base revision before mutation; failure-atomic regressions pass. |
| `packages/open-bitcoin-mempool/src/pool/prospective.rs` and `pool/prospective/limits.rs` | Sparse overlay and affected-subgraph recomputation | ✓ VERIFIED | Production remains sparse; full materialization/recomputation is test-only. Generated 25-member oracle cases pass. |
| `packages/open-bitcoin-mempool/src/pool/package_admission.rs` and extracted submodules | Shared dry-run/submit evaluator, residual policy, final membership, lifecycle | ✓ VERIFIED | Request-order evaluation, one trim, one submit apply, replacement/TRUC/ephemeral ordering, final rewrite, and retry-clear lifecycle are wired. |
| `packages/open-bitcoin-mempool/src/fee.rs` | Static/rolling floors, groups, and safe fee arithmetic | ✓ VERIFIED | Review-fixed checked rounding and grouping are exercised by current unit/parity coverage. |
| `packages/open-bitcoin-mempool/src/policy/output.rs` | Pinned dust/null-data/output/bare-policy behavior | ✓ VERIFIED | Exact dust, null-data, output counts, bare-datacarrier, and configurable bare-anchor behavior are substantive and receive the corrected pinned default. |
| `packages/open-bitcoin-mempool/src/types.rs` | Typed pinned policy defaults | ✓ VERIFIED | `permit_bare_anchor: true` matches pinned Knots `DEFAULT_PERMITBAREANCHOR{true}`; ephemeral anchor/send/dust defaults remain separately correct. |
| `packages/open-bitcoin-mempool/src/policy/replacement.rs`, `truc.rs`, and `ephemeral.rs` | Selected pure package-policy boundaries | ✓ VERIFIED | Pure evaluators remain wired into residual admission and pass focused/integrated coverage. |
| `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs` | Policy and lifecycle parity regressions | ✓ VERIFIED | Review-fix coverage includes the corrected default/override bare-anchor matrix and passes in the 344-test mempool suite. |
| `packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs` | Integrated PACK-01 through PACK-07 matrix | ✓ VERIFIED | Package bounds, ordering, partial acceptance, rollback, one trim, sparse overlay, and selected policy boundaries pass. |
| `scripts/check-phase132-typed-package-staged-admission.ts` | Mutation-tested structural and claim guardrail | ✓ VERIFIED | The checker independently scopes `EphemeralPolicy` and `PolicyConfig` defaults, and all 36 mutation/control cases pass. |
| `docs/parity/catalog/mempool-policy.md`, breadcrumb registry, and READMEs | Auditable narrow claims and Knots evidence | ✓ VERIFIED | Later peer/cache/durability/adapter boundaries remain explicit; all 439 Rust breadcrumbs pass. |

The mechanical helper reports 30/31 plan artifact literals and 27/31 key-link literals. Manual wiring inspection confirms the misses are module-layout or exact-regex false negatives: expiry applies a revision-bound patch through `apply_prepared`; submission reads separate `package()` and `kind()` accessors; replacement lives in extracted `package_admission/residual.rs`; replacement lifecycle and retry-clear facts finalize in `package_admission/finalization.rs`.

## Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `package/shape.rs` | cached identities and fingerprint | checked `TryFrom` / `PackageFingerprint::from_members` | ✓ WIRED | Request order is retained; fingerprint sorting does not mutate members. |
| typed dry-run/submit commands | package evaluator | distinct entry points over shared evaluation | ✓ WIRED | Submit consumes only the opaque refinement's immutable accessors. |
| evaluator | candidate/prospective overlay | pre-script preparation and sparse composition | ✓ WIRED | Failed preparation cannot mutate the live pool. |
| residual evaluator | fee/TRUC/replacement/limits/ephemeral/scripts | ordered pure policy stages | ✓ WIRED | Static → TRUC → rolling → limits/replacement → ephemeral → late-script behavior passes. |
| prospective patch | live mempool | `apply_prepared` | ✓ WIRED | Revision equality is checked before moves; submit applies once and dry-run never applies. |
| final prospective state | report and lifecycle delta | final rewrite plus `lifecycle_delta` | ✓ WIRED | Post-trim truth, witness aliases, removals, and retry clears derive from final state plus immutable base facts. |
| `PolicyConfig::default()` | bare-anchor branch | `permit_bare_anchor` | ✓ WIRED | The corrected true default reaches the transaction-wide branch; explicit false remains a tested rejection override. |
| `scripts/verify.sh` | Phase 132 checker/tests | ordered default verifier steps | ✓ WIRED | Full repo contract invokes checker mutations and live checker after Phase 131 and before the final release-boundary check. |

## Data-Flow Trace (Level 4)

| Artifact | Data | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| `pool/package_admission.rs` | ordered member results | validated package, chainstate snapshot, and current mempool | Yes | ✓ FLOWING |
| `pool/package_admission/residual.rs` | additions/removals and fee groups | pure policy decisions over the prospective overlay | Yes | ✓ FLOWING |
| `pool/package_admission/finalization.rs` | final report and lifecycle delta | post-trim membership plus immutable-base removal facts | Yes | ✓ FLOWING |
| `policy/output.rs` | transaction-wide output decision | per-output facts plus `PolicyConfig` | Yes | ✓ FLOWING |
| `pool.rs` | committed state | revision-bound sparse prepared patch | Yes | ✓ FLOWING |

## Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Review-fixed mempool/package behavior | `bun run scripts/command-timings.ts run --key phase132-mempool-lib-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool --lib` | 344 passed, 0 failed | ✓ PASS |
| Corrected bare-anchor default and override | `bun run scripts/command-timings.ts run --key phase132-bare-anchor-gap-closure -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool --lib transaction_output_facts_enforce_bare_anchor_toggle_and_companions` | 1 passed, 0 failed | ✓ PASS |
| Phase structural/claim failure sensitivity | `bun test scripts/check-phase132-typed-package-staged-admission.test.ts` | 36 passed, 0 failed | ✓ PASS |
| Live PACK checker | `bun run scripts/check-phase132-typed-package-staged-admission.ts` | PACK-01 through PACK-07 checker passed | ✓ PASS |
| Parity evidence coverage | `bun run scripts/check-parity-breadcrumbs.ts` | 439 Rust files verified | ✓ PASS |
| Full repo contract | `bash scripts/verify.sh` | Completed successfully in 13m 29.304s against the exact five-file state committed as `da871c84`, including format/lint/build, Rust tests and doctests, coverage, benchmark smoke validation, Bazel build/run, and Phase checkers | ✓ PASS |
| Pinned bare-anchor default comparison | `rg -n "DEFAULT_PERMITBAREANCHOR\\|permit_bare_anchor" packages/bitcoin-knots/src/policy/policy.h packages/bitcoin-knots/src/kernel/mempool_options.h packages/open-bitcoin-mempool/src/types.rs` | Knots `true`; Open Bitcoin `true` | ✓ PASS |

## Requirements Coverage

| Requirement | Source Plans | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| PACK-01 | 01, 08 | Cheap-first bounded package shape, identity, topology, and conflict validation | ✓ SATISFIED | Constructor and exact-bound/fingerprint/mutation tests pass. |
| PACK-02 | 04, 08 | Ordered mutation-free package dry-run | ✓ SATISFIED | Shared non-mutating evaluator; complete snapshot and report equality pass. |
| PACK-03 | 01, 04, 05, 08 | Checked child-with-unconfirmed-parents submission, ordered outcomes, status, and fee membership | ✓ SATISFIED | Opaque APIs, checked reports, and integrated partial/effective-group cases pass. |
| PACK-04 | 04, 08 | Individual-first partial acceptance | ✓ SATISFIED | Valid parent remains after invalid child; final report is partial. |
| PACK-05 | 02, 03, 04, 08 | Coherent sparse delta and failure-atomic preparation/application | ✓ SATISFIED | Revision-bound patch, prospective oracle, and rollback paths pass. |
| PACK-06 | 05, 08 | Effective-fee grouping with static and active rolling floors | ✓ SATISFIED | Group/floor ordering and review-fixed safe arithmetic pass. |
| PACK-07 | 01, 05, 06, 07, 08 | Final membership and selected replacement/TRUC/ephemeral/witness/reconsiderable policy boundaries | ✓ SATISFIED | Finalization and all selected policies pass; bare-anchor default/override behavior matches the pinned baseline and is independently mutation-guarded. |

All seven planned requirement IDs exist in `.planning/REQUIREMENTS.md`, and no additional requirement is mapped to Phase 132 without plan ownership.

## Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| None | — | No blocker or warning pattern found in corrective commit `da871c84` | ℹ️ Info | The prior misleading bare-anchor regression and structurally ambiguous checker match are both closed. |

No production TODO/FIXME/placeholder, empty implementation, hardcoded-empty result, or adapter import was found in the Phase 132 package core. Full-state materialization remains explicitly test-only.

## Human Verification Required

None. The phase contracts and corrective default are deterministically covered by code inspection, mutation tests, and runnable behavior checks.

## Deferred Items

No Phase 132 gap is deferred. Later phases continue to own peer package assembly (133), cross-cache projection (134), durability (135), transport (136), adapters (137), and integrated parity/release guardrails (138).

## Lifecycle Provenance

`132-CONTEXT.md`, all eight PLAN files, all eight SUMMARY files, and this report share lifecycle mode `yolo` and phase lifecycle ID `132-2026-07-25T18-13-00`. Direct validation with `gsd-tools verify lifecycle 132 --require-plans --require-verification` passes.

## Gaps Summary

The prior bare-anchor parity gap is closed by the exact five-file corrective commit `da871c84`: the production default matches Knots, the policy test proves default and override behavior, and separately scoped mutation checks prevent either the ephemeral defaults or the bare-anchor default from masking the other. All five roadmap truths, PACK-01 through PACK-07, review fixes, wiring, data flow, documentation boundaries, and focused repository verification surfaces pass.

The source correction, regression/checker coverage, and generated metric refresh are committed atomically as `da871c84`; unrelated planning/review working-tree changes remain unstaged or untracked and were excluded from this verification scope.

***

_Verified: 2026-07-26T14:37:18Z_
_Verifier: the agent (gsd-verifier)_
