---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T18:37:27Z"
---

# Plan167-05 bounded ownership amendment

This records a planning amendment, not implementation or passed execution evidence. Original Plan05 generated_at2026-10-21T16:55:59Z, lifecycle167-2026-10-21T16-38-23/yolo, wave3 and dependency167-03 remain intact. Local AGENTS guidance, Bright Builds sidecar/overrides and architecture/code-shape/testing/verification/Rust standards informed this amendment; both active lesson files were read fully (8856bytes,2953 conservative tokens). No new dependency or crate is needed.

## Exact implementation ownership

Root authorized these exact additions to Plan05 Task2 ownership alongside original `block.rs`,`block/sigops.rs` and `block/sigops/tests.rs`:

- `packages/open-bitcoin-consensus/src/script/sigops.rs`: repair existing shared counters and raw final-push selection.
- `packages/open-bitcoin-consensus/src/script/encoding.rs`: minimal shared syntax-only GetOp decoding seam; retain existing execution-limit wrapper,no broad encoding refactor.
- `packages/open-bitcoin-consensus/src/script/tests/verify_script_matches_knots_equal_vector.rs`: test-only inaccurate count expectation described below.
- `packages/open-bitcoin-mempool/src/policy.rs`: test-only counter expectations described below; production policy unchanged.

Public script count helper `Result<usize,ScriptError>` signatures remain compatible. New source-derived regression tests go in already owned `block/sigops/tests.rs`. Share one underlying syntax decoder with explicit counting-versus-execution limits; no second block parser or VM rewrite. The existing OP_0 accurate-multisig case already counts20 and remains a passing control; no fix/RED is claimed for it. Task1 only clarifies which BIP68 inputs consume creating-ancestor MTP; it still requires actual candidate-branch history whenever consumed. These are bounded extensions of the existing Task2 concern,two existing assertion-owner corrections rather than new subsystems.

Original-plan snapshot is `.local/open-bitcoin-dev/phase167/plan05-amendment/before-PLAN.md`,SHA256 `95100f785e587f65ffd87d91a02f303d447f1ae85b06b917e8bb65f071ef51b2`; planner independently matched that digest before finalizing. It remains unchanged and permits exact scope comparison. No source edits are authorized before root checker acceptance of this revision.

The planner edits only167-05-PLAN.md,this amendment and targeted167-DECISION-COVERAGE.md references. It does not edit implementation, summaries, STATE/ROADMAP/config/AGENTS, acquisition scripts/data, guards, manifests or deadlines. No builds,tests,Git saving or agent spawning occur in this amendment task. Root independently checks the revision before releasing shared-source edits.

## Pinned source and exact contracts

All anchors are local `packages/bitcoin-knots/src/` at approved pin `a9aee730466ac67d35a3c03ee24676be5e045878`; these are source-derived expectations, not daemon observations.

