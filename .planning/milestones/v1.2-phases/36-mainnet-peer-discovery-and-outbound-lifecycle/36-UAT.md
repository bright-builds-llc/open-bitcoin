---
status: complete
phase: 36-mainnet-peer-discovery-and-outbound-lifecycle
source: [36-01-SUMMARY.md, 36-02-SUMMARY.md, 36-03-SUMMARY.md, 36-04-SUMMARY.md]
started: 2026-05-11T13:59:13.372Z
updated: 2026-05-13T23:54:08.858Z
---

## Current Test

[testing complete]

## Tests

### 1. Peer Config Is Accepted Or Rejected Before Runtime Startup
expected: With the sample config at `.planning/phases/36-mainnet-peer-discovery-and-outbound-lifecycle/36-sample-open-bitcoin.jsonc`, `open-bitcoin --config ... config paths --format json` accepts the Open Bitcoin JSONC path, and `open-bitcoind -datadir=/tmp/open-bitcoin-phase36-uat -openbitcoinconf="$PWD/..." -server=1` reaches the mainnet sync preflight signal before manual Ctrl-C. If a copied config changes a peer to `localhost:not-a-port`, the daemon exits before startup with `invalid rpc port: not-a-port`. `bitcoin.conf` compatibility stays strict: Phase 36 peer settings belong in `open-bitcoin.jsonc`, not in the Knots-compatible config surface.
result: pass

### 2. Resolver-Driven Peer Discovery Uses Configured Sources
expected: Starting a sync-oriented daemon/operator workflow with manual peers and DNS seed overrides uses the resolver-backed peer discovery boundary: manual peers and DNS seeds remain distinct sources, resolved endpoints are normalized before connection attempts, and resolver failures are reported as typed peer outcomes instead of hidden transport errors.
result: pass

### 3. Bounded Outbound Lifecycle Stops At The Target
expected: A sync round respects the configured outbound peer target. When the target is satisfied, no further candidate peers are attempted in that round, and negotiated peer capabilities are captured for connected peers.
result: pass

### 4. Unhealthy Peers Rotate Out When Alternatives Exist
expected: When a peer stalls, fails resolution, times out, sends invalid data, or hits a transport error, the runtime records a typed failure reason, disconnects or deprioritizes the unhealthy peer, and tries an alternative resolved endpoint when one is available.
result: pass

### 5. Peer Telemetry Is Truthful Without Over-Claiming Later Sync Work
expected: Runtime summaries/logs expose resolved endpoint labels, source/network identity, contribution counters, last activity, negotiated capability summaries, and failure reasons while preserving existing peer-count status compatibility. The surface does not claim header-first sync, block download/connect, inbound serving, addrman parity, or final dashboard/RPC polish.
result: pass

### 6. Operator And Architecture Docs Match The Delivered Phase Boundary
expected: Contributor/operator docs describe Phase 36 peer configuration, resolver behavior, and bounded outbound lifecycle settings. They also clearly defer header-first sync, block download/connect, richer operator presentation, and persistent addrman-style behavior to later phases.
result: pass

## Summary

total: 6
passed: 6
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
