---
phase: 165-shared-exact-work-authority
reviewed: "2026-10-17T13:08:30Z"
generated_by: gsd-code-reviewer
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
lifecycle_mode: yolo
depth: deep
scope: Plan29-exact-literal-relocation-before-Task2
files_reviewed: 3
files_reviewed_list:
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase165-shared-exact-work.test.ts
  - scripts/check-phase165-shared-exact-work/test-support.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 165 Plan 29: Independent Exact Relocation Review

**CLEAN for Task 1's exact three-file relocation and proof boundary.** All 64 original downstream controls retain their literal values, order, consumer loop and assertions. No actionable source or proof-integrity issue was found. Task 2 remains unexecuted and requires root release after this review.

## Authentic Before and Reviewed Packet

The actual checked Plan 29 has SHA-256 `4cf40fa4f86a20df17fb4ef67d491f817b0bde4317b7337a0e52e05a523fe898`. The parent-owned authentic capture `.local/open-bitcoin-dev/phase165-p29-root-before.json` hashes to `c5283dd7613a7f3e915adbcf02c8220abd769170dc68578b287042e0ff2b2929`. It preserves all three owner bodies plus the current entry.

The review packet `.local/open-bitcoin-dev/phase165-plan29-review.json` hashes to `e7e6d329baa938b638693d1b11d6d2b18c849aab1e2ff9256059f38231d9dfa3`. Every artifact hash and actual current source was independently authenticated. The executor's before sources and entry were compared byte-for-byte against the parent capture, not against later reconstructed after sources.

Repo instructions, Bright Builds sidecar, placeholder-only overrides, relevant code-shape/testing/verification/TypeScript guidance and the completely loaded active lesson set informed review. The lessons remain 7,188 bytes/2,397 conservative estimated tokens, with no archives or project skills loaded. Review is part of the active assigned GSD workflow.

Only this report was written. The reviewer ran no Bun, project imports, tests, Cargo, builds, source/shared-state/Git edits or pin updates. Independent calculations used read-only Python byte/JSON inspection. Task 1 Bun execution remains attributed to the executor.

## Exact Source Move and Complete Inverse

The sole contracts change removes the complete `export const DOWNSTREAM_SOURCE_MUTATIONS` declaration, retaining the comment and every other byte. The sole support change appends that exact declaration immediately after the entire existing support file. The main file changes only the binding lists of its two existing imports: add the table to the support import and remove it from the contracts import.

Independent whole-source comparisons prove both exact forward transformations and complete inverse recovery of all three authentic before bodies and hashes. The main inverse covers every test registration, loop, anchor check, positive baseline, assertion, owner/rule diagnostic and remaining byte. Original support imports, copiedInputs function and CLI_MODULES values remain exact. No new import declaration, module, helper logic or executable data is introduced.

| Owner                                                      | Before lines/SHA-256                                                     | After lines/SHA-256                                                      |
| ---------------------------------------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------ |
| `scripts/check-phase165-shared-exact-work/contracts.ts`    | 618 / `502dc79ab6b5a7264aaf4dbaf62b534982a12195af23f46f99d77360022cded4` | 552 / `f0a1d981afbd85c58b23d1845dff2408dcf12eb6b7c62bf9e139e7d2c1bb5fc6` |
| `scripts/check-phase165-shared-exact-work.test.ts`         | 627 / `d1f595f02d18a630a769135a593ced452c63d0f43608d674433c66c3ce227037` | 627 / `c6170a3f239a2c95b957b454434c4a745a0c85325f41dacde65fde637102e248` |
| `scripts/check-phase165-shared-exact-work/test-support.ts` | 23 / `19cfa9b55b5a46be045fa6a7d6a24e79ffdc558290ed5a7227b5d512c04e6f6a`  | 89 / `f554a59e279a901ee6b5ef975dc80c2a0628d512e6b8539378fc5e0bd242c650`  |

All three remain within 628 physical lines without compaction, padding or exception. This creates actual contracts capacity; it does not alter Plan 24's permitted owners or authorize later support-table changes.

