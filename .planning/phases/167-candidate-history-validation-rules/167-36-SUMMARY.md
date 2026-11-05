---
phase: 167-candidate-history-validation-rules
plan: "36"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: chainstate-node-storage
tags: [rust, native-inventory, staged-projection, basic-filters, local-submission]
requires:
  - phase: 167-12
    provides: Accepted bounded safe canonical-history prover and configured reopen
  - phase: 167-33
    provides: Accepted complete Coins, undo, counts and durable row interfaces
  - phase: 167-34
    provides: Accepted canonical network owner and runtime integration
provides:
  - Same-store bounded native observations and exact genuine staged-state row projections
  - Shared BASIC checkpoint construction preserving actual writer authority and work ordering
  - Real early-header and pre-absorption resource guards with achieved logical and physical checks
affects: [167-30, 167-17, 167-19, 167-20, future-compact-B-E]
tech-stack:
  added: []
  patterns: [Opaque data-only observations, genuine staged overlay projection, shared row producers, consume-once achieved comparison]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/canonical_history/records/projection.rs
    - packages/open-bitcoin-node/src/storage/canonical_history/records/projection/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/append/rows.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/append/rows/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/workflow.rs
  modified:
    - packages/open-bitcoin-node/src/storage/canonical_history/records.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs
    - packages/open-bitcoin-chainstate/src/engine/stage.rs
    - packages/open-bitcoin-node/src/chainstate/work_authority.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/typed_boundaries.rs
key-decisions:
  - Reuse the unchanged native row policy together with the existing streamed reader budget for every namespace and row.
  - Derive prospective data only from actual authorized preparation; projected bytes confer no work or acceptance authority.
  - Recapture the full source after genuine early durable writes and compare actual achieved rows after Always and BASIC.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T18:26:38.605735+00:00"
commits: false
duration: "99m 52.317s through source, final QA, independent review and summary creation; final document QA is recorded separately"
completed: 2026-10-22
---

# Phase 167 Plan 36: Native Resource Projection Summary

**Actual prepared effects now predict native rows before resource-sensitive durable writes and absorption, and real submission success checks achieved rows after Always/BASIC.**

## Final Source Files

All fifteen declared owners are normally formatted and at most628 physical lines; maximum621. The immutable before state contains ten present sources and five genuinely absent children. No six-child baseline is manufactured. The six protected external existence states remain exact.

| Source                                                                                                  | Lines | SHA256                                                           |
| ------------------------------------------------------------------------------------------------------- | ----: | ---------------------------------------------------------------- |
| `packages/open-bitcoin-node/src/storage/canonical_history/records.rs`                                   |   329 | 8052e65b6c3f24765ee6baa445112013ea2ca27cc3bada098b396453a41c6b33 |
| `packages/open-bitcoin-node/src/storage/canonical_history/records/projection.rs`                        |   278 | 1a242f0b8e55b2484e98c7ef6ec763d2ba3599a786e7aee4a846c173740704bd |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs`                                      |   398 | 2bcfb997a607d1041616872773fc8d90ca72fd44147e9cadb0d73cd5ca5d7557 |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs`                               |    77 | e221e05406e1fc9a5e9f0d954af2b13ff150065b1f2a4f0302785dc081700dd4 |
| `packages/open-bitcoin-node/src/storage/canonical_history/records/projection/tests.rs`                  |   526 | 72f492facd55503b6265c4db6a9e5e8b2df820e30a277cf66274f266f4f5615a |
| `packages/open-bitcoin-chainstate/src/engine/stage.rs`                                                  |   621 | 9f5a8edb5975f4ef21fb9697c7e0daa27b4d1a2ac202a1526827765cd98844b9 |
| `packages/open-bitcoin-node/src/chainstate/work_authority.rs`                                           |   361 | f814ed9a66a4532f229cfaac03d8b69071b0805fab87d3e1ee2a1f74e1032a67 |
| `packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows.rs`                                    |   166 | 126a55725265a788c0d03bd9f52b5ee0963191b40f86c89f8e126fb28262bba6 |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs`                                           |   484 | c0a931d9bdcd4a1923f71a7d49ba5d2e653a66ac8de1e4b3d5c51d2cfbf4ea13 |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs`                                  |   580 | 8049273687682fe97cbe368af0ed15762dfcfaf37f2036c7929dfddbddd2cf21 |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/append/rows.rs`                             |   180 | 208b82b0d61afc0f5f9fbc312e6ae89fb98b5dcc915b7e17515ec032de7dac17 |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/append/rows/tests.rs`                       |    71 | 6899aaae77556a16e0d1b72d248a298798e90a318a4b76c6ee01070a6054cd89 |
| `packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs`                        |   374 | aa9720dcf47f9e89b17bc85e078d7a919451201893b7589d2f6a1d0ecfc0d7ec |
| `packages/open-bitcoin-node/src/network/runtime_authority/validation_history/workflow.rs`               |   315 | 4ebda6fa155b0715ca6364ce4c79621639efe2c49789297e19320a43405e4dc2 |
| `packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/typed_boundaries.rs` |   218 | 8d841b9febadf491ab37182ae7fdeec6d313d72d04344965e1bf6112bc555771 |

