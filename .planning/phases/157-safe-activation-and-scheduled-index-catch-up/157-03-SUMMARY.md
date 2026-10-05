---
phase: 157-safe-activation-and-scheduled-index-catch-up
plan: "03"
subsystem: chainstate
tags: [rust, basic-filters, ordered-progress, work-budgets, pure-core]
requires:
  - phase: 154-basic-generation-and-commitment-parity
    provides: Body-bound BASIC inputs and immutable filter commitments
  - phase: 155-recoverable-index-and-pre-prune-startup-protection
    provides: Verified recovered chainstate fence and conservative checkpoints
  - phase: 156-index-owned-manual-and-automatic-prune-coordination
    provides: Checked lifecycle generations and reserved input protection
provides:
  - Constant-size ordered accepted, processed and safe durable progress reducer
  - Immutable generation, branch and prior-frontier preparation snapshots
  - Checked ten-counter admission with normal and explicit absolute singleton limits
affects: [157-04, 157-05, 157-06, 157-07, 157-10]
tech-stack:
  added: []
  patterns: [functional-core, bounded-suffix-reduction, checked-pre-work-reservation]
key-files:
  created:
    - packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs
    - packages/open-bitcoin-chainstate/src/filter_index/catch_up/budget.rs
    - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests.rs
    - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/budget.rs
    - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/durability.rs
  modified:
    - packages/open-bitcoin-chainstate/src/filter_index.rs
key-decisions:
  - "Keep normal aggregate and absolute singleton limits independent; several legal blocks can exceed a one-block maximum in aggregate."
  - "Prepared work cannot process above its captured accepted target, although later accepted connects can enlarge the backlog."
  - "Durability correspondence is checked against VerifiedChainstateFence; storage authority and complete prefix proofs remain private adapter responsibilities."
  - "Keep production budget defaults measurement-pending for Plan 07."
patterns-established:
  - "Failed reducers preserve their complete prior state, including accepted target, safe checkpoint and conservative protection."
  - "A disabled owner cannot be resurrected by pause/resume; checked disable advances its generation before release."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T16:48:31Z
duration: 21min
completed: 2026-10-05
---

# Phase 157 Plan 03: Pure Ordered Owner and Budget Contracts Summary

**One pure ordered owner separates accepted targets, contiguous processed records, initial completion and safe durability, with checked admission for every turn resource.**

## Performance

- Started: 2026-10-05T16:24:28Z
- Completed: 2026-10-05T16:45:42Z
- Duration: 21 minutes
- Tasks: 2/2
- First-party Rust files created/modified: 6; summary: 1

## Accomplishments

- `BasicIndexProgress` has private fields and fallible construction. Durable progress cannot exceed processed progress, processed progress cannot exceed the accepted target, equal-height identities must match, adjacent endpoint parent/header facts must agree, and protection must cover safe durability. More distant ancestry is supplied by the adapter's complete prefix proof rather than an index-owned chain cache.
- The next input is genesis for an empty prefix or the processed endpoint's checked successor. Rejected connects leave all state unchanged. Validated direct children update accepted work even while paused after a persistence error; incomplete initial work stays incomplete, and later accepted growth restores lag without clearing the initial-completion latch.
- Prepared turn identities capture generation, branch, processed/durable endpoints, protection and accepted target. Completion validates a nonempty suffix of at most 128 records, checks every parent/header edge, refuses work above the captured target, and applies progress atomically only after all records pass. Generation, branch, frontier and protection changes refuse stale work.
- Safe durability advances independently through `ValidatedIndexDurability`, which matches an endpoint to the already-verified recovered fence. Failure/paused/disabled paths do not advance durability or relax protection. Checked lifecycle disable handles generation exhaustion and remains idempotent.
- `TurnWork` accounts blocks, body bytes, undo bytes, cloned bytes, script items, script bytes, encoded envelope bytes, record operations, checkpoint operations and projection operations. All additions are checked. Admission includes zero-block preparation/publication reservations, yields on aggregate saturation, admits a legal oversized candidate only as an explicitly bounded singleton, and fails an absolute overrun instead of looping without progress.

## Local RED / GREEN / REFACTOR Evidence

