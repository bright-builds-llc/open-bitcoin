# Phase 165 Plans 38–39: independent plan check

## ISSUES FOUND

Checked 2026-10-17 under lifecycle `165-2026-10-14T16-09-59Z`, mode `yolo`. Scope is the two prospective plans and their ledgers, with static source correspondence and historical caller analysis. This is not source execution, phase verification or acceptance of the outstanding final Plan 14.

**Result:** one blocker in Plan 38 and one scope warning in Plan 39. No historical data-contract or immutable-attribution blocker was found.

## Reviewed identities

| Artifact                             | Bytes | SHA256                                                             |
| ------------------------------------ | ----- | ------------------------------------------------------------------ |
| `165-38-PLAN.md`                     | 16839 | `66c71e526df17a07a22c930eb93f932143018b1706abba0418436f69d2fda769` |
| `165-38-FILTER-SEAMS-REPLAN.md`      | 4111  | `0c22e06d41010052a14b3ea9d29b02eb4fa58af7bd98ceb1fe07e0d5aef8b7c4` |
| `165-39-PLAN.md`                     | 25833 | `166172052672d1abd2d44740292b957156fb7ead89b0e233da00708070e6b12b` |
| `165-39-HISTORICAL-FILTER-REPLAN.md` | 9210  | `a2458dc557fa212cdce14df1a99eea002ba1bea93082da195d58dde90d24fc89` |

Authenticated the complete 344-file before packet (`a6f23987c5d1ea36909f8f49866f17bd086199014509171d63524c4252e229fd`), 81-CLI diagnosis (`8be56c1ff47c6e99d9acca120c00f0fe609131ead92a00ee803098bcff9572bd`) and filter diagnosis (`482ea24309e54c5295a4f0ab9185cd694fde5df2bcef2fef12ff2cc53770729e`). These remain diagnostic inputs; this review did not rerun those tests.

## Findings

### 1. Plan 38 Task 1 names nonexistent helper symbols

The action says to trace `Dense::seed→assemble_runtime` in `index_protection.rs`. The actual owner is `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs`: it declares `DenseHistory`, calls `DenseHistory::seed(&store)` at line 261 and calls `crate::assemble_durable_runtime(&runtime_config, runtime)` at line 264. Neither specified helper name matches that source.

Correct the action to name the actual file and `DenseHistory::seed` → `crate::assemble_durable_runtime` chain. Preserve the existing normal maturity, genuine root-inclusive acceptance/physical flush, retention, disable/deletion and reopen requirements. This is a small planning correction, not authorization to restore obsolete names in Rust.

### 2. Plan 39 reaches the 10-file scope warning threshold

Plan 39 declares ten owners and combines current guard semantics, three exact historical restorations, two entry gates, mutation support and current row integration. Its three tasks have 3/5/2 owners and are serialized with independent source/data acceptance, which reduces the risk. The checker threshold nevertheless calls for a warning at ten files.

Prefer splitting the historical adapter work from current source activation if execution cannot maintain the declared bounds. Otherwise retain the existing task boundaries, complete before-source packets, intermediate RED distinction, source-first reviews, 628-line limits and explicit stop/replan rules. This warning does not permit reducing the historical controls or later full-native gates.

## Dimension results