## Bounded Outcome

All four checked tasks are implemented and independently reviewed CLEAN with zero findings. Requirements-completed copies the plan's IDs verbatim and records only this bounded contribution. Root alone accepts implementation and shared requirements; no partial Plan30 or phase acceptance is asserted. The baseline commit identifies existing history. No task commits, staging, push, root metadata, guard digests, metrics or central manifest edits were performed.

Task1 reads one actual cross-keyspace Snapshot through the held database accessor, preserving all nine namespaces and existing unsupported-row refusal. The opaque observation binds actual store identity, catalog, visible sequence, complete rows and source digest. The existing reader Budget is reused for every namespace and row alongside unchanged records::inventory. Its actual3,479-byte function remains byte-exact, SHA256 d6bfbe7cfb3a67a55da8b0f571e557e5498e23c8aa4ce944a8c73347dce4a7d7. The trailing-empty-wallet536,870,913-byte counterexample is retained: the old row-loop policy returns Ok, while the real combined reader budget refuses. No limit, policy constant, numerical mirror, namespace repair or second engine is added.

Task2 checks the actual stage binding and prepared body authorization, reads complete admission Coins, applies the genuine staged overlay, and clones actual next chain/count/undo facts. This yields data without absorbing the stage or minting authority. Actual serializers, segmented-family preparation/ownership, Coins formats, complete counts and the same undo row producer serve projections and writers. Genuine controls include100 matured blocks, an existing-Coin spend, same-block cancellation and103 transaction counts. Foreign/stale bindings refuse. Pure-core inline units execute both positive projection/absorption equivalence and foreign/stale refusal; the compile-fail document proves the data result cannot become a Chainstate owner.

Task3 shares checkpoint/protection/state/prune-lock construction with the actual BASIC completion writer. Actual work reservation remains before encoded buffers; proof/fence/owner/retention/fault/commit order remains intact. Projection uses genuine captured historical inputs, real predecessor and held generation/progress/state. Missing/stale facts refuse operationally; Disabled preserves BASIC bytes. Complete projected/achieved maps match actual Enabled/Disabled writes across all nine namespaces. Unrelated operator locks, retry semantics and large-small-large family replacement are exercised. Projected bytes never become an append proof, publication fence or accepted batch.

Task4 supplies the real production consumer. Under the existing operation mutex and actual authority it checks earned header/KnownOnly rows before installing/publishing them, calls direct already-locked trusted header persistence, and captures a NEW complete baseline after ALL early durable writes. It preserves genuine normalized volatile body/eligibility registration before real preference authorization and stage/lifecycle preparation. Complete normalized body/Coins/counts/undo/validation/BASIC projection and freshness checks precede save_block and absorption. A standard boxed FnOnce carries only expected data and the original store borrow through real Always flush and BASIC completion. Final same-owner/target/receipt/BASIC checks, fresh achieved-map equality and actual physical capture/recheck precede Accepted. Late failures remain operational with honestly advanced state and receipts.

