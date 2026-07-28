---
phase: 134-authoritative-cross-cache-lifecycle-integration
reviewed: 2026-07-28T22:27:06Z
depth: standard
files_reviewed: 68
files_reviewed_list:
  - README.md
  - docs/metrics/lines-of-code.md
  - docs/parity/catalog/mempool-policy.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/README.md
  - packages/open-bitcoin-mempool/src/pool/admission.rs
  - packages/open-bitcoin-mempool/src/pool/expiry.rs
  - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
  - packages/open-bitcoin-mempool/src/pool/package_admission.rs
  - packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs
  - packages/open-bitcoin-mempool/src/pool/tests/prepared_lifecycle_cases.rs
  - packages/open-bitcoin-mempool/src/pool/tests/prepared_maintenance_cases.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_cases.rs
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
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/oracle.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/reconciliation.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_target_cases.rs
  - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
  - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/reorg_reject_evidence.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs
  - packages/open-bitcoin-node/src/sync/session.rs
  - packages/open-bitcoin-rpc/src/dispatch.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/check-phase133-package-aware-download-orphan-bridge.ts
  - scripts/check-phase134-apply-boundaries.ts
  - scripts/check-phase134-authoritative-lifecycle.test.ts
  - scripts/check-phase134-authoritative-lifecycle.ts
  - scripts/verify.sh
findings:
  critical: 1
  warning: 11
  info: 0
  total: 12
status: issues_found
---

# Phase 134: Code Review Report

**Reviewed:** 2026-07-28T22:27:06Z
**Depth:** standard
**Files Reviewed:** 68
**Status:** issues_found

## Summary

The review covered every scoped source, test, checker, and parity-document file at standard depth. Repo guidance and the managed architecture, code-shape, testing, verification, Rust, and TypeScript standards materially informed the review, especially the requirements for one authoritative owner, bounded state, functional-core/imperative-shell separation, and evidence-backed parity claims.

The aggregate lifecycle projection is thoughtfully structured, and its targeted Phase 134 checks pass, but twelve correctness and verification defects remain. The most severe is that all independent managed-network authorities reuse the same authority epoch and effect IDs, so a receipt created by one authority can consume another authority's pending effect; for snapshots, that can incorrectly clear dirty state without persisting the second authority's mempool. Other findings affect stale validated transitions, atomic peer completion, peer-session freshness, abandoned effect reservations, orphan alias cleanup, bounded package work, reconciliation completeness, and the accuracy of phase-completion guardrails.

Targeted verification completed during review:

- `bun run scripts/check-phase134-apply-boundaries.ts`
- `bun test scripts/check-phase134-authoritative-lifecycle.test.ts` — 88 passed, 0 failed
- `bun run scripts/check-phase134-authoritative-lifecycle.ts`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node lifecycle_projection_cases` — 44 passed, 0 failed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc announcement_successful_prefix` — 5 passed, 0 failed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node announcement_transport_cases` — 12 passed, 0 failed
- `jq empty docs/parity/index.json docs/parity/source-breadcrumbs.json`

These findings are not detected by those passing checks. The full `bash scripts/verify.sh` repository contract was not rerun as part of this read-only review.

## Critical Issues

### CR-01: Effect receipts are not bound to a unique authority incarnation

**Files:**

- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/relay_serving.rs:510-534`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/lifecycle_projection.rs:38-55`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs:84-125`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/lifecycle_effects.rs:291-344`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/lifecycle_effects.rs:361-414`

**Issue:** Every `ManagedPeerNetwork` starts with `AuthorityEpoch::INITIAL`, and each peer/snapshot ledger starts allocating IDs from zero. Completion looks up and removes pending state by the raw effect ID before checking the rest of the receipt binding. Two independent handles therefore produce identical `(epoch, generation, effect_id)` values. Passing authority A's receipt to authority B can consume B's pending effect and return `Applied`. In the snapshot family, it can also clear B's `dirty_generation` even though the bytes written came from A. A later crash can therefore lose B's mempool state.

**Fix:** Give every authority incarnation a genuinely unique, persisted or process-unique identity, make that identity part of the ledger key, and atomically take a pending entry only after the complete binding matches. For example:

```rust
let binding = receipt.binding();
let Some(pending) = network.snapshot_effect_ledger.take_exact(binding) else {
    return Ok(EffectCompletion::AchievedButStale);
};
if pending.persistence_generation == network.lifecycle_generation {
    network.dirty_generation = None;
}
```

Add a regression test with two independent handles whose first effect IDs are both zero; neither peer nor snapshot receipts from one handle may alter the other.

## Warnings

