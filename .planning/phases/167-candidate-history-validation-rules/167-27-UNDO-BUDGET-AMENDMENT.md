---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T22:27:05.597Z"
commits: false
---

# Phase167 complete-undo allocation amendment

This adds exactly one cohesive Plan27 task for resource accounting in the existing node accepted-BASIC work boundary. It preserves every locked decision, requirement, original task XML block and automated command, and adds no consensus, filter-content, proof, runtime dependency or future-phase scope. Independent root planning review is required before source execution. No implementation or test result is asserted by this amendment.

## Finding and evidence boundary

Accepted07 added BlockUndo.overwrites:Vec<BlockOverwriteUndo>; each batch owns overwritten:Vec<OverwrittenCoin>, and each entry retains previous_coin and new_coin with independently owned scripts.07's accepted summary explicitly distinguishes zero batches (unknown provenance) from one complete batch containing zero entries (captured no-overwrite). It records the approved stronger restoration behavior and its pinned normal-disconnect difference. Nothing in this resource amendment revises that evidence or grants canonical authority to imported same-shape records.

Read-only08 investigation and root confirmation identified that node chainstate/filter_index.rs fact_work charges sizeof(BlockUndo), ordinary TxUndo/restored Coin elements and scripts, then capture/capture_reorg clones the full undo. borrowed_undo_work repeats the ordinary accounting. sizeof(BlockUndo) includes the new outer Vec header, but neither traversal charges its heap batch elements, nested entry elements or previous/new script bytes. The shared counters therefore omit allocations that are copied/read. This is a source-identified accounting gap, not a claimed observed runtime failure.08's historical tests have no BASIC owner on their seeded path; they do not establish a failing BASIC behavior. The root-reported running unfiltered node suite and its successful/failed outcomes remain attributable to08; partial progress never counts as final acceptance or an observed regression here.

The planner's immutable source snapshot at2026-10-21T22:27:05.597Z measured522physical lines/19810bytes, SHA2569cf19c472ce8b25d0c3d0c33dc7e2465d689fe813333baeb50e558dced088c48. Earlier approximate535-line observations remain historical; this records the actual current measurement only. All evidence/producer/codecs/fixtures are read-only in this revision. Meaningful omitted-allocation RED must be produced by27 against its actual then-current source before the fix; a setup failure, zero-test result or retrospective assertion cannot substitute.

## Exact bounded ownership

| Plan/task | Owner                                                                    | Responsibility                                                                                                                     |
| --------- | ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------- |
| 27.1      | packages/open-bitcoin-node/src/chainstate/filter_index.rs                | One private add_undo_work accumulator shared by fact_work and borrowed_undo_work, plus test child registration inside this parent. |
| 27.1      | packages/open-bitcoin-node/src/chainstate/filter_index/phase167_tests.rs | Cohesive test-only resource regressions, small capture positives and checked boundary tests.                                       |

The second owner is the optional cohesive child from the request, selected for the executable plan so the existing parent stays focused. No third source owner, public export, crate/dependency, authority constructor, new error variant, codec/fixture change or proof seam is authorized. New child source comments carry defensible pinned-source breadcrumbs; existing Plan19 reconciles its manifest. No Plan18 native registration or frozen-guard modification is needed solely for this ordinary Cargo-discovered test child.17 retains independent semantic source review before any current guard renewal.20 retains full native/fresh pure-core coverage/Bazel and independent source/security/goal/lifecycle acceptance.

## Accounting and semantic invariants

Exact logical clone-sized undo bytes are:

```text
sizeof(BlockUndo)
+ each TxUndo: sizeof(TxUndo)
+ each restored input: sizeof(Coin) + restored script length
+ each overwrite batch: sizeof(BlockOverwriteUndo)
+ each overwritten entry: sizeof(OverwrittenCoin) + previous script length + new script length
```

All additions are checked and bounded by the existing384*1024*1024-byte ceiling. Keep the existing1_000_000 script-item ceiling and all prior resource controls. Logical length describes copied allocations; spare Vec capacity, allocator bookkeeping and RSS are outside the existing contract. Do not double count the outer Vec header inside BlockUndo, the entry Vec header inside BlockOverwriteUndo, or either inline Coin/output inside OverwrittenCoin. Equal previous/new script contents still require both independently owned script byte sequences to be charged. Every allocated batch/entry contributes even when its scripts or entries are empty.

