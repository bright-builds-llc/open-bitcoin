---
status: complete
phase: 30-configless-live-status-and-dashboard-bootstrap
source:
  - 30-01-SUMMARY.md
  - 30-02-SUMMARY.md
started: 2026-05-07T09:52:26.668Z
updated: 2026-05-07T09:58:40.842Z
---

## Current Test

[testing complete]

## Tests

### 1. Configless status attempts live RPC
expected: From the repo root, create `/tmp/open-bitcoin-uat-phase-30` with a `.cookie` file and no `bitcoin.conf`, then run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --datadir=/tmp/open-bitcoin-uat-phase-30 status --format json`. The command exits successfully and reports `node.state` as `unreachable`, not `stopped`, proving status attempted live RPC bootstrap without an implicit config file.
result: pass
evidence: Command exited 0 and JSON reported `node.state` as `unreachable` with RPC connection refused details.

### 2. Configless dashboard reuses live RPC bootstrap
expected: Using the same configless datadir with `.cookie` and no `bitcoin.conf`, run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --datadir=/tmp/open-bitcoin-uat-phase-30 dashboard --format json`. The command exits successfully and reports `node.state` as `unreachable`, matching `status` through the shared bootstrap path.
result: pass
evidence: Command exited 0 and JSON reported `node.state` as `unreachable`, matching the status command through dashboard JSON output.

### 3. No-credential fallback remains stopped
expected: Remove the temporary `.cookie` while keeping `bitcoin.conf` absent, then run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --datadir=/tmp/open-bitcoin-uat-phase-30 status --format json`. The command exits successfully, reports `node.state` as `stopped`, and includes a `live_rpc_bootstrap` health signal explaining that live RPC was not attempted because no rediscoverable RPC credentials were found.
result: pass
evidence: Command exited 0, JSON reported `node.state` as `stopped`, and `health_signals` included a `live_rpc_bootstrap` warning about missing rediscoverable RPC credentials.

### 4. Runtime guide documents configless bootstrap truthfully
expected: Open `docs/operator/runtime-guide.md` and inspect the Status And Dashboard section. It should say `status` and `dashboard` reuse the selected datadir, network, and normal RPC auth sources; a datadir-local `bitcoin.conf` is canonical for user/password auth, but a discoverable `.cookie` also works; if neither exists, the command falls back to a stopped snapshot with a live-RPC bootstrap warning.
result: pass
evidence: `docs/operator/runtime-guide.md` Status And Dashboard section contains the expected datadir, network, RPC auth, `.cookie`, stopped fallback, and live-RPC bootstrap warning language.

## Summary

total: 4
passed: 4
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps
