# Open Bitcoin Architecture

Last updated: 2026-05-01

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

`DurableSyncRuntime` provides real-network sync foundations, durable state
integration, TCP peer transport, telemetry, and benchmarkable runtime behavior.
It is not yet wired into `open-bitcoind` as an unattended public-mainnet
full-sync daemon loop. Docs and parity claims must preserve that boundary until
v1.2 implements and verifies daemon-integrated sync operation.

## Migration Boundary

Migration from Bitcoin Core or Bitcoin Knots remains detection and dry-run
planning only. Source datadirs, services, configs, cookies, and external wallets
are high-value user data and must not be mutated unless a later milestone
explicitly designs and verifies an apply-mode workflow.
