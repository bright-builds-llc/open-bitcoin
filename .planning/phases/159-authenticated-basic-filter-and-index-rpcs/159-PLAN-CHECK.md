---
phase: "159"
generated_by: gsd-plan-checker
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T16:37:23Z"
status: passed
iteration: 2
reviewed_at: "2026-10-09T16:42:44Z"
---

# Independent Phase 159 Plan Check

Current result: **VERIFICATION PASSED**, 0 blockers and 0 warnings, on revision recheck at 2026-10-09T16:42:44Z. The initial audit below is preserved as history; the authoritative recheck and issue closures follow it.

## Initial Check: Issues Found (Superseded)

**Phase:** Authenticated BASIC Filter and Index RPCs
**Plans checked:** 8; 21 implementation/verification tasks
**Issues:** 4 blockers, 4 warnings, 0 info

This is static plan verification, not implementation acceptance. No application, Cargo, Bazel, Git mutation, or requirement/state mutation was performed. The sole new artifact is this report. CFRP-01/02 remain Pending until formal phase verification and the parent-owned native/source/security/lifecycle gates succeed.

Material guidance: repository AGENTS.md and AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md and architecture/code-shape/verification/testing/Rust pages; both active lesson files were fully loaded (7,188 bytes, 2,397 estimated tokens). No project .claude/skills or .agents/skills directories were found. GSD revision gates, planning thinking models and plan-checker calibration informed issue severity. CONTEXT, RESEARCH and the approved nonvisual UI-SPEC were reviewed alongside the roadmap, requirements, relevant project scope and all eight complete plans.

## Requirement and Decision Coverage

| Requirement | Covering plans | Goal-backward result |
| --- | --- | --- |
| CFRP-01 | 01–08 | Complete intended coverage: genuine accepted history; bounded integrity reads; exact parser/errors; same-authority dispatch; authenticated actual-daemon stale/prune/reopen/fault proof. Execution handoff blockers below must be fixed. |
| CFRP-02 | 02, 04–08 | Complete intended coverage: BASIC name/selection, exact fields, processed height/zero and initial latch separated from later readiness. |

All roadmap requirement IDs appear in plan requirements frontmatter. No additional PROJECT requirement directly implied by this phase is omitted. CFPR-02, CFNET, CFOP and CFGR milestone completion remains correctly deferred to its owning phases.

| Locked decision | Implementing tasks |
| --- | --- |
| D-01 | 02/2; 06/1–3 |
| D-02 | 02/1–2; 06/1; 07/3 |
| D-03 | 04/1–2; 06/1; 07/2–3 |
| D-04 | 02/1–2; 07/1 |
| D-05 | 01/2; 04/1–2; 05/1; 06/1–2; 07/1 |
| D-06 | 01/1–2; 03/1–3; 04/2; 07/1–3 |
| D-07 | 04/1–3 |
| D-08 | 02/1; 04/2; 06/1 |
| D-09 | 04/2; 05/1; 07/1–2 |
| D-10 | 05/1–2; 06/1–3; 07/1,3 |
| D-11 | 06/2–3; 07/1,3 |
| D-12 | 03/3; 04/3; 05/2; 06/3; 07/1–3 |
| D-13 | 08/1–3 |

Each D-01–D-13 has explicit implementing action references. No invented v1/static/stub reduction or deferred product expansion was found. UnknownLegacy is an explicit conservative compatibility choice within D-06 discretion, not fabricated historical validation. Numeric read limits are execution measurements with an explicit derivation/admission contract, not omitted resource limits.

## Plan and Dependency Summary

| Plan | Tasks | Files | Wave | Dependencies | Assessment |
| --- | --- | --- | --- | --- | --- |
| 159-01 | 2 | 6 | 1 | none | Sealed monotonic history/coverage and recovery are concrete. |
| 159-02 | 2 | 6 | 1 | none | Registry additions break compile boundary before 06. |
| 159-03 | 3 | 8 | 2 | 01 | Genuine accept/reorg/header seams; bounded unresolved batch. |
| 159-04 | 3 | 12 | 3 | 03 | Future type handoff undefined; scope warning. |
| 159-05 | 2 | 9 | 4 | 04 | Captured frontier, bounded waiter Future and owner signaling are concrete. |
| 159-06 | 3 | 11 | 5 | 02,05 | Actual HTTP lock release/auth/parser integration; scope warning. |
| 159-07 | 3 | 5 | 6 | 06 | Genuine continuous acceptance, paired deletion, all-handle reopen and fault proof. |
| 159-08 | 3 | 9 | 7 | 07 | Empty task files, reference paths and omitted final threat ID need correction. |

