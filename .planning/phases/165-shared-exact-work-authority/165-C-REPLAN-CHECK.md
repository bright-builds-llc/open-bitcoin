---
phase: 165-shared-exact-work-authority
generated_by: gsd-plan-checker
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
review_scope: proposed-C-correction-static-source-and-plan-review
status: issues_found
---

# Phase 165 C correction independent check

## ISSUES FOUND

**Checked:** new 165-25, revised 165-19, new 165-26 and the C integration note at the exact identities below. **Result:** one verification blocker and one scope warning. The proposed C1–C4 source design is feasible within the declared owners and preserves the original Plan 19 obligations. Correct the receipt expressions before implementation; the current documents cannot complete their required RED/GREEN verification as written.

This is static source and plan analysis. No application, Rust test/build, Bun test/checker matrix, native verification, Git operation or hook was executed. Lightweight GSD structure checks, Bash syntax checks, byte/hash calculations and regex checks against synthetic result-summary text were performed. Those checks are not behavioral RED/GREEN.

## Reviewed identities

| Document                                                                          | SHA-256                                                          | Tasks | Declared files | Wave/dependency           |
| --------------------------------------------------------------------------------- | ---------------------------------------------------------------- | ----- | -------------- | ------------------------- |
| 165-25-PLAN.md                                                                    | 91d4bf8b1d3bba3b00617f175b5f4b126ade3c54e3f9dead79b777ab6239f0ef | 3     | 7              | 18 / 165-18               |
| 165-19-PLAN.md                                                                    | 89ea8ae8e09927ec6b23bc41d6d052ffe62c75eaed3514029715fdddf5cab3ce | 3     | 13             | 19 / 165-25               |
| 165-26-PLAN.md                                                                    | 825d42955f4420f5af0e26a49568c77e9179e98e574a48553806c6eba1a327bf | 3     | 9              | 20 / 165-19               |
| 165-C-INTEGRATION-REPLAN.md                                                       | fb897b2def01b588c077b1b62ee28a5a6029f8cb4d9b0932414b05429fcc8bd9 | n/a   | n/a            | proposed integration note |
| Preserved original 19, .local/open-bitcoin-dev/phase165-plan19-before-c-replan.md | 5be51ad5f0a9ddfd49c7cdf1313c5744381412ecdebb0d2526d3bcc0eae929cd | 3     | 13             | historical planning basis |

All three GSD plan-structure results are valid with three complete files/action/verify/done tasks and zero structural errors/warnings. All nine extracted shell blocks pass `bash -n`. Syntax validation does not detect the receipt semantics blocker below.

## Blocker: receipt quantifiers were escaped into literal stars

Revised 19 Task 2 has four broken `rg` patterns, at reviewed lines 245, 248, 265 and 268. Plan 26 Task 3 has two more, at reviewed lines 263 and 266. In a single-quoted shell argument, `\*` reaches ripgrep as a literal star. It does not mean repetition. For example, `[1-9][0-9]\* pass` requires two digits followed by an actual `*`, and cannot match normal `12 pass` output. The daemon RED pattern also has `FAILED..\*` where the intended expression is `FAILED\..*`.

Each of these six exact patterns returned ripgrep exit 1 against ordinary synthetic summaries containing `test result: FAILED. 1 passed; 1 failed; ...`, `test result: ok. 12 passed; 0 failed; ...`, `12 pass`, `0 fail` and `1 fail`. Under the specified `set -euo pipefail`, genuine GREEN or genuine RED is therefore rejected. This is a runnable-verification defect, not missing metadata.

Replace the affected daemon Rust expressions with:

```bash
rg '^test result: FAILED\..* [1-9][0-9]* failed;' "$receipt"
rg '^test result: ok\. [1-9][0-9]* passed; 0 failed;' "$receipt"
```

Replace the affected Bun expressions with actual quantifiers, preserving the separate zero-failure check and real child exit checks:

```bash
rg '[1-9][0-9]* fail' "$receipt"
rg '[1-9][0-9]* pass' "$receipt"
rg '0 fail' "$receipt"
```

Recheck corrected receipt predicates against representative result lines, then save new plan/note identities. Plan 25 and revised 19 Task 1 already use the correct Rust quantifiers and need no corresponding change.

## Source feasibility and scope preservation

### C1: genuine public source and daemon attribution

The actual public `DurableSyncRuntime::from_live_fixture_source` has the seven declared arguments. The existing external Node fixture demonstrates all required public source preparation operations: empty concrete Fjall parent, consuming genuine empty transfer, `ManagedChainstate::from_chainstate`, actual managed body acceptance, `into_parts`, real `FlushLifecycle::execute_flush` with actual bodies/undo/positions and empty headers, and rewrapping the same core/sink. The constructor itself rechecks live origin/full parameters, independently replays retained bodies, authenticates physical content and binds its captured graph before normal finish/BASIC assembly. This supports the six-argument RPC test helper without Node cfg-test exports or count normalization. The plan correctly replaces the example's maturity-one parameters with the actual immutable maturity-100 recipe parameters.

The existing public modeled Version/Verack/Headers path is demonstrated in the external Node fixture. It supports new contiguous descendants after the captured genesis graph is bound. Neither the Node nor RPC helper needs a second root binding, a null-parent P2P header, raw header writing or a fabricated body event. The actual callers keep their body persistence/connect effects outside that header helper.

