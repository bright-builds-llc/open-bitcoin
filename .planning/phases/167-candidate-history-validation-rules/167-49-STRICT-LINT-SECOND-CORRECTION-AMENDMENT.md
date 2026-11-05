---
phase: 167-candidate-history-validation-rules
plan: "49"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-28T01:14:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Second Plan 49 strict lint diagnostic and replan

After the first two-owner Plan 49 lint repair, a fresh strict Clippy attempt exposed two additional errors in historical test-only files: needless explicit lifetime in `phase167_historical/contexts.rs::selected_row` and `err_expect` in `phase167_historical/source.rs`. The first strict attempt could not reach these diagnostics because the network crate failed earlier. Both are outside C4's five-file lease and its source remains frozen. Two strict attempts have now failed, so this checked refinement adds a separate exact two-owner correction before any final-gate restart.

Elide only `selected_row`'s input/output lifetime, preserving the same borrowed value. Replace `.err().expect("changed listing refuses")` with `.expect_err("changed listing refuses")`, preserving the same asserted error string and positive-path type. Capture original whole files, prove normal format, rerun focused historical named tests and strict Clippy under the same reviewed Plan 49 nine-key async probe, approved isolated target, natural global process guard and serial timing lock. If strict lint exposes another issue, stop and refine again; do not hide errors with lint allows or reinterpret setup/lint as RED.

The first two-owner correction remains pending independent acceptance. Both corrections must prove exact source ownership, protected/native conservation and strict whole-original BSD inverse/forward. After ROOT accepts them, C4 must capture a fresh final source/native frame and rerun all 13 Plan 49 final commands from the beginning. Earlier passed format and failed strict attempts are diagnostic only. Plan 50/51 target policy is unchanged; no Git or Phase 168 work is authorized by this amendment.
