# Phase 57: Block Download and Connect Progress - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md - this log preserves the
> alternatives considered.

**Date:** 2026-06-03T13:56:54.625Z
**Phase:** 57-block-download-and-connect-progress
**Mode:** Yolo
**Areas discussed:** Block Download Runtime Contract, Block Connect Evidence,
Failure Attribution and No-Credit Paths, Scope Controls

---

## Block Download Runtime Contract

| Option | Description | Selected |
| --- | --- | --- |
| Validated best-chain headers | Request blocks only for accepted headers on the best-chain path and skip speculative or unvalidated data. | Yes |
| Broad inventory-first download | Request any announced block inventory before proving header-chain context. | No |
| the agent's Discretion | Let the executor decide whether validated headers or broad inventory should drive block requests. | No |

**User's choice:** Auto-selected validated best-chain headers.
**Notes:** This preserves Phase 56's accepted-header progress boundary and keeps
Phase 57 scoped to auditable block IBD progress.

---

## Block Connect Evidence

| Option | Description | Selected |
| --- | --- | --- |
| Connected-chain progress | Treat the first non-genesis or checkpoint-adjacent connected block as the primary success signal. | Yes |
| Download-only progress | Treat any saved block body as success even if active chainstate does not advance. | No |
| the agent's Discretion | Let the planner choose which block-progress signal is enough. | No |

**User's choice:** Auto-selected connected-chain progress with separate
downloaded-height evidence.
**Notes:** A stored block can be reported as download evidence, but Phase 57's
success claim needs active chainstate movement.

---

## Failure Attribution and No-Credit Paths

| Option | Description | Selected |
| --- | --- | --- |
| Typed peer-attributed failures | Record missing, notfound, malformed, invalid, duplicate, disconnected, and non-extending responses against the peer without advancing active chainstate. | Yes |
| Aggregate no-progress only | Collapse block failures into a single no-progress status. | No |
| the agent's Discretion | Let implementation decide how specific failure attribution should be. | No |

**User's choice:** Auto-selected typed peer-attributed failures.
**Notes:** This mirrors prior compatibility diagnostics and supports operator
next-action guidance when public peers do not provide usable block data.

---

## Scope Controls

| Option | Description | Selected |
| --- | --- | --- |
| Keep Phase 57 narrow | Implement block download/connect progress only; leave restart/resume and release closeout to Phases 58 and 59. | Yes |
| Pull in restart/resume proof | Add same-datadir interruption and resume evidence now. | No |
| Pull in release closeout | Add support bundle, threat-model, and release-boundary closeout now. | No |

**User's choice:** Auto-selected narrow Phase 57 scope.
**Notes:** The phase remains deterministic by default, with public network checks
kept as opt-in UAT evidence outside `bash scripts/verify.sh`.

## the agent's Discretion

- Internal representation for block progress and first-block evidence may be
  chosen by the planner and executor if it remains additive, typed, and
  truth-aligned.
- Test placement may follow existing module boundaries across sync runtime,
  managed network, RPC/config, and smoke-script checks.

## Deferred Ideas

- Same-datadir restart/resume proof remains Phase 58.
- Support bundle, threat-model update, release-boundary copy, and final operator
  evidence closeout remain Phase 59.
