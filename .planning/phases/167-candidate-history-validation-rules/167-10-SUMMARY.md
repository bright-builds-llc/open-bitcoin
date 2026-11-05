---
phase: 167-candidate-history-validation-rules
plan: "10"
subsystem: chainstate
tags: [rust, managed-connect, original-clock, reorg-admission, basic-filters]
requires:
  - phase: 167-08
    provides: Complete durable undo and actual seeded storage foundations
  - phase: 167-09
    provides: Owned HeaderStore contexts and narrow managed header bridge
  - phase: 167-27
    provides: Complete overwrite allocation accounting at unchanged limits
provides:
  - Actual trusted-clock managed direct admission and exact ordered replacement admission
  - Fresh live canonical descendant absorption, validation publication and BASIC continuation evidence
  - Explicit test-only seeded managed kernel and full flush without canonical promotion
affects: [167-11, 167-12, 167-15, 167-16, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Existing coins owner admission, Held original clock batches, Typed accepted-state comparisons]
key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/validation_history/tests/phase167.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/phase167_tests.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate/validation_history.rs
    - packages/open-bitcoin-node/src/sync/open_runtime.rs
    - packages/open-bitcoin-node/src/chainstate/validation_history/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
    - packages/open-bitcoin-node/src/sync/canonical_genesis/basic_tests.rs
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/sync/canonical_genesis/tests.rs
  reviewed:
    - packages/open-bitcoin-node/src/sync/block_reconcile.rs
    - packages/open-bitcoin-node/src/sync/canonical_genesis/basic.rs
key-decisions:
  - "Direct mutable admission captures actual shell time; the no-clock wrapper can only consume exact held admission."
  - "Collect actual owned HeaderStore histories and clocks before one complete coins batch and existing preference authorization."
  - "Retain the already-correct BASIC processed/safe distinction rather than manufacture a production correction."
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T00:28:27Z"
commits: false
duration: 46min02s
completed: 2026-10-22
---

# Phase 167 Plan 10: Managed connects and accepted BASIC lineage Summary

**Proved-regtest direct and preferred replacement routes now earn coins admission before staging; fresh accepted descendants continue BASIC through the existing receipt and durable owner.**

The existing main-repository checkout baseline is `1b472fb225a290df5cdb06eeea2dc67c624c648e`; this is not a task commit. Both original tasks are complete within the independently checked twelve-owner scope. The initial eleven-owner implementation was held at a checkpoint until root approved the narrow legacy-test amendment; all eleven source hashes remain exact. The requirement list records this bounded contribution, not whole-requirement or phase acceptance. No staging, commit, push, worktree, hook bypass, guard renewal, frozen corpus change, shared-state update or Phase168 work occurred. Root owns final acceptance and saving.

## Files Created/Modified

- Modified: `packages/open-bitcoin-node/src/chainstate/validation_history.rs` — trusted direct clock, exact batch bridge, narrow eligibility-only deferral, conditional fixture flush and real BIP30 duplicate regression;439lines.
- Reviewed unchanged: `packages/open-bitcoin-node/src/sync/block_reconcile.rs` — actual cached-body reconciliation already delegates through the managed guards and authorization;547lines.
- Modified: `packages/open-bitcoin-node/src/sync/open_runtime.rs` — test-only genuine canonical runtime setup and stored/local route tests; production factory and parameter wiring unchanged;339lines.
- Created: `packages/open-bitcoin-node/src/chainstate/validation_history/tests/phase167.rs` — genuine mining/owner setup, typed snapshots and direct/local/replacement/clock/binding/fixture tests;593lines.
- Modified: `packages/open-bitcoin-node/src/chainstate/validation_history/tests.rs` — child registration only;582lines.
- Modified: `packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs` — child registration only;273lines.
- Created: `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/phase167_tests.rs` — accepted descendant, refusal, owner/error and exact-batch controls;586lines.
- Reviewed unchanged: `packages/open-bitcoin-node/src/sync/canonical_genesis/basic.rs` — existing saved-mode resolution and genesis behavior remain correct;46lines.
- Modified: `packages/open-bitcoin-node/src/sync/canonical_genesis/basic_tests.rs` — actual runtime enabled/disabled descendant chain and durable-safe tests;627lines.
- Modified: `packages/open-bitcoin-node/src/chainstate.rs` — thin mutable direct hook and no-clock immutable preparation;598lines.
- Modified: `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` — actual owned replacement contexts before preference authorization, preserving root diagnostic precedence;513lines.
- Modified: `packages/open-bitcoin-node/src/sync/canonical_genesis/tests.rs` — only the obsolete four-selector refusal contract, with observed exact typed Regtest errors and all original effect guards;474lines.

