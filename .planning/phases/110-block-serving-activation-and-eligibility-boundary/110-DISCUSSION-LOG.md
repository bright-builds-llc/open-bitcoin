# Phase 110: Block Serving Activation and Eligibility Boundary - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in `110-CONTEXT.md` - this log preserves the alternatives considered.

**Date:** 2026-07-04T02:39:48.367Z
**Phase:** 110 - Block Serving Activation and Eligibility Boundary
**Mode:** Yolo
**Areas discussed:** Activation contract, Peer eligibility matrix, Block status classification, Resource governance, Evidence/docs/guardrails

## Activation Contract

| Option | Description | Selected |
| --- | --- | --- |
| Block-serving-specific activation | Keep block serving and compact relay default-off through explicit Open Bitcoin-owned settings, separate from transaction relay activation. | yes |
| Reuse transaction relay activation directly | Reuse the v2.0 relay config and types as the only activation switch. | no |
| Defer activation until serving path | Wait until Phase 111 to model activation. | no |

**Auto-selected choice:** Block-serving-specific activation.
**Notes:** This follows Phase 100 and Phase 107's typed activation pattern while avoiding ambiguous transaction-relay coupling.

## Peer Eligibility Matrix

| Option | Description | Selected |
| --- | --- | --- |
| Pure block-serving eligibility matrix | Model outbound, inbound, manual, protected, and permissioned peers with stable policy labels before runtime effects. | yes |
| Runtime-only checks | Add eligibility checks directly where requests are processed later. | no |
| Permission-only eligibility | Allow permissioned peers to bypass activation/resource checks. | no |

**Auto-selected choice:** Pure block-serving eligibility matrix.
**Notes:** Protected admission is not serving eligibility, and `download` is only a scoped input, not an archive-node or public-serving claim.

## Block Status Classification

| Option | Description | Selected |
| --- | --- | --- |
| Pure status classifier before storage reads | Return typed labels for validated, available, stale, side-chain, pruned, unavailable, unvalidated, unknown, and suppressed. | yes |
| Storage-backed serving decision | Let the later storage adapter decide status while serving. | no |
| Optimistic serving | Serve whenever an inventory hash is known. | no |

**Auto-selected choice:** Pure status classifier before storage reads.
**Notes:** This keeps Phase 110 inside policy boundaries and prevents optimistic serving of unvalidated, stale, side-chain, pruned, or unavailable data.

## Resource Governance

| Option | Description | Selected |
| --- | --- | --- |
| Reuse Phase 94 governance as policy input/output | Make request caps, queue pressure, timeouts, churn, ban/discourage, and cleanup evidence part of the policy boundary. | yes |
| Add serving-specific runtime caps only | Enforce limits later in node runtime without a pure decision model. | no |
| Permissioned-peer capacity bypass | Grant scoped peers extra capacity without aggregate evidence. | no |

**Auto-selected choice:** Reuse Phase 94 governance as policy input/output.
**Notes:** Permissioned and protected peers still count toward resource evidence; tests should use synthetic records and injected timestamps.

## Evidence, Docs, And Guardrails

| Option | Description | Selected |
| --- | --- | --- |
| Shared-status-first evidence and deterministic no-claim checks | Project stable labels through shared evidence contracts and reject public-default, archive-node, filter, package-relay, production, and public-network-CI overclaims. | yes |
| Renderer-local evidence | Add CLI/support text first and backfill shared status later. | no |
| Docs-only guardrails | Rely on prose without deterministic checker coverage. | no |

**Auto-selected choice:** Shared-status-first evidence and deterministic no-claim checks.
**Notes:** Default verification remains `bash scripts/verify.sh` and public-network-free; operator UAT uses repo-local Cargo and Bazel command forms when needed.

## the agent's Discretion

- Exact config key names, Rust type names, module split, status field names, and checker filenames are left to planning.
- Planner should prefer small pure APIs and avoid overloading transaction relay types if a block-serving-specific type makes illegal states clearer.

## Deferred Ideas

- Full block and witness block serving belongs to Phase 111.
- BIP152 codecs belong to Phase 112.
- Compact relay negotiation belongs to Phase 113.
- Compact reconstruction belongs to Phase 114.
- Missing transaction round trip, fallback, and validation handoff belong to Phase 115.
- Operator evidence rollout belongs to Phase 116.
- Parity/UAT/release closeout belongs to Phase 117.
