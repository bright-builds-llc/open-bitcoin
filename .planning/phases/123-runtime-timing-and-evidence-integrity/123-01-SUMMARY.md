---
phase: 123-runtime-timing-and-evidence-integrity
plan: 01
subsystem: sync-runtime
tags: [compact-block-relay, tcp-framing, caller-clock, idle-maintenance, HARD-02]

requires:
  - phase: 120-compact-download-timeout-and-misbehavior-runtime-bridge
    provides: peer-targeted compact timeout expiration and full-block fallback actions
  - phase: 122-compact-relay-peer-completion
    provides: completed bounded compact-relay peer behavior before runtime hardening
provides:
  - typed Message, Idle, and Closed receive outcomes across every first-party sync session
  - byte-progress-aware TCP framing that rejects partial-frame timeout and EOF
  - caller-clocked idle maintenance with same-session target enforcement and no false progress
affects:
  - 123-runtime-timing-and-evidence-integrity
  - v2.1 hardening and closeout evidence

tech-stack:
  added: []
  patterns:
    - Caller-owned clocks enter the runtime shell only at an explicit idle maintenance boundary
    - Typed receive outcomes replace optional sentinels for protocol lifecycle states
    - Peer-targeted timeout actions are validated as a complete batch before any fallback write

key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs
  modified:
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/tcp.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-bench/src/runtime_fixtures.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs

key-decisions:
  - "Only a zero-progress header timeout is Idle and only zero-progress header EOF is Closed; every partial header or payload interruption is an I/O error"
  - "Every Idle wake samples the injected clock once, expires compact downloads once, validates all targets, then writes same-session fallback through send_all"
  - "Existing timestamp-only wrappers retain fixed clocks while the daemon uses the required live clock entrypoint"

patterns-established:
  - "Pattern: effectful adapters acquire wall time while the sync runtime consumes an injected FnMut clock"
  - "Pattern: finite scripted sessions end explicitly with Closed; Idle never consumes receive budget or progress credit"

requirements-completed:
  - HARD-02
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 123-2026-07-15T18-12-00
generated_at: 2026-07-16T01:59:41Z

duration: 24min
completed: 2026-07-16
---

# Phase 123 Plan 01: Deterministic Idle Maintenance Summary

**Blocking peer sessions now distinguish message, idle, and close outcomes so compact-download timeouts advance from a fresh caller clock without discarding partial frames, misrouting fallback, or manufacturing receive progress.**

## Performance

- **Duration:** 24 min
- **Started:** 2026-07-16T01:36:00Z
- **Completed:** 2026-07-16T01:59:41Z
- **Tasks:** 1 TDD migration
- **Files changed:** 8

## Accomplishments

- Replaced the ambiguous optional receive sentinel with an exhaustive public `SyncPeerReceiveOutcome` and migrated TCP, node tests, runtime fixtures, and bench fixtures atomically.
- Replaced `read_exact_or_stall` with one byte-counting read loop that retries interruptions and distinguishes clean zero-progress idle/close from every truncated header or payload error.
- Threaded a required caller clock through resolver, retry, and connected-session paths; the daemon now supplies `current_timestamp_unix_seconds` without another timer, thread, or async dependency.
- Preserved peer targets from the existing timeout forwarder, rejected any non-owning target before fallback writes, and kept Idle outside message budgets and progress accounting.
- Added nine deterministic local tests covering framing, retained idle sessions, fake-clock fallback, receive budgets, clean close, and cross-peer target rejection.

## Task Commits

1. **Task 1 RED: failing receive and idle-maintenance coverage** — `749a471f`
2. **Task 1 GREEN: typed receive migration and clocked maintenance** — `f07bbaa3`

## Verification Results

```text
Phase 123 TCP receive filter: 2 passed, 0 failed
Phase 123 complete focused suite: 9 passed, 0 failed
Existing compact-timeout regressions: 4 passed, 0 failed
open-bitcoin-bench --all-targets --all-features check: passed without warnings
open-bitcoin-rpc daemon_sync filter: 13 passed, 0 failed
affected node/RPC/bench clippy --all-targets --all-features -D warnings: passed
all SyncPeerSession implementation files contain SyncPeerReceiveOutcome: passed
git diff --check: passed
```

The orchestrator owns the merged-wave `bash scripts/verify.sh` contract, including the Bazel smoke build and final parity breadcrumb gate.

## Decisions Made

- The TCP classifier exposes one deterministic generic `read_stage` loop; the production wrapper only replaces the generic test peer label with the real peer label.
- Target validation happens across the entire timeout action batch before `send_all`, preventing an earlier same-peer action from being written when a later action targets another peer.
- The live clock is a method collaborator rather than runtime state, keeping deterministic callers and the daemon on the same code path without a second scheduler.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- The operator summary intentionally sanitizes network error details, so the target-mismatch test was moved to the private connected-session seam to assert the exact fixed error while separately proving no full-block fallback reached the current session.

## User Setup Required

None - no external service configuration required.

## Residual Risks

- Plan 123-07 owns registration of the new focused Rust test file in `docs/parity/source-breadcrumbs.json` and the final deterministic Phase 123 checker.
- Full repository verification, coverage, and Bazel smoke remain the orchestrator's merged-wave gate.

## Next Phase Readiness

- HARD-02 runtime behavior is complete and ready for the successful block-emission evidence work in Plan 123-02.
- The receive contract is publicly re-exported for all later first-party adapters.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-node/src/sync/types.rs` (`SyncPeerReceiveOutcome`)
- FOUND: `packages/open-bitcoin-node/src/sync/tcp.rs` (`ReadStageOutcome` and `read_stage`)
- FOUND: `packages/open-bitcoin-node/src/sync.rs` (`sync_until_idle_with_clock` and target validation)
- FOUND: `packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs` (all nine Phase 123 tests)
- FOUND: `packages/open-bitcoin-bench/src/runtime_fixtures.rs` (external first-party producer migration)
- FOUND: `.planning/phases/123-runtime-timing-and-evidence-integrity/123-01-SUMMARY.md`
- FOUND COMMITS: `749a471f`, `f07bbaa3`
