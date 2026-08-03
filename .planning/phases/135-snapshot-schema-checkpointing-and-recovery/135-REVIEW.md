---
phase: 135-snapshot-schema-checkpointing-and-recovery
reviewed: 2026-08-03T03:32:25Z
depth: standard
files_reviewed: 45
files_reviewed_list:
  - docs/metrics/lines-of-code.md
  - docs/parity/catalog/mempool-policy.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-mempool/src/pool.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/checkpoint.rs
  - packages/open-bitcoin-node/src/network/checkpoint/tests.rs
  - packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/checkpoint.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs
  - packages/open-bitcoin-node/src/network/recovery.rs
  - packages/open-bitcoin-node/src/network/recovery/staging.rs
  - packages/open-bitcoin-node/src/network/recovery/topology.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/recovery.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/capture.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/evidence.rs
  - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/connected_block_removal.rs
  - packages/open-bitcoin-node/src/network/tests/recovery_cases.rs
  - packages/open-bitcoin-node/src/network/tests/recovery_cases/metadata.rs
  - packages/open-bitcoin-node/src/network/tests/recovery_cases/staging.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/load_limits.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/write_execution_failures.rs
  - packages/open-bitcoin-node/src/storage/mempool_snapshot.rs
  - packages/open-bitcoin-node/src/storage/mempool_snapshot/tests.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_limits.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/checkpoint.rs
  - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - scripts/check-phase134-authoritative-lifecycle.test.ts
  - scripts/check-phase135-snapshot-recovery.test.ts
  - scripts/check-phase135-snapshot-recovery.ts
  - scripts/verify.sh
findings:
  critical: 1
  warning: 7
  info: 2
  total: 10
status: issues_found
---

# Phase 135: Code Review Report

**Reviewed:** 2026-08-03T03:32:25Z  
**Depth:** standard  
**Files Reviewed:** 45  
**Status:** issues_found

## Summary

Phase 135 establishes a strong affine checkpoint shape and broad recovery test coverage, but the implementation still has one startup resource-exhaustion path and seven correctness gaps across snapshot sizing, recovery freshness, exact unbroadcast identity, context bootstrap, classification evidence, operator counters, and abort retryability. Two additional traceability/checker weaknesses can let regressions escape the phase guard.

The review applied the repository's functional-core/imperative-shell boundary, fail-closed error handling, exact parity evidence, bounded-input, and behavioral-test requirements from `AGENTS.md`, `AGENTS.bright-builds.md`, and the relevant architecture, code-shape, testing, verification, Rust, and TypeScript standards.

Focused verification passed:

