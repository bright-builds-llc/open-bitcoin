# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 466 |
| Total lines | 189,045 |
| Code/content lines | 166,770 |
| Comment-only lines | 6,303 |
| Blank lines | 15,972 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,620 | 0 | 85 | 3,705 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 87 | 20,610 | 18,744 | 97 | 39,451 | 90.9% |
| open-bitcoin-codec | 13 | 1,144 | 170 | 28 | 1,345 | 14.9% |
| open-bitcoin-consensus | 30 | 6,346 | 7,519 | 28 | 13,893 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 16 | 2,428 | 2,618 | 30 | 5,076 | 107.8% |
| open-bitcoin-network | 42 | 9,211 | 9,586 | 30 | 18,827 | 104.1% |
| open-bitcoin-node | 65 | 14,336 | 15,618 | 36 | 29,990 | 108.9% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 42 | 8,538 | 6,590 | 57 | 15,185 | 77.2% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 16 | 3,523 | 2,467 | 34 | 6,024 | 70.0% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 255 | 72,357 | 63,133 | 2,183 | 7,041 |
| Rust tests | 76 | 65,130 | 56,858 | 3,417 | 4,855 |
| TypeScript/Bun scripts | 76 | 35,235 | 31,335 | 643 | 3,257 |
| Fixture/data | 6 | 8,233 | 8,228 | 5 | 0 |
| Shell scripts | 13 | 4,014 | 3,588 | 49 | 377 |
| TOML/config | 16 | 3,460 | 3,094 | 0 | 366 |
| Bazel/Starlark | 18 | 411 | 376 | 0 | 35 |
| YAML | 2 | 142 | 114 | 4 | 24 |
| CI/templates | 1 | 27 | 16 | 1 | 10 |
| Other config | 2 | 26 | 22 | 0 | 4 |
| Hooks | 1 | 10 | 6 | 1 | 3 |

## Included TypeScript/Bun Scripts

| File | Lines |
| --- | --- |
| scripts/check-bazel-build-provenance.ts | 187 |
| scripts/check-benchmark-report.ts | 210 |
| scripts/check-parity-breadcrumbs.ts | 427 |
| scripts/check-phase100-relay-activation-boundary.test.ts | 393 |
| scripts/check-phase100-relay-activation-boundary.ts | 566 |
| scripts/check-phase101-transaction-inventory-download-scheduling.test.ts | 500 |
| scripts/check-phase101-transaction-inventory-download-scheduling.ts | 585 |
| scripts/check-phase102-orphan-admission-bridge.test.ts | 582 |
| scripts/check-phase102-orphan-admission-bridge.ts | 682 |
| scripts/check-phase103-mempool-lifecycle.test.ts | 202 |
| scripts/check-phase103-mempool-lifecycle.ts | 376 |
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
| scripts/check-phase92-address-boundaries.test.ts | 569 |
| scripts/check-phase92-address-boundaries.ts | 627 |
| scripts/check-phase93-peer-policy.test.ts | 206 |
| scripts/check-phase93-peer-policy.ts | 556 |
| scripts/check-phase94-dos-resource-governance.test.ts | 471 |
| scripts/check-phase94-dos-resource-governance.ts | 627 |
| scripts/check-phase95-network-participation-release-boundary.test.ts | 608 |
| scripts/check-phase95-network-participation-release-boundary.ts | 671 |
| scripts/check-phase96-peer-policy-runtime-bridge.test.ts | 374 |
| scripts/check-phase96-peer-policy-runtime-bridge.ts | 428 |
| scripts/check-phase97-inbound-metrics.test.ts | 450 |
| scripts/check-phase97-inbound-metrics.ts | 439 |
| scripts/check-phase98-traceability-reconciliation.test.ts | 483 |
| scripts/check-phase98-traceability-reconciliation.ts | 452 |
| scripts/check-phase99-peer-policy-structured-log-emission.test.ts | 330 |
| scripts/check-phase99-peer-policy-structured-log-emission.ts | 252 |
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
| 2 | packages/open-bitcoin-node/src/sync/tests.rs | Rust tests | 7,339 |
| 3 | scripts/run-live-mainnet-smoke.ts | TypeScript/Bun scripts | 3,816 |
| 4 | packages/open-bitcoin-consensus/src/script/tests.rs | Rust tests | 3,258 |
| 5 | packages/Cargo.lock | TOML/config | 3,202 |
| 6 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 2,910 |
| 7 | packages/open-bitcoin-cli/tests/operator_binary.rs | Rust tests | 2,687 |
| 8 | packages/open-bitcoin-cli/src/operator/support/tests.rs | Rust tests | 2,336 |
| 9 | packages/open-bitcoin-cli/src/operator/status/tests.rs | Rust tests | 2,263 |
| 10 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 2,173 |
| 11 | scripts/test-run-live-mainnet-smoke.sh | Shell scripts | 2,081 |
| 12 | packages/open-bitcoin-node/src/network/tests.rs | Rust tests | 1,782 |
| 13 | packages/open-bitcoin-cli/src/operator/service/tests.rs | Rust tests | 1,652 |
| 14 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 15 | packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs | Rust tests | 1,575 |
| 16 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 17 | packages/open-bitcoin-rpc/src/config/tests.rs | Rust tests | 1,521 |
| 18 | packages/open-bitcoin-node/src/status/tests.rs | Rust tests | 1,513 |
| 19 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,384 |
| 20 | packages/open-bitcoin-network/src/inbound/tests.rs | Rust tests | 1,255 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | cf0cfce5edf76c261909ee4c375f5bca7498f5b8b71975135e65cb4367376ab4 |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
