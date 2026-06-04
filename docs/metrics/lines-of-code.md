# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 288 |
| Total lines | 97,276 |
| Code/content lines | 85,958 |
| Comment-only lines | 2,944 |
| Blank lines | 8,374 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,596 | 0 | 80 | 3,676 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 55 | 12,146 | 7,876 | 86 | 20,108 | 64.8% |
| open-bitcoin-codec | 13 | 1,144 | 170 | 28 | 1,345 | 14.9% |
| open-bitcoin-consensus | 30 | 6,346 | 7,519 | 28 | 13,893 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 10 | 1,867 | 1,268 | 30 | 3,165 | 67.9% |
| open-bitcoin-network | 12 | 2,581 | 1,654 | 30 | 4,265 | 64.1% |
| open-bitcoin-node | 35 | 8,617 | 5,584 | 36 | 14,237 | 64.8% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 32 | 5,717 | 2,608 | 53 | 8,378 | 45.6% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 16 | 3,523 | 2,467 | 34 | 6,024 | 70.0% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 177 | 48,138 | 41,653 | 1,610 | 4,875 |
| Rust tests | 46 | 30,964 | 27,313 | 1,287 | 2,364 |
| Fixture/data | 6 | 8,217 | 8,212 | 5 | 0 |
| TypeScript/Bun scripts | 6 | 3,544 | 3,150 | 0 | 394 |
| TOML/config | 16 | 3,429 | 3,066 | 0 | 363 |
| Shell scripts | 13 | 2,411 | 2,066 | 36 | 309 |
| Bazel/Starlark | 18 | 406 | 371 | 0 | 35 |
| YAML | 2 | 104 | 83 | 4 | 17 |
| CI/templates | 1 | 27 | 16 | 1 | 10 |
| Other config | 2 | 26 | 22 | 0 | 4 |
| Hooks | 1 | 10 | 6 | 1 | 3 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 8,201 |
| 2 | packages/open-bitcoin-node/src/sync/tests.rs | Rust tests | 3,787 |
| 3 | packages/open-bitcoin-consensus/src/script/tests.rs | Rust tests | 3,258 |
| 4 | packages/Cargo.lock | TOML/config | 3,186 |
| 5 | scripts/run-live-mainnet-smoke.ts | TypeScript/Bun scripts | 1,975 |
| 6 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 7 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 8 | packages/open-bitcoin-cli/tests/operator_binary.rs | Rust tests | 1,518 |
| 9 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,384 |
| 10 | packages/open-bitcoin-cli/src/operator/status/tests.rs | Rust tests | 1,133 |
| 11 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 1,048 |
| 12 | packages/open-bitcoin-mempool/src/pool/tests.rs | Rust tests | 964 |
| 13 | scripts/test-run-live-mainnet-smoke.sh | Shell scripts | 952 |
| 14 | packages/open-bitcoin-consensus/tests/parity_closure.rs | Rust tests | 940 |
| 15 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 936 |
| 16 | packages/open-bitcoin-cli/src/operator/service/tests.rs | Rust tests | 891 |
| 17 | packages/open-bitcoin-wallet/src/descriptor/tests.rs | Rust tests | 842 |
| 18 | packages/open-bitcoin-cli/tests/operator_flows.rs | Rust tests | 767 |
| 19 | packages/open-bitcoin-rpc/src/config/tests.rs | Rust tests | 746 |
| 20 | packages/open-bitcoin-node/src/storage/fjall_store/tests.rs | Rust tests | 668 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 45efde480f06d6e625e2801be283b570b1b6043f5e763b1cdc332aeaffec66f8 |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