The current daemon source SHA is 933da5cfb57612c9e93a231967a17da3cc0163e1badcd5f7d44debde78868b18. Independently applying only the current adapter's two exact substitutions recovers the complete original FAA source SHA faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615. The plan's selected original seed span independently measures 1,095 bytes and hashes to b01011aa8b2703d2cc3fcb8f63963d9b291891e43bfbd23db2cd598e007b6f25. Those are real current-byte identities, not inferred future implementation hashes.

Revised 19 owns fixture-source.ts and the independent current 164 entrypoint pin, in addition to the original guard owners. Its complete include and prelude/loop inverse is implementable there, and the final whole-source FAA requirement detects an edited receipt, flags, recipe, loop, assertion, surrounding byte or partial wiring even if selected fragments still match. Retaining the current import/context branch and raw unproved return shape preserves both previous attribution and unknown-source refusal. The finalized new fragments cannot be pre-certified during planning; actual independent source review and exact inverse output remain execution conditions.

### C2: real future Node callers

The source confirms that `missing_history::extend` creates its larger recipe after the initial adapter returns, checks the immutable prefix, then saves/connects the suffix. Its header prerequisite belongs at that exact caller boundary. The revised plan supplies it without preloading future branches into `AcceptedRecipe::at`.

`publication_faults::saved_with_stale` actually creates and accepts alternate h102, indexes it, captures its real undo and saves that displaced mate before constructing canonical h102–104. Separate alternate and canonical header batches preserve that body order. Replacing the invented caller score with ZERO while asserting genuinely derived greater work retains the existing managed reorg authority boundary and the actual three-block replacement. All original physical fault, stale-row, safe-101/accepted-104 and raw snapshot obligations remain in the plan. No recovery implementation owner is silently added.

### C3: explicit historical function contracts and current authority

The incompatibility is real: current check162 feeds raw current text to `checkFunctions`, and immutable CONTRACTS require the old raw open/drop lifetimes for transport/RPC/daemon setup. Its claim view alone does not solve that structural requirement. Dummy calls or retaining dead old raw setup would misrepresent executable evidence; the explicit authenticated pre-C function view is a coherent alternative.

The captured bundle independently verifies as exactly 124,139 bytes with whole SHA 4f557ede3d82567c18c44ba7fc86c88afb9f83c00bfdb110ad26a15040bee981. Metadata is `captured_at=2026-10-17T05:20:52.045Z`, `git_base=d0104a997c0362bf65b81de1c8fec8ddc3012a1b` and `scope=pre-C workspace source; prospective legacy function-contract bridge only`. Every row's source bytes hash correctly and equal its current pre-C working file. This supports exact copying, not replacement of original Phase 162/163 receipts.

| Fixed source suffix / owner            | Bytes | Pre-C full SHA-256                                               |
| -------------------------------------- | ----- | ---------------------------------------------------------------- |
| Node filter_index/phase162.rs          | 14611 | 623b910dba79389972fcbaf4c22c69d6af827b1531035133a95b729fee384580 |
| Node peer_filter_transport/phase162.rs | 11654 | 3610463d888bb31d2319d435270bc59f71448c20d536f7bb0d242ac6f6cd7020 |
| Node phase162/missing_history.rs       | 11533 | 43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a |
| Node phase162/publication_faults.rs    | 18737 | 3001c1bad93739a7e84e1fc9aea1d43a0634c2eb9882bc0d5f8a5abcaaa6e813 |
| RPC peer_filters/phase162.rs           | 22182 | abe2651ff16c3d872695e8bde1222de63c2b5a28c519ffa9da299ac9166ab8b6 |
| Daemon phase162/fixtures.rs            | 15320 | 933da5cfb57612c9e93a231967a17da3cc0163e1badcd5f7d44debde78868b18 |
| Daemon phase162/inbound.rs             | 23886 | 1d991eeabfe62e1722828a9cdeb79fbf9ebc122b7a9e0fdb3b7d6ac0b85e7c60 |

Plan 26 owns every declared mutation needed for the bridge: existing pure fixture-source adapter; current 162 checker, fixed loader, test baselines and explicit synthetic API; current tooling inverse; 164 input/pins; and 165 input/data/adapter identities and executable controls. It introduces no extra TS module. The existing current 165 loader supplies bounded O_NOFOLLOW/fstat reads and a pre-read ordered inventory pin. Plan 26 explicitly stages the one data input through the existing fixed 164 inventory and proves stale-pin refusal before renewal; it owns the check162 loader that must gain bounded regular/no-follow handling for that new bundle. There is no need to let JSON drive reads or use eager imports.

The current tooling files independently hash to the already-pinned manifest `current_sha` values:

- check162.ts: caf9147ecd5b4ac189486bca0d2e81298ff0cfc0ffc218769a5f690a20ca8350 (404 lines).
- check162.test.ts: 784e29775202503f892ee7b005a5595f77ac8d0f8bdb42ca19818c138a86f6d5 (612 lines).

Thus the plan's exact finalized wiring inverse has a concrete preexisting target; it does not need manifest hash renewal. Final matching still needs execution proof. The proposed copied function view, original daemonFAA claim view and tooling snapshot view have different consumers and identities. Requiring exact reviewed current/pre-C full versions before normal function projection prevents changed current recipes/assertions/comments from borrowing old function authority. Keeping raw current module/native/oracle/prefix/claim checks independent avoids dropping targeted diagnostics after a projection error.

