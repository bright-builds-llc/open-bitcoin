---
phase: 162-real-prune-retention-and-integrated-parity-proof
status: passed
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-12T04:26:36Z
source_digest: a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575
execution_implementation_scope_digest: 3750ddc7d23753bd1cc1d016cee81b68fab34583072178088fc4e3ec5940f27a
execution_review_scope_digest: 1adc17e8332c1805647a4675f029bdf9f0457bdb33ff479acae3854139deb407
publication_review_scope_digest: 44e1df5374ebdf91f601d99431b17f4cb1c7e48b7e35e9943db8d84e95740095
publication_implementation_scope_digest: 9663f807d975196433987eee02a4f8bb766bbb7f5d60cc666f145d69870c9cdf
execution_started_at: 2026-10-12T02:51:15.991000Z
execution_ended_at: 2026-10-12T03:26:53.544000Z
---

# Phase 162 Native Verification Evidence

## Successful Entire Default Contract

Attempt 5 completed the entire default/full repository contract with exit **0**, inner duration **2,136,960ms (35m36.960s)** and cooperative-wrapper duration **2,137,553ms**. Actual receipt: `.local/open-bitcoin-dev/command-timings/verify-full/2026-10-12T02-51-15.991Z-6c7c6001-7fbd-42ec-875d-840c0c380707.json`; run ID `6c7c6001-7fbd-42ec-875d-840c0c380707`. Raw execution log: `/tmp/phase162-native-full-5.log`.

