---
phase: 167-candidate-history-validation-rules
plan: "13"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: rpc-canonical-history
tags: [rust, rpc, canonical-history, submitblock, evidence]
requires:
  - phase: 167-12
    provides: Accepted sealed canonical history producer and consuming node runtime
  - phase: 167-30
    provides: Accepted canonical submission command and selected validation authority
  - phase: 167-47
    provides: Accepted read-only earned submission observation
provides:
  - Configuration-only sealed RPC history factory with private same-command submission and observation
  - One baseline-compatible SubmitBlock method and earned public chain-information projection
affects: [167-48, 167-14, 167-15, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Sealed paired runtime, private command bridge, detached observation, exact source frames]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/context/canonical_history.rs
    - packages/open-bitcoin-rpc/src/context/canonical_history/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch/submit_block.rs
  modified:
    - packages/open-bitcoin-rpc/src/context/canonical_chain.rs
    - packages/open-bitcoin-rpc/src/context/canonical_genesis.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
    - packages/open-bitcoin-rpc/src/context/wallet_state.rs
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-rpc/src/method/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
key-decisions:
  - Keep the submission and observation bridge private and bound to the same earned command Arc.
  - Preserve generic and fixture refusal, unavailable wallet and separate header observation.
  - Expand the verified source frame to all RPC source files after strict Clippy exposed one omitted input.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-27T10:17:48.218286+00:00"
completed: "2026-10-27T10:17:48.218286+00:00"
commits: false
status: source-core-accepted-independent-summary-qa-pending
---

# Phase 167 Plan 13: Sealed RPC History and SubmitBlock Summary

**All four original post-repair commands, the expanded scratch inverse, sealed CORE and independent CORE review passed. This canonical Summary records Plan 13 for independent document QA; Root FINAL13 and whole Phase 167 acceptance remain pending.**

## Bounded behavior

The specialized `from_canonical_history_runtime_config` factory takes configuration, obtains the genuine paired sealed node runtime and command, and returns an RPC context with a private submission/observation bridge bound to that same command Arc. Generic and fixture context paths retain `None` submission authority and refuse unproved ordinary history. The wallet stays unavailable. `submitblock` is the single added baseline node method: normalization, strict ASCII-hex and complete wire decoding precede a full `WireBlock` handoff to the earned command. It maps accepted, duplicate, inconclusive, conclusive invalid and operational outcomes without inventing a public authority setter.

The canonical `getblockchaininfo` branch reads actual accepted active blocks/hash/MTP from a detached observation, then separately reads genuine best-header height. Persisted peer status cannot mask a newly earned tip, and a nonpermanent body refusal may leave headers ahead without advancing accepted coins. This plan does not activate unattended networking, qualify other ordinary chains, make a wallet available or claim a coherent instant across the separate reads.

The checked Plan 13 has eleven original RPC source owners. Strict Clippy closure added two existing test-only helper owners under the separately checked amendment, giving thirteen final source owners. All thirteen live bodies match the post-repair capture and are below the normal 628-line threshold.

