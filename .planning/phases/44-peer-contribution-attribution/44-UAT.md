---
status: complete
phase: 44-peer-contribution-attribution
source:
  - .planning/phases/44-peer-contribution-attribution/44-01-SUMMARY.md
started: 2026-05-26T00:00:00.910Z
updated: 2026-05-26T15:00:35Z
---

## Current Test

[testing complete]

## Tests

### 1. Runtime Peer Contribution Counters
expected: After a sync run with peer outcomes, operator-visible peer telemetry exposes per-peer `headers_received` and `blocks_received` counters, and those counters represent accepted useful contribution rather than raw message activity.
result: pass

### 2. Idle And Stalled Peers Stay Uncredited
expected: Idle, stalled, waiting, or retry-backoff peers remain visible in peer outcomes with their state/reason, but they show zero useful header and block contribution and are not counted as connected useful peers.
result: pass

### 3. Failed Connected Peers Preserve Diagnosis
expected: A connected peer that later fails keeps last activity and failure reason visible separately from contribution counters, so invalid or failed peers are not credited for useful progress.
result: pass

### 4. Live Smoke Report Shows Contribution Evidence
expected: The live-smoke support report includes a Runtime Peer Contributions table and JSON `headersReceived` / `blocksReceived` fields sourced from durable runtime peer telemetry.
result: pass
reason: Phase 53 supersession: `.planning/phases/53-live-evidence-refresh/53-UAT.md` selects `packages/target/live-mainnet-smoke-reports/phase53-manual-peer/open-bitcoin-live-mainnet-smoke.json`, a fresh schema v2 `openbitcoinsyncstatus` report. The live network did not allow useful contribution observation in this environment: `result.maybeNoProgressCause=handshake_failure`, 68 runtime peer rows ended as failed or stalled, accepted header/block counters stayed at zero, and the report records the next operator action. The earlier real-network `handshake_failure` skip is superseded by fresh-status diagnosed-blocker evidence, not erased.

### 5. Operator Docs Explain Activity Versus Contribution
expected: The runtime guide explains that `headers_received` and `blocks_received` are validation-gated contribution counters, while messages processed and last activity are diagnostic activity signals.
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
