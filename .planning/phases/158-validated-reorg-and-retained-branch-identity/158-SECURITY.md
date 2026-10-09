---
phase: 158-validated-reorg-and-retained-branch-identity
report: security
status: secured
asvs_level: 1
block_on: high
threats_total: 25
threats_closed: 25
threats_open: 0
generated_by: gsd-security-auditor
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: "2026-10-09T04:33:48Z"
final_snapshot: true
---

# Phase 158 Security Mitigation Verification

## SECURED

**Closed: 25/25. Open: 0/25.** All declared mitigations have source, concrete assertion and applicable executed-evidence support at the frozen reviewed snapshot. Actual full default native verification passed. Formal goal verification now passes 21/21, the full same-attempt lifecycle gate is valid, and canonical CFIX-03 is Complete. Git finalization remains root-owned. ASVS L1 describes the relevant internal access, input, business logic and resource-control scope; this report is not ASVS certification, a broad vulnerability scan or a production/funds-readiness claim.

## Scope, method and guidance

Read all seven PLAN threat-model blocks, all seven SUMMARY files, CONTEXT, the original and consolidated independent reviews, both fix checks, historical harness review, native evidence and implementation/test boundaries cited below. All 25 threats are classified by their declared STRIDE category and disposition before verification. Every disposition is **mitigate**; no accepted or transferred risk is inferred. Verification traces the declared producer/check/publication patterns and reads concrete behavior assertions; predecessor test executions are cited separately from this auditor's static checks.

Material guidance: AGENTS.md/Repo-Local Guidance, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md and architecture/code-shape/testing/verification/Rust standards; the gsd-security-auditor role and gsd-secure-phase evidence rules. Both active lesson files were fully loaded: 7,188 bytes and 2,397 conservative estimated tokens. This report is the sole audit-owned mutation in the parent's active GSD workflow. No Cargo, Bazel, Git mutation or implementation edit was performed.

Checkable audit plan:

- [x] Extract and classify every declared threat; inspect summary flags and source-review findings.
- [x] Trace capability, identity, publication, preflight, bounded work, recovery and prune mitigations to source and concrete tests.
- [x] Recheck final WR-158-01 source/test bytes and actual positive/negative regression receipts after the fix freezes.
- [x] Inspect final Plan07 native/summary evidence and review/lifecycle gate enforcement after the documentation/scripts freeze.
- [x] Recompute fingerprints and issue SECURED for all declared mitigations.

## Threat verification

Paths are repository-relative. CLOSED means the declared mitigation pattern and associated concrete assertions were found; it does not imply the auditor reran Cargo or that all phase finalization gates have completed.

