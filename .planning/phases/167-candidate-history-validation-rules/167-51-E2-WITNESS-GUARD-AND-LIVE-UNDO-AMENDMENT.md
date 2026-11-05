---
phase: 167-candidate-history-validation-rules
plan: "51"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-30T11:59:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 51 generic witness guard and live undo expectations

The expanded Task 2 selector compiled 18 named tests: 16 passed and two failed. These failures are test-boundary assumptions, not demonstrated product regressions. In the E3 Connected case, `connect_stored_block` commits the live Chainstate undo but does not run the provider's `FlushMode::Always` durable flush. Requiring an immediate durable undo row is premature. The E3 test must assert the live typed `Chainstate::undo_by_block` and compare accepted/work/Coins/BASIC/provenance and durable state to the correct pre-flush baseline; it may claim durable undo only after a real explicit flush/reopen path.

In the E2 generic `connect_stored_block` path, `guard_body` and `retain_checked_candidate` run before the Duplicate branch. A submitted witness-altered body is refused at the actual `unexpected-witness` guard, before cache mutation or stored-body substitution. The E2 test must assert this typed refusal and preservation of the matching and unrelated requests, original retained cache and accepted state. It must **not** claim an E2 Duplicate success by weakening the guard, changing the submitted witness bytes, or skipping source validation. The E4 public `command.submit` path separately accepts a witness-altered Duplicate from the actual same-store retained body; its existing 5/5 Task 4 control supplies that positive stored-body substitution evidence. The two interfaces have distinct admitted inputs, and the canonical Summary must state the distinction without claiming E2 directly proved Duplicate success.

Retain the raw 16/2 Task 2 failure as diagnostic history. Rerun the exact Task 2 and Task 3 selectors after these two test-only corrections, with nonzero named discovery and all old controls. Any failure at a different stage or loss of matching/unrelated request state is a new diagnosis HOLD. Keep the current 13 declared owners, original E2 test-first RED, E1 full-linkage correction RED/GREEN, separate E4 public-success RED and fault controls, default-target nine-key async probe, source/native/Knots conservation and final twenty-command requirement. No new production owner, public authority, fault hook, target exception, Git action, global CVLD closure or Phase 168 work is granted.
