# Release Readiness

This is the current release-hardening handoff surface for the headless v1.1
operator runtime. It points reviewers at repo-owned evidence instead of
reproducing full phase logs or checking generated benchmark artifacts into git.

## Readiness Verdict

The current repository is ready for a release-readiness review of the in-scope
v1.1 node, wallet, RPC, CLI, service, dashboard, migration-planning, and
runtime-verification surfaces. Known follow-up work remains explicit in
[`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md).
This readiness claim does not include unattended public-mainnet full sync
through `open-bitcoind`.

Treat [`docs/parity/index.json`](index.json) as the machine-readable root and
[`docs/parity/checklist.md`](checklist.md) as the human checklist view.

Readiness is evidence-based, not timing-threshold based. The blocking local
verification command is:

```bash
bash scripts/verify.sh
```

That command now includes smoke benchmark generation and report validation via
[`scripts/check-benchmark-report.ts`](../../scripts/check-benchmark-report.ts).

## Complete Surfaces

[`docs/parity/checklist.md`](checklist.md) records these checklist surfaces as
`done`:

- `reference-baseline`
- `architecture-workspace`
- `core-serialization`
- `consensus-validation`
- `chainstate`
- `mempool-policy`
- `p2p-networking`
- `wallet`
- `rpc-cli-config`
- `verification-harnesses-fuzzing`
- `drop-in-audit-migration`
- `real-sync-benchmarks`
- `operator-runtime-release-hardening`

The newest completion surfaces are `real-sync-benchmarks` and
`operator-runtime-release-hardening`. Generated benchmark reports remain local
inspection artifacts under `packages/target` rather than checked-in release
gates.

Primary current-cycle evidence:

- [`docs/parity/benchmarks.md`](benchmarks.md) records the expanded runtime
  benchmark contract, report schema, and local benchmark commands.
- [`docs/parity/catalog/operator-runtime-release-hardening.md`](catalog/operator-runtime-release-hardening.md)
  records the Phase 22 audit matrix for verification, runtime benchmarks,
  operator docs, migration boundaries, and release-ledger closeout.
- [`docs/operator/runtime-guide.md`](../operator/runtime-guide.md) provides the
  operator-facing installation, onboarding, service, status, dashboard,
  migration, and real-sync verification guidance.
- [`docs/parity/catalog/drop-in-audit-and-migration.md`](catalog/drop-in-audit-and-migration.md)
  preserves the explicit dry-run migration boundaries that still apply.
- [`scripts/verify.sh`](../../scripts/verify.sh) and
  [`scripts/check-benchmark-report.ts`](../../scripts/check-benchmark-report.ts)
  provide the repo-owned local verification contract for the release surface.

## Intentional Deferrals

[`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md) is the
current deferral and risk register. [`docs/parity/index.json`](index.json)
records no intentional in-scope external behavior deviations, but it preserves
deferred surfaces for review.

Current release-hardening deferrals include:

- packaged or signed release installation flows beyond the source-built path
- Windows service support
- migration apply mode, source-datadir mutation, or automatic cutover
- unattended public-mainnet full sync through `open-bitcoind`
- public-network sync as part of the default local verification gate
- timing-threshold benchmark gates that would pass or fail a release on elapsed
  numbers alone
- hosted or public dashboard work beyond the local terminal dashboard

Relevant catalog and audit docs:

- [`docs/parity/catalog/rpc-cli-config.md`](catalog/rpc-cli-config.md) records
  deferred RPC, CLI, config, auth, and operator ergonomics such as richer send
  flows, peer-info views, multiwallet selection, ACL features, wait-for-daemon,
  daemon supervision, and broader process-control helpers.
- [`docs/parity/catalog/verification-harnesses.md`](catalog/verification-harnesses.md)
  records deferred harness and fuzzing work such as vendored Knots process
  management, upstream Python suite translation, and a dedicated fuzz runtime.
- [`docs/parity/catalog/drop-in-audit-and-migration.md`](catalog/drop-in-audit-and-migration.md)
  records the current dry-run-only migration posture.
- [`docs/parity/catalog/operator-runtime-release-hardening.md`](catalog/operator-runtime-release-hardening.md)
  records the packaged-install, Windows-service, hosted-dashboard, and
  optional-public-network boundaries that remain outside the current shipped
  claim.
- [`docs/parity/benchmarks.md`](benchmarks.md) states that benchmark reports are
  audit and trend evidence, not release timing gates.

## Known Gaps And Unknowns