| Threat ID | Category | Disposition | Status | Declared mitigation and assessment | Evidence |
| --- | --- | --- | --- | --- | --- |
| T-158-01-01 | S/E | mitigate | CLOSED | Private stages and receipts; genuine absorption only; external mutation/clone/preview cannot create authority. | `packages/open-bitcoin-chainstate/src/engine.rs:112`; `packages/open-bitcoin-chainstate/src/engine.rs:153`; `packages/open-bitcoin-chainstate/src/engine/stage.rs:184` |
| T-158-01-02 | T | mitigate | CLOSED | Exact old/hash/ancestor endpoints, checked renewal, immutable trial state. | `packages/open-bitcoin-chainstate/src/filter_index/catch_up/reorg.rs:28`; `packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/reorg.rs:170` |
| T-158-01-03 | D | mitigate | CLOSED | Fixed-size replacement facts; checked arithmetic; no new full-history copy in reducer. | `packages/open-bitcoin-chainstate/src/filter_index/catch_up/reorg.rs:12`; `packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/reorg.rs:256` |
| T-158-02-01 | S/E | mitigate | CLOSED | Same-store proof, sealed absorption and tracked lineage; raw tip grants no authority. | `packages/open-bitcoin-node/src/chainstate/fjall_store/reorg.rs:255`; `packages/open-bitcoin-node/src/chainstate/fjall_store/reorg.rs:38`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs:324` |
| T-158-02-02 | T | mitigate | CLOSED | Accepted next positions, exact predecessor commitments, immutable byte equality, guarded stale comparison. | `packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs:155`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs:172`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs:231`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs:380` |
| T-158-02-03 | T/D | mitigate | CLOSED | Atomic checkpoint/generation/protection; exact authenticated displaced fence and shared prefix; reserved lock. WR-158-01 final bytes and positive/three negative receipts checked. | `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs:160`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs:287`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs:161`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs:68` |
| T-158-02-04 | R | mitigate | CLOSED | Ambiguous publication returns no achievement, poisons control; real configured reopen asserts conservative state. | `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs:564`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs:587`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs:509` |
| T-158-02-05 | D | mitigate | CLOSED | Local endpoint checks and bounded turn writes, no new suffix/forest scan. | `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs:253`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs:90`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs:163` |
| T-158-03-01 | S/E | mitigate | CLOSED | Private receipt and achieved bridge; generic/preview/foreign paths lack lineage. | `packages/open-bitcoin-node/src/chainstate/fjall_store.rs:18`; `packages/open-bitcoin-node/src/chainstate/fjall_store.rs:149`; `packages/open-bitcoin-node/src/chainstate/fjall_store/reorg.rs:255` |
| T-158-03-02 | T/R | mitigate | CLOSED | Explicit preview/accepted/durable states; expose genuine absorption before fallible index publication and persistence. | `packages/open-bitcoin-node/src/chainstate/filter_reorg.rs:16`; `packages/open-bitcoin-node/src/chainstate.rs:429`; `packages/open-bitcoin-node/src/chainstate/filter_reorg.rs:242`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs:418` |
| T-158-03-03 | T/D | mitigate | CLOSED | Own coins/metadata receipt; shared safe prefix, ordinary cadence and reserved locks. Genuine consecutive-unflushed return and ordinary own-flush/reopen assertions verified on final bytes. | `packages/open-bitcoin-node/src/chainstate/fjall_store.rs:99`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs:161`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs:68` |
| T-158-04-01 | T | mitigate | CLOSED | Native required body/undo presence and complete identity; no cached or other-branch substitution. | `packages/open-bitcoin-node/src/chainstate/filter_reorg/preflight.rs:176`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs/undo.rs:56`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs:252` |
| T-158-04-02 | D | mitigate | CLOSED | Account native source probes and conservative clone/decode reservations before allocation/effects. | `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs:109`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs/undo.rs:38`; `packages/open-bitcoin-node/src/chainstate/filter_reorg/preflight.rs:252`; `packages/open-bitcoin-node/src/chainstate/filter_reorg.rs:52` |
| T-158-04-03 | E/T | mitigate | CLOSED | Existing serialized owner; store suspension before preview; reserved locks. | `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs:79`; `packages/open-bitcoin-node/src/chainstate/filter_reorg.rs:206`; `packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs:236`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs:225` |
| T-158-04-04 | R | mitigate | CLOSED | Explicit bounded source refusal; sync body waiting and accepted failure distinctions. | `packages/open-bitcoin-node/src/chainstate/filter_index.rs:91`; `packages/open-bitcoin-node/src/sync/block_reconcile.rs:163`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs:525`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs:344` |
| T-158-05-01 | T | mitigate | CLOSED | Separate genuine staging oracle compares expected historical/same-block scripts, exact parent/hash/header chain. | `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs:128`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs:194`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs:491` |
| T-158-05-02 | R | mitigate | CLOSED | Drop runtime/store/proof owners; reopen same path; inspect raw durable rows separately from configured state. | `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs:263`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs:9`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs:57` |
| T-158-05-03 | T/E | mitigate | CLOSED | Only verified immutable reuse; consensus-required bodies/undo mandatory; reserved startup protection before prune. | `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs:32`; `packages/open-bitcoin-node/src/chainstate/filter_reorg/preflight.rs:66`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs:86`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs:276` |
| T-158-06-01 | T/R | mitigate | CLOSED | Actual preview/absorption/rewind/append/writer fault matrix, explicit safe and accepted state, genuine reopen. | `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs:344`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs:418`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs:558` |
| T-158-06-02 | T/E | mitigate | CLOSED | Reserved CRUD and generation changes; manual/automatic pruning and interrupted startup cannot weaken protection. | `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs:10`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs:164`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs:236` |
| T-158-06-03 | D | mitigate | CLOSED | Concrete complete work ledger; checked component ceilings; exact versus one-under admission; no prefix-growing new scan/cache. | `packages/open-bitcoin-node/src/chainstate/filter_reorg.rs:52`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs:163`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs:199`; `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REORG-MEASUREMENTS.md:11` |
| T-158-06-04 | I | mitigate | CLOSED | Typed failure labels and count/size/time observations; no new raw wallet, secret or source-payload output in audited diagnostics. | `packages/open-bitcoin-node/src/chainstate/filter_index.rs:91`; `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs:21`; `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs:86` |
| T-158-07-01 | R | mitigate | CLOSED | Mutation-tested fixed current evidence, truthful scoped claims and actual complete default native success. Final native record and Plan07 summary directly corroborated against log and timing receipts. | `scripts/check-phase158-validated-reorg.ts:21`; `scripts/check-phase158-validated-reorg.test.ts:48`; `docs/parity/v2-5-validated-reorg.md:4`; `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-NATIVE-EVIDENCE.md:13`; `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-07-SUMMARY.md:93` |
| T-158-07-02 | I | mitigate | CLOSED | Published diagnostics bounded by categories/counters and finite finding output; no raw payload/credential dump. | `scripts/check-phase158-validated-reorg.ts:213`; `scripts/check-phase158-validated-reorg.test.ts:267`; `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REORG-MEASUREMENTS.md:60` |
| T-158-07-03 | T/D | mitigate | CLOSED | Explicit default checker/test wiring, no future-artifact prerequisite, clean independent review and current-attempt lifecycle-plan validation. Root enforced formal verification/final lifecycle before finalization; 21/21 formal truths and the full same-attempt lifecycle gate now pass. | `scripts/verify.sh:325`; `scripts/check-phase158-validated-reorg.ts:84`; `scripts/check-phase158-validated-reorg.test.ts:177` |

