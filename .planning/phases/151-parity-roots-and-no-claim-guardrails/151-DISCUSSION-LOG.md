# Phase 151: Parity Roots and No-Claim Guardrails - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-29
**Phase:** 151-Parity Roots and No-Claim Guardrails
**Mode:** Yolo
**Areas discussed:** Parity surface ownership, Knots anchors and the Fjall difference, Last-gate no-claim checker, Allowed scoped claim versus denied overclaims, UAT and historical phase dirs, Closeout metadata

---

## Parity surface ownership

| Option | Description | Selected |
|--------|-------------|----------|
| Phase 151 owns only GRD-01 | Earlier phases keep SNAP, PRUN, LOCK, UNLK, SERV, LABL, and OPER | ✓ |
| One collapsed v2.4 surface | Move every v2.4 requirement onto the closeout row | |
| Planning docs as the evidence root | Treat ROADMAP and phase plans as requirement owners | |

**User's choice:** Phase 151 owns only GRD-01 (recommended default)
**Notes:** [auto] Backfill distinct index and checklist surfaces for Phases 146–150, then add `v2-4-parity-roots-and-no-claim-guardrails` for GRD-01 only. Reuse existing catalog pages.

---

## Knots anchors and the Fjall difference

| Option | Description | Selected |
|--------|-------------|----------|
| Cite prune, lock, unlink, limited-serving, and RPC anchors | Document Fjall key deletion versus `blk`/`rev` files as an intentional difference | ✓ |
| Cite only `blockstorage.cpp` | Leave policy, locks, and RPC unanchored | |
| Implement `blk`/`rev` files | Match Knots file layout in this closeout phase | |

**User's choice:** Cite the prune anchor set and document the Fjall difference (recommended default)
**Notes:** [auto] Researcher confirms exact symbol names in the pinned tree before the checker freezes them. No second block store.

---

## Last-gate no-claim checker

| Option | Description | Selected |
|--------|-------------|----------|
| New Phase 151 checker after Phase 145 | Phase 145 export shape, curated corpus, scoped successor exception for the v2.4 prune sentence | ✓ |
| Extend Phase 145 in place | No new checker | |
| Scan all `.planning/` prose | History-wide blocking corpus | |

**User's choice:** New Phase 151 checker wired immediately after Phase 145 (recommended default)
**Notes:** [auto] Keep the v2.3 D14 sentence. Update the Phase 145 `prune-mode` denial only enough to accept the exact v2.4 sentence. Phases 146–150 did not add `verify.sh` steps; do not backfill one checker per phase.

---

## Allowed scoped claim versus denied overclaims

| Option | Description | Selected |
|--------|-------------|----------|
| Bounded Fjall prune sentence | Height-window delete, `NODE_NETWORK_LIMITED`, `Pruned` only after a durable delete | ✓ |
| Full archive and prune node | Broader than GRD-01 | |
| Leave README on the v2.3 sentence only | No current-state prune claim | |

**User's choice:** Bounded Fjall prune sentence (recommended default)
**Notes:** [auto] Still reject archive, assumeutxo, BIP37, compact filters, public defaults, public-network CI, production readiness, `blk`/`rev` as a store, LevelDB import, automatic reindex, txindex+prune, and `-pruneduringinit`.

---

## UAT and historical phase dirs

| Option | Description | Selected |
|--------|-------------|----------|
| Committed `151-UAT.md` | Deterministic required tests, Cargo and Bazel forms, public-network may be not run, historical phase dirs stay tracked | ✓ |
| Public-network soak as a release gate | Would change the default verifier | |
| No UAT file | Checker-only closeout | |

**User's choice:** Committed UAT package with hermetic verification (recommended default)
**Notes:** [auto] Researcher confirms Phase 150 argv before freezing UAT commands. Do not gitignore or delete historical `.planning/phases/` directories.

---

## Closeout metadata

| Option | Description | Selected |
|--------|-------------|----------|
| Reconcile metadata, do not archive | Flip leftover Pending rows only after evidence exists; `/gsd-complete-milestone v2.4` stays later | ✓ |
| Archive v2.4 inside Phase 151 | Milestone completion during the phase | |
| Re-implement prune runtime | Use this phase to change unlink or serving | |

**User's choice:** Reconcile metadata without archiving or runtime rework (recommended default)
**Notes:** [auto] ROADMAP still lists OPER-01 through LOCK-02 as Pending while REQUIREMENTS.md marks them Complete. Reconcile that table to named evidence. Do not reopen Phase 150.

---

## Claude's Discretion

Exact checker helper names, fixture layout, paragraph classification, smallest honest breadcrumb splits, doc section placement, confirmed Knots symbol spellings, and whether optional UAT items are pending or not run.

## Deferred Ideas

Milestone archival, `-pruneduringinit`, archive serving, compact filters, BIP37, assumeutxo, LevelDB chainstate import, automatic reindex, public defaults, public-network CI, production readiness, txindex combined with prune, and a hosted GUI.
