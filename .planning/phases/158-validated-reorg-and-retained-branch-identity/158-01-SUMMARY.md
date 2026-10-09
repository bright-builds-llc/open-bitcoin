---
phase: 158-validated-reorg-and-retained-branch-identity
plan: "01"
subsystem: chainstate
tags: [rust, compact-filters, reorg, staged-acceptance]
requires:
  - phase: 157-safe-activation-and-scheduled-index-catch-up
    provides: Ordered constant-size BASIC progress and checked stale-work identities
provides:
  - Checked trial branch replacement with separately verified indexed and safe prefixes
  - Private genuine staged reorg facts and consume-only absorbed endpoint receipt
affects: [158-02, 158-03, 158-04, 158-05, 158-06, 158-07]
tech-stack:
  added: []
  patterns: [pure trial reducer, borrowed staged facts, opaque accepted receipt]
key-files:
  created:
    - packages/open-bitcoin-chainstate/src/filter_index/catch_up/reorg.rs
    - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/reorg.rs
  modified:
    - packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs
    - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests.rs
    - packages/open-bitcoin-chainstate/src/engine.rs
    - packages/open-bitcoin-chainstate/src/engine/stage.rs
    - packages/open-bitcoin-chainstate/src/lib.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/staged_connect_and_reorg.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Generation renewal uses checked exact +1 directly without disable/re-enable
  - A replacement can return to the same stable branch hash; renewed generation invalidates old work
  - Only successful genuine staging constructs a reorg stage and only absorption mints accepted receipts
  - Common ancestry and durable achievements remain separately authenticated adapter responsibilities
patterns-established:
  - Public pure replacement facts grant no storage permission
  - Capture original endpoints during validation before preview can alter live state
requirements-addressed: [CFIX-03]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: 2026-10-08T03:05:30Z
duration: 12min
completed: 2026-10-08
---

# Phase 158 Plan 01: Pure Replacement and Sealed Acceptance Summary

**Exact common-prefix replacement preserves processed/safe distinctions, and genuine staged absorption earns an opaque endpoint receipt.**

## Performance

- Started after root execution activation at 2026-10-08T02:53:32Z; implementation and targeted verification completed on 2026-10-08 UTC.
- Tasks: 2/2 implemented and locally verified.
- Implementation files: 9, including two new Rust modules; this summary is the tenth plan-owned file.
- Baseline HEAD remained `2f21ac2052208c7e7a084ed006d908c5ccb714aa` on the shared main checkout.

## Accomplishments

- The pure reducer requires exact old acceptance/generation/branch, exact checked next generation, and separately verified indexed/safe shared endpoints. Lagging progress never jumps to an unprocessed ancestor. Safe progress never advances and protection must cover both its shared prefix and previous protection.
- Equal-height forks, shorter/disconnect-only branches, root replacement, empty progress, paused/disabled state, maximum-height arithmetic, malformed ancestry/identities, stale preparation, and generation exhaustion have behavior tests. The initial synchronization latch survives replacement but cannot become true merely because backlog shrinks.
- Every staged reorg field is private, including captured old tip and common ancestor. No Clone, Default, From, Deserialize, raw constructor, mutable accessor, or test construction bypass was added. Replacement undo lookup is restricted to the exact connected hash identities of the genuine stage.
- Genuine fork staging includes a historical coinbase spend and a same-block spend. Preview then absorption preserves captured old identity, retained ancestor, generated undo metadata, replacement position, and accepted endpoint facts. Disconnecting the entire chain yields an accepted receipt with no new endpoint.

## Exact Contracts

Exported from `filter_index::catch_up`:

```rust
pub struct BasicIndexReplacementFacts {
    pub expected_generation: IndexGeneration,
    pub expected_branch_identity: BlockHash,
    pub old_target: AcceptedIndexTarget,
    pub new_target: AcceptedIndexTarget,
    pub maybe_common_ancestor: Option<AcceptedIndexTarget>,
    pub maybe_indexed_common: Option<FilterRecordIdentity>,
    pub maybe_shared_safe: Option<FilterRecordIdentity>,
    pub achieved_generation: IndexGeneration,
    pub achieved_branch_identity: BlockHash,
    pub protection: IndexInputProtection,
}
BasicIndexProgress::replace_validated_branch(
    self, facts: BasicIndexReplacementFacts,
) -> Result<Self, BasicIndexCatchUpError>
```

`new_target` is a nonempty accepted index endpoint; no ancestor requires Empty indexed/safe prefixes. The branch fact equals the new accepted hash. Generation is exactly old +1, even when the new hash returns to the prior stable branch incarnation. The receiver is copied and returns a trial state; errors cannot mutate it. Complete ancestry is authenticated by adapters, not invented by these pure endpoint values.

Stage and receipt contracts, re-exported through crate `lib.rs`:

```rust
StagedChainstateReorg::transition(&self) -> &ChainTransition
StagedChainstateReorg::maybe_position_at_height(&self, height: u32) -> Option<&ChainPosition>
StagedChainstateReorg::maybe_replacement_undo(&self, block_hash: BlockHash) -> Option<&BlockUndo>
StagedChainstateReorg::maybe_old_tip(&self) -> Option<&ChainPosition>
StagedChainstateReorg::maybe_common_ancestor(&self) -> Option<&ChainPosition>
Chainstate::absorb_staged_reorg_with_receipt(
    &mut self, staged: StagedChainstateReorg,
) -> (ChainTransition, AcceptedChainstateReorg)
AcceptedChainstateReorg::maybe_old_endpoint(&self) -> Option<(u32, BlockHash)>
AcceptedChainstateReorg::maybe_new_endpoint(&self) -> Option<(u32, BlockHash)>
AcceptedChainstateReorg::maybe_common_ancestor_endpoint(&self) -> Option<(u32, BlockHash)>
```

