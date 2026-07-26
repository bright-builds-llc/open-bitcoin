---
phase: 133-package-aware-download-and-orphan-bridge
reviewed: 2026-07-26T22:14:43Z
depth: standard
files_reviewed: 52
files_reviewed_list:
  - README.md
  - docs/parity/catalog/mempool-policy.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/README.md
  - packages/open-bitcoin-mempool/src/package/report.rs
  - packages/open-bitcoin-mempool/src/package/tests.rs
  - packages/open-bitcoin-mempool/src/pool/candidate.rs
  - packages/open-bitcoin-mempool/src/pool/package_admission.rs
  - packages/open-bitcoin-mempool/src/pool/package_admission/residual.rs
  - packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/inventory_state.rs
  - packages/open-bitcoin-network/src/peer/relay_download.rs
  - packages/open-bitcoin-network/src/peer/tests.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/serving.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/fanout_cases.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/reject_evidence_cases.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/edge_cases.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/received_cases.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/serving_cases.rs
  - packages/open-bitcoin-network/tests/parity.rs
  - packages/open-bitcoin-node/src/mempool.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/action_translation.rs
  - packages/open-bitcoin-node/src/network/admission_bridge.rs
  - packages/open-bitcoin-node/src/network/admission_bridge/package.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs
  - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
  - packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs
  - packages/open-bitcoin-node/src/network/types.rs
  - packages/open-bitcoin-rpc/src/dispatch.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests.rs
  - scripts/check-phase102-orphan-admission-bridge.test.ts
  - scripts/check-phase102-orphan-admission-bridge.ts
  - scripts/check-phase133-package-aware-download-orphan-bridge.test.ts
  - scripts/check-phase133-package-aware-download-orphan-bridge.ts
  - scripts/verify.sh
findings:
  critical: 1
  warning: 3
  info: 0
  total: 4
status: issues_found
---

# Phase 133: Code Review Report

**Reviewed:** 2026-07-26T22:14:43Z
**Depth:** standard
**Files Reviewed:** 52
**Status:** issues_found

## Summary

The Phase 133 package-aware download, orphan-candidate, reject-evidence, authoritative package-admission, documentation, and source-guard changes were reviewed in their exact 52-file scope. The review applied the repository's local parity, functional-core/imperative-shell, resource-bound, verification, Rust, TypeScript, and test-reliability rules, together with the Bright Builds standards and active lessons. No active standards override changed the assessment.

The new bridge has one remotely triggerable memory-amplification path, one orphan-accounting cap bypass, and one singleton rejection-category regression. The Phase 133 source guard also gives false confidence for the two resource-bound failures because it checks declarations and test names rather than the required behavior.

Focused verification passed:

- `bun test scripts/check-phase133-package-aware-download-orphan-bridge.test.ts` — 22 passed, 0 failed
- `bun run scripts/check-phase133-package-aware-download-orphan-bridge.ts` — passed
- `bun test scripts/check-phase102-orphan-admission-bridge.test.ts` — 10 passed, 0 failed
- `bun run scripts/check-phase102-orphan-admission-bridge.ts` — passed
- `git diff --check 12c975b2^..HEAD -- . ':!.planning/'` — passed

Per the review request, no broad Cargo build or test command was run against the absent default target directory.

## Critical Issues

### CR-01: Candidate cursors duplicate remotely supplied orphan transaction bodies

**File:** `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs:48-99`

**Issue:** `SamePeerCandidateCursor` owns a full parent `Transaction` and a `Vec` containing a cloned `Transaction` for every eligible child. `begin_same_peer_candidate` eagerly clones all matching orphan bodies before the traversal limit is applied, and the cursor can remain stored in `candidate_cursors` while subsequent candidates are processed. A remote peer can therefore turn the bounded orphan store into a second, potentially repeated allocation of large transaction bodies. The configured count and traversal caps do not bound retained bytes, and this contradicts the documented PPKG-02 guarantee that orphan bodies are stored once. This is a network-reachable memory-exhaustion risk.

**Fix:** Store only child identities in the persistent cursor and resolve one child body from the canonical orphan map when advancing. Do not retain cloned transaction bodies for unvisited candidates. Add an aggregate retained-byte or weight budget covering orphan bodies and cursor state so count bounds cannot hide oversized allocations.

```rust
pub(super) struct SamePeerCandidateCursor {
    pub(super) parent: Transaction,
    pub(super) parent_peer: PeerId,
    pub(super) child_wtxids: Vec<Wtxid>,
    pub(super) next_child: usize,
    pub(super) visited: usize,
}

// In advance_same_peer_candidate:
let child = self.orphans.get(child_wtxid)?;
```

Add a regression test that fills the candidate set with large orphan bodies and proves persistent cursor state retains identities/provenance only, with an explicit aggregate byte bound.

## Warnings

### WR-01: Late announcers bypass the per-peer orphan cap

**File:** `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:299-310`

**Issue:** `add_announcer` increments `orphan_count_by_peer` without checking `max_orphans_per_peer` and without invoking cap enforcement. The late-inventory path calls this method directly at `packages/open-bitcoin-network/src/peer/inventory_state.rs:113-120`. A peer already at its orphan limit can therefore announce additional retained orphans and push its count beyond the configured per-peer cap, contradicting the PPKG-02 bounded-accounting contract.

**Fix:** Before adding a new peer association, reject it when `peer_len(peer_id) >= max_orphans_per_peer`, or return/apply the same deterministic eviction actions used by initial orphan staging. Add a regression test with a per-peer cap of one where the same late peer attempts to attach to two bodies initially retained by other peers.

### WR-02: Singleton package admission collapses typed policy failures into internal invariants

**File:** `packages/open-bitcoin-node/src/network/admission_bridge/package.rs:329-372`

**Issue:** The singleton peer-transaction path now converts package reports back into `MempoolOutcome`, but `hard_rejection_category` maps every `HardMemberFailure::Policy` to `InternalInvariant`. The package layer creates that variant for any `MempoolError` at `packages/open-bitcoin-mempool/src/pool/package_admission.rs:513-517`, discarding whether the actual failure was validation, non-standardness, conflict, a limit, or a real invariant. The previous typed mapping distinguished these categories. Downstream reject evidence and operator-facing outcomes can now misclassify ordinary peer transaction rejections as internal faults.

**Fix:** Preserve a typed rejection category, or the original typed policy error, in `HardMemberFailure::Policy` and use it in `hard_rejection_category`. Add bridge tests that drive validation, non-standard, conflict, limit, and internal-invariant failures through the singleton package path and assert their exact `MempoolRejectionCategory`.

### WR-03: The Phase 133 guard does not enforce its resource-bound claims

**File:** `scripts/check-phase133-package-aware-download-orphan-bridge.ts:157-213`

**Issue:** `checkBoundedOrphanCandidate` labels its checks as proof that orphan bodies, per-peer totals, and traversal are independently bounded, but it only requires constants, field declarations, source fragments, and three test names. It accepts both CR-01's body-cloning cursor and WR-01's uncapped late-announcer path; the focused checker and all 22 checker tests pass with those bugs present. This makes the completion gate unreliable for PPKG-02 and allows the parity ledger to claim a resource contract that the implementation does not meet.

**Fix:** Add behavior-focused Rust tests for the late-announcer per-peer cap and cursor retained-body/byte accounting, then make the guard require those specific regression oracles. Mutation tests should delete or bypass the actual cap check and reintroduce transaction-body storage in the cursor, proving the checker fails for each semantic regression instead of only for renamed source tokens.

______________________________________________________________________

_Reviewed: 2026-07-26T22:14:43Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
