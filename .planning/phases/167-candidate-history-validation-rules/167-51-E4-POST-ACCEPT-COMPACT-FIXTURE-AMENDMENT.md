---
phase: 167-candidate-history-validation-rules
plan: "51"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-30T05:11:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 51 E4 post-accept matching-request fixture

The focused E4 Task 4 command compiled four named tests. The actual Accepted final-success control passed. Three warm Duplicate tests failed during setup before invoking Duplicate: `request_missing_blocks` deliberately filters a hash already in `known_blocks`. The accepted block had already called `note_local_position`/`note_local_block_hash` and `finish_bookkeeping`, so an ordinary post-accept request cannot be created through that public API. These are fixture setup failures, not Duplicate behavior failures or TDD RED. The separate preimplementation E4 public-success RED and old positive remain valid.

Independent read-only source review found a narrower **potential genuine route** within the same E4 test child: after warm Accepted, enable compact relay through the existing public setter; handshake a real peer with Version, Verack and SendCompact; send an incomplete one-transaction CompactBlock for the accepted hash through managed `receive_message`. The existing compact manager registers a request for selected partial/fallback actions without the ordinary `known_blocks` filter, and a known header can pass its compact depth check. The fixture must observe actual GetBlockTxn, matching request order and an in-flight partial before using the public `command.submit` Duplicate path. This route is statically plausible, **not yet runtime proof**. If it refuses at any stage, HOLD for checked refinement; do not disable the known-block guard, inject private state, call a test-only registration API or claim matching cleanup from an unrelated request.

ROOT permits a test-only fixture correction in the already-owned `packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/compact_download_cleanup.rs`; the command parent and all other E4 production code remain frozen for this correction. Reuse real accepted/store/peer handles, exact current header and BlockTxn facts. The normal warm stored body with a different submitted witness must still return Duplicate and clear only actually matching requests. The real corrupt serialized BlockIndex row must yield the genuine StorageError source; actual row removal must yield unavailable support with no fabricated source. Capture full accepted/request/native baseline **after** each intentional tamper/removal and compare post-command to that baseline for no repair or new E bookkeeping, preserving unrelated request/order, cache warmth and the original intrinsic/domain/accepted-identity error precedence. Keep all raw setup failures.

The checked E1–E3 linkage/coverage amendment proceeds in its own already-owned files after current jobs close. Both corrections must be captured against one fresh current source/native/Knots before frame and receive an exact ROOT source reopen release; no source write may overlap a queued/running command. Run the named compact registration probe first, then the full E4 Task 4 selector with nonzero discovered cases. If matching request setup proves impossible through the genuine managed compact route, stop for a separate checked decision and disclose the reachability limit. Only after E1–E4 controls, normal fit, whole original inverse, independent source review and a new frozen final frame may all twenty final commands run fresh. This amendment adds no owner, public API, dependency, target exception, Git action, global CVLD closure or Phase 168 work.