Declared dependencies exist, are acyclic and have correct maximum-dependency-plus-one waves. Wave 1 plans have disjoint files. Shared node/RPC modules in later waves are sequential. The problems are compilation/data-contract staging across those otherwise valid edges, not a same-wave merge race.

## Dimension Results

| Dimension | Result | Evidence |
| --- | --- | --- |
| 1 Requirement coverage | PASS | CFRP-01/02 appear in frontmatter and have specific end-to-end tasks. |
| 2 Task completeness | FAIL | 08/3 has an empty files element; incorrect read-first references also need correction. All tasks otherwise name actions, behavioral checks, done and acceptance criteria. |
| 3 Dependency correctness | FAIL | 02 registers variants before dependent dispatch exists; 04 names a carrier type only introduced by 05. |
| 4 Key links planned | PASS, subject to handoff fixes | Node acceptance → ledger; recovered integrity → bounded reader; same authority → RPC; owner → barrier; auth → parser/dispatch; real prune/reopen → HTTP. |
| 5 Scope sanity | WARNING | 04 has 12 files and 06 has 11; no plan has 5+ tasks or 15+ files and no task has 10+ files. |
| 6 Verification derivation | PASS | Every must_haves parsed with truths/artifacts/key_links; user-visible truthful history, results/errors, readiness and retention are linked to concrete tasks. |
| 7 Context compliance | PASS | All 13 decisions implemented; no deferred surface included. |
| 7b Scope reduction | PASS | No locked decision silently simplified. |
| 8 Nyquist | SKIPPED | workflow.nyquist_validation=false; no VALIDATION.md gate applies. |
| 9 Cross-plan contracts | FAIL | 04 Pending carrier is not independently defined before 05; other shared identity/bytes/provenance/barrier contracts are compatible. |
| 10 AGENTS.md compliance | PASS, subject to file/reference fixes | Native contract, serialized timed Cargo/Bazel, Bun script ownership, breadcrumbs, explicit UAT and root-owned commits are specified. |
| 11 Research resolution | FAIL | RESEARCH has five questions without RESOLVED markers or a resolved heading. |
| Security planning | WARNING | All plans contain trust boundaries and STRIDE mitigations; final audit range stops at 26 although 27 is declared. ASVS L1 remains enabled and high findings block finalization. |

The GSD plan-structure command reports all eight plans valid with 2/2/3/3/2/3/3/3 tasks. Its hasFiles check accepts an empty element, so manual semantic review found the 08/3 problem. Each automated behavioral selector explicitly requires positive discovery; no empty selector is accepted. Full native/source/security/lifecycle gates remain parent-owned and are not replaced by these static checks.

## Important Goal-Backward Checks

- History is independent of BASIC, body/undo, active membership and coins durability. Genuine ordinary/reorg absorption is observed before fallible effects; every replacement identity is captured; one pre-admitted unresolved batch prevents unbounded memory growth. New trusted header admission and legacy/raw-seeded unknown are distinguished, with no public accepted-fact factory.
- Request work uses fully parsed target/immediate parent under maintained recovered all-record integrity authority. Raw clones invalidate before mutation. Recovery walkers remain recovery-only. Fixed reads, checked envelope/copy/hex costs, legitimate singleton and one-over controls are measured at genesis/20/400/stale positions.
- Initial incomplete queries can use an available record without waiting. Later queries capture a finite incarnation/generation/branch/sequence frontier, await outside context/authority locks, and revalidate before completing. Cancellation, full waiter admission, newer target independence, publication failure, disable/reorg and stop wake controls are explicit.
- Dedicated parser tasks cover named/mixed/duplicate/null/arity/type aggregation before strict hash handling, lowercase result encoding, uint256 reversal and recognized-disabled V0. Raw HTTP duplicate preservation is actually wired rather than asserted through a collapsed Value map.
- Real daemon config/open/context/HTTP and scheduled owner evidence is specified. A genuine 400+ accepted chain, actual paired height-window deletion, absent body/undo, durable have_pruned, stale branch, all-handle drop and actual reopen are required. Missing-row and backend/publication/lifecycle faults cannot become empty successful data.