Full resource refusal preserves accepted state and mempool, but explicitly retains genuine earlier KnownOnly/trusted-header writes and volatile normalized body/linkage/eligibility/header-context effects. It is not a whole-owner rollback or wholly effect-free failure. The original new-at306 pre-effect guard and its one existing storage getter remain unchanged.

## Final Commands and QA

The exact thirteen planned argv and supplementary Core argv are authenticated in `.local/open-bitcoin-dev/phase167/plan36/execution/final-evidence-index.json`, SHA2566e7edcd745550265470240f9954d311801aae8c7c317cb78e523ff3793397aa5. Every command exits0. Every test selection discovers its intended nonzero cases with zero ignored/failing tests; empty auxiliary harnesses are not named discovery. Counts overlap and total146 test executions, not146 unique tests. Seconds below are exact recorded wrapper wall intervals.

|   # | Command key / selection                                                                                  |     Passed |  Seconds |
| --: | -------------------------------------------------------------------------------------------------------- | ---------: | -------: |
|   1 | `phase167-36-inventory` / `phase167_submission_inventory -- --test-threads=1`                            |          6 |  190.296 |
|   2 | `phase167-36-stage` / `phase167_submission_projection -- --test-threads=1`                               |          6 |   11.933 |
|   3 | `phase167-36-inferred-api` / `phase167_submission_projection_api_inferred_inventory -- --test-threads=1` |          1 |    2.044 |
|   4 | `phase167-36-core-api` / `--doc project_staged_connect`                                                  |          1 |   21.792 |
|   5 | `phase167-36-row-regressions` / `phase167_durable_rows -- --test-threads=1`                              |         32 |  114.909 |
|   6 | `phase167-36-basic-rows` / `phase167_submission_basic_rows -- --test-threads=1`                          |          2 |    1.671 |
|   7 | `phase167-36-projection` / `phase167_submission_projection -- --test-threads=1`                          |          6 |    8.273 |
|   8 | `phase167-36-basic-regressions` / `storage::fjall_store::filters::tests::append:: -- --test-threads=1`   |         36 |   33.426 |
|   9 | `phase167-36-inspection-regressions` / `phase166_inspection -- --test-threads=1`                         |         39 |  211.924 |
|  10 | `phase167-36-history-regressions` / `phase167_preeffect_history -- --test-threads=1`                     |         12 |  644.966 |
|  11 | `phase167-30-resource-probes` / `phase167_local_submission_resource_probe -- --test-threads=1`           |          2 |   26.516 |
|  12 | `phase167-36-guard-smoke` / `phase167_submission_guard_smoke -- --test-threads=1`                        |          1 |   37.301 |
|  13 | `phase167-36-clippy` / `strict Node+Core all-target/all-feature Clippy`                                  | lint exit0 |    1.357 |
|  14 | `phase167-36-core-units` / `phase167_submission_projection_core -- --test-threads=1`                     |          2 | 1132.201 |

Source execution began `2026-10-22T16:46:46.289Z`. Final command completion is `2026-10-22T18:18:45.709Z`:5,519.420seconds (91m59.420s), including final QA. Independent review is dated2026-10-22T18:20:25.192073+00:00. Summary creation is `2026-10-22T18:26:38.605735+00:00`: 5992.317seconds from actual source start, including the intervening review/QA wait. The initial Prettier document QA remains unchanged in `.local/open-bitcoin-dev/phase167/plan36/execution/document-QA.json`, with its exact summary and raw proof preserved under `execution/document-QA/revision1-prettier/`. Final installed mdformat GFM/frontmatter QA and updated elapsed time including that check are recorded in `.local/open-bitcoin-dev/phase167/plan36/execution/document-QA-revision2.json`.

The supplementary Core compiler completed naturally after18m51s (wrapper1,132.201seconds), then both inline units passed. ReadDir/\_\_getdirentries64 samples and automatic advisory evidence are retained; no command timeout, process kill, cache/target/config/security change or build overlap occurred. The intended compile-fail E0308 and existing malformed-engine child abort diagnostics belong to passing parent controls. All ad-hoc Cargo used pinned Bun1.3.9, Rust1.94.1, command-timings/cooperative locking, the existing target and physical TMPDIR; no Bazel job was required by this local plan.

