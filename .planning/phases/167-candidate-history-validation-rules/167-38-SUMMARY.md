---
phase: 167-candidate-history-validation-rules
plan: "38"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: node-storage
tags: [rust, fjall, logical-generation, canonical-seals]
requires:
  - phase: 167-37
    provides: Accepted native carriers and the single sealed native record reader
provides:
  - Complete private same-open database, keyspace, snapshot and batch facades
  - Logical mutation stamps with active-writer RAII and serialized catalog access
  - Genuine canonical genesis/history consumption across physical maintenance
affects: [167-39, 167-30, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Opaque per-open stamps, BEGIN-only generation, private native delegation]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/tracked/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests/freshness.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/tracked.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/opening.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows.rs
    - packages/open-bitcoin-node/src/storage/coins_view.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests.rs
key-decisions:
  - Use one shared logical generation and active-writer state for every actual held mutation carrier, independent of physical engine allocation.
  - Retain the single native reader and hold the separate catalog mutex throughout tracked delegation.
  - Preserve native error and empty-batch behavior, complete canonical proof checks and truthful physical batch observations.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T23:41:16Z"
commits: false
duration: "49m 59.099s from initial source capture through final command completion; document QA and root review are separate"
completed: 2026-10-22
---

# Phase 167 Plan 38: Tracked Storage and Canonical Logical Freshness Summary

**Actual held storage mutations now invalidate opaque same-open stamps, while genuine canonical genesis and history seals survive unchanged physical maintenance.**

## Final Source Files

Exactly ten declared owners changed across three tasks. Their final source totals129605 bytes/3757 normal lines; the maximum file is626 lines. Both new children are below the620-line target. This table describes the actual version2 freeze, not the historical909e source.

| Source                                                                                    | Lines | SHA256                                                           |
| ----------------------------------------------------------------------------------------- | ----: | ---------------------------------------------------------------- |
| `packages/open-bitcoin-node/src/storage/fjall_store/tracked.rs`                           |   362 | dc113a198c0756e4d69b40c3a085c675f3960a62c0b3dd58fdbfd7c6b678e077 |
| `packages/open-bitcoin-node/src/storage/fjall_store/opening.rs`                           |    56 | 7c674c877305bcdf44a7dc1f1ca6c3c119f47926fc28dd76474a5124a818b99e |
| `packages/open-bitcoin-node/src/storage/fjall_store/tracked/tests.rs`                     |   431 | 71c64a36c060f2309a68fb6e4b7af0ee1010f9936954459f927f1ad8403e74ea |
| `packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs`                 |   359 | 4a65e4568b06af964e19b919521151694a864d96ed52bcde2789b3386e1556b9 |
| `packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs`                     |   249 | bca825a69721db1850a6e12132dd0f5d1a9240b9b331970987d56de2e95869a6 |
| `packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows.rs`                      |   166 | b531e6d4ad2b40991d0c42940a824e8805b0345671e7a4b2bf6c3f35d4902724 |
| `packages/open-bitcoin-node/src/storage/coins_view.rs`                                    |   490 | f7ff90370722d63deff4ba689d89ec6e9565528edb4da284168ee9ececdcc831 |
| `packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests.rs`    |   626 | e035b7413661ff3d854378d4ba2ef976f7419c4d565de5f61132b950f058e339 |
| `packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests.rs`           |   608 | f18cfba8d470e1cab58b4ffaa10817b78ea8727bbaab8fdd0efc72dab9e82f96 |
| `packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests/freshness.rs` |   410 | 169be85ce2589e253559b62bda53ddbea3bfd596ce9ba0f1b3f52c96893d6eca |

## Bounded Implementation

Task1 replaces the accepted37 aliases with complete private Database, Keyspace, Snapshot and OwnedWriteBatch wrappers, wraps the actual leased backend once before constructing all nine keyspaces, and migrates every production genesis/history physical-sequence fence to MutationStamp. All actual database/keyspace clones, returned keyspace/database carriers and batches retain the same Arc identity. Stamps are opaque and neither Copy nor externally constructible. Existing publication identity, prove_live, full history proof/source digest and before/after checks remain. LAST_BATCH_SEQUENCES still observes the real native sequence around one actual authority batch.

