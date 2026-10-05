---
phase: 157-safe-activation-and-scheduled-index-catch-up
reviewed: 2026-10-05T21:31:28Z
depth: standard
files_reviewed: 7
files_reviewed_list:
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T21:31:28Z
review_partition: daemon-bin
scope_source_sha256: 05701ca851cd9866768624c5d40fe5c24723158434dca6d4848d44a18da61bfb
base_commit: 26bc454a66d6d7a241fc01be84dc6fbf0416d15a
source_state: uncommitted frozen implementation including explicitly scoped untracked source
whole_phase_review_complete: false
verification_owner: root
---

# Phase 157: Daemon Partition Code Review Report

**Reviewed:** 2026-10-05T21:31:28Z\
**Depth:** standard\
**Files Reviewed:** 7\
**Status:** issues_found within this explicit partition

## Summary

The actual daemon carries resolved BASIC activation into configured durable open and advances catch-up through the existing maintenance worker. One warning concerns outer daemon shutdown after a retained idle index error: the coins worker settles, but the daemon returns before explicitly settling and joining its other workers. No evidenced security vulnerability was found in this partition.

The seven files listed above were read completely, including registered tests and fixtures. Applicable constructor and shutdown callees were cross-referenced to establish actual behavior. Phase CONTEXT and Plan 08/08-SUMMARY supplied intent and reported check provenance; they were not substitutes for source evidence. This report does not establish whole-phase completion. The 58-file runtime report remains unchanged; Plan 10 scripts, guards, documentation and catalog work are excluded and remain root-owned.

## Warnings

### WR-01: Retained index error bypasses final mempool settlement and worker joins

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs:156-157`\
**Affected caller:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:186-192`\
**Severity:** Warning

**Issue:** An idle BASIC turn error is now retained until shutdown. The loop correctly runs Always first, then returns that earlier error. `CoinsFlushWorker::shutdown_always` joins the coins worker and maps the error to `ShutdownCheckpoint`; `serve_authoritative_runtime` immediately propagates it with `?`. The subsequent `shutdown_and_mark_clean` for the mempool checkpoint worker and `retry_worker.shutdown` are skipped. Neither worker has a Drop implementation that joins its thread: dropping their senders only disconnects the channels and dropping their JoinHandles detaches the threads. The binary can therefore return from main and terminate before the final mempool checkpoint finishes, losing uncheckpointed mempool state on an otherwise requested orderly shutdown. The retry producer is also left unjoined. This reachable path is newly exposed by returning a retained nonfatal idle index error even when Always succeeds.

`phase157_idle_explicit_disable_refuses_stale_turn_and_keeps_always_cleanup` directly establishes the retained error and successful Always combination, but stops at `coins_flush_worker_loop`; it does not exercise the outer daemon cleanup. The clean-marker gate should remain conservative on errors, so merely swallowing the index error or marking the store clean is not a correct fix.

**Fix:** Record shutdown errors and report them only after all workers have been stopped and joined. Separate final mempool settlement/join from clean-marker publication, so an earlier coins/index error still permits required mempool cleanup without marking a failed shutdown clean. Keep the index error visible and preserve Active protection. Add an injected outer-shutdown control with a retained BASIC error and successful Always that proves final mempool settlement and retry join still occur, the error is returned, and the clean marker is withheld. A small shutdown coordinator accepting settle/join callbacks can make this deterministic without sockets or timing sleeps.

## Reviewed Boundaries

