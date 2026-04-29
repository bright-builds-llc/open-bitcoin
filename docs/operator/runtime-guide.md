# Operator Runtime Guide

This guide describes the current v1.1 operator workflow for Open Bitcoin on
macOS and Linux. It is intentionally conservative: the runtime is source-built,
service integration is local-machine only, migration remains dry-run only, and
release readiness stays evidence-based rather than timing-threshold based.

Use this guide for the practical workflow. Use
[`docs/architecture/config-precedence.md`](../architecture/config-precedence.md),
[`docs/architecture/status-snapshot.md`](../architecture/status-snapshot.md),
and [`docs/parity/`](../parity/) when you need the lower-level contracts and
audit record.

## Install From Source

The current install path is source-built. From the repo root:

```bash
git submodule update --init --recursive
bun install
bash scripts/install-git-hooks.sh
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
```

Before making release or operator claims on a checkout, run the repo-native
verification contract:

```bash
bash scripts/verify.sh
```

That verification path stays offline by default. It runs formatting, linting,
builds, tests, parity-breadcrumb checks, bounded smoke benchmarks, and Bazel
smoke targets without requiring public-network sync.

## Binaries

The current source build exposes three relevant binaries:

- `open-bitcoind` for the node or RPC server runtime
- `open-bitcoin-cli` for the baseline-compatible RPC client path
- `open-bitcoin` for Open Bitcoin-owned operator workflows such as onboarding,
  status, service management, dashboard, migration planning, and managed-wallet
  helpers

You can run them directly from `packages/target/{debug,release}/` after
building or through `cargo run`.

## Datadir And Config Ownership

Open Bitcoin keeps baseline-compatible settings in `bitcoin.conf` and
Open Bitcoin-only settings in `open-bitcoin.jsonc`.

The precedence order is:

`CLI flags > environment > Open Bitcoin JSONC > bitcoin.conf > cookies > defaults`

The intended split is:

- `bitcoin.conf`: baseline-compatible node and RPC settings
- `open-bitcoin.jsonc`: onboarding answers, service settings, dashboard options,
  migration metadata, metrics and logging paths, storage settings, and sync
  knobs
- cookie files: RPC auth fallback only

The onboarding and migration flows should not write Open Bitcoin-only keys into
`bitcoin.conf`. See
[`docs/architecture/config-precedence.md`](../architecture/config-precedence.md)
for the stricter contract language.

## First Run And Onboarding

A common local workflow is:

```bash
mkdir -p /tmp/open-bitcoin-preview

cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- \
  -regtest -datadir=/tmp/open-bitcoin-preview -rpcport=18443 \
  -rpcuser=preview -rpcpassword=preview
```

Then, from another shell:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --network regtest --datadir=/tmp/open-bitcoin-preview status --format human --no-color
```

To write the Open Bitcoin-owned JSONC config non-interactively:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --network regtest --datadir=/tmp/open-bitcoin-preview \
  --config=/tmp/open-bitcoin-preview/open-bitcoin.jsonc \
  onboard --non-interactive --approve-write --detect-existing
```

Important onboarding behaviors:

- `--approve-write` is required before onboarding writes files.
- `--detect-existing` asks the onboarding flow to inspect existing Core or Knots
  evidence in supported locations.
- `--force-overwrite` is available when a previously generated
  `open-bitcoin.jsonc` must be replaced deliberately.
- `--disable-metrics` and `--disable-logs` let operators opt out of those local
  runtime surfaces.

## Service Lifecycle

Open Bitcoin has repo-owned service integration for macOS `launchd` and Linux
`systemd`.

The current commands are:

```bash
open-bitcoin service status
open-bitcoin service install
open-bitcoin service install --apply
open-bitcoin service enable
open-bitcoin service disable
open-bitcoin service uninstall
open-bitcoin service uninstall --apply
```

Service lifecycle notes:

- `install` and `uninstall` are dry-run previews unless `--apply` is supplied.
- `status`, `enable`, and `disable` operate against the platform service manager
  directly.
