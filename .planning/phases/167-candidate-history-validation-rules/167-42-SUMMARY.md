---
phase: 167-candidate-history-validation-rules
plan: "42"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: node-storage
tags: [rust, fjall, physical-inventory, work-bounds]
requires:
  - phase: 167-41
    provides: Accepted complete typed physical walker, original logical stamp, held root and cumulative work ledger
provides:
  - Changed-only whole-pair live observation governed by the existing nonrefundable work allowance
  - Genuine two-native-change availability and default-ledger physical churn exhaustion evidence
  - Exact terminal precharge accounting and complete source-bound verification
affects: [167-40, 167-39, 167-30, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Whole failed-pair discard, actual action-associated Changed attribution, successful-pair stage-local work proof]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/live.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/live/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/live/tests/refusals.rs
key-decisions:
  - Continue only actual Changed through the same complete attempt and original stamp, held root and external ledger.
  - Derive finite progress from the existing root visit reservation without another runtime attempt cap or budget peek.
  - Preserve exact injected actions and immediate refusal counts while selecting the actual successful pair independently of ordinal.
  - Keep the default-ledger physical churn proof distinct from the precharged terminal boundary control.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-23T08:17:01.066Z"
commits: false
duration: "1h 11m 3.905s from actual source-before capture through final command completion; independent review and document QA are separate"
completed: 2026-10-23
---

# Phase 167 Plan 42: Work-Bounded Whole-Pair Live Observation Summary

**Live physical observation now discards each safely changed pair and continues only while the original work allowance funds another complete capture and full recheck.**

## Final Source Files

Exactly THREE existing owners changed across three tasks. Their final source totals56,797bytes/1,499normal lines; the largest owner is616lines and both test files remain within620. All59 protected inputs remain exact, including copy.rs595, inspection.rs399, original40 opening/pressure and the entire original72-line305unit.

| Source                                                                                 | Bytes | Lines | SHA256                                                             |
| -------------------------------------------------------------------------------------- | ----: | ----: | ------------------------------------------------------------------ |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/live.rs`                |  8078 |   279 | `152d5245d44d5219b70217e548240a3ade4f72581ed843e513f95d666a1537a5` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/live/tests.rs`          | 24202 |   616 | `1b7081e115a3f68f3cd4732ebdfbd22c14f2a56ea96d7b025aa331ff29fbd64d` |
| `packages/open-bitcoin-node/src/storage/fjall_store/inspection/live/tests/refusals.rs` | 24517 |   604 | `d478d5267daa8b6ada6af5f3679ae6cb2ccfcfe46ca264e551e3ab00630b026f` |

## Bounded Implementation

Only the private live driver changes production policy: it loops over the SAME complete attempt_once after actual typed Changed. Ok and every other result return immediately through the original StorageError conversion. Each failed pair is discarded in full; no earlier inventory, partial walk, hash or baseline is reused. No independent attempt number, fresh stamp, generic retry framework, delay, writer pause, forced maintenance, new API, dependency or special40branch was added.

ONE original opaque stamp is acquired before filesystem observation. ONE read-only held root and ONE external work ledger span every attempt. Root/stamp validation still precedes and follows each attempt and final return; lost authority overrides Changed. No generation mutex spans I/O. The held anchor retains canonical path/dev/inode/directorytype/mode binding against original fd and fresh path. Only its dynamic directory nlink/length/mtime exclusions remain; every Inventory field and full equality still include those facts.

The protected walker retains full64KiB hashing, real EOF, opened-file/path checks and security-first classification. Safe directory metadata/link differences remain Changed. Mode/type, regular-file hardlink, root, permission, permanent-I/O, arithmetic and resource errors remain Refused. Strict offline original-error/no-retry and non-Unix single-attempt behavior remain exact.