Twelve reviewed Rust owners, ten actually changed, two new files. All are below629physical lines; maximum627. Root accepted the bounded twelve-owner warning; no thirteenth owner, compressed formatting or size exception was used. Complete original bytes, the preserved eleven-owner checkpoint and final twelve-owner hashes are under `.local/open-bitcoin-dev/phase167/plan10/` in `before/manifest.json`, `final-source-closure.json` and `final-source-closure-12.json`. The eleven-owner checkpoint was never rewritten. The shared provider initially matched the accepted09 bridge at306lines and SHA2563ec75ff36811ac40447c28ae48de86b26ade1a1fb5c2c5ae1f82bfb7f2450e22.

## Task 1: Actual managed coins and earned context

The public mutable connect path calls admit_direct_canonical_header before immutable preparation. Only a genuine canonical core selection enters shell time capture. Checked SystemTime/UNIX-seconds conversion supplies the existing core admit_canonical_header; caller current_time, header.time, flags and numerical work do not establish this observation. Identical held admission retains the core's original clock and generation. The no-clock convenience method prepares directly: missing admission refuses, while exact genuine held context remains usable. The immutable prepare-only and public unauthorised reorg refusals are preserved.

Preferred reorg admission requires existing owned support, then gathers canonical_header_history and canonical_header_validation_context from this actual HeaderStore. It checks exact candidate/header/hash/parent order, selected chain and the held context's derived parameters, ends the immutable borrow, and delegates the complete ordered disconnect/header-clock batch to the same coins owner's admit_canonical_reorg_headers. This happens before authorize_reorg, retention, immutable staging and preview. Existing exact-work, body-linkage, availability and preference checks remain mandatory. No descendant singleton guard runs after the batch. A root-shaped replacement still uses the existing genesis guard before the owned-scope gate, preserving the original foreign/altered-root diagnostic order.

The09 singleton bridge now defers off-tip Regtest coins admission only after its existing network adapter has checked the actual held context. Successful side-body caching means eligibility only: it grants no pending coins proof, accepted receipt, ScriptsValid or BASIC progress. Applicable tip-extending admission errors propagate. Direct connect still invokes the core directly, and activation still requires the exact complete batch.

Tests execute actual direct, local, stored-runtime reconciliation and preferred two-header replacement absorption from real proved genesis. The fork has a nonempty disconnect and different branch MTP; both resulting position MTP and the new Coin's creating MTP are checked. Controls cover original-clock reuse under caller reversal, true future time under caller MAX, missing/no-clock/immutable admission, changed generation, foreign stages, wrong disconnect/order/count/work and direct preference refusal. Complete typed comparisons include all effective UTXOs, undo, transaction-count facts, active positions/exact work, coins best block, live/durable validation provenance and BASIC state/progress. A genuinely matured accepted spend followed by its live-output duplicate reaches OutputOverwrite before the already-spent input could mask BIP30.

Existing canonical factories already bind complete selected parameters, and the core derives flags for each candidate. No production factory/setter change was needed: custom maturity/halving carriers refuse and caller NONE cannot disable selected P2SH execution. The legacy fixture flag carrier in generic runtime construction is not canonical rule authority; the tests exercise the actual selected interpreter.

The cfg(test), crate-private connect_seeded_historical_fixture handoff takes Block, checked CandidateHistory and original clock. It delegates the existing mechanical/IndependentlySeeded stage, genuine coins absorption and full managed flush; live/BASIC/pending-publication owners refuse. It publishes no canonical ScriptsValid receipt and cannot bind live work. Its new probe is an explicitly conditional genesis-shaped fixture, not earned canonical ancestry or a real historical-body result. Plan16 can feed the exact acquired historical envelopes through this same kernel. The accepted08 physical repeat/restoration evidence remains08's evidence; this plan does not relabel it.

## Task 2: Accepted validation and BASIC continuation

The existing acceptance/lineage/guarded-append/checkpoint pipeline satisfies the contract after Task1 admission is wired. No Task2 production behavior was changed. Header-only and real immutable stage previews leave complete accepted state unchanged. Genuine absorption and an ordered turn advance processed progress; safe remains at genesis until a full same-owner coins flush. Only then does a subsequent turn publish the exact descendant safe checkpoint. Tests compare the saved complete undo to the accepted undo and verify zero genesis predecessor plus exact successive filter-header linkage.

