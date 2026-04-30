---
phase: 13-operator-runtime-foundations
plan: "03"
subsystem: status
tags: [status, provenance, observability, obs-01]
provides:
  - Shared status snapshot model
  - Build provenance and explicit unavailable-field semantics
affects: [OBS-01, node]
requirements-completed: [OBS-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 13-2026-04-26T18-50-22
generated_at: 2026-04-26T18:58:37.416Z
completed: 2026-04-26
---

# Phase 13 Plan 03: Shared Status Snapshot

## Accomplishments

- Added `docs/architecture/status-snapshot.md` documenting field ownership and stopped-node status semantics.
- Added `packages/open-bitcoin-node/src/status.rs` with `OpenBitcoinStatusSnapshot`, `FieldAvailability`, `BuildProvenance`, daemon state, config, sync, peer, mempool, wallet, service, log, metrics, and health-signal contracts.
- Exported status contracts from `open-bitcoin-node` and added parity breadcrumb coverage.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node status::` passed.

## Notes

No status renderer, collector, service manager, RPC call, filesystem read, or dashboard panel was added.
