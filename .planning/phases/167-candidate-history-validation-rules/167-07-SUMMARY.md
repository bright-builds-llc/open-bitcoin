---
phase: 167-candidate-history-validation-rules
plan: "07"
subsystem: chainstate
tags: [rust, bip30, complete-undo, historical-fixtures, provenance]
requires:
  - phase: 167-06
    provides: Exact selected candidate context and explicitly tainted historical staging
  - phase: 167-23
    provides: Complete actual coins/script kernel and non-skipped predecessor regression
  - phase: 167-26
    provides: Accepted complete source-bound tagged repeat fixtures
provides:
  - Exact pre-block live-outpoint scan and derived coinbase replacement
  - Complete block-bound prior/new Coin records and explicit no-overwrite provenance
  - Lossless real repeat disconnect/reconnect and atomic refusal controls
affects: [167-08, 167-22, 167-24, 167-16, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Pre-block scanning, Complete batch integrity, Mechanical relative suffix work]
key-files:
  created:
    - packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/phase167_bip30.rs
  modified:
    - packages/open-bitcoin-chainstate/src/types.rs
    - packages/open-bitcoin-chainstate/src/engine/apply.rs
    - packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs
    - packages/open-bitcoin-chainstate/src/engine.rs
    - packages/open-bitcoin-chainstate/src/engine/tests.rs
    - packages/open-bitcoin-chainstate/src/block_filter/tests.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs
key-decisions:
  - "Capture the complete pre-block scan before any transaction spends an input."
  - "One complete batch with zero entries represents captured no-overwrite; an empty batch collection remains missing provenance."
  - "Integrity and binding checks are reusable structural checks, never canonical proof or an authority constructor."
