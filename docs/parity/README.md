# Parity Ledger

This directory tracks how Open Bitcoin relates to the pinned Bitcoin Knots behavioral baseline.

## Purpose

Open Bitcoin targets behavioral parity with Bitcoin Knots
`29.3.knots20260210` for the in-scope headless node and wallet surfaces. The
parity ledger is the current source of truth for README-facing status claims,
release-readiness review, and intentional deviation tracking.

The ledger exists so contributors and reviewers can answer three questions quickly:

1. Which surface is being compared?
2. What is the current status of that surface in Open Bitcoin?
3. Is any difference from Knots intentional, documented, and reviewable?

The current shipped headless v1 surfaces remain marked `done` in
[`index.json`](index.json) and summarized in [`checklist.md`](checklist.md).
The v1.1 drop-in audit and migration slice is tracked through the same ledger,
with cross-cutting evidence in
[`catalog/drop-in-audit-and-migration.md`](catalog/drop-in-audit-and-migration.md).
The current v1.8 production claim boundary is rooted in
[`production-claim-boundary.md`](production-claim-boundary.md),
[`release-readiness.md`](release-readiness.md), this README,
[`index.json`](index.json), [`checklist.md`](checklist.md), and the
`v1-8-production-claim-boundary` checklist surface. Phase 82 defines the
support terms and evidence gates required before a future production full-node
readiness claim. v1.7 remains historical evidence for source-built, explicit
opt-in full-sync soak and recovery hardening; the v1.6, v1.5, v1.4, and v1.3
threat models also remain historical evidence and should not be read as the
current milestone claim.

The current v1.6 closeout evidence remains preserved as historical context in
[`threat-model-v1.6.md`](threat-model-v1.6.md),
[`release-readiness.md`](release-readiness.md), and the
`v1-6-full-sync-completion-release-boundaries` checklist surface.

The current v1.5 closeout evidence remains preserved as historical context in
[`threat-model-v1.5.md`](threat-model-v1.5.md),
[`release-readiness.md`](release-readiness.md), and the
`v1-5-unattended-operation-release-boundaries` checklist surface. The v1.4 and
v1.3 threat models remain historical evidence.

Phase 73 opt-in public-mainnet UAT and deterministic verification evidence is
rooted in [`catalog/p2p.md`](catalog/p2p.md),
[`catalog/chainstate.md`](catalog/chainstate.md),
[`catalog/operator-runtime-release-hardening.md`](catalog/operator-runtime-release-hardening.md),
[`checklist.md`](checklist.md), and `scripts/check-phase73-uat-verification.ts`.
These roots keep UAT, fixtures, compatibility harness reports, support bundles,
live-smoke reports, deterministic checkers, and source breadcrumbs local and
auditable without claiming public-network CI, release-blocking live sync, or
production-node readiness.

Phase 75 multi-day soak evidence is rooted in the
`phase75-multi-day-soak-runner-evidence-ledger` surface. The root links
[`catalog/p2p.md`](catalog/p2p.md), [`catalog/chainstate.md`](catalog/chainstate.md),
[`catalog/operator-runtime-release-hardening.md`](catalog/operator-runtime-release-hardening.md),
[`checklist.md`](checklist.md), the runtime guide, and the status/observability
contracts so bounded opt-in soak commands, durable ledger semantics, support
projections, and non-claim boundaries stay auditable.

Phase 76 disk and resource-bound enforcement is rooted in the
`phase76-disk-and-resource-bound-enforcement` surface. The root links the
runtime guide, status snapshot contract, operator observability contract,
support bundle evidence, soak `resource_stop` reports, and deterministic
`scripts/check-phase76-resource-bounds.ts` checker so disk, file, cache, queue,
peer, in-flight, log, metric, and support-bundle bounds stay auditable without
public-network or raw-artifact verification.

Phase 77 corruption and lock recovery hardening is rooted in the
`phase77-corruption-and-lock-recovery-hardening` surface. The root links
`packages/open-bitcoin-node/src/recovery.rs`,
`packages/open-bitcoin-node/src/storage/lock_probe.rs`,
`packages/open-bitcoin-cli/src/operator/status/recovery_evidence.rs`, the
runtime guide, status and storage architecture contracts,
`scripts/check-phase77-corruption-lock-recovery.ts`, and the Phase 77 planning
directory so REC-05, REC-06, REC-07, and REC-08 stay auditable as diagnosis and
evidence only.

Phase 78 progress guarantees and stall diagnosis are rooted in the
`phase78-progress-guarantees-stall-diagnosis` surface. The root links shared
progress-credit and stall evidence types, deterministic PROG-01, PROG-02,
PROG-03, and PROG-04 tests, the runtime guide, status and observability
contracts, P2P/chainstate/operator-runtime catalogs, and
`scripts/check-phase78-progress-guarantees.ts` so credited progress and
diagnosed stalls stay auditable without adding public-network or multi-day
default verification.

