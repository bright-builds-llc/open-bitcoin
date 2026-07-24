---
phase: 130-resource-time-and-fee-primitives
reviewed: 2026-07-24T08:32:58Z
depth: standard
files_reviewed: 55
files_reviewed_list:
  - README.md
  - docs/parity/README.md
  - docs/parity/catalog/mempool-policy.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/README.md
  - packages/open-bitcoin-cli/tests/operator_binary.rs
  - packages/open-bitcoin-mempool/src/context.rs
  - packages/open-bitcoin-mempool/src/fee.rs
  - packages/open-bitcoin-mempool/src/pool.rs
  - packages/open-bitcoin-mempool/src/pool/admission.rs
  - packages/open-bitcoin-mempool/src/pool/admission_outcome.rs
  - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
  - packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs
  - packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs
  - packages/open-bitcoin-mempool/src/pool/tests/lifecycle_delta_cases.rs
  - packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs
  - packages/open-bitcoin-mempool/src/resource.rs
  - packages/open-bitcoin-mempool/src/types.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs
  - packages/open-bitcoin-node/src/mempool.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/admission_bridge.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/recovery.rs
  - packages/open-bitcoin-node/src/network/relay_fanout.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/tests.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs
  - packages/open-bitcoin-node/src/network/tests/compact_receive_cases.rs
  - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
  - packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs
  - packages/open-bitcoin-node/src/network/types.rs
  - packages/open-bitcoin-node/src/storage/mempool_snapshot.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/dispatch/node.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests.rs
  - packages/open-bitcoin-rpc/src/method/node.rs
  - scripts/check-current-documentation-reconciliation.test.ts
  - scripts/check-current-documentation-reconciliation.ts
  - scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts
  - scripts/check-phase130-resource-time-fee-primitives.test.ts
  - scripts/check-phase130-resource-time-fee-primitives.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 3
  info: 3
  total: 6
status: issues_found
---

# Phase 130: Code Review Report

**Reviewed:** 2026-07-24T08:32:58Z
**Depth:** standard
**Files Reviewed:** 55
**Status:** issues_found

## Summary

Phase 130’s resource, fee-role, metadata/context, lifecycle-delta, and shell-injection contracts are coherent and largely fail-closed: typed newtypes prevent role confusion, snapshot metadata is all-or-none, retry jitter is bounded, and RPC `getmempoolinfo` preserves distinct Knots meanings. Three warnings remain around error classification / recovery ordering, plus residual fail-closed wallet admission and stale removal claims on deprecated adapters.

## Warnings

### WR-01: Local RPC maps every `Rejected` category to `InternalInvariant`

**File:** `packages/open-bitcoin-rpc/src/dispatch/node.rs:343-347`
**Issue:** `send_raw_transaction_response` collapses all `MempoolOutcome::Rejected` categories (`RelayFeeTooLow`, `NonStandard`, `ReplacementRejected`, etc.) into `MempoolError::InternalInvariant`. Authenticated callers lose the typed policy reason and may treat ordinary policy rejects as internal failures.
**Fix:** Map each `MempoolRejectionCategory` back to the corresponding `MempoolError` (or a dedicated RPC reject payload) before `mempool_outcome_failure`:

```rust
MempoolOutcome::Rejected { category, .. } => Err(mempool_outcome_failure(
    mempool_error_from_rejection_category(category),
)),
```

### WR-02: Hard mempool invariant failures become soft `Rejected` + recent-reject side effects

**File:** `packages/open-bitcoin-mempool/src/pool/admission_outcome.rs:78-81`
**Issue:** `accept` converts every remaining `MempoolError`—including `InternalInvariant` from resource-ledger / lifecycle-builder failures—into `MempoolOutcome::Rejected`. Peer admission then treats that as a normal reject and records recent-reject state (`admission_bridge.rs:98-103`), which can poison reject filters when the real problem is an internal accounting/lifecycle bug.
**Fix:** Propagate `MempoolError::InternalInvariant` (and any other non-policy failures) as `Err(...)` from the transition API, or branch peer/local bridges so `InternalInvariant` skips recent-reject bookkeeping and surfaces as a hard managed-network error.

### WR-03: Snapshot recovery admits in txid order, so parent/child packages can be dropped

**File:** `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs:71`
**Issue:** `from_mempool` sorts records by `txid`, and both pure replay and managed recovery (`recovery.rs:91-121`) perform a single forward pass. A child whose txid sorts before its parent is classified `DroppedMissingParent` even when the parent is present later in the same snapshot, permanently losing that package from recovered mempool state.
**Fix:** Recover in dependency order (e.g. topological sort by spent mempool outpoints), or run a bounded multi-pass reconsider until a full pass admits nothing new:

```rust
// Prefer: sort so parents precede children, then single pass;
// or: loop replay until recovered_count stops increasing.
```

## Info

### IN-01: Stale “Plan 130-11 removes this adapter” claims

**File:** `packages/open-bitcoin-node/src/mempool.rs:86-113`
**Issue:** Comments/deprecation notes still say Plan 130-11 removes the no-context adapters, but `submit_transaction` / `submit_transaction_outcome` remain (wallet/compatibility). Similar stale wording exists on `accept_transaction_outcome*` in `pool/admission.rs:236-238`.
**Fix:** Update notes to name the real remaining owner (wallet/`AdmissionResult` migration) or remove the false “already removed by 130-11” claim.

### IN-02: Wallet local admission still uses fail-closed legacy metadata

**File:** `packages/open-bitcoin-rpc/src/context/network.rs:473-480`
**Issue:** The retained `submit_local_transaction` path injects `AdmissionContext::legacy_unknown()` (via the node bridge). Wallet-broadcast txs therefore lack `Known` acceptance time, `Local` origin, and `Requested` relay intent, and skip `record_local_submission_outcome` fanout/rebroadcast recording. Plan 130-11 documents this as intentional, but it remains an operator-visible gap versus `sendrawtransaction`.
**Fix:** Migrate wallet submission to `submit_local_transaction_with_relay_evidence_at` (shell-sampled time + activation-resolved relay intent) and delete the deprecated `AdmissionResult` adapter once callers are gone.

### IN-03: `FeeRate` silently clamps oversized virtual sizes to `i64::MAX`

**File:** `packages/open-bitcoin-mempool/src/fee.rs:38-40`
**Issue:** `i64::try_from(virtual_size.as_usize()).unwrap_or(i64::MAX)` masks conversion failure instead of returning an error. Unreachable for realistic Bitcoin vsizes on 64-bit targets, but it weakens the otherwise checked arithmetic posture of Phase 130.
**Fix:** Prefer `checked` conversion that returns a typed fee/validation error on overflow, matching `ResourceAccountingError` style.

---

_Reviewed: 2026-07-24T08:32:58Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