- `bun test scripts/check-phase135-snapshot-recovery.test.ts`
- `bun run scripts/check-phase135-snapshot-recovery.ts`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features recovery` (50 passed)
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features snapshot_persistence` (13 passed)
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --bin open-bitcoind checkpoint` (10 passed)
- `jq empty docs/parity/index.json docs/parity/source-breadcrumbs.json`
- `git diff --check 78082343..HEAD`

## Critical Issues

### CR-01: Snapshot resource limits are applied after large allocations

**Files:** `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs:52-66,207-214`; `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs:126-143`  
**Issue:** `load_mempool_snapshot_with_limits` first asks Fjall for the complete value, whose shared storage helper copies it into a `Vec<u8>`, and only then checks `max_encoded_bytes`. The codec then deserializes the complete JSON DTO before `preflight_payload` checks record counts, per-transaction bytes, or aggregate transaction bytes. `from_policy` compounds this by treating the byte-valued 300,000,000-byte mempool capacity as both a 300,000,000-record limit and an approximately 1.2 GB encoded-value limit, bypassing the codec's 50,000-record default until after DTO construction. A corrupt persisted snapshot can therefore exhaust process memory and abort startup before the typed `ResourceBoundExceeded` path runs.

**Fix:** Clamp policy-derived record limits to `MAX_MEMPOOL_SNAPSHOT_RECORDS`; inspect/reject the stored value length before cloning it; and decode records and transaction byte sequences with a bounded streaming/custom Serde visitor that stops as soon as any count or byte budget is crossed. Add a loader test that proves an over-limit Fjall value is rejected before full materialization and a codec test that proves sequence limits fire during deserialization rather than after DTO allocation.

## Warnings

### WR-01: Policy-derived encoded limit can reject snapshots written by the same node

**Files:** `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs:52-66`; `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs:114-119`  
**Issue:** The load bound assumes encoded data needs at most `4 * mempool_capacity + 1 MiB`, while the shared encoder serializes `Vec<u8>` transaction bodies as pretty-printed JSON arrays. Decimal byte values plus indentation, commas, and newlines can require far more than four encoded bytes per transaction byte. Checkpoint writes do not enforce the derived load bound, so a valid near-capacity checkpoint can be persisted successfully and then rejected as resource exhaustion on the next startup.

**Fix:** Use a compact bounded transaction encoding such as hex/base64 or a binary snapshot format, or derive and enforce a proven upper bound for the exact serializer on both save and load. Add a near-policy-capacity save/reopen/load round-trip test using `MempoolSnapshotDecodeLimits::from_policy`.

### WR-02: Recovery installation is not anchored to the chainstate used for staging

**Files:** `packages/open-bitcoin-node/src/network/runtime_authority/recovery.rs:16-38`; `packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs:93-119`  
**Issue:** Staging captures a chainstate snapshot and the authority incarnation, but `PreparedMempoolRecovery` carries no chainstate revision or active-tip identity. The install guard checks only the authority incarnation and mempool freshness. A block connection with an empty mempool produces an empty lifecycle delta, so the freshness fields remain at their initial values even though chainstate changed. A candidate prepared before that block can then install transactions that the new tip confirmed or conflicted.

**Fix:** Capture an active-tip identity or monotonic chainstate revision with the staging inputs and compare it under the install authority guard before any aggregate mutation. Reject and reprepare on mismatch. Add a test that stages a candidate, connects a block confirming it while the live mempool is empty, and verifies installation is rejected without mutation.

### WR-03: Unbroadcast recovery intersects by txid instead of exact member identity

**Files:** `packages/open-bitcoin-node/src/network/recovery/staging.rs:135-147`; `packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs:130-148`  
**Issue:** Topology deterministically keeps one `(txid, wtxid)` identity when witness variants share a txid, but unbroadcast restoration retains every persisted identity whose txid survived. If the persisted unbroadcast entry names the dropped witness variant, installation sees an identity outside the canonical survivor set and rejects the entire otherwise salvageable recovery candidate.

**Fix:** Build a `BTreeSet<MempoolMemberIdentity>` from `working.entries()` and intersect persisted unbroadcast membership with that exact set. Add a same-txid/different-wtxid duplicate fixture where only the losing variant is unbroadcast and verify that the primary transaction is recovered without a stale unbroadcast identity. Update the Phase 135 checker, which currently requires the incorrect txid-only filter.

### WR-04: Store-backed RPC context replays against an empty chainstate

**File:** `packages/open-bitcoin-rpc/src/context/network.rs:99-128`  
**Issue:** `from_runtime_config_with_store` creates a default empty `MemoryChainstateStore` and immediately recovers the mempool from the supplied Fjall store without loading that store's chainstate snapshot. Transactions spending durable UTXOs are therefore misclassified and dropped during restart, even though the same store contains the chainstate needed to validate them. The recovered generation is then installed as clean, masking the loss.

**Fix:** Seed the memory chainstate from the effective Fjall store before constructing/recovering the network, route the constructor through the durable runtime bootstrap, or remove/restrict the constructor if empty-chain semantics are intentional. Add a restart test containing persisted chainstate plus a mempool transaction that spends one of its UTXOs.

### WR-05: Fully spent confirmed transactions receive the wrong recovery status

**File:** `packages/open-bitcoin-node/src/network/recovery/staging.rs:216-225`  
**Issue:** `transaction_is_confirmed` defines confirmation as having at least one output currently present in the UTXO set. Once every output of a confirmed transaction has been spent, the predicate becomes false and replay reports `DroppedMissingParent` or `DroppedPolicyIncompatible` instead of `DroppedConfirmed`. Membership remains safe, but the required typed recovery records and counters are factually incorrect for a common chainstate.

**Fix:** Classify confirmation from active-chain transaction membership or another explicit chain index rather than current unspent outputs. If the required evidence is unavailable, avoid claiming the specific confirmed class. Add a fixture that confirms a transaction, spends all its outputs in a later block, and replays its snapshot record.

### WR-06: Operator recovery evidence drops the expiry counter

**File:** `packages/open-bitcoin-node/src/network/recovery.rs:96-105`  
**Issue:** `ManagedMempoolRecoverySummary` counts `DroppedExpired`, but its conversion to `RelayRecoveryCounters` omits that field. Operator-visible totals therefore disagree with the detailed recovery records whenever restart expiry removes an entry.

**Fix:** Add `dropped_expired_count` to `RelayRecoveryCounters`, populate it in this conversion, project/serialize it on all operator surfaces, and extend the expiry recovery test to assert the exposed counter.

### WR-07: Abort-dispatch failure permanently strands the checkpoint reservation

**Files:** `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs:262-296`; `packages/open-bitcoin-node/src/network/runtime_authority/effects.rs:29-38`; `packages/open-bitcoin-node/src/network/checkpoint.rs:217-224,274-278`  
**Issue:** On encode/storage failure, `abort_failed_write` consumes the affine capability into `SnapshotWriteAbort`. If dispatch fails, `CheckpointAbortDispatchError` retains only the source error, unlike completion dispatch which retains its achieved receipt. The coordinator then resets to `Idle`, while the authority ledger still has the pending snapshot effect. Every later preparation is rejected as already pending, so the documented retryable pre-achievement failure path is permanently wedged. The injected failure test confirms the in-flight generation remains set but never attempts a retry.

**Fix:** Return the owned abort capability on dispatch failure and add an `UnachievedAwaitingAbort` coordinator state. Retry that exact abort before permitting a new capture, mirroring retained completion receipts. Add a regression test that injects one abort-dispatch failure and proves a subsequent coordinator call clears the reservation and can checkpoint again.

## Info

### IN-01: Structural checker can pass semantic regressions it claims to guard

**Files:** `scripts/check-phase135-snapshot-recovery.ts:61-73,103-110,228-250,304-322`; `scripts/check-phase135-snapshot-recovery.test.ts:68-97`  
**Issue:** The checker extracts Rust bodies with raw brace counting that does not ignore comments or strings; its source-only schema guard rejects only four derived field names instead of allowlisting the two permitted record fields; its startup guard checks only that recovery call names occur somewhere, not their order or bounded-load/publication relationship; and verifier ordering counts tokens in any line, including comments. The mutation suite changes only exact known strings, so these bypasses are not exercised.

**Fix:** Prefer AST/token-aware inspection where practical. At minimum, strip comments/strings before brace and command checks, enforce an exact v2 field allowlist, compare call positions inside the startup function, match executable verifier commands rather than substrings, and add mutations for each bypass.

### IN-02: MPDUR-03 parity evidence is mapped to the checkpoint requirement

**File:** `docs/parity/catalog/mempool-policy.md:612-619`  
**Issue:** `MPDUR-03` requires rolling-fee reset plus retained age and local-unbroadcast semantics, but its row cites affine checkpointing. Checkpointing belongs to `MPDUR-04`, which already has its own row, leaving MPDUR-03 without correctly labeled evidence and duplicating MPDUR-04 evidence.

**Fix:** Point the MPDUR-03 row to fresh rolling-state initialization, restored acceptance-time/age behavior, and exact local-unbroadcast recovery tests. Keep coordinator, durability, freshness, and loss-range evidence under MPDUR-04, and cite the authority module that owns durable-generation evidence.

***

_Reviewed: 2026-08-03T03:32:25Z_  
_Reviewer: the agent (gsd-code-reviewer)_  
_Depth: standard_
