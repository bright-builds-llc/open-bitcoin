---
phase: 167-candidate-history-validation-rules
plan: "51"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-30T11:54:44Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 51 E2 fault reachability and E4 durable controls

Plan 51 Task 2 asks for storage, flush, Always and BASIC failure request conservation **when available through existing real fault controls**. The E2 managed test fixtures instantiate `MemoryChainstateStore`. Its existing chainstate and flush persistence methods have no failure switch and return success for persistence, undo and chain metadata; it has no durable BASIC owner or `CanonicalSubmissionCommand` Always path. Adding a new fault adapter, fake outcome or public hook would exceed Plan 51's 13-owner scope and undermine the test. E2 must not claim that its current 10/10 (or later) named cases exercised those unavailable failure stages.

The existing E4 durable provider fixture does have real fault routes: `ABSORB_FAULT` at publication, `set_basic_filter_fault` at named BASIC body/undo/coins stages, and `CHANGE_AFTER_BASIC`/`ADVANCE_AFTER_BASIC` at actual source freshness and header/reopen boundaries. ROOT conditionally assigns the unavailable E2 fault-class request-conservation checks to E4's already-owned `runtime_authority/validation_history/tests/compact_download_cleanup.rs` child. The worker must establish real matching compact requests and an unrelated request through the checked managed post-accept compact route or earlier genuine request ownership, invoke the actual public command with each reachable fault, and assert the named stage/source, post-effect accepted/native facts and preserved matching/unrelated request state. A post-absorption error may advance accepted state; compare against the correct baseline and do not claim fictitious rollback. Keep separate cases for failures before durable success and after partial effects where the existing hooks actually reach them.

This is a **conditional disposition**, not present test credit. The current E4 5/5 proves final Accepted and actual stored Duplicate/corrupt/missing rows, but does not yet prove Always/BASIC failure request conservation. Only compiled, named controls that hit genuine fault sources and pass after the E4 extension may close this gap. If any required named stage cannot be reached inside the current owner or a fault blocks setup before the intended boundary, HOLD for a checked refinement and disclose that exact limit. Independent source/evidence review must verify actual fault provenance and no new E cleanup on error.

E2 remains responsible for all reachable in-memory invalid receipt, retained body, Duplicate, cache/epoch, local/reorg, sidebranch and idempotence controls; E3 remains responsible for canonical earned-history and full accepted/request conservation. The already checked E1 failed-ancestor guard and E4 Duplicate post-tamper baseline requirements remain unchanged. This amendment adds no owner, command, target/cache exception, dependency, production fault hook, Git action, global CVLD closure or Phase 168 work. All four task selectors, twenty final commands, whole original inverse and complete source/native/Knots conservation remain required.
