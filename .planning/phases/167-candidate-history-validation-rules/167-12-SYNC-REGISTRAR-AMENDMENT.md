---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T04:30:00Z"
commits: false
---

# Phase 167 Plan 12 sync registrar amendment

R1 closes the independently identified child-worker full-replay blocker while retaining the conventional sync registrar. The current proposal has thirteen source owners: the original nine, sync.rs, existing inspection/worker.rs and inspection/protocol.rs, and new inspection/protocol/history.rs. Both original tasks, every original action/acceptance/verification command and the29-plan/50-task/24-wave schedule remain. This is not a checker result, source acceptance or execution release. Independent R1 checking and root release are required; Plan12 also waits for root acceptance of11.

The sections below through “Planner observations awaiting independent check” retain the initial R0 proposal as historical context. R0 received ISSUES FOUND with one blocker; its ten-owner closure and production-child assumption are superseded by the mandatory “R1 child-worker execution contract” at the end. No successful R0 structural check waives the blocker. All original draft/checker/receipt bytes are separately preserved in revision0-before.

## Source-grounded need and exact ownership

The existing `packages/open-bitcoin-node/src/sync.rs` declares `mod canonical_genesis;` and the other conventional sync children, but does not declare the already-planned sibling `sync/canonical_history.rs`. A file alone is unreachable. The full new source allowance is:

```diff
 mod canonical_genesis;
+mod canonical_history;
 mod live_fixture;
```

Add only `packages/open-bitcoin-node/src/sync.rs` to Plan12 frontmatter and Task2 files. Its baseline is567lines/21186bytes, SHA256 `a3ad8c32886cea6ae778fa5ec79d2775817e57e2f680d79d288ae406eb76e599`. The isolated proposed delta must be exactly the conventional declaration in the ordinary module section, with necessary standard placement only. No unrelated formatting, refactor, visibility/re-export, inline-path module or lint allowance is permitted. The expected source is568lines and must remain formatted below629lines. Execution must retain complete before/after bytes, exact patch and an inverse removing that single declaration that restores the entire original source SHA256.

The original nine owners remain unchanged: inspection parent/child/tests, storage/fjall_store/canonical_genesis.rs, sync/open_runtime.rs, sync/canonical_genesis.rs, sync/canonical_history.rs and its tests, and chainstate/fjall_store/canonical_genesis.rs. The original scope is not replaced. Explicit scope warnings are ten total owners and six Task2 files. This narrow cohesive registration is below the existing15-owner hard boundary, but the independent checker must assess the warnings. Any other required source/test/registrar owner or overlong formatted source stops execution for another checked amendment.

No new task, prerequisite, dependency, wave or lifecycle is added. Plan12 stays wave15 with dependencies167-08/10/11. Its registrar has no same-wave file conflict. All other28 plan files and all original lifecycle/schedule/requirements fields remain byte-exact. No deferred decision is implemented.

## Existing APIs and proposed checked assembly

These are source-grounded proposed API uses to check during execution, not implemented or passing evidence:

- `lib.rs` already calls `register_inspection_worker!()` in the unit-test root. New node unit-test modules use that same executable and exact existing worker dispatch; no lib.rs owner or second worker is needed.
- Already-owned `storage/fjall_store/canonical_genesis.rs` can define private-field pub(crate) opened/recovered history wrappers. The actual private `ProvedHistoryRecords` stays inside storage. A consuming wrapper method must validate the same live instance/sequence/source and return the actual earned public `CanonicalGenesisProof`, `HeaderStore` and core. Sync uses the full existing module path; no storage/fjall_store.rs parent re-export is required.
- The existing `storage::canonical_bootstrap::validate_history_records(selected, records)` captures fallible shell SystemTime and calls full replay. Route copied inspection and immediate original reproof through this real entry and consume the actual source-bound proof. Pure direct proof tests or textual calls do not constitute production integration.
- For the managed core, obtain the earned replay's active `admission_snapshot()`, construct the target with the real held coins view, then consume `rebind_into`. That existing operation compares complete coins, best block, active positions, active undo and complete occurrences and transfers actual canonical work/rules ownership. Do not construct the target from the physical union of active and retained orphan undo.
- Within the sealed checked sync assembly, `ManagedPeerNetwork::from_initialized_chainstate(...).into_canonical_scope(selected)` can install the actual replay-earned HeaderStore with `peer_manager_mut().seed_header_store`. Then call `retain_checked_candidate` with the actual proved active-tip body, selected params and actual held HeaderStore observation. It must execute the real linked-parent eligibility check, cache that actual body and bind the real consensus params before `finish_initialized_runtime`. Assert actual checked/cached outcomes so its disconnected-parent early-return cannot masquerade as success. No raw DTO/entry seeding, dummy setter/consumer, manufactured time/context or global support label substitutes. The genesis-only constructor retains its exact strict single-genesis boundary.

