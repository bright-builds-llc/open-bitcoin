---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T07:26:29Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 Phase 98 current status-token boundary

After the checked Phase97 current constructor guard passed, a direct Phase98 preflight found `P98 stale status in .planning/STATE.md: 7 pending`. The checker scans the whole STATE document with `text.includes("7 pending")`. A dated historical Session Continuity line contains `fresh30 all17 pending`, whose suffix matches the shorter string; the current-state guard therefore reports a false stale count. The synthetic Phase98 positive fixture omits that historical line and passes, while an existing negative explicitly appends `7 pending` and must continue to fail. The accepted STATE history is accurate and must not be rewritten to satisfy a substring check.

ROOT adds exactly `scripts/check-phase98-traceability-reconciliation.ts` and `scripts/check-phase98-traceability-reconciliation.test.ts` to Plan20 Task1. Together with the checked Phase97 checker, VALIDATION, generated LOC and three independent reports, Plan20 has **eight unique direct owners**, Task1 **five** and Task2 **three**. Both Phase98 files are present in the original 1,828-file accepted product frame, leaving **1,825 protected product source files**; LOC remains separately owned and four reports truly absent. Before either Phase98 file changes, close all jobs, capture their exact original/live whole bodies and the current provisional LOC in an eight-owner stage frame, issue a supplementary ROOT source release and obtain independent release review. Earlier first-run and preflight failures remain diagnostic for their own source frames.

Renew only the `7 pending` stale-status match to require a whole numeric token, preserving all other historical phrase checks and exactly matching the genuine `7 pending` negative. Add one focused Arrange/Act/Assert positive test that places `fresh30 all17 pending` in the STATE fixture and expects no stale-status finding. Retain the existing exact-seven negative and all other Phase98 tests. No Phase98 production source or command registration changes are authorized; Phase99/162 command-name references and frozen historical receipts stay unchanged.

Run the focused Phase98 suite and direct checker to GREEN, plus Phase97 and affected current source guards. Regenerate provisional worktree LOC after source edits, perform a fresh coverage clean through command-timings, then rerun the complete default `bash scripts/verify.sh` on the new frozen source. Preserve raw failures, timings, liveness and final source hashes. Independent source/security/goal review and ROOT staged-index LOC/hook reconciliation remain required. No target/cache/flag/toolchain override, staging, Git action, global CVLD closure or Phase168 work is granted.