| Final owner                                                        | Bytes | Lines | SHA-256                                                            |
| ------------------------------------------------------------------ | ----: | ----: | ------------------------------------------------------------------ |
| `packages/open-bitcoin-rpc/src/context/canonical_chain.rs`         |  9860 |   259 | `a6ed586eb42dc50901b518572cb2eb4eb1380d6433fbb9886a8cb61e13ab33df` |
| `packages/open-bitcoin-rpc/src/context/canonical_genesis.rs`       |  5623 |   130 | `8d3dc8a8516022f647ee159dd390c81ecd5e679fa463c1b91325ae05b03bd889` |
| `packages/open-bitcoin-rpc/src/context/canonical_history.rs`       |  6776 |   153 | `b8e797712a1533baddf56fd38c94c24c6c20573d86f0195a5cb7f54c324f90a9` |
| `packages/open-bitcoin-rpc/src/context/canonical_history/tests.rs` | 20067 |   539 | `95c7f661bf3bd7f67b205627ec92b0c1b72b2b559548c2effac5ac0fa4c1ce87` |
| `packages/open-bitcoin-rpc/src/context.rs`                         | 18825 |   583 | `076be80b2ef756f67dbc3c4c3967fe120e021a3b61955f7687e8d40978defef0` |
| `packages/open-bitcoin-rpc/src/method.rs`                          | 19146 |   522 | `efa7df573af4cb99f0ec6dae6732fd986f975e95febb280f9d6744cf0f103876` |
| `packages/open-bitcoin-rpc/src/method/tests.rs`                    | 17254 |   526 | `8d967aa2fcdec3c7362d1b4f104450a382d700c472c82ddec9ee504b24d027f2` |
| `packages/open-bitcoin-rpc/src/dispatch.rs`                        |  9799 |   229 | `50459bf5b3a2a506086c888e144c87f1f5e6e92b28f6b937c59ded515473567b` |
| `packages/open-bitcoin-rpc/src/dispatch/submit_block.rs`           | 12008 |   317 | `fd18eabe57ba98615421d223b45e9415ffc3d99b7571360eecf3a70b38992764` |
| `packages/open-bitcoin-rpc/src/context/network.rs`                 | 20871 |   576 | `134707156e8e5cc35533ef92392612cb48f4fe3582bda6e418e11f6629743dbf` |
| `packages/open-bitcoin-rpc/src/dispatch/node.rs`                   | 21826 |   567 | `acaf0c4a7d62d9947c20f13c0ab4894bc46fdca75950038b38590147c8692320` |
| `packages/open-bitcoin-rpc/src/context/mempool_recovery.rs`        | 17803 |   520 | `64e1be8722751da5d16f3315d0f715a00e75cdc3e0e5db40cdb5cf926347be06` |
| `packages/open-bitcoin-rpc/src/context/wallet_state.rs`            | 21932 |   582 | `d67c65c42629c9573392ff9ca974adbff69fa1e41bd97905d8af564477e4e431` |

## Provenance and staged behavior

Root Plan 12 acceptance is `.local/open-bitcoin-dev/phase167/plan12/root-source-acceptance.json` (SHA-256 `f7ec6e6455ba1002196062d072d430459e53f19323e8d40d1558266ab8be6268`), Plan 30 is `.local/open-bitcoin-dev/phase167/plan30/root-acceptance.json` (`6b97b4ad35305c7e52840ee1aa40bb4b33bc428746ff520873f3d210a28d738c`), and FINAL47 is `.local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/final-source-acceptance.json` (`516eb6f3ed16f3c1819f507f29124fe413d3f2f8ce3dfd266d6c36c973e07072`). Plan 47 binds the accepted Plan 35/45/46 ancestry.

The exclusive original eleven-owner capture `.local/open-bitcoin-dev/phase167/plan13/root-after-final47-fresh13-v1/original/capture.json` (`d3f2cd358cf14412cd72b57184102e4ddcd761accb88828976f68b9daf18d1a1`) preserves eight whole bodies and three true absences. The first accepted-47 causal frame had 335 source paths plus 201 native inputs and a separate Byteview input. A later strict Clippy run exposed one compiled RPC file outside that bounded frame. The checked Clippy closure amendment added all 103 missing RPC source paths. Fresh pre-repair and post-repair captures have 438 source paths, 436 present and two true absences, with thirteen owners, 425 protected paths, 201 native inputs and one separate supplement. The post-repair capture `.local/open-bitcoin-dev/phase167/plan13/root-strict-clippy-repair-v1/post-repair-capture/capture.json` (`041780aeec6384a244c3e0202c0d2d7e4c83b324ad99da28cf9dd48e07cb9f75`) and independent audit `.local/open-bitcoin-dev/phase167/plan13/independent-post-strict-clippy-capture-v1/review.json` (`eb94a703efa8d46015afde0c45599eaa3df3a8c449aa5c1c56f50301ce9d96c1`) authenticate all 638 present copied/live bodies.

