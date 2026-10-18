---
phase: 165-shared-exact-work-authority
plan: "30"
subsystem: chainstate-preparation-regression-proof
tags: [same-core, closed-flush, parent-only-coins, exact-error, source-controls]
requires:
  - phase: 165-31
    provides: Accepted consumer fixtures, current guard data hosts and complete prior read boundary
provides:
  - Actual second-input MissingCoin regression through a genuine parent-only owner
  - Zero-cache and unchanged-state evidence after rejected preparation
  - Reviewed source and enabled-test ownership with 26 current preparation controls
  - Proportional focused verification closed over 709 stable raw entries
affects: [165-14]
tech-stack:
  added: []
  patterns: [Same-core normal closed flush, Exact typed error assertion, Existing strict source-control loop]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/chainstate/tests.rs
    - scripts/check-phase165-shared-exact-work.ts
    - scripts/check-phase165-shared-exact-work/contracts.ts
    - scripts/check-phase165-shared-exact-work.test.ts
    - scripts/check-phase164-canonical-identity/test-modules.ts
key-decisions:
  - Prove the old false pass by strengthening only the expected error before changing setup.
  - Flush the original genuine core into its own parent and retain complete existing parameters.
  - Reuse the existing strict mutation loop and current TEST ownership data host.
  - Use reviewed proportional checks here and retain complete final Plan14 gates.