Phase 79 diagnostics and support-bundle forensics are rooted in the
`phase79-diagnostics-support-bundle-forensics` surface for DIAG-01, DIAG-02,
DIAG-03, and DIAG-04. The root links `support_forensics`, forensic timeline,
checkpoint chain, failure narrative, likely cause, evidence basis, next action,
confidence, redaction, size bounds, timeline ordering, and cross-surface consistency evidence across the runtime guide, architecture contracts, parity
catalogs, and Phase 79 summaries. It explicitly excludes inbound serving,
relay, production-funds wallet use, migration apply mode, packaging, GUI,
hosted dashboards, public-network default checks, multi-day default gates,
automatic support-bundle upload, and production-node readiness.

Phase 80 opt-in soak UAT and release boundaries are rooted in the
`v1-7-full-sync-soak-recovery-release-boundaries` surface for VER-05, VER-06,
VER-07, and REL-04. The root links the runtime guide UAT matrix,
release-readiness matrix, machine root, human checklist, parity README,
deviations register, operator-runtime catalog, source breadcrumbs,
deterministic Phase 75 through Phase 80 checkers, and `scripts/verify.sh`.
Source breadcrumbs remain the mechanism for first-party Rust source and test
traceability, via [`source-breadcrumbs.json`](source-breadcrumbs.json) and
[`scripts/check-parity-breadcrumbs.ts`](../../scripts/check-parity-breadcrumbs.ts).
Phase 80 does not add a new evidence manifest; it keeps the v1.7 claim scoped to
source-built, explicit opt-in full-sync soak and recovery hardening.

Phase 82 production claim boundary evidence is rooted in the
`v1-8-production-claim-boundary` surface for PROD-01, PROD-02, PROD-03, and
PROD-04. The root links the canonical
[`production-claim-boundary.md`](production-claim-boundary.md), release
readiness, machine root, human checklist, deviations register, README, runtime
guide, and `scripts/verify.sh`. It defines gates only and does not duplicate
the canonical claim-to-evidence matrix here.

Phase 83 support matrix and issue evidence is rooted in the
`v1-8-support-matrix-issue-evidence` surface for SUP-01, SUP-02, SUP-03, and
SUP-04. The canonical [`support-matrix.md`](support-matrix.md) uses the Phase
82 terms from [`production-claim-boundary.md`](production-claim-boundary.md)
and is the single source for support levels, issue evidence, contributor update
rules, residual risks, and next gates; this README only points to it.

Phase 84 upgrade and rollback policy is rooted in the
`v1-8-upgrade-rollback-policy` surface for UPG-01, UPG-02, UPG-03, and UPG-04.
The canonical [`upgrade-and-rollback-policy.md`](upgrade-and-rollback-policy.md)
is the single source for source-built rollback, backup, schema compatibility,
failed-upgrade evidence, and no-hidden-mutation boundaries; this README only
points to it.

Phase 85 operator runbooks are rooted in the `v1-8-operator-runbooks` surface
for RUN-01, RUN-02, and RUN-03. The canonical
[`operator-runbooks.md`](operator-runbooks.md) is the single source for
production-boundary preflight, long-run monitoring, no-progress diagnosis,
recovery and stop decisions, redacted support-bundle timelines, and escalation
evidence; this README only points to it.

Phase 86 service operation expectations are rooted in the
`v1-8-service-operation-expectations` surface for SVC-01 and SVC-02. The
canonical
[`service-operation-expectations.md`](service-operation-expectations.md) is the
single source for source-built daemon operation, service preview, opt-in real
service lifecycle UAT, restart/resume fields, repo-local Cargo/Bazel commands,
and production-service non-claims; this README only points to it.

