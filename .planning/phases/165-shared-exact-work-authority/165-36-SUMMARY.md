---
phase: 165-shared-exact-work-authority
plan: "36"
subsystem: testing
tags: [bun, typescript, source-guards, provenance, activation]
requires:
  - phase: 165-35
    provides: Accepted exact table move and unchanged current inventory
provides:
  - Current coverage, documentation, initialized-network and served-block source guards
  - Independent semantic regressions preserving all previous test cases
  - Reviewed 168-source-row ownership inventory and actual 287-input pin
  - Independently accepted 5018-entry generation and ten final focused checks
affects: [165-37, 165-38, 165-39, phase165-final14]
tech-stack:
  added: []
  patterns: [Owning-function source evidence, independent mutation regressions, source review before pin activation]
key-files:
  created:
    - scripts/check-phase127-authoritative-network-state-unification/production-proof-tests.ts
    - .planning/phases/165-shared-exact-work-authority/165-36-SUMMARY.md
  modified:
    - scripts/check-phase73-uat-verification/constants.ts
    - scripts/check-phase73-uat-verification.test.ts
    - scripts/check-phase75-soak-runner.ts
    - scripts/check-phase75-soak-runner.test.ts
    - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
    - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
    - scripts/check-phase127-authoritative-network-state-unification.ts
    - scripts/check-phase127-authoritative-network-state-unification.test.ts
    - scripts/check-phase164-canonical-identity/test-modules.ts
    - scripts/check-phase165-shared-exact-work.ts
key-decisions:
  - Require current owning code and exact separate documentation spellings without changing Rust or documentation.
  - Preserve previous tests and add one normally imported independent Phase127 test module within readable file bounds.
  - Accept all nine source bodies before deriving ownership identities and activating only one renewal, eight appends and the actual inventory pin.
patterns-established:
  - Complete source inverses, real RED/GREEN and independent review precede ownership activation.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T23:05:35Z"
duration: 21min
completed: 2026-10-17
---

# Phase 165 Plan 36: Current Coverage and Runtime Source Guards Summary

**Four source guards now recognize current coverage, distinct ledger-path spellings and the actual initialization/served-block owners, with 179 passing native-registered cases and independently reviewed ownership activation.**

## Performance and bounded scope

- Started: 2026-10-17T22:44:34.325631Z, authentic complete before capture.
- Completed: 2026-10-17T23:05:35Z; three tasks, eleven source files including one new test module.
- Four full test roots execute 19 / 27 / 102 / 31 passing cases respectively; four repaired live checkers exit 0.
- Final ten-command child time totals 41.100213 seconds, including a fresh 18-control ownership proof and 178 affected Phase165 cases.
- Root owns shared metadata, external verification tools, independent acceptance and all Git. This executor changed only the eleven declared source owners and this summary.

Repo-local AGENTS guidance, the Bright Builds sidecar, architecture/code-shape/testing/verification/TypeScript standards and both complete active lesson inputs informed execution. No applicable override or project skill was needed. Existing fixture/table/scanner conventions preserve readable bounds and functional source predicates.

## Task 1: Current coverage and distinct documentation paths

Phase73 now requires `bounded_block_requests_prioritize_best_headers_and_include_accepted_sides`. Its independent fixture uses that current name. Two appended cases remove it or substitute only the obsolete best-chain-only name, asserting a unique changed anchor and the exact coverage diagnostic. All seventeen previous cases, including the six Plan34 live/raw observability cases, retain their complete original bodies and helpers; only the fixture needle changes.

Phase75 changes only the runtime-guide expectation and its independent fixture to require one literal U+005C backslash immediately before the run identifier’s opening angle in its ledger path. The observability sentence separately requires no backslash at that position. Six appended cases independently omit, substitute the spelling or alter the ledger filename for each document, with unique changed anchors and exact missing-evidence diagnostics. All twenty-one previous cases and helpers retain their bodies. Neither document was edited or globally normalized.

## Task 2: Actual initialization and served-block owners

