# Release Readiness

This is the milestone handoff surface for the headless v1 parity audit. It is a
repo-local document that points reviewers at existing evidence instead of
reproducing full phase reports or checking generated timing output into git.

## Readiness Verdict

The current repository is ready for a release-readiness review of the in-scope
headless milestone surfaces, with known follow-up work still visible in
[`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md). The
review should treat [`docs/parity/index.json`](index.json) as the
machine-readable root and [`docs/parity/checklist.md`](checklist.md) as the
human checklist view.

Readiness is evidence-based, not timing-threshold based. The blocking local
verification command is:

```bash
bash scripts/verify.sh
```

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

The checklist also records the Phase 10 `benchmarks-audit-readiness` surface as
the remaining item that this plan promotes after benchmark smoke output and this
handoff document exist.

Primary completion evidence:

- Phase 8 operator interface evidence:
  [`.planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md`](../../.planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md)
- Phase 9 harness and property evidence:
  [`.planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md`](../../.planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md)
- Phase 10 benchmark and audit summaries:
  [`10-01-SUMMARY.md`](../../.planning/phases/10-benchmarks-and-audit-readiness/10-01-SUMMARY.md),
  [`10-02-SUMMARY.md`](../../.planning/phases/10-benchmarks-and-audit-readiness/10-02-SUMMARY.md),
  [`10-03-SUMMARY.md`](../../.planning/phases/10-benchmarks-and-audit-readiness/10-03-SUMMARY.md), and
  [`10-04-SUMMARY.md`](../../.planning/phases/10-benchmarks-and-audit-readiness/10-04-SUMMARY.md)

## Intentional Deferrals

[`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md) is the
current deferral and risk register. It records no intentional in-scope external
behavior deviations in [`docs/parity/index.json`](index.json), but it preserves
deferred surfaces for review.

Relevant catalog and audit docs:

- [`docs/parity/catalog/rpc-cli-config.md`](catalog/rpc-cli-config.md) records
  deferred RPC, CLI, config, auth, and operator ergonomics such as richer send
  flows, peer-info views, multiwallet selection, ACL features, wait-for-daemon,
  daemon supervision, and broader process-control helpers.
- [`docs/parity/catalog/verification-harnesses.md`](catalog/verification-harnesses.md)
  records deferred harness and fuzzing work such as vendored Knots process
  management, upstream Python suite translation, and a dedicated fuzz runtime.
- [`docs/parity/catalog/chainstate.md`](catalog/chainstate.md),
  [`docs/parity/catalog/mempool-policy.md`](catalog/mempool-policy.md),
  [`docs/parity/catalog/p2p.md`](catalog/p2p.md), and
  [`docs/parity/catalog/wallet.md`](catalog/wallet.md) preserve subsystem
  follow-up surfaces that are outside the current milestone slice.
- [`docs/parity/benchmarks.md`](benchmarks.md) states that benchmark reports are
  audit and trend evidence, not release timing gates.

This phase did not add a dashboard, GUI, service dependency, broad CLI feature,
or broad panic refactor.

## Known Gaps And Unknowns

Current suspected unknown themes from
[`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md):

- Deprecated or ambiguous hex acceptance at future user-facing boundaries.
- Serializer parameter contexts that may need explicit typed Rust boundaries as
  disk and networking adapters grow.
- Address-codec, protocol, and witness edge cases that may need more
  repo-owned fixtures.
- Cache-flush, multi-manager, and long-lived runtime policy ownership between
  pure core and shell adapters.
- Future peer governance, discovery, address relay, HD descriptors, multisig,
  PSBT, send, peer-info, and multi-endpoint semantics.
- Future Knots-backed harness strategy: translate upstream functional cases
  into Rust or wrap a managed baseline process.

Folded todo risks remain audit notes only:

- AI-agent-friendly CLI affordances are evidenced through Phase 8, not expanded
  in Phase 10.
- Panic and illegal-state exposure should be handled by a separate quality phase
  if the milestone owner wants that risk reduced before wider release.

## Verification Evidence

Use these commands and artifacts to prove the current state:

```bash
bash scripts/verify.sh
bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports
node -e "const fs=require('fs'); const report=JSON.parse(fs.readFileSync('packages/target/benchmark-reports/open-bitcoin-bench-smoke.json','utf8')); if (!Array.isArray(report.groups) || report.groups.length < 7) process.exit(1); console.log(report.groups.length);"
```

Evidence links:

- [`scripts/verify.sh`](../../scripts/verify.sh) runs format, lint, build,
  tests, benchmark smoke output, Bazel smoke targets, coverage, and architecture
  checks.
- [`scripts/run-benchmarks.sh`](../../scripts/run-benchmarks.sh) is the
  contributor-facing benchmark wrapper.
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

[`docs/parity/benchmarks.md`](benchmarks.md) records the seven benchmark groups,
Knots mapping policy, local commands, report names, and non-goals. The default
comparison remains mapping-only; optional Knots JSON or binary paths are report
metadata only.

## Reviewer Inspection Checklist

Before a release decision, inspect:

- [`docs/parity/index.json`](index.json) for machine-readable checklist and audit
  roots.
- [`docs/parity/checklist.md`](checklist.md) for current status, evidence, known
  gaps, and suspected unknowns.
- [`docs/parity/benchmarks.md`](benchmarks.md) for benchmark scope and report
  semantics.
- [`docs/parity/deviations-and-unknowns.md`](deviations-and-unknowns.md) for
  deferred surfaces and folded todo risks.
- `packages/target/benchmark-reports/open-bitcoin-bench-smoke.json` for the
  generated smoke benchmark JSON report.
- `packages/target/benchmark-reports/open-bitcoin-bench-smoke.md` for the
  generated smoke benchmark Markdown report.
- [`.planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md`](../../.planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md)
  for RPC, CLI, and config verification.
- [`.planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md`](../../.planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md)
  for parity harness and property coverage verification.
- Phase 10 execution summaries:
  [`10-01-SUMMARY.md`](../../.planning/phases/10-benchmarks-and-audit-readiness/10-01-SUMMARY.md),
  [`10-02-SUMMARY.md`](../../.planning/phases/10-benchmarks-and-audit-readiness/10-02-SUMMARY.md),
  [`10-03-SUMMARY.md`](../../.planning/phases/10-benchmarks-and-audit-readiness/10-03-SUMMARY.md),
  [`10-04-SUMMARY.md`](../../.planning/phases/10-benchmarks-and-audit-readiness/10-04-SUMMARY.md), and
  the final `10-05-SUMMARY.md` once this plan completes.

## Bookkeeping Notes

- `.planning/STATE.md` frontmatter reports `completed_plans: 72` of
  `total_plans: 73` and `percent: 99`, while the body also shows
  `Progress: ... 100%` before Plan 10-05 is complete. The file also still lists
  older velocity and recent-trend values that do not reflect all completed
  phases. This plan leaves those discrepancies for the deterministic GSD state
  update commands.
- `.planning/ROADMAP.md` contains stale progress rows for several already
  verified phases, including Phase 8 and Phase 9 rows that still show
  `0/... | Not started` even though
  `.planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md` and
  `.planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md` are
  passed verification artifacts.
- `.planning/ROADMAP.md` also shows Phase 07.5 unchecked while later Phase 07.6
  is marked complete. That ordering should be reviewed by milestone bookkeeping
  rather than silently rewritten inside this readiness task.
