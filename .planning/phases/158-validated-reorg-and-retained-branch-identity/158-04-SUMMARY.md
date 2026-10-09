---
phase: 158-validated-reorg-and-retained-branch-identity
plan: "04"
subsystem: node
tags: [rust, compact-filters, reorg, retained-inputs, scheduled-catch-up]
requires:
  - phase: 158-03
    provides: Genuine accepted reorg owner, preview suspension and ordinary own-flush fencing
provides:
  - Native retained body and exact historical undo preflight before preview and mempool effects
  - Genuine nonempty accepted-position append for live and recovered scheduled owners
  - Bounded immutable reuse, independent accepted-target observation and composed preparation work
affects: [158-05, 158-06, 158-07]
tech-stack:
  added: []
  patterns: [native envelope admission, genuine accepted-position append, compositional work bounds]
key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/filter_reorg/preflight.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests/faults.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs/undo.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/filter_reorg.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store/reorg.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs
    - packages/open-bitcoin-node/src/sync/block_reconcile.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Require actual disconnected body and exact complete historical undo before preview even when a filter exists
  - Use genuine staged undo for new replacements and exact predecessor-bound immutable reuse for shared history
  - Authenticate every nonempty live and recovered turn through the same borrowed accepted-position bridge
  - Keep zero-record checkpoint promotion on the existing own coins and metadata fence
  - Retain checked composed preparation work while leaving existing consensus-stage history costs separate
  - Refresh only observed exact inherited counters and strengthen the full-ancestry control
requirements-addressed: [CFIX-03]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: "2026-10-09T02:36:36Z"
duration: "22h 2min"
completed: 2026-10-09
---

# Phase 158 Plan 04: Required Sources and Production Scheduled Reorg Summary

**Production reorgs preflight retained body/undo before preview, and the ordinary scheduled driver authenticates live replacement and recovered-prefix records from the current accepted chain.**

## Performance and Ownership

- Tasks: 2/2 implemented and locally verified.
- Initial dispatch: 2026-10-08T04:34:33Z. Completion: 2026-10-09T02:36:36Z (2026-10-08 CDT / 2026-10-09 UTC).
- Elapsed 22h 2min includes the host launch blockage and overnight checkpoint gap. Resumed verification began at 2026-10-09T02:16:09.358Z. These elapsed windows are not reorg latency measurements.
- Implementation scope: fifteen Rust modules and the breadcrumb registry, including five new Rust modules. Checkpoint and this summary are separate planning artifacts.
- All source is on the shared checkout. Timing receipts still identify baseline HEAD 2f21ac2052208c7e7a084ed006d908c5ccb714aa. No staging, commit, push, worktree, hook bypass or requirement activation occurred.
- No dependency, crate, persistent schema/key, public endpoint, extra worker, full-history index cache or forced coins flush was added.

## Accomplishments

- The genuine lineage passes its private same-store handle into reusable node-chainstate preflight. Both direct manager preparation and the actual serialized network reorg path use it before preview, mempool edits or acceptance/publication effects. Existing mempool ordering is preserved.
- Disconnected consensus blocks require actual native retained bodies and non-genesis undo even if filters already exist. The body scanner admits the wire envelope before decoding; historical inputs then verify exact accepted header/hash/body-merkle identity and complete undo shape. Native undo admission precedes serde and reserves a conservative complete container/count/copy envelope. Its decoded value must equal the full genuine old accepted undo, including spent scripts, amounts, heights and coinbase metadata.
- Lagging shared-prefix heights may reuse a verified immutable hash record with exact canonical height/hash/parent and predecessor header when consensus does not disconnect that height. Missing records require the actual historical source pair. Newly staged replacements use their generated genuine undo, including historical and same-block spends, without demanding previously durable replacement undo.
- The native reusable-row reader distinguishes Missing, Deferred and Ready. Its admission callback selects the existing normal/singleton policy before clone/decode. Deferred rows never trigger a second probe or fallback generation. Reused records are checked against the driver's exact processed predecessor before publication.
- Every nonempty scheduled turn uses the genuine manager's accepted-position bridge for both ValidatedReorg and RecoveredPrefix. The choice does not depend solely on maybe_replacement_target, which may be None after recovery. Positions remain a bounded borrow from the existing current accepted chain; preparation creates owned immutable work and completion retains the same-store generation/revision/branch/frontier checks. Later turns work after the stage has been consumed.
- Zero-record maintenance uses the ordinary shared append adapter for earned checkpoint promotion: it has no forward projection and still needs the actual own coins/metadata fence. Full disconnect keeps conservative definite-target progress paused, exposes an absent accepted target and prepares no nonempty work.
- Existing turn evidence now carries maybe_accepted_target, maybe_accepted_lag and reused_records. Publication failure exposes genuine new acceptance independently of old conservative pure progress; the driver refuses pending publication and preserves a Persistence pause. Genuine later validated connects advance the projection and captured historical facts.
- Genuine prepared work now carries the checked sum of storage preparation, admitted preflight and bounded next-height fact capture. Preview and commit validate its compositional bound. This genuinely consumes the previously pending PreparedBasicFilterReorg::work interface; it is not a dummy getter call.

