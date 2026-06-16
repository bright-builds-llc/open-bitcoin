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

## Follow-Up Triggers

Update this entry when:

- the benchmark report schema adds new required runtime-hardening scenarios
- `scripts/verify.sh` changes the benchmark or release-hardening evidence path
- packaged install or signed release work becomes a shipped surface
- migration apply mode or automated cutover becomes in scope
- hosted or public dashboard work moves into a claimed release surface