## Structured Issues

```yaml
issues:
  - plan: "159-02"
    dimension: dependency_correctness
    severity: blocker
    task: 2
    description: "Wave 1 adds GetBlockFilter/GetIndexInfo MethodCall/SupportedMethod registry variants, but the exhaustive dispatch match and working handlers are owned by Wave 5 Plan06. Plan02's cargo test cannot compile this intermediate state."
    fix_hint: "Keep Plan02 limited to exported pure typed parser/contracts/error support; defer MethodCall/SupportedMethod variants and normalize_method_call registry routing to Plan06 Task1 alongside real dispatch. Add method.rs to Plan06 task/frontmatter ownership, and move its dispatch tests to Task3 so Task1 remains at five files. Update key links/done criteria to match the staged ownership; do not insert temporary fake handlers."
  - plan: "159-04"
    dimension: cross_plan_data_contracts
    severity: blocker
    task: 2
    description: "Plan04 defines BasicFilterQuery::Pending(BasicFilterReadBarrier), but BasicFilterReadBarrier is only created in dependent Plan05. Plan04's authority tests require a concrete independently compiling carrier."
    fix_hint: "Define/export an opaque captured-frontier carrier in Plan04, with private authority/generation/branch/frontier identity and explicit no-Future-yet staging. Plan05 should turn that existing carrier into or wrap it with the bounded operational Future, preserving the captured target. Align both interfaces, artifacts, tasks and verification without introducing a cycle or reporting fake readiness."
  - plan: "159-08"
    dimension: task_completeness
    severity: blocker
    task: 3
    description: "The auto verification task has an empty <files></files> despite mandatory Files content and generated LOC freshness changes. Structural validation only checks element presence."
    fix_hint: "List docs/metrics/lines-of-code.md in Task3 files and files_modified as the intentionally tracked generated verification artifact; retain root-owned formal phase/native/source/security/lifecycle completion and Git gates."
  - plan: null
    dimension: research_resolution
    severity: blocker
    description: "159-RESEARCH.md has ## Open Questions with five entries and no RESOLVED status. Plans select approaches, but the required research-resolution gate is still open."
    fix_hint: "Have the research owner resolve each question explicitly: conservative UnknownLegacy; selected narrow generic query adapter; codec-bound measured resource-budget procedure with final numeric limits produced in04/3; duplicate-preserving HTTP visitor; exact whole pinned help comparison. Mark each chosen outcome RESOLVED FOR PLANNING with its assigned plan/task and replace contradictory unresolved metadata; keep empirical resource measurements explicitly pending execution evidence. Do not invent numeric timing guarantees or historical evidence."
  - plan: "159-06,159-08"
    dimension: task_completeness
    severity: warning
    description: "Read-first references name nonexistent existing inputs: http/tests/boundary.rs, check-phase158-retained-branch-identity.ts and packages/open-bitcoin-rpc/README.md. These are not outputs supplied by earlier dependencies."
    fix_hint: "Use existing http/tests.rs, scripts/check-phase158-validated-reorg.ts and packages/README.md; update Plan08 files_modified/task files and README actions accordingly. Upstream-produced new159 files in read-first lists are valid and should remain."
  - plan: "159-08"
    dimension: security_planning
    severity: warning
    task: 3
    description: "Final source/security review covers T-159-01–T-159-26 although the same plan declares T-159-27 for source breadcrumb integrity."
    fix_hint: "Extend the final audit range through T-159-27, keeping ASVS L1 applicability and high-finding block explicit."
  - plan: "159-04"
    dimension: scope_sanity
    severity: warning
    description: "Plan04 modifies 12 files across storage integrity publication/recovery, generic authority hooks and measurements, above the 10-file warning threshold."
    fix_hint: "Split storage reader/recovery authority from generic query/measurement work if either requires broad refactoring, or document the narrow module/export glue versus substantive files and keep each task within its five-file ownership. No expansion of adjacent lifecycle or recovery behavior is justified by this plan."
  - plan: "159-06"
    dimension: scope_sanity
    severity: warning
    description: "Plan06 already modifies 11 files and will also own method registry wiring after the compile-handoff fix; HTTP parsing/async locking is the highest-risk integration seam."
    fix_hint: "Keep the registry/context/dispatch task at five files by moving tests to Task3 and identify thin declarations separately from substantive adapters; split HTTP integration into a dependent plan if broad unrelated transport refactoring becomes necessary. Record the scope rationale and preserve sequential ownership."
```

