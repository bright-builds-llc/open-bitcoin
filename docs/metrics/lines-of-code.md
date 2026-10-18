# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 1,620 |
| Total lines | 482,559 |
| Code/content lines | 422,668 |
| Comment-only lines | 24,871 |
| Blank lines | 35,020 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,895 | 0 | 85 | 3,980 | 0.0% |
| open-bitcoin-chainstate | 65 | 5,798 | 11,160 | 26 | 16,984 | 192.5% |
| open-bitcoin-cli | 171 | 23,533 | 26,592 | 97 | 50,270 | 113.0% |
| open-bitcoin-codec | 19 | 2,074 | 1,072 | 28 | 3,177 | 51.7% |
| open-bitcoin-consensus | 58 | 8,136 | 9,641 | 28 | 17,805 | 118.5% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 82 | 10,799 | 15,651 | 30 | 26,480 | 144.9% |
| open-bitcoin-network | 144 | 16,718 | 24,848 | 30 | 41,596 | 148.6% |
| open-bitcoin-node | 481 | 52,839 | 98,819 | 42 | 152,479 | 187.0% |
| open-bitcoin-primitives | 9 | 917 | 0 | 20 | 937 | 0.0% |
| open-bitcoin-rpc | 156 | 16,012 | 31,851 | 59 | 47,922 | 198.9% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 21 | 3,529 | 2,705 | 34 | 6,268 | 76.7% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust tests | 690 | 222,339 | 192,796 | 15,245 | 14,298 |
| Rust production | 516 | 144,951 | 126,142 | 6,309 | 12,500 |
| TypeScript/Bun scripts | 340 | 97,223 | 86,673 | 3,234 | 7,316 |
| Fixture/data | 6 | 8,236 | 8,231 | 5 | 0 |
| Shell scripts | 23 | 4,596 | 4,117 | 69 | 410 |
| TOML/config | 16 | 3,467 | 3,101 | 0 | 366 |
| Other config | 6 | 1,110 | 1,060 | 0 | 50 |
| Bazel/Starlark | 18 | 415 | 380 | 0 | 35 |
| YAML | 3 | 184 | 145 | 7 | 32 |
| CI/templates | 1 | 28 | 17 | 1 | 10 |
| Hooks | 1 | 10 | 6 | 1 | 3 |

## Included TypeScript/Bun Scripts