The current six-source11 basis is retained in the snapshot. Its proof.rs final hash after the authorized single-character coercion correction is `94761ed26a0adf309eebe10c0206d1be557632ed1914214516103262cc79a477`; the other five hashes match `plan11/retained-graph-implementation-check.json`. That old receipt describes its exact earlier proof hash and remains historical. The new correction and its live close receipts belong to11/root; they are not frozen by this planner.

## Complete retained-fork and pre-effect obligations

The current11 provider proves every retained ScriptsValid branch through one bounded event stack, actual re-admitted HeaderStore-held clock, real core connect/disconnect, full generated undo, status/provenance and BASIC. Its source/generation-bound consuming handoff returns those actual earned genesis/header/core owners. The306 total known/active/accepted-node envelope covers305-known/303-active checkpoint plus306-known/304-active next block. The checkpoint physically retains304undos while the returned active core has302. All fully proved physical orphan rows stay stored; none are injected into the live core.

Plan12 must actually release every handle, reopen this retained-fork product checkpoint through the copied-worker/original reproof path and accept the next valid block with full exact state/work/coins/occurrence/active-and-retained-undo/provenance/BASIC comparisons. This is part of its original cold-reopen obligation, not a new differential phase or a waiver using Phase169. Genesis+301 and every original positive/refusal/tamper case remain mandatory; the actual original source manifest must remain equal on pre-effect refusals.

Preserve the original whole-copy worker, lease, source-manifest equality and immediate original reproof before recovery/config/index/prune effects, with exactly one original live Fjall owner. No raw original database fallback precedes compatibility. All actual family membership/count/order/generation and complete-state bounds retain65536-byte authority rows,4096entries,512MiB and30seconds. No repair, import, migration or source cleanup is granted. The source remains frozen during this planning amendment.

## Strict consumer-stage gate and attribution

Read the preserved `plan11/root-provider-staging-adjudication.json`. Root classified the24 current private-provider dead_code diagnostic groups as temporary staging for the as-yet unwired12 consumer. Original11 strict Clippy failure stays FAILED. No allow/expect, visibility expansion, dummy consumer or erased diagnostic is authorized; no strict pass is inferred from provider tests.

After real production wiring, Plan12 must additionally run this unchanged strict node all-target/all-feature command through the existing pinned Bun PATH and cooperative timing lock, and require exit0 before acceptance:

```bash
bun run scripts/command-timings.ts run --key phase167-12-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings
```

Both original task automated commands remain exact and require nonzero named test discovery. Final summary and receipts must bind accepted11/provider hashes, all ten final12 source hashes, every original/additional command outcome and failed attempt, copied-worker complete physical/logical/row/time measurements, original-source manifest equality, actual all-handle retained-fork reopen/next evidence and the complete one-declaration registrar inverse. Preserve earlier source/security/goal/native outcomes with their own actual hashes/evidence classes; no result retroactively validates different source. Plans17/20 still own compact-fallback adjudication,17 source-before-guards ordering, and20 full native/fresh pure-core coverage/Bazel/security/goal/lifecycle. No phase or whole-requirement acceptance is granted here.

## Baseline, inverse and planning checks