The explicit synthetic control API must apply the original structural predicates directly to deliberately mutated captured bytes, with a passing unmodified legacy baseline and the original owner/rule expectation. Normal version rejection alone is insufficient for those mutation controls. Plan 26 explicitly requires this distinction, original rows/counts and manual CF/dead-code/31-byte controls. In execution the manual Unix wrong-file registration test also needs that same audit when its selected first CONTRACTS file is one of the seven projected owners; its expected `Unix ownership/registration` finding must survive, not be replaced by an unrelated version error.

Actual factory inspection supports keeping both provenance factory files unchanged. `fixtures.ts` copies fixed FILES162 plus a finite 32-module relative import tree; adding the fixed bundle to CHECK_FILES carries it into hermetic map/filesystem/subprocess roots. `fixture-controls.ts` changes claims/layout, not removed raw-opener anchors. None of its four exact PROVENANCE_CLAIMS appears in the authenticated bundle, so its current JSON claim stripping does not change the bundle bytes. The native assertion mutation controls shown target unprojected retention.rs and retain their substantive predicates. A subsequently discovered unowned requirement still needs the explicit replan required by 26.

### C4: actual async ownership

The existing 165 counter matches only immediate `#[test]` before the `fn` region, while the owned inbound fixture uses actual Tokio async tests. Its named attribute mutation helper uses `lastIndexOf("#[test]")`, which cannot identify a Tokio declaration and can select an earlier test. The existing frozen 159 `attributesBefore` already removes `async` and other modifiers while preserving source offsets; modifying that frozen module for Tokio recognition is unnecessary.

Revised 19 correctly owns the local 165 counter/attribute controls and requires immediate attribute spans, supported Tokio options, preserved async modifiers and exact offsets, both ignore forms, cfg/file controls, a second named async test and real nonzero execution. Explicit local rejection of reasoned ignore remains necessary because 159's generic supportedAttributes does not reject `ignore=...` itself. The plan calls for that local handling and does not assume 159 supplies it. Literal mutation data may move only into the already-owned contracts file with its full ordered values preserved; executable test loops remain in the main test owner.

## Goal-backward coverage and remaining gates

| Dimension               | Result                                                                                                                                                                                                                                                            |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Requirement coverage    | CWOR-02/03/05 claimed by all three repair plans and concretely supported by genuine fixture/current guard work. CWOR-01 remains in immutable earlier arithmetic plans; no relevant original C requirement was dropped.                                            |
| Task completeness       | Structure passes. Receipt semantics blocker prevents approval of all runnable verification.                                                                                                                                                                       |
| Dependencies            | 18→25→19→26 is acyclic and waves 18/19/20 are consistent with 18's predecessor wave. Root must still shift 20–24/final14 as stated; no approval of stale downstream wiring is implied.                                                                            |
| Key links               | Public same-core source, actual future caller header ingress, helper inclusion, current digest pins and checkFunctions-only legacy view are explicitly wired in actions.                                                                                          |
| Scope                   | Three tasks each; 7/13/9 declared files. Revised 19 exceeds the ten-file warning threshold and combines cross-crate setup, exact attribution and async controls. Its existing-file/data-table strategy is plausible, but caps must be proven. No 15-file blocker. |
| Must-have derivation    | Truths concern actual recipe receipts, future branches, physical evidence and bounded attribution; artifacts/actions support them.                                                                                                                                |
| Context/scope reduction | D-01–D-12 retain concrete contributions/inherited owners. No invented v1 simplification, ordinary qualification, historical tie reconstruction, count repair, raw promotion or deferred 166–168 implementation.                                                   |
| Cross-plan contracts    | Public seven-argument constructor and six-argument helper match actual interfaces; all shared guard/adapter owners execute serially. Distinct legacy function/claim/tooling views avoid incompatible source transformations.                                      |
| AGENTS/standards        | Public/pure/shell boundaries, recognized test helper/breadcrumb, pinned Bun, timing-wrapped serial Cargo, source caps, root-owned default verify and normal hooks are retained.                                                                                   |
| Research resolution     | All four research Open Questions have explicit RESOLVED markers.                                                                                                                                                                                                  |
| Nyquist                 | Dimension 8: SKIPPED (nyquist_validation is explicitly false).                                                                                                                                                                                                    |

Original Plan 19's normal maturity-100 raw recipe, real returned genesis receipt, h101 spend/undo evidence, outbound 31-byte prefix/oracle/socket faults, RPC physical effects, raw cold observations and handle-free daemon handoff obligations remain in 25/19. Plan 26 adds honest legacy structural attribution without qualifying repaired current behavior from old reports. All six native registrations remain independently required by current guards and are outside this correction's mutation ownership.

After correction, any passing planning verdict remains conditional on actual 165-18 GREEN and sequential completed 25→19→26 execution with nonzero behavioral/source RED/GREEN, actual child exits, unchanged ignore status, fixed source/adapter/input snapshots and complete scoped matrices. Root alone finalizes after all 26 plans, full default `bash scripts/verify.sh`, coverage and independent source/security/formal gates with normal hooks. No Phase 166 execution or final current-native acceptance follows from this check.

## Structured issues

