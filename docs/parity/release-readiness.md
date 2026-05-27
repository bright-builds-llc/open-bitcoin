# Release Readiness

This is the current release-hardening handoff surface for the headless v1.3
Public Mainnet Sync Proof and Node Hardening milestone. It points reviewers at
repo-owned evidence instead of reproducing full phase logs, checking generated
benchmark artifacts into git, or making public-network checks part of default
verification.

## Readiness Verdict

The current v1.3 readiness claim is a source-built, opt-in, local-evidence
public-mainnet sync proof and node-hardening review surface. It covers the
documented live-smoke workflow, durable sync status truth, peer resilience,
resource bounds, durable recovery, redacted support evidence, and reviewer
traceability needed before Phase 50 evidence closeout.

This is not a production-node or production-funds claim. It does not claim
inbound serving, transaction relay, migration apply mode, packaging or signed
installers, hosted/public dashboard operation, GUI parity, or unattended
production-node readiness.

Treat [`docs/parity/index.json`](index.json) as the machine-readable root,
[`docs/parity/checklist.md`](checklist.md) as the human checklist view, and
[`docs/parity/threat-model-v1.3.md`](threat-model-v1.3.md) as the v1.3 scoped
threat model.

Readiness is evidence-based, not timing-threshold based. The blocking local
verification command is:

```bash
bash scripts/verify.sh
```

That command remains deterministic and public-network-free. It includes local
formatting, linting, builds, tests, benchmark smoke evidence, parity breadcrumb
checks, Bazel smoke builds, coverage, panic-site checks, and the deterministic
v1.3 release-boundary assertion.

## Complete Surfaces

[`docs/parity/checklist.md`](checklist.md) records these current review
surfaces as `done`:

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
- `live-mainnet-smoke-closeout`
- `security-analysis-audit`
- `v1-3-threat-model-release-boundaries`

Primary current-cycle evidence:

- [`docs/parity/threat-model-v1.3.md`](threat-model-v1.3.md) records the
  scoped STRIDE register, evidence acceptance criteria, boundary matrix, and
  requirement traceability for PROOF-06, SEC-01, and SEC-02.
- [`docs/operator/runtime-guide.md`](../operator/runtime-guide.md) provides the
  source-built operator workflow, opt-in live-mainnet smoke commands, support
  bundle commands, redaction boundaries, and known limitations.
- [`scripts/run-live-mainnet-smoke.ts`](../../scripts/run-live-mainnet-smoke.ts)
  provides the explicit opt-in live-mainnet evidence flow and writes local JSON
  plus Markdown reports.
- [`docs/architecture/status-snapshot.md`](../architecture/status-snapshot.md)
  defines `OpenBitcoinStatusSnapshot`, the shared status model embedded in
  support evidence.
