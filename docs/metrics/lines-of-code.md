# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 404 |
| Total lines | 159,982 |
| Code/content lines | 141,592 |
| Comment-only lines | 4,988 |
| Blank lines | 13,402 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,616 | 0 | 85 | 3,701 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 86 | 20,242 | 18,139 | 97 | 38,478 | 89.6% |
| open-bitcoin-codec | 13 | 1,144 | 170 | 28 | 1,345 | 14.9% |
| open-bitcoin-consensus | 30 | 6,346 | 7,519 | 28 | 13,893 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 10 | 1,867 | 1,268 | 30 | 3,165 | 67.9% |
| open-bitcoin-network | 24 | 5,308 | 4,948 | 30 | 10,286 | 93.2% |
| open-bitcoin-node | 55 | 12,810 | 12,955 | 36 | 25,801 | 101.1% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 37 | 7,153 | 5,437 | 55 | 12,645 | 76.0% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 16 | 3,523 | 2,467 | 34 | 6,024 | 70.0% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 229 | 64,610 | 56,340 | 1,960 | 6,310 |
| Rust tests | 62 | 54,721 | 48,208 | 2,567 | 3,946 |
| TypeScript/Bun scripts | 54 | 24,397 | 21,661 | 407 | 2,329 |
| Fixture/data | 6 | 8,233 | 8,228 | 5 | 0 |
| Shell scripts | 13 | 3,960 | 3,540 | 43 | 377 |
| TOML/config | 16 | 3,458 | 3,092 | 0 | 366 |
| Bazel/Starlark | 18 | 410 | 375 | 0 | 35 |
| YAML | 2 | 130 | 104 | 4 | 22 |
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
| scripts/check-phase78-progress-guarantees.test.ts | 311 |
| scripts/check-phase78-progress-guarantees.ts | 275 |
| scripts/check-phase79-diagnostics-support-bundle.test.ts | 365 |
| scripts/check-phase79-diagnostics-support-bundle.ts | 364 |
| scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts | 379 |
| scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts | 537 |
| scripts/check-phase82-production-claim-boundary.test.ts | 439 |
| scripts/check-phase82-production-claim-boundary.ts | 620 |
| scripts/check-phase83-support-matrix-issue-evidence.test.ts | 601 |
| scripts/check-phase83-support-matrix-issue-evidence.ts | 693 |
| scripts/check-phase84-upgrade-rollback-policy.test.ts | 461 |
| scripts/check-phase84-upgrade-rollback-policy.ts | 545 |
| scripts/check-phase85-operator-runbooks.test.ts | 451 |
| scripts/check-phase85-operator-runbooks.ts | 643 |
| scripts/check-phase86-service-operation-expectations.test.ts | 455 |
| scripts/check-phase86-service-operation-expectations.ts | 656 |
| scripts/check-phase87-release-readiness.test.ts | 394 |
| scripts/check-phase87-release-readiness.ts | 455 |
| scripts/check-phase88-deterministic-claim-guardrails.test.ts | 325 |
| scripts/check-phase88-deterministic-claim-guardrails.ts | 510 |
| scripts/check-phase90-inbound-listener-admission.test.ts | 465 |
| scripts/check-phase90-inbound-listener-admission.ts | 712 |
| scripts/check-phase91-peer-permissions.test.ts | 431 |
| scripts/check-phase91-peer-permissions.ts | 727 |
| scripts/check-phase92-address-boundaries.test.ts | 568 |
| scripts/check-phase92-address-boundaries.ts | 23 |
| scripts/check-v1.3-release-boundaries.ts | 184 |
| scripts/check-v1.4-release-boundaries.ts | 241 |
| scripts/check-v1.5-release-boundaries.ts | 296 |
| scripts/check-v1.6-release-boundaries.ts | 330 |
| scripts/generate-loc-report.ts | 571 |
| scripts/run-live-mainnet-smoke.ts | 3,816 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 8,217 |
| 2 | packages/open-bitcoin-node/src/sync/tests.rs | Rust tests | 7,195 |
| 3 | scripts/run-live-mainnet-smoke.ts | TypeScript/Bun scripts | 3,816 |
| 4 | packages/open-bitcoin-consensus/src/script/tests.rs | Rust tests | 3,258 |
| 5 | packages/Cargo.lock | TOML/config | 3,201 |
| 6 | packages/open-bitcoin-cli/tests/operator_binary.rs | Rust tests | 2,687 |
| 7 | packages/open-bitcoin-cli/src/operator/status/tests.rs | Rust tests | 2,166 |
| 8 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 2,149 |
| 9 | scripts/test-run-live-mainnet-smoke.sh | Shell scripts | 2,081 |
| 10 | packages/open-bitcoin-cli/src/operator/support/tests.rs | Rust tests | 2,014 |
| 11 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 1,911 |
| 12 | packages/open-bitcoin-cli/src/operator/service/tests.rs | Rust tests | 1,652 |
| 13 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 14 | packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs | Rust tests | 1,575 |
| 15 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 16 | packages/open-bitcoin-node/src/status/tests.rs | Rust tests | 1,453 |
| 17 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,384 |
| 18 | packages/open-bitcoin-rpc/src/config/tests.rs | Rust tests | 1,357 |
| 19 | packages/open-bitcoin-cli/src/operator/soak/tests.rs | Rust tests | 1,254 |
| 20 | packages/open-bitcoin-network/src/inbound/tests.rs | Rust tests | 1,216 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | c2b9e4ccb1af845cd14988966d30a8e9f1b1e637a0a4741cbb63c98661fda4d6 |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
