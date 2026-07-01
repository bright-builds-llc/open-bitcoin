---
phase: 102-orphan-handling-and-admission-outcome-bridge
plan: 03
subsystem: network transaction relay
tags:
  - rust
  - managed-runtime
  - mempool
  - orphanage
  - transaction-relay
requires:
  - phase: 102-01
    provides: typed mempool admission outcomes for accepted, duplicate, orphan, rejected, replaced, and evicted submissions
  - phase: 102-02
    provides: bounded transaction orphanage and scheduler-backed orphan parent requests
  - phase: 101-02
    provides: transaction download scheduler actions and request cleanup
provides:
  - managed runtime admission bridge for peer transactions after the Phase 101 download boundary
  - compatibility-safe local outcome submission path beside the existing AdmissionResult API
  - bounded orphan staging, parent request scheduling, and child reconsideration from managed node code
  - disconnect cleanup that removes peer-owned orphanage state alongside transaction request state
  - in-memory integration tests for peer/local admission outcomes, orphan lifecycle, caps, and cleanup
affects:
  - managed peer network
  - transaction relay admission
  - mempool outcome bridge
  - orphan handling
tech-stack:
  added: []
  patterns:
    - managed runtime bridge owns mempool/orphan mutation for peer transactions
    - outcome-aware APIs added beside compatibility APIs instead of changing existing callers
    - parent requests translated through scheduler actions rather than direct socket writes
key-files:
  created:
    - packages/open-bitcoin-node/src/network/admission_bridge.rs
    - packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/network/inbound.rs
    - packages/open-bitcoin-node/src/network/inventory.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Preserved `ManagedPeerNetwork::submit_local_transaction -> AdmissionResult` and added `submit_local_transaction_outcome` for outcome-aware local callers."
  - "Kept peer transaction mempool and orphan mutation in the managed node admission bridge, after `PeerAction::ReceivedTransaction` and the Phase 101 download boundary."
  - "Translated missing-parent requests through the existing transaction download scheduler/action path instead of adding direct socket writes."
  - "Ran orphanage peer cleanup from managed disconnect cleanup alongside transaction request cleanup."
patterns-established:
  - "Peer admission maps `MempoolOutcome` into bounded orphan staging, reconsideration, index maintenance, and scheduler actions in one managed bridge module."
  - "Tests use in-memory peers and fake-time orphan policy helpers to prove expiry and cap behavior without wall-clock sleeps."
requirements-completed:
  - DL-03
  - DL-04
  - DL-05
  - MEM-01
  - MEM-02
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 102-2026-06-30T14-54-50
generated_at: 2026-07-01T04:11:24Z
duration: 50m 54s
completed: 2026-07-01
---

# Phase 102 Plan 03: Managed Runtime Admission Bridge Summary

**Managed peer transaction admission bridge mapping mempool outcomes into bounded orphan staging and scheduler-backed parent requests while preserving RPC AdmissionResult compatibility.**

## Performance

- **Duration:** 50m 54s
- **Started:** 2026-07-01T03:20:30Z
- **Completed:** 2026-07-01T04:11:24Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Added `process_peer_transaction_admission` and `TxOrphanage` ownership to `ManagedPeerNetwork`, so peer transactions are admitted only after inventory download delivers `PeerAction::ReceivedTransaction`.
- Added `ManagedPeerNetwork::submit_local_transaction_outcome` while keeping the existing `submit_local_transaction -> AdmissionResult` path for RPC callers.
- Mapped accepted, duplicate, orphaned, rejected, replaced, evicted, and expired `MempoolOutcome` states into managed in-memory index updates and typed evidence.
- Routed missing-parent requests through Phase 101 scheduler actions and verified request caps/resource suppression instead of direct network writes.
- Added managed disconnect cleanup for peer-owned orphanage state while preserving transaction request cleanup.
- Added in-memory bridge tests covering peer/local outcome paths, orphan lifecycle, parent reconsideration, resource caps, replacement index removal, and disconnect cleanup.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add managed admission bridge and local outcome submission** - `eeef6bc2` (`feat`)
2. **Task 2: Prove managed bridge behavior with in-memory integration tests** - `a06c0a8f` (`test`)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/admission_bridge.rs` - Managed admission/orphan bridge translating peer and local `MempoolOutcome` values into orphanage actions, parent requests, reconsideration, and inventory index updates.
- `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs` - In-memory regression suite for bridge outcomes, orphan staging, parent request scheduling, reconsideration, expiry, eviction, replacement, and disconnect cleanup.
- `packages/open-bitcoin-node/src/network.rs` - Bridge field, peer transaction routing, compatibility-preserving local submission APIs, and orphanage ownership.
- `packages/open-bitcoin-node/src/network/action_translation.rs` - Managed `disconnect_peer_at` cleanup path and transaction request/orphan cleanup integration.
- `packages/open-bitcoin-node/src/network/inventory.rs` - Stored transaction removal helper for replaced and evicted mempool outcomes.
- `packages/open-bitcoin-node/src/network/inbound.rs` - Refreshed parity breadcrumb metadata after source registration.
- `packages/open-bitcoin-node/src/network/tests.rs` - Test module wiring and refreshed parity breadcrumb metadata.
- `docs/parity/source-breadcrumbs.json` - Registered new managed admission bridge source and tests in `node-network-adapter`.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC metric during verification hooks.

## Decisions Made

- Preserved the existing RPC-facing `AdmissionResult` API and introduced the outcome-aware local method beside it to avoid a breaking return-type change.
- Kept all peer admission mutation in the managed runtime layer, so peer/socket code still emits actions and does not submit directly to mempool or orphanage state.
- Used the Phase 102 orphanage and Phase 101 scheduler together: orphan staging stays bounded and deterministic, while parent fetches continue through governed `getdata` scheduling.
- Cleaned peer orphanage state during managed disconnect handling at the same boundary where transaction request state is cleaned, giving one teardown path for peer-owned relay state.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Preserved hook requirement during TDD RED**
- **Found during:** Task 1 and Task 2 TDD flow
- **Issue:** Formal RED commits could not be recorded because repo hooks require passing verification before every commit.
- **Fix:** Observed failing focused tests locally during implementation, then committed the passing implementation and test slices atomically per task.
- **Files modified:** No additional files beyond task implementation.
- **Verification:** Focused bridge tests failed before completion and passed after implementation; final task commits passed hooks.
- **Committed in:** `eeef6bc2`, `a06c0a8f`

**2. [Rule 3 - Blocking] Refreshed tracked parity and LOC artifacts**
- **Found during:** Task 1 and Task 2 verification hooks
- **Issue:** Adding new first-party Rust source/test files required parity breadcrumb registration, and verification regenerated the tracked LOC report.
- **Fix:** Registered the new bridge files under `node-network-adapter` and committed the regenerated `docs/metrics/lines-of-code.md`.
- **Files modified:** `docs/parity/source-breadcrumbs.json`, `docs/metrics/lines-of-code.md`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check` passed with 319 Rust files; `bash scripts/verify.sh` reported the LOC report current.
- **Committed in:** `eeef6bc2`, `a06c0a8f`

