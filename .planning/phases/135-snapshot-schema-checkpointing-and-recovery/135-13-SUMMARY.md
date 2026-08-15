---
phase: 135-snapshot-schema-checkpointing-and-recovery
plan: "13"
subsystem: recovery
tags: [rust, mempool-snapshot, representability, persisted-input, checkpoint]
requires:
  - phase: 135-12
    provides: fallible CapturedMempoolGeneration::try_new and public decode_mempool_snapshot_with_limits
provides:
  - one assert_mempool_snapshot_representable used by capture and encode
  - encoder ceiling from persisted_mempool_input_limits().max_encoded_bytes
  - exact/one-over Sync/reopen and abort-preserve regressions
affects: [phase-135-verification, MPDUR-01, MPDUR-02, MPDUR-03, snapshot-recovery]
tech-stack:
  added: []
  patterns:
    - prove persisted-input representability before reserve_next and before put_bytes
    - compare encoded bytes to the fixed reader ceiling, not a snapshot-derived bound
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/representability.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/representability.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/representability.rs
    - .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-13-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_limits.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs
    - scripts/check-phase135-snapshot-recovery/persisted-input.ts
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Capture and encode share one assert_mempool_snapshot_representable against persisted_mempool_input_limits()."
  - "encode_mempool_snapshot compares bytes.len() to limits.max_encoded_bytes, not encoded_size_upper_bound of snapshot dimensions."
  - "Keep MPDUR Pending and leave requirements-completed empty until lifecycle-valid phase verification."
  - "Do not recreate canonical 135-VERIFICATION.md; Plan 14 owns fresh verification."
patterns-established:
  - "Unrepresentable live mempools fail capture/encode without dropping records or replacing the prior durable snapshot."
  - "Record-count one-over is proven on persisted_record_count_is_representable rather than a 220097-object Sync fixture."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-15T20:07:08Z
duration: 34m
completed: 2026-08-15
---

# Phase 135 Plan 13: Writer/Reader Representability Summary

**Capture and encode now share one persisted-input representability contract, so a successful Sync checkpoint is admissible to the same binary's bounded startup reader and an unrepresentable live mempool fails closed without dropping records.**

## Performance

- **Duration:** 34 min
- **Started:** 2026-08-15T19:32:25Z
- **Completed:** 2026-08-15T20:07:08Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- `assert_mempool_snapshot_representable` enforces the versioned persisted-input contract (220096 records, 5000 unbroadcast members, 4 MiB per transaction, 64 MiB aggregate transaction bytes, 1636801 input edges, 102300 per-record edges) before any durable replacement.
- `encode_mempool_snapshot` compares final bytes to `limits.max_encoded_bytes` (`268435456`) from `persisted_mempool_input_limits()`, not a snapshot-derived `encoded_size_upper_bound`.
- `PrepareSnapshot` proves representability after `try_new_current` and before `reserve_next`, still copying the live generation through `CapturedMempoolGeneration::try_new`.
- Exact 4 MiB per-transaction prepare/encode/Sync/reopen with `for_persisted_input()` preserves record count, acceptance time, and unbroadcast membership.
- One-over 4 MiB + 1 encode returns `SnapshotWriteExecutionError::Encode` with detail `mempool snapshot exceeds a resource bound`, leaves the prior snapshot unchanged, and exact-aborts so a later prepare succeeds.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add the shared representability contract and close the encoder under reader limits** - `d74d9f88` (feat)
2. **Task 2: Prove representability at capture and through prepare → encode → Sync → reopen** - `8b048ce2` (feat)

**Plan metadata:** pending `docs(135-13): complete writer/reader representability plan`

