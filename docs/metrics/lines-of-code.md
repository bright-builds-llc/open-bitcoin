# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 227 |
| Total lines | 64,774 |
| Code/content lines | 56,912 |
| Comment-only lines | 2,083 |
| Blank lines | 5,779 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 17 | 2,186 | 0 | 73 | 2,259 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 34 | 5,493 | 4,362 | 70 | 9,925 | 79.4% |
| open-bitcoin-codec | 13 | 1,144 | 170 | 28 | 1,345 | 14.9% |
| open-bitcoin-consensus | 30 | 6,346 | 7,519 | 28 | 13,893 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 10 | 1,867 | 1,268 | 30 | 3,165 | 67.9% |
| open-bitcoin-network | 11 | 1,729 | 1,416 | 30 | 3,175 | 81.9% |
| open-bitcoin-node | 24 | 5,186 | 2,450 | 36 | 7,672 | 47.2% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 21 | 3,173 | 1,632 | 53 | 4,858 | 51.4% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 12 | 2,245 | 1,292 | 34 | 3,571 | 57.6% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 128 | 31,970 | 27,306 | 1,228 | 3,436 |
| Rust tests | 42 | 21,927 | 19,412 | 823 | 1,692 |
| Fixture/data | 6 | 6,057 | 6,052 | 5 | 0 |
| TOML/config | 16 | 2,101 | 1,871 | 0 | 230 |
| Shell scripts | 9 | 1,179 | 945 | 21 | 213 |
| TypeScript/Bun scripts | 2 | 988 | 849 | 0 | 139 |
| Bazel/Starlark | 18 | 385 | 350 | 0 | 35 |
| YAML | 2 | 104 | 83 | 4 | 17 |
| CI/templates | 1 | 27 | 16 | 1 | 10 |
| Other config | 2 | 26 | 22 | 0 | 4 |
| Hooks | 1 | 10 | 6 | 1 | 3 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 6,041 |
| 2 | packages/open-bitcoin-consensus/src/script/tests.rs | Rust tests | 3,258 |
| 3 | packages/Cargo.lock | TOML/config | 1,861 |
| 4 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 5 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 6 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,051 |
| 7 | packages/open-bitcoin-node/src/sync/tests.rs | Rust tests | 990 |
| 8 | packages/open-bitcoin-mempool/src/pool/tests.rs | Rust tests | 964 |
| 9 | packages/open-bitcoin-consensus/tests/parity_closure.rs | Rust tests | 940 |
| 10 | packages/open-bitcoin-cli/tests/operator_flows.rs | Rust tests | 770 |
| 11 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 698 |
| 12 | packages/open-bitcoin-cli/src/operator/status/tests.rs | Rust tests | 627 |
| 13 | packages/open-bitcoin-wallet/src/address.rs | Rust production | 626 |
| 14 | packages/open-bitcoin-consensus/src/transaction.rs | Rust production | 623 |
| 15 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 621 |
| 16 | packages/open-bitcoin-mempool/src/pool.rs | Rust production | 615 |
| 17 | packages/open-bitcoin-rpc/src/config/loader.rs | Rust production | 604 |
| 18 | packages/open-bitcoin-mempool/src/policy.rs | Rust production | 582 |
| 19 | packages/open-bitcoin-cli/src/operator/status.rs | Rust production | 575 |
| 20 | packages/open-bitcoin-node/src/storage/fjall_store/tests.rs | Rust tests | 563 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 38b0de3c830f58ca8e829566d38e5cdbeddf020fa01497ded7ffa39510f5a09c |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
