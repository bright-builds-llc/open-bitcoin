# Phase 91: Peer Permissions and Connection Classes - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-25T13:36:55.195Z
**Phase:** 91-Peer Permissions and Connection Classes
**Mode:** Yolo
**Areas discussed:** Permission vocabulary and parsing, connection classes and admission effects, bounded permission effects, operator evidence and redaction, verification and UAT

---

## Permission Vocabulary And Parsing

| Option | Description | Selected |
| --- | --- | --- |
| Knots names with Open Bitcoin-owned config | Reuse Knots permission labels while keeping config under JSONC/Open Bitcoin CLI ownership. | yes |
| Full Knots `-whitelist`/`-whitebind` compatibility | Accept baseline flags as production-compatible config now. | |
| New Open Bitcoin-only names | Avoid Knots vocabulary and invent new labels. | |

**User's choice:** Auto-selected Knots vocabulary with Open Bitcoin-owned config boundaries.
**Notes:** This preserves parity traceability without implying full baseline config compatibility.

## Connection Classes And Admission Effects

| Option | Description | Selected |
| --- | --- | --- |
| Extend Phase 90 pure admission policy | Use permission classes as inputs to reserved/protected admission decisions. | yes |
| Runtime-only accept-loop checks | Add permission branching directly to socket/runtime code. | |
| Defer all admission effects | Parse permissions but leave admission unchanged. | |

**User's choice:** Auto-selected pure admission extension.
**Notes:** This matches Phase 90 functional-core decisions and keeps outbound sync safety explicit.

## Bounded Permission Effects

| Option | Description | Selected |
| --- | --- | --- |
| Enable only v1.9 scoped effects | Admission protection, eviction-policy inputs, address-response inputs, download-serving inputs, and diagnostics. | yes |
| Enable Knots relay permissions now | Turn on relay, forcerelay, mempool, bloomfilter, and compact-filter behavior. | |
| Reject every unsupported permission | Fail all permissions that do not have active behavior today. | |

**User's choice:** Auto-selected bounded active effects with explicit inactive/deferred relay-like permissions.
**Notes:** This satisfies PERM-03 by proving relay-like permissions cannot silently enable deferred behavior.

## Operator Evidence And Redaction

| Option | Description | Selected |
| --- | --- | --- |
| Shared status/support projection | Extend shared inbound evidence with permission labels, inactive effects, and decision reasons. | yes |
| Renderer-local permission summaries | Let CLI/support renderers infer permission details independently. | |
| Raw config/peer dump | Preserve raw permission strings and peer tables for debugging. | |

**User's choice:** Auto-selected shared status/support projection.
**Notes:** This keeps evidence consistent and avoids leaking secrets, raw endpoints, or unbounded peer tables.

## Verification And UAT

| Option | Description | Selected |
| --- | --- | --- |
| Deterministic pure tests plus checker | Cover parser, policies, negative relay safeguards, status/support redaction, and docs. | yes |
| Public-network permission tests | Contact public peers to validate permission behavior. | |
| Manual-only UAT | Rely on operator instructions without deterministic regression coverage. | |

**User's choice:** Auto-selected deterministic local verification.
**Notes:** Default verification must remain public-network-free and use repo-local Cargo/Bazel forms where UAT commands are needed.

## the agent's Discretion

- Exact module names and plan decomposition.
- Whether unsupported relay-like permissions are rejected or parsed as inactive, as long as behavior remains disabled and evidence is stable.
- Whether permission status is a child of `InboundPeerServingStatus` or a clearly owned nested type.

## Deferred Ideas

- Actual address response implementation belongs to Phase 92.
- Actual eviction, ban, discourage, and misbehavior behavior belongs to Phase 93.
- Broad DoS/resource governance belongs to Phase 94.
- Release-boundary closure belongs to Phase 95.
