---
phase: 167-candidate-history-validation-rules
plan: "05"
subsystem: consensus
tags: [rust, bip34, bip68, bip113, witness, sigops]
requires:
  - phase: 167-01
    provides: Immutable selected network parameters
  - phase: 167-03
    provides: Checked CandidateHistory and exact CandidateRules
  - phase: 167-04
    provides: Shared contextual header rules and original admission clock contract
provides:
  - Strict selected CScript height encoding and branch-derived transaction contexts
  - One checked aggregate block sigop budget callable before coins disappear
  - Shared source-correct malformed-script and raw-push observational counts
affects: [167-06, 167-08, 167-09, 167-16, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Pure structural context transformations, Atomic checked sigop accumulation, Shared syntax-only decoding]
key-files:
  created:
    - packages/open-bitcoin-consensus/src/block/phase167_tests.rs
    - packages/open-bitcoin-consensus/src/block/sigops.rs
    - packages/open-bitcoin-consensus/src/block/sigops/tests.rs
  modified:
    - packages/open-bitcoin-consensus/src/block.rs
    - packages/open-bitcoin-consensus/src/context.rs
    - packages/open-bitcoin-consensus/src/script/encoding.rs
    - packages/open-bitcoin-consensus/src/script/sigops.rs
    - packages/open-bitcoin-consensus/src/script/tests/verify_script_matches_knots_equal_vector.rs
    - packages/open-bitcoin-mempool/src/policy.rs
key-decisions:
  - "Selected body checks require exact CScript integer prefixes; explicit generic fixture checks retain legacy small-height byte pushes through the same contextual rule function."
  - "Creating MTP is derived from the candidate ancestor only when the pinned BIP68 condition consumes it; saved scalar metadata never substitutes for required history."
  - "The incremental block budget and existing contextual validator reuse one flag-sensitive cost function and existing public script counters."
  - "Counting shares syntax-only GetOp decoding with the interpreter while the execution wrapper retains its element-size limit and errors."
patterns-established:
  - "Public structural helpers and counters confer no canonical lineage or accepted-coins authority."