## Resolved findings and pending gate evidence

WR-158-01 is resolved for T-158-02-03 and T-158-03-03. The repaired guard (`storage/fjall_store/filters/reorg.rs:182`) permits a taller durable fence only through `proof.durable_displaced()`, after same-store/current identity, revision, owner, processed identity and actual coins checks. The unchanged predicate (`filters/append.rs:590`) compares the complete exact `(height, hash)` displaced tuple. The independent processed-frontier rejection remains. The next marker is recomputed from genuine staged ancestry.

Read the entire new test at `sync/tests/filter_index/reorg/branches.rs:259`: real A23 → B14 → A23 through production network reorg, no intermediate own coins/metadata flush, required B undo retained from genuine acceptance, processed/safe height 10 at rewind, unchanged actual A23 coins, exact active A and immutable B records, ordinary later flush and configured all-handle-drop reopen. The three corruption-only negative controls at `filters/reorg.rs:528/533/541` remove or mismatch the achieved displaced tuple; each first passes the remaining proof authentication and then demands the precise taller-fence refusal before effects. No production constructor, unconditional ancestry relaxation or forced flush was added.

The exact timing JSON receipts cited in 158-REVIEW-FIX.md were independently read: retained-source RED exit101; final production GREEN exit0; three negative controls exit0; full Phase158 suite exit0 (worker output: 102 passed, no failures, one explicit measurement ignored); strict node Clippy exit0. Both repaired file hashes match the fix worker and independent 158-REVIEW-FIX-CHECK.md. All 59 consolidated 158-REVIEW.md fingerprints also match current bytes.

WR-158-02 is resolved as supplementary protection for T-158-01-01/02-01/07-01. The checker masks only the existing exact `BasicFilterAppendProof.identity` `pub(super)` seam, then rejects every other `pub` token. Read all 35 sealed-field visibility controls and five alternate identity-visibility controls. The independent 158-REVIEW-AUTOMATION-FIX-CHECK.md records clean roots plus 40 separately rejected mutations; the parent/executor report all 449 checker tests green. The two final checker hashes match. This lexical checker evidence supplements actual Rust behavior.

**Open items: none in the declared threat register.** The complete default native pass supplies the previously pending T-158-07-01 evidence. The actual saved native report and Plan07 summary match the log, exit file and successful timing receipts. At initial mitigation closure CFIX-03 remained pending until the separate formal goal/lifecycle gates. Those gates now pass and canonical CFIX-03 is Complete, as verified in the final closure audit below.