The receipt is non-Clone and holds only three optional endpoint tuples. Its only construction expression follows overlay and metadata installation in the absorption method. Existing `absorb_staged_reorg` delegates and discards the receipt, preserving its `ChainTransition` return. `stage_reorg` retains its signature and sole genuine construction path. The only external stage consumer found was node `chainstate.rs`; its `transition()` borrowing compiles unchanged.

## Verification Evidence

All Cargo commands used pinned Bun 1.3.9 through `scripts/command-timings.ts` with serialized checkout target access and Rust 1.94.1.

| Check | Result |
| --- | --- |
| Initial reducer RED, `phase158-core-reorg-red` | Exit 101; missing replacement type/method, 20 errors |
| Initial staged behavior RED, `phase158-staged-contract-red` | Exit 101; missing borrowed accessors/absorption method, 11 errors |
| Privacy RED, `phase158-staged-api-red` | Exit 101; all six raw construction/field mutation examples compiled unexpectedly before sealing |
| Final `cargo test -p open-bitcoin-chainstate --lib phase158_reorg`, key `phase158-core-reorg` | 17 passed, zero failed/ignored; 15 reducer tests and two genuine engine tests |
| Broader chainstate `--lib`, key `phase158-core-all` | 328 passed, zero failed/ignored, including existing staged compatibility behavior |
| `cargo test -p open-bitcoin-chainstate --doc`, key `phase158-staged-api` | 12 passed: ten external compile-fail cases and two positive borrowed API examples |
| `cargo check -p open-bitcoin-node --all-targets`, key `phase158-staged-callers` | Passed, including after staging-method relocation |
| Crate Clippy `--all-targets --all-features -- -D warnings`, key `phase158-core-style` | Passed |
| Cargo workspace format, key `phase158-format` | Passed |
| `bun scripts/bright-builds-check.ts all` | Zero findings after scoped simplification |
| `bun run scripts/check-parity-breadcrumbs.ts --check` | 997 currently tracked Rust files verified; new modules registered and exact comments manually checked |
| Diff review, named-test discovery, `git diff --check` | Passed; no unrelated source changes |

The parity checker enumerates tracked files. Under the requested no-staging gate, the two new modules remain intentionally untracked; the parent final gate must rerun the checker after staging includes them. They already carry exact base.cpp/blockfilterindex.cpp/blockstorage.cpp comments and explicit manifest registrations. Touched engine/stage tests add undo.h to the existing anchors; `lib.rs` keeps its explicit infrastructure/re-export `none` anchor.

## Task Commits

Both task commits and metadata finalization are **deferred** by the user's strict phase wrapper. No staging, commit, push, checkout change, or hook bypass occurred. Root owns STATE, ROADMAP, requirement activation, full native verification, independent review and final Git operations.

## Deviations from Plan

**[Rule 2 - Instruction enforcement] Relocated genuine `stage_reorg` implementation into the existing `engine/stage.rs`.** Required external API examples increased `engine.rs` to 692 lines, failing the managed 628-line checker. Moving the existing method reduced it to 628 and kept all fields private, the API unchanged, and one successful validated construction path. No exception, new module, dependency, or semantic staging change was needed. All unit/API/style checks passed after relocation.

During simplification, removed an unnecessary branch-hash inequality and added a return-to-same-incarnation regression. Checked generation renewal already invalidates old work and allows valid disconnect-only return after ordinary accepted growth. This corrects the new implementation without expanding scope.

## Guidance and Simplification Review

Repo-local AGENTS commands, Bright Builds sidecar, placeholder-only overrides, pure architecture/code-shape/testing/verification/Rust pages and active lessons informed the implementation. Both active lesson inputs total 7,188 bytes / 2,397 conservative tokens; no new audit trigger was needed. Pure reducer helpers reuse existing endpoint order/protection rules, absorption delegates from the compatibility method, and staging moved within its existing module boundary to satisfy size guidance.

## Remaining Downstream Contracts

This plan alone does not activate runtime reorg indexing or complete CFIX-03. Plans 02–06 must consume the receipt through sealed same-store tracked lineage, achieve bounded guarded storage replacement, restore the ordinary owner/flush path, preflight real required payloads, retain immutable rows, and prove faults plus actual reopen. Plan 07/root owns native coverage/Bazel, parity claims, README/UAT updates, independent source/security review and lifecycle finalization.

The new reducer and receipt are constant-size and introduce no history map/vector. Existing staging and preview still clone chain/undo metadata; no claim of whole-reorg constant memory is made. `maybe_replacement_undo` checks the connected identities with a linear scan of the staged replacement list before its existing map lookup; downstream resource accounting must include that work. No new numeric cap is calibrated here.

No stubs, new endpoint, file-access trust boundary, schema, external service, authentication gate or unresolved plan-local issue was introduced.

## Self-Check: PASSED

Both new Rust modules and this summary exist; all seven modified implementation/registry files are present. Named tests executed with positive counts, exact public/private contracts were reviewed, and baseline HEAD is unchanged. Commit existence checks are intentionally deferred under the strict no-Git-finalization instruction; no nonexistent commit is claimed.