## Exact Interfaces and Source Routes

The lineage method remains restricted to crate::chainstate:

```rust
ValidatedChainstateLineage::preflight_required_sources(
    &self,
    staged: &StagedChainstateReorg,
    replacements: &[AnchoredBlock],
    positions: &[ChainPosition],
    undo: &HashMap<BlockHash, BlockUndo>,
    processed: FilterCheckpoint,
) -> Result<TurnWork, StorageError>

FjallNodeStore::basic_filter_required_undo(
    &self, hash: BlockHash, expected: &BlockUndo,
    work: &mut TurnWork, maximum: TurnWork,
) -> Result<BlockUndo, StorageError>

FjallNodeStore::maybe_basic_filter_reusable_record(
    &self, position: &ChainPosition, work: &mut TurnWork,
    maximum: TurnWork,
    admit: impl FnOnce(&mut TurnWork, TurnWork)
        -> Result<Option<TurnWork>, StorageError>,
) -> Result<BasicFilterReusableRead, StorageError>

// cfg(test), observes a genuine preparation and grants no authority:
PreparedChainstateReorg::maybe_basic_filter_preparation_work(
    &self,
) -> Option<TurnWork>
```

BasicFilterReusableRead has Missing, Deferred and Ready(StoredFilterRecord) variants with internal observation/consumption methods. The physical-row deletion helper is cfg(test), deletes only its named body/undo row, and constructs no authority.

Nonempty production append calls the already implemented manager.authorize_basic_filter_append_positions followed by store.prepare_basic_filter_replacement_append. This retains the bridge's admitted issuance/preparation comparisons and captured proof checks in the achieved turn ledger.

Actual network path:

```text
mempool_lifecycle::reorg_to_branch
  -> runtime_authority/filter_index/reorg::prepare_reorg
  -> ManagedChainstate::prepare_reorg
  -> prepare_basic_index_reorg
  -> lineage.preflight_required_sources
  -> chainstate/filter_reorg/preflight::required_sources
  -> only then install_prepared_reorg_preview and mempool effects
```

The sync path remains block_reconcile -> handle.reorg_to_branch. Missing replacement bodies retain BranchCompetitionAwaitingBodies; genuinely required old-source failures refuse explicitly. The existing RPC ordinary maintenance adapter already invokes the same parameter-free drive_basic_filter_index_turn; no worker or serving path was added.

The unchanged production_budget definition moved from network/runtime_authority/filter_index/catch_up.rs to chainstate/filter_reorg/preflight.rs. The former delegates through the narrow chainstate::basic_filter_turn_budget re-export; the whole filter_reorg module stays private. Plan 07/native source guards must follow this route. Likewise retain Plan 03's ownership/proofs.rs proof-acquisition route when refreshing guards.

## Resource Evidence and Limits

The preflight borrows the existing scheduler source envelope; storage preparation and captured next-height facts retain their independently established limits. The total permitted preparation ledger is the checked sum of those three ceilings, and actual composed work is checked before preview/commit. No unmeasured new numeric cap was finalized.