Phase123 preserves the ordered recovered-chainstate initialization prefix and follows the exact caller binding through `Self::prepare_initialized_network`, consensus-parameter derivation and `Self::finish_initialized_runtime`. Exact call vectors require one preparation and one finish call. The owning preparation helper must retain its exact seven-argument `ManagedPeerNetwork::from_initialized_chainstate` call, configured indexing resolution, local peer configuration, saved-header seeding and returned network. Existing compatibility defaults, daemon/explicit-fixture activation and all timing/write/authority guards remain.

The independent synthetic fixture models that complete actual helper flow. Twelve rows added to the existing activation mutation table cover missing/commented preparation, substituted store/authority/config/startup policy, missing/wrong/early finish and substituted constructor config/policy/bound. Every new row was separately proved to have exactly one actual source anchor and change its bytes. Reversing those twelve additions and the single fixture-entry replacement recovers the entire original test file exactly. Its previous ninety cases remain and the file is 627 lines.

Phase127 replaces only the obsolete inline block-pattern requirement with the actual owning `matches_block` function's exact executable block-variant/header-hash predicate and the served/mixed response assertions in the real composition function. Comment/literal masking and owning-function extraction reject unrelated source. All other integration, shared-authority, durable-serving, frozen-schema, parity and redaction anchors remain.

The existing Phase127 test root is exactly its complete original source plus one ordinary import. The new independent 78-line module uses the existing target list, real source corpus, temporary complete corpora and cleanup. Its current-source positive and twelve changed-anchor negatives exercise wrong variant/hash, unconditional/missing helper, comment/literal decoys and omitted/wrong/disconnected assertions. All eighteen old cases remain; the existing root is 619 lines. These are source-policy mutations, with no execution of mutated Rust.

## Actual test-first and repaired-checker evidence

Every RED below preceded its corresponding checker fix and failed through source-policy/assertion behavior with actual test discovery. It was not an import, syntax or no-op failure. All full GREEN roots have zero failures and no skipped/ignored cases. Artifacts in this summary with a `phase165-plan36-` basename reside under `.local/open-bitcoin-dev/`.

| Run                   | Actual cases / assertions                   | Exit / seconds | Full actual log SHA256                                             |
| --------------------- | ------------------------------------------- | -------------- | ------------------------------------------------------------------ |
| Phase73 RED           | 16 pass / 3 fail / 28 assertions            | 1 / 0.341393   | `0e33618456df65b95f09f86cd920018c99d6a5e689d389c63bb6c18b185166d6` |
| Phase75 RED           | 23 pass / 4 fail / 66 assertions            | 1 / 2.485239   | `c33618e125b7e999d0912d1a17e104184312469550479ae33c6eb3e86e5dde43` |
| Phase123 RED          | 0 pass / 3 fail / 4 assertions; 99 filtered | 1 / 0.172988   | `f0acce0dc65fd525c4d6ccca9b58dc2286cc1b3d81a6626feeb223b66538df16` |
| Phase127 RED          | 0 pass / 1 fail / 1 assertion; 30 filtered  | 1 / 0.788147   | `b04043f55c6ff3d480a87d60df612ead7a2c6c60115e4c0bd584feb2c311fd8a` |
| Full Phase73 GREEN    | 19 pass / 29 assertions                     | 0 / 0.258019   | `6b7941e2af7e41e8a2fb1710f91c30ac6ddef386eced85f9481ed95630723073` |
| Full Phase75 GREEN    | 27 pass / 67 assertions                     | 0 / 2.526283   | `9fbdceed8f1586f418814e04103f06d4ca71cca44efb145ec295aeed2858201b` |
| Full Phase123 GREEN   | 102 pass / 324 assertions                   | 0 / 2.887641   | `96fbf9123e7b04d57e20bf08d408651f51bbcf206bf2e35d9ae267bf1fb39d2d` |
| Full Phase127 GREEN   | 31 pass / 68 assertions                     | 0 / 24.348368  | `9b75a40a4f891c29f2c87fc31a6411a37280fddb0be42baab56ac5596b0ffba1` |
| Live Phase73 checker  | Current repository                          | 0 / 0.363623   | `8c5c1fde6293da87a10a568ee170403d295faab4b8219466551b27a0882ae8e2` |
| Live Phase75 checker  | Current repository                          | 0 / 0.447979   | `10c2cc457946806ac156d1546a4b33af75d0aa22d9fc745b49505ec20e4d0589` |
| Live Phase123 checker | Current repository                          | 0 / 0.058295   | `56848629a16445393539e060264710c6ff5546333657c07909c2054db8967e1b` |
| Live Phase127 checker | Current repository                          | 0 / 0.750858   | `9e20d1087131c5e1e95ce213e9744194a5456654f9015e965faf96d1b9ab0819` |