### WR-01: Accepted-package count is unbounded

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs:343-366`

**Issue:** `validate_identity_work_bounds` caps admissions, teardowns, and each package's member count, but never caps `input.accepted_packages.len()`. Preparation then allocates one `fingerprint_admissions` entry per input package at lines 491-510, including repeated identical fingerprints. An otherwise bounded lifecycle command can therefore perform and retain arbitrarily large work.

**Fix:** Reject an accepted-package count above a named lifecycle bound and emit an admission only when insertion changes the prospective map:

```rust
if input.accepted_packages.len() > PHASE102_MAX_ORPHAN_TRANSACTIONS {
    return Err(PeerTransactionLifecyclePreparationError::PackageCountLimit {
        count: input.accepted_packages.len(),
        maximum: PHASE102_MAX_ORPHAN_TRANSACTIONS,
    });
}
```

### WR-02: Txid-alias orphan removal leaves stale candidate cursors undetected

**Files:**

- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs:464-480`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs:163-174`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer/transaction_lifecycle/reconciliation.rs:73-89`

**Issue:** Orphans are removed when either their txid or wtxid matches an affected identity, but child cursor cleanup compares only affected wtxids. If an existing orphan and a canonical transaction have the same txid but different witness identities, the orphan is removed by txid while cursors that reference its old wtxid survive. `remove_orphan_without_candidate_scan` deliberately does not repair cursors, and reconciliation repeats the wtxid-only child test, so the stale cursor can be reported as clean.

**Fix:** Build an exact set of wtxids for the orphan entries selected for removal and remove every cursor whose child set intersects it. Reconciliation must resolve cursor child wtxids back to orphan txids, or retain enough identity information to compare both aliases. Add a same-txid/different-wtxid regression case that asserts both cursor removal and non-zero mismatch detection before repair.

### WR-03: Fingerprint capacity is checked before same-transition retirements

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs:484-524`

**Issue:** New package fingerprints are inserted and the maximum is checked before fingerprints intersecting teardown members are retired. At exact capacity, an atomic transition that retires one fingerprint and admits one replacement is rejected at the transient size of `limit + 1`, even though its committed size would remain within the bound.

**Fix:** Determine and remove all same-transition retirements from the prospective map before applying admissions, then validate the final committed size:

```rust
for fingerprint in &fingerprint_retirements {
    prospective_fingerprints.remove(fingerprint);
}
for package in &input.accepted_packages {
    // validate and insert the replacement
}
validate_fingerprint_capacity(prospective_fingerprints.len())?;
```

### WR-04: Public validate/apply split allows a stale capability to overwrite newer mempool state

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs:223-255`

**Issue:** `validate_prepared_mempool_transition` returns an owned capability with no borrow tying it to the current mempool. A caller can validate revision N, mutate the same `Mempool` through another public operation, and then apply the old capability. The apply path performs no revision check, so an old patch can overwrite resource, topology, and rolling-fee state and advance the revision from stale data. The node lock follows the documented sequencing, but the public safe Rust API does not enforce it.

**Fix:** Encode exclusivity in the capability rather than relying on a comment. One option is a validated guard that retains the mutable borrow and exposes only consuming `commit`:

```rust
pub struct ValidatedMempoolTransition<'a> {
    mempool: &'a mut Mempool,
    transition: PreparedMempoolTransition,
}

impl ValidatedMempoolTransition<'_> {
    pub fn commit(self) -> MempoolLifecycleDelta {
        self.mempool.apply_validated_core(self.transition)
    }
}
```

Add a compile-fail or behavioral regression proving that an intervening mutation cannot coexist with a validated capability.

### WR-05: Peer effect completion and evidence recording are not atomic

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/runtime_authority/effects.rs:89-100`

**Issue:** `complete_peer_emission` completes the effect under one authority lock, releases it, and then acquires the lock again to record evidence. A disconnect/reconnect or other session mutation can occur between those operations. The old receipt may be classified `Applied`, then have its evidence attached to a new session—or fail after the ledger has irreversibly recorded completion. This bypasses the phase's claimed single-dispatcher atomicity.

**Fix:** Add a typed `CompletePeerEmission` lifecycle command containing both the effect receipt and its evidence, then validate the binding, consume the pending effect, and record evidence inside one dispatcher critical section:

```rust
LifecycleCommand::CompletePeerEmission { receipt, evidence } => {
    let completion = network.complete_exact_peer_effect(receipt)?;
    if completion == EffectCompletion::Applied {
        network.record_peer_emission(receipt.peer_id(), evidence);
    }
    Ok(LifecycleCommandResult::PeerEffectCompleted(completion))
}
```

### WR-06: Unrelated peer churn makes valid emissions stale

**Files:**

- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network.rs:99-106`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network.rs:296-307`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/action_translation.rs:103-127`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs:72-101`

**Issue:** A single global `peer_session_generation` is captured for every peer effect and incremented whenever any peer connects or disconnects. An emission prepared for peer A is therefore rejected as stale if unrelated peer B churns before A's successful write is acknowledged. The outward effect has occurred, but its authoritative completion and evidence are discarded.

**Fix:** Keep a monotonic allocation counter if desired, but store the current session identity per peer and bind each capability to its target peer's session:

```rust
let session = network.peer_sessions.current(request.peer_id)?;
PeerEffectCapability::new(
    network.authority_epoch,
    network.lifecycle_generation,
    effect_id,
    request.peer_id,
    session,
)
```

Add a regression where B reconnects between A's preparation and completion; A must still complete, while reconnecting A must invalidate its old receipt.

### WR-07: Failed or dropped capabilities permanently consume pending-effect capacity

**Files:**

- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs:58-83`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/lifecycle_projection.rs:546-559`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/lifecycle_effects.rs:16-19`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/lifecycle_effects.rs:313-344`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/lifecycle_effects.rs:383-414`
- `/Users/peterryszkiewicz/Repos/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs:332-360`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/session.rs:538-558`

**Issue:** Preparation reserves pending capacity, but the command vocabulary has no cancel/abort operation. Encode failures, target mismatches, rejected/disconnected writes, unsent suffixes, and dropped capabilities leave their reservations forever. One failed snapshot consumes the sole snapshot slot; repeated peer failures eventually consume all 128 peer slots and disable future relay preparation.

**Fix:** Add affine, consuming abort commands for failures known to occur before achievement, and route every early-return/unsent-suffix path through them:

```rust
LifecycleCommand::AbortPeerEffect(capability)
LifecycleCommand::AbortSnapshotEffect(prepared_snapshot)
```

The abort handler should release only an exact pending binding and grant no success credit. For ambiguous partial writes, add an explicit reconciliation state rather than silently freeing or permanently leaking the slot.

### WR-08: Unbroadcast reconciliation checks only unexpected members, not missing expected members

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs:151-165`