```yaml
issues:
  - plan: "165-19,165-26"
    dimension: task_completeness
    severity: blocker
    task: "165-19 Task 2; 165-26 Task 3"
    description: "Six required receipt rg predicates escape repetition stars as literal stars and cannot accept normal nonzero Rust/Bun RED or GREEN summaries."
    fix_hint: "Use FAILED\\..* and [1-9][0-9]* in the two daemon and four Bun predicates; check representative summary matches and refresh the plan/note identities before implementation."
  - plan: "165-19"
    dimension: scope_sanity
    severity: warning
    description: "Three tasks span thirteen owners and combine cross-crate fixtures, exact historical inverse and async ownership changes; this exceeds the ten-file warning threshold."
    fix_hint: "Retain the explicit bounded thirteen-owner scope, use the declared shared helper/data-only table move, prove <=628 lines and stop for replan if capacity or an unowned factory requires expansion."
```

## Guidance and review limits

Loaded AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md and applicable architecture/code-shape/verification/testing/local-guidance/Rust/TypeScript standards. Both active lesson files were completely loaded: 5,230 + 1,958 = 7,188 bytes; per-file conservative estimates 1,744 + 653 = 2,397 tokens. No lesson archives or project skill directories were loaded. The role's revision gate and planning calibration references informed this bounded independent review.

Read current CONTEXT, summaries 15–17, preserved original 19, all four proposed C documents and the relevant actual fixture/loader/checker/contracts/test/factory/attribution interfaces. Only this review file was created. Saved plan and source identities were checked directly; future repaired source hashes and actual GREEN receipts remain unclaimed.

## Recheck C revision | 2026-10-17T05:37:37Z

### VERIFICATION PASSED — conditional planning approval

This appended verdict applies to the corrected identities below. The initial `issues_found` frontmatter, original findings, warning and first-reviewed hashes above remain historical and unchanged. The receipt blocker is resolved. Root explicitly accepts revised 19's bounded thirteen-owner planning risk with the documented mitigations; this does not waive actual source review, verification or the 628-line caps. There are no outstanding planning blockers in this C correction.

| Current document            | SHA-256                                                          | Disposition                                                                                     |
| --------------------------- | ---------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| 165-25-PLAN.md              | 91d4bf8b1d3bba3b00617f175b5f4b126ade3c54e3f9dead79b777ab6239f0ef | Byte-identical to first review; 3 tasks / 7 owners                                              |
| 165-19-PLAN.md              | 87c0448c047af15e0f5df37c138e60e5a9f96fb75040c2150c96be8acabbec03 | Four receipt predicates corrected; explicit thirteen-owner risk discipline; 3 tasks / 13 owners |
| 165-26-PLAN.md              | 52cb5dfbc857ea76a511842adedabb03d1a97c9a07546ae17887fb7270ee325c | Two receipt predicates corrected; explicit manual Unix structural control; 3 tasks / 9 owners   |
| 165-C-INTEGRATION-REPLAN.md | a926d37b9e4c7a23429fbae11a84ce9c586956f1cd432b79737bb36f9b7e3f9e | Corrected identities and append-only first-review attribution                                   |

All six corrected `rg` expressions were extracted from the formatted saved plans and successfully matched retained actual execution log summaries, without running those tests again:

| Corrected predicates              | Existing actual log                      | Matched summary                                |
| --------------------------------- | ---------------------------------------- | ---------------------------------------------- |
| 19 Task 2 daemon RED              | phase165-plan17-red-0.log                | `test result: FAILED. 1 passed; 3 failed; ...` |
| 19 Task 2 daemon GREEN            | phase165-plan17-final-0.log              | `test result: ok. 4 passed; 0 failed; ...`     |
| 19 Task 2 and 26 Task 3 Bun RED   | phase165-plan16-claim-vocabulary-red.log | `2 fail`                                       |
| 19 Task 2 and 26 Task 3 Bun GREEN | phase165-plan17-full165.log              | `1230 pass`                                    |

These retained logs verify receipt-format compatibility only; their old outcomes are not current C RED/GREEN evidence. An additional independent sample check passed 32 positive matches across all receipt predicates in the three plans, including one- and multiple-digit counts, plus four explicit zero-count refusal checks. All nine extracted shell blocks pass `bash -n`. Current 19/26 GSD structure checks remain valid with three complete tasks and no errors/warnings. Scoped GFM/frontmatter formatter checks pass for all four current documents.

No hidden implementation scope change was found. Read-only reversal of only the specified four receipt changes, the new risk paragraph and the generated_at timestamp in current 19 exactly reconstructs first-review SHA 89ea8ae8e09927ec6b23bc41d6d052ffe62c75eaed3514029715fdddf5cab3ce. Reversal of only the two receipt changes, new Unix-control paragraph and generated_at timestamp in current 26 exactly reconstructs first-review SHA 825d42955f4420f5af0e26a49568c77e9179e98e574a48553806c6eba1a327bf. Thus actions, declared owners, interfaces, requirements, dependency chain, evidence limits and original retained obligations otherwise remain exactly as reviewed.

Plan 26 now explicitly seeds the manual `the exact Unix registration cannot normalize another source file` test with authenticated captured legacy sources, requires the unmodified baseline to pass the pure synthetic function-control API, prepends the actual registration to CONTRACTS[0]'s projected owner and requires that same owner's original `Unix ownership/registration` diagnostic. Unknown-version, missing-input or unrelated errors cannot satisfy this regression. Normal-current unknown-version refusal remains separately required, as do substantive CF receipt/dead-code/31-byte predicates.

