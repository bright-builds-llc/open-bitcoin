---
phase: 165-shared-exact-work-authority
plans_checked: ["36", "37"]
generated_by: gsd-plan-checker
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
checked_at: "2026-10-17T22:42:55Z"
status: issues_found
blockers: 0
warnings: 1
parent_disposition: accepted-bounded-scope-warning
---

# Independent Plan 36–37 Check

## ISSUES FOUND

No requirement, source-contract, wiring, dependency or preservation blocker was found. One scope warning remains for Plan36's eleven modified owners. This is a static plan review, not execution acceptance or a successful native verification receipt.

Parent disposition after this review: Root explicitly accepted the nonblocking eleven-owner scope risk within the authorized yolo workflow, citing the4/5/2 owner split, source-before-pins review and readable physical caps. No human approval is required. The review's warning remains recorded rather than being silently removed; the parent may proceed with36 and then37 once their stated predecessor gates are accepted.

| Plan   | Tasks | Modified owners | Wave | Dependency | Structure            |
| ------ | ----- | --------------- | ---- | ---------- | -------------------- |
| 165-36 | 3     | 11              | 35   | 165-35     | Valid; scope warning |
| 165-37 | 2     | 7               | 36   | 165-36     | Valid                |

## Structured issues

```yaml
issues:
  - plan: "165-36"
    dimension: scope_sanity
    severity: warning
    description: "Plan36 lists eleven modified files, crossing the ten-file warning threshold. Its nine implementation/test owners span four source-policy families; two further owners activate the reviewed rows and inventory pin."
    metrics:
      tasks: 3
      files: 11
      maximum_source_owners_per_task: 5
    fix_hint: "Before execution, the parent should explicitly disposition this bounded scope risk or split the four-file coverage/docs task from the five-file activation/served-block task. Preserve independent source-before-pins review and sequential generation closure in either case. Existing mitigations include accepted35 capacity, distinct task ownership, one import-only127 test edit, unchanged Rust/history, readable628-line limits and a mandatory bounded replan for any additional owner."
```

The warning is a context-budget signal, not evidence that the specified repair changes are incorrect. Three tasks are within the task-count target. Plan37's five implementation owners plus two activation owners are within its file-count target. No task has ten source files or more.

## Goal and coverage

The roadmap goal is one exact target-derived work authority for live header selection, connected candidates and reorg decisions, with safe legacy-state refusal. Roadmap requirements CWOR-01/02/03/05 all have frontmatter owners across the existing39-plan set. Plan36 contributes CWOR-02/05; Plan37 contributes CWOR-02/03/05. Neither continuation silently takes ownership of CWOR-01, marks any requirement complete, or introduces Phase166's CWOR-04.

All D-01–D-12 decisions have explicit preservation or implementation rationale in both ledgers. The source repair preserves accepted-side eligibility, actual capability consumption, pre-mutation refusal, default-off networking, four-selector classifications and bounded tie evidence. Arithmetic changes, bootstrap/reopen acceptance, contextual consensus and actual daemon differential qualification remain with their existing or later owners. No deferred feature or reduced substitute was found.

## Current-source findings

| Family | Independently inspected current contract                                                                                                                                                                                                      | Planned proof                                                                                                                                                                                           |
| ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 73     | Actual block_requests test is bounded_block_requests_prioritize_best_headers_and_include_accepted_sides.                                                                                                                                      | Exact current needle, independently updated fixture, omission and obsolete-only negatives; all17 existing tests retained.                                                                               |
| 75     | Runtime-guide uses runs/\<run_id>/events.jsonl; observability uses runs/\<run_id>/events.jsonl.                                                                                                                                               | Change only runtime-guide expectation/fixture; independently refuse omission of either distinct required sentence.                                                                                      |
| 123    | Configured open recovers managed state, forwards seven arguments to prepare_initialized_network, derives consensus params and finishes the runtime. The helper owns the original exact seven-argument constructor.                            | Exact caller/helper argument vectors and ordering, result/finish wiring, configuration/default substitution negatives, existing activation/initialization controls retained.                            |
| 127    | fixtures.rs matches_block checks the Block variant and expected header hash. The production composition test asserts it on served and mixed_block_response.                                                                                   | Executable owning helper and both actual assertions; wrong variant/hash, unconditional/missing helper and assertion omission negatives.                                                                 |
| 128    | Every requested Some(block) is saved before disposition. Both storage/notification failures return before local-hash/accepted/queue effects. Successful save records the local hash; only requested Connected actual-active-tip bodies queue. | Focused extracted pure predicate retaining ordinary-owner/attribute/decoy checks, actual source grammar, independent ordering/side-pending/failure/tip controls and retargeted changed-anchor mutation. |
| 134    | Both actual seams acquire authorize_connect(block, consensus_params), consume that capability in chainstate.prepare_authorized_connect, prepare mempool/dependent lifecycle and call the existing transaction.                                | Narrow seam selectors/check only; exact capability receiver/arguments/consumption/order; both direct/stored timestamp distinctions and existing aggregate/effect/persistence/discovery rules retained.  |
| 129    | Existing checker imports127/128.                                                                                                                                                                                                              | Read-only integration: after36 only the known128 failure remains; after37 full129 tests/CLI must pass.                                                                                                  |

