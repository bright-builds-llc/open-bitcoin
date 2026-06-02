# Phase 54: Peer Compatibility Baseline and Diagnostic Harness - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-02T20:32:15.521Z
**Phase:** 54-Peer Compatibility Baseline and Diagnostic Harness
**Mode:** Yolo
**Areas discussed:** Harness shape, baseline comparison, diagnostics, integration and scope

---

## Harness Shape

| Option | Description | Selected |
|--------|-------------|----------|
| Hermetic transcript harness | Use scripted peer transcripts against pure network state; default verification remains deterministic. | yes |
| Public-network smoke harness | Use live manual peers to reproduce failures. | |
| Documentation-only comparison | Capture expected behavior without executable reproduction. | |

**User's choice:** Hermetic transcript harness.
**Notes:** Auto-selected because Phase 54 requires deterministic reproduction and keeps public-network checks outside `bash scripts/verify.sh`.

---

## Baseline Comparison

| Option | Description | Selected |
|--------|-------------|----------|
| Message-order comparison | Compare externally observable early-message order and outcomes against Knots. | yes |
| Source-line comparison | Mirror Knots internals directly. | |
| Broad ecosystem survey | Research current peer behavior across many implementations. | |

**User's choice:** Message-order comparison.
**Notes:** Auto-selected because the project prioritizes externally observable parity and v1.4 explicitly skips broad ecosystem research.

---

## Diagnostics

| Option | Description | Selected |
|--------|-------------|----------|
| Typed outcomes | Report precise variants for version rejection, mismatch, service-bit mismatch, message-order failure, timeout, disconnect, malformed payload, and local configuration failure. | yes |
| Free-form text | Emit unstructured failure descriptions. | |
| Live-smoke-only cause strings | Reuse only the current live-smoke no-progress cause labels. | |

**User's choice:** Typed outcomes.
**Notes:** Auto-selected because COMPAT-04 requires distinct operator-facing diagnostics and Phase 55 needs structured causes.

---

## Integration and Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Pure network module first | Add the reusable transcript/report core in `open-bitcoin-network`; expose operator commands only if needed. | yes |
| CLI-first harness | Start with a command-line surface before proving the core. | |
| Daemon-live harness | Tie reproduction directly to `open-bitcoind` runtime behavior. | |

**User's choice:** Pure network module first.
**Notes:** Auto-selected to preserve functional-core boundaries and keep this phase from drifting into Phase 55 live-peer fixes.

---

## the agent's Discretion

- Exact module names and report schema can be chosen during planning as long as typed diagnostics, deterministic tests, and parity breadcrumbs are preserved.

## Deferred Ideas

- Live peer compatibility fixes, header convergence, block progress, restart/resume evidence, and release-boundary documentation remain in later v1.4 phases.