- [`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md)
  preserves deferred production-adjacent surfaces for review.
- [`scripts/verify.sh`](../../scripts/verify.sh) provides the repo-owned local
  verification contract for the release surface.

## v1.3 Release Claim Boundary Matrix

| Surface | v1.3 Proven Claim | Accepted Evidence | Explicit Non-Claim | Future Gate | Requirements / Phases |
| --- | --- | --- | --- | --- | --- |
| Public-mainnet sync evidence | Source-built, opt-in live evidence can show validated header/block progress, restart/resume progress, or a diagnosed environment/network blocker. | `bash scripts/verify.sh`, live-smoke JSON/Markdown, support evidence JSON/Markdown, `OpenBitcoinStatusSnapshot`, Phase 50 UAT. | v1.3 does not add public-network checks to default verification. | Phase 50 evidence closeout. | PROOF-06, SEC-01, SEC-02, Phase 49, Phase 50 |
| Phase 50 live evidence closeout | Reviewers can accept observed progress or a diagnosed blocker when required evidence is present. | Typed no-progress cause, endpoint outcomes, status snapshots, next operator action, live-smoke report paths. | v1.3 does not treat DNS/TCP reachability or support-bundle existence alone as successful sync proof. | Phase 50 UAT. | PROOF-03, PROOF-04, PROOF-05, SEC-03 |
| Outbound public peer resilience | Existing daemon sync evidence distinguishes failed, waiting, stalled, connected, and useful-contribution peer states. | Phase 42, Phase 43, and Phase 44 summaries; live-smoke endpoint outcomes and peer contribution rows. | v1.3 does not claim inbound serving and address advertisement. | Future PRODNODE-02 phase. | PEER-01 through PEER-04, SEC-01 |
| Runtime resource bounds and durable recovery | Existing status and docs expose bounded runtime caps, separated durable progress, restart recovery, invalid-data rejection, and recovery guidance. | Phase 45 and Phase 46 summaries, runtime guide, status snapshot contract, support evidence. | v1.3 does not claim unattended production-node operation. | Future PRODNODE-01 phase with long-run evidence. | NODE-01 through NODE-05, SEC-01 |
| Operator RPC controls | Local status, sync pause/resume, dashboard, and support commands use the shared status truth surface and documented credential sources. | Runtime guide, `OpenBitcoinStatusSnapshot`, support evidence, Phase 47 summary. | v1.3 does not claim remote hosted administration, public RPC control, or broad ACL management. | Future remote-operator/auth scope. | OBS-01, OBS-02, SEC-01 |
| Redacted support evidence | Operators can generate local redacted support evidence with config paths, status snapshot, store health, redaction metadata, and allowlisted live-smoke summary fields. | `support-evidence.json`, `support-evidence.md`, Phase 48 support summary. | support bundles are local redacted evidence, not release validators or public-mainnet proof by themselves. | Future artifact validator or hosted support design. | OBS-03, OBS-04, SEC-01 |
| Inbound serving and address advertisement | No shipped v1.3 claim. | Deferred-surface docs, parity checklist, threat model boundary matrix. | v1.3 does not claim inbound serving and address advertisement. | Future PRODNODE-02 phase. | SEC-02 |
| Transaction relay and mempool propagation | No shipped v1.3 claim. | Deferred-surface docs, parity checklist, threat model boundary matrix. | v1.3 does not claim transaction relay or mempool propagation behavior. | Future PRODNODE-03 phase. | SEC-02 |
| Production-funds wallet use | No shipped v1.3 claim. | Deferred-surface docs, parity checklist, threat model boundary matrix. | v1.3 does not claim production-funds wallet use. | Future WALPROD-01 threat model and parity evidence. | SEC-02 |
| Migration apply mode and source datadir mutation | No shipped v1.3 claim. | Drop-in audit docs, migration dry-run docs, threat model boundary matrix. | v1.3 does not claim migration apply mode, source-service cutover, or source datadir mutation. | Future MIGAPPLY-01 phase. | SEC-02 |
| Packaging or signed installers | No shipped v1.3 claim. | Source-built install docs and deferred-surface docs. | v1.3 does not claim packaging or signed installer readiness. | Future PKG-01 phase. | SEC-02 |
| Hosted/public dashboard | No shipped v1.3 claim. | Local operator dashboard docs and deferred-surface docs. | v1.3 does not claim a hosted/public dashboard. | Future hosted operations design. | SEC-02 |
| GUI parity | No shipped v1.3 claim. | Headless and terminal-first scope docs. | v1.3 does not claim GUI parity with the reference Qt app. | Future GUI milestone. | SEC-02 |
| Unattended production-node operation | No shipped v1.3 claim. | Deferred-surface docs, runtime guide limitations, threat model boundary matrix. | v1.3 does not claim unattended production-node operation. | Future PRODNODE-01 phase. | SEC-02 |

## Phase 50 Evidence Acceptance Contract

Phase 50 can close through either observed header/block/restart-resume progress
or a diagnosed environment/network blocker.

Observed-progress evidence must include:

- live-smoke JSON and Markdown reports showing header or block progress;
- peer endpoint, source, timestamp, and before/after status snapshots;
- restart/resume evidence from the same datadir when restart/resume is the
  evidence path;
- local support evidence when useful for review context.

Diagnosed-blocker evidence must include:

- typed no-progress cause;
- endpoint outcomes;
- status snapshots;
- next operator action.

Required local commands:

```bash
bash scripts/verify.sh
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet \
  --timeout-seconds=60 --poll-seconds=5 --manual-peer=HOST[:PORT]
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support \
  --include-live-smoke-report=packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support
```

Reviewer artifact paths:

- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json`
- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md`
- `support-evidence.json`
- `support-evidence.md`
- `OpenBitcoinStatusSnapshot`

public-network checks remain opt-in and outside `bash scripts/verify.sh`.
support bundles are local redacted evidence, not release validators, and they
must be reviewed with the live-smoke report, status snapshots, or
diagnosed-blocker evidence.

## Intentional Deferrals

[`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md) is the
current deferral and risk register. [`docs/parity/index.json`](index.json)
records no intentional in-scope external behavior deviations beyond the already
documented migration differences, but it preserves deferred surfaces for
review.

Current v1.3 deferrals include:

- inbound serving and address advertisement
- transaction relay and mempool propagation
- production-funds wallet use
- migration apply mode, source-service cutover, or source datadir mutation
- packaging or signed release installation flows
- Windows service support
- hosted/public dashboard work
- GUI parity
- unattended production-node operation
- public-network sync as part of the default local verification contract
- checked-in live-mainnet report fixtures or timing-threshold release gates

Relevant catalog and audit docs:

- [`docs/parity/catalog/rpc-cli-config.md`](catalog/rpc-cli-config.md) records
  deferred RPC, CLI, config, auth, and operator ergonomics.
