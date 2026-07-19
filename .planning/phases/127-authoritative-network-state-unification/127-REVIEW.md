---
phase: 127-authoritative-network-state-unification
reviewed: 2026-07-19T22:05:07Z
depth: standard
files_reviewed: 24
files_reviewed_list:
  - docs/metrics/lines-of-code.md
  - docs/parity/catalog/p2p.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
  - packages/open-bitcoin-cli/src/operator/support/tests.rs
  - packages/open-bitcoin-node/src/network/block_serving.rs
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/types.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/inbound_status.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/dispatch/node.rs
  - packages/open-bitcoin-rpc/src/inbound_listener.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity.rs
  - scripts/check-phase124-milestone-gap-closure.test.ts
  - scripts/check-phase124-post-audit-gap-planning.ts
  - scripts/check-phase127-authoritative-network-state-unification.test.ts
  - scripts/check-phase127-authoritative-network-state-unification.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 4
  info: 0
  total: 4
status: issues_found
---

# Phase 127: Code Review Report

**Reviewed:** 2026-07-19T22:05:07Z
**Depth:** standard
**Files Reviewed:** 24
**Status:** issues_found

## Summary

The shared `ManagedNetworkHandle` keeps its mutex guard private, returns owned snapshots, and the inbound durable-serving path performs Fjall lookup, serialization, and socket writes after releasing the authoritative-network guard. Authority poison failures also fail closed through typed errors. Four correctness and verification weaknesses remain: durable sync metadata is cached in RPC, mixed `getdata` responses are reordered relative to Knots, the structural checker is anchor-spoofable, and the production-shaped redaction assertion does not exercise the data it claims to protect.

The review applied the repo-local authority/parity and verifier guidance from `AGENTS.md`, plus `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/core/operability.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md`. No active standards override applies.

## Warnings

### WR-01: Restarted RPC contexts retain stale durable sync metadata

**File:** `packages/open-bitcoin-rpc/src/context/network.rs:209`

**Issue:** `from_runtime_config_with_network_handle` loads `DurableSyncState` once and stores it in `ManagedRpcContext`; `maybe_durable_sync_state` at lines 276-278 only returns that cached value. The daemon then starts a sync worker that continues updating runtime metadata in Fjall, but no path refreshes the context field. `getblockchaininfo` in `dispatch/node.rs:42-90` therefore takes `blocks`, `headers`, lifecycle, progress, and warnings from stale startup metadata while taking `bestblockhash` and median time from the live authoritative chain tip. After a restart with existing metadata, subsequent sync progress can produce a self-contradictory RPC response indefinitely. The Phase 127 black-box test constructs its restarted context only after sync has finished and asserts the best hash and schema keys, not live height or warning updates, so it does not catch this case.

**Fix:** Keep a clonable durable metadata source in the context and load the latest runtime metadata for each `getblockchaininfo` projection outside the network critical section, or explicitly publish every sync-state update into a shared typed snapshot. Build the response from one current live chainstate snapshot plus the latest durable metadata, and fail unavailable rather than falling back to an older cached value. Add a regression that opens a context with pre-existing metadata, advances sync through the shared runtime after context construction, and asserts updated `blocks`, `headers`, progress, lifecycle, and warnings.

### WR-02: Mixed durable `getdata` batches emit `notfound` before block responses

**File:** `packages/open-bitcoin-rpc/src/context.rs:124-129`

**Issue:** The call chain loses response ordering. `gate_inventory_for_durable_serving` in `network/inventory.rs:97-150` separates block intents from ordinary messages and missing inventory. `process_actions` in `network/action_translation.rs:205-215` appends the aggregate `NotFound` to `outbound`, while `InboundWireResponsePlan::resolve` emits every `outbound` response before resolving any block intent. For a request batch containing a missing transaction followed by an eligible durable block, Open Bitcoin therefore writes `notfound` and then the block. The pinned Knots path in `net_processing.cpp:2438-2491` processes the block before sending the accumulated `notfound`. This is externally observable parity drift and can affect peers that use response order to retire queued requests. Current tests cover three successful block variants together and a single missing block, but no mixed missing-transaction/durable-block batch.

**Fix:** Preserve a single ordered response-plan sequence, for example:

```rust
enum PlannedInboundResponse {
    Immediate(WireNetworkMessage),
    DurableBlock(ManagedBlockServeIntent),
}
```

Translate each action into this sequence and resolve it in order, while retaining the Knots rule that accumulated transaction `notfound` is emitted after the processed block item. Add a loopback regression for a mixed missing transaction plus durable block and assert the exact wire order.

### WR-03: The Phase 127 checker can be satisfied by dead textual anchors

**File:** `scripts/check-phase127-authoritative-network-state-unification.ts:69-165`

**Issue:** The production-authority, durable-serving, and operator-projection checks use exact `includes`, occurrence counts, and broad function slicing rather than verifying the executed data flow. A split regression can retain all required strings while replacing `authoritative_runtime.network` with a fresh `ManagedNetworkHandle::transient_runtime`; a cache-only resolver can retain `source.load_block(intent.block_hash())` in dead code or a comment; and a direct projection can retain an unused `operator_snapshot` call. All would pass the checker. The mutation suite at `check-phase127-authoritative-network-state-unification.test.ts:57-146` only deletes or replaces the exact anchors the checker searches for, so it demonstrates sensitivity to anchor removal rather than resistance to semantic spoofing.

**Fix:** Parse the Rust source into syntax-aware function bodies or combine narrower negative invariants with behavioral integration checks. At minimum, reject reassignment/reconstruction of the authority after `open_authoritative_network_runtime`, reject cache lookup anywhere in the production block-intent resolver, and require the returned projection values to derive from the snapshot binding. Add adversarial mutations that keep every current anchor present while introducing each prohibited executed path.

### WR-04: The black-box redaction assertion is largely vacuous

**File:** `packages/open-bitcoin-rpc/tests/black_box_parity.rs:540-605`

**Issue:** The test serializes only `authoritative_operator_snapshot().block_relay()` and compares it with `status_response["result"]["block_relay"]`, then checks that this block-relay-only JSON lacks the listener endpoint, RPC credentials, permission string, transaction marker, and dynamic label. The endpoint and permission live on the inbound/config side, the credentials live only in HTTP auth, and the transaction and dynamic-label constants are never inserted into the serialized block-relay value. The assertions therefore pass even if the authoritative inbound or support projection leaks those values. This does not prove the Plan 127 production-path claim that support-compatible evidence from the shared authority preserves the disclosure boundary.

**Fix:** Drive the actual authoritative RPC snapshot through the same support-bundle sanitization adapter used in production, seed every forbidden value into the corresponding inbound, relay, peer-policy, resource, and transaction-bearing fields, and assert absence in both JSON and Markdown support outputs. If the crate dependency direction prevents importing CLI support code, move the sanitization contract to a lower shared crate or add an integration test in the CLI crate that consumes the live RPC response. Remove forbidden constants that are not injected into the value under test.

## Verification

- `bun test scripts/check-phase127-authoritative-network-state-unification.test.ts` — passed, 5 tests.
- `bun run scripts/check-phase127-authoritative-network-state-unification.ts` — passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --test black_box_parity phase127` through the repo timing wrapper — passed, 1 test.
- `git diff --check` — passed.
- Source files were not modified.

______________________________________________________________________

_Reviewed: 2026-07-19T22:05:07Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