New ignored evidence lives only under `.local/open-bitcoin-dev/phase167/amend12registrar/`. Before mutation, `before-manifest.json` captured all29 plans, the current entire canonical PLAN-CHECK prefix,38 protected historical phase documents including accepted28/29 summaries,270 historical receipts, original12 source/existence states, the added registrar and read-only API providers. Live11 close receipts/SUMMARY and shared live STATE/ROADMAP/todo/config are explicitly excluded. Their ordinary progress is not an immutability failure.

Original Plan12 is151lines/16713bytes, SHA256 `e3808ea90486b3b25d7cb6ebfb2863eda3e08f93d1db314759eca9b5090296f3`. A complete inverse must remove exactly the owner additions, the new tagged context and the added strict integration verification; it must reconstruct the full original file byte-for-byte. Task1 stays exact; Task2 differs solely in its files list. Original actions, behavior, acceptance, done/read-first/threat model and both original task commands remain exact. Every other plan inverse is identity. Separate proposed-source inverse evidence is an ignored candidate only; no source in the checkout changes.

`planner-validation.json` records frontmatter/task validation, complete plan inverses, commands/actions preservation, unchanged historical inputs/source basis, lifecycle/requirements, dependency ordering, all29plans/50tasks/24waves and zero same-wave owner collisions. `source-proposal.json` records isolated rustfmt/line-count and full registrar inverse evidence. `markdown-checks.json` records actual formatter outcomes. These are planner checks only; the independent checker must still assess semantics and append its own result. No self-approval or canonical report append is permitted.

Original Plan12 already fails mdformat check at this baseline. Preserve its bytes and record its baseline/after formatter difference rather than normalize original plan text. Only this new amendment may be formatted. The installed mdformat1.0.0 has GFM1.0.0/frontmatter2.1.2 support; a failed probe through the system Python found no package metadata because mdformat uses its separate existing pipx runtime. That tooling lookup is not a source/test result.

Local AGENTS/sidecar/overrides, architecture/code-shape/verification/testing/Rust and bounded active-lesson guidance informed the amendment. Both distinct active lesson files were loaded within8856bytes/2953estimated tokens; no lesson/source/guard edits or new dependency were needed. Existing source interfaces and accepted history satisfy internal Level0 discovery. GSD init confirms29plans, the five CVLD requirements, yolo lifecycle and disabled Nyquist; root-only saving overrides generic commit_docs. No Cargo, Bazel, production build/test, source edit, staging, commit, push, guard renewal, independent acceptance or execution release occurs here.

## Decision coverage and simplification

Full means preserved full planning coverage with existing owners, never executed acceptance.

| Decisions                | Plan12 task coverage                                | Status | Preserved obligation                                                                               |
| ------------------------ | --------------------------------------------------- | ------ | -------------------------------------------------------------------------------------------------- |
| D-01,D-07                | Task1 and Task2                                     | Full   | Selected immutable scope and real earned sealed owners; strict genesis path.                       |
| D-02,D-03,D-04,D-05,D-06 | Accepted prerequisite contracts and unchanged Task2 | Full   | Full actual replay/context/scripts/subsidy/maturity/work/undo/count rules without new calculators. |
| D-08                     | Task1 and Task2                                     | Full   | Exact nonmutation and genuine all-handle retained-fork reopen/next with full state.                |
| D-09                     | Task1 and Task2                                     | Full   | Attributable source/receipt inverses and original resource/source guards.                          |
| D-10                     | Added strict gate; existing17/19/20 owners          | Full   | Real consumer strict success and final independent/native/coverage/Bazel/lifecycle gates.          |

The simplification pass selects one conventional declaration and the existing storage/core/network APIs, retaining one live Fjall owner and no new proof factory or network setter. A genuine extra owner is a stop condition, not permission for silent expansion.

## Planner observations awaiting independent check

The scoped metadata/preservation run finds no errors: all29 frontmatters/task structures,50tasks/24waves, all29 complete inverses, all original actions/acceptance/task commands, other28 plan files,38 historical documents and270 historical receipts remain exact. All read-only source/existence states and the captured canonical checker prefix remain exact. The only inherited GSD structure advisory is Plan02's independent producer at a wave greater than1. Scope warnings remain ten owners and six Task2 files.