| Task | RED evidence | Final GREEN evidence | REFACTOR / review |
| --- | --- | --- | --- |
| 1 | Registered owner tests failed compilation with the exported contracts absent. | `phase157_owner_`: 34 passed, 0 failed, 0 ignored; 274 filtered out. | Separated durability fixtures/tests, checked adjacent endpoint correspondence, preserved Disabled across pause, and bound completion to the immutable preparation target. |
| 2 | Registered budget tests failed compilation with `TurnWork`, budget and admission contracts absent. | `phase157_budget_`: 18 passed, 0 failed, 0 ignored; 289 filtered out. | Independent aggregate/singleton ceilings, checked array-based accounting, named budget module, simplified Clippy boolean guard. |

Both initial RED checks exited 101. They proved registration and missing-contract failure; no behavioral assertion failure is claimed before the contracts existed. Tests use Arrange/Act/Assert and cover exact/one-over/overflow boundaries for all ten counters.

## Verification

All Cargo runs used the pinned Bun 1.3.9 command-timings wrapper and the shared target serially.

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --lib phase157_owner_`: **34 passed**.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --lib phase157_budget_`: **18 passed**.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --lib filter_index`: **96 passed**, 0 failed, 0 ignored; 212 filtered out. This includes 52 new plan tests and 44 pre-existing filter-index tests.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --all-targets --all-features -- -D warnings`: **passed** after final code changes.
- Scoped `rustfmt --edition 2024 --check` and `git diff --check`: **passed**.
- Reviewed all owned source changes. Production modules contain no filesystem, network, storage, timer, thread or runtime effects and own no chain/undo vectors. Stub scan found no goal-blocking placeholders.
- Native whole-phase verification, measured coverage, Bazel smoke, lifecycle/source/security review, breadcrumb catalog registration and Git finalization remain root-owned. These scoped results do not claim a full-phase pass or measured zero-uncovered coverage.

## Adapter API Handoff

Exports live under `chainstate::filter_index::catch_up`.

- `AcceptedIndexTarget::new(height: u32, block_hash: BlockHash)` captures already-validated acceptance facts; it does not validate a block or grant storage authority.
- `BasicIndexProgress::new(generation: IndexGeneration, branch_identity: BlockHash, accepted_target: AcceptedIndexTarget, maybe_processed_endpoint: Option<FilterRecordIdentity>, maybe_safe_durable_endpoint: Option<FilterRecordIdentity>, protection: IndexInputProtection) -> Result<BasicIndexProgress, BasicIndexCatchUpError>` parses recovered progress facts. Adapters must prove complete recovered prefix ancestry before calling it.
- `observe_validated_connect(&mut self, target: AcceptedIndexTarget, parent: BlockHash) -> Result<(), BasicIndexCatchUpError>` belongs at the actual accepted-state boundary before fallible persistence. Rejected validation must not call it.
- `prepare_turn(self) -> Result<BasicIndexPreparedTurn, BasicIndexCatchUpError>` captures immutable facts; `complete_turn(&mut self, prepared: BasicIndexPreparedTurn, records: &[FilterRecordIdentity]) -> Result<(), BasicIndexCatchUpError>` follows achieved store-bound publication. Adapters must reserve work and prove byte/full-prefix/storage identity independently.
- `ValidatedIndexDurability::new(generation, branch_identity, checkpoint: FilterCheckpoint, fence: &VerifiedChainstateFence) -> Result<ValidatedIndexDurability, BasicIndexCatchUpError>` checks endpoint correspondence; `confirm_safe_durable(&mut self, validated, protection)` records achieved verified safe progress. The constant-size value is neither a private store capability nor a full-prefix proof.
- `pause(BasicIndexPause)`, identity-checked `resume(generation, branch_identity)`, and checked/idempotent `disable()` express operational state independently of current lag and initial completion.
- Getters expose `generation`, `branch_identity`, `accepted_target`, `maybe_processed_endpoint`, `maybe_safe_durable_endpoint`, `protection`, `state`, `initially_synchronized`, `current_lag: u64` and checked `maybe_next_height`.
- `BasicIndexTurnBudget::new(normal: TurnWork, absolute_singleton: TurnWork) -> Result<BasicIndexTurnBudget, TurnBudgetError>` accepts independent checked policy limits; `admit(used: TurnWork, candidate: TurnWork) -> Result<TurnAdmission, TurnBudgetError>` returns `Normal(total)`, `OversizedSingleton(total)` or `Yield`. A singleton ends its turn. `TurnWork::checked_add` and `fits` support adapter accounting.
- Envelope ceilings mirror the existing storage contract: `BASIC_INDEX_MAX_CANDIDATES = 128`, `BASIC_INDEX_MAX_ENCODED_BYTES = MAX_SIZE + 128 * 170`, and `BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES = MAX_SIZE + 170`, where existing codec `MAX_SIZE = 0x0200_0000`. No production timing/input defaults are invented. Later storage/guard integration must keep these ceilings synchronized with the existing codec/publisher.