The test-first sequence retained genuine failure classes. An initial history-test compile exited 101 for a moved `OutPoint`; Root recorded it as a compiler diagnostic with no behavioral RED credit. The one-line `outpoint.clone()` test-fixture repair was separately reviewed, and the rerun failed at the actual unavailable observation owner. The final factory then passed that boundary, but the same test could not be counted as an early GREEN because `submitblock` was still absent from the old method registry. A separate interim registry test failed at genuine method-not-found. After registration and dispatch, the old public node projection failed the stale-status assertion: saved blocks zero versus earned blocks one. The final node projection closed that path. Exact stage/repair/milestone receipts are listed in the companion checklist; these failed runs are not final-source pass credit.

An earlier final-capture helper compared the same eleven owners in different path orders and held after writing a partial frame. Root recorded that failure at `.local/open-bitcoin-dev/phase167/plan13/consumer13-execution-after-final47-v4-v1/runtime-v5/final-capture-hold.json` (`7c952c81a27640edb1a572c35b3635ea3bf4811e8ed9d9ace3e10b75c43a48af`). The reviewed V5d recovery captured the complete eleven-owner frame separately at `.local/open-bitcoin-dev/phase167/plan13/consumer13-execution-after-final47-v4-v1/final-v5d/capture.json` (`13c2d5bd1506b1e37fed67ef414cf70d8222bb3d74881bb5674045029ffd32e9`). Neither old frame substitutes for the later post-Clippy 438-path capture.

The first eleven-owner final frame passed seven history tests but still emitted two test-only dead-code warnings. The serial helper held, and strict Clippy exited 101 on those helpers, an even-length expression and two test-only complex types. The reviewed four-file repair gated the helpers/imports to tests, used `is_multiple_of(2)`, and named the complex test types. Root applied exactly those bodies; independent actual-source review found no extra edits. The post-repair formatter and early strict Clippy both exited zero. The independent expanded source review `.local/open-bitcoin-dev/phase167/plan13/independent-expanded13-final-source-code-review-v1/review.json` (`27b74e148b0d708422d5fdd8275f0e6bca28a37ad31209e7d2def5d695d12188`) found no actionable code issue. The later four-command, inverse and CORE receipts close the final-source evidence for this Summary stage.

## Original commands on the post-repair capture

All Cargo commands used the repo-owned Bun timing wrapper and one shared target directory serially. Each row closed with native exit zero, exact 438/201/1 before/after projections, and no warning or error diagnostic. Filtered zero-test targets do not add pass credit. The 22 named passing executions overlap across rows 1 and 3 and are not 22 distinct tests.

| Order | Original key                       | Named passes | Native exit | State                                                                                   |
| ----: | ---------------------------------- | -----------: | ----------: | --------------------------------------------------------------------------------------- |
|     1 | `phase167-13-task1`                |            7 |           0 | Closed; 301-block reopen, maturity, refusal, header-ahead and same-command cases passed |
|     2 | `phase167-13-submitblock`          |            8 |           0 | Closed; one registry and seven dispatch/decoding cases passed                           |
|     3 | `phase167-13-factory-after-submit` |            7 |           0 | Closed; final same-command factory and public history tests passed again                |
|     4 | `phase167-13-submit-clippy`        |            0 |           0 | Closed; strict all-target/all-feature Clippy with zero diagnostics                      |

Actual child closures in row order are `.local/open-bitcoin-dev/phase167/plan13/root-strict-clippy-repair-v1/final-jobs/01-phase167-13-task1/child-completion.json` (`8910b7b119a52b9f71eeabd1a47d94a74fc82977f5f426b2f44ea606a2ac6c1a`), `.local/open-bitcoin-dev/phase167/plan13/root-strict-clippy-repair-v1/final-jobs/02-phase167-13-submitblock/child-completion.json` (`67c3aaddc38be16232bb62691ecda0f1b320a76bb92c886d5369dc4566a25a54`), `.local/open-bitcoin-dev/phase167/plan13/root-strict-clippy-repair-v1/final-jobs/03-phase167-13-factory-after-submit/child-completion.json` (`5b772627b427e9d3e573375a4f09eaad948f5eec1efc10e82ab378be100a80f5`), and `.local/open-bitcoin-dev/phase167/plan13/root-strict-clippy-repair-v1/final-jobs/04-phase167-13-submit-clippy/child-completion.json` (`4d5c9c71cc18fb5e74b0f5f396579c03b54cb6195aa2bec67c651c4973c04d1d`). Root's ordered four-command review `.local/open-bitcoin-dev/phase167/plan13/root-strict-clippy-repair-v1/final-commands-root-review.json` (`8a6870816592e033b15c13ece8021592662e005935e0a121e5c15e8963534cb8`) and independent review `.local/open-bitcoin-dev/phase167/plan13/independent-four-original-commands-after-lint-v1/review.json` (`870314c8aad9773aaeb223cb0042ddaed6ba566b64f6d4c717a865b89aaec778`) bind actual raw streams, named discovery and unchanged frames. The earlier strict Clippy exit zero is diagnostic evidence only; row 4 is the required original final-source Clippy pass.

