---
phase: 167-candidate-history-validation-rules
plan: "35"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: consensus-network-fixtures
tags: [rust, compact-block, witness, mutation, evidence]
requires:
  - phase: 167-30
    provides: Accepted selected witness commitment and normalizer contract
  - phase: 167-32
    provides: Shared block and transaction views and serialization facts
  - phase: 167-34
    provides: Accepted compact provenance foundations
  - phase: 167-43
    provides: Accepted physical inventory repair
  - phase: 167-44
    provides: Accepted missing-metadata repair
  - phase: 167-46
    provides: Accepted original-peer lifetime authority
  - phase: 167-45
    provides: Accepted Session and compact completion integration
provides:
  - Shared fallible pure block mutation predicate and witness fact wrappers
  - Coherent witnessed compact-block test fixtures with complete-body commitments
affects: [167-47, compact-lifecycle-CDE, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Borrowed block views, shared witness facts, complete-body-before-identity fixtures]
key-files:
  created:
    - packages/open-bitcoin-consensus/src/block/mutation.rs
    - packages/open-bitcoin-consensus/src/block/mutation/tests.rs
    - packages/open-bitcoin-network/src/test_block_commitments.rs
  modified:
    - packages/open-bitcoin-consensus/src/block.rs
    - packages/open-bitcoin-consensus/src/block/witness.rs
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-network/src/compact_download/tests.rs
    - packages/open-bitcoin-network/src/compact_download/tests/completion_cases.rs
    - packages/open-bitcoin-network/src/peer/tests/compact_fixtures.rs
    - packages/open-bitcoin-network/src/peer/tests/compact_download_fallback_cases.rs
    - packages/open-bitcoin-node/src/network/tests.rs
key-decisions:
  - Preserve the exclusive original 11-file before-image while incorporating five accepted repair origins.
  - Keep test fixture authority separate from production compact classification.
  - Retain original six focused and three final verification commands without substitution.
