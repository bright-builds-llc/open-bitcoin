# Phase 28: Service Log-Path Truth and Operator Docs Alignment - Discussion Log

> **Audit trail only.** Do not use as input to planning or implementation.
> Decisions are captured in `28-CONTEXT.md`; this log preserves the yolo
> recommendation pass.

**Date:** 2026-04-29
**Phase:** 28-service-log-path-truth-and-operator-docs-alignment
**Mode:** Yolo
**Areas discussed:** service log-path contract, installed-definition recovery,
docs alignment

---

## Service Log-Path Contract

| Option | Description | Selected |
|--------|-------------|----------|
| Derive one combined service log file under the selected log directory | Resolve `<log_dir>/open-bitcoin.log` and point both stdout and stderr there on both supported managers. | ✓ |
| Keep manager defaults and surface unavailability only | Leave systemd on journald and only explain the mismatch in status output. | |
| Split stdout and stderr into separate files | Introduce multiple service log files and expand the service snapshot later. | |

**User's choice:** Derive one combined service log file under the selected log
directory.

**Notes:** Auto-selected in yolo mode because it matches the single-path
`ServiceStateSnapshot` contract, aligns launchd and systemd, and fixes the
audit blocker without introducing a broader operator-surface redesign.

---

## Installed-Definition Recovery

| Option | Description | Selected |
|--------|-------------|----------|
| Parse the installed plist or unit file for the effective log path | Recover the path from the same artifact the preview or apply path generated. | ✓ |
| Recompute the path from current operator config only | Assume the current config still matches the installed service definition. | |
| Report no path once install has completed | Keep preview truthful but let status stay lossy. | |

**User's choice:** Parse the installed plist or unit file for the effective log
path.

**Notes:** Auto-selected in yolo mode because status needs to remain truthful if
config and installed service state diverge later.

---

## Docs Alignment

| Option | Description | Selected |
|--------|-------------|----------|
| Document the concrete service log file derived from the selected log directory | Explain the shipped preview, apply, and status behavior in the operator runtime guide. | ✓ |
| Keep the existing abstract "selected log path" wording | Avoid doc churn and rely on code comments and tests only. | |
| Move the explanation into parity docs only | Treat the runtime guide as unchanged and document the nuance elsewhere. | |

**User's choice:** Document the concrete service log file derived from the
selected log directory.

**Notes:** Auto-selected in yolo mode because `VER-07` is explicitly reopened
for the operator-facing docs surface, not just internal parity notes.

---

## Claude's Discretion

- Exact helper names for service-log path derivation and installed-file parsing.
- Exact status-output wording for unavailable log-path reasons, provided the line
  is explicit and platform-backed.

## Deferred Ideas

- Separate stdout and stderr file paths for service-managed logs.
- Broader journald integration or manager-specific log-tail support.
- Any expansion of the shared `open-bitcoin status` service contract beyond this
  blocker fix.
