---
status: complete
phase: 15-real-network-sync-loop
source:
  - 15-01-SUMMARY.md
  - 15-02-SUMMARY.md
  - 15-03-SUMMARY.md
  - 15-04-SUMMARY.md
started: 2026-05-03T03:41:06Z
updated: 2026-05-03T03:49:27Z
---

## Current Test

[testing complete]

## Tests

### 1. Phase 15 verification report
expected: Open `.planning/milestones/v1.1-phases/15-real-network-sync-loop/15-VERIFICATION.md`. It should show `status: passed` with `score: 5/5 success criteria verified`, and all five criteria should be marked `VERIFIED`: outbound peer connectivity through configured/manual or DNS-seeded sources, durable header resume, bounded block download/connect flow, typed failure and health-signal reporting, and explicitly opt-in live-network smoke coverage.
result: pass

### 2. Durable sync runtime summary evidence
expected: Open `.planning/milestones/v1.1-phases/15-real-network-sync-loop/15-03-SUMMARY.md`. It should state that `DurableSyncRuntime` was added with configurable peer sources, bounded per-peer loops, retry handling, and status summaries; that `SyncTransport` plus `TcpPeerTransport` were added; that persisted headers and chainstate are loaded on startup and saved after accepted sync work; and that hermetic tests plus an ignored live-network smoke path were added.
result: pass

### 3. Mainnet sync activation contract
expected: Open `docs/operator/runtime-guide.md` in the mainnet sync activation section. It should document JSONC and CLI activation for `-openbitcoinsync=mainnet-ibd`, explain `sync.manual_peers`, `sync.dns_seeds`, and `sync.target_outbound_peers`, reject activation on non-mainnet chains, and describe the bounded background sync loop as the active daemon behavior when enabled.
result: pass

### 4. Live-network smoke remains opt-in
expected: Open `packages/open-bitcoin-node/src/sync/tests.rs` around `live_network_smoke_is_explicitly_opt_in`. The test should be marked `#[ignore = "requires public Bitcoin network; set OPEN_BITCOIN_LIVE_SYNC_SMOKE=1 to run"]`, and the body should return early unless `OPEN_BITCOIN_LIVE_SYNC_SMOKE=1`, proving default verification does not contact public peers.
result: pass

### 5. Closeout verification coverage
expected: Open `.planning/milestones/v1.1-phases/15-real-network-sync-loop/15-04-SUMMARY.md` and `.planning/milestones/v1.1-phases/15-real-network-sync-loop/15-VERIFICATION.md`. They should show both targeted sync checks and the full required verification set, including `cargo fmt --manifest-path packages/Cargo.toml --all --check`, workspace `cargo clippy`, workspace `cargo build`, workspace `cargo test`, and `bash scripts/verify.sh` as passed.
result: pass

## Summary

total: 5
passed: 5
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps
