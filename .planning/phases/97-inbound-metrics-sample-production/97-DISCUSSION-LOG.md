# Phase 97: Inbound Metrics Sample Production - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-28T16:14:18.242Z
**Phase:** 97-inbound-metrics-sample-production
**Mode:** Yolo
**Areas discussed:** Metric source, metric mapping, runtime persistence, operator evidence, verification

---

## Metric Source

| Option | Description | Selected |
|--------|-------------|----------|
| Shared inbound status | Derive metrics from `OpenBitcoinStatusSnapshot.peers.inbound` / `InboundPeerServingStatus`, preserving one source of truth. | yes |
| Separate metric counters | Add a parallel counter accumulator specifically for retained metrics. | |
| Storage-only reconstruction | Infer inbound metric samples from persisted support/log artifacts. | |

**User's choice:** Yolo recommendation selected shared inbound status.
**Notes:** This best matches the existing architecture and avoids duplicate counter state.

## Metric Mapping

| Option | Description | Selected |
|--------|-------------|----------|
| Fixed `MetricKind` mapping | Map every existing inbound metric kind to one aggregate status counter. | yes |
| Dynamic labels | Emit one metric per reason, endpoint, permission class, or peer-policy label. | |
| Partial mapping | Emit only the admission counters needed to clear the narrowest gap. | |

**User's choice:** Yolo recommendation selected fixed `MetricKind` mapping.
**Notes:** Dynamic labels would violate the low-cardinality observability contract.

## Runtime Persistence

| Option | Description | Selected |
|--------|-------------|----------|
| Existing retention append path | Append inbound samples through `FjallNodeStore::append_metric_samples` with sync samples. | yes |
| New inbound metrics store | Create a separate persisted snapshot for inbound samples. | |
| Dashboard-only computation | Keep samples in memory and compute dashboard points without persisted history. | |

**User's choice:** Yolo recommendation selected existing retention append path.
**Notes:** Phase 97 is specifically a retained sample production gap, so dashboard-only computation is insufficient.

## Operator Evidence

| Option | Description | Selected |
|--------|-------------|----------|
| Prove existing surfaces consume retained samples | Use current dashboard/status/support paths and add only necessary evidence. | yes |
| Add new dashboard UI | Build new visual widgets for inbound metrics. | |
| Status-only evidence | Skip dashboard retained history proof. | |

**User's choice:** Yolo recommendation selected proving existing surfaces.
**Notes:** Dashboard labels already cover inbound metric kinds.

## Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic checker plus repo verification | Add a Phase 97 checker and include it in `bash scripts/verify.sh`. | yes |
| Unit tests only | Rely on Rust unit tests without an integration checker. | |
| Manual UAT only | Document operator commands without default verification. | |

**User's choice:** Yolo recommendation selected deterministic checker plus repo verification.
**Notes:** The roadmap success criteria require default verification and a public-network-free proof.

## the agent's Discretion

- Helper/module placement.
- Exact test fixture construction.
- Whether documentation changes are needed beyond checker evidence.

## Deferred Ideas

- Phase 98 owns stale traceability reconciliation.
- Public-network listener exposure, relay behavior, production readiness, and new dashboard UX remain out of scope.