## Literal Parser and Authentic Value Oracle

The complete moved declaration is 13,214 bytes/66 lines, SHA-256 `a0ff4f7db2c790d988d92d810bb9d1e61c9c2ca8be128e693e78de131a3f75d8`. It occurs once in the authentic contracts source and is byte-identical in support.

The readable ignored harness `phase165-plan29-extraction-proof.ts` is a real before/after oracle. In capture mode it first requires all original owner bytes to match the parent capture, imports the original contracts table in that fresh process, and compares its imported values against a separate literal parser over the before declaration. It persists these before values, AST shape, full sources, arrays, original support export values and every other contracts export.

The independent parser is deliberately bounded rather than presented as a general Bun AST API. It accepts only the exact outer array grammar and 64 inner rows, each with four double-quoted JSON string tokens, exact punctuation and no executable suffix. JSON.parse validates each complete string token. Expressions, identifiers, calls, comments, spread elements, extra fields, missing rows or executable suffixes fail punctuation/shape checks. Bun.Transpiler additionally checks TypeScript syntax; the parser never evaluates the moved declaration to manufacture expectations.

After the move, a fresh support import is compared with the authenticated before imported values, not with values newly derived solely from the after source. It also compares the complete declaration and separately parsed AST with their before forms, checks all other contracts exports and support values, and proves full three-file reversal. Complete byte equality independently rules out hidden neighboring edits or weakened helper/loop code.

Independent reviewer decoding of the parent-authenticated original declaration reproduces all 64 four-string rows and both exact digests:

| Proof                                                  | SHA-256                                                            |
| ------------------------------------------------------ | ------------------------------------------------------------------ |
| Ordered imported/parsed values                         | `bc7bdfb0698b69ef52ff205182464f0a0271ec0415dae57b0fd26efbc7508b9b` |
| Independent nested ArrayExpression/StringLiteral shape | `499437537747a16bc189e94e424d5ac239732085b98104253fcd18a794170a37` |

The existing downstream loop still selects the real owning region, requires an unchanged checkPhase165 baseline and an actual before anchor, proves changed bytes and requires that file's work-owner finding. Its test prefix still generates 64 mutations plus the existing unchanged-source positive case. Actual execution of those 65 matched cases is pending Task 2.

## Arrays, Entry and Read Boundary

The ordered SOURCE_CONTRACTS 135/TEST_OWNERS 71/INPUT_FILES 264 arrays are identical before and after. The complete current entry remains `b37cc5399386eefcf0a749075c5d48b5d07cb030539a20760768c0942d7cf402`, byte-identical to both the parent capture and executor's before entry. Its ordered input digest remains `967b67cdf5a41019a931d9016b32dfc3c42dff8b07713cdab2d8abee48b1868b`. No source/test/input row or pin is renewed.

The reviewed collector preserves the complete existing fixed checker input lists, actual optional-loader outputs, the actual factory liveFiles initializer and observed dynamic 168-entry corpus, static test/checker modules, CLI closure, factory entry closures and explicit critical Rust dependencies. Its exact differences from the completed 28 collector are the 29 artifact names/modes, focused-scope label, explicit 36-module expectation and stronger closing comparisons for CLI/entry closures/factory initializer/corpus bytes. The proof driver and collector themselves are explicitly frozen. No prior incomplete 310-entry map is used as acceptance evidence; its reference supplies only a historical additions listing.

The post-move union contains 491 entries, 489 present and the same 2 permitted absences. Independent reviewer rehash at 2026-10-17T13:08:30.805257Z verified every current canonical path, presence state, byte length and raw SHA against the final pre-Task2 freeze. Comparing actual before/after freezes shows exactly three changed entries, the declared source owners; the path set and every other entry are exact.

All fixed lists, 36 static modules, 14 CLI modules, factory entry closures, live initializer digest and observed corpus facts remain identical before and after. The support module remains in the test/static/raw union and outside fixed-input, corpus, liveFiles and CLI lists. Its existing literal CLI_MODULES still names the same 14 files. Moving a table into that already imported test module adds no dependency edge.

