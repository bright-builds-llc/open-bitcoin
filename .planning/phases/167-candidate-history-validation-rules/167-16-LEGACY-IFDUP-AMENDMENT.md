---
phase: 167-candidate-history-validation-rules
plan: "16"
type: integration-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-27T21:50:31Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 16 legacy `OP_IFDUP` execution

After the checked legacy sighash repair, the exact Plan 16 Task 1 command passed mainnet block 170060 and reached a second genuine managed-script RED in pinned mainnet block 692261: transaction index 1252, input 0, P2SH/P2WSH witness script opcode `0x73` returned `UnsupportedOpcode`. Pinned Knots `script/interpreter.cpp:765-773` defines `OP_IFDUP`: require one stack item, duplicate the top item only when `CastToBool` is true. This is valid historical script behavior, not a fixture, deployment or selected-flag change. Pinned script test vectors also cover false, true and empty-stack behavior.

Root owns the narrow cross-plan source repair in `packages/open-bitcoin-consensus/src/script/opcodes.rs`, `packages/open-bitcoin-consensus/src/script/legacy.rs` and `packages/open-bitcoin-consensus/src/script/tests/verify_script_matches_knots_equal_vector.rs`. Add the opcode constant and one legacy evaluator arm using existing `cast_to_bool` and `push_stack`, preserving conditional skipping, opcode counting and stack limits. Add test-first focused cases for empty, false including negative zero, true arbitrary bytes, a skipped conditional branch and overflow as appropriate. Do not change the historical fixture, selected flags, signature engine or the Plan 16 Task 1 executor's seven owned files.

Capture the three original whole bodies before editing; retain the exact managed RED separately from setup/compiler stalls. Run focused consensus RED/GREEN and relevant legacy-script regressions through the cooperative timing wrapper, then the Plan 16 executor reruns its literal original Task 1 command on final source. Root's three files are separate protected external owners in the final causal source/native frame and inverse. The actual consumed-flag/interpreter sensitivity inverse remains a separate Plan 16 obligation; this amendment does not authorize an inert forwarded-flag mutation or an unreviewed live `apply.rs` edit. Independent review, normal formatted source fit, Plan 19 breadcrumbs, Plan 20 full native/coverage/Bazel/security/goal/lifecycle checks, normal-hook Git finalization and STOP before Phase 168 remain required.
