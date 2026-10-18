---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T02:20:12Z
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-17T02:20:12Z
depth: standard
review_base: d0104a99
review_base_full: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
preliminary: true
source_only: true
files_reviewed: 38
files_reviewed_list:
  - packages/open-bitcoin-chainstate/src/coins.rs
  - packages/open-bitcoin-chainstate/src/coins/cache.rs
  - packages/open-bitcoin-chainstate/src/coins/memory.rs
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs
  - packages/open-bitcoin-chainstate/src/engine/stage.rs
  - packages/open-bitcoin-chainstate/src/engine/tests.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/apply_and_connect_errors.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/connect_snapshot_and_cache.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/staged_connect_and_reorg.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata/difficulty.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/phase165_rebind.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/phase165_tests.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/phase165_work_authority.rs
  - packages/open-bitcoin-chainstate/src/engine/work_authority.rs
  - packages/open-bitcoin-chainstate/src/error.rs
  - packages/open-bitcoin-chainstate/src/lib.rs
  - packages/open-bitcoin-chainstate/src/types.rs
  - packages/open-bitcoin-consensus/src/block.rs
  - packages/open-bitcoin-consensus/src/chain_proof.rs
  - packages/open-bitcoin-consensus/src/chain_proof/tests.rs
  - packages/open-bitcoin-consensus/src/chain_proof/tests/phase165_vectors.rs
  - packages/open-bitcoin-consensus/src/crypto.rs
  - packages/open-bitcoin-consensus/src/crypto/tests/phase165_tests.rs
  - packages/open-bitcoin-network/src/header_store.rs
  - packages/open-bitcoin-network/src/header_store/eligibility.rs
  - packages/open-bitcoin-network/src/header_store/eligibility/tests.rs
  - packages/open-bitcoin-network/src/header_store/eligibility/tests/phase165_authority.rs
  - packages/open-bitcoin-network/src/header_store/eligibility/tests/phase165_cached_order.rs
  - packages/open-bitcoin-network/src/header_store/proof.rs
  - packages/open-bitcoin-network/src/header_store/proof/tests.rs
  - packages/open-bitcoin-network/src/header_store/tests.rs
  - packages/open-bitcoin-network/src/header_store/tests/phase165_tests.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/eligibility.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 165: Preliminary Core Source Review

**Status:** clean within the assigned core scope. No actionable correctness or security finding was established. This is a preliminary source review before full native verification, not final phase acceptance.

## Scope and Method

Reviewed the current uncommitted changes against `d0104a99` plus all untracked Rust files in the assigned consensus work/crypto/block, network header-store/eligibility/facade, and chainstate engine/coins/type/error/facade paths. The exact 38-file scope is preserved above and hashed below. Implementation and new behavioral tests were read in context; inherited test changes were checked for preserved intended validation/error paths. Source searches checked relevant exports, uses, and authority consumers. No Git-log task-commit heuristic was used.

Guidance materially applied: root `AGENTS.md` and repo-local guidance, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, `standards/index.md`, architecture, code-shape, testing, verification and Rust standards. No project skills directory was present. Both active lesson files were read completely: 5,230 global bytes plus 1,958 repository bytes, 7,188 combined bytes and 2,397 conservative estimated tokens; no active lesson content was omitted. Phase 165 CONTEXT/RESEARCH and Plans 01, 04, 06, 07, 08 and 09 SUMMARY substantive contracts informed the review.

Local pinned Knots source was consulted for `SetCompact`, `GetBlockProof`, the work comparator and `ReceivedBlockTransactions`. These are source comparisons only.

## Reviewed Invariants

