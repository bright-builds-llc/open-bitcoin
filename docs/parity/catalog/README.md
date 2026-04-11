# Parity Catalog

This directory is the human-readable companion to [`../index.json`](../index.json). It exists to satisfy `REF-03` without turning parity knowledge into scattered TODOs or tribal memory.

## Conventions

- Keep [`../index.json`](../index.json) as the machine-readable root.
- Add one Markdown entry per subsystem-sized surface or phase-sized audit slice.
- For each entry, record:
  - the in-scope features and boundaries being tracked
  - the concrete Knots source files and tests or vectors that anchor the note
  - notable quirks that Open Bitcoin must preserve intentionally
  - confirmed bugs, if any are known
  - suspected unknowns that later phases still need to audit

## Current entries

| Entry | Scope | Phase |
| --- | --- | --- |
| [`core-domain-and-serialization.md`](core-domain-and-serialization.md) | Amounts, hashes, serialization primitives, scripts, transactions, blocks, and protocol framing reused by later phases | 2 |
