---
phase: 158-validated-reorg-and-retained-branch-identity
plan: "02"
subsystem: storage
tags: [rust, fjall, compact-filters, reorg, recovery, capabilities]
requires:
  - phase: 158-01
    provides: Private genuine stages, absorbed endpoint receipts and pure replacement progress
provides:
  - Same-store tracked absorption authorization and achieved SyncAll branch rewind
  - Authenticated bounded projection replacement and recovered-prefix rebinding
  - Exact displaced durable-coins fence binding until own flush completion
  - Genuine concrete Fjall publication, refusal, fault and reopen evidence
affects: [158-03, 158-04, 158-05, 158-06, 158-07]
tech-stack:
  added: []
  patterns: [sealed tracked lineage, borrowed accepted positions, guarded SyncAll publication]
key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/fjall_store/reorg.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store/tests.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg/faults.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/filter_index.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Accepted absorption records pending lineage before effects and only achieved publication confirms it
  - Replacement authority borrows bounded accepted positions from the genuine manager without retaining a stage
  - Displaced fencing binds the original durable height and hash exactly and clears only on own flush confirmation
  - Recovery projection authority is distinct from live replacement and requires positions for every existing forward projection row
  - All Git and phase-state finalization remain root-owned after full phase verification
patterns-established:
  - Public endpoints, generic construction and preview cannot authorize projection rebinding
  - Recovered hidden projections are checked per requested row rather than with an install-time flag or suffix scan
requirements-addressed: [CFIX-03]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: "2026-10-08T03:54:40Z"
duration: 46min
completed: 2026-10-08
---

# Phase 158 Plan 02: Authenticated Fjall Rewind and Projection Replacement Summary

**Tracked absorbed reorgs publish bounded atomic rewinds, and genuine accepted-position authority rebinds live or recovered projections while preserving immutable headers and exact coins fencing.**

## Performance and Scope

- First measured RED command: 2026-10-08T03:08:34.550Z. Final scoped checks completed by 2026-10-08T03:51:01Z; summary/self-check completed at 2026-10-08T03:54:40Z, approximately 46 minutes including preparation and the amended recovery closure.
- Tasks: 3/3 implemented and locally verified under the amended Plan 02 scope.
- Implementation/registry files: 15, including six new Rust modules; this summary is the sixteenth owned file.
- Baseline HEAD remains `2f21ac2052208c7e7a084ed006d908c5ccb714aa` on the shared main checkout.
- No dependency, crate, public endpoint, persistent schema or key was added. Existing publication.rs SyncAll/fault machinery is reused without changes.

## Accomplishments

- Preparation reads the genuine stage's captured displaced tip, new endpoint and ancestor. It checks the same-store captured proof and bounded immediate record/projection identities, retains lagging indexed progress below the ancestor, and separately rewinds safe progress. Generation and revision advance through checked exact addition.
- Only tracked lineage receiving the genuine absorbed receipt can authorize publication. It records the new optional accepted endpoint and pending transition before effects. Pending, stale, failed or ambiguous transitions cannot prepare or complete a validated flush. A full disconnect publishes Empty progress and cannot mint a nonempty flush receipt.
- One guarded SyncAll batch publishes generation, checkpoint and the complete stronger/equal reserved protection map. Existing ordinary locks survive. Immutable record keys and suffix projection keys are never deleted; the checkpoint masks incompatible suffixes. Before-record/checkpoint/protection faults preserve the prior batch, while AfterCommit poisons the writer and returns no achieved capability.
- A bounded borrowed slice from the live genuine manager authenticates every admitted record's height, hash and parent. Both issuance and preparation compare those facts, and completion rechecks the captured store/generation/branch/revision/frontier. No stage, chain vector, undo map or history cache is retained by the permission.
- Live replacement preserves the original durable coins `(height, hash)` in `maybe_displaced_fence`. Index safe progress remains on the shared prefix until genuine own coins plus metadata completion. The narrow coins.rs exception checks that marker against the exact previous append identity's durable tuple; existing actual coins/fence and own-completion checks remain intact.
- Trusted exclusive recovery installs `BasicFilterProjectionAuthority::RecoveredPrefix`, distinct from `ValidatedReorg`, without minting live replacement or displaced-fence facts. Any requested existing projection above processed progress requires sealed positions, including equal-hash stale replay and a later hidden row after a first missing row. Fresh missing projections retain existing generic behavior. The check uses the existing bounded per-record projection read and never scans a suffix.

