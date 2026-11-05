---
phase: 167-candidate-history-validation-rules
plan: "49"
type: evidence-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-28T00:52:42Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# C3 selected caller migration test sensitivity

Plan 49 C3 migrates tests to the permanent selected completion APIs already implemented and accepted by C1/C2. Its two required direct selected controls compiled and passed on the first runnable source. The preceding `E0382` test fixture move was compiler setup, not a behavioral RED. A preimplementation behavioral RED cannot be claimed without deliberately regressing the live selected producer or inventing a failure. C3 must preserve that history and may complete its 14 direct caller migrations without a fabricated RED; every original Task 3 and final behavior, category, ownership and command gate remains required.

After final C3 source passes, Root may test its new mutation-to-full-fetch assertion in an ignored disposable source copy, excluding Git state and build caches. Capture complete copied source and the original copied `packages/open-bitcoin-network/src/compact_download/completion.rs`. In **only the copied child**, change the selected complete-fill mutation guard from `selected_block_mutated(&block, check_witness_root)?` to `selected_block_mutated(&block, check_witness_root)? && false`; keep the borrowed predicate call and typed error propagation but disable the fallback decision. Run the exact named C3 `selected_complete_merkle_mutation_falls_back_with_partial_retained` test through the cooperative timing wrapper with the reviewed Plan 49 nine-key async probe, natural global guard and distinct isolated Cargo target. Credit a compiled failure only if the test reaches the selected completion path and fails at its expected Fallback/retained-partial assertion because the mutated body is Completed. Compiler/setup, unrelated assertion or zero discovery earns no sensitivity credit. Restore the copied child byte-for-byte; prove all scratch and live protected source bytes unchanged and obtain independent review.

This is explicitly **post-GREEN mutation sensitivity**, not a preimplementation RED. Root must record the TDD process deviation in C3 and Plan 49 summaries before bounded acceptance. The genuine C1 Merkle-mismatch and C2 canonical mismatch REDs retain their own credit. This amendment grants no live completion-child ownership, no change to C3's three test-file lease, no altered literal task/final argv, no target/cache override beyond the separately checked diagnosed Plan 49 exception, no hook bypass, Git action or Phase 168 work.
