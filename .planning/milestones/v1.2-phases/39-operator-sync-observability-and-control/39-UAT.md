---
status: complete
phase: 39-operator-sync-observability-and-control
source:
  - .planning/phases/39-operator-sync-observability-and-control/39-01-SUMMARY.md
started: 2026-05-15T10:20:15.113Z
updated: 2026-05-23T02:22:20Z
---

## Current Test

[testing complete]

## Tests

### 1. Operator Sync Status CLI
expected: Run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-preview sync status --format json` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-preview sync status --format json` against a review datadir. The command succeeds without manual store-file inspection and prints durable sync metadata, including `sync_control.paused` plus the available lifecycle, phase, and last-update state.
result: pass
evidence: "`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-phase39-uat sync status --format json` passed and printed runtime metadata with `sync_control.paused: false`."

### 2. Operator Sync Pause And Resume
expected: Run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-preview sync pause`, then `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-preview sync status --format json`, then `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-preview sync resume`. The pause command reports that daemon sync paused, status shows `sync_control.paused` as `true`, resume reports that daemon sync resumed, and the next status shows `sync_control.paused` as `false`. The same flow also works through `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.
result: pass
evidence: "Cargo operator CLI pause/status/resume flow passed against `/tmp/open-bitcoin-phase39-uat`: pause reported daemon sync paused, JSON status showed `sync_control.paused: true`, resume reported daemon sync resumed, and final JSON status showed `sync_control.paused: false`. Regression test `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_sync_pause_and_resume_update_durable_control_state --test operator_binary -- --nocapture` also passed."

### 3. Shared Operator Status And Dashboard Sync Truth
expected: Run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --datadir=/tmp/open-bitcoin-preview status --format json --no-color` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --network regtest --datadir=/tmp/open-bitcoin-preview status --format json --no-color`. The status output exposes the durable sync lifecycle, phase, lag or unavailable reason, resource pressure, peer counts, and recent error or recovery guidance when available instead of reducing initial block download to `headers == blocks`. The dashboard view uses the same sync and peer truth.
result: pass
evidence: "`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --datadir=/tmp/open-bitcoin-phase39-uat status --format json --no-color` and `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --datadir=/tmp/open-bitcoin-phase39-uat --format json --no-color dashboard --tick-ms 1000` both passed and returned shared status snapshot JSON with explicit sync lifecycle, phase, lag, resource pressure, peer-count, error, and recovery fields. `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node populated_snapshot_serializes_obs_01_fields -- --nocapture`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_projection_includes_required_sections_and_charts -- --nocapture`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_status_json_uses_fake_running_rpc --test operator_binary -- --nocapture`, and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_dashboard_json_is_snapshot_and_ansi_free --test operator_binary -- --nocapture` passed."

