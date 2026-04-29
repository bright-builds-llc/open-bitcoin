# Status Snapshot Contract

## OpenBitcoinStatusSnapshot

`OpenBitcoinStatusSnapshot` is the sole shared status model for later CLI status output, JSON automation, service diagnostics, dashboard panels, and support reports. Live RPC is not the only status source; stopped-node inspection can still report local datadir, config paths, service state, log paths, locally collected health signals, metrics policy, and build provenance when those collectors are available.

## Field Ownership

| Field | Owner | OBS-01 details |
| --- | --- | --- |
| `node` | Runtime/process collector | daemon state and version |
| `config` | Config/datadir collector | `datadir` and `config paths` |
| `service` | Service lifecycle collector | service manager and installed/enabled/running state |
| `sync` | Sync/runtime collector | `network`, `chain tip`, and `sync progress` |
| `peers` | Network collector | `peer counts` |
| `mempool` | Mempool collector | mempool summary |
| `wallet` | Wallet collector | `trusted_balance_sats`, `freshness`, and `scan_progress` so balances never imply completeness by themselves |
| `logs` | Logging collector | log paths and retention |
| `metrics` | Metrics collector | retention, enabled series, and bounded samples when a metrics snapshot exists |
| `health_signals` | Log/status collectors | recent `health signals` |
| `build` | Build/release collector | version, commit, build time, target, and profile |

## Stopped-node status

Stopped-node status must not omit live fields. Fields that cannot be collected because the daemon is stopped use `Unavailable` with a `reason`. For example, live `network`, `chain tip`, `sync progress`, `peer counts`, mempool, and wallet values can be unavailable while datadir, config paths, service state, logs, metrics policy, health signals, and build provenance remain visible.

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