Native JSON length conservatively bounds every possible decoded element before serde; exact genuine undo equality follows decoding. Replacement txid/merkle validation reserves the existing four-copy body allowance before validation. Script and undo metadata passes are admitted before their deeper loops. These are logical resource reservations, not RSS or whole-node constant-memory claims. Existing consensus staging, preview, metadata persistence and mempool history costs remain separate. Plan 06 owns integrated measurements and any justified calibration.

The inherited fixed eight-record workload produced these actual values, constant across prefixes 16, 128 and 512:

| Counter | Historical Phase 157 expectation | Current observed expectation | Delta |
| --- | ---: | ---: | ---: |
| Record operations | 70 | 98 | +28 |
| Projection operations | 36 | 44 | +8 |
| Checkpoint operations | 5,415 | 7,621 | +2,206 |
| Indexed point reads | 40 | 56 | +16 |

The added immutable absence probes and genuine accepted-position proof/validation work are charged. Eight body reads/decodes, undo borrows and generations, body bytes <= 68,000, cloned bytes < 5,000,000 and one persistence batch remain unchanged. The first actual observation had 67,680 body bytes and 4,511,390 cloned bytes. The smallest-prefix full ancestry helper required 64 point reads; the counterfactual is now strengthened to exceed the actual turn's 56 reads. Historical Phase 157 reports were preserved; current deltas belong to this summary and Plan 06.

Resource/malformed-input errors retain the existing index_corruption/CoinsStorage representations, with explicit required-source mapping in sync. This plan introduces no broad operator diagnostic taxonomy; further projections remain later scope.

## Verification Evidence

All Cargo commands used Bun 1.3.9 through the timing/cooperative-lock runner and Rust 1.94.1. Target access was serialized. Counts below are actual harness results; the 15 new tests are a subset of the 66 Phase 158 tests.

| Timing key / check | Actual result |
| --- | --- |
| Initial phase158-production-behavior-red | 0 passed / 2 failed: corrupted physical undo was concealed; scheduled replacement lacked accepted-position permission |
| Earlier phase158-production-green | 2 passed, zero failed/ignored, 1.13s harness |
| Earlier phase158-production-matrix | 7 passed / 3 failed; three fixtures incorrectly used PreserveSaved rather than explicit configured Enabled policy; fixed |
| phase158-production-format-resume / format-ledger | Cargo workspace formatting passed |
| phase158-production-style-resume | Latest foreign-undo fixture compiled; strict all-target/all-feature Clippy -D warnings passed, 5.33s |
| phase158-production-tests-resume | 15 passed, zero failed/ignored, 6.80s harness |
| phase158-production-allphase-resume | 66 passed, zero failed/ignored, 38.72s harness |
| Initial phase158-production-phase157-regressions | 128 passed / 1 failed / 1 existing timing experiment ignored, 291.55s; stale exact record-operation expectation 70 versus observed 98 |
| phase158-production-ledger-observe-red | Focused RED observed all four current counters and full ancestry reads 64 |
| phase158-production-ledger-green | One test covering all three prefixes passed, 16.40s |
| phase158-production-style-ledger-final | Strict all-target/all-feature Clippy -D warnings passed, 4.48s; no -A or added source allow/expect |
| phase158-production-phase157-final | 129 passed, zero failed, one existing explicit timing experiment ignored, 284.89s |
| Breadcrumb --check | 997 currently tracked Rust files passed; all five new modules manually matched exact registered comments and pinned source paths |
| Bright Builds all / scoped diff check | Zero findings; git diff --check clean; actual changed paths reviewed |
| Stub / threat surface scan | No known production stub or new unmodeled endpoint/schema/key/access boundary |

The concrete new cases cover missing body versus undo before preview/mempool effects; corrupt undo; genuine consensus-staged same-height foreign undo with a different spent script; equal-height reorg; later bounded work after stage consumption; unavailable common-row reuse; stale already-owned append; truthful acceptance after publication failure; full disconnect; a genuine later connect with historical/same-block facts; actual sync reconciliation/refusal; and real handle-drop/configured-reopen rebinding after a concrete post-accept body-writer failure.

