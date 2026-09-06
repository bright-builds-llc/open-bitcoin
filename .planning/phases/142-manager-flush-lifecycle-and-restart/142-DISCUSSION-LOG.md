# Phase 142: Manager Flush Lifecycle and Restart - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-06
**Phase:** 142-manager-flush-lifecycle-and-restart
**Mode:** Yolo
**Areas discussed:** Crash-loss window, Cache-byte defaults, Interrupted-flush recovery, Manager ownership and CanFlush, Ordered flush and persist cutover

---

## Crash-loss window

| Option | Description | Selected |
|--------|-------------|----------|
| Periodic + IfNeeded + Always shutdown | Match Knots: crash may lose unflushed cache; clean shutdown Always-flushes | ✓ |
| Every-connect persist | Keep today's snapshot dump after every connect/reorg | |
| Always flush after every connect | Avoid the window by writing coins on every block | |

**User's choice:** Periodic + IfNeeded + Always shutdown (recommended default)
**Notes:** [auto] Roadmap research flag. Every-connect leftover dumps are the snapshot-era path Phase 142 replaces.

---

## Cache-byte defaults

| Option | Description | Selected |
|--------|-------------|----------|
| Shell-injected Knots-aligned defaults | 450 MiB / 4 MiB min / 8 MiB coins-DB cap as adapter facts; first-party occupancy bytes | ✓ |
| Pin defaults in core `decide_flush` | Contradicts 140 D-10 | |
| Defer all defaults | Leave limits unspecified and hope tests inject them | |

**User's choice:** Shell-injected Knots-aligned defaults (recommended default)
**Notes:** [auto] Core stays unpinned. Shell resamples 50–70 minute jitter after a successful Periodic write.

---

## Interrupted-flush recovery

| Option | Description | Selected |
|--------|-------------|----------|
| Replay when undo and bodies exist; else fail closed | Knots `ReplayBlocks` shape; no invented tip | ✓ |
| Always fail closed | Never replay even when undo and bodies are present | |
| Always replay / invent tip | Replay or synthesize consistency when bodies are missing | |
| Automatic destructive reindex | Out of scope (FUT-23) | |

**User's choice:** Replay when undo and bodies exist; else fail closed (recommended default)
**Notes:** [auto] Also lock 141-REVIEW WR-02 (classify `H` before leftover-empty) and IN-01 (typed remap, not Display text).

---

## Manager ownership and CanFlush

| Option | Description | Selected |
|--------|-------------|----------|
| One manager, single active chainstate | Init → health-check/recover → cache init → CanFlush; Fjall parent in shell | ✓ |
| Dual snapshot/IBD chainstate | Out of scope (FUT-21) | |
| Keep MemoryCoinsView as production parent | Would leave coins writes on the leftover snapshot path | |

**User's choice:** One manager, single active chainstate (recommended default)
**Notes:** [auto] MGR-01. MemoryChainstateStore stays the test parent.

---

## Ordered flush and persist cutover

| Option | Description | Selected |
|--------|-------------|----------|
| Block/undo/index then coins; cut leftover writes | Abort coins if earlier step fails; progress from coins `B` | ✓ |
| Keep leftover snapshot as live truth | Contradicts MGR-02 and success criterion 5 | |
| Write coins first | Violates Knots flush order and PITFALLS.md | |

**User's choice:** Block/undo/index then coins; cut leftover writes (recommended default)
**Notes:** [auto] Flip Phase 140/141 leftover-write guard tests when cutover lands.

---

## Claude's Discretion

- Manager type/module naming
- How `Chainstate` becomes generic over a `CoinsView` parent
- Exact `ReplayBlocks` helper and crash-simulation fixture style
- Whether leftover snapshot files are deleted or only unread
- Internal typed facts for later Phase 144 surfaces

## Deferred Ideas

- Honest payload-present availability — Phase 143
- Operator flush/recovery evidence — Phase 144
- Parity-root closeout — Phase 145
- Prune/archive, assumeutxo, dual-chainstate, LevelDB import, auto-reindex, public defaults, production readiness — FUT-18 through FUT-26
