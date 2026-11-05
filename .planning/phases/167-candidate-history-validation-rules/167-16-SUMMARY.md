---
phase: 167-candidate-history-validation-rules
plan: "16"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: seeded-historical-script-and-bip30-evidence
tags: [rust, consensus, chainstate, regtest, historical-scripts, bip30, evidence]
requires:
  - phase: 167-15
    provides: Accepted serialized ordinary Regtest history and independently expected parent and prevout facts
provides:
  - Exact historical exception bodies through the managed seeded validation kernel
  - Seven ordinary flag-sensitive managed positive and negative pairs and creating-context controls
  - Both exact BIP30 repeats, physical reopen and full prior-Coin restoration evidence
  - Thirty-nine attributable historical results rows with explicit evidence classes
affects: [167-49, 167-50, 167-51, 167-17, 167-19, 167-20, CVLD]
tech-stack:
  added: []
  patterns: [source-bound mechanical context, no earned authority from seeded history, complete physical restoration]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/phase167_historical.rs
    - packages/open-bitcoin-node/src/sync/tests/phase167_historical/source.rs
    - packages/open-bitcoin-node/src/sync/tests/phase167_historical/scripts.rs
    - packages/open-bitcoin-node/src/sync/tests/phase167_historical/contexts.rs
    - packages/open-bitcoin-node/src/sync/tests/phase167_historical/ordinary_flags.rs
    - packages/open-bitcoin-node/src/sync/tests/phase167_historical/bip30_repeats.rs
    - packages/open-bitcoin-node/src/sync/tests/phase167_historical/bip30_restore.rs
    - docs/parity/fixtures/phase167-historical-results.json
  modified:
    - packages/open-bitcoin-node/src/sync/tests.rs
key-decisions:
  - Keep exact historical bodies and derived ordinary candidates in independently seeded mechanical owners without canonical or earned-work authority.
  - Report full overwritten Coin restoration as Open Bitcoin's stronger contract, not Knots normal-disconnect parity.
  - Disclose Task 2's missing preimplementation behavioral RED and Root's bounded post-GREEN mutation-sensitivity disposition.
requirements-completed: [CVLD-02, CVLD-03, CVLD-04, CVLD-06]
requirements_scope: bounded-plan16-evidence-only
global_cvld_acceptance: open
generated_by: gsd-execute-plan
generated_at: "2026-10-27T23:46:23Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
status: canonical-summary-pending-independent-qa-and-root-final16
---

# Phase 167 Plan 16: Source-bound historical scripts and BIP30 restoration

The bounded Plan 16 tests now exercise exact pinned historical script exceptions, seven derived ordinary flag-sensitive spends, creating-history controls, both exact BIP30 repeats and full durable restoration through the managed seeded kernel. This canonical Summary has passed independent whole-plan source/evidence review; document QA and Root FINAL16 remain. It makes no global CVLD, pinned-daemon or whole-phase acceptance claim. No Git staging, commit or push occurred; Phase 168 remains held.

## Basis and checked scope

Accepted [FINAL15](../../../.local/open-bitcoin-dev/phase167/plan15/root-final15-v1/final-source-acceptance.json), SHA-256 `898a20a004047259b6def19e47cc550a84c5b1569c511043e76b83567ff219e8`, supplies the serialized ordinary source and selected canonical Regtest baseline. The [refined Plan 16](167-16-PLAN.md), SHA-256 `86182d7e5d81e23f4683136011f43b8c0dffa958b277d3fadc3b57d632b9101c`, retains its two literal commands, original requirements and wave. Its checked [context allocation](167-15-16-CONTEXT-EVIDENCE-AMENDMENT.md) gives Plan 16 the missing/changed creating-context probes without reclassifying Plan 15 ordinary canonical execution.

The checked Plan 16 amendments are [historical test split](167-16-HISTORICAL-TEST-SPLIT-AMENDMENT.md), [ordinary flag spends](167-16-ORDINARY-FLAG-SPENDS-AMENDMENT.md), [legacy sighash/CODESEPARATOR](167-16-LEGACY-SIGHASH-CODESEPARATOR-AMENDMENT.md), [legacy `OP_IFDUP`](167-16-LEGACY-IFDUP-AMENDMENT.md), [consumed-flag scratch mutation](167-16-CONSUMED-FLAG-SCRATCH-MUTATION-AMENDMENT.md), and [BIP30 restoration mutation validation](167-16-BIP30-RESTORE-MUTATION-VALIDATION-AMENDMENT.md). The repo-local AGENTS guidance, Bright Builds sidecar and overrides, and relevant architecture, code-shape, testing, verification and Rust standards informed the evidence boundaries. Both active lesson files were loaded within their 10,714-byte combined budget.

