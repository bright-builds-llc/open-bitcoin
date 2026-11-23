# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 1,786 |
| Total lines | 546,630 |
| Code/content lines | 480,883 |
| Comment-only lines | 28,581 |
| Blank lines | 37,166 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,895 | 0 | 85 | 3,980 | 0.0% |
| open-bitcoin-chainstate | 76 | 8,104 | 14,086 | 26 | 22,216 | 173.8% |
| open-bitcoin-cli | 174 | 24,248 | 26,770 | 98 | 51,224 | 110.4% |
| open-bitcoin-codec | 22 | 2,458 | 1,429 | 28 | 3,918 | 58.1% |
| open-bitcoin-consensus | 78 | 11,898 | 12,609 | 28 | 24,535 | 106.0% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 82 | 10,789 | 15,651 | 30 | 26,470 | 145.1% |
| open-bitcoin-network | 153 | 18,631 | 27,913 | 30 | 46,574 | 149.8% |
| open-bitcoin-node | 580 | 73,244 | 122,298 | 42 | 196,410 | 167.0% |
| open-bitcoin-primitives | 9 | 926 | 0 | 20 | 946 | 0.0% |
| open-bitcoin-rpc | 168 | 17,136 | 35,197 | 64 | 52,397 | 205.4% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 21 | 3,529 | 2,705 | 34 | 6,268 | 76.7% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust tests | 766 | 258,658 | 225,823 | 17,197 | 15,638 |
| Rust production | 597 | 175,559 | 153,849 | 7,890 | 13,820 |
| TypeScript/Bun scripts | 347 | 93,950 | 83,834 | 3,373 | 6,743 |
| Fixture/data | 6 | 8,237 | 8,232 | 5 | 0 |
| Shell scripts | 24 | 4,842 | 4,294 | 100 | 448 |
| TOML/config | 16 | 3,474 | 3,106 | 1 | 367 |
| Other config | 7 | 1,233 | 1,165 | 0 | 68 |
| Bazel/Starlark | 18 | 422 | 387 | 0 | 35 |
| YAML | 3 | 217 | 170 | 13 | 34 |
| CI/templates | 1 | 28 | 17 | 1 | 10 |
| Hooks | 1 | 10 | 6 | 1 | 3 |

## Included TypeScript/Bun Scripts