| Area | Source observations |
| --- | --- |
| Exact numeric work | `consensus/src/chain_proof.rs:60,175` retains unsigned four-limb ordering, modulo accumulation and the complemented-target floor formula. Fixed-width byte/hex imports preserve all bits and do not mint provenance. `crypto.rs:130,181` normalizes the compact mantissa before zero/sign checks and adds selected powLimit/hash admission. |
| Accepted header authority | `network/src/header_store.rs:169,421` requires accepted parents, derives parent-plus-own proof and changes best only for strictly greater work. Complete selected parameters bind the store. Raw records/imports cannot produce accepted proof facts or overwrite accepted entries. `header_store.rs:58` requires identical full selected parameters and actual proof facts for both best and side request projections. |
| Body linkage and ordering | `header_store/eligibility.rs:121,261` validates actual selected-context bytes before registry receipt, assigns sequence only on transaction ancestry linkage and releases pending children breadth-first through FIFO vectors/queue. Present duplicates preserve the original event. Failure and unavailable nonactive ancestry are excluded separately. |
| Unknown cached provenance | `header_store/eligibility.rs:172,185,204` keeps cached order unknown through pending and later descendants and ordinary duplicates. Known candidates precede unknown candidates at equal work. Cached receipt still proves actual linkage rather than asserting arrival parity. |
| Immutable event versus freshness | `network/src/lib.rs:105,117` and the registry's custom Clone preserve immutable linkage allocation identity while giving independent mutable histories fresh epoch ownership. Facts alone cannot construct a binding; mutation/replacement/drop revokes an issued weak epoch. |
| Stage construction and origin | `chainstate/src/engine/work_authority.rs:37,46,93` and `engine/stage.rs:344` bind private non-Clone stages to owner, generation, full source tip and selected parameters. Raw and mechanical origins remain distinct; public coins mutation taints provenance. Exclusive validated-stage borrows prevent intervening owner mutation. |
| Preview continuation | `engine/stage.rs:418,461` permits preview once on the original source generation. WorkAuthority's private seal permits only that stage's continuation. Preview cannot supply live source authority, start another genuine stage or create an earned clone; intervening mutation revokes continuation. |
| Consuming rebind | `engine/work_authority.rs:241` consumes a genuinely live source and compares complete effective coins, B/heads, active positions, undo and confirmation counts before target assignment. Unsupported complete views refuse; overlay tombstones participate. Success creates fresh owner/generation and preserves selected parameters. Pristine empty count normalization is narrowly bounded. |

The inherited `prefer_candidate_tip` numerical utility retains height/hash tiebreaking, but the source search found no production live-selection caller in the reviewed chainstate/node paths. It is not treated as the new live preference authority.

## Behavioral Test-Source Assessment

The new fixed corpus tests exercise high limbs, exact byte order, malformed encodings, compact normalization, target equality/exceedance and carry/wrap without deriving expected values using the production helper. Header tests cover shorter-harder selection, accepted equal-work order, taller equal-work retention, raw/inflated/forged ancestry and selected/unbound transitions.

Body tests cover sibling/grandchild FIFO release, duplicates, unavailable/failed ancestry, bounded counter reservation, clone/event identity, epoch changes and owner drop, selected request projections, both cached iteration orders and sticky unknown ancestry. Chainstate tests cover maximum supplied scores, raw/mechanical/foreign/stale sources, owner/generation rotation, preview once/continuation/taint, complete-view errors, uncached coins, every coin field, markers, metadata/undo/count mismatch, and rebind parameter preservation. Compile-fail examples cover stage construction/mutation/Clone and validated-stage exclusive borrowing.

These observations describe the assertions present in source. They do not assert that tests compiled or executed during this review.

## Limits and Required Follow-Up

- No Cargo, Bazel, Bun tests, builds, coverage runs, native verification, daemon execution or store opening was performed. This source-only limit was explicitly assigned to avoid interference with the parent execution workflow.
- Existing summary receipts were treated as context, not independently reproduced runtime proof. No pass count or coverage result is claimed by this review.
- Parent final full-source, security and goal review remains required after Plan 14 and full native verification. Node-managed preference, persistence, sync, ordinary compatibility and tooling changes outside this core scope require their separately assigned reviews.
- Phase 166 canonical bootstrap/reopen acceptance, Phase 167 contextual completeness and Phase 168 actual pinned-daemon qualification remain deferred. The current four ordinary refusal classes and default-off networking are not qualified or expanded by this report.
- Only this report was created. No source, test, checker, tracker, config, hook, index, commit, remote or worktree was changed.