| Dimension                 | Result             | Evidence                                                                                                                                                           |
| ------------------------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Requirement coverage      | PASS               | Both new plans cover CWOR-02/03/05 support obligations; all four Phase 165 roadmap requirements appear in existing plan frontmatter. CWOR-04 remains Phase 166.    |
| Task completeness         | BLOCKER            | Every auto task has files/action/verify/done; Plan 38 Task 1 has the incorrect concrete helper chain above.                                                        |
| Dependencies              | PASS for new chain | 36 → 37 → 38 → 39 have waves 35/36/37/38 and existing references; no cycle. Root's final-14 rewire is a separate pending integration step.                         |
| Key links                 | PASS               | Both 156 rows update together; 157 follows caller/prepare/finish and genuine shutdown helpers; 159/161 preserve real typed/source/reopen edges.                    |
| Scope                     | WARNING            | Plan 38 has 3 tasks/9 owners; Plan 39 has 3 tasks/10 owners. Neither authorizes extra owners, packed code or a cap exception.                                      |
| Must-have derivation      | PASS               | Current evidence/refusal behavior, original regressions and immutable attribution trace to the phase's auditable shared-authority goal.                            |
| Context compliance        | PASS               | D-04/07/09/10/11/12 are addressed; no later bootstrap, canonical reopen, contextual consensus or daemon qualification is claimed. No silent scope reduction found. |
| Nyquist                   | SKIPPED            | `workflow.nyquist_validation` is explicitly false.                                                                                                                 |
| Cross-plan data contracts | PASS               | Current and historical generations are separated as detailed below.                                                                                                |
| AGENTS compliance         | PASS               | Pinned Bun, bounded readers, source-first review, preserved tests, no Rust/Git/hook mutation and final native acceptance remain explicit.                          |
| Research resolution       | PASS               | All four Open Questions have inline RESOLVED decisions.                                                                                                            |

All 39 available plans returned `valid: true`, no structural errors, from `gsd-tools verify plan-structure`; lifecycle verification returned scalar `valid`. All four reviewed Markdown files passed `mdformat --check --extensions gfm --extensions frontmatter`. No project skill directories were present at `.claude/skills` or `.agents/skills`.

## Historical contract assessment

- Current full 164/165 maps contain H (`historical-tooling.ts`) and E (`fixture-source.ts`), but not raw old-164 main/contracts merely because their executable modules are copied. Plan 39 explicitly recognizes this distinction and does not invent an existing raw input.
- H's fixed inverse is stored outside H in old-164 contracts, whose actual imports/re-exports are crypto and inert data leaves. Adding H → contracts introduces no H/E/main import cycle or self-hash fixed point. The new H must reconstruct complete original `47bbcd...`; it cannot renew that destination.
- E stays byte-exact at `cec470...`. Its separate complete one-file, three-hunk restoration to `b92d2d...`, the current Rust restoration to `2d0a7e...` and final `faa009...` attribution stay unchanged.
- The full current entry gates must bind PASSED raw H/E before imported classifiers count. Historical projections remain data-only: the filtered 159 transition legitimately omits H/E, and absent nonmanifest 161 checker data cannot trigger filesystem fallback. Missing manifest-bound 159 test still refuses.
- Changed 159 test and 161 checker receive exact current-to-authentic-before restorations before existing immutable qualification. Original manifest, snapshots, receipts and captured source identities cannot be rewritten. Current-only row and ordered input pin updates occur after independent source acceptance.
- Existing copied adapter support already runs actual fresh 164 and 165 entrypoints. Plan 39 requires new cases and direct stale-PASSED-map controls without changing the 14-module copy set; the actual Phase 163 import walker enforces 32 modules and 400 evidence entries.
- Original 159 test is 589 lines and has 39 lines of headroom; original 161 test SHA is `114dd2bb65d95ad3efcf51273e7d8198b46f31c8ad75600435ccb52988280e8a` and stays read-only. Old historical test is 89 lines, copied support 183, H 213 and old contracts 97. Plans require actual final caps and forbid adding an undeclared helper to evade them.

## Structured issues

```yaml
issues:
  - plan: "165-38"
    dimension: task_completeness
    severity: blocker
    task: 1
    description: "The concrete Dense::seed→assemble_runtime helper chain does not exist in the actual index_protection.rs owner."
    fix_hint: "Name packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs and its DenseHistory::seed→crate::assemble_durable_runtime chain; retain all specified behavior and negatives."
  - plan: "165-39"
    dimension: scope_sanity
    severity: warning
    description: "Ten declared owner files reach the checker warning threshold while spanning current guards, strict historical inverses and current activation."
    metrics:
      tasks: 3
      files: 10
      task_file_counts: [3, 5, 2]
    fix_hint: "Prefer splitting historical qualification from activation if the declared execution/review bounds cannot be maintained; otherwise preserve the serialized source-first gates and stop for a bounded replan before any extra owner or cap overrun."
```

