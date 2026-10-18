---
phase: 165-shared-exact-work-authority
generated_by: gsd-plan-checker
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
status: passed
scope: bounded-current158-plan27-and-plan26-resume
---

# Current158 compatibility plan check

## VERIFICATION PASSED

**Conditional planning PASS:** Plan 27's eight-tool repair and Plan 26's Task 3 resumption address the actual current158 mismatch without projecting production Rust, rewriting historical qualification or waiving the failed163-native gate. No planning blocker or unowned implementation dependency was found. The complete 27-plan schedule remains a separate root-owned graph check before release.

This review is static. It inspected existing source, artifacts, logs and byte identities, and ran lightweight plan-structure, shell-syntax, receipt-format and scoped Markdown checks. It did not run Cargo/Bun tests or matrices, modify source, operate Git/hooks, change shared state or release an executor. Only this report was created.

## Authenticated planning and checkpoint inputs

| Artifact                                                  | Full SHA-256                                                     |
| --------------------------------------------------------- | ---------------------------------------------------------------- |
| 165-27-PLAN.md                                            | 3b211a49732cba8d8f15c9ca75e925c262b58b830ff5ef2e9cdfc9b5b54adf38 |
| Resumed 165-26-PLAN.md                                    | ce3bdd8d6bba635b6daec3506daaa563fa0684b598cf1b07946c1ffb63ae2304 |
| 165-C-INTEGRATION-REPLAN.md                               | a8f767bdc3cabe84048f41193c5e1c644ccd0294b6c5144bc7a7561bf3404efd |
| Full before-plan archive, phase165-before-158-replan.json | f663fbe2b7ccf730f0d1800f3481804560facd603a8acb3966aae4470784c049 |
| Eight before-tool sources, phase165-pre-27-tooling.json   | 8dc55a3e8565fc0b8da29a948e03e24bfc20457cec441b19478a2b29cd5dcc3f |
| Phase26 source-review packet                              | d04dd21b27154a985136338dbf0e6878cea042d6c3dd190ed6bf4aa3e8a08c7d |
| Actual failed163-native log                               | 148f0c0cb471def513055a325bda67da3ed900e06f43aa3d1e2d6ac4084405c9 |
| Post-run158 discovery                                     | 023d9378dc4c130f7995d3004bbe2f78dd3a7ff2a3854ec677afb60a0d5a37ea |
| Incomplete310-entry prior freeze                          | b468471f08887f7c2212abe4e8324de9a974bb67237cb0e15bd70bae166b4dc0 |

Every one of the eight captured pre27 source rows independently has its declared byte count and full source SHA and is byte-identical to the corresponding working file. The actual nine-file post-pin checkpoint digest independently computes to cb782f42290d465f79b47f32189fa1dfcb6d6aa64bb82595af5f4b2fcf141c8c using the documented packet order and `path<TAB>fullSHA<newline>` construction. This is an actual preserved execution checkpoint, not a fabricated Plan 26 completion summary.

The full before-plan archive contains authentic original Plan 26 SHA edca9b19186c072e89ad1f82a1611d007d8ab44052b4a8137545adf4fec663a1. Extracted XML Task 1 and Task 2 blocks in resumed26 are byte-identical to that source:

- Task 1 block SHA: a970b969834b489c33de4555a119949571d4b2ed441d22c3d2740676d0aa99b5.
- Task 2 block SHA: 2b5c32d445f42957871a34348a696c757b923731f27cdac5994b67fac8243fd2.

Plan 27 remains wave20/depends completed19 plus the explicit authenticated26 Tasks1/2 checkpoint. Resumed26 is wave21/depends27 with resume_from_task 3. Plan 27 requires no nonexistent26 SUMMARY. The future27 SUMMARY reference in resumed26 is explicitly a prerequisite only after actual27 completion. There is no cycle in these two plans: completed26 implementation is historical checkpoint input to27, while future26 closure waits for27. This does not approve unshifted20–24/final14 or the eventual full schedule.

