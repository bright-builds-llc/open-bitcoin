---
phase: 165-shared-exact-work-authority
plan: "36"
reviewed: 2026-10-17T22:53:44Z
depth: deep
files_reviewed: 9
files_reviewed_list:
  - scripts/check-phase73-uat-verification/constants.ts
  - scripts/check-phase73-uat-verification.test.ts
  - scripts/check-phase75-soak-runner.ts
  - scripts/check-phase75-soak-runner.test.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
  - scripts/check-phase127-authoritative-network-state-unification.ts
  - scripts/check-phase127-authoritative-network-state-unification.test.ts
  - scripts/check-phase127-authoritative-network-state-unification/production-proof-tests.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
scope: task1-task2-source-before-ownership-identities-and-inventory-pin
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
phase_acceptance: false
native_qualification: false
---

# Plan 36 independent source review

CLEAN for the exact nine current source bodies below. No actionable correctness, security, integration or test-reliability finding was identified. All reviewed files meet quality standards within this bounded repair scope.

This is source acceptance evidence before ordinary ownership identities or inventory-pin derivation. This reviewer derived no ordinary ownership identity, changed no source or activation owner, and ran no Bun tests, Cargo, Bazel, full native verification or heavy matrices. The review authenticates the executor's actual receipts separately from independent source inspection. Task 3 activation, generation closure and whole-phase acceptance remain later gates.

## Guidance and authenticated inputs

The review follows AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md and the code-shape, testing, verification and TypeScript/JavaScript standards. Material rules are actual current-source evidence, independent AAA regressions, readable physical file bounds, pinned Bun, immutable historical attribution and the final native verification contract. No project skill directories or applicable local override were found. Both active lesson files were loaded completely: 5,230 global bytes plus 1,958 repository bytes, totaling 7,188 bytes and 2,397 conservative estimated tokens. No archives were loaded.

| Input                                      | SHA-256                                                            |
| ------------------------------------------ | ------------------------------------------------------------------ |
| Checked 165-36-PLAN.md                     | `3040d19e226e1a77038c7f97978b7ef280cbc4f4aa733d5c9f08413f69f36d90` |
| Complete original 344-entry before receipt | `a6f23987c5d1ea36909f8f49866f17bd086199014509171d63524c4252e229fd` |
| phase165-plan36-before.json                | `cd640952c84501195945c889eac1761106310a479909fc2f1dc804d49dc7fd49` |
| Final Task 1/2 complete source proposal    | `eae9303ad85ac00dc2de22a86b8d0bf7d3c9cde541c81d08f1d58d99f80c9b45` |
| Complete Task 1/2 diff                     | `9e10529a47e7e9e209f2c57d3ed5481118553c03d077a5ed2d4878dcb4aa4ae8` |

All eight embedded before bodies match the authentic 344-entry receipt and their own full byte counts and raw hashes. The ninth file has an authenticated absent-before entry. All nine live complete source bodies match the final proposal, their canonical paths, byte counts and raw hashes:

| Source                                                                                   | Lines | Full-file SHA-256                                                  |
| ---------------------------------------------------------------------------------------- | ----: | ------------------------------------------------------------------ |
| scripts/check-phase73-uat-verification/constants.ts                                      |   255 | `c439208c788a99d2e0b038e0683a227406fee350c0cf13e50ca9863ab104684c` |
| scripts/check-phase73-uat-verification.test.ts                                           |   568 | `ceccf28ae62aaf64fb9187e085345f76e21e7f6a4d86d1a3f3ead3163524d23c` |
| scripts/check-phase75-soak-runner.ts                                                     |   390 | `c43ea0d1ea2b1e54d2e2b9f5b53af0871dbaf0d01ab463302ca33504aa71f890` |
| scripts/check-phase75-soak-runner.test.ts                                                |   522 | `db954c5f7cd8841331a16003b0c82d61c9be159f2aebe6a8261ee85bbb252cb1` |
| scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts                       |   332 | `780c60a7b3aad52af73a7011c17afe5319eb06ffcf18f14af3b30b9c6bb98761` |
| scripts/check-phase123-runtime-timing-evidence-integrity.test.ts                         |   627 | `e043a206cb175bdc5e4251050d7eccbfb9c5d51b36a87c1e86241e6d939fcb6a` |
| scripts/check-phase127-authoritative-network-state-unification.ts                        |   566 | `abaed07a9863c66b8588ebf33494e303e396265d39017fd780c140d2e45f4c92` |
| scripts/check-phase127-authoritative-network-state-unification.test.ts                   |   619 | `1f494d8813e2145beb15c5b55f438dddfe7208e7bcafc802d9b0169b3a495aef` |
| scripts/check-phase127-authoritative-network-state-unification/production-proof-tests.ts |    78 | `4bce72cde7265b9ebf851a407b40d0b9529baa99e02b17759a4b858eaec19c7c` |

Every file stays within 628 physical lines. Ordered, nonoverlapping inverse hunks were independently applied in memory to the actual complete current strings, checking every current span. Each restored string equals the complete authentic before body and its full raw hash. The new file's inverse authenticates its exact current raw hash and restores its recorded absence. This is whole-source review evidence, without modifying historical adapters or recorded historical identities.

