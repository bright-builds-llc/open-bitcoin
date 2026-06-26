# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 409 |
| Total lines | 163,113 |
| Code/content lines | 144,363 |
| Comment-only lines | 5,066 |
| Blank lines | 13,684 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,616 | 0 | 85 | 3,701 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 86 | 20,364 | 18,326 | 97 | 38,787 | 90.0% |
| open-bitcoin-codec | 13 | 1,144 | 170 | 28 | 1,345 | 14.9% |
| open-bitcoin-consensus | 30 | 6,346 | 7,519 | 28 | 13,893 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 10 | 1,867 | 1,268 | 30 | 3,165 | 67.9% |
| open-bitcoin-network | 27 | 5,874 | 5,335 | 30 | 11,239 | 90.8% |
| open-bitcoin-node | 55 | 13,088 | 13,125 | 36 | 26,249 | 100.3% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 37 | 7,180 | 5,456 | 55 | 12,691 | 76.0% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 16 | 3,523 | 2,467 | 34 | 6,024 | 70.0% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust production | 231 | 65,603 | 57,229 | 1,978 | 6,396 |
| Rust tests | 63 | 55,484 | 48,852 | 2,627 | 4,005 |
| TypeScript/Bun scripts | 56 | 25,764 | 22,891 | 407 | 2,466 |
| Fixture/data | 6 | 8,233 | 8,228 | 5 | 0 |
| Shell scripts | 13 | 3,968 | 3,548 | 43 | 377 |
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
| scripts/check-phase92-address-boundaries.test.ts | 569 |
| scripts/check-phase92-address-boundaries.ts | 627 |
| scripts/check-phase93-peer-policy.test.ts | 206 |
| scripts/check-phase93-peer-policy.ts | 556 |
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
| 7 | packages/open-bitcoin-cli/src/operator/status/tests.rs | Rust tests | 2,212 |
| 8 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 2,168 |
| 9 | packages/open-bitcoin-cli/src/operator/support/tests.rs | Rust tests | 2,115 |
| 10 | scripts/test-run-live-mainnet-smoke.sh | Shell scripts | 2,081 |
| 11 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 1,965 |
| 12 | packages/open-bitcoin-cli/src/operator/service/tests.rs | Rust tests | 1,652 |
| 13 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 14 | packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs | Rust tests | 1,575 |
| 15 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 16 | packages/open-bitcoin-node/src/status/tests.rs | Rust tests | 1,464 |
| 17 | packages/open-bitcoin-wallet/src/wallet/tests.rs | Rust tests | 1,384 |
| 18 | packages/open-bitcoin-rpc/src/config/tests.rs | Rust tests | 1,357 |
| 19 | packages/open-bitcoin-node/src/network/tests.rs | Rust tests | 1,297 |
| 20 | packages/open-bitcoin-cli/src/operator/soak/tests.rs | Rust tests | 1,254 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | c1724c1a77743690c622d3475acc3471af76a7eb2ce38859d1a5ad7835af1c32 |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
