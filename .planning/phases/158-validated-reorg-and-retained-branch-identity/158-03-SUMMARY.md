---
phase: 158-validated-reorg-and-retained-branch-identity
plan: "03"
subsystem: chainstate
tags: [rust, compact-filters, reorg, accepted-owner, durable-fencing]
requires:
  - phase: 158-02
    provides: Sealed same-store absorbed-reorg publication and genuine accepted-position authority
provides:
  - Prepared-transition-bound preview suspension for ordinary and already-owned work
  - Genuine absorbed replacement owner restoration and independent accepted-target visibility
  - Ordinary own-flush replacement fencing without a forced reorg checkpoint
affects: [158-04, 158-05, 158-06, 158-07]
tech-stack:
  added: []
  patterns: [bound preview barrier, accepted versus achieved state, ordinary own-flush fencing]
key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/filter_reorg.rs
    - packages/open-bitcoin-node/src/chainstate/filter_reorg/tests.rs
    - packages/open-bitcoin-node/src/chainstate/filter_reorg/tests/faults.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership/proofs.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/filter_index.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store/reorg.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Preview suspension binds exact prepared old and achieved identities and survives ordinary invalidation
  - Record genuine optional accepted identity before publication while installing pure progress only after achievement
  - Propagate preview freeze failure before core preview and mempool effects
  - Preserve ordinary IfNeeded cadence and mark replacement fencing only after completed own coins and metadata receipts
  - Keep all Git and phase-state finalization root-owned
patterns-established:
  - Already-owned append batches, legacy work tokens and pending own-flush receipts are revoked at preview
  - Later genuine connects advance explicit accepted identity before fallible progress or persistence operations
requirements-addressed: [CFIX-03]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: "2026-10-08T04:32:18Z"
duration: 36min
completed: 2026-10-08
---

# Phase 158 Plan 03: Accepted Reorg Owner and Ordinary Flush Summary

**Genuine manager absorption restores the ordered replacement owner, preview blocks previously prepared publications, and ordinary own coins/metadata completion earns the replacement fence.**

## Performance and Scope

- Tasks: 2/2 implemented and locally verified under the amended Plan 03 scope.
- Approximately 36 minutes from the root's execution dispatch at 03:56 UTC through final targeted checks at 04:29 UTC and summary/self-check completion at 04:32 UTC.
- Four created Rust modules and ten modified predecessor/registry files; this summary is the fifteenth owned artifact.
- Shared-main baseline HEAD remains `2f21ac2052208c7e7a084ed006d908c5ccb714aa`. No staging, commit, push, worktree or hook bypass occurred.
- No production dependency, crate, network endpoint, persistent key/schema, forced coins flush, retained stage or second history cache was added.

## Accomplishments

- `PreparedChainstateReorg` privately carries the already-authenticated storage transition, a checked pure trial progress value and at most one genuine next-height replacement body/undo fact. The next block is selected by checked ancestor-relative offset and staged height lookup. After absorption the preparation is consumed; later append positions remain borrowed from the manager's current accepted chain through Plan 02's sealed bridge.
- Preview freezes the ordered owner while retaining the displaced accepted target and conservative processed/safe progress. The concrete publication control binds suspension to the prepared previous/next append identities and captured old/new/ancestor endpoints. It rejects new ordinary proofs/tokens, an already-owned append batch, an already-owned legacy work token and own-flush confirmation. Ordinary raw coins/metadata invalidation cannot clear suspension. Matching genuine absorbed authorization plus guarded achieved publication is the only live transition that clears it.
- Effectful preview now returns a `Result`. A failed store freeze returns before core preview or mempool effects; the sole production caller propagates it with `?`. A failure after a successfully installed preview preserves the old accepted index identity and explicit suspension; this does not claim rollback of the existing previewed chainstate/mempool contract.
- Both direct `ManagedChainstate::reorg` and the existing prepared network path use the same manager acceptance sequence. The manager consumes `AcceptedChainstateReorg`, records the genuine optional new target and releases displaced next-height facts before index publication or ordinary persistence. It authorizes, completes and confirms Plan 02's genuine same-store bridge; only then does it install the checked replacement progress and its bounded next-height fact. Successful nonempty replacement restores ordinary owner readiness without reopening or re-enabling.
- Failed publication keeps explicit accepted replacement identity visible while leaving prior pure processed/safe progress paused and unclaimed as a new achievement. Failed later normal persistence keeps the genuinely restored replacement target visible and pauses with `Persistence`. `maybe_basic_index_accepted_target` distinguishes acceptance from the pure achieved progress snapshot; later genuine connects update this identity before any fallible observer/persistence operation.
- Successful ordinary own coins plus metadata confirmation marks `DurablyFencedReplacement`. Existing IfNeeded/Periodic/Always and automatic-prune cadence remain responsible for coins durability. Accepted-unflushed lower/equal/higher replacements retain the displaced durable coins endpoint and shared safe checkpoint. Authenticated record append and later ordinary own flush plus earned zero-record publication promote only the actual replacement fence. Full disconnect records `None`, publishes the storage Empty checkpoint, keeps the definite-target pure scheduler paused, and returns no nonempty pending flush receipt.

