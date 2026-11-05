---
phase: 167-candidate-history-validation-rules
plan: "17"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T01:08:50Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 17 Phase 158 current guard dependency

The pinned Bun 1.3.9 Phase163 native suite encountered 38 failures through a shared Phase158 clean-current precondition, while its provenance companion passed. The old Phase158 checker is also run directly by unchanged `scripts/verify.sh`, before Phase163. Its failures are two actual source-row drifts, not a Bun version issue or a fixture-only spelling difference: `maybe_basic_filter_turn_body` now admits length/work before `parse_serialized_block` rather than directly calling `parse_block`, and the `complete_coins_write` full writer moved from `chainstate/flush_lifecycle.rs` to the registered `chainstate/flush_lifecycle/metadata.rs` child, where it uses `prepare_current_best_block` and `persist_chain_meta_with_counts` after counts-support/prefix/checkpoint/prune preflight. A Phase163-only tolerated-failure list would mask the current source guard and leave the default verifier red.

ROOT adds exactly **two existing Phase158 files** to Plan17 direct ownership: `scripts/check-phase158-validated-reorg.ts` in Task1 and `scripts/check-phase158-validated-reorg.test.ts` in Task2. The checked allocation becomes **23 unique paths**, 22 present original whole bodies and one truly absent Phase123 test-fixtures child, tasks **12 / 6 / 5**. Of the post-FINAL51 full 1,817-file source frame, 1,795 original paths remain protected. The current Phase158 test is 623 lines and has five lines of normal-formatted headroom; add adversarial cases through cohesive in-owner refactoring or HOLD for a separately checked child, never packing, dropping old tests or adding a hidden owner. No production Rust, generic parser, verifier wiring, immutable historical receipt or bundled native owner is added.

Before either new owner changes, preserve the running pinned Phase163 native job's complete raw outcome, close all owned Bun jobs, capture an exact current 23-owner stage frame and both new original whole bodies, issue a supplementary ROOT lease and obtain independent source-release review. Run the exact standalone Phase158 checker on accepted FINAL51 source to retain a compiling two-row current-source RED, plus an unchanged authenticated historical positive/refusal control. Renew only the two current source contracts against the actual reviewed bodies and call order: require real bounded turn input admission and parser sequencing, and real registered metadata writer with counts support, prefix/checkpoint/prune preflight, current-best preparation, persistence and source/epoch error propagation. Reject old parse_block/direct old-writer bodies, comment/string/cfg(test)/disconnected helpers, omitted or reordered checks, and dead branches. Do not weaken the historical frozen source/manifest/snapshot attribution.

Run `bun test ./scripts/check-phase158-validated-reorg.test.ts` with nonzero named discovery and `bun run scripts/check-phase158-validated-reorg.ts` with zero current findings under the repo-pinned Bun 1.3.9 environment. The already-owned Phase164 strict tooling inverse must exact-project the final Phase158 checker/test **current** bytes back to their prior reviewed current hashes, preserving original historical manifests, snapshots and Phase163 receipts; then rerun Phase163 provenance/native, Phase162/164/165/166 affected guards and all four original Plan17 commands fresh on one frozen source frame. Earlier Phase163 native 38-failure run remains diagnostic and receives no final credit. Full original-before inverse, 1,795 protected source, independent CORE/Summary QA and later default verifier/coverage/Bazel/normal-hook main commit/push remain required.

This amendment grants no unowned source edit, target/cache override, historical waiver, Git action, global CVLD closure or Phase 168 work.
