# Operator Runtime Release Hardening

This entry tracks the Phase 22 closeout slice for the v1.1 operator runtime. It
ties together repo-native verification, deterministic real-sync benchmark
evidence, operator-facing documentation, and the parity-ledger updates that keep
shipped claims separate from deferred or out-of-scope work.

## Coverage

- repo-native verification for the current CLI, config, service, storage, sync,
  metrics, logging, dashboard, migration, and parity-breadcrumb surfaces
- deterministic runtime-backed benchmark evidence for headers sync, block
  download or connect, storage write or read, restart recovery,
  runtime-collected status or dashboard projection, and wallet-rescan cost
- operator-facing documentation for source-built install, onboarding, config
  ownership, service lifecycle, status, dashboard, migration planning, benchmark
  workflow, and known limitations
- release-readiness and checklist updates that keep current v1.1 claims auditable

## Knots sources

- [`packages/bitcoin-knots/src/headerssync.cpp`](../../../packages/bitcoin-knots/src/headerssync.cpp)
- [`packages/bitcoin-knots/src/sync.cpp`](../../../packages/bitcoin-knots/src/sync.cpp)
- [`packages/bitcoin-knots/src/node/blockstorage.cpp`](../../../packages/bitcoin-knots/src/node/blockstorage.cpp)
- [`packages/bitcoin-knots/src/bitcoin-cli.cpp`](../../../packages/bitcoin-knots/src/bitcoin-cli.cpp)
- [`packages/bitcoin-knots/src/init.cpp`](../../../packages/bitcoin-knots/src/init.cpp)
- [`packages/bitcoin-knots/src/interfaces/node.h`](../../../packages/bitcoin-knots/src/interfaces/node.h)
- [`packages/bitcoin-knots/contrib/init/org.bitcoin.bitcoind.plist`](../../../packages/bitcoin-knots/contrib/init/org.bitcoin.bitcoind.plist)
- [`packages/bitcoin-knots/contrib/init/bitcoind.service`](../../../packages/bitcoin-knots/contrib/init/bitcoind.service)
- [`packages/bitcoin-knots/doc/init.md`](../../../packages/bitcoin-knots/doc/init.md)
- [`packages/bitcoin-knots/doc/managing-wallets.md`](../../../packages/bitcoin-knots/doc/managing-wallets.md)

## First-party implementation

- [`scripts/verify.sh`](../../../scripts/verify.sh)
- [`scripts/run-benchmarks.sh`](../../../scripts/run-benchmarks.sh)
- [`scripts/check-benchmark-report.ts`](../../../scripts/check-benchmark-report.ts)
- [`packages/open-bitcoin-bench/src/registry.rs`](../../../packages/open-bitcoin-bench/src/registry.rs)
- [`packages/open-bitcoin-bench/src/report.rs`](../../../packages/open-bitcoin-bench/src/report.rs)
- [`packages/open-bitcoin-bench/src/cases/sync_runtime.rs`](../../../packages/open-bitcoin-bench/src/cases/sync_runtime.rs)
- [`packages/open-bitcoin-bench/src/cases/storage_recovery.rs`](../../../packages/open-bitcoin-bench/src/cases/storage_recovery.rs)
- [`packages/open-bitcoin-bench/src/cases/operator_runtime.rs`](../../../packages/open-bitcoin-bench/src/cases/operator_runtime.rs)
- [`packages/open-bitcoin-bench/src/cases/wallet_rescan.rs`](../../../packages/open-bitcoin-bench/src/cases/wallet_rescan.rs)
- [`docs/operator/runtime-guide.md`](../../operator/runtime-guide.md)
- [`docs/architecture/cli-command-architecture.md`](../../architecture/cli-command-architecture.md)
- [`docs/architecture/config-precedence.md`](../../architecture/config-precedence.md)
- [`docs/parity/benchmarks.md`](../benchmarks.md)
- [`docs/parity/release-readiness.md`](../release-readiness.md)

## Audit Matrix