Owned undo_bytes includes the complete sum exactly once; owned cloned_bytes remains body_bytes+undo_bytes+sizeof(ChainPosition). Borrowed undo_bytes agrees for the same undo and borrowed cloned_bytes stays zero. Both callers share add_undo_work and retain existing error conversion and absent-undo behavior. Existing body, witness-elision, transaction and restored-input accounting stays intact.

Overwrite scripts are resource bytes, not additional BASIC spent inputs/filter elements. Do not call count_script for them or alter script_items/script_bytes, BASIC inputs, generated filter content, integrity validation, BIP30/restoration semantics or accepted lineage. Accounting cannot normalize missing batches into an empty scan, validate an arbitrary batch as history or mint authority. Explicitly synthetic positive capture_reorg fixtures are structural/resource evidence only. No forged canonical identity or claimed real historical acceptance is permitted.

## Actual execution evidence required later

27 must first run independent exact-byte expectations through existing fact_work/borrowed_undo_work and retain actual nonzero omitted-allocation RED. Its final GREEN covers multiple entries/both scripts, exact empty/nonempty deltas, complete owned clone bytes, borrowed consistency, unchanged BASIC script contributions, positive small empty/nonempty captures and preservation of original batch representations. Existing phase157 byte/item/overflow tests remain and run with the whole local module filter.

Keep boundary tests resource-conscious: initialize the actual shared private accumulator near the unchanged ceiling or at overflow, pass small undo inputs and prove exact-limit acceptance/one-over/overflow refusal, including refusal caused by newly counted overwrite data. Do not allocate hundreds ofMiB, add cfg(test) budget overrides or raise production limits. Source review must trace both actual capture paths through the fallible reservation before clone_basic_body/full undo.clone; executable accumulator refusal and small positive capture evidence complement that ordering review. The summary distinguishes tested boundary behavior from reviewed clone ordering and claims no instrumented oversized capture test.

Executor runs focused phase167_undo_budget tests with nonzero discovery, existing chainstate::filter_index:: controls, strict node all-target/all-feature Clippy, scoped rustfmt and diff checks serially through pinned Bun1.3.9 scripts/command-timings.ts. Full exact commands live in27. Existing target/cooperative-lock ownership, complete failures/retries, liveness and polling contracts remain. No new timeout, skip, synthetic success, historical receipt rewrite or relaxed guard is permitted.

## Schedule and dependency ledger

Before:26plans/45tasks/22waves. After:27plans/46tasks/22waves. All original26 plan task blocks and automated commands remain byte-identical. Existing plan metadata remains byte-identical except Plan10's depends_on appends167-27. Its only body addition is one narrow context paragraph pointing here; its original tasks, generated_at, lifecycle, owners, requirements, truth/verification/acceptance and wave11 remain unchanged.

| Plan | Needs                                                                                                                                          | Creates                                                              | Wave | Checkpoint |
| ---- | ---------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ---- | ---------- |
| 27   | Fully accepted07 and08, including both08 focused commands and full unfiltered node --all-features suite; root independent amendment acceptance | Exact shared bounded full-undo accounting and focused local evidence | 10   | No         |
| 09   | Existing06/07/08 unchanged                                                                                                                     | Existing contextual network ingress obligations unchanged            | 10   | No         |
| 10   | Existing08/09 plus27                                                                                                                           | Both original managed connect/BASIC tasks unchanged                  | 11   | No         |

27 and09 have zero source-owner overlap. Their Cargo commands still serialize using the existing cooperative lock; same wave is not permission for overlapping targets. All other wave/dependency metadata is unchanged.27's08 prerequisite is complete whole-plan root acceptance; running output, elapsed estimates, a focused-only pass or partial suite cannot release execution. No active08 source owner/log/receipt is touched or moved. Existing02/26 acquisition and all accepted evidence remain frozen under their original contracts.

## Decision and requirement coverage

This table adds only this resource contribution; it does not replace or reduce the existing full phase decision/requirement coverage. All original ten decisions remain mapped to their original tasks. No partial approved decision or deferred idea is introduced.

