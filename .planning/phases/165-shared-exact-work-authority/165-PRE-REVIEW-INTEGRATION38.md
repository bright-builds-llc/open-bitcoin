---
phase: 165-shared-exact-work-authority
reviewed: "2026-10-17T23:48:57Z"
depth: standard
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
review_scope: plan38-tasks1-2-source-before-identities
files_reviewed: 7
files_reviewed_list:
  - scripts/check-phase155-filter-index.ts
  - scripts/check-phase155-filter-index.test.ts
  - scripts/check-phase156-prune-coordination.ts
  - scripts/check-phase156-prune-coordination/contracts.ts
  - scripts/check-phase156-prune-coordination.test.ts
  - scripts/check-phase157-index-catch-up/contracts.ts
  - scripts/check-phase157-index-catch-up.test.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Plan38 Source Review Before Identity Activation

## Result and scope

**CLEAN: no open findings in the seven final Tasks1–2 owners.** Two material evidence gaps identified during this independent review were demonstrated with actual failing controls and repaired before this result. This report covers source and test correctness, actual Rust correspondence, exact complete-source inverses and authenticated command evidence. Root still owns source acceptance and Task3 identity activation. This is not the canonical whole-phase review, native verification or Phase165 completion.

The checked Plan38 SHA256 is `ff27041bc9bc512d08dd2950c287e9fd352a751966f1d519859fb296438eca45`. Repository AGENTS.md, AGENTS.bright-builds.md, standards/index.md, architecture/code-shape/testing/verification and TypeScript/JavaScript/Rust guidance informed this review. standards-overrides.md contains no applicable exception. Both active lesson files were loaded completely: 7188bytes and2397 estimated tokens, with no archives. Neither project skill directory exists. The review stays within the existing GSD lifecycle.

## Authenticated complete sources

The original344-file packet has SHA256 `a6f23987c5d1ea36909f8f49866f17bd086199014509171d63524c4252e229fd`. The seven authentic before bodies in `.local/open-bitcoin-dev/phase165-plan38-before.json`, SHA256 `2c552cdd345240c35e19f980e27e24051c5ab6f376f7e4cd535c6c35ce5eb03a`, match those original bodies and hashes exactly.

Final handoff `.local/open-bitcoin-dev/phase165-plan38-source-handoff-v2.json` is714352bytes, SHA256 `2fe263e829fab9161c4f880c6b7cd61a69d0538cc389faed2c5c3785f3248dff`. Every complete final body was checked against its actual regular canonical working file, raw bytes, length and SHA256. All seven ordered full-file inverses were independently applied in memory and reconstructed the complete authentic originals exactly. Each of the three test-file inverses consists only of removing inserted lines; all original test bodies, loops and helper bytes remain intact.

| Final owner        | Physical lines | Full raw SHA256                                                  |
| ------------------ | -------------- | ---------------------------------------------------------------- |
| Phase155 checker   | 418            | 9f38bd65a10a64cf987926bb639876ea80945ec23db637cce0cd7707f1ffd803 |
| Phase155 test      | 528            | 1f39b34a522ba738a50315dbcfcf8bef651790e134223a48aa2bd0c3f90c5887 |
| Phase156 checker   | 331            | f739fc72481612818c0edd93605538f778f5cd6f9277995babe9386cbc2b0ede |
| Phase156 contracts | 138            | 4b1f1eccec2b8b63c117a3b1d64edfe5d14a733262ea3e6f7a51d56bc6617a40 |
| Phase156 test      | 512            | 511938a2729d042b24c62828f13fcf69ca1ca88d47a513500ffc5f0f164135b6 |
| Phase157 contracts | 164            | 2bb54afe25a7cacada808fbd26c1f797a99c9f6e54424d6086f3b750d5ea5211 |
| Phase157 test      | 590            | cfc509de9b0c17f461e11808c819018f99d65c6bf57a17d6589beb9c2d3a6921 |