The ignored source proposal formats without any additional change:568lines/21209bytes, SHA256 `adb5ee133d03c2c9a942c80912c8f43966dc803a99797406d962a2c236eafada`; removing the one declaration restores the complete original registrar. No checkout source was formatted or changed. New amendment Markdown and both scoped whitespace checks pass. Original Plan12's inherited mdformat miss remains, and removing only declared additions from the formatter-normalized amended copy restores the entire formatter-normalized original SHA256 `09d074b669a2d98ec9465d270deeae36ee3fb70534d2032123f7c113c68286d1`. Thus no new formatting residual was introduced. These observations grant no semantic approval or source execution release.

## R1 child-worker execution contract

### Actual blocker and preserved history

The independent R0 checker found that existing inspection.rs::with_inspection delegates raw recovery/serialization to worker::recover, then invokes its proof callback in the parent after successful worker exit/source checks/cleanup. Full script replay in that callback would run outside the monitored child deadline. Root rejected that weakening. R1 therefore adds the necessary real worker/protocol owners and a cohesive history codec child rather than relabeling untimed parent replay as bounded copied proof.

Before modifying R0, `revision0-before/manifest.json` captured589files/6420032bytes: the complete initial draft, all initial receipts and independent checker outputs, the entire canonical checker result and newly inspected protocol source/existence states. Canonical PLAN-CHECK is exactly200153bytes, SHA256 `01d5b3113f92dd574b9fa7e71ae731a52210d9f55dba7e983b9a0ec696d641f4`. The original root `before-manifest.json` and full original Plan12 SHA256 remain the complete-inverse target. Initial scripts/receipts are never rerun into their old output paths. R1 uses only new `r1/` scripts/results. Live11 SUMMARY/close and shared root metadata remain outside immutability claims.

### Exact current ownership and scheduling

All paths in this table are relative to `packages/open-bitcoin-node/src/`:

| Task  | Current exact owners                                                                                                                                                                                                                                                                                                         | Count |
| ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----: |
| Task1 | storage/fjall_store/inspection.rs; storage/fjall_store/inspection/canonical_history.rs; storage/fjall_store/canonical_genesis.rs; storage/fjall_store/inspection/phase167_tests.rs; storage/fjall_store/inspection/worker.rs; storage/fjall_store/inspection/protocol.rs; storage/fjall_store/inspection/protocol/history.rs |     7 |
| Task2 | sync/open_runtime.rs; sync/canonical_genesis.rs; sync/canonical_history.rs; sync/canonical_history/tests.rs; chainstate/fjall_store/canonical_genesis.rs; sync.rs                                                                                                                                                            |     6 |

These thirteen distinct owners are below15, with warnings for thirteen total/seven Task1/six Task2 files requiring independent assessment and root adjudication. No worker/tests.rs, copy.rs, reader.rs, lib.rs, fjall_store.rs, core, network, build, guard or extra test owner is authorized. Existing worker/tests.rs and protocol.rs original tests remain byte-exact. New protocol tests live inline in the cohesive new history child; actual child/lease/source/runtime controls use the already-owned phase167_tests.rs and sync/canonical_history/tests.rs. Any required fourteenth owner, overlong formatted file or missed task context budget stops for another checked amendment. No minification, raised line limit or contorted module placement is allowed.

Original12 remains two tasks at wave15 after167-08/10/11. All other28 plans and all lifecycle/schedule/requirement fields stay exact. Only necessary task file lists change. Existing13owners are one coherent pre-effect/runtime integration concern, not permission to rewrite generic inspection. The sync.rs source allowance remains exactly the single23-byte declaration with full inverse.

### Private typed protocol and limits

Add private `WorkerJob::{Generic(OwnedScratchJob),History(HistoryJob)}` dispatch in protocol.rs. Keep `OwnedScratchJob` and its original encode/decode, MAGIC, write_result/read_result implementations unchanged, and keep generic with_inspection's callback timing/behavior unchanged. Generic operations call those exact v1 codecs. Dispatch reads at most MAX_JOB+1 once, refuses oversize before interpretation, recognizes exact v1 versus new history framing and never falls back after a malformed selected mode.

