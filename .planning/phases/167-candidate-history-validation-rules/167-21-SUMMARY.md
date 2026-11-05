---
phase: 167-candidate-history-validation-rules
plan: "21"
subsystem: consensus
tags: [rust, script, cltv, csv, candidate-history]
requires:
  - phase: 167-06
    provides: Accepted context task and preserved original locktime implementation/evidence
  - phase: 167-05
    provides: Branch-derived transaction input and precomputation interfaces
provides:
  - Pinned pure CLTV/CSV execution with signed five-byte operands and unchanged four-byte callers
  - Registered funded-input interpreter boundary probes, separate from canonical stage acceptance
affects: [167-23, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Pure transaction checks, Non-consuming bounded signed operands, Interpreter-only funded probes]
key-files:
  created:
    - packages/open-bitcoin-consensus/src/script/locktime.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/phase167_locktime.rs
  modified:
    - packages/open-bitcoin-consensus/src/script.rs
    - packages/open-bitcoin-consensus/src/script/legacy.rs
    - packages/open-bitcoin-consensus/src/script/opcodes.rs
    - packages/open-bitcoin-consensus/src/script/stack.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/phase167_scripts.rs
    - packages/open-bitcoin-chainstate/src/engine/tests.rs
key-decisions:
  - "Carry genuine original06 RED/GREEN forward without an attribution-only replay; preserve all five consensus source files byte-for-byte."
  - "Direct probes use actual funding coins and derived input context but confer no canonical stage acceptance."
  - "Only funded/spending/body gain test-only pub(super) visibility; canonical controls remain in the existing sibling for23."
patterns-established:
  - "CLTV/CSV peek a bounded signed operand; explicit flags govern numeric minimality and inactive NOP behavior."