BEGIN advances generation and active count once before each actual insert/remove/weak-remove/clear/catalog mutation or nonempty commit. RAII closes active writers on success, native Err or unwind without incrementing generation again. Observations require the same open, exact generation and no active writer; poison/exhaustion permanently fail closed without wrap or double panic. No generation mutex spans native I/O, conversions/callbacks, reader iteration or publication/history/payload locks. Existing lookup, reads, snapshots, buffering, persistence and physical maintenance remain benign. Foreign batch association refuses before any effect, including a foreign-only empty native buffer; valid native empty commit delegates without BEGIN. Native write errors propagate unchanged.

The separate shared catalog mutex serializes actual lookup/create/delete and covers the ENTIRE tracked RecordSource call into the same unchanged raw reader/Budget. No exists-to-create race, second reader or raw handle escape is introduced. Scratch lease/engine-lock/copy/manifest/source/budget protections remain unchanged. No Deref, AsRef, Borrow, raw getter, raw into_inner, wrapper builder/config escape or handle-exposing callback exists. Native row guards yield data. The read-only Snapshot exposes only needed native data shapes; independent design adjudicationdfefda8e confirms no unused Shared Arc or invented foreign-read policy is required.

Task2 removes only the four audited obsolete Readable imports. The626-line Node parent is exactly its old parent minus that import; no test/helper/oracle moved or changed. Normal-build unused read forwarding was restricted to its actual test-only surface before Task1 completion, with no allowances or dummy production consumers. visible_seqno remains a normal inventory consumer input until39.

Task3 adds only mod freshness to the607-line canonical test parent; removing that registrar reproduces the entire original parent byte-for-byte, including its original direct identical schema.insert ChangedStore control. The child uses real canonical factories, genuine managed history acceptance/checkpoints, complete nine-map/catalog comparisons and actual runtime consumption. All original owners drop before safe history reopen; maintained owners are consumed, released and safely reopened again with full state/proof correspondence. Both BASIC modes are covered for history. No recovered proof is fabricated.

## Real Controls and Final Verification

The12 generation tests cover held fields/clones, same-value/ABA, empty add/delete, nonexistent delete plus actual rotation/compaction, clear/restore, weak remove, same-name catalog recreation, buffering/empty/nonempty batches, foreign stamps/batches/delete, benign reads/lookup/persist/snapshot/rotation, real in-flight overlap, actual native Err, actual native conversion unwind, overflow and poison. The channel-scheduled conversion runs inside real native insert after BEGIN; no sleeps or raw-handle scheduling callback is used.

The strengthened native Err case obtains an actual proved Regtest store through open_canonical_genesis/into_parts and drops its recovered owner. It creates/clones/deletes only a temporary tracked namespace, then requires supported inspect_submission_inventory AFTER deletion, with exactly nine maps/catalog. Retained clone.insert reaches actual fjall::Error::KeyspaceDeleted after BEGIN. All nine maps/catalog remain identical, the old LOGICAL stamp is stale and a fresh idle stamp validates. This is a real engine refusal before native row/WAL effects, not disk failure, partial commit recovery or rollback; no old physical inventory staleness is claimed.

Five canonical controls prove real physical advance with complete unchanged rows/catalog and original seal validity: genesis15→16, disabled history22→23 and enabled history24→25. Original maintained seals successfully enter actual runtimes, drop all owners and safely reopen. Identical writes, ABA, empty add/delete, clear/restore and catalog recreation restore full source bytes/digest yet return ChangedStore. Equal full source/digest cannot transfer a history capability to a foreign open.

All nine exact planned automated occurrences pass on the ONE immutable final53 version2 freeze. Their argv, selected environment, raw logs, timing receipts, exits and full before/after source copies are indexed; empty auxiliary harnesses are not discovery.

| Final command key             |                        Passed cases | Wrapper seconds |
| ----------------------------- | ----------------------------------: | --------------: |
| phase167-38-generation        |                                  12 |          22.347 |
| phase167-38-same-value        |                                   1 |          11.112 |
| phase167-38-integrated-clippy | strict all-target/all-feature exit0 |           9.933 |
| phase167-38-seal-freshness    |                                   5 |         159.343 |
| phase167-38-canonical-store   |                                  29 |         548.383 |
| phase167-38-raw-reader        |                                  39 |         219.786 |
| phase167-38-history-proof     |                                   1 |          51.549 |
| phase167-38-final-format      |        workspace format check exit0 |           2.979 |
| phase167-38-final-clippy      | strict all-target/all-feature exit0 |           0.496 |

