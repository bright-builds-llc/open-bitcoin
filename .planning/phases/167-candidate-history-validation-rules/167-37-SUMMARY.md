---
phase: 167-candidate-history-validation-rules
plan: "37"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: node-storage
tags: [rust, fjall, native-carriers, sealed-reader]
requires:
  - phase: 167-36
    provides: Accepted native inventory, resource projection and bounded record reader
provides:
  - Actually used private native database, keyspace and owned-batch carrier aliases
  - Sealed record-source dispatch around the single unchanged native reader
  - Verified native seal, reader and proof behavior with strict final diagnostics
affects: [167-38, 167-39, 167-30, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Private native carrier aliases, sealed detached record source]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/tracked.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins_access.rs
    - packages/open-bitcoin-node/src/storage/coins_view.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/canonical_bootstrap/test_support.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs
key-decisions:
  - Preserve native behavior and physical fences while routing every explicit owned carrier through the private boundary.
  - Implement the raw source once in the existing reader and return only detached InspectedRecords.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T22:44:38.593830+00:00"
commits: false
duration: "16m 24.356s from actual initial capture through final command completion; document QA is recorded separately"
completed: 2026-10-22
---

# Phase 167 Plan 37: Native Carriers and Sealed Reader Summary

**The store, Coins view, explicit batches and helper return now use private native aliases, and sealed dispatch reaches the same bounded raw reader.**

## Final Source Files

Exactly eight source owners changed across two tasks. All are normally formatted and at most628 physical lines; the maximum is619. The fresh baseline contains seven existing owners and the genuine absence of tracked.rs. The other three future test children remain absent.

