---
phase: 167-candidate-history-validation-rules
plan: "49"
type: preflight-integration-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-27T23:50:13Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Preflight consensus test lint before compact source release

A targeted strict `open-bitcoin-consensus` Clippy run after Plan 16 found `clippy::field_reassign_with_default` in the accepted Plan 14 fixture test at `packages/open-bitcoin-consensus/src/chain_spec/tests.rs:562-566`. The [retained diagnostic](../../../.local/open-bitcoin-dev/phase167/plan16/root-legacy-sighash-v1/deferred-clippy-hold.json), SHA-256 `95edc5716b7ea18de4eac35b2c2341a91bb78a658000fa1e4533485b085c9834`, records the failure. The fixture's selected-chain classification assertion remains valid; this is a test construction style issue, not a consensus behavior change.

Root owns only that one existing test file for a narrow initializer rewrite: construct the caller fixture with the same three field values in a `ConsensusParams { ..Default::default() }` expression after selecting the immutable Regtest scope. Preserve all assertions, selected facts and other source bytes. Capture the original whole body, run scoped Rustfmt and strict consensus Clippy plus the named support-scope test through the cooperative timing wrapper, and retain exact logs/outcomes. Independently review the one-file diff and scratch inverse. The prior FINAL14 and FINAL16 receipts remain historical source snapshots; Plan 49 starts only after a fresh source/native rebase that includes this correction.

This repair grants no Plan 49 source lease, compact completion, global CVLD acceptance, Git staging/commit/push or Phase 168 work. Plan 20 still owns the full default verifier, coverage, Bazel, security/goal/lifecycle and final normal-hook Git gate.
