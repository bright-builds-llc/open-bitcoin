---
phase: 135-snapshot-schema-checkpointing-and-recovery
plan: "04"
subsystem: node-runtime
tags: [rust, mempool, checkpointing, lifecycle-authority, durability-evidence]
requires:
  - phase: 135-01
    provides: versioned mempool snapshot envelopes with authoritative generation, capture time, and unbroadcast membership
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    provides: affine lifecycle effect capabilities and sole runtime authority
provides:
  - coherent CurrentV2 mempool checkpoint capture under one authority guard
  - generation-, trigger-, time-, and identity-bound affine checkpoint capabilities and receipts
  - bounded checkpoint durability evidence with exact loss intervals and typed completion or failure outcomes
affects: [135-05, mempool-recovery, operator-status, fjall-checkpoint-executor]
tech-stack:
  added: []
  patterns: [functional-core-imperative-shell, affine-effect-receipts, durable-high-water-evidence]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs
    - packages/open-bitcoin-node/src/network/lifecycle_projection/checkpoint.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/capture.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/evidence.rs
  modified:
    - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs
key-decisions:
  - "CurrentV2 is the sole production capture format; LegacyV1 remains decode-only for compatibility."
  - "Typed Sync completion advances the durable high-water mark before freshness classification, while only an exact current completion clears matching dirty state."
  - "Completion-dispatch failure returns ownership of the original non-Clone receipt so a later plan can retry idempotently without fabricating terminal evidence."
patterns-established:
  - "Coherent authority capture: canonical records, exact unbroadcast membership, generation, trigger, and injected capture time are gathered under one guard, then all I/O occurs after the guard is released."
  - "Bounded durability evidence: expose fixed classes, generations, counts, and injected times without transaction identities or dynamic labels."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-02T21:33:05Z
duration: 1h 6m
completed: 2026-08-02
---

# Phase 135 Plan 04: Authoritative Checkpoint Capture and Durability Evidence Summary

**Coherent CurrentV2 checkpoint capture with affine Sync receipts and exact generation-loss evidence owned by the sole network runtime authority**

## Performance

- **Duration:** 1h 6m
- **Started:** 2026-08-02T20:27:24Z
- **Completed:** 2026-08-02T21:33:05Z
- **Tasks:** 2
- **Files modified:** 29

## Accomplishments

- Captures canonical mempool records and authoritative unbroadcast membership together with generation, injected capture time, and Periodic or Shutdown trigger under one lifecycle guard.
- Carries epoch, effect identity, snapshot identity, generation, capture time, trigger, completion time, and Sync strength through non-Clone capability and receipt contracts.
- Projects current, dirty, in-flight, and durable generations with exact `(last_durable, current]` loss intervals, overdue state, clock-rollback saturation, and low-cardinality outcomes or failures.
- Preserves newer dirty state across stale success, treats duplicate completion idempotently, and retains the original achieved receipt after completion-dispatch failure.
- Keeps encoding and Fjall storage outside the authority guard while retaining the one-pending-snapshot limit.

## Task Commits

Each task was committed atomically:

1. **Task 1: Capture one coherent source envelope through the dispatcher** - `a5e04b52` (feat)
2. **Task 2: Add truthful generation, completion, and loss-window evidence** - `908003a0` (feat)

The plan summary and generated metadata freshness are recorded by the final documentation commit.

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs` - Owns immutable checkpoint requests, affine capabilities and receipts, typed completion strength, and typed abort failures.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/checkpoint.rs` - Defines capture triggers and checkpoint projection support.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs` - Tracks authoritative capture, in-flight, dirty, durable-high-water, outcome, overdue, and loss-window evidence.
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` - Captures CurrentV2 source truth under one guard and applies exact completion or abort transitions.
- `packages/open-bitcoin-node/src/network/runtime_authority/effects.rs` - Exposes explicit capture and typed terminal APIs while retaining the Plan 05 compatibility seam.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/capture.rs` - Proves coherent authoritative capture and immutable metadata binding.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/evidence.rs` - Proves exact, stale, duplicate, abort, foreign, dispatch-failure, overdue, and loss-range behavior.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs` - Owns the remaining legacy Fjall executor coverage until Plan 05 migrates its terminal contract.
- `docs/parity/source-breadcrumbs.json` - Registers breadcrumbs for the new first-party Rust test source.
- `scripts/check-phase130-resource-time-fee-primitives.ts` and `scripts/check-phase134-authoritative-lifecycle.ts` - Keep live architecture checks aligned with the split modules and decode-only v1 boundary.

## Decisions Made

- Kept LegacyV1 strictly decode-only and removed production construction routes that could invent generation, capture time, or unbroadcast authority.
- Used distinct typed terminal method names so Plan 04 can add truthful checkpoint evidence without overloading or prematurely deleting the Fjall executor's Plan 05 compatibility API.
- Recorded achieved durable generation before classifying receipt freshness; this makes stale success truthful without allowing it to clear newer dirty state.
- Boxed the retained receipt inside completion-dispatch errors to keep the error size bounded while preserving affine ownership through `into_receipt()`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Updated stale snapshot fixtures for CurrentV2 validation**

- **Found during:** Task 1 verification
- **Issue:** Existing Fjall and RPC fixtures still constructed legacy-shaped snapshots and failed once production capture required CurrentV2 metadata and exact membership validation.
- **Fix:** Migrated fixtures to deterministic CurrentV2 values and added the RPC test-only mempool dependency needed to build canonical records.
- **Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs`, `packages/open-bitcoin-rpc/Cargo.toml`, `packages/open-bitcoin-rpc/src/context/tests.rs`, and snapshot codec tests.
- **Verification:** Workspace tests and `bash scripts/verify.sh` passed.
- **Committed in:** `a5e04b52`

