# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 579 |
| Total lines | 249,440 |
| Code/content lines | 219,526 |
| Comment-only lines | 8,947 |
| Blank lines | 20,967 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,628 | 0 | 85 | 3,713 | 0.0% |
| open-bitcoin-chainstate | 8 | 1,023 | 1,818 | 26 | 2,867 | 177.7% |
| open-bitcoin-cli | 93 | 21,734 | 19,990 | 97 | 41,821 | 92.0% |
| open-bitcoin-codec | 15 | 1,811 | 779 | 28 | 2,621 | 43.0% |
| open-bitcoin-consensus | 33 | 6,549 | 7,665 | 28 | 14,242 | 117.0% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 18 | 2,777 | 2,927 | 30 | 5,734 | 105.4% |
| open-bitcoin-network | 54 | 12,075 | 17,963 | 30 | 30,068 | 148.8% |
| open-bitcoin-node | 97 | 20,939 | 23,969 | 40 | 44,948 | 114.5% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 46 | 9,884 | 9,091 | 57 | 19,032 | 92.0% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 16 | 3,523 | 2,467 | 34 | 6,024 | 70.0% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust tests | 98 | 86,669 | 75,493 | 4,608 | 6,568 |
| Rust production | 294 | 85,521 | 74,616 | 2,701 | 8,204 |
| TypeScript/Bun scripts | 128 | 60,566 | 53,657 | 1,570 | 5,339 |
| Fixture/data | 6 | 8,233 | 8,228 | 5 | 0 |
| Shell scripts | 13 | 4,353 | 3,885 | 57 | 411 |
| TOML/config | 16 | 3,464 | 3,098 | 0 | 366 |
| Bazel/Starlark | 18 | 413 | 378 | 0 | 35 |
| YAML | 2 | 157 | 126 | 4 | 27 |
| CI/templates | 1 | 27 | 16 | 1 | 10 |
| Other config | 2 | 27 | 23 | 0 | 4 |
| Hooks | 1 | 10 | 6 | 1 | 3 |

## Included TypeScript/Bun Scripts