Plan 19 now explicitly requires its shared helper to reduce the existing 627/628-line adapters, permits only the declared data-only table move for guard capacity, and forbids extra owners, cap exceptions, assertion weakening or blind digest renewal. Independent review of the final exact inverse and async declaration/attribute implementation must precede affected pin renewal. A concrete missing-owner or capacity conflict returns to root for bounded replan. Frozen 159 remains outside ownership, and the exact public-source/actual-receipt/raw-observation requirements remain intact.

Approval remains conditional on actual 165-18 GREEN and sequential 25→19→26 execution, each predecessor's actual completed evidence, nonzero executed RED/GREEN and stable scoped source/input hashes. Root must separately shift and check existing 20–24/final14 dependencies after 26; this recheck does not approve their stale graph. Root alone finalizes after all 26 plans, complete default native verification, coverage, independent source/security/formal gates and normal hooks. No Phase 166 work is authorized by this verdict.

```yaml
recheck:
  status: passed
  planning_blockers: []
  resolved:
    - "Initial receipt quantifier blocker: six corrected predicates match retained actual log summaries."
  accepted_planning_risks:
    - "165-19 thirteen-owner scope; root acceptance retains helper/cap discipline and independent inverse/async review before pin renewal."
  execution_conditions:
    - "Actual165-18 GREEN, then sequential completed25→19→26."
    - "Root-owned downstream graph shift/check and all final native/source/security/formal/normal-hook gates."
```

Only this recheck section was appended. No source, test, build, heavy Bun matrix, Git, hook or shared-state action was performed.

## Execution-evidence amendment recheck | 2026-10-17T05:55:15Z

### VERIFICATION PASSED — narrowed Task 2 scope

The evidence-only C2 amendment and exact one-sentence Plan 26 clarification pass independent recheck at the identities below. No new planning blocker was found. The earlier C2 assessment in this report overstated a separately delivered Headers prerequisite: the actual body-validation and reorg-authorization paths already validate and insert new headers. Those earlier findings and hashes remain preserved as historical review evidence; the corrected mechanism and retained actual results control this verdict.

| Current document            | SHA-256                                                          | Reviewed change                                                                                                                                                  |
| --------------------------- | ---------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 165-25-PLAN.md              | 5dfe17c0cfe9383f43a1336a7cf146723f8116406b2f76315538e841bed34622 | Task 2 becomes evidence-only; five modified owners and two explicit read-only callers; completed Task 1 evidence is distinguished from pending observer coverage |
| 165-19-PLAN.md              | 87c0448c047af15e0f5df37c138e60e5a9f96fb75040c2150c96be8acabbec03 | Byte-identical to the previous conditional approval                                                                                                              |
| 165-26-PLAN.md              | 4bfde0b76fc18383750010663633b2795467140999a30cafac0692180e7c765f | Exactly one authorized actual-owner/test-count sentence substitution                                                                                             |
| 165-C-INTEGRATION-REPLAN.md | d72f2f87a482c672e4a7680a83fa1d3a7f6a906680b35d0630d15b91d4644c34 | Append-only execution-disproved premise and subsequent count-reference clarification                                                                             |

Plan 25 now limits modification ownership to the initial Node recipe adapter, outbound adapter and three existing current165 guard files. missing_history.rs and publication_faults.rs are explicit `read_only_files`; Task 2 is an ordinary verification task with no TDD/RED demand, source mutation or test addition. Task 3 adds/renews only the two actual changed Rust owner/input paths, with real enabled-test counts. It expressly forbids registering unchanged callers, invented omitted-Headers mutation anchors and a forced ZERO substitution. Its current full Node phase162 and outbound execution plus strict/source checks remain required; retained focused receipts do not replace that final evidence.

Both caller files independently match their captured pre-C bytes exactly:

| Read-only caller      | Bytes | SHA-256                                                          |
| --------------------- | ----- | ---------------------------------------------------------------- |
| missing_history.rs    | 11533 | 43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a |
| publication_faults.rs | 18737 | 3001c1bad93739a7e84e1fc9aea1d43a0634c2eb9882bc0d5f8a5abcaaa6e813 |

The retained actual logs match all four ledger hashes. In particular, Task 2's files named `phase165-plan25-future-red-...` contain genuine GREEN results, not behavioral RED:

| Retained log suffix                                                    | Actual result                           | SHA-256                                                          |
| ---------------------------------------------------------------------- | --------------------------------------- | ---------------------------------------------------------------- |
| setup-green2-sync::tests::filter_index::phase162::phase162_recipe.log  | 3 passed / 0 failed / 0 ignored; 60.97s | 64c9fdaae4191387673790baf907e6e57b5987ea213648fe4b770091287d6c31 |
| setup-green2-sync::tests::peer_filter_transport::phase162.log          | 7 passed / 0 failed / 0 ignored; 7.57s  | 3926ce8490ae3f5e1b68da2c51575cb965de0d213d56c0bdee35f6e282c7181b |
| future-red-sync::tests::filter_index::phase162::missing_history.log    | 5 passed / 0 failed / 0 ignored; 11.15s | 7d470be4e40a61d3750429d97444bb878b48097966441547cfa625a9698239fd |
| future-red-sync::tests::filter_index::phase162::publication_faults.log | 8 passed / 0 failed / 0 ignored; 9.42s  | a67e08a15f56d9fc7444a6d25e868396f129d62e6cd8d5958fd73b9da07e09dc |

