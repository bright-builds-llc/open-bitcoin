# Phase 113: Compact Relay Negotiation and Announcement Policy - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-07-04T22:53:48.000Z
**Phase:** 113-Compact Relay Negotiation and Announcement Policy
**Mode:** Yolo
**Areas discussed:** Negotiation State, Announcement Policy, Scope Isolation, Verification And Parity

---

## Negotiation State

| Option | Description | Selected |
|--------|-------------|----------|
| Typed per-peer compact state | Track capability, supported version, high-bandwidth preference, low-bandwidth preference, and eligibility explicitly. | yes |
| Ad hoc booleans on peer state | Add simple flags as needed while implementing message handling. | |
| Delay negotiation until reconstruction | Skip peer state until mempool reconstruction exists. | |

**User's choice:** Auto-selected typed per-peer compact state.
**Notes:** The selected approach matches prior phase decisions to make policy explicit, pure, and auditable. Unsupported `sendcmpct` versions decode as data but map to stable ineligible/suppressed outcomes in policy.

---

## Announcement Policy

| Option | Description | Selected |
|--------|-------------|----------|
| Multi-gate pure announcement decision | Require activation, negotiation, header/tip context, validated block availability, and resource capacity before compact announcements. | yes |
| Runtime-local decision | Let node runtime decide directly when a block is available. | |
| Always fall back to headers | Avoid compact announcements until reconstruction work exists. | |

**User's choice:** Auto-selected multi-gate pure announcement decision.
**Notes:** The selected approach lets Phase 113 prove `cmpctblock` announcement eligibility without implementing reconstruction, missing transaction scheduling, or validation handoff.

---

## Scope Isolation

| Option | Description | Selected |
|--------|-------------|----------|
| Explicit compact-relay isolation | Keep compact relay separate from transaction relay, package relay, filters, public defaults, and production claims. | yes |
| Reuse transaction relay activation | Treat transaction relay or mempool activation as the compact relay switch. | |
| Permission-led activation | Let download/protected permissions activate compact relay directly. | |

**User's choice:** Auto-selected explicit compact-relay isolation.
**Notes:** Prior phases repeatedly locked default-off activation and no-claim guardrails. Phase 113 should preserve those boundaries and test against accidental coupling.

---

## Verification And Parity

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic unit and policy tests | Cover supported/unsupported `sendcmpct`, high-bandwidth toggles, fallback, default-disabled behavior, and scope isolation locally. | yes |
| Public-network compact relay smoke | Use live peers as the main proof of compact relay negotiation. | |
| Docs-only parity evidence | Rely on parity notes without focused tests. | |

**User's choice:** Auto-selected deterministic unit and policy tests.
**Notes:** Default verification remains public-network-free. New Rust source/test files need parity breadcrumb entries.

---

## Claude's Discretion

- Exact Rust type names and module boundaries for compact negotiation state.
- Exact fallback action names and suppression reason strings, provided they remain stable and low-cardinality.
- Whether compact announcement policy lives in a new compact relay module or an existing peer module, as long as the policy stays pure.

## Deferred Ideas

- Compact-block reconstruction from mempool state belongs to Phase 114.
- Missing transaction round trip, `blocktxn` matching, fallback to full block fetch, cleanup, and validation handoff belong to Phase 115.
- Operator evidence, RPC, CLI, dashboard, metrics, logs, and support rollout belong to Phase 116.
- Parity/UAT/release guardrails belong to Phase 117.
