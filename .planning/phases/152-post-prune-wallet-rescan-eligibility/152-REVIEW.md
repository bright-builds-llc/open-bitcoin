---
phase: 152-post-prune-wallet-rescan-eligibility
reviewed: 2026-10-03T01:46:22Z
depth: standard
files_reviewed: 12
files_reviewed_list:
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/sync/wallet_rescan.rs
  - packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime.rs
  - packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime/eligibility.rs
  - packages/open-bitcoin-node/src/wallet_registry.rs
  - packages/open-bitcoin-node/src/wallet_registry/rescan.rs
  - packages/open-bitcoin-node/src/wallet_registry/rescan/tests.rs
  - packages/open-bitcoin-rpc/src/context/rescan.rs
  - packages/open-bitcoin-rpc/src/context/wallet_state.rs
  - packages/open-bitcoin-rpc/src/context/tests.rs
  - packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs
  - packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility/fixtures.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 152-2026-10-03T00-26-52
---

# Phase 152: Code Review Report

**Reviewed:** 2026-10-03T01:46:22Z
**Depth:** standard
**Files Reviewed:** 12
**Status:** clean

## Summary

Reviewed the explicit uncommitted source scope against `git diff HEAD`, including intent-to-add files, the three current plans, context, resolved research dispositions and Plans 01/02 summaries. The supplied base is `9b7695f2a165bd102f04c73a3f61a7908c072c5e`; committed-only ranges would omit this implementation. No scoped paths are ignored, and no `.claudeignore` or project skill directories were present.

The shared staged replacement checks the requested interval and selected candidates' creating heights before wallet persistence. Node jobs are identified before fallible authority reads, interrupted heads refuse at both storage and preparation boundaries, and missing metadata or payload/read failures persist sanitized Failed evidence. Durable RPC retains live manager admission authority and closes the direct supplied-snapshot helper bypass. Actual paired-delete/reopen tests assert retained coins, best-block and have-pruned evidence, full prior wallet equality, safe diagnostics and success controls. The changed-target freshness warning found during review was fixed and re-reviewed with behavioral RED-to-GREEN evidence. No unresolved findings remain in the reviewed source scope.

The local `AGENTS.md`, Bright Builds sidecar, placeholder-only overrides, architecture, code-shape, testing, verification and Rust standards materially informed this review. Both active lesson files were loaded completely (5,230 + 1,958 bytes, 2,397 estimated tokens); no archives were read. This report is part of the parent GSD lifecycle. No source modifications, commits or Cargo/Bazel commands were performed by the reviewer.

## Resolved Finding

### WR-01: Reused RPC jobs retain Fresh after their target changes — resolved

**Original severity:** Warning

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/context/rescan.rs:284-289` in the initial reviewed version; final correction at lines 300-309.

**Original issue:** `pending_rescan_job` cloned an existing job and replaced its target/hash and state with Pending, but retained the previous freshness. A successful RPC range `0..1` produced a Complete/Fresh job with target 1 and checkpoint 1. A later request through 3 persisted that same job as Pending/Fresh with target 3 and checkpoint 1. If eligibility failed and saving Failed failed, or the process stopped after saving Pending, `wallet_freshness` reported the Pending job as Fresh at target 3 even though the wallet was never scanned beyond 1. A successfully saved Failed job also carried the mismatched target/freshness. The initial later-request test at `packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs:128-145` exercised the target change and explicitly preserved the old Fresh value, so its passing result did not guard this inconsistency. This was separate from `mark_failed` preserving the successful checkpoint and did not require changing that protocol.

**Fix:** When constructing the Pending job, recalculate freshness from the retained checkpoint/next height and new target using the existing `WalletRescanState::from_progress` contract (or rebuild through `WalletRescanJob::new` and restore MTP). Keep the successful checkpoint and next height unchanged, then let `mark_failed` retain the newly coherent Pending freshness. Add changed-target regressions for both persisted Failed and failed-Failed-save/Pending states, asserting accurate `wallet_freshness`, new target hash/height and preserved full wallet/checkpoint before and after reopen.

**Resolution:** The final implementation unifies new and reused job construction, preserves checkpoint/next/MTP, and derives freshness after setting the new target: absent checkpoint gives Scanning; known checkpoint at or above the target gives Fresh; earlier checkpoint gives Partial. Job state supplies activity. The progress domain type is unavailable through the existing RPC dependencies/facade, so an equivalent small pure match avoids a dependency or export expansion. `mark_failed` remains unchanged. The reviewer inspected the final implementation and updated tests; no source edits were performed by the reviewer.

**Verification:** The executor reproduced the actual stale Pending/Fresh bug with a failing changed-target regression before correction. The final eligibility matrix passed 19/19, and the freshness-focused filter passed 4/4. `durable_changed_target_failed_save_keeps_pending_freshness_partial` directly asserts durable Pending/Partial, target 3/hash, checkpoint 1/next 2/MTP, unchanged full wallet and RPC scanning=true/Partial. The later-request regression asserts Failed/Partial before and after reopen. `durable_unknown_checkpoint_refusal_keeps_scanning_freshness` asserts None/Scanning. `durable_retained_checkpoint_at_target_refusal_keeps_fresh_metadata` performs actual prune at 1 and request stop 1, asserting Failed/Fresh, target/checkpoint 1 and preserved wallet. These are executor test results confirmed by the parent; the reviewer performed source/test inspection and `git diff --check`, which passed. Final scoped Clippy was still running at report closeout; root owns remaining verification.

## Evidence and Limits

Plan 01 records an actual paired-prune RED failure in the previous adapter, then 8 shared tests, 10 final node eligibility tests, 15 coins-migration tests and node Clippy passing. Plan 02 and parent evidence record RPC durable RED-to-GREEN, 5 construction compatibility tests, 1 existing rescanblockchain compatibility test, 39 node wallet tests, then the final 19-case eligibility and 4-case freshness results after WR-01. Earlier scoped Clippy, 0 managed findings and 911 registered breadcrumb files passed; the final test-only addition's scoped Clippy run remained pending at report closeout. These are executor/parent evidence, not new reviewer test runs. Root owns the full default native verifier and final lifecycle verification, which are not yet claimed here.

The source preserves the research's completed-prune/manual HTTP serialization boundary. Presence probes and wallet saving remain separate operations for direct library callers. No global atomic probe/save guarantee is inferred; a future concurrent retention owner remains the documented replan trigger.

## Simplification Pass

The shared helper reuses existing pure wallet selection and eliminates the node's duplicate durable filter. The remaining RPC filter serves local memory behavior. Typed safe refusal facts, the existing registry/job schema and small preparation/failure seams avoid a new matcher, storage mutation API, dependency or locking architecture. WR-01 uses one small checkpoint projection after unified Pending construction; it preserves the domain contract without extending the RPC dependency surface.

______________________________________________________________________

_Reviewed: 2026-10-03T01:46:22Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
