---
phase: 167-candidate-history-validation-rules
plan: "49"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-28T01:09:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 49 final strict lint correction

C4 Task 4 passed 38 compact tests and final format passed. The first final strict Clippy application found two errors in accepted C1/C2 owners outside C4's exact five-file lease: `expect_used` in `peer/compact_download_state.rs:225` and `cloned_ref_to_slice_refs` in `compact_download/completion.rs:554`. These are compiler/lint failures, not behavioral REDs. C4 source and jobs are frozen while ROOT corrects only these two files under a separate exact release. The C4 worker must not edit them.

Replace the C2 `expect` with a typed missing-binding return if the matching mutable state is absent, preserving the existing observed branch and `UnexpectedBlockHash` handling. Replace the C1 test-only one-element cloned slice with `std::slice::from_ref(&missing)`; the test's later move remains valid. Do not change selected production behavior, public API, manager policy, witness selection, C3 tests, or any other source. Capture complete original whole bodies and exact hashes, run normal format and the focused strict lint/behavioral checks with the Plan 49 nine-key async child probe and approved isolated target. Check strict whole-original BSD patch `-F0` inverse/forward and independent review. If a new compile or behavior mismatch appears, stop for another checked refinement.

After ROOT accepts this bounded correction, C4 must recapture its final causal/protected/native source frame, then rerun **all 13** Plan 49 final commands fresh and serially. Prior final format and failed strict are diagnostic attempts, not final credit. C4's already passing Task 4 test remains task evidence if its five owner bytes are unchanged, with any focused correction checks recorded separately. Final Plan 49 Summary must disclose this correction, and retain C3's no-preimplementation-RED disposition. No Git action, Phase 168 work, target exception extension to Plans 50/51, or global CVLD closure is granted.
