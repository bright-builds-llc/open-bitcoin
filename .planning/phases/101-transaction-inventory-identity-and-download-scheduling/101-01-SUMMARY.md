---
phase: 101-transaction-inventory-identity-and-download-scheduling
plan: 01
subsystem: network
tags: [rust, transaction-relay, txid, wtxid, scheduler, fake-clock]
requires: []
provides:
  - typed txid/wtxid transaction relay identity contracts
  - pure deterministic transaction download scheduler
  - low-cardinality request, suppression, fallback, expiry, and cleanup actions
  - fake-clock scheduler tests and parity breadcrumbs
affects:
  - 101-transaction-inventory-identity-and-download-scheduling
  - open-bitcoin-network
tech-stack:
  added: []
  patterns:
    - pure scheduler with explicit caller-provided time
    - transaction relay identity conversion at InventoryVector boundary
    - test wrappers in public module with split helper modules for file-length compliance
key-files:
  created:
    - packages/open-bitcoin-network/src/peer/transaction_relay.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/edge_cases.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/received_cases.rs
  modified:
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Keep transaction download scheduling pure: callers provide time, local facts, and peer state inputs."
  - "Expose relay scheduler contracts through peer and crate re-exports without wiring runtime behavior in this plan."
  - "Split scheduler tests into helper modules while preserving required test names in transaction_relay/tests.rs."
patterns-established:
  - "Scheduler APIs return typed TxDownloadAction values instead of mutating runtime/network state."
  - "TxRelayId is the single request-state identity for txid and wtxid inventory."
  - "Fake-clock tests cover delay, timeout, fallback, notfound, disconnect, and received-transaction cleanup."