## Original RED and Preserved History

The original two Plan30 resource cases actually returned Accepted for a66,147-byte body and4,200 durable Coins, then failed configured safe reopen. Their immutable receipt is `.local/open-bitcoin-dev/phase167/plan30/execution/root-resource-probes/result.json`, SHA256467ed4700b43821067359a6b7df34b145386b780ea2ae4b41d00cc81998661b5; raw log SHA256795852ab258687b03c37658eb6c5d9f799578faffad6b9d8d0a14e2fd7594a89. Those jobs retain their original eight-source fences. They are never relabeled21-fenced or successful.

The same two named probes and body construction now return operational Err before absorption, preserve complete accepted state and mempool, retain actual KnownOnly/volatile effects without permanent invalidity, persist no candidate body, drop every outcome/command/runtime handle, and actually reopen the old accepted source. Their final GREEN receipt is `execution/final11-resource-probes/result.json`, SHA2563ec9340262aab9b9939cac912d16013c03b6a402cb3bcc3ca58401a2143a395c. The added malformed raw BASE amount control preserves the mined header and earns bad-txnmrklroot before resource work. The actual public factory smoke also succeeds in BOTH modes, drops all owners, safely reopens and accepts a valid next block; an always-refusing implementation cannot pass.

All24 historical Plan36 commands, including12 nonzero outcomes, remain immutable with source copies, started argv, results and raw logs. Their exact labels/hashes/classifications are in the final index:

| Nonzero receipt label             | Honest classification                                                                          |
| --------------------------------- | ---------------------------------------------------------------------------------------------- |
| `task1-green`                     | compiler/setup: keyspace options actual function-item API                                      |
| `task1-green-keyspace-options`    | genuine existing byte-policy counterexample4/5; root selected reuse of unchanged reader Budget |
| `task1-red`                       | genuine stale-source RED plus separately wrong missing-namespace test assumption               |
| `task2-core-red`                  | fixture/setup: historical genesis BIP34 parameter                                              |
| `task2-core-red-fixture`          | genuine missing projected Coin and accepted foreign binding RED                                |
| `task3-basic-red`                 | fixture/setup: actual enabled owner/root turn had not run                                      |
| `task3-basic-red-real-owner`      | genuine changed-checkpoint protection RED                                                      |
| `task3-projection`                | compiler/setup: private Flush imports                                                          |
| `task3-projection-api`            | test fixture omitted actual command body writer; retained exact map failure                    |
| `task3-projection-map-diagnostic` | diagnostic confirmed only missing actual body writer; not projection weakening                 |
| `task4-clippy`                    | strict lint failure: four style issues, not behavioral RED                                     |
| `task4-probes`                    | compiler/setup: actual KnownOnly is StoredValidationStatus, not provenance variant             |

The initial missing-namespace assertion used the wrong isolated policy gate and was corrected separately from genuine stale-source RED. The first whole-map fixture omitted the actual command's body save; diagnostic output identified that one row and the fixture added the genuine writer. Neither projection nor expected limits was weakened. Compiler/fixture/lint setup failures are not claimed as behavioral RED.

## Evidence, Review and Inverse

The immutable source freeze is `execution/final-freeze/source-freeze.json`, SHA256f6afd9beeedc2b19baf35c9e0435d0cb6025aef6462b8cc1d90bb90df756b85f. Each final job captures the actual21-state union before and after:15 owned sources, five protected present inputs and one protected absent future resources child. The separate ten external API owners are authenticated at the immutable freeze and final endpoints only; no31-state per-job capture is claimed. Freeze/index pending flags retain their original capture-time meaning.

Root independently authenticated all38 historical/final started/result/raw-log/source-copy pairs, serial intervals, exact argv and current31 endpoints in `.local/open-bitcoin-dev/phase167/plan36/root-evidence-audit.json`, SHA256602bd983830c104ead14a88890bb0f37bb9ae46371fd970141524572694f59be.

