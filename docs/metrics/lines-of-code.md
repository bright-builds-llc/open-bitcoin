# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 265 |
| Total lines | 82,453 |
| Code/content lines | 72,774 |
| Comment-only lines | 2,519 |
| Blank lines | 7,160 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,594 | 0 | 80 | 3,674 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 48 | 10,067 | 6,394 | 86 | 16,547 | 63.5% |
| open-bitcoin-codec | 13 | 1,144 | 170 | 28 | 1,345 | 14.9% |
| open-bitcoin-consensus | 30 | 6,346 | 7,519 | 28 | 13,893 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 10 | 1,867 | 1,268 | 30 | 3,165 | 67.9% |
| open-bitcoin-network | 11 | 1,729 | 1,416 | 30 | 3,175 | 81.9% |
| open-bitcoin-node | 27 | 6,406 | 2,755 | 36 | 9,197 | 43.0% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 30 | 4,576 | 1,995 | 53 | 6,624 | 43.6% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 16 | 3,523 | 2,467 | 34 | 6,024 | 70.0% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 160 | 41,853 | 36,067 | 1,464 | 4,322 |
| Rust tests | 45 | 25,802 | 22,788 | 1,022 | 1,992 |
| Fixture/data | 6 | 8,217 | 8,212 | 5 | 0 |
| TOML/config | 16 | 3,429 | 3,066 | 0 | 363 |
| TypeScript/Bun scripts | 4 | 1,378 | 1,181 | 0 | 197 |
| Shell scripts | 10 | 1,201 | 962 | 22 | 217 |
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
| 4 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 5 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 6 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,384 |
| 7 | packages/open-bitcoin-node/src/sync/tests.rs | Rust tests | 1,133 |
| 8 | packages/open-bitcoin-cli/src/operator/status/tests.rs | Rust tests | 1,021 |
| 9 | packages/open-bitcoin-mempool/src/pool/tests.rs | Rust tests | 964 |
| 10 | packages/open-bitcoin-cli/tests/operator_binary.rs | Rust tests | 943 |
| 11 | packages/open-bitcoin-consensus/tests/parity_closure.rs | Rust tests | 940 |
| 12 | packages/open-bitcoin-wallet/src/descriptor/tests.rs | Rust tests | 842 |
| 13 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 829 |
| 14 | packages/open-bitcoin-cli/src/operator/service/tests.rs | Rust tests | 786 |
| 15 | packages/open-bitcoin-cli/tests/operator_flows.rs | Rust tests | 767 |
| 16 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 698 |
| 17 | packages/open-bitcoin-node/src/storage/fjall_store/tests.rs | Rust tests | 668 |
| 18 | packages/open-bitcoin-wallet/src/address.rs | Rust production | 626 |
| 19 | packages/open-bitcoin-cli/src/operator/wallet.rs | Rust production | 624 |
| 20 | packages/open-bitcoin-consensus/src/transaction.rs | Rust production | 623 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 34b83d75e87dd9233e49048a6d411a194e5156b0e1682bd3df249fe79df6a99f |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