## Task 1: Historical script and creating-context evidence

The [accepted bounded Task 1 receipt](../../../.local/open-bitcoin-dev/phase167/plan16/root-task1-acceptance-v1/task1-source-acceptance.json), SHA-256 `c8254cae943db2a7476db740822d80723db4148941ee35e8ebe9772e483ff102`, and [independent final Task 1 review](../../../.local/open-bitcoin-dev/phase167/plan16/ordinary-flag-spend-attempt-v1/independent-final-task1-review.json), SHA-256 `1672e366b7b816782d3aa9e9ac22705ff9288e281e63172cc7217323b28a716d`, cover three exact mainnet/testnet historical script-exception bodies through the actual managed seeded interpreter and Coins kernel. Source-bound wrong envelope, network, hash, missing anchor, recovery and policy-only high-S controls retain their distinct refusal classes. The two height-201 creating-MTP pairs reach the actual kernel: missing versus complete creating history and mismatched versus matching creating MTP. Source-bound Regtest retarget rows 143/144/145 and 287/288/289 connect header and body derivation, with changed MTP/bits refusals.

Seven ordinary P2SH, DERSIG, CLTV, CSV, NULLDUMMY, WITNESS and TAPROOT spends start from Plan 15 exact parent and independently expected prevout facts at heights 101, 151 or 201. Their changed coins and candidate bodies are explicitly **derived mechanical fixtures**, never mislabeled frozen Plan 15 bodies. Each flag has a managed selected-rule positive and negative plus a pure single-flag bypass control. Positives compare complete snapshots; negatives assert their named interpreter refusal and unchanged accepted state. These seeded owners neither earn work nor gain canonical chain, BASIC progress or `ScriptsValid` provenance. The consumed WITNESS flag is additionally tested by Root's reviewed [disposable consumed-site mutation closure](../../../.local/open-bitcoin-dev/phase167/plan16/root-consumed-flag-mutation-v1/root-mutation-closure.json), SHA-256 `132e016d2a15e469cc4a9811221b60c3c161110f56a8266c8667f4c764fb39c4`: one compiled named mutant failed at the intended script rejection, and copied/live bytes were restored exactly. Mutation of an inert forwarded flag was not credited.

Two genuine historical Task 1 behavioral REDs exposed cross-plan consensus defects. In mainnet 170060 transaction 3, the old legacy sighash byte filter removed `0xab` inside pushed HASH160 data; the intact source-derived digest is `ab199c185cdcc0d20de9d4a3e613a64678313bb3f314c35db29f9e3e31a5e802`, while the bad filter produced `da9d7f292cc92660ae1fabe30084d77726b87c065476367c88d783357628e7b4`. Root's [sighash closure](../../../.local/open-bitcoin-dev/phase167/plan16/root-legacy-sighash-v1/root-fix-closure.json), SHA-256 `2a1d48d8814fbf88fb7fac29123a30967946f5925d9566ec60a9d809e0c1de6e`, records focused 2-fail RED, 2-pass GREEN and 9-pass regression. In mainnet 692261 transaction 1252 input 0, valid witness opcode `0x73` was rejected as `UnsupportedOpcode`; Root's [`OP_IFDUP` closure](../../../.local/open-bitcoin-dev/phase167/plan16/root-ifdup-v1/root-fix-closure.json), SHA-256 `470b17478389698f1059f7ce5aabf2a9c95720e2c5356a3ea284037bc901bc45`, records 3-fail/1-pass focused RED, 4-pass GREEN and 56-pass script regression. The initial Task 1 compile failure and later incorrect provenance assertion have **no behavioral RED credit**; the ordinary P2SH and consumed-flag mutants are post-GREEN sensitivity checks.

## Task 2: Exact repeats, restoration and results

The [Task 2 evidence index](../../../.local/open-bitcoin-dev/phase167/plan16/task2-attempt-v1/evidence-index.json), SHA-256 `5de034becb7249b02dca829b1fc1bd05bb0cbb10b5d0065852099d6a32170b97`, [executor draft](../../../.local/open-bitcoin-dev/phase167/plan16/task2-attempt-v1/SUMMARY-DRAFT.md), [independent Task 2 review](../../../.local/open-bitcoin-dev/phase167/plan16/task2-attempt-v1/independent-final-task2-review.json), SHA-256 `00d1c8ae6b6f09d1e5bc33b6c3d0bb9024c75f9f13a6188e077d1b40809cc79a`, and [independent prior-frame review](../../../.local/open-bitcoin-dev/phase167/plan16/root-task2-final-frame-v1/independent-frame-review.json), SHA-256 `0c19d6e286e0aac03357d55e50c86794222b7dc7d6007103f76f9ba371263a12`, found no bounded source/evidence blockers at their earlier source frame. Their frame and logs are retained as history, not final formatted-source credit. Both pinned repeat bodies, 91842 over 91812 and 91880 over 91722, reach a disk-backed managed seeded owner with independently reconstructed complete prior Coin output, height, coinbase and parent MTP. The 91880 body also spends its source-bound external outpoint. Each connected body is checkpointed; all handles are released; the state is physically reopened, disconnected with a flush, reopened to compare the complete restored active/Coins endpoint, reconnected and reopened again. Historical body/undo retention is checked separately from active undo.