The exact missing-history timing JSON for run 5633ba8a-380f-4a9a-b79d-bb62849b859d records 2026-10-17T05:44:32.952Z–05:44:44.231Z, 11279ms, success and exitStatus 0. Its full file SHA is 6432a87b0a7e6cbd93a252ad8f096ea6066ce26e0d27d52257e702df19deeb99. The publication JSON for run e4395e5c-d841-41f8-889e-b4981cfa2368 records 05:44:50.210Z–05:44:59.754Z, 9544ms, success and exitStatus 0; its SHA is 4d515ddf67dcac51ce69a369330a821d82e5549ef46f23b3a3369c53e8d801d4. Both identify Rust 1.94.1 and the stated command-timings keys. I executed only the amended Task 2's read-only source/log hash and `rg` receipt contract; it completed exit 0 against these retained files. No Cargo/Bun test or build was started.

Read-only source inspection verifies the causal mechanism rather than inferring it from passing counts:

- `ManagedPeerNetwork::connect_local_block` guards the actual body, checks direct-connect authority, calls `retain_checked_candidate` with the selected full params and actual block time, then obtains connect authorization and performs its prepared commit.
- `retain_checked_candidate` checks accepted-parent proof and parameter agreement, then calls peer-manager body eligibility. `HeaderStore::check_body_eligibility_inner` checks the header context, selected-parameter header and contextual block, calls `insert_header_with_params`, then records the actual body linkage. Separate pre-delivered Headers is not required for these accepted-parent calls.
- `authorize_reorg` checks actual retained ancestor/disconnect identities and undo, requires contiguous replacement ancestry, derives each own proof from actual bits, calls `check_cached_body_eligibility` on its cloned header store and verifies the resulting exact accumulated work/facts before preference authorization. That cached path also inserts validated replacement headers. The original len+1000 numerical carrier is not read as authorization. The unchanged saved_with_stale caller still accepts/indexes alternate h102 and persists its real undo before saving/reorganizing canonical h102–104.

| Traced current source                 | SHA-256                                                          |
| ------------------------------------- | ---------------------------------------------------------------- |
| Node network/mempool_lifecycle.rs     | fa186eeafab8925c40006f7a20671c265af261a364698712b751562b6baa8bbb |
| Node network/candidate_eligibility.rs | 14e3334a388bdde9037fc1521a86e8a42d3ede428fc0fff8b4fe5be078b1ca1a |
| Node network/work_authority.rs        | a0952b43f0f4e6eb598421abfb6e6c2e8d01d30ebbb224ba80bd8c6fe8f71447 |
| Network header_store/eligibility.rs   | 751b989eec04f97e280a0eccc2dc234a6fa413bd0804e5a3d1e802b0f120cc9c |

The isolated test `phase162_recipe_isolated_genesis_raw_reopen_preserves_observation_without_authority` is present in the current recipe owner but absent from the retained 3/3 recipe log. It closes the genuine fixture, opens a raw observer, asserts original B/position/BASIC observations and unearned work, closes every raw handle and cleans up. This static inspection does not prove that new test passes. Its actual execution remains pending, and Task 1/3 explicitly require a later unchanged-source scoped receipt that includes it. The current recipe owner SHA is 3e644f38a58a3423046c62614924fa7f566e0f03b251a856f3e23d35c14e7a5c; outbound owner SHA is c35b11b189473843d2c34b95a75109c30bb064925019f8bf916c56fdf9241d39. The older 3/3/7/7/5/5/8/8 outcomes must not certify all current recipe-owner bytes.

Plan 26's new count sentence occurs once. Replacing only that complete sentence with the previously approved `Keep current fixture7 owner rows/test counts from25/19 unchanged and nonzero; no new Rust test ownership is invented.` reconstructs the whole previous SHA 52cb5dfbc857ea76a511842adedabb03d1a97c9a07546ae17887fb7270ee325c exactly. There is no other Plan 26 edit, metadata update, hidden owner, command change or read-set growth. Its seven exact snapshot rows and seven-path version validation remain intact. The captured JSON independently remains 124139 bytes, seven rows and SHA 4f557ede3d82567c18c44ba7fc86c88afb9f83c00bfdb110ad26a15040bee981. Unchanged caller current versions may equal their authenticated pre-C versions; this neither requires their registration as modified current owners nor changes normal unknown-version refusal.

Plan 25 structure validation passes with three complete tasks; all three extracted shell blocks pass `bash -n`, including the now read-only Task 2 contract. Scoped GFM/frontmatter formatting checks pass for current 25/26/ledger. Existing decision/requirement coverage, public source boundaries, immutable historical data, six native registrations and root-owned final gates remain as previously checked. Plan 19's accepted thirteen-owner risk is unchanged. This narrow amendment creates no later-phase acceptance or automatic repair authority.

```yaml
execution_evidence_recheck:
  status: passed
  blockers: []
  evidence_only_task: "165-25 Task 2: both callers remain exact pre-C bytes; retained actual 5/5 and 8/8 success/exit-zero receipts verified."
  pending:
    - "Actual current isolated raw-genesis observer execution; retained 3/3 recipe result excludes it."
    - "Remaining Plan25 source review/guard renewal and complete scoped verification, then sequential19/26 and root-owned final gates."
  evidence_limits: "Retained logs and static source trace inspected; no new Cargo/Bun test/build or source mutation performed."
```

Only this amendment recheck section was appended; prior review evidence remains unchanged. The latest PASS approves the narrowed execution-evidence contract and count clarification, not current C completion or final native acceptance.

