---
status: complete
phase: 13-operator-runtime-foundations
source:
  - 13-01-SUMMARY.md
  - 13-02-SUMMARY.md
  - 13-03-SUMMARY.md
  - 13-04-SUMMARY.md
  - 13-05-SUMMARY.md
started: 2026-05-03T01:55:20Z
updated: 2026-05-03T01:58:54Z
---

## Current Test

[testing complete]

## Tests

### 1. Operator CLI routing boundary
expected: Run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --help`. It should print the operator tree with `status`, `sync`, `config`, `service`, `dashboard`, `migrate`, `onboard`, and `wallet`, plus shared flags `--config`, `--datadir`, `--network`, `--format`, and `--no-color`. Then run `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -getinfo`. It should stay on the compatibility path and fail for missing RPC credentials instead of being parsed as an operator subcommand.
result: pass

### 2. Open Bitcoin JSONC ownership and precedence
expected: Open `docs/architecture/config-precedence.md`. It should say `open-bitcoin.jsonc` is the Open Bitcoin-owned, user-editable JSONC file for wizard, onboarding, dashboard, service, migration, metrics, logging, storage, and sync settings; the precedence order is `CLI flags > environment > Open Bitcoin JSONC > bitcoin.conf > cookies > defaults`; and Open Bitcoin-only keys such as `dashboard`, `service`, or `openbitcoinsync` must not be accepted in `bitcoin.conf`.
result: pass

### 3. Durable storage decision and recovery contract
expected: Open `docs/architecture/storage-decision.md` and `packages/open-bitcoin-node/src/storage.rs`. They should show `fjall` as the Phase 13 storage decision target, compare it against `redb` and `rocksdb`, and expose typed namespaces, schema versions, persist modes, recovery actions, and storage errors without introducing concrete database I/O in this phase.
result: pass

### 4. Observability retention defaults
expected: Open `docs/architecture/operator-observability.md`, `packages/open-bitcoin-node/src/metrics.rs`, and `packages/open-bitcoin-node/src/logging.rs`. They should define bounded metrics retention at 30 second sampling, 2880 samples, and 24 hours max age, plus structured log retention with daily rotation, 14 files, 14 days, and 268435456 bytes total retained size, while keeping Phase 13 limited to serializable contracts rather than live writers or pruning jobs.
result: pass

### 5. Shared status snapshot unavailable semantics
expected: Open `docs/architecture/status-snapshot.md` and `packages/open-bitcoin-node/src/status.rs`. They should define `OpenBitcoinStatusSnapshot` as the shared operator-facing status model covering node, config, service, sync, peers, mempool, wallet, logs, metrics, health signals, and build provenance, and they should keep stopped or unreachable live fields explicit as `Unavailable` with a reason instead of hiding them.
result: pass

## Summary

total: 5
passed: 5
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
