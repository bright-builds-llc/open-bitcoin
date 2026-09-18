# Lines Of Code Report

Deterministic first-party LOC report for Open Bitcoin code and tooling.

## Aggregate

| Metric | Value |
| --- | --- |
| Included files | 1,191 |
| Total lines | 339,099 |
| Code/content lines | 295,244 |
| Comment-only lines | 15,529 |
| Blank lines | 28,326 |

## Per-Crate Modules

| Module | Files | Production Rust | Test Rust | Manifest/Build | Total | Test/Source |
| --- | --- | --- | --- | --- | --- | --- |
| open-bitcoin-bench | 22 | 3,765 | 0 | 85 | 3,850 | 0.0% |
| open-bitcoin-chainstate | 23 | 2,596 | 4,527 | 26 | 7,149 | 174.4% |
| open-bitcoin-cli | 147 | 22,614 | 21,664 | 97 | 44,375 | 95.8% |
| open-bitcoin-codec | 15 | 1,811 | 779 | 28 | 2,621 | 43.0% |
| open-bitcoin-consensus | 47 | 6,592 | 7,851 | 28 | 14,471 | 119.1% |
| open-bitcoin-core | 3 | 39 | 0 | 36 | 75 | 0.0% |
| open-bitcoin-mempool | 82 | 10,799 | 15,647 | 30 | 26,476 | 144.9% |
| open-bitcoin-network | 124 | 14,538 | 21,148 | 30 | 35,716 | 145.5% |
| open-bitcoin-node | 280 | 33,389 | 44,430 | 42 | 77,861 | 133.1% |
| open-bitcoin-primitives | 9 | 877 | 0 | 20 | 897 | 0.0% |
| open-bitcoin-rpc | 97 | 12,736 | 12,844 | 59 | 25,639 | 100.8% |
| open-bitcoin-test-harness | 7 | 662 | 0 | 28 | 690 | 0.0% |
| open-bitcoin-wallet | 21 | 3,529 | 2,509 | 34 | 6,072 | 71.1% |

## Language And Category Breakdown

| Category | Files | Total | Code/Content | Comments | Blank |
| --- | --- | --- | --- | --- | --- |
| Rust tests | 450 | 131,399 | 111,473 | 9,442 | 10,484 |
| Rust production | 398 | 113,947 | 99,087 | 4,212 | 10,648 |
| TypeScript/Bun scripts | 274 | 76,878 | 68,746 | 1,797 | 6,335 |
| Fixture/data | 6 | 8,234 | 8,229 | 5 | 0 |
| Shell scripts | 22 | 4,509 | 4,036 | 64 | 409 |
| TOML/config | 16 | 3,468 | 3,102 | 0 | 366 |
| Bazel/Starlark | 18 | 415 | 380 | 0 | 35 |
| YAML | 3 | 184 | 145 | 7 | 32 |
| CI/templates | 1 | 28 | 17 | 1 | 10 |
| Other config | 2 | 27 | 23 | 0 | 4 |
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
| scripts/check-current-documentation-reconciliation.test.ts | 342 |
| scripts/check-current-documentation-reconciliation.ts | 461 |
| scripts/check-parity-breadcrumbs.ts | 427 |
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
| scripts/check-phase103-mempool-lifecycle.test.ts | 238 |
| scripts/check-phase103-mempool-lifecycle.ts | 401 |
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
| scripts/check-phase123-runtime-timing-evidence-integrity.test.ts | 479 |
| scripts/check-phase123-runtime-timing-evidence-integrity.ts | 14 |
| scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts | 271 |
| scripts/check-phase123-runtime-timing-evidence-integrity/constants.ts | 71 |
| scripts/check-phase123-runtime-timing-evidence-integrity/evidence.ts | 215 |
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
| scripts/check-phase127-authoritative-network-state-unification.test.ts | 554 |
| scripts/check-phase127-authoritative-network-state-unification.ts | 549 |
| scripts/check-phase128-production-compact-announcement-transport.test.ts | 264 |
| scripts/check-phase128-production-compact-announcement-transport.ts | 573 |
| scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts | 242 |
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
| scripts/check-phase134-apply-boundaries/aggregate-roots.ts | 314 |
| scripts/check-phase134-apply-boundaries/call-resolution.ts | 239 |
| scripts/check-phase134-apply-boundaries/reachability.ts | 326 |
| scripts/check-phase134-apply-boundaries/receiver-evidence.ts | 131 |
| scripts/check-phase134-apply-boundaries/rust-calls.ts | 297 |
| scripts/check-phase134-apply-boundaries/rust-lexer.ts | 223 |
| scripts/check-phase134-apply-boundaries/strict-syntax.ts | 336 |
| scripts/check-phase134-authoritative-lifecycle.test.ts | 640 |
| scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts | 512 |
| scripts/check-phase134-authoritative-lifecycle.test/apply-helpers/aggregate-reachability.ts | 513 |
| scripts/check-phase134-authoritative-lifecycle.test/apply-helpers/strict-reachability.ts | 256 |
| scripts/check-phase134-authoritative-lifecycle.test/apply-helpers/token-scanner-reachability.ts | 557 |
| scripts/check-phase134-authoritative-lifecycle.test/scope-claims.ts | 175 |
| scripts/check-phase134-authoritative-lifecycle.ts | 520 |
| scripts/check-phase134-authoritative-lifecycle/scope.ts | 170 |
| scripts/check-phase135-snapshot-recovery.test.ts | 552 |
| scripts/check-phase135-snapshot-recovery.ts | 604 |
| scripts/check-phase135-snapshot-recovery/persisted-input-mutations.ts | 310 |
| scripts/check-phase135-snapshot-recovery/persisted-input.ts | 253 |
| scripts/check-phase135-snapshot-recovery/source.ts | 217 |
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
| scripts/check-phase145-parity-uat-release-boundary.test.ts | 417 |
| scripts/check-phase145-parity-uat-release-boundary.ts | 14 |
| scripts/check-phase145-parity-uat-release-boundary/checks.ts | 195 |
| scripts/check-phase145-parity-uat-release-boundary/claims.ts | 93 |
| scripts/check-phase145-parity-uat-release-boundary/constants.ts | 190 |
| scripts/check-phase145-parity-uat-release-boundary/test-fixtures.ts | 185 |
| scripts/check-phase145-parity-uat-release-boundary/verifier.ts | 142 |
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
| scripts/source-corpus.ts | 119 |
| scripts/test-run-live-mainnet-smoke/assert-report.ts | 163 |

