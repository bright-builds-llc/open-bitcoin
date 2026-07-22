# Open Bitcoin Architecture

Last updated: 2026-07-22

Open Bitcoin v2.1 shipped and was archived on 2026-07-22. Future milestone
work starts with `/gsd-new-milestone`.

## Architectural Shape

Open Bitcoin follows a functional-core / imperative-shell architecture.
Consensus, chainstate, mempool, wallet, wire parsing, and other business rules
should stay in pure first-party crates. File systems, sockets, clocks, terminal
I/O, service managers, RPC HTTP, durable storage adapters, and process control
belong in shell-owned packages.

## Package Boundaries

- `open-bitcoin-primitives`, `open-bitcoin-codec`, `open-bitcoin-consensus`,
  `open-bitcoin-chainstate`, `open-bitcoin-mempool`,
  `open-bitcoin-network`, and `open-bitcoin-wallet` hold the pure-core Bitcoin
  behavior surface.
- `open-bitcoin-node` owns adapter-facing orchestration, durable storage,
  status, metrics, logs, sync runtime foundations, and wallet rescan runtime.
- `open-bitcoin-rpc` owns the JSON-RPC dispatch and current `open-bitcoind`
  server binary.
- `open-bitcoin-cli` owns both binaries: `open-bitcoin-cli` for
  Bitcoin/Knots-compatible RPC invocation and `open-bitcoin` for Open
  Bitcoin-specific operator workflows.
- `open-bitcoin-test-harness` and `open-bitcoin-bench` keep parity,
  integration, and benchmark evidence reusable.

## Operator Runtime Model

The shared `OpenBitcoinStatusSnapshot` is the common status contract for CLI
status, dashboard, service diagnostics, JSON automation, support reports, and
stopped-node inspection. Renderers should consume that model instead of
inventing local status truth.

Open Bitcoin-owned operator workflows are intentionally separate from the
baseline-compatible RPC client parser. `open-bitcoin` owns onboarding, status,
service, dashboard, migration planning, and managed-wallet helper flows.
`open-bitcoin-cli` keeps Bitcoin/Knots-style RPC flags and method routing.

## Config And Storage

Configuration precedence is:

`CLI flags > environment > Open Bitcoin JSONC > bitcoin.conf > cookies > defaults`

Baseline-compatible settings stay in `bitcoin.conf`. Open Bitcoin-only
onboarding, service, dashboard, migration, metrics, logging, storage, and sync
settings stay in `open-bitcoin.jsonc`.

The v1.1 storage decision is Fjall. Concrete storage effects are contained in
node-shell adapters, while storage contracts and recovery actions stay typed and
auditable.

## Sync Boundary

`DurableSyncRuntime` now provides the explicit opt-in full-sync path with
durable state integration, peer lifecycle, header-first IBD, block
download/connect, telemetry, restart/resume evidence, and live-mainnet smoke
review. The same shell composes bounded, explicit, default-off inbound serving,
v2.0 transaction relay, v2.1 block serving, and v2.1 compact-block relay.
These are review boundaries, not public defaults: unattended public-network
operation, public serving or relay defaults, production service/deployment,
production full-node readiness, and production-funds wallet use remain outside
the current claim.

## Migration Boundary

Migration from Bitcoin Core or Bitcoin Knots remains detection and dry-run
planning only. Source datadirs, services, configs, cookies, and external wallets
are high-value user data and must not be mutated unless a later milestone
explicitly designs and verifies an apply-mode workflow.