requirements-completed: [CVLD-02, CVLD-03, CVLD-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T20:04:36Z"
commits: false
duration: 2min44s
completed: 2026-10-21
---

# Phase 167 Plan 21: Pure lock opcodes and funded interpreter boundaries Summary

**CLTV/CSV enforce pinned signed operands and actual transaction fields; six registered interpreter tests preserve the original boundaries and verify exact stack retention without promoting their evidence to canonical acceptance.**

## Files Created/Modified

- Carried modified: `packages/open-bitcoin-consensus/src/script.rs` — private module plus two typed errors and Display arms;158 lines.
- Carried modified: `packages/open-bitcoin-consensus/src/script/legacy.rs` — executed opcode dispatch through concrete locktime calls;536 lines.
- Carried modified: `packages/open-bitcoin-consensus/src/script/opcodes.rs` —0xb1/0xb2 constants;97 lines.
- Carried modified: `packages/open-bitcoin-consensus/src/script/stack.rs` — shared explicitly bounded decoder;163 lines.
- Carried created: `packages/open-bitcoin-consensus/src/script/locktime.rs` — pure CLTV/CSV signed operand and transaction checks;95 lines.
- Created: `packages/open-bitcoin-chainstate/src/engine/tests/phase167_locktime.rs` — four relocated tests, unchanged probe helper and two additional controls;345 lines.
- Modified: `packages/open-bitcoin-chainstate/src/engine/tests/phase167_scripts.rs` — removes only relocated direct probes/imports; exposes funded/spending/body as pub(super);328 lines.
- Modified: `packages/open-bitcoin-chainstate/src/engine/tests.rs` — adds only the new sibling registration relative to continuation-start bytes;314 lines.

Eight declared Rust owners encompass two created and six modified files across original execution and this continuation. This continuation created one Rust file and edited two existing Rust files. All five carried consensus files independently match the scope-amendment checkpoint SHA256 values exactly; they were reviewed and freshly verified, not reimplemented. No extra source owner was needed. All eight remain below629 lines. New test-file pinned-source comments are present;19 alone owns manifest registration.

Source checkout reference only, not a task commit: `1b472fb225a290df5cdb06eeea2dc67c624c648e`. Its existing commit object was checked. commits:false remains in force.

## Completed task and original obligation mapping

Task1 is complete within current21. Original06 Task2 remains composite21+23 under the checked execution-scope amendment and final19:53:38Z plan review. Original06 Task1 stays separately accepted in06.

| Original obligation | Current implementation/evidence | Boundary |
| --- | --- | --- |
| Signed five-byte CLTV/CSV, typed negative/unsatisfied errors and unchanged four-byte arithmetic/count callers | Private pure helper plus real executed legacy dispatch; existing decode_script_num delegates with4, only lock operands request5. Fresh6/6 focused tests and351 consensus regression cases. | Interpreter evidence; no independent stage authority. |
| CLTV threshold/types/equality/final input and operand bounds | Fixed500000000 split, signed i64 comparison against actual tx.lock_time, checked actual input index and nonfinal sequence; no height/MTP substitution or u32 narrowing. | Actual funded-output probes preserve the complete chain snapshot. |
| CSV version/disable/type/mask/ignored bits/checker order | Negative/size/minimality checked first; operand-disable precedes input access; otherwise actual version>=2 and input-disable clear. Both operands mask only type/low16 bits. | BIP68 age/512-second conversion remains a separate transaction rule. |
| Active empty/negative/six-byte and signed negative-zero/minimality controls | Exact InvalidStackOperation/NegativeLockTime/NumOverflow/UnsatisfiedLockTime assertions; nonminimal numeric encoding refuses with existing VerifyFailed only under explicit MINIMALDATA. | No unrelated arithmetic/push-policy retrofit. |
| Peek/non-consumption, inactive NOP2/3, skipped branches | OP_DROP-dependent existing cases retained; added EQUALVERIFY of the exact five-byte retained operand under MINIMALDATA. Inactive invalid/empty operands plus DISCOURAGE and invalid checker index succeed. Generic no-context eval/verify retain inactive NOP semantics. | Generic eval has no verify flags; active transaction checks use real context and checked indexes. |
| Canonical equality/positive/omission/refusal, rewards and all remaining actual coins obligations | Existing sibling source preserved after the precise move and visibility/import changes. Original historical logs remain unchanged. |23 owns actual canonical acceptance, pending controls and final complete affected regression. |

The probe obtains an actual accepted funding Coin, derives CandidateHistory and TransactionValidationContext from the actual branch, precomputes the real serialized transaction and calls public verify_input_script. Each probe asserts complete chain snapshot equality before/after, covering accepted chain/work/coins/undo. It never stages or absorbs the probed transaction. BASIC/runtime/durable authority is not asserted by these interpreter tests.

## Exact test relocation

Only the prefix phase167_scripts_ changed to phase167_locktime_ in these four names; each full old/new name is recorded in preservation.json under the log directory below:

| Old test name | New test name |
| --- | --- |
| phase167_scripts_cltv_funded_interpreter_transaction_boundaries | phase167_locktime_cltv_funded_interpreter_transaction_boundaries |
| phase167_scripts_csv_funded_interpreter_masks_and_checker_order | phase167_locktime_csv_funded_interpreter_masks_and_checker_order |
| phase167_scripts_lock_operands_signed_size_minimal_and_inactive_nops | phase167_locktime_lock_operands_signed_size_minimal_and_inactive_nops |
| phase167_scripts_lock_skipped_branches_and_four_byte_arithmetic_remain_distinct | phase167_locktime_lock_skipped_branches_and_four_byte_arithmetic_remain_distinct |

Independent token comparison confirms all four complete function bodies and the probe helper are preserved, accounting only for renamed prefixes, comments/formatting and expansion of the skipped-branch helper call into the identical seven consensus flags. The remaining canonical sibling source is preserved after exactly those removals/import changes and the three visibility changes. Registrar comparison confirms a single module addition. Additional tests are phase167_locktime_minimal_five_byte_operands_preserve_the_exact_stack_value and phase167_locktime_generic_no_context_interfaces_keep_inactive_nop_semantics.

## Executed commands and actual outcomes

All Cargo commands prepended PATH=/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:$PATH and ran serially through scripts/command-timings.ts, reusing the existing target and cooperative lock. Complete current logs are under `.local/open-bitcoin-dev/phase167/plan21/`.

| Command/check | Actual outcome | Receipt |
| --- | --- | --- |
| bun run scripts/command-timings.ts run --key phase167-21-locktime -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate phase167_locktime | Exit0;6 discovered/6 passed,0 ignored,405 filtered; separate parity integration binary has0 matching/3 filtered and is not counted. | locktime.log |
| bun run scripts/command-timings.ts run --key phase167-21-numeric-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --all-features | Exit0;334 unit+8 basic-filter integration+3 parity integration+2 taproot integration+4 doctests=351 passed,0 ignored/filtered. | numeric-regression.log |
| bun run scripts/command-timings.ts run --key phase167-21-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-consensus -p open-bitcoin-chainstate --all-targets --all-features -- -D warnings | Exit0; strict affected all-target/all-feature lint completed in58.20s. | clippy.log |
| rustfmt --edition2024 --config skip_children=true --check on the eight declared owners | Exit0 after scoped formatting; no child traversal or unrelated formatting. | rustfmt-check.log |
| git diff --check restricted to the eight owners and owned source/diff review | Exit0; complete new sibling and carried helper reviewed. | diff-check.log |
| gsd-tools verify key-links for current21 PLAN | all_verified:true;2/2 actual source links: executed helper calls and registered test sibling. | key-links.json |
| Independent checkpoint/preservation comparisons | Five consensus hashes exact; four relocated test bodies/probe preserved; canonical sibling retained; registrar only adds module. | preservation.json |

An initial focused run waited normally on the Cargo artifact lock. Root identified the live holder as Cursor rust-analyzer's workspace/all-targets cargo check, with observed progress through RPC/bench/CLI. Preflight had no active Cargo; no other executor job was started. Cargo serialized naturally, the full wait remains in locktime.log, and no IDE process/settings were altered or terminated. Resumable checks were polled within60seconds; no target copy or alternate target was created. All current commands passed on their first execution; no new behavioral RED was manufactured.

## Preserved original outcomes and failures

The complete original06 checkpoint, including19:35Z, both original amendments and06 summary were read. Current checked split controls allocation; the checkpoint's earlier proposed allocation of all Task2 to21 is historical. Original logs remain at .local/open-bitcoin-dev/phase167/plan06/, with the unchanged21-log amendment evidence copies and559938-byte manifest retained.

| Historical receipt | Exact interpretation retained |
| --- | --- |
| locktime-positive-red.log |1 discovered/1 missing-CLTV failure; loop stopped at b1, so no CSV claim from that run. |
| locktime-positive-pair-red.log |2 discovered/2 genuine missing-opcode failures in actual funded canonical positives, UnsupportedOpcode(b1/b2). These are not canonical flag-bypass REDs. |
| locktime-green-initial.log |11/11 canonical/script cases passed after implementation, including both active equality positives and canonical omission/refusal pairs. Historical acceptance only at that source point. |
| locktime-boundaries.log |15 discovered/14 passed/1 test setup failure: inherited OP_1 scriptSig meant the stack was not empty. UnsatisfiedLockTime was correct; explicit empty scriptSig fixed setup only. |
| locktime-boundaries-final.log |15/15 passed after corrected setup, before rewards were added. |
| rewards.log |Separate1/1 passed for149/150/299/300, actual10000sat fees and+1sat refusal. No combined16-case result is invented. |

Earlier retained history includes setup/import errors, non-distinguishing MTP-vector mistake, genuine missing-admission and corrected12-versus11-sample RED, coinbase-encoding setup errors, seven original Task2 behavioral REDs, initial unsupported opcode omission controls, Clippy failure/fix, and partial400unit+3integration+23doctest/21focused results with two explicit skips. Those remain failures or partial results as recorded in the original checkpoint/06 summary; none substitutes for final23 acceptance. No historical log was rewritten or copied over, and no auth gate occurred.

## Source, guidance and simplification

Pinned interpreter.cpp521–600 and1747–1830, script.h CScriptNum/NOP aliases, interpreter.h checker defaults and script_error.h define the checked semantics. Rechecked source SHA256 receipts are in source-digests.log; interpreter.cpp6e7367740f8d8ecc8dca4ec129d66782d09fa5733acc96469f367267fb42cc60 and script.h ebd3aa6de49ac597957923328bca3abc8697e63f2b0c3b06d260f6ebdd89016c match the earlier source evidence. No daemon was run and no differential result is claimed.

AGENTS.md Repo-Local Guidance, Bright Builds sidecar/overrides, standards index, architecture/code-shape/testing/verification/local-guidance and Rust rules informed execution. Both distinct active lesson files were measured/read:6898+1958=8856bytes,2300+653=2953 conservative tokens. Existing October19 audit baseline applies; no trigger or project skills directory was found. Execute-plan, execution-reasoning and summary-template contracts were applied within the explicit root-owned shared-state/saving exception.

Simplification retained one shared signed decoder and one pure lock helper; no second fixture, checker, production authority or public interface was introduced. The cohesive sibling reuses three existing test-only helpers. No new crate/dependency, runtime I/O, endpoint, auth path, filesystem boundary or durable schema was added. No unresolved stub was found. T-167-21-1 is covered by bounded signed parsing and exact flags/input/mask/order controls;T-167-21-2 by preserved assertions/history, unchanged snapshots and explicit interpreter evidence classification.

## Deviations from Plan

None. The checked continuation explicitly carries previously implemented source and genuine RED/GREEN without rerunning RED for plan attribution. Two additional focused tests strengthen the planned exact stack/minimal-five-byte and generic-interface obligations within the existing eight owners. No ownership expansion or policy change occurred.

## Remaining phase obligations and saving

23 retains canonical positive/omission/refusal and rewards tests unchanged. It must complete canonical CSV ignored-bit and operand-disable/version1 positives, maturity100 versus99-depth and zero genesis UTXOs, actual LOW_S absence with explicit policy rejection, MINIMALDATA/CLEANSTACK separation, consumed-only distant creating-MTP, missing/inconsistent saved metadata, finite4096 history refusal, same-block locks and replacement-branch disagreement. Selected flags, aggregate80000/80001, accepted-state preservation and original-clock/stage protections remain binding. Final combined nonzero discovery and full consensus+chainstate --all-features regression without exclusions, final strict affected Clippy and scoped format/diff/keylinks remain23 after its later edits. Current351 consensus regression is complete for21 but does not replace that later combined gate.

17 retains source review before guard renewal;19 registers the new Rust breadcrumb;20 retains complete native bash scripts/verify.sh, fresh coverage/Bazel and independent source/security/goal/lifecycle acceptance. No Plan02 acquisition/corpus/cache, apply/overlay/candidate_rules, guard/breadcrumb manifest, shared STATE/ROADMAP/requirements/config/todo/AGENTS or other owner's source was edited here. No staging, commits, push, hook bypass or worktree operation occurred. Root alone owns shared progress and final normal-hook saving; this executor stops before23 release. Requirements in frontmatter record this bounded contribution only.

## Performance attribution

One completed task/eight declared source owners. Measured continuation implementation/check interval starts at2026-10-21T20:01:52Z before editing and ends at2026-10-21T20:04:36Z after all required checks:164seconds(2min44s), excluding initial context loading and subsequent summary/receipt validation. This is separate from original06's shared approximately29minutes beginning19:05:48Z and spanning Task1, partial Task2, continuation and planning wait; that historical duration is not added again. generated_at records the successful final-check observation.

Summary validation at2026-10-21T20:06:19Z returned passed:true/errors:[], with all8/8 declared paths found and self_check:passed; frontmatter returned valid:true/missing:[]. Receipts are verify-summary.json and frontmatter.json in the current21 log directory. The validator's commits_exist:true resolves the explicitly labelled checkout object, not a newly created task commit. Summary/receipt preparation through that observation brings the measured interval from20:01:52Z to267seconds(4min27s); the implementation/check duration above remains separately attributable.

## Self-Check: PASSED

All eight declared Rust files and this summary exist. Six focused cases and351 complete consensus cases pass; strict scoped Clippy, formatting, whitespace and both real key links pass. Five carried consensus hashes and preserved test/probe/canonical-source comparisons are exact. The labelled pre-existing checkout commit object exists; no task commit is asserted. Root-owned shared metadata and other owners remain preserved.
