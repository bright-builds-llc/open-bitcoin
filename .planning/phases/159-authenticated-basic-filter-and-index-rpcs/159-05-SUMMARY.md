---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "05"
subsystem: node-readiness
tags: [basic-filters, captured-frontier, std-future, cancellation, lifecycle]
requires:
  - phase: 159-04
    provides: Same-authority bounded queries and opaque branch-local captured frontier
provides:
  - Bounded cancellation-safe std Future over the existing captured accepted frontier
  - Achieved ordinary-owner progress and lifecycle/failure notifications outside locks
  - Checked final reads bound to original request identity and provenance
affects: [159-06, 159-07, 159-08]
tech-stack:
  added: []
  patterns: [finite captured position, checked waiter identities, deferred waker callbacks]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/owner.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/failures.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/interleavings.rs
  modified:
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
key-decisions:
  - Preserve accepted height/hash as an immutable branch-local position; allocate independent checked waiter IDs.
  - Bind completion to the original requested hash, known identity and request-time provenance.
  - Notify from the existing shared authority mutation seam after actual scheduled work; add no second worker or request indexing.
  - Clone, replace, remove, cancel and wake wakers only with user callbacks outside authority/publication/registry guards.
patterns-established:
  - A completed notification authorizes a checked final read, never data or a newer captured frontier.
  - Registry polling and cancellation contend only with bounded memory operations; storage inspection occurs outside the registry mutex.
requirements-addressed: [CFRP-01, CFRP-02]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T18:23:08Z"
duration: approximately 29min including host-loader diagnostics and WR-03 closure
completed: 2026-10-09
---

# Phase 159 Plan 05: Captured-Frontier Readiness Summary

**BASIC reads can await a finite accepted position on the ordinary owner, cancel safely, and finish with the original request's identity and provenance under revalidated authority.**

## Accomplishments

- Both implementation tasks are complete with scoped verification; their commits remain pending root's consolidated gate. Five new Rust files and nine narrow existing-source edits implement readiness, tests and the WR-03 poison-dispatch closure. The precise initial executor timestamp was not separately recorded; duration is approximate.
- `BasicFilterQuery::Pending` now owns a non-Clone `BasicFilterReadBarrier`, wrapping the exact Plan04 `BasicFilterReadFrontier`. No public constructor, fabricated sequence, Tokio dependency, detached owner, thread worker, request-side generation or forced coins flush was added.
- Initial incomplete synchronization still returns promptly: existing rows are served and missing rows retain initial indexing classification. After the latch, known requests capture the accepted height/hash once before immutable lookup. New acceptance cannot extend that barrier.
- Success requires the ordinary owner's processed prefix to cover the captured height and the current canonical position at that height to match the captured hash, with the same authority incarnation, lifecycle generation and branch. The initial latch alone never grants readiness.
- Every registered barrier owns one of 64 slots. IDs increase independently of branch position using checked `u64` addition. Capacity and counter exhaustion are explicit typed failures without registration side effects. Poll completion and Drop cancellation reclaim slots.
- Pending notification collection follows the existing authority mutation and typed lifecycle dispatcher. Store inspection occurs outside the registry mutex. Wakers are moved out under guards, then invoked only after the authority and publication guards have released. Incoming Waker cloning and replaced/canceled Waker destruction also occur outside locks.
- Disable, changed branch/generation, owner failure, unresolved accepted history publication, invalid required history, publication poison, explicit scheduled-owner stop, final-handle drop and observed authority poison settle waits conservatively. Success never substitutes for a final immutable read.
- Final completion checks the original hash, known height/parent, incarnation, generation, branch, exact captured position and shared storage integrity. Missing rows use captured provenance. A genuine header-only block that connects while waiting still produces NeverConnected absence for that original request; a successful stored row still wins.
- Summary continues to report the initial synchronization latch and processed height independently of the safe durable checkpoint. Accepted-unflushed facts satisfy readiness at height 2 while the safe checkpoint remains at height 1.

