# Open Bitcoin Architecture

Last updated: 2026-10-03

Open Bitcoin v2.1 shipped and was archived on 2026-07-22. Open Bitcoin v2.2 shipped and was archived on 2026-08-22. Open Bitcoin v2.3 shipped and was archived on 2026-09-20. Open Bitcoin v2.4 shipped and was archived on 2026-10-03. v2.5 Prune-Aware Compact-Filter Serving is planned in Phases 154–162; compact-filter support is not yet shipped. Start its first phase with `/gsd-discuss-phase 154`. Use `/gsd-new-milestone` for later milestone cycles.

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

The shipped v2.4 prune surface keeps height/window/lock decisions in the pure
chainstate crate and logical payload accounting, paired Fjall deletion and
recovery in node adapters. Ordinary automatic flush activity and manual prune
share the existing serialized authority; nonempty automatic plans force the
full coins/chain-metadata checkpoint. Explicit prune mode with a datadir opens
recovered durable storage without activating networking. Retention targets are
soft logical value-byte targets, not physical disk bounds. Shared wallet
replacement eligibility checks requested and selected creating payloads;
probing and saving remain separate effects. The [archived audit](milestones/v2.4-MILESTONE-AUDIT.md)
preserves the three accepted recovery/sink/counter advisories and fixture limits.

## Sync Boundary

The configured local daemon path consumes proved canonical genesis into its
`DurableSyncRuntime`/RPC context for all four existing selectors. An explicit
durable trigger such as BASIC initializes an admissible empty datadir; actual
daemon parsing requires the explicitly supplied directory to exist. Reopen
independently proves complete genesis/header/coins/work bindings. Explicit BASIC
uses validated genesis lineage and its zero-predecessor commitment. The genesis
context carries an unavailable wallet; ordinary maintenance workers stay absent.

Offline operator `sync status` reopens existing proved genesis and preserves saved
BASIC settings without saving synthesized status/progress. An admitted Fjall
open can have filesystem effects. Bounded private-copy inspection and refusal
preservation remain separate from successful backend recovery. Cooperating
application leases cover supported Unix; recorded process evidence is macOS.

Ordinary history and sync/inbound/relay/serving activation remain refused pending
contextual prerequisites, including Signet challenge validation. Historical
infrastructure includes the explicit opt-in full-sync path, bounded, explicit,
default-off inbound serving, v2.0 transaction relay, v2.1 block serving and
v2.1 compact-block relay. These are not public defaults and do not authorize the
current canonical route. Older fixture, sync, reorg, soak and live-smoke machinery
retains its historical evidence scope.
Actual pinned Knots-daemon differential qualification belongs to Phase168.
Public defaults, unattended full sync, production service and production-funds
readiness remain outside the claim. See the [current genesis evidence](../docs/parity/v2-6-canonical-durable-bootstrap.md).

## Migration Boundary

Migration from Bitcoin Core or Bitcoin Knots remains detection and dry-run
planning only. Source datadirs, services, configs, cookies, and external wallets
are high-value user data and must not be mutated unless a later milestone
explicitly designs and verifies an apply-mode workflow.