| File | Lines |
| --- | --- |
| scripts/bright-builds-check.ts | 571 |
| scripts/check-active-milestone-verification-traceability.test.ts | 3 |
| scripts/check-active-milestone-verification-traceability.ts | 16 |
| scripts/check-active-milestone-verification-traceability/checks.ts | 161 |
| scripts/check-active-milestone-verification-traceability/constants.ts | 53 |
| scripts/check-active-milestone-verification-traceability/filesystem.ts | 124 |
| scripts/check-active-milestone-verification-traceability/lifecycle-scenarios.ts | 192 |
| scripts/check-active-milestone-verification-traceability/lifecycle.ts | 146 |
| scripts/check-active-milestone-verification-traceability/ownership.ts | 264 |
| scripts/check-active-milestone-verification-traceability/parsing.ts | 314 |
| scripts/check-active-milestone-verification-traceability/success-and-coverage.ts | 301 |
| scripts/check-active-milestone-verification-traceability/test-fixtures.ts | 176 |
| scripts/check-bazel-build-provenance.ts | 187 |
| scripts/check-benchmark-report.ts | 210 |
| scripts/check-current-documentation-reconciliation.test.ts | 316 |
| scripts/check-current-documentation-reconciliation.ts | 428 |
| scripts/check-panic-sites.test.ts | 342 |
| scripts/check-panic-sites.ts | 286 |
| scripts/check-parity-breadcrumbs.test.ts | 65 |
| scripts/check-parity-breadcrumbs.ts | 430 |
| scripts/check-parity-ledger.test.ts | 466 |
| scripts/check-parity-ledger.ts | 542 |
| scripts/check-phase158-validated-reorg.test.ts | 628 |
| scripts/check-phase158-validated-reorg.ts | 520 |
| scripts/check-phase159-filter-rpcs.test.ts | 624 |
| scripts/check-phase159-filter-rpcs.ts | 328 |
| scripts/check-phase159-filter-rpcs/contracts.ts | 441 |
| scripts/check-phase159-filter-rpcs/evidence.ts | 171 |
| scripts/check-phase160-peer-filter-serving.test.ts | 175 |
| scripts/check-phase160-peer-filter-serving.ts | 330 |
| scripts/check-phase161-operator-index-evidence.test.ts | 137 |
| scripts/check-phase161-operator-index-evidence.ts | 163 |
| scripts/check-phase162-integrated-filter-proof.test.ts | 614 |
| scripts/check-phase162-integrated-filter-proof.ts | 479 |
| scripts/check-phase162-integrated-filter-proof/contracts.ts | 256 |
| scripts/check-phase163-provenance.test.ts | 400 |
| scripts/check-phase163-provenance.ts | 301 |
| scripts/check-phase163-provenance/fixture-controls.ts | 34 |
| scripts/check-phase163-provenance/fixtures.ts | 374 |
| scripts/check-phase163-provenance/native.test.ts | 466 |
| scripts/check-phase164-canonical-identity.test.ts | 628 |
| scripts/check-phase164-canonical-identity.ts | 282 |
| scripts/check-phase164-canonical-identity/claims.ts | 214 |
| scripts/check-phase164-canonical-identity/contracts.ts | 98 |
| scripts/check-phase164-canonical-identity/fixture-source.ts | 187 |
| scripts/check-phase164-canonical-identity/historical-tooling.ts | 104 |
| scripts/check-phase164-canonical-identity/historical.test.ts | 430 |
| scripts/check-phase164-canonical-identity/regions.ts | 30 |
| scripts/check-phase164-canonical-identity/source-files.ts | 628 |
| scripts/check-phase164-canonical-identity/test-modules.ts | 606 |
| scripts/check-phase165-shared-exact-work.test.ts | 581 |
| scripts/check-phase165-shared-exact-work.ts | 235 |
| scripts/check-phase165-shared-exact-work/contracts.ts | 493 |
| scripts/check-phase165-shared-exact-work/test-support.ts | 235 |
| scripts/check-phase166-canonical-bootstrap.test.ts | 516 |
| scripts/check-phase166-canonical-bootstrap.ts | 252 |
| scripts/check-phase166-canonical-bootstrap/contracts.ts | 366 |
| scripts/check-phase167-candidate-history.test.ts | 201 |
| scripts/check-phase167-candidate-history.ts | 155 |
| scripts/check-phase167-candidate-history/contracts.ts | 164 |
| scripts/check-push-verification.test.ts | 337 |
| scripts/check-push-verification.ts | 223 |
| scripts/check-v1-5-compat-wrapper.ts | 138 |
| scripts/check-v1-5-resource-recovery.ts | 152 |
| scripts/check-v1-5-service-lifecycle.ts | 308 |
| scripts/check-v1-5-service-restart.ts | 190 |
| scripts/check-v1-5-support-review.ts | 140 |
| scripts/check-v1-5-sync-truth.ts | 265 |
| scripts/check-v1-6-active-chain.test.ts | 32 |
| scripts/check-v1-6-active-chain.ts | 199 |
| scripts/check-v1-6-observability.ts | 551 |
| scripts/check-v1-6-opt-in-uat.test.ts | 568 |
| scripts/check-v1-6-opt-in-uat.ts | 3 |
| scripts/check-v1-6-opt-in-uat/checks.ts | 368 |
| scripts/check-v1-6-opt-in-uat/constants.ts | 255 |
| scripts/check-v1-6-opt-in-uat/parity.ts | 30 |
| scripts/check-v1-6-reorg-recovery.ts | 158 |
| scripts/check-v1-6-resource-restart.ts | 169 |
| scripts/check-v1-6-tip-current.ts | 224 |
| scripts/check-v1-7-corruption-recovery.test.ts | 330 |
| scripts/check-v1-7-corruption-recovery.ts | 271 |
| scripts/check-v1-7-progress-guarantees.test.ts | 267 |
| scripts/check-v1-7-progress-guarantees.ts | 226 |
| scripts/check-v1-7-release-boundary.test.ts | 289 |
| scripts/check-v1-7-release-boundary.ts | 448 |
| scripts/check-v1-7-resource-bounds.test.ts | 343 |
| scripts/check-v1-7-resource-bounds.ts | 323 |
| scripts/check-v1-7-soak-runner.test.ts | 522 |
| scripts/check-v1-7-soak-runner.ts | 390 |
| scripts/check-v1-7-support-bundle.test.ts | 333 |
| scripts/check-v1-7-support-bundle.ts | 324 |
| scripts/check-v1-8-claim-guardrails.test.ts | 315 |
| scripts/check-v1-8-claim-guardrails.ts | 498 |
| scripts/check-v1-8-operator-runbooks.test.ts | 441 |
| scripts/check-v1-8-operator-runbooks.ts | 14 |
| scripts/check-v1-8-operator-runbooks/checks.ts | 322 |
| scripts/check-v1-8-operator-runbooks/constants.ts | 237 |
| scripts/check-v1-8-operator-runbooks/parity.ts | 66 |
| scripts/check-v1-8-production-claim.test.ts | 434 |
| scripts/check-v1-8-production-claim.ts | 611 |
| scripts/check-v1-8-release-readiness.test.ts | 384 |
| scripts/check-v1-8-release-readiness.ts | 443 |
| scripts/check-v1-8-service-operation.test.ts | 445 |
| scripts/check-v1-8-service-operation.ts | 14 |
| scripts/check-v1-8-service-operation/checks.ts | 332 |
| scripts/check-v1-8-service-operation/constants.ts | 227 |
| scripts/check-v1-8-service-operation/parity.ts | 79 |
| scripts/check-v1-8-support-matrix.test.ts | 589 |
| scripts/check-v1-8-support-matrix.ts | 14 |
| scripts/check-v1-8-support-matrix/checks.ts | 342 |
| scripts/check-v1-8-support-matrix/constants.ts | 192 |
| scripts/check-v1-8-support-matrix/parity.ts | 142 |
| scripts/check-v1-8-upgrade-rollback.test.ts | 451 |
| scripts/check-v1-8-upgrade-rollback.ts | 533 |
| scripts/check-v1-9-address-discovery.test.ts | 466 |
| scripts/check-v1-9-address-discovery.ts | 513 |
| scripts/check-v1-9-dos-governance.test.ts | 440 |
| scripts/check-v1-9-dos-governance.ts | 521 |
| scripts/check-v1-9-inbound-admission.test.ts | 460 |
| scripts/check-v1-9-inbound-admission.ts | 14 |
| scripts/check-v1-9-inbound-admission/checks.ts | 340 |
| scripts/check-v1-9-inbound-admission/constants.ts | 268 |
| scripts/check-v1-9-inbound-admission/parity.ts | 90 |
| scripts/check-v1-9-inbound-metrics.test.ts | 479 |
| scripts/check-v1-9-inbound-metrics.ts | 448 |
| scripts/check-v1-9-peer-permissions.test.ts | 426 |
| scripts/check-v1-9-peer-permissions.ts | 14 |
| scripts/check-v1-9-peer-permissions/checks.ts | 338 |
| scripts/check-v1-9-peer-permissions/constants.ts | 253 |
| scripts/check-v1-9-peer-permissions/parity.ts | 122 |
| scripts/check-v1-9-peer-policy-bridge.test.ts | 321 |
| scripts/check-v1-9-peer-policy-bridge.ts | 353 |
| scripts/check-v1-9-peer-policy-logs.test.ts | 225 |
| scripts/check-v1-9-peer-policy-logs.ts | 170 |
| scripts/check-v1-9-peer-policy.test.ts | 169 |
| scripts/check-v1-9-peer-policy.ts | 466 |
| scripts/check-v1-9-release-boundary.test.ts | 1 |
| scripts/check-v1-9-release-boundary.test/scenarios-1.ts | 225 |
| scripts/check-v1-9-release-boundary.test/setup.ts | 375 |
| scripts/check-v1-9-release-boundary.ts | 14 |
| scripts/check-v1-9-release-boundary/checks.ts | 275 |
| scripts/check-v1-9-release-boundary/constants.ts | 172 |
| scripts/check-v1-9-release-boundary/parity.ts | 129 |
| scripts/check-v1.3-release-boundaries.ts | 184 |
| scripts/check-v1.4-release-boundaries.ts | 241 |
| scripts/check-v1.5-release-boundaries.ts | 296 |
| scripts/check-v1.6-release-boundaries.ts | 330 |
| scripts/check-v2-0-durable-relay-recovery.test.ts | 144 |
| scripts/check-v2-0-durable-relay-recovery.ts | 235 |
| scripts/check-v2-0-mempool-lifecycle.test.ts | 218 |
| scripts/check-v2-0-mempool-lifecycle.ts | 401 |
| scripts/check-v2-0-operator-evidence.test.ts | 218 |
| scripts/check-v2-0-operator-evidence.ts | 493 |
| scripts/check-v2-0-orphan-admission.test.ts | 571 |
| scripts/check-v2-0-orphan-admission.ts | 15 |
| scripts/check-v2-0-orphan-admission/bridge.ts | 136 |
| scripts/check-v2-0-orphan-admission/checks.ts | 29 |
| scripts/check-v2-0-orphan-admission/constants.ts | 231 |
| scripts/check-v2-0-orphan-admission/filesystem.ts | 40 |
| scripts/check-v2-0-orphan-admission/helpers.ts | 68 |
| scripts/check-v2-0-orphan-admission/parity.ts | 135 |
| scripts/check-v2-0-orphan-admission/verifier.ts | 28 |
| scripts/check-v2-0-relay-activation.test.ts | 368 |
| scripts/check-v2-0-relay-activation.ts | 479 |
| scripts/check-v2-0-relay-eligibility.test.ts | 308 |
| scripts/check-v2-0-relay-eligibility.ts | 17 |
| scripts/check-v2-0-relay-eligibility/checks.ts | 146 |
| scripts/check-v2-0-relay-eligibility/claims.ts | 39 |
| scripts/check-v2-0-relay-eligibility/constants.ts | 217 |
| scripts/check-v2-0-relay-eligibility/filesystem.ts | 35 |
| scripts/check-v2-0-relay-eligibility/helpers.ts | 114 |
| scripts/check-v2-0-relay-eligibility/parity.ts | 107 |
| scripts/check-v2-0-relay-fanout.test.ts | 208 |
| scripts/check-v2-0-relay-fanout.ts | 349 |
| scripts/check-v2-0-release-boundary.test.ts | 203 |
| scripts/check-v2-0-release-boundary.ts | 488 |
| scripts/check-v2-0-tx-download-scheduling.test.ts | 459 |
| scripts/check-v2-0-tx-download-scheduling.ts | 444 |
| scripts/check-v2-1-block-request-path.test.ts | 423 |
| scripts/check-v2-1-block-request-path.ts | 467 |
| scripts/check-v2-1-block-serving-boundary.test.ts | 289 |
| scripts/check-v2-1-block-serving-boundary.ts | 476 |
| scripts/check-v2-1-compact-announcement.test.ts | 619 |
| scripts/check-v2-1-compact-announcement.ts | 557 |
| scripts/check-v2-1-compact-announcement/durable-trigger.ts | 228 |
| scripts/check-v2-1-compact-announcement/impl-attributes.ts | 26 |
| scripts/check-v2-1-compact-hardening.test.ts | 465 |
| scripts/check-v2-1-compact-hardening.ts | 382 |
| scripts/check-v2-1-compact-peer-completion.test.ts | 263 |
| scripts/check-v2-1-compact-peer-completion.ts | 434 |
| scripts/check-v2-1-integration-guardrails.test.ts | 213 |
| scripts/check-v2-1-integration-guardrails.ts | 141 |
| scripts/check-v2-1-network-state.test.ts | 608 |
| scripts/check-v2-1-network-state.ts | 562 |
| scripts/check-v2-1-network-state/production-proof-tests.ts | 323 |
| scripts/check-v2-1-operator-evidence.test.ts | 228 |
| scripts/check-v2-1-operator-evidence.ts | 418 |
| scripts/check-v2-1-release-boundary.test.ts | 4 |
| scripts/check-v2-1-release-boundary.ts | 489 |
| scripts/check-v2-1-release-boundary/claims.ts | 110 |
| scripts/check-v2-1-release-boundary/lifecycle-routing.ts | 60 |
| scripts/check-v2-1-release-boundary/success-and-parity.ts | 199 |
| scripts/check-v2-1-release-boundary/test-fixtures.ts | 270 |
| scripts/check-v2-1-release-boundary/verifier.ts | 27 |
| scripts/check-v2-1-runtime-metrics-logs.test.ts | 440 |
| scripts/check-v2-1-runtime-metrics-logs.ts | 345 |
| scripts/check-v2-1-runtime-timing.test.ts | 552 |
| scripts/check-v2-1-runtime-timing.ts | 14 |
| scripts/check-v2-1-runtime-timing/checks.ts | 511 |
| scripts/check-v2-1-runtime-timing/constants.ts | 65 |
| scripts/check-v2-1-runtime-timing/evidence.ts | 327 |
| scripts/check-v2-1-runtime-timing/filesystem.ts | 53 |
| scripts/check-v2-1-runtime-timing/helpers.ts | 41 |
| scripts/check-v2-1-runtime-timing/parity.ts | 105 |
| scripts/check-v2-1-runtime-timing/test-fixtures.ts | 461 |
| scripts/check-v2-2-cross-cache-apply.ts | 452 |
| scripts/check-v2-2-cross-cache-apply/aggregate-roots.ts | 357 |
| scripts/check-v2-2-cross-cache-apply/call-resolution.ts | 239 |
| scripts/check-v2-2-cross-cache-apply/reachability.ts | 330 |
| scripts/check-v2-2-cross-cache-apply/receiver-evidence.ts | 131 |
| scripts/check-v2-2-cross-cache-apply/rust-calls.ts | 297 |
| scripts/check-v2-2-cross-cache-apply/rust-lexer.ts | 223 |
| scripts/check-v2-2-cross-cache-apply/strict-syntax.ts | 336 |
| scripts/check-v2-2-cross-cache-lifecycle.test.ts | 181 |
| scripts/check-v2-2-cross-cache-lifecycle.test/apply-helpers.ts | 604 |
| scripts/check-v2-2-cross-cache-lifecycle.test/apply-helpers/aggregate-reachability.ts | 513 |
| scripts/check-v2-2-cross-cache-lifecycle.test/apply-helpers/strict-reachability.ts | 256 |
| scripts/check-v2-2-cross-cache-lifecycle.test/apply-helpers/token-scanner-reachability.ts | 557 |
| scripts/check-v2-2-cross-cache-lifecycle.test/fixture.ts | 149 |
| scripts/check-v2-2-cross-cache-lifecycle.test/mutations.ts | 347 |
| scripts/check-v2-2-cross-cache-lifecycle.test/scope-claims.ts | 175 |
| scripts/check-v2-2-cross-cache-lifecycle.ts | 556 |
| scripts/check-v2-2-cross-cache-lifecycle/scope.ts | 170 |
| scripts/check-v2-2-fee-pressure.test.ts | 158 |
| scripts/check-v2-2-fee-pressure.ts | 264 |
| scripts/check-v2-2-fee-primitives.test.ts | 269 |
| scripts/check-v2-2-fee-primitives.ts | 499 |
| scripts/check-v2-2-package-admission.test.ts | 390 |
| scripts/check-v2-2-package-admission.ts | 16 |
| scripts/check-v2-2-package-admission/checks.ts | 304 |
| scripts/check-v2-2-package-admission/claims.ts | 51 |
| scripts/check-v2-2-package-admission/constants.ts | 78 |
| scripts/check-v2-2-package-admission/filesystem.ts | 25 |
| scripts/check-v2-2-package-admission/helpers.ts | 72 |
| scripts/check-v2-2-package-admission/parity.ts | 179 |
| scripts/check-v2-2-package-admission/policy.ts | 159 |
| scripts/check-v2-2-package-orphan-bridge.test.ts | 377 |
| scripts/check-v2-2-package-orphan-bridge.ts | 535 |
| scripts/check-v2-2-release-boundary.test.ts | 326 |
| scripts/check-v2-2-release-boundary.ts | 14 |
| scripts/check-v2-2-release-boundary/checks.ts | 240 |
| scripts/check-v2-2-release-boundary/claims.ts | 93 |
| scripts/check-v2-2-release-boundary/constants.ts | 207 |
| scripts/check-v2-2-release-boundary/matrix.ts | 167 |
| scripts/check-v2-2-release-boundary/test-fixtures.ts | 151 |
| scripts/check-v2-2-release-boundary/verifier.ts | 40 |
| scripts/check-v2-2-snapshot-recovery.test.ts | 602 |
| scripts/check-v2-2-snapshot-recovery.ts | 573 |
| scripts/check-v2-2-snapshot-recovery/persisted-input-mutations.ts | 368 |
| scripts/check-v2-2-snapshot-recovery/persisted-input.ts | 253 |
| scripts/check-v2-2-snapshot-recovery/source.ts | 293 |
| scripts/check-v2-3-operator-evidence.test.ts | 145 |
| scripts/check-v2-3-operator-evidence.ts | 450 |
| scripts/check-v2-3-release-boundary.test.ts | 372 |
| scripts/check-v2-3-release-boundary.ts | 14 |
| scripts/check-v2-3-release-boundary/checks.ts | 179 |
| scripts/check-v2-3-release-boundary/claims.ts | 48 |
| scripts/check-v2-3-release-boundary/constants.ts | 131 |
| scripts/check-v2-3-release-boundary/test-fixtures.ts | 136 |
| scripts/check-v2-3-release-boundary/verifier.ts | 15 |
| scripts/check-v2-4-release-boundary.test.ts | 403 |
| scripts/check-v2-4-release-boundary.ts | 14 |
| scripts/check-v2-4-release-boundary/checks.ts | 225 |
| scripts/check-v2-4-release-boundary/claims.ts | 56 |
| scripts/check-v2-4-release-boundary/constants.ts | 177 |
| scripts/check-v2-4-release-boundary/gap-closure.test.ts | 477 |
| scripts/check-v2-4-release-boundary/gap-closure.ts | 148 |
| scripts/check-v2-4-release-boundary/planning-sources.ts | 66 |
| scripts/check-v2-4-release-boundary/test-fixtures.ts | 152 |
| scripts/check-v2-4-release-boundary/verifier.ts | 20 |
| scripts/check-v2-5-basic-filters.test.ts | 218 |
| scripts/check-v2-5-basic-filters.ts | 138 |
| scripts/check-v2-5-filter-index.test.ts | 528 |
| scripts/check-v2-5-filter-index.ts | 418 |
| scripts/check-v2-5-index-catch-up.test.ts | 613 |
| scripts/check-v2-5-index-catch-up.ts | 267 |
| scripts/check-v2-5-index-catch-up/contracts.ts | 168 |
| scripts/check-v2-5-index-catch-up/rust-evidence.ts | 48 |
| scripts/check-v2-5-prune-coordination.test.ts | 512 |
| scripts/check-v2-5-prune-coordination.ts | 331 |
| scripts/check-v2-5-prune-coordination/contracts.ts | 138 |
| scripts/check-v2-5-prune-coordination/rust-evidence.ts | 96 |
| scripts/checker-shared/claim-text.ts | 105 |
| scripts/checker-shared/parity-index.ts | 35 |
| scripts/checker-shared/repo-paths.ts | 49 |
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
| scripts/knots-regtest/branch-qualification.ts | 60 |
| scripts/knots-regtest/branches.ts | 122 |
| scripts/knots-regtest/build.ts | 143 |
| scripts/knots-regtest/daemon.ts | 211 |
| scripts/knots-regtest/process.ts | 58 |
| scripts/knots-regtest/qualification.ts | 91 |
| scripts/knots-regtest/receipt.ts | 38 |
| scripts/knots-regtest/regtest-case.ts | 41 |
| scripts/knots-regtest/regtest-history.ts | 95 |
| scripts/knots-regtest/rejections.ts | 133 |
| scripts/knots-regtest/state.ts | 165 |
| scripts/phase162-filter-oracle.test.ts | 250 |
| scripts/phase162-filter-oracle.ts | 232 |
| scripts/phase166-genesis-vectors.test.ts | 47 |
| scripts/phase166-genesis-vectors.ts | 193 |
| scripts/phase167-bip30-fixtures.test.ts | 350 |
| scripts/phase167-bip30-fixtures.ts | 339 |
| scripts/phase167-historical-corpus.test.ts | 282 |
| scripts/phase167-historical-corpus.ts | 569 |
| scripts/phase167-regtest-corpus.test.ts | 318 |
| scripts/phase167-regtest-corpus.ts | 615 |
| scripts/phase167-regtest-serialization.ts | 245 |
| scripts/process-liveness.ts | 53 |
| scripts/run-knots-regtest-differential.ts | 148 |
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
| scripts/verify-shards.test.ts | 359 |
| scripts/verify/sample-checker-tests.test.ts | 88 |
| scripts/verify/sample-checker-tests.ts | 19 |
| scripts/verify/sampled-bun-test.ts | 65 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 8,215 |
| 2 | packages/Cargo.lock | TOML/config | 3,205 |
| 3 | packages/open-bitcoin-node/testdata/basic_operator_captures.json | Other config | 695 |
| 4 | packages/open-bitcoin-cli/src/client/tests.rs | Rust tests | 628 |
| 5 | packages/open-bitcoin-cli/src/operator/support/tests/recovery_progress_inbound.rs | Rust tests | 628 |
| 6 | packages/open-bitcoin-network/src/header_store/canonical_history/tests.rs | Rust tests | 628 |
| 7 | packages/open-bitcoin-network/src/peer/tests/compact_retry_cases.rs | Rust tests | 628 |
| 8 | packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/protection.rs | Rust tests | 628 |
| 9 | packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests.rs | Rust tests | 628 |
| 10 | packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/durable_tests.rs | Rust production | 628 |
| 11 | packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs | Rust tests | 628 |
| 12 | packages/open-bitcoin-rpc/src/config/loader.rs | Rust production | 628 |
| 13 | packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs | Rust tests | 628 |
| 14 | scripts/check-phase158-validated-reorg.test.ts | TypeScript/Bun scripts | 628 |
| 15 | scripts/check-phase164-canonical-identity.test.ts | TypeScript/Bun scripts | 628 |
| 16 | scripts/check-phase164-canonical-identity/source-files.ts | TypeScript/Bun scripts | 628 |
| 17 | packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs | Rust production | 627 |
| 18 | packages/open-bitcoin-mempool/src/pool/prospective.rs | Rust production | 627 |
| 19 | packages/open-bitcoin-network/src/filter_serving/policy/tests.rs | Rust tests | 627 |
| 20 | packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs | Rust production | 627 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 043546788318d5d9599e44bc7d4aaf11f82847eefeea126da86cabd275e3eb5e |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
