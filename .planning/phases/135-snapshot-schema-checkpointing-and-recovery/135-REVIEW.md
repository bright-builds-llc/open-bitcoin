---
phase: 135-snapshot-schema-checkpointing-and-recovery
reviewed: 2026-08-03T09:28:57Z
depth: standard
files_reviewed: 90
files_reviewed_list:
  - MODULE.bazel.lock
  - docs/architecture/operator-observability.md
  - docs/architecture/status-snapshot.md
  - docs/metrics/lines-of-code.md
  - docs/operator/runtime-guide.md
  - docs/parity/catalog/mempool-policy.md
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata/difficulty.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs
  - packages/open-bitcoin-chainstate/src/types.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests/projection.rs
  - packages/open-bitcoin-cli/src/operator/status/render/relay.rs
  - packages/open-bitcoin-cli/src/operator/status/tests/relay_and_filesystem_fixtures.rs
  - packages/open-bitcoin-cli/src/operator/status/tests/snapshot.rs
  - packages/open-bitcoin-cli/src/operator/support/render/relay.rs
  - packages/open-bitcoin-cli/src/operator/support/tests/forensics_recovery_relay.rs
  - packages/open-bitcoin-cli/src/operator/support/tests/sync_fixtures.rs
  - packages/open-bitcoin-mempool/src/pool.rs
  - packages/open-bitcoin-node/Cargo.toml
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/logging.rs
  - packages/open-bitcoin-node/src/logging/tests/redaction.rs
  - packages/open-bitcoin-node/src/metrics.rs
  - packages/open-bitcoin-node/src/metrics/tests/contracts.rs
  - packages/open-bitcoin-node/src/metrics/tests/relay_projection.rs
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
  - packages/open-bitcoin-node/src/status/relay_evidence.rs
  - packages/open-bitcoin-node/src/status/tests/availability_and_relay.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/confirmation_migration.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/load_limits.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/reopen.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/write_execution_failures.rs
  - packages/open-bitcoin-node/src/storage/mempool_snapshot.rs
  - packages/open-bitcoin-node/src/storage/mempool_snapshot/tests.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode/key_preflight.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode/transaction.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_limits.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/tests/block_requests.rs
  - packages/open-bitcoin-node/src/sync/tests/reorg_reconciliation.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/checkpoint.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/inbound_runtime.rs
  - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/context/tests.rs
  - packages/open-bitcoin-rpc/src/context/tests/construction.rs
  - scripts/check-phase108-durable-mempool-relay-state-recovery.ts
  - scripts/check-phase130-resource-time-fee-primitives.test.ts
  - scripts/check-phase130-resource-time-fee-primitives.ts
  - scripts/check-phase134-authoritative-lifecycle.test.ts
  - scripts/check-phase135-snapshot-recovery.test.ts
  - scripts/check-phase135-snapshot-recovery.ts
  - scripts/check-phase135-snapshot-recovery/source.ts
  - scripts/verify.sh
findings:
  critical: 1
  warning: 1
  info: 1
  total: 3
status: issues_found
---

# Phase 135: Code Review Report

**Reviewed:** 2026-08-03T09:28:57Z
**Depth:** standard
**Files Reviewed:** 90
**Status:** issues_found

## Summary

This capped confirmation audit reviewed the exact union of the prior review scope and every existing non-planning path changed from `fa9b8b22..HEAD`. Commit `42fa3ea2` correctly places the raw-key preflight before Serde for well-formed keys, runs legacy confirmation migration before both authoritative runtime constructors publish a handle, checks decoded block-header identity against the active-chain key, keeps generic snapshots confirmation-unknown unless authoritative evidence is supplied, and preserves checker/parity coverage across the module splits.

Three residual issues remain. The raw-key scanner accepts unterminated escaped strings and can still hand a large malformed token to Serde, confirmation migration authenticates the header but not the transaction body committed by that header, and the phase checker does not prove that the preflight function is actually called. Prior Phase 135 findings were rechecked across the full scope; no other regressions were found. No source fixes were applied during this read-only review.

## Iteration 3 Confirmation