Fully spent old outpoints capture empty overwrite batches; changed old Coin and unrelated live duplicate cases refuse. Wrong network, height, hash and ancestor facts refuse before accepted mutation. Twelve checked synthetic BIP34 contexts separate mainnet/testnet3 height 1,983,701 match/mismatch/absence from 1,983,702 match/mismatch/absence; they are contextual rules evidence, **not** exact-PoW admission. For each repeat, missing logical undo, malformed decode and decodable wrong binding refuse against an unchanged post-injection physical manifest while retaining the exact body. Source preflight checks selected block identity/Merkle/PoW, envelope and listing, header response digest and 80-byte facts, external transaction response pointer/digest, before store open. Refreshed-digest altered body, context and missing-envelope controls still refuse. The generic Mainnet ordinary-open guard and cross-network canonical Regtest reproof refuse this seeded fixture; the latter alone does not prove same-network seeded-origin refusal. Complete overwritten Coin restoration is Open Bitcoin's stronger contract, not a Knots normal-disconnect parity claim.

The tracked [historical results](../../../docs/parity/fixtures/phase167-historical-results.json), SHA-256 `5948da8d8c5776e9bfeecd71202fe99db2125a7ca8f38d8d0b8e4f5b93700cde`, contain **39 unique passed rows**. The 35 Task 2 rows cite distinct once-only markers in the final formatted-source case log; the four `RG16-CREATING-*` rows cite the accepted Task 1 named tests and receipt `c8254cae943db2a7476db740822d80723db4148941ee35e8ebe9772e483ff102`. Classes are six real seeded bodies, two physical restores, eleven source refusals, six checked synthetic rules, six checked context refusals, four durable refusals and four Task 1 actual-kernel observations. Source pinning names Knots commit `a9aee730466ac67d35a3c03ee24676be5e045878` and exact corpus digests. Only receipt paths/hashes and three attempt-history entries changed during final formatting; all 39 expected and observed outcomes stayed unchanged. No skipped or failed case is counted as passed.

Task 2 did **not** obtain a genuine preimplementation behavioral RED. Its moved-value compile failure, deliberate unimplemented source-boundary stub, memory-backed reopen wiring, invalid undo binding injection, genesis header-list false rejection and default-target directory-read stall are setup, scaffold, host or test-regression evidence. Root's [BIP30 restore mutation closure](../../../.local/open-bitcoin-dev/phase167/plan16/root-bip30-restore-mutation-v1/root-mutation-closure.json), SHA-256 `1a4bd9f8deddc3b7b8a0d14630347d1f54be1a97a027f37a56227f6b2818e8f3`, records a **post-GREEN** disposable restore-loop mutation: one compiled named test failed at the complete prior-Coin assertion, then 7,846 copied/live files were restored exactly. Root's [TDD disposition](../../../.local/open-bitcoin-dev/phase167/plan16/root-task2-tdd-disposition-v1/disposition.json), SHA-256 `f2752f0eaca00384765a9543bc66caf05558215b2adc74db490228fab7670435`, accepts that documented process deviation for bounded Task 2 source acceptance after summary and whole-plan review. It grants no preimplementation RED credit and reduces no behavioral requirement.

## Actual verification and attempt history

Both original commands ran serially through the cooperative timing wrapper with pinned Bun on PATH and the diagnosed isolated `CARGO_TARGET_DIR=.local/open-bitcoin-dev/phase167/plan14/cargo-target`; their Cargo argv remained literal. The formatted-source final [Task 1 log](../../../.local/open-bitcoin-dev/phase167/plan16/root-final-format-v1/task1-final.log), SHA-256 `3ba5e88e4b6aa319eb93102732d0907f8b2e7bd2194f656b8faa38205a88d703`, reports **16 passed, zero failed** named tests. The formatted-source final [Task 2 log](../../../.local/open-bitcoin-dev/phase167/plan16/root-final-format-v1/task2-final.log), SHA-256 `0af6007ec057f3b57a77a68aba1c264bd8d3dfca757c999d6d4018a57a04c035`, reports **7 passed, zero failed** named tests. Separate integration binaries with zero matching tests earn no credit.