| Surface | Baseline expectation | Open Bitcoin current behavior | Evidence | Deferred or out-of-scope notes |
| --- | --- | --- | --- | --- |
| Repo-native verification | Local review should prove correctness and operator-surface integrity without requiring public-network access by default. | `bash scripts/verify.sh` runs format, lint, build, tests, benchmark smoke, benchmark-report validation, parity-breadcrumb checks, and Bazel smoke builds from one repo-owned entrypoint. | `scripts/verify.sh`, `scripts/check-benchmark-report.ts`, `packages/open-bitcoin-cli/tests/operator_flows.rs` | Public-network sync remains outside the default local gate. |
| Real-sync benchmark evidence | Reviewers need explicit evidence for sync, storage, restart, status, dashboard, and wallet-rescan cost. | `open-bitcoin-bench` now emits deterministic runtime-backed cases and records profile plus measurement metadata in JSON and Markdown reports. | `packages/open-bitcoin-bench/src/registry.rs`, `packages/open-bitcoin-bench/src/report.rs`, `docs/parity/benchmarks.md` | Timing thresholds remain intentionally disabled. |
| Operator install and onboarding docs | Operators should understand how to build, configure, and bootstrap the current runtime without guessing from tests. | The new operator guide explains the source-built install path, onboarding flags, config ownership, and regtest preview flow. | `docs/operator/runtime-guide.md`, `README.md`, `docs/architecture/config-precedence.md` | Packaged install flows remain outside the current slice. |
| Service, status, and dashboard docs | Service lifecycle and runtime inspection surfaces should be documented as shipped behavior, not as placeholders. | README and architecture docs now describe the actual `service`, `status`, and `dashboard` flows, including dry-run install or uninstall semantics and non-TTY dashboard snapshots. | `docs/operator/runtime-guide.md`, `docs/architecture/cli-command-architecture.md`, `packages/open-bitcoin-cli/tests/operator_binary.rs` | Windows service support and hosted dashboards remain out of scope. |
| Migration limits in release narrative | Release docs must keep migration non-claims visible instead of implying automatic cutover. | The operator guide and parity ledger continue to frame migration as dry-run only, with manual cutover and external-wallet mutation left out of scope. | `docs/operator/runtime-guide.md`, `docs/parity/catalog/drop-in-audit-and-migration.md`, `docs/parity/index.json` | Migration apply mode is future work. |
| Release-readiness ledger | The machine-readable root should separate shipped v1.1 claims from deferred or out-of-scope surfaces. | `docs/parity/index.json`, `docs/parity/checklist.md`, and `docs/parity/release-readiness.md` now treat the Phase 22 operator-runtime closeout as explicit audit evidence. | `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/release-readiness.md` | Future packaging, public dashboards, and broader runtime parity claims remain deferred. |
| Phase 72 observability/support evidence | Operator evidence should prove what it claims without expanding the production scope. | CLI status, dashboard, RPC durable sync status, metrics, structured logs, live-smoke reports, and support bundles now share compact full-sync truth fields and typed verdicts. | `docs/operator/runtime-guide.md`, `scripts/check-phase72-observability-evidence.ts`, `packages/open-bitcoin-cli/tests/operator_binary.rs` | This adds observability/support evidence only, not inbound serving, address relay, block serving, transaction relay, compact block relay, production-funds wallet claims, migration apply mode, signed packaging, Windows service support, GUI, hosted dashboards, or broad production-node readiness. |
| Phase 73 opt-in UAT and deterministic verification | Contributors should audit local UAT, fixture, compatibility-harness, support-bundle, live-smoke, deterministic checker, and breadcrumb evidence without reading every implementation file. | The operator guide documents opt-in public-mainnet UAT and `support bundle --output-dir=/tmp/open-bitcoin-support` as local redacted evidence, while `scripts/check-phase73-uat-verification.ts` runs through `scripts/verify.sh` to keep default verification deterministic. | `docs/operator/runtime-guide.md`, `scripts/check-phase73-uat-verification.ts`, `scripts/verify.sh`, `docs/parity/source-breadcrumbs.json`, `scripts/check-parity-breadcrumbs.ts` | Phase 73 does not add production-node readiness, inbound serving, address relay, block serving, transaction relay, compact block relay, production-funds wallet claims, migration apply mode, signed packaging, Windows service support, GUI, hosted dashboards, public-network CI, or release-blocking live sync. |
| v1.6 release boundary closeout | Reviewers should audit source-built, explicit opt-in full-sync completion evidence without expanding the operator runtime claim. | v1.6 roots connect Phase 68 through Phase 73 evidence, `docs/parity/threat-model-v1.6.md`, `docs/parity/release-readiness.md`, this catalog, README, runtime guide, and `scripts/check-v1.6-release-boundaries.ts`. | `docs/operator/runtime-guide.md`, `docs/parity/threat-model-v1.6.md`, `docs/parity/release-readiness.md`, `scripts/check-v1.6-release-boundaries.ts`, `scripts/verify.sh` | v1.6 does not add inbound serving, address relay, block serving, transaction relay, compact block relay, production-funds wallet safety, migration apply mode, signed packaging, Windows service support, GUI parity, hosted dashboards, public-network CI, release-blocking live sync, or broad production-node readiness. |
| Phase 75 soak runner and evidence ledger | Operators and reviewers should have exact repo-local soak commands, durable ledger semantics, and scoped non-claim wording. | The `phase75-multi-day-soak-runner-evidence-ledger` surface documents `open-bitcoin soak start`, `resume`, `stop`, and `report`, the datadir-owned run index plus JSONL ledger, event kinds, final outcomes, and support/report projection boundaries. | `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `README.md` | A soak run can prove bounded opt-in full-sync soak behavior, durable resume evidence, or diagnosed blocker evidence; it does not prove inbound serving, relay, production-funds wallet safety, migration apply mode, signed packages, GUI readiness, hosted dashboards, or broad production-node readiness. |
| Phase 76 disk and resource-bound enforcement | Operators should see disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle bounds from one shared status contract before long-running evidence writes begin. | The `phase76-disk-and-resource-bound-enforcement` surface adds `resource_bounds`, 80% warning and 95% stop-required thresholds, soak preflight refusal before ledger mutation, `resource_stop` source evidence, dashboard/status summaries, and compact support-bundle `Resource Bound Evidence`. | `packages/open-bitcoin-node/src/status/resource_bounds.rs`, `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/parity/index.json`, `scripts/check-phase76-resource-bounds.ts` | Phase 76 does not claim unlimited unattended operation, public-network resource stress, hosted monitoring, raw support upload, broad production-node readiness, or full resource-governance parity. |
| Phase 77 corruption and lock recovery hardening | Operators should be able to diagnose lock contention, stale-lock evidence, concurrent datadir use, corruption markers, schema mismatches, partial writes, unreadable stores, and backend-open failures with typed recovery guidance. | The `phase77-corruption-and-lock-recovery-hardening` surface records `recovery_evidence`, action classes, causes, compatibility categories, probe-only lock evidence, status/support/dashboard/soak projections, and operator docs. | `packages/open-bitcoin-node/src/recovery.rs`, `packages/open-bitcoin-node/src/storage/lock_probe.rs`, `packages/open-bitcoin-cli/src/operator/status/recovery_evidence.rs`, `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/storage-decision.md`, `docs/parity/index.json`, `scripts/check-phase77-corruption-lock-recovery.ts`, `.planning/phases/77-corruption-and-lock-recovery-hardening/` | Phase 77 is diagnosis and evidence only; automatic destructive repair, lock cleanup, source datadir mutation, process scanning as required evidence, public-network default checks, and production-node readiness remain outside scope. |
| Phase 78 progress guarantees and stall diagnosis | Operators should be able to trust soak progress credit and identify the stalled subsystem without reading renderer-specific prose. | The `phase78-progress-guarantees-stall-diagnosis` surface records `progress_credit`, `last_useful_work`, `last_peer_contribution`, `expected_progress_window`, `no_progress_threshold`, and `stall_diagnosis` across shared status, soak, support, dashboard, live-smoke, docs, and the deterministic Phase 78 checker for PROG-01, PROG-02, PROG-03, and PROG-04. | `packages/open-bitcoin-node/src/status/progress_guarantee.rs`, `packages/open-bitcoin-node/src/sync/progress/guarantee.rs`, `packages/open-bitcoin-node/src/sync/runtime_state.rs`, `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/parity/index.json`, `scripts/check-phase78-progress-guarantees.ts`, `.planning/phases/78-progress-guarantees-and-stall-diagnosis/` | Phase 78 is progress-guarantee and stall-diagnosis evidence only; public-network default checks, multi-day default gates, support-bundle forensics, inbound serving, relay, production-wallet use, migration apply mode, packaging, GUI, hosted dashboards, and production-node readiness remain outside scope. |
| Phase 79 diagnostics and support-bundle forensics | Operators should be able to inspect why a soak or support handoff passed, failed, or remained inconclusive without trusting artifact existence, raw logs, or elapsed time. | The `phase79-diagnostics-support-bundle-forensics` surface records DIAG-01, DIAG-02, DIAG-03, and DIAG-04 through local `support_forensics`, forensic timeline, checkpoint chain, failure narrative, likely cause, evidence basis, next action, confidence, redaction, size bounds, timeline ordering, and cross-surface consistency across JSON and Markdown support artifacts. | `packages/open-bitcoin-cli/src/operator/support/forensics.rs`, `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/parity/index.json`, `.planning/phases/79-diagnostics-and-support-bundle-forensics/` | Phase 79 is local diagnosis and evidence only; inbound serving, relay, production-funds wallet use, migration apply mode, packaging, GUI, hosted dashboards, public-network default checks, multi-day default gates, automatic support-bundle upload, and production-node readiness remain outside scope. |
| Phase 80 opt-in soak UAT and release boundaries | Reviewers should be able to audit VER-05, VER-06, VER-07, and REL-04 through existing roots, deterministic checkers, support schema anchors, and operator docs without a new evidence manifest. | The `v1-7-full-sync-soak-recovery-release-boundaries` surface makes Phase 80 the v1.7 closeout root, keeps the runtime-guide UAT matrix explicit opt-in, makes the release-readiness matrix current, and preserves source breadcrumbs as the Rust traceability mechanism. | `docs/operator/runtime-guide.md`, `docs/parity/release-readiness.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, `docs/parity/deviations-and-unknowns.md`, `docs/parity/source-breadcrumbs.json`, `scripts/check-parity-breadcrumbs.ts`, `scripts/check-phase75-soak-runner.ts`, `scripts/check-phase76-resource-bounds.ts`, `scripts/check-phase77-corruption-lock-recovery.ts`, `scripts/check-phase78-progress-guarantees.ts`, `scripts/check-phase79-diagnostics-support-bundle.ts`, implemented Phase 80 checker `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts`, `scripts/verify.sh`, `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md` | Phase 80 is release-boundary and audit closeout only; it does not add a new evidence manifest, public-network default checks, release-blocking live sync, inbound serving, address relay, block serving, transaction relay, compact block relay, production-funds wallet use, migration apply mode, signed packaging, Windows service support, GUI, hosted dashboards, automatic support-bundle upload, destructive repair, or broad production-node readiness. |
| Phase 82 production claim boundary | Reviewers should be able to audit PROD-01, PROD-02, PROD-03, and PROD-04 as docs/parity metadata boundaries without treating v1.8 as production readiness. | The `v1-8-production-claim-boundary` surface links [`docs/parity/production-claim-boundary.md`](../production-claim-boundary.md), release readiness, the parity index, checklist, deviations register, README, runtime guide, and `scripts/verify.sh` so the support terms, evidence gates, and deferred surfaces are traceable. | `docs/parity/production-claim-boundary.md`, `docs/parity/release-readiness.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/deviations-and-unknowns.md`, `README.md`, `docs/operator/runtime-guide.md`, `scripts/verify.sh` | Production service operation, signed packaging, Windows service integration, hosted dashboards, public-network CI, automatic support-bundle upload, destructive repair, and broad production-node readiness remain deferred. |
| Phase 83 support matrix and issue evidence | Reviewers should be able to audit SUP-01, SUP-02, SUP-03, and SUP-04 without treating catalog pages as alternate support registries. | The `v1-8-support-matrix-issue-evidence` surface classifies source-built install, runtime, service, support-bundle, opt-in public-network, and production-boundary support terms in the canonical support matrix. | [`docs/parity/support-matrix.md`](../support-matrix.md), `docs/parity/index.json`, `docs/parity/checklist.md`, `README.md`, and `docs/operator/runtime-guide.md` | Catalog pages point to the canonical matrix instead of duplicating support rows; production service operation, public-network CI, support upload, destructive repair, and broad production-node readiness remain deferred unless future scoped gates change them. |
| Phase 84 upgrade and rollback policy | Reviewers should be able to audit UPG-01, UPG-02, UPG-03, and UPG-04 without treating source-built rollback guidance as release-channel or repair support. | The `v1-8-upgrade-rollback-policy` surface links [`upgrade-and-rollback-policy.md`](../upgrade-and-rollback-policy.md) as the canonical source-built policy for pre-upgrade evidence, failed-upgrade handling, rollback, backups, and state/schema compatibility. | `docs/parity/upgrade-and-rollback-policy.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `README.md`, `docs/operator/runtime-guide.md`, and `scripts/verify.sh` | package-manager rollback, signed release channels, automatic update behavior, hidden config/service mutation, and destructive repair remain outside the policy. |
| Phase 85 operator runbooks | Reviewers should be able to audit RUN-01, RUN-02, and RUN-03 without treating procedural runbooks as production-readiness support. | The `v1-8-operator-runbooks` surface links the canonical runbook for production-boundary preflight, long-run monitoring, no-progress diagnosis, recovery/stop decisions, redacted support-bundle timeline, and escalation evidence. | `docs/parity/operator-runbooks.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `README.md`, `docs/operator/runtime-guide.md`, and `scripts/verify.sh` | public-network default checks, real service-manager defaults, multi-day default gates, automatic support-bundle upload, destructive repair, and broad production-node readiness remain outside the runbook. |
| Phase 86 service operation expectations | Reviewers should be able to audit SVC-01 and SVC-02 without treating service evidence as production service support. | The `v1-8-service-operation-expectations` surface links the canonical service document for source-built daemon operation, launchd/systemd preview, opt-in real service lifecycle UAT, restart/resume fields, repo-local Cargo/Bazel commands, and production-service non-claims. | `docs/parity/service-operation-expectations.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `README.md`, `docs/operator/runtime-guide.md`, `scripts/check-phase86-service-operation-expectations.ts`, and `scripts/verify.sh` | packaged service distribution, Windows service support, automatic update behavior, production service ownership, uptime guarantees, public-network default checks, real service-manager defaults, multi-day default gates, automatic support-bundle upload, destructive repair, and broad production-node readiness remain outside this surface. |

## Known Gaps

- packaged or signed release installation flows
- Windows service support
- unattended public-mainnet full sync through `open-bitcoind`; Phase 35 adds
  opt-in activation and durable preflight only
- public-network sync as part of the default local verification contract
- public-network CI or release-blocking live sync
- hosted or public dashboard work beyond the local terminal dashboard
- migration apply mode, source-service cutover, source-datadir mutation, and
  external-wallet rewrite or import
- inbound serving, address relay, block serving, transaction relay, compact
  block relay, production-funds wallet claims, GUI, hosted dashboards, and
  broad production-node readiness remain outside the Phase 72 evidence-only
  scope
- automatic destructive repair, lock cleanup, source datadir mutation, process
  scanning as required evidence, public-network default checks, and
  production-node readiness remain outside the Phase 77 diagnosis-only scope
- public-network default checks, multi-day default gates, support-bundle
  forensics, inbound serving, relay, production-wallet use, migration apply
  mode, packaging, GUI, hosted dashboards, and production-node readiness remain
  outside the phase78-progress-guarantees-stall-diagnosis surface
- inbound serving, relay, production-funds wallet use, migration apply mode,
  packaging, GUI, hosted dashboards, public-network default checks, multi-day
  default gates, automatic support-bundle upload, and production-node readiness
  remain outside the phase79-diagnostics-support-bundle-forensics surface
- inbound serving, address relay, block serving, transaction relay, compact
  block relay, production-funds wallet use, migration apply mode, signed
  packaging, Windows service support, GUI, hosted dashboards, public-network
  default checks, public-network CI, release-blocking live sync, automatic
  support-bundle upload, destructive repair, and broad production-node readiness
  remain outside the Phase 80 v1.7 closeout surface
- production service operation, signed packaging, Windows service integration,
  hosted dashboards, public-network CI, automatic support-bundle upload,
  destructive repair, and broad production-node readiness remain deferred under
  the Phase 82 v1.8 boundary until future scoped gates exist

## Follow-Up Triggers

Update this entry when:

- the benchmark report schema adds new required runtime-hardening scenarios
- `scripts/verify.sh` changes the benchmark or release-hardening evidence path
- packaged install or signed release work becomes a shipped surface
- migration apply mode or automated cutover becomes in scope
- hosted or public dashboard work moves into a claimed release surface