| File | Lines |
| --- | --- |
| scripts/check-active-milestone-verification-traceability.test.ts | 773 |
| scripts/check-active-milestone-verification-traceability.ts | 747 |
| scripts/check-bazel-build-provenance.ts | 187 |
| scripts/check-benchmark-report.ts | 210 |
| scripts/check-current-documentation-reconciliation.test.ts | 313 |
| scripts/check-current-documentation-reconciliation.ts | 411 |
| scripts/check-parity-breadcrumbs.ts | 427 |
| scripts/check-phase100-relay-activation-boundary.test.ts | 431 |
| scripts/check-phase100-relay-activation-boundary.ts | 584 |
| scripts/check-phase101-transaction-inventory-download-scheduling.test.ts | 509 |
| scripts/check-phase101-transaction-inventory-download-scheduling.ts | 587 |
| scripts/check-phase102-orphan-admission-bridge.test.ts | 588 |
| scripts/check-phase102-orphan-admission-bridge.ts | 687 |
| scripts/check-phase103-mempool-lifecycle.test.ts | 221 |
| scripts/check-phase103-mempool-lifecycle.ts | 396 |
| scripts/check-phase104-relay-serving-fanout.test.ts | 215 |
| scripts/check-phase104-relay-serving-fanout.ts | 401 |
| scripts/check-phase105-operator-relay-evidence.test.ts | 233 |
| scripts/check-phase105-operator-relay-evidence.ts | 603 |
| scripts/check-phase106-parity-uat-release-boundary.test.ts | 234 |
| scripts/check-phase106-parity-uat-release-boundary.ts | 560 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts | 338 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility.ts | 686 |
| scripts/check-phase108-durable-mempool-relay-state-recovery.test.ts | 140 |
| scripts/check-phase108-durable-mempool-relay-state-recovery.ts | 230 |
| scripts/check-phase110-block-serving-boundary.test.ts | 353 |
| scripts/check-phase110-block-serving-boundary.ts | 567 |
| scripts/check-phase111-full-block-serving-request-path.test.ts | 461 |
| scripts/check-phase111-full-block-serving-request-path.ts | 581 |
| scripts/check-phase116-operator-block-relay-evidence.test.ts | 242 |
| scripts/check-phase116-operator-block-relay-evidence.ts | 467 |
| scripts/check-phase117-parity-uat-release-boundary.test.ts | 786 |
| scripts/check-phase117-parity-uat-release-boundary.ts | 580 |
| scripts/check-phase121-block-relay-metrics-log-runtime.test.ts | 440 |
| scripts/check-phase121-block-relay-metrics-log-runtime.ts | 344 |
| scripts/check-phase122-compact-relay-peer-completion.test.ts | 249 |
| scripts/check-phase122-compact-relay-peer-completion.ts | 402 |
| scripts/check-phase123-runtime-timing-evidence-integrity.test.ts | 473 |
| scripts/check-phase123-runtime-timing-evidence-integrity.ts | 740 |
| scripts/check-phase124-archive-ready.ts | 550 |
| scripts/check-phase124-milestone-closeout-lifecycle.ts | 146 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts | 1,169 |
| scripts/check-phase124-milestone-closeout-reconciliation.test.ts | 1,027 |
| scripts/check-phase124-milestone-closeout-reconciliation.ts | 621 |
| scripts/check-phase124-milestone-gap-closure.test.ts | 1,078 |
| scripts/check-phase124-milestone-gap-closure.ts | 1,528 |
| scripts/check-phase124-post-audit-gap-planning.ts | 704 |
| scripts/check-phase126-compact-relay-residual-hardening.test.ts | 455 |
| scripts/check-phase126-compact-relay-residual-hardening.ts | 421 |
| scripts/check-phase127-authoritative-network-state-unification.test.ts | 560 |
| scripts/check-phase127-authoritative-network-state-unification.ts | 541 |
| scripts/check-phase128-production-compact-announcement-transport.test.ts | 255 |
| scripts/check-phase128-production-compact-announcement-transport.ts | 532 |
| scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts | 229 |
| scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts | 232 |
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
| scripts/check-phase95-network-participation-release-boundary.test.ts | 630 |
| scripts/check-phase95-network-participation-release-boundary.ts | 677 |
| scripts/check-phase96-peer-policy-runtime-bridge.test.ts | 374 |
| scripts/check-phase96-peer-policy-runtime-bridge.ts | 428 |
| scripts/check-phase97-inbound-metrics.test.ts | 450 |
| scripts/check-phase97-inbound-metrics.ts | 439 |
| scripts/check-phase98-traceability-reconciliation.test.ts | 483 |
| scripts/check-phase98-traceability-reconciliation.ts | 453 |
| scripts/check-phase99-peer-policy-structured-log-emission.test.ts | 330 |
| scripts/check-phase99-peer-policy-structured-log-emission.ts | 252 |
| scripts/check-v1.3-release-boundaries.ts | 184 |
| scripts/check-v1.4-release-boundaries.ts | 241 |
| scripts/check-v1.5-release-boundaries.ts | 296 |
| scripts/check-v1.6-release-boundaries.ts | 330 |
| scripts/command-timing-cli.ts | 90 |
| scripts/command-timing-lock.ts | 165 |
| scripts/command-timings.test.ts | 447 |
| scripts/command-timings.ts | 607 |
| scripts/diagnose-rust-test-stall.test.ts | 238 |
| scripts/diagnose-rust-test-stall.ts | 570 |
| scripts/generate-loc-report.ts | 571 |
| scripts/process-liveness.ts | 53 |
| scripts/run-live-mainnet-smoke.ts | 3,816 |
| scripts/rust-source-invariants.ts | 398 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 8,217 |
| 2 | packages/open-bitcoin-node/src/sync/tests.rs | Rust tests | 7,953 |
| 3 | packages/open-bitcoin-network/src/peer/tests.rs | Rust tests | 6,523 |
| 4 | scripts/run-live-mainnet-smoke.ts | TypeScript/Bun scripts | 3,816 |
| 5 | packages/open-bitcoin-consensus/src/script/tests.rs | Rust tests | 3,258 |
| 6 | packages/Cargo.lock | TOML/config | 3,204 |
| 7 | packages/open-bitcoin-cli/src/operator/support/tests.rs | Rust tests | 2,936 |
| 8 | packages/open-bitcoin-node/src/network/tests.rs | Rust tests | 2,829 |
| 9 | packages/open-bitcoin-cli/tests/operator_binary.rs | Rust tests | 2,687 |
| 10 | packages/open-bitcoin-cli/src/operator/status/tests.rs | Rust tests | 2,585 |
| 11 | packages/open-bitcoin-rpc/src/dispatch/tests.rs | Rust tests | 2,532 |
| 12 | scripts/test-run-live-mainnet-smoke.sh | Shell scripts | 2,081 |
| 13 | packages/open-bitcoin-rpc/src/inbound_listener/tests.rs | Rust tests | 1,936 |
| 14 | packages/open-bitcoin-node/src/status/tests.rs | Rust tests | 1,750 |
| 15 | packages/open-bitcoin-rpc/src/config/tests.rs | Rust tests | 1,737 |
| 16 | packages/open-bitcoin-cli/src/operator/service/tests.rs | Rust tests | 1,652 |
| 17 | packages/open-bitcoin-consensus/src/block/tests.rs | Rust tests | 1,594 |
| 18 | packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs | Rust tests | 1,578 |
| 19 | packages/open-bitcoin-chainstate/src/engine/tests.rs | Rust tests | 1,553 |
| 20 | scripts/check-phase124-milestone-gap-closure.ts | TypeScript/Bun scripts | 1,528 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | b599a9e44b416199ce6fcd3ab926f36113f024127cfe90703c37ec1bb1f035ad |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
