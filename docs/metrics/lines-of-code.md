# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 347 |
| Total lines | 128,793 |
| Code/content lines | 114,157 |
| Comment-only lines | 3,885 |
| Blank lines | 10,751 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,603 | 0 | 80 | 3,683 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 78 | 18,168 | 15,685 | 92 | 33,945 | 86.3% |
| open-bitcoin-codec | 13 | 1,144 | 170 | 28 | 1,345 | 14.9% |
| open-bitcoin-consensus | 30 | 6,346 | 7,519 | 28 | 13,893 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 10 | 1,867 | 1,268 | 30 | 3,165 | 67.9% |
| open-bitcoin-network | 12 | 2,581 | 1,654 | 30 | 4,265 | 64.1% |
| open-bitcoin-node | 48 | 11,312 | 10,485 | 36 | 21,833 | 92.7% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 33 | 5,816 | 3,260 | 53 | 9,129 | 56.1% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 16 | 3,523 | 2,467 | 34 | 6,024 | 70.0% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 203 | 56,961 | 49,590 | 1,772 | 5,599 |
| Rust tests | 57 | 44,326 | 39,102 | 1,969 | 3,255 |
| TypeScript/Bun scripts | 28 | 11,768 | 10,555 | 93 | 1,120 |
| Fixture/data | 6 | 8,233 | 8,228 | 5 | 0 |
| Shell scripts | 13 | 3,483 | 3,099 | 40 | 344 |
| TOML/config | 16 | 3,446 | 3,082 | 0 | 364 |
| Bazel/Starlark | 18 | 409 | 374 | 0 | 35 |
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
| scripts/check-phase64-service-restart-resume.ts | 190 |
| scripts/check-phase65-support-review.ts | 140 |
| scripts/check-phase66-compatibility-wrapper.ts | 138 |
| scripts/check-phase68-active-chain-persistence.ts | 178 |
| scripts/check-phase69-tip-stay-current.ts | 223 |
| scripts/check-phase70-reorg-recovery.ts | 161 |
| scripts/check-phase71-resource-restart.ts | 172 |
| scripts/check-phase72-observability-evidence.ts | 565 |
| scripts/check-phase73-uat-verification.test.ts | 447 |
| scripts/check-phase73-uat-verification.ts | 632 |
| scripts/check-phase75-soak-runner.test.ts | 438 |
| scripts/check-phase75-soak-runner.ts | 370 |
| scripts/check-phase76-resource-bounds.test.ts | 343 |
| scripts/check-phase76-resource-bounds.ts | 328 |
| scripts/check-phase77-corruption-lock-recovery.test.ts | 361 |
| scripts/check-phase77-corruption-lock-recovery.ts | 319 |
| scripts/check-v1.3-release-boundaries.ts | 184 |
| scripts/check-v1.4-release-boundaries.ts | 241 |
| scripts/check-v1.5-release-boundaries.ts | 296 |
| scripts/check-v1.6-release-boundaries.ts | 330 |
| scripts/generate-loc-report.ts | 571 |
| scripts/run-live-mainnet-smoke.ts | 3,592 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 8,217 |
| 2 | packages/open-bitcoin-node/src/sync/tests.rs | Rust tests | 6,541 |
| 3 | scripts/run-live-mainnet-smoke.ts | TypeScript/Bun scripts | 3,592 |
| 4 | packages/open-bitcoin-consensus/src/script/tests.rs | Rust tests | 3,258 |
| 5 | packages/Cargo.lock | TOML/config | 3,200 |
| 6 | packages/open-bitcoin-cli/tests/operator_binary.rs | Rust tests | 2,665 |
| 7 | scripts/test-run-live-mainnet-smoke.sh | Shell scripts | 1,932 |
| 8 | packages/open-bitcoin-cli/src/operator/status/tests.rs | Rust tests | 1,803 |
| 9 | packages/open-bitcoin-cli/src/operator/service/tests.rs | Rust tests | 1,652 |
| 10 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 11 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 12 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 1,384 |
| 13 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,384 |
| 14 | packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs | Rust tests | 1,345 |
| 15 | packages/open-bitcoin-cli/src/operator/soak/tests.rs | Rust tests | 1,061 |
| 16 | packages/open-bitcoin-node/src/status/tests.rs | Rust tests | 1,023 |
| 17 | packages/open-bitcoin-node/src/storage/fjall_store/tests.rs | Rust tests | 974 |
| 18 | packages/open-bitcoin-mempool/src/pool/tests.rs | Rust tests | 964 |
| 19 | packages/open-bitcoin-consensus/tests/parity_closure.rs | Rust tests | 940 |
| 20 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 936 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | de15a28adb09299d5fc3ab17fc0c6fc0a5f54951fc0806d2ebd96708a8041fbf |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