These are87 overlapping final test executions, with zero failed or ignored cases, not87 unique tests. The21 actual jobs also retain77 historical overlapping executions and two zero-test compile/setup failures. Expected raw-engine abort-characterization child diagnostics belong to the passing39-case reader control. The existing history-proof selection retains its genuine302-position enabled history and original resource bounds.

Initial capture2026-10-22T22:50:54.338Z through final command completion2026-10-22T23:40:53.437Z is2999.099seconds, including checked holds and version1 history. Final version2 command intervals total1025.928seconds. Document QA and independent/root acceptance occur separately. All scheduled intervals are disjoint; pinned Bun1.3.9, Rust1.94.1, command-timings/cooperative lock, original target/cache and physical TMPDIR are retained. Ambient IDE artifact-lock waiting and CPU-active test progress are recorded. Silence/advisories were not treated as timeouts and no build was interrupted or cache copied.

## Immutable Evidence, Source Fence and Exact Inverse

Final index `.local/open-bitcoin-dev/phase167/plan38/execution/final-evidence-index.json` is1219686 bytes, SHA256b64a94598fd1f6037450a5649d69e35aae19f32bc6e019a8465b0abe3d07c7ca. It authenticates all21 real jobs,2172 per-job source copies, explicit absences, exact planned final argv matches, raw/environment/timing records, original/final freezes, controller history, all10 owners and separate engine inputs. Final source is `execution/freeze-v2/result.json`, SHA2568290bef34b17daafd634693bf45b00384efe79c98c495cac9df4dce30c191d04:52 present/one protected future-child absence across53 inputs. All43 protected states remain exact. The201 engine/lock inputs have separately authenticated actual bytes and retained copies; no per-job third-party capture is claimed.

Original38 root-before8ffe34fa remains the inverse authority: eight existing accepted37 owners and two actual absent test children. Original release40c7a9a7 and accepted375b0fad39 remain unchanged. Checked APIf0942af5/decisiond04cda26 and fresh working530d46300b expand future evidence with READONLY mempool.rs; they do not replace that original baseline or relabel the first failed52 receipt.

Actual isolated strict patch --batch -E -F0 -p1 reverse and forward applications pass: `execution/inverse-proof-v2/result.json`, SHA256ff816991219842e9be8080bf512c77d20d2aa550207f0346ef4f340a4aaa4efa. Reverse restores all eight accepted37 before bodies and both true test-child absences; forward reproduces every final hash. Raw patch logs, full patch and source copies are retained. The proof patches only an ignored sandbox, composes accepted37 and preserves original Plan30 inverse provenance. The raw native reader remains whole-byte exact. Source scans find no stubs or additional unmodeled threat surface. New child breadcrumbs are immediate; only Plan19 reconciles the central manifest.

## Checked Corrections and Preserved Version1 History

The first Task1 compile is exit101 with zero tests and52 stable inputs: resulte7a8f23b/rawbcd8f473. Required native Keyspace::size_of forwarding and the old offline fixture Database binding were missing. Execution stopped for independent166-check API closure; root released both corrections inside original Task1 owners with future53 captures. Private cfg(test) use fjall::Database in canonical_genesis.rs restores the sole old child offline builder expression without editing any old function, exposing a wrapper builder or changing production access. The actual old engine-lock test passes.

Task3 initially attempted private record-field assertions; that zero-test53-input compile failure remains retained. Only owned freshness assertions changed to the existing public stored snapshot observation, complete native rows/digest and actual replay/runtime checks. No new getter, owner or API weakening was added. These are setup failures, not behavioral RED. The causal behavioral RED remains the original root-authenticated physical-only diagnostic; new controls are not falsely represented as pre-fix failures.

