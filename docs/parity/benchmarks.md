# Benchmarks

Open Bitcoin keeps repo-owned benchmark reports as audit and trend evidence, not release timing gates. The default benchmark comparison is mapping-only: each report records the Bitcoin Knots benchmark names and source files that correspond to the Open Bitcoin group, without requiring Knots execution.

## Benchmark Scope

Benchmarks cover the in-scope headless node and wallet surfaces where stable smoke evidence helps reviewers inspect parity readiness. The smoke path is intentionally bounded and threshold-free so `scripts/verify.sh` can run it without turning local machine timing into a release criterion.

Full benchmark mode is available for deeper local investigation, but release readiness still depends on correctness, parity tests, and reviewed evidence rather than elapsed-time thresholds.

## Benchmark Groups

The `open-bitcoin-bench` registry emits these D-01 groups:

| Group | Surface |
| --- | --- |
| `consensus-script` | Consensus script validation |
| `block-transaction-codec` | Block and transaction parsing or serialization |
| `chainstate` | Chainstate connect, disconnect, reorg, and storage-adjacent operations |
| `mempool-policy` | Mempool admission, replacement, and policy accounting |
| `network-wire-sync` | Network wire encoding, address management, peer policy, and sync planning |
| `wallet` | Wallet balance, coin selection, signing, and transaction creation |
| `rpc-cli` | RPC and CLI request dispatch |

## Knots Mapping

The checked-in benchmark registry maps each Open Bitcoin group to the pinned Bitcoin Knots `29.3.knots20260210` benchmark names and source files. Mapping-only is the default because it is deterministic, does not require a local Knots build, and keeps CI independent of optional external binaries.

Knots execution is optional report enrichment through explicit JSON/bin paths:

```bash
bash scripts/run-benchmarks.sh --smoke --knots-json path/to/knots.json
bash scripts/run-benchmarks.sh --smoke --knots-bin path/to/bitcoin-knots
```

Those paths are recorded as report metadata. They are not required for normal smoke verification.

## Running Locally

List the registered groups:

```bash
bash scripts/run-benchmarks.sh --list
```

Run the bounded smoke suite through the repo-owned wrapper:

```bash
bash scripts/run-benchmarks.sh --smoke
```

Override the report location when needed:

```bash
OPEN_BITCOIN_BENCHMARK_REPORT_DIR=/tmp/open-bitcoin-benchmarks bash scripts/run-benchmarks.sh --smoke
```

Run a full local benchmark with an explicit iteration count:

```bash
bash scripts/run-benchmarks.sh --full --iterations 100
```

## Reports

By default, reports are written under `packages/target/benchmark-reports`:

- `open-bitcoin-bench-smoke.json`
- `open-bitcoin-bench-smoke.md`
- `open-bitcoin-bench-full.json`
- `open-bitcoin-bench-full.md`

CI uploads the generated benchmark report directory as a separate `benchmark-reports` artifact. The artifact is for audit review and trend inspection; benchmark reports are not release timing gates.

## Non-Goals

- Benchmarks do not replace parity tests, fuzz/property tests, or correctness review.
- Benchmark reports do not define pass/fail timing thresholds for release decisions.
- The default smoke path does not build, launch, or require Bitcoin Knots.
- Optional Knots JSON/bin inputs do not change the mapping-only default comparison.
