---
phase: 167-candidate-history-validation-rules
plan: "08"
subsystem: storage
tags: [rust, fjall, complete-undo, bip30, seeded-history]
requires:
  - phase: 167-02
    provides: Complete accepted historical bodies and envelopes
  - phase: 167-07
    provides: Complete captured overwrite batches and reusable structural checks
  - phase: 167-26
    provides: Bounded source-bound typed historical projection
  - phase: 167-22
    provides: Six mechanically adapted node fixture owners
  - phase: 167-24
    provides: Five mechanically adapted node fixture owners
provides:
  - Lossless overwrite DTO conversion through per-block and whole-snapshot carriers
  - Missing-legacy refusal without invented no-overwrite provenance
  - Actual seeded Fjall checkpoint, all-handle reopen, restoration and refusal evidence
affects: [167-09, 167-10, 167-16, 167-17, 167-19, 167-20, 167-27]
tech-stack:
  added: []
  patterns: [Required untrusted fields, Preserved captured integrity, Explicit seeded recovery]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/snapshot_codec/overwrite_undo.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/phase167_undo.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store/phase167_undo_tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store/canonical_genesis.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
key-decisions:
  - "Preserve every captured Coin field and integrity digest; validate rather than re-seal imported facts."
  - "A missing legacy field refuses; an explicit empty outer collection remains unknown data."
  - "Finite historical suffixes retain unknown global confirmation counts and never earn canonical authority."
