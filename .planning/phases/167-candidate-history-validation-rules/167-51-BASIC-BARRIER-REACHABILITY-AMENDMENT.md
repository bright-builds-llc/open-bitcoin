---
phase: 167-candidate-history-validation-rules
plan: "51"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-30T13:15:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 51 BASIC barrier reachability

The E4 test child attempted a real no-hook pending-barrier workload: nine genuine Accepted prefix bodies with BASIC disabled, same-datadir reopen with BASIC enabled, two outstanding requests, then public submission of a tenth block. The test compiled and reached the actual command, but returned `prepared BASIC resource projection` **before** retained-body save or absorption, not `accepted BASIC work remains pending`. This raw failure is diagnostic setup for the proposed barrier route, not a barrier test pass.

The source-derived reason is an earlier invariant. `FjallNodeStore::submission_basic_rows` requires active progress with `maybe_processed_endpoint() == safe durable predecessor`, the predecessor exactly equal to `accepted_target`, and the prepared position exactly one child of that target. A BASIC backlog violates this pre-effect gate. Enabled reopen drives one production turn with normal budget eight blocks, so a prefix of at most eight catches up and leaves only the newly submitted target; a larger backlog remains behind and is refused before absorption. The normal turn budget handles up to eight ordinary blocks, while an oversized single valid block has its separate absolute singleton budget. No existing deterministic E4-owned hook acts between the successful BASIC turn and barrier poll. Independent review found no supported no-hook route to force either a Pending barrier or a Ready Err at that terminal stage without first violating the pre-effect gate. This is a supported-surface reachability limit, **not** proof that the barrier code can never return those results under arbitrary external faults or future policy.

ROOT replaces the planned E4 absorbed-then-failed BASIC-barrier test obligation with a named **pre-effect backlog refusal and request-conservation control** using that same real fixture. Assert exact `prepared BASIC resource projection` stage and genuine source, unchanged matching and unrelated ordered/partial/consumed request state, unchanged full accepted/native baseline, absence of the tenth retained body, and accepted tip still at nine. Verify separately that the normal BASIC-enabled public Accepted path reaches final success only after the barrier completes; keep existing BASIC BeforeBody/BeforeCoins, actual Always flush, achieved and physical failure controls. Do not label the backlog refusal `accepted BASIC barrier`, claim post-absorption rollback, fabricate an interposed hook, mutate an unowned workflow, or hide the unrun terminal error branches.

The correction stays inside the already-owned E4 `compact_download_cleanup.rs` child and must keep normal formatted length at most 628; if the test cannot fit without packing, HOLD for a checked source split. Rerun the named backlog control and literal Task 4 selector with nonzero discovery and all old fault tests. An independent source/evidence review must validate the invariant, exact stage/source and full request/native conservation before final twenty-command release. The original E2 timing deviation, E1 failed-ancestor correction and E4 Duplicate fault evidence remain separate. No new owner, target/cache override, Git action, global CVLD closure or Phase 168 work is granted.