The current dynamic corpus has 168 entries/4,406,607 bytes, SHA `8ef29ccfca26f5acc52c34627b8269a7182a4c6e34cb5d70a96f8e28b0e6234b`, unchanged across this relocation. These scoped reads are not a claim to the entire Cargo compiler dependency closure.

## Actual Task 1 Execution Attribution

The executor reports the following actual serial command-local pinned Bun 1.3.9 invocations, each exit 0, with observed tool stdout and no separate saved stdout log:

1. `bun .local/open-bitcoin-dev/phase165-plan29-extraction-proof.ts capture`
1. `bun .local/open-bitcoin-dev/phase165-plan29-freeze.ts before`
1. Exact three-source patch.
1. `bun .local/open-bitcoin-dev/phase165-plan29-extraction-proof.ts`
1. `bun .local/open-bitcoin-dev/phase165-plan29-freeze.ts`

The inspected persisted outputs establish the before capture at 2026-10-17T13:05:24.163Z, before raw freeze at 13:05:41.939Z, after proof at 13:05:58.501Z and final pre-command freeze at 13:05:58.601Z. The proof result records passed=true, 64 rows, exact inverse, caps 552/627/89 and unchanged arrays 135/71/264. This is actual extraction/import proof, not Task 2 test execution.

| Artifact under `.local/open-bitcoin-dev/` | Complete SHA-256                                                   |
| ----------------------------------------- | ------------------------------------------------------------------ |
| `phase165-plan29-before.json`             | `8f6ea77e33da78fba78d5b95e1b1ec935a65a4ad1c356263b993cb2af3257b7a` |
| `phase165-plan29-proof-result.json`       | `40db1b714adf39425a751abd13ee0bb77386717bd68ce8f4fd79bf279d718a89` |
| `phase165-plan29-before-freeze.json`      | `1c68f1c82485d6a2a2e265028456d0f5a7ef8820096ee659a2a7d5418d304920` |
| `phase165-plan29-full-freeze.json`        | `94e9c1d61860fce596bf19c74a2b8aebf51ab92fb3a17f3d1053f4d4bffb027d` |
| `phase165-plan29-extraction-proof.ts`     | `2e4015e91b9271c9234c696ce207c928aa0bfeda5a5b78dc1995828bbaca8f9a` |
| `phase165-plan29-freeze.ts`               | `e8abef5a90f8901c182469e5be024ea85aa7c49983378118e5b951713ba47e37` |

Root separately supplied `phase165-plan29-root-source-proof.json`, SHA `3062ef66fb9365818c28c8b464df2dad1bb467782487e3778b6ed7534467a9f7`. The review conclusions above were independently recomputed and do not rely solely on that assertion.

## Release Scope and Remaining Gates

**Task 1 is CLEAN; the exact frozen move may proceed to Task 2 after root's independent release.** No additional source change or pin renewal is approved.

Require the actual 65 matched cases to pass: 64 unchanged controls plus 1 existing baseline, with no ignored matching case; filtered unrelated cases are not executed evidence. Both live 164/165 checkers, quick 158/162 checks, managed/whitespace/caps, repeated before/after extraction/import proof and unchanged full 491-entry closure remain required. Any source drift or real failure invalidates this frozen approval and returns to the parent.

The checked plan intentionally does not duplicate full 164/165 mutation matrices for this exact data-only move. Existing full 28 receipts retain their own original source scope; they do not qualify new bytes. Plan 24 must still run its full consumer/Rust/source matrices, and Plan 14/root retain fresh default native/coverage/Bazel/benchmark, final code/security/formal/lifecycle and normal-hook Git gates. No Plan 29 Summary completion, Plan 24 readiness, runtime acceptance, ordinary/canonical qualification, historical renewal or future-phase delivery is claimed here.

_Reviewer: gsd-code-reviewer; independent exact-data source review before Task 2._
