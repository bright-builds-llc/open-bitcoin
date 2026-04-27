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
Deferred and suspected follow-up work remains visible in
[`deviations-and-unknowns.md`](deviations-and-unknowns.md).

## Files

- `index.json` is the machine-readable root for parity status, intentional deviations, and catalog entries.
- `source-breadcrumbs.json` maps first-party Rust files to source-level Bitcoin Knots anchors used by parity breadcrumb comments.
- `checklist.md` is the human-readable parity checklist view backed by `index.json`.
- `deviations-and-unknowns.md` summarizes current deviations, deferred surfaces, suspected unknowns, and folded todo risks.
- `benchmarks.md` documents the benchmark groups, Knots mappings, local commands, reports, and non-goals.
- `release-readiness.md` is the milestone handoff and reviewer inspection checklist.
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
