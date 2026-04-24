# Parity Ledger

This directory tracks how Open Bitcoin relates to the pinned Bitcoin Knots behavioral baseline.

## Purpose

Open Bitcoin targets behavioral parity with Bitcoin Knots `29.3.knots20260210` for the in-scope node and wallet surfaces. The parity ledger exists so contributors can answer three questions quickly:

1. Which surface is being compared?
2. What is the current status of that surface in Open Bitcoin?
3. Is any difference from Knots intentional, documented, and reviewable?

## Files

- `index.json` is the machine-readable root for parity status, intentional deviations, and catalog entries.
- `checklist.md` is the human-readable parity checklist view backed by `index.json`.
- `deviations-and-unknowns.md` summarizes current deviations, deferred surfaces, suspected unknowns, and folded todo risks.
- `benchmarks.md` documents the benchmark groups, Knots mappings, local commands, reports, and non-goals.
- `catalog/README.md` explains the subsystem-level catalog structure used to satisfy `REF-03`.
- `catalog/core-domain-and-serialization.md` is the Phase 2 seed catalog for major domain and serialization surfaces.
- `catalog/consensus-validation.md` tracks the currently implemented Phase 3 consensus slice and its remaining gaps.

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
