---
phase: 167-candidate-history-validation-rules
plan: "16"
type: integration-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-27T21:44:04Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 16 legacy sighash opcode boundary

The compiled Plan 16 Task 1 test reached the actual managed script kernel for pinned mainnet block 170060 and rejected transaction index 3, input 0. Its source-bound P2PKH prevout contains byte `0xab` **inside** a pushed HASH160 value. The selected historical flags are correctly `NONE`, and independent signature verification succeeds with the intact 25-byte script and legacy sighash `ab199c185cdcc0d20de9d4a3e613a64678313bb3f314c35db29f9e3e31a5e802`. Current `packages/open-bitcoin-consensus/src/sighash.rs` removes every `0xab` byte, producing a different 24-byte script and digest `da9d7f292cc92660ae1fabe30084d77726b87c065476367c88d783357628e7b4`. Pinned Knots `script/interpreter.cpp:1266-1285` skips only decoded `OP_CODESEPARATOR` opcodes and preserves push data.

Root owns this narrow cross-plan production repair in `packages/open-bitcoin-consensus/src/script.rs`, `packages/open-bitcoin-consensus/src/sighash.rs` and `packages/open-bitcoin-consensus/src/sighash/tests.rs`. Reuse the existing syntax-aware `script/encoding.rs::read_instruction_syntax` through a crate-visible script helper; preserve the entire malformed suffix after a failed decode, and remove only actual separator opcodes. Do not change signature validation, historical fixture bytes, selected flags, mempool behavior or the Plan 16 Task 1 executor's seven owned files. Add a test-first focused regression with `0xab` inside pushed data and a real separator opcode; bind the source-derived tx3 digest where feasible.

Capture all three original whole bodies before editing. Retain the actual managed Task 1 RED separately from the earlier loader-order setup failure and the diagnosed default-target directory-read stall. Run the focused consensus test through the cooperative timing wrapper, then the Plan 16 executor reruns its exact original Task 1 command on final source. Include Root's three files as separate protected external owners in Plan 16's final causal frame and inverse. Independent source/evidence review, normal formatted source fit, Plan 19 breadcrumbs, Plan 20 full native/coverage/Bazel/security/goal/lifecycle checks, normal-hook Git finalization and STOP before Phase 168 remain required.
