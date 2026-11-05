---
phase: 167-candidate-history-validation-rules
plan: "47"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: node-observation
tags: [rust, chainstate, observation, inventory, provenance]
requires:
  - phase: 167-30
    provides: Accepted canonical submission command and selected validation authority
  - phase: 167-35
    provides: Accepted complete-body mutation and witnessed fixture source
  - phase: 167-45
    provides: Accepted peer Session and compact completion integration
  - phase: 167-46
    provides: Accepted original-peer lifetime authority
provides:
  - Bounded read-only observation of actual accepted chain and storage facts
  - Source-checked consuming native-row projection and detached public-data fold
affects: [167-13, 167-48, 167-14, 167-15, 167-16, 167-17, 167-20]
tech-stack:
  added: []
  patterns: [Operation-then-authority lock order, complete inventory freshness, owned detached observations]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/observation.rs
  modified:
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs
    - packages/open-bitcoin-node/src/storage/canonical_history/records/projection.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests.rs
key-decisions:
  - Keep the observation type opaque and consume it into five owned public-data facts after releasing locks.
  - Include every retained header automatically and bound explicit queries separately before their at-most-612 union.
  - Require the original unprojected inventory rows and a fresh full-source check before returning native maps.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-26T04:44:25Z"
completed: "2026-10-26T04:44:25Z"
commits: false
status: source-core-accepted-independent-summary-qa-pending
---

# Phase 167 Plan 47: Earned Submission Observation Summary

**The actual Regtest submission command now exposes a bounded, detached read-only observation. Source-bound Task 1, Task 2 and seven final commands passed; sealed CORE and independent CORE review are accepted for the Summary stage. Independent document QA and Root FINAL47 remain pending.**

This canonical Summary records the accepted source/runtime CORE contribution and its independently reviewed evidence, subject to separate document QA and Root FINAL47. The earlier compiler-only test signal and the held first independent CORE reviewer receive no behavioral pass credit.

## Bounded behavior and owners

The checked plan owns four files. The command observes complete chainstate, actual BASIC progress and stored BASIC state, all nine native namespace maps, and live versus durable validation provenance. It acquires the command operation lock before the authority read lock, checks the real owner, captures a bounded inventory before reading facts, checks the owner again and consumes the full original-source freshness proof after the fact reads. Returned observation fields are private. Its consuming fold invokes one caller only after the guards drop, so the caller receives owned public data without an engine or lock.

Queried hashes are read-only input. The command bounds explicit queries at 306 and retained headers at 306, includes all retained hashes automatically, and deduplicates the union at 612. The new crate-private inventory extractor checks the saved original row digest before checking current stamp, catalog, full rows and final stamp. It refuses projected rows even when the original storage source remains fresh. These changes grant no new admission, durable write, wallet, network activation or support qualification.

| Source owner                                                                                     | Bytes | Lines | SHA256                                                           |
| ------------------------------------------------------------------------------------------------ | ----: | ----: | ---------------------------------------------------------------- |
| packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs                   | 22467 |   548 | 967c20772a21dc802d1ff706eb43f9fc3a053464183f3355c5ce92c33204bd78 |
| packages/open-bitcoin-node/src/storage/canonical_history/records/projection.rs                   | 11806 |   306 | 6f7c74a158c7cb24313fd54eb0fc98f0939f421113c28082d5e77977fb0afab3 |
| packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests.rs             | 22240 |   627 | 995d7be38b205771d37054fb4d36bee94f3e7b1885bb0b6cc218b88292d2c058 |
| packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/observation.rs | 20389 |   553 | 25f238a3176526c0468752f1c159cdd06e3c5398166ae968e948807ecd384b20 |

The test child carries the pinned parity breadcrumb and stays under the normal line limit. It exercises enabled and disabled command facts, accepted and duplicate blocks, intrinsic and contextual refusal distinctions, projected rows, same-value and A-to-B-to-A source changes, foreign store and incarnation, post-absorption faults, the 306-plus-306 query bound, unlocked reentrant fold and genuine reopen. The public observer documentation example compiles through the inferred command type without naming its private module.