## Exact Interfaces and Next Consumer Contract

```rust
ManagedChainstate<S, V>::install_prepared_reorg_preview(
    &mut self, prepared: &PreparedChainstateReorg,
) -> Result<(), ChainstateError>

ManagedChainstate<S, V>::maybe_basic_index_accepted_target(
    &self,
) -> Option<AcceptedIndexTarget>

ValidatedChainstateLineage::prepare_reorg_storage(
    &self, staged: &StagedChainstateReorg,
) -> Result<PreparedBasicFilterReorg, StorageError>

ValidatedChainstateLineage::freeze_index_reorg(
    &mut self, prepared: &PreparedBasicFilterReorg,
) -> Result<(), StorageError>

ValidatedChainstateLineage::check_index_reorg(
    &self, prepared: &PreparedBasicFilterReorg,
) -> Result<(), StorageError>

ValidatedChainstateLineage::complete_index_reorg(
    &mut self, accepted: &AcceptedChainstateReorg,
    prepared: PreparedBasicFilterReorg,
) -> Result<(), StorageError>

ValidatedChainstateLineage::reorg_is_pending(&self) -> bool

FjallNodeStore::suspend_basic_filter_reorg(
    &self, prepared: &PreparedBasicFilterReorg,
) -> Result<(), StorageError>

FjallNodeStore::check_basic_filter_reorg_suspension(
    &self, prepared: &PreparedBasicFilterReorg,
) -> Result<(), StorageError>
```

The lineage methods remain `pub(in crate::chainstate)`; concrete storage methods remain crate-internal. No public construction path or test factory was introduced. `complete_index_reorg` wraps the actual Plan 02 `authorize_reorg -> complete_basic_filter_reorg -> confirm_reorg` sequence.

The private owner state is `Ordinary`, `PreviewFrozen`, `AcceptedReplacement { maybe_target }` or `DurablyFencedReplacement { maybe_target }`. The accessor returns the displaced target during preview and the genuine optional replacement target after absorption, including publication failure or full-disconnect absence. The existing `BasicIndexProgress` has a definite target, so a failed/empty replacement does not fabricate a new processed/safe snapshot. Plan 04 must consume the accepted-target accessor when reporting target/lag/pause and use the existing genuine accepted-position permission for replacement/recovered projection preparation. The ordinary driver's production replacement consumer is still Plan 04; this wave does not claim its end-to-end completion.

Proof acquisition moved from `filters/ownership.rs` into its private child `filters/ownership/proofs.rs`. Ordinary getters refuse suspension. The separate crate-internal `maybe_basic_filter_reorg_proof_with_budget` reads the same bounded, same-store old proof during the privileged transition; it grants no append/flush authority, and completion still requires matching sealed preparation and genuine absorption. Historical source guards in Plan 07 must follow this method move if they inspect the former file text.