Enabled and disabled histories both execute genuine accepted blocks and reject excessive reward, aggregate sigops, nonfinal locks, unexpected witness, wrong height and the mature P2SH false redeem script despite NONE. Typed accepted state and durable provenance remain unchanged on consensus refusal. The actual DurableSyncRuntime factory is exercised for two descendants in both modes; disabled mode has no index owner/artifacts. A storage fault cannot publish false processed/safe endpoints or erase genuine accepted provenance; subsequent work correctly remains fail-closed until proved reopen. Foreign store proofs and stale generations refuse without endpoint or accepted-chain progress.

Genesis remains unspendable. Wallet availability, networking defaults and retained-body protection are unchanged. No filter import, prune feature or cold descendant replay is introduced; Plans11/12 retain descendant reopen/product-proof ownership.

## Actual RED/GREEN and retry history

Complete logs retain actual outcomes; filenames beginning green do not override their recorded nonzero exits. Evidence is progressive: earlier missing layers masked later ones. No simultaneous or retrospective RED is claimed for cases that were not observed before correction. New controls over already-working behavior are regression evidence, not invented production RED.

| Receipt                                                 | Actual outcome                                                                                                                                                                                                                                      |
| ------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| red-direct.log                                          | Exit101;1 discovered/1 failed with missing-canonical-header-admission before the direct hook. Genuine RED.                                                                                                                                          |
| green-direct.log                                        | Exit0;1/1 after the direct hook and safe no-clock wrapper.                                                                                                                                                                                          |
| red-reorg.log                                           | Exit101;1 passed/1 failed with ParentMismatch in singleton fork admission. Genuine replacement RED.                                                                                                                                                 |
| red-nonextending.log                                    | Exit101;1 passed/2 failed with ParentMismatch for fork activation and nonextending cache eligibility. Genuine RED before deferral.                                                                                                                  |
| red-reorg-scope.log                                     | Exit101;2 passed/1 failed at unsupported-validation-scope after deferral exposed the unchanged caller gate.                                                                                                                                         |
| red-reorg-batch.log                                     | Exit101;2 passed/1 failed with missing-canonical-header-admission after using the existing owned-scope gate, before the batch bridge. Genuine whole-batch RED.                                                                                      |
| green-reorg.log                                         | Exit101;2 passed/1 failed. An added comparison incorrectly equated root carrier parameters with candidate-derived parameters. Removed only that incorrect comparison; held context still matches actual derived history. Implementation correction. |
| green-reorg-selected-context.log                        | Exit101;2 passed/1 failed with canonical-work-mismatch. The positive test supplied ZERO replacement work. Corrected to actual admitted HeaderStore work; wrong-work refusal remains explicitly tested. Test setup correction.                       |
| green-reorg-exact-work.log                              | Exit0;3/3 actual direct/cache/replacement positives.                                                                                                                                                                                                |
| initial-basic.log; initial-basic-corrected-snapshot.log | Both exit101 at compilation: snapshot then guessed try_snapshot are unavailable on generic Fjall-backed Chainstate. Corrected to the existing managed export_chainstate_snapshot after inspecting its implementation. Setup errors, not RED.        |
| initial-basic-complete-snapshot.log                     | Exit101;0/1. The new test prematurely expected safe at the unflushed child. Existing processed/safe separation was correct; this is a test expectation error, not product RED.                                                                      |
| basic-real-durable-fence.log                            | Exit0;1/1 after asserting safe remains genesis before actual flush and advances only afterward. No production correction.                                                                                                                           |
| expanded-task1.log; expanded-basic.log                  | Exit0;11/11 and3/3 respectively at their earlier source checkpoints.                                                                                                                                                                                |
| basic-owner-controls.log                                | Exit101;4 passed/1 failed. The new retry expectation ignored the existing publication-fault reopen requirement. Endpoint/provenance controls passed; changed only the new retry assertion to require continued refusal. No production correction.   |
| basic-expanded-final-controls.log                       | Exit0;6/6 at that checkpoint.                                                                                                                                                                                                                       |
| task1-all-controls.log                                  | Exit101 at compilation: new BIP30 test used nonexistent ScriptBuf::new. Corrected to Default. Setup error, not RED.                                                                                                                                 |
| task1-full-controls-corrected.log                       | Exit0;13/13.                                                                                                                                                                                                                                        |
| canonical-admission-regression.log                      | Exit101;27 passed/1 failed at the unchanged altered-genesis expectation. New reorg ordering had returned unsupported-validation-scope before noncanonical-genesis-body. Genuine regression RED.                                                     |
| canonical-admission-final.log                           | Exit0;28/28 after root-only recognition before batch creation. Original assertion unchanged.                                                                                                                                                        |
| canonical-genesis-regression.log                        | Exit101;15 passed/1 failed at an out-of-scope inherited blanket-refusal assertion; see Deferred Issues. This is not a passing module receipt.                                                                                                       |
| rustfmt-final.log                                       | Exit1; nullable-field rename needed one line wrap. Scoped write on that owned child corrected formatting; subsequent checks exit0.                                                                                                                  |