| Boundary                         | Source-backed assessment                                                                                                                                                                                                                                                                                                                                                                                                  |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Actual durable selection         | `open-bitcoind.rs:373-403` requires an existing directory for BASIC. Valid-directory explicit zero also selects durable open. Omission without another durable trigger remains transient, preserving saved ownership until a real durable startup. Existing sync/inbound/prune triggers remain present.                                                                                                                   |
| Mode before prune initialization | `open-bitcoind.rs:306-319` maps BASIC to Enabled and actual durable omission/zero to Disabled. The configured constructor calls `initialize_configured` before owner construction, then performs the first turn. The daemon adds no duplicate first turn.                                                                                                                                                                 |
| Startup refusal and preservation | Actual loader/startup tests cover bare/1/basic, missing/nonexistent datadir, empty validated history, separate missing body/undo at height 12, preserved sources/owner state, complete saved checkpoint zero/reopen, and omitted transient preservation. The post-startup ordinary outcome at height 15 supplies indirect evidence for one eight-row startup prefix; Plan 07 owns direct private first-turn observations. |
| P2P activation                   | Runtime relay/block-serving/inbound settings pass through unchanged; sync worker selection returns None when sync is disabled. Actual tests inspect disabled inbound listener, absent endpoints/workers, zero peers and absent compact-filter service bit. The existing authenticated local RPC listener remains part of the baseline daemon.                                                                             |
| Typed ordinary maintenance       | `coins_flush.rs:51-68` uses concrete durable and memory adapters without downcasts. `start_coins_flush_worker` is selected by durable storage, including offline index-only startup. Each elapsed event runs ordinary Periodic maintenance and one bounded index turn, then returns to the existing one-second wait.                                                                                                      |
| Ordered progress and durability  | Registered injected controls extend the accepted target through genuine local connect and observe prefixes 15/23/31/39/40, bounded generation/read/publication counts, truthful initial synchronization, genuine Periodic B40 and zero-generation release, future-height protection and closed configured Fjall reopen. These assertions exercise production elapsed bodies rather than a separate test scheduler.        |
| Shutdown and ownership           | A stop event runs no further index turn; Always runs while saved Active protection remains. Genuine accepted-unflushed Always/reopen and explicit-disable stale-work controls are present. Actual coins-worker signal/join is tested. The outer cleanup error gap is WR-01 above.                                                                                                                                         |

## Material Guidance and Scope

AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md and standards/index.md informed review, together with architecture, code-shape, testing, verification, local-guidance and Rust standards. The GSD code-review skill and reviewer contract were applied. Project skill directories are absent.

Both canonical active lesson files were read completely: global 5,230 bytes and repository 1,958 bytes, totaling 7,188 bytes and 2,397 conservative estimated tokens. No archive was loaded. Existing phase context records an audit baseline; this review performs no lesson or audit mutation.

The exact explicit seven-file scope is preserved in frontmatter. None is ignored or generated. The fingerprint is SHA256 over entries in inventory order, concatenating each UTF-8 repository-relative path, NUL, lowercase SHA256 of its bytes, and newline. No source or shared-state file was modified.

## Verification Provenance and Limits

This reviewer ran no Cargo, Bazel, build or test command and did not enumerate quarantined dependencies. Root supplied Plan 08 results: final startup 9 passing, idle 7 passing, inherited daemon 59 passing, strict RPC Clippy and scoped formatting passing. The summary records that the broad daemon regression preceded only the equivalent positive-predicate cleanup, followed by affected startup controls and strict Clippy. Counts overlap and are not summed. Native whole-phase verification, coverage, Bazel, Plan 10 closure and Git finalization remain root-owned.

The existing focused controls support local source reasoning but do not close WR-01: no listed test exercises the outer shutdown coordinator after a retained idle index error. Source-level conclusions do not claim a newly executed reproduction. Fixtures use continuous consensus staging with synthetic easy-proof headers and test maturity one; they do not establish public-mainnet behavior. BASIC turn budgets do not impose hard latency bounds on full startup preflight, ordinary coins/prune work or storage.

## Balanced Simplification Pass

The typed two-adapter seam, configured constructor and single existing maintenance loop keep the daemon change small. One elapsed helper avoids duplicating scheduler behavior in tests; the startup uses the constructor's existing first turn. All owned files remain below the managed 628-line refactor trigger. No extra worker, history cache, runtime downcast or dependency is needed. WR-01 is best addressed with a small settlement coordinator and separated clean-marker gate, avoiding repeated ad hoc error branches while preserving explicit failure reporting.

## Result and Remaining Review

Seven files reviewed at standard depth: zero critical findings, one warning and zero informational findings. Resolve WR-01 and re-review the changed shutdown path before claiming this partition clean. The excluded Plan 10 guards/docs/catalog review and full native verification must also close before any whole-phase completion or commit/push claim.

______________________________________________________________________

_Reviewed: 2026-10-05T21:31:28Z_\
_Reviewer: gsd-code-reviewer_\
_Depth: standard; daemon/bin partition only_
