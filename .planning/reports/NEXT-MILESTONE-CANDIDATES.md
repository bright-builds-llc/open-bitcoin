# Next Milestone Candidates after v2.3

**Status:** Recommendation (not an active milestone)
**Recorded:** 2026-09-21
**Context:** v2.3 Chainstate Durability and Historical Serving shipped and was archived on 2026-09-20. No milestone is currently active.

This note ranks post-v2.3 milestone candidates from archived GSD artifacts. It does not create requirements, a roadmap, or a claim. Start the next version with `/gsd-new-milestone`.

## Current position

- Latest shipped: **v2.3** (Phases 139–145, 15/15 requirements, 8/8 seams, 8/8 flows)
- Active milestone: **none**
- Explicit leftover inventory: **FUT-18 through FUT-26** in `.planning/milestones/v2.3-REQUIREMENTS.md`
- Non-blocking v2.3 debt: wallet rescan still reads leftover snapshot bytes; chainstate restart does not

## How this project chooses the next milestone

From v1.2 through v2.3, each milestone took the next fundamental node capability that the previous one unblocked, kept it opt-in and evidence-backed, and refused public defaults, GUI, destructive datadir mutation, and production-readiness claims.

v2.3 research said prune-mode product behavior had to wait until honest availability existed, then implement `-prune`, file unlinking, prune locks, and `NODE_NETWORK_LIMITED` on top of that gate. That gate shipped. `HAVL-02` reserved the `Pruned` label for a real prune-mode delete. FEATURES.md listed prune first under later milestones.

## Recommended next

**v2.4 Prune-Mode Product Behavior (FUT-18)**

Scope the milestone around Knots-aligned prune product behavior for the single active chainstate:

- Height windows, file unlinking, `m_have_pruned`, prune locks
- `NODE_NETWORK_LIMITED` serving limits
- Emit `Pruned` only when prune actually deleted files; keep `Unavailable` for missing payloads without prune
- First phase: cut wallet rescan off leftover snapshot bytes so prune cannot resurrect snapshot-as-truth
- Continue phase numbering at **146**

Keep archive-node serving, assumeutxo dual-chainstate, public defaults, and production-readiness claims out of v2.4.

## Ranked candidates

| Rank | Candidate | Inventory | When | Why |
| --- | --- | --- | --- | --- |
| 1 | v2.4 Prune-mode product behavior | FUT-18 | Now | Honest availability plus durable coins were built so files can be deleted without lying about payload presence. |
| 2 | v2.5 Compact-filter serving (BIP157/158) | FUT-20 | After prune | Light-client serving on honest historical data. Design prune interaction first; keep BIP37 out. |
| 3 | Wallet leftover-snapshot cutover | v2.3 debt | First prune phase | Wallet rescan still reads leftover snapshot bytes. Too small for a full milestone; close it as Phase 146. |
| 4 | AssumeUTXO / dual chainstate | FUT-21 | Later | Sync-speed milestone. Needs a second chainstate that v2.3 deliberately refused to add. |
| 5 | Full address relay | v1.9 leftover | Optional later | Independent of storage. Completes network participation, but is not the capability v2.3 unblocked. |
| 6 | Wallet PSBT / HD / multisig slice | wallet catalog | Theme change | Real remaining wallet gaps. Production-funds safety stays deferred; this would pause the node-capability chain. |

## Suggested sequence

1. **v2.4 Prune-mode product behavior** — bounded-disk full-node operation on the v2.3 honesty contract.
2. **v2.5 Compact-filter serving (BIP157/158)** — light-client serving, prune-aware index retention, no BIP37.
3. **v2.6 AssumeUTXO / dual chainstate** — sync-speed milestone after prune is a separate manager surface.

Do not combine prune and assumeutxo in one milestone. v2.3 kept a single active chainstate on purpose.

## Keep out of the next milestone

These remain valid FUT / out-of-scope items after v2.3. Pulling them forward would violate the project's claim-gate pattern.

| ID | Surface | Why not next |
| --- | --- | --- |
| FUT-19 | Archive-node / production-scale historical serving | Honesty about stored bytes is not an archive claim. Opposite operator problem from prune. |
| FUT-20 (BIP37 only) | BIP37 bloom serving | Privacy and DoS surface. Compact filters are the better light-client path. |
| FUT-22 | Knots/Core LevelDB `chainstate/` import | Different engine and schema. Migration stays dry-run-first. |
| FUT-23 | Automatic destructive reindex / repair | Recovery stays fail-closed and diagnostic. |
| FUT-24 / FUT-25 | Public defaults and public-network CI gates | v1.8 already defined those gates. Capability gaps still remain. |
| FUT-26 | Production readiness / production-funds wallet | Durable coins and prune do not independently satisfy the v1.8 evidence gates. |

Also parked, not next: general package wire / Erlay / cluster mempool (v2.2 residual), migration apply mode (wait until prune and coins schema settle).

## Sources

- `.planning/PROJECT.md`
- `.planning/STATE.md`
- `.planning/ROADMAP.md`
- `.planning/MILESTONES.md`
- `.planning/milestones/v2.3-REQUIREMENTS.md` (FUT-18 through FUT-26)
- `.planning/milestones/v2.3-MILESTONE-AUDIT.md`
- `.planning/research/FEATURES.md`
- `.planning/research/SUMMARY.md`
- `docs/parity/catalog/wallet.md`
- `docs/parity/catalog/p2p.md`
- `docs/parity/production-claim-boundary.md`

## How to start

Run `/gsd-new-milestone` with a name like **Prune-Mode Product Behavior**. Lock the same no-claim boundary v2.3 used: no archive-node serving, no assumeutxo dual-chainstate, no public defaults, no production-readiness claim. Continue phase numbering at 146.