| Decision | Plan/task                          | Coverage of amendment obligation | Notes                                                                                                   |
| -------- | ---------------------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------- |
| D-01     | Existing phase owners retained     | Full, unchanged                  | No selector/support/qualification change.                                                               |
| D-02     | Existing phase owners retained     | Full, unchanged                  | No subsidy/maturity/consensus parameter change.                                                         |
| D-03     | Existing phase owners retained     | Full, unchanged                  | No ancestry/deployment derivation change.                                                               |
| D-04     | Existing phase owners retained     | Full, unchanged                  | No script flags, interpreter or policy change.                                                          |
| D-05     | Existing phase owners retained     | Full, unchanged                  | No difficulty/time/history authority change.                                                            |
| D-06     | 27.1 plus existing07/08/16owners   | Full                             | Full prior/new Coin undo remains lossless and exactly budgeted; accounting never invents provenance.    |
| D-07     | Existing phase owners retained     | Full, unchanged                  | No canonical ingress or proof bypass added.                                                             |
| D-08     | 27.1 plus existing affected tasks  | Full                             | Existing preclone byte refusal and BASIC/accepted state semantics preserved; new contributions checked. |
| D-09     | 27.1 plus existing evidence owners | Full                             | Actual RED/GREEN required, resource evidence classified, all original tasks/logs/limits retained.       |
| D-10     | 27.1 plus existing17–20/25owners   | Full                             | Local nonzero/strict checks feed unchanged final native/review/lifecycle/root normal-hook saving gates. |

27's frontmatter CVLD-06/CVLD-03 describes a bounded supporting contribution to full-undo handling at the durable/BASIC boundary.27 alone does not implement or close all BIP30/durable-script behavior, and no requirement completion marker is edited. Phase168 daemon qualification,169 differential recovery,170 pruning and171 classified historical closure remain deferred.

## Immutable planning snapshots and change scope

The complete26 original plan bytes are copied under .local/open-bitcoin-dev/phase167/amend27/before/. before-manifest.json records SHA256/byte/task counts at2026-10-21T22:26:21.161Z. Plan10 before bytes=16017, SHA256=12a71f40c2419adb2cf297011d2366394bdf50c3e4ecdaea0dbb8697e6633830. The source snapshot filter_index.rs is also retained unchanged there. These are new ignored amendment receipts, not rewritten historical receipts or guard-renewal records.

Owned tracked planning outputs are new167-27-PLAN.md, this new167-27-UNDO-BUDGET-AMENDMENT.md and the bounded10dependency/context update. Existing167-DECISION-COVERAGE.md and historical scope amendments stay unchanged; this separately attributable record supplies the new coverage/schedule extension. Shared STATE/ROADMAP/REQUIREMENTS/config/todo and independent PLAN-CHECK remain root/checker-owned. No source/test/build, staging/commit/push, guard renewal, rollback or historical evidence mutation is part of this planner assignment.

## Guidance and artifact verification boundary

AGENTS.md local commands/ownership, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md and architecture/code-shape/testing/verification/Rust guidance materially informed the private shared accumulator, small behavioral tests, cohesive child and final gate preservation. Both distinct active lesson files were loaded completely; no archives, audit, new lesson or project skill dependency was needed. Existing current167 research and actual source interfaces suffice for Level0 internal discovery; no external library/API choice is made.

Planner validation results are recorded separately in the ignored amend27 directory. They verify artifacts, task preservation, exact bounded10change and DAG/ownership only. Root will request independent review before any source execution; planner checks cannot stand in for actual future RED/GREEN,08acceptance or full-phase gates.

## Actual planner checks completed

All27 frontmatter and XML structure validations pass;46tasks across22waves. Static checks find no missing/cyclic/later prerequisite, same-wave source overlap or unordered shared owner. Every existing original45task block and every original automated command remains byte-identical to the new before snapshots. All25existing plans other than10 retain complete original file bytes;10 exactly reverses to its snapshot by removing only the added167-27dependency and one context paragraph. Original lifecycle/generated_at values remain;27 uses the existing yolo lifecycle and its actual new generation time.

The sole GSD advisory is the existing frozen02 wave>1/empty-dependency producer metadata; it is not changed or treated as new release permission. New27 has one complete task and two declared owners, every task verify has an automated command, and the three changed/new planning outputs each contain only the top opening/closing frontmatter delimiters. The accounting source still matches its before snapshot. Full per-plan results and exact output hashes are in .local/open-bitcoin-dev/phase167/amend27/planner-validation.json, produced by the retained ignored validate-plans.ts. These are artifact checks only; no source/Cargo/Clippy/rustfmt/native behavioral execution or Git saving ran in this planner revision.