The coherent new protocol/history.rs owns private `HistoryJob { owned: OwnedScratchJob, selected: CanonicalChain }`, constructor/accessors and history request/result framing. The fixed history request schema is:

```text
magic = OBIJ\x02\0\0\0 (8 bytes)
mode = 1 (1 byte; only complete canonical history)
selector = 0/1/2/3 (1 byte; Mainnet/Testnet3/Signet/Regtest)
payload = exact original v1 OwnedScratchJob bytes through EOF
```

The response uses `OBIR\x02\0\0\0`, the same mode/selector and the exact original v1 full records frame. All unsupported selectors/modes/versions, every truncation/trailing byte, duplicate/count/length/overflow and identity mismatch refuse. No second recovery-only or proof-skipping history mode exists. Outer request total, including the10-byte envelope, must fit unchanged MAX_JOB=16384. Outer result total, including envelope, must fit the exact existing MAX_RESULT formula; use checked bounded-write accounting before every byte reaches the result file. Payload/entry budgets remain shared and unchanged, not separate per envelope. A value that cannot fit the existing outer limit refuses; the cap is not increased.

The existing nested nonce and root device/inode fields must match the exact parent-generated job. Existing full root path/check_root and result device/inode/mode/symlink/link-count/opened-and-selected metadata checks bind the same scratch/result owners. Response decode requires the expected history mode/selector and nested v1 nonce/root identity; a generic response cannot satisfy a history request, nor vice versa. The selector is a typed private operation request, never proof authority. Transport returns complete bounded data only: no serialized actual core/HeaderStore/proof, public compatibility boolean or caller-provided authority DTO. Successful parent acceptance requires genuine child completion at the checked control-flow boundary, not a response-shaped byte string alone.

### One monitored child, real replay before any result

`worker::recover_history(&ScratchGuard, CanonicalChain) -> Completion<InspectedRecords>` and existing recover must delegate to one common spawn/writer/monitor/finish implementation. Use the same executable, exact ARGS/root macro, existing panic/refusal categories, private scratch and one child. Capture the original single Instant immediately before spawn and never reset it. The same unchanged30seconds cover stdin delivery, raw recovery, DB/keyspace/lease destruction, full history/script replay, output write/sync and successful exit/reap. Do not add a second stage deadline, child, worker binary or parent-side substitute.

In actual run_child, decode the typed request and preserve existing root layout/inventory/private metadata checks. Complete raw_recover inside its existing scope and drop every scratch Fjall/keyspace/lease owner. Before job.result_file(true), existing output-fault seams, any result bytes/success or child exit, history mode calls the already-owned inspection/canonical_history.rs::validate_in_child(selected,&InspectedRecords). That callback decodes CanonicalHistoryRecords, calls real storage::canonical_bootstrap::validate_history_records with its fallible shell SystemTime, executes full scripts/every retained ScriptsValid branch and drops child earned proof/genesis/HeaderStore/core before returning. It writes no authority receipt. Only then may the history result codec run. Generic mode does not run this callback and retains its original byte/output behavior.

Monitor requires successful stdin delivery plus actual successful exit/reap before decoding the expected operation response; valid-looking output while still live or followed by nonzero exit fails. Preserve file identity/length rechecks, kill/reap uncertainty semantics and no cleanup while completion is uncertain. Test hooks may be cfg(test)-only helpers in owned worker.rs/canonical_history.rs; preserve the existing worker/tests.rs contents and all original generic faults. New fault codes may use the existing cfg(test) OwnedScratchJob fault field without changing its layout or old code meanings. Full-replay-entry delay hooks belong inside the child proof callback after raw recovery, and test-only stage receipts must remain private, bounded and distinct from any result/success.

### Parent original proof and actual earned-owner transfer

