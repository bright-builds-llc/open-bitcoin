# Phase 121: Block Relay Metrics and Log Runtime Projection - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-14
**Phase:** 121-block-relay-metrics-and-log-runtime-projection
**Mode:** Yolo
**Areas discussed:** Metric provider wiring, Persist omission semantics, Structured log emission path, Verification boundary

---

## Metric Provider Wiring

| Option | Description | Selected |
|--------|-------------|----------|
| Mirror Phase 97 inbound provider on DurableSyncRuntime | Add availability-gated provider; call existing `block_relay_metric_samples` from `persist_metrics` | ✓ |
| Call helpers only from ManagedRpcContext | Emit metrics from RPC context rather than sync persist path | |
| Inline mapping inside persist_metrics | Duplicate counter mapping instead of reusing helpers | |

**User's choice:** [auto] Mirror Phase 97 inbound provider on DurableSyncRuntime (recommended default)
**Notes:** Audit gap and success criteria explicitly name `DurableSyncRuntime::persist_metrics`. Phase 97 is the proven pattern.

---

## Persist Omission Semantics

| Option | Description | Selected |
|--------|-------------|----------|
| Omit samples when status unavailable | Match inbound Phase 97 D-03 — no manufactured zero availability | ✓ |
| Always emit zero-filled samples | Always append nine block-relay kinds even when evidence never projected | |
| Emit only non-zero kinds | Conditional per-kind emission (higher cardinality / harder dashboards) | |

**User's choice:** [auto] Omit samples when status unavailable (recommended default)
**Notes:** Roadmap criterion 1: append when block-relay status is available.

---

## Structured Log Emission Path

| Option | Description | Selected |
|--------|-------------|----------|
| Sync runtime structured-log path with same availability gate | Emit `block_relay_log_record` via `append_structured_record` / `write_summary_logs` sibling | ✓ |
| Event-at-mutation logging only | Log each compact/block serving mutation like Phase 99 peer-policy | |
| Summary-only embedding | Fold into SyncRunSummary::structured_log_records without dedicated helper call | |

**User's choice:** [auto] Sync runtime structured-log path with same availability gate (recommended default)
**Notes:** Keeps metrics/logs coherent on the same runtime tick; reuses Phase 116 helper sanitization.

---

## Verification Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Runtime tests + Phase 121 Bun checker in verify.sh | Prove persist/log wiring beyond helper unit tests; leakage assertions; verifier wiring | ✓ |
| Helper unit tests only | Rely on existing Phase 116 unit coverage | |
| Full public-network UAT required | Require live mainnet evidence in default verify | |

**User's choice:** [auto] Runtime tests + Phase 121 Bun checker in verify.sh (recommended default)
**Notes:** Success criteria require runtime projection proof and no leakage; default verify stays local/public-network-free.

---

## Claude's Discretion

Provider wrapper naming, shared vs twin provider for logs, exact module/test file placement, and checker script naming.

## Deferred Ideas

None