The current Phase123 independent synthetic, real repository and immutable live-corpus positives all rejected only the stale inline-constructor expectation during RED. Phase127's independent actual-corpus positive rejected the stale production-proof contract. After repair, every new negative starts from a passing baseline and tests the intended diagnostic. Across the four full GREEN roots, 146 prior cases and 33 new cases produce 179 passes / 488 assertions.

One auxiliary Phase129 invocation used an incorrect filename and failed module discovery: log SHA256 `4caf1fc1f6df7284c05ab16569054b3dedb7340feb013b708f10258a1492b6fb`. This remains a setup failure, separate from semantic RED. The corrected registered Phase129 checker actually exits 1 with only `P128 durable trigger: accepted best-tip blocks must queue only after durable save`; its log SHA256 is `57c501a0743310f1fc69ff3bc9cd29e71452458f4fced8e8e95f59db161d2389`. Phase129 source was not edited and no Phase129 GREEN is claimed. Task2 cap and whitespace checks also exit 0.

## Complete source acceptance before identities

The authentic 344-body baseline remains SHA256 `a6f23987c5d1ea36909f8f49866f17bd086199014509171d63524c4252e229fd`. The complete Plan36 before packet is `phase165-plan36-before.json`, SHA256 `cd640952c84501195945c889eac1761106310a479909fc2f1dc804d49dc7fd49`: eight complete existing Task1/2 bodies, explicit absence of the new module and three held activation dependencies.

The complete actual nine-source packet `phase165-plan36-task12-source-proposal.json`, SHA256 `eae9303ad85ac00dc2de22a86b8d0bf7d3c9cde541c81d08f1d58d99f80c9b45`, contains all before/after bodies, eight complete ordered raw-SHA-gated inverses, the exact new-file absence inverse, all sixteen Task1/2 command receipts and complete logs. Its diff SHA256 is `9e10529a47e7e9e209f2c57d3ed5481118553c03d077a5ed2d4878dcb4aa4ae8`. It proves 328 other captured scripts and 1252 original R2 Rust/documentation raw dependencies unchanged. All source files fit within 628 physical lines.

| Actual Task1/2 owner                                                                       | After lines | Full accepted raw SHA256 and actual sourceIdentity                 |
| ------------------------------------------------------------------------------------------ | ----------- | ------------------------------------------------------------------ |
| `scripts/check-phase73-uat-verification/constants.ts`                                      | 255         | `c439208c788a99d2e0b038e0683a227406fee350c0cf13e50ca9863ab104684c` |
| `scripts/check-phase73-uat-verification.test.ts`                                           | 568         | `ceccf28ae62aaf64fb9187e085345f76e21e7f6a4d86d1a3f3ead3163524d23c` |
| `scripts/check-phase75-soak-runner.ts`                                                     | 390         | `c43ea0d1ea2b1e54d2e2b9f5b53af0871dbaf0d01ab463302ca33504aa71f890` |
| `scripts/check-phase75-soak-runner.test.ts`                                                | 522         | `db954c5f7cd8841331a16003b0c82d61c9be159f2aebe6a8261ee85bbb252cb1` |
| `scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts`                       | 332         | `780c60a7b3aad52af73a7011c17afe5319eb06ffcf18f14af3b30b9c6bb98761` |
| `scripts/check-phase123-runtime-timing-evidence-integrity.test.ts`                         | 627         | `e043a206cb175bdc5e4251050d7eccbfb9c5d51b36a87c1e86241e6d939fcb6a` |
| `scripts/check-phase127-authoritative-network-state-unification.ts`                        | 566         | `abaed07a9863c66b8588ebf33494e303e396265d39017fd780c140d2e45f4c92` |
| `scripts/check-phase127-authoritative-network-state-unification.test.ts`                   | 619         | `1f494d8813e2145beb15c5b55f438dddfe7208e7bcafc802d9b0169b3a495aef` |
| `scripts/check-phase127-authoritative-network-state-unification/production-proof-tests.ts` | 78          | `4bce72cde7265b9ebf851a407b40d0b9529baa99e02b17759a4b858eaec19c7c` |

