---
phase: 134-authoritative-cross-cache-lifecycle-integration
reviewed: 2026-07-30T00:05:11Z
depth: standard
files_reviewed: 83
files_reviewed_list:
  - README.md
  - docs/metrics/lines-of-code.md
  - docs/parity/catalog/mempool-policy.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/README.md
  - packages/open-bitcoin-mempool/src/lib.rs
  - packages/open-bitcoin-mempool/src/pool.rs
  - packages/open-bitcoin-mempool/src/pool/admission.rs
  - packages/open-bitcoin-mempool/src/pool/expiry.rs
  - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
  - packages/open-bitcoin-mempool/src/pool/package_admission.rs
  - packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs
  - packages/open-bitcoin-mempool/src/pool/tests/prepared_lifecycle_cases.rs
  - packages/open-bitcoin-mempool/src/pool/tests/prepared_maintenance_cases.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_cases/bounded_packages.rs
  - packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_cases/identity_aliases.rs
  - packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_independence_cases.rs
  - packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs
  - packages/open-bitcoin-network/src/peer/transaction_lifecycle/reconciliation.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs
  - packages/open-bitcoin-node/src/mempool.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/action_translation.rs
  - packages/open-bitcoin-node/src/network/admission_bridge.rs
  - packages/open-bitcoin-node/src/network/admission_bridge/package.rs
  - packages/open-bitcoin-node/src/network/admission_bridge/singleton.rs
  - packages/open-bitcoin-node/src/network/announcement_transport.rs
  - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/network/lifecycle_effects.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/tests.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/relay_fanout.rs
  - packages/open-bitcoin-node/src/network/relay_fanout/lifecycle.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission/partial_package.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts/peer_abort.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts/peer_sessions.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts/snapshot_abort.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/oracle.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/reconciliation.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_target_cases.rs
  - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
  - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/reorg_reject_evidence.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs
  - packages/open-bitcoin-node/src/sync/session.rs
  - packages/open-bitcoin-node/src/sync/session/emission_terminal.rs
  - packages/open-bitcoin-rpc/src/dispatch.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs
  - scripts/check-phase122-compact-relay-peer-completion.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
  - scripts/check-phase126-compact-relay-residual-hardening.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/check-phase133-package-aware-download-orphan-bridge.ts
  - scripts/check-phase134-apply-boundaries.ts
  - scripts/check-phase134-authoritative-lifecycle.test.ts
  - scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts
  - scripts/check-phase134-authoritative-lifecycle.test/scope-claims.ts
  - scripts/check-phase134-authoritative-lifecycle.ts
  - scripts/check-phase134-authoritative-lifecycle/scope.ts
  - scripts/verify.sh
findings:
  critical: 1
  warning: 1
  info: 0
  total: 2
status: issues_found
---

# Phase 134: Code Review Report

**Reviewed:** 2026-07-30T00:05:11Z
**Depth:** standard
**Files Reviewed:** 83
**Status:** issues_found

## Summary

The review covered all 83 scoped source, test, checker, storage, RPC, and parity-document files against the current Phase 134 implementation. Repo guidance and the managed architecture, code-shape, testing, verification, Rust, and TypeScript standards materially informed the review, especially the requirements for preflight-before-mutation, one authoritative lifecycle owner, bounded retained state, exact external-effect accounting, and auditable parity claims.

The repairs in Plans 14-24 resolve the twelve findings from the previous report: authority incarnations and effect bindings are now exact, peer sessions and completions are peer-local and atomic, failed peer/snapshot effects have terminal abort paths, identity cleanup and reconciliation are symmetric, accepted-package state is bounded and replacement-aware, mempool and aggregate lifecycle commits reject stale work atomically, the apply checker follows transitive calls, and parity remains explicitly `in_progress`. Those stale findings are not carried forward.

Two correctness issues remain. Most seriously, block connection and reorganization commit chainstate before executing a still-fallible mempool/cross-cache projection, so an error can be returned after authoritative state has already changed. Separately, the peer projection applies orphan-cache limits to whole canonical lifecycle deltas, rejecting valid block or maintenance removals above 100 members (or more than 32 fingerprint retirements).