## Recommendation

Revise the four blockers and address the four warnings, then run the independent checker again. The intended feature design covers the phase goal; its execution boundaries must become concrete and compilable before implementation starts. This is revision iteration 1 of a maximum of 3, with escalation if issue count does not decrease. No requirement is marked complete by this report.

## VERIFICATION PASSED

**Rechecked:** 2026-10-09T16:42:44Z
**Lifecycle:** 159-2026-10-09T16-11-49
**Phase:** Authenticated BASIC Filter and Index RPCs
**Plans verified:** 8, with 21 tasks
**Status:** All applicable checks passed; 0 blockers, 0 warnings, 0 info.

The revised full plan set and resolved research outcomes were rechecked. The initial four blockers and four warnings are closed, with no new plan-level compile/dependency defect identified. This is approval to execute the plans, not evidence that their implementation or runtime behavior has passed.

### Prior Issue Closures

| Initial issue | Resolution verified |
| --- | --- |
| 02 registry before 06 real dispatch | Plan02 now exports pure typed normalizers/contracts only. Plan06/1 atomically owns SupportedMethod/MethodCall variants, exhaustive registry/routing and real context/dispatch handlers in five files. Initial dispatcher tests are inline before the dedicated broader test module is introduced. |
| 04 undefined Pending barrier type | Plan04 defines/exports private-field BasicFilterReadFrontier and returns the opaque carrier without claiming readiness. Plan05 consumes that exact captured target and updates operational Pending constructors/payload together with its std Future implementation. No forward type or reverse dependency remains. |
| 08/3 empty files | Task3 and files_modified now include intentionally tracked generated docs/metrics/lines-of-code.md; the full verifier owns regeneration/freshness. Every task files element has nonempty content. |
| Five research questions unresolved | All five have inline RESOLVED FOR PLANNING outcomes and assigned plan/tasks: UnknownLegacy; narrow generic same-handle query backend; codec-bound budget derivation plus execution measurements; raw duplicate-preserving HTTP visitors; complete pinned help fixtures. Actual measurements and fixture verification remain honestly pending execution. |
| Three wrong read-first/document references | 06 uses existing http/tests.rs; 08 uses scripts/check-phase158-validated-reorg.ts and packages/README.md. Every other missing read-first input is explicitly produced within its own or a preceding dependent plan; no unsupplied existing input remains. |
| Missing T-159-27 closeout audit | 08/3 now requires independent ASVS1/STRIDE review across T-159-01–T-159-27, with high findings blocking finalization. |
| 04 12-file scope warning | Explicit scope-budget separates substantive reader/authority/measurement modules from thin declaration and epoch hooks. Three tasks each own at most five files; no adjacent recovery/coins/lifecycle refactor is authorized, and expansion requires a parent plan amendment. This resolves the warning without artificial plan proliferation. |
| 06 integration scope warning | Explicit five/four/four task ownership and thin existing-root wiring bound the 12-file integration. Initial inline tests permit positive discovery before broader test modules exist. Unrelated parsing/auth/batch/owner refactors are forbidden, and broader work requires a parent amendment. |

### Final Coverage and Plan Summary