The29560-byte complete diff has SHA256 `a1c8879cd779937172153b983aed58faf03b9417c9ea0f358262301486df14fc` and was read in full. The first complete handoff remains byte-exact at `6dbb1fc8b8758d4cd473afee36b1761acc6538ebf98ecba118b6843c61025898`. Only the two157 owners differ from that handoff; their separate exact inverses reconstruct its complete earlier bodies. Its other five owners, six read-only Rust helper records, three held activation records, command artifacts and earlier results retain their own generation.

The held activation files match the before packet and current working bytes: old164 test-modules.ts at `4ced68e96f261bdb3e29f19f451dd22785587665d20686614873818c5aa3bcf4`, current165 entry at `b8ffdd7d25b10fd251b92e18af5eb99d3358b4d4b98519d83b36a1f59eb5b325`, and current165 contracts at `c6bbdb06f393d41f00fe15b52f81026802aa9a5bc2bdd4345ab91b2f8f753ef4`. No source digest, ordinary source row or input pin is approved by this source-only report.

## Source correspondence and resolved review findings

### Validated recovery and genuine prune evidence

Phase155 checker:85 resolves the actual recovery constructor body. It requires the current undo accessor, P2SH/locktime/sequence flags, the two-input undo assertion and exact spent-script assertion, followed by staged commit, committed position, retained undo and historical filter generation. Existing recovery/refusal/startup/publication obligations remain. Its five new controls start from a clean fixture, require a unique changed source anchor and check the validated-history diagnostic.

Both Phase156 selectors now name the actual executable genuine fixture test. Checker:170 follows `DenseHistory::seed(&store)` through measured threshold and `crate::assemble_durable_runtime`. The seed body requires normal Regtest maturity100, genuine empty rebind,1002 accepted blocks including root, accepted height/hash assertions, actual admission facts and root-inclusive derived undo. Its real Always flush, wrote-coins evidence, independent encoded body/undo measurements and consuming same-state configured constructor remain required. The outer test and concrete helpers retain protected Periodic/Always turns, disable/reenable ownership, exactly714 paired deletions, cache/undo cleanup, complete handle/store close and actual disabled reopen/refused reacquisition.

The first review finding was that the new concrete rows required calls to `assert_payloads` while leaving its implementation unchecked. An assert-true stub could therefore remove per-height body/undo comparisons without losing a required needle. Six independently changed controls demonstrated this gap from a clean baseline:0pass/6fail, with the guard incorrectly accepting each weakened helper. The final checker:198 now resolves the actual helper and requires active-chain enumeration and both complete executable `assert_eq!` operand comparisons: stored block versus retained expected block, and stored undo versus actual derived expected undo. Both retention predicates are bound. Stubbed helper, missing body/undo read, inverted retention and substituted expected values now refuse with the intended concrete-evidence diagnostic.

### Typed acceptance, configured startup and shutdown assertions

Phase157 contracts:88 follow actual `commit_prepared_connect`: live origin and acceptance availability, fallible typed stage validation, absorb, sealed history, lineage and BASIC acceptance, then publication/persistence failures that retain BASIC failure and return the mapped error. Accepted FactLimit/InvalidFacts errors remain explicit.

The configured caller forwards the same store, managed state, config, mode and activation arguments through preparation and finishing. Separate actual helper bodies bind the selected indexing configuration, initialized network and header source. Finishing creates the authoritative handle before fallible BASIC owner initialization and first turn, then returns that same network and configuration. The existing ordinary versus explicitly test-owned fixture qualification remains intact.

Shutdown evidence follows `history.open_genuine` through its real open helper, managed acceptance loop, saved bodies, derived undo, physical Always flush and same-state consuming configured source. It keeps the worker spawn, elapsed/shutdown events, actual shutdown join call, coins fence, retry/checkpoint settlement, no-clean-marker assertion and retained typed error outcome.

The second review finding was that bare `matches!` needles could survive removal of three critical `assert!` wrappers. Three controls each removed exactly one actual wrapper/message while preserving its complete inner predicate as an unused expression. Actual RED was0pass/3fail with received failures`[]`, demonstrating lost retained-BASIC, joined-shutdown and outer typed-error assertions. The final contracts:134 require executable `assert!(matches!(...))` structure and the real downcast/type operands. All three targeted controls now pass; the generic concrete-evidence mutation also passes with the actual raw-source-compatible prefix.