## Recommendation

Revise the incorrect Plan 38 helper names, then recheck the exact revised plan bytes before release. Keep Plan 39's scope warning visible to the coordinator. Root must interpose the accepted repair chain before final Plan 14 while preserving its task XML and full native, coverage, Bazel, review/security/formal and normal-hook commit/push obligations. Neither repair plan is a substitute for those gates; Phase 166 remains outside this run.

Local `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, architecture/code-shape/verification/testing/TypeScript standards and the complete 7188-byte active lesson set materially informed this check. Only this review artifact was written; implementation, shared metadata and Git were left to their owners.

## Revision 1 recheck: VERIFICATION PASSED with retained scope warning

The coordinator requested a targeted revision and accepted Plan 39's nonblocking scope warning with its existing three task boundaries, independent whole-source inverse review, current-cohort controls, complete raw-input closure and source-before-pin gates retained. No human permission or new security-risk acceptance was inferred.

| Revised artifact                | Bytes | SHA256                                                             |
| ------------------------------- | ----- | ------------------------------------------------------------------ |
| `165-38-PLAN.md`                | 17588 | `ff27041bc9bc512d08dd2950c287e9fd352a751966f1d519859fb296438eca45` |
| `165-38-FILTER-SEAMS-REPLAN.md` | 4909  | `5922237015fba14f54ae7ca4fd7801d06135eaeb6a24690ac4b8eeed21979469` |

Independently read the revised Task 1 paragraph and complete ledger. The helper chain now names `DenseHistory::seed(store: &FjallNodeStore) -> (Self, DurableSyncRuntime)`, actual test call `DenseHistory::seed(&store)`, `populate_threshold` and its threshold assertion, then `crate::assemble_durable_runtime(&runtime_config, runtime)`. The expanded acceptance requirements preserve genuine empty rebind, normal maturity, root-inclusive acceptance/undo, actual physical flush and measured payloads, the consuming configured source, disable/deletion and true close/reopen evidence.

An exact paragraph inverse reconstructs the complete originally reviewed Plan 38 body at `66c71e...`; removing only the added helper-route paragraph reconstructs the complete initial ledger at `0c22e0...`. All other plan frontmatter, tasks, verify commands and bytes are therefore unchanged. Plan 39 and its ledger remain byte-exact at `166172...` and `a2458d...` respectively.

Fresh Plan 38 and 39 structure checks both returned `valid: true`, three complete tasks, empty errors and empty schema warnings. Lifecycle verification again returned scalar `valid`. All four prospective planning artifacts passed scoped GFM/frontmatter Markdown check. No application, source guard, native verifier or heavy test suite was run by this checker.

**Final scoped result: PASS for release, zero blockers, one coordinator-accepted nonblocking scope warning.** Initial findings above remain as authentic revision history, not unresolved blockers. The root-owned final-14 dependency rewire and all execution, source/pin review, full-native, formal/security/code-review and normal-hook Git gates remain outstanding.

```yaml
status: passed_with_warning
blockers: 0
warnings: 1
resolved_issues:
  - plan: "165-38"
    dimension: task_completeness
    task: 1
    resolution: "Actual DenseHistory::seed→populate_threshold/threshold assertion→crate::assemble_durable_runtime chain is now specified; exact whole-plan inverse confirms the revision is localized."
issues:
  - plan: "165-39"
    dimension: scope_sanity
    severity: warning
    disposition: "Coordinator accepted the ten-owner, three-task scope with source-first reviews, exact inverses, complete closure and bounded stop/replan rules retained."
```
