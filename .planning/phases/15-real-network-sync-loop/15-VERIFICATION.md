---
phase: 15-real-network-sync-loop
verified: 2026-04-26T21:26:58Z
status: passed
score: 5/5 success criteria verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 15-2026-04-26T21-05-56
generated_at: 2026-04-26T21:26:58Z
lifecycle_validated: true
overrides_applied: 0
provenance_warnings: []
---

# Phase 15: Real Network Sync Loop Verification Report

**Phase Goal:** Turn the existing peer/message/header primitives into a long-running sync runtime that can talk to real network peers and resume progress.
**Requirements:** SYNC-01, SYNC-02, SYNC-03, SYNC-04, SYNC-05
**Verified:** 2026-04-26T21:26:58Z
**Status:** passed

## Success Criteria

| # | Criterion | Status | Evidence |
|---|-----------|--------|----------|
| 1 | The node can establish long-running outbound peer connections using configured peers and DNS/manual seed sources for supported networks. | VERIFIED | `SyncRuntimeConfig`, `SyncPeerAddress`, `SyncNetwork`, `SyncNetwork::consensus_params`, `TcpPeerTransport`, `DurableSyncRuntime::sync_once` |
| 2 | Headers sync persists progress, resumes after restart, and reports progress through the shared status model. | VERIFIED | `scripted_headers_sync_persists_progress_and_status`, `runtime_seeds_headers_from_durable_store_on_restart`, `SyncRunSummary::sync_status` |
| 3 | Block download, validation, persistence, and connect flow handles bounded in-flight work and observable retry behavior. | VERIFIED | `headers_response_caps_block_requests_to_in_flight_limit`, `scripted_block_download_connects_and_persists_block`, `FjallNodeStore::save_block`, `FjallNodeStore::load_block` |
| 4 | Peer disconnects, invalid data, timeouts, stalls, and competing branches produce typed errors, metrics, and logs. | VERIFIED | `SyncRuntimeError`, `PeerSyncOutcome`, health signals in `SyncRunSummary`, metrics persisted by `persist_metrics`; branch/ancestor errors remain typed through `NetworkError` and `ManagedNetworkError` |
| 5 | Default verification remains hermetic; optional live-network smoke tests are explicitly opt-in. | VERIFIED | `live_network_smoke_is_explicitly_opt_in` is ignored and additionally gated by `OPEN_BITCOIN_LIVE_SYNC_SMOKE=1` |

## Requirements Coverage

| Requirement | Status | Evidence |
|---|---|---|
| SYNC-01 | SATISFIED | The sync runtime establishes long-running outbound peers through configured peers and supported network seed sources. |
| SYNC-02 | SATISFIED | Header-sync restart tests prove sync progress is persisted and resumed from durable storage. |
| SYNC-03 | SATISFIED | Block download, validation, persistence, and connect flow are covered by bounded in-flight sync and durable block-storage tests. |
| SYNC-04 | SATISFIED | Typed sync outcomes, network errors, metrics, and health-signal reporting cover disconnects, invalid data, timeouts, stalls, and competing branches. |
| SYNC-05 | SATISFIED | Live-network smoke coverage remains opt-in while the default verification path stays deterministic and hermetic. |

## Targeted Verification

| Surface | Command | Result |
|---------|---------|--------|
| Network peer flow control | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer::` | passed |
| Node network adapters | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node network::` | passed |
| Node storage blocks | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::` | passed |
| Node sync runtime | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::` | passed; includes network consensus-parameter regression coverage |
| Node all features | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features` | passed |
| Node clippy | `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` | passed |
| Network clippy | `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` | passed |

## Required Verification

| Command | Result |
|---------|--------|
| `cargo fmt --manifest-path packages/Cargo.toml --all --check` | passed |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | passed |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | passed |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | passed |
| `bash scripts/verify.sh` | passed |

## Residual Gaps

Phase 15 intentionally stops at the runtime loop and deterministic sync evidence. Bounded metrics history, log rotation, rich CLI status, service lifecycle, TUI rendering, migration inspection, and real-sync benchmark reporting remain in later v1.1 phases.