- [`docs/parity/catalog/verification-harnesses.md`](catalog/verification-harnesses.md)
  records deferred harness and fuzzing work.
- [`docs/parity/catalog/drop-in-audit-and-migration.md`](catalog/drop-in-audit-and-migration.md)
  records the current dry-run-only migration posture.
- [`docs/parity/catalog/operator-runtime-release-hardening.md`](catalog/operator-runtime-release-hardening.md)
  records packaged-install, Windows-service, hosted-dashboard, and
  optional-public-network boundaries that remain outside the current shipped
  claim.
- [`docs/parity/benchmarks.md`](benchmarks.md) states that benchmark reports are
  audit and trend evidence, not release timing gates.

## Verification Evidence

Use these commands and artifacts to prove the current state:

```bash
bash scripts/verify.sh
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet
bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports
bun scripts/check-benchmark-report.ts --report=packages/target/benchmark-reports/open-bitcoin-bench-smoke.json
```

Evidence links:

- [`scripts/verify.sh`](../../scripts/verify.sh) runs deterministic local
  verification and must not run live public-network sync.
- [`scripts/check-v1.3-release-boundaries.ts`](../../scripts/check-v1.3-release-boundaries.ts)
  checks that Phase 49 docs and parity roots keep PROOF-06, SEC-01, and SEC-02
  linked without public-network access.
- [`scripts/run-live-mainnet-smoke.ts`](../../scripts/run-live-mainnet-smoke.ts)
  launches the explicit live-mainnet review flow, polls durable sync status,
  and writes local JSON plus Markdown evidence reports.
- [`scripts/check-benchmark-report.ts`](../../scripts/check-benchmark-report.ts)
  enforces the smoke report schema, required benchmark groups, required Phase 22
  case ids, and durability metadata.
- [`scripts/check-panic-sites.sh`](../../scripts/check-panic-sites.sh) scans
  first-party production Rust code for unclassified panic-like sites.
- [`docs/parity/checklist.md`](checklist.md) mirrors the checklist root from
  [`docs/parity/index.json`](index.json).

## Security Analysis Audit

Phase 41 is the v1.2 planning-security closeout gate. It reviewed tracked
`*-SECURITY.md` files from active and archived planning directories, active
v1.2 plan threat models, summary threat flags, and residual-risk sections.

Result: the reviewed corpus has `threats_open: 0` and `needs_phase_count: 0`.
The only remaining security-relevant items were deferred product-scope claims:
production-node operation, production-funds wallet use, inbound peer serving,
transaction relay, and packaged-service hardening.

Phase 49 adds the v1.3 scoped threat model in
[`docs/parity/threat-model-v1.3.md`](threat-model-v1.3.md). It does not claim a
formal security certification, and it does not expand runtime behavior.

## Benchmark Evidence

Benchmark smoke evidence is generated under
`packages/target/benchmark-reports` and is intentionally not checked into git.

Reviewer paths:

- `packages/target/benchmark-reports/open-bitcoin-bench-smoke.json`
- `packages/target/benchmark-reports/open-bitcoin-bench-smoke.md`

[`docs/parity/benchmarks.md`](benchmarks.md) records the benchmark groups, local
commands, report schema, and Knots mapping policy. Benchmark reports remain
threshold-free audit and trend evidence.

## Live Mainnet Smoke Evidence

Live mainnet smoke evidence is generated under
`packages/target/live-mainnet-smoke-reports` and is intentionally not checked
into git.

Reviewer paths:

- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json`
- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md`

Live-smoke reports record endpoint outcomes, typed no-progress causes, status
snapshots, peer contribution rows, daemon output tails, and next operator
action. They remain explicit operator evidence, not a default verification
gate.

## Reviewer Inspection Checklist

Before a release decision, inspect:

- [`docs/parity/threat-model-v1.3.md`](threat-model-v1.3.md) for the v1.3
  threat model, release boundary matrix, and requirement traceability.
- [`docs/parity/index.json`](index.json) for machine-readable checklist and
  audit roots.
- [`docs/parity/checklist.md`](checklist.md) for current status, evidence,
  known gaps, and suspected unknowns.
- [`docs/operator/runtime-guide.md`](../operator/runtime-guide.md) for the
  operator-facing workflow.
- [`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md) for
  deferred surfaces and suspected unknowns.
- [`scripts/verify.sh`](../../scripts/verify.sh) and
  [`scripts/check-v1.3-release-boundaries.ts`](../../scripts/check-v1.3-release-boundaries.ts)
  for the deterministic verification contract.
- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json`
  for the generated live-smoke JSON report.
- `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md`
  for the generated live-smoke Markdown report.
- `support-evidence.json` for redacted support evidence.
- `support-evidence.md` for the support evidence human summary.
- [`docs/parity/benchmarks.md`](benchmarks.md) and local benchmark reports when
  reviewing runtime benchmark evidence.
