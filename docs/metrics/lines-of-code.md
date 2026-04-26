# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 197 |
| Total lines | 52,054 |
| Code/content lines | 46,170 |
| Comment-only lines | 1,315 |
| Blank lines | 4,569 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 17 | 2,186 | 0 | 73 | 2,259 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 16 | 1,427 | 1,702 | 53 | 3,182 | 119.3% |
| open-bitcoin-codec | 13 | 1,144 | 170 | 28 | 1,345 | 14.9% |
| open-bitcoin-consensus | 30 | 6,346 | 7,519 | 28 | 13,893 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 10 | 1,867 | 1,268 | 30 | 3,165 | 67.9% |
| open-bitcoin-network | 11 | 1,616 | 1,364 | 30 | 3,010 | 84.4% |
| open-bitcoin-node | 12 | 2,214 | 409 | 36 | 2,659 | 18.5% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 21 | 3,173 | 1,632 | 53 | 4,858 | 51.4% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 12 | 2,245 | 1,292 | 34 | 3,571 | 57.6% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 108 | 24,819 | 21,321 | 787 | 2,711 |
| Rust tests | 32 | 17,174 | 15,438 | 496 | 1,240 |
| Fixture/data | 6 | 5,561 | 5,556 | 5 | 0 |
| TOML/config | 16 | 1,796 | 1,598 | 0 | 198 |
| Shell scripts | 9 | 1,179 | 945 | 21 | 213 |
| TypeScript/Bun scripts | 2 | 988 | 849 | 0 | 139 |
| Bazel/Starlark | 18 | 370 | 336 | 0 | 34 |
| YAML | 2 | 104 | 83 | 4 | 17 |
| CI/templates | 1 | 27 | 16 | 1 | 10 |
| Other config | 2 | 26 | 22 | 0 | 4 |
| Hooks | 1 | 10 | 6 | 1 | 3 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 5,545 |
| 2 | packages/open-bitcoin-consensus/src/script/tests.rs | Rust tests | 3,258 |
| 3 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 4 | packages/Cargo.lock | TOML/config | 1,558 |
| 5 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 6 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,051 |
| 7 | packages/open-bitcoin-mempool/src/pool/tests.rs | Rust tests | 964 |
| 8 | packages/open-bitcoin-consensus/tests/parity_closure.rs | Rust tests | 940 |
| 9 | packages/open-bitcoin-cli/tests/operator_flows.rs | Rust tests | 770 |
| 10 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 646 |
| 11 | packages/open-bitcoin-wallet/src/address.rs | Rust production | 626 |
| 12 | packages/open-bitcoin-consensus/src/transaction.rs | Rust production | 623 |
| 13 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 621 |
| 14 | packages/open-bitcoin-mempool/src/pool.rs | Rust production | 615 |
| 15 | packages/open-bitcoin-rpc/src/config/loader.rs | Rust production | 604 |
| 16 | packages/open-bitcoin-mempool/src/policy.rs | Rust production | 582 |
| 17 | scripts/generate-loc-report.ts | TypeScript/Bun scripts | 561 |
| 18 | packages/open-bitcoin-consensus/src/signature/tests.rs | Rust tests | 560 |
| 19 | packages/open-bitcoin-rpc/src/method.rs | Rust production | 555 |
| 20 | packages/open-bitcoin-network/src/message.rs | Rust production | 549 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 164043c9edc3ae10e2c31292c81d6b32b78796723d48a569c69eef342069f3bb |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
