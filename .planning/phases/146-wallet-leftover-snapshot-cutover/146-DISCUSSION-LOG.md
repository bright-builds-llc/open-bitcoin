# Phase 146: Wallet Leftover-Snapshot Cutover - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-21T20:57:50.113Z
**Phase:** 146-Wallet Leftover-Snapshot Cutover
**Mode:** Yolo
**Areas discussed:** Wallet scan authority, Leftover blob disposition, Missing payload, Have-pruned and unlink boundary, Operator and parity surface

---

## Wallet scan authority

| Option | Description | Selected |
|--------|-------------|----------|
| Coins best-block plus payload-present blocks | Matches SNAP-01 and success criterion 1. Shell assembles the view; wallet stays I/O-free. | ✓ |
| Overlay coins onto the leftover snapshot | Keeps snapshot-as-truth when the blob disagrees. | |
| Coins only, ignore block bodies | Drops the payload-present requirement. | |

**User's choice:** Coins best-block plus payload-present blocks, assembled in the node shell.
**Notes:** [auto] Same-datadir reopen must ignore a planted leftover snapshot for balances and history. Existing chunked rescan jobs stay. A wallet entry requires its creating payload to be present.

---

## Leftover blob disposition

| Option | Description | Selected |
|--------|-------------|----------|
| Leave the blob unread | Phase 142 already stopped live snapshot writes. Deletion is not SNAP-01 and must not look like prune. | ✓ |
| Delete the blob during rescan | Extra mutation. Easy to confuse with payload unlink. | |
| Stop every remaining reader, including schema migration | Would break the one-way schema-1 migrate that still reads the blob once. | |

**User's choice:** Leave the blob on disk. Wallet rescan does not read it. Schema-1 migration may still read it once to seed coins.
**Notes:** [auto] Do not restore `save_chainstate_snapshot` from `persist_progress`.

---

## Missing payload

| Option | Description | Selected |
|--------|-------------|----------|
| Fail the rescan chunk closed | No snapshot fallback, no invented transactions. | ✓ |
| Skip the height and continue | Would hide a missing body and can rebuild a partial history. | |
| Fall back to leftover snapshot UTXOs or undo | Restores snapshot-as-truth, which this phase exists to remove. | |

**User's choice:** Fail the chunk closed.
**Notes:** [auto] Missing payload stays `Unavailable`, not `Pruned`.

---

## Have-pruned and unlink boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Do not delete payloads and do not set have-pruned | Success criterion 3. Unlink is Phase 148. | ✓ |
| Set have-pruned because the snapshot was ignored | Invents have-pruned without a durable delete. | |
| Delete old payloads as part of cutover | Pulls Phase 148 into this phase. | |

**User's choice:** Neither delete nor have-pruned.
**Notes:** [auto] Contributors must be able to observe that payloads remain.

---

## Operator and parity surface

| Option | Description | Selected |
|--------|-------------|----------|
| No new prune surfaces; breadcrumbs only | Phase 150 owns operator evidence. Phase 151 owns GRD-01. | ✓ |
| Add a rescan-source RPC field now | New operator surface outside SNAP-01. | |
| Full prune parity catalog in this phase | Belongs to Phase 151. | |

**User's choice:** No new prune operator surfaces. Touched Rust files get parity breadcrumbs. Verification stays `bash scripts/verify.sh`.
**Notes:** [auto] Selected all gray areas and recommended defaults in one pass.

## Claude's Discretion

- Scan-view type: reuse `ChainstateSnapshot` assembled from coins and payload-present blocks, or a narrower scan input, as long as the wallet crate stays I/O-free.
- Which existing coins and block-store methods supply best-block and payload bytes.
- Fixture style for a disagreeing leftover snapshot.

## Deferred Ideas

- Prune policy and lock windows (Phase 147)
- Fjall unlink and have-pruned (Phase 148)
- Limited serving and honest `Pruned` labels (Phase 149)
- Operator prune surfaces (Phase 150)
- v2.4 parity roots and no-claim checker (Phase 151)
- Deleting leftover snapshot blobs
- `-pruneduringinit` (FUT-27)
- assumeutxo, archive serving, BIP37, public defaults, and production-funds claims
