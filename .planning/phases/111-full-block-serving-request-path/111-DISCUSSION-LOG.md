# Phase 111: Full Block Serving Request Path - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-07-04T14:58:18.000Z
**Phase:** 111-Full Block Serving Request Path
**Mode:** Yolo
**Areas discussed:** Request Routing, Local Block Availability, Resource Governance And Cleanup, Historical And Pruned Boundaries, Evidence And Guardrails

***

## Request Routing

| Option | Description | Selected |
|--------|-------------|----------|
| Gate full/witness `getdata` through Phase 110 policy before storage reads | Uses the existing peer-manager request-pressure path and Phase 110 block-serving gates before serving. | yes |
| Serve directly from `blocks_by_hash` when present | Fast path but makes the cache the serving policy and bypasses eligibility/status evidence. | |
| Defer all block responses | Too conservative for Phase 111's roadmap goal. | |

**User's choice:** Auto-selected the gated full/witness request path as the recommended default.
**Notes:** Compact-block inventory remains bounded and classified but not served until BIP152 phases.

## Local Block Availability

| Option | Description | Selected |
|--------|-------------|----------|
| Require eligible peer, available status, and local validated block data | Proves bounded serving while preserving Phase 110 status and no-optimistic-read rules. | yes |
| Serve any locally cached block | Risks stale, side-chain, unvalidated, or historical archive-node claims. | |
| Require durable block-store abstraction first | Useful later, but Phase 111 can start with a named adapter seam over current local data. | |

**User's choice:** Auto-selected the three-fact availability gate.
**Notes:** Witness block serving must be backed by codec/regression evidence that witness data is preserved.

## Resource Governance And Cleanup

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse existing request, queue, in-flight, and cleanup limits | Keeps Phase 111 aligned with GOV-01 and Phase 110 resource labels. | yes |
| Add separate block-serving caps | Creates duplicated policy unless existing limits prove insufficient. | |
| Give permissioned peers larger serving queues | Violates bounded serving and permission-scope constraints. | |

**User's choice:** Auto-selected existing resource-governance reuse.
**Notes:** Permissioned/protected peers remain bounded and still produce aggregate evidence.

## Historical And Pruned Boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| Truthful low-cardinality unavailable/pruned/stale/suppressed outcomes | Preserves bounded behavior without leaking prune heights or implying archive-node availability. | yes |
| Serve any historical cache hit as archive evidence | Overclaims Phase 111 scope. | |
| Hide all unavailable cases as generic missing | Too little evidence for GOV-05 and operator diagnosis. | |

**User's choice:** Auto-selected truthful bounded outcomes.
**Notes:** Public-network review remains opt-in UAT, not default verifier scope.

## Evidence And Guardrails

| Option | Description | Selected |
|--------|-------------|----------|
| Extend shared status/evidence and deterministic checker only when touched surfaces require it | Keeps evidence auditable and avoids renderer-local flags. | yes |
| Add broad operator rollout now | Belongs to Phase 116 unless Phase 111 needs minimal evidence plumbing. | |
| Skip docs/parity/checkers | Risks unguarded block-serving or archive-node claims. | |

**User's choice:** Auto-selected minimal shared evidence plus checker guardrails when docs/parity/verifier surfaces change.
**Notes:** New Rust files need parity breadcrumbs and source-breadcrumb registry entries.

## the agent's Discretion

- Exact type/helper names, module boundaries, test fixture names, and checker naming are delegated to the planner.
- The planner may choose whether to create a focused block-serving adapter module or keep changes in the existing node network inventory module, provided the policy/effect boundary stays clear.

## Deferred Ideas

- BIP152 wire codecs, compact-block response payloads, compact reconstruction, `getblocktxn`, `blocktxn`, fallback/validation handoff, broad operator evidence rollout, package relay, filter serving, public defaults, archive-node claims, production-readiness claims, and production-funds wallet use.
