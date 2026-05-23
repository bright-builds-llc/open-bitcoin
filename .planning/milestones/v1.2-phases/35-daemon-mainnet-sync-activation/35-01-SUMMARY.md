---
phase: 35
phase_name: "Daemon Mainnet Sync Activation"
plan_id: "35-01"
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: "35-2026-05-01T21-26-04"
generated_at: "2026-05-01T21:29:26.254Z"
status: completed
---

# Summary 35-01: Sync Activation Config Contract

## Completed

- Added `DaemonSyncMode` and `DaemonSyncConfig` to `RuntimeConfig`, defaulting daemon sync to disabled.
- Extended `open-bitcoin.jsonc` sync config with `mode`, while preserving `network_enabled`.
- Added `-openbitcoinconf=<path>` and `-openbitcoinsync=disabled|mainnet-ibd` daemon CLI parsing.
- Loaded optional datadir-local Open Bitcoin JSONC without making missing default JSONC fatal.
- Kept Open Bitcoin-only sync activation out of `bitcoin.conf`.

## Tests Added

- Default runtime config leaves daemon sync disabled.
- JSONC can express `network_enabled = true` plus `mode = "mainnet-ibd"`.
- Daemon loader resolves explicit JSONC activation.
- CLI override can enable sync or disable an enabled JSONC config.
- Partial JSONC activation and non-mainnet activation fail with deterministic errors.
- Explicit missing Open Bitcoin JSONC path fails with an actionable error.

## Residual Risks

- Phase 35 does not resolve DNS seeds, open outbound peers, or run IBD; those remain Phases 36-38.

