---
phase: 15-real-network-sync-loop
plan: "01"
subsystem: network
tags: [sync, peer-manager, flow-control, sync-01, sync-03, sync-04]
provides:
  - Bounded in-flight block requests per peer
  - Header-store seeding and export helpers for runtime resume
affects: [SYNC-01, SYNC-03, SYNC-04, network, node]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 15-2026-04-26T21-05-56
generated_at: 2026-04-26T21:08:01.619Z
completed: 2026-04-26
---

# Phase 15 Plan 01: Bounded Peer Sync Hooks

## Accomplishments

- Added `DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER` and `PeerManager::with_max_blocks_in_flight`.
- Capped `headers`-driven `getdata` requests to available per-peer slots.
- Added managed-network helpers to seed persisted headers and export current header entries.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node network::` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` passed.

## Notes

The cap prevents unbounded request fanout from a large `headers` response while preserving existing handshake and inventory behavior.