Existing work limits remain8,192visits,1,073,741,824requested content bytes and8,192SEPARATE EOF probes, without reset or refund. Partial reads retain their complete requested charge. Per-capture512MiB/4,096entries/depth16/path4,096bytes stay unchanged. Every continued Changed has already spent the existing root visit. Therefore at most8,192funded Changed iterations plus ONE final denied-root-precharge start can occur:8,193actual starts is a derived bound, not a new runtime cap. Failed reserve does not increment. Existing root-anchor filesystem work stays outside the walker ledger; finite iteration does not bound syscall latency or guarantee scheduling or native quiescence.

The simplification pass retained this small Changed-only loop and the existing complete attempt, authority and work owners. Test-only attribution records actual actions and actual driver Changed attempts. Positive controls select the real successful attempt's own L0/L1/L2, footprints, completed hashing, EOF and full Eq. Exact intended action counts and ZERO/ONE-start Refused/offline boundaries remain; incidental positive attempt ordinals may vary.

## Actual Behavioral Controls

Task1 retargets the existing two-distinct originally populated native control. On the UNCHANGED accepted41driver, both real rotations, original stamp, all nine maps/catalog and both actual driver Changed results execute before the expected failing success assertion. Its actual receipt is0PASS/1FAIL/0ignored11.08seconds, natural exit101, SHA256284c6c56012e02d06e716916208aabf39edf8b7535c1e62dbdd629067cf6c0ce. No setup error or undiscriminating action supplies RED, and RED was not repeated.

Task2's exact bounded selection passes3cases/0failed/0ignored57.29seconds. Both final bounded and full-live selections subsequently prove two genuine native changes on attempts1/2 followed by a complete equal pair on attempt3. Each successful walk independently consumes77entries/67,172,839content bytes/55EOF reads; earlier268,678,670bytes remain charged in L0.

The complete live selection retains all23prior concerns, explicitly retargets only the two-native outcome and adds TWO distinct work-bound controls, yielding25meaningful cases. Seven final successful-pair occurrences across the bounded/full-live selections independently prove both walks' OWN footprint/ledger/completed hash/EOF deltas. No aggregate-only total supplies a full-pair proof.

| Final live positive                                               | Own entries/content/EOF for EACH successful walk |
| ----------------------------------------------------------------- | ------------------------------------------------ |
| Two distinct originally populated native changes                  | 77 /67,172,839 /55                               |
| One populated native rotation                                     | 75 /67,169,275 /53                               |
| Retained owned root directory                                     | 73 /67,167,281 /50                               |
| Root metadata churn                                               | 72 /67,167,281 /50                               |
| Safe descendant-directory link change, each classification branch | 74 /67,167,281 /50                               |

Real default-ledger churn uses owned root File::set_times at the existing DirectoryRead boundary. Each actual mtime change is compared by the complete walker; the hook constructs no Changed result. Both final selections observe15real Changed results then original requested-content Refused on actual start16,0completed captures and work(1,152visits,1,073,693,456requested bytes,799EOF). Remaining48,368bytes cannot fund the next65,536-byte request. Original stamp and all9maps/catalog remain valid; prior work is cumulative and never refunded.

A SEPARATE terminal-edge control precharges through existing reservations and invokes the actual private driver: exactly1start, original denied-root-work error, visits still8,192,0walker metadata-hook calls/hash/EOF/capture, no continuation. Root-anchor checks remain. This precharged control does not substitute for genuine default-ledger churn.

Safety/logical/root controls, actual PermissionDenied, opened-directory IsADirectory/OS21,19-byte partial progress with4,096requested charge, restored-metadata full hashing, descendant deletion, real sockets, exact path/per-source/arithmetic bounds and strict original errors pass. In-flight release/join precedes result assertions. The original40pressure and72-line305fixtures remain whole; no forced maintenance was introduced.

## Fresh Final Verification

All EIGHT exact final commands executed fresh on ONE all62-present freeze. Each retains its own62before/after actual copied bodies, giving992fresh final copies. All113overlapping test executions pass with0failed/ignored; they are not113unique tests. Empty auxiliary harnesses supply no discovery.