## Largest Included Files

| Rank | File | Category | Lines |
| --- | --- | --- | --- |
| 1 | MODULE.bazel.lock | Fixture/data | 8,217 |
| 2 | packages/Cargo.lock | TOML/config | 3,206 |
| 3 | packages/open-bitcoin-chainstate/src/coins/tests.rs | Rust tests | 755 |
| 4 | packages/open-bitcoin-chainstate/src/coins/tests/flush.rs | Rust tests | 741 |
| 5 | packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo.rs | Rust tests | 714 |
| 6 | packages/open-bitcoin-node/src/sync/tests/restart_chainstate.rs | Rust tests | 700 |
| 7 | packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests.rs | Rust tests | 674 |
| 8 | packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs | Rust tests | 659 |
| 9 | packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs | Rust tests | 646 |
| 10 | scripts/check-phase134-authoritative-lifecycle.test.ts | TypeScript/Bun scripts | 640 |
| 11 | scripts/verify.sh | Shell scripts | 638 |
| 12 | packages/open-bitcoin-node/src/network/tests/block_serving.rs | Rust tests | 632 |
| 13 | packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs | Rust production | 627 |
| 14 | packages/open-bitcoin-mempool/src/pool/prospective.rs | Rust production | 627 |
| 15 | packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs | Rust production | 627 |
| 16 | packages/open-bitcoin-node/src/network/relay_fanout.rs | Rust production | 627 |
| 17 | packages/open-bitcoin-node/src/sync/session.rs | Rust production | 627 |
| 18 | scripts/check-phase92-address-boundaries.ts | TypeScript/Bun scripts | 627 |
| 19 | scripts/check-phase94-dos-resource-governance.ts | TypeScript/Bun scripts | 627 |
| 20 | packages/open-bitcoin-wallet/src/address.rs | Rust production | 626 |

## Metadata

| Field | Value |
| --- | --- |
| Source mode | CLI-selected worktree or index; report output is mode-stable |
| Input fingerprint | 2b1a19a35b3c344afe05f7bf83aa69cb8adaccb3011e36f031b8d3e2806ad29c |
| Generator command | bun run scripts/generate-loc-report.ts --source=MODE --output=docs/metrics/lines-of-code.md |
| Included scope | open-bitcoin crates under packages/, repo scripts, hooks, CI, and root build/config files |
| Excluded scope | vendored Knots, generated/build outputs, GSD planning artifacts, docs, and this report |
