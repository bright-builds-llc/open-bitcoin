---
phase: 167-candidate-history-validation-rules
plan: "17"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-30T22:23:32Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 17 exact guard input and inventory owners

Post-FINAL51 read-only Plan 17 fit review found that the original twelve-file allocation cannot directly inspect the final runtime boundary it promises. `scripts/check-phase123-runtime-timing-evidence-integrity/constants.ts` owns `TARGET_FILES` and omits the actual production `sync/block_reconcile/peer_requests.rs`; `checks.ts` reads only that fixed input inventory. The Phase 123 guard cannot claim direct current-source inspection by changing `checks.ts` alone. Plan 17 Task 3 therefore gains this **exact existing constants file** and must add the actual peer-request production path in the input set without losing old targets or synthetic/live fixture coherence.

Task 1 may change the ordered Phase 165 `SOURCE_FILES`/`SOURCE_CONTRACTS` membership while binding current Phase 167 source. The fixed `INPUT_INVENTORY_SHA` lives in `scripts/check-phase165-shared-exact-work.ts`, outside the original allocation. Plan 17 Task 1 gains this **exact existing entrypoint** as a conditional owner: update its hash only if the ordered membership actually changes, derived from the final reviewed file list; otherwise leave its whole body byte-identical. Do not insert a dummy source row or alter historical receipts solely to exercise this owner.

The checked Plan 17 direct-owner scope becomes **14 unique paths**: 13 present whole originals and one genuinely absent Phase 123 `test-fixtures.ts` child. Tasks become 6 / 4 / 4 owners. No other source/test owner is implicit. Existing Phase 164/165/166 tests at 628/627/626 lines may be retargeted in place and Phase 123 completeFiles/breadcrumb fixture bulk moves to the declared child; normally formatted fit at most 628 remains an execution gate. `source-files.ts` begins at 621 lines, so any semantic edge additions beyond its available headroom require a separately checked cohesive split, never line packing, assertion removal or a hidden child. All old test names and counterfactual failures remain.

This is a planning/source-ownership correction only. Before Plan 17 source edits, ROOT must accept FINAL51, capture the full post-FINAL51 first-party source frame and its 77 out-of-Plan51 Rust provenance supplement, close independent deep product-source findings, rehash all 14 direct originals/absence and grant an exact source lease. Task 1/2/3 genuine checker/test REDs, four literal Bun commands, synthetic/live corpus, source-hash rationale, claim/selector refusals, strict owner/protected inverse, independent CORE/Summary/QA and downstream native/coverage/Bazel/normal-hook main commit/push all remain required. No broad ordinary-configuration activation, target override, Git action, global CVLD closure or Phase 168 work is authorized here.
