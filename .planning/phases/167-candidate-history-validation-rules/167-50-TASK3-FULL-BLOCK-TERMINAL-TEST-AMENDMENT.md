---
phase: 167-candidate-history-validation-rules
plan: "50"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-28T05:04:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 50 Task 3 actual full-block terminal control

After the exact two-test-owner expansion, a fresh literal Task 3 run discovered 93 tests, passed 92 and failed one in `sync/tests/peer_resolution/peer_protocol_failures/lifecycle.rs`. The test's first case sends an incomplete compact block, advances the session to strict timeout, then reannounces compact and sometimes BlockTxn while expecting the block to be accepted. Plan 50 D1 deliberately consumes the compact attempt when it emits a full-fetch GetData. A same-hash compact reannouncement remains suppressed and a matching BlockTxn is duplicate misbehavior until terminal cleanup. The stale test stimulus cannot earn its expected accepted tip. An independently reviewed Phase 123 control already proves the intended full-Block-after-fallback route; no production bug is identified by this failure.

ROOT explicitly adds **only** `packages/open-bitcoin-node/src/sync/tests/peer_resolution/peer_protocol_failures/lifecycle.rs` to Task 3 and the Plan 50 direct-owner list. It is a present 596-line test child with an unchanged original body in the pre-source causal frame. The checked allocation becomes **16 unique direct owners**, 14 present whole originals and two true-absent children, tasks 4 / 2 / 5 / 5; protected first-party source becomes 479. This is a test-only expansion and grants no additional production, registrar, native, dependency, public API, timer, cap or policy owner.

For the cases that are meant to finish with actual block acceptance, deliver the real `WireNetworkMessage::Block(block.clone())` after the emitted full-fetch GetData. Preserve exact accepted tip, stored-body and terminal cleanup assertions for all 12 cases. Move the finish-0 Ping before the Block response to keep a direct control that first consumed membership survives until the terminal receipt. Retain consumed compact reannouncement as a negative stimulus, use an unrelated stray BlockTxn for silence, and keep the separate active BlockTxn positive controls. Do not weaken assertions to accept either inventory type or a missing accepted tip.

Before editing this file, ROOT must capture its complete original whole body/current hash, confirm it remains equal to the pre-source frame, issue an exact one-owner supplementary lease and obtain independent release review. After the edit, rerun the exact Task 3 selector and focused peer-resolution/node-compact selectors with nonzero named discovery. If actual full-Block recovery still fails, hold and diagnose product behavior rather than broaden test expectations. Preserve the 83/10, 87/6 and 92/1 failures as diagnostic progression, not D1/D4 RED credit.

Run all 16 final Plan 50 commands fresh and serially on one final frozen source/native frame. The whole original-before BSD `patch -F0` inverse/forward must restore all 14 present original direct bodies and both true absences, with 479 protected first-party files, 201 original native, Byteview1 and four pinned Knots anchors conserved. Independent final CORE/source/evidence review, separate Summary/QA and ROOT acceptance remain required. No Git action, global CVLD closure or Phase 168 work is granted.