The independent four-owner source review (.local/open-bitcoin-dev/phase167/plan13/independent47-actual-source-review-v1/review.json; 4228 bytes; SHA256 47691999f478708b513f77e0fa306800df04571d2147e46fa14bd5242a6178ed) found no actionable source issue. The earlier current-API review (.local/open-bitcoin-dev/phase167/plan13/independent47-after-final35-api-review-v1/review.json; 7742 bytes; SHA256 e9095b8df584214e3ab3c9c65a06eb93398227f54c13512050e6b72ce3712669) established static fit before the source lease; actual compile and tests above provide the later behavior evidence.

## Prerequisites and fresh before-images

Actual accepted prerequisites are Root Plan 30 (.local/open-bitcoin-dev/phase167/plan30/root-acceptance.json; 6847 bytes; SHA256 6b97b4ad35305c7e52840ee1aa40bb4b33bc428746ff520873f3d210a28d738c), FINAL35 (.local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-final-source-acceptance.json; 6508 bytes; SHA256 1d468f364ce14608ca7b81e58191087bf13139db7246e115b1556d17b7454dfc), FINAL45 (.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root45-r8-2-final-acceptance-v1/acceptance.json; 11054 bytes; SHA256 bc9710892e4217a04eedc887c7d04208745f7d3bf558b47926f4f11135547154) and FINAL46 (.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root46-r8-2-final-acceptance-v1/acceptance.json; 8213 bytes; SHA256 8fc69656cb28d2aa097f7ae76de795c4efe228b64ff405f1322280fdd338fc0d). FINAL35 explicitly binds the Plan 30, 45 and 46 ancestry required by Plan 47.

Root captured the four-owner original before-image (.local/open-bitcoin-dev/phase167/plan13/root-after-final35-fresh47-v1/original/capture.json; 3179 bytes; SHA256 a774ca7cfa08cdb959316897f12a52e3d04770bde8d1c7e662ffa0b7a776bd60): three whole present bodies and one true observation-child absence. The separate common capture (.local/open-bitcoin-dev/phase167/plan13/root-after-final35-fresh47-v1/common/capture.json; 337535 bytes; SHA256 30e166cb17117ad1e7328f6a7999cbd7372b274556d1b4d70bf255c659fa6623) contains 297 source paths, 296 present and the same absence: accepted35's 284 paths plus 13 new causal inputs. It retains 201 native inputs and the separate Byteview supplement. Root authentication (.local/open-bitcoin-dev/phase167/plan13/root-after-final35-fresh47-v1/authentication.json; 1916 bytes; SHA256 7a9bb253a6287fc60f35be96db6f46dc4e985e944082161fb94aa859ac2b3468) and the current API reconciliation (.local/open-bitcoin-dev/phase167/plan13/root47-after-final35-reconciliation-v1/reconciliation.json; 2325 bytes; SHA256 e8b1fb84931c931d003baa9588e341bed78e4f3a9a01567662804397bbcb8d90) keep original and current frames distinct. They claim ordered read evidence, not an atomic snapshot.

The v6 helper keeps every literal Plan 47 command while allowing only the known passing node compile-fail diagnostics in the external API doc row. Its source is .local/open-bitcoin-dev/phase167/plan13/observation47-v5-doctest-diagnostic-repair-v1/runner.candidate.ts (19694 bytes; SHA256 dca36022df8c1fa59c08b975f5efae9b5b87f6fc44489008c9a5a015d07bff03); its independent source-inert review is .local/open-bitcoin-dev/phase167/plan13/independent-observation47-v6-runner-review-v1/review.json (3897 bytes; SHA256 84ffd7c5cbfe42e85561bdf8931916a407db4a5921574ef77c05701172f6593c). The actual source/build lease is .local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/actual47-initial-release-v2.json (10541 bytes; SHA256 a4e40dc66c9769d8d708f5f41c02cc68032ffe570ecf29ad9ff820525a1fa0a4). An earlier same-byte command-matrix reference-path preflight failed before effects and was replaced by this exact release; no original argv changed.

## Test-first source application and execution