## Verification Evidence

All Cargo checks used the pinned Bun 1.3.9 timing/lock runner and Rust 1.94.1. Cargo target access was serialized; quiet resumable commands were polled and no command was interrupted.

| Check / timing key | Actual result |
| --- | --- |
| Initial `phase158-manager-reorg-red` | Exit 101: test setup used the incorrect argument-taking `prepare_turn` form; corrected before behavior RED |
| `phase158-manager-reorg-behavior-red` | 0 passed / 2 failed: preview permitted an already-owned append to publish; equal-height reorg retained the displaced accepted owner target |
| `phase158-manager-reorg-green` | 2 passed, zero failed/ignored, 1.25s harness |
| `phase158-manager-reorg-matrix` | 9 passed, zero failed/ignored, 9.73s harness |
| `phase158-manager-node-final` | 51 passed, zero failed/ignored, 30.21s harness, before final direct-offset simplification |
| Final `phase158-manager-node-simplification-final` | 51 passed, zero failed/ignored, 32.73s harness on final source; 14 new manager tests plus all 37 predecessor bridge/storage/reopen tests |
| `phase158-manager-append-regressions` | 35 passed, zero failed/ignored, 20.60s harness on final source |
| Final Cargo workspace format | Passed |
| `phase158-manager-style-simplification`, all-target/all-feature Clippy `-D warnings -A dead_code` | Passed, 10.36s; excludes only the named pending production-consumer category |
| `phase158-manager-style-strict-final`, Clippy `-D warnings` | Exit 101: exactly seven dead_code groups below, no other diagnostics |
| Bright Builds checker `all` | Zero findings; all new/touched modules below 628 lines after format |
| Parity breadcrumb checker `--check` | 997 tracked Rust files passed; four new modules manually verified against exact registered comments and existing pinned source paths |
| Named discovery / scoped diff / stub scan | Fourteen named manager tests discovered, `git diff --check` passed, no new stubs |

The concrete manager cases cover preview old-batch/legacy-token/new-issuance rejection; previously owned pending-flush revocation; freeze failure before core preview; raw writer invalidation without unfreezing; old identity after pre-absorb failure; lower/equal/higher and lagging branches; public direct-path delegation; full disconnect; visible accepted identity after all four rewind publication faults; genuine next connect; ordinary later undo/coins/metadata errors; immediate commit errors when existing normal IfNeeded pressure legitimately flushes; and genuine later own-flush promotion. Storage controls additionally retain all predecessor foreign/generic/preview/raw/actual-coins/fault/reopen evidence.

BeforeBody evidence deliberately uses the actual concrete body adapter's retained-body write after accepted-unflushed reorg, followed by refusal of the manager's ordinary flush. `flush_window` has no body payload responsibility, so this test does not claim production network writer orchestration; Plans 04/06 own that full boundary. No extra coins-write counter was introduced: unchanged actual coins B, shared safe checkpoint, normal flush decision, pending receipt checks and later achieved own flush are the evidence for preserved cadence.

Several rebuilt test executables spent approximately three to six minutes before harness entry. A liveness sample showed only `_dyld_start` with 112 KiB footprint and zero CPU, rather than a Rust test hang. Each subsequently ran to completion. These host launch delays are separate from the reported test runtimes and are not calibrated reorg latency measurements.

## Exact Remaining Strict-Clippy Diagnostics

Seven groups remain; no source allow/expect attribute, dummy consumer or hook bypass was added. Strict Clippy is **not** claimed as passing.