- Service configuration is derived from the selected datadir, config path, and
  operator log directory rather than from a separate service-only config file.
- When that managed log directory is available, the generated plist or unit
  derives a concrete service-managed log file at
  `<log_dir>/open-bitcoin.log`, and `open-bitcoin service status` surfaces the
  effective path from the installed service definition.
- The dashboard action bar reuses the same service lifecycle operations.

## Status And Dashboard

`open-bitcoin status` is the shared operator summary surface. It can render in
human or JSON form and keeps stopped-node fields visible with explicit
`Unavailable` reasons where live runtime data is missing.

```bash
open-bitcoin --network regtest --datadir=/tmp/open-bitcoin-preview status --format json
```

`open-bitcoin dashboard` reuses the same shared status snapshot:

- on a TTY, it opens the interactive ratatui dashboard
- on a non-TTY, it falls back to a deterministic text snapshot
- with `--format json`, it emits the shared snapshot as JSON

Example:

```bash
open-bitcoin --network regtest --datadir=/tmp/open-bitcoin-preview dashboard --tick-ms 1000
```

Interpretation guidance:

- `Unavailable` means the collector chose to report absence explicitly instead
  of inventing a default value.
- `wallet.freshness` matters as much as `trusted_balance_sats`; a balance alone
  does not imply the wallet view is current.
- `dashboard` and `status` both surface the same node, config, service, sync,
  peer, mempool, wallet, log, metrics, health, and build sections.

For the shared data contract, see
[`docs/architecture/status-snapshot.md`](../architecture/status-snapshot.md).

## Migration Planning

Phase 21 added a read-only migration planner for existing Core or Knots
installations:

```bash
open-bitcoin --network regtest --datadir=/tmp/open-bitcoin-preview migrate plan \
  --source-datadir=/tmp/source/.bitcoin
```

The current migration contract is intentionally limited:

- it is explanation-first and dry-run only
- it detects existing installs, datadirs, configs, services, cookies, and wallet
  candidates
- it explains backup requirements, rollback expectations, and intentional
  differences before any later cutover work
- it does not disable source services, mutate source datadirs, or rewrite
  external wallets

Use [`docs/parity/catalog/drop-in-audit-and-migration.md`](../parity/catalog/drop-in-audit-and-migration.md)
for the current audit matrix and explicit non-claims.

## Real-Sync Verification And Benchmarks

Open Bitcoin keeps benchmark evidence as reproducible local reports, not release
timing gates.

Use the repo-owned wrapper:

```bash
bash scripts/run-benchmarks.sh --smoke
bash scripts/run-benchmarks.sh --full --iterations 5
```

Benchmark modes:

- `--smoke` is the bounded local path used by `bash scripts/verify.sh`; it runs
  the benchmark binary in the debug profile and writes reports under
  `packages/target/benchmark-reports`
- `--full` uses a release build for deeper local inspection and trend review
- both modes remain threshold-free; correctness and reviewed evidence matter
  more than elapsed-time pass or fail numbers

The generated reports now record:

- the benchmark mode and iteration count
- the binary profile (`debug` or `release`)
- the measurement focus, fixture type, and durability level for each case
- the relevant Knots benchmark names or source anchors when they exist

## Known Limitations

Open Bitcoin does not currently claim all of the following:

- packaged or signed release installation flows
- Windows service support
- automatic migration apply, source-service cutover, or source-datadir mutation
- external-wallet import, restore, or rewrite
- public-network sync as part of the default local verification contract
- a hosted public dashboard or GUI parity with the reference Qt app

The parity ledger and deferred-surface record live under
[`docs/parity/`](../parity/). Start with:

- [`docs/parity/index.json`](../parity/index.json)
- [`docs/parity/checklist.md`](../parity/checklist.md)
- [`docs/parity/deviations-and-unknowns.md`](../parity/deviations-and-unknowns.md)
