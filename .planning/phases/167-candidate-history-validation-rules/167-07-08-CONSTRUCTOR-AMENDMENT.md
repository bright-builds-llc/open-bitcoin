---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T19:26:38Z"
---

# Plan07/08 constructor compatibility amendment

This is a bounded planning revision, pending independent checker acceptance. It does not release implementation or report any future source/test/key-link check as passed. The original two tasks in each plan, complete CVLD-06 behavior and both durable historical repeat cases remain mandatory.

## Original snapshots

Both original plans were copied before edits into `.local/open-bitcoin-dev/phase167/plan07-08-amendment/`; these local originals are not rewritten by the amendment.

| Original | SHA256 |
| --- | --- |
| `167-07-PLAN.original.md` | `04dffe61b588938778c24ca43bc968f564870fdb44a0f3a682f4b6ca54c88992` |
| `167-08-PLAN.original.md` | `8ce2d90856e056753b5b4e5a7f81cc248295cb25bbc4764bc8d80dae364d2522` |

## Inspection and narrow ownership delta

Read-only `rg` inspection of chainstate/node source and node integration tests confirmed the direct-literal owners below. Line numbers identify the inspected pre-implementation source and are not a promise that line numbers remain fixed.

| Plan/task | Added existing owner | Direct `BlockUndo` literal lines |
| --- | --- | --- |
| 07.2 | `packages/open-bitcoin-chainstate/src/block_filter/tests.rs` | 353 |
| 07.2 | `packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs` | 60,300,382 |
| 08.1 | `packages/open-bitcoin-node/src/chainstate/replay/tests.rs` | 233 |
| 08.1 | `packages/open-bitcoin-node/src/chainstate/tests.rs` | 217,289 |
| 08.1 | `packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs` | 203 |
| 08.1 | `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs` | 219 |
| 08.1 | `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs` | 62 |
| 08.1 | `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs` | 91 |
| 08.1 | `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append.rs` | 40 |
| 08.1 | `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append_proof.rs` | 31 |
| 08.1 | `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs` | 11 |
| 08.1 | `packages/open-bitcoin-node/src/sync/tests/phase165_live_fixture.rs` | 309 |
| 08.1 | `packages/open-bitcoin-node/tests/phase165_live_fixture.rs` | 548 |

This adds exactly two chainstate test owners and eleven node test owners, matching `files_modified` and the corresponding existing task's `<files>`. Plan07 now has nine owned paths and Plan08 eighteen. No new task or wave is introduced: the added paths are mechanical compatibility edits rather than new behaviors. None of these paths is owned by another current167 plan, so they add no same-wave conflict or scheduling edge.

Plan08 already owns production `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` and the direct-literal fixture in `src/storage/snapshot_codec/tests.rs:146`; these are not additional owners. The real production conversion is `impl From<&BlockUndo> for BlockUndoDto` and `impl TryFrom<BlockUndoDto> for BlockUndo`, whose latter constructor uses `Self`. `decode_block_undo(bytes)` has no candidate/history/proof argument. The per-block and whole-snapshot encoders both consume the DTO conversion, so both directions and routes must retain all fields.

`HistoricalBlockUndo { block_hash, undo: &BlockUndo }` wraps an existing reference and needs no new field. `engine/stage.rs`'s `undo() -> &BlockUndo` and node `network/runtime_authority/filter_index/catch_up/tests/faults.rs`'s `genuine_foreign_undo(...) -> BlockUndo` are return signatures, not literals. The `BlockUndo` definition and its `TryFrom` implementation header likewise are not fixture constructors. Existing `BlockUndo::default()` callers do not expand ownership.

## Compatibility and authority obligations

The added test owners permit only explicit empty overwrite-record collections in existing direct literals for ordinary/nonoverwrite and deliberately malformed ordinary transaction-undo fixtures. Preserve every existing input, assertion, expected error and fault injection, including the nonempty-genesis undo refusal and Phase165 changed-undo refusal controls. Do not regenerate expected hashes, weaken tests, change production adapters in added files, or renew guards/source-ownership hashes. Frozen originals and historical artifacts remain unchanged.

Plan07 Task1 retains a safe nonauthorizing in-memory `BlockUndo::default()` with empty collections. Default is useful data construction, never proof that overwrite records are unnecessary. Missing complete records must still refuse wherever an applicable overwrite context requires them. A fixture's explicit empty new-format collection does not authorize an empty legacy fallback.

