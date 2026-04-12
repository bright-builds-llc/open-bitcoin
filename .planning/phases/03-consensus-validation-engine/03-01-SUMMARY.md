---
phase: 03-consensus-validation-engine
plan: 01
subsystem: consensus-core
tags: [consensus, hashing, script, pure-core]
provides:
  - pure-core consensus crate
  - repo-owned SHA-256 and txid or wtxid hashing
  - initial deterministic non-signature script evaluator
affects: [consensus, validation, later phase-3 work]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 3-2026-04-11T21-32-30
generated_at: 2026-04-11T23:10:00.000Z
duration: ongoing
completed: 2026-04-11
---

# Phase 3 Plan 01 Summary

Added the first pure-core consensus crate and the deterministic hashing or script foundation it depends on.

## Accomplishments

- Added `open-bitcoin-consensus` and re-exported it through `open-bitcoin-core`.
- Implemented repo-owned SHA-256, txid and wtxid hashing, proof-of-work target decoding, and merkle-root helpers.
- Added a deterministic non-signature script evaluator with strong unit and coverage tests.

## Notes

- This plan slice is complete enough to support later contextual and signature-path work.
- Signature opcodes and witness-program execution are still deferred to later Phase 3 work.
