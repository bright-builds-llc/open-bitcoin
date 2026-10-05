---
phase: 157-safe-activation-and-scheduled-index-catch-up
generated_by: gsd-debug
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
status: verified
---

# Completed-ledger snapshot correction

After actual phase closure, the source guard passes on the live checkout, but two positive mutation snapshots fail because they inherit ledger `done` while copying no earned verifier/security reports. The complete rerun is566/568; the two diagnostics identify the missing reports. Production and guard behavior are unchanged.

- [x] Copy available real completion reports into the read-only fixture snapshot, preserving bytes and pre-closure absence behavior.
- [x] Explicitly remove both completion reports in the missing-proof negative and retain independent completion provenance controls.
- [x] Prove the positive baseline and exact async negative, then rerun the complete mutation suite/live/scoped style and independent fixture review before normal commit-hook verification.

Only `scripts/check-phase157-index-catch-up.test.ts` may change. No production guard/lexer, runtime, dependency, verifier bypass, fabricated shipped proof, phase ledger or Git mutation is delegated. Root owns final source/goal fingerprint refresh and saving commit/push.

## Completion review 2026-10-05T23:34:30.308Z

Actual completed-ledger positiveRED failed only on missing proof copies; optional real verifier/security snapshots and explicit missing-proof removal restored568/568 controls,1573assertions,48.64s. Independent one-file re-review is clean. No shipped proof, runtime/guard/lexer behavior or phase ledger was fabricated or bypassed; final normal hook verification remains separate.
