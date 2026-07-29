# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 1,042 |
| Total lines | 295,294 |
| Code/content lines | 257,612 |
| Comment-only lines | 13,064 |
| Blank lines | 24,618 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,779 | 0 | 85 | 3,864 | 0.0% |
| open-bitcoin-chainstate | 12 | 1,023 | 1,855 | 26 | 2,904 | 181.3% |
| open-bitcoin-cli | 135 | 21,734 | 20,300 | 97 | 42,131 | 93.4% |
| open-bitcoin-codec | 15 | 1,811 | 779 | 28 | 2,621 | 43.0% |
| open-bitcoin-consensus | 47 | 6,592 | 7,851 | 28 | 14,471 | 119.1% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 82 | 10,720 | 15,478 | 30 | 26,228 | 144.4% |
| open-bitcoin-network | 122 | 14,042 | 20,695 | 30 | 34,767 | 147.4% |
| open-bitcoin-node | 209 | 25,285 | 31,757 | 40 | 57,082 | 125.6% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 79 | 10,064 | 10,154 | 57 | 20,275 | 100.9% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 21 | 3,529 | 2,509 | 34 | 6,072 | 71.1% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust tests | 398 | 111,378 | 95,013 | 7,738 | 8,627 |
| Rust production | 336 | 100,157 | 87,132 | 3,619 | 9,406 |
| TypeScript/Bun scripts | 239 | 66,901 | 59,552 | 1,627 | 5,722 |
| Fixture/data | 6 | 8,233 | 8,228 | 5 | 0 |
| Shell scripts | 22 | 4,499 | 4,020 | 66 | 413 |
| TOML/config | 16 | 3,464 | 3,098 | 0 | 366 |
| Bazel/Starlark | 18 | 413 | 378 | 0 | 35 |
| YAML | 3 | 184 | 145 | 7 | 32 |
| CI/templates | 1 | 28 | 17 | 1 | 10 |
| Other config | 2 | 27 | 23 | 0 | 4 |
| Hooks | 1 | 10 | 6 | 1 | 3 |

## Included TypeScript/Bun Scripts