The existing conditional-save mutation has a24-space stale anchor while current local-hash continuation has16 spaces. The existing134 prepared-chainstate mutation targets absent prepare_connect_block; actual code uses prepare_authorized_connect. Both plans explicitly retarget these mutations and require real changed anchors. Import/syntax failures, absent anchors and zero discovery cannot qualify as RED.

## Wiring, data contracts and capacity

The127 new test module is registered through one import from the existing618-line test root. It imports the checker, not its importing test root; no cycle is required. All old root bodies remain exact except that registration import. The123599-line test uses its existing activation table and synthetic fixture, with a mandatory628-line stop rather than deletion or packing. Plan37 moves hasDurableSaveBeforeTip into a focused module imported by its original caller. Existing scanner and impl-attribute owners remain read-only; no new parser or external dependency is required.

The actual accepted35 source table is in old164 test-modules.ts:554 lines, SHA10a0a32f77e9ae44baac673085584fe08d331f58b433b7a6c83be22af3e3ac95. The current contracts re-export remains the route used by the current guard. Actual imported arrays are160SOURCE/88TEST/279INPUT. The correct existing pin is SHA256(JSON.stringify(INPUT_FILES)) =80dca53e69a2f0c917ae78297108ee16ad1187f2afb6d46e255790585770f624; newline-joined serialization is not the pin contract.

Actual membership confirms that only73.test already has a SOURCE/input row among Plan36's nine owners. Its other eight paths are absent. All five Plan37 owners are absent. Thus the proposed168/287 then173/292 SOURCE/INPUT counts are arithmetically consistent, with88TEST unchanged; the plans correctly require actual derivation after source acceptance. The shared554-line host has sufficient row capacity for these thirteen append rows without approaching628. The two activation owners are serialized by36→37, not same-wave writes.

Source acceptance precedes sourceIdentity derivation, inert proposal review, real stale-pin refusal and new-pin activation. No source body's identity includes the table row that authenticates it, so no hash fixed point is required. Ordinary SOURCE ownership rows do not fabricate Rust TEST ownership. Current raw inputs are distinguished from executed import/copy dependencies. New helpers' actual read graphs and fresh copied CLI behavior must be proved before acceptance; another required copy owner triggers bounded replan.

The two continuations do not touch manifest-bound159/160/161 source owners or the historical adapter. They preserve original historical hashes, receipts, manifests, snapshots and inverses. Their source-specific full inverses are before/after review evidence, not reinterpretation of historical attribution. Both full raw generation unions carry accepted35/36 entries and the original4829 R2 entries, explicitly attribute deltas, include optional absences and close actual logs. The source/provenance paths are compatible across these plans.

## Verification dimensions

| Dimension                   | Result                                                                                                         |
| --------------------------- | -------------------------------------------------------------------------------------------------------------- |
| 1 Requirement coverage      | PASS for bounded continuations; all four phase requirements retain frontmatter owners.                         |
| 2 Task completeness         | PASS; all five tasks have concrete files/action/automated verification/done; TDD tasks also state behavior.    |
| 3 Dependencies              | PASS;39 existing references resolve, no cycle, waves increase;35→36→37 is valid.                               |
| 4 Key links                 | PASS;127 test registration,128 extraction import,123 caller/helper/finish and129 integration are explicit.     |
| 5 Scope sanity              | WARNING only for36 eleven-owner count; all remaining task/file bounds are addressed.                           |
| 6 Verification derivation   | PASS; source truths map to actual refusal/authority/persistence proof, not installed dependencies.             |
| 7 Context compliance        | PASS; all locked decisions preserved/referenced, no deferred work.                                             |
| 7b Scope reduction          | PASS; no hardcoded/stub/later-wiring substitute for a locked decision.                                         |
| 8 Nyquist                   | SKIPPED: workflow.nyquist_validation=false; no applicable Validation Architecture section.                     |
| 9 Cross-plan data contracts | PASS; source table location, row renewal/appends, actual serialization and raw closure are compatible.         |
| 10 AGENTS compliance        | PASS; Bun pin, native final gate, AAA, physical caps, no Rust/history edits, no nested commits or hook bypass. |
| 11 Research resolution      | PASS; all four Open Questions have explicit RESOLVED markers.                                                  |