Independent CLEAN review `165-PRE-REVIEW-INTEGRATION36.md` has SHA256 `bd048308b5217bce2bdf94da56358eac3929fd71865f917dbf868b243da42a2d`. Root's independent complete source/inverse/dependency/log audit is `49e60f85ee923a5740a6edcf5bd849cb1e6fff444d6a3cfff02b4d036ff8c270`; explicit source acceptance `phase165-plan36-root-source-acceptance.json` is `33dec83529a532ba2fe711895d7c34b18e9f8dfd588bee918909949ecd983fd8`. These precede the first sourceIdentity derivation and any ownership-row/pin change. For these nine actual files, the derived literal-bearing identities equal their complete raw hashes; this was measured rather than assumed.

## Task 3: Reviewed ordinary rows and real inventory refusal

The inert proposal `phase165-plan36-inert-proposal.json`, SHA256 `0eaf0ca9aac748e8a8e2d67a43e3b7301246b681e964e88e50939580a1aef89a`, uses fresh isolated imports of the actual fourteen-module CLI closure. It renews only the existing Phase73 test row at index 159 and appends eight ordinary ownership rows in declared task order. The appended paths are constants73, checker75, test75, checks123, test123, checker127, test127 and the new127 module; their actual input positions are 266–273. All other previous rows and order remain exact. The actual imported 168 SOURCE rows / 88 TEST owners / 287 INPUT paths produce ordered JSON pin `cda7b11af8aa9996d2c84226fc8af2e2cbd963ac7e20d9112b42a8ada70a4a5a`.

The proposal preserves all 143 old SOURCE_FILES, 606 EDGES, 26 TEST_MODULES, 88 TEST_OWNERS, every non-SOURCE contracts export and the complete original 388-line host prefix. Current contracts remain exact at `c6bbdb06f393d41f00fe15b52f81026802aa9a5bc2bdd4345ab91b2f8f753ef4`; the table stays in the existing old data leaf. No historical identity, manifest, snapshot, receipt or adapter is renewed.

| Activation owner                                            | Actual after lines / bytes / full raw SHA256                                     | Complete inverse restores                                          |
| ----------------------------------------------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `scripts/check-phase164-canonical-identity/test-modules.ts` | 562 / 46280 / `4428312183d12e6ce73cae507b4a6b546c39106a70552a16da040d57344a863a` | `10a0a32f77e9ae44baac673085584fe08d331f58b433b7a6c83be22af3e3ac95` |
| `scripts/check-phase165-shared-exact-work.ts`               | 269 / 14766 / `92c545cb37f04967349a085517739872f93bc5e2499b23918a88929f37e851c7` | `f50d86004db1e033654fb8eacd2e5ffb79d3436592743e0ec6c69dc34c094f81` |

Root activation approval SHA256 `662481a9ce2654288c29d5db3c801af88e3f67c27e10e4253f2053f8769ab8b9` follows independent actual array/identity/pin/inverse/control review. Only the approved host preview was applied first. With the unchanged old pin `80dca53e69a2f0c917ae78297108ee16ad1187f2afb6d46e255790585770f624`, the real current CLI then exited 1 in 0.032877 seconds with exactly the bounded inventory-refusal message. Its log SHA256 is `23c1e64fed4753c2397625462c6823bedb6e76f5cd34fec20537cd874304a2da`; receipt `phase165-plan36-stale-inventory.json` is `8430e775a7f46acd0ac62ba6bdcf9e40e2e332fada7f0f59caa51af5b2c72126`. This is refusal before bounded source reads; import-time module I/O remains separately attributed. Only then was the approved entry preview applied, changing solely its inventory constant.

