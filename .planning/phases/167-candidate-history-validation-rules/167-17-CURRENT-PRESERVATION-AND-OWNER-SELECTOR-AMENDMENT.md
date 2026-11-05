---
phase: 167-candidate-history-validation-rules
plan: "17"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-30T23:13:01Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 17 current preservation pin and Phase 166 owner selector

Phase164 current source guards are now GREEN on reviewed source, but two later checks exposed owner/scope limits. Independent read-only reviews bound both against FINAL51 source. No additional guard outcome is credited by this planning correction.

## Phase 165 current preservation pin

The already-leased `scripts/check-phase165-shared-exact-work.ts` entrypoint owns `checkHistory`'s **current mechanically adapted** preservation test size/SHA, separate from the immutable original Phase164 Git blob. Accepted Plan22 added exactly `overwrites: Vec::new(),` to the current preservation file; the current body is 20,736 bytes, SHA-256 `f0d7d56fe2abd6858336e03e65d25719e65a96c5df33783b12351ca23e7a0ec3`, versus the old current pin 20,688 bytes/`e6cb5740…`. The original 20,556-byte historical blob/`3d004f91…` must remain unchanged. The original Plan17 owner refinement permitted this entrypoint to change only `INPUT_INVENTORY_SHA` when ordered membership changed. ROOT now also permits **only this checkHistory current-body size/SHA renewal**, with source-hash rationale, a passing real current checker, and adversarial old-body/altered-assertion mutation refusals. Do not update a historical baseline, remove the preservation check, or change the expected old blob to satisfy current code.

## Phase 166 actual owner and checkpoint selection

The Plan17 Task1 scope adds exactly one existing file, `scripts/check-phase166-canonical-bootstrap.ts`. Its whole-file `maybeFunction(source,symbol,true)` cannot select `OpenedCanonicalGenesis::into_parts` or `RecoveredCanonicalGenesis::check` now that the same OPENER source also contains genuine History impls with those names. `contracts.ts` row-string changes alone cannot fix the entrypoint or ENTRY_GUARDS. Implement a narrow **exact impl-owner-qualified, fail-closed selector** in this entrypoint for only the ambiguous Genesis rows, using existing offset-preserving Rust masking and balanced direct impl-body method boundaries. Require one unconditional production method under the exact owner; reject missing/duplicate/wrong-owner/nested/comment/string/cfg(test)/cfg-disabled decoys and retain raw anchor offsets, order, depth, first-operation and unconditional-return checks. Continue the existing path for unambiguous functions. Do not choose by ordinal, return type guess, dummy Rust call or a widened generic parser owner.

The same entrypoint's `checkOrderAndScope` still names the old `save_validated_chain_meta` delegate. Actual decode, identical-ancestry count retention, validation coverage, append invalidation and replacement are in `storage/fjall_store/chain_meta.rs::save_validated_chain_meta_with_counts`. Retarget the owned Phase166 contract path, exact full-writer body anchors and entrypoint order check to that unique production method; retain a separate check that the old public delegate passes active_chain/None/mode to it. Test real positives and mutations of decode/transpose, same-ancestry count, coverage-before-invalidation, count precedence, final write and wrong-owner/comment/test decoys. Do not claim the old coins.rs/delegate body still performs those effects.

The checked Plan17 scope becomes **15 unique direct owners**: 14 present original bodies, one genuinely absent Phase123 test-fixtures child, tasks 7 / 4 / 4. All other 1,803 files in the post-FINAL51 1,817-file full source frame remain protected. No other source/test owner or command is implicit. The existing Phase166 test file is 626 lines and Phase164 source-files.ts is currently 627 after mechanical renewal; add adversarial controls through coherent in-owner simplification, not line packing, dropped tests or a hidden child. If fit fails, HOLD for a separately checked split. Before editing the new Phase166 owner or the Phase165 preservation pin, ROOT must capture their exact current whole bodies, verify the full frame and already-owned source/current jobs, and issue a supplementary exact source release with independent review. The earlier mixed-time three-suite baseline retains zero pre-edit/final credit; a fresh four-command final source run remains mandatory.

No production Rust, historical receipt, verifier wiring, dependency, target/cache setting, public claim exemption, Git action, global CVLD closure or Phase 168 work is authorized by this amendment.
