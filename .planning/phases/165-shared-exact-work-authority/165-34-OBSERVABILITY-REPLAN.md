# Phase165 Plan34 observability repair after the actual first native failure

Lifecycle:165-2026-10-14T16-09-59Z. Mode:yolo. This bounded planning transaction adds only165-34-PLAN.md and this ledger. No implementation, other plan, root metadata, historical evidence or Git mutation is authorized to this planner.

## Actual failure and authentic before

Full default `bash scripts/verify.sh` genuinely exited1 at Phase72 on2026-10-17. The verifier reported4163ms internal duration; wrapper receipt records4679ms. Failure receipt `.local/open-bitcoin-dev/phase165-final14-native-first-failure.json` SHA256 `fdf2396d3f8603c144c5ef79c67020eb41df554f89f1b0ee54677371e669b914`; complete2278-byte raw log `.local/open-bitcoin-dev/phase165-final14-native.log` SHA256 `0c021085f6a949259701fbcc27be80874e0f34a2a7ee072ed95a2dd164a416e1`. Four diagnostics require840005 for CLI human status, Dashboard projection, RPC durable sync status and Metrics and structured logs. Their actual literal-bearing Rust assertions require `00000000000000000000000000000000000000000000000000000000000cd145`.

The run passed the shown early breadcrumb/release/61–71 checks, including the current Phase68 exact-carrier tests. It did not reach later guards, full Rust tests, coverage or Bazel. Initial native failure is retained as failure regardless of later repair. Source308/docs8 pre-native review scope and prior receipt scopes are separate; no all-native GREEN exists.

Authentic pre-replan receipt `.local/open-bitcoin-dev/phase165-final34-planning-before.json` SHA256 `7ecfc37a20b6bde7ee4d10741164356d997ec0a8a19982e212bcc4fd714e9cdb` contains complete seven before bodies, not a diff reconstructed after edits.

| Before path                                                     | Bytes | Lines where measured | Raw SHA256                                                       |
| --------------------------------------------------------------- | ----: | -------------------: | ---------------------------------------------------------------- |
| .planning/ROADMAP.md                                            | 19143 |                    — | 0d26455710af8030ab840ae17d62a83b4f9d0fbc6f2128f63d9c983c1d1802d7 |
| .planning/STATE.md                                              | 81363 |                    — | cc2d383fddd2b04a36e27e93d560258bf196b41509d4e05ab7ceb74bc211e605 |
| .planning/phases/165-shared-exact-work-authority/165-14-PLAN.md | 22951 |                    — | efb681d9bb74d6c2b7a003b6a6d58a3b04cfb9086f06f2c4c0b6d2d3ba730aee |
| scripts/check-phase72-observability-evidence.ts                 | 21049 |                  546 | 82a238ae1320ae3f3b6ca4792742dadcd034a6f3009d59d92cad70fb3b2ee0c8 |
| scripts/check-phase73-uat-verification.test.ts                  | 15030 |                  447 | ee9a62a71d2f72c5607732e94bf2189f9028badfda8f6c37ae55e94518daa104 |
| scripts/check-phase165-shared-exact-work/contracts.ts           | 85812 |                  618 | 64008967e4446d07b70e6514d2bdfa7e99ae44b911da2d2866992f016e854c8f |
| scripts/check-phase165-shared-exact-work.ts                     | 14766 |                  269 | 9adb6a82bbfe62f8624cc008000d3d0409a4e4d08079c8eb54c580250d13ac5c |

## Root cause and bounded correction

`sharedStatusSnapshotFixture` supplies decimal840005 to both current and raw fixture comparisons. Only four current live matrix validated-work slots are stale. Raw support evidence in `packages/open-bitcoin-cli/tests/operator_binary/support_sync_soak.rs` retains decimal at144/148/228/240, as do smoke `scripts/test-run-live-mainnet-smoke/fixtures-daemon.sh`72 and `assert-report.ts`89. Changing the shared fixture globally would break faithful raw evidence and hide the distinction between a numeric carrier and authority.

| Current live matrix row     | Actual unchanged Rust evidence                                                          |
| --------------------------- | --------------------------------------------------------------------------------------- |
| CLI human status            | packages/open-bitcoin-cli/src/operator/status/render/tests/inbound_and_sync.rs449       |
| Dashboard projection        | packages/open-bitcoin-cli/src/operator/dashboard/model/tests/sync_and_recovery.rs83/331 |
| RPC durable sync status     | packages/open-bitcoin-rpc/src/dispatch/tests/sync_status.rs412                          |
| Metrics and structured logs | packages/open-bitcoin-node/src/sync/types/summary/tests.rs222                           |

