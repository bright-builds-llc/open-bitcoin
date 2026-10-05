---
phase: 157-safe-activation-and-scheduled-index-catch-up
plan: "06"
subsystem: chainstate
tags: [rust, basic-filters, accepted-facts, persistence-failures, bounded-owner]
requires:
  - phase: 157-03
    provides: Pure ordered progress and checked TurnWork counters
  - phase: 157-04
    provides: Sealed recovered-manager lineage and genuine own-flush receipts
  - phase: 157-05
    provides: Bounded achieved append publication and independent safe checkpoints
provides:
  - Shared direct/staged accepted notification before persistence
  - One next-height complete body-bound historical facts object
  - Truthful accepted progress and dependent mempool effects after writer errors
affects: [157-07, 157-08, 157-09, 157-10]
tech-stack:
  added: []
  patterns: [accepted-before-persist, bounded-live-facts, private-authority-bridge]
key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/filter_index.rs
    - packages/open-bitcoin-node/src/chainstate/tests/filter_index.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/accepted.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
    - packages/open-bitcoin-node/src/chainstate/tests.rs
    - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
    - packages/open-bitcoin-node/src/sync/block_response.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
key-decisions:
  - "Pure accepted progress grants no storage authority; public constructors and Clone carry no owner."
  - "Retain only the next processed successor's complete facts; later acceptance enlarges the backlog."
  - "Apply dependent mempool projections before returning an already-accepted persistence error."
  - "Keep existing sync peer failure reporting and preserve the original storage error."
