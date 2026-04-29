---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 32-2026-04-29T22-28-38
generated_at: 2026-04-29T22:28:38.658Z
---

# Phase 32: Benchmark Wrapper List-Mode Hygiene - Context

**Gathered:** 2026-04-29
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Repair the contributor-facing benchmark wrapper so `bash scripts/run-benchmarks.sh
--list` works again without reopening benchmark fidelity, report schema, or
release-hardening scope. The fix must keep one repo-owned benchmark entrypoint,
preserve the current smoke and full execution paths, and add a focused regression
guard that keeps the wrapper internally consistent. It does not add new
benchmark groups, change benchmark report contents, or widen the benchmark
contract beyond the existing bounded local flow.

</domain>

<decisions>
## Implementation Decisions

### Wrapper behavior
- **D-01:** Keep `scripts/run-benchmarks.sh` as the single contributor-facing
  benchmark wrapper. Phase 32 should repair the existing `--list` path, not
  introduce a second benchmark entrypoint.
- **D-02:** Initialize the shared Cargo invocation before the `--list` fast path
  so list mode can delegate to `open-bitcoin-bench --list` without requiring a
  smoke or full mode selection.
- **D-03:** Preserve the current `--list` exclusivity rule. `--list` must still
  reject combinations with run-mode options instead of quietly ignoring them.

### Regression coverage
- **D-04:** Add a cheap repo-native regression guard by exercising
  `bash scripts/run-benchmarks.sh --list` in `bash scripts/verify.sh`. This
  directly covers the shipped wrapper surface without building a larger harness.
- **D-05:** Keep the benchmark binary contract, group registry, and smoke report
  validation unchanged unless the wrapper repair requires a tiny adjacent tweak.

### Scope discipline
- **D-06:** Treat this as optional cleanup. Phase 32 does not reopen any
  requirement IDs in `.planning/REQUIREMENTS.md`.
- **D-07:** Update contributor-facing docs only if the fix changes the truthful
  wrapper contract. If the existing docs already match the intended behavior,
  leave them alone.

### Claude's Discretion
- Helper extraction inside `scripts/run-benchmarks.sh` is flexible as long as
  the script stays small, guard-oriented, and easy to re-run.
- The focused regression may either discard list-mode output during verification
  or leave it visible, provided failures remain obvious.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and audit evidence
- `.planning/ROADMAP.md` - Phase 32 goal, dependency edge, and success criteria.
- `.planning/v1.1-MILESTONE-AUDIT-PHASE-29.md` - benchmark-wrapper UX tech-debt
  note and the exact `cargo_args[@]: unbound variable` failure.
- `.planning/phases/22-real-sync-benchmarks-and-release-hardening/22-CONTEXT.md`
  - original benchmark wrapper and verify-path decisions.
- `.planning/phases/27-operator-runtime-benchmark-fidelity/27-CONTEXT.md` -
  recent benchmark fidelity work that must not be reopened by this cleanup.

### Benchmark wrapper and verification surfaces
- `scripts/run-benchmarks.sh` - current wrapper implementation and failure site.
- `scripts/verify.sh` - repo-native verification contract and benchmark smoke
  invocation.
- `packages/open-bitcoin-bench/src/main.rs` - benchmark binary `--list`,
  `--smoke`, and `--full` command contract.
- `docs/parity/benchmarks.md` - contributor-facing benchmark wrapper contract.

### Repo workflow and standards
- `AGENTS.md` - repo-local verification contract and documentation freshness
  guidance.
- `AGENTS.bright-builds.md` - repo-native verification and thin-script guidance.
- `standards-overrides.md` - local exception ledger (currently no substantive
  overrides recorded).
- [Bright Builds `standards/core/code-shape.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md)
  - early returns and readable script structure.
- [Bright Builds `standards/core/verification.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/verification.md)
  - repo-native verification expectations.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `packages/open-bitcoin-bench/src/main.rs` already treats `--list` as a valid
  top-level command and returns stable registry output without requiring a run
  mode.
- `scripts/run-benchmarks.sh` already uses Bash arrays and explicit option
  parsing, so the bug is a control-flow ordering issue rather than an unsafe
  command-construction problem.
- `bash scripts/verify.sh` already exercises the benchmark wrapper through the
  bounded smoke path and is the natural place for one cheap list-mode regression
  check.

### Established Patterns
- Benchmark wrapper behavior should stay deterministic, offline, and bounded in
- the default local verify path.
- Optional cleanup phases should prefer narrow fixes and avoid reopening larger
  milestone narratives when a small repo-owned verification guard is enough.
- Bash scripts in this repo stay thin and guard-oriented rather than growing
  large inline programs.

### Integration Points
- The defect occurs because `scripts/run-benchmarks.sh` reaches the `--list`
  exec path before the shared `cargo_args` array is initialized.
- The cleanest regression guard is a direct `bash scripts/run-benchmarks.sh
  --list` check in `scripts/verify.sh`, adjacent to the existing smoke-mode
  wrapper invocation.

</code_context>

<specifics>
## Specific Ideas

- Build the shared Cargo command once before the list-mode fast path, then reuse
  it for both list and run flows.
- Keep the `--list cannot be combined with run options` guard exactly as-is so
  the repair only changes the broken control-flow ordering.
- Prefer proving list-mode success by checking the real wrapper surface for
  expected benchmark-group output or at least clean execution, rather than
  adding a second synthetic harness.

</specifics>

<deferred>
## Deferred Ideas

- Any redesign of the benchmark wrapper into a Bun or Rust-owned CLI.
- New benchmark groups, report schema changes, or timing-threshold work.
- Broader benchmark UX improvements beyond the `--list` failure and focused
  regression guard.

</deferred>

---

*Phase: 32-benchmark-wrapper-list-mode-hygiene*
*Context gathered: 2026-04-29*
