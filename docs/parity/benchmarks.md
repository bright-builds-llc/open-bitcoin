# Benchmarks

Open Bitcoin keeps repo-owned benchmark reports as audit and trend evidence, not
release timing gates. Phase 22 extends the earlier mapping-first benchmark suite
with deterministic runtime evidence for sync, storage, status, dashboard, and
wallet-rescan behavior while keeping the default local verify path bounded and
offline.

## Benchmark Scope

Benchmarks now cover two layers:

- pure or fixture-backed audit groups for consensus, codec, chainstate,
  mempool, network, wallet, and RPC or CLI behavior
- runtime-hardening groups for headers sync, block download or connect, durable
  storage reopen, runtime-collected status or dashboard rendering, and
  wallet-rescan cost

The default smoke path remains threshold-free so `bash scripts/verify.sh` can
run it without turning machine-local timing into a release criterion.

## Modes

Use the repo-owned wrapper:

```bash
bash scripts/run-benchmarks.sh --smoke
bash scripts/run-benchmarks.sh --full --iterations 5
```

Mode behavior:

- `--smoke` runs the benchmark binary in the debug profile and is the mode used
  by `bash scripts/verify.sh`
- `--full` runs the benchmark binary in the release profile for deeper local
  inspection
- both modes remain threshold-free and evidence-oriented

List the currently registered groups:

```bash
bash scripts/run-benchmarks.sh --list
```

Override the report location when needed:

```bash
OPEN_BITCOIN_BENCHMARK_REPORT_DIR=/tmp/open-bitcoin-benchmarks bash scripts/run-benchmarks.sh --smoke
```

## Benchmark Groups

The `open-bitcoin-bench` registry emits these v1.1 groups:

| Group | Surface |
| --- | --- |
| `consensus-script` | Consensus script validation |
| `block-transaction-codec` | Block and transaction parsing or serialization |
| `chainstate` | Chainstate connect, disconnect, reorg, and storage-adjacent operations |
| `mempool-policy` | Mempool admission, replacement, and policy accounting |
| `network-wire-sync` | Network wire encoding, address management, peer policy, and sync planning |
| `sync-runtime` | Headers sync plus block download or connect through the durable runtime |
| `storage-recovery` | Durable storage write or read and restart recovery |
| `operator-runtime` | Status rendering and dashboard projection from runtime-collected operator snapshots |
| `wallet` | Wallet balance, coin selection, signing, and transaction creation |
| `wallet-rescan` | Durable wallet rescan runtime and managed-wallet freshness updates |
| `rpc-cli` | RPC and CLI request dispatch |

## Report Contract

Generated JSON and Markdown reports now record:

- benchmark mode and iteration count
- binary profile (`debug` or `release`)
- threshold-free intent
- per-case measurement focus, fixture type, and durability level
- Knots benchmark names or source anchors when a direct benchmark mapping exists

This keeps the output auditable even when a runtime-hardening case has no
defensible one-to-one Knots benchmark name.

## Knots Mapping

The checked-in benchmark registry maps each Open Bitcoin group to the pinned
Bitcoin Knots `29.3.knots20260210` benchmark names or source files where that
mapping is defensible. Mapping-only remains the default because it is
deterministic, does not require a local Knots build, and keeps CI independent
of optional external binaries.

Knots execution is still optional report enrichment through explicit JSON or
binary paths:

```bash
bash scripts/run-benchmarks.sh --smoke --knots-json path/to/knots.json
bash scripts/run-benchmarks.sh --smoke --knots-bin path/to/bitcoin-knots
```

Those paths are recorded as report metadata only. They are not required for the
normal smoke verification path.

## Reports

By default, reports are written under `packages/target/benchmark-reports`:

- `open-bitcoin-bench-smoke.json`
- `open-bitcoin-bench-smoke.md`
- `open-bitcoin-bench-full.json`
- `open-bitcoin-bench-full.md`

CI uploads the generated benchmark report directory as a separate
`benchmark-reports` artifact. The artifact is for audit review and trend
inspection; benchmark reports are not release timing gates.

## Non-Goals

- Benchmarks do not replace parity tests, fuzz or property tests, or
  correctness review.
- Benchmark reports do not define pass or fail timing thresholds for release
  decisions.
- The default smoke path does not build, launch, or require Bitcoin Knots.
- The default smoke path does not require public-network access.
- Optional Knots JSON or binary inputs do not change the mapping-first default
  comparison.