## Actual failure and source feasibility

The retained163-native log reports 140 pass / 20 fail / 1510 expectations and 160 tests, with its original 267.05s printed duration. The plan preserves the separately reported 267.057s/exit1 attribution. The discovery artifact explicitly identifies itself as post-run discovery and lists the same ten raw/historical158 findings: four function boundaries, four obsolete TESTS selectors and two sealed-field locations. It records 33 omitted FILES158 paths. Stable hashes for the old310 subset cannot certify the uncovered163 boundary; neither plan claims they do.

Current158 evaluates raw Rust through its semantic checks. historicalToolingView affects tooling/provenance qualification, not that Rust reader. Therefore the new plan correctly repairs the actual-current semantic predicates instead of adding another historical Rust view or regenerating receipt hashes.

The proposed replacement boundaries match the actual source:

- `absorb_staged_reorg_with_receipt` delegates to `validate_staged_reorg(staged)?.absorb()`. Validation invokes `check_staged_reorg` before creating the exclusively borrowed capability. Its actual generic reorg impl's consuming `absorb(self)` invokes `owner.absorb_validated_reorg(staged)`; a different same-name connect absorb method exists and must not be selected accidentally.
- `check_staged_reorg` calls the WorkAuthority owner/allowed-preview/generation-tip check. Owner identity precedes allowed matching preview; otherwise generation and source-tip identity are required. The private effect callee installs overlay/active chain/undo/counts, accepts the binding, then creates the accepted receipt.
- Network preparation takes ReorgAuthorization and calls `prepare_authorized_reorg`. Node's old unqualified prepare returns PreferenceAuthorizationRequired. Authorized preparation checks before staging and checks the stage before validation-history/BASIC preparation. Commit checks authorization, acceptance availability, stage and BASIC before validated absorption, candidate binding and subsequent publication/persistence/error handling.

Checking these guard/effect callees and order is materially stronger than merely changing an obsolete wrapper anchor. The plan specifies unique actual owner selection, matching file/symbol failures, changed present anchors and passing baselines for guard deletion/reordering tests. The same-file pure semantic-core extraction permits genuine source RED/GREEN while historical tooling attribution is still pending; normal158 then calls that same core. It creates no alternate runtime or history bypass.

Actual `StagedChainstateReorg` and `AcceptedChainstateReorg` definitions are in engine/stage.rs with the eight and three exact fields declared by the plan, all `pub(super)` within the engine parent. ValidatedChainstateReorg owner/staged fields are private. engine.rs registers stage/work_authority and reexports the public types rather than defining obsolete private fields there. The plan binds exact location, complete field set, enclosing module and restricted scope, keeps engine.rs as a required input and preserves the other private proof policies and precise BasicFilterAppendProof.identity exception. Legitimate pub(super) becomes an explicit positive; broader/missing/extra/duplicate/moved declarations remain meaningful negatives with edit() asserting changed bytes. No dummy production field or broad sealed-type exemption is needed.

All four replacement behavior names exist in the stated actual files. The weaker-shorter case asserts BranchNotPreferred and unchanged state/index/checkpoint observations. The explicit disconnect wrappers call the existing assertive helper. The stronger unflushed-return case preserves its actual durable coins fence/retained undo and extended return behavior. Fresh timing-wrapped execution of all four with exactly one passed case each is still required; this inspection does not claim those new receipts. Parent `mod manual_lifecycle`, core stage/work_authority and Node work_authority registrations exist at the stated owners. Each newly required current dependency has exactly one breadcrumb manifest group and the corresponding comments. The plan requires targeted absent/commented/cfg-disabled/redirected module/file/test and helper-assertion controls without editing those Rust files.

## Historical evidence and exact tooling layers

