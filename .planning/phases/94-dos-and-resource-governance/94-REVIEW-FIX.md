---
phase: 94-dos-and-resource-governance
fixed_at: 2026-06-27T00:59:13Z
review_path: .planning/phases/94-dos-and-resource-governance/94-REVIEW.md
iteration: 1
findings_in_scope: 2
fixed: 2
skipped: 0
status: all_fixed
---

# Phase 94: Code Review Fix Report

**Fixed at:** 2026-06-27T00:59:13Z
**Source review:** .planning/phases/94-dos-and-resource-governance/94-REVIEW.md
**Iteration:** 1

**Summary:**
- Findings in scope: 2
- Fixed: 2
- Skipped: 0

## Fixed Issues

### WR-01: Socket read timeout resets after every partial read

**Files modified:** `MODULE.bazel.lock`, `docs/metrics/lines-of-code.md`, `packages/open-bitcoin-rpc/Cargo.toml`, `packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs`
**Commit:** f2fc2c4a
**Applied fix:** Replaced per-read timeout resets with one absolute socket-read deadline shared across header and payload reads, and added a paused-time regression test for partial header bytes that advance past the deadline.

### WR-02: Request-cap resource-governance events are dropped before managed status/log projection

**Files modified:** `docs/metrics/lines-of-code.md`, `packages/open-bitcoin-network/src/compatibility.rs`, `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/peer/inventory_state.rs`, `packages/open-bitcoin-network/src/peer/tests.rs`, `packages/open-bitcoin-network/tests/parity.rs`, `packages/open-bitcoin-node/src/network.rs`, `packages/open-bitcoin-node/src/network/inventory.rs`, `packages/open-bitcoin-node/src/network/tests.rs`
**Commit:** 53c4f1c3
**Applied fix:** Preserved `InboundResourceEvent` in peer actions, recorded it into managed resource-governance status/log projection before disconnecting, and added regression coverage for request-cap disconnects from `inv`, `getdata`, and `getheaders`.

---

_Fixed: 2026-06-27T00:59:13Z_
_Fixer: the agent (gsd-code-fixer)_
_Iteration: 1_