Plan08's original production ownership still must serialize and validate complete prior Coin fields, outpoint/new-occurrence/block identity, bounds and duplicate/malformed controls through `From`/`TryFrom` and both per-block/snapshot routes. Missing legacy overwrite data is unproved: `decode_block_undo(bytes)` cannot infer the necessary history. Refuse it as `StorageError` unless the original complete prerequisite approach genuinely proves that the specific record required no overwrite. `serde(default)`, `unwrap_or_default`, an empty list or a caller claim cannot supply that proof. This amendment adds no automatic migration or schema-wide default. Positive full-record roundtrips, missing/wrong-record atomic refusals and all-handle physical checkpoint/release/reopen/disconnect evidence remain mandatory.

## Concrete planned key links

Plan07's conceptual `to: BlockUndo overwrite records` becomes a concrete consumer/provider edge from `packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs` to its originally planned `engine/overwrite_undo.rs`, with pattern `overwrite_undo::restore_overwritten_coins`. Task2 explicitly requires the real `apply_disconnect_on_overlay` flow to invoke that helper for verified complete restoration; no comment or dummy source is authorized to satisfy a pattern.

Root additionally requested the same metadata correction for Plan08's conceptual `to: BlockUndoDto`. Its consumer is the already-owned `storage/snapshot_codec.rs`, its provider the already-owned planned `storage/snapshot_codec/overwrite_undo.rs`, and its pattern is `overwrite_undo::(encode|decode)_overwrite_records`. Task1 explicitly requires real complete conversion/validation calls from `From`/`TryFrom`, preserving the original per-block/snapshot-to-Fjall intent. These helpers are implementation contracts, not claims of existing source or passing future wiring. No source key-link verification was run for this amendment.

## Decision coverage and preserved invariants

| Decision | Plan/task | Coverage | Preserved full obligation |
| --- | --- | --- | --- |
| D-06 | 07.1–2,08.1–2 | Full | Exact BIP30 admission, both repeats, full staged/durable prior-Coin restoration and explicit stronger pinned-disconnect difference. |
| D-08 | 07.1–2,08.1–2 | Full | Missing/malformed/duplicate/wrong identity refuses before accepted authority changes; prior fixture assertions remain intact. |
| D-09 | 07.2,08.1–2 | Full | Attributable source/serialized/seeded evidence, preserved negatives, no frozen artifact or guard/evidence weakening. |

Original `generated_at: 2026-10-21T16:55:59Z`, `generated_by`, yolo lifecycle/id, requirements `[CVLD-06]`, autonomy, both truths/artifacts, all original task action text, behavior, acceptance, done and verification commands remain unchanged; narrow compatibility/helper instructions are appended to the original actions. Plan08 Task2 is byte-identical to its original. Plan07 stays wave5 after06; Plan08 stays wave6 after both02 and07. The complete02 acquisition, both original offline checks and attributable summary must be accepted, and07 accepted, before08 begins. Elapsed time, Task1-only completion or partial corpus cannot release08. No deadline, budget, receipt, selected-network authority, fixture class or existing evidence class changes.

At inspection time02 and06 summaries were absent, and07 is still unimplemented; no absent dependency summary is invented. Available03/04/05 summaries inform the existing pure CandidateRules, earned admission, branch context and tested counter handoffs. This revision neither consumes partial02 data nor changes its producer.

## Actual planning checks and scope

Both final plan files passed `gsd-tools.cjs frontmatter validate <plan> --schema plan` and `gsd-tools.cjs verify plan-structure <plan>`: two tasks each, no errors or warnings. A read-only snapshot comparison additionally checked original lifecycle/generated_at/wave/dependencies/requirements, truths/artifacts, original action prefixes and task behavior/acceptance/verification/done, with exact Plan08 Task2 equality. Each plan has exactly two standalone frontmatter delimiters. A read-only path search confirmed no other current167 plan owns an added path. These are metadata/structure checks only; no source execution is claimed.

AGENTS.md Repo-Local Guidance, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/local-guidance and Rust/TS standards informed the amendment. Both active lessons were read completely:6898 global bytes plus1958 repo bytes,8856 total bytes and2953 conservative tokens; no archive was loaded and no project skill directory was present. The bounded GSD revision honors functional-core/adapter separation, preserved refusal evidence and root-owned saving.

Only07/08 plans, this amendment and targeted07/08 coverage notes were edited, plus the two requested local original snapshots. No source/test/build/guard, Git operation, shared STATE/ROADMAP/requirements/config/todo, PLAN-CHECK, frozen acquisition/corpus or other owner edit was performed. The root's independent checker must review the final amendment before any implementation release; full native/coverage/Bazel and final acceptance remain Plan20.
