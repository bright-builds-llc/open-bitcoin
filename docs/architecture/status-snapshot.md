# Status Snapshot Contract

## OpenBitcoinStatusSnapshot

`OpenBitcoinStatusSnapshot` is the sole shared status model for later CLI status output, JSON automation, service diagnostics, dashboard panels, and support reports. Live RPC is not the only status source; stopped-node inspection can still report local datadir, config paths, service state, log paths, locally collected health signals, metrics policy, and build provenance when those collectors are available.

## Field Ownership

| Field | Owner | OBS-01 details |
| --- | --- | --- |
| `node` | Runtime/process collector | daemon state and version |
| `config` | Config/datadir collector | `datadir` and `config paths` |
| `service` | Service lifecycle collector | service manager and installed/enabled/running state |
| `sync` | Sync/runtime collector | `network`, `chain tip`, `sync progress`, lifecycle, phase, lag, resource pressure, recovery guidance, and last error |
| `peers` | Network collector | `peer counts` plus recent peer telemetry when durable sync state is available |
| `mempool` | Mempool collector | mempool summary |
| `wallet` | Wallet collector | `trusted_balance_sats`, `freshness`, and `scan_progress` so balances never imply completeness by themselves |
| `logs` | Logging collector | log paths and retention |
| `metrics` | Metrics collector | retention, enabled series, and bounded samples when a metrics snapshot exists |
| `health_signals` | Log/status collectors | recent `health signals` |
| `build` | Build/release collector | version, commit, build time, target, and profile |

## Stopped-node status

Stopped-node status must not omit live fields. Fields that cannot be collected because the daemon is stopped use `Unavailable` with a `reason`. For example, live `network`, `chain tip`, `sync progress`, `peer counts`, mempool, and wallet values can be unavailable while datadir, config paths, service state, logs, metrics policy, health signals, and build provenance remain visible.

`node.state = stopped` can also mean live RPC was not attempted because the
operator side could not rediscover credentials for the selected datadir. That
bootstrap distinction should surface through warning health signals, not a
separate top-level status field.

When durable sync metadata exists, stopped or unreachable-node status may still
surface the last known sync lifecycle, phase, lag, peer telemetry, recovery
guidance, and last sync error from the durable store rather than collapsing
those fields back to renderer-local guesses.

## Sync progress semantics

`sync.sync_progress` separates validated header, durable download, and connected
chainstate progress:

- `header_height`: best validated header height.
- `downloaded_block_height`: highest contiguous best-chain block body available
  in the durable store.
- `connected_block_height`: active chainstate height.
- `block_height`: compatibility alias for `connected_block_height`.

Consumers should use the explicit downloaded and connected fields for recovery
diagnostics. `last_error` and `recovery_action` are separate fields so a status
snapshot can report active progress and the latest recoverable error at the
same time.

## Sync resource pressure

`sync.resource_pressure` reports observed pressure and configured bounds
together. Consumers should treat `blocks_in_flight`, `outbound_peers`, and
durable progress counters as observations, while the `max_*` fields are the
currently configured runtime envelope:

- `max_header_requests_in_flight_per_peer`
- `max_headers_per_message`
- `max_blocks_in_flight_per_peer`
- `max_blocks_in_flight_total`
- `max_messages_per_peer`
- `max_sync_rounds`
- `target_outbound_peers`

This keeps status, dashboard, RPC JSON, and support reports aligned on one
source of truth for public-network runtime bounds.

## Build provenance semantics

`build.version` should reflect the workspace package version, and the remaining
`build.*` fields should come from truthful compile-time metadata supplied by the
active build system.

- Cargo builds can surface Cargo `TARGET` and `PROFILE` values.
- Bazel builds can surface Bazel `TARGET_CPU` and `COMPILATION_MODE` values.

Consumers should treat those strings as build-system-specific provenance, not as
one normalized cross-build enum.

## Wallet freshness semantics

`wallet.trusted_balance_sats` remains part of the shared snapshot, but operator-facing consumers must treat it as incomplete unless `wallet.freshness` says otherwise.

- `fresh`: the wallet view has caught up to the durable node tip.
- `stale`: the wallet tip lags the durable node tip and no active scan progress is being reported.
- `partial`: the wallet view is incomplete and only partial scan progress is known.
- `scanning`: an active rescan is in progress and `wallet.scan_progress` reports the current `scanned_through_height` and `target_tip_height`.

When the daemon is stopped or the wallet state cannot be collected, both `wallet.freshness` and `wallet.scan_progress` stay `Unavailable` with a reason instead of silently defaulting to a balance-only summary.

## Non-Goals

This contract does not implement a status command, renderer, dashboard, service manager, RPC collector, filesystem collector, or clock source. Later collectors map their evidence into this data model.