**2. [Rule 3 - Blocking] Repaired live Phase 130 and Phase 134 checkers after module extraction**

- **Found during:** Tasks 1 and 2 verification
- **Issue:** Architecture checks assumed all lifecycle markers remained in root modules and treated any v1 decoder text as a production write route.
- **Fix:** Made the checks aggregate the owned child modules, recognize the decode-only v1 boundary, and update mutation fixtures to the extracted checkpoint module path.
- **Files modified:** `scripts/check-phase130-resource-time-fee-primitives.ts`, its tests, `scripts/check-phase134-authoritative-lifecycle.ts`, and its tests.
- **Verification:** All 248 Phase 134 mutation cases, the Phase 130 checker tests, and the full verifier passed.
- **Committed in:** `a5e04b52`, `908003a0`

**3. [Rule 3 - Blocking] Split checkpoint contracts to satisfy managed file-shape limits**

- **Found during:** Task 2 full verification
- **Issue:** Adding typed evidence pushed lifecycle modules beyond the managed source-length limit; the initial projection root was exactly 628 lines where the verifier requires fewer than 628.
- **Fix:** Moved checkpoint-specific contracts into the existing child module and trimmed the projection root to 626 lines without changing behavior.
- **Files modified:** `packages/open-bitcoin-node/src/network/lifecycle_effects.rs`, `packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs`, and `packages/open-bitcoin-node/src/network/lifecycle_projection.rs`.
- **Verification:** Bright Builds checks and the complete repo verifier passed.
- **Committed in:** `908003a0`

**4. [Rule 3 - Blocking] Added required parity breadcrumb and generated metadata freshness**

- **Found during:** Task 2 verification
- **Issue:** The new first-party evidence test file required a parity breadcrumb, and verification regenerated the tracked lines-of-code report.
- **Fix:** Registered the evidence test source and committed the verifier-generated metrics updates.
- **Files modified:** `docs/parity/source-breadcrumbs.json`, `docs/metrics/lines-of-code.md`.
- **Verification:** Parity breadcrumb checks and the full verifier passed.
- **Committed in:** `a5e04b52`, `908003a0`

**5. [Rule 3 - Blocking] Preserved staged requirement activation until phase verification**

- **Found during:** Final metadata verification
- **Issue:** Copying the plan requirement IDs into `requirements-completed` would activate MPDUR-01 and MPDUR-04 before Phase 135 has lifecycle-valid verification coverage, violating the repository's active-milestone traceability contract.
- **Fix:** Followed the existing Phase 135 staged-summary convention and left `requirements-completed` empty; Phase 135's final implementation/parity and verification plans retain responsibility for activating and closing the requirements.
- **Files modified:** `.planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-04-SUMMARY.md`.
- **Verification:** The active-milestone traceability checker and metadata commit hook passed.
- **Committed in:** final documentation commit

**Total deviations:** 5 auto-fixed (1 bug, 4 blocking)
**Impact on plan:** All changes were necessary to keep existing fixtures, managed architecture checks, file-shape policy, and tracked generated metadata correct. No architectural scope expansion was introduced.

## Issues Encountered

- The complete verifier initially stopped on the strict projection-file length boundary. The module was simplified from 628 to 626 lines and the verifier then passed.
- Clippy identified a verbose optional comparison and an oversized completion-dispatch error. The comparison was simplified and retained receipt ownership was boxed without changing caller semantics.

## Verification

- `cargo fmt --all` - passed
- `cargo clippy --all-targets --all-features -- -D warnings` - passed
- `cargo build --all-targets --all-features` - passed
- `cargo test --all-features` - passed
- Focused checkpoint capture, evidence, storage, and lifecycle-effect suites - passed
- Phase 130 and Phase 134 live/mutation checkers - passed, including 248 Phase 134 mutation cases
- `bun scripts/bright-builds-check.ts all` - passed
- `bash scripts/verify.sh` - passed before Task 2 commit; the Task 2 pre-commit hook passed it independently again in 3m 58s

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 05 can migrate the Fjall executor to the distinct typed terminal transitions, then remove the retained snapshot compatibility methods and rename the typed APIs atomically.
- MPDUR-01 and MPDUR-04 remain intentionally unactivated in staged plan summaries until lifecycle-valid Phase 135 verification exists.
- No blockers remain for recovery and operator-status consumers of checkpoint evidence.

## Self-Check

PASSED

- Summary file exists at the declared output path.
- Task commits `a5e04b52` and `908003a0` exist in repository history.
- Stub scan found no TODO, FIXME, placeholder, coming-soon, or unavailable markers in the plan's created and modified runtime/test files.
- `git diff --check` passed.

***

*Phase: 135-snapshot-schema-checkpointing-and-recovery*
*Completed: 2026-08-02*
