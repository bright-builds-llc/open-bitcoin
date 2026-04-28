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

## Known Gaps

- packaged or signed release installation flows
- Windows service support
- public-network sync as part of the default local verification contract
- hosted or public dashboard work beyond the local terminal dashboard
- migration apply mode, source-service cutover, source-datadir mutation, and
  external-wallet rewrite or import

## Follow-Up Triggers

Update this entry when:

- the benchmark report schema adds new required runtime-hardening scenarios
- `scripts/verify.sh` changes the benchmark or release-hardening evidence path
- packaged install or signed release work becomes a shipped surface
- migration apply mode or automated cutover becomes in scope
- hosted or public dashboard work moves into a claimed release surface