Current suspected unknown themes from
[`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md):

- Which packaging or signed-release workflow should become the canonical
  install surface once source-built operation is no longer the only path.
- Which future milestone should wire `DurableSyncRuntime` into `open-bitcoind`
  as an operator-ready public-network full-sync flow.
- Whether any public-network sync verification should become optional release
  evidence in a later milestone without expanding the default local gate.
- Deprecated or ambiguous hex acceptance at future user-facing boundaries.
- Serializer parameter contexts that may need explicit typed Rust boundaries as
  disk and networking adapters grow.
- Address-codec, protocol, and witness edge cases that may need more repo-owned
  fixtures.
- Cache-flush, multi-manager, and long-lived runtime policy ownership between
  pure core and shell adapters.
- Future peer governance, discovery, address relay, HD descriptors, multisig,
  PSBT, send, peer-info, and multi-endpoint semantics.
- Future Knots-backed harness strategy: translate upstream functional cases
  into Rust or wrap a managed baseline process.

Folded status:

- AI-agent-friendly CLI affordances remain evidenced through the earlier
  operator CLI phase and were not separately expanded by Phase 22.
- Panic and illegal-state exposure remains guarded by
  [`scripts/check-panic-sites.sh`](../../scripts/check-panic-sites.sh).

## Verification Evidence

Use these commands and artifacts to prove the current state:

```bash
bash scripts/verify.sh
bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports
bun scripts/check-benchmark-report.ts --report=packages/target/benchmark-reports/open-bitcoin-bench-smoke.json
```

Evidence links:

- [`scripts/verify.sh`](../../scripts/verify.sh) runs format, lint, build,
  tests, smoke benchmarks, benchmark-report validation, parity-breadcrumb
  checks, Bazel smoke targets, coverage, and architecture checks, including the
  production panic-site guard.
- [`scripts/check-benchmark-report.ts`](../../scripts/check-benchmark-report.ts)
  enforces the smoke report schema, required benchmark groups, required Phase 22
  case ids, and durability metadata.
- [`scripts/check-panic-sites.sh`](../../scripts/check-panic-sites.sh) scans
  first-party production Rust code for unclassified panic-like sites.
- [`scripts/run-benchmarks.sh`](../../scripts/run-benchmarks.sh) is the
  contributor-facing benchmark wrapper and distinguishes smoke `debug` reports
  from full `release` reports.
- [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) runs
  `bash scripts/verify.sh`, uploads `parity-reports`, and uploads
  `benchmark-reports`.
- [`docs/parity/checklist.md`](checklist.md) mirrors the checklist root from
  [`docs/parity/index.json`](index.json).

## Benchmark Evidence

Benchmark smoke evidence is generated under
`packages/target/benchmark-reports` and is intentionally not checked into git.

Reviewer paths:

- `packages/target/benchmark-reports/open-bitcoin-bench-smoke.json`
- `packages/target/benchmark-reports/open-bitcoin-bench-smoke.md`

[`docs/parity/benchmarks.md`](benchmarks.md) records the eleven benchmark
groups, the expanded runtime-hardening cases, the profile-aware report schema,
and the Knots mapping policy. The default comparison remains mapping-only;
optional Knots JSON or binary paths are report metadata only.

## Reviewer Inspection Checklist

Before a release decision, inspect:

- [`docs/parity/index.json`](index.json) for machine-readable checklist and
  audit roots.
- [`docs/parity/checklist.md`](checklist.md) for current status, evidence,
  known gaps, and suspected unknowns.
- [`docs/operator/runtime-guide.md`](../operator/runtime-guide.md) for the
  operator-facing workflow.
- [`docs/parity/catalog/operator-runtime-release-hardening.md`](catalog/operator-runtime-release-hardening.md)
  for the Phase 22 audit matrix.
- [`docs/parity/catalog/drop-in-audit-and-migration.md`](catalog/drop-in-audit-and-migration.md)
  for the current migration boundaries.
- [`docs/parity/benchmarks.md`](benchmarks.md) for benchmark scope and report
  semantics.
- [`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md) for
  deferred surfaces and suspected unknowns.
- [`scripts/verify.sh`](../../scripts/verify.sh) and
  [`scripts/check-benchmark-report.ts`](../../scripts/check-benchmark-report.ts)
  for the current verification contract.
- `packages/target/benchmark-reports/open-bitcoin-bench-smoke.json` for the
  generated smoke benchmark JSON report.
- `packages/target/benchmark-reports/open-bitcoin-bench-smoke.md` for the
  generated smoke benchmark Markdown report.