| Timing key                            |      Named passes | Test seconds | Full command wall seconds | Exit |
| ------------------------------------- | ----------------: | -----------: | ------------------------: | ---: |
| `phase167-42-final-work-bounded`      |                 3 |        57.33 |                   331.444 |    0 |
| `phase167-42-final-live`              |                25 |       166.55 |                   167.554 |    0 |
| `phase167-42-final-strict`            |                39 |       893.87 |                   894.486 |    0 |
| `phase167-42-final-canonical-genesis` |                29 |       577.16 |                   577.897 |    0 |
| `phase167-42-final-generation`        |                12 |        10.43 |                    11.138 |    0 |
| `phase167-42-final-seals`             |                 5 |       164.97 |                   165.334 |    0 |
| `phase167-42-final-format`            | check-only format |          n/a |                     2.238 |    0 |
| `phase167-42-final-clippy`            |       strict lint |          n/a |                    10.151 |    0 |

Workspace format retains --check. Node Clippy retains --all-targets/--all-features/-- -D warnings and reports9.80seconds separately from full command wall time. Pinned Bun1.3.9/Rust1.94.1, original physical TMPDIR/default target/cache and cooperative command-timings remain. Command intervals do not overlap and no first-party source write occurs during runtime jobs.

Actual source-before2026-10-23T06:50:54.672Z through final command completion08:01:58.577Z is4,263.905seconds, including layout hold and compiler/backtrace waits. Independent review/document QA follow separately. No per-task or metadata commit was made under commits:false.

## Immutable Evidence and Source Inverse

Complete index `.local/open-bitcoin-dev/phase167/plan42/execution/revision2/final-evidence-index.json` is1,662,941bytes/SHA25654ebdcef4686adc713655788e7ecdef5f352a9a38b115f16a781e9ccf8506321. All3,419references authenticate. Seventeen actual timed jobs contribute2,108command copies; the exact layout application contributes124SEPARATE copies, totaling2,232actual copies and ZEROabsence observations. Two historical external-address markers bind retained old copies rather than new live owners. All201native/lock inputs authenticate separately; no per-job engine capture is claimed.

Final freeze `execution/revision2/freeze/result.json` under the same plan42directory is537,080bytes/SHA256f2d43955aa97d5a4f58e562e7ca221f39c0fc5e55f878584028270b0f90ab313. All62final bodies and actual copies match;59nonowners remain exact. Trace authentication is45,795bytes/SHA256afa5ff10a0e3f194a7676f550be377c6c0740367035255f449a7e7b281f3383d.

Original42SOURCEinverse authority is fresh root-before/capture.json,523,067bytes/SHA256d5ebef6b358e530376d100007103c611dd11a5afe51bde5278f65fa81be9975a: THREE actual existing accepted41bodies plus59protected bodies, ZEROsource absences. Old17e working layout and older41 inverse never substitute for that authority.

Executor strict sandbox inverse/forward passes with BSD patch --batch -E -F0 -p1:62present final→62present original→62present final, restoring all3exact original bodies and preserving59protected states without offset/fuzz. Proof is53,801bytes/SHA256ca871f0ece267c9f59ebf719a3d4871f549136ab9f57fc24c7a1322889ccd497. Live source is not patched. Independent new inverse is40,331bytes/SHA256fac06a7a8fd8a49e05764fa8ea83aea53f2c4c7e70df441d974893fc9ee0bda3. Original305unit remains72lines/2,559bytes/SHA2564b89e729cfadd393be0aadf9e17414017bf58dfec856727d7e3c277a819027d6; this is preservation evidence, not actual305success.

## Deviations from Plan

[Rule 3 - Blocking source contract] Independent review held the new compact churn/terminal Trace blocks even though rustfmt passed. Root release72caed1fa2fc779a5ab1b55e4a5b2b28d0d3c4cc5b23e271a6f18c4e55725c06 authorizes the exact normal-layout correction: move the complete churn Trace proof into a cohesive private parent helper, retain every componentwise comparison/oracle and expand the terminal tuple/two-handle guard. Only TWO test bodies change; driver and other60inputs remain exact. Normal counts become279/616/604, without a fourth owner or assertion/scope reduction.