_Note: TDD RED was proven locally (named representability tests failed before the predicate existed) but not committed separately because pre-commit `verify.sh` rejects a failing tree._

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/representability.rs` - Shared persisted-input representability predicate
- `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs` - Encode calls the predicate and the fixed encoded-byte ceiling
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` - Re-exports `assert_mempool_snapshot_representable`
- `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_limits.rs` - Declares the representability unit-test child
- `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/representability.rs` - Exact/one-over predicate and encode-ceiling tests
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` - Capture-before-reserve representability
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs` - Declares the Sync/reopen child
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/representability.rs` - Exact Sync/reopen and one-over abort-preserve regressions
- `scripts/check-phase135-snapshot-recovery/persisted-input.ts` - Requires the encode-body ceiling and representability call
- `docs/parity/source-breadcrumbs.json` - Registers the new representability sources under `node-mempool-storage`

## Decisions Made

- Use one `assert_mempool_snapshot_representable` for both capture and encode; do not drop records to make a live set fit.
- Compare encoded bytes to `persisted_mempool_input_limits().max_encoded_bytes`, and keep `encoded_size_upper_bound` only as the format-formula proof inside limit construction.
- Prove record-count one-over on `persisted_record_count_is_representable(220_096, 220_097)` instead of materializing 220097 transactions.
- Leave `requirements-completed` empty and MPDUR Pending until Plan 14's independent verification.
- Do not recreate canonical `135-VERIFICATION.md`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split representability unit tests into a child module**
- **Found during:** Task 1
- **Issue:** Adding the five named tests to `mempool_limits.rs` would exceed the 628-line file-length check. `tests.rs` is already at the limit, so a sibling `mod` line there was also blocked.
- **Fix:** Added `snapshot_codec/tests/representability.rs` declared from `mempool_limits.rs`.
- **Files modified:** `snapshot_codec/tests/mempool_limits.rs`, `snapshot_codec/tests/representability.rs`
- **Verification:** Named representability/encode tests passed; both files stay under 628 lines
- **Committed in:** `d74d9f88` (Task 1)

**2. [Rule 3 - Blocking] Required the encode-body persisted ceiling in the Phase 135 checker**
- **Found during:** Task 1 commit
- **Issue:** Encode now contains `if bytes.len() > limits.max_encoded_bytes {`, so the first-occurrence mutation "encoded byte bound removed" no longer touched the decode body the checker inspected. The copied-corpus probe produced zero diagnostics.
- **Fix:** The checker now requires that needle (and `assert_mempool_snapshot_representable`) inside `encode_mempool_snapshot`.
- **Files modified:** `scripts/check-phase135-snapshot-recovery/persisted-input.ts`
- **Verification:** `bun test scripts/check-phase135-snapshot-recovery.test.ts` — 75 pass
- **Committed in:** `d74d9f88` (Task 1)

**3. [Rule 3 - Blocking] Used one Cargo test filter**
- **Found during:** Task 1
- **Issue:** Cargo 1.94 rejects two positional `TESTNAME` arguments from the plan's verify command.
- **Fix:** Ran `representability` (Task 1) and `per_transaction` (Task 2) as single filters that still select the named tests.
- **Files modified:** none
- **Verification:** Named tests passed
- **Committed in:** n/a (command-only)

**4. [Rule 3 - Blocking] cfg(test)-only re-export of the record-count helper**
- **Found during:** Task 2 commit
- **Issue:** `pub(crate) use persisted_record_count_is_representable` in `mempool.rs` was unused in the non-test lib and failed `-D unused-imports`.
- **Fix:** Keep the helper `pub(crate)` in `representability.rs` and re-export it only under `cfg(test)`.
- **Files modified:** `snapshot_codec/mempool.rs`, `snapshot_codec.rs`
- **Verification:** Hook `verify.sh` compiled the lib without unused-import errors
- **Committed in:** `8b048ce2` (Task 2)

---

**Total deviations:** 4 auto-fixed (4 blocking)
**Impact on plan:** All auto-fixes were required for hook-passable commits or the declared test commands. No scope creep, no raised reader limits, and no MPDUR/parity promotion.

## Issues Encountered

- Pre-commit `verify.sh` forbids a TDD RED commit of failing tests; RED was proven locally, then GREEN was committed.
- Canonical `135-VERIFICATION.md` remains absent by instruction. The historical `135-VERIFICATION.historical-2026-08-10.md` was left untracked.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 14 can add writer-side structural mutation coverage and run the reserved full verifier.
- MPDUR-01 through MPDUR-04 remain Pending. Age, unbroadcast, and rolling-fee reset semantics are unchanged.
- `decode_mempool_snapshot_with_limits` stays public; Plan 12's terminal-generation rejection is unchanged.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/representability.rs`
- FOUND: `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/representability.rs`
- FOUND: `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/representability.rs`
- FOUND: `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs`
- FOUND: `.planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-13-SUMMARY.md`
- FOUND: `d74d9f88`
- FOUND: `8b048ce2`
- OK: no canonical `135-VERIFICATION.md`

---
*Phase: 135-snapshot-schema-checkpointing-and-recovery*
*Completed: 2026-08-15*