```bash
bun run scripts/command-timings.ts run --key phase167-16-task1 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_historical_scripts -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-16-task2 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_historical_bip30 -- --test-threads=1
```

The extra timed `--nocapture` formatted-source [case-observation log](../../../.local/open-bitcoin-dev/phase167/plan16/root-final-format-v1/task2-cases-final.log), SHA-256 `e3e5777ba89e76f6023d4f61d3059b1b0e0b10c6d2cf0d071654ec547ad301b3`, passed 7 named tests and emitted 35 distinct Task 2 markers. The [final checks receipt](../../../.local/open-bitcoin-dev/phase167/plan16/root-final-format-v1/checks-final.json), SHA-256 `37a3cad603c14edfde49653c4bb741c757984fe657e9e8c106a73c9a813aab1e`, records zero exit for `bun scripts/phase167-historical-corpus.ts --check` (seven source blocks and 5,263 previous transactions), `bun scripts/bright-builds-check.ts all` (zero findings), scoped Rustfmt and `git diff --check`. Owned formatted source files fit the 628-line threshold. The initial default-target rustc process was sampled twice while stalled in `ReadDir/__getdirentries64` before termination; its logs and samples remain in the Task 2 historical evidence index. No stall, compiler or test-wiring attempt is counted as a product GREEN or behavioral RED. Full repo-native `bash scripts/verify.sh`, coverage and Bazel acceptance remain Plan 20 work.

## Whole source frame and inverse

Root's [format correction receipt](../../../.local/open-bitcoin-dev/phase167/plan16/root-final-format-v1/correction.json), SHA-256 `c4e062e22378444b8fd25ceaf17ebffa47fe00ec5fd61aed90f30749dcb5500a`, binds the [complete formatted-source final frame](../../../.local/open-bitcoin-dev/phase167/plan16/root-whole-plan-final-v2/final-frame.json), SHA-256 `ef6831ace6e1bdd26341b91ae3f0e6ab66c1a3fcad40284b3c3b3d9a87093872`: **482 whole source bodies**, 201 native and one supplemental native state. The original Plan 16 union is **11 owners**, comprising three original whole bodies and eight original true absences; the other **471 protected source paths** include six separately reviewed Root consensus fixes. Of the three original whole bodies, `sync/tests.rs` changed to register the historical module and received only a Rustfmt module-order swap in the final correction, while shared `phase167_support.rs` and `candidate_rules.rs` retained their original bytes. Eight original absences became the seven historical test modules and the results JSON. The [formatted-source whole original-owner scratch inverse](../../../.local/open-bitcoin-dev/phase167/plan16/root-whole-plan-final-v2/whole-inverse.json), SHA-256 `f8acd04c7fd71ef6e41d53ba33f3d07d2e70ca84923d478f9ec152d65493da15`, restored all 11 original states, preserved 471 protected and 201+1 native states, then reapplied all 482 final source bodies without a live source write. The previous `c43a0c...` frame, `407603...` inverse and previous logs are historical with no final-source credit. Root records zero protected/native drift; whole-plan source acceptance is still pending.

## Completion review and remaining gates

The independent [whole-plan evidence review](../../../.local/open-bitcoin-dev/phase167/plan16/independent-whole-plan16-evidence-review-2026-10-27.json), SHA-256 `1c017d29ff4d8d1810a610e32b90cf2407aa468d14d6b8919e2c5dd3c2d384d1`, matched the formatted 482-source/201+1-native frame, all 39 attributed rows, the final commands and whole inverse with no substantive blocker. A prior targeted strict consensus Clippy probe found an unrelated accepted Plan 14 test-style lint in `chain_spec/tests.rs:564`; its [deferred hold](../../../.local/open-bitcoin-dev/phase167/plan16/root-legacy-sighash-v1/deferred-clippy-hold.json), SHA-256 `95edc5716b7ea18de4eac35b2c2341a91bb78a658000fa1e4533485b085c9834`, must be repaired and rechecked before the final commit. No strict Clippy pass is claimed here.

Plan 16's bounded Task 1 and Task 2 evidence passed independent whole-plan review, subject to the disclosed Task 2 TDD deviation; canonical Summary QA and Root FINAL16 remain. The mechanical seeded context does not imply actual pinned Knots-daemon differential qualification, ordinary mainnet/testnet product activation, unattended history sync, canonical earned work or BASIC publication. Global CVLD-01/02/03/04/06 acceptance remains open. Plan 19 must reconcile parity breadcrumbs; Plan 20 and Root retain full native, coverage, Bazel, security, goal and lifecycle gates plus normal-hook commit/push on main. Stop before Phase 168.