T-158-07-03 gate implementation is verified without creating a circular future-artifact prerequisite. `scripts/verify.sh:325/326` directly runs the checker tests and checker after Phase157; the checker's fixed current roots omit future VERIFICATION/SECURITY existence. The clean independent aggregate covers all 59 source/test/script/doc files at current hashes. Root explicitly retains final formal goal verification and exact same-attempt lifecycle validation before requirement activation/Git finalization under Plan07's written gate. At this audit turn, the read-only command `node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs verify lifecycle 158 --require-plans --raw` independently returned `valid`, exit0. Its validator implementation is read-only and checks required plans plus lifecycle identity; `--require-verification` additionally requires the formal artifact when requested. After the formal report was produced, the final stronger invocation was independently rerun during closure and returned valid/exit0. Initial gate verification preceded that formal result; the final closure audit records its completed execution.

Two earlier native attempts remain failures: stale generated LOC stopped the first before Rust; the second hit two historical Phase103 Bun mutation-test 5000ms timeouts before Rust. The root-authorized one-file harness amendment splits the two grouped four-fixture cases into eight independently named cases with unchanged mutations/assertions, fresh fixtures and default timeout. Read 158-REVIEW-HARNESS-CHECK.md: clean independent delta, 14/14 focused pass and the same 16 assertions. Its final SHA-256 matches the native-tested file; the production Phase103 checker remains unchanged. The final default retry then passed every required gate.

No unresolved high finding or accepted risk is recorded. Formal goal verification and final `--require-verification` lifecycle execution were mandatory root actions after the formal verifier wrote its artifact; both now pass. Initial mitigation closure did not depend circularly on that future artifact. CFIX-03 is now Complete within the verified internal scope.

## Summary Threat Flags and unregistered flags

Plans01–06 SUMMARY files contain no dedicated `## Threat Flags` section or `threat_flag` entries. Their narrative limitations were read and mapped; no missing flag content was invented. This historical formatting limitation is retained explicitly. The final Plan07 SUMMARY has an actual `## Threat Flags` section declaring **None unregistered** and mapping both review fixes to declared controls. Its notes describe no added production network/authentication/schema/file-access trust boundary and infer neither accepted risk nor certification.

Narrative mapping: sealed/core authority → T-158-01-01; same-store storage/recovery provenance → T-158-02-01/02/03/04; accepted-versus-safe/persistence → T-158-03-02/03 and T-158-06-01; retained-input/resource refusal → T-158-04-01/02 and T-158-06-03; genuine fixture scripts/default maturity → T-158-05-01; prune/startup → T-158-05-03 and T-158-06-02; finite retained-fork resource-policy and claims limits → T-158-07-01. WR-158-01 maps to existing fence/cadence threats; WR-158-02 maps to existing sealed-authority/evidence/gate controls. **Unregistered flags: none.**

## Accepted risks and transfers

Accepted risks: none declared or approved. Transfers: none declared. Existing consensus staging/preview history copies, finite source-policy refusal, test-only compact maturity-one fixtures, software-only fault injection and deferred external serving are scope/measurement limits, not silently accepted threats.

## Verification evidence and limits

The auditor inspected the source and assertion bodies cited in the table, confirmed all 25 declared rows and dispositions, checked bounded diagnostic output/default verifier wiring, and captured SHA-256 fingerprints. No Cargo/Bazel/runtime behavior command was launched because root owns serialized target access and full native verification. The separate read-only lifecycle-plan validator returned valid/exit0.

Recorded predecessor evidence: Plan01 reports 17 reducer/engine tests, 328 chainstate tests and 12 doctests (including external compile-fail cases). Plan06 reports 98 Phase158 node tests, then 8 focused failure and 4 focused protection tests after scoped refinements; its separate explicitly executed timing experiment records 54 configurations and 149 turns. These are summary receipts, not executions performed by this auditor. The repaired final-source and complete default-native receipts below supersede the affected earlier snapshots.

Final complete default-native evidence was independently corroborated from `/tmp/phase158-native.log`, `/tmp/phase158-native.exit` containing 0, and the successful `verify-full/2026-10-09T03-59-49.916Z-d41180bd-1821-41b8-a17d-ad0f740bac81.json` timing receipt. The run started 2026-10-09T03:59:49.916Z and ended 04:19:15.143Z; verifier footer 19m24.571s/1,164,571ms, outer wrapper receipt 1,165,227ms. Log SHA-256: `9329d7223b43394587969f22b07ad43c0d53559237d95ae27718a90a2fd638a9`.

