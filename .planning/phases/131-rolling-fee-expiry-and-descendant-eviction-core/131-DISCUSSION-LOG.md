# Phase 131: Rolling Fee, Expiry, and Descendant Eviction Core - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-24
**Phase:** 131-rolling-fee-expiry-and-descendant-eviction-core
**Mode:** Yolo
**Areas discussed:** capacity enforcement, descendant eviction and rolling bump, block-gated decay, expiry cleanup, oracle and performance bounds

---

## Capacity enforcement

| Option | Description | Selected |
|--------|-------------|----------|
| Accounted memory vs `MempoolCapacity`; retire legacy vsize trim | Matches PRESS-01 and Phase 130 handoff | ✓ |
| Keep dual trim (vsize and accounted) during transition | Safer rollout but delays truthful enforcement labels | |
| Keep legacy vsize trim; only report accounted usage | Violates PRESS-01 | |

**User's choice:** [auto] Accounted memory vs `MempoolCapacity`; retire legacy vsize trim (recommended default)
**Notes:** Phase 130 already built the ledger/oracle and left `legacy_vsize` as transitional.

---

## Descendant eviction and rolling bump

| Option | Description | Selected |
|--------|-------------|----------|
| Descendant-score package eviction + bump from actual package + incremental | Knots `TrimToSize` / `trackPackageRemoved` | ✓ |
| Evict single txs only; bump from individual feerate | Simpler but breaks package-score parity | |
| Evict packages but defer rolling bump to later phase | Leaves PRESS-02 incomplete | |

**User's choice:** [auto] Descendant-score package eviction + bump from actual package + incremental (recommended default)
**Notes:** Existing `descendant_score` selection and Pressure lifecycle facts are reused.

---

## Block-gated decay

| Option | Description | Selected |
|--------|-------------|----------|
| Block-gated 12h/6h/3h occupancy half-lives + Knots rounding | PRESS-03 / `GetMinFee` | ✓ |
| Continuous wall-clock decay without block gate | Diverges from Knots | |
| Decay only on explicit operator/RPC poll | Underspecifies connect-time behavior | |

**User's choice:** [auto] Block-gated 12h/6h/3h occupancy half-lives + Knots rounding (recommended default)
**Notes:** Pure core consumes `BlockLifecycleContext.connected_at`; shell never lets mempool read clocks.

---

## Expiry cleanup

| Option | Description | Selected |
|--------|-------------|----------|
| Pure expire API with injected time; Expiry cause + descendant cleanup via authority | Matches FEEP time model and PRESS-04 | ✓ |
| Shell-only expiry that mutates entries without lifecycle deltas | Breaks typed lifecycle contract | |
| Defer expiry until Phase 136 maintenance | Leaves PRESS-04 incomplete | |

**User's choice:** [auto] Pure expire API with injected time; Expiry cause + descendant cleanup via authority (recommended default)
**Notes:** `LegacyUnknown` acceptance times must not be invented.

---

## Oracle and performance bounds

| Option | Description | Selected |
|--------|-------------|----------|
| Hermetic recomputation-oracle agreement + documented perf thresholds in default verifier | PRESS-05 | ✓ |
| Manual soak only; no verifier gates | Weak regression protection | |
| Public-network soak gates | Violates deterministic verifier policy | |

**User's choice:** [auto] Hermetic recomputation-oracle agreement + documented perf thresholds in default verifier (recommended default)
**Notes:** Threshold magnitudes left to Claude's discretion if documented and hermetic.

---

## Claude's Discretion

- Internal module layout for trim/bump/decay/expiry helpers
- Representation of block-since-last-bump / last-update state
- Exact pressure-context threading shape
- Exact performance threshold numbers
- Temporary test inject seam for rolling fee until internal ownership is complete

## Deferred Ideas

- Package admission / TRUC exceptions → Phase 132
- Package-aware download bridge → Phase 133
- Full cross-cache lifecycle projection → Phase 134
- Durable checkpoint/recovery including rolling fee → Phase 135
- Broader maintenance/transport receipts → Phase 136
- Broader RPC/operator evidence expansion → Phase 137