**Issue:** `unbroadcast_mismatch_count` iterates only the current `unbroadcast_members` set. It detects stale or ineligible entries, but cannot detect an eligible locally submitted/rebroadcast-requested mempool member that is missing from the set. Deleting an expected member can therefore leave reconciliation reporting zero mismatches.

**Fix:** Derive the expected eligible set from canonical mempool metadata and compare both directions:

```rust
let expected = canonical
    .iter()
    .filter(|member| retry_eligible_and_requested(member))
    .copied()
    .collect::<BTreeSet<_>>();
symmetric_difference_count(&self.unbroadcast_members, &expected)
```

Add a corruption test that removes one expected member and requires reconciliation to detect and repair it.

### WR-09: Apply-boundary checker can be bypassed through an effectful helper

**Files:**

- `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase134-apply-boundaries.ts:123-145`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase134-apply-boundaries.ts:175-190`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase134-authoritative-lifecycle.test.ts:117-143`

**Issue:** The checker scans only the lexical body of exact `apply_prepared_*` functions for direct I/O tokens and discovers new targets only by that name prefix. An apply body can call an innocuously named helper that performs I/O, decoding, or fallible derivation and still pass. The mutation tests cover only direct forbidden syntax, so the advertised structural guarantee is weaker than the behavior it protects.

**Fix:** Use Rust syntax/semantic analysis to inspect the transitive call graph, or enforce a small explicit allowlist of pure helper calls and reject every unclassified callee. Add a mutation that inserts `persist_projection(plan);` in an apply function and defines `persist_projection` with `std::fs::write`; the checker must fail.

### WR-10: Canonical parity surfaces claim completion before their stated verification gate

**Files:**

- `/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/index.json:3181-3189`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/index.json:3246-3252`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/checklist.md:77`

**Issue:** The machine index and human checklist mark Phase 134's parity surface `done`, while both also state that MPLIFE-01 through MPLIFE-04 remain pending until phase-level verification. No Phase 134 verification artifact exists yet. The canonical parity ledger therefore presents a completed status before its own evidence gate has been satisfied.

**Fix:** Keep both statuses `in_progress` until phase verification exists and the requirement ledger is updated, then promote the JSON and checklist atomically:

```json
"status": "in_progress"
```

The phase checker should reject `done` unless the verification artifact and completed requirement traceability are both present.

### WR-11: Deferred-scope claim guard ignores canonical parity documents and wording variants

**Files:**

- `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase134-authoritative-lifecycle.ts:20-49`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase134-authoritative-lifecycle.ts:418-434`
- `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase134-authoritative-lifecycle.test.ts:379-407`

**Issue:** The guard checks ten exact English sentences only in `README.md`. It does not scan the machine parity index, parity checklist, mempool-policy catalog, or package README, and equivalent wording bypasses it. The mutation suite reinforces the blind spot by appending every prohibited claim only to the root README. A Phase 135-138, public-relay, or production-readiness overclaim can therefore enter an authoritative public surface without failing verification.

**Fix:** Define a claim corpus containing every canonical public/parity surface and prefer structured status/known-gap assertions or normalized prohibited-claim patterns over exact sentences. Add mutations for each surface and representative wording variants, such as “production-ready” and “public relay is enabled by default.”

***

_Reviewed: 2026-07-28T22:27:06Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
