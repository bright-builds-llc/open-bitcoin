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

> Status: Open Bitcoin v2.1 provides bounded, explicit, default-off block
> serving and compact-block relay with deterministic local evidence and optional
> public-network operator review. The release evidence is summarized in
> [`docs/parity/release-readiness.md`](./docs/parity/release-readiness.md).
> Phase 128 closes the production composition for compact negotiation,
> durable-tip announcement fanout, real transport writes, and post-write
> aggregate evidence. Phase 129 completed the aggregate integration guard over
> the four repaired flows, independent verification of the reassigned
> requirements, and the final milestone reconciliation; the rerun v2.1
> milestone audit passed and the milestone is archive-ready pending the
> completion workflow.
> Package relay, BIP37 bloom-filter serving, compact-filter serving, public
> serving or relay defaults, archive-node and production-scale historical
> serving, public-network CI or release gates, production service/deployment,
> production full-node readiness, production-funds wallet use, packaging, GUI
> and hosted dashboards, migration apply mode, destructive repair, and
> automatic support upload remain deferred or unsupported.

## Parity At A Glance

The current status source is the parity ledger:
[`docs/parity/index.json`](./docs/parity/index.json), the human checklist
[`docs/parity/checklist.md`](./docs/parity/checklist.md), the release-readiness
handoff [`docs/parity/release-readiness.md`](./docs/parity/release-readiness.md),
the v1.8/v1.9 production claim boundary
[`docs/parity/production-claim-boundary.md`](./docs/parity/production-claim-boundary.md),
the support matrix
[`docs/parity/support-matrix.md`](./docs/parity/support-matrix.md),
the source-built upgrade, rollback, backup, and compatibility decisions policy
[`docs/parity/upgrade-and-rollback-policy.md`](./docs/parity/upgrade-and-rollback-policy.md),
the operator runbooks
[`docs/parity/operator-runbooks.md`](./docs/parity/operator-runbooks.md),
the service operation expectations
[`docs/parity/service-operation-expectations.md`](./docs/parity/service-operation-expectations.md),
the v1.9 network participation release-boundary closeout
[`docs/parity/release-readiness.md#v19-network-participation-evidence-and-release-boundary`](./docs/parity/release-readiness.md#v19-network-participation-evidence-and-release-boundary),
and project state [`.planning/STATE.md`](./.planning/STATE.md). Older roadmap
or requirements rows may lag those artifacts.
The compact Phase 87 v1.8 release-readiness checklist remains at
docs/parity/release-readiness.md#v18-release-readiness-checklist for legacy
guardrails.

| Surface | Bitcoin Knots baseline | Open Bitcoin | Evidence | Notes |
| --- | --- | --- | --- | --- |
| Reference baseline | `29.3.knots20260210` vendored under `packages/bitcoin-knots/` | ✓ done | [`docs/parity/index.json`](./docs/parity/index.json) | The pinned baseline is the external behavior contract. |
| Core domain and serialization | Amounts, hashes, scripts, transactions, blocks, and wire framing | ✓ done | [`catalog/core-domain-and-serialization.md`](./docs/parity/catalog/core-domain-and-serialization.md) | Rust types preserve Bitcoin encoding and identity boundaries. |
| Consensus and validation | Script execution, transaction checks, block checks, PoW, merkle behavior | ✓ done | [`catalog/consensus-validation.md`](./docs/parity/catalog/consensus-validation.md) | Consensus parity includes legacy, segwit-v0, taproot, and parity-closure fixes. |
| Chainstate and UTXO engine | Connect, disconnect, reorg, UTXO, undo, and best-chain behavior | ✓ done | [`catalog/chainstate.md`](./docs/parity/catalog/chainstate.md) | Disk-backed databases and full manager behavior remain follow-up depth. |
| Mempool policy | Admission, replacement, fee accounting, ancestor/descendant, eviction | ✓ done | [`catalog/mempool-policy.md`](./docs/parity/catalog/mempool-policy.md) | Long-lived pressure and package-relay depth remain future work. |
| P2P networking and sync | Handshake, peer lifecycle, headers, blocks, inventory, tx relay | ✓ done | [`catalog/p2p.md`](./docs/parity/catalog/p2p.md) | v2.1 adds bounded, explicit, default-off block serving and compact-block relay with aggregate local evidence; package relay, bloom/filter serving, public defaults, archive/production-scale serving, public-network gates, and production readiness remain deferred. |
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
| Typed resource-bound evidence for status, dashboard, soak preflight/resource_stop reports, and support bundles | [`docs/architecture/status-snapshot.md`](./docs/architecture/status-snapshot.md), [`docs/operator/runtime-guide.md`](./docs/operator/runtime-guide.md) |
| Diagnosis-only recovery evidence for `safe_retry`, `read_only_inspection`, `backup_then_rebuild`, `stop_and_escalate`, `stale_lock_evidence`, and `concurrent_datadir_use` | [`docs/architecture/status-snapshot.md`](./docs/architecture/status-snapshot.md), [`docs/operator/runtime-guide.md`](./docs/operator/runtime-guide.md) |
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

Install the repo-managed Git hooks:

```bash
bash scripts/install-git-hooks.sh
```

That installer is safe to rerun. `bash scripts/verify.sh` also self-heals the
local `core.hooksPath` setting when it is missing or wrong, so the repo-managed
hooks stay active for normal local work.

Run the repo-native verification flow:

```bash
bash scripts/verify.sh
```

Local verification records overall and per-step durations under the disposable,
gitignored `.local/open-bitcoin-dev/` directory. Inspect the current runs and
median, p90, p95, and maximum history with:

```bash
bun run scripts/command-timings.ts report
```

Use the same timing wrapper for ad-hoc Cargo or Bazel work. A checkout-scoped
cooperative lock serializes commands that share the Cargo target directory;
waiting and running commands emit a heartbeat every 60 seconds.

```bash
bun run scripts/command-timings.ts run --key cargo-test-workspace -- \
  cargo test --manifest-path packages/Cargo.toml --workspace --all-features
```

Historical durations are advisory, not hard timeouts. Quiet output or a polling
yield is not evidence that a command is stuck. When a compiled Rust test appears
to stall before its harness starts, compile the selected target and run the
exact emitted executable with `--list` five times while retaining macOS
`sample`, `lsof`, process, and disk evidence after a 10-second soft threshold:

```bash
bun run scripts/diagnose-rust-test-stall.ts
```

The clean reproduction refuses to start with less than 100 GiB of filesystem
headroom. The diagnostic never terminates a process at the soft threshold.
Supply an explicit `--stop-after-ms <milliseconds>` only when an operator wants
a hard stop. Dead cooperative locks are preserved under
`.local/open-bitcoin-dev/locks/abandoned/` and recovered automatically. If a
shared artifact tree is confirmed stale or corrupt, stop only this checkout's
build processes, then reclaim only its regenerable caches:

```bash
bazel shutdown
cargo clean --manifest-path packages/Cargo.toml
bazel clean --expunge
df -h .
```

Do not begin the clean reproduction until the final command reports at least
100 GiB available on the active Cargo target filesystem. An isolated
`CARGO_TARGET_DIR` on another volume is supported and receives a separate lock
and timing history classification.

`docs/metrics/lines-of-code.md` is an intentionally tracked generated artifact.
Hook and verification flows may refresh it when first-party code, scripts, or
tracked hook content changes.

## Operator Preview

v2.0 closes the bounded transaction relay and mempool participation evidence
boundary through deterministic local traceability, repo-local UAT commands, and
no-claim guardrails. It covers Phase 100 relay activation, Phase 101 inventory
and download scheduling, Phase 102 orphan/admission bridging, Phase 103 mempool
lifecycle and recovery, Phase 104 relay serving/fanout, Phase 105 operator
evidence, Phase 106 parity/UAT/release-boundary closeout, Phase 107 runtime
activation/download eligibility integration, and Phase 108 durable mempool
relay-state recovery. Recovered accepted records re-enter managed serving and
fanout identity state without startup socket I/O, and status/support surfaces
show fixed `Relay recovery` fields: `recovered_count`,
`dropped_confirmed_count`, `dropped_duplicate_count`,
`dropped_missing_parent_count`, `dropped_policy_incompatible_count`, and
`dropped_evicted_count`. v2.0 does not claim public relay by default,
public relay defaults, compact block relay, package relay, bloom/filter
serving, public-network relay CI, production full-node readiness,
production-service operation, or production-funds wallet use.

v1.9 closes the network participation boundary while preserving the Phase 82
support terms and evidence gates required before a future production full-node
readiness claim. It does not claim production full-node readiness. The canonical
boundary is
[`docs/parity/production-claim-boundary.md`](./docs/parity/production-claim-boundary.md).
v1.8 defines the support terms and evidence gates required before a future production full-node readiness claim; v1.9 adds bounded opt-in inbound evidence without broadening that claim.
The companion support matrix
[`docs/parity/support-matrix.md`](./docs/parity/support-matrix.md)
classifies source-built install, runtime, bounded opt-in inbound evidence,
storage, service-supervision, wallet, migration, packaging, dashboard, GUI,
support-upload, destructive-repair, and verification/CI surfaces by the same
Phase 82 support terms and does not claim production full-node readiness.
For source-built upgrade, rollback, backup, and compatibility decisions, use
[`docs/parity/upgrade-and-rollback-policy.md`](./docs/parity/upgrade-and-rollback-policy.md).
For production-boundary preflight, long-run monitoring, no-progress diagnosis,
recovery/stop decisions, redacted support-bundle timelines, and escalation
evidence, use
[`docs/parity/operator-runbooks.md`](./docs/parity/operator-runbooks.md).
For direct `open-bitcoind` operation, source-built service preview,
opt-in launchd/systemd lifecycle UAT, restart/resume fields, repo-local
Cargo/Bazel commands, and production-service non-claims, use
[`docs/parity/service-operation-expectations.md`](./docs/parity/service-operation-expectations.md).
For release review, use the v1.9 network participation closeout in
[`docs/parity/release-readiness.md`](./docs/parity/release-readiness.md), which
maps the current Phase 95 boundary requirements to canonical evidence,
deterministic checks, UAT or manual evidence, residual risk, and no-claim or
next-gate status.
The v1.8 deterministic claim guardrails prevent overbroad production-readiness
and deferred-surface claims in the public release/operator docs; they do not
claim production full-node readiness.
v1.7 remains historical source-built, explicit opt-in full-sync soak and recovery hardening evidence covering durable multi-day soak evidence, resource
bounds, recovery diagnosis, progress guarantees, stall diagnosis,
support-bundle forensics, and deterministic release-boundary checks.
For the practical install, onboarding, service, status, dashboard, migration,
benchmark, limitation, and v1.7 UAT workflow, start with
[`docs/operator/runtime-guide.md`](./docs/operator/runtime-guide.md). For the
v1.9 boundary and historical v1.7 review posture, use the runtime guide with
[`docs/parity/production-claim-boundary.md`](./docs/parity/production-claim-boundary.md),
[`docs/parity/release-readiness.md`](./docs/parity/release-readiness.md),
[`docs/parity/index.json`](./docs/parity/index.json), and
[`docs/parity/checklist.md`](./docs/parity/checklist.md).
The preview commands below start the current local RPC/operator surfaces; they
are not a production-node claim. `open-bitcoind` now has an opt-in mainnet sync
loop plus durable header-and-block sync foundations, truthful operator-facing
sync status, and explicit `open-bitcoin sync pause|resume|status` controls.
Status evidence also includes bounded branch/reorg details through
`sync.latest_reorg` and typed no-progress guidance through
`sync.no_progress_diagnosis` plus `sync.no_progress_next_action`.
Explicit live-mainnet smoke evidence now lives in
[`scripts/run-live-mainnet-smoke.ts`](./scripts/run-live-mainnet-smoke.ts),
which writes local JSON and Markdown reports under
`packages/target/live-mainnet-smoke-reports` without changing the default
hermetic verification contract.

v1.6 remains historical source-built, explicit opt-in full-sync completion
evidence. v1.7 remains historical source-built, explicit opt-in full-sync soak and recovery hardening evidence without broadening release claims. It preserves bounded opt-in
full-sync soak behavior, durable resume evidence, or diagnosed blocker
evidence. v1.9 has bounded opt-in inbound listener/admission, permission,
address-boundary, eviction/ban, and resource-governance evidence for local
review only. It does not claim public inbound defaults, transaction relay,
compact block relay, mempool propagation, full address relay, block serving,
production-funds wallet safety, production-funds wallet use, migration apply
mode, signed packaging, Windows service integration, hosted dashboards, GUI
parity, public-network default checks, public-network CI, release-blocking live
sync, automatic support-bundle upload, destructive repair, or broad production-node readiness.

The commands below are a minimal regtest preview. Create a scratch data
directory, start the RPC server, then call it from another shell:

```bash
mkdir -p /tmp/open-bitcoin-preview
cat > /tmp/open-bitcoin-preview/bitcoin.conf <<'EOF'
regtest=1
rpcconnect=127.0.0.1
rpcport=18443
rpcuser=preview
rpcpassword=preview
EOF

cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- \
  -datadir=/tmp/open-bitcoin-preview
```

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- \
  -rpcconnect=127.0.0.1 -rpcport=18443 \
  -rpcuser=preview -rpcpassword=preview getblockchaininfo
```

`open-bitcoind` CLI flags are not automatically rediscoverable by later
operator commands. `open-bitcoin status` and `open-bitcoin dashboard` need a
normal RPC auth source they can resolve themselves from the selected datadir,
such as `bitcoin.conf` or a discoverable `.cookie`.

The Open Bitcoin-specific operator binary exposes status, config discovery, and
first-run onboarding flows:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-preview --network regtest status --format human --no-color
```

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-preview sync status --format json
```

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --network regtest --datadir=/tmp/open-bitcoin-preview \
  --config=/tmp/open-bitcoin-preview/open-bitcoin.jsonc \
  onboard --non-interactive --approve-write --detect-existing
```

`onboard` writes only `open-bitcoin.jsonc`; it intentionally does not create or
modify `bitcoin.conf`. If you start `open-bitcoind` separately and want later
`open-bitcoin status` calls to reach live RPC, keep the baseline-compatible RPC
auth in `bitcoin.conf` or an equivalent discoverable `.cookie`.

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
- [`docs/parity/release-readiness.md`](./docs/parity/release-readiness.md) is the current v2.1 review handoff and historical release-evidence index.
- [`docs/parity/support-matrix.md`](./docs/parity/support-matrix.md) is the canonical support classification and issue-evidence checklist.