Old17e's four finals actually pass96overlapping executions, but source approval remains layout-held and four later commands never start. The nine old timed jobs/1,116copies, formatting setups, old source/inverse/hold and queue retirement stay immutable. Only the idle Bun queue and auxiliary monitor retire after the active canonical command naturally exits; no actual Cargo/test process is signaled. No old pass supplies new final credit. The exact application49b4f1c retains124actual before/after copies, then ALL8commands run fresh on f2d43955. This resolves the layout contract without protocol, allowance, deadline, action or assertion change; no planned scope is deferred.

The first new compiler completes naturally in4m33s. Strict characterization takes893.87seconds and its parent passes expected child signal6/scratch-retained/source-preserved controls. Read-only sampling observes775/775test-thread samples in the expected panic backtrace's gimli→ReadDir→readdir→getdirentries64. The exact directory/descriptor and underlying kernel cause are UNPROVED. Sample/observationa47dec68 and30second liveness remain retained. No source/target/cache/flags change, retry, suppressed check or ETA-based intervention supplies a pass.

## Independent Core Review and Document QA

The WHOLE immutable report was read and authenticated: `.local/open-bitcoin-dev/phase167/plan42/independent-review/revision2/167-42-CORE-REVIEW.md`,16,844bytes/SHA2569e64384bdbba7ac83b4ab7e8775a72e04f10d0708c6dfbb52b9604e22dd56bd9. It is CLEAN with0critical/warning/info findings; the layout gate is closed on exact revision2source. It independently authenticates all3,419references, complete eight-command matrix, actual per-walk/default-churn/terminal arithmetic and a distinct original-source inverse. Core index1df27ca9 records SummaryQA pending with NO Summary dependency.

Scoped document QA uses installed GFM/frontmatter mdformat plugins, check-before-write on this Summary only, exact baseline/lifecycle/commits/requirements fields, all3source hashes and GSD verify-summary --check-count3. Initial/final Summary copies and actual receipts are retained under execution/revision2/document-QA/ and document-QA-revision2/. The second version records the actual Summary creation timestamp in frontmatter. A separate independent SummaryQA/final packet must bind the actual Summary hash and immutable CORE hash before root acceptance. No circular dependency is introduced.

Repo AGENTS, Bright Builds sidecar/overrides and architecture/code-shape/testing/verification/Rust standards informed execution. Both active lesson files were read completely:9,838bytes/3,280conservative tokens. No lesson/archive/shared STATE/ROADMAP/requirements/guards/manifest/canonical REVIEW/Git write occurred. No unresolved source stub or new unmodeled production trust surface was found. No external user setup is required.

## Remaining Gates and Scope

The requirements-completed array copies all five plan IDs verbatim for this bounded contribution. CVLD-01, CVLD-02, CVLD-03, CVLD-04 and CVLD-06 remain globally OPEN. Root alone accepts42 after separate document gates and releases unchanged40ALL7fresh62→39ALL9includingoriginal305/306→30ALL17, with their own original inverse authorities. Prior failures stay attributable. This contribution supplies no actual40/305/safe-reopen, native-quiescence, latency/scheduling, differential or whole-phase theorem.

Later35FIRST/13/product/corpora/formal C/D/E using actual35interfaces (D\<5tasks/E→17), canonical SOURCE review BEFORE17guards including123ownership,18security/native/mutants,19parity/README/manifest,25writing-for-agents and20ENTIREdefaultnative/freshpureCorecoverage/Bazel/source/security/goal/lifecycle/normal-hookmaincommit/push remain. Root owns shared metadata and Git. Phase168 stays unstarted.

## Self-Check: PASSED

All three final source files and required actual evidence exist and match the immutable62-input freeze. All eight exact fresh final commands exited0; strict source inverse/forward reproduces three original existing bodies/59protected and all62final bodies. Immutable independent CORE is CLEAN0. No commits are expected under commits:false. Separate Summary hash binding and root acceptance remain pending.