The initial capture (.local/open-bitcoin-dev/phase167/plan13/observation47-execution-after-final35-v6-v1/initial/capture.json; 214837 bytes; SHA256 a7bcd56d43f58afa9a4852ab03d258c2f14aa59a79d6c9426a1d54d09ebbb28e) and fence (.local/open-bitcoin-dev/phase167/plan13/observation47-execution-after-final35-v6-v1/initial/fence.json; 1107 bytes; SHA256 b98368886748a6abd24156e3d439840e635ea5b25c4aec46b9e21c2adc098e29) closed before any source edit. The two test owners were staged first. The resulting Task 1 contract command compiled with native exit 101 and no tests discovered because the proposed observation types and methods were still absent. Root classified its 30 errors as an API compilation signal, not a behavioral RED or pass credit (.local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/task1-contract-signal-root-review.json; 2633 bytes; SHA256 30b94ba38b2a4c7e6d786a2c2644fd71d1a8fb9a2dbfbf40b5192b8f23bfa9ce). Only then were the two production bodies applied. Root's four-owner application review (.local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/four-owner-application-root-review.json; 3644 bytes; SHA256 ffbef15eb920d1fdd39fac65076f4e0dd9e1f82246890280855db328aeb1b316) binds those four exact proposal bodies, 293 protected source files and the 201-plus-one native inputs. Scoped Rustfmt check passed with no delta; no extra source owner or dependency was added.

The compiled Task 1 result (.local/open-bitcoin-dev/phase167/plan13/observation47-execution-after-final35-v6-v1/jobs/task1-green-v1/result.json; 468752 bytes; SHA256 20e45c0288cb3ff438e8809d638854be6b697ce185863f5859a85005c25be82f) and Task 2 result (.local/open-bitcoin-dev/phase167/plan13/observation47-execution-after-final35-v6-v1/jobs/task2-green-v1/result.json; 468753 bytes; SHA256 9dd6b616d1aee205f801d87933e352c64ecfc90d317013f248e23ccd5742ad50) each contain nine named passing observation tests, zero failed/ignored/unparsed cases and no diagnostics. All nine Task 2 required names are present. The final frame (.local/open-bitcoin-dev/phase167/plan13/observation47-execution-after-final35-v6-v1/final/capture.json; 214115 bytes; SHA256 97adee6dc0d532bbb63722ed6266ef09e0b183beceef45d4a793536725d9a1ba) has all 297 source bodies present, exactly four approved owner changes, 293 protected paths and unchanged 201-plus-one native inputs. Root's source-frame review is .local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/final-frame-root-review.json (2281 bytes; SHA256 bad8338ed24de4655263c6db0a67a8fcba09cc96579806ad6a316eeace43ed55).

The two tasks and seven original final rows all closed with native exit zero and review-required false. Counts below are named passing test executions; overlap across rows is not unique-test coverage.

| Sequence | Original key                   | Named passes | Failed | Ignored | Diagnostic headers |
| -------: | ------------------------------ | -----------: | -----: | ------: | -----------------: |
|        1 | phase167-47-task1              |            9 |      0 |       0 |                  0 |
|        2 | phase167-47-task2              |            9 |      0 |       0 |                  0 |
|        3 | phase167-47-final-format       |            0 |      0 |       0 |                  0 |
|        4 | phase167-47-final-observation  |            9 |      0 |       0 |                  0 |
|        5 | phase167-47-final-command      |           38 |      0 |       0 |                  0 |
|        6 | phase167-47-final-projection   |           17 |      0 |       0 |                  0 |
|        7 | phase167-47-final-external-api |           16 |      0 |       0 |        26 expected |
|        8 | phase167-47-final-build        |            0 |      0 |       0 |                  0 |
|        9 | phase167-47-final-clippy       |            0 |      0 |       0 |                  0 |

The external API row passed three ordinary node doc examples, including the new public observer example, plus 13 pinned compile-fail examples. Its 26 stderr diagnostic occurrences are 13 expected primary errors paired with 13 expected abort lines; no warning or extra error was accepted. All other rows have no diagnostic occurrences. Aggregate named passing executions are 98; none failed or were ignored. Root's reviewed seven-row record is .local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/final-seven-root-review.json (4179 bytes; SHA256 fd42005a300141d284534a8d8df8ea1cfad2100c5f9b559adb27c81295f9b48f). Its all-nine/inverse review is .local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/nine-and-inverse-root-review.json (5373 bytes; SHA256 5e7d8c920408264c4f5827c1161707fef9a3edee84286d5ecf40cd03f9fc55f3).

## Original four-owner inverse and sealed CORE

The scratch inverse result (.local/open-bitcoin-dev/phase167/plan13/observation47-execution-after-final35-v6-v1/inverse/result.json; 2044 bytes; SHA256 292ed56eca15edd60054289bbb9e27d2688bb1a937db53997c8a707829de0687) restores the three historical bodies and observation child absence, then reapplies all four approved bodies. Both strict BSD patch reverse/forward steps exited zero, with no offset or fuzz. The full 293 protected source bodies, native201 and separate supplement remain exact; live source was never patched.

The CORE (.local/open-bitcoin-dev/phase167/plan13/observation47-execution-after-final35-v6-v1/CORE-v1/CORE.json; 35316 bytes; SHA256 d82e964dfe04fffe0f03fbcf5ffb2359dc5100e861645e926d7651fc9f56e9c4) has status PLAN47_ALL_NINE_ACTUAL_CORE_SEALED_PENDING_INDEPENDENT_REVIEW, sourceAccepted false, nine bound job results and 98 named passing executions with zero failed/ignored. It binds the Root permission, accepted35 receipt, final frame, scratch inverse and Root final-seven review. Root's CORE-stage review (.local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/actual-core-root-stage-review.json; 2789 bytes; SHA256 0f580fd1d5358567014b5eab23c55ae3a92198699d17524ee12e903f88f421dd) confirms 499 current bodies but does not accept the source.

The first independent CORE reviewer release (.local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/independent-core-review-release.json; 6357 bytes; SHA256 dd0fc730c1502bcbffee935e3224b5c2f05416c010b25f193d6f74741ded45e6) stopped before output because the script expected serial-exit fields absent from the immutable Root review. The pre-effect HOLD (.local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/independent-core-pre-effect-schema-hold.json; 1216 bytes; SHA256 f8056f1187e54fb8867b1b509875e9089e52872b52d69c257999b26869d33987) records no review verdict, source edit or product command. The corrected V3 release (.local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/independent-core-review-release-v3.json; 7169 bytes; SHA256 eb0fea9e95d5b97451a1fbc75c308588aee179bd34eb5205e75945cb896f0ee4) produced the actual independent CORE review (.local/open-bitcoin-dev/phase167/plan13/independent-core-after-final35-v2-v1/review.json; 6001 bytes; SHA256 21611168860dcb131a84f95eb8ef5203ed0c9e917dd2b6edc1891150dc129388), status INDEPENDENT47_ACTUAL_NINE_CORE_REVIEW_PASS with zero findings. Root accepted that exact CORE and review for this Summary stage (.local/open-bitcoin-dev/phase167/plan13/root47-v6-selection-and-deployment-v1/actual-core-summary-stage-review.json; 2891 bytes; SHA256 2ff5e426bd663aab0cd17b19a39b4253e5e64cca46b926d62c1bb988e6cbcd95).

## Residual gates

The independent CORE review validated all nine raw jobs, exact current source, the four-owner scratch inverse, accepted prerequisites, reviewed sealer identities and source/fence/guard chronology. This canonical Summary still requires scoped document QA, independent document QA and Root FINAL47. The Plan 47 requirements contribution does not close global CVLD-01/02/03/04/06 or whole Phase 167. Plans 13/48/14/15/16, compact lifecycle C/D/E, the full source review before Plan 17, security/parity/docs work and full default verifier/coverage/Bazel/final audits remain. No commit or push is authorized by this Summary; stop before Phase 168.

## Self-Check: PASSED

The canonical document binds the four exact source owners, nine original command results, scratch inverse, sealed CORE, independent zero-finding CORE review and Root Summary-stage acceptance. Its 98 named passes overlap across commands; the earlier compiler-only exit 101 receives no behavioral credit. Independent document QA, Root FINAL47, global CVLD closure and whole Phase 167 verification remain open.