| Anchor | Required behavior and regression |
| --- | --- |
| `script/script.cpp:160–182`,GetSigOpCount(bool) | Malformed GetOp ends counting and returns prior count: `[ac,4c]`→1,`[01]`→0. Accurate CHECKMULTISIG/VERIFY accepts only preceding OP_1..OP_16: OP_0→20 (already correct; passing control),OP_1→1,OP_16→16; other predecessors→20. Legacy multisig always20. |
| `script/script.cpp:184–205`,GetSigOpCount(scriptSig) | P2SH scans whole scriptSig,returning0 on malformed GetOp or opcode>OP_16; count the final raw data as accurate redeem script. `[01,ac]`→1,`[01,ac,51]`→0; malformed outer suffix→0; intact push of `[ac,4c]`→1. |
| `script/script.cpp:383–433`,GetScriptOp | Raw data clears for every opcode; OP_1..OP_16 are not interpreted as executable pushes. Complete payloads over520bytes parse: PUSHDATA2(521)+payload+CHECKSIG→1. Share syntax-only decoding beneath existing execution-limit wrapper; same bytes passed to eval_script must reject PushSize(521). |
| `script/interpreter.cpp:2125–2170`,WitnessSigOps/CountWitnessSigOps | WITNESS gates contribution; native v0 keyhash counts1; v0 scripthash counts final witness script accurately,including malformed count-so-far. Nested witness extracts last raw GetOp data using the same selection semantics as P2SH. |
| `consensus/tx_verify.cpp:112–166`,transaction cost helpers;`validation.cpp:2936–2944`,ConnectBlock | Legacy always*4; P2SH only flag-enabled and noncoinbase*4; witness only flag-enabled and noncoinbase unscaled; aggregate every transaction including coinbase,refuse only above80000. All original exact80000/80001,flags,scaling,overflow and missing-output tests remain. |
| `consensus/tx_verify.cpp:38–96`,CalculateSequenceLocks | Creating-ancestor MTP is consumed only for CSV-enforced version>=2,input not disabled,time-type sequence. Derive ancestor max(created_height-1,0) on actual candidate branch; same-block input uses parentMTP. Missing consumed history refuses; absent unused history is not an added gate. |

Affected consumers: public `script::{count_legacy_sigops,count_p2sh_sigops,count_witness_sigops}` wrappers,contextual block budget and `open-bitcoin-mempool/src/policy.rs::transaction_sigops_cost`. No unrelated mempool scaling/policy rewrite is in scope. Script execution continues to reject malformed pushes independently of these observational counts.

## Identified existing test-owner conflicts

Root initially authorized only the shared source addition and original Plan05 files. The following existing tests contradict pinned count-so-far behavior and were reported before adding ownership or editing them; root then explicitly authorized only these test-scope corrections:

| Existing owner | Exact conflict | Required bounded correction |
| --- | --- | --- |
| `packages/open-bitcoin-consensus/src/script/tests/verify_script_matches_knots_equal_vector.rs:405` | `eval_script_rejects_opcount_and_pushdata_errors` expects count_legacy_sigops(`[01]`) to return TruncatedPushData. | Replace only that counter expectation with exact Ok(0); retain all eval_script opcode/pushdata errors. Name/comment the distinction clearly. |
| `packages/open-bitcoin-mempool/src/policy.rs:371–426` | `transaction_sigops_cost_maps_script_parsing_errors_from_all_sources` expects malformed legacy input,legacy output,P2SH redeem and P2WSH script counters all to error. | Preserve all four source cases and assert exact Ok(0) costs; add nonzero-prefix count evidence in owned new regressions. Keep context-length and independent standardness/script-execution rejection controls. No production-policy changes. |

Ownership resolution is explicit and bounded as above; no other existing assertion or production-policy change is authorized. `script/tests/witness_and_sigop_helpers_are_covered.rs` requires no known expectation change and remains read-only. Do not delete/ignore tests,mask failures or create a compatibility path that preserves incorrect counter semantics. Preserve actual script rejection: all existing eval_script parser errors/520byte limits stay intact; add paired521byte counting/execution evidence and malformed-prefix count-so-far in owned new regressions. Shared-counter acceptance requires affected consensus and mempool regressions to pass with all four policy source cases retained.

## Execution verification contract

Plan05 retains its focused `phase167_sigops` run and adds serial timing-wrapper regression command `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus -p open-bitcoin-mempool`,with pinned Bun PATH and normal cooperative lock. Record actual discovery counts,RED/GREEN and all failures/retries. These commands are requirements for the executor; this planner did not run them. Root retains final native/coverage/Bazel/security/goal/lifecycle gates and all Git saving ownership.

Planner frontmatter validation and plan-structure validation passed: two tasks,no errors or warnings. The original-plan snapshot digest matched root's recorded value. Added source/test owners occur in no other phase167 plan,so no existing wave/dependency edge changes are required. These are planning checks only; independent root review still precedes newly owned source edits.
