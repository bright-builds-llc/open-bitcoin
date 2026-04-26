---
phase: 15-real-network-sync-loop
plan: "03"
subsystem: sync
tags: [sync, tcp, status, metrics, live-smoke, sync-01, sync-02, sync-03, sync-04, sync-05]
provides:
  - Durable sync runtime
  - Pluggable sync transport and real TCP adapter
  - Hermetic simulated-network tests
  - Ignored opt-in live-network smoke test
affects: [SYNC-01, SYNC-02, SYNC-03, SYNC-04, SYNC-05, sync, node]
requirements-completed: [SYNC-01, SYNC-02, SYNC-03, SYNC-04, SYNC-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 15-2026-04-26T21-05-56
generated_at: 2026-04-26T21:08:01.619Z
completed: 2026-04-26
---

# Phase 15 Plan 03: Durable Sync Runtime

## Accomplishments

- Added `DurableSyncRuntime` with configurable peer sources, sync network constants, bounded per-peer loops, retry handling, and status summaries.
- Added a `SyncTransport` abstraction plus `TcpPeerTransport` for real sockets and deterministic scripted transports for tests.
- Loaded persisted chainstate/header progress from `FjallNodeStore` on startup and persisted headers, chainstate, blocks, runtime metadata, and metric samples after accepted sync work.
- Added hermetic tests for headers sync, block download/connect/persist, restart resume, no-peer errors, connect failures, and ignored live-network smoke behavior.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` passed.

## Notes

Default tests do not contact public peers. The live smoke test is ignored and gated by `OPEN_BITCOIN_LIVE_SYNC_SMOKE=1`.
