# Phase 145: Parity Roots and No-Claim Guardrails - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-18T03:38:24.949Z
**Phase:** 145-parity-roots-and-no-claim-guardrails
**Mode:** Yolo
**Areas discussed:** Parity surface ownership, Knots anchors and intentional differences, Last-gate no-claim checker, Allowed scoped claim vs denied overclaims, UAT and default verification, Closeout metadata without milestone archive

---

## Parity surface ownership

| Option | Description | Selected |
|--------|-------------|----------|
| Backfill 139–144 surfaces plus one CSVFY closeout row | Mirror Phase 117/138 exactly-once ownership; index.json currently has no v2.3 entries | ✓ |
| One collapsed v2.3 catalog entry | Faster, but hides CACHE/FLUSH/COIN/MGR/HAVL/CSOBS owners | |
| Skip index/checklist backfill and only add a checker | Leaves CSVFY-01 without machine-readable roots | |

**User's choice:** [auto] Backfill 139–144 surfaces plus one CSVFY closeout row (recommended default)
**Notes:** Phase 145 owns only CSVFY-01 and CSVFY-02. Prefer index/checklist rows over new catalog pages when `catalog/chainstate.md` already exists.

---

## Knots anchors and intentional differences

| Option | Description | Selected |
|--------|-------------|----------|
| Cite coins.h/cpp, validation.cpp, chainstate.cpp, blockstorage.cpp and document Fjall/LevelDB, occupancy, leftover-snapshot, reserved Pruned, single chainstate | Matches CSVFY-01 and already-locked 139–144 differences | ✓ |
| Claim line-by-line Knots source parity | Contradicts project parity model and Fjall-in-shell architecture | |
| Leave chainstate.md on Phase 4 snapshot wording | Would keep leftover snapshots looking like live UTXO truth | |

**User's choice:** [auto] Cite the four Knots families and document intentional differences (recommended default)
**Notes:** Refresh `docs/parity/catalog/chainstate.md` so disk-backed coins are current and snapshot-era prose is labeled historical.

---

## Last-gate no-claim checker

| Option | Description | Selected |
|--------|-------------|----------|
| New Phase 145 Bun checker pair after Phase 144, Phase 138-shaped export, curated current corpus | Last `check-phase*` v2.3 gate; does not scan historical `.planning/` | ✓ |
| Extend the Phase 138 checker to also police v2.3 | Mixes milestone gates and risks weakening the v2.2 D-21 contract | |
| Scan all of `.planning/` for forbidden phrases | Noisy, blocks honest historical prose, violates 117/138 corpus rules | |

**User's choice:** [auto] New Phase 145 checker after Phase 144 (recommended default)
**Notes:** Keep Phase 138's v2.2 D-21 sentence in its required files. Do not globally allow unscoped historical serving, archive, or prune.

---

## Allowed scoped claim vs denied overclaims

| Option | Description | Selected |
|--------|-------------|----------|
| Verbatim D-14 sentence plus FUT-18..26 overclaim denials, with deferred/no-claim markers remaining valid | Matches CSVFY-02 and PROJECT.md out-of-scope | ✓ |
| Allow "historical serving" as a positive claim because HaveBlockData exists | Reads as archive-node / production-scale serving | |
| Omit a required verbatim sentence and rely on prose tone | Undermines deterministic last-gate checking | |

**User's choice:** [auto] Verbatim scoped sentence plus denied overclaims (recommended default)
**Notes:** Companion allowed wording covers sanitized operator evidence, leftover-snapshot non-authority, restart from coins best-block, and hermetic default verification.

---

## UAT and default verification

| Option | Description | Selected |
|--------|-------------|----------|
| Committed 145-UAT.md, repo-local Cargo/Bazel forms, deterministic default verify.sh, historical phase dirs stay tracked | Matches success criterion 3 and AGENTS.md | ✓ |
| Make public-network historical-serving review a default CI gate | Violates FUT-25 and hermetic verify.sh | |
| Archive or gitignore historical `.planning/phases/` after closeout | Breaks existing `check-phase*` evidence paths | |

**User's choice:** [auto] Deterministic UAT package; keep historical phase dirs tracked (recommended default)
**Notes:** Optional public-network review may be recorded as not run.

---

## Closeout metadata without milestone archive

| Option | Description | Selected |
|--------|-------------|----------|
| Flip leftover Pending v2.3 rows only after the checker names evidence; reconcile ROADMAP/REQUIREMENTS/PROJECT/STATE; do not archive | Mirrors 138 D-17/D-18/D-25 | ✓ |
| Archive v2.3 from this phase | Milestone completion is a separate GSD command | |
| Leave CACHE-01 / MGR-03 Pending after the last-gate checker exists | Forces a later reconciliation phase | |

**User's choice:** [auto] Closeout without archive; flip leftover Pending after evidence (recommended default)
**Notes:** No runtime coins/flush/manager/serve changes unless a named evidence gap has no honest existing proof.

---

## Claude's Discretion

Checker helper names, fixture organization, paragraph-classification implementation, smallest honest breadcrumb or catalog-owner splits, exact doc section placement, and whether optional UAT items are recorded as pending or not run.

## Deferred Ideas

- `/gsd-complete-milestone v2.3` after this phase passes
- FUT-18 through FUT-26 prune/archive/assumeutxo/filters/public-default/production claims
- `getblock` product surface
- Hosted web dashboard / GUI