Material guidance: AGENTS.md, AGENTS.bright-builds.md, placeholder-only overrides and architecture/code-shape/testing/verification/Rust standards. Both active lesson files were read completely: 7,188 bytes, 2,397 conservative estimated tokens; no lesson audit was needed. The approved Plan05/CONTEXT/RESEARCH/UI contract and Plan03/04 summaries/actual query measurements informed implementation. The existing pinned `BaseIndex::BlockUntilSyncedToCurrentChain` remains the readiness baseline.

## Verification Evidence

All Cargo work used pinned Bun through the timing wrapper and serialized target access. No full native verifier, staging, commits, pushes, hook bypass or shared planning-state mutation was performed here.

| Check | Result |
| --- | --- |
| Initial phase159-readiness-red | Expected compile RED for absent Future, typed failure and checked completion APIs. |
| phase159-readiness-earned-red semantic mutation | 1 discovered test failed its pre-owner Pending assertion when processed-prefix coverage was replaced with unconditional success. Restored immediately. |
| phase159-readiness-waker-red semantic mutation | 1 discovered test failed its lock-availability assertion when wake ran before authority release. Try-lock controls prevented an indefinite test hang. Restored immediately. |
| phase159-readiness-owner | 19 discovered readiness tests passed before final combined verification. |
| Initial closeout phase159-query-readiness-final | 33 passed, 0 failed, 0 ignored, 1,385 filtered; 45.32s test execution. Includes all 14 Plan04 query tests and the original 19 readiness tests; WR-03 final evidence below supersedes the readiness count. |
| Final phase159-readiness-clippy, node --lib --tests -- -D warnings | Passed, no warning suppression. |
| Scoped Rust 1.94.1 rustfmt and diff checks | Passed for all 13 owned source paths. |
| Bright Builds managed all check | Zero findings; 1,351 tracked source files scanned. Five new Rust files also measured individually below 628 lines. |

The tests exercise real managed acceptance, the actual ordinary scheduled-turn function, genuine unflushed historical facts, single-block budgets, fixed target versus newer accepted work, completion before first poll, latest-waker replacement, cancellation, exact registry capacity, checked overflow, request-hash mismatch, distinct authority incarnations, shared integrity invalidation, real reorgs, explicit stop and final-handle drop. A fresh genuine genesis/coins fence and genuine header admission prove the NeverConnected request snapshot, rather than a seeded validity bit. The invalid required-body case proves the existing MissingHistory pause and terminal readiness failure. Publication controls cover BeforeRecords, BeforeCheckpoint, BeforeProtection and AfterCommit; accepted-history controls cover BeforeCommit and AfterCommit.

Future tests use deterministic direct polls around bounded scheduled work. They do not wait on a Tokio executor, sleep for public semantics, or make a timeout grant readiness. Reentrant Wake/Drop checks use nonblocking try-lock assertions for authority, registry and publication guards, then perform an actual summary read when available.

`phase159_basic_readiness_owner_poisoned_authority_wakes_typed_failure` registers and polls a waiter, poisons the real authority, then invokes `maybe_basic_index_summary()` and asserts the counting waker has fired before polling the Future again. This proves a subsequent handle-error path actively wakes an awaiting caller. `stop_basic_filter_readiness()` uses that same poison handler: it first drops the poisoned mutex guard, collects/wakes pending registrations with AuthorityUnavailable, then returns the typed authority error. It does not abandon waits when authority locking fails.

## Host-Loader Diagnostic

One linked test process stalled before the test harness. Sampling showed only `_dyld_start`, 112 KiB footprint, zero CPU and no Rust frames. After repeated unchanged evidence, the parent authorized termination of only that owned child and one retry of the same already-linked binary through the timing runner. The retry launched normally and reached the tests. The interrupted run is diagnostically aborted, neither a semantic test failure nor a pass. No security metadata, global services or unrelated processes were changed.

The retry found a fixture omission: the second real reorg lacked the first accepted replacement's retained undo. The fixture now persists the actual accepted undo independently of BASIC processing or a coins checkpoint. The fresh header-only fixture also stopped resubmitting genesis into an already initialized header store. These changes corrected test setup without relaxing validation. Strict Clippy found one production expect despite a logically present registry entry; it was replaced with an explicit typed failure guard before the final combined GREEN run.

## API Handoff to Plan06