requirements-completed: [CVLD-02, CVLD-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T18:47:33Z"
commits: false
duration: 17min
completed: 2026-10-21
---

# Phase 167 Plan 05: Contextual block rules and shared sigop budget Summary

**Selected body and transaction checks now derive branch locks, exact BIP34 prefixes and witness activation from CandidateHistory, with one checked flag-sensitive block sigop accumulator and source-correct shared counting.**

Source-checkout baseline, not a task commit: `1b472fb225a290df5cdb06eeea2dc67c624c648e`, independently observed before implementation and after checks. This existing main-repository object identifies the checkout; commits:false remains in force.

## Files Created/Modified

- Modified: `packages/open-bitcoin-consensus/src/block.rs` — one contextual body-rule function, strict selected wrapper, height encoding, shared aggregate consumption and owned test registration.
- Modified: `packages/open-bitcoin-consensus/src/context.rs` — branch-derived transaction carrier and checked sequence-time arithmetic.
- Created: `packages/open-bitcoin-consensus/src/block/phase167_tests.rs` — twenty contextual positive/refusal tests.
- Created: `packages/open-bitcoin-consensus/src/block/sigops.rs` — incremental pure budget and the shared legacy/split cost transformations.
- Created: `packages/open-bitcoin-consensus/src/block/sigops/tests.rs` — seventeen aggregate, flags, scaling, malformed/push and execution-limit controls.
- Modified: `packages/open-bitcoin-consensus/src/script/sigops.rs` — source-correct count-so-far and final raw GetOp data, shared by existing consumers.
- Modified: `packages/open-bitcoin-consensus/src/script/encoding.rs` — minimal syntax-only decoding seam behind the unchanged execution limit.
- Modified: `packages/open-bitcoin-consensus/src/script/tests/verify_script_matches_knots_equal_vector.rs` — only the contradictory observational-count expectation; execution refusals retained.
- Modified: `packages/open-bitcoin-mempool/src/policy.rs` — only the four malformed-counter test expectations and their test name; production policy unchanged.

The existing witness implementation was read and reused without modification. Its last-matching-output, exact nonce shape, zero coinbase leaf and commitment checks already matched the pin. The existing contextual weight regression remains passing. Three new Rust files have pinned-source comments; Plan19 owns manifest registration.

## Completed tasks

1. Correct BIP34, branch locks and witness commitments. Strict selected checking uses OP_1 through OP_16, then minimal signed-number pushes. Tests cover1–17,127/128,32767/32768, wrong/nonminimal prefixes and coinbase lengths1/2/100/101. One common body-rule function retains finality, height, witness and weight order for both explicit fixture and selected paths. The selected wrapper binds exact header bytes and invokes shared selected PoW/version/time/difficulty/expiry checks before contextual body rules. Immutable CandidateRules supplies exact consensus parameters and flags. Finality uses candidate time before CSV and parent MTP after, with strict inequality and final-sequence override. BIP68 applies only to version>=2 with CSV; disabled/type/mask/512-second granularity and equality controls pass. Consumed creating time is MTP at max(created_height-1,0) on the candidate branch, including same-block parent MTP. Missing consumed ancestry refuses; unused distant creating history is not spuriously required. Checked arithmetic refuses extreme fixture overflow. New branch-disagreement, foreign-candidate, activation/nonce/malformed/last-output/trailing-data and zero-leaf tests pass.
2. Expose one flag-sensitive aggregate sigop budget. The budget includes coinbase and every transaction, counts legacy*4, P2SH*4 only with P2SH, and unscaled witness only with WITNESS. Exactly80000 passes and80001 refuses. Missing/extra noncoinbase coins refuse even when flags are NONE; overflow and overbudget additions leave the counter unchanged. The existing contextual validator calls the same accumulator as the later overlay owner can call before spending coins. Existing script counters now stop at malformed GetOp and return prior counts; accurate multisig recognizes only immediate OP_1..OP_16 predecessors. OP_0 was already correct and is retained as a passing control, with no fabricated fix or RED claim. P2SH and nested witness use the final raw pushed bytes, clearing them for small/reserved opcodes and refusing malformed outer suffixes. P2WSH counts its raw witness script, including count-so-far. Complete521-byte pushes are traversed for counting while the same script still fails execution with PushSize(521). Malformed execution still fails independently.

## API and Plan06 handoff

block::check_block_with_candidate_history(block,history,shell_observed_now) returns a pure BlockValidationError result. It binds the exact candidate header, selected version/time/difficulty/PoW checks and strict CScript coinbase-height behavior. Canonical consumers must obtain the original admission clock from the actual Plan04 header owner, not substitute candidate.time or a new caller clock. The generic check_block_contextual and validate_block_with_context remain explicit fixture APIs; their legacy small-height byte-push compatibility is intentionally retained. Canonical consumers must use the selected wrapper, not copied mutable params as authority.

TransactionValidationContext::from_candidate_history(transaction,inputs,history) returns the existing carrier with exact selected flags/params, spend height, finality cutoff and parent MTP. It requires one supplied coin context per input and refuses future creation height. It replaces stored creating-MTP metadata only for CSV-enforced version>=2, non-disabled, time-type BIP68 inputs, deriving the complete required candidate-ancestor window. Input ordering, actual coin identity, header membership, branch ownership and owner generation remain the coins owner's responsibility. A returned carrier, including one from IndependentlySeeded history, is data rather than an acceptance receipt.

block::sigops::BlockSigopBudget exposes new(), cost() and add_transaction(transaction,spent_outputs,flags). The actual overlay owner must start one budget per block, account the coinbase, resolve actual input coins in transaction order, call add_transaction before those inputs disappear, and pass history.rules().script_flags(). The public transaction_sigop_cost helper is the same checked pure cost transformation. No interpreter bypass, canonical support flag or durable accepted-state owner was introduced. The existing generic validator supplies each transaction context's explicit fixture flags; canonical flags must come from CandidateRules. No policy-only flags are added by the derived constructor.

## Executed commands and actual outcomes

All Cargo invocations used the pinned Bun PATH prefix and ran serially through scripts/command-timings.ts with the existing target/cooperative lock. Logs are retained under `.local/open-bitcoin-dev/phase167/plan05/`. The observed Rust creation/RED interval begins2026-10-21T18:31:02Z; generated_at records the final completed-check observation above. Duration rounds that interval to17minutes.

| Command after required PATH prefix | Actual outcome | Log |
| --- | --- | --- |
| `bun run scripts/command-timings.ts run --key phase167-05-task1 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus phase167_block_context` | Actual RED2 discovered:1 passed/1 failed at height1. First GREEN attempt compile exit101 from slice comparison after helper extraction; replaced with starts_with. Next11 discovered:10 passed/1 synthetic setup failure because the mainnet test omitted its retarget anchor. Test history extended to include it. GREEN17/17, then19/19 after length/unused-history controls, then20/20 after branch disagreement and common-rule simplification. | task1-red.log; task1-green.log; task1-green-after-prefix.log; task1-green-complete.log; task1-green-final.log; task1-green-after-simplification.log |
| `bun run scripts/command-timings.ts run --key phase167-05-task2 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus phase167_sigops` | Actual initial RED3:1 passed/2 failed for P2SH-disabled and malformed counting. Expanded RED17:10 passed/7 failed for malformed legacy/redeem/witness, final raw data and521-byte pushes. Post-amendment GREEN17/17. Final17/17 includes additional paired execution-limit/malformed controls. | task2-red.log; task2-expanded-red.log; task2-green.log; task2-green-final-controls.log |
| `bun run scripts/command-timings.ts run --key phase167-05-shared-counter-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus -p open-bitcoin-mempool` |733 passed:334 consensus unit,13 consensus integration,4 consensus doctests,371 mempool unit,5 mempool integration,6 mempool doctests. Existing legacy fixtures, weight, execution and mempool behavior all passed. | shared-counter-regression.log |
| `bun run scripts/command-timings.ts run --key phase167-05-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-consensus -p open-bitcoin-mempool --all-targets --all-features -- -D warnings` | Exit0 without warnings. Initial run97seconds; final run after the two added test-only controls exit0 in3.58seconds. | clippy.log; clippy-final.log |
| Scoped rustfmt edition2024 with skip_children=true check of the nine changed Rust files | Exit0; scoped write/check preserved other owners. | rustfmt-check.log; rustfmt-check-final.log |
| git diff --check and owned diff review | Exit0; reviewed tracked changes and complete new files. Mempool production code is unchanged. | diff-check.log; diff-check-final.log |
| gsd-tools verify key-links for Plan05 | Initial3/5 reflected the inaccurate direct-private edge and not-yet-implemented decoder. Root removed the conceptual edge; final actual call chain4/4 passed. | key-links-initial.json; key-links-final.json |

The733 regression preceded only the final two assertions added to existing focused sigop tests; final17/17, final Clippy and formatting cover those assertions. No production source changed after the full regression. Zero-test focused integration binaries are retained but not counted as behavioral success. All37 new named focused tests were discovered and passed; the full regression also ran them. The quiet Clippy interval retained process identity/liveness in clippy-liveness.log and was polled within60seconds; no job was killed, target copied or overlapping Cargo command started. All failed outcomes above remain attributable. No auth gate occurred.

## Source and evidence

Pinned Knots source HEAD was independently observed at a9aee730466ac67d35a3c03ee24676be5e045878. Source-derived expectations use ContextualCheckBlock, CheckWitnessMalleation, GetBlockScriptFlags, CalculateSequenceLocks/EvaluateSequenceLocks, GetTransactionSigOpCost, GetScriptOp/GetSigOpCount and WitnessSigOps/CountWitnessSigOps. Source SHA256 values are retained in source-digests.log: validation5bb31c7417ef3fe06a2afefee8a1c32607aed737fdd623368cdc4be72d23d495; tx_verify8fbc9bde843c2cfb0577ae7d179ad11f928b09bf8b0fc3e43a9ede7dc34a7370; script.cpp4c567d48d43a066d68ddd3d129bc39641db54be17faba0b09abf18fa71bf9c04; script.h ebd3aa6de49ac597957923328bca3abc8697e63f2b0c3b06d260f6ebdd89016c; interpreter6e7367740f8d8ecc8dca4ec129d66782d09fa5733acc96469f367267fb42cc60.

Synthetic historical branches remain conditional structural/source-formula evidence. Mined regtest body probes exercise pure checks, not actual canonical coins ownership. No Plan02 partial corpus/acquisition files were consumed or changed. These tests do not establish historical serialized durable execution, actual pinned-daemon differential qualification or whole-chain acceptance.

AGENTS.md, Bright Builds sidecar/overrides and architecture/code-shape/testing/verification/Rust guidance informed the implementation. Both active lesson files were loaded completely:8856bytes,2953 conservative tokens. No project skills were present. Execute-plan, summary, TDD/checkpoint and execution-reasoning contracts were applied within the explicit root-owned saving/state exception.

## Decisions and simplification

One common contextual-rule function avoids competing body validators; its private height-prefix strategy preserves the explicit fixture contract while the selected path remains strict. One pure accumulator reuses existing script wrappers rather than copying counters into chainstate. One syntax decoder serves observational counting and the existing execution wrapper; neither the block budget nor a new parser duplicates it. No dependency, crate, runtime I/O, production panic/unwrap site or authority owner was added. All nine touched source/test files remain below the managed628-line trigger.

## Deviations from Plan

Root-reviewed167-05-AMENDMENT extends Task2 to the existing shared script counter, minimal encoding seam and two exact conflicting test owners. Source mismatches were reported before any unowned edit; implementation waited for the independent0-blocker/0-warning release. This corrects the root counting behavior instead of duplicating or suppressing it. The existing consensus test now expects the pinned zero count for an initial malformed push while preserving all execution errors; the four existing mempool cases retain their inputs and exact zero-cost expectations. No production mempool policy was changed, test ignored/deleted, or missing context replaced by defaults. Initial setup errors, genuine RED failures and metadata-link outcomes are recorded above. No other scope expansion occurred.

## Task commits

commits:false. No staging, task/metadata commits, push, hook bypass, worktree or agent delegation was performed. Root owns STATE/ROADMAP/requirements/config/AGENTS, the amendment and final normal-hook saving. Other owners' source, acquisition, guards and breadcrumb manifest were preserved.

## Known stubs and threat scan

No unresolved stubs remain. Empty witness stacks and missing ancestry are explicit source states/refusals, not UI placeholders. New helpers transform structural data and do not mint accepted authority. No endpoint, auth path, filesystem access or durable schema was introduced. T-167-05-01/02 are covered by strict heights, branch locks, witness/flags/budget/malformed controls; T-167-05-03 by pinned source hashes, retained failures and evidence-class limits. The shared untrusted-byte decoder retains syntax bounds and execution's size checks.

## Remaining phase obligations

Requirements above record bounded Plan05 contributions, not phase completion. Plan06 must wire the shared checks/counter to real coins and actual earned header ownership before acceptance. Accepted-tip/coins/work/BASIC/durable refusal preservation remains the actual owner integration test obligation; no such state was mutated by these pure tests. Plan19 registers the three new Rust breadcrumb entries and current guards. Plan20 owns full native/fresh coverage/Bazel and independent final code/security/goal/lifecycle acceptance. Unsupported selectors, Signet challenge checks, product/network defaults and actual-daemon claims remain outside this completed plan.

## Self-Check: PASSED

All nine changed Rust files and this summary exist. New focused tests20/20 and17/17, affected complete regression733/733, final scoped Clippy, formatting and diff checks passed. All four concrete key links pass4/4. No task commit is claimed or fabricated; the labelled source-checkout baseline is an existing Git object. Shared state and other owners' changes were preserved.