| Requirement | Plans | Status |
| --- | --- | --- |
| CFRP-01 | 01–08 | Covered; implementation verification pending |
| CFRP-02 | 02,04–08 | Covered; implementation verification pending |

| Plan | Tasks | Files | Wave | Status |
| --- | --- | --- | --- | --- |
| 159-01 | 2 | 6 | 1 | Valid |
| 159-02 | 2 | 5 | 1 | Valid |
| 159-03 | 3 | 8 | 2 | Valid |
| 159-04 | 3 | 12 | 3 | Valid; bounded substantive/glue ownership documented |
| 159-05 | 2 | 9 | 4 | Valid |
| 159-06 | 3 | 12 | 5 | Valid; bounded substantive/glue ownership documented |
| 159-07 | 3 | 5 | 6 | Valid |
| 159-08 | 3 | 10 | 7 | Valid; includes four parity docs, two READMEs, checker/test/native routing and generated LOC artifact |

GSD structural checks pass for all eight plans. Independent semantic checks confirm nonempty task files, matching lifecycle IDs, valid maximum-dependency-plus-one wave assignments, acyclic references and disjoint same-wave ownership. Final registry wiring remains sequential after parser and node readiness. Shared identity/provenance/bytes/epoch/frontier data contracts remain compatible, with the initial carrier-to-Future staging now explicit.

All D-01–D-13 retain the implementing tasks in the initial coverage table. Complete provenance sealing/coverage, accepted-before-fallible-effects behavior, all-replacement retention, recovered integrity/raw-writer invalidation, fixed target/parent reads and checked resource accounting remain required. The finite captured frontier still waits outside context/authority locks, uses the ordinary owner, and earns completion through achieved work rather than the initial-sync latch. Authentication ordering, exact named/null/type/hash/error behavior, disabled V0 and exact successful result fields remain planned. Production configured daemon plus genuine continuous chain, actual paired deletion/all-handle reopen, stale lookup, missing/backend/publication/lifecycle fault and private-data controls remain required; no seeded-snapshot acceptance shortcut was introduced.

Dimensions 1–7b, 9–11 and security planning pass. **Dimension 8: SKIPPED (nyquist_validation disabled or not applicable).** Scope warnings are resolved through concrete ownership rationale; 08's 10 paths include generated/documentation artifacts and do not add another substantive integration seam. No task reaches the 10-file task threshold, no plan reaches 15 files, and all plans retain 2–3 tasks.

The original repository guidance and bounded active-lesson loading remain applicable. No source, plan, state, roadmap or requirement files were changed by the checker; only this report was updated. No application/tests/Cargo/Bazel/commits were run. Root retains formal phase verification, default native verification, independent source/security review, lifecycle validation, requirement completion and authorized final commit/push.

```yaml
issues: []
```

Plans verified. Continue the already-authorized `/gsd-execute-phase 159` workflow; mark no requirement complete before formal implementation verification.

## Plan 07 Execution Amendment: VERIFICATION PASSED

**Checked:** 2026-10-09T18:50:57Z
**Lifecycle:** 159-2026-10-09T16-11-49
**Subject:** Plan07 Task3 private fault evidence placement, amended at 2026-10-09T18:47:46Z
**Result:** PASS; 0 blockers, 0 warnings. This approves the amended plan, not unexecuted tests.

Read the amended Plan07, D-12/CFRP-01/02, Plan04–06 summaries, actual publication/readiness and target/parent storage tests, actual RPC classifier/projection tests, and production source wiring. Plan07 still has three complete tasks and now eight owned paths; Task3 has four paths. Structural validation passes. Existing Plan04/05 query/catch-up files are edited only in a later dependent wave, so the amendment introduces no concurrent ownership race. Tasks1/2 and their configured daemon, continuous genuine acceptance, real paired deletion and all-handle reopen obligations are unchanged.

D-12 requires the real daemon/shared-dispatch/continuous-retention product path and publication/backend corruption evidence. It does not require exporting private node fault controls into a normal RPC dependency. Composed proof remains adequate here because both layers use the same production query/error types and functions, and the amended acceptance criteria explicitly require source links, exact projections, actual executing boundaries and positive named test discovery. No required case is replaced with a fabricated connected flag, synthetic clean epoch or successful empty response. Summary claims must distinguish actual HTTP flows from composed private-fault/RPC mapping evidence.