The history-specific adapter in inspection/canonical_history.rs calls existing inspect_owned_then with worker::recover_history, preserving original inventory/source recheck, source-engine lock, continuously held application lease, scratch identity/security, cleanup and confirmed reap gates. Its final parent callback only decodes data/compares digest; it must not repeat COPIED full script replay outside the child timer. Existing generic with_inspection remains unchanged.

Only after child full proof and all parent gates may the compatible original be opened. Immediately read the complete current original data and compare every namespace/key/value and source digest against the complete child-returned data. Then perform mandatory ROOT shell full reproof on those ORIGINAL records through canonical_bootstrap::validate_history_records, followed by source-bound consuming proof/wrapper handoff. This original reproof is deliberately separate from the bounded copied child operation; it cannot replace that operation. No recovery/config/index/prune effects may occur before it. No original raw database fallback precedes compatibility.

Parent proof consumption returns actual parent-owned genesis/HeaderStore/core; the child proof never crosses the process. Retain the earned active admission_snapshot/real-coins rebind, proved physical orphan preservation, selected-chain/params/local-magic/held-context binding, actual tip-body eligibility/cache and strict genesis constructor obligations from the prior sections. Every original retained-fork all-handle product reopen/next and301/maturity/halving/negative-control requirement remains.306-known/304-active next acceptance and physical-versus-active undo distinction are unchanged. All copied logical/physical/value/deadline limits remain fixed.

### Must-haves, critical links and behavioral verification

Observable must-haves are: actual history-mode child full proof rejects well-formed script-invalid claimed history before any original effects; generic mode remains an unchanged data inspection; proof delay and live/nonzero/uncertain lifecycle cases cannot mint history success; source equality/cleanup/lease gates hold; actual original proof yields the real owners for product reopen/next.

Critical planned links are history adapter → recover_history → typed history job → same run_child → validate_in_child → real canonical_bootstrap::validate_history_records **before result output**; expected bound/reaped response → source/scratch/cleanup gates → complete ORIGINAL data equality → ORIGINAL full proof/consume → active snapshot/real coins/rebind → actual HeaderStore/tip-body checked runtime. The original frontmatter truths/links remain; this contract supplies their missing child insertion point.

Tests must include a paired recoverable valid/script-invalid history: unchanged generic raw recovery succeeds, while history mode enters the actual child proof callback and rejects the well-formed invalid script case. Establish that its arranged body/header/work/record context reaches the actual script check rather than failing only transport/decoding; retain a paired valid control. Actual replay-entry stage receipts and delays are cfg(test)-only, under the same monitored child, and cannot substitute for the script-invalid behavioral control. Exercise a delay at full-replay entry, after raw recovery, and a delay after proof before output to show the deadline spans the complete operation without reset. Real successful output followed by a live/nonzero child must refuse. Include kill/reap versus uncertain completion, cleanup failure, source swap, lock retention, selectors/modes/versions/nonce/root/result identity, all truncations/trailing bytes/outer-budget boundaries and complete retained-fork data. Compare original physical manifest and accepted state for every pre-effect refusal. Do not edit or weaken old generic assertions.

Name new codec/child/lifecycle tests with `phase167_preeffect_history_worker` so original Task1's broader filter discovers them. The additional focused selection is `bun run scripts/command-timings.ts run --key phase167-12-history-worker -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_preeffect_history_worker -- --test-threads=1`. Rerun existing inspection regressions with `bun run scripts/command-timings.ts run --key phase167-12-generic-inspection -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase166_inspection -- --test-threads=1`. Both original task commands and strict node all-target/all-feature Clippy remain exact and mandatory. Each selection needs nonzero named discovery, zero failed tests, zero ignored tests and final thirteen-owner source attribution. Preserve every intermediate failure. Raw recovery timing is not copied full-proof timing; actual child duration/complete resource measurements must cover recovery plus scripts/replay/result/exit.

### File-size feasibility and stop conditions

Ignored formatted planning sketches in r1/ are interfaces/control-flow proposals, not executable implementation or build evidence. The dispatch sketch reuses the exact v1 codecs rather than duplicating them. Its formatted addition is measured with the original495-line protocol parent; worker sketch describes one small common-mode delegation and one child callback insertion, not another supervisor. Projected implementation allocations are:

