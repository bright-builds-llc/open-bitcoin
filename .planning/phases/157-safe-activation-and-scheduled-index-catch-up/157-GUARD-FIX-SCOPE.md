---
phase: 157-safe-activation-and-scheduled-index-catch-up
generated_by: gsd-code-review-fix
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T22:00:00Z
status: verified
finding: WR-02
---

# Review correction: exclude test-only async production evidence

The closeout review demonstrated that `#[cfg(test)]` on the actual async `serve_authoritative_runtime` function remains selectable as production. The current masker recognizes direct `mod`/`fn` declarations but misses async modifiers. T-157-30 remains open until correction and independent re-review.

- [x] Add an independent mutation of the actual async daemon function and capture the complete guard's false-negative as behavioral RED.
- [x] Correct offset-preserving test-only masking or production selection in the existing Phase 157 evidence helper; cover equivalent declaration modifiers without introducing another parser/framework or modifying shared Phase 156 behavior.
- [x] Prove the actual async mutation fails for the intended ordinary-production diagnostic, then run the full Phase 157 live/mutation suite, scoped style/diff checks and independent re-review.

Ownership is restricted to `scripts/check-phase157-index-catch-up/rust-evidence.ts`, `scripts/check-phase157-index-catch-up.test.ts`, this scope's progress and the current `157-REVIEW-FIX.md`. Preserve Plan 10's frozen source and documentation elsewhere. No Rust/runtime behavior, source inventory, dependency or verifier stage changes are authorized. All commits/content staging/push and shared phase state remain root-owned after clean whole-phase verification.

## Iteration 2 progress | 2026-10-05T22:03:49Z

The actual full-guard async mutation produced RED after its positive baseline: the expected ordinary-boundary diagnostic was absent. The modifier-aware existing matcher then earned GREEN with exactly the intended diagnostic. Eleven lexical modifier/visibility controls preserve byte length, all newline positions and retained ordinary body/start/end offsets. The final explicit-file suite passed 568 tests with 1,573 assertions in 48.11s; the live guard, scoped Bun parse/style checks and ordinary diff review passed. Source changes are frozen for root independent re-review; T-157-30 and the third checkbox remain pending that review. See `157-REVIEW-FIX.md` iteration 2 for evidence and scope limits.
