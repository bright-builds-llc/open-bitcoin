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
| `wallet` | Wallet collector | wallet summary |
| `logs` | Logging collector | log paths and retention |
| `metrics` | Metrics collector | retention and enabled series |
| `health_signals` | Log/status collectors | recent `health signals` |
| `build` | Build/release collector | version, commit, build time, target, and profile |

## Stopped-node status

Stopped-node status must not omit live fields. Fields that cannot be collected because the daemon is stopped use `Unavailable` with a `reason`. For example, live `network`, `chain tip`, `sync progress`, `peer counts`, mempool, and wallet values can be unavailable while datadir, config paths, service state, logs, metrics policy, health signals, and build provenance remain visible.

## Non-Goals

This contract does not implement a status command, renderer, dashboard, service manager, RPC collector, filesystem collector, or clock source. Later collectors map their evidence into this data model.