Actual post-pin loading reads the 287 fixed inputs and passes the pure guard. Eighteen independent append/disabled loaded-source controls each yield exactly the targeted ordinary ownership finding across the nine owners. No mutated module or Rust was executed. Complete actual byte inverses restore both held activation files, and every accepted Task1/2 source remains exact afterward.

## Actual read union and reviewed external tools

Fresh read-union capture `phase165-plan36-read-union.json`, SHA256 `1df48b1bc22df596563d2be58554ba2a37a724a238515f16bad22142536c9116`, measures 57 static modules, 14 copied CLI modules, 287 fixed inputs and 339 known script paths. Its 786 finite source/factory/loader/corpus observations include 80 additional roots and 346 corpus descendants. Source-corpus membership for Phase72 remains 30 roots / 133 raw files. These are actual finite repository/provenance observations, without an OS or compiler coverage claim.

Independent external-tool review, SHA256 `61f4e1e0bda74f1fcbcd84782aedfcb1489287c320bbd67f636ee216dfc6bc1d`, resolved three issues before the final freeze: unique output naming for the driver-fresh post-pin proof, the correct activation-entry allowlist owner, and explicit membership for all seven current non-test daemon helpers. The authenticated supplement `phase165-plan36-complete-read-union.json`, SHA256 `83ae6254a71ab65bd2fc6957d45b802e591a71519f6db6b15ba280aed089368d`, adds three observations to reach789. All seven helper files already belonged to the carried4936-entry raw closure. Their membership is recomputed before every command check. The initial786 receipt remains immutable.

The original proof program SHA256 `aa36ac85ed9350b1711d18b7c84a8f2de40ef0d3774efb0267b224e74e25547a` is preserved separately. The reviewed program changes only its optional output argument and has SHA256 `0d61833e16c8eb492048713c353079ef706a46518d55feb686d0b5db1f7fff2b`. The first actual post-pin run and later driver run each execute the eighteen controls with separate output paths and durations. Their deterministic report bodies both hash to `9630b785e45f5f6a1d8b573156e592e14eb85147282df71f30e281cc1f17206d`; the second result is fresh execution, not reused evidence.

The current derived factory has 171 files / 4,603,966 bytes, with digest `de2291118f006726b0a08e90f0c8caebae8bd90f79bcfd62e250ff747baeb65d`. This current raw-source derivation does not renew historical evidence.

## Final accepted raw generation and focused commands

Freeze `phase165-plan36-freeze.json`, SHA256 `1d96076e02ae0855e94be74c0528e50bc1f8ec43250959162dfd8990bab8bdfb`, contains5018 canonical presence/bytes/full-SHA states. It retains all4936 accepted35 entries, including every original4829 R2 entry. Exactly14 prior rows differ: eight existing Task1/2 owners, the two approved activation owners and Root's STATE/ROADMAP/todo/Plan14 metadata. The new test module is an added path. No old path is subtracted. The two inherited explicit absences remain represented.

Root independently accepted every raw state, all789 fresh observations, all nine source bodies, both activation inverses and current18-control proof before dispatch: `phase165-plan36-root-freeze-acceptance.json`, SHA256 `aeb5b91dea362f115f35ae85d5d5c33f290a23325b7e85cca9193a028fb4db96`. The external accepted freeze SHA is checked before manifest parsing. Complete raw states and dynamic helper membership are checked before and after each of ten sequential commands with pinned Bun1.3.9; every check reports zero mismatches.