Actual normal Cargo aggregation before coverage: 40 successful unit/integration/doctest summaries, **3,735 passed / 0 failed / 3 ignored**. Node: 1,353 passed; the three opt-ins are public-network smoke and Phase157/158 timing experiments. No required behavior test is ignored. The genuine shorter-return and all three missing/mismatched fence tests show `... ok` in this full run. Phase158 checker: **449 passed / 0 failed / 849 assertions**; Phase157: 600 passing checker tests; historical Phase103: 14 passing tests with the same 16 assertions. Workspace format, strict all-target/all-feature Clippy/build, benchmark smoke/report, six Bazel targets and provenance passed. Individual format/clippy/build/test/Bazel/provenance/llvm-cov timing receipts all report exit0. The pure-core native helper enforces no `Uncovered Lines:` section and its coverage step succeeded; no unavailable percentage or adapter-wide coverage result is invented. Read the saved 158-NATIVE-EVIDENCE.md and 158-07-SUMMARY.md in full; their versions, outcomes, prior failures, scope, lifecycle and pending root activation agree with the actual evidence.

Actual tests distinguish preview from absorbed acceptance, retain old coins until normal cadence earns an own receipt, check every rewind/first/later append publication seam, and physically drop/reopen Fjall. Missing required body/undo refuses even when a filter row exists; valid shared immutable reuse does not replace consensus-required history. The fault matrix cannot prove hardware power-loss behavior. Prefix-independent *new index preparation/turn operation counts* do not imply whole-runtime constant work/memory or latency. Some fully retained deep/shared-gap reorgs intentionally hit finite preflight resource limits; parity documentation records this local policy difference.

## Audited byte fingerprints

All 67 final consolidated review file hashes were recomputed and match current bytes, including the independently reviewed closure metadata and historical harness deltas. Final native/summary artifacts were read and captured below. Any later source change requires targeted reinspection. Completion-metadata reconciliation after formal verification may update root-owned claims; it must preserve the verified scope. SHA-256 covers complete bytes.

