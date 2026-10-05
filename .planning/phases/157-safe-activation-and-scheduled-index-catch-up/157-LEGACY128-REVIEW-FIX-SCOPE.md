---
phase: 157-safe-activation-and-scheduled-index-catch-up
generated_by: gsd-code-review-fix
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T22:39:51Z
status: verified
finding: WR-L01
---

# Review correction: reject test-only implementation attributes

Phase 128's selected save root rejects only an immediately adjacent test attribute on its owning implementation. An additional attribute between `cfg(test)` and `impl DurableSyncRuntime` can bypass that check. The shared Phase 157 helper remains unchanged; correct the source-specific owning attribute check.

- [x] Capture a full-checker RED using a test-only owning implementation with an intervening attribute, after a positive live/snapshot baseline.
- [x] Inspect the full owning attribute block and reject test-only ownership independent of attribute ordering or additional attributes, preserving exact implementation and save-error ordering requirements.
- [x] Run independent reordered/multiple-attribute/comment decoys and positive non-test attribute controls, complete Phase 128/unchanged Phase 129 live/mutation suites, and scoped parse/style/diff checks.
- [x] Root performs the independent nine-file legacy re-review after the three changed sources freeze, then restarts the full native verifier from its beginning.

Ownership is the existing Phase 128 guard and test file, the small `scripts/check-phase128-production-compact-announcement-transport/impl-attributes.ts` child, this scope's progress and current `157-REVIEW-FIX.md` (iteration 3). The guard was already 627 lines; root authorized the coherent child before creation to preserve readability and the 628-line limit. Root owns exact new-source intent-to-add and the nine-file legacy review. No shared lexer, production/Rust, dependency, verifier stage, Git or shared phase state changes are delegated. Preserve the earlier frozen guard corrections and their evidence.

The actual intervening-attribute RED received no failures after its positive full-checker baseline passed. The final scoped GREEN has 50 Phase 128 tests / 67 assertions and 12 unchanged Phase 129 tests / 12 assertions, all passing; both live guards pass. Guard/test/helper line counts are 624/366/26. Iteration 3's `157-REVIEW-FIX.md` records the diagnostic, final logs and source hashes. Independent review, native verification, formal lifecycle closure and Git finalization remain root-owned.
