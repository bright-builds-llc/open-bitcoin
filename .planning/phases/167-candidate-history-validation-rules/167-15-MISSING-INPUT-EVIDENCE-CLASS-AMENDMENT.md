---
phase: 167-candidate-history-validation-rules
plan: "15"
type: evidence-allocation-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-27T20:48:33Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 15 missing-input evidence class

The compiled ordinary-history negative for `missing-input` reaches the actual direct managed `connect_local_block` path and returns `ChainstateError::MissingCoin` with the candidate's absent outpoint. Its frozen `expected_reason: bad-txns-inputs-missingorspent` is the source-derived **public block-rejection** reason from pinned Knots, not the direct Rust error's display text. Open Bitcoin's existing canonical submission maps `MissingCoin` to that public reason and its RPC bridge forwards the mapped reason. The direct-path test must not claim it executed the public RPC mapping.

Within Plan 15's existing `packages/open-bitcoin-node/src/sync/tests/phase167_ordinary.rs` ownership, assert the exact nested `MissingCoin` variant and absent outpoint against the serialized candidate input, retain the frozen public expectation unchanged, and compare accepted tip, work, coins, BASIC and provenance before and after refusal. Keep the other 18 negative outcomes and their source-derived checks. A separate public `submitblock` test is required before claiming the missing-input public reason was executed; source inspection alone is not runtime evidence.

This clarifies the direct and public evidence classes without changing production behavior, source ownership, test commands, source bounds or the global parity obligation. Retain the earlier failed assertion log and rerun the literal Plan 15 Task 2 command on final owned source. No staging, commit, push, source acceptance or Phase 168 work is granted.