| File | Lines |
| --- | --- |
| scripts/bright-builds-check.ts | 488 |
| scripts/check-active-milestone-verification-traceability.test.ts | 3 |
| scripts/check-active-milestone-verification-traceability.ts | 16 |
| scripts/check-active-milestone-verification-traceability/checks.ts | 137 |
| scripts/check-active-milestone-verification-traceability/constants.ts | 53 |
| scripts/check-active-milestone-verification-traceability/filesystem.ts | 124 |
| scripts/check-active-milestone-verification-traceability/lifecycle-scenarios.ts | 162 |
| scripts/check-active-milestone-verification-traceability/lifecycle.ts | 146 |
| scripts/check-active-milestone-verification-traceability/ownership.ts | 252 |
| scripts/check-active-milestone-verification-traceability/parsing.ts | 314 |
| scripts/check-active-milestone-verification-traceability/success-and-coverage.ts | 222 |
| scripts/check-active-milestone-verification-traceability/test-fixtures.ts | 176 |
| scripts/check-bazel-build-provenance.ts | 187 |
| scripts/check-benchmark-report.ts | 210 |
| scripts/check-current-documentation-reconciliation.test.ts | 318 |
| scripts/check-current-documentation-reconciliation.ts | 439 |
| scripts/check-parity-breadcrumbs.ts | 427 |
| scripts/check-phase100-relay-activation-boundary.test.ts | 431 |
| scripts/check-phase100-relay-activation-boundary.ts | 584 |
| scripts/check-phase101-transaction-inventory-download-scheduling.test.ts | 509 |
| scripts/check-phase101-transaction-inventory-download-scheduling.ts | 546 |
| scripts/check-phase102-orphan-admission-bridge.test.ts | 613 |
| scripts/check-phase102-orphan-admission-bridge.ts | 15 |
| scripts/check-phase102-orphan-admission-bridge/bridge.ts | 135 |
| scripts/check-phase102-orphan-admission-bridge/checks.ts | 30 |
| scripts/check-phase102-orphan-admission-bridge/constants.ts | 249 |
| scripts/check-phase102-orphan-admission-bridge/filesystem.ts | 40 |
| scripts/check-phase102-orphan-admission-bridge/helpers.ts | 72 |
| scripts/check-phase102-orphan-admission-bridge/parity.ts | 135 |
| scripts/check-phase102-orphan-admission-bridge/verifier.ts | 88 |
| scripts/check-phase103-mempool-lifecycle.test.ts | 234 |
| scripts/check-phase103-mempool-lifecycle.ts | 396 |
| scripts/check-phase104-relay-serving-fanout.test.ts | 215 |
| scripts/check-phase104-relay-serving-fanout.ts | 401 |
| scripts/check-phase105-operator-relay-evidence.test.ts | 234 |
| scripts/check-phase105-operator-relay-evidence.ts | 544 |
| scripts/check-phase106-parity-uat-release-boundary.test.ts | 234 |
| scripts/check-phase106-parity-uat-release-boundary.ts | 560 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts | 338 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility.ts | 17 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/checks.ts | 145 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/claims.ts | 84 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/constants.ts | 235 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/filesystem.ts | 35 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/helpers.ts | 114 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/parity.ts | 107 |
| scripts/check-phase108-durable-mempool-relay-state-recovery.test.ts | 140 |
| scripts/check-phase108-durable-mempool-relay-state-recovery.ts | 230 |
| scripts/check-phase110-block-serving-boundary.test.ts | 353 |
| scripts/check-phase110-block-serving-boundary.ts | 567 |
| scripts/check-phase111-full-block-serving-request-path.test.ts | 461 |
| scripts/check-phase111-full-block-serving-request-path.ts | 550 |
| scripts/check-phase116-operator-block-relay-evidence.test.ts | 239 |
| scripts/check-phase116-operator-block-relay-evidence.ts | 468 |
| scripts/check-phase117-parity-uat-release-boundary.test.ts | 4 |
| scripts/check-phase117-parity-uat-release-boundary.ts | 580 |
| scripts/check-phase117-parity-uat-release-boundary/claims.ts | 125 |
| scripts/check-phase117-parity-uat-release-boundary/lifecycle-routing.ts | 60 |
| scripts/check-phase117-parity-uat-release-boundary/success-and-parity.ts | 199 |
| scripts/check-phase117-parity-uat-release-boundary/test-fixtures.ts | 289 |
| scripts/check-phase117-parity-uat-release-boundary/verifier.ts | 141 |
| scripts/check-phase121-block-relay-metrics-log-runtime.test.ts | 440 |
| scripts/check-phase121-block-relay-metrics-log-runtime.ts | 345 |
| scripts/check-phase122-compact-relay-peer-completion.test.ts | 263 |
| scripts/check-phase122-compact-relay-peer-completion.ts | 432 |
| scripts/check-phase123-runtime-timing-evidence-integrity.test.ts | 475 |
| scripts/check-phase123-runtime-timing-evidence-integrity.ts | 14 |
| scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts | 264 |
| scripts/check-phase123-runtime-timing-evidence-integrity/constants.ts | 69 |
| scripts/check-phase123-runtime-timing-evidence-integrity/evidence.ts | 212 |
| scripts/check-phase123-runtime-timing-evidence-integrity/filesystem.ts | 49 |
| scripts/check-phase123-runtime-timing-evidence-integrity/helpers.ts | 61 |
| scripts/check-phase123-runtime-timing-evidence-integrity/parity.ts | 131 |
| scripts/check-phase124-archive-ready.ts | 550 |
| scripts/check-phase124-milestone-closeout-lifecycle.ts | 146 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts | 35 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/base.ts | 353 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/mutations.ts | 29 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/phase125.ts | 329 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/phase126.ts | 158 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/phase127.ts | 158 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/phase128.ts | 83 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/phase129.ts | 136 |
| scripts/check-phase124-milestone-closeout-reconciliation.test.ts | 4 |
| scripts/check-phase124-milestone-closeout-reconciliation.test/scenarios-1.ts | 355 |
| scripts/check-phase124-milestone-closeout-reconciliation.test/scenarios-2.ts | 380 |
| scripts/check-phase124-milestone-closeout-reconciliation.test/scenarios-3.ts | 255 |
| scripts/check-phase124-milestone-closeout-reconciliation.test/setup.ts | 49 |
| scripts/check-phase124-milestone-closeout-reconciliation.ts | 621 |
| scripts/check-phase124-milestone-gap-closure.test.ts | 3 |
| scripts/check-phase124-milestone-gap-closure.test/scenarios-1.ts | 373 |
| scripts/check-phase124-milestone-gap-closure.test/scenarios-2.ts | 373 |
| scripts/check-phase124-milestone-gap-closure.test/scenarios-3.ts | 69 |
| scripts/check-phase124-milestone-gap-closure.test/setup.ts | 362 |
| scripts/check-phase124-milestone-gap-closure.ts | 5 |
| scripts/check-phase124-milestone-gap-closure/constants.ts | 104 |
| scripts/check-phase124-milestone-gap-closure/filesystem.ts | 348 |
| scripts/check-phase124-milestone-gap-closure/lifecycle.ts | 322 |
| scripts/check-phase124-milestone-gap-closure/parsing.ts | 106 |
| scripts/check-phase124-milestone-gap-closure/projection.ts | 323 |
| scripts/check-phase124-milestone-gap-closure/routing.ts | 360 |
| scripts/check-phase124-post-audit-gap-planning.ts | 2 |
| scripts/check-phase124-post-audit-gap-planning/constants.ts | 57 |
| scripts/check-phase124-post-audit-gap-planning/projection.ts | 358 |
| scripts/check-phase124-post-audit-gap-planning/routing.ts | 310 |
| scripts/check-phase126-compact-relay-residual-hardening.test.ts | 478 |
| scripts/check-phase126-compact-relay-residual-hardening.ts | 432 |
| scripts/check-phase127-authoritative-network-state-unification.test.ts | 550 |
| scripts/check-phase127-authoritative-network-state-unification.ts | 542 |
| scripts/check-phase128-production-compact-announcement-transport.test.ts | 264 |
| scripts/check-phase128-production-compact-announcement-transport.ts | 568 |
| scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts | 242 |
| scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts | 233 |
| scripts/check-phase130-resource-time-fee-primitives.test.ts | 286 |
| scripts/check-phase130-resource-time-fee-primitives.ts | 583 |
| scripts/check-phase131-rolling-fee-expiry-pressure.test.ts | 184 |
| scripts/check-phase131-rolling-fee-expiry-pressure.ts | 352 |
| scripts/check-phase132-typed-package-staged-admission.test.ts | 399 |
| scripts/check-phase132-typed-package-staged-admission.ts | 16 |
| scripts/check-phase132-typed-package-staged-admission/checks.ts | 305 |
| scripts/check-phase132-typed-package-staged-admission/claims.ts | 77 |
| scripts/check-phase132-typed-package-staged-admission/constants.ts | 87 |
| scripts/check-phase132-typed-package-staged-admission/filesystem.ts | 25 |
| scripts/check-phase132-typed-package-staged-admission/helpers.ts | 92 |
| scripts/check-phase132-typed-package-staged-admission/parity.ts | 179 |
| scripts/check-phase132-typed-package-staged-admission/policy.ts | 159 |
| scripts/check-phase133-package-aware-download-orphan-bridge.test.ts | 377 |
| scripts/check-phase133-package-aware-download-orphan-bridge.ts | 556 |
| scripts/check-phase134-apply-boundaries.ts | 206 |
| scripts/check-phase134-authoritative-lifecycle.test.ts | 571 |
| scripts/check-phase134-authoritative-lifecycle.ts | 497 |
| scripts/check-phase61-resource-recovery-boundaries.ts | 152 |
| scripts/check-phase62-sync-truth-surfaces.ts | 265 |
| scripts/check-phase63-service-lifecycle.ts | 308 |
| scripts/check-phase64-service-restart-resume.ts | 190 |
| scripts/check-phase65-support-review.ts | 140 |
| scripts/check-phase66-compatibility-wrapper.ts | 138 |
| scripts/check-phase68-active-chain-persistence.ts | 179 |
| scripts/check-phase69-tip-stay-current.ts | 224 |
| scripts/check-phase70-reorg-recovery.ts | 158 |
| scripts/check-phase71-resource-restart.ts | 169 |
| scripts/check-phase72-observability-evidence.ts | 546 |
| scripts/check-phase73-uat-verification.test.ts | 447 |
| scripts/check-phase73-uat-verification.ts | 3 |
| scripts/check-phase73-uat-verification/checks.ts | 368 |
| scripts/check-phase73-uat-verification/constants.ts | 255 |
| scripts/check-phase73-uat-verification/parity.ts | 30 |
| scripts/check-phase75-soak-runner.test.ts | 438 |
| scripts/check-phase75-soak-runner.ts | 365 |
| scripts/check-phase76-resource-bounds.test.ts | 343 |
| scripts/check-phase76-resource-bounds.ts | 323 |
| scripts/check-phase77-corruption-lock-recovery.test.ts | 361 |
| scripts/check-phase77-corruption-lock-recovery.ts | 314 |
| scripts/check-phase78-progress-guarantees.test.ts | 311 |
| scripts/check-phase78-progress-guarantees.ts | 270 |
| scripts/check-phase79-diagnostics-support-bundle.test.ts | 365 |
| scripts/check-phase79-diagnostics-support-bundle.ts | 368 |
| scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts | 379 |
| scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts | 537 |
| scripts/check-phase82-production-claim-boundary.test.ts | 439 |
| scripts/check-phase82-production-claim-boundary.ts | 620 |
| scripts/check-phase83-support-matrix-issue-evidence.test.ts | 601 |
| scripts/check-phase83-support-matrix-issue-evidence.ts | 14 |
| scripts/check-phase83-support-matrix-issue-evidence/checks.ts | 342 |
| scripts/check-phase83-support-matrix-issue-evidence/constants.ts | 192 |
| scripts/check-phase83-support-matrix-issue-evidence/parity.ts | 153 |
| scripts/check-phase84-upgrade-rollback-policy.test.ts | 461 |
| scripts/check-phase84-upgrade-rollback-policy.ts | 545 |
| scripts/check-phase85-operator-runbooks.test.ts | 451 |
| scripts/check-phase85-operator-runbooks.ts | 14 |
| scripts/check-phase85-operator-runbooks/checks.ts | 322 |
| scripts/check-phase85-operator-runbooks/constants.ts | 237 |
| scripts/check-phase85-operator-runbooks/parity.ts | 78 |
| scripts/check-phase86-service-operation-expectations.test.ts | 455 |
| scripts/check-phase86-service-operation-expectations.ts | 14 |
| scripts/check-phase86-service-operation-expectations/checks.ts | 332 |
| scripts/check-phase86-service-operation-expectations/constants.ts | 227 |
| scripts/check-phase86-service-operation-expectations/parity.ts | 91 |
| scripts/check-phase87-release-readiness.test.ts | 394 |
| scripts/check-phase87-release-readiness.ts | 455 |
| scripts/check-phase88-deterministic-claim-guardrails.test.ts | 325 |
| scripts/check-phase88-deterministic-claim-guardrails.ts | 510 |
| scripts/check-phase90-inbound-listener-admission.test.ts | 465 |
| scripts/check-phase90-inbound-listener-admission.ts | 14 |
| scripts/check-phase90-inbound-listener-admission/checks.ts | 348 |
| scripts/check-phase90-inbound-listener-admission/constants.ts | 268 |
| scripts/check-phase90-inbound-listener-admission/parity.ts | 90 |
| scripts/check-phase91-peer-permissions.test.ts | 431 |
| scripts/check-phase91-peer-permissions.ts | 14 |
| scripts/check-phase91-peer-permissions/checks.ts | 346 |
| scripts/check-phase91-peer-permissions/constants.ts | 253 |
| scripts/check-phase91-peer-permissions/parity.ts | 122 |
| scripts/check-phase92-address-boundaries.test.ts | 569 |
| scripts/check-phase92-address-boundaries.ts | 627 |
| scripts/check-phase93-peer-policy.test.ts | 206 |
| scripts/check-phase93-peer-policy.ts | 556 |
| scripts/check-phase94-dos-resource-governance.test.ts | 471 |
| scripts/check-phase94-dos-resource-governance.ts | 627 |
| scripts/check-phase95-network-participation-release-boundary.test.ts | 1 |
| scripts/check-phase95-network-participation-release-boundary.test/scenarios-1.ts | 255 |
| scripts/check-phase95-network-participation-release-boundary.test/setup.ts | 401 |
| scripts/check-phase95-network-participation-release-boundary.ts | 14 |
| scripts/check-phase95-network-participation-release-boundary/checks.ts | 358 |
| scripts/check-phase95-network-participation-release-boundary/constants.ts | 192 |
| scripts/check-phase95-network-participation-release-boundary/parity.ts | 129 |
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
| scripts/run-live-mainnet-smoke.ts | 9 |
| scripts/run-live-mainnet-smoke/cli.ts | 285 |
| scripts/run-live-mainnet-smoke/command.ts | 167 |
| scripts/run-live-mainnet-smoke/diagnosis.ts | 481 |
| scripts/run-live-mainnet-smoke/options.ts | 197 |
| scripts/run-live-mainnet-smoke/preflight.ts | 454 |
| scripts/run-live-mainnet-smoke/report.ts | 521 |
| scripts/run-live-mainnet-smoke/session.ts | 495 |
| scripts/run-live-mainnet-smoke/status.ts | 549 |
| scripts/run-live-mainnet-smoke/types.ts | 542 |
| scripts/rust-source-invariants.ts | 398 |
| scripts/source-corpus.test.ts | 33 |
| scripts/source-corpus.ts | 80 |
| scripts/test-run-live-mainnet-smoke/assert-report.ts | 163 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 8,217 |
| 2 | packages/Cargo.lock | TOML/config | 3,204 |
| 3 | scripts/verify.sh | Shell scripts | 628 |
| 4 | packages/open-bitcoin-cli/src/operator/runtime.rs | Rust production | 627 |
| 5 | packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs | Rust production | 627 |
| 6 | packages/open-bitcoin-cli/src/operator/support/render.rs | Rust production | 627 |
| 7 | packages/open-bitcoin-mempool/src/pool/prospective.rs | Rust production | 627 |
| 8 | packages/open-bitcoin-node/src/network/lifecycle_effects.rs | Rust production | 627 |
| 9 | scripts/check-phase92-address-boundaries.ts | TypeScript/Bun scripts | 627 |
| 10 | scripts/check-phase94-dos-resource-governance.ts | TypeScript/Bun scripts | 627 |
| 11 | packages/open-bitcoin-cli/src/operator/status/render.rs | Rust production | 626 |
| 12 | packages/open-bitcoin-node/src/network/admission_bridge.rs | Rust production | 626 |
| 13 | packages/open-bitcoin-wallet/src/address.rs | Rust production | 626 |
| 14 | packages/open-bitcoin-rpc/src/config/loader.rs | Rust production | 625 |
| 15 | packages/open-bitcoin-cli/src/operator/wallet.rs | Rust production | 624 |
| 16 | packages/open-bitcoin-node/src/network/block_serving.rs | Rust production | 624 |
| 17 | packages/open-bitcoin-cli/src/operator/support/forensics.rs | Rust production | 623 |
| 18 | packages/open-bitcoin-consensus/src/transaction.rs | Rust production | 623 |
| 19 | packages/open-bitcoin-network/src/peer/compact_relay/tests.rs | Rust tests | 623 |
| 20 | packages/open-bitcoin-cli/src/operator/runtime/support.rs | Rust production | 622 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 368df87e5765894b84e23d870f813cd965e844737e7af3b3a2bac76df4c712f7 |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
