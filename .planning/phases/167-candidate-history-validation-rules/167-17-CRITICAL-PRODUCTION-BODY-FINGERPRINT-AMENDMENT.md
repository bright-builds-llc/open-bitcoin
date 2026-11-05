---
phase: 167-candidate-history-validation-rules
plan: "17"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T00:38:12Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 17 critical production-body source binding

The renewed Phase123 checker passed 167/167 focused synthetic/live tests, including the first seven reviewed source mutations. Independent review then found three more valid source changes that kept ordered anchors but broke behavior: an early `Ok(Vec::new())` after request membership drops already-accounted at-cap resend, a shadowed all-peer retained set truncates peer membership, and an early `Ok(())` before the unchanged send loop drops nonempty outbound writes. These are genuine checker false positives, not production bugs. Adding only another literal substring ban for each variant would leave the same class of bypass.

ROOT permits a **small, fixed, fail-closed reviewed body fingerprint** for the critical current production functions, paired with the existing source-derived semantic checks, owner qualification, direct call/branch/order/literal checks and adversarial tests. Extract the unique enabled non-test production function body from its exact physical source owner before hashing; refuse missing, duplicate, cfg(test), comment/string or disconnected decoys. Bind expected SHA-256 values to the accepted FINAL51 source frame and record path, source hash, function-body digest and why each body is critical. Do not generate expected hashes from the input under test, auto-refresh on failure, treat a hash alone as proof of correct behavior, or change accepted historical receipts for digest churn.

The bounded critical set is `packages/open-bitcoin-node/src/sync/session.rs::{run_peer_messages, process_peer_message}`, `packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs::{handoff_core, request_core, handoff}`, `packages/open-bitcoin-node/src/sync/block_reconcile.rs::release_unrequested_hashes`, `packages/open-bitcoin-node/src/sync/session/emission_terminal.rs::{send_all_with_cancel, send_peer_emissions}` and `packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs::open_configured_history_runtime`, or the exact physical paths/function names demonstrated by source navigation if a path has moved. Existing exact-body guards for expire/reconcile/settle/record_ignored/finish and registered sender/cleanup remain; do not clone a second generic checker. `normalizeRust` removes whitespace but not literal semantics after masking, so retain explicit raw literal/error/result checks where they matter or use a raw-body digest with a documented literal rationale.

Add the three newly found live source mutants and rerun the seven earlier ones, unchanged positive/refusal controls and the complete synthetic/live corpus. Each mutated production body must fail at an attributable reviewed-source or semantic contract; a parse/setup failure or zero discovery is not mutation credit. Normal formatted owners remain at most628 lines; if a cohesive in-owner helper cannot fit, HOLD for an exact checked split rather than pack tests or edit a production Rust file. Final Plan17 four literal commands, full 21-owner original-before inverse, 1,797 protected current source, independent CORE/Summary QA and later default verifier/coverage/Bazel/Git gates remain required. No new owner, target/cache override, generic exemption, Git action, global CVLD closure or Phase 168 work is granted.