requirements-completed: [CVLD-01, CVLD-03, CVLD-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-26T01:55:33.014Z"
completed: "2026-10-26T01:55:33.014Z"
commits: false
status: source-core-accepted-independent-summary-qa-pending
---

# Phase 167 Plan 35: Pure Mutation and Coherent Compact Fixtures Summary

**The corrected source implements a shared fallible mutation predicate and coherent witnessed compact fixtures. All nine R2 commands, CORE sealing and independent CORE review passed; document QA and final source acceptance remain pending.**

This canonical Summary records the closed R2 verification and independently reviewed CORE under the Root Summary-stage release `.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-summary-publication-release.json` (3322 bytes; SHA256 `688d2b5269239fdf23dfcaa9cf71adbae6ad93d4666e97fd2bdf6b55dcd62564`). Independent document QA and Root final source acceptance remain pending.

## Bounded source behavior

The four checked tasks own exactly 11 source files. The public `block::is_block_mutated` wrapper delegates to one pure implementation; `block::witness_merkle_root` and the ordinary witness checks use shared borrowed facts. The test-only network helper is registered with `#[cfg(test)]`. Network and node positive fixtures construct the complete witnessed body before final commitments, block identity and compact short IDs. This bounded plan does not implement the later selected production compact classifier, first/later retry policy or stored-body lifecycle.

| Source owner                                                                      | Bytes | Lines | SHA256                                                             |
| --------------------------------------------------------------------------------- | ----: | ----: | ------------------------------------------------------------------ |
| `packages/open-bitcoin-consensus/src/block.rs`                                    | 15809 |   501 | `78990e0f2420f36839bb7d013790b5f2c3329edb1bd229cfc2c6beb91c81cae3` |
| `packages/open-bitcoin-consensus/src/block/mutation.rs`                           |  2692 |    75 | `af4e7fa38c2abe41789f5c35d25207017f8fe0eff643ee0ba94b48434d9a7e18` |
| `packages/open-bitcoin-consensus/src/block/mutation/tests.rs`                     | 11071 |   366 | `0853162939030601333bb424260810c567621637a85e746d1fa1fe6cc3b81613` |
| `packages/open-bitcoin-consensus/src/block/witness.rs`                            | 23709 |   622 | `ae9a38091bf2a4626832893b027a35fde2255e16bc781b705e74effc9380afcb` |
| `packages/open-bitcoin-network/src/compact_download/tests.rs`                     |  7219 |   191 | `80ff604b0b0e29fe83b321e7b875b22cc282feba2d71e749710d886ea8fd2650` |
| `packages/open-bitcoin-network/src/compact_download/tests/completion_cases.rs`    |  5206 |   157 | `2ea1c1ec4737d633eb65d82c29573269b43d2295714ddfb03f5f996361f9ed99` |
| `packages/open-bitcoin-network/src/lib.rs`                                        | 11650 |   223 | `56baff9390ade53893c84c5d473c6ac583216832ce9f4478787e0d3281158460` |
| `packages/open-bitcoin-network/src/peer/tests/compact_download_fallback_cases.rs` |  8173 |   233 | `312cdf24d88264e8b36f04886e1f03c14e1defa54893a5766ce8ac71ad5810f3` |
| `packages/open-bitcoin-network/src/peer/tests/compact_fixtures.rs`                |  9711 |   280 | `304725f66151cbb3337022ccaa7f07a14db4a31f8b8ba84c81a01ff1b81776e8` |
| `packages/open-bitcoin-network/src/test_block_commitments.rs`                     |  1931 |    54 | `fbf945d4f5251606695bf3a9b22c0b519156696a8cd46137dbc078bf5c46181e` |
| `packages/open-bitcoin-node/src/network/tests.rs`                                 | 18773 |   581 | `0f9570a062ba4aad0091c0657e568c463dcc014fd5b2dac2aeb3a1cd179b46be` |

All 11 current owners are present and no file exceeds the normal 628-line threshold. Three source files were genuinely absent in the exclusive original before-image; those absences are not replaced by empty files or current-present copies. The changed-transaction test control now retains the missing transaction, changes its `lock_time` against the unchanged committed header and expects the shared predicate to detect mutation. The original five controls remain.

## Provenance and source conservation

Exclusive original Plan 35 authority is `.local/open-bitcoin-dev/phase167/plan35/root-source-before-after30-v2/capture.json` (140423 bytes; SHA256 `5b0b899b3604e5d15e72bc3671d51c518c996ee359c4fe8441886333af8991c7`): 82 historical source states, 11 direct owners, eight whole before bodies and three TRUE absences. Its failed full-run evidence remains failed, including four original test failures; no old pass or ignored result is reused.

Accepted origins are actual Root receipts for Plan 30 (`6b97b4ad35305c7e52840ee1aa40bb4b33bc428746ff520873f3d210a28d738c`), 43 (`07a82dab92c61febb2faa887ec7d6b4c15612c43b9735f05958fcbd6294ca293`), 44 (`2f26386b860a6540d269877ef52fc379fa618176e035cd741229a305eb3c3be3`), 46 (`8fc69656cb28d2aa097f7ae76de795c4efe228b64ff405f1322280fdd338fc0d`) and 45 (`bc9710892e4217a04eedc887c7d04208745f7d3bf558b47926f4f11135547154`). The reviewed R1 `.local/open-bitcoin-dev/phase167/plan35/root-after-final45-46-binding-v1/composite-binding.json` (23304 bytes; SHA256 `b97e9b1d0896f554d74b71f869ad9dc133f76f6f0130a54e5506d88a85d80424`) derived 284 source paths and 273 protected paths, separately retaining 201 native inputs and one Byteview supplement. Its explicit readonly additions were empty. The R1 Root capture `.local/open-bitcoin-dev/phase167/plan35/root-after-final45-46-current-v1/capture.json` (325186 bytes; SHA256 `5e8d7e81713ecfeb618899cd1019fa83435713ba548cae96d6967f4933a14d0a`) matched all 486 bodies and read records; it is later than the exclusive historical before-image and does not replace it.

The R1 all-nine record `.local/open-bitcoin-dev/phase167/plan35/execution-current35-v1/serial/all-nine-complete.json` (2688 bytes; SHA256 `7bcf143f64b34b12f2eba9033ca4ae9037cbd68b84b62ad31e0a334a0394a85c`) closed naturally. Its full ninth command had 3483 named passes, zero failures and four exact ignored results, with the historical 3401 occurrences/3400 distinct names covered. Independent source review then found the checked Task 3 changed-missing-transaction control absent, so R1 remains historical and supplies no R2 pass credit. Root's sole-owner correction `.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/correction-receipt.json` (3317 bytes; SHA256 `1d15db0464de7eaa525716ffcd58f096edf1425717044aae07daca74e3b7ca8d`) binds the exact old and new whole test body. The R2 Root capture `.local/open-bitcoin-dev/phase167/plan35/root-after-task3-correction-v1/capture.json` (323470 bytes; SHA256 `1f435562c6baa69c0a3dbb8fb163af2b042c7b5aeb0e6e19092cc9b912438a87`) authenticates all 486 current body copies and lossless read records: one changed source body, the other 283 source bodies, 201 native inputs and the Byteview supplement exact.

Two distinct R2 authorized scratch runs applied strict BSD `patch --batch -E -F0 -p1` in reverse then forward against the exclusive original8/3 and corrected current284. Their results are `.local/open-bitcoin-dev/phase167/plan35/root-composite-r2-first35-v1/result.json` (2416 bytes; SHA256 `f2a4019d4f1f3933a85276b691b90b6211284726294281e6100da6efdb41983a`) and `.local/open-bitcoin-dev/phase167/plan35/root-composite-r2-independent35-v1/result.json` (2435 bytes; SHA256 `6d340427131007f62a2315b9d0b0d46160279d88c32c6a42778076bd4cba2d0a`). Each preserved 273 protected source bodies, native201 and the separate supplement; both patch steps exited0 without offset or fuzz. The independent corrected-composite review `.local/open-bitcoin-dev/phase167/plan35/independent-r2-composite-inverse-review-v1/review.json` (8244 bytes; SHA256 `15ff83b66a0800ec7ba35c95774610ca97ba68acb30c429a70d58123b45fd6be`) has zero findings. Root's pre-runtime review `.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-root-pre-runtime-review.json` (9050 bytes; SHA256 `e1d2c2a846c74f69fb4ef0e634f5f78be98f58b3ab901e4aa652b565112af0cd`) binds both complete 13-reference run provenances. The inverse runs did not patch live source.

## Actual runtime verification: nine commands closed

The corrected R2 verification-only lease `.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-runtime-release.json` (5967 bytes; SHA256 `6adf545ae186810640a74285bfeb40f9a11ec75398aaf2bf5bdc699302043b6f`) passed the actual full authority metadata gate with source writes CLOSED. Its fresh final freeze `.local/open-bitcoin-dev/phase167/plan35/execution-current35-r2-v1/final/capture.json` (201798 bytes; SHA256 `c7586dcc342aaa58297d19ad85d68dd8ebd10d5cb826d02f3ece398d39eef8e3`) authenticated all 284/201/1 source/native/supplement rows. The original six focused and three final literal commands closed in order under `execution-current35-r2-v1/jobs/`. The complete R2 record `.local/open-bitcoin-dev/phase167/plan35/execution-current35-r2-v1/serial/all-nine-complete.json` (2713 bytes; SHA256 `ed2d73a0778ec511877728a9ba5a4d85820eed2d51554ea016618924fd1533a7`) binds nine exact results with native and wrapper exits0, complete streams and unchanged before/after source groups. Root's raw-result review `.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-root-nine-review.json` (3566 bytes; SHA256 `39e8aa74922e3eb3ce736912dd19066ecaf2cce2d9611cd17a79c8ccaea1029c`) confirms full09 named 3483 PASS, zero FAILED or unparsed, four exact ignored cases and the historical 3401-occurrence/3400-distinct lower bound. Ignored cases receive no pass credit. Independent CORE review passed with zero findings; canonical Summary QA and Root final source acceptance remain PENDING.

The six focused selectors cover pure mutation, ordinary witness, network compact and node compact tests. The three finals run whole-workspace format check, strict all-target/all-feature Clippy for consensus/network/node and all-feature tests for those three packages. Final09 is the complete three-package command, not the whole-workspace verifier; Plan20 retains that responsibility. The old full-run baseline has 3401 test-name occurrences/3400 distinct names and four exact ignored cases; current outcomes must be independently counted from raw complete output. Ignored tests receive no pass credit. No nested commit is expected under `commits:false`.

| Original sequence | Stable key                  | Result | Native exit | Named passing tests | Ignored |
| ----------------: | --------------------------- | ------ | ----------: | ------------------: | ------: |
|                 1 | `phase167-35-task1`         | PASS   |           0 |                  18 |       0 |
|                 2 | `phase167-35-task2-matrix`  | PASS   |           0 |                  18 |       0 |
|                 3 | `phase167-35-task2-witness` | PASS   |           0 |                   7 |       0 |
|                 4 | `phase167-35-task3`         | PASS   |           0 |                  34 |       0 |
|                 5 | `phase167-35-task4-network` | PASS   |           0 |                 181 |       0 |
|                 6 | `phase167-35-task4-node`    | PASS   |           0 |                  92 |       0 |
|                 7 | `phase167-35-final-format`  | PASS   |           0 |          check only |       0 |
|                 8 | `phase167-35-final-clippy`  | PASS   |           0 |          check only |       0 |
|                 9 | `phase167-35-final-tests`   | PASS   |           0 |                3483 |       4 |

The nine commands contain 3833 named passes in aggregate, zero failures and four exact ignored tests. That aggregate includes repeated coverage across focused and full commands. The changed-included-missing-transaction control runs in the corrected Task 3 focused test and the full09 test; the Root review independently binds the corrected source body and both results.

## Sealed CORE and independent review

The first Root sealing permission `.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-core-seal-permission.json` (1800 bytes; SHA256 `7d72684e6ebbe1ccef620a23e2955ff9fd29debf3a1f6f5ee54d64e65d79326d`) encountered a pre-effect reference-path mismatch: its sealer path was relative while the runtime resolved the same file absolutely. The recorded HOLD `.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-core-seal-preflight-hold-v1.json` (744 bytes; SHA256 `1eeb830782faf58a24bac0fd71d84f64fe5dc5562be7a490e56544c4299a88f3`) confirms outer exit1, no CORE created and no source or product command run. The corrected immutable permission `.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-core-seal-permission-v2.json` (2324 bytes; SHA256 `213badb099a4781cf71d5fc091b53e079f11a4bc143332471c0e1ea2e02c0b1a`) binds the same reviewed sealer bytes with the absolute path and preserves the earlier HOLD.

The resulting `.local/open-bitcoin-dev/phase167/plan35/execution-current35-r2-v1/CORE-v1/CORE.json` (25736 bytes; SHA256 `2ffd745ba0da9053ad0251567abf23f3d36ac19db684a9697abce989556db046`) has status `PLAN35_ALL_NINE_ACTUAL_CORE_SEALED_PENDING_INDEPENDENT_REVIEW`. It binds the R2 lease, source frame, nine results and Root permission; its totals are 3833 passed, zero failed, four ignored and 3837 named test occurrences. Its `sourceAccepted` and `SummaryReleased` flags are false. This seal is bounded evidence only. Root accepted it for the Summary stage after the independent CORE review `.local/open-bitcoin-dev/phase167/plan35/independent-core-r2-v1/review.json` (4445 bytes; SHA256 `820b46823dec2b134ef1529f3551692475b8f93929068c0b31f9f24dc92d3799`) found zero issues and verified all nine raw jobs, 486 current bodies and the sole corrected test owner. Root’s stage review `.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-root-core-stage-review.json` (2718 bytes; SHA256 `df4b944c55a707be94c81cfc1b0441c333a9ca3003365053ce4394bc6ee6cacf`) keeps source acceptance false. The first independent-review release was held before invocation because its script would have rejected expected compile-fail doc-test diagnostics; `.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-independent-core-pre-effect-hold.json` (881 bytes; SHA256 `cb482272f8fbe019c86a551f9822a96b802c759ae543e3244d861645451e192e`) records no reviewer output. The corrected release `.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-independent-core-review-release-v2.json` (3139 bytes; SHA256 `756cc217c50dac81ac39eb8fad35d2553245f8332e90e85c6dff809acb8039c7`) authorized the successful independent review.

## Decision and residual gates

The source preserves ordinary witness diagnostics and the selected Plan30 normalizer. The pure mutation predicate and test helpers carry no admission or durable-tip authority. Later compact lifecycle units C/D/E must actually integrate selected-rule classification, first/later retry and stored-body cleanup before Plan17 guard renewal. Plan47 observation and Plans13/48 runtime integration also await their own actual source and tests. Plan19 must add source breadcrumbs and parity/docs evidence; Plan20 must run the full default verifier, fresh pure-core coverage, Bazel and final source/security/goal/lifecycle reviews. Global CVLD-01/02/03/04/06, whole Phase167 acceptance, Git finalization and Phase168 remain open.

## Publication and residual gates

- Independent CORE review and Root Summary-stage release passed, with exact receipts above. The canonical Summary is published once from this stage.
- Scoped GFM/frontmatter Markdown and GSD Summary checks are required on the saved canonical file before independent document QA.
- Root final source acceptance and shared STATE/ROADMAP/TODO count updates remain pending until independent Summary QA. Global CVLD requirements remain open.

## Self-Check: PASSED

This Summary binds the eleven corrected source bodies, nine closed R2 commands, Root raw review, sealed CORE and clean independent CORE review by exact references. Document QA, Root final source acceptance and all global CVLD requirements remain pending.