### 4. RPC Blockchain Info Uses Durable Sync Truth
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc blockchain_info_uses_durable_sync_truth_when_available -- --nocapture`. The filtered RPC test passes, proving `getblockchaininfo` reports the durable sync chain tip, header height, block height, progress, IBD state, warnings, and verification-progress truth when durable sync metadata is present.
result: pass
evidence: "`cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc blockchain_info_uses_durable_sync_truth_when_available -- --nocapture` passed: 1 RPC test passed with 43 lib tests filtered out."

### 5. Daemon Sync Activation Honors Durable Control
expected: Start `open-bitcoind` with the documented opt-in mainnet sync review path and an existing datadir, then use `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=<DATADIR> sync pause` and `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=<DATADIR> sync resume` while the daemon is running. The daemon-owned sync worker keeps durable sync state current, observes the durable pause flag without editing internal files, and remains explicitly scoped as an operator review workflow rather than an unattended production-node claim.
result: pass
evidence: "User reran the live `open-bitcoind` mainnet UAT after the gap-closure fix and reported that the documented `sync status`, `sync pause`, and `sync resume` steps passed against the running daemon without the prior `FjallError: Locked` or timeout failure."

## Summary

total: 5
passed: 5
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

- truth: "Start `open-bitcoind` with the documented opt-in mainnet sync review path and an existing datadir, then use `open-bitcoin sync pause` and `open-bitcoin sync resume` while the daemon is running. The daemon-owned sync worker keeps durable sync state current and observes the durable pause flag without editing internal files."
  status: resolved
  reason: "User reported: while `open-bitcoind` was running, `open-bitcoin sync status`, `open-bitcoin sync pause`, and `open-bitcoin sync resume` all failed with `storage backend failure in runtime: FjallError: Locked; Restart the node and retry the storage operation.`"
  severity: blocker
  test: 5
  root_cause: "`open-bitcoind` opens the Fjall-backed durable store through `start_daemon_sync_worker` and keeps it open in the daemon sync worker. The operator CLI sync control path independently calls `FjallNodeStore::open(data_dir)` before reading or writing `RuntimeMetadata`. Fjall rejects the second process while the daemon owns the store lock, so the documented out-of-process `sync status|pause|resume` control surface cannot operate against a live daemon."
  artifacts:
    - path: "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs"
      issue: "`start_daemon_sync_worker` opens `FjallNodeStore` for the daemon and keeps that store alive for the worker lifetime."
    - path: "packages/open-bitcoin-cli/src/operator/runtime/support.rs"
      issue: "`execute_sync_command` opens the same datadir with `FjallNodeStore::open` for status, pause, and resume instead of using a daemon-accessible control channel."
    - path: "packages/open-bitcoin-node/src/storage.rs"
      issue: "`RuntimeMetadata.sync_control` is persisted in the same Fjall store as daemon-owned runtime state, making live out-of-process control depend on a store that is already locked."
  missing:
    - "Move live sync status/pause/resume control to a daemon-accessible surface, such as local RPC methods handled by the running `open-bitcoind`, or split the control flag/status summary into a concurrency-safe sidecar store that the daemon and CLI can both access."
    - "Add a regression that starts a daemon-owned store holder and proves operator sync status/pause/resume do not fail with Fjall `Locked` while the daemon is active."
  debug_session: ""
  resolved_by: "Phase 39 Plan 02 added authenticated daemon RPC sync-control methods and a store-backed daemon control handle that avoids both second-process Fjall opens and sync-worker channel timeout."
  verified: "User reran the original live mainnet `open-bitcoind` UAT on 2026-05-23 and reported that it passed."

## Gap Closure Update

- date: 2026-05-23
- status: fixed deterministically; live mainnet retest passed
- fix: "`open-bitcoin sync status|pause|resume` now attempts authenticated local RPC first when daemon RPC is configured, using `openbitcoinsyncstatus`, `openbitcoinsyncpause`, and `openbitcoinsyncresume` served by the running `open-bitcoind` process. Offline direct-store access remains the fallback only when no local RPC is reachable or configured."
- evidence: "Regression tests cover a held Fjall store lock with live RPC and prove the CLI no longer reports `FjallError: Locked`; auth failures from a reachable daemon are terminal and do not fall back to direct-store mutation. Full repo verification passed after refreshing `docs/metrics/lines-of-code.md`."
- retest: "User reran the original public-mainnet `open-bitcoind` UAT against a fresh daemon process on 2026-05-23 and reported that it passed."

## Timeout Follow-Up

- date: 2026-05-23
- reported: "The first live retest reached daemon RPC but failed with `daemon sync control timed out`."
- root_cause: "The RPC handler used a control channel serviced by the daemon sync worker, but the worker can spend longer than the control timeout inside a live `sync_until_idle` network round."
- fix: "`open-bitcoind` now installs a store-backed `DaemonSyncControl` handle using a clone of the daemon process's already-open Fjall database handle. RPC status, pause, and resume no longer wait for the sync worker to receive a channel message."
- evidence: "The store-backed RPC regression passed, and a local live-shape daemon check against `/tmp/open-bitcoin-mainnet-uat-codex-timeout` returned from `sync status`, `sync pause`, JSON status with `sync_control.paused: true`, and `sync resume` without timeout or `FjallError: Locked`."
- retest: "User reran the same commands against the live `/tmp/open-bitcoin-mainnet-uat` daemon after rebuilding from this working tree and reported that they passed."

## Live UAT Acceptance

- date: 2026-05-23
- result: passed
- source: "User-reported live UAT result after rerunning the documented open-bitcoind mainnet sync-control steps."
- accepted_scope: "Confirms the Phase 39 operator sync-control path through a real running daemon. It does not expand the phase's explicit scope into an unattended production-node claim."
