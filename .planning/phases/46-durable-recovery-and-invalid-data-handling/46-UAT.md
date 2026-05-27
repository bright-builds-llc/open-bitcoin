---
status: complete
phase: 46-durable-recovery-and-invalid-data-handling
source:
  - 46-SUMMARY.md
started: 2026-05-27T00:29:15Z
updated: 2026-05-27T00:53:25Z
---

## Current Test
[testing complete]

## Tests

### 1. Durable Progress Fields
expected: Operator-visible sync status separates validated header height, durable downloaded block height, and connected chainstate height. The existing `block_height` field remains a connected-height compatibility alias, so status does not confuse downloaded-but-unconnected blocks with validated chainstate.
result: pass

### 2. Restart After Partial Sync
expected: After an interrupted sync with headers and some durable block bodies already saved, reopening the runtime reconnects valid saved blocks before requesting only the missing block. The returned durable status preserves header, downloaded, and connected progress without duplicating block connects.
result: pass

### 3. Invalid Peer Block Handling
expected: When a peer provides an invalid block body during sync, the failure is attributed to that peer as invalid data, the peer receives no useful block contribution credit, the invalid block is not saved, and active chainstate does not advance.
result: pass

### 4. Recovery Guidance Priority
expected: Durable sync status preserves the latest peer failure error and peer-derived recovery guidance when no storage recovery action takes priority. Storage corruption, incompatible store, resource exhaustion, transient network failure, invalid data, and intentional cancellation guidance stay distinguishable.
result: pass

### 5. Operator Documentation
expected: Operator-facing docs explain validated header height, durable downloaded block height, connected block height, last error, and recovery action. The docs include repo-local Cargo and Bazel status commands rather than only referring to an installed `open-bitcoin` alias.
result: pass

## Summary

total: 5
passed: 5
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