## Exact Contracts for Plans 03 and 04

All following capabilities have private fields, no Clone/From/Default/Deserialize/raw/test constructor, and only read-only validation/fact access. FjallNodeStore is the existing cloneable store handle; these capabilities are not cloneable.

```rust
FjallNodeStore::prepare_basic_filter_reorg(
    &self, proof: &BasicFilterAppendProof, staged: &StagedChainstateReorg,
) -> Result<PreparedBasicFilterReorg, StorageError>

ValidatedChainstateLineage::authorize_reorg(
    &mut self, accepted: &AcceptedChainstateReorg,
    prepared: &PreparedBasicFilterReorg,
) -> Result<ValidatedBasicFilterReorg, StorageError> // pub(in crate::chainstate)

ValidatedBasicFilterReorg::validate_for(
    &self, store: &FjallNodeStore, prepared: &PreparedBasicFilterReorg,
) -> Result<(), StorageError>

FjallNodeStore::complete_basic_filter_reorg(
    &self, prepared: PreparedBasicFilterReorg,
    authorization: ValidatedBasicFilterReorg,
) -> Result<CompletedBasicFilterReorg, StorageError>

ValidatedChainstateLineage::confirm_reorg(
    &mut self, achieved: &CompletedBasicFilterReorg,
) -> Result<(), StorageError> // pub(in crate::chainstate)

ManagedChainstate<FjallChainstateStore, FjallCoinsView>
    ::authorize_basic_filter_append_positions<'a>(
        &'a self, proof: &BasicFilterAppendProof, records: &[StoredFilterRecord],
    ) -> Result<ValidatedBasicFilterAppendPositions<'a>, StorageError>

FjallNodeStore::prepare_basic_filter_replacement_append(
    &self, proof: BasicFilterAppendProof,
    positions: ValidatedBasicFilterAppendPositions<'_>,
    records: &[StoredFilterRecord],
) -> Result<PreparedBasicFilterAppend, StorageError>
```

The manager producer has the exact amended record-slice signature, replacing the temporary count-only development form. Plan 04 can issue it after bounded generation and before preparation, for live replacement and recovered hidden projection work. It does not need an absorbed stage. Empty record slices are valid for earned checkpoint promotion after an own flush.

PreparedBasicFilterReorg supplies `belongs_to`, `maybe_old_endpoint`, `maybe_new_endpoint`, `maybe_ancestor_endpoint`, `old_identity`, `achieved_identity`, `processed`, `safe_checkpoint`, `protection`, `work`, and static `maximum_work`. Identities are `(IndexGeneration, BlockHash, u64 revision)`; endpoint values are optional `(u32 height, BlockHash)`. CompletedBasicFilterReorg::prepared borrows these achieved facts. The private pending lineage compares all captured endpoint and incarnation facts before clearing its pending state.

The manager's internal accepted endpoint is now `maybe_accepted`; PendingValidatedFlush still carries a definite accepted tuple. Original observe/flush behavior remains unchanged for nonempty normal growth. Only narrow crate-internal capability and test-fixture re-exports were added to chainstate.rs; its entire adapter module remains private.

## Work Accounting and Limits

- Reorg prepare/authorization/complete use bounded local endpoint/parent reads and admitted complete lock maps. Authorization's counted proof acquisition is added to the achieved preparation ledger before effects. Completion reserves lock encoding, state/owner/protection encoding, bounded point rechecks and durable coins checks before committing.
- Reorg admission reuses existing scheduler acquisition maxima: 1 GiB cloned bytes, 512 record operations, 1,000,000 checkpoint operations and 256 projection operations. It has no candidate generation/body/undo/script work. The larger existing singleton clone ceiling preserves admission for legal large rows instead of imposing the smaller normal-turn clone cap. These are reused limits, not new calibration evidence.
- Position issuance records its bounded proof checks, `6 * admitted_count + 4` checkpoint operations for issuance/preparation comparisons and fixed token size. Its work is admitted before comparisons and incorporated in PreparedBasicFilterAppend. Existing predecessor, immutable equality, projection, batch and completion work stays charged.
- Lineage confirmation performs a separately bounded current proof check. Plan 06 must measure the integrated reorg/confirmation/controller costs and calibrate limits; this summary claims no whole-runtime constant-memory or calibrated latency result. Genuine staging and existing metadata flush still clone/serialize chain/undo data outside this new constant-size storage capability.

## Verification Evidence

