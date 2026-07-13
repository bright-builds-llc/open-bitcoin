---
phase: 119-compact-receive-mempool-candidate-injection
reviewed: 2026-07-13T19:05:00Z
depth: standard
files_reviewed: 8
files_reviewed_list:
  - packages/open-bitcoin-network/src/peer/compact_download_state.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
  - packages/open-bitcoin-node/src/network/admission_bridge.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/tests/compact_receive_cases.rs
findings:
  critical: 0
  warning: 0
  info: 1
  total: 1
status: clean
---

# Phase 119: Code Review Report

**Reviewed:** 2026-07-13T19:05:00Z
**Depth:** standard
**Files Reviewed:** 8
**Status:** clean (advisory)

## Summary

Phase 119 wires live CompactBlock receive to inject mempool + bounded extras, and hooks mempool exits into `PeerManager::on_mempool_transaction_removed`. The seam is sound: shell-owned snapshots avoid PeerManager/mempool borrow conflicts; admission extras/lifecycle hooks resolve wtxid before demotion; production `receive_message` / `receive_sync_message` bypass empty-facts dispatch. No production `unwrap`/`expect`. No critical or warning findings.

## Info

### IN-01: Empty-facts CompactBlock path remains callable

**File:** `packages/open-bitcoin-network/src/peer/message_dispatch.rs:48-57`
**Issue:** `PeerManager::handle_message` still dispatches CompactBlock with `CompactBlockReceiveFacts::default()`. Documented as non-production (D-03); shell intercepts both receive paths. Risk only if a future caller bypasses `ManagedPeerNetwork`.
**Fix:** Optional: `#[cfg(test)]` the empty-facts branch later, or assert in debug that production never hits it. Not required for Phase 119 closeout.

---

_Reviewed: 2026-07-13T19:05:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
_Advisory: does not block_
