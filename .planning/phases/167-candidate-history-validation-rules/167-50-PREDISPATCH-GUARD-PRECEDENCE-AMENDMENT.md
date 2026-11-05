---
phase: 167-candidate-history-validation-rules
plan: "50"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-28T04:34:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 50 pre-dispatch error order

The accepted post-FINAL49 release preflight found that `network/action_translation.rs` calls the existing effectless `guard_message_admission` before dispatching CompactBlock to the Plan 50 owned receive path. The published Plan 50 phrase “preserving UnknownPeer precedence” applies **within inputs that pass this pre-dispatch guard**. A malformed genesis or owner-mismatched message can retain its earlier guard error; Plan 50 does not alter that error to `UnknownPeer` and does not gain ownership of `action_translation.rs`.

Within a guard-passing CompactBlock, the Plan 50 owned preflight checks the actual peer before any new header admission, KnownOnly publication, snapshot write, request enrollment or reconstruction. A missing peer returns the established typed UnknownPeer error with no new effect. Disabled or incapable peers remain silent without new header effects. Tests must cover both guard-passing UnknownPeer and the existing guard-first malformed/owner-mismatch error order, using actual managed dispatch rather than an invented direct-call precedence.

This interpretation leaves the 13 direct owner files, all four task and sixteen final commands, selected C API, error categories, source/native evidence and full original inverse unchanged. It grants no unowned caller edit, target/cache override, Git action, global CVLD closure or Phase 168 work. If the actual runtime differs from this order, stop for another checked refinement before source execution.
