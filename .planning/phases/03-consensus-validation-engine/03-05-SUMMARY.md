---
phase: 03-consensus-validation-engine
plan: 05
subsystem: contextual-validation
tags: [consensus, context, finality, witness]
provides:
  - explicit transaction and block validation contexts
  - contextual finality, sequence-lock, maturity, witness-commitment, and block-weight checks
affects: [consensus, validation]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 3-2026-04-11T21-32-30
generated_at: 2026-04-12T02:05:30.000Z
duration: ongoing
completed: 2026-04-11
---

# Phase 3 Plan 05 Summary

Added the explicit contextual validation layer that the remaining execution phases build on.

## Accomplishments

- Added `ConsensusParams`, `ScriptVerifyFlags`, `TransactionValidationContext`, `BlockValidationContext`, and `PrecomputedTransactionData`.
- Implemented contextual finality, sequence-lock, coinbase-maturity, witness-commitment, coinbase-height, and block-weight checks.
- Added pure-core tests that keep those contextual branches covered under the repo-native verification gate.