- `basic_filter_query(hash) -> Result<BasicFilterQuery, BasicFilterQueryError>` retains its existing entry point and states. Pending's payload is now `BasicFilterReadBarrier`; the query enum is no longer Clone because registrations cannot be duplicated.
- `BasicFilterReadBarrier: Future<Output = Result<BasicFilterReadCompletion, BasicFilterQueryError>>`. It owns its registration and cancellation. `frontier()`, `accepted_height()` and `accepted_hash()` inspect the immutable capture; none constructs authority.
- Await Pending after releasing the RPC context mutex. On success, call the same handle's `complete_basic_filter_read(original_hash, completion) -> Result<BasicFilterQuery, BasicFilterQueryError>`. This final operation never recaptures newer acceptance and cannot return Pending. The token is private and consumed by the final read.
- `BasicFilterQueryError::Readiness(BasicFilterReadFailure)` is additive alongside Authority and Storage. Failure variants are Capacity, CounterExhausted, Invalidated, OwnerFailed, OwnerStopped, AuthorityUnavailable and AlreadyCompleted. Plan06 must project fixed redacted RPC errors; do not display internal backend/readiness strings.
- `stop_basic_filter_readiness() -> Result<(), ManagedNetworkAuthorityError>` must be called when the existing scheduled maintenance owner exits or fails, including daemon shutdown. Stop is terminal for that authority; reopening installs a fresh owner. Plan06 owns this daemon lifecycle call, not a new worker. Last network-handle drop is also a tested terminal fallback.
- `maybe_basic_index_summary()` and BasicIndexSummary retain their existing shape and semantics.

## Limits and Scope

The registry admits at most 64 outstanding requests, including terminal but not yet polled registrations. Every completed poll or cancellation reclaims its slot. Counter exhaustion is explicit; accepted height remains a branch-local position rather than a monotonic work sequence. Notification inspection is bounded by the outstanding 64 frontiers and uses point lifecycle/progress/canonical-position checks, not ancestry walks or filter generation. The existing Plan04 record/header bounds remain unchanged. No latency, RSS or allocator-capacity guarantee is inferred from these structural limits.

The pure notification Future uses only std synchronization and wakers. Request queries still use the existing serialized authority for bounded reads; no Condvar, blocking receive, request worker or new runtime dependency was introduced. HTTP authentication, context-lock release/await integration, real daemon lifecycle shutdown, paired-prune RPC proof, full native verification and independent source/security/lifecycle acceptance remain Plans06–08/root gates.

## Deviations from Plan

1. **Rule2: Preserve request-time facts across the wait.** Parent review required binding the final token to the original requested hash/height/parent and provenance. Added private ReadRequest capture and wrong-hash plus connect-during-wait controls. This prevents a newly connected requested block from turning original absence into fabricated corruption.
2. **Rule2: Use the shared mutation notification seam.** Existing scheduled turns, failures, disable and reorg already use the shared mutate authority. Notification collection therefore lives there and in the existing typed lifecycle dispatcher instead of separately editing catch_up.rs/reorg.rs. This retains one owner and covers both successful and failed commands. Existing read/mutate methods moved into the new owner child to keep the root below the managed size limit; root is 609 lines and network.rs is 621.
3. **Rule3: Narrow module/reader/test glue.** Added thin network.rs/lib.rs exports, reader generation/integrity inspection in storage/fjall_store/filters/query.rs and an include under the existing continuous-history test owner in sync/tests/filter_index/catch_up.rs. Split owner integration and failure/interleaving tests into child modules. Parent approved this glue; no history cache, new schema, endpoint, authentication scheme or architectural expansion was added.

The simplification pass removed an unnecessary optional readiness-owner parameter, reused the existing query/storage/history fixtures, separated storage inspection from registry memory operations, and moved only the existing read/mutate seams into the owned child module. Accepted preparation, mempool transaction guards, live pending provenance and borrowed historical reads were preserved.

## Task Commits and Root Gates

1. Task1: Captured-frontier Future, admission/cancellation and query readiness — pending root consolidated commit.
2. Task2: Achieved progress/lifecycle/failure notifications and regression evidence — pending root consolidated commit.

No authentication gate occurred. No requirements are marked complete. STATE, ROADMAP, REQUIREMENTS, todo, lesson and config updates remain root-owned. This executor did not perform Plan06 work.