requirements-completed: [INV-01, INV-02, INV-03, INV-04, DL-01, DL-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 101-2026-06-29T21-00-59
generated_at: 2026-06-29T23:32:21Z
duration: 1h 8m
completed: 2026-06-29
---

# Phase 101 Plan 01: Typed Transaction Relay Scheduler Summary

**Typed txid/wtxid relay identity plus a pure fake-clock transaction download scheduler with fallback, expiry, and cleanup actions.**

## Performance

- **Duration:** 1h 8m
- **Started:** 2026-06-29T22:24:56Z
- **Completed:** 2026-06-29T23:32:21Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments

- Added `TxRelayId`, `TxRelayPeerMode`, identity mismatch errors, fixed Phase 101 policy constants, suppression reasons, and low-cardinality action labels.
- Added a pure `TxDownloadScheduler` that records announcements, suppresses duplicates and local-known inventory, expires in-flight requests, schedules fallback peers, and cleans up on `notfound`, disconnect, or received transaction.
- Covered scheduler behavior with deterministic fake-clock tests and registered every new Rust source/test helper in parity breadcrumbs.

## Task Commits

1. **Task 1: Define typed relay identity, policy constants, and action labels** - `bc1b9eb1` (`feat`)
2. **Task 2: Implement pure scheduler state and deterministic edge-case tests** - `13337c52` (`feat`)

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` - Public transaction relay identity, action, policy, and scheduler re-export surface.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - Pure transaction download scheduler state machine.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs` - Required public test names for identity and scheduler contracts.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs` - Main deterministic scheduler behavior cases.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/edge_cases.rs` - Branch coverage for no-op cleanup, cap, and missing-state paths.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/received_cases.rs` - Received transaction cleanup and already-have behavior.
- `packages/open-bitcoin-network/src/peer.rs` - Re-exported transaction relay contracts without runtime wiring.
- `packages/open-bitcoin-network/src/lib.rs` - Re-exported public network crate transaction relay contracts.
- `docs/parity/source-breadcrumbs.json` - Registered transaction relay source and test helper breadcrumbs.
- `docs/metrics/lines-of-code.md` - Refreshed tracked generated LOC report from repo hooks.

## Decisions Made

- Scheduler logic remains in the functional core: no sockets, storage, mempool calls, Tokio, logging, or wall-clock APIs.
- The scheduler accepts `now_unix_seconds` and local facts explicitly, which keeps timeout and delay behavior deterministic.
- Test wrappers stay in `transaction_relay/tests.rs` so plan-required test names remain discoverable while helper modules keep files below repo file-length limits.
- Public re-exports were added through `peer.rs` and `lib.rs` so the new contracts are reachable and clippy-clean without changing runtime behavior.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all --check`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features --no-run`
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib transaction_relay -- --nocapture`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `bash scripts/check-file-lengths.sh`
- No forbidden time APIs found in transaction relay source or tests.
- Pure-core `cargo llvm-cov` slice reported no uncovered `transaction_relay` lines.
- Full Rust pre-commit sequence passed: fmt, workspace clippy, workspace build, and workspace tests.
- Git hooks ran the repo verifier for both task commits.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split transaction relay tests for file-length compliance**
- **Found during:** Task 1 and Task 2 verification
- **Issue:** Inline scheduler and identity tests would exceed the repo file-length gate.
- **Fix:** Kept required test names in `transaction_relay/tests.rs` and moved longer bodies into sibling helper modules.
- **Files modified:** `packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/edge_cases.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/received_cases.rs`
- **Verification:** `bash scripts/check-file-lengths.sh`
- **Committed in:** `bc1b9eb1`, `13337c52`

**2. [Rule 3 - Blocking] Re-exported new contracts through existing public module boundaries**
- **Found during:** Task 1 and Task 2 clippy verification
- **Issue:** Private child-module contracts would be harder for later plans to use and risk dead-code/lint friction.
- **Fix:** Re-exported transaction relay contracts from `peer.rs` and `lib.rs` without wiring runtime behavior.
- **Files modified:** `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/lib.rs`
- **Verification:** workspace clippy and focused transaction relay tests
- **Committed in:** `bc1b9eb1`, `13337c52`

**3. [Rule 1 - Bug] Covered scheduler edge branches required by pure-core coverage**
- **Found during:** Task 2 commit hook
- **Issue:** The first Task 2 commit attempt failed the pure-core coverage gate on no-op `notfound`, empty/candidate-only cleanup, fallback cap, and candidate-removal branches.
- **Fix:** Added focused deterministic edge-case tests for those paths.
- **Files modified:** `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/edge_cases.rs`
- **Verification:** pure-core `cargo llvm-cov` slice reported no uncovered `transaction_relay` lines.
- **Committed in:** `13337c52`

**Total deviations:** 3 auto-fixed.
**Impact on plan:** No scope creep; the adjustments were required for repo verification, public contract usability, and coverage compliance.

## Issues Encountered

- The initial Task 2 hook attempt failed at coverage after all build/test stages had passed. The uncovered branches were fixed with additional deterministic tests, and the final commit hook passed.
- The user interrupted while the Task 2 commit hook was still running. The hook continued in the background and completed successfully; no duplicate commit was started.

## Auth Gates

None.

## Known Stubs

None. Stub scan found no `TODO`, `FIXME`, placeholder text, empty hardcoded UI data, or unwired mock data in files created or modified by this plan.

## Threat Flags

None. This plan added pure in-memory transaction relay scheduling contracts only; it did not introduce new network endpoints, auth paths, file access, schema changes, or runtime trust-boundary behavior.

## User Setup Required

None.

## Next Phase Readiness

Plan 02 can wire transaction inventory admission and relay policy to these typed identities and scheduler actions. The scheduler is ready for caller-provided mempool-known, already-have, recent-reject, peer-mode, and fake-clock inputs.

## Phase State

Per orchestrator instruction, this executor did not advance phase state, update roadmap progress, mark requirements complete, or transition to the next phase.

## Self-Check: PASSED

- Found summary file: `.planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-01-SUMMARY.md`
- Found task commit: `bc1b9eb1`
- Found task commit: `13337c52`
- Verified summary uses standalone `---` only for frontmatter delimiters.