Actual invocation supplied the pinned PATH prefix and redirected the complete log. Its effective command is:

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
bash scripts/verify.sh
```

The default verifier acquired its own cooperative build lock through the `verify-full` timing runner. No Cargo/Bazel job overlapped it, and no source changed during execution. Environment: Darwin arm64, Bun1.3.9, Rust1.94.1 (`e408947bf 2026-03-25`), Bazel8.6.0, main baseline `a77edab276a9daa086055aa61e6143ce86bc1218`, pinned clean Knots `a9aee730466ac67d35a3c03ee24676be5e045878`. No dependency or toolchain change was required.

Passed stages include every historical/current Bun claim and mutation suite, parity breadcrumbs, documentation/lifecycle reconciliation, dependency/import/file-length/panic policies, workspace formatting, shared fixture rustfmt, strict all-target/all-feature Clippy, all-target/all-feature build, full workspace tests/doctests, explicit CLI RPC-client binary, benchmark list/smoke/report, all six Bazel smoke targets, build provenance and the exact native pure-core coverage helper. The helper completed without an `Uncovered Lines:` section. No failing Rust result group exists in the successful log.

Selected nonempty Rust result groups: chainstate328/0; node1695/0 with4 declared ignores; RPC library375/0 with1 declared ignore; daemon104/0 with1 declared ignore. The complete `phase162_integrated_lifecycle` and the corrected older automatic-prune/reopen case both pass. The ignores include pre-existing opt-in controls and explicitly supervised process-child entry points; they are not credited as default behavior tests. The log contains7,128 Bun pass records and zero failures; this is a sum of suite result lines, not a claim of unique test identities.

The six-anchor digest above binds the proof roots. The separate implementation digest binds every Rust/TypeScript/Python/shell implementation or test path in the70-file independent scope plus the breadcrumb registry. The review digest additionally covers current documentation/index bytes at the pre-activation freeze. Source hashes supplement executed behavior; they do not replace it.

## Preserved Failed Default Attempts

| Attempt | Actual interval UTC       | Inner ms | Wrapper ms | Exit | Failure boundary                                                                             |
| ------- | ------------------------- | -------: | ---------: | ---: | -------------------------------------------------------------------------------------------- |
| 1       | 00:52:06.581–00:52:13.669 |     6677 |       7088 |    1 | Phase80 raw `/proc` filename false positive, before Cargo                                    |
| 2       | 01:08:24.863–01:09:58.007 |    92743 |      93144 |    1 | Phase157 missing explicit remaining-deferred scope,654pass/3fail, before Cargo               |
| 3       | 01:22:41.870–01:25:20.587 |   158322 |     158717 |    1 | Phase158 formal fixture/missing required-root failures,586pass/3fail, before Cargo           |
| 4       | 01:45:02.413–02:01:00.919 |   957941 |     958506 |  101 | All guards/format/Clippy/build passed; stale core source guard stopped tests at327pass/1fail |

All intervals are2026-10-12. Actual receipt filenames under `.local/open-bitcoin-dev/command-timings/verify-full/` are respectively:

- `2026-10-12T00-52-06.581Z-ec6a6493-03f3-41e3-bbb2-8ed21b1276a0.json`
- `2026-10-12T01-08-24.863Z-b2d1cedd-7a3f-400b-8658-53f09040b4e8.json`
- `2026-10-12T01-22-41.870Z-79f1e1cb-eb05-4022-9511-b63728ceee9c.json`
- `2026-10-12T01-45-02.413Z-2f20e759-9f45-4171-8336-2e883e9c5c76.json`

Logs are `/tmp/phase162-native-full-1.log` through `-4.log`. None is relabeled as a passing full contract. The narrow Phase80 correction required a second hardening after independent WR-02 found shell metacharacter escapes; actual62/0 and exported-checker controls close it. Phase157 retained its guard and regained one truthful D-15 deferral gap. Phase158 retained its semantic guards and repaired synthetic all-document transitions plus bounded missing-requirements handling; inverse patches reconstruct original guard bytes and both frozen-source tamper refusals remain effective. Two existing Rust tests now recognize the actual delegated IfNeeded path with snapshot prohibitions preserved.

Durable records: [native guard repair](162-NATIVE-GUARD-FIX.md), [parity repair](162-NATIVE-PARITY-FIX.md), [claim/loader repair](162-NATIVE-CLAIM-DIAGNOSIS.md), [Rust guard repair](162-NATIVE-RUST-DIAGNOSIS.md), [independent review](162-REVIEW.md) and [security](162-SECURITY.md). The earlier independent WR-01 source-map isolation correction is recorded in [review fixes](162-REVIEW-FIX.md).

## Additional Tail Failure and Verified Repairs

The first remaining-workspace preflight exited101 after1,104,776ms. Node1695/0/4ignored and RPC375/0/1ignored passed; daemon102/1/1ignored included a passing completePhase162 lifecycle. Its sole older automatic-prune codec fixture had six sparse headers with null parents. The writer retained them and the strict cold reader correctly refused missing ancestry. The one-file fixture repair supplies coherent computed0..1001headers while preserving six sparse payload/coins heights, original pressure/body/undo measurements, every main retention assertion and its unvalidated boundary. No production validator was relaxed. One initial fast-test compilation failed on private API access/borrow shape; the correction expanded no visibility or API.

Actual repair verification: fast ancestry/typed-refusal1/0 in1.09s; unchanged pressure/reopen1/0 in17.36s; all automatic-prune7/0 in148.12s; strict RPC lint; shared127suite18/0; live127/154–162; parity1141Rust files; managed0; source/cleanup invariants. [Prune diagnosis](162-NATIVE-PRUNE-DIAGNOSIS.md) preserves the detailed receipts, initial compilation failure and public provenance boundary. Only this fixture changed among its1,464-file snapshot; runtime, six roots and main retention test remained byte-identical.

The resumed tail separately passed remaining workspace64871ms, all workspace doctests36059ms, CLI539ms, benchmark list213ms/smoke5893ms/report89ms, six Bazel targets7685ms/provenance13049ms, coverage clean1023ms/native coverage25861ms. Every receipt exits0. Its2,332,833-byte coverage text at `/tmp/phase162-native-tail-coverage.txt` has no uncovered-lines section. These preflights informed the subsequent clean entire Attempt5; they do not substitute for it.

## Closure Boundary

At initial publication, all three requirements remain Pending and the parity owner remains in_progress. Fresh formal goal verification, consistent requirement/document activation and the normal mandatory full commit hook precede commit/push. v2.5 audit/archive/shipment remain separate. Later report-metadata refresh must preserve the actual execution interval and source boundary rather than imply a new native run.

## Formal Publication Refresh | 2026-10-12T03:40:12Z

The actual full execution interval, receipt, six roots and implementation digest remain unchanged. This refresh follows initial formal PASS and consistent three-requirement/owner/seven-document activation; it does not imply a new native execution on the activated document state. The final index still requires the normal full commit hook. Its actual outcome is derived from Git and the hook timing receipt. Closing independent reviews and fresh lifecycle reconciliation follow this publication timestamp; v2.5 audit/archive/shipment remain separate.

Publication prose refresh at 2026-10-12T03:43:29Z corrects stale pending-gate text in the six current documentation blocks and current links. The actual native interval, execution review digest, code digest and receipt are preserved; this metadata refresh is not another execution.

Publication data refresh at 2026-10-12T04:05:08Z replaces only the stale Phase158 current serving gap with the exact scoped completed sentence required by its unchanged guard. Native code, six roots, execution interval and receipt are unchanged; no new execution is implied.

## Final EOF Normalization | 2026-10-12T04:26:36Z

Staged whitespace review found an extra terminal blank line in the contract table and planning artifacts. Root removed only terminal LF bytes; the contract file before normalization equals its current bytes plus one LF exactly. No executable statement, function, assertion, selector or six proof root changed. Independent semantic findings and goal verification remain applicable. Current byte review scope is`44e1df5374ebdf91f601d99431b17f4cb1c7e48b7e35e9943db8d84e95740095` and current implementation byte digest is`9663f807d975196433987eee02a4f8bb766bbb7f5d60cc666f145d69870c9cdf`; the actual Native5 execution digest remains`3750ddc7d23753bd1cc1d016cee81b68fab34583072178088fc4e3ec5940f27a`. The normal mandatory full commit hook verifies this final normalized index before a commit is created. This mechanical binding refresh does not claim another completed native execution.