| Final driver command                      | Exit / seconds | Full actual log SHA256                                             |
| ----------------------------------------- | -------------- | ------------------------------------------------------------------ |
| Fresh post-pin proof and18 owner controls | 0 /10.248822   | `9f67e1d7c642cf6a4ccd9a5f73caa77222e39decbd59e27297af6b5d8e3e48a6` |
| Existing affected Phase165 selector       | 0 /26.323460   | `ee6bfb4eeba38843db1b1df0f42aa45d810f2d12f98fc122fc16041050a47126` |
| Live164 checker                           | 0 /0.594313    | `0d78b33a529f49e707acf1017c7d05661ff87947635206b6eab5729c657ceb5e` |
| Live165 checker                           | 0 /0.577867    | `c38a677714e8b67093a8aa9f55f7473c4325b6886ca039193cc1a19920964ca5` |
| Fast158 checker                           | 0 /1.855632    | `767b6db464f7d582a64e48de5e0dcc876f62dd29338976b5de1af04e4d6152dc` |
| Fast162 checker                           | 0 /0.995479    | `10cf94bc32c825073ad4c98764e738cd6865d709876e2323591ea90252a2df53` |
| Parity breadcrumbs                        | 0 /0.245234    | `f98277765cf0be0f6892990cd622d341dfa5bd2966263ec6a0df64a2606cb73f` |
| All managed checks                        | 0 /0.160155    | `f6020fc944ea1440798b82442c8121b662350d28252e146d89a4aaa45caf0f9b` |
| Physical line bounds                      | 0 /0.004111    | `4d9ccaf3c4b19133b4c59e568e2bb2fc73e9940a1233d155bf774a91a4431adb` |
| Whitespace diff                           | 0 /0.095142    | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

The affected selector actually executes 178 cases / 490 assertions, with 0 failures and 1788 intentionally filtered cases from the unchanged 1966-case test root. There are no skips/ignored/todo cases. Six explicitly discovered family subsets are baseline 1, ordered inventory 1, copied fresh CLI 14, exact tooling inverse 4, historical child ancestry 4 and fixed loader 4. Every exact case name, argv, duration, exit and log hash is in `phase165-plan36-results.json`, SHA256 `d0a855241125e08eb403442143e94d62fca7a399b8fc92a97c8bdb8466a262cc`; the ten-log close is `phase165-plan36-closed.json`, SHA256 `bae36db99348efc03d5f7101f73fc40b391ffedd0d9bdfce72ded122c8a5d2b1`.

Root independently rehashed every complete5018 raw state and ten full actual logs, authenticated all178 names/six families and the fresh18 controls, then accepted the close in `phase165-plan36-root-command-acceptance.json`, SHA256 `40d8ac25aa7fc6144fe666379fdae45762bcd5e46f4721d938b6ba9a04f70969`. All nine accepted source bodies remain unchanged on this closing generation.

## Decisions, threats and simplification

Two exact text needles and current owning-function predicates resolve the demonstrated stale assumptions. One new test-registration module supplies capacity while keeping the original test bodies and readable627/619-line roots. No production Rust, documentation, target-list/verifier wiring, current165 contracts/tests/support or historical normalization changed. Existing fixture strings are intentional test corpora; no implementation stub was introduced.

T-165-36-01 is covered by current coverage and distinct document negatives;02 by exact caller/helper/finish arguments and activation mutations;03 by owning variant/hash/assertions and independent decoys;04 by source-before-identities review, exact activation inverses, ordinary controls and real stale-pin refusal;05 by file bounds, complete read/raw/log closure and bounded claims. All planned mitigations are evidenced. No additional trust-boundary surface or threat flag was found.

## Task commits, issues and remaining ownership

Tasks1–3 are complete. No nested Git, hook change or shared metadata mutation was performed by this executor. Per the execution contract, commits and all planning metadata remain Root-owned and deferred to strict phase-wide verification and normal-hook finalization. The auxiliary129 setup error and pre-freeze external-tool corrections above retain their actual scope; they do not count as semantic RED or production changes. No authentication gate occurred.

Requirements-completed remains empty. Four repaired checker families are green, while the known128/129/134/155/156/157/159/161 failures await Plans37–39. This plan did not run a fresh81-CLI diagnostic and does not relabel the earlier69pass/12fail discovery. Both actual native failures remain failed evidence; neither is replaced with a partial native pass.

Final14 retains its original Task XML/commands and entire fresh default native, complete164/165 matrices, Rust/benchmark/fresh coverage/Bazel, whole-phase code/security/formal/lifecycle, real LOC and normal-hook strict-main commit/push gates. No such final acceptance is claimed here. CWOR-04 remains Phase166-owned; stop before166. No user setup is required.

## Self-Check: PASSED

The self-check found all 57 required source, proof, log, plan, review and summary files. It verified all nine actual source bodies, eight complete source inverses, the authenticated new-file absence inverse and both complete activation-file inverses. All 26 Task1/2 and final-driver actual command logs match their recorded full raw hashes.