## Mixed-provenance and native-order amendment recheck | 2026-10-17T07:20:05Z

### VERIFICATION PASSED — conditional Plan 26 approval

The amended Plan 26 resolves two real frozen-contract mismatches within the same nine owners and three tasks. No planning blocker was found. Approval covers the proposed mixed-provenance function bridge and narrowly projected historical command comparison; final generated data, exact tooling inverse, negative diagnostics, pin renewal and complete execution evidence remain mandatory before completion.

| Reviewed artifact                         | Full SHA-256                                                     |
| ----------------------------------------- | ---------------------------------------------------------------- |
| Current 165-26-PLAN.md                    | edca9b19186c072e89ad1f82a1611d007d8ab44052b4a8137545adf4fec663a1 |
| Current 165-C-INTEGRATION-REPLAN.md       | cba3db37af4efafbacb18a060911d781f5ce80be1ca3f194b733ce090fbeb403 |
| Ledger prefix before this amendment       | 13916b7622db5b38a8ed378d0618516b987d87afe0b93c184a795ea0224fbcbc |
| Completed 165-19-SUMMARY.md               | f3738aca433cb6ac61dbac222055a3e40f9f0bfe3f24a1b10ac0b00ea64b2277 |
| Retained pre-edit current162 RED log      | abe91cc3a29dc5d84d338c4c232e09204babdc511fa27d5a7e9b611f24750d24 |
| Retained stale pre-read inventory RED log | 23c1e64fed4753c2397625462c6823bedb6e76f5cd34fec20537cd874304a2da |

The preceding approved full Plan 26 document was not saved before amendment. I do not claim an exact whole-document inverse/diff proof against its recorded 4bfde0b76fc18383750010663633b2795467140999a30cafac0692180e7c765f hash. Instead, this recheck compares the full amended actions, ownership and evidence limits substantively against the earlier report and actual current interfaces. The separate authentic implementation before-source tree does exist: its check162.ts and check162.test.ts independently hash to CAF9147... and 784e297... below. That supports later exact implementation inversion and does not recreate the missing old plan document.

### Authentic eighth source and bounded projection

The original seven-source ignored capture remains exactly 124139 bytes/SHA 4f557ede3d82567c18c44ba7fc86c88afb9f83c00bfdb110ad26a15040bee981. The incomplete tracked seven-row copy currently matches those exact bytes; it is explicitly partial work, not the final amended data format. The current refusal-only exports and one-path 164 input addition likewise are unfinished implementation, not a weaker accepted design.

The separate ignored header capture independently verifies as 22002 bytes/SHA 2f15d035bfc1e57bc9ef2a9e93f69607d8c46b24ba441938ae5236b24a9bf783. Its metadata truthfully identifies a pre-phase Git structural baseline, not an eighth pre-C snapshot or runtime result. Its embedded parent is exactly 20234 bytes/617 lines/SHA 831e8741d8e4f7d1a746a85a5f359b9b18e00ff96a069d832a4a6517715185cb. Computing the Git blob object identity from those source bytes and the normal `blob <length>\0` prefix independently yields 31dc52b18c51ee4a03e0aba0412443111fd153d7, matching the recorded base-commit capture. No Git command was run for this check.

The legacy parent actually contains `stale_runtime_configured` and all five exact assertion contracts plus the original `for accepted in [&genesis, &old]` fact. Current source has moved that helper into the child. Both current source hashes match the fixed reviewed pair:

| Existing current owner                               | Full SHA-256                                                     |
| ---------------------------------------------------- | ---------------------------------------------------------------- |
| sync/runtime_state/tests/header_checkpoint.rs        | e449aeed8ae0a65e23e7e654a20cccd4cc8609f982e2008b48aa372dbb777e8d |
| sync/runtime_state/tests/header_checkpoint/faults.rs | 418404faac133e33635924f4881458a73bc9402f50df1d414dfd571ed12373ac |

The completed Plan 18 summary records 12/12 for the checkpoint subtree, and its final source-review supplement records this same pair. The actual pre-edit current162 RED contains the moved parent's ordered-function, exact-assertion and protocol/fact failures. Frozen contracts already select both parent and child. Thus mapping the authentic entire old parent, after authenticating both exact current files, closes a real historical selector mismatch without inventing raw calls in current code or redirecting old assertions to incompatible current helper text.

The new fixed JSON schema keeps the first seven source records and their original metadata/order intact, adds the authentic Git parent as an eighth record and records separate pre_c_capture/base_git_capture provenance. Whole new bundle bytes/SHA authenticate before parsing; fixed code validates metadata, order, source lengths/hashes and the current pair. The new bundle identity is deliberately unknown until generation and independent review. Original 4f557... and 2f15d035... captures remain separately immutable. There is no authorized inline 20KB Rust string, guessed future digest, recaptured current-as-legacy source, dynamic JSON path discovery or ignored-file/Git read at runtime.

The normal function view requires the exact current parent and child (or the explicitly admitted authentic legacy parent with that same current child for synthetic-baseline input), projects only the eight fixed target keys and leaves the child raw/current. Missing or changed child cannot borrow a known parent. Tests must construct the passing authentic eight-source baseline before any old selector/assertion/fact extraction; row174's five assertions/fact and child175–178 predicates must fail through their actual structural diagnostics, never generic version/input rejection.