requirements-completed: [CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T21:24:00Z"
commits: false
duration: 11min42s
completed: 2026-10-21
---

# Phase 167 Plan 07: Exact BIP30 and lossless overwrite undo Summary

**Both real repeated coinbases now stage through complete historical bodies, preserve the exact earlier Coins, and disconnect/reconnect without loss under the approved stronger CVLD06 contract.**

## Files Created/Modified

- Modified: `packages/open-bitcoin-chainstate/src/types.rs` — explicit complete batch and per-outpoint prior/new Coin data;209lines.
- Modified: `packages/open-bitcoin-chainstate/src/engine/apply.rs` — pre-block capture, narrowly permitted replacement and complete staged undo;309lines.
- Created: `packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs` — scan, complete integrity/binding, verified restoration and focused controls;622lines.
- Modified: `packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs` — pre-removal occurrence checks and actual restore_overwritten_coins call;168lines.
- Modified: `packages/open-bitcoin-chainstate/src/engine.rs` — registers the cohesive private module;552lines.
- Created: `packages/open-bitcoin-chainstate/src/engine/tests/phase167_bip30.rs` — bounded standard-library typed loader and actual full-body stage round trips;520lines.
- Modified: `packages/open-bitcoin-chainstate/src/engine/tests.rs` — test sibling registration and test-only visibility for the owned helper module;316lines.
- Modified: `packages/open-bitcoin-chainstate/src/block_filter/tests.rs` — only an explicit empty overwrites collection on its direct BlockUndo literal;529lines.
- Modified: `packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs` — only explicit empty collections on its three direct BlockUndo literals;616lines.

Exactly nine declared Rust owners:two created and seven modified. Every original fixture assertion, expected error, input and fault remains unchanged in the two mechanical literal owners. HistoricalBlockUndo wrappers are unchanged. Accepted predecessor integration remains preserved. New files have pinned-source comments;19 owns the manifest. Full owner byte counts/SHA256 values are in .local/open-bitcoin-dev/phase167/plan07/final-owned-stats.json. This summary and ignored receipts are execution records.

Source checkout reference only, not a task commit:`1b472fb225a290df5cdb06eeea2dc67c624c648e`. No staging, commit, push, worktree or hook bypass occurred. Shared metadata/plans/todo and subsequent releases remain root-owned.

## Completed Tasks

1. Exact duplicate scanning and overwrite provenance:complete. The entire pre-block overlay is checked for every transaction output before any transaction may spend an input. A same-block earlier spend cannot erase a later duplicate. Generic/unselected contexts keep strict overwrite refusal. Only a checked history whose candidate bytes/height/parent-MTP agree and whose derived disposition skips BIP30 can permit the applicable coinbase replacement. Live noncoinbase duplicates, unrelated/wrong-context duplicates and corrupt prior coinbase facts refuse. Fully spent prior outputs are distinct from live replacement; txid occurrence counts do not authorize replacement.
2. Verified disconnect and atomic refusal:complete. Every successful connect captures exactly one complete batch, even when no outputs were overwritten. Before removal, disconnect checks complete binding/integrity and compares each overwritten new occurrence against the full Coin, including creating MTP. Transaction undo runs privately, then the actual overwrite_undo::restore_overwritten_coins helper restores only captured complete prior Coins with collision refusal. Branch counts update only along successful disconnect and reconnect reproduces the full connected snapshot. Missing provenance never becomes an empty scan or a reconstructed historical Coin. For old deliberately malformed fixtures, a disposable diagnostic overlay preserves their original specific failure before missing provenance refuses; it cannot publish changes or authorize a successful disconnect.

## Precise Plan08 Codec Contract

The public data lives under open_bitcoin_chainstate::types; no extra export owner is required:

- BlockUndo keeps transactions:Vec<TxUndo> and adds overwrites:Vec<BlockOverwriteUndo>. A generated complete undo has exactly one batch. Zero batches is unknown/missing provenance; multiple batches refuse. BlockUndo::default() remains empty and nonauthorizing.
- BlockOverwriteUndo contains block_hash:BlockHash,height:u32,creating_median_time_past:i64,overwritten:Vec<OverwrittenCoin>,integrity:[u8;32]. An empty overwritten vector inside the sole valid batch is an explicitly captured no-overwrite result, including the fully-spent duplicate case.
- OverwrittenCoin contains transaction_index:u32,outpoint:OutPoint,previous_coin:Coin,new_coin:Coin. Both Coins retain output amount/script,is_coinbase,created_height and created_median_time_past. Together with batch block identity and outpoint, they distinguish the old and new occurrences. No old Coin is inferred from a known pair, a current output or a missing record.
- BlockOverwriteUndo::validate_integrity() and BlockUndo::validate_overwrite_binding(&Block,&ChainPosition) return Result<(),ChainstateError>. They validate structural integrity/shape and exact occurrence binding only. A self-consistent serialized batch remains untrusted until the existing validation owner/replay path checks it against actual history and coins. These methods mint no canonical seal or accepted receipt.

Integrity uses existing first-party Sha256, with domain bytes open-bitcoin/complete-overwrite-undo/v1; internal block-hash bytes; little-endian height(u32),creating-MTP(i64),entry-count(u64); then each entry's transaction-index(u32),internal txid bytes,vout(u32),previous Coin and new Coin. Each Coin contributes amount(i64),script-length(u64),script bytes,coinbase byte,height(u32),creating-MTP(i64),in that order.08 must preserve the actual captured digest and all fields, then reuse these checks; recomputing a digest to bless altered imported facts or adding missing-field defaults would violate the contract. Record bounds, actual durable encoding and physical reopen remain08's work.

## Real Fixture and Evidence Boundaries

The fixed include_str! consumer reads accepted packages/testdata/phase167/bip30-fixtures.txt with a bounded ASCII/LF tab/tag parser using only standard-library decimal/hex conversion and ordinary primitives. It checks closed order/arity/roles/indexes/counts,version/pin/fixed source identities,canonical numbers,ranges,hex,header identities,full transaction txids/wtxids,merkle binding,complete prior Coins and every required window/link. Every displayed hash reverses once into internal wrappers; script/witness bytes never reverse. No JSON parser, Bitcoin wire decoder/encoder,new dependency or giant Rust constants were added.

For91842→91812 and91880→91722, exactly155real parent-first rows feed checked seeded history, while stored positions ascend in height. Ten separate raw prefix headers support a true eleven-sample MTP for every stored position and never become positions or recursively fabricated metadata. Original-parent eleven-header windows derive the earlier creating MTP. Work sums actual header proofs from a local zero starting point over the suffix; this is relative mechanical fixture work, never alleged global canonical work.

Both complete repeat bodies and both complete original bodies are reconstructed.91880 executes its real second transaction using external input cae24697a15419b1de45c99a41149101c252c2dbd3fd0bbce06ac5809d4547e3:1,4480000000sat,noncoinbase,height90644,creating-MTP1289277562 and its exact P2PKH script. Its missing-input refusal compares the whole accepted snapshot. Stage calls are actual from_snapshot_for_mechanical_fixture plus stage_seeded_historical_block,independent header/body/merkle/selected checks,absorption,disconnect and reconnect. Both remain MechanicalFixture/IndependentlySeeded,have no earned work and cannot initialize canonical selection.

The actual detached mainnet227931header is used only as an independent seed for synthetic linked high-height structural/rule/kernel probes. Matching/nonmatching ancestry, genuine preactivation absence,missing required227931anchor and1983701/1983702recheck are distinct. Missing high-height ancestry returns MissingAncestor; it is never called genuine absence. Synthetic probes perform no PoW stage and claim no canonical/history acceptance.

Pinned validation.cpp2784–2861 scans pre-block live outpoints and grants only the exact repeats or BIP34 commitment before recheck. Pinned normal DisconnectBlock/UpdateCoins/undo.h do not restore overwritten coinbase outputs; they tolerate missing earlier originals91722/91812 instead. CVLD06 explicitly requires the stronger complete earlier-Coin restoration implemented here. **No historical disconnect parity with Knots is claimed.** No daemon was run.

## Commands and Actual Outcomes

All Cargo commands used pinned Bun1.3.9 PATH and scripts/command-timings.ts serially with existing targets/cooperative lock. Complete logs are under .local/open-bitcoin-dev/phase167/plan07/.

| Command/stage | Actual outcome | Receipt |
| --- | --- | --- |
| Initial focused attempt | Compile/setup failure:Hash32 supplied where MerkleRoot is required. Corrected the test wrapper; no behavioral claim. | red.log |
| First behavioral focused RED |1 discovered/1 failed:the full real91842body reached existing stage behavior and returned OutputOverwrite. The loop stopped there; no second-repeat or disconnect RED is claimed from this run. | behavior-red.log |
| First implementation attempt | Compile/setup failure:an overly broad mechanical replacement also touched three HistoricalBlockUndo wrappers. Immediately removed those additions; final wrappers are unchanged. | green-initial.log |
| Full real pair GREEN |1 named test passed;both complete repeat body connect/disconnect/reconnect loops executed. | green-pairs.log |
| Initial expanded controls | Compile/setup failure:test helper visibility was narrower than the new test module. Reused a parsed real transaction instead; no additional constructor owner or visibility change. | controls-first.log |
| Expanded controls GREEN |7/7 passed. | controls-fixed.log |
| Expanded exact-context controls |8/8 passed before final public structural checks/relative-work refinement. | focused-final.log |
| Initial complete chainstate all-features regression |427unit+3integration+23doctests=453passed,0failed/ignored/filtered. This predates the final small explicit refinements and does not substitute for the next final run. | regression.log |
| Final cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --all-targets --all-features -- -D warnings | Exit0,first attempt,5.71s. | clippy.log |
| Final cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate phase167_bip30 |8discovered/8passed,0ignored/skipped;7.17s. Separate zero-match integration binary is not counted. | final-focus.log |
| Final cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --all-features |427unit+3integration+23doctests=453passed,all result groups0failed/ignored/filtered;unit88.40s. | final-regression.log |
| Scoped rustfmt --edition2024 --config skip_children=true --check on all nine owners | Exit0,no recursive unrelated formatting. | rustfmt-final.log |
| Scoped git diff --check and full owned diff/new-file review | Exit0;only the explicit empty collections remain in the two mechanical fixture owners. | diff-check.log;owned-integration.diff |
| Current07key-links | all_verified:true,2/2. Real executed restore helper and fixture include/load/stage consumer,not comments or orphan patterns. | key-links-final.json |

The final eight named tests include19undo corruptions per real repeat,including four re-sealed semantic cases beyond checksum rejection:duplicate entries,invalid prior occurrence height,wrong outpoint and wrong new output. Other controls cover missing undo/batch/entry,duplicate batch,wrong block/height/txid/index/coin/MTP/digest,new-coin MTP mismatch,restoration into an occupied live outpoint,fully-spent distinction,unrelated live noncoinbase duplicate,earlier same-block spend,wrong network/hash/height,changed body/merkle and missing91880external input. All refused accepted operations compare complete tip/header/work/coins/undo/count snapshots. Collision and high-height probes remain explicitly private kernel evidence. Parser controls delete and duplicate each of404required records,plus version/network/decimal/range/hex/tag/trailing/encoding mutations. These are cases within eight tests,not hundreds of independent discovered tests.

Initial Cargo waited for Cursor rust-analyzer's existing workspace artifact lock; observed rustc progress established liveness and the wait resolved normally. No IDE process/settings were changed and no quiet work was killed. Resumable checks were polled within60seconds. Exploratory rg of an absent coins/overlay.rs returned exit2; the actual owner is coins/cache.rs. Optional skill-directory rg returned exit2 because both directories are absent. Oversized displayed context reads were repeated in smaller targeted sections. These are setup observations,not product failures or behavioral RED. All original accepted receipts remain untouched. No authentication gate occurred.

## Guidance, Simplification and Scope

AGENTS.md,its Bright Builds sidecar/overrides,architecture/code-shape/testing/verification/Rust guidance,accepted summaries and current context/research/coverage/latest plan check informed execution. Both canonically distinct active lessons were loaded completely:6898+1958=8856bytes,2300+653=2953estimated tokens. Existing audit baseline applies; no audit trigger or project skill was found. The GSD execution/summary contracts apply with the explicit parent-owned shared-state/saving exception.

The simplification pass keeps one selected rule derivation,one pre-block scan and one complete batch rather than per-caller overwrite switches or another authority owner. Existing primitives/hash/merkle/coins/stage/transaction checks are reused. Tests fit in the two owned cohesive modules; all nine files stay below629lines. No new crate/dependency,production endpoint,auth path,file I/O,source guard/hash renewal or breadcrumb manifest change was introduced. Complete provenance is the declared trust boundary,not a new canonical proof mechanism.

T-167-07-01 is covered by exact context/disposition and pre-block/refusal controls;T-167-07-02 by complete capture,binding,new-occurrence/collision checks,19corruption cases per repeat and full state round trips;T-167-07-03 by accepted source/data preservation,retained failures and explicit evidence classes. No unresolved stub or additional undeclared threat surface remains. Empty collections only represent explicit absent provenance or a genuine complete empty scan; they are never guessed previous Coins.

## Deviations and Remaining Work

No ownership,architecture,dependency or accepted-behavior expansion occurred. The initial integrated behavioral test was written before production work and genuinely failed at existing stage behavior. The restoration portion was implemented with that integrated test after the scan gap; there is no separate claimed pre-implementation disconnect RED. Setup failures and progressive checks are retained above. Public pure structural checks were confirmed with root as the bounded08reuse interface; they add no authority owner.

Both planned tasks are complete within07.22/24next own node fixture constructor adaptation;08then owns durable encoding/physical reopen and the paired node compile. **No node compilation pass is claimed here.**16 retains later durable historical evidence;19 owns breadcrumbs/parity/README updates,17owns review before guard renewal and20owns full native/fresh coverage/Bazel/source/security/goal/lifecycle gates. No Phase168 work started. Frontmatter requirement IDs describe this bounded contribution,not whole-phase closure.

commits:false. Shared STATE/ROADMAP/config/requirements/todo,plans,all accepted02/26source/data and other owners were not edited. Root alone updates shared progress and performs normal-hook saving after final acceptance. This executor stops before22/24/08release.

## Performance Attribution

Two tasks,nine actual Rust owners. New loader source birth2026-10-21T21:12:18.813135Z is the first measured implementation timestamp;first command receipt starts21:12:29Z. Final source/check observation21:24:00.469691Z gives702seconds(11min42s),excluding earlier context loading and subsequent summary validation. final-owned-stats.json records these measured timestamps,not a guessed start. The preexisting accepted producer/interpreter execution durations are not added again.

## Self-Check: PASSED

- All nine declared Rust files and this summary exist;two new files have pinned-source comments and every source is below629lines.
- The explicitly labelled preexisting checkout commit resolves;no task commit is asserted.
- Final8focused tests,453complete affected tests/doctests,strict scoped Clippy,formatting and whitespace checks pass without exclusions.
- Current07has both actual source links2/2;accepted fixture SHA256 remains7b91df3ceee0f702c46f98911f4dc92a85c85a0327697e424810c46a379fff5a.
- gsd-tools verify-summary --check-count9 returns passed:true,9checked/9found and errors:[];frontmatter validation returns valid:true/missing:[]. Receipts are verify-summary.json and frontmatter.json. The commits_exist result resolves only the labelled checkout baseline.
