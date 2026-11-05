---
phase: 167-candidate-history-validation-rules
plan: "14"
type: integration-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-27T17:59:12Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 14 CLI `submitblock` integration closure

Plan 13 added the typed `MethodCall::SubmitBlock` variant. The CLI client's exhaustive request mapping in `packages/open-bitcoin-cli/src/client.rs` lacks that arm, so the first Plan 14 CLI compilation fails with Rust `E0004` before any Plan 14 behavior test can run. The failure is a compiler/setup signal, not a behavioral RED or a Plan 14 source acceptance.

Root owns this narrow cross-plan repair in `packages/open-bitcoin-cli/src/client.rs` and `packages/open-bitcoin-cli/src/client/tests.rs`, separately from the seven Plan 14 owner paths. Map the normalized request to the existing named JSON-RPC parameter envelope using only its public `hexdata`; the optional compatibility dummy has already been discarded by typed normalization. Add a CLI envelope regression for accepted input forms. Preserve the existing RPC method and parser contracts.

After a source-safe window, run a focused nonzero CLI test and then rerun both original Plan 14 commands on final owned source. Retain the original `E0004` output as failed setup evidence. No other source ownership, source acceptance, milestone completion, Git action or Phase 168 start is granted by this amendment.
