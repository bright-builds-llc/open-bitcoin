---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-11-01T05:14:14Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 default test-profile performance trial

The complete node suite on the test-only measurement mutex still failed six tests (2,402 pass/four ignored), including four helper `<30s` panics, an independent inspection worker deadline, and a safe history reopen refusal. A checked temporary test-only stage probe then reproduced three failures under the unchanged full parallel test command, while all six exact focused tests passed alone. Its contended measurements separated a 14.711s pre-inventory source gate, 10.570–14.489s physical copy, 3.241–29.154s supervised child recovery and ~2.5s source recheck. The same 67 MB inspections take about 10–13s focused. SHA-only or helper-only changes cannot explain all failures. The temporary probe is now removed byte for byte: `inspection.rs` is back to SHA `4102a31631cdb36d8d7a6d6436f4e079101a2eae8c396aba22135186f5a5450c`; the independent rollback audit found 32 unchanged owner hashes and 1,801 protected originals. All failed and diagnostic runs remain retained with zero acceptance credit.

The helper mutex was a failed candidate, so restore only its added comment/static/acquisition to the exact pre-lock helper SHA `85c90a8fe7eb15b188e31d304639a7c2cb6c1529aa42feeefee5fa902f9c2d4a` (622 lines). This keeps the earlier checked callback-entry copy/recovery timer correction, both strict `<30s` assertions, all row/corruption checks and the production worker's independent 30-second deadline. Independently verify that rollback before a new source release. The earlier contention amendment remains honest failure history and grants no final lock credit.

ROOT adds exactly one tracked Task1 owner, `packages/Cargo.toml`, after capturing its exact original and the restored 32-owner frame. ROOT must issue a supplementary source release for that exact manifest owner before the worker edits it. Add only a workspace-root `[profile.test]` with `opt-level = 1`, `debug-assertions = true`, and `overflow-checks = true`. This is a repo-native test build setting trial, not an ad hoc command flag, target/cache/toolchain override or reduced test-thread run. It narrowly supersedes prior Plan20 wording that forbade any build-setting change. The trial creates 33 unique direct owners (Task1 30, Task2 three) and 1,800 protected original product paths while the manifest differs. Production release behavior, physical copy/hash/recheck, worker scheduling, test cases, default Cargo test command, all deadlines and fixture sizes remain unchanged. No production source file may be edited in this trial.

First obtain independent review of the exact manifest delta, source frame and unchanged helper/inspection files. Run pinned `cargo fmt --all`, strict Clippy, all-target build and focused former failures plus the 47-case concurrent selector through the cooperative lock; preserve nonzero discovery and measured `<30s` evidence. Then run the complete default-parallel node suite and the full default `bash scripts/verify.sh` on a fixed source, including fresh pure-core coverage with **zero uncovered lines** and Bazel. The test profile may change LLVM coverage mapping; do not weaken the coverage gate or claim success from a focused/diagnostic pass. Check disk and build liveness, retain raw runs and source before/after. If opt-level1 fails any full native/coverage gate, stop for separate checked diagnosis or a separately reviewed profile revision; do not silently escalate optimization, change test threads, relax timeouts or mask failed cases. Normal-hook index LOC reconciliation, independent final source/security/goal reports, lifecycle, commit and push remain open. Stop before Phase168.
