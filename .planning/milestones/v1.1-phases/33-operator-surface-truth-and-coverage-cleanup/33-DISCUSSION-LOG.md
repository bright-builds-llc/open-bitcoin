# Phase 33: Operator Surface Truth and Coverage Cleanup - Discussion Log

> **Audit trail only.** Do not use as input to planning or execution agents.
> Decisions are captured in `33-CONTEXT.md`; this log preserves the alternatives
> considered during the yolo discuss pass.

**Date:** 2026-04-30
**Phase:** 33-operator-surface-truth-and-coverage-cleanup
**Mode:** Yolo
**Areas discussed:** status CLI contract, service preview guidance, dashboard
action coverage, fake RPC stability

---

## Status CLI Contract

| Option | Description | Selected |
|--------|-------------|----------|
| Remove `--watch` | Drop the dead flag so clap/help matches the existing one-shot runtime behavior. | ✓ |
| Implement `--watch` | Add a real watch loop and repeated rendering late in milestone cleanup. | |
| Keep it undocumented | Preserve the dead flag and leave the mismatch in place. | |

**User's choice:** Auto-selected the narrow truthful cleanup: remove the dead
`--watch` surface instead of adding a new runtime mode.
**Notes:** The runtime never branches on `watch`, the docs do not mention it,
and Phase 33 is optional cleanup rather than a feature-expansion phase.

---

## Service Preview Guidance

| Option | Description | Selected |
|--------|-------------|----------|
| Align all hints to preview-by-default | Keep `service install` as the preview command and `--apply` as the mutate switch across errors and unmanaged diagnostics. | ✓ |
| Reintroduce `--dry-run` wording | Change the CLI contract back toward an explicit dry-run flag that does not actually exist. | |
| Add a new alias flag | Introduce `--dry-run` support just to satisfy the stale text. | |

**User's choice:** Auto-selected the existing truthful contract: preview is the
default, so the stale `--dry-run` wording should be removed.
**Notes:** `docs/operator/runtime-guide.md` already describes this correctly, so
the repair should stay in shared service code unless docs drift is found.

---

## Dashboard Action Coverage

| Option | Description | Selected |
|--------|-------------|----------|
| Add app-level dashboard action tests | Exercise the confirmation loop and shared service path through `handle_action()` with hermetic fakes. | ✓ |
| Keep only `action.rs` unit tests | Leave the higher-level interactive path uncovered. | |
| Build a full TTY binary harness | Add a pseudoterminal-style end-to-end dashboard test during optional cleanup. | |

**User's choice:** Auto-selected hermetic app-level coverage as the narrowest
higher-level safety net.
**Notes:** This raises coverage above the current action-state-machine tests
without turning Phase 33 into a terminal-automation project.

---

## Fake RPC Stability

| Option | Description | Selected |
|--------|-------------|----------|
| Harden the fake RPC fixture | Fix readiness or request-handling timing so the existing binary test becomes deterministic. | ✓ |
| Add retries in the test | Hide intermittent failures by rerunning the same assertions. | |
| Accept the flake | Leave the cleanup debt in place and keep relying on lucky reruns. | |

**User's choice:** Auto-selected fixture hardening over retries or debt
acceptance.
**Notes:** The repo-native verification contract should stay deterministic, and
the fake RPC harness is shared by nearby wallet and status binary tests.

---

## Claude's Discretion

- Exact helper extraction for `handle_action()` coverage is flexible as long as
  the tests stay hermetic and clearly cover pending, confirmed, and cancelled
  flows.
- The fake RPC stabilization may land in the fixture, the request reader, or a
  nearby test helper if that is the smallest deterministic fix.

## Deferred Ideas

- A real streaming `status --watch` experience with repeated repainting and
  cancellation semantics.
- Full interactive dashboard operator-binary TTY automation.
- Broader migration detection ownership cleanup beyond Phase 34.
