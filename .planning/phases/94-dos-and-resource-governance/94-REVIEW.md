---
phase: 94-dos-and-resource-governance
reviewed: 2026-06-26T23:51:32Z
depth: standard
files_reviewed: 45
files_reviewed_list:
  - docs/architecture/operator-observability.md
  - docs/architecture/status-snapshot.md
  - docs/operator/runtime-guide.md
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
  - packages/open-bitcoin-cli/src/operator/status/render/inbound.rs
  - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
  - packages/open-bitcoin-cli/src/operator/status/tests.rs
  - packages/open-bitcoin-cli/src/operator/support/render/inbound.rs
  - packages/open-bitcoin-cli/src/operator/support/tests.rs
  - packages/open-bitcoin-network/src/compatibility.rs
  - packages/open-bitcoin-network/src/error.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/inventory_state.rs
  - packages/open-bitcoin-network/src/peer/tests.rs
  - packages/open-bitcoin-network/src/resource.rs
  - packages/open-bitcoin-network/src/resource/tests.rs
  - packages/open-bitcoin-node/src/logging.rs
  - packages/open-bitcoin-node/src/logging/tests.rs
  - packages/open-bitcoin-node/src/metrics.rs
  - packages/open-bitcoin-node/src/metrics/tests.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/inbound.rs
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/status/inbound.rs
  - packages/open-bitcoin-node/src/status/inbound/tests.rs
  - packages/open-bitcoin-node/src/status/tests.rs
  - packages/open-bitcoin-rpc/BUILD.bazel
  - packages/open-bitcoin-rpc/Cargo.toml
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/context/resource_governance.rs
  - packages/open-bitcoin-rpc/src/context/tests.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests.rs
  - packages/open-bitcoin-rpc/src/inbound_listener.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - scripts/check-phase94-dos-resource-governance.test.ts
  - scripts/check-phase94-dos-resource-governance.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
status: issues_found
---

# Phase 94: Code Review Report

**Reviewed:** 2026-06-26T23:51:32Z
**Depth:** standard
**Files Reviewed:** 45
**Status:** issues_found

## Summary

Reviewed the Phase 94 resource-governance implementation, operator/RPC/status projections, parity docs, and verifier wiring. The no-public-network/no-relay-claim boundary is preserved in the reviewed docs and checker. The review was materially informed by `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/index.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/core/operability.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md`; no `standards-overrides.md` or project skills were present.

Verification run during review:

- `bun test scripts/check-phase94-dos-resource-governance.test.ts` passed.
- `bun run scripts/check-phase94-dos-resource-governance.ts` passed.

Generated/lock artifacts were read for context but excluded from `files_reviewed_list`: `MODULE.bazel.lock`, `packages/Cargo.lock`, and `docs/metrics/lines-of-code.md`.

## Warnings

### WR-01: Socket read timeout resets for each partial read

**File:** `packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs:217`
**Issue:** `read_wire_message_with_timeout_duration` applies `timeout_duration` separately to the header read and the payload read, and `read_exact_with_timeout` starts a fresh `tokio::time::timeout(timeout_duration, stream.readable())` on every loop iteration at line 283. A peer can therefore send one byte just under the timeout repeatedly and keep the inbound task alive far longer than the configured slow-handshake or idle-peer deadline. That weakens the Phase 94 DoS boundary for slowloris-style partial reads.
**Fix:** Use one absolute deadline for the full header or payload read, and pass the remaining time into each wait. Add a paused-time test that dribbles header bytes just under the timeout and asserts a `timeout_disconnect` instead of allowing the read to continue indefinitely.

```rust
let deadline = tokio::time::Instant::now() + timeout_duration;
if read_exact_until(stream, &mut header, deadline).await? == SocketIoOutcome::Timeout {
    return Ok(ReadWireMessageOutcome::Rejected(timeout_event_after_elapsed(
        resource_policy,
        connected_at_unix_seconds,
        last_activity_unix_seconds,
        handshake_state,
    )));
}
```

### WR-02: Request-cap governance events are dropped before status/log projection

**File:** `packages/open-bitcoin-network/src/peer/inventory_state.rs:297`; `packages/open-bitcoin-node/src/network.rs:606`
**Issue:** `ResourceGovernancePolicy::decide_request` returns an `InboundResourceEvent` with `request_cap_reached`, and request-cap checks call it from `getheaders`, `getdata`, and `inv` handling. `resource_limit_disconnect_actions` discards that event and returns only `PeerAction::Disconnect(DisconnectReason::ResourceLimit)`. `ManagedPeerNetwork::process_actions` then disconnects the peer without calling `record_resource_governance_event`, so request-cap disconnects are enforced but do not update `request_cap_events`, latest resource-governance status, structured logs, support output, or fixed metric inputs.
**Fix:** Preserve the event in the peer action path, for example by adding a `PeerAction::ResourceGovernanceDisconnect(InboundResourceEvent)` variant or by carrying the event alongside `DisconnectReason::ResourceLimit`. In `ManagedPeerNetwork::process_actions`, record the event before disconnecting. Add tests for over-cap `inv`, `getdata`, and `getheaders` inputs proving `request_cap_events == 1` and the latest decision has `next_action == "request_cap_reached"`.

```rust
PeerAction::ResourceGovernanceDisconnect(event) => {
    self.record_resource_governance_event(event);
    self.disconnect_peer(peer_id)?;
    return Err(ManagedNetworkError::Network(NetworkError::ResourceLimit(peer_id)));
}
```

---

_Reviewed: 2026-06-26T23:51:32Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
