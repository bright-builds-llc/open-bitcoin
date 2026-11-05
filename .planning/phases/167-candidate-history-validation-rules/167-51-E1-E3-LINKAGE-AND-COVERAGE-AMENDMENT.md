---
phase: 167-candidate-history-validation-rules
plan: "51"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-30T05:02:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 51 E1–E3 linkage and managed coverage correction

An independent read-only review of the first Plan 51 E1–E3 source returned HOLD. The genuine E2 test-first RED, conditional positive-control timing closure, Task 1 7/7 and Task 2/3 3/3 remain valid only for their named paths. E4 has a separate genuine provider RED and is still undergoing focused implementation. No final source frame or final twenty-command credit exists yet.

The E1 `finish_bookkeeping` guard currently checks the target body's exact stored bytes, header/hash/work, current binding/epoch and target `available && !failed`. It does not check failed or unavailable ancestors. The existing HeaderStore body eligibility `activatable` rule walks ancestor states and refuses a failed/unavailable ancestor; the managed `eligible_body_candidates` projection additionally requires a retained suffix. A linked NonExtending child whose ancestor later fails could therefore clear matching requests from `finish_bookkeeping` despite lacking current full-linkage eligibility. This is a **production correctness HOLD**, not merely a missing test.

ROOT permits a new focused failed-ancestor control in the already-owned E3 canonical stored-body child **before** repairing E1. The test must build actual retained child and ancestor/request state, mark the ancestor failed through existing real APIs, then reach the E1 completion boundary and show the current wrong matching-request cleanup while preserving unrelated order and full accepted/work/Coins/undo/BASIC/provenance. A compiling named failure at that intended behavior is a correction RED; compile/setup or an earlier unrelated refusal is not. Repair only the already-owned `mempool_lifecycle.rs` by requiring current **full-linkage** eligibility through the existing HeaderStore/managed candidate projection at the completion boundary. Do not duplicate the ancestor algorithm, add a public authority constructor, change an unowned HeaderStore/eligibility file, or infer eligibility from target-only booleans. Verify connected, sidebranch, local and reorg completion still pass after the guard.

The E2 and E3 new managed children must then cover the published retained-body contract beyond their first three paths. E2: actual stored Duplicate with different submitted witness, header-only and unknown-parent NonExtending/Disconnected caches, stale cache/epoch, local/reorg success and failure, repeat idempotence, matching versus unrelated ordered/partial/consumed request state, and reachable storage/flush/Always/BASIC failures with correct post-absorption effects. E3: genuine canonical retained sidebranch, actual-body Duplicate, header-only and body rejection, stale epoch, failed descendant, owner/import/fixture mismatch, full typed accepted/work/Coins/undo/BASIC/provenance and request conservation. Retain E2's invalid-full sender-only RED/GREEN and E3's genuine Connected positive. Reuse the declared test owners and existing real fixtures; if a required fault or state cannot be reached within the 13-file lease, HOLD for an exact checked refinement rather than inject an outcome or claim unrun coverage.

The E1 unused `block_hash` import in owned `action_translation.rs` must be removed before strict lint. Keep all owner files normally formatted at most 628 lines; the provider test registrar remains exactly 628 and cannot gain a second declaration or packed helper. No source write is permitted while an owned Task 4 application is active. After it closes, capture the current 13-owner hashes and complete protected/native/Knots copies before the correction window, then run focused Task 2/3 and the E1 failed-ancestor control. Later E4 controls and all four task commands must be revalidated on a final frozen source. Only then run all twenty published final commands fresh and serially with original-before whole-file BSD `patch -F0` inverse/forward, protected/native/Knots conservation and independent CORE/source/evidence review.

This amendment adds no owner, command, target/cache exception, Git action, global CVLD closure or Phase 168 work. The missing pre-E1 same-frame positive control remains disclosed under its separate checked timing disposition; no retrospective TDD credit is granted.