**3. [Rule 1 - Test Fixture] Corrected orphan reconsideration test expectations**
- **Found during:** Task 2 focused test run
- **Issue:** The first still-missing-parent test expected a child outcome after only one of two missing parents arrived, but `TxOrphanage::reconsider_after_parent` correctly waits until all missing parents are satisfied.
- **Fix:** Adjusted the test to assert no reconsidered child, the child remaining staged, and no transaction store growth after the first parent.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs`
- **Verification:** `timeout 180s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib managed_admission_bridge -- --nocapture` passed: 13 tests.
- **Committed in:** `a06c0a8f`

**4. [Rule 1 - Test Fixture] Corrected local orphan compatibility fixture**
- **Found during:** Task 2 focused test run
- **Issue:** The initial compatibility fixture accidentally spent a transaction already accepted into the mempool when it was meant to exercise an orphaned local submission.
- **Fix:** Used a distinct coinbase fixture for the missing-parent local child case.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs`
- **Verification:** `managed_admission_bridge_existing_local_submission_preserves_admission_result_compatibility` passed in the focused bridge suite.
- **Committed in:** `a06c0a8f`

**Total deviations:** 4 auto-fixed (2 Rule 1, 2 Rule 3)
**Impact on plan:** No scope creep. The fixes preserved repo verification, corrected test fixtures, and kept the admission bridge behavior aligned with the planned bounded orphan semantics.

## Issues Encountered

- The repo hook regenerated `docs/metrics/lines-of-code.md` during both task commits; it was intentionally included with the relevant task commits.
- The final full verifier produced expected third-party `secp256k1-sys` C build warnings under Bazel, but the Bazel build and `bash scripts/verify.sh` completed successfully.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all --check` passed.
- `timeout 180s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib managed_admission_bridge -- --nocapture` passed: 13 tests.
- `timeout 120s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib managed_network_transaction_relay -- --nocapture` passed: 5 tests.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --no-run` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed: 319 Rust files verified.
- `cargo fmt --manifest-path packages/Cargo.toml --all`, `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`, and `cargo test --manifest-path packages/Cargo.toml --all-features` passed before the Task 2 commit.
- `bash scripts/verify.sh` passed after both task commits; final plan-level run completed in 6m 12.025s.

## Known Stubs

None. The touched Rust files were scanned for placeholder text, TODO/FIXME markers, and hardcoded empty/null UI-flow stubs; no matches were found.

## Threat Flags

None. The plan added no new network listener, auth path, file access pattern, schema change, or new external trust boundary. The new runtime behavior stays inside the existing managed peer message/admission path.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 102 now has the runtime bridge between transaction download, mempool outcomes, bounded orphan staging, and scheduler-governed parent requests. Later Phase 102 work can build on typed managed admission outcomes without changing existing RPC local transaction callers or bypassing the Phase 101 request governance path.

*Phase: 102-orphan-handling-and-admission-outcome-bridge*
*Completed: 2026-07-01*

## Self-Check: PASSED

- Verified key created/modified files exist.
- Verified task commits `eeef6bc2` and `a06c0a8f` resolve to commit objects in git history.
- Verified Markdown frontmatter uses only the opening and closing `---` delimiters.