### Verified Evidence Connections

| Required behavior | Existing evidence / amended obligation |
| --- | --- |
| Actual configured daemon/auth/shared dispatch, initial available/missing and later owner completion | Nine named phase159_daemon_rpc tests exist across rpc.rs, retention.rs and failures.rs, including actual maintenance-worker completion. Executor-reported pass/counts remain execution evidence to be consolidated, not newly run by this checker. |
| Real active/stale/paired-prune/all-handle reopen | Existing retention test and unchanged Task2 require actual deletion/body+undo absence/durable have_pruned and exact original HTTP commitments after production reopen. Reported 402 original accepts, 11 replacements and height20/1126 logical-byte deletion are scoped hermetic fixture facts, not ordinary prune-RPC or automatic-threshold claims. |
| NeverConnected/legacy/connected absence | Actual configured header-only and legacy controls remain; natural ScriptsValid displaced-but-unindexed absence supplies real ready corruption before/after reopen. Task3 retains authentic provenance on explicit node-private missing-row faults and permits truthful reopen refusal for inconsistent checkpoint/ancestry. |
| Four publication boundaries | phase159_basic_readiness_owner_publication_faults_wake_terminal_failure enumerates BeforeRecords, BeforeCheckpoint, BeforeProtection and AfterCommit. It genuinely accepts pending work, registers a real production-query barrier, invokes drive_basic_filter_index_turn, and asserts wake plus typed OwnerFailed. Reuse is valid with the four cases individually enumerated in the evidence map; no duplicate copies are required. |
| Target/parent corruption and missing predecessor | phase159_basic_point_query_target_parent_corruption_and_missing_parent_fail_closed mutates private backend rows beneath an earned integrity epoch and calls the real guarded storage reader. It proves storage validation; it alone is not full ManagedNetworkHandle query routing. Amended rpc_faults therefore still requires actual basic_filter_query/complete_basic_filter_read outcomes on genuine accepted runtime, reusing storage controls where sufficient. |
| Raw clone invalidation | Existing storage/readiness tests and amended actual-query cases retain invalidation before raw mutation and final-read refusal. Fault injection must never readmit or fabricate a clean integrity epoch. |
| Actual backend read failure | Previously absent from actual-query evidence. The planned narrow crate-private cfg(test), store-bound record/read seam in query.rs must inject a backend read failure through the production point-query path on a genuine recovered runtime; rpc_faults asserts BasicFilterQueryError::Storage(BackendFailure), distinct from Missing/Found. The seam may simulate backend failure/corruption only; it grants no provenance, readiness or recovery authority. This planned addition closes the evidence design gap once executed. |
| Exact RPC output/redaction and real wiring | Actual dispatch tests exact_missing_matrix and query_faults_are_allowlisted assert pinned -5/-1/ready -32603 versus fixed unavailable -32603 and backend-marker redaction. Production prepare_filter calls basic_filter_query → query_failure/project_query; HTTP finish awaits the barrier then calls complete_basic_filter_read → query_failure/project_query. Thus private actual node outcomes connect to the same real public projection, rather than a separate test classifier. |

The additional seam must remain cfg(test), crate-private and tied to its store/test so parallel fixtures cannot affect each other. No public API, production dependency, feature flag, direct accepted constructor, detached index owner or recovery-success override is authorized. Backend mutation underneath earned integrity simulates a disk fault; it must not set integrity or bypass the production query checks being tested. The production build continues to contain no fault API.

Task3's automated verification covers configured HTTP failures, the new private owner/query selector and the actual RPC dispatch/projection suite, with positive test discovery mandatory. Existing reused controls must be named with their actual prior evidence or rerun evidence in the summary; the new selector must never be credited for tests it did not execute. Synthetic genesis, test maturity one and manual owner-level prune-plan limits remain explicit. Formal phase/native/source/security/lifecycle acceptance and all requirement completion remain root-owned.

```yaml
issues: []
```
