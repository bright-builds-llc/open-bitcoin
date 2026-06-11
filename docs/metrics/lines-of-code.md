# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 308 |
| Total lines | 109,661 |
| Code/content lines | 97,142 |
| Comment-only lines | 3,267 |
| Blank lines | 9,252 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,600 | 0 | 80 | 3,680 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 61 | 13,993 | 10,763 | 90 | 24,846 | 76.9% |
| open-bitcoin-codec | 13 | 1,144 | 170 | 28 | 1,345 | 14.9% |
| open-bitcoin-consensus | 30 | 6,346 | 7,519 | 28 | 13,893 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 10 | 1,867 | 1,268 | 30 | 3,165 | 67.9% |
| open-bitcoin-network | 12 | 2,581 | 1,654 | 30 | 4,265 | 64.1% |
| open-bitcoin-node | 39 | 9,992 | 7,192 | 36 | 17,220 | 72.0% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 33 | 5,816 | 3,090 | 53 | 8,959 | 53.1% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 16 | 3,523 | 2,467 | 34 | 6,024 | 70.0% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 184 | 51,463 | 44,617 | 1,681 | 5,165 |
| Rust tests | 50 | 35,941 | 31,741 | 1,535 | 2,665 |
| Fixture/data | 6 | 8,217 | 8,212 | 5 | 0 |
| TypeScript/Bun scripts | 15 | 6,752 | 6,094 | 0 | 658 |
| TOML/config | 16 | 3,434 | 3,071 | 0 | 363 |
| Shell scripts | 13 | 3,279 | 2,907 | 40 | 332 |
| Bazel/Starlark | 18 | 408 | 373 | 0 | 35 |
| YAML | 2 | 104 | 83 | 4 | 17 |
| CI/templates | 1 | 27 | 16 | 1 | 10 |
| Other config | 2 | 26 | 22 | 0 | 4 |
| Hooks | 1 | 10 | 6 | 1 | 3 |

## Included TypeScript/Bun Scripts

| File | Lines |
| --- | --- |
| scripts/check-bazel-build-provenance.ts | 187 |
| scripts/check-benchmark-report.ts | 210 |
| scripts/check-parity-breadcrumbs.ts | 427 |
| scripts/check-phase61-resource-recovery-boundaries.ts | 152 |
| scripts/check-phase62-sync-truth-surfaces.ts | 265 |
| scripts/check-phase63-service-lifecycle.ts | 308 |
| scripts/check-phase64-service-restart-resume.ts | 172 |
| scripts/check-phase65-support-review.ts | 140 |
| scripts/check-phase66-compatibility-wrapper.ts | 138 |
| scripts/check-phase68-active-chain-persistence.ts | 178 |
| scripts/check-v1.3-release-boundaries.ts | 184 |
| scripts/check-v1.4-release-boundaries.ts | 241 |
| scripts/check-v1.5-release-boundaries.ts | 296 |
| scripts/generate-loc-report.ts | 571 |
| scripts/run-live-mainnet-smoke.ts | 3,283 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 8,201 |
| 2 | packages/open-bitcoin-node/src/sync/tests.rs | Rust tests | 4,789 |
| 3 | scripts/run-live-mainnet-smoke.ts | TypeScript/Bun scripts | 3,283 |
| 4 | packages/open-bitcoin-consensus/src/script/tests.rs | Rust tests | 3,258 |
| 5 | packages/Cargo.lock | TOML/config | 3,189 |
| 6 | packages/open-bitcoin-cli/tests/operator_binary.rs | Rust tests | 1,955 |
| 7 | scripts/test-run-live-mainnet-smoke.sh | Shell scripts | 1,808 |
| 8 | packages/open-bitcoin-cli/src/operator/status/tests.rs | Rust tests | 1,775 |
| 9 | packages/open-bitcoin-cli/src/operator/service/tests.rs | Rust tests | 1,652 |
| 10 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 11 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 12 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,384 |
| 13 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 1,224 |
| 14 | packages/open-bitcoin-mempool/src/pool/tests.rs | Rust tests | 964 |
| 15 | packages/open-bitcoin-consensus/tests/parity_closure.rs | Rust tests | 940 |
| 16 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 936 |
| 17 | packages/open-bitcoin-wallet/src/descriptor/tests.rs | Rust tests | 842 |
| 18 | packages/open-bitcoin-cli/tests/operator_flows.rs | Rust tests | 767 |
| 19 | packages/open-bitcoin-rpc/src/config/tests.rs | Rust tests | 746 |
| 20 | packages/open-bitcoin-node/src/storage/fjall_store/tests.rs | Rust tests | 668 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 2f71b5c936d48ad09fb5576c607a2f9aaf1dbc0c6e48339c998466d0616a823f |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
