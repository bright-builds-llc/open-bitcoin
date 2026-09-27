# Phase 148: Fjall Payload Unlink and Have-Pruned - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-27T02:47:11.140Z
**Phase:** 148-Fjall Payload Unlink and Have-Pruned
**Mode:** Yolo
**Areas discussed:** Paired delete unit, Have-pruned durability, Interrupted prune, Flush order and lock consumption, Cache and later-phase boundary

---

## Paired delete unit

| Option | Description | Selected |
|--------|-------------|----------|
| Block key only | Remove `block:<hash>` and leave undo in place | |
| Paired block and undo | Remove both keys for the planned height; success requires both absent | ✓ |
| Namespace wipe | Delete entire block or undo namespaces | |

**User's choice:** Paired block and undo (recommended default)
**Notes:** UNLK-01 and Pitfall 8. If one mate is already absent, delete the remaining mate. Do not invent bytes. Resolve height through the active-chain hash. No `blk`/`rev` files.

---

## Have-pruned durability

| Option | Description | Selected |
|--------|-------------|----------|
| Set at config parse | Flip have-pruned when prune mode is enabled | |
| Flag then unlink | Match Knots by setting the flag before the delete commits | |
| After durable delete | Record have-pruned only after a non-empty paired delete batch commits | ✓ |

**User's choice:** After durable delete (recommended default)
**Notes:** UNLK-02 and success criterion 2. Empty plans leave the flag unchanged. A failed batch must not set it.

---

## Interrupted prune

| Option | Description | Selected |
|--------|-------------|----------|
| Automatic reindex | Repair a partial delete by rebuilding chainstate | |
| Finish or refuse closed | Complete the same height's paired delete, or refuse without inventing blocks | ✓ |
| Ignore partial state | Continue as if the prune never started | |

**User's choice:** Finish or refuse closed (recommended default)
**Notes:** UNLK-03. Crash seams cover index cleared while keys remain, one key left, have-pruned with a key still present, and coins advanced in the delete batch. FUT-23 reindex stays deferred.

---

## Flush order and lock consumption

| Option | Description | Selected |
|--------|-------------|----------|
| Standalone prune_now | Delete keys outside the flush lifecycle | |
| Extend flush lifecycle | Persist, clear presence, delete the pair, then have-pruned, then coins | ✓ |
| Trust the plan only | Delete every planned height even if a lock now forbids it | |

**User's choice:** Extend the existing flush lifecycle and re-check locks (recommended default)
**Notes:** Phase 147 D-13 handed flush-before-delete here. Skipping a lock-protected height does not fail the rest. Do not move coins best-block in the delete batch.

---

## Cache and later-phase boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Leave caches | Rely on durable absence alone | |
| Evict then stop | Drop deleted hashes from the block cache and do not rewrite snapshots; leave labels and RPC to later phases | ✓ |
| Full operator surface | Also advertise limited services and add prune RPC in this phase | |

**User's choice:** Evict then stop (recommended default)
**Notes:** Pitfall 9. `Pruned` labels, `NODE_NETWORK_LIMITED`, and operator commands stay in Phases 149–150. Parity docs stay in Phase 151.

---

## Claude's Discretion

- Storage location of the have-pruned marker, provided it is durable and not inferred from missing payloads.
- Exact Fjall batch shape, provided one successful commit cannot leave a single mate key while reporting the height pruned.
- Interrupted-prune intent record, provided restart can finish or refuse that height.
- Module placement inside existing node storage and flush code.

## Deferred Ideas

- Limited serving and honest pruned labels (Phase 149).
- Operator prune surfaces and lock setters (Phase 150).
- Parity roots and no-claim guardrails (Phase 151).
- `-pruneduringinit` (FUT-27) and automatic reindex (FUT-23).
