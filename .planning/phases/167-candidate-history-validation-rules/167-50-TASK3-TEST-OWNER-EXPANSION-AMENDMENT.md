---
phase: 167-candidate-history-validation-rules
plan: "50"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-28T04:50:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 50 Task 3 exact test-owner expansion

The Task 3 literal node compact selector discovered 93 tests: 83 passed and ten failed. Four failures were in the original D-owned node test files and used stale `Block` expectations under the actual default Version `WITNESS` service; the worker corrected those exact assertions, and a fresh run passed 87 with six remaining failures. Five remaining failures share the `is_full_block_getdata*` helper in `packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs` (353 lines), which recognizes only `Block` even though that test's real handshake advertises `NETWORK|WITNESS`. The sixth is in `packages/open-bitcoin-node/src/sync/tests/peer_resolution/peer_protocol_failures.rs` (613 lines), expecting the first request to disappear immediately after timeout fallback and Ping. Plan 50 D1 intentionally retains that first bounded request/attempt/start through consumed lifetime until terminal cleanup. Independent review found these are stale test expectations; it did not identify a new production owner or authorize source edits.

ROOT explicitly adds **only these two existing test files** to Plan 50 Task 3 and the direct-owner list, changing the checked count from 13 to **15 unique direct owners** (13 present whole originals, two true-absent children; Task allocation 4/2/4/5). Both files are present and byte-identical to the pre-source 493-path frame. This is a test-only scope refinement; no production code, other test owner, public API, timer, cap, policy, selected completion, native input or Knots anchor is added. The existing pre-dispatch guard interpretation and default-target policy remain in force.

Migrate the timing helper or its exact five cases to assert `WitnessBlock` and the same hash under the factual default WITNESS handshake. Do not broaden a generic helper to accept either inventory type without checking actual service bits; preserve explicit no-WITNESS `Block` controls elsewhere. Migrate the peer-resolution Ping assertion to retain the first request after consumed fallback, while preserving its existing terminal cleanup and unrelated-peer checks. Test-only changes must remain normally formatted under 628 lines and use the original Arrange/Act/Assert structure.

Before either added owner is edited, ROOT must issue a separate exact two-owner source release with complete original whole-body copies and current hashes, reclassify them from protected to leased in the Plan 50 frame (protected source 482→480), and record the expanded 15-path owner union. After edits, rerun the exact Task 3 selector and the focused runtime-timing and peer-resolution selectors with nonzero named discovery. The five timing failures did not print the outbound inventory; the rerun must demonstrate actual `WitnessBlock` type/hash. A continuing failure requires renewed diagnosis, not a broadened assertion. Keep all raw failures as evidence; they are not TDD RED for D1/D4.

The final 16 literal Plan 50 commands still run fresh and serially on one frozen final source/native frame. The final original-before BSD `patch -F0` inverse/forward must restore all 13 present original owner bodies and both true absences, with 480 protected first-party source paths and original 201 native plus Byteview and four pinned Knots anchors conserved. Independent CORE/source/evidence review, separate Summary/QA and ROOT acceptance remain required. No Git action, global CVLD closure or Phase 168 work is granted.