## Current semantics and regression preservation

Phase 73 changes one required test-name needle and its independent fixture spelling. The actual block_requests.rs test at line 9 requires best-header priority plus accepted-side eligibility. Its assertions preserve request order, per-peer/total bounds, exclusions for active/local/in-flight/unvalidated blocks, retained in-flight state and explicitly unearned fixture work. The two new cases remove the current name or retain only the obsolete best-chain-only name; each checks a unique changed fixture anchor and the exact intended missing-coverage diagnostic. All 17 original test and helper bodies remain exact apart from the one independent fixture needle.

Phase 75 changes only the runtime-guide durable-ledger sentence and its independent fixture. Runtime-guide.md:2706 contains the escaped opening angle in `runs/\<run_id>/events.jsonl`; operator-observability.md:149 remains unescaped. No global normalization or alternative-spelling allowance was introduced. Six independent cases remove each sentence, substitute the opposite spelling or replace events.jsonl with report.json. Every mutation checks a unique changed anchor and the specific missing-text diagnostic. All 21 original tests and helpers remain exact apart from the one runtime-guide fixture needle; all ordinary-refusal, projection, redaction and default-verification guards remain intact.

Phase 123 keeps the existing recovered-state initialization prefix and follows the actual configured caller in open_runtime.rs:88. It requires preparation before consensus-parameter derivation and finishing, exactly one preparation call with all seven ordered caller arguments, and exactly one finish call with the actual network result. The owning prepare_initialized_network helper at line 121 retains the exact seven constructor arguments, selected BASIC startup policy, local peer configuration, configuration recording, saved-header seeding and network return. Existing compatibility defaults, daemon/fixture activation, transport, write, timing and projection guards remain. Comment/literal masking is provided by the existing rustFunction/rustCallArguments scanner. Twelve new mutation rows target unique current Rust anchors and change bytes; existing initialization and comment regressions remain. Reversing only those twelve added rows and the synthetic helper-flow fixture entry recovers the entire original 123 test source.

Phase 127 removes one obsolete inline-pattern assumption and requires the exact executable matches_block helper's Block variant and expected header hash. Both served and mixed-block assertions must occur inside the owning production composition function. Comments, literals, a disconnected function and an unconditional or wrong-hash helper cannot substitute for these checks. All original shared-authority, durable serving, restart, eight frozen status keys, projection, redaction, parity, deterministic-scope and verifier checks remain exact.

The original 127 test root recovers byte-for-byte by deleting exactly one registration import. The new module imports the checker rather than its importing test root, has no import cycle, resolves the real repository root from its own directory, builds its own complete temporary corpus and cleans its own roots. Its one positive plus twelve negative cases execute through the existing native-registered test root. Each negative starts from a passing baseline, checks an actual unique anchor, changes bytes and requires the production-proof diagnostic. These are source-policy mutants, not executed Rust mutants.

## Actual RED/GREEN receipts

All eight receipt JSON hashes and all sixteen complete raw command-log hashes were independently authenticated. Logs identify Bun 1.3.9. RED is an actual source-contract or assertion failure with discovered cases. No import, syntax or zero-discovery failure is counted as semantic RED.

| Actual command                   | Result                                             | Seconds | Full raw log SHA-256                                               |
| -------------------------------- | -------------------------------------------------- | ------: | ------------------------------------------------------------------ |
| Full 73 RED                      | Exit 1; 16 pass, 3 fail                            |   0.341 | `0e33618456df65b95f09f86cd920018c99d6a5e689d389c63bb6c18b185166d6` |
| Full 75 RED                      | Exit 1; 23 pass, 4 fail                            |   2.485 | `c33618e125b7e999d0912d1a17e104184312469550479ae33c6eb3e86e5dde43` |
| Full 73 GREEN                    | Exit 0; 19 pass, 0 fail; 29 assertions             |   0.258 | `6b7941e2af7e41e8a2fb1710f91c30ac6ddef386eced85f9481ed95630723073` |
| Full 75 GREEN                    | Exit 0; 27 pass, 0 fail; 67 assertions             |   2.526 | `9fbdceed8f1586f418814e04103f06d4ca71cca44efb145ec295aeed2858201b` |
| Actual 73 CLI                    | Exit 0                                             |   0.364 | `8c5c1fde6293da87a10a568ee170403d295faab4b8219466551b27a0882ae8e2` |
| Actual 75 CLI                    | Exit 0                                             |   0.448 | `10c2cc457946806ac156d1546a4b33af75d0aa22d9fc745b49505ec20e4d0589` |
| Selected 123 positive RED        | Exit 1; 0 pass, 3 fail                             |   0.173 | `f0acce0dc65fd525c4d6ccca9b58dc2286cc1b3d81a6626feeb223b66538df16` |
| Selected 127 positive RED        | Exit 1; 0 pass, 1 fail                             |   0.788 | `b04043f55c6ff3d480a87d60df612ead7a2c6c60115e4c0bd584feb2c311fd8a` |
| Full 123 GREEN                   | Exit 0; 102 pass, 0 fail; 324 assertions           |   2.888 | `96fbf9123e7b04d57e20bf08d408651f51bbcf206bf2e35d9ae267bf1fb39d2d` |
| Full 127 GREEN                   | Exit 0; 31 pass, 0 fail; 68 assertions             |  24.348 | `9b75a40a4f891c29f2c87fc31a6411a37280fddb0be42baab56ac5596b0ffba1` |
| Actual 123 CLI                   | Exit 0                                             |   0.058 | `56848629a16445393539e060264710c6ff5546333657c07909c2054db8967e1b` |
| Actual 127 CLI                   | Exit 0                                             |   0.751 | `9e20d1087131c5e1e95ce213e9744194a5456654f9015e965faf96d1b9ab0819` |
| Incorrect auxiliary 129 filename | Exit 1; setup failure only                         |   0.007 | `4caf1fc1f6df7284c05ab16569054b3dedb7340feb013b708f10258a1492b6fb` |
| Correct actual 129 CLI           | Exit 1; only known P128 durable-trigger diagnostic |   0.866 | `57c501a0743310f1fc69ff3bc9cd29e71452458f4fced8e8e95f59db161d2389` |
| Physical caps                    | Exit 0; 627/619/78 lines                           |   0.004 | `918f69f3459ec39b6529736888ea9692454742a89230f98bb2d9618bafc6ba55` |
| Whitespace                       | Exit 0                                             |   0.110 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