Root then required the native Err fixture's stronger genuine supported-inventory baseline. Only tracked/tests.rs changed from historical909e to version2; the other nine owners remain byte-exact. Historical909e freeze, its five completed final selections, complete29-case receipt and original inverse/static/source evidence remain preserved. The29-test command actually finished naturally in544.35 test seconds/544.824 wrapper seconds. Only its queue orchestrator was suspended while awaiting an inherited-output child; run.ts independently drained Cargo. After natural receipt and confirmation no scheduled child remained, the stopped controller was retired(exit143), separately from all build outcomes. No active run/timing/Cargo/test process was suspended or killed. New distinct freeze-v2/run-v2/v2-final labels reran all nine selections after the concrete test enhancement.

The simplification pass retained one Shared/RAII state, the single reader/Budget and native batch is_empty rather than a second counter, duplicated reader, redundant buffer flag or generalized transaction framework. The native error enhancement is isolated to its own genuine fixture; other low-level controls retain their small temporary stores. Repo guidance, Bright Builds sidecar/overrides and architecture/code-shape/testing/verification/local-guidance/Rust standards informed execution. Both active lesson files were read completely:8856 bytes/2953 conservative tokens, with no omissions or new audit trigger since the2026-10-19 baseline.

## Independent Review and Document QA

The complete final independent report was read: `.local/open-bitcoin-dev/phase167/plan38/independent-review/revision2-8290/REVIEW.md`, SHA25683da177211307ae7591d844dd3a531694ba8efb200f2e499c5dac24326e284f4. Its separate hash index is SHA256171f74141fb9ce47c5dfcdcceed7f4b8f2444370d0959d365f4358e7420e8209. Verdict is CLEAN/PASS with zero unresolved source, security, API, inverse or evidence findings. It independently authenticates2364 unique indexed artifact references and2172 per-job copies plus52 final-freeze copies,2224 combined. Root's separately scoped audit authenticates2375 references. The reviewer independently reproduces the strict version2 inverse/forward, genuine supported native Err baseline, all nine new final commands,87 overlapping final executions, separate201 engine inputs and controller-only retirement history. Historical909e receives no final acceptance. This bounded review is separate from root source/phase acceptance.

Initial configured GFM/frontmatter formatting passes. The first GSD summary parser result is preserved: its broad self-check fail regex classified the phrase "no failures" as a negative result despite the PASSED heading and actual green checks. Only that paragraph's wording was clarified; test outcomes, evidence and verification policy are unchanged. Document QA revision2 repeats scoped formatting, exact frontmatter/lifecycle/baseline/requirements checks and all ten source spot-checks. The initial result remains document-tool history, not a code/test failure.

## Remaining Gates and Scope

Historical32-source freeze3f07f102, full retained-bound failure33ac77c9/raw6af31048 and deterministic physical-only RED94aa911c/rawc2cc1c7d with root4073f9d8 remain authenticated unchanged. Original Plan30 beforef077d0a3 and7359b36c resume provenance remain intact. All accepted37 source/evidence provenance remains;38 inverse does not undo37. The unchanged590-line Node physical inventory probe remains discoverable and the inventory consumer is still Plan39 work. No test was ignored, deleted or weakened, and no original failure is relabeled passed.

Requirements-completed copies the plan's IDs verbatim and describes this bounded contribution. Global CVLD requirements remain OPEN. This execution does not accept Plan30, qualify ordinary history/network/daemon behavior, close the phase, renew source guards or release39/30. Full native/coverage/Bazel/source/security/goal/lifecycle/differential/Git obligations remain. Root owns independent acceptance, shared STATE/ROADMAP/requirements metadata, guards, central manifest and Git. commits:false was honored: no staging, commit, push or shared metadata update occurred. Stop before39 pending separate actual root acceptance/release; Phase168 remains unstarted.

## Self-Check: PASSED

All ten declared sources and required evidence exist; current owner hashes match the immutable version2 freeze, all53 before/after states remain exact for every final job, all201 separate engine/lock inputs authenticate, and exact eight-before/two-absence inverse/forward reproduction passes. All nine planned final commands and every selected case pass with meaningful named discovery, ordinary628 caps, formatting and strict diagnostics. No commits are expected under commits:false. Scoped GFM/frontmatter formatting, lifecycle/baseline/requirement equality and GSD verify-summary --check-count10 are retained in document QA revision2. Root acceptance remains separate.
