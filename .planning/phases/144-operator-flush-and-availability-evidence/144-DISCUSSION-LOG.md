# Phase 144: Operator Flush and Availability Evidence - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-17
**Phase:** 144-operator-flush-and-availability-evidence
**Mode:** Yolo
**Areas discussed:** Shared status contract, Flush/cache-size vocabulary, Recovery and coins best-block, Have-bytes vs do-not, Surface projection

---

## Shared status contract

| Option | Description | Selected |
|--------|-------------|----------|
| Dedicated snapshot field consumed by all surfaces | New `chainstate_durability` / `flush_availability` on `OpenBitcoinStatusSnapshot`; RPC/CLI/dashboard/metrics/logs/support project it | ✓ |
| Fold into `recovery_evidence` | Reuse sync-recovery taxonomy for flush/availability | |
| Fold into `block_relay` | Extend serving/compact counters with flush facts | |
| Invent `getblock` | New stored-block RPC as the have-bytes surface | |

**User's choice:** Dedicated snapshot field consumed by all surfaces
**Notes:** [auto] Mirrors Phases 105/116/137. 143 D-11 forbids inventing getblock. `recovery_evidence` is a different taxonomy.

---

## Flush/cache-size vocabulary

| Option | Description | Selected |
|--------|-------------|----------|
| Project existing `FlushDecisionFacts` | `ok`/`large`/`critical` and `none`/`needed`/`periodic`/`always`/`failed_disk`, plus write-kind and CanFlush readiness | ✓ |
| Recompute thresholds in operator crates | Duplicate LARGE/CRITICAL math at the renderer | |
| Bytes-only occupancy | Omit classified cache-size and only show raw bytes | |

**User's choice:** Project existing `FlushDecisionFacts`
**Notes:** [auto] 140 D-03 locked that Phase 144 reports these fields and must not invent a second derivation.

---

## Recovery and coins best-block

| Option | Description | Selected |
|--------|-------------|----------|
| Distinct recovery labels plus height+hash best-block | `consistent`/`replayed`/`interrupted`/`fail_closed`; coins best-block like existing tip hashes; no coin dumps | ✓ |
| Overload `SyncRecoveryCategory` | Map interrupted flush onto `unclean_shutdown` / `store_corruption` | |
| Hide coins best-block | Status shows only classified labels, never the coins tip | |

**User's choice:** Distinct recovery labels plus height+hash best-block
**Notes:** [auto] ROADMAP requires coins best-block visibility without peer ids or raw coin dumps. Tip hashes already appear on sync status.

---

## Have-bytes vs do-not

| Option | Description | Selected |
|--------|-------------|----------|
| Aggregate counters plus last classification labels | HAVL-03 facts and `available`/`unavailable`; no hash-keyed request log; never `Pruned` | ✓ |
| Per-request hash log | Table of recent block hashes with probe results | |
| Emit `Pruned` for missing historical payload | Reuse reserved prune label | |

**User's choice:** Aggregate counters plus last classification labels
**Notes:** [auto] 143 D-05/D-06/D-10. CSOBS-01 "per-request" means labels, not a request diary.

---

## Surface projection

| Option | Description | Selected |
|--------|-------------|----------|
| Open Bitcoin status + compact TUI rows, eight charts unchanged | `openbitcoinnetworkstatus`/snapshot; CLI lines; dashboard rows; fixed metrics/logs; redacted support; checker | ✓ |
| Expand baseline `getblockchaininfo` | Add Open Bitcoin flush keys onto Knots-shaped RPC | |
| New dashboard chart/panel or web UI | Ninth sparkline or hosted dashboard | |

**User's choice:** Open Bitcoin status + compact TUI rows, eight charts unchanged
**Notes:** [auto] Follow 137 UI-SPEC TUI contract. UI hint is terminal dashboard, not web.

---

## Claude's Discretion

Exact type/module/checker names, CLI line placement, and whether last classification vs counters are split — as long as CONTEXT D-01 through D-23 hold.

## Deferred Ideas

Parity-root and no-claim closeout (Phase 145). getblock product. Prune/archive/assumeutxo/production claims. Hosted web dashboard.
