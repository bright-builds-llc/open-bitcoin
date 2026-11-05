---
phase: 167-candidate-history-validation-rules
plan: "16"
type: evidence-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-27T21:58:47Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 16 consumed-flag sensitivity in disposable source

Plan 16 Task 1's `phase167_historical_scripts_consumed_witness_flag_rejects_corrupt_prevout` uses an exact source-bound mainnet 692261 candidate with a one-byte-corrupted previous output witness program in a mechanical snapshot. The selected rules derive `WITNESS`; a pure transaction control refuses under `WITNESS` and accepts under deliberately wrong `NONE`. The selected chainstate kernel stage must refuse with complete state preservation. The existing `candidate_rules.rs` forwarded flag argument is overwritten by `history.rules().script_flags()` at the chainstate apply site, and `candidate_rules::transaction_context` reselects the flags again before script verification. Mutating only the forwarded value or the earlier apply local is inert and earns no sensitivity evidence.

After the normal final Task 1 source and focused test pass, Root may create one ignored disposable source copy under `.local/open-bitcoin-dev/phase167/plan16/root-consumed-flag-mutation-v1/`, excluding Git state and build caches. Capture and hash the original copied `packages/open-bitcoin-chainstate/src/engine/apply.rs` and the complete protected test/source basis. In **only the copied apply file**, set `transaction_context.verify_flags = ScriptVerifyFlags::NONE` after `candidate_rules::transaction_context` returns and immediately before transaction validation; do not edit the live checkout. Run the exact named sensitivity test through the cooperative command-timings wrapper with a distinct isolated Cargo target. Credit the mutant only if the compiled named test fails at the intended script rejection because the corrupted spend is accepted; an unrelated assertion, compiler/setup failure or zero discovery earns no mutation credit. Restore the copied apply body byte-for-byte and prove the scratch inverse/protected source equality. The live source and its normal passing command remain the acceptance basis.

This authorizes no live `apply.rs` ownership, no altered original Plan 16 Task 1/Task 2 argv, no fixture or selected-rule relaxation, no commit/push, and no Phase 168 work. Retain full logs, actual exit codes, source/native digests and independent review. Plan 19 breadcrumbs and Plan 20 full native/coverage/Bazel/security/goal/lifecycle gates remain mandatory.
