---
phase: 167-candidate-history-validation-rules
plan: "15"
type: integration-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-27T20:35:02Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 15 block-context BIP68 rejection parity

The compiled Plan 15 ordinary-history tests reached three real managed block refusals. Open Bitcoin returned transaction-level `PrematureSpend` / `non-BIP68-final`; pinned Knots `ConnectBlock` instead returns block-consensus `bad-txns-nonfinal` for a failed sequence lock (`packages/bitcoin-knots/src/validation.cpp:2929-2931`). Knots uses `non-BIP68-final` for mempool admission at `validation.cpp:974-982`. Open Bitcoin's canonical submission currently exposes the transaction reason unchanged. The frozen Plan 15 expectations are correct and remain unchanged.

Root owns the narrow block-context repair in `packages/open-bitcoin-chainstate/src/engine/apply.rs` and the affected existing regression in `packages/open-bitcoin-chainstate/src/engine/tests/phase167_scripts.rs`. Convert only the BIP68 sequence-lock error from block application into `ChainstateError::BlockValidation` with `BlockValidationResult::Consensus` and `bad-txns-nonfinal`, preserving the transaction-level reason for mempool and all other transaction failures. Keep the exact accepted-state and coin preservation controls. The Plan 15 executor retains its original nine owner paths and source-derived JSON; its node tests must assert the corrected block refusal and keep all 19 negative cases.

This is a checked source-ownership extension for an externally observable block-rejection defect. Before Root edits, retain current whole bodies and the failed Task 2 logs. After the repair, run a focused chainstate regression and rerun both literal Plan 15 commands on the final source. Capture an amended causal/protected/native frame and scratch inverse that treats Root's two files separately from the original nine. Independent source/evidence review and Root FINAL15 remain required. No prior Plan 23 receipt is rewritten; no staging, commit, push or Phase 168 work is authorized here.
