---
phase: 167-candidate-history-validation-rules
plan: "15"
type: evidence-extension-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-27T20:51:21Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 15 executed public missing-input reason

The Plan 15 direct managed test now proves a typed `MissingCoin` refusal for the exact source-bound missing outpoint. The frozen `bad-txns-inputs-missingorspent` expectation is the pinned Knots **public** block reason. Source inspection confirms an existing canonical submission mapping, but source inspection alone is not execution evidence for the public result.

Root owns one supplemental test-only source path, `packages/open-bitcoin-rpc/src/context/canonical_history/tests.rs`. The focused test must load the same frozen serialized default-Regtest rows through height 100, submit the frozen `missing-input` candidate through the actual `submitblock` RPC dispatch, assert the exact public JSON reason, and compare accepted coins, progress, BASIC and provenance before and after. It must preserve the permitted KnownOnly header effect and retain the full source bytes; it does not change production logic or the independent corpus.

This path is separate from Plan 15's original nine owners and the two Root-owned BIP68 chainstate files. Run a focused nonzero RPC command through the cooperative Cargo timing wrapper and include its source/native frame and exact outcome in Plan 15's final evidence. The test may pass immediately because the public mapping already exists; do not manufacture a RED. The original Plan 15 Task 1 and Task 2 commands, original-nine inverse, independent reviews, global requirements, full final verifier, normal-hook Git finalization and STOP before Phase 168 remain required.