Current `checkParity` derives its evidence iteration from DOC, unique CONTRACTS files, unique TESTS files, UAT/measurements and the two guard paths. Independently reconstructing that original ordered iteration yields 35 entries, with compact JSON SHA 22c43450a9a707ae80da237a5cb1922d0da84c0159366a51d261ad75f63e1a7f. Every required original path is present in the current catalog. That identity is a static review reference; the executor must independently prove its final fixed HISTORICAL_PARITY_EVIDENCE has the complete original values/order and preserves every original removal control.

The plan explicitly keeps that historical requirement independent from the catalog under test and from the newly added semantic dependencies. It forbids filtering absent paths or deriving expected evidence from the owner row. New work-authority/manual-lifecycle files remain mandatory fixed inputs with raw semantic, ownership, module and breadcrumb checks even though the old done CFIX receipt did not qualify them. No catalog edit or expanded old receipt is needed; root14 may later document current165 sources separately. This preserves old requirements while avoiding a misleading qualification claim for new files.

The immutable working158 destinations exist at the captured before sources: checker385b6c6d62067ba13e8e4d29d0ea1005252614830430f800b0824ec7ba954fdd and testbebdd0c7f2cc6e787f34502bb82f6ae6fd27c7a5ce5ac9bdd14be73c054193a3. Exact finalized contextual inverses back to these complete bytes are feasible in the existing crypto-only adapter. They must include actual contract/test/field/registration/evidence-set changes; raw original assertions or unknown appended bytes cannot borrow attribution. No source hash replaces semantic guard evaluation.

Plan 27 routes only the two158 fingerprint operands in162 through that exact recovery, retaining the same385b/bebdd constants and file-specific frozen diagnostics. The163-provenance fingerprint remains raw. Existing162 tests cover arbitrary raw tool tampering; the plan adds passing positives and selected-owner tampering controls for both158 tools and raw provenance. Projection/version errors elsewhere cannot satisfy those frozen-rule tests.

I independently applied the existing contextual26 layer data to the current checkpoint sources as a static exact replacement calculation. All current fragments occur uniquely; the eight checker blocks recover CAF9147ecd5b4ac189486bca0d2e81298ff0cfc0ffc218769a5f690a20ca8350, and the fourteen test blocks recover784e29775202503f892ee7b005a5595f77ac8d0f8bdb42ca19818c138a86f6d5. Plan 27 preserves those blocks and adds a separate exact outer layer returning875460713003992a2701dc1f5ea300430eb2991b30e3635fefedb8bf387ce877 /3ab730fcb1998459f410dfb3a60bb580d0fe67e8e47c8c76835080ad2062acac before the existing layers run. Unique complete context and whole intermediate/final hashes are required at each stage; any failed/mixed/unrelated layer returns the original raw candidate. No circular import, loose source allowlist, original manifest renewal or replacement snapshot is authorized.

The mixed8 bundle remains1330515f81cb39108b7605814f50ed4c1f1572a0280b8415902085791f9eab5c and fixture-source remainsb92d2d91da1dd7715e7de37737769f30bd543cb2fd8a7a1eecb92aaf3335f3e3. Both plans preserve that data and all original receipt/manifest/recipe identities, the rawCurrentDigest distinction and all six native registrations. The existing14-module current CLI topology gains no new module or inverse runtime dependency. Only the reviewed historical-tooling digest is renewed in the two current guards after mandatory independent review. The current165241-path inventory/pin must stay unchanged because it does not derive from FILES158.

## Complete rerun closure and bounded ownership

The existing163 factory explicitly reads FILES158/FILES162, lazy optional provenance/evidence sources, its own liveFiles inventory and both entrypoints' finite relative-import tree. A complete source checkpoint must also cover the exercised tests/checkers/support modules and manifest/snapshots. Both new27 and resumed26 require the exact union, order, canonical paths, hashes and optional presence states before full runs, prove every actual fixed read is covered, and recheck every entry afterward. This explicitly includes all33 old omissions and new27 dependencies. It distinguishes original raw inputs from synthetic layout/mutation outputs and rejects unknown reads or drift. The old310 map/post-run discovery cannot be reused as full163 proof.