| File | Exact symbol | Required production consumer |
| --- | --- | --- |
| chainstate/filter_index.rs | `authorize_basic_filter_append_positions` | Plan 04 |
| chainstate/filter_index.rs | `maybe_basic_index_accepted_target` | Plan 04 |
| chainstate/fjall_store/reorg.rs | `authorize_append_positions` | Plan 04 |
| storage/fjall_store/filters/append.rs | `prepare_basic_filter_replacement_append` | Plan 04 |
| storage/fjall_store/filters/append.rs | `BasicFilterAppendProof::admit_work` | Plan 04 |
| storage/fjall_store/filters/reorg.rs | `PreparedBasicFilterReorg::work` | Plan 06 accounting |
| storage/fjall_store/filters/reorg.rs | `BasicFilterAppendProof::maybe_replacement_target` | Plan 04 |

All Plan 03 bridge/publication production-consumer diagnostics from Plan 02 are closed. Root's full native verification must pass strict warnings after the remaining genuine consumers exist.

## Deviations and Simplification

1. **[Rule 2 - Correctness; root-approved amendment] Store-bound preview barrier.** A producer pause alone did not revoke already-owned work. Suspension now binds exact prepared identities under the existing publication guard and gates ordinary proof/token acquisition, append and legacy completion, and flush confirmation. Raw invalidation preserves the barrier. The meaningful RED and positive/negative manager tests prove the boundary.
2. **[Rule 2 - Error handling; root-approved amendment] Effectful preview returns Result.** The previous void API could hide a freeze failure before mempool effects. Its sole production caller now propagates the result. A genuine invalidated-preparation test proves core preview and accepted progress remain unchanged on refusal.
3. **[Rule 2 - Accepted-state correctness; root-approved amendment] Separate optional accepted-target accessor.** Failed publication cannot safely install achieved pure progress, but genuine accepted identity must remain visible. The accessor reads the explicit owner state, and later genuine acceptance advances it before fallible observers. Preview, failed publication, absence and subsequent child acceptance have tests.
4. **[Rule 3 - Module size] Private proof/test splits.** Proof acquisition moved to `ownership/proofs.rs`; five additional manager controls live in `filter_reorg/tests/faults.rs`. All four new files have registered exact breadcrumbs, and no checker exception was added.

The simplification pass removed a redundant lineage wrapper and replaced nested replacement/connected scans with checked direct selection. Existing sealed `maybe_replacement_undo(hash)` still performs a membership scan internally, and genuine stage/preview plus normal metadata persistence have existing history costs. No whole-runtime constant-memory/constant-time or newly calibrated bound is asserted. Plan 06 must measure those costs separately from the bounded storage capabilities.

## Task Commits, State Ownership and Remaining Integration

Task and metadata commits, staging and push remain deferred to the strict root wrapper. STATE.md, ROADMAP.md, todo, requirement activation, full native format/lint/build/tests/coverage/Bazel, independent source/security review and formal lifecycle validation are root-owned. CFIX-03 is addressed and remains unactivated.

Plan 04 owns the actual scheduled replacement/recovered append consumer and accepted-target observation. Plan 05 owns complete durable retained-input preflight/refusal. Plan 06 owns integrated driver/reopen and measured work evidence. Plan 07/root owns parity/readme/UAT claims and final native/source/security/lifecycle gates. No serving, public default, production/funds, whole-phase or full-native success claim is made here.

Material guidance: repo AGENTS/Repo-Local Guidance, Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/Rust standards, canonical phase contexts, actual Plan 01/02 capabilities and pinned Knots rewind/flush/undo seams. Both active lesson files were loaded completely: 7,188 bytes / 2,397 conservative tokens; no new audit trigger. The ancestry-dependent commitment definition was checked against [BIP157](https://bips.dev/157/). No authentication gate, known stub or unmodeled external endpoint/schema/file-access boundary was introduced.

## Self-Check: PASSED

All four created Rust modules and this summary exist. Final-source phase tests (51), affected append regressions (35), focused lint, formatting, breadcrumb registration, file-size and scoped diff checks have positive evidence. Only the opening/closing frontmatter delimiters are standalone `---`. Baseline HEAD is unchanged; nonexistent commits are not claimed, and commit/state checks remain intentionally root-deferred. The seven strict-Clippy consumer gaps and later production/native evidence are explicitly listed above.