requirements-completed: []
requirements-contributed: [CWOR-03, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T21:13:16Z"
client_date: "2026-10-17"
duration: 14min
completed: 2026-10-17
tasks_completed: 2
files_modified: 5
---

# Phase 165 Plan 30: Genuine Parent-Only Preparation Regression Summary

**The existing preparation regression now reaches its deliberately missing second input through a genuine earned core, while proving that rejection leaves the parent coin spendable and the live cache empty.**

## Scope and Source Proof

Both tasks completed under checked plan `486ad9b2cbfefe5b7b77ad07dbea73d9188f3e304dbb27fd0ab6aaae66344b0e`. Exactly five declared existing files changed. No production code, API, module, dependency, breadcrumb row or cap exception was added. UTC and America/Chicago completion dates are both 2026-10-17.

The authentic five-file baseline and actual imported arrays are preserved in `.local/open-bitcoin-dev/phase165-plan30-before.json`. The Rust baseline was 568 lines, SHA `a77c8ca532e9d14716adacbb8dd8f9bf786367a05da9a1502aef88a64c10e472`.

Only `prepare_connect_then_drop_leaves_parent_unpopulated` and the minimal `CoinsFlush` import changed in Rust. Reversing those changes reproduces the complete baseline; all other ten direct tests and helpers are byte-exact. The existing child module and parity breadcrumbs remain intact. The final Rust file is 613 lines, SHA `6a654bf137f8be887a525c64f025c9e080a78f08d3daf3bb90e94b382c33cfb6`; its literal-bearing source identity is `d8722a49e277139f00bd3f9e2996fe808b2fe7a7641388eb8d81aefcb6ffc32c`.

## Restored Failure Boundary

The expectation-only RED retained the old raw snapshot arrangement and required exact `MissingCoin` for txid-nine/vout-zero. The actual named test returned `UntrustedWorkLineage`, exposing the original generic-error false pass. The saved RED source and receipt establish a runtime assertion failure, with one test discovered.

GREEN consumes the genuinely accepted seeded manager into its original store and same core. `CoinsFlush::prepare_current_best_block` followed by normal closed `flush` moves actual effective coins into that core's parent and empties its overlay. `ManagedChainstate::from_chainstate` rewraps that same core with its original lifecycle. Live origin, earned work and binding under the unchanged complete maturity-one parameters are asserted.

The original first input is the actual parent-only genesis coin; the exact second input is absent. Two-input order is checked explicitly. The existing `build_transaction_context` loop visits inputs in order through `peek_coin`, so the exact second `MissingCoin` establishes traversal of the available first parent coin. Preparation runs once on the original child/body, height, coinbase, spend, P2SH flags and parameters.

Cache occupancy is zero before lookup, after parent observation and after rejection. The original no-warming and spendable-parent assertions remain meaningful. Snapshot, tip and coins-best-block observations remain unchanged. Snapshots serve only as observations; they do not reconstruct authority. No mutable-coin import, normalization or production observer was introduced.

## Executed Evidence

All Cargo work ran serially through the command-timings wrapper with Bun 1.3.9 and Rust 1.94.1. Exact argv, timestamps, durations and complete log hashes are recorded in `.local/open-bitcoin-dev/phase165-plan30-red-commands.json` and `.local/open-bitcoin-dev/phase165-plan30-green-commands.json`.

| Actual command                            | Result                                  | Rounded duration |
| ----------------------------------------- | --------------------------------------- | ---------------: |
| Exact target list                         | One named test; exit 0                  |             10 s |
| Expectation-only target RED               | 0 passed, 1 failed, 0 ignored; exit 101 |        Under 1 s |
| Same target GREEN                         | 1 passed, 0 failed, 0 ignored; exit 0   |             10 s |
| Complete `chainstate::tests` module       | 14 passed, 0 failed, 0 ignored; exit 0  |        Under 1 s |
| Workspace format check                    | Exit 0                                  |              2 s |
| Node all-target/all-feature strict Clippy | Exit 0                                  |              8 s |
| Node all-target/all-feature build         | Exit 0                                  |        Under 1 s |

The full module comprises eleven direct tests and three existing child tests. The named GREEN overlaps that module; counts are not summed as independent coverage.

## Independent Review and Exact Activation

The frozen pre-pin packet `.local/open-bitcoin-dev/phase165-plan30-prepin-review.json`, SHA `23c43eeee05e08d5fe4ca57836f4b0040dc9f4c3e3c3f7cf4b753a1b649440c7`, contains all five source bodies, nine unchanged dependencies, exact inverses and runtime receipts. Independent review is CLEAN in `.planning/phases/165-shared-exact-work-authority/165-PRE-REVIEW-INTEGRATION30.md`; the final formatted report SHA is `f0a6dedc823a50c6d1a41cc6dd7a6030810787aaed4591460258617309da973f`.

Twenty-six loaded-source controls protect the same consumed core, genuine closed flush, origin and full parameter binding, real ordered inputs, exact second error, zero occupancy, parent retention, unchanged state and target test registration. Each actual anchor is unique within its source; the two declaration controls deliberately share an anchor with different replacements. The existing strict loop receives only three same-line import/spread/condition changes. These are source-policy mutations, not executed Rust mutants.

The inert proposal `.local/open-bitcoin-dev/phase165-plan30-proposal.json`, SHA `4548c5c83f8adf440a3f297c646c8738711289d13f075097c1e9197aa63b69fc`, adds one SOURCE row at index 157 and one TEST row at index 87 with eleven direct tests. Only that current TEST row changes the existing old164 data leaf; its unrelated declarations, types and helpers remain exact. All old164 SOURCE_FILES, TEST_MODULES and EDGES remain unchanged.

Actual arrays become 158 SOURCE owners, 88 TEST owners and 277 INPUT paths. The sole new input is the Rust target at index 263; all 276 prior paths retain their order. After rows were applied with the old pin, a fresh real CLI exited 1 at the intended inventory refusal before fixed `loadInputs` reads. This is distinct from import-time I/O denial. Only then was the inventory literal renewed to `9d2fb8f19b943757a968574d6997b65350d29a8a20e4d57b37f834d468f3ae8c`.

Root independently reversed all three complete pin-bearing files to their reviewed pre-pin bodies and checked actual imported arrays. `.local/open-bitcoin-dev/phase165-plan30-postpin-review.json` preserves the full five-file result. Root acceptance `.local/open-bitcoin-dev/phase165-plan30-root-postpin-acceptance.json` has SHA `1ffecaf7b107d890bbd103a1bb7b70197e4c6805b0a2254e1d4773badd7beb00`.

Final physical line counts are Rust 613, current165 entry 269, contracts 618, current165 test 627 and old164 TEST data leaf 388. No file exceeds 628 lines.

## Focused Verification and Complete Read Closure

Root approved the actual collector and nine-command driver before execution. The collector preserves the prior algorithms and all 658 completed31 paths. The new freeze `.local/open-bitcoin-dev/phase165-plan30-focused-freeze.json`, SHA `f762bf86fc0b7fd74f37a38f2cd753aeb3308cac783c025f0ee3cd7cd7c81fde`, contains 709 paths: 707 present and the same two optional absences. Fifty-one explicit additions include actual source dependencies and Plan30 review, proof and command provenance.

The static/test/checker/CLI/factory/observed dynamic boundary includes all five owners and nine explicit unchanged dependencies. The 36-module union, 14-module CLI set, every entry closure and factory initializer remain exact. The only fixed-list change is the one current165 input. This is the declared raw read boundary, not a complete compiler or system dependency closure.

The current derived corpus retains 171 entries and contains 4,575,939 bytes, SHA `d6cdb3b021f68072ad990538d601bf78cec3d0cffc4db30d6eb037066a0b1a15`. Its measured difference from completed31 reflects current reviewed bytes and coordinator progress metadata. Immutable historical manifests, recipes, snapshots, receipts and attribution targets were not regenerated.

| Actual focused command                 | Result                                                                    |
| -------------------------------------- | ------------------------------------------------------------------------- |
| Planned preparation/inventory selector | 37 passed, 0 failed, no ignored matches; 264 expectations; 1,929 filtered |
| Live current164                        | Exit 0; 143 fixed Rust inputs and 606 reviewed source edges               |
| Live current165                        | Exit 0; 158 source owners and 88 executable test owners                   |
| Fast current158 and current162         | Both exit 0                                                               |
| Breadcrumb checker                     | Exit 0                                                                    |
| Managed standards checker              | Exit 0; no findings or exceptions                                         |
| Five-file physical cap check           | Exit 0; all within 628                                                    |
| Whitespace check                       | Exit 0                                                                    |

The 37 selected cases comprise all 26 new preparation controls, six existing enabled-owner controls and five existing inventory controls. Full selected names, argv, actual exits, timings and log hashes are preserved in `.local/open-bitcoin-dev/phase165-plan30-focused-closing-proof.json`, SHA `e1ee7e6dc43df1b2303060b91c52677914ab025d287fdbc99001110081629084`. The nine-command journal SHA is `318bbc916e9be6ced89d6c4a17d354527b88d397f3106d28227b7881b6b5bf3f`.

The driver checked all frozen entries before every command and after completion. The collector then closed all 709 entries unchanged; closing receipt SHA is `0b6ea9434013138993626c9a057a4ad966e1ef3162bb7ae016374862a667d0c8`. Root independently authenticated every entry and all nine successful logs in `.local/open-bitcoin-dev/phase165-plan30-root-focused-acceptance.json`, SHA `6ee24e38ecb7e73b115a5f1e49a9e2b8f7020e0a4f2caad4dedcf214bd09aa29`.

## Adjustments and Remaining Gates

The first external selected-name extraction assumed every Bun pass row had a timing suffix. One valid inventory row omitted it, so that receipt parser assertion stopped. Splitting actual pass markers with optional timing suffixes preserved all 37 names. The diagnostic and correction are retained in the external closing proof; no test rerun or frozen source change occurred.

WR-01 is closed by this bounded regression proof and actual affected verification. There are no new stubs or security-relevant production surfaces. This plan intentionally uses proportional focused verification and does not repeat full guard matrices. Final Plan14 still owns both complete current matrices, the default repo-native verifier, fresh coverage, Bazel, independent review, security/formal/lifecycle acceptance and Git finalization. Later 166–168 qualification remains outside this result.

Task commits, shared metadata, staging and push remain coordinator-deferred. No per-plan commit was created or claimed.

## Self-Check: PASSED

The Summary and all five declared owners exist. All 709 frozen raw entries retain their exact canonical paths, presence, lengths and hashes. Native and focused receipt logs match their recorded full hashes. Both tasks have actual scoped evidence and independent root acceptance. Task commits remain intentionally deferred to coordinator finalization.