Both header files already belong to frozen ROOTS/CHECK_FILES and current165 source/test ownership. No additional header read/owner registration is needed. Only the one fixed bridge JSON path grows current162/current164/current165 inventories. The current partial 164 INPUT_FILES line appends exactly LEGACY_CONTRACT_BUNDLE, and the plan retains ordered-delta/stale-pin/missing/extra/reordered/redirected controls. The one pre-read inventory pin may be staged only after separate approval of that exact delta; source/data/adapter pins still wait for independent final data/inverse review.

### Historical order projection preserves current native diagnostics

Static reconstruction independently confirms current verify SHA e1823a94c0394107ecdfbb194b32d93f290cc7b79195dac5583dd7cc7a811fb7 has each of the existing six additions exactly once. Applying the existing helper's four exact anchored groups yields complete script SHA 2925344e06e3604fbba1821e49a00b0f2f292448eae205107882d0ed65aacc43. Each raw command section has exactly these three additions relative to its projected section: Phase68's test, Phase165's test and Phase165's checker. No original native command is absent. All four raw Phase161/162 commands and the shared-format command occur exactly once in each section. After projection and the existing frozen filtering, both resulting historical arrays equal HISTORICAL_COMMANDS exactly.

This supports applying verificationHistoricalBase only to the complete frozen HISTORICAL_COMMANDS comparison. Exact-once/contiguous raw Phase161/162 checks, raw shared-format checks, raw mandatory Cargo/Bazel/benchmark/coverage checks and raw module/oracle/claim checks must continue independently, including when projection refuses. No whole projected checkNative call or broad command filtering is authorized. The positive full script hash is attribution evidence, not an early gate that can suppress targeted negative findings.

The amended plan explicitly requires passing baselines and original diagnostics for old command removal/duplication/replacement/reordering, shared-format and mandatory-gate mutations. Historical-only controls may start from the explicitly projected original native script before mutation so inverse-anchor errors cannot replace the intended old predicate. Current missing/duplicate/reordered/partial/commented additions still refuse, unknown extra commands still fail complete order, and current164/current165 independently retain all-six-removal enforcement. The existing original-script/no-additions behavior remains historical-only. These are meaningful negative contracts rather than permission to credit unrelated input/version errors.

### Exact implementation ownership and remaining proof

The nine owners, wave20/depends_on165-19, CWOR-02/03/05 contributions, lifecycle 165-2026-10-14T16-09-59Z and three complete tasks are unchanged in scope. The pure existing historical-tooling module imports crypto and has no dependency on check162; directly importing it from check162 introduces no reverse runtime edge or new CLI module. The explicit fourteen-module current CLI inventory remains fourteen. Runtime snapshot/manifest reads stay lazy, fixed, bounded and authenticated.

Both actual current working checker/test files and the saved implementation before-source copies independently retain these immutable destination identities:

- check162.ts: caf9147ecd5b4ac189486bca0d2e81298ff0cfc0ffc218769a5f690a20ca8350.
- check162.test.ts: 784e29775202503f892ee7b005a5595f77ac8d0f8bdb42ca19818c138a86f6d5.

The exact finalized inverse must cover eight-source and synthetic-baseline wiring plus the new native import/section comparison, then recover those whole files before any affected current identity pin renewal. The original manifest current_sha/original_sha, snapshots, receipt/recipe bytes and rawCurrentDigest semantics remain immutable. Extra or partial inverse wiring cannot be approved merely by a source hash allowlist.

The proposed capacity relief is concrete and bounded: current165 main.test.ts is 606 lines and contracts.ts is 300; data-only literal-table movement within those already-owned files must prove complete ordered values and generated test names/counts/expectations unchanged, with all executable tests/loops retained in the main test file. Current162 test wiring starts at 612 lines and must fit 628 or return a concrete replan. This review does not pre-authorize a helper/module, hidden factory edit, cap exception, deleted assertion or broad digest renewal if execution finds that constraint difficult.

All three prospective shell blocks pass `bash -n`; GSD structure validation passes with three complete tasks and no errors/warnings. The saved regex predicates have nine successful static representative-output matches, including nonzero historical/current Bun summaries; no prospective Bun command was executed. Scoped GFM/frontmatter checks pass for the amended plan and ledger. The new STRIDE dispositions cover provenance/source spoofing, metadata/order tampering, exact inverse attribution, synthetic/current diagnostic separation, bounded imports and caps. Final mixed-data/parser/pair/native/inverse negative controls and complete162/163native/current164/current165 matrices remain execution obligations.

```yaml
mixed_provenance_recheck:
  status: passed
  planning_blockers: []
  approved_scope: "Same9 owners/3 tasks; eight authentic legacy function targets with separately authenticated current child dependency; historical complete-command comparison only."
  conditions:
    - "Independent final mixed8 data and exact CAF914/784 tooling inverse review before source/data/adapter pins."
    - "Separate reviewed one-path pre-read inventory staging; targeted negatives must reach their actual predicates."
    - "<=628 caps, truthful provenance, existing14-module boundary and all-six current registrations."
    - "Actual nonzero scoped matrices/stable hashes and all root-owned final gates."
  limitations: "No saved full prior plan source; no exact before/after plan proof claimed. No new source/test/build/heavy Bun/Git operation."
```

Only this verdict section was appended. The prior complete review prefix is retained; worker implementation may resume only under root release and these explicit conditions. PASS does not mark the partial refusal stubs, final data/inverse, C integration or native acceptance complete.