Plan34 changes exactly four existing TypeScript owners. Task1 exports the existing pure matrix, guards only final `await main()` with `import.meta.main`, appends six meaningful AAA cases to the existing native-registered73 file, observes real four-fail/two-new-pass/11-original-pass RED, then binds exactly four live interpolations to one explicit canonical64hex constant. Independent expectations compute `(840005).toString(16).padStart(64, "0")`; complete text/json! arrays are compared, including RPC field key. Two raw cases retain complete support/smoke row shapes and decimal connected/validated work. All original73 bodies, all eight matrix rows, other fields/paths/main checks/source-corpus behavior remain exact. Direct72 CLI still demonstrates the four diagnostics at RED and must pass at GREEN. Silent import and actual17-case full73 execution are required; compile failure/zero discovery cannot substitute.

The newly public pure matrix receives one succinct JSDoc explaining returned source expectations/no verification execution, included in its complete source inverse. Private helpers/main and all eight rows otherwise remain exact except the four work substitutions. The import command demonstrates exit0 and silence only; complete top-level static inspection proves no checker verification/source-corpus reads are invoked on import. Ordinary Bun module-loading I/O is separate.

Task2 reviews the complete actual four bodies/semantic correspondence/RED-GREEN/logs and full two-source inverses BEFORE source digests. Only then derive an inert proposal for two ordinary `ownership` SOURCE_CONTRACTS rows and the actual ordered INPUT_FILES pin. Initial158 SOURCE/88TEST/277INPUT become measured160/88/279 if the bounded assumptions hold. All old rows/order, old164 arrays/edges, historical/corpus/adapter/inverse data stay exact. TypeScript owners get no Rust TEST row. Contracts618 has10 physical lines of room; two readable rows fit, with no source relocation, new mutation table, cap exception or627-line current165 test edit.

Root reviews rows/pin/order/full pin-file inverses before activation. Apply rows first and observe actual current165 CLI stale-pin refusal before fixed loadInputs reads, explicitly separate from import-time activity; only then activate actual derived inventory literal. External bounded new-owner loaded-map controls must produce each ordinary ownership refusal against a passing actual baseline. Existing enabled Rust-owner/file-disable/inventory controls remain intact. Exact full-file inverses restore all four complete before bodies, with no unreviewed byte delta.

## Dependency graph and final14 boundary

| Plan/task              | Needs                                                    | Creates                                                                   | Checkpoint                        |
| ---------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------- | --------------------------------- |
| 165-34 Task1           | Accepted30 plus authentic final14 failure/before sources | Import contract, six real cases and exact four-live repair with RED/GREEN | None; yolo/internal source review |
| 165-34 Task2           | Task1 actual complete source review                      | Reviewed two-row/pin activation, raw closure and actual Summary34         | None; root acceptance             |
| 165-14 remaining tasks | Accepted actual Summary34                                | Entire fresh full native/review/security/formal/normal-hook closure       | Existing contract                 |

Plan34 is wave33, depends_on165-30. Proposed root-only final14 amendment is wave33→34, depends_on165-30→165-34, plus context references to actual34 plan/ledger and required future actual Summary34/root acceptance. Preserve ENTIRE existing final14 Task XML/actions/automated commands byte-exact; do not rewrite its earlier completed documentation or failed native evidence. Root metadata alone may reflect34 plans/32 actual summaries pending34; no fabricated Summary14, completion checkbox or lifecycle transition.

All existing33 plans otherwise retain content, graph, requirements and lifecycle. Task1/2 share a sequential source-review→pin dependency inside one two-task plan; no parallel mutation or fifth owner is required. The graph extends the same Phase165, without166 work.

## Exact evidence closure and acceptance

Root independently reviews a new external collector/driver/freeze before proportional execution. Preserve every709 accepted30 raw path and union actual34 source/provenance/planning/review/receipts, four critical Rust dependencies, complete source-corpus paths including support/smoke descendants, actual fixed/static module/helper/factory/CLI and observed dynamic reads. Record canonical paths/presence/bytes/full hashes and optional absences. Existing14-module CLI closure must be measured; new73→72→source-corpus test edge is part of the real test graph, not silently omitted. Changed existing raw entries require precise reviewed attribution, never a semantic projection. Freeze before commands, check before each command, close every entry/log unchanged; root accepts exact collector/read set and complete proof.

