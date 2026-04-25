# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 189 |
| Total lines | 49,304 |
| Code/content lines | 44,593 |
| Comment-only lines | 522 |
| Blank lines | 4,189 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 17 | 2,125 | 0 | 73 | 2,198 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,004 | 1,804 | 26 | 2,834 | 179.7% |
| open-bitcoin-cli | 14 | 1,269 | 1,615 | 53 | 2,937 | 127.3% |
| open-bitcoin-codec | 13 | 1,114 | 165 | 28 | 1,310 | 14.8% |
| open-bitcoin-consensus | 30 | 6,212 | 7,473 | 28 | 13,713 | 120.3% |
| open-bitcoin-core | 3 | 37 | 0 | 36 | 73 | 0.0% |
| open-bitcoin-mempool | 10 | 1,834 | 1,254 | 30 | 3,118 | 68.4% |
| open-bitcoin-network | 11 | 1,588 | 1,332 | 30 | 2,950 | 83.9% |
| open-bitcoin-node | 8 | 1,262 | 403 | 30 | 1,695 | 31.9% |
| open-bitcoin-primitives | 9 | 856 | 0 | 20 | 876 | 0.0% |
| open-bitcoin-rpc | 20 | 2,908 | 1,451 | 51 | 4,410 | 49.9% |
| open-bitcoin-test-harness | 7 | 638 | 0 | 28 | 666 | 0.0% |
| open-bitcoin-wallet | 12 | 2,197 | 1,278 | 34 | 3,509 | 58.2% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 102 | 23,044 | 20,342 | 224 | 2,478 |
| Rust tests | 31 | 16,775 | 15,324 | 269 | 1,182 |
| Fixture/data | 6 | 5,545 | 5,540 | 5 | 0 |
| TOML/config | 16 | 1,779 | 1,583 | 0 | 196 |
| Shell scripts | 9 | 1,131 | 907 | 18 | 206 |
| Node scripts | 1 | 504 | 443 | 0 | 61 |
| Bazel/Starlark | 18 | 367 | 333 | 0 | 34 |
| YAML | 2 | 96 | 77 | 4 | 15 |
| CI/templates | 1 | 27 | 16 | 1 | 10 |
| Other config | 2 | 26 | 22 | 0 | 4 |
| Hooks | 1 | 10 | 6 | 1 | 3 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 5,529 |
| 2 | packages/open-bitcoin-consensus/src/script/tests.rs | Rust tests | 3,251 |
| 3 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,588 |
| 4 | packages/Cargo.lock | TOML/config | 1,546 |
| 5 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,546 |
| 6 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,043 |
| 7 | packages/open-bitcoin-mempool/src/pool/tests.rs | Rust tests | 957 |
| 8 | packages/open-bitcoin-consensus/tests/parity_closure.rs | Rust tests | 932 |
| 9 | packages/open-bitcoin-cli/tests/operator_flows.rs | Rust tests | 765 |
| 10 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 637 |
| 11 | packages/open-bitcoin-wallet/src/address.rs | Rust production | 620 |
| 12 | packages/open-bitcoin-consensus/src/transaction.rs | Rust production | 615 |
| 13 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 610 |
| 14 | packages/open-bitcoin-mempool/src/pool.rs | Rust production | 608 |
| 15 | packages/open-bitcoin-rpc/src/config/loader.rs | Rust production | 598 |
| 16 | packages/open-bitcoin-mempool/src/policy.rs | Rust production | 575 |
| 17 | packages/open-bitcoin-consensus/src/signature/tests.rs | Rust tests | 554 |
| 18 | packages/open-bitcoin-network/src/message.rs | Rust production | 544 |
| 19 | packages/open-bitcoin-rpc/src/method.rs | Rust production | 544 |
| 20 | packages/open-bitcoin-consensus/src/script/legacy.rs | Rust production | 523 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 5ac11c161bf6a1ffaf351de1347f3fc93dbabc03070969b9c4e4ba8aa4763298 |
| Generator command | node scripts/generate-loc-report.mjs --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
