# Phase 61: Resource Bounds and Recovery Taxonomy - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md; this log preserves the
> alternatives considered.

**Date:** 2026-06-06T03:45:06.553Z
**Phase:** 61-resource-bounds-and-recovery-taxonomy
**Mode:** Yolo
**Areas discussed:** Bounded resource envelope, Recovery taxonomy, Operator truth surfaces, Verification posture

---

## Bounded Resource Envelope

| Option | Description | Selected |
|--------|-------------|----------|
| Shared `SyncResourcePressure` contract | Use the existing status type as the resource-bound truth source and extend tests/projections around it. | yes |
| New standalone bounds report | Add a separate report surface for resource limits and long-run pressure. | |
| Documentation-only bounds | Document existing limits without strengthening code-level assertions. | |

**User's choice:** Auto-selected shared `SyncResourcePressure` contract.
**Notes:** This keeps Phase 61 additive and avoids a competing operator truth
surface before Phase 62.

---

## Recovery Taxonomy

| Option | Description | Selected |
|--------|-------------|----------|
| Typed shared recovery categories | Normalize existing storage, peer, stop, runtime, and live-smoke signals into stable operator-facing categories. | yes |
| Renderer-local strings | Let each status/support/docs renderer translate failures independently. | |
| Live-smoke-only taxonomy | Keep typed recovery diagnosis only in opt-in live-smoke evidence. | |

**User's choice:** Auto-selected typed shared recovery categories.
**Notes:** Phase 61 success criteria require consistent typed states across
status, logs, support bundles, and docs.

---

## Operator Truth Surfaces

| Option | Description | Selected |
|--------|-------------|----------|
| Compact shared labels and summaries | Reuse stable category labels and bounded summaries across status, dashboard, RPC, support evidence, logs, metrics, and docs where Phase 61 touches them. | yes |
| Broad renderer rewrite | Rework every operator surface now, including surfaces that Phase 62 owns. | |
| Status-only output | Limit Phase 61 to status JSON and leave support/docs alignment for later. | |

**User's choice:** Auto-selected compact shared labels and summaries.
**Notes:** The selected approach preserves Phase 62 scope while still proving
RR-04 consistency for the touched taxonomy and guidance fields.

---

## Verification Posture

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic targeted tests plus repo verify | Use Rust/Bun fixture checks and `bash scripts/verify.sh`; keep long-run public-network checks opt-in UAT. | yes |
| Add live long-run checks to default verify | Run public-mainnet long-run evidence inside the default verification contract. | |
| Manual-only verification | Document operator review without adding deterministic regression checks. | |

**User's choice:** Auto-selected deterministic targeted tests plus repo verify.
**Notes:** This carries forward the v1.3 through v1.5 boundary that public-network
checks remain opt-in and outside default verification.

---

## the agent's Discretion

- Decide whether a new recovery category enum/helper is justified after
  reading the current status/support/live-smoke code.
- Decide the smallest renderer and support-evidence changes needed to prove
  RR-04 without consuming Phase 62 or Phase 65 scope.
- Keep tests focused on deterministic fixtures and pure/status projection where
  possible.

## Deferred Ideas

- Long-run cross-surface truth expansion belongs to Phase 62.
- Service lifecycle and supervised restart recovery belong to Phase 63 and
  Phase 64.
- v1.5 support bundle collection and compatibility wrapper work belong to
  Phase 65 and Phase 66.
