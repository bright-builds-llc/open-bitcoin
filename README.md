# open-bitcoin

<!-- bright-builds-rules-readme-badges:begin -->

<!-- Managed upstream by bright-builds-rules. If this badge block needs a fix, open an upstream PR or issue instead of editing the downstream managed block. Keep repo-local README content outside this managed badge block. -->

[![GitHub Stars](https://img.shields.io/github/stars/bright-builds-llc/open-bitcoin)](https://github.com/bright-builds-llc/open-bitcoin)
[![CI](https://img.shields.io/github/actions/workflow/status/bright-builds-llc/open-bitcoin/ci.yml?style=flat-square&logo=github&label=CI)](https://github.com/bright-builds-llc/open-bitcoin/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/bright-builds-llc/open-bitcoin?style=flat-square)](./LICENSE)
[![Rust 1.94.1](https://img.shields.io/badge/Rust-1.94.1-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Bright Builds: Rules](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/public/badges/bright-builds-rules-flat.svg)](https://github.com/bright-builds-llc/bright-builds-rules)

<!-- bright-builds-rules-readme-badges:end -->

Open Bitcoin is a headless Bitcoin node and wallet implementation in Rust. Its
external behavior targets Bitcoin Knots `29.3.knots20260210` for the in-scope
consensus, validation, chainstate, mempool, networking, wallet, RPC, CLI, and
configuration surfaces while keeping the first-party internals strongly typed,
auditable, and modular.

> Status: the in-scope headless v1 parity surfaces are implemented and ready
> for review and operator testing. Open Bitcoin is not yet recommended for
> production funds or unattended mainnet operation.

## Parity At A Glance

The current status source is the parity ledger:
[`docs/parity/index.json`](./docs/parity/index.json), the human checklist
[`docs/parity/checklist.md`](./docs/parity/checklist.md), the release-readiness
handoff [`docs/parity/release-readiness.md`](./docs/parity/release-readiness.md),
and project state [`.planning/STATE.md`](./.planning/STATE.md). Older roadmap
or requirements rows may lag those artifacts.

| Surface | Bitcoin Knots baseline | Open Bitcoin | Evidence | Notes |
| --- | --- | --- | --- | --- |
| Reference baseline | `29.3.knots20260210` vendored under `packages/bitcoin-knots/` | ✓ done | [`docs/parity/index.json`](./docs/parity/index.json) | The pinned baseline is the external behavior contract. |
| Core domain and serialization | Amounts, hashes, scripts, transactions, blocks, and wire framing | ✓ done | [`catalog/core-domain-and-serialization.md`](./docs/parity/catalog/core-domain-and-serialization.md) | Rust types preserve Bitcoin encoding and identity boundaries. |
| Consensus and validation | Script execution, transaction checks, block checks, PoW, merkle behavior | ✓ done | [`catalog/consensus-validation.md`](./docs/parity/catalog/consensus-validation.md) | Consensus parity includes legacy, segwit-v0, taproot, and parity-closure fixes. |
| Chainstate and UTXO engine | Connect, disconnect, reorg, UTXO, undo, and best-chain behavior | ✓ done | [`catalog/chainstate.md`](./docs/parity/catalog/chainstate.md) | Disk-backed databases and full manager behavior remain follow-up depth. |
| Mempool policy | Admission, replacement, fee accounting, ancestor/descendant, eviction | ✓ done | [`catalog/mempool-policy.md`](./docs/parity/catalog/mempool-policy.md) | Long-lived pressure and package-relay depth remain future work. |
| P2P networking and sync | Handshake, peer lifecycle, headers, blocks, inventory, tx relay | ✓ done | [`catalog/p2p.md`](./docs/parity/catalog/p2p.md) | Discovery, address relay, bans, and long-running socket policy are deferred. |
| Wallet | Descriptors, addresses, balances, UTXOs, coin selection, signing | ✓ done | [`catalog/wallet.md`](./docs/parity/catalog/wallet.md) | HD, multisig, PSBT, encryption, and external signers remain follow-up surfaces. |
| RPC, CLI, and config | Local JSON-RPC, `bitcoin-cli`-style flags, config, auth, operator flows | ✓ done | [`catalog/rpc-cli-config.md`](./docs/parity/catalog/rpc-cli-config.md) | The supported slice is single-wallet and local-operator focused. |
| Verification harnesses and property tests | Functional-suite concepts and fuzz/property targets | ✓ done | [`catalog/verification-harnesses.md`](./docs/parity/catalog/verification-harnesses.md) | Managed Knots process spawning and full upstream Python-suite coverage are deferred. |
| Benchmarks and audit readiness | Benchmark mappings and release-review evidence | ✓ done | [`docs/parity/release-readiness.md`](./docs/parity/release-readiness.md) | Benchmarks are audit and trend evidence, not release timing gates. |

## Open Bitcoin Differentiators

These are Open Bitcoin design choices, not Knots parity claims:

| Capability | Where to inspect |
| --- | --- |
| First-party Rust Bitcoin domain types instead of production dependencies on existing Rust Bitcoin libraries | [`packages/`](./packages/) |
| Functional-core boundaries that keep pure business logic free of direct I/O and runtime effects | [`scripts/check-pure-core-deps.sh`](./scripts/check-pure-core-deps.sh) |
| Operator runtime contracts for storage, observability, status, CLI routing, and config layering | [`docs/architecture/`](./docs/architecture/) |
| Machine-readable parity and deviation ledger with human catalog pages | [`docs/parity/`](./docs/parity/) |
| Deterministic parity, benchmark, and lines-of-code reports for review | [`scripts/verify.sh`](./scripts/verify.sh), [`docs/metrics/lines-of-code.md`](./docs/metrics/lines-of-code.md) |
| Production panic-site guard for first-party Rust code | [`scripts/check-panic-sites.sh`](./scripts/check-panic-sites.sh) |

## Repository Layout

- `packages/bitcoin-knots/` is the pinned upstream behavioral baseline. Treat it as the reference implementation, not the first-party production path.
- `packages/open-bitcoin-primitives/`, `packages/open-bitcoin-codec/`, `packages/open-bitcoin-consensus/`, `packages/open-bitcoin-chainstate/`, `packages/open-bitcoin-mempool/`, `packages/open-bitcoin-network/`, and `packages/open-bitcoin-wallet/` hold the first-party pure-core libraries.
- `packages/open-bitcoin-node/` owns adapter-facing orchestration over the pure-core crates.
- `packages/open-bitcoin-rpc/` provides the JSON-RPC server and `open-bitcoind` binary.
- `packages/open-bitcoin-cli/` provides the `open-bitcoin-cli` client binary.
- `packages/open-bitcoin-test-harness/` and `packages/open-bitcoin-bench/` provide parity, property, and benchmark infrastructure.
- `docs/architecture/` records the operator runtime contracts behind the current storage, sync, status, service, dashboard, and migration surfaces.
- `docs/parity/` tracks parity status and intentional deviations from the pinned baseline.
- `.githooks/` contains the repo-managed Git hooks used to run the local verification contract before commit.
- `scripts/verify.sh` is the source-of-truth local verification command for first-party code.

## Contributor Quickstart

Materialize the pinned reference baseline:

```bash
git submodule update --init --recursive
```

Install the repo-managed Git hooks once per clone:

```bash
bash scripts/install-git-hooks.sh
```

Run the repo-native verification flow:

```bash
bash scripts/verify.sh
```

## Operator Preview

The current v1.2 operator runtime is source-built and intended for local review,
testing, and parity audit. For the practical install, onboarding, service,
status, dashboard, migration, benchmark, and limitation workflow, start with
[`docs/operator/runtime-guide.md`](./docs/operator/runtime-guide.md).
The preview commands below start the current local RPC/operator surfaces; they
are not an unattended public-mainnet full-sync recipe. `open-bitcoind` has an
opt-in mainnet sync preflight plus durable header-and-block sync foundations,
including bounded block download/connect and restart recovery. Full operator
controls, richer observability, and live-mainnet closeout remain later v1.2
work.

The commands below are a minimal regtest preview. Create a scratch data
directory, start the RPC server, then call it from another shell:

```bash
mkdir -p /tmp/open-bitcoin-preview
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- \
  -regtest -datadir=/tmp/open-bitcoin-preview -rpcport=18443 \
  -rpcuser=preview -rpcpassword=preview
```

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- \
  -rpcconnect=127.0.0.1 -rpcport=18443 \
  -rpcuser=preview -rpcpassword=preview getblockchaininfo
```

The Open Bitcoin-specific operator binary exposes status, config discovery, and
first-run onboarding flows:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-preview --network regtest status --format human --no-color
```

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --network regtest --datadir=/tmp/open-bitcoin-preview \
  --config=/tmp/open-bitcoin-preview/open-bitcoin.jsonc \
  onboard --non-interactive --approve-write --detect-existing
```

Operators with an existing Core or Knots install can also generate a dry-run
migration plan before any later cutover work:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --network regtest --datadir=/tmp/open-bitcoin-preview \
  migrate plan --source-datadir=/tmp/source/.bitcoin
```

That planner is explanation-first and dry-run only. It surfaces migration
tradeoffs, backup requirements, and intentional differences without mutating the
source install. See
[`docs/parity/catalog/drop-in-audit-and-migration.md`](./docs/parity/catalog/drop-in-audit-and-migration.md)
for the current audit matrix and explicit Phase 21 boundaries.

Supported baseline-backed RPC methods currently include `getblockchaininfo`,
`getmempoolinfo`, `getnetworkinfo`, `sendrawtransaction`, `deriveaddresses`,
`getwalletinfo`, `getbalances`, `listunspent`, `importdescriptors`,
`rescanblockchain`, `sendtoaddress`, `getnewaddress`, `getrawchangeaddress`,
and `listdescriptors`. Open Bitcoin also exposes deterministic extension methods
`buildtransaction` and `buildandsigntransaction` for the current wallet adapter
slice. Wallet-scoped methods honor `-rpcwallet` and `/wallet/<name>` for the
implemented subset.

The operator binary also exposes Open Bitcoin-owned wallet workflows that stay
outside the baseline-compatible parser surface:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --network regtest --datadir=/tmp/open-bitcoin-preview \
  wallet --wallet alpha send mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn 12000 \
  --fee-rate-sat-per-kvb 2000 --replaceable
```

That command renders a deterministic preview and refuses mutation until
`--confirm` is added. Managed-wallet backups are likewise Open Bitcoin-owned:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --network regtest --datadir=/tmp/open-bitcoin-preview \
  wallet --wallet alpha backup /tmp/open-bitcoin-preview/backups/alpha.json
```

The backup export is one-way JSON for the managed wallet snapshot. It rejects
destinations that overlap detected Core or Knots wallet candidates and does not
copy, restore, or mutate external wallet formats. See
[`docs/parity/catalog/wallet.md`](./docs/parity/catalog/wallet.md) for the
shipped wallet slice and explicit deferrals.

For the broader operator lifecycle, including source-built install steps,
service dry-run versus apply behavior, dashboard usage, config ownership, and
real-sync benchmark commands, see
[`docs/operator/runtime-guide.md`](./docs/operator/runtime-guide.md).

## Future Work

Known follow-up themes are tracked in
[`docs/parity/deviations-and-unknowns.md`](./docs/parity/deviations-and-unknowns.md).
High-level areas include:

- richer wallet-send RPC ergonomics beyond the current `sendtoaddress`-style path, peer-info and `-netinfo` views, full multiwallet lifecycle parity, remote-operator ACL/auth, and daemon supervision
- managed Knots process support, fuller upstream functional-suite coverage, and a dedicated fuzzing runtime
- deeper wallet, P2P, chainstate, and long-lived runtime policy behavior beyond the current headless v1 slice
- future GUI work and any hosted or public dashboard work after the local operator dashboard matures

For contributor workflow details beyond those two entrypoints, see [CONTRIBUTING.md](./CONTRIBUTING.md).

## Parity And Deviations

- [`docs/parity/README.md`](./docs/parity/README.md) explains the parity ledger and its source-of-truth role.
- [`docs/parity/index.json`](./docs/parity/index.json) is the machine-readable status index for in-scope surfaces, intentional deviations, catalog entries, checklist state, and audit roots.
- [`docs/parity/checklist.md`](./docs/parity/checklist.md) is the human-readable parity checklist.
- [`docs/parity/release-readiness.md`](./docs/parity/release-readiness.md) is the current headless v1 review handoff.