The first two RED logs demonstrate the obsolete needle mismatch, including diagnostic-sensitive new negatives. The selected 123 positives fail on the old inline constructor requirement; 127's selected positive fails on the old production-proof predicate. The full GREEN counts include the preserved original cases and the registered new cases; selected/full runs overlap and must not be added together. The corrected actual 129 filename is check-phase129-integration-guardrails-and-milestone-reconciliation.ts. Its sole remaining diagnostic is `P128 durable trigger: accepted best-tip blocks must queue only after durable save`.

## Unchanged dependencies and remaining gates

All 328 recorded other script paths and all 1,252 recorded Rust/document paths were rehashed from current disk and matched their preservation entries. This records the packet's exact inventory; it is not a compiler or complete filesystem-read closure claim. Actual request/helper/composition/doc evidence and scanner/corpus behavior were independently read. Selected dependency identities:

| Dependency                                                               | Full raw SHA-256                                                   |
| ------------------------------------------------------------------------ | ------------------------------------------------------------------ |
| packages/open-bitcoin-node/src/sync/tests/block_requests.rs              | `af81fb3a8d0060734decdce10d787e3017aae6a1dd64ea1df97f3f7672ecefd9` |
| packages/open-bitcoin-node/src/sync/open_runtime.rs                      | `e0d80123609b1f85a9edb06c18f1c1682268215bace22f343a2c420357c338d5` |
| packages/open-bitcoin-rpc/tests/black_box_parity/fixtures.rs             | `299555281521d51b7e19cb59cd2467f720357c2fc866b4f8446e0be2bd7c2304` |
| packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs | `a787810f831b3f68e78d4a85890e9b9fa05aeafade8317aa0e4e1465037fca99` |
| docs/operator/runtime-guide.md                                           | `c6c497329f53f86e1e2435f664b06f743a0036d7bec052f0ce127401934b6057` |
| docs/architecture/operator-observability.md                              | `8bf68243aeac5746ab0cb42c83fff885ab73db3896f211681fad7be2423f905c` |
| scripts/rust-source-invariants.ts                                        | `746389684fcdb8588388e6d1a6027510debf7ed311ed97e66494e63128b2bcd3` |
| scripts/source-corpus.ts                                                 | `a37575853f5a0501b502882dcb898c04f429699216c1f4eb97512845845acb8f` |

All three held activation dependencies remain exactly equal to their complete captured source bodies: old164 test-modules SHA `10a0a32f77e9ae44baac673085584fe08d331f58b433b7a6c83be22af3e3ac95`, current165 contracts SHA `c6bbdb06f393d41f00fe15b52f81026802aa9a5bc2bdd4345ab91b2f8f753ef4` and current165 entry SHA `f50d86004db1e033654fb8eacd2e5ffb79d3436592743e0ec6c69dc34c094f81`. No premature row or inventory-pin activation was observed.

The original 160 SOURCE rows, 88 TEST owners, 279 INPUT paths and existing inventory pin retain their accepted35 generation at this boundary. Task 3's future row/pin proposal, real stale-pin refusal, new-owner controls, import/copy/read union and closing proof are outside this report. No immutable historical manifest, snapshot, receipt, binding or production Rust behavior was changed by this repair.

Both full-native failures remain failed evidence. Remaining P128/P129/P134/filter repairs, the entire fresh final14 native/matrix/Rust/coverage/Bazel contract, canonical code/security/formal review, lifecycle acceptance and normal-hook commit/push remain Root-owned obligations. This report does not mark any phase requirement complete or activate Phase166–168 scope. Reuse only for the exact nine fingerprints above.