| File | Lines |
| --- | --- |
| scripts/bright-builds-check.ts | 571 |
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
| scripts/check-current-documentation-reconciliation.test.ts | 375 |
| scripts/check-current-documentation-reconciliation.ts | 480 |
| scripts/check-parity-breadcrumbs.test.ts | 65 |
| scripts/check-parity-breadcrumbs.ts | 430 |
| scripts/check-phase100-relay-activation-boundary.test.ts | 431 |
| scripts/check-phase100-relay-activation-boundary.ts | 584 |
| scripts/check-phase101-transaction-inventory-download-scheduling.test.ts | 509 |
| scripts/check-phase101-transaction-inventory-download-scheduling.ts | 546 |
| scripts/check-phase102-orphan-admission-bridge.test.ts | 617 |
| scripts/check-phase102-orphan-admission-bridge.ts | 15 |
| scripts/check-phase102-orphan-admission-bridge/bridge.ts | 136 |
| scripts/check-phase102-orphan-admission-bridge/checks.ts | 30 |
| scripts/check-phase102-orphan-admission-bridge/constants.ts | 250 |
| scripts/check-phase102-orphan-admission-bridge/filesystem.ts | 40 |
| scripts/check-phase102-orphan-admission-bridge/helpers.ts | 72 |
| scripts/check-phase102-orphan-admission-bridge/parity.ts | 135 |
| scripts/check-phase102-orphan-admission-bridge/verifier.ts | 88 |
| scripts/check-phase103-mempool-lifecycle.test.ts | 218 |
| scripts/check-phase103-mempool-lifecycle.ts | 401 |
| scripts/check-phase104-relay-serving-fanout.test.ts | 215 |
| scripts/check-phase104-relay-serving-fanout.ts | 401 |
| scripts/check-phase105-operator-relay-evidence.test.ts | 234 |
| scripts/check-phase105-operator-relay-evidence.ts | 544 |
| scripts/check-phase106-parity-uat-release-boundary.test.ts | 234 |
| scripts/check-phase106-parity-uat-release-boundary.ts | 560 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts | 339 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility.ts | 17 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/checks.ts | 148 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/claims.ts | 84 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/constants.ts | 236 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/filesystem.ts | 35 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/helpers.ts | 114 |
| scripts/check-phase107-runtime-relay-activation-download-eligibility/parity.ts | 107 |
| scripts/check-phase108-durable-mempool-relay-state-recovery.test.ts | 144 |
| scripts/check-phase108-durable-mempool-relay-state-recovery.ts | 235 |
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
| scripts/check-phase122-compact-relay-peer-completion.ts | 434 |
| scripts/check-phase123-runtime-timing-evidence-integrity.test.ts | 627 |
| scripts/check-phase123-runtime-timing-evidence-integrity.ts | 14 |
| scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts | 332 |
| scripts/check-phase123-runtime-timing-evidence-integrity/constants.ts | 73 |
| scripts/check-phase123-runtime-timing-evidence-integrity/evidence.ts | 305 |
| scripts/check-phase123-runtime-timing-evidence-integrity/filesystem.ts | 49 |
| scripts/check-phase123-runtime-timing-evidence-integrity/helpers.ts | 61 |
| scripts/check-phase123-runtime-timing-evidence-integrity/parity.ts | 131 |
| scripts/check-phase124-archive-ready.ts | 550 |
| scripts/check-phase124-milestone-closeout-lifecycle.ts | 146 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts | 37 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/base.ts | 357 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/mutations.ts | 29 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/phase125.ts | 333 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/phase126.ts | 158 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/phase127.ts | 158 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/phase128.ts | 83 |
| scripts/check-phase124-milestone-closeout-reconciliation.fixtures/phase129.ts | 136 |
| scripts/check-phase124-milestone-closeout-reconciliation.test.ts | 4 |
| scripts/check-phase124-milestone-closeout-reconciliation.test/scenarios-1.ts | 355 |
| scripts/check-phase124-milestone-closeout-reconciliation.test/scenarios-2.ts | 380 |
| scripts/check-phase124-milestone-closeout-reconciliation.test/scenarios-3.ts | 255 |
| scripts/check-phase124-milestone-closeout-reconciliation.test/setup.ts | 50 |
| scripts/check-phase124-milestone-closeout-reconciliation.ts | 626 |
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
| scripts/check-phase126-compact-relay-residual-hardening.ts | 434 |
| scripts/check-phase127-authoritative-network-state-unification.test.ts | 619 |
| scripts/check-phase127-authoritative-network-state-unification.ts | 566 |
| scripts/check-phase127-authoritative-network-state-unification/production-proof-tests.ts | 78 |
| scripts/check-phase128-production-compact-announcement-transport.test.ts | 447 |
| scripts/check-phase128-production-compact-announcement-transport.ts | 566 |
| scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts | 81 |
| scripts/check-phase128-production-compact-announcement-transport/impl-attributes.ts | 26 |
| scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts | 243 |
| scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts | 238 |
| scripts/check-phase130-resource-time-fee-primitives.test.ts | 295 |
| scripts/check-phase130-resource-time-fee-primitives.ts | 593 |
| scripts/check-phase131-rolling-fee-expiry-pressure.test.ts | 184 |
| scripts/check-phase131-rolling-fee-expiry-pressure.ts | 358 |
| scripts/check-phase132-typed-package-staged-admission.test.ts | 399 |
| scripts/check-phase132-typed-package-staged-admission.ts | 16 |
| scripts/check-phase132-typed-package-staged-admission/checks.ts | 305 |
| scripts/check-phase132-typed-package-staged-admission/claims.ts | 77 |
| scripts/check-phase132-typed-package-staged-admission/constants.ts | 87 |
| scripts/check-phase132-typed-package-staged-admission/filesystem.ts | 25 |
| scripts/check-phase132-typed-package-staged-admission/helpers.ts | 92 |
| scripts/check-phase132-typed-package-staged-admission/parity.ts | 179 |
| scripts/check-phase132-typed-package-staged-admission/policy.ts | 159 |
| scripts/check-phase133-package-aware-download-orphan-bridge.test.ts | 386 |
| scripts/check-phase133-package-aware-download-orphan-bridge.ts | 578 |
| scripts/check-phase134-apply-boundaries.ts | 452 |
| scripts/check-phase134-apply-boundaries/aggregate-roots.ts | 357 |
| scripts/check-phase134-apply-boundaries/call-resolution.ts | 239 |
| scripts/check-phase134-apply-boundaries/reachability.ts | 330 |
| scripts/check-phase134-apply-boundaries/receiver-evidence.ts | 131 |
| scripts/check-phase134-apply-boundaries/rust-calls.ts | 297 |
| scripts/check-phase134-apply-boundaries/rust-lexer.ts | 223 |
| scripts/check-phase134-apply-boundaries/strict-syntax.ts | 336 |
| scripts/check-phase134-authoritative-lifecycle.test.ts | 181 |
| scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts | 604 |
| scripts/check-phase134-authoritative-lifecycle.test/apply-helpers/aggregate-reachability.ts | 513 |
| scripts/check-phase134-authoritative-lifecycle.test/apply-helpers/strict-reachability.ts | 256 |
| scripts/check-phase134-authoritative-lifecycle.test/apply-helpers/token-scanner-reachability.ts | 557 |
| scripts/check-phase134-authoritative-lifecycle.test/fixture.ts | 149 |
| scripts/check-phase134-authoritative-lifecycle.test/mutations.ts | 389 |
| scripts/check-phase134-authoritative-lifecycle.test/scope-claims.ts | 175 |
| scripts/check-phase134-authoritative-lifecycle.ts | 585 |
| scripts/check-phase134-authoritative-lifecycle/scope.ts | 170 |
| scripts/check-phase135-snapshot-recovery.test.ts | 585 |
| scripts/check-phase135-snapshot-recovery.ts | 596 |
| scripts/check-phase135-snapshot-recovery/persisted-input-mutations.ts | 310 |
| scripts/check-phase135-snapshot-recovery/persisted-input.ts | 253 |
| scripts/check-phase135-snapshot-recovery/source.ts | 238 |
| scripts/check-phase138-parity-uat-release-boundary.test.ts | 369 |
| scripts/check-phase138-parity-uat-release-boundary.ts | 14 |
| scripts/check-phase138-parity-uat-release-boundary/checks.ts | 241 |
| scripts/check-phase138-parity-uat-release-boundary/claims.ts | 93 |
| scripts/check-phase138-parity-uat-release-boundary/constants.ts | 229 |
| scripts/check-phase138-parity-uat-release-boundary/matrix.ts | 167 |
| scripts/check-phase138-parity-uat-release-boundary/test-fixtures.ts | 187 |
| scripts/check-phase138-parity-uat-release-boundary/verifier.ts | 129 |
| scripts/check-phase144-operator-flush-availability-evidence.test.ts | 189 |
| scripts/check-phase144-operator-flush-availability-evidence.ts | 495 |
| scripts/check-phase145-parity-uat-release-boundary.test.ts | 448 |
| scripts/check-phase145-parity-uat-release-boundary.ts | 14 |
| scripts/check-phase145-parity-uat-release-boundary/checks.ts | 208 |
| scripts/check-phase145-parity-uat-release-boundary/claims.ts | 101 |
| scripts/check-phase145-parity-uat-release-boundary/constants.ts | 196 |
| scripts/check-phase145-parity-uat-release-boundary/test-fixtures.ts | 185 |
| scripts/check-phase145-parity-uat-release-boundary/verifier.ts | 142 |
| scripts/check-phase151-parity-uat-release-boundary.test.ts | 439 |
| scripts/check-phase151-parity-uat-release-boundary.ts | 14 |
| scripts/check-phase151-parity-uat-release-boundary/checks.ts | 256 |
| scripts/check-phase151-parity-uat-release-boundary/claims.ts | 111 |
| scripts/check-phase151-parity-uat-release-boundary/constants.ts | 234 |
| scripts/check-phase151-parity-uat-release-boundary/gap-closure.test.ts | 477 |
| scripts/check-phase151-parity-uat-release-boundary/gap-closure.ts | 148 |
| scripts/check-phase151-parity-uat-release-boundary/planning-sources.ts | 66 |
| scripts/check-phase151-parity-uat-release-boundary/test-fixtures.ts | 185 |
| scripts/check-phase151-parity-uat-release-boundary/verifier.ts | 149 |
| scripts/check-phase154-basic-filters.test.ts | 218 |
| scripts/check-phase154-basic-filters.ts | 138 |
| scripts/check-phase155-filter-index.test.ts | 528 |
| scripts/check-phase155-filter-index.ts | 418 |
| scripts/check-phase156-prune-coordination.test.ts | 512 |
| scripts/check-phase156-prune-coordination.ts | 331 |
| scripts/check-phase156-prune-coordination/contracts.ts | 138 |
| scripts/check-phase156-prune-coordination/rust-evidence.ts | 96 |
| scripts/check-phase157-index-catch-up.test.ts | 590 |
| scripts/check-phase157-index-catch-up.ts | 262 |
| scripts/check-phase157-index-catch-up/contracts.ts | 164 |
| scripts/check-phase157-index-catch-up/rust-evidence.ts | 48 |
| scripts/check-phase158-validated-reorg.test.ts | 623 |
| scripts/check-phase158-validated-reorg.ts | 491 |
| scripts/check-phase159-filter-rpcs.test.ts | 624 |
| scripts/check-phase159-filter-rpcs.ts | 323 |
| scripts/check-phase159-filter-rpcs/contracts.ts | 439 |
| scripts/check-phase159-filter-rpcs/evidence.ts | 171 |
| scripts/check-phase160-peer-filter-serving.test.ts | 175 |
| scripts/check-phase160-peer-filter-serving.ts | 319 |
| scripts/check-phase161-operator-index-evidence.test.ts | 137 |
| scripts/check-phase161-operator-index-evidence.ts | 164 |
| scripts/check-phase162-integrated-filter-proof.test.ts | 623 |
| scripts/check-phase162-integrated-filter-proof.ts | 434 |
| scripts/check-phase162-integrated-filter-proof/contracts.ts | 592 |
| scripts/check-phase163-provenance.test.ts | 401 |
| scripts/check-phase163-provenance.ts | 301 |
| scripts/check-phase163-provenance/fixture-controls.ts | 34 |
| scripts/check-phase163-provenance/fixtures.ts | 374 |
| scripts/check-phase163-provenance/native.test.ts | 468 |
| scripts/check-phase164-canonical-identity.test.ts | 625 |
| scripts/check-phase164-canonical-identity.ts | 288 |
| scripts/check-phase164-canonical-identity/claims.ts | 42 |
| scripts/check-phase164-canonical-identity/contracts.ts | 234 |
| scripts/check-phase164-canonical-identity/fixture-source.ts | 187 |
| scripts/check-phase164-canonical-identity/historical-tooling.ts | 241 |
| scripts/check-phase164-canonical-identity/historical.test.ts | 627 |
| scripts/check-phase164-canonical-identity/regions.ts | 30 |
| scripts/check-phase164-canonical-identity/source-files.ts | 610 |
| scripts/check-phase164-canonical-identity/test-modules.ts | 586 |
| scripts/check-phase165-shared-exact-work.test.ts | 627 |
| scripts/check-phase165-shared-exact-work.ts | 273 |
| scripts/check-phase165-shared-exact-work/contracts.ts | 455 |
| scripts/check-phase165-shared-exact-work/test-support.ts | 281 |
| scripts/check-phase61-resource-recovery-boundaries.ts | 152 |
| scripts/check-phase62-sync-truth-surfaces.ts | 265 |
| scripts/check-phase63-service-lifecycle.ts | 308 |
| scripts/check-phase64-service-restart-resume.ts | 190 |
| scripts/check-phase65-support-review.ts | 140 |
| scripts/check-phase66-compatibility-wrapper.ts | 138 |
| scripts/check-phase68-active-chain-persistence.test.ts | 32 |
| scripts/check-phase68-active-chain-persistence.ts | 199 |
| scripts/check-phase69-tip-stay-current.ts | 224 |
| scripts/check-phase70-reorg-recovery.ts | 158 |
| scripts/check-phase71-resource-restart.ts | 169 |
| scripts/check-phase72-observability-evidence.ts | 551 |
| scripts/check-phase73-uat-verification.test.ts | 568 |
| scripts/check-phase73-uat-verification.ts | 3 |
| scripts/check-phase73-uat-verification/checks.ts | 368 |
| scripts/check-phase73-uat-verification/constants.ts | 255 |
| scripts/check-phase73-uat-verification/parity.ts | 30 |
| scripts/check-phase75-soak-runner.test.ts | 522 |
| scripts/check-phase75-soak-runner.ts | 390 |
| scripts/check-phase76-resource-bounds.test.ts | 343 |
| scripts/check-phase76-resource-bounds.ts | 323 |
| scripts/check-phase77-corruption-lock-recovery.test.ts | 361 |
| scripts/check-phase77-corruption-lock-recovery.ts | 314 |
| scripts/check-phase78-progress-guarantees.test.ts | 311 |
| scripts/check-phase78-progress-guarantees.ts | 270 |
| scripts/check-phase79-diagnostics-support-bundle.test.ts | 365 |
| scripts/check-phase79-diagnostics-support-bundle.ts | 368 |
| scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts | 516 |
| scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts | 544 |
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
| scripts/check-phase97-inbound-metrics.test.ts | 479 |
| scripts/check-phase97-inbound-metrics.ts | 448 |
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
| scripts/command-timings.test.ts | 453 |
| scripts/command-timings.ts | 607 |
| scripts/diagnose-rust-test-stall.test.ts | 238 |
| scripts/diagnose-rust-test-stall.ts | 570 |
| scripts/generate-basic-filter-vectors.ts | 176 |
| scripts/generate-loc-report.ts | 571 |
| scripts/generate-peer-filter-fixtures.test.ts | 286 |
| scripts/generate-peer-filter-fixtures.ts | 366 |
| scripts/phase162-filter-oracle.test.ts | 250 |
| scripts/phase162-filter-oracle.ts | 232 |
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
| scripts/source-corpus.ts | 121 |
| scripts/test-run-live-mainnet-smoke/assert-report.ts | 163 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 8,215 |
| 2 | packages/Cargo.lock | TOML/config | 3,205 |
| 3 | packages/open-bitcoin-node/testdata/basic_operator_captures.json | Other config | 695 |
| 4 | packages/open-bitcoin-cli/src/operator/support/tests/recovery_progress_inbound.rs | Rust tests | 628 |
| 5 | packages/open-bitcoin-node/src/chainstate/flush_lifecycle/fjall_sink.rs | Rust production | 628 |
| 6 | packages/open-bitcoin-node/src/network.rs | Rust production | 628 |
| 7 | packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/protection.rs | Rust tests | 628 |
| 8 | packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs | Rust tests | 628 |
| 9 | packages/open-bitcoin-rpc/src/config/loader.rs | Rust production | 628 |
| 10 | packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs | Rust production | 627 |
| 11 | packages/open-bitcoin-mempool/src/pool/prospective.rs | Rust production | 627 |
| 12 | packages/open-bitcoin-network/src/filter_serving/policy/tests.rs | Rust tests | 627 |
| 13 | packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs | Rust production | 627 |
| 14 | packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs | Rust production | 627 |
| 15 | packages/open-bitcoin-node/src/network/relay_fanout.rs | Rust production | 627 |
| 16 | packages/open-bitcoin-node/src/network/runtime_authority.rs | Rust production | 627 |
| 17 | packages/open-bitcoin-node/src/storage/fjall_store.rs | Rust production | 627 |
| 18 | packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs | Rust tests | 627 |
| 19 | scripts/check-phase123-runtime-timing-evidence-integrity.test.ts | TypeScript/Bun scripts | 627 |
| 20 | scripts/check-phase164-canonical-identity/historical.test.ts | TypeScript/Bun scripts | 627 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 52cdea63dd88cea2b0bd59f54cb1989ef1d88044a8e496f3719b3a39b6bc9ea9 |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