Plan 27 owns exactly eight tools and no Rust, data, catalog,163 factory/provenance/native test or shared state. The three fixed current dependencies are reachable through expanded FILES158 and the factory's existing fixed copying; no hidden factory owner was found. Its625-line158 test file can share duplicated sealed selection/mutation setup in the existing two owners; current165's621-line main test can move complete literal data into its already-owned contracts with ordered/name/count/expectation proof. These are plausible bounded refactors, not completed capacity evidence. No dropped cases, changed-source assertion removal, inline cap trick, helper/module owner or exception is approved if execution cannot fit628 lines.

| Check dimension              | Result                                                                                                                                                                                                                                                            |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Requirements/decisions       | CWOR-02/03/05 and D-01–D-12 have concrete repair contributions or unchanged earlier owners; immutable arithmetic is not duplicated and166–168 remain deferred.                                                                                                    |
| Task completeness            | Both plans have three structurally complete tasks. New27 actions specify actual semantic/field/test/inverse/freeze obligations; resumed26 is explicit Task3-only.                                                                                                 |
| Dependencies                 | 27 depends completed19 plus authentic26 checkpoint; future26 closure depends27. No local cycle/nonexistent26 summary. Full schedule approval remains separate.                                                                                                    |
| Key links                    | Actual guarded absorption/preparation/commit, current tests/registration, historical evidence list and two-only fingerprint mapping are wired explicitly.                                                                                                         |
| Scope                        | Eight27/nine26 declared owners; three tasks each. Tight test caps have explicit bounded refactoring and replan conditions.                                                                                                                                        |
| Verification                 | Six shell blocks parse. Rust/Bun receipt patterns accept representative nonzero outputs; exact ten-finding RED count matches discovery. Actual source RED/GREEN, fresh Rust and complete matrices remain prospective.                                             |
| Historical/current contracts | Original CFIX evidence values stay fixed; new current dependencies remain mandatory. Exact layered tool recovery retains immutable destinations and raw provenance.                                                                                               |
| Evidence integrity           | Failed140/20 and33-omission discovery remain failed/incomplete evidence; complete pre-run union is mandatory for replacement receipts.                                                                                                                            |
| Context/security             | No scope reduction, production Rust projection, caller-score authority restoration, raw promotion, qualification expansion or166 execution. STRIDE dispositions cover semantic authority, sealed ownership, attribution, fingerprint inputs and full freeze/caps. |
| Research/Nyquist             | Existing research questions are resolved; Dimension8 SKIPPED because nyquist_validation is explicitly false.                                                                                                                                                      |

GSD structure validation passes for both plans with zero structural errors/warnings. All six extracted shell blocks pass `bash -n`; scoped GFM/frontmatter checks pass for both plans and the ledger. These checks do not execute the prospective commands. Initial/current source predicate and attribution matrices, four fresh Rust cases, complete158/162/163-native/164/165 gates, stable full-union closure and independent source/inverse review remain required before completion or pin renewal.

```yaml
issues: []
verdict:
  status: passed
  scope: "Bounded Plan27 and resumed26 Task3 contract only."
  conditions:
    - "Root separately shifts/checks the complete27-plan graph before implementation release."
    - "Authenticate26 checkpoint/pre27 sources; preserve26 Tasks1/2 and mixed8/old inverse layers."
    - "Independent semantic/field/module/evidence-list/cap/exact inverse review before affected digest renewal."
    - "Four fresh real Rust cases and complete stable pre-run factory/input/module-union matrices."
    - "Root-owned final native/coverage/source/security/formal/normal-hook/Git gates; no166 work."
```

Local AGENTS, Bright Builds sidecar, placeholder-only overrides and applicable architecture/code-shape/testing/verification/Rust/TypeScript guidance informed this review. The active lesson sources were fully loaded earlier in this continuous check:7188bytes/2397 conservative tokens, no archives. No project skills were present. Only this new report was written; the prior C check and all plan/source/shared-state files were left unchanged.

## Completed-receipt reuse approval | 2026-10-17T09:21:14Z

