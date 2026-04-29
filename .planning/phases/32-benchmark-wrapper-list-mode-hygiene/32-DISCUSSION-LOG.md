# Phase 32: Benchmark Wrapper List-Mode Hygiene - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in `32-CONTEXT.md` - this log preserves the
> alternatives considered during the yolo discuss pass.

**Date:** 2026-04-29T22:28:38.658Z
**Phase:** 32-benchmark-wrapper-list-mode-hygiene
**Mode:** Yolo
**Areas discussed:** Wrapper repair path, Regression coverage, Scope control

---

## Wrapper repair path

| Option | Description | Selected |
|--------|-------------|----------|
| Leave list mode broken and rely on smoke mode only | Keep the current script because `--smoke` still passes in `verify.sh`. | |
| Repair the current wrapper ordering so list mode uses the same Cargo entrypoint cleanly | Keep one wrapper and move shared Cargo command construction before the `--list` fast path. | ✓ |
| Replace the wrapper with a new benchmark launcher | Rework the contributor-facing benchmark entrypoint instead of fixing the control-flow bug. | |

**User's choice:** Auto-selected the narrow wrapper-ordering repair.
**Notes:** The defect is an uninitialized-array path inside the existing wrapper,
so replacing the entrypoint would be broader than the optional cleanup warrants.

---

## Regression coverage

| Option | Description | Selected |
|--------|-------------|----------|
| Verification-only one-off command during the phase | Prove list mode once while fixing the bug, but leave no durable automated guard. | |
| Add a cheap repo-native `--list` check to `scripts/verify.sh` | Cover the actual wrapper surface in the existing verification contract without a larger harness. | ✓ |
| Build a separate benchmark-wrapper test harness | Add new dedicated wrapper-test infrastructure just for this cleanup. | |

**User's choice:** Auto-selected the repo-native `--list` regression check.
**Notes:** This keeps future regressions visible while staying trivial compared to
the already-shipped smoke benchmark invocation.

---

## Scope control

| Option | Description | Selected |
|--------|-------------|----------|
| Broaden into benchmark UX or report cleanup | Use Phase 32 to revisit more wrapper or benchmark ergonomics. | |
| Keep the fix limited to list-mode behavior and internal consistency | Close the audit note without reopening report schema, benchmark groups, or fidelity work. | ✓ |
| Move the wrapper into Bun or Rust now | Treat the bug as a trigger for a language-level rewrite. | |

**User's choice:** Auto-selected the narrow cleanup.
**Notes:** The existing docs and benchmark binary contract already describe the
intended list behavior; the issue is implementation drift in the shell wrapper.

---

## Claude's Discretion

- Whether the shared Cargo invocation stays inline or moves into a small helper
  inside `scripts/run-benchmarks.sh`.
- Whether the verify-time list-mode check suppresses normal list output or leaves
  it visible.

## Deferred Ideas

- A future wrapper rewrite into Bun or Rust if the shell surface grows beyond a
  thin orchestration layer.
- Additional benchmark UX work such as richer list formatting or wrapper help
  improvements.
