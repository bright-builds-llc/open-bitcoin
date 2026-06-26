# Phase 94: DoS and Resource Governance - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-26T15:47:23.352Z
**Phase:** 94-DoS and Resource Governance
**Mode:** Yolo
**Areas discussed:** Message envelope and payload allocation, Request/queue/backpressure bounds, Timeouts/churn/reconnects, Operator evidence and verification

---

## Message Envelope And Payload Allocation

| Option | Description | Selected |
|--------|-------------|----------|
| Typed pre-allocation resource gate | Validate magic, header, command, payload size, checksum, and malformed payloads before allocation-heavy work. | yes |
| Parser-only errors | Rely on existing parser failures without a resource-governance evidence model. | no |
| Runtime socket filter only | Put rejection logic directly in the accept/read loop. | no |

**User's choice:** Auto-selected recommended approach: typed pre-allocation resource gate.
**Notes:** Existing `ParsedNetworkMessage::decode_wire` and payload decoders already provide a starting seam; Phase 94 should add stable resource labels and evidence without broadening command support.

---

## Request, Queue, And Backpressure Bounds

| Option | Description | Selected |
|--------|-------------|----------|
| Pure request/queue policy | Evaluate per-peer and aggregate read/write queues, inventory/request caps, and backpressure from typed inputs before runtime effects. | yes |
| Adapter-local queue checks | Let socket tasks enforce ad hoc queue limits without shared policy evidence. | no |
| Defer queues to future relay work | Cover payload limits only and leave queue pressure unspecified. | no |

**User's choice:** Auto-selected recommended approach: pure request/queue policy.
**Notes:** The policy should preserve existing caps and request tracking while avoiding transaction relay, mempool propagation, compact blocks, BIP37, compact filters, or broader serving claims.

---

## Timeouts, Churn, Idle Peers, And Reconnects

| Option | Description | Selected |
|--------|-------------|----------|
| Injected-time deterministic policy | Model slow handshakes, idle peers, churn, repeated failures, and reconnect suppression with injected timestamps and stable labels. | yes |
| Runtime sleep-based checks | Use wall-clock sleeps or long-running tests to prove timeout behavior. | no |
| Ban-only reconnect filter | Reuse ban state but skip churn and idle-peer policy. | no |

**User's choice:** Auto-selected recommended approach: injected-time deterministic policy.
**Notes:** Phase 93 ban/discourage evidence should be an input to reconnect suppression; Phase 94 should not duplicate broad ban semantics.

---

## Operator Evidence And Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Shared inbound status first | Extend `InboundPeerServingStatus` and project through RPC/status/support/metrics/logs with bounded labels and next actions. | yes |
| Renderer-local summaries | Add resource text independently in CLI and support renderers. | no |
| Hidden internal policy | Enforce limits without exposing operator-visible pressure evidence. | no |

**User's choice:** Auto-selected recommended approach: shared inbound status first.
**Notes:** Default verification must stay deterministic and public-network-free. Operator UAT, if added, must include repo-local Cargo and Bazel command forms.

---

## the agent's Discretion

- Exact cap values, type names, and module splits.
- Whether to add a new focused `resource`/`peer_resource` module or small extensions to existing modules.
- Whether Phase 94 needs a deterministic checker immediately or only after docs/parity evidence changes.

## Deferred Ideas

- Phase 95 owns final release-boundary and no-claim evidence across v1.9.
- Future milestones own transaction relay, compact block relay, mempool propagation, public inbound defaults, public-network CI, production service packaging, and production full-node readiness.
