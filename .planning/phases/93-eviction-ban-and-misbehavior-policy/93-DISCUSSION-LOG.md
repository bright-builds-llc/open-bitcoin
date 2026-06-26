# Phase 93: Eviction, Ban, and Misbehavior Policy - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-26T13:15:10.369Z
**Phase:** 93 - Eviction, Ban, and Misbehavior Policy
**Mode:** Yolo
**Areas discussed:** Eviction scoring and disconnect reasons, Discourage/ban persistence and unban, Misbehavior accounting and protected peers, Operator evidence and verification

---

## Eviction Scoring And Disconnect Reasons

| Option | Description | Selected |
|--------|-------------|----------|
| Pure deterministic scoring | Score typed peer records from connection class, handshake state, activity, diversity, and permission inputs before runtime side effects. | yes |
| Runtime-local heuristics | Let the listener or socket loop decide candidates directly. | no |
| Defer eviction scoring | Keep only cap rejections and leave eviction to a later phase. | no |

**User's choice:** Auto-selected recommended approach: pure deterministic scoring.
**Notes:** This matches the existing functional-core boundary and keeps Phase 91 protected-peer effects visible.

---

## Discourage, Ban, Expiry, And Unban

| Option | Description | Selected |
|--------|-------------|----------|
| Typed scoped ban state | Model address/subnet scope, reason, source, created time, expiry, active status, and unban outcomes as domain data. | yes |
| Minimal in-memory deny list | Store banned peers as untyped strings with ad hoc expiry checks. | no |
| Docs-only ban boundary | Document that bans are deferred and implement only disconnects. | no |

**User's choice:** Auto-selected recommended approach: typed scoped ban state.
**Notes:** Ban expiry should be evaluated with injected timestamps and should never hide broad implicit bans.

---

## Misbehavior Accounting And Protected Peer Handling

| Option | Description | Selected |
|--------|-------------|----------|
| Bounded violation mapping | Map named protocol violations to observe, disconnect, discourage, ban, or protected/no-action outcomes. | yes |
| Single global score | Use one opaque counter and threshold for every violation. | no |
| Phase 94 only | Defer all violation handling to DoS/resource governance. | no |

**User's choice:** Auto-selected recommended approach: bounded violation mapping.
**Notes:** Phase 93 can handle already-supported protocol violations and address/admission misuse, while Phase 94 keeps broader resource governance.

---

## Operator Evidence And Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Shared status/support evidence | Extend shared inbound status first, then render low-cardinality redacted evidence in CLI/support/metrics/docs. | yes |
| Renderer-local summaries | Add CLI/support text without a shared status contract. | no |
| Hidden internal policy | Keep eviction and bans internal until a later release-boundary phase. | no |

**User's choice:** Auto-selected recommended approach: shared status/support evidence.
**Notes:** Verification stays deterministic through pure tests, synthetic records, fixed docs/checkers, and `bash scripts/verify.sh`.

---

## the agent's Discretion

- Exact module names and score weights.
- Whether the first durable ban store is Fjall-backed or snapshot-backed.
- Exact stable reason labels, provided they remain low-cardinality and documented.
- Whether docs/checker work is one plan or split across implementation and evidence closure.

## Deferred Ideas

- Queue pressure, payload-size governance, slow handshakes, churn limits, reconnect behavior, and broader resource pressure remain Phase 94.
- v1.9 release-boundary closeout and no-claim evidence remain Phase 95.