- Escaped, well-formed unknown keys are bounded before Serde allocation, but malformed unterminated escaped keys remain exposed as CR-01.
- `DurableSyncRuntime::open_with_runtime_activation` and the alternate store-backed constructor migrate legacy confirmation evidence or fail before publishing network authority.
- Loaded block headers must match their active-chain keys, but the migration still trusts an unverified block body as described in WR-01.
- Generic chainstate snapshots keep `maybe_confirmed_txid_counts: None`; only authoritative evidence supplies `Some(...)`.
- The chainstate, Fjall snapshot tests, RPC context tests, and phase checker splits preserve behavior and remain within the managed file-length policy. Breadcrumb validation and the Bright Builds checker pass.

## Critical Issues

### CR-01: Unterminated escaped object keys still reach Serde allocation

**File:** `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode/key_preflight.rs:14-33`

**Issue:** The raw-key scanner skips two bytes for every backslash, but treats reaching the end of an unterminated quoted token as success. A trailing backslash can also advance `cursor` past the input, bypassing the `cursor == encoded.len()` check. An attacker-controlled snapshot containing a very large malformed key with escapes therefore passes this preflight and reaches `serde_json`, which can allocate scratch storage proportional to the token before reporting the syntax error. This leaves the startup memory-exhaustion boundary open for malformed snapshots even though well-formed overlong keys are now rejected.

**Fix:** Make the preflight scanner track JSON container state so it knows when an object is expecting a key. While scanning a key token, enforce the raw bound before its closing quote and reject an incomplete escape or missing closing quote before constructing the Serde deserializer. Keep value strings on their existing field-specific bounds. For example, the object-key branch should follow this shape:

```rust
if container.expects_object_key() {
    cursor = scan_bounded_json_string(
        encoded,
        cursor,
        MAX_MEMPOOL_FIELD_TOKEN_BYTES,
    )?;
    container.record_object_key()?;
}
```

Add regressions for an oversized unterminated key containing escapes and for a key ending in a trailing backslash; both must fail before Serde is invoked.

## Warnings

### WR-01: Confirmation migration trusts the block body after checking only header identity

**File:** `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs:204-241`

**Issue:** The migration recomputes the hash of the decoded header and compares it with `position.block_hash`, then counts transaction IDs from the decoded body. `load_block` only performs structural decoding, so a stored block with the expected header but a body that does not match the header's Merkle root passes this check and persists incorrect authoritative confirmation counts. Header identity alone does not authenticate the transactions used by the migration.

**Fix:** Before counting transactions, compute the Merkle root for `block.transactions`, reject mutated trees, and compare it with `block.header.merkle_root` (or invoke an appropriate existing full block-integrity check). Return chainstate corruption on mismatch. Add a regression that stores the expected header/key with a different syntactically valid transaction body and confirms migration fails without publishing or persisting evidence.

## Info

### IN-01: The phase checker does not prove the raw-key preflight is called

**File:** `scripts/check-phase135-snapshot-recovery.ts:183-195`

**Issue:** The checker concatenates the decoder, preflight, and transaction sources and only requires the identifier `validate_raw_object_keys` to appear somewhere. Its mutation test changes the bound constant to `usize::MAX`; it does not remove the call. Deleting `validate_raw_object_keys(bytes)?` from `decode_bounded_versioned` would therefore leave the identifier in the helper/import and allow this contract check to pass.

**Fix:** Inspect the exact `decode_bounded_versioned` body and require `validate_raw_object_keys(bytes)?` before `serde_json::Deserializer::from_slice(bytes)`. Add a mutation that removes the call itself and assert the bounded-decode diagnostic.

## Verification

- `bun test scripts/check-phase135-snapshot-recovery.test.ts` — 50 passed, 0 failed.
- `bun scripts/check-phase135-snapshot-recovery.ts` — passed.
- `bun scripts/check-parity-breadcrumbs.ts` — passed for 761 Rust files.
- `bun scripts/bright-builds-check.ts all` — 1,041 files scanned, 0 findings.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate` — 40 unit tests and 3 parity tests passed.
- Targeted `open-bitcoin-node` snapshot codec, confirmation migration, and sync reorganization suites — 44 tests passed.
- Targeted `open-bitcoin-rpc` context and daemon sync suites — 35 tests passed.
- `git diff --check fa9b8b22..HEAD` — passed.

***

_Reviewed: 2026-08-03T09:28:57Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
