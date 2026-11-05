---
phase: 167-candidate-history-validation-rules
plan: "14"
type: integration-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-27T18:50:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 14 descendant status diagnostic

The accepted Plan 14 descendant opener maps a later runtime or metadata observation failure to the existing `OperatorRuntimeError::CanonicalObservationUnavailable`. Its shared `Display` implementation in `packages/open-bitcoin-cli/src/operator/runtime.rs` still says `canonical genesis observation unavailable`, which mislabels a descendant failure in CLI stderr.

Root owns only the neutral wording change in `operator/runtime.rs`: `canonical status observation unavailable`. The Plan 14 executor owns a focused assertion in its existing `operator/runtime/support/phase167_tests.rs` test file. The variant and typed `CompatibilityRefused` behavior remain unchanged. This is a diagnostic correction, not a new history authority or capability.

Apply after the current Plan 14 command closes. Re-run the original Plan 14 Task 2 command on the final source, retain the prior run as historical if changed, and independently review the final source frame. No other ownership, source acceptance, Git action or Phase 168 start is granted.