Targeted verification completed during this review:

- `bun run scripts/check-phase134-apply-boundaries.ts`
- `bun test scripts/check-phase134-authoritative-lifecycle.test.ts` — 172 passed, 0 failed
- `bun run scripts/check-phase134-authoritative-lifecycle.ts`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network transaction_lifecycle_cases` — 28 passed, 0 failed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node lifecycle_projection_cases` — 72 passed, 0 failed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node mempool_lifecycle_cases` — 17 passed, 0 failed
- `jq empty docs/parity/index.json docs/parity/source-breadcrumbs.json`

The full `bash scripts/verify.sh` repository contract was not rerun during this read-only review. The passing targeted checks do not cover either remaining failure mode.

## Critical Issues

### CR-01: Block and reorg APIs can fail after chainstate has already committed

**Files:**

- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:40-58`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:62-116`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:119-151`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:193-247`

**Issue:** `connect_local_block`, `connect_stored_block`, and `reorg_to_branch` mutate chainstate first, then update tip-related peer/maps state, and only afterward call the fallible lifecycle projection. `LifecycleProjectionPlan::prepare` and `apply_lifecycle_command` can still return errors, including the reachable peer-work limits described in WR-01. A connected-block caller can therefore receive `Err` while the block is already on the active chain and its confirmed/conflicting transactions remain in the mempool and dependent caches. Retrying `connect_stored_block` takes the duplicate fast path at lines 70-81 and never repairs the skipped lifecycle. Reorg handling is worse: its connected-block and reconsideration steps are applied sequentially after the chainstate reorg, so a later failure can leave both a committed reorg and a partially projected mempool.

This violates the phase's preflight-before-mutation contract and can persist a chainstate/mempool split through the dirty-snapshot path.

**Fix:** Make chainstate and lifecycle projection one recoverable transaction boundary. For a normal block, prepare and seal every fallible mempool/cross-cache consequence before connecting chainstate; after a successful chainstate commit, run only an infallible sealed projection. For reorgs, stage the complete sequential mempool result against a temporary state (or add a durable rollback/recovery journal) before exposing the chainstate transition. No return path after chainstate mutation should report failure without either rolling chainstate back or deterministically completing/recovering the projection. Add public-path failure-injection tests for block connect and reorg that assert each operation is either a complete no-op or a complete aggregate commit, and that retry always converges.

## Warnings

### WR-01: Orphan-policy limits reject valid whole-mempool lifecycle deltas

**Files:**

- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:28-31`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs:370-390`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs:526-539`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/lifecycle_projection.rs:501-530`

**Issue:** `prepare_peer_projection` forwards every canonical admission and teardown to `PeerManager::prepare_transaction_lifecycle`, but that function caps the complete admission and teardown vectors at `PHASE102_MAX_ORPHAN_TRANSACTIONS` (100). It also rejects cleanup of more than `PHASE102_MAX_RECONSIDERATIONS_PER_PARENT` (32) accepted-package fingerprints even though the retained fingerprint cache itself can hold 100. These are orphan-cache policy limits, not protocol or mempool lifecycle limits. The default mempool capacity is 300 MB, and a valid block can confirm or conflict with far more than 100 mempool members. Expiry, pressure, or reorg maintenance can likewise produce a larger teardown. Such a legitimate transition fails peer preparation instead of updating the bounded peer caches.

**Fix:** Bound retained peer-cache state, not the size of authoritative cleanup required to keep it consistent. Prepare an exact replacement/delta that can consume every canonical teardown, or chunk internal cleanup behind one sealed aggregate commit without exposing partial lifecycle state. Fingerprint retirement must be able to retire every entry in the bounded retained cache; the unrelated per-parent reconsideration cap should not limit it. Add integration cases for a connected block removing more than 100 mempool members and for cleanup of more than 32 independent retained fingerprints; both must commit with every projection reconciled.

***

_Reviewed: 2026-07-30T00:05:11Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