| Owner                                    | Current lines | Projected formatted envelope | Responsibility                                                                                               |
| ---------------------------------------- | ------------: | ---------------------------: | ------------------------------------------------------------------------------------------------------------ |
| inspection.rs                            |           260 |                       \<=300 | Conventional history child/test registration; preserve generic callback and gates.                           |
| inspection/worker.rs                     |           288 |                       \<=420 | Private operation delegation, callback before result, expected response dispatch and narrow cfg(test) hooks. |
| inspection/protocol.rs                   |           495 |                       \<=590 | Typed dispatch/accessors around unchanged v1 code; no duplicated codec/tests.                                |
| inspection/protocol/history.rs           |           New |                       \<=500 | Cohesive outer envelope/selector/bounded writer and inline codec controls.                                   |
| inspection/canonical_history.rs          |           New |                       \<=350 | Child proof callback, parent history adapter/data-only digest and private test seams.                        |
| inspection/phase167_tests.rs             |           New |                       \<=620 | Original plus full-proof/child/lifecycle/source/lease controls using existing fixtures.                      |
| storage/fjall_store/canonical_genesis.rs |           245 |                       \<=450 | Existing sealed original proof/consuming history wrappers.                                                   |
| sync.rs                                  |           567 |                  568 exactly | One conventional23-byte declaration.                                                                         |
| Remaining five original Task2 files      |  Existing/new |                   Each \<629 | Existing runtime/core assembly and full product-reopen tests.                                                |

These are planning feasibility bounds, not measured final implementation lengths or proof that all tests fit. The concrete ignored whole-parent dispatch sketch formats to567lines for protocol.rs, and the whole-parent shared supervisor/callback sketch formats to312lines for worker.rs, leaving room within the stated590/420 envelopes for final adaptations and cfg(test) hooks. The history child contract sketch is52lines of interfaces/schema only; its actual bounded codec/tests remain an implementation obligation under500lines. Execution must measure every formatted file after real implementation and stop if any reaches629 or if the cohesive tests need another owner. No line-limit exemption, minification, omitted negative/301/retained-fork test or silent owner expansion is authorized. Only the sync registrar has a full one-line proposed-source inverse; none of these sketches is compiled or accepted source.

### R1 preservation and remaining gates

R1 must reconstruct the original full Plan12 by removing the four added owners from frontmatter, the three Task1 additions and one Task2 addition, both new tagged context blocks and added verification text. All original actions, behavior, acceptance, read-first, done, threats, interfaces and two task commands stay exact. Other28 plans have identity inverses. All original38documents/270historical receipts and all589 revision0-before states must remain exact; the200153-byte canonical checker prefix remains unchanged. Current11's final94761ed… proof and other five source hashes remain the basis; its new live SUMMARY is root/executor-owned active output, not a historical freeze target.

The initial checker ISSUES result and previous successful structural/inverse observations stay attributable. New r1/planner-validation.json and r1/markdown-checks.json contain actual R1 static outcomes only. Their checks find29valid plan frontmatters/task structures,50tasks/24waves, all original task contracts/commands and full inverses, other28plans exact,38protected documents/270historical receipts intact, and all589R0 snapshot states preserved with only these two current metadata documents deliberately revised. The canonical200153-byte checker prefix and all current11/source/existence states remain exact. Added owner overlaps are dependency-ordered with zero same-wave collisions. New amendment Markdown and scoped whitespace checks pass; the full formatter-normalized inverse exactly reconstructs the inherited original Plan12 formatter residual. These metadata observations do not exercise child replay, transport, runtime or strict Clippy.

No old receipt, canonical report, source, build, shared metadata, summary or guard is edited by this revision. No independent approval is claimed. Strict consumer wiring must remove the staged24dead-code groups by real use and achieve unchanged strict Clippy exit0 before12 acceptance.17source-before-guards,17/20compact-fallback adjudication,19parity/docs,20full native/fresh coverage/Bazel/security/goal/lifecycle and root saving remain mandatory. No release occurs until independent R1 checker and root acceptance.