**APPROVED:** current Plan 26 SHA 55b38732385d1dae9907651b1fc1b3ba90a52feb2f88d02ebcd3689577fd1c6e adds only the previously recommended completed-receipt gate in Task 3 and the verification note giving that authenticated gate precedence over duplicate execution. The rest of the plan, including source scope, commands, original failure/omission evidence and final obligations, is unchanged. No source or gate weakening was found.

The authentic before-plan artifact `.local/open-bitcoin-dev/phase165-p26-before-receipt-reuse.json` has SHA c96a1c386bf69d6b11a111c36b617d3f1c81b401b8bb2bdd69cb71f8a6a0c071. Its embedded complete source independently hashes to the recorded old Plan 26 identity ce3bdd8d6bba635b6daec3506daaa563fa0684b598cf1b07946c1ffb63ae2304. Direct before/current comparison shows exactly the two text additions and their separating blank lines; no existing paragraph, shell command, task, dependency, requirement, owner or lifecycle field changed.

The three immutable evidence references in the new gate independently match their actual artifact bytes:

| Evidence artifact under .local/open-bitcoin-dev/ | Full SHA-256                                                     |
| ------------------------------------------------ | ---------------------------------------------------------------- |
| phase165-p27-matrix-results.json                 | 4b53898b90959263a046d7e777c98378e01f737e51aa1d6df272a442197b6171 |
| phase165-p27-full-freeze.json                    | 0ac0cd250c299b1e024a9d54b3fb2fa3d0b6143140a09125630348ae02f4be45 |
| phase165-p27-full-freeze-closed.json             | 76bc1218952ccdb0ae6cf9363eab35667221bf134cea4d4afd4feeb20b3ed650 |

The completed results cover all five required full suites, exact checker commands and policy checks: 158: 900/0, 162: 2620/0, 163-native: 160/0, 164: 868/0 and 165: 1526/0, with all 13 recorded command exits zero. The complete 165 log includes 151 passing `phase165 C legacy bridge` cases, so its focused subset is already covered. The closed proof records 465 entries, 463 present and two optional absences, all unchanged after the full run. Root has again authenticated all 465 current hashes/presence states. Reusing these exact fresh runs closes the identical scoped verification requirement without another full or focused invocation.

The metadata drift is preserved honestly. Root reports that an early ROADMAP progress-counter update from 19 to 20 briefly changed one frozen document after the completed run; no test ran during that interval. Root restored the exact frozen ROADMAP bytes before receipt reuse and rechecked the whole 465 boundary. This review independently confirms `.planning/ROADMAP.md` is again 19143 bytes/SHA baf41838b904d108f93ce5055fff43750575d5d909b1f2694f1b293648b1dbfd, exactly its frozen entry. This is restoration to the tested inputs, not a claim that no post-run metadata drift ever occurred. STATE and task-tracker updates are outside the 465 runtime boundary; this review changed none of them. Leave the frozen ROADMAP unchanged through Plan 26 summary closure.

The original Plan 26 140/20 failed163-native receipt, incomplete310-entry freeze and post-run33-omission discovery remain retained. The new160/0 receipt supersedes their unmet verification requirement without rewriting their outcomes. Record all reused runs as Plan 27 execution evidence consumed by Plan 26; do not relabel them as newly executed26 tests or infer ordinary/canonical/daemon qualification.

No repeat of already-passed checks is required without changed inputs, an uncovered requirement or a concrete new concern. Remaining work is to reconcile the existing independent reviews/approved overlap deltas, record Plan 26's completion attribution and residual risks, check newly created completion documents as appropriate, and preserve later plans plus root-owned default-native/coverage/source/security/formal/normal-hook gates. Any subsequent boundary change invalidates only the affected reuse and requires the corresponding reviewed rerun. This approval does not waive those gates or approve Phase166 work.

Only this timestamped approval was appended. No tests, source, Git, hooks or shared state were changed; the original report prefix is retained.
