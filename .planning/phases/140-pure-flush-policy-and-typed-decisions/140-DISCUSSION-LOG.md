# Phase 140: Pure Flush Policy and Typed Decisions - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-01
**Phase:** 140-pure-flush-policy-and-typed-decisions
**Mode:** Yolo
**Areas discussed:** Decision outcome vocabulary, Injected facts vs computed classification, Flush vs Sync mapping, Recovery types and wiring boundary

---

## Decision outcome vocabulary

| Option | Description | Selected |
|--------|-------------|----------|
| Unified enum-first `FlushDecision` | One `decide_flush` returns write `None`/`Flush`/`Sync`, first-class `RefuseDiskSpace`, classified cache-size, and last-flush-reason | ✓ |
| Split helpers | Public `classify_cache_size` plus thin write/empty decision and a separate disk-space predicate | |
| Minimal Pattern 2 sketch | `{ write: Flush \| Sync \| None, empty_cache: bool }` only; leave refusal/reason for later phases | |

**User's choice:** Unified enum-first `FlushDecision` (yolo recommended default)
**Notes:** [auto] Disk-space refusal must be a first-class variant so Phase 142 cannot skip it. `empty_cache` is implied by Flush vs Sync.

---

## Injected facts vs computed classification

| Option | Description | Selected |
|--------|-------------|----------|
| Raw occupancy + injected time/disk facts | Shell injects bytes/limits, `now`, jittered `next_write`, `memory_pressure`, disk-free + entry count; policy classifies and compares | ✓ |
| Pre-classified booleans only | Shell injects `CoinsCacheSizeState`, `periodic_due`, `memory_pressure`, `disk_space_ok` | |
| Hybrid | Policy classifies cache-size; shell injects `periodic_due` and `memory_pressure`; disk as free+required bytes | |

**User's choice:** Raw occupancy + injected time/disk facts (yolo recommended default)
**Notes:** [auto] Inject already-jittered `next_write`, not last-flush plus 50–70 minute bounds. Do not pin 450 MiB dbcache defaults here.

---

## Flush vs Sync mapping

| Option | Description | Selected |
|--------|-------------|----------|
| Lock Knots mapping in `decide_flush` | Always → Flush; Periodic+Large/Critical → Flush; Periodic time-due+Ok → Sync; IfNeeded+Critical/pressure → Flush; IfNeeded+Large → None | ✓ |
| Write vs no-write only | Phase 142 chooses Flush vs Sync | |
| Prefer Sync except shutdown Always | Ignore LARGE/CRITICAL empty-cache rule | |

**User's choice:** Lock Knots mapping in `decide_flush` (yolo recommended default)
**Notes:** [auto] LARGE is Periodic-only. IfNeeded+LARGE without Critical or pressure is None.

---

## Recovery types and wiring boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Flush types + sketched `RecoveryDecision` | I/O-free marker-fact enum; no replay; no persist_progress change | ✓ |
| Flush policy only | Defer all recovery enums to Phase 142 | |
| Also wire manager call sites now | IfNeeded/Periodic/Always against leftover snapshot persist | |

**User's choice:** Flush types + sketched `RecoveryDecision`; no persist wiring (yolo recommended default)
**Notes:** [auto] Wiring flush modes against snapshot persist would be a false durability claim.

---

## Claude's Discretion

- Exact variant names and whether cache-size/reason live on every variant or a shared payload
- `classify_cache_size` visibility
- Injected time representation, as long as core does not call `Instant::now()`

## Deferred Ideas

- Fjall coins, `head_blocks`, leftover-snapshot non-authority — Phase 141
- Manager wiring, jitter resampling, replay vs fail-closed, dbcache defaults — Phase 142
- Honest availability, operator evidence, no-claim guardrails — Phases 143–145