## Expanded scratch inverse and sealed CORE

After all four commands, the V2 inverse permission failed before creating scratch output because its helper compared different physical reference roles. The explicit HOLD `.local/open-bitcoin-dev/phase167/plan13/root-strict-clippy-repair-v1/expanded-inverse-v2-pre-effect-hold.json` (`59b7821749bcf0d09536310c511853f33130f9642bbfbe92a6ad949d677543e3`) receives no inverse credit. The corrected V3 scratch inverse `.local/open-bitcoin-dev/phase167/plan13/root-strict-clippy-repair-v1/expanded-inverse-execution-v3/completion.json` (`cb8bb0c4f366411a3895a3ed407ca3bc3f78b8115265ec2c8ceee455465ac561`) restored eight original whole bodies and three true absences, restored the two promoted whole before bodies, preserved all 425 protected source paths and native201 plus the separate supplement, then reapplied thirteen final bodies. It made zero live source writes. Independent actual inverse review `.local/open-bitcoin-dev/phase167/plan13/independent-actual-expanded-inverse-v3-review-v1/review.json` (`f8da8a9b8d0ca4c4d417c75624f15f165a93c3a526e8605d1f3d4f8d0d52ee87`) passed with zero findings.

The sealed CORE `.local/open-bitcoin-dev/phase167/plan13/actual-core-after-lint-v2/CORE.json` (`d6243c66e387f5d69e403a488e9cfb2a04d6de63259d413424a1c48f8f04ef9a`) binds accepted prerequisites, original and final frames, staged failure classifications, thirteen final owners, four closed original commands, Root/independent command reviews, scratch inverse and independent source/capture/inverse reviews. It records 22 named passing executions, zero failed, zero ignored pass credit, three genuine behavioral REDs, and no compiler-only or early non-GREEN pass credit. V2's independent CORE review `.local/open-bitcoin-dev/phase167/plan13/independent-actual-core-v2-review-v1/review.json` (`19cc7a8bbd029d48df1bf477110ae579fd977c6a263dd83aa6da8be5c9cc8cb1`) has zero findings. Root released these exact V2 bytes for the canonical Summary at `.local/open-bitcoin-dev/phase167/plan13/root-strict-clippy-repair-v1/summary-stage-release.json` (`3e6444b8228c2af6c85b81e2896616da4b3a88670e7e40e6c104a2715dc58174`). This release does not itself accept Plan 13 source.

## Residual gates

- Independent document QA and Root FINAL13 are pending on this canonical Summary. Plan 48 has a separate five-owner daemon/process contribution and remains held until FINAL13.
- Plan 19 must map the three new Plan 13 Rust files in `docs/parity/source-breadcrumbs.json`. Inline breadcrumbs exist, but whole-phase parity verification remains open.
- Global CVLD-01/02/03/04/06, whole Phase 167, default `bash scripts/verify.sh`, fresh coverage, Bazel, security/goal/lifecycle review and normal-hook commit/push remain separate gates. Stop before Phase 168.

## Self-Check: PASSED

This Summary binds thirteen exact final source bodies, the 438/201-plus-one post-repair frame, all four original command results, the V3 scratch inverse, sealed CORE V2 and independent zero-finding CORE review. Historical compiler-only, attempted early-GREEN, capture-order, warning/Clippy and pre-effect inverse holds receive no final pass credit. Independent document QA, Root FINAL13, global CVLD closure and whole Phase 167 verification remain pending.
