---
phase: 35
phase_name: "Daemon Mainnet Sync Activation"
plan_id: "35-02"
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: "35-2026-05-01T21-26-04"
generated_at: "2026-05-01T21:29:26.254Z"
status: completed
---

# Summary 35-02: Daemon Sync Bootstrap Preflight

## Completed

- Wired `open-bitcoind` startup to run sync preflight before binding RPC when daemon sync is enabled.
- Added preflight validation that requires an existing datadir.
- Opened `FjallNodeStore` and constructed `DurableSyncRuntime` from the selected runtime config.
- Reported mode, datadir, best header height, and best block height for enabled preflight.
- Preserved disabled-mode behavior for current local RPC startup.

## Tests Added

- Disabled sync skips preflight.
- Enabled sync opens the durable runtime and returns a durable-state summary without starting transport.
- Enabled sync fails before RPC bind when no datadir is available.

## Residual Risks

- Preflight intentionally drops the sync runtime after construction. A long-lived daemon sync task, shutdown handling for that task, and peer transport startup remain later v1.2 work.