Fixtures use actual Fjall, production handle/sync callers and genuinely staged continuous spends with explicitly test-only coinbase maturity one. Named row deletion is Unavailable evidence, not actual paired-prune evidence. Successful live reorg/later turns need no reopen or re-enable. The poisoned body-publication failure is separately tested through a genuine closed reopen, which is fault recovery rather than a normal-success workaround. Plans 05/06 retain broader fork/fault/paired-prune/measurement obligations.

## Host Boundary and Resumption

Two earlier newly linked executables remained before the Rust harness. Child samples showed only _dyld_start, zero CPU and 112KiB footprint; read-only codesign verification passed. The first stalled child was stopped with SIGTERM only after over 30 minutes of invariant concrete non-progress and preserved diagnostics. Its receipt is exit 101 / signal 15, not a passing test or a command timeout. The next pre-harness retry produced the saved 158-04-CHECKPOINT.md and a root-managed external-host handoff.

The user's later explicit continue-and-retry authorization resumed this same lifecycle. Old PIDs were gone; current commands entered Rust and produced the positive results above. No host cause or actual restart is inferred, and no host security/cache changes or foreign-process control occurred. The checkpoint preserves the historical diagnostics, exact samples, failed/terminated receipts and resume contract; it is superseded by these earned results.

## Deviations and Simplification

1. **[Rule 2 - Correctness; root-amended] Native full-undo preflight.** Shape-only projection and in-memory availability could conceal a physically missing or other-branch durable mate. The private lineage wrapper plus native admitted reader verifies actual inputs before effects. Five new modules are registered; no public store/trait authority factory was added.
2. **[Rule 2 - Correctness; root-amended] Common nonempty position authorization.** Recovery can have hidden physical suffix rows without maybe_replacement_target. Both authority modes use genuine current accepted positions. Empty checkpoint-only publication retains ordinary own-fence checks.
3. **[Rule 2 - Resource accounting; root-amended] Composed preparation work and admitted reuse.** Actual storage/source/capture work is retained and checked. Missing/Deferred/Ready prevents accidental fallback regeneration. The common budget was moved unchanged into the lower reusable adapter to avoid reverse layer dependence.
4. **[Rule 3 - Verification; root-authorized] Exact inherited counter refresh.** Actual RED observations justified only the four new exact expectations and stronger full-ancestry comparison. Every body/byte/batch/prefix assertion and historical measurement report remains intact.
5. **[Rule 3 - Host launch] Evidence-preserving handoff.** The native host boundary required a checkpoint and later user-authorized retry; no failing execution was called successful.

The simplification pass reused the existing native body scanner and shared append machinery, kept one budget definition and one nonempty authorization path, and split fault controls instead of growing the main test module. No new worker, authority flag, retained stage or duplicate history cache was needed. The ordered admission/generation/prepare/complete sequence stays explicit.

## Task Commits and Remaining Gates

Task and metadata commits remain deferred to the strict root wrapper. Root retains state/roadmap/todo updates, CFIX-03 activation, full native build/tests/coverage/Bazel, independent source/security review and formal lifecycle validation. This plan's positive local evidence does not claim whole-phase completion, filter serving, public defaults, archive scale, production readiness or funds safety.

Material guidance came from the repo AGENTS/Repo-Local Guidance, Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/Rust standards, canonical phase decisions, implemented predecessor contracts and pinned Knots rewind/undo/fence seams. Both active lesson files were loaded completely in initial execution (7,188 bytes / 2,397 conservative tokens); no task correction or new audit trigger required an append. No authentication gate occurred.

## Self-Check: PASSED

All five new Rust modules, the historical checkpoint and this summary exist. The summary has only its two top frontmatter delimiters, preserves the exact originating lifecycle, and leaves requirements-completed empty. Actual final evidence is 15 new tests, 66 Phase 158 tests and 129 affected regressions passed; the sole ignored case is an existing explicit timing experiment. Latest-source formatting, strict Clippy, breadcrumb registration, policy checks and scoped diff review/check have positive evidence. No nonexistent commit or full-native success is claimed; all Git/state/activation and remaining phase gates stay root-owned.