| Source                                                                       | Lines | SHA256                                                           |
| ---------------------------------------------------------------------------- | ----: | ---------------------------------------------------------------- |
| `packages/open-bitcoin-node/src/storage/fjall_store/tracked.rs`              |    20 | 48383772834e15bf6798610f679534329e95bd3397760444437a944192fb23ae |
| `packages/open-bitcoin-node/src/storage/fjall_store.rs`                      |   566 | 9bc1d067d8decec72c9205f5b14b7fe2455e96ff3205ba35497c9fbf200016b2 |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins_access.rs`         |    19 | 9372b3796f6d54ce77322787f48f2fda9f3bc964b81d77a5392142d9169fc653 |
| `packages/open-bitcoin-node/src/storage/coins_view.rs`                       |   490 | c22236aa5739467007c0b9f5364a3d971112758d49a945a45e4dc507076af185 |
| `packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows.rs`         |   167 | f8c6905b493d26627ced398d99537bbac46216c483cd7323877d03480a77798f |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs`  |   619 | 7d62c9b32735d043f0d4a03a02ae06d3ed339aaafcb24c399fa0a55e3f5fc201 |
| `packages/open-bitcoin-node/src/storage/canonical_bootstrap/test_support.rs` |   609 | e90f8c913f33158149984a000c3df43ba7a490ad2e0002899fb13b4f05f13c45 |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs`    |    89 | 4711995b4e76c9f0dd0ab515b69a8cd9d7d70d4a4a6cc90c9df6c090421d8ccc |

## Bounded Outcome

Task1 routes the held database and all nine keyspace fields, open_keyspace signature, crate-private accessor returns, Coins database/keyspace/batch carriers and bounded-row batch parameter through three actually used native aliases. Task2 routes publication's batch and the canonical fixture helper's keyspace return through those aliases, defines sealed RecordSource in tracked.rs and implements the single raw Database source in the existing reader. InspectedRecords::read_existing dispatches to that implementation; no raw owner is returned.

The native reader body is byte-exact after the necessary Self-to-InspectedRecords result spelling substitution. The existing record-length arithmetic is byte-exact. Catalog handling, missing/unknown namespaces, snapshot iteration, every Budget increment and original error order remain unchanged. Existing scratch inspection leases, locks, copy/manifest/source checks and cleanup controls remain in their original owners. No inspection.rs edit or reexport was required. Native Readable imports remain actually used and strict Clippy passes.

All44 protected support/API/writer states match the fresh root baseline. Canonical physical sequence fences and the original direct-keyspace identical-authority rewrite test remain unchanged. This mechanical prerequisite introduces no logical stamp, mutation generation, new authority, new database constructor, raw escape, second reader, dependency or writer algorithm. Complete wrapper activation and canonical controls belong to38; inventory adoption belongs to39.

Requirements-completed copies the plan's IDs verbatim and describes only this bounded contribution. Global CVLD requirements remain open. Root alone owns independent source acceptance, shared metadata, guards, central manifest reconciliation, Git and subsequent source releases. No staging, commit, push or shared metadata update was performed.

## Exact Verification and Evidence

The final evidence index is `.local/open-bitcoin-dev/phase167/plan37/execution/final-evidence-index.json`,488468 bytes, SHA256f0afd47d90697c1825d8075e59be9085846dd711e207ca044b72d46699dc8412. It authenticates the exact planned argv, all nine real jobs, source-copy paths/bytes/hashes, started/environment/result/raw records, retained timing receipts, initial state, final freeze, inverse proof and separately versioned engine inputs. Every scheduled job exits0 and has identical full52 before/after captures. All eight automated occurrences execute exactly; Task1 also reruns on final source because tracked.rs changed during Task2.

| Job / command key                               |      Passed cases | Wrapper seconds |
| ----------------------------------------------- | ----------------: | --------------: |
| task1-carriers / phase167-37-carriers           |                 1 |          24.536 |
| task2-reader / phase167-37-reader               |                 1 |          82.492 |
| final1-carriers / phase167-37-carriers          |                 1 |          11.813 |
| final3 / phase167-37-final-native-same-value    |                 1 |          11.840 |
| final4 / phase167-37-final-native-seals         |                24 |         419.901 |
| final5 / phase167-37-final-native-reader        |                39 |         219.295 |
| final6 / phase167-37-final-native-history-proof |                 1 |          33.071 |
| final7 / phase167-37-final-native-format        |      format exit0 |           2.241 |
| final8 / phase167-37-final-native-clippy        | strict lint exit0 |         101.559 |

These are68 overlapping test executions, not68 unique tests. Every named test selection has meaningful nonzero discovery and zero failing or ignored cases. Empty auxiliary harnesses are not claimed as discovery. The final eight jobs, including the already-final Task2 reader, share one immutable final source. Only the first Task1 receipt belongs to its earlier five-file carrier state.

The actual initial capture is2026-10-22T22:27:52.004Z; final command completion is2026-10-22T22:44:16.360Z,984.356seconds later. Installed mdformat1.0.0 with GFM1.0.0/frontmatter2.1.2 handles the scoped summary QA separately under execution/document-QA. GSD verify-summary with check-count8 validates the summary after formatting; its result is retained separately.

Every Cargo command used pinned Bun1.3.9, Rust1.94.1, command-timings and the cooperative lock with the original target/cache and physical TMPDIR. Scheduled intervals do not overlap. An ambient IDE cargo check temporarily held the artifact lock; the reader job waited and completed naturally. The expensive final seal case remained CPU-active and completed naturally after419.45seconds for the full24-case native suite. Process evidence is retained; no command timeout, kill, target/cache change or source write during a scheduled job occurred. Expected raw-engine abort-characterization child diagnostics belong to the passing parent inspection control.

## Source Freeze and Exact Inverse

The immutable full52 final freeze is `execution/freeze/result.json` under the Plan37 evidence directory, SHA2568a0c3c8751946272d7b896d3da21d98e2d9cd6e4a779dc86efb2964db2e9acd5. Root release082d07f0 and actual root-before180fd442 remain the execution authorization and fresh baseline. The52 paths include the eight direct owners and44 protected inputs. All four original absences are represented; after tracked.rs creation, three protected future children remain absent in every job.

The201 engine/lock inputs retain their separately versioned copies and classifications. Actual bytes and copy hashes were checked independently; no per-job third-party capture is claimed. Old31/32-state command receipts remain historical and do not acquire a52-state fence.

Actual disposable BSD patch --batch -E -F0 -p1 inverse and forward commands pass: `execution/inverse-proof/result.json`, SHA256fb497d3b4389b2a9bc384493ce7bcb5c9ad344c6abcfca817214597c5f6f7a0a. The inverse restores every one of the seven existing root-before owner bodies and tracked.rs true absence; forward application reproduces all eight final hashes. Both raw patch logs and the full patch are retained. The proof never patches live source or replaces Plan30's original7359/f077 inverse authority.

## Decisions, Simplification and Deviations

The simplification pass retained only three needed aliases and one sealed source method. The implementation moves the existing raw body rather than adding a reader or generalized storage abstraction. Existing inferred snapshots and callers require no extra owner. Publication and fixture-helper edits stay narrow, at619 and609 lines respectively. The new source has an immediate infrastructure breadcrumb; Plan19 alone owns the central manifest.

One evidence-orchestration setup issue was corrected under Rule3: the ignored final-command driver initially indexed --key one token too late and refused before spawning any Cargo/Bazel child. Its original source, exact tool-output text, argv, environment, exit1 and honest timing limitation are retained under execution/driver-setup. Its52 source copies explicitly reproduce the authenticated unchanged freeze and do not claim a contemporaneous capture. The corrected driver changes no repository source or planned command. This is setup failure, not behavioral RED or retry-until-green. There were no source-scope deviations, authentication gates or goal-blocking stubs.

The repo-local guidance, Bright Builds sidecar/overrides and architecture, code-shape, testing, verification, local-guidance and Rust standards informed ownership, native behavior preservation and verification. Both canonical active lesson files were read completely:8856 bytes and2953 conservative estimated tokens. The2026-10-19 audit baseline remains within its thresholds; no new audit trigger or lesson mutation was needed.

## Remaining Gates and Preserved History

The existing physical-inventory RED remains real and unfixed until39. Its immutable result94aa911c7deb4a7290d9e8422da9d6bc74b58dab641146f498c806ba9f21ba8d and rawc2cc1c7da3541c28d529825e73f0444734d3049fef8d7ff40320b7ef35484785 retain root authentication4073f9d8. Original complete retained-bound failure33ac77c9/raw6af31048 and historical32-source freeze3f07f102 also retain their original source/outcome. The unchanged590-line Node probe and its original tests remain discoverable. No test was ignored, deleted, weakened or relabeled passed by this refactor.

The complete independent source/security/API/evidence/inverse review is CLEAN with zero findings: `.local/open-bitcoin-dev/phase167/plan37/independent-review/REVIEW.md`, SHA25602e52e5b77ef08bb33a668ccb1321c2cebdabf959740935adfad99f859bec776; hashes.json SHA2563c15dac35cbb857c9da6cf9ca29f61ba5e4118d7bfdeeaecac08c918647c161c. Its complete narrative was read. It independently authenticates972 unique referenced files,931 per-job/freeze source copies,201 separate engine/lock inputs and a strict eight-source inverse/forward reproduction. It confirms67 overlapping final test executions plus the one historical Task1 execution. Original summary/document QA is preserved under execution/document-QA; the bounded review addition has separate document-QA-revision2 receipts.

Root acceptance and fresh38/39 releases remain separate. Plan30, all global requirements, whole-phase source/native/coverage/Bazel/code/security/formal/lifecycle/daemon/differential/Git obligations remain open. No Phase168 work or qualification claim follows this summary.

## Self-Check: PASSED

All eight declared source files and required local evidence exist. Every final source hash and52-state before/after capture matches its retained copy; all201 separately versioned engine/lock inputs match their records. Exact inverse and forward restoration passes, normal formatting and628-line caps pass, and all eight planned verification commands plus the final-source Task1 repetition pass. No commits are expected under commits:false. Summary parser/formatter and eight-file verification receipts are retained separately.
