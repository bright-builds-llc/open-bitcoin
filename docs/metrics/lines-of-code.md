# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 271 |
| Total lines | 85,924 |
| Code/content lines | 75,796 |
| Comment-only lines | 2,655 |
| Blank lines | 7,473 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,596 | 0 | 80 | 3,676 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 50 | 10,659 | 6,858 | 86 | 17,603 | 64.3% |
| open-bitcoin-codec | 13 | 1,144 | 170 | 28 | 1,345 | 14.9% |
| open-bitcoin-consensus | 30 | 6,346 | 7,519 | 28 | 13,893 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 10 | 1,867 | 1,268 | 30 | 3,165 | 67.9% |
| open-bitcoin-network | 11 | 2,003 | 1,654 | 30 | 3,687 | 82.6% |
| open-bitcoin-node | 29 | 7,112 | 3,231 | 36 | 10,379 | 45.4% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 32 | 5,041 | 2,246 | 53 | 7,340 | 44.6% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 16 | 3,523 | 2,467 | 34 | 6,024 | 70.0% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 166 | 43,892 | 37,858 | 1,521 | 4,513 |
| Rust tests | 45 | 27,231 | 24,018 | 1,099 | 2,114 |
| Fixture/data | 6 | 8,217 | 8,212 | 5 | 0 |
| TOML/config | 16 | 3,429 | 3,066 | 0 | 363 |
| TypeScript/Bun scripts | 4 | 1,378 | 1,181 | 0 | 197 |
| Shell scripts | 10 | 1,204 | 963 | 24 | 217 |
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
| 4 | packages/open-bitcoin-node/src/sync/tests.rs | Rust tests | 1,609 |
| 5 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 6 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 7 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,384 |
| 8 | packages/open-bitcoin-cli/src/operator/status/tests.rs | Rust tests | 1,072 |
| 9 | packages/open-bitcoin-cli/tests/operator_binary.rs | Rust tests | 1,030 |
| 10 | packages/open-bitcoin-mempool/src/pool/tests.rs | Rust tests | 964 |
| 11 | packages/open-bitcoin-consensus/tests/parity_closure.rs | Rust tests | 940 |
| 12 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 936 |
| 13 | packages/open-bitcoin-cli/src/operator/service/tests.rs | Rust tests | 891 |
| 14 | packages/open-bitcoin-wallet/src/descriptor/tests.rs | Rust tests | 842 |
| 15 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 829 |
| 16 | packages/open-bitcoin-cli/tests/operator_flows.rs | Rust tests | 767 |
| 17 | packages/open-bitcoin-node/src/storage/fjall_store/tests.rs | Rust tests | 668 |
| 18 | packages/open-bitcoin-rpc/src/config/tests.rs | Rust tests | 663 |
| 19 | packages/open-bitcoin-cli/src/operator/runtime.rs | Rust production | 626 |
| 20 | packages/open-bitcoin-wallet/src/address.rs | Rust production | 626 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 0bc563be0a793b1b71fcc1f226d7b7e3cf0f5dd7c8e78220038b3acf0ac87ae4 |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