Phase 87 release readiness is rooted in the
`v1-8-release-readiness-checklist` surface for REL-01, REL-05, and REL-06. The
canonical checklist lives in
[`release-readiness.md`](release-readiness.md#v18-release-readiness-checklist)
and maps all current v1.8 requirements to canonical evidence, deterministic
verification, UAT or manual evidence, residual risk, and no-claim or next-gate
status; this README only points to it.

Phase 88 deterministic claim guardrails are rooted in the
`v1-8-deterministic-claim-guardrails` surface for REL-02, REL-03, and REL-04.
The v1.8 deterministic claim guardrails prevent overbroad
production-readiness and deferred-surface claims in the curated public
release/operator docs; they do not claim production full-node readiness.

Deferred and suspected follow-up work remains visible in
[`deviations-and-unknowns.md`](deviations-and-unknowns.md).

## Files

- `index.json` is the machine-readable root for parity status, intentional deviations, and catalog entries.
- `source-breadcrumbs.json` maps first-party Rust files to source-level Bitcoin Knots anchors used by parity breadcrumb comments.
- `checklist.md` is the human-readable parity checklist view backed by `index.json`.
- `deviations-and-unknowns.md` summarizes current deviations, deferred surfaces, suspected unknowns, and folded todo risks.
- `benchmarks.md` documents the benchmark groups, Knots mappings, local commands, reports, and non-goals.
- `production-claim-boundary.md` is the current v1.8 production claim boundary and support-term root.
- `support-matrix.md` is the canonical v1.8 support matrix, issue-evidence checklist, contributor update-rule, and residual-risk root.
- `upgrade-and-rollback-policy.md` is the canonical v1.8 source-built upgrade, rollback, backup, and state/schema compatibility root.
- `operator-runbooks.md` is the canonical v1.8 operator runbook for preflight, monitoring, no-progress diagnosis, recovery, support-bundle timelines, and escalation evidence.
- `service-operation-expectations.md` is the canonical v1.8 service operation expectation root.
- `release-readiness.md` is the current v1.8 release-readiness checklist and handoff plus historical milestone inspection checklist.
- `threat-model-v1.6.md` is the historical v1.6 scoped threat model and release-boundary companion for REL-01, REL-02, and REL-03.
- `threat-model-v1.5.md` is the historical v1.5 scoped threat model and release-boundary companion for REL-01, REL-02, REL-03, and REL-04.
- `threat-model-v1.4.md` is the historical v1.4 scoped threat model and release-boundary companion for OBS-01, OBS-02, OBS-03, SEC-01, SEC-02, and SEC-03.
- `threat-model-v1.3.md` is the historical v1.3 scoped threat model and release-boundary companion for PROOF-06, SEC-01, and SEC-02.
- `catalog/README.md` explains the subsystem-level catalog structure used to satisfy `REF-03`.
- `catalog/core-domain-and-serialization.md` tracks domain primitives, serialization, scripts, transactions, blocks, and protocol framing.
- `catalog/consensus-validation.md` tracks consensus validation, script execution, PoW, merkle behavior, and typed validation outcomes.
- `catalog/chainstate.md` tracks chainstate, UTXO, connect/disconnect, reorg, and persistence-boundary behavior.
- `catalog/mempool-policy.md` tracks admission, replacement, accounting, eviction, and policy orchestration.
- `catalog/p2p.md` tracks peer lifecycle, wire handling, sync, and relay behavior.
- `catalog/wallet.md` tracks descriptor wallets, balances, coin selection, signing, and adapter-owned persistence.
- `catalog/rpc-cli-config.md` tracks the supported JSON-RPC, CLI, config, auth, and deferred operator surfaces.
- `catalog/drop-in-audit-and-migration.md` tracks the drop-in audit matrix, migration dry-run scope, and intentional migration differences.
- `catalog/verification-harnesses.md` tracks black-box parity, integration isolation, property tests, and CI report output.

Generated timing outputs live under `packages/target/benchmark-reports` rather than being checked into git.

## Source breadcrumbs

First-party Rust files under `packages/open-bitcoin-*/src` and
`packages/open-bitcoin-*/tests` carry a plain comment block near the top:

```rust
// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/interpreter.cpp
```

The paths are repo-root-relative anchors into the pinned Knots baseline. They
are evidence breadcrumbs, not claims of line-for-line ports. Files with no
direct source anchor use an explicit `none` breadcrumb so the sweep remains
complete and auditable.

Keep `source-breadcrumbs.json` as the source of truth and run:

```sh
bun run scripts/check-parity-breadcrumbs.ts --write
bun run scripts/check-parity-breadcrumbs.ts --check
```

The checker verifies that every in-scope Rust file has exactly one breadcrumb
block and that every Knots path exists. `bash scripts/verify.sh` runs the check
as part of the repo-native verification contract.

VS Code and Cursor do not document raw relative source-comment paths as a
built-in editor link contract. This repo includes a local VS Code-compatible
helper at `.vscode/extensions/open-bitcoin-parity-breadcrumb-links/` that turns
`packages/bitcoin-knots/...` breadcrumb paths into document links when enabled.

## Intentional deviations

Intentional deviations are allowed only when they are explicit. Each deviation should:

- point at the affected surface
- describe the difference from the pinned baseline
- explain why the difference exists
- link to the phase, plan, or commit where it was introduced

If a change affects in-scope behavior and is not yet represented here, treat that as unfinished work rather than an acceptable omission.

## Catalog maintenance

Keep `index.json` as the root index, then add or update human-readable catalog pages under `catalog/` when a phase uncovers:

- a major subsystem boundary that later phases will reuse
- a Knots quirk that downstream code must preserve intentionally
- a known bug or compatibility trap worth tracking explicitly
- a suspected unknown that should stay visible until it is audited

Update this README when the ledger structure, source-of-truth policy, or
top-level catalog list changes.