requirements-completed: [CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T22:40:24Z"
commits: false
duration: 62min12s
completed: 2026-10-21
---

# Phase 167 Plan 08: Durable complete overwrite undo Summary

**Both explicitly seeded real repeat bodies now retain complete overwrite undo through actual Fjall checkpoint and all-handle reopen, then restore the exact earlier Coins and reconnect.**

## Files Created/Modified

| Actual Rust owner | Change | Lines |
| --- | --- | --- |
| `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` | Required overwrite DTO field; real From/TryFrom helper calls; bounded per-block/snapshot boundaries and duplicate-record refusal | 624 |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/overwrite_undo.rs` | Complete conversion, preserved integrity, structural validation and resource bounds | 234 |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs` | One mechanical empty-field addition and the owned codec test registration | 209 |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/phase167_undo.rs` | Legacy refusal, explicit unknown data and oversized-carrier controls | 55 |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/canonical_genesis.rs` | Test-only bounded tagged-data adapter; production genesis recovery unchanged | 453 |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/phase167_undo_tests.rs` | Complete real-pair codec and physical checkpoint/reopen/restoration/refusal tests | 501 |
| `packages/open-bitcoin-node/src/chainstate/fjall_store.rs` | Owned durable test-module registration | 383 |

Exactly seven Rust owners: three created and four modified. Every source is below629lines. New Rust files have pinned-source comments; Plan19 owns breadcrumb-manifest registration. The existing snapshot fixture keeps every prior input/assertion/fault and gains only the required field, plus a separate test registrar. The eleven Plan22/24 fixture owners were not edited here; their complete paired compilation and default regression ran below.

Source hashes, byte/line counts and final unchanged-source closure are in `.local/open-bitcoin-dev/phase167/plan08/final-owned-stats.json` and `final-checks.json`. This summary and ignored logs are execution records. The preexisting checkout reference `1b472fb225a290df5cdb06eeea2dc67c624c648e` resolves to a commit; it is **not a task commit**.

## Completed Tasks

1. Complete both-direction DTO persistence: complete within the seven-owner plan. BlockUndoDto From/TryFrom call the actual overwrite_undo::encode_overwrite_records/decode_overwrite_records helpers. Both per-block and whole-snapshot paths retain batch block hash,height,creating MTP,transaction index,outpoint,complete prior/new Coin and the original32-byte digest. Coin output value/script,coinbase,height and creating MTP survive exact equality checks. The codec calls existing BlockOverwriteUndo::validate_integrity; it contains no second digest encoder and never repairs/re-seals altered facts.
2. Physical historical evidence: complete within the explicitly seeded class. Both91842→91812 and91880→91722 use the complete accepted bodies/envelopes, actual seeded staging/absorption, managed Fjall flush, release of every manager/store/view/lease handle, physical reopen/read/decode, exact disconnect restoration and reconnect. This is actual durable evidence rather than just DTO equality.

## Codec Contract and Bounds

The overwrite field is required by Deserialize. decode_block_undo(bytes) receives no history proof, so missing legacy overwrite data returns StorageError::Corruption. No missing-field default, fallback empty scan or inferred old Coin was introduced. Explicit new-format overwrites=[] can roundtrip only as unknown/unproved data. Exactly one valid batch with overwritten=[] represents a captured empty scan; the positive fully-spent controls obtain that batch from actual staging and retain it through both carriers. Multiple batches refuse.

The boundary checks256MiB encoded/input bytes,4,000,000 carrier records and cumulative restored-input records,32-byte hashes/digest,typed numeric widths,signed-chain-height and header-clock ranges,existing Amount bounds and the existing10,000-byte ScriptBuf bound. Snapshot decoding rejects duplicate undo keys and UTXO outpoints; complete batches must agree with their stored block position and available parent creating-MTP context. A linear position map avoids repeated ancestry scans. These are structural checks, never ancestry or prior-Coin proof.

Final eight codec tests include both complete pair roundtrips, captured empty scans, missing legacy fields in both carriers, explicit unknown data, oversized bytes,18 per-block corruption cases per pair,encoder refusal of changed captured facts,and five snapshot duplicate/binding controls per pair whose captured digest remains unchanged. The real domain validate_overwrite_binding(&Block,&ChainPosition) is also called with its actual tip semantics. Neither successful decoding nor a self-consistent digest grants canonical authority.

## Physical Evidence and Refusals

The bounded test adapter consumes only the accepted101,893-byte tagged projection. It checks closed tags/order/arity/counts,indexes,canonical decimal/hex,pin/source identities,full header/body/txid/wtxid/merkle bindings,155-row history and genuine eleven-sample clocks using ten raw prefix headers. Hash display bytes reverse once; scripts/witness never reverse. Relative suffix work sums actual proofs from a local zero and is never global canonical work. No80MiB compile-time include,repeated corpus clone,Bitcoin wire parser,production dependency or new authority constructor was added.

91880 executes its real noncoinbase transaction and external input cae24697a15419b1de45c99a41149101c252c2dbd3fd0bbce06ac5809d4547e3:1. Its full Coin is4480000000sat,P2PKH script76a914b5635b9c16b17bf34f29fdc999b440e0638c5b9088ac,noncoinbase,height90644,creating MTP1289277562. Full prior/connected/reopened/disconnected/reconnected snapshot comparisons retain all UTXOs,undo,positions and exact work. Global confirmation counts remain explicitly unknown: a finite suffix cannot prove them for the entire ancestry.

Seven physical mutations per pair cover missing legacy field,missing outer batch,removed inner records,invalid digest bytes,wrong outpoint,actual paired body/undo record deletion,and importing the other genuine batch with its intact valid digest. Decode or disconnect refuses as appropriate. Complete coins/best-block/head markers,metadata/positions/work and BASIC state/checkpoint remain unchanged by refused acceptance. BASIC is absent in this seeded class; no live BASIC lineage is claimed. Raw decoded carriers remain untrusted; explicit mechanical recovery remains noncanonical and has no earned work. Mainnet/Testnet3/Signet ordinary opener refusals remain.

The path helper canonicalizes the physical temporary parent on macOS. No inspector route was introduced, so no new inspection-worker registration was necessary. One physical store instance owns each path; clones used for observation share that instance, and every handle is released before the next physical open.

## Commands and Actual Outcomes

All Cargo commands ran serially through scripts/command-timings.ts with existing targets/cooperative lock and PATH=/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:$PATH. Final commands also use TMPDIR=/private/var/folders/b6/j7bsvp3j6jzbl8r28p9wqtnh0000gn/T. This matches the existing scripts/verify.sh12–14 process-local pwd -P canonicalization; no verifier or lease guard was changed. Complete receipts are under `.local/open-bitcoin-dev/phase167/plan08/`.

| Stage / command | Actual outcome | Receipt |
| --- | --- | --- |
| Initial cargo test -p open-bitcoin-node phase167_undo_codec | Exit101: inherited production BlockUndo TryFrom lacked overwrites; both lib/test failed compilation. This is setup/shape failure, **not behavioral RED**. | task1-red.log |
| Initial implemented codec filter |3/3 passed. | task1-first-green.log |
| Initial physical filter |1 discovered/1 failed at first pair: the new test incorrectly expected global occurrence counts to survive a changed checkpoint. Existing storage deliberately preserves those counts only for identical ancestry. Root approved unknown counts for the finite seeded suffix; no storage policy/assertion outside this new fixture changed. This is a fixture expectation failure, not codec-loss RED. | task2-first.log |
| First physical controls |2/2 passed. | task2-controls.log |
| Expanded codec attempt | Exit101: my serde_json::json!([0;32]) test syntax was invalid. Corrected to vec![0;32]; no product/behavioral failure claim. | task1-expanded.log |
| Expanded codec checks |6/6,then7/7 passed as context and byte controls were added. | task1-expanded-fixed.log;task1-final.log;task1-final-with-bounds.log |
| Expanded physical wrong-genuine-batch control |2/2 passed. | task2-final.log |
| First unfiltered cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features | Exit101/SIGABRT: inherited fixtures used logical /var TMPDIR; the existing lease correctly refused unsafe source. Phase162 cleanup then panicked during unwinding.1044 ok/615 FAILED result lines were observed before abort, **not a complete harness result**; no integration/doctest pass is attributed to this run. | full-node.log;full-node-first-source.json |
| Existing previously failing exact fixture with physical TMPDIR |1/1 passed unchanged, confirming the environment boundary. | canonical-temp-probe.log |
| Final cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings | Exit0,15m24s,zero compiler warnings/errors. Slow directory reads advanced through dependency and integration targets; unchanged advisory threshold crossed, with no timeout/interruption. | clippy-first.log |
| Final cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_undo_codec |8 discovered/8 passed,0failed/ignored;3m23s compile/read phase,0.18s tests. Separate zero-match integration binary is not counted. | task1-accepted-source.log |
| Final cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_durable_undo -- --test-threads=1 |2 discovered/2 passed,0failed/ignored;15.62s tests. | task2-accepted-source.log |
| Corrected unfiltered cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features | Exit0:2050unit+15integration+12doctests=2077passed,0failed,0filtered. Unit1280.00s,integration6.20s,doctests5.58s. Four inherited default ignores are disclosed below. | full-node-canonical-temp.log |
| Scoped rustfmt --check --edition2024 --config skip_children=true,owned git diff --check and full owned source review | Exit0 for scoped checks; all seven owners remain unchanged from checked hashes and below629lines. Untracked-addition diff exit1 is expected, with no whitespace findings. | rustfmt-final.log;diff-check-final.log;owned-tracked.diff;final-checks.json |
| Current executable key link | all_verified:true,1/1; actual From/TryFrom calls, exercised by both carrier tests and physical persistence. | key-links-final.json |

The four unchanged default ignores are: public-network smoke requiring explicit OPEN_BITCOIN_LIVE_SYNC_SMOKE=1; Phase157 timing measurements; Phase158 timing measurements; and the Phase162 subprocess-only child control. The child is exercised by its normal parent controls. The public network was not enabled and an early-return smoke was not reported as evidence. Established timing opt-ins remain for their existing benchmark/final gates. Root accepted the exact prescribed default harness result with these inherited ignores: no new ignore,--skip or test filter was introduced into the unfiltered run. **Zero ignored/literal all-tests execution is not claimed.**

Clippy liveness samples/open-file offsets show advancing rustc search-path directory reads in packages/target/debug/deps. No IDE was killed or reconfigured,no target was copied/cleaned,and no quiet job was interrupted. The timing wrapper's advisory diagnostics are under .local/open-bitcoin-dev/stall-diagnostics/2026-10-21T22-03-31.857Z-phase167-08-clippy; local sample/offset receipts are clippy-liveness.sample,clippy-open-files.log,clippy-directory-progress.log and final-test-directory-progress.log. Sessions stayed resumable and were repeatedly polled with liveness evidence. No authentication gate occurred.

Exploratory reads of absent guessed owners and oversized context outputs were setup observations; actual owners/critical contract sections were located/read subsequently. Optional project-skill directories are absent. No new source was added merely to manufacture a behavioral RED around the inherited compiler failure. All failed logs above retain their original outcomes.

## Post-Implementation Counterfactual Sensitivity

After the initial summary and completed full regression,root requested a faithful bounded inverse to close the behavioral-sensitivity evidence gap. This does **not** rewrite the earlier chronology or turn compile/fixture mistakes into pre-implementation TDD. The baseline parent codec was unbuildable against the new07 domain shape; its historical runtime behavior is not asserted.

The inverse restored exactly the checkout baseline snapshot_codec.rs plus only `overwrites: Vec::new(),` in its BlockUndo TryFrom construction so the old lossy DTO projection could compile against07. It omitted the new helper module registration,as the baseline did,avoiding duplicate TxUndo impls. All new tests/registrars and the other six final source owners stayed byte-identical. No assertions were weakened and no captured fact/digest was invented or re-sealed. This temporary unknown collection was part of the explicitly labelled counterfactual,never final production behavior.

Baseline SHA256 is2fefee5664d1f447a649b864362aa04f6594195088c6623ef5e94a8bbd0e98f6; sole-adaptation inverse SHA256 is64ff487f23c06519092ba1126ea5b980d79e470fb01ff903cf959d0c8634f5d3. Exact baseline,adapted text,all seven before snapshots and the one-file-only inverse observation are retained under `.local/open-bitcoin-dev/phase167/plan08/inverse/` in before.json,baseline-snapshot_codec.rs,adapted-snapshot_codec.rs,before/ and observed-inverse.json.

| Exact test / restoration step | Actual outcome | Receipt under inverse/ |
| --- | --- | --- |
| cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib chainstate::fjall_store::phase167_undo_tests::phase167_undo_codec_both_complete_real_pairs_roundtrip_per_block_and_snapshot -- --exact | Exit101,1discovered/1failed. Real91842decoded undo had overwrites=[] versus the captured complete batch.14.94s compile,0.04s test. | codec-red.log |
| cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib chainstate::fjall_store::phase167_undo_tests::phase167_durable_undo_both_real_repeats_checkpoint_drop_reopen_restore_reconnect -- --exact --test-threads=1 | Exit101,1discovered/1failed. Actual91842checkpoint/all-handle reopen lost its captured batch at full snapshot equality;0.68s test. | durable-red.log |
| Restore all seven checked source snapshots |7/7 byte-identical to the successful full-suite/Clippy source; restored at2026-10-21T22:37:23.470142Z. | restoration.json |
| Prescribed restored codec filter |8/8 passed;14.72s compile,0.17s tests. | codec-restored-green.log |
| Prescribed restored physical filter |2/2 passed;11.61s tests. | durable-restored-green.log |

Both counterfactual assertions stop on the first91842case. No91880counterfactual RED or inverse disconnect execution is claimed. Restored GREEN runs execute both complete pairs and all current controls. The same pinned Bun,physical TMPDIR,serial timing wrapper and existing targets apply. The full2077-test and strict-Clippy receipts remain attributable to exact restored bytes; root explicitly required no redundant broad rerun. Final formatting,whitespace,current key link and source-hash closure also pass; receipts are rustfmt-after-inverse.log,diff-check-after-inverse.log,key-links-after-inverse.json and final-checks-after-inverse.json.

## Guidance and Scope Review

AGENTS.md local guidance,its Bright Builds sidecar/overrides,architecture/code-shape/testing/verification/Rust guidance,accepted02/07/26/22/24 summaries,current context/research/coverage and checked amendments informed execution. Both canonically distinct active lesson inputs were loaded completely:6898+1958=8856bytes and2300+653=2953conservative tokens. The existing audit baseline applies; no lesson/audit/archive or shared task file was changed.

The simplification pass keeps one domain integrity checker,one complete batch representation,ordinary typed Coin conversions and a bounded test-only projection adapter. It moves TxUndo conversions into the declared helper owner to keep snapshot_codec.rs below629lines. Existing assertions and captured facts are retained. No crate/dependency,production export/error owner,ordinary opener,source guard/hash renewal,frozen corpus mutation,real-datadir migration or unrelated schema namespace bump occurred. The accepted fixture SHA256 remains7b91df3ceee0f702c46f98911f4dc92a85c85a0327697e424810c46a379fff5a; manifest/blocks/contexts hashes also match the accepted producer receipts.

T-167-08-01 is covered by complete conversions,bounds,required fields and corruption/context controls. T-167-08-02 is covered by actual checkpoint/all-handle reopen,full Coin restoration and unchanged-endpoint refusal controls. T-167-08-03 is covered by preserved captured digests,frozen input identities,complete actual command outcomes and explicit evidence classes. No unresolved stub remains in the seven owned sources. Intentional unknown collections,empty captured scans and genuine empty witness/external-input sets are not manufactured prior-Coin facts.

## Later-Owned Accounting Seam and Remaining Gates

Read-only review found that existing BASIC accepted-fact fact_work and borrowed_undo_work in packages/open-bitcoin-node/src/chainstate/filter_index.rs charge BlockUndo's outer size and normal transaction Coins,but omit overwrite batch/entry/script heap data. Root independently confirmed this integration seam and assigned a separate bounded Plan27 before Plan10. **This is not fixed by08 and remains mandatory later-owned work.** No outside owner or accounting guard was edited. Root explicitly retained this plan's unchanged-source test result and bounded scope; whole-phase security/goal acceptance is not claimed.

The complete earlier-Coin restoration intentionally exceeds pinned Knots normal overwritten-coinbase undo; no historical disconnect parity with Knots is claimed. These are independently seeded historical/storage fixtures,not ordinary historical product startup,ancestry-complete canonical qualification,actual pinned-daemon differential success or unattended full sync.

Plan16 retains the later runtime historical script path;19 owns parity/breadcrumb/README reconciliation;17/20 retain independent source/security/goal reviews,current guard renewal,full native/fresh coverage/Bazel and lifecycle gates. Shared STATE/ROADMAP/requirements/config/todo and normal-hook saving remain root-owned. commits:false; no staging,commit,push,worktree or hook bypass occurred. This executor stops before09; root acceptance and release are separate.

## Performance Attribution

Two tasks,seven actual Rust owners. Final source/check interval2026-10-21T21:36:53Z through2026-10-21T22:39:05.583192Z is3732seconds(62min12s),including retained failed attempts,slow liveness-observed compilation,full default regression and the later counterfactual/restored focused checks. It excludes initial context loading and subsequent summary validation. final-checks-after-inverse.json records the final measured interval and exact seven-source hash closure. The original3330second checkpoint remains in final-checks.json; no predecessor acquisition/interpreter duration is added.

The first summary validator rejected the final self-check wording because its literal /fail/ scan matched the truthful zero-failure outcome text. Its original result remains in verify-summary.json. The exact test outcomes remain above; only final self-check phrasing was corrected. This is a metadata/parser rejection,not a source or behavioral test failure.

## Self-Check: PASSED

- All seven declared Rust owners and this summary exist. Final source hashes match the checked sources; every owner is below629lines.
- All seven restored owners are byte-identical to the full-suite/Clippy inputs. Restored focused codec and physical filters pass8/8 and2/2 respectively.
- The explicitly labelled preexisting checkout baseline resolves to a commit; no task commit is asserted. commits:false.
- Final8codec/2physical tests and strict affected Clippy passed. The prescribed unfiltered default outcome and four inherited ignores are explicitly recorded above.
- Scoped formatting/whitespace/source review and the current1/1 executable key link pass. Accepted projection/corpus identities remain unchanged.
- verify-summary --check-count7 reports passed:true,7checked/7found and errors:[]. Frontmatter validation reports valid:true,missing:[]. Receipts are verify-summary-final.json and frontmatter-final.json; the original parser rejection remains separately attributable.
- Shared metadata/saving and whole-phase acceptance remain root-owned. The confirmed later BASIC accounting seam remains assigned to27before10; no resolution is asserted here.

