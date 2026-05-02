# Operator Runtime Guide

This guide describes the current v1.2 operator workflow for Open Bitcoin on
macOS and Linux. It is intentionally conservative: the runtime is source-built,
service integration is local-machine only, migration remains dry-run only, and
release readiness stays evidence-based rather than timing-threshold based.
It does not make a production-node or production-funds claim for unattended
public-mainnet operation yet.

Use this guide for the practical workflow. Use
[`docs/architecture/config-precedence.md`](../architecture/config-precedence.md),
[`docs/architecture/status-snapshot.md`](../architecture/status-snapshot.md),
and [`docs/parity/`](../parity/) when you need the lower-level contracts and
audit record.

## Install From Source

The current install path is source-built. From the repo root:

```bash
git submodule update --init --recursive
bun --version  # should match .bun-version
bash scripts/install-git-hooks.sh
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
```

Bun is required as the pinned runtime for repo-owned TypeScript automation used
by `bash scripts/verify.sh`. This repository does not have a `package.json`, so
there is no `bun install` step. The hook installer is safe to rerun, and
`bash scripts/verify.sh` will self-heal the local repo hook configuration
outside CI when `core.hooksPath` is missing or wrong.

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

- `open-bitcoind` for the current local JSON-RPC server runtime
- `open-bitcoin-cli` for the baseline-compatible RPC client path
- `open-bitcoin` for Open Bitcoin-owned operator workflows such as onboarding,
  status, service management, dashboard, migration planning, and managed-wallet
  helpers

`open-bitcoind` now has an explicit mainnet sync activation path with a
daemon-owned bounded sync loop. When enabled, daemon startup opens the selected
durable store, constructs `DurableSyncRuntime`, starts the sync worker, and
keeps truthful durable sync state available to status, dashboard, RPC, and
operator CLI control surfaces. This is still an operator-ready review workflow,
not a production-node claim.

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

## Mainnet Sync Activation

Mainnet sync activation is disabled by default. It can be enabled only for the
mainnet chain through Open Bitcoin-owned config or an `open-bitcoind` CLI
override.

JSONC form:

```jsonc
{
  "sync": {
    "network_enabled": true,
    "mode": "mainnet-ibd",
    "manual_peers": ["198.51.100.10:8333"],
    "dns_seeds": ["seed.bitcoin.sipa.be", "dnsseed.bluematt.me"],
    "target_outbound_peers": 2
  }
}
```

Daemon CLI form:

```bash
mkdir -p /tmp/open-bitcoin-mainnet

cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- \
  -datadir=/tmp/open-bitcoin-mainnet \
  -openbitcoinsync=mainnet-ibd \
  -server=1
```

Important boundaries:

- `-openbitcoinsync=mainnet-ibd` is an Open Bitcoin-only daemon flag; do not put
  it in `bitcoin.conf`.
- If the JSONC file is not at `<datadir>/open-bitcoin.jsonc`, pass the explicit
- `sync.manual_peers` configures explicit outbound peers as `host` or
  `host:port`; IPv6 literals should use bracket form such as
  `[2001:db8::7]:8333`.
- `sync.dns_seeds` overrides the default mainnet seed list. Set it to an empty
  array if you want manual peers only for deterministic or controlled testing.
- `sync.target_outbound_peers` caps how many successful outbound peer slots a
  sync round tries to satisfy before moving on.
- `sync.network_enabled = true` without `sync.mode = "mainnet-ibd"` is rejected
  so partial config does not accidentally activate public-network behavior.
- Activation is rejected on `-regtest`, `-signet`, or `-testnet`; this Phase 35
  path is only for mainnet IBD bootstrap.
- The daemon now keeps a bounded background sync loop active when mainnet sync
  is enabled, while the normal local RPC server continues to serve operator and
  wallet requests.
- `open-bitcoin status`, `open-bitcoin dashboard`, `open-bitcoin sync status`,
  and RPC `getblockchaininfo` read the same durable sync truth for header
  height, block height, lag, lifecycle, recovery guidance, and last error.
- `open-bitcoin sync pause` and `open-bitcoin sync resume` toggle the durable
  pause flag without requiring operators to inspect or edit internal store
  files directly.
- Live mainnet smoke evidence, packaged-service hardening, and milestone
  closeout remain Phase 40 work.

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

`open-bitcoin sync` is the focused control surface for daemon mainnet sync:

```bash
open-bitcoin --datadir=/tmp/open-bitcoin-preview sync status --format json
open-bitcoin --datadir=/tmp/open-bitcoin-preview sync pause
open-bitcoin --datadir=/tmp/open-bitcoin-preview sync resume
```

For live RPC bootstrap, `status` and `dashboard` reuse the selected datadir,
network, and normal RPC auth sources. A datadir-local implicit `bitcoin.conf`
is optional for this workflow, not required.

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
- Sync-focused status now includes lifecycle (`active`, `paused`,
  `recovering`, `failed`, or `stopped`), current phase, lag, resource pressure,
  recovery guidance, and the last sync error when durable state is available.
- The `build` section stays compile-time truthful across supported local build
  paths: Cargo builds surface Cargo metadata, while Bazel builds surface the
  workspace version plus Bazel target and compilation-mode identifiers.
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
- with `--source-datadir`, it only shows concrete service review paths when a
  detected service definition can be tied to the selected source install;
  otherwise service cutover review stays explicit manual follow-up
- it explains backup requirements, rollback expectations, and intentional
  differences before any later cutover work
- it does not disable source services, mutate source datadirs, or rewrite
  external wallets

Use [`docs/parity/catalog/drop-in-audit-and-migration.md`](../parity/catalog/drop-in-audit-and-migration.md)
for the current audit matrix and explicit non-claims.

## Real-Sync Verification And Benchmarks

Open Bitcoin keeps benchmark evidence as reproducible local reports, not release
timing gates.

The sync runtime has durable peer/sync foundations, TCP transport coverage, and
an opt-in daemon-owned mainnet sync loop. Public-network operation is still an
explicit opt-in review surface, is not part of the default local verification
contract, and is not yet a production-node claim.

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
- production-node or production-funds readiness for unattended public-mainnet
  operation through `open-bitcoind`
- automatic migration apply, source-service cutover, or source-datadir mutation
- external-wallet import, restore, or rewrite
- public-network sync as part of the default local verification contract
- a hosted public dashboard or GUI parity with the reference Qt app

The parity ledger and deferred-surface record live under
[`docs/parity/`](../parity/). Start with:

- [`docs/parity/index.json`](../parity/index.json)
- [`docs/parity/checklist.md`](../parity/checklist.md)
- [`docs/parity/deviations-and-unknowns.md`](../parity/deviations-and-unknowns.md)