All Cargo checks used pinned Bun 1.3.9 through scripts/command-timings.ts with the cooperative target lock and Rust 1.94.1. Commands were serialized and live test sessions were polled; no build/test was interrupted.

| Check | Actual result |
| --- | --- |
| Initial `phase158-store-bridge-red` | Exit 101 before the new APIs; timing receipt records 03:08:34.550Z–03:08:42.967Z |
| `phase158-store-append-red` | Exit 101: missing genuine fixture integration and dependent unresolved types |
| Own-flush behavior RED | 7 passed / 2 failed; genuine replacement flush failed `incompatible BASIC durable metadata publication`; the other failure was a fixture's incorrect Empty-prefix expectation |
| `phase158-recovery-rebind-red` | Specific genuine reopened-manager test failed `BASIC replacement requires achieved reorg` |
| `phase158-recovery-stale-red` | Specific same-projection old-branch replay test failed because generic preparation accepted the stale row |
| Earlier standalone bridge selector | 5 passed, zero failed/ignored |
| Final `cargo test -p open-bitcoin-node --lib phase158`, key `phase158-node-complete` | 37 passed, zero failed/ignored; 5 bridge and 32 storage/recovery/control cases; 18.75s test runtime |
| Final `cargo test -p open-bitcoin-node --lib phase157_append`, key `phase158-node-append-regressions` | 35 passed, zero failed/ignored; 19.70s test runtime after recovery closure |
| Broader `--lib phase157`, key `phase158-node-regressions` | 129 passed, zero failed, one existing explicit timing experiment ignored; 291.83s, before the final recovery amendment |
| Cargo workspace format | Passed after final source/test additions |
| Focused all-target/all-feature Clippy `-D warnings -A dead_code`, key `phase158-node-style-focused` | Passed; only the documented pending production-consumer diagnostics were excluded by the command |
| Strict all-target/all-feature Clippy `-D warnings`, key `phase158-node-style-strict` | Exit 101: exactly 17 dead_code groups listed below; no other diagnostics |
| Bright Builds checker `all` | Zero findings; all six new Rust files manually confirmed below 628 lines |
| Parity breadcrumb checker `--check` | 997 currently tracked Rust files passed; all six untracked new modules have exact comments and manifest entries |
| Named test discovery and scoped diff review / `git diff --check` | Passed |

The final 37 tests include genuine historical and same-block spends, equal-height and longer replacement, lagging progress, complete disconnect, foreign/generic/preview/stale refusal, counter exhaustion, stronger reserved and ordinary locks, immutable equality/conflict, exact own-flush promotion, raw metadata/no-achievement/actual-coins mismatch controls, twelve rewind/first-turn/later-turn publication fault cases, and actual recovered projection rebinding. New source files remain intentionally untracked under the no-staging gate, so root must rerun the tracked-file checker after final staging includes them.

## Exact Pending Strict-Clippy Diagnostics

Strict Clippy has **17 diagnostic groups across five files**, all caused by future production consumers. No source allow/expect attribute, test-only production path or hook bypass was added. This is an explicit partial-wave limitation, not a strict-Clippy pass.

| File | Groups | Exact unused symbols/fields | Consumer closure |
| --- | --- | --- | --- |
| chainstate/filter_index.rs | 1 | `ManagedChainstate::authorize_basic_filter_append_positions` | Plan 04 driver |
| chainstate/fjall_store/reorg.rs | 5 | PendingBasicFilterReorg fields `old_identity`, `achieved_identity`, `maybe_old`, `maybe_new`, `maybe_ancestor`; `ValidatedBasicFilterReorg`; its `work`/`validate_for`; PendingBasicFilterReorg `from_prepared`/`matches`; ValidatedChainstateLineage `authorize_append_positions`/`authorize_reorg`/`confirm_reorg` | Plans 03/04 |
| storage/fjall_store/filters/append.rs | 2 | `FjallNodeStore::prepare_basic_filter_replacement_append`; `BasicFilterAppendProof::admit_work` | Plan 04 |
| storage/fjall_store/filters/reorg.rs | 8 | `BASIC_INDEX_TURN_WORK_LIMIT`; `PreparedBasicFilterReorg`; `CompletedBasicFilterReorg`; Prepared methods `maximum_work`, `belongs_to`, `maybe_old_endpoint`, `maybe_new_endpoint`, `maybe_ancestor_endpoint`, `old_identity`, `achieved_identity`, `processed`, `safe_checkpoint`, `protection`, `work`; Completed `prepared`; FjallNodeStore `prepare_basic_filter_reorg`/`shared_basic_filter_prefix`/`complete_basic_filter_reorg`; `stronger_protection`; BasicFilterAppendProof `maybe_replacement_target` | Plan 03 and Plan 06 accounting |
| storage/fjall_store/filters/ownership.rs | 1 | `BasicFilterAppendProof::bounded_reorg_copy` | Plan 03 |

