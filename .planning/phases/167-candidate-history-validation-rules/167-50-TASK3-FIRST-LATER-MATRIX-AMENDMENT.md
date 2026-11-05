---
phase: 167-candidate-history-validation-rules
plan: "50"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-28T05:15:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 50 Task 3 first and later lifecycle matrix

The exact lifecycle test-owner expansion corrected the stale post-timeout compact retry to a full Block response, but the Task 3 rerun still passed 92/93 and failed its universal GetData assertion. The 12-case fixture has a separate `shared` axis: with `shared=false`, peer 99 holds a different hash and peer 1 is the **First** requester of the candidate; with `shared=true`, peer 99 registered the candidate hash before peer 1 and peer 1 is **Later**. Plan 50 D1 intentionally emits one full-fetch GetData only for First. Later emits none and releases only its own request/partial. The first three `shared=false` cases passed before the loop reached `shared=true`. Independent review found no production bug or new source owner.

The previously checked `167-50-TASK3-FULL-BLOCK-TERMINAL-TEST-AMENDMENT.md` applies its full-Block terminal response only to the `shared=false` First cases. This case-specific matrix supersedes that amendment's implied all-12 acceptance expectation while retaining the same 16 exact direct owners, 14 original whole bodies, two true absences, 479 protected source, native/Knots frame and all literal commands.

| Axis | Expected full fetch and stimulus | Required state assertion |
| --- | --- | --- |
| `shared=false`, `finish=0/1` | Exactly one GetData for candidate hash; deliver the actual full `WireNetworkMessage::Block(block.clone())` after timeout. | Candidate tip/body accepted and stored, peer 1 terminally cleaned; peer 99's unrelated request and runtime inflight survive. The finish-0 Ping precedes the Block to prove consumed first membership before receipt. |
| `shared=false`, `finish=2` | Exactly one GetData; the existing send-triggered disconnect prevents a full-body response. | No candidate acceptance; peer 1 cleanup and peer 99 unrelated request/inflight preserved. |
| `shared=true`, all `finish` values | Zero GetData for candidate hash and no unsolicited full Block from peer 1. Do not reannounce the same compact hash after Later release: genuine new enrollment could be valid then. | Candidate tip/body remain at the pre-session baseline; peer 99's earlier matching request and global inflight survive; peer 1 is cleaned on session close. `finish=2` is a no-send cleanup control, while the First `finish=2` case covers send-triggered disconnect. |

Keep all 12 fixture combinations, exact fallback inventory type/hash for the First cases, active BlockTxn positive controls elsewhere and any independent consumed-first reannouncement negative. A matching BlockTxn after Later release is not a substitute for full-body success; an unrelated stray may remain a silence control. Rerun the exact Task 3 and focused peer-resolution/node-compact selectors with real nonzero discovery. Any Later GetData, loss of peer 99, or First full-Block acceptance failure is a product diagnosis HOLD, not a reason to loosen assertions. The prior 83/10, 87/6, 92/1 and post-terminal 92/1 runs remain diagnostic progression, not additional D1/D4 RED credit.

After a clean task run, all 16 Plan 50 final commands still run fresh on one frozen final source/native frame, followed by whole original-before BSD `patch -F0` inverse/forward, independent CORE/source/evidence review, canonical Summary/QA and ROOT acceptance. No owner expansion, target override, Git action, global CVLD closure or Phase 168 work is granted here.
