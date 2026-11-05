---
phase: 167-candidate-history-validation-rules
plan: "16"
type: evidence-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-27T23:09:08Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 16 BIP30 restoration test sensitivity

The Plan 16 Task 2 tests exercise already-implemented managed BIP30 overwrite and durable restoration behavior. The first compiled exact-repeat test passed. The earlier missing-import/moved-value failures were setup, an explicit unimplemented source-boundary stub was scaffold evidence, and the initial memory-backed reopen and malformed injection failures were test wiring. None is a genuine pre-implementation behavioral RED. Do not reclassify them or deliberately regress live source to create one.

Task 2 retains every original action, all 39 required case observations, both physical repeat/reopen/restore/reconnect paths, missing and malformed undo/source controls, its literal original command and final full-source verification. After the final Task 2 source passes, Root may validate the new restore test's sensitivity in an ignored disposable copy that excludes Git state and build caches. Capture and hash the complete copied source and the original copied `packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs`. Change **only the copied restoration loop** to skip applying prior overwritten Coins (for example, iterate `overwritten.iter().take(0)` only at the `overlay.add_coin` loop). Run the exact named `phase167_historical_bip30_both_repeats_restore_across_physical_reopen` test through the cooperative timing wrapper with a distinct isolated Cargo target. Credit a compiled failure only if it occurs at the expected complete prior-Coin/endpoint restoration assertion; compiler/setup, unrelated assertion or zero discovery earns no sensitivity credit. Restore the copied file byte-for-byte and prove the full scratch and live protected manifests unchanged, with independent review.

This is explicitly a **post-GREEN mutation-sensitivity check**, not a claimed pre-implementation RED. Root must record that limitation and adjudicate the checked TDD deviation before FINAL16. No live `overwrite_undo.rs` ownership, production change, relaxed test, altered original Task 1/Task 2 argv, commit, push or Phase 168 work is authorized. Plan 19 parity and Plan 20 full native/coverage/Bazel/security/goal/lifecycle gates remain mandatory.