| File | SHA-256 |
| --- | --- |
| `.codex/tasks/todo.md` | `25f16abe94b53052683c1377427dc069c4014e2bdd1ef195f783718836faa25c` |
| `.planning/PROJECT.md` | `234efcb4b0e5cd889296b1fa6c86c55c0d61fa689912852733ce427767b203a5` |
| `.planning/REQUIREMENTS.md` | `19f0cbacbb85c9cd94e2962bd2926747a6c0110bc489dd9e58137a7bae1a8ecc` |
| `.planning/ROADMAP.md` | `80627d7345205e052513fb1e7bdc08e75b48132794155b1f9cdd7694c8b0fbac` |
| `.planning/STATE.md` | `f197a653f42ffea106ab6c740418b08800021e7ad28d461d1c3ca76c30755ca1` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-01-PLAN.md` | `d4e5293bc3bcae59561a8092496ccac5114a271aff3be41b0704c311a6981b9f` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-01-SUMMARY.md` | `b88a2db0a4bc60ae4d319a59bdac5542a40c27d5f53d06a444bc70803dbcb112` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-02-PLAN.md` | `0900905d505126d4a914ae30d3fdb10efcfe580bae59d88a8313e055b5447878` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-02-SUMMARY.md` | `c8f4332b0e53ab17aca25cd8bb689c478a9770b0c63676255b2ed45b56cbef16` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-03-PLAN.md` | `0cb7b95aa2a92efbb79505183ade551d0795adfd2db329764e178ea1f807f9db` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-03-SUMMARY.md` | `1d4a6c53598adf5c375b4814cf0d5f11d9119a95e112677922bac4c021ab705b` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-04-PLAN.md` | `471f25666d7e4a8770aaa5e2561d8541122360b11862f33e465ae7739038f417` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-04-SUMMARY.md` | `483a580db521a88722cf42c34dd82527204782a5d853684b544fa105c091dc16` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-05-PLAN.md` | `6a4c7ee22efb7f27d8d4bbd5bad4665af6ccd2511451dcac5f595b6170a9be87` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-05-SUMMARY.md` | `5511e3abf58b1dc019d6434b70e7206a04136c6b486a314242d639ff2a7f03e6` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-06-PLAN.md` | `6e32782426bddb950e9809f2f81458400f5a283251abe69856aacb0d0156854a` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-06-SUMMARY.md` | `d040865cab8d08068123918315676d0c6446a202e42d3f476c1ebca1c180f2f2` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-07-PLAN.md` | `8cfd91836b1d79cc34663c3fabcf7e8917c5dac1e5ced02c9795a1ae5bb02f60` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-07-SUMMARY.md` | `16352b50def17ee3d9bb7f4a060e9823be00685b81d25d2d9fd7f0dc97c21add` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-CONTEXT.md` | `1b9919e77b03bf06898945e08a0ea72ff8461c960e123c89986d2ba97315e135` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-NATIVE-EVIDENCE.md` | `0562662d673b4a71105c62a39952705970a556beb267ea649aa37cb2038408ee` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REORG-MEASUREMENTS.md` | `9d0db0f4de5d57c3f734ae8b5be2223ee12272952dbf41b30a1b83d4994eb463` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REVIEW-AUTOMATION-FIX-CHECK.md` | `1a2084ce2ea71f3dcb217de966afaf631324f09c5793a4d66ca89e9618be39bc` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REVIEW-AUTOMATION.md` | `9ba08f701b7e03206c5c1569f5748a74d1b2d0d1ec49b6e675544300cf955b8c` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REVIEW-CLOSURE-CHECK.md` | `9ee3e68027dc1e413cff5005f11bf814fd64692776ffacff880ef1ce34794e33` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REVIEW-FIX-CHECK.md` | `f88deaf3144bd068c7685eb44c6602e76fb68ec3f656c8c6d60bcd05e9c3b1e9` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REVIEW-FIX.md` | `42db2247db4912b5dc777c49e7d72d25ff9966e85aba09d9d85c15ce18d66f65` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REVIEW-HARNESS-CHECK.md` | `5981e0f74c2ed8943d19e1d01180df0a1b45adf0fce6e663bc7e2b35b83d422b` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REVIEW-RUNTIME.md` | `f24901efeab93135173d3d352b32ff806abf87797f44131c5274f02cbf166cb0` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REVIEW-STORAGE.md` | `1b83c4d9b43ac93abd322eb47abb75ce4c8f0cccd1805d790edbd384724c88fd` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REVIEW.md` | `91f9da071656917f7acce5aeec5e77caa75c54e197ee4242d7629df0cb70da78` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-UAT.md` | `400b57813d74210865458d94c2c7b0b2903b058ac91f6bc8ba4360ce18abd599` |
| `README.md` | `3e6804c35bca8f52b6e28dd15ca2c8b910dfde0de9105a8ef75861e24e6d996a` |
| `docs/parity/index.json` | `3411caff99b127816e897ace1b6a538cd0499d894fafc16a1f19d1d87f483bb4` |
| `docs/parity/source-breadcrumbs.json` | `02c2b000bb46934ed4261185f4ff86551d16d4e634f1079df46bdf077eefb67d` |
| `docs/parity/v2-5-validated-reorg.md` | `80e4ea951a4794fe60a33184fe5922926d4c4bc1356c3f725a79250729c912b5` |
| `packages/README.md` | `3cbbdcb8e07c22c53c122cce0f808c44d919e3be4ae83fe03067c959293af023` |
| `packages/open-bitcoin-chainstate/src/engine.rs` | `ce642b9f3c4f266c09712550f66acc53d8038f11da1463cef65c79effa7b69a7` |
| `packages/open-bitcoin-chainstate/src/engine/stage.rs` | `cc04325a42f7f45ce8d68f5a39300d2af89398842232e3fec7586bcf529c18e8` |
| `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/staged_connect_and_reorg.rs` | `b918d51b6c6a95857d4e1897ca6828a73529a0639bf7b163e2963fd968fc7518` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs` | `332b402ce454f35cc88eba2e0d9b11fa294a53b1d41a9a9f656ba9ea69cef503` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up/reorg.rs` | `a37403ada7a4ffcdd0bceee5a4f3f388404211f924c97b780c52ad216da26abf` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests.rs` | `8f9e7102c97e66474bcb48e647065c34b77431857167c7921e63195422d96492` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/reorg.rs` | `22d74c633756ba3d91cadaf2f878b7f2fa33a6bf44fde21c4c201de017e8fe73` |
| `packages/open-bitcoin-chainstate/src/lib.rs` | `5f8153f6b948ae2871ecd066ff091f8dd444b3a228eed7c5a00c6f3c0fc5738a` |
| `packages/open-bitcoin-node/src/chainstate.rs` | `3e04ceb45cfc3affdaae01ebd4ab791cd7b82110738e26e910b5242b14eb19f1` |
| `packages/open-bitcoin-node/src/chainstate/filter_index.rs` | `9902f1021730914bfffa4b1b99764dcf8350ebf20d33b3ed777808c98a86ec0d` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg.rs` | `cbde61693ef860a2210b4a7261b74f095a9c60aae3771cf0775c0f9acd1134c5` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg/preflight.rs` | `b7adbc9f1ec32d45e36429e2702fde89409050268b22f084bf2f5b5ede33d377` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg/tests.rs` | `43fd99edb144250f6c10b59781993a4a7189bfc424d92d4b25d540012961b32a` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg/tests/faults.rs` | `b29db3b5927f81879c2ef90552775ad6ed65f9926d486b339dcc6dee6e5aa357` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store.rs` | `322b973e2f687d6ff4c7b9d06d46682a4723f499115da9d53d4f412d71d2353b` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/reorg.rs` | `41755114f92096de344dd0e2f987e324d713d1d9a4f3195700c192d1a644268a` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/tests.rs` | `951ec9688e42561d353dd724540a924dc28d63a0db9ac7d6da26b50d905e1fdd` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs` | `9f1115b019ae6846d251b71f2543c15e82721db1db782e555d6eca5275e9eecb` |
| `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` | `d82c02194ec2bd0de54d16e4bb006f43a498c172a3d2a1c15ff79377f8fec16e` |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs` | `051cc7f80d4b937f17086197571abbb61f65f936f184e7c82dbea72296da934c` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs` | `f13b4e896dd803cf640dbdbafba13b86bb5b8c12921c7662563d90b193010f0b` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs` | `935a82ecc7838fe0a109180a4a2961dc492f787a06b36537c2d7274bbdafd2c4` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs` | `cc6f28cdea3a16517fac0bffa0b7c267ea2765db3163cf80fbd9501c068e4838` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests/faults.rs` | `4b065e69f0aeff73a974f6e4c93d1d5a932aeb3a65d74c34b8ec2838f0637497` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs` | `89104de96b6cca93e3b5321027786ea9570dd610ecf5a1d1e40de51c10f01eb9` |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | `a3bad66bbf4d848fd9c9ac0d01a4c556586cb34ef66c3804506f8187cd906ee1` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters.rs` | `91a61729172f33581dc6b7ff652effc96faf80e4d367c24e4241f5ffcc5ed442` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs` | `83bbaa80175bd9cd05b7a4e9b638faa7e0be81e89723b7793b2041b17883c69e` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs` | `379442b8623ad8f517de892ceace6603c01267f9c8460fe0b22b25420b5329d9` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership/proofs.rs` | `1298113df10505036770b40ed560853ed7d94513801408e4390b0384fc7a8542` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs` | `1c98f0b8ad4c7738befecff4dbdb2b76b1ca0f732a606ead5bc70fc51da72f41` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs` | `f85e0da039b66b24669b3a085cca2d5c70889d5d28ef5b0098439f37b581e715` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs` | `6ed895afa1d1ac0126019ef6dca79d97a2bc81b53031cd8f6010c2fb84813d07` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs` | `625d2a2932a2a7fe470fcde30f63fe8010355ad661444111d2fe854a10d40b41` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg/faults.rs` | `f767df6bd156930b1c625c49916ae6e9f8e014ff99ab514a0f069fe1e2fa43b7` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs` | `81c363d712cf6aff21b81f5cbc691acdfc520fd17368c02a2fd4d1e3be45db79` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs/undo.rs` | `923371e2e2207190afdd1fbab69e6ae9970d556ea44a4ca3d10b3d66da6e15e7` |
| `packages/open-bitcoin-node/src/sync/block_reconcile.rs` | `1c2088e1d39572e91d28247993f3bf73f234beb8200895969daf597ae1d0319d` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index.rs` | `0a0fd9b0a775c55b8c598f2abc410fcef6f255c6afd431c335405f0b95a18b7d` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs` | `5c2a5ba1c93ded830ed969e04ee7682dd3840a38c31652c6afb2a5fc4dd66bc8` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg.rs` | `597b2276dc5b5719c67fa555fcb5578c2bc49f0ab954e83852c7378127330c49` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs` | `a9bcf1d5e3de9e719f29cbff8e6fac29511dc3571c1326b52cd880497e23654e` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs` | `f450a4741c0da057e7a2e0f941eb8cae2e26515ee4ce49720ed8366e86b8b100` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs` | `5dcec3e9bbf7380e82258bd9e1e05e0fc9d60d469104fd33ee11b3803e02b0b0` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs` | `7a8e1d798bc974fc0123ab2a8355453149e081ad404a5d4f772f1eacfb2c6a74` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs` | `2cdc131e3c7586d523e36924fa8c095e4f95b3da5f6c497cfb5bb5e4fb87810f` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs` | `8a3f4e94741796c8071277c5a2c732204631ff828a0a1d5f2e811baa35e91eae` |
| `scripts/check-phase103-mempool-lifecycle.test.ts` | `05991df40afc4d2d515da813443e343c27de0289ceb38d727796aa71465dd458` |
| `scripts/check-phase157-index-catch-up.test.ts` | `53d897f7720f9a2989399a641956058f3937fa1507f39e02e8e605c182d60e6c` |
| `scripts/check-phase157-index-catch-up.ts` | `0b5890ac87f4d27e0fbeb5f0e28c36119045ade3db92853ca5deacaaca9d7ad0` |
| `scripts/check-phase157-index-catch-up/contracts.ts` | `eeaa568a615924dacdc253da4551203e7c31ebe25f40b0ec5d63936f102256f1` |
| `scripts/check-phase158-validated-reorg.test.ts` | `05d98dcd2abf30dad2df49a2f5ab058fbf60d070d76ea38e88b6c65f033ef109` |
| `scripts/check-phase158-validated-reorg.ts` | `e9b8ae3ddc7a6ccaffa67a0b5bf7b7614eae7938d3cc1829de4cd2ca9aaa64e9` |
| `scripts/verify.sh` | `d9c6fbff98535c283c3adc01726ea18f58e159c541e01bc27a238414520af3fc` |

## Final closure audit | 2026-10-09T04:33:48Z

Read the independent 158-REVIEW-CLOSURE-CHECK.md and final claim/metadata deltas; all **67** consolidated reviewed-path hashes match. Exactly five of the original 60 paths changed to reflect earned completion: root/package READMEs, parity page/index and UAT. The other **55** native-tested source/test/script/doc paths remain byte-identical. The seven additional reviewed paths are closure tracking/evidence; no new production trust boundary or mitigation change appears. Updated all affected fingerprints above; no Cargo/Bazel, behavior test or Git operation was repeated.

Canonical REQUIREMENTS has CFIX-03 checked and its unique Phase158 row Complete; direct counting finds **10 Complete / 12 Pending / 22 total**. ROADMAP/STATE agree on **5/9 phases and 33/33 created plans**; the plan percentage is distinct from 56% phase progress. Parity has one pinned CFIX-03 owner with status done and preserved finite-cap, fixture/halving, software-fault and deferred-serving limits. Plan07's root closure and completed frontmatter supersede its historical pending handoff; native outcome/receipt remains unchanged. No commit/hook/push result is invented.

Read formal verification: passed, **21/21**, exact originating lifecycle, zero overrides. Independently ran the read-only `node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs verify lifecycle 158 --require-plans --require-verification --raw`: **valid, exit0**. Formal-verifier canonical-status prose refresh is owned separately and adds no production change. Phases159–162 and external RPC/peer/operator/integrated-client products remain pending/nonshipped; existing performance, hardware and production/funds exclusions stand.

**Reconfirmed: SECURED, 25/25 mitigate dispositions CLOSED, threats_open: 0, no unregistered flags or accepted risks.** Git staging/finalization remains root-owned.

## Report formatting audit | 2026-10-09T04:35:02Z

Root normalized trailing whitespace and extra EOF blank lines only in 158-REVIEW-RUNTIME.md and 158-REVIEW-FIX-CHECK.md, then obtained a clean staged diff check. Refreshed exactly those two report digest rows. All 67 consolidated reviewed paths remain byte-identical; no implementation or evidence assertion changed. Mitigation result remains **25 CLOSED / 0 OPEN**. No tests or full re-audit were repeated.
