---
status: complete
phase: 35-daemon-mainnet-sync-activation
source:
  - .planning/phases/35-daemon-mainnet-sync-activation/35-01-SUMMARY.md
  - .planning/phases/35-daemon-mainnet-sync-activation/35-02-SUMMARY.md
  - .planning/phases/35-daemon-mainnet-sync-activation/35-03-SUMMARY.md
started: 2026-05-11T03:14:30.393Z
updated: 2026-05-11T03:21:19.148Z
---

## Current Test

[testing complete]

## Tests

### 1. Default Disabled Daemon Startup
expected: With no Open Bitcoin sync config and no -openbitcoinsync override, open-bitcoind uses the existing local RPC startup path. Startup does not require a mainnet datadir and does not run sync preflight or peer transport.
result: pass
evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind sync -- --nocapture` passed `disabled_sync_skips_daemon_preflight`.

### 2. Open Bitcoin JSONC Activation and CLI Override
expected: An Open Bitcoin JSONC file with sync.network_enabled = true and sync.mode = "mainnet-ibd" enables daemon sync preflight. -openbitcoinsync=mainnet-ibd can enable the same path from the daemon CLI, and -openbitcoinsync=disabled overrides an enabled JSONC config back to disabled.
result: pass
evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc daemon_sync -- --nocapture` passed JSONC activation and CLI enable/disable override tests.

### 3. Invalid Activation Rejections
expected: Partial sync activation, non-mainnet activation, or an explicitly missing Open Bitcoin JSONC path fails before RPC bind with deterministic actionable errors. bitcoin.conf remains strict and does not accept Open Bitcoin-only sync keys.
result: pass
evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc daemon_sync -- --nocapture` passed partial, non-mainnet, invalid peer, and missing JSONC rejection tests; `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc bitcoin_conf_rejects_open_bitcoin_only_keys -- --nocapture` passed the bitcoin.conf boundary test.

### 4. Preflight Durable State Summary
expected: Enabled daemon sync preflight requires an existing datadir, opens the durable store, constructs DurableSyncRuntime, and reports mode, datadir, best header height, and best block height without starting peer transport.
result: pass
evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind sync -- --nocapture` passed `enabled_sync_opens_durable_runtime_without_starting_transport` and `enabled_sync_requires_datadir_before_daemon_binds_rpc`.

### 5. Operator Documentation Boundaries
expected: README, operator runtime docs, config precedence docs, parity docs, AGENTS, and planning docs describe Phase 35 as disabled-by-default activation/preflight only and clearly leave DNS seeds, outbound peers, IBD, unattended full sync, and long-lived sync shutdown to later phases.
result: skipped
reason: Current HEAD includes intentional later-phase updates from Phases 36-40. Phase 35 artifacts, AGENTS, architecture notes, and parity docs preserve the Phase 35 activation/preflight boundary, but README and operator runtime docs now describe the current v1.2 sync worker, operator control, and live-smoke surfaces added after Phase 35.

## Summary

total: 5
passed: 4
issues: 0
pending: 0
skipped: 1
blocked: 0

## Gaps

[none yet]
