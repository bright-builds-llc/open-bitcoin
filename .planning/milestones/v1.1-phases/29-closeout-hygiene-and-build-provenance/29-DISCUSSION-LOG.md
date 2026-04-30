---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 29-2026-04-29T14-01-32
generated_at: 2026-04-29T14:01:32Z
---

# Phase 29 Discussion Log

## Prompt

Close the remaining optional post-audit cleanup around build provenance
truthfulness for Bazel-built CLI targets and leave milestone closeout surfaces
internally consistent.

## Auto-Selected Decisions

1. **Fix the non-Cargo provenance path instead of downgrading docs only.**
   The audit called out a real truthfulness gap in Bazel-built CLI status
   output, so the phase should repair the compile-time metadata path rather than
   accept all-`Unavailable` provenance as the final state.
2. **Use Bazel stamping and checked-in repo scripts.**
   A thin workspace-status script plus Rust target env wiring is the smallest
   repo-owned way to inject Git commit metadata without guessing at runtime.
3. **Stamp the shared CLI library target.**
   That keeps the provenance fix on the code that actually owns
   `current_build_provenance()` and avoids binary-specific drift.
4. **Treat Bazel identifiers as truthful build provenance.**
   `TARGET_CPU` and `COMPILATION_MODE` differ from Cargo's `TARGET` and
   `PROFILE`, but they are still honest build-system identifiers and should be
   surfaced instead of hidden.
5. **Add a focused automated Bazel runtime proof.**
   A small checker that runs a Bazel-built `open-bitcoin status --format json`
   provides better regression protection than a one-off manual command in the
   verification report.

## Boundaries Confirmed

- Do not reopen the service log-path repair from Phase 28.
- Do not introduce new requirement IDs or reset already-closed requirements.
- Keep milestone closeout hygiene limited to the artifacts touched by this
  optional cleanup.