Root's separately authorized final14 dependency/context amendment behind39 remains a pre-execution obligation. The currently old14→35 tail is not permission to run final14 concurrently with36–39. Preserve every original14 Task XML and automated command. Full native/matrices/Rust/coverage/Bazel, whole-phase review/security/formal/lifecycle and normal-hook commit/push remain mandatory later gates. Both failed native attempts remain failed evidence.

## Actual checks and input identities

Actual read-only checks: both gsd-tools plan-structure commands returned valid=true/errors=[]/warnings=[]; lifecycle165 --require-plans --raw returned valid; all four plan/ledger files passed mdformat --check --extensions gfm --extensions frontmatter. A static39-plan dependency/requirement pass found valid references/increasing waves/no cycles. Both new modules are currently absent. All twelve existing implementation/test bodies were authenticated against the complete before receipt and are still byte-exact; current activation host is intentionally the accepted35 generation.

The initial Python module spelling for mdformat was unavailable; the installed mdformat executable with the required extensions subsequently ran and passed. No application, heavy tests, Cargo/Bazel, source edits, shared metadata edits or Git mutation occurred in this check. Importing current arrays was a read-only Bun1.3.9 observation, not an executed source-policy acceptance.

| Reviewed artifact                       | Full SHA256                                                      |
| --------------------------------------- | ---------------------------------------------------------------- |
| 165-36-PLAN.md                          | 3040d19e226e1a77038c7f97978b7ef280cbc4f4aa733d5c9f08413f69f36d90 |
| 165-36-SOURCE-GUARD-REPLAN.md           | 3caab28af639089d9cac9e91c4a6dfd4ca739811176a7174f637c4720af96cea |
| 165-37-PLAN.md                          | d383153ecf174623ab20b6d45ae823b76ba6cad953e2924383f3a46e5170a914 |
| 165-37-SOURCE-GUARD-REPLAN.md           | 95fa8a0cf4dae8718bab14667b6810dd0778956ff87d3f0df1e95ccb7c5abeee |
| Complete344-entry source before receipt | a6f23987c5d1ea36909f8f49866f17bd086199014509171d63524c4252e229fd |
| 73 constants                            | 464e00fd7dd8ff9e3ea966f39fa9eff0c464a6467cbf9e085aa7c96e9cc9de4f |
| 73 test                                 | 91d61c382c08591d5cf004be63a2facf6d32dc524256eafa3af5d780f4c1d229 |
| 75 checker                              | 364b244e6e11326a9ec0724d4e687a1831f2099453d0af26cc0bb16fa2fd4d14 |
| 75 test                                 | ef4ce1a7fe418378b029dfed7fa4ae68145932de91355e0bb31b249df660745d |
| 123 checks                              | 0e244a82de3f37253f27daae236f1977b83f78b2c8c531d77521d9e27a59c2ea |
| 123 test                                | 7cac9ea665fb7497ac45d98c5f896aa6b44ec6050389dcae6e6acece42d66254 |
| 127 checker                             | 7ac427147c31edd6099ef493679e4fc660a58c8ebad316a7dd2da22eafd14ee5 |
| 127 test                                | 30b632eb5472468eaef098688eb7c2f79f550a8e51d84a543470b01ca612d993 |
| 128 checker                             | 98c41aeae904b267b629318f92857946fc7e098324e754215f4fa86da12b398e |
| 128 test                                | d7fd55969427869650a9e1d84ffdf806e000f853489e3551f2429c282b10927a |
| 134 aggregate-roots                     | 87ae686fb5d7893dc8bc6a8ba721ed2d88ba1269c415be65ce207e1ff617c61d |
| 134 apply-helpers                       | 574b61733a318667e25d50af076bf84fa60865a04366e912457c75a507aad2ea |
| Current165 entry                        | f50d86004db1e033654fb8eacd2e5ffb79d3436592743e0ec6c69dc34c094f81 |
| Current open_runtime.rs                 | e0d80123609b1f85a9edb06c18f1c1682268215bace22f343a2c420357c338d5 |
| Current matches_block fixture owner     | 299555281521d51b7e19cb59cd2467f720357c2fc866b4f8446e0be2bd7c2304 |
| Current phase127 composition owner      | a787810f831b3f68e78d4a85890e9b9fa05aeafade8317aa0e4e1465037fca99 |
| Current block_response.rs               | 718f8eb2fa0e552cb26fd43e62cd92ade6145973fc96b957885d54121060d11c |
| Current mempool_lifecycle.rs            | fa186eeafab8925c40006f7a20671c265af261a364698712b751562b6baa8bbb |

Active lessons were loaded completely: global5230B and repository1958B, combined7188B and summed2397 estimated tokens; no archives. Neither project skill directory exists. This report is the only file written by this checker.