Root's final native verification must pass strict `-D warnings` after real consumers exist. The focused lint does not replace that requirement.

## Task Commits and State Ownership

Task commits, metadata commits, staging and pushing are deferred by the strict parent wrapper. No Git mutation or worktree operation occurred. Root owns STATE.md, ROADMAP.md, todo, requirement activation, full native coverage/build/Bazel, independent source/security review and final lifecycle/Git operations. CFIX-03 is addressed here and remains unactivated.

## Deviations and Amendments

1. **[Rule 2 - Correctness; root-approved amendment] Sealed bounded current accepted-position authorization.** An absorbed stage cannot survive to later scheduled turns. The genuine manager now borrows current accepted positions for each generated bounded record slice, and storage consumes that permission into its existing owned prepared batch. This replaces the temporary stage/count-only interpretation without retaining history or introducing a raw factory.
2. **[Rule 3 - Blocking; root-approved amendment] Narrow displaced metadata ancestry exception.** The meaningful own-flush RED showed the pre-existing predicate rejected every genuine replacement. Only the achieved identity's exact old durable height/hash marker permits bypassing that old ancestry check; actual coins metadata fencing and sealed own receipts remain required. Positive and three negative controls execute.
3. **[Rule 1 - Recovery liveness; root-approved amendment] Distinct recovered-prefix rebinding.** Real reopen discarded live replacement facts while masked physical rows remained, making ordinary catch-up unable to replace them. Exclusive trusted recovery now installs distinct constant-size projection authority, consumed only with genuine positions for any existing forward requested projection. Specific RED/GREEN tests cover old coins, replacement coins/partial suffix, same-projection stale replay, sparse later-hidden rows and foreign proofs. Full ordinary-driver reopen evidence belongs to Plan 06 after wiring.
4. **[Rule 2 - Instruction enforcement] Module splits and scoped method placement.** Formatting would exceed the managed 628-line guidance. Existing proof tests, their genuine fixture, bridge logic and storage fault cases were split into the six declared modules; small specialized proof helpers stay with their append/reorg consumers. No file-length exception or managed-checker change was added.

The lagging fixture originally assumed Empty even though configured startup had genuinely indexed genesis. It now compares the captured valid genesis prefix against a higher fork ancestor, proving that rewind does not invent progress. Ordinary stronger-lock test recovery now reconstructs a genuine manager after actual reopen, preserving its branch incarnation rather than retaining stale lineage across exclusive recovery.

## Simplification and Remaining Integration

Preparation and completion reuse existing codecs, immediate-edge checks, full lock-map admission, SyncAll batches, and one-shot fault controls. Ordinary and authenticated append share one implementation; immutable equality checks are unchanged. The recovered-row predicate uses the already required point read for each admitted row, covering later hidden rows without a history cache or a permanent install-time approximation.

Plans 03/04 must wire the actual accepted/preview/paused owner, genuine lineage confirmation and scheduled driver. Plans 05/06 own production input refusal, full closed-reopen ordinary-driver recovery matrix and integrated resource evidence. Plan 07/root owns README/parity/UAT claims and final native/source/security/lifecycle checks. No runtime-reorg activation, serving capability, production/funds claim or CFIX-03 completion is asserted by this storage-only wave.

Material guidance came from AGENTS.md, the Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/Rust standards, Plan 01's exact APIs, pinned Knots rewind/hash-retention/coins-undo seams, and BIP157's chained-header commitment definition. Both active lesson files were read completely: 7,188 bytes / 2,397 conservative tokens; no new audit trigger. No stubs, new external service, authentication gate or unmodeled endpoint/schema/file-access trust boundary was introduced.

## Self-Check: PASSED

All six created Rust modules, all nine modified implementation/registry files and this summary exist. Final named tests, focused style checks, breadcrumb registrations and scoped diff checks have positive evidence. Baseline HEAD is unchanged. Commit-existence checks and root state updates are intentionally deferred; no nonexistent commit or strict-Clippy success is claimed.