All five created Rust paths in key-files.created have exact source breadcrumb comments naming index/base.cpp and index/blockfilterindex.cpp. Plan08 owns their manifest registration. No unintentional stubs or undeclared threat surface was found. T-159-16 is covered by bounded admission/counter/cancel controls, T-159-17 by captured request/frontier identity and checked final reads, and T-159-18 by owner/failure/lifecycle settlement plus reentrant callbacks. Independent full-phase security review remains root-owned.

## User Setup Required

None.

## Review Fix WR-03: Direct Dispatcher Poison Settlement

Implemented and behaviorally verified following the confirmed early review in 159-REVIEW.md. Independent reviewer recheck and whole-phase approval remain root-owned; this executor did not change the review report.

- The actual minimal RED registered and polled a counting-waker barrier, poisoned the shared authority, then invoked public prepare_peer_relay_effect(1). The caller received an error while wake count remained zero. phase159-readiness-direct-poison-red discovered one test and failed its count assertion before any second Future poll.
- Added one private lock_authority helper that explicitly drops the PoisonError-contained guard before calling basic_filter_authority_unavailable. Existing read/mutate/stop helpers and all three direct lifecycle dispatchers use it. The checkpoint-evidence facade now uses shared read. Thus every production lock of the shared network authority settles registered reads on poison; unrelated automatic-prune mutexes were not changed.
- Existing lifecycle error projection and owned checkpoint receipt/abort error carriers remain intact. Completion evidence comes from actual execute_prepared_mempool_snapshot_write Sync persistence. Abort evidence flows through the actual encoder-failure adapter and its existing retained-abort dispatch-failure mechanism; no fabricated capability or completion/abort token was introduced. Tests compare the returned achieved receipt and retained abort with the original carriers.
- Two direct-dispatch regressions now prove counting wakes before repoll. The second covers relay preparation, checkpoint evidence, checkpoint completion, checkpoint abort and poisoned stop individually. Its waker performs nonblocking authority/publication/registry checks, handles an unlocked poisoned mutex distinctly from WouldBlock, and reenters the registry after guards release. No second summary read or last-handle drop earns the wake.
- Scope is lifecycle.rs, effects.rs, readiness/owner.rs and the existing failure/interleaving test files, plus this summary. No new Rust path, dependency, public API, RPC edit, tracker mutation or Git operation was added. Centralizing the lock helper also removes repeated poison-handler code.

| WR-03 check | Result |
| --- | --- |
| phase159-readiness-direct-poison-green | 2 passed; second test covers all five dispatcher families and exact effect-carrier retention. |
| phase159-readiness-wr03-final | 35 passed, 0 failed, 0 ignored, 1,385 filtered; 43.32s. Includes all 14 query tests and all 21 readiness tests. |
| phase159-readiness-wr03-effects | 47 managed lifecycle/effect regressions passed, including foreign/stale/duplicate carriers and dispatch-failure ownership. |
| phase159-readiness-wr03-checkpoint | 8 checkpoint worker/retry/shutdown regressions passed. |
| phase159-readiness-wr03-clippy | Node --lib --tests -- -D warnings passed; 5.22s. |
| Scoped rustfmt, diff and Bright Builds | Passed; zero managed findings. All affected files remain below 628 lines. |

One initial test import path was corrected before GREEN; it is not semantic RED evidence. The newly linked direct-dispatch test process also paused at the previously observed pre-Rust loader boundary. A sample again showed only _dyld_start and 112 KiB footprint. It resumed on its own and both tests passed before the authorized targeted recovery was attempted; no termination, retry, security metadata or global service change occurred for this second diagnostic. Cargo target access remained serialized and the slot was explicitly released after the checks.

## Self-Check: PASSED

- All five created Rust files, all nine modified source paths and this summary exist.
- Final evidence is 35 passing combined tests, 47 managed-effect regressions, eight checkpoint regressions, strict scoped Clippy, scoped formatting, zero Bright Builds findings and clean diff checks. Both deliberate semantic mutations were restored before final GREEN; the WR-03 direct dispatcher semantic RED is fixed.
- No commit hash, completed requirement, full native pass or authenticated daemon proof is claimed; those remain root's consolidated gates.
