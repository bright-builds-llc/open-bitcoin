# Phase 44: Peer Contribution Attribution - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-25T16:03:34.725Z
**Phase:** 44-Peer Contribution Attribution
**Mode:** YOLO recommended
**Areas discussed:** Attribution semantics, peer state and failure separation, operator evidence surfaces

---

## Attribution Semantics

| Option | Description | Selected |
|--------|-------------|----------|
| Validation-gated counters | Keep existing telemetry path but count useful headers/blocks only after sync validation accepts them. | yes |
| Raw message counters | Count any received headers/block message, even if later rejected. | no |
| New peer scoring system | Introduce reputation/usefulness scores beyond the current phase scope. | no |

**User's choice:** Validation-gated counters.
**Notes:** Auto-selected because the phase goal explicitly says progress should identify peers that contributed validated headers or blocks while avoiding credit to idle peers.

---

## Peer State And Failure Separation

| Option | Description | Selected |
|--------|-------------|----------|
| Separate activity from contribution | Preserve message count, last activity, and failure reason even when useful contribution remains zero. | yes |
| Collapse failures to zero activity | Keep failed outcomes as empty failure records. | no |
| Add broad idle/scoring policy | Rework peer lifecycle classification beyond telemetry attribution. | no |

**User's choice:** Separate activity from contribution.
**Notes:** Auto-selected to satisfy the requirement that failing peers retain last activity and failure reason separately from contributed progress.

---

## Operator Evidence Surfaces

| Option | Description | Selected |
|--------|-------------|----------|
| Extend existing status and live-smoke evidence | Use durable peer telemetry and add final report rows for contribution evidence. | yes |
| Add a new report format | Create a separate contribution report outside the current status/live-smoke flow. | no |
| Defer evidence to public-mainnet UAT only | Wait for live-network proof before adding deterministic surfaces. | no |

**User's choice:** Extend existing status and live-smoke evidence.
**Notes:** Auto-selected because Phase 44 success criteria require data to remain available to live smoke and support evidence flows while default verification stays deterministic.

---

## Agent Discretion

- Exact helper names and additive live-smoke report field names.
- Whether to preserve only existing `headers_received`/`blocks_received` fields
  or add clarifying additive fields, provided existing consumers keep working.

## Deferred Ideas

- Peer scoring, reputation, eviction policy, and long-run peer selection heuristics.
- Runtime resource bounds and single-writer store coordination.
- Final public-mainnet progress evidence.
