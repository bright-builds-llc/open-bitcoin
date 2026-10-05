---
phase: 157-safe-activation-and-scheduled-index-catch-up
fixed_at: 2026-10-05T21:47:23Z
review_path: .planning/phases/157-safe-activation-and-scheduled-index-catch-up/157-REVIEW.md
iteration: 1
fix_scope: critical_warning
findings_in_scope: 1
fixed: 1
skipped: 0
status: all_fixed
generated_by: gsd-code-review-fix
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T21:47:23Z
commits: []
git_finalization: pending root clean whole-phase gate
whole_phase_review_complete: false
verification_owner: root
---

# Phase 157: Code Review Fix Report

**Fixed at:** 2026-10-05T21:47:23Z\
**Source review:** `.planning/phases/157-safe-activation-and-scheduled-index-catch-up/157-REVIEW.md`\
**Iteration:** 1

**Summary:**

- Findings in scope: 1 warning.
- Fixed: 1.
- Skipped: 0.
- Git finalization and independent re-review remain root-owned.

## Fixed Issues

### WR-01: Retained index error bypasses final mempool settlement and worker joins

**Status:** fixed: requires human verification (shutdown ordering logic; root independent re-review remains pending).\
**Commit:** none; the authorized strict wrapper defers all task/TDD/metadata staging, commits and pushes until root's clean whole-phase gate.\
**Files modified:**

- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/checkpoint.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs`
- `scripts/check-phase135-snapshot-recovery.ts`
- `scripts/check-phase135-snapshot-recovery.test.ts`

**Applied fix:** The actual daemon calls `settle_daemon_shutdown`, which eagerly attempts sync join, coins Always/join, retry join and final mempool settlement/join. Every worker and HTTP result is examined after these effects; every failure is reported and the first error is returned. Retry quiesces before final mempool capture. Clean-marker publication is a separate callback reached only after all worker and HTTP results succeed. The production mempool worker now exposes settlement-only `shutdown_settle`; old clean conveniences remain test-only. Coins and mempool shutdown still join if their stop channel is disconnected, and report secondary join/settlement failures before returning the signal failure.

The index failure remains visible, Always/Periodic cadence and automatic pruning remain owned by the existing coins loop, and saved Active protection is retained. No new service, worker, dependency, source path, storage schema, status projection, network activation or runtime import/repair was introduced. The eight-path scope was confirmed internally before edits; seven paths required changes. The eighth allowed startup test path remained untouched. No rollback was needed; the initial worktree already contained uncommitted phase implementation and other agents' changes were preserved.

## Behavioral Evidence and Verification

The actual bug RED at **21:40:29.037 UTC** executed one registered outer-shutdown control against the extracted old early-return sequence: **0 passed, 1 failed**, 59 filtered, **1.86s** test time / **4,571ms** wrapper. Its genuine idle turn reported `missing BASIC required body`; Always had earned accepted coins fence B40. The failure was `outer shutdown must join retry despite retained BASIC error`, proving cleanup was skipped. Two earlier fixture attempts were setup/expectation failures and are not bug RED evidence: retained successor facts initially prevented the intended missing-body error, and an absent metadata record initially tripped a fixture expectation.

The final regression accepts B40 while the configured startup owner is behind at B7, omits body40, advances ordered backlog prefixes through B39, and injects a genuine elapsed turn needing the missing body. Its actual `CoinsFlushWorker` receives a channel stop and joins after successful Always, returning the retained failure. The same production outer coordinator joins the actual retry worker and settles/joins the actual mempool worker. Public checkpoint evidence equals the current durable generation, the clean marker remains absent/false, the error is returned, and saved Active protection/prune locks are unchanged. It uses no timing sleeps, sockets or public network.

| Check                                                                  | Result                                                                                                      |
| ---------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Initial corrected coordinator GREEN                                    | 1 passed, 0 failed; 1.29s harness / 4,179ms wrapper                                                         |
| Final `cargo test --bin open-bitcoind phase157_shutdown_`              | **4 passed**, 0 failed, 59 filtered; **1.48s** harness / **4,945ms** wrapper; 21:46:43.607–21:46:48.552 UTC |
| Inherited `cargo test --bin open-bitcoind`                             | **63 passed**, 0 failed/ignored/filtered; **139.81s** harness / **140,477ms** wrapper                       |
| Strict normal RPC `clippy --all-targets --all-features -- -D warnings` | Passed; **3.36s** Cargo / **3,810ms** wrapper                                                               |
| Phase 135 live guard                                                   | Passed: snapshot recovery invariants verified                                                               |
| Phase 135 mutation/snapshot suite                                      | **88 passed**, 0 failed; **155 expectations**, **1.56s**                                                    |
| Scoped Rust 2024 formatting and owned diff whitespace                  | Passed                                                                                                      |

All Cargo commands used pinned Bun 1.3.9 and the command-timings cooperative build lock; there was no Cargo overlap or quarantined-dependency enumeration. The inherited run includes genuine Periodic/Always, checkpoint/clean-marker, automatic retention/index protection, retry and daemon controls. It preceded only the final real coins-thread test strengthening and secondary signal/join failure diagnostics; all four affected shutdown controls, strict normal RPC Clippy and scoped formatting passed afterward. No further broad rerun was warranted. Existing inherited networking tests use loopback; the new controls perform no network work.

The four production-coordinator controls are:

- `phase157_shutdown_retained_basic_error_settles_all_workers_without_clean_marker`
- `phase157_shutdown_attempts_every_settlement_before_returning_first_failure`
- `phase157_shutdown_http_failure_withholds_clean_marker_after_settlement`
- `phase157_shutdown_success_marks_clean_only_after_all_settlement`

## Guard Adaptation and Simplification

Phase 135's historical shutdown assertions now inspect the actual production coordinator and actual daemon callbacks, including eager joins, retry-before-checkpoint order, failure-before-clean gating and production availability. Independent negative controls reject an early coins-error return, missing retry join, premature clean marker, swallowed retained error and a test-only coordinator. The Rust source assertion likewise targets the real production sequence; no fake calls or comments preserve old anchors.

A separate stale Phase 135 startup anchor surfaced during guard validation: `open_with_runtime_activation` now delegates to the configured constructor. Its guard was retargeted to `open_with_configured_runtime_activation`, genuine `initialize_configured`, coins-cache hydration and sealed recovered-chainstate construction. The existing independent negative mutation removing actual coins-cache attachment remains green. This was stale guard routing, not additional WR-01 bug RED evidence; no production startup change was made. An introduced duplicate TypeScript local name and initially insufficient premature-marker predicate were corrected before the positive live/snapshot baseline and all independent mutations passed.

The simplification pass keeps one small eager coordinator in the existing checkpoint adapter, settlement-only worker ownership and an explicit clean callback. It avoids a new shutdown service, aggregate public error type or duplicated per-worker branching. Plan 10 received the frozen helper/API and four test names and owns the corresponding Phase 157 guard mutation. Root owns refreshed source fingerprints, independent re-review, whole-phase native verification, lifecycle/security closure and Git finalization.

AGENTS.md, its Bright Builds sidecar, placeholder-only overrides, the standards index and architecture/code-shape/testing/verification/local-guidance/Rust/TypeScript standards materially informed this fix. Both active lesson sources were fully loaded within **7,188 bytes / 2,397 estimated tokens**; no archives or new audit trigger were used. Project skill directories are absent. The authorized GSD review-fix workflow supplied the checkable scope plan and iteration-one canonical review. No STATE/ROADMAP/REQUIREMENTS/config mutation or requirement activation occurred.

______________________________________________________________________

_Fixer: gsd-code-fixer_\
_Iteration: 1; root finalization pending_