## Stable Source Snapshot

SHA-256 of exact reviewed file bytes, captured at `2026-10-17T02:19:15Z`. Recompute before final acceptance; changed hashes require delta review. The complete scope is included so test or production changes cannot silently escape the final comparison.

| File | SHA-256 |
| --- | --- |
| `packages/open-bitcoin-chainstate/src/coins.rs` | `cd59da7b3d47af02d06266ef1fcfaaa4e7d9984438d2a28ef36f201284aa54b2` |
| `packages/open-bitcoin-chainstate/src/coins/cache.rs` | `c4607eb78e30621e1edc093563cc7fb1fe650d72eadf3430a3aae78508222664` |
| `packages/open-bitcoin-chainstate/src/coins/memory.rs` | `b6919e5204326331f3f61970046b766ed454dec6403b7dadb4f219859edd4003` |
| `packages/open-bitcoin-chainstate/src/engine.rs` | `c1d99d5b719d3c04176442f5fe3faadf18dbd8d6bdc71865a92d17a367306156` |
| `packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs` | `0bce54ac9cbe8024fd80ea704e7f6a7a0813f963d400a971b52e2919bd0908f9` |
| `packages/open-bitcoin-chainstate/src/engine/stage.rs` | `475ac4c27ac612a61b69805c313d5ca8235b8720608dddf2a9584163a0c2641e` |
| `packages/open-bitcoin-chainstate/src/engine/tests.rs` | `9ff68e9f8ab2ed085c885c7ab706d4270aba3ecd09c8e18a252adc54cc4fd234` |
| `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/apply_and_connect_errors.rs` | `491b3ad6a78719eac51702e44cb2f1fa8c5287930da9dc4cb0534bfaae976d24` |
| `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/connect_snapshot_and_cache.rs` | `c2f88d891c3856a75043c2ff38e46b8ddf102ac97a117de41964e02e776ee3cd` |
| `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/staged_connect_and_reorg.rs` | `9c6747b6d4e1a9ff220a8c2b7aafa43385247a80c2669301be725d000733b95d` |
| `packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata.rs` | `f800289e4ab36c7fd1491ffe80381fee6c4c1db7e006fda24a8f709a9ef20c1a` |
| `packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata/difficulty.rs` | `951de46b475a1620eb639f80d3dd82a0477e000b882b9d315d6bcba189348346` |
| `packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs` | `6f6fc595b1ec022b1be64f51583e1e55f15023c10f20ffebbec5207fba7d493a` |
| `packages/open-bitcoin-chainstate/src/engine/tests/phase165_rebind.rs` | `a3f4080591ed73bde274e1458198b7a3cae80a8873b04b11eb5a602ee226831b` |
| `packages/open-bitcoin-chainstate/src/engine/tests/phase165_tests.rs` | `dda6a8efff0c2d4bd23f9620bc8e9fc296cd5b9487aba4004436b75584819951` |
| `packages/open-bitcoin-chainstate/src/engine/tests/phase165_work_authority.rs` | `dcc91e72fe8ab8f492dab9318c5176e864e11c64c2b87d22b9c06857fa42f653` |
| `packages/open-bitcoin-chainstate/src/engine/work_authority.rs` | `6c3412159f23e761a283adcccbaf45af3b0cf9aca62af607184d5fc612785dbb` |
| `packages/open-bitcoin-chainstate/src/error.rs` | `bcebfceedef8d23ab4defa74cfec2b4e59f07d89c8867faa52ac8f8c44b33e4f` |
| `packages/open-bitcoin-chainstate/src/lib.rs` | `d6a0d6b678ac45ac9e50bfbe68f23de9f4ffd2d8c476b0d67540b3110c16f189` |
| `packages/open-bitcoin-chainstate/src/types.rs` | `6ecbd780c0c0501a28b749a496ca4f7431f8ab89c36a0af1a5c487ca9c5b4f62` |
| `packages/open-bitcoin-consensus/src/block.rs` | `a5219e5800c8b247afecbb509f481bc9d0cb8049ccd34fb48f10b07ea2bf8cea` |
| `packages/open-bitcoin-consensus/src/chain_proof.rs` | `9201f9f7604c66f6d3a514eb9f498ff658ebf7ec07e22fab04e311d7c55b785f` |
| `packages/open-bitcoin-consensus/src/chain_proof/tests.rs` | `ef7f17b25c47df0b49d481ac4386cba834a236f2c52abd64c6321c96731948c3` |
| `packages/open-bitcoin-consensus/src/chain_proof/tests/phase165_vectors.rs` | `abd0371668601b6e244068fedf05d00ce0e19340ef8de6955f34342d2425a2c0` |
| `packages/open-bitcoin-consensus/src/crypto.rs` | `631710ae9083ab3a6acd21e923aba9fb052a016b70b2d4380f55530d01fb108d` |
| `packages/open-bitcoin-consensus/src/crypto/tests/phase165_tests.rs` | `654623113c80f3cc0a71dc301ce3b805a407b7421db9ca7c21fb3d58582c33d9` |
| `packages/open-bitcoin-network/src/header_store.rs` | `4955b6481396d6dcec0172921e8f10f1fcf4f4b7b655611a73a70f9fc79d9803` |
| `packages/open-bitcoin-network/src/header_store/eligibility.rs` | `751b989eec04f97e280a0eccc2dc234a6fa413bd0804e5a3d1e802b0f120cc9c` |
| `packages/open-bitcoin-network/src/header_store/eligibility/tests.rs` | `dea3cba79ff5c05e9752947798d1d909471b485bd65e34afa5a569e54e11a007` |
| `packages/open-bitcoin-network/src/header_store/eligibility/tests/phase165_authority.rs` | `b157fdd575ff04f19d7df5c51cfd81dccc05253ee8a74bbdb2467ea4f6556151` |
| `packages/open-bitcoin-network/src/header_store/eligibility/tests/phase165_cached_order.rs` | `7d4ae687ec178e401bf425bb43c92db70f5c17330e1a2070e44bc67e00723fb6` |
| `packages/open-bitcoin-network/src/header_store/proof.rs` | `3fd34b9d5f0b48939e50a731a330065d84c427b12c312bb18da41e856b662d27` |
| `packages/open-bitcoin-network/src/header_store/proof/tests.rs` | `39858dbe40d4beb551a29c8d7c2f1d2a29e359b1301efde99ab8b452424d3c09` |
| `packages/open-bitcoin-network/src/header_store/tests.rs` | `93e1e1154e4eae7a3943b8156c9e87e06d1441cd880adfcc6eac14d6a9025275` |
| `packages/open-bitcoin-network/src/header_store/tests/phase165_tests.rs` | `1e05a4b55c2522ef66e08bc6a47b11398e7807ea9630e0f76124dda713880af8` |
| `packages/open-bitcoin-network/src/lib.rs` | `5f5250766167d46197f8f89f9355ac15cb97ae792f6935af1f2c7c5f58620fe0` |
| `packages/open-bitcoin-network/src/peer.rs` | `4673cc6c9ab5d1885e304c20c5e0bf9ee67020330eea7d2f17a9177ee6f4a615` |
| `packages/open-bitcoin-network/src/peer/eligibility.rs` | `893c33ce182e4e9e5813a4a4be16f3e4f4ca1fcd5eb2a4dff59883a39aa78b85` |

_Reviewer: gsd-code-reviewer. Depth: standard. Preliminary source-only review._
