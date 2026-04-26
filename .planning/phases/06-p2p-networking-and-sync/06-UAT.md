---
status: complete
phase: 06-p2p-networking-and-sync
source:
  - .planning/phases/06-p2p-networking-and-sync/06-01-SUMMARY.md
  - .planning/phases/06-p2p-networking-and-sync/06-02-SUMMARY.md
  - .planning/phases/06-p2p-networking-and-sync/06-03-SUMMARY.md
  - .planning/phases/06-p2p-networking-and-sync/06-04-SUMMARY.md
started: 2026-04-26T10:40:06Z
updated: 2026-04-26T10:44:41Z
---

## Current Test

[testing complete]

## Tests

### 1. Peer Wire Handshake Surface
expected: From the repo root, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network outbound_handshake_negotiates_verack_sendheaders_and_wtxidrelay` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network wire_message_round_trips_version_and_inventory_payloads` pass, showing the pure-core network crate can encode/decode the Phase 6 wire messages and drive version, verack, sendheaders, wtxidrelay, and ping/pong lifecycle state explicitly.
result: pass

### 2. Header-First Sync Requests
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network block_inventory_triggers_getheaders_then_getdata_for_missing_blocks`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network header_store_builds_exponential_locators`, and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network headers_after_locator_respects_stop_hash` pass, proving block announcements request headers first, construct deterministic locators, respect stop hashes, and issue full-block requests only after headers connect.
result: pass

### 3. Managed Node Network Adapter
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node network` passes, proving `ManagedPeerNetwork` can host peer state, apply received blocks through `ManagedChainstate`, submit received transactions through `ManagedMempool`, and choose txid or wtxid inventory requests according to peer negotiation.
result: pass

### 4. P2P Parity Evidence and Ledger
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --test parity` passes, `docs/parity/catalog/p2p.md` and `docs/parity/index.json` mark the p2p surface as done with known deferred capabilities, and `bash scripts/verify.sh` succeeds with the network crate included in the repo-native verification contract.
result: pass

## Summary

total: 4
passed: 4
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