Read-only exploration also encountered missing guessed paths and display truncation; actual sources were then located/read in bounded portions. The formatter reports mdformat1.0.0 with GFM1.0.0/frontmatter2.1.2; a separate system pip query did not locate those environment-owned packages and was not used to install or change tooling. No authentication gate occurred.

## Preserved eleven-owner verification and checkpoint interval

All Cargo commands, including the later test-only closure below, used pinned Bun1.3.9, Rust1.94.1, physical process-local TMPDIR from the verify.sh pwd-P pattern, existing targets and serial cooperative command-timings ownership. Initial Cursor artifact-lock waits resolved without killing/reconfiguring it. Quiet work was polled within60seconds. The213.49s canonical-genesis run continued making progress; CPU/process liveness is retained in genesis-regression-liveness.log. No deadline, threshold, work limit or target layout changed.

| Final command after PATH/TMPDIR setup                                                                                                                                                                              | Outcome                                                          |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------- |
| bun run scripts/command-timings.ts run --key phase167-10-task1 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_managed_connect -- --test-threads=1                                 | Exit0;13/13; final-task1.log                                     |
| bun run scripts/command-timings.ts run --key phase167-10-task2 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_basic_continuation -- --test-threads=1                              | Exit0;6/6; final-task2.log                                       |
| bun run scripts/command-timings.ts run --key phase167-10-canonical-admission-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node network::canonical_admission:: -- --test-threads=1  | Exit0;28/28; canonical-admission-final.log                       |
| bun run scripts/command-timings.ts run --key phase167-10-validation-history-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node chainstate::validation_history:: -- --test-threads=1 | Exit0;30/30; validation-history-regression.log                   |
| bun run scripts/command-timings.ts run --key phase167-10-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings                               | Exit0; clippy.log                                                |
| Scoped rustfmt --edition2024 --config skip_children=true --check on all eleven owners                                                                                                                              | Exit0; rustfmt-final-owned.log                                   |
| Scoped git diff --check                                                                                                                                                                                            | Exit0; diff-check-final.log and final independent invocation     |
| GSD verify key-links against167-10-PLAN.md                                                                                                                                                                         | all_verified:true;1/1 actual provider link; key-links-final.json |

Focused filters have nonzero named discovery and zero ignored tests; their zero-match integration binaries are not counted. These are not whole-node or whole-native results. Accepted08's2077-pass node result with four inherited default ignores remains historical08 evidence. Full native/fresh pure-core coverage/Bazel and independent phase reviews remain Plan20/root.

The preserved checkpoint source/check interval is2026-10-21T23:42:25.281909Z through2026-10-22T00:07:12.466Z:1487seconds,24minutes47seconds. It includes setup corrections, observed REDs, added module checks and review time; it is elapsed time, not active CPU time. Earlier context loading and subsequent summary work are excluded. Original bytes, all logs, copied actual timing records, before/final diff and exact final source hashes are retained under plan10. Initial provisional source-closure and pre-format diff are preserved separately rather than represented as final bytes.

## Checked legacy-test closure and final twelve-source verification

Root accepted167-10-LEGACY-REFUSAL-AMENDMENT.md after independent checking with one explicit twelve-owner scope warning. Only the existing phase166_runtime_descendant_routes_refuse_without_owner_or_storage_effects function changed. Its original shared SELECTORS and all other tests/helpers remain byte-for-byte exact. The earlier failed15/16 module was preserved, and no completion SUMMARY existed while the plan awaited this approval.

The diagnostic-only run retained in legacy-diagnostic.log executed the exact named test with all four selectors, all message/direct/fixture/reorg routes and every effect guard. It passed1/1 as an observation run, not as final expectation evidence. It recorded these actual Regtest errors:

| Route in this unconnected/unadmitted fixture                      | Exact observed NetworkError                                                                                                                                                                          |
| ----------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Headers through sync, general and durable ingress                 | InvalidHeader { reject_reason: high-hash, maybe_debug_message: Some(proof of work failed) }                                                                                                          |
| Block through all three ingress methods; stored/local body; reorg | InvalidHeader { reject_reason: unproved-canonical-header, maybe_debug_message: None }                                                                                                                |
| CompactBlock and BlockTxn through all three ingress methods       | UnknownPeer(1)                                                                                                                                                                                       |
| Fixture-header seeding                                            | InvalidHeader { reject_reason: unsupported-validation-scope, maybe_debug_message: Some(ordinary history validation is unsupported for regtest: canonical_bootstrap, exact_work, candidate_history) } |

All routes for Mainnet/Testnet3/Signet retain the observed unsupported-validation-scope category. Final assertions check the actual authority/operation/network variant and exact per-route Regtest NetworkError, including debug data and peer id. The original unsupported assertion stays for the other selectors. Generic is_err checks were strengthened; no arbitrary error category, skipped selector or contextual-validation claim was substituted. Temporary eprintln/message diagnostics were removed, with the diagnostic source retained in legacy-diagnostic-source.rs. The specific legacy fixture still has no connected peer1 and no contextually admitted child; actual09/10 genuine context evidence stays separate.

The final legacy-exact-final.log passes1/1 with exact assertions. The complete unfiltered sync::canonical_genesis:: module passes16/16 with no new ignore or skip, including the old case; canonical-genesis-final.log is the full356.24s run. The legacy-clippy-final.log strict node all-target/all-feature Clippy exits0. Scoped rustfmt and whitespace checks cover all twelve owners. The original13/13 managed,6/6 BASIC,28/28 admission and30/30 validation-history receipts remain attributable to the preserved eleven-source basis; they were not redundantly rerun solely for this isolated test correction.

| Continued command after the same pinned PATH/TMPDIR setup                                                                                                                                                                                                                                         | Actual result                                                                                   |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| bun run scripts/command-timings.ts run --key phase167-10-legacy-refusal -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::canonical_genesis::tests::phase166_runtime_descendant_routes_refuse_without_owner_or_storage_effects -- --exact --test-threads=1 --nocapture | Diagnostic observation1/1, then exact final assertions1/1; both exit0, separately retained logs |
| bun run scripts/command-timings.ts run --key phase167-10-canonical-genesis-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::canonical_genesis:: -- --test-threads=1                                                                                        | Exit0;16/16,0ignored;356.24s harness; canonical-genesis-final.log                               |
| bun run scripts/command-timings.ts run --key phase167-10-legacy-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings                                                                                                       | Exit0;12.70s; legacy-clippy-final.log                                                           |
| Scoped rustfmt and git diff --check on all twelve owners                                                                                                                                                                                                                                          | Both exit0; legacy-style-final.log and legacy-whitespace-final.log                              |

legacy-contract-final.json verifies exact whole-file prefix/suffix around the single named test, original setup, complete equality closure, exact message array, cleanup, all seven route argument lists, all seven source assert_unchanged calls and the unchanged panic-on-durable-payload callback. It also checks diagnostic removal and all eleven frozen source hashes, then records the twelfth hash and all twelve physical source sizes. final-source-closure-12.json preserves the final closure. Full original input hash is checked against the independently captured3d56611964cc0be077e230fc6842af33d8fd347199e6ebb274f7ecf6f4247bcc; no earlier preimplementation source basis was substituted.

The FINAL measured source/check interval runs from2026-10-21T23:42:25.281909Z through2026-10-22T00:28:27.063Z: 46min02s, including the checked legacy-test closure and the interval between checkpoint and resumed execution. The original checkpoint remains24min47s; the resumed source/check interval is2026-10-22T00:17:42.138Z through2026-10-22T00:28:27.063Z, 10min45s. The intervening10min30s covers checkpoint handoff, amendment review/release and context reload, not continuous build/test CPU time. The two measured source/check portions total35min32s. Summary formatting/validation afterward is excluded. Root receives the actual millisecond values in final-source-closure-12.json; the earlier24min47s is not relabelled the final duration.

The continuation also preserved the first ignored checkpoint report and its initial metadata-format finding. Diagnostic observation was authorized only after independent scope release; there was no guessed category, production correction or retroactive behavioral RED. The final expectation test and whole-module pass are the closure evidence.

## Simplification, guidance and security review

The simplification pass retains the existing pure candidate rules, independent coins authority, work binding, staged transaction and consume-only acceptance owners. Shell time and held HeaderStore context stay in thin node adapters; there is no parallel proof pipeline or public mutable core accessor. Shared typed fixture helpers avoid duplicated authority setup. Full owned sources, complete new children and the before/final diff were reviewed. Actual links include the direct hook, core single/batch calls, replacement collection before authorize_reorg, and registered behavior children; no comment or dummy call stands in for the key link.

AGENTS.md local guidance, Bright Builds sidecar/overrides and managed architecture/code-shape/testing/verification/Rust pages informed the shell boundary, minimal ownership and actual verification. Both active lesson files were read completely:6898+1958=8856bytes,2953conservative tokens; the recent October19 audit baseline applies. No archive, new audit trigger, project skill, dependency or workspace crate was required. Remote refs were fetched and the dirty shared checkout preserved without rebase. Contributor/parity/breadcrumb reconciliation remains Plan19; new Rust children already carry pinned-source comments.

T-167-10-01 is covered by actual selected body checks, exact batch/owner/generation controls and refusal snapshots. T-167-10-02 is covered by header/preview controls, real absorption, processed-versus-safe checkpoint evidence and foreign/stale/error controls. T-167-10-03 is covered by attributable logs and separate seeded/ordinary evidence. No undeclared production network endpoint, auth path, file-access boundary or schema was introduced. No authorizing stub remains; empty collections in negative/seeded fixtures are explicit test inputs, not missing ancestry promoted to proof.

## Deviations and Deferred Issues

[Rule1 - Bug] Restored exact root-body diagnostic precedence in the newly changed reorg caller after the unchanged regression asserted the original error. The adjustment is entirely inside the existing caller and does not singleton-admit descendants after the batch. Other corrections above are explicit test/setup or local implementation fixes, with their initial outcomes retained. The approved eleven-owner caller amendment was followed, then the separately checked legacy-test amendment added only the twelfth existing test owner. The test-only historical adapter is not represented as a new production authority correction or retroactive RED.

**The inherited blanket-refusal assertion is now closed by the checked test-only amendment.** The original15/16 receipt remains failed and attributable to its old expectation. Root independently approved only sync/canonical_genesis/tests.rs after the eleven-owner checkpoint. Actual diagnostics and precise final assertions, described above, replace blanket refusal for the qualified Regtest setup while preserving every selector, route and effect guard. No production owner changed during this closure and no genuine contextual validation is inferred from this unconnected/unadmitted fixture.

**Pre-existing compact fallback parity gap also remains open.** Pinned blockencodings.cpp:207–212 and net_processing.cpp:3377–3393 request a full block after mutated reconstruction; current transport fills then body validation rejects bad-txnmrklroot. This predates09 and10; no compact source was owned or changed. The accepted09 refusal control does not establish Knots fallback parity. Root retains17/20 scope/closure adjudication, with no permanent waiver inferred here.

Fresh-live regtest continuation is the bounded product evidence. No canonical Mainnet/Testnet3/Signet ordinary history, cold descendant replay/product proof, actual pinned-daemon qualification, unattended sync, wallet availability, public network activation, import or pruning claim is made. The stronger complete earlier-Coin restoration versus Knots stays attributable to07/08 and future16 evidence. Root alone updates STATE/ROADMAP/REQUIREMENTS/config/todo and saves with normal hooks after entire-phase acceptance. Stop before11/source-guard work and Phase168.

## Task commits

None: commits:false. The explicit root-owned saving/state contract supersedes generic executor per-task commits and state advancement. Both original tasks and their regression closure are complete; root retains plan acceptance and all whole-phase saving/state gates.

The first completion-summary mdformat check returned1. The available mdformat1.0.0 with GFM1.0.0/frontmatter2.1.2 formatted only this summary, and the subsequent check passed. The three formatter receipts are retained separately; no source bytes changed during metadata formatting.

## Self-Check: PASSED

All twelve declared source owners exist; ten changed and two remain byte-identical to the starting snapshot. All eleven checkpoint source hashes remain exact. The twelve-source limit, genuine key link, attributable nonzero focused checks, final sixteen-case regression, strict Clippy and scoped style checks are verified. The labelled checkout baseline resolves to the existing commit. No task commit is asserted. Root retains phase acceptance and saving.

GSD verify-summary with check-count12 reports passed:true,12checked/12found and errors:[]. Frontmatter validation reports valid:true and missing:[]. Actual key-link verification reports1/1. The corresponding JSON receipts are retained under plan10.
