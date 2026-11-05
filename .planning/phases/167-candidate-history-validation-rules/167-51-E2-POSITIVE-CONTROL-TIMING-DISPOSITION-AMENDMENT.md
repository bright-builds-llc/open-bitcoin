---
phase: 167-candidate-history-validation-rules
plan: "51"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-30T03:51:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# E2 unchanged-positive timing disposition

The Plan 51 E2 test-only RED ran before any E1 production edit and produced one compiled, discovered behavioral failure: an invalid full Block receipt cleared peer 167512's still-owned same-hash request (`[]` instead of `[167512]`). All 13 direct-owner pre/post hashes were equal during the command and the four E1 production owner hashes still matched the ROOT source release. Independent review accepts this **behavioral RED**.

The executor then began E1 production edits before running a separate unchanged positive control on the E2 test-only source frame. This violates the planned test-first **control timing**. Do not claim a same-frame preimplementation positive pass, substitute a compiler/setup failure, or retroactively label a later control as earlier. The E2 RED itself remains causally valid.

Two accepted Plan 50 node-compact controls, `disconnect_cleanup_clears_volatile_compact_state_only` and `received_block_clears_matching_in_flight_across_peers`, passed in the final Plan 50 node-compact and node-all logs before E1 implementation. Their complete function bodies are byte-identical in the saved Plan 51 before frame and current E2-owned `compact_cleanup_cases.rs`; that file only widens six helper visibilities, while the new E2 test lives in the declared `compact_stored_body_cases.rs` child. The old-function body SHA-256 values are `86a27f919c600929e86a7065bfa23a22cc4cec20936b8bbe4f201fa09c8c3865` and `3c461f47703412cccef493d5252605e2e6a07c80eeff0fa084cf6cbaadd8bec0`. This is prior positive evidence on unchanged behavior, **not** a separate Plan 51 same-frame positive run.

After the owned Task 1 application closes and before any final source acceptance, rerun both old named controls and the literal Task 2 selector under the reviewed nine-key async probe, default target, natural global guard and cooperative lock. Preserve exact raw streams, names, ignored reasons, source hashes and exits. If either old control fails or the E2 RED test cannot turn GREEN through the real E1 path, HOLD for diagnosis. If they pass, ROOT may accept the bounded E2 contribution with this explicitly disclosed TDD process deviation; independent review must verify the old body hashes and both later passes. No claim of an on-time positive control is permitted in the CORE or canonical Summary.

This amendment grants no source owner, target exception, command rewrite, skipped final test, Git action, global CVLD closure or Phase 168 work. All four task commands, twenty final commands, whole original inverse, full source/native conservation and independent plan/phase reviews remain required.