## Files Created/Modified

- `filter_index.rs`: public catch-up module registration.
- `filter_index/catch_up.rs`: constant-size progress, prepared identities, verified durability correspondence and explicit error/state contracts.
- `filter_index/catch_up/budget.rs`: injectable checked accounting, admission and envelope ceilings.
- `filter_index/catch_up/tests.rs`: ordered progress, lifecycle, initial completion, exhaustion and snapshot boundary tests.
- `filter_index/catch_up/tests/budget.rs`: resource/admission boundary matrix.
- `filter_index/catch_up/tests/durability.rs`: verified fence, checkpoint/protection and durability failure tests.

All five new Rust files immediately carry the existing pinned index/base.cpp, index/blockfilterindex.cpp and node/blockstorage.cpp breadcrumb block. Plan 10 owns their consolidated manifest registration/check.

## Decisions and Simplification

The local AGENTS entrypoint/sidecar, placeholder-only overrides, architecture, code-shape, testing, verification, local-guidance and Rust standards informed pure modules, private state, `maybe_` names, explicit failures, scoped verification and small test groups. Both active lesson sources were read completely (7,188 bytes, 2,397 conservative estimated tokens); the existing audit baseline has no new trigger.

One explicit simplification pass kept constant-size copyable state and atomic trial reductions, reused the existing lifecycle disable semantics and predecessor checker, and represented ten checked resources through one counter helper rather than ten independent arithmetic implementations. Budget and durability-test modules keep every new file below the 628-line gate; final line counts are 410, 187, 571, 389 and 332 respectively.

## Deviations from Plan

- **Rule 2 / code-shape adaptation:** The coordinating agent approved three additional child paths: `catch_up/budget.rs`, `catch_up/tests/budget.rs` and `catch_up/tests/durability.rs`. They preserve the planned parent API and responsibilities while avoiding oversized files; there is no new crate, dependency, effect boundary or product scope.
- **Rule 1 / implementation review:** Pausing a Disabled owner initially could replace its state with Paused and allow resume. The pause guard now preserves Disabled, with a regression test. Prepared completion also refuses above its captured target, and constructors check available adjacent endpoint parent/header correspondence.
- One syntax diagnostic during initial budget GREEN and one Clippy boolean simplification diagnostic were corrected; final scoped tests/lint pass. No unresolved out-of-scope issue was changed.
- The strict phase wrapper explicitly defers every task/TDD/metadata commit, staging operation, push and shared planning-state mutation to root. No Git commit or STATE/ROADMAP/REQUIREMENTS/config update was made by this executor.

## Task Commits

Task 1, Task 2 and summary finalization are **pending root Git finalization** after clean whole-phase verification. No commit hashes are claimed.

## Failure Boundaries and Known Limits

The pure values do not mint storage authority, establish undo authenticity, reconstruct missing history, perform I/O, repair state or replace branches. Complete ancestry/body/undo validation, recovery-minted private append proof, achieved atomic persistence and accounting of every adapter validation/publication pass remain Plans 04–07 responsibilities. Constructor-local consistency checks do not substitute for a complete historical proof across gaps.

Operational error state preserves accepted progress, safe checkpoint and protection. Initial completion is about reaching the same validated accepted identity; it is distinct from current lag and safe durability. Numeric production budgets, authority hold time and elapsed performance await Plan 07 measurement. Runtime branch replacement and public filter/RPC/peer/operator surfaces remain later phases.

No new endpoint, authentication path, filesystem access or schema trust boundary is introduced; the new pure validation/admission surface is covered by T-157-08, T-157-09 and T-157-10. No additional threat flag or known stub was found. There was no authentication gate or manual user setup.

## Next Plan Readiness

Plans 04–07 can consume the exported contracts. CFIX-01 remains Pending until the ordinary daemon consumers, real persistence boundaries, scheduled progress and complete phase gates are proven. Shared state/requirements activation and final commits stay with root.

## Self-Check: PASSED

All five created Rust files and this summary exist, the parent module registration is present, and the final owned diff passes whitespace checks. Final command outputs establish 34/18 targeted tests, 96 scoped regressions and warning-free scoped Clippy. New files are intentional pending root finalization; no generated runtime file is introduced. Commit-existence checking is deferred because the strict wrapper forbids child commits; no hash or full-phase completion is claimed.