Scoped normal rustfmt --check, caps, source hashes and stub scan pass in `execution/final-static/result.json`, SHA25604bb4e36eaf56476bf78fa3c42c7171b71afcd13a60e0ae3e803ea76e50d435d. All five new Rust children have immediate breadcrumb comments. Central breadcrumb manifest reconciliation remains Plan19-owned.

Actual disposable BSD `patch --batch -E -F0 -p1` forward and inverse passes reproduce all fifteen final/original files and restore five real absences: `execution/inverse-proof/result.json`, SHA25675ea2eb0f784ea7d767fdf85c648ea484f083e3f85f1d70c385e4265258558d5. Root independently reproduced the same round trip in `root-inverse-audit/result.json`, SHA256b8bdf70e25c18cd9228b9569a9886d2d13abb3ef8da49df2a2a576a483b01bd5. No inverse patch was applied to live source.

Independent final local source/API/authority/evidence review is CLEAN0findings: `.local/open-bitcoin-dev/phase167/plan36/source-review/final/REVIEW.md`, SHA25601f107f14ff53187bd2e8a84f6453ede194501410f22674f5e98996688a39f00; `hashes.json`, SHA256ec16829be5e96e9fc97f5644e6ab1b5e8c62810fa82258e7d2efba3dfaa3db5d. The complete narrative and limitations were read before this summary. It confirms the exact rows, authority/work/effect/lock ordering, privacy, counts, genuine controls, complete failure history and distinct per-job versus endpoint evidence.

## Simplification and Scope Decisions

The explicit simplification pass retained one opaque same-store observation, the existing reader/row policies, genuine staged overlay facts, shared row preparation/checkpoint encoding and a standard consume-once data check. This avoids a generic write engine, duplicate serializers, raw runtime producer, public DTO/proof owner or additional export/registrar. Private aliases only clarify standard boxed closure signatures. No new dependency, suppression, cap increase, packing or sixteenth source owner was introduced.

The root-adjudicated reader Budget reuse is within the existing Task1 owner/policy closure; records::inventory remains unchanged. Five actual absent children correct stale prose mentioning six. A future30 readiness miscount of recursively emitted parent+child692lines was withdrawn: actual skip_children parent472 and typed child218; no mandatory split follows from that combined number. These factual adjustments are retained in `execution/DECISIONS.md`.

## Remaining Gates and Limits

This is the bounded Plan36 implementation/review handoff. Root source acceptance remains separate. Plan30 still owns all remaining pure controls, the full resource/fork/fault/late-error/mature-spend oversized-undo matrix, legacy306/307/membership controls, its nine-source contract and all fourteen original plus two resource commands. No future resources.rs source or partial30 acceptance is inferred.

The inherited reader exclusivity comment does not establish a live namespace lock or arbitrary raw-writer serializability. The implementation uses actual bounded observations and catalog/sequence/digest conflict detection under cooperating-owner scope. Actual current physical capture/recheck does not predict future allocation, IO/disk-space failure, copy-worker runtime or deadlines. Late operational failure can retain genuine advanced state/receipts; no rollback or arbitrary-history qualification is claimed.

Whole-phase17 review and the code-review skill, canonical/native/fresh pure-core coverage/Bazel/security/goal/lifecycle/daemon/differential/Git gates remain mandatory. No source, metadata, new build, Git action or Plan30 continuation is authorized by this summary.

## Self-Check: PASSED

All fifteen declared source files, the summary and required frozen/evidence/review files exist. Current twenty-one source existence/hash states and ten separate external API endpoints remain exact; normal formatting and caps pass. All thirteen planned commands plus the actual supplementary Core selection pass, with complete retained before/after copies and logs. Independent review is CLEAN0findings and root evidence/inverse audits pass. Five genuine before absences restore exactly in disposable inverse proof. No commits are expected under commits:false. Final document parser/format/reference QA is recorded separately; this summary does not accept Plan30 or the phase.
