---
phase: 167-candidate-history-validation-rules
plan: "16"
type: evidence-and-owner-refinement-amendment
generated_by: gsd-plan-phase
generated_at: "2026-10-27T22:15:14Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 16 ordinary flag-sensitive spends

Independent Task 1 review found one unsatisfied original action: the exact three historical script bodies, creating-context pairs and 9/9 named tests do not exercise an ordinary spend sensitive to each of P2SH, DERSIG, CLTV, CSV, NULLDUMMY, WITNESS and TAPROOT. Plan 15's frozen ordinary corpus contains OP_TRUE spends at heights 101 and 201 and one P2WSH OP_TRUE spend at 151; it does not supply all seven exact serialized spend cases. The reviewed [seven-flag source design](../../../.local/open-bitcoin-dev/phase167/plan16/ordinary-flag-spend-design-2026-10-27.md), SHA-256 `a3a0780929fefcf50b46e9e08c5b20113f7951eb8a36f3ce6d9069d8b8c5ed50`, identifies the source-bound parent rows/prevouts and concrete script controls.

The checked Plan 16 ownership adds only the true-absent test child `packages/open-bitcoin-node/src/sync/tests/phase167_historical/ordinary_flags.rs` to Task 1. The historical parent registers it; the already-owned `contexts.rs` may expose its source-bound ordinary case helper to this sibling. This child must load Plan 15 serialized parent/history and independently expected prevout facts first, then create **explicitly derived mechanical** test candidates by changing only the stated previous-output script and necessary transaction scriptSig/witness, recomputing Merkle/Segwit commitment and valid Regtest PoW when bytes change. Both positive selected-rule execution and a negative flag-sensitive control must reach the real managed seeded chainstate interpreter for each of the seven flags. A pure bypass control may demonstrate sensitivity but cannot replace managed negative/positive execution. Assert unchanged selected scalar facts, complete accepted state and no earned/canonical/BASIC authority for fixture-origin paths. Never call the mutated coin or candidate an exact frozen Plan 15 body.

Selected default-Regtest rules include TAPROOT at these ordinary heights even though the exact mainnet 692261 exception intentionally does not. Use source parent height 101 for P2SH/DERSIG/NULLDUMMY/TAPROOT, 151 for WITNESS and 201 for CLTV/CSV as appropriate. Keep each new Rust file normally formatted at no more than 628 lines with a pinned-source breadcrumb. Plan 19 owns its breadcrumb manifest entry. The Task 1 original command must discover and pass the new cases on final source; rerun all original Task 1 checks and renew the causal source/native frame and inverse after adding the child.

This addendum preserves every original Task 1/Task 2 action, acceptance criterion, dependency, wave, requirement and literal automated command. It changes only the Plan 16 frontmatter and Task 1 `<files>` ownership metadata, adding one test-only path. The frozen Task 1 source frame has 478 paths and seven Task 1 owners; the new child would make a 479-path frame and eight Task 1 owners, still protecting 471 other source paths including six separately reviewed Root fixes. These are prospective counts, not acceptance. Task 1 remains held until the seven actual managed flag-sensitive cases and independent review pass; Task 2 remains held. No new production dependency, staging, commit, push or Phase 168 work is authorized.
