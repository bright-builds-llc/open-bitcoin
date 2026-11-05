---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-11-02T03:20:39Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 Bazel lock freshness after coverage diagnosis

The clean unoptimized `cargo llvm-cov --profile dev` diagnostic returned the **same** 159 uncovered lines in 20 pure-core files as the opt-level2 default v13 report. The conditional helper-profile change in `167-20-UNOPTIMIZED-COVERAGE-AND-BAZEL-LOCK-AMENDMENT.md` is therefore **not activated**: `scripts/verify/helpers.sh` remains byte-identical to its original SHA `6721f6ba01df119c7f4bf768f73c9bce8c3b7999fa18836c9d08e261395d3878` and is removed from Plan20 direct edit ownership. The zero-uncovered, eight-package coverage gate remains unchanged. Closing the real coverage gaps requires a separately checked source/test scope; no diagnostic run earns final coverage credit.

The Bazel lock change is independent. The successful v13 Bazel six-target build and provenance check regenerated tracked `MODULE.bazel.lock` from original SHA `fee54340c9e992be8a50d99728b793a108fcada9914284091c6b07df2e3171ba` to current SHA `411cbf90933d78d3b7954525ce2dcbbcd8d6474b92effaeea7ff5b41af3a6ea1`. Its only Git hunk is the `recordedFileInputs` hash for `packages/Cargo.toml` at line 2834, changing the original manifest SHA `2b18fb528f59dbec2141299545cd536e0e61a5ee8eea3b3f88c0190bfa98f14d` to the live opt-level2 manifest SHA `6fc738138972ed88e5624b3b8ab2c1c629b409acb08bc93596e1cb2faa09bf2b`. No other lock entry changed. Preserve this exact generated freshness change; do not revert it to make the source frame appear unchanged or rewrite unrelated lock data.

ROOT retains exactly one direct Task1 owner from the prior conditional pair: `MODULE.bazel.lock`. Plan20 now has 44 direct owners (Task1 41, Task2 three) and 1,789 protected original product paths. Capture original/current whole lock bytes, the one-hunk diff, live manifest hash and exact current source frame; independently review and issue a ROOT acceptance receipt for the already-generated lock bytes before treating them as reviewed source. No new lock mutation is authorized. A subsequent Bazel six-target build and provenance check must leave the lock byte-identical, and normal staged-index hooks must include it. The coverage-closure owner plan remains open; no phase, requirement, Git or Phase168 acceptance is granted by this lock reconciliation.
