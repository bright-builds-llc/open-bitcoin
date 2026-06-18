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
The current v1.7 closeout evidence is rooted in
[`release-readiness.md`](release-readiness.md), this README,
[`index.json`](index.json), [`checklist.md`](checklist.md), and the
`v1-7-full-sync-soak-recovery-release-boundaries` checklist surface. Phase 80
opt-in soak UAT and release boundaries are the current v1.7 closeout root. The
v1.6, v1.5, v1.4, and v1.3 threat models remain historical evidence and should
not be read as the current milestone claim.

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

Deferred and suspected follow-up work remains visible in
[`deviations-and-unknowns.md`](deviations-and-unknowns.md).

## Files

- `index.json` is the machine-readable root for parity status, intentional deviations, and catalog entries.
- `source-breadcrumbs.json` maps first-party Rust files to source-level Bitcoin Knots anchors used by parity breadcrumb comments.
- `checklist.md` is the human-readable parity checklist view backed by `index.json`.
- `deviations-and-unknowns.md` summarizes current deviations, deferred surfaces, suspected unknowns, and folded todo risks.
- `benchmarks.md` documents the benchmark groups, Knots mappings, local commands, reports, and non-goals.
- `release-readiness.md` is the current v1.7 milestone handoff and reviewer inspection checklist.
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