Proportional checks: direct72 CLI; all17 actual73 cases; current165 actual baseline and all existing inventory cases; externally recorded two new-owner tampering/disable controls; live164/165; fast158/162; breadcrumbs; managed standards; four-file628 caps; whitespace. Preserve actual counts/names/argv/exits/durations/full log hashes. No separate full20-minute164/165 matrices or Cargo build repetition are part of34. Both fresh full matrices and whole Rust remain final14 obligations, alongside ENTIRE fresh default `bash scripts/verify.sh`, actual generated LOC freshness, fresh coverage, Bazel, independent whole-phase code/security/formal review and normal-hook commit/push. This proportional plan cannot satisfy or weaken them.

No implementation/retry, pin activation or source acceptance is claimed by this planning ledger. Stop on first unexpected failure and preserve it; after two attempts at one blocker, return for bounded replan. Summary34/root acceptance must precede final14 retry. No old failed/partial/source-only receipt qualifies new native bytes.

## Decision and requirement coverage

All decisions remain Full across the retained phase plan set. Existing implementation remains unaltered;34 strengthens the evidence boundary where applicable.

| Decision | Existing plan/task ownership                 | Plan34 preservation/contribution                                                 | Coverage |
| -------- | -------------------------------------------- | -------------------------------------------------------------------------------- | -------- |
| D-01     | 01/03/07 arithmetic/carrier/provenance tasks | No production arithmetic/dependency change                                       | Full     |
| D-02     | 01/02/03 exact carrier and boundary tasks    | Task1 exact64hex current observation; decimal raw fixtures remain explicit       | Full     |
| D-03     | 01/05 target/admission tasks                 | Rust source unchanged                                                            | Full     |
| D-04     | 01/12/13 independent evidence tasks          | Tasks1/2 independently computed expectations and actual source-vs-runtime scopes | Full     |
| D-05     | 04/05/09 header/selection tasks              | Task1 corresponding current live work evidence retained                          | Full     |
| D-06     | 06/08/09 candidate-order tasks               | Rust/behavioral owners unchanged                                                 | Full     |
| D-07     | 03/07/08/09/12 pre-mutation authority tasks  | Task2 all existing current owners/controls retained                              | Full     |
| D-08     | 04/06/08/09/12/13 bounded tie tasks          | Task2 evidence does not invent restart parity                                    | Full     |
| D-09     | 10/11/12 filesystem refusal tasks            | Tasks1/2 legacy/raw and unavailable evidence unchanged                           | Full     |
| D-10     | 02/03/07/10/13 legacy representation tasks   | Task1 raw decimal remains explicit, never promoted to live authority             | Full     |
| D-11     | 10/11/12/13 refusal/support tasks            | Tasks1/2 support dimensions and every original guard retained                    | Full     |
| D-12     | 02–05/07–13 scope tasks; final14 acceptance  | Task2 final14 still full; later166–168 ownership unchanged                       | Full     |

CWOR-02/CWOR-05 are contributed evidence scopes for34, not newly completed requirements. CWOR-01/CWOR-03 remain covered by existing unchanged plans and final14. CWOR-04 remains Phase166;34 includes no checkpoint/reopen/bootstrap/context/actual-daemon qualification or future outcome claim. No deferred decision is implemented or reduced.

## Threats and planning checks

Plan34 STRIDE entries T-165-34-01 through05 all mitigate: exact independently expected four-live values; unchanged raw support/smoke semantics; import/directCLI execution separation; source-before-pins/ordered fixed reads/full inverses; complete evidence closure/resource caps and final14 separation. No existing threat disposition is removed. No new credentials, network operation or production service interface is introduced.

Constraint analysis identified the shared raw fixture as the hardest boundary; Task1 proves it before pin work. Pre-mortem failures are global conversion, test import suppressing CLI execution and reviewed hashes blessing unintended edits. Six behavior cases/directCLI checks/full source inverses catch them, respectively. The simplest robust correction is one export/main gate plus one constant/four substitutions, followed by two ordinary rows/one pin. No generic formatter, new representation abstraction or larger audit is warranted.

Planning used the gsd-plan-phase skill/workflow and agent schema, repo-local AGENTS/sidecar, placeholder-only overrides and architecture/code-shape/verification/testing/Bun standards. Both complete active lesson files total7188 bytes/2397 estimated tokens; no archives. Level0 discovery uses actual existing source contracts/research/native registration, with no external library choice. Root retains all metadata/commits and independent checker routing. This ledger and Plan34 alone are this planner's outputs.
