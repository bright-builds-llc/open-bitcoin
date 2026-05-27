# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 284 |
| Total lines | 93,174 |
| Code/content lines | 82,316 |
| Comment-only lines | 2,791 |
| Blank lines | 8,067 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,596 | 0 | 80 | 3,676 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 55 | 12,067 | 7,723 | 86 | 19,876 | 64.0% |
| open-bitcoin-codec | 13 | 1,144 | 170 | 28 | 1,345 | 14.9% |
| open-bitcoin-consensus | 30 | 6,346 | 7,519 | 28 | 13,893 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 10 | 1,867 | 1,268 | 30 | 3,165 | 67.9% |
| open-bitcoin-network | 11 | 2,003 | 1,654 | 30 | 3,687 | 82.6% |
| open-bitcoin-node | 33 | 8,085 | 3,950 | 36 | 12,071 | 48.9% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 32 | 5,676 | 2,572 | 53 | 8,301 | 45.3% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 16 | 3,523 | 2,467 | 34 | 6,024 | 70.0% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 174 | 46,908 | 40,592 | 1,555 | 4,761 |
| Rust tests | 46 | 29,141 | 25,718 | 1,192 | 2,231 |
| Fixture/data | 6 | 8,217 | 8,212 | 5 | 0 |
| TOML/config | 16 | 3,429 | 3,066 | 0 | 363 |
| TypeScript/Bun scripts | 5 | 3,078 | 2,723 | 0 | 355 |
| Shell scripts | 13 | 1,828 | 1,507 | 33 | 288 |
| Bazel/Starlark | 18 | 406 | 371 | 0 | 35 |
| YAML | 2 | 104 | 83 | 4 | 17 |
| CI/templates | 1 | 27 | 16 | 1 | 10 |
| Other config | 2 | 26 | 22 | 0 | 4 |
| Hooks | 1 | 10 | 6 | 1 | 3 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 8,201 |
| 2 | packages/open-bitcoin-consensus/src/script/tests.rs | Rust tests | 3,258 |
| 3 | packages/Cargo.lock | TOML/config | 3,186 |
| 4 | packages/open-bitcoin-node/src/sync/tests.rs | Rust tests | 2,328 |
| 5 | scripts/run-live-mainnet-smoke.ts | TypeScript/Bun scripts | 1,693 |
| 6 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 7 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 8 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,384 |
| 9 | packages/open-bitcoin-cli/tests/operator_binary.rs | Rust tests | 1,365 |
| 10 | packages/open-bitcoin-cli/src/operator/status/tests.rs | Rust tests | 1,133 |
| 11 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 1,046 |
| 12 | packages/open-bitcoin-mempool/src/pool/tests.rs | Rust tests | 964 |
| 13 | packages/open-bitcoin-consensus/tests/parity_closure.rs | Rust tests | 940 |
| 14 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 936 |
| 15 | packages/open-bitcoin-cli/src/operator/service/tests.rs | Rust tests | 891 |
| 16 | packages/open-bitcoin-wallet/src/descriptor/tests.rs | Rust tests | 842 |
| 17 | packages/open-bitcoin-cli/tests/operator_flows.rs | Rust tests | 767 |
| 18 | packages/open-bitcoin-rpc/src/config/tests.rs | Rust tests | 712 |
| 19 | packages/open-bitcoin-node/src/storage/fjall_store/tests.rs | Rust tests | 668 |
| 20 | packages/open-bitcoin-wallet/src/address.rs | Rust production | 626 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 7fed9ca92a6b29f99008d27782a7e017620bf23b13a9b6d118b116a7c69b65b8 |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
