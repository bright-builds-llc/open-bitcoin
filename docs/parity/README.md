# Parity Ledger

This directory tracks how Open Bitcoin relates to the pinned Bitcoin Knots behavioral baseline.

## Purpose

Open Bitcoin targets behavioral parity with Bitcoin Knots `29.3.knots20260210` for the in-scope node and wallet surfaces. The parity ledger exists so contributors can answer three questions quickly:

1. Which surface is being compared?
2. What is the current status of that surface in Open Bitcoin?
3. Is any difference from Knots intentional, documented, and reviewable?

## Files

- `index.json` is the machine-readable index used to summarize phase status across major surfaces.
- Future subsystem notes in this directory should capture detail that does not belong in the top-level index.

## Intentional deviations

Intentional deviations are allowed only when they are explicit. Each deviation should:

- point at the affected surface
- describe the difference from the pinned baseline
- explain why the difference exists
- link to the phase, plan, or commit where it was introduced

If a change affects in-scope behavior and is not yet represented here, treat that as unfinished work rather than an acceptable omission.
