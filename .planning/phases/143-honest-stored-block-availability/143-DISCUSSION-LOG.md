# Phase 143: Honest Stored-Block Availability - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-17
**Phase:** 143-honest-stored-block-availability
**Mode:** Yolo
**Areas discussed:** durable_availability probe shape, reserved Pruned label, operator facts versus Phase 144 surfaces, inventory and RPC refuse contract

---

## durable_availability probe shape

| Option | Description | Selected |
|--------|-------------|----------|
| Payload-byte probe | Available only when cache or durable block-store body bytes are present; coins tip and header index cannot authorize serve | ✓ |
| Index-or-tip is enough | Treat header-index membership or coins best-block as Available | |
| Keep caller override | Leave `durable_availability: bool` as a caller-supplied shortcut | |

**User's choice:** Payload-byte probe (recommended default)
**Notes:** [auto] Yolo selected the recommended default. `gate_inventory_for_durable_serving` currently passes `true` unconditionally; that override must die. Classifier stays I/O-free.

---

## Reserved Pruned label

| Option | Description | Selected |
|--------|-------------|----------|
| Reserve Pruned; missing payload is Unavailable | Do not emit Pruned until prune mode actually deleted files; prune is deferred so this phase never emits it | ✓ |
| Keep current Pruned-for-missing-active-body | Continue mapping active-non-tip-missing to Pruned | |
| Remove the Pruned variant now | Delete the enum so later prune work must reintroduce it | |

**User's choice:** Reserve Pruned; missing payload is Unavailable (recommended default)
**Notes:** [auto] Help text and `as_str` must not read as prune-mode. Tests that expect Pruned for missing active bodies flip to Unavailable.

---

## Operator facts versus Phase 144 surfaces

| Option | Description | Selected |
|--------|-------------|----------|
| Typed facts on the classify/report seam | Add `payload_present`, `index_known`, `validated_on_active_chain`; leave full operator rollout to Phase 144 | ✓ |
| Full operator evidence now | Also ship status/CLI/dashboard/metrics/logs/support have-bytes surfaces | |
| Facts only inside comments | Do not expose distinguishable facts this phase | |

**User's choice:** Typed facts on the classify/report seam (recommended default)
**Notes:** [auto] HAVL-03 is the typed-fact contract. CSOBS-01/CSOBS-02 stay Phase 144. Do not invent getblock unless research finds an existing stored-block RPC.

---

## Inventory and RPC refuse contract

| Option | Description | Selected |
|--------|-------------|----------|
| Existing NotFound + Unavailable | Refuse missing payload with current missing-inventory machinery and Unavailable label | ✓ |
| New wire error | Invent a dedicated missing-payload P2P/RPC error | |
| Treat refuse as archive honesty | Document missing-payload refuse as archive-node or public-default serving | |

**User's choice:** Existing NotFound + Unavailable (recommended default)
**Notes:** [auto] Carry forward 110/111 no-claim boundaries. Verification stays `bash scripts/verify.sh`.

---

## Claude's Discretion

- Exact fact type names and whether they extend `BlockServingStatusFacts`
- Probe API shape (trait vs helper vs block-store method)
- Compact-announcement inheritance details
- Reserved-`Pruned` copy without renaming the enum

## Deferred Ideas

- Phase 144 operator flush and availability evidence
- Phase 145 parity roots and no-claim guardrails
- FUT-18 through FUT-26 prune/archive/assumeutxo/public-default/production
- New getblock product RPC if none exists
