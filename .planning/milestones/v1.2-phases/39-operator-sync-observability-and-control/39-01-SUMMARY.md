---
phase: 39
phase_name: "Operator Sync Observability and Control"
plan_id: "39-01"
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: "39-2026-05-02T11-46-08"
generated_at: "2026-05-02T12:22:11Z"
status: completed
---

# Summary 39-01: Durable Sync Truth Across Daemon, CLI, Dashboard, And RPC

## Completed

- Extended the shared status contracts and durable runtime metadata so sync lifecycle, phase, lag, pressure, recovery guidance, last error, and recent peer telemetry can be persisted once and consumed across operator surfaces.
- Replaced the old `open-bitcoind` preflight-only path with a daemon-owned bounded sync worker that keeps durable sync state current and honors a durable pause flag.
- Added `open-bitcoin sync status`, `open-bitcoin sync pause`, and `open-bitcoin sync resume` so operators can inspect and control daemon sync without touching internal store files manually.
- Made `open-bitcoin status`, dashboard projections, and RPC `getblockchaininfo` consume durable sync truth when it is available instead of flattening IBD into `headers == blocks`.
- Refreshed contributor/operator docs and prepared the planning metadata closeout needed after earlier stale Phase 37 and Phase 38 bookkeeping.

## Tests Added

- RPC blockchain info uses durable sync truth when available.
- Operator CLI routes the new `sync pause` subcommand correctly.
- The operator binary pause/resume flow toggles durable sync control state as expected.

## Residual Risks

- The daemon worker currently runs as an opt-in local review workflow; this phase still does not make a production-node claim for unattended public-mainnet operation.
- Dashboard action-bar sync control remains future UX depth; the supported control surface in this phase is the explicit `open-bitcoin sync` CLI.