The critical-helper pass inspected these directly followed bodies and assertions. It did not expand into recursive qualification of generic library primitives or unrelated fixture helpers. Existing parsers mask comments/literals before ordered executable checks; names or comments alone cannot replace the new required helper bodies.

## Test preservation and simplification

The final files preserve all original test bytes and add49 independent cases:5 for155,25 for156 and19 for157. The expanded existing157 contract/evidence loops contribute60 further generated cases. New controls prove a clean baseline and changed unique source before asserting the intended diagnostic; they do not substitute missing-file or parse failures for semantic refusals.

The implementation reuses literal contract rows, existing lexical helpers and fixed loaders. All newly followed paths already belonged to those loaders. There is no new module, dependency, discovery expansion, general parser, Rust mutation or assertion deletion. All seven files remain readable and below628 physical lines. No further simplification is needed for this bounded repair.

## Authenticated execution evidence

The reviewer independently authenticated all13 receipt-artifact bodies and35 distinct raw logs, including the complete final six-command run. These are executor-run pinned Bun1.3.9 results; the reviewer did not rerun Bun or Rust tests.

| Final command               | Actual result                | Full raw log SHA256                                              |
| --------------------------- | ---------------------------- | ---------------------------------------------------------------- |
| Phase155 complete test file | 161pass,0fail,344assertions  | 6cc14a0af9a7710f06dac95e4be2dcbe893cbf8f161ba5d8eb67871fe73d30ba |
| Phase155 CLI                | exit0                        | 7e3bf89d504623f4251af0954c1cd1a50679b7a6d614eeaecd8e9a156ffd8dcb |
| Phase156 complete test file | 152pass,0fail,339assertions  | c69df4ace3ea18d9db45be80a9d16e9f2004919dd18aed29fa2bf81d37590c10 |
| Phase156 CLI                | exit0                        | 8b926508079349426a02ad7d67bb79c760e566621686bbe764f0e104c7151e95 |
| Phase157 complete test file | 755pass,0fail,2146assertions | 392745898879b1db178e52da589cae6254693e4e569df8ae5b01779c070a8513 |
| Phase157 CLI                | exit0                        | ded3d26ecaba3627ca29455f35b9370822caeb1f93a166e1b9a7eae8e439b55d |

The final serial run totals1068passing tests and2829assertions. Its receipt SHA256 is `a0d0de9a824d3cac7fd512d361072e15aca5b72f4a55cd225993f5f4b6f8c659`. The149-entry source-stage raw union, SHA256 `c7172c39fc346a83f4aa4501438aa6cb0fad33e67502ba1f2b77c72fb75b3201`, remained unchanged around the six commands; its closing record has SHA256 `09a62b887a63c43f3729f3288ca0b2ac733c0b14a6a555126ce079d06cea92f2`. The reviewer independently rechecked all149 present/absent/canonical/byte/hash records against current files. This bounded read union is not the final native/compiler closure.

Retained failures remain failures: the first full157 attempt had751pass/1fail from a new ambiguous forwarding mutation anchor; the intermediate assertion revision had754pass/1fail from a raw-versus-masked `expect_err()` generic mutation anchor. They are test-setup failures, not semantic RED. Each was narrowed to the actual unique source prefix, followed by focused checks and the complete fresh final six-command run. The nine demonstrated review-gap refusals are separate genuine semantic RED records. The original full1065-test GREEN handoff remains dated evidence for its own generation.

## Remaining gates

Root retains independent source acceptance, inert ordinary-row/input-pin proposal review, actual stale-pin refusal, activation and the rest of Plan38 Task3 verification/complete raw closure. Plan39 remains outstanding. Both earlier failed native attempts stay failed evidence. Final14 still requires the entire fresh default verifier including Rust, coverage and Bazel, final documentation/LOC freshness, canonical whole-phase code review, security/formal lifecycle acceptance and normal-hook commit/push. No Phase166 work is authorized by this report.

This report is the only file written by the reviewer. No source, test, activation, metadata or Git operation was performed, and no Bun, Cargo, Bazel or native verification command was run by the reviewer.