requirements-completed: []
requirements-addressed: [CFIX-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T19:46:52Z
duration: 21min
completed: 2026-10-05
commits: []
git_finalization: pending root whole-phase verification and strict finalization
---

# Phase 157 Plan 06: Accepted Historical Facts Summary

**Ordinary direct, local and requested validated blocks advance one ordered accepted owner before persistence, retaining complete spent facts and truthful failure state.**

## Performance

- First recorded RED: 2026-10-05T19:26:09.251Z.
- Completed summary self-check: 2026-10-05T19:46:52Z.
- Execution/documentation window: approximately 21 minutes, excluding initial context loading.
- Tasks: 3/3 after root's checked adapter-fault amendment.
- Source paths: 13, including three new files; summary: one.
- Every Cargo command used pinned Bun 1.3.9 and the timing wrapper. No preserved ignored target was enumerated or changed. No Git staging, commit, push or shared STATE/ROADMAP/REQUIREMENTS/config change occurred.

## Accomplishments

- Direct validation now uses the same existing prepare/absorb path as network acceptance. Preparation captures complete BASIC inputs from `StagedChainstateConnect` before its undo is consumed. After infallible absorb, the existing sealed lineage and pure accepted owner observe the position before any fallible persistence.
- `AcceptedBasicFilterFacts` has private Block/BlockUndo/ChainPosition fields and is not Clone. The complete undo comes from actual staging, including historical and same-block spends. A borrowed `BasicFilterInputs` regenerates the exact previously proved filter. Current coins never reconstruct spent scripts.
- A caught-up owner retains at most one complete next-height facts object. Behind-owner acceptance enlarges the target without moving the processed endpoint, safe checkpoint, protection or next required height; it adds no fact queue or chain/undo history copy. Existing accepted payload/cache and undo ownership remains available for ordered replay.
- Persistence failures, including later explicit flushes, pause the same owner without erasing accepted progress. Requested post-connect body errors notify through a crate-private serialized authority method. If that notification also fails, the error preserves both the original storage failure and notification failure.
- The network aggregate still applies its accepted mempool core, peer position and block cache changes. Dependent projections now apply before the persistence error is returned, so removal evidence, caches and membership stay aligned with the accepted core.
- Plan 04's private lineage, untracked public/Clone constructors, exact genuine own-flush confirmation and replacement invalidation remain intact. No raw target/fence constructor or successful Connected-only callback refreshes store authority. No per-connect Always flush was introduced.
- The root-approved test-only adapter seams use the existing shared publication control for one-shot BeforeBody, BeforeUndo and BeforeCoins faults. Existing BeforeChainMeta is reused. Actual pre-effect writer calls preserve publication-before-payload lock order; no global flag, filesystem permission mutation or new production surface was added.

## Local RED / GREEN / REFACTOR Evidence

| Task/boundary | Observed RED | Final evidence |
| --- | --- | --- |
| 1: owner contract | Registered three tests failed compilation because the owner APIs were absent; an incorrect fixture `IndexGeneration::initial` was separately corrected. | Direct/rejected/Clone controls pass; complete direct/staged spent facts regenerate the exact historical record. |
| 1/2: accepted facts and late flush | Actual registered run passed 4/6 and failed caught-up facts retention plus late metadata-failure pause. | Processed-successor eligibility and common flush-failure notification fix both; the same six controls passed. |
| 2: dependent mempool boundary | Restoring the original error-before-dependent ordering produced 0/1 pass: core removal succeeded, but cleared evidence was 0 instead of 1. | Applying dependent projections before returning Err passes; final aggregate retains the precise cleared-count assertion. |
| 3: real writer faults | Initial 9/11 and later requested controls exposed fixture assumptions: a due clock does not force IfNeeded, an already-stored future body avoids requesting it, and sync_once returns a bounded peer failure report. These are fixture corrections, not claimed product-bug REDs. | Real direct/local/requested BeforeUndo/BeforeCoins/BeforeChainMeta and requested BeforeBody controls pass; cursor/protection checks include actual reopen. |
| Parallel fixture isolation | One intermediate aggregate passed 13/14; the two requested tests reused a timestamped fixture name and collided on a datadir lock. | Fault-specific fixture names produced the final default-parallel 14/14 pass. |
| Scoped style | Clippy identified a complex test evidence return type. | A named test-only evidence struct passes the style gate without suppressing warnings. |

The Task 3 seams and controls were added together; no separate missing-seam or behavioral pre-implementation RED is claimed for that task. All compile failures exited 101. Exact behavioral REDs are distinguished above from fixture/API mistakes.

The simplification pass reused existing staging, typed BASIC generation, the pure progress reducer and publication mutex. It retained one complete next-height object instead of a new queue, reused the existing authority bridge, and kept the dependent-projection fix to one ordering change. No production dependency, crate, schema or repair operation was added.

## Verification

All Cargo arguments use `--manifest-path packages/Cargo.toml -p open-bitcoin-node` through pinned Bun and the timing wrapper. Counts overlap and are not a distinct-test sum.

| Check | Result |
| --- | --- |
| `test --lib phase157_accepted_`, final aggregate | **14 passed**, 0 failed, 0 ignored; 1,208 filtered; **8.04s**. |
| `test --lib filter_index`, after final test evidence refactor | **130 passed**, 0 failed; 1,092 filtered; **80.13s**. Includes every new accepted control plus inherited activation/fencing/prune controls. |
| `test --lib phase157_proof_` | **36 passed**, 0 failed; 1,186 filtered; **23.59s**. Genuine direct/staged lineage and raw/interleaved forgery controls remain intact. |
| `test --lib chainstate::tests` | **14 passed**, 0 failed; **0.01s**. Includes the 11 inherited manager controls and no forced Always flush. |
| `test --lib persist_progress_does_not_credit_when_coins_b_lags_memory_tip` | **1 passed**, 0 failed; **0.81s**. Accepted-unflushed coins remain uncredited as durable progress. |
| `test --lib connected_block_removal` | **7 passed**, 0 failed; **0.19s**. Confirmed/conflicting removals and runtime caches remain aligned. |
| `clippy --all-targets --all-features -- -D clippy::all` | **Passed**, **6.07s**, with 12 ordinary normal-lib dead-code diagnostic groups and one inherited-by-test variant warning visible. |
| `cargo fmt --all -- --check` | **Passed** after scoped rustfmt on the 13 owned paths. |
| Owned diff/whitespace and source length review | **Passed**; every owned source is at most 628 lines. |

Full native verification, coverage, Bazel, source/security/lifecycle review and `-D warnings` remain root-owned gates. The unused owner/append driver graph requires actual Plan 07 production consumption; no fake call, public visibility widening or allow was added. The historical constructor source assertions remain Plan 10-owned.

## Bounds and Scheduler Handoff

- `install_basic_index_owner(progress)` matches the current accepted tip and refuses a second owner. It installs pure progress only, grants no store authority, and is not yet called by production startup; Plan 07 must initialize it from the actual configured recovered runtime's authenticated proof and saved endpoints.
- `maybe_basic_index_progress`, `maybe_basic_index_failure` and `maybe_accepted_basic_facts` are narrow immutable reads. `complete_basic_index_turn(prepared, identities)` applies the contiguous pure reducer only after achieved adapter publication and drops consumed live facts. Plan 07 owns actual production preparation/completion, safe confirmation and resume bridges.
- Complete facts are selected using the processed successor even when current lag is zero. This matters because preparation precedes the new accepted target observation. Later accepts keep the existing next-height facts and enlarge the target; they cannot append out-of-order headers.
- Checked logical clone reservation counts Block, Transaction, input/output and witness container elements plus their byte storage, BlockUndo/TxUndo/Coin elements and restored scripts, and ChainPosition. Every increment is checked before cloning. The combined retained clone ceiling is **64 MiB**; examined output/spent scripts are capped at **1,000,000 items**. Tests cover exact ceiling, one over and checked overflow.
- `facts.work()` returns body/undo logical object reservations, cloned bytes and all examined output/spent script item/byte counts. It leaves blocks and encoded_bytes at zero. Plan 07 must merge these costs with actual body/undo acquisition and generation costs; append outcomes already count selected blocks and encoded output once, so those two counters must not be duplicated.
- These are explicit adapter retention/refusal bounds, not measured production turn defaults, allocator RSS or a proof that every wire-legal witness-heavy block fits 64 MiB of decoded Vec storage. Root was notified of the witness expansion consideration; Plan 07 must derive legal singleton limits from actual codec/consensus boundaries and measurements. Over-limit capture still accepts the validated block, pauses the owner, applies baseline effects and persistence, then returns a bounded index capture error. It cannot publish safe progress.
- Behind live work uses existing body/cache and accepted undo ownership or retained storage; it has no second success-only queue. Missing required body/undo must remain explicit failure, never current-coins reconstruction or an out-of-order append. Body and undo writer errors remain independent from index publication success.
- Ordinary requested error evidence follows the existing sync contract: sync_once returns a summary with a failed peer and error evidence, not necessarily top-level Err. Body-save failure grants zero block credit; the accepted chainstate and owner target remain visible.

## Decisions and Deviations

Repo-local AGENTS guidance, sidecar, placeholder-only overrides, architecture, code-shape, testing, verification, local-guidance and Rust standards informed this work. Both active lessons were loaded completely within their budget: 7,188 bytes and 2,397 conservative estimated tokens. No archive was loaded; the inherited audit baseline required no new audit.

- Root approved one owner-None initialization in the existing specialized Fjall constructor and the narrow existing runtime-authority child bridge.
- Root added Task 3's four-path real adapter fault scope after inspection found no suitable body/undo/ordinary coins failure seams.
- Rule 1: actual late metadata failure and dependent mempool projection controls identified and fixed missing accepted-state failure/effect propagation.
- Fixture corrections, parallel naming isolation and the Clippy evidence struct are recorded above without claiming they are product fixes.
- Strict wrapper instructions defer all Git task/TDD/metadata commits and shared state/requirement updates to root.

## Threat Mitigations and Known Limits

T-157-17 is covered by one actual accepted observation before fallible effects and direct/local/requested failure controls. T-157-18 is covered by opaque complete staged undo and the exact historical/same-block script comparison and generated record. T-157-19 is covered by monotone accepted targets, unchanged next height while behind, one complete facts slot, checked pre-clone bounds and preserved no-forced-flush behavior.

No new network endpoint, authentication path, filesystem trust boundary or schema was introduced. Test fault injection models software writer boundaries and real reopen; it is not hardware power-loss simulation. No goal-blocking placeholder was found in this plan's handoff. Production scheduler/startup initialization and measured singleton/default budgets intentionally remain Plan 07 and must be completed before CFIX-01 is marked done. This summary makes no whole-phase, public-mainnet, production-readiness or funds claim.

## Task Commits and Next Readiness

Tasks 1–3 and this summary are pending root Git finalization after clean whole-phase verification. No commit hashes are claimed. Requirements remain uncompleted here. Plan 07 can consume complete facts and ordered owner progress; Plan 10 owns the three new breadcrumb catalog entries and final checker.

## Self-Check: PASSED

All 13 owned source files and this summary exist, the two new test children are registered, and every new Rust source immediately carries pinned breadcrumbs. Final nonzero accepted, filter, proof, manager, accepted-unflushed and mempool evidence appears above. Scoped formatting and diff whitespace pass; owned sources stay at most 628 lines. Stub/threat-surface review found no unplanned trust surface or handoff-blocking placeholder. Commit-existence checks do not apply because root explicitly defers every commit. The summary uses only the opening and closing frontmatter delimiters. Production owner initialization, scheduler consumption and full-phase gates remain explicitly pending.
