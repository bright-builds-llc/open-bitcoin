---
phase: 135-snapshot-schema-checkpointing-and-recovery
plan: "09"
subsystem: mempool-snapshot-recovery
tags: [rust, typescript, mempool, snapshot, recovery, mutation-testing]
requires:
  - phase: 135-01
    provides: bounded versioned mempool snapshot codec
  - phase: 135-05
    provides: authoritative checkpoint capture and Sync persistence
  - phase: 135-08
    provides: pre-Serde key preflight and function-scoped checker ordering
provides:
  - explicit-null preservation for legacy-unknown acceptance age in current v2 snapshots
  - policy-accounting-derived capture, decode, and recovery topology ceilings
  - direct-statement lexical proof for preflight-before-Serde execution
affects: [phase-135-verification, MPDUR-01, MPDUR-04]
tech-stack:
  added: []
  patterns:
    - explicit nullable source facts without fabricated fallback time
    - shared versioned accounting bounds across live and durable mempool paths
    - brace-depth-aware exact statement mutation guards
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/gap_closure.rs
  modified:
    - packages/open-bitcoin-mempool/src/resource.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - packages/open-bitcoin-node/src/network/recovery/topology.rs
    - scripts/check-phase135-snapshot-recovery/source.ts
    - scripts/check-phase135-snapshot-recovery.ts
    - scripts/check-phase135-snapshot-recovery.test.ts
key-decisions:
  - "Represent current-v2 unknown acceptance age as a required key with an explicit null value, preserving missing-key rejection."
  - "Derive record and topology ceilings from versioned mempool memory accounting rather than independent snapshot constants."
  - "Accept checker evidence only from exact statements at direct function-body brace depth zero."
patterns-established:
  - "Durable allocation ceilings derive from the same conservative lower-bound facts that govern live accounted memory."
  - "Source-order checks reject nested dead-code decoys and malformed brace structure before comparing statement indices."
requirements-completed: [MPDUR-01, MPDUR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-03T22:35:30Z
duration: 19m
completed: 2026-08-03
---

# Phase 135 Plan 09: Snapshot Recovery Representability Repairs Summary

Current-v2 explicit-null age preservation, policy-accounting-derived durability bounds, and depth-aware preflight mutation guards close the final audited snapshot representability gaps.

## Performance

- **Duration:** 19m
- **Started:** 2026-08-03T22:16:42Z
- **Completed:** 2026-08-03T22:35:30Z
- **Tasks:** 3
- **Files modified:** 19

## Accomplishments

- Preserved `LegacyUnknown` acceptance time through exact v1 recovery, authoritative install, later known-age mutation, current-v2 periodic Sync checkpoint completion, Fjall reopen, and bounded decode without fabricating freshness.
- Removed the fixed 50,000-record and 1,600,000-edge ceilings, replacing them with versioned, conservative entry/input bounds derived from `PolicyConfig::mempool_capacity` and applied before capture cloning, decoder allocation, and recovery graph construction.
- Proved a 50,001-record current snapshot survives real codec encoding, Fjall Sync persistence, close/reopen, and policy-bounded loading with deterministic first/last identities and explicit corpus size ceilings.
- Hardened the Phase 135 checker so only exact direct preflight and Serde statements count; closure, local-function, async, nested-block, comment, string, raw-string, and malformed-brace decoys cannot satisfy the invariant.

## Task Commits

Each task was committed atomically:

1. **Task 1: Preserve unknown age across current checkpoints** - `6f496819` (`fix`)
2. **Task 2: Derive durable bounds from live policy accounting** - `e7695ebb` (`fix`)
3. **Task 3: Prove direct preflight execution** - `3bf379e7` (`test`)

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/resource.rs`, `lib.rs`, and `pool/tests/resource_cases.rs` - Export versioned capacity bounds and verify accounting lower bounds plus exact-capacity live eviction.
- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` and tests - Accept and preserve explicit unknown age while removing the fixed record constructor ceiling.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs`, `mempool/decode.rs`, and codec tests - Encode unknown age as null, require the key on decode, and derive record limits from policy accounting.
- `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` and snapshot persistence tests - Apply policy-derived load limits and prove legacy-unknown plus 50,001-record Sync/reopen behavior.
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs`, `network/recovery/staging.rs`, and `network/recovery/topology.rs` - Check membership before cloning and share accounting-derived vertex/input-edge limits through recovery.
- `scripts/check-phase135-snapshot-recovery/source.ts`, checker, and mutation tests - Detect only balanced direct statements and guard both runtime repairs.
- `docs/parity/source-breadcrumbs.json` and `docs/metrics/lines-of-code.md` - Register the new parity test source and refresh repository-generated metrics.

## Decisions Made

- The current-v2 field name and two-field DTO remain unchanged; only the acceptance-time value becomes nullable, with absent and duplicate fields still rejected by the streaming decoder.
- Capacity bounds are conservative allocation ceilings, not a new live admission count rule. Full accounted-memory trimming remains authoritative.
- The checker intentionally remains dependency-free and narrow: it masks non-code, validates balanced braces, requires line-leading exact statements at depth zero, and compares their original offsets.

## Verification Evidence

- Task 1 focused suite: 3 legacy-unknown tests passed, including the end-to-end recovery, periodic Sync checkpoint, and reopen regression.
- Task 2 accounting suite: 16 resource tests passed; the separate 50,001-record Sync/reopen test passed with all records present.
- Task 3 checker suite: 67 tests passed with 116 assertions; deletion, direct reorder, closure decoy, and local-function decoy each produce the exact bounds diagnostic. The live checker reported all Phase 135 snapshot recovery invariants verified.
- Parity breadcrumbs: 762 Rust files verified.
- Bright Builds: 1,042 source files scanned with 0 file-length findings; lesson checks also reported 0 findings.
- Generated LOC report freshness and `git diff --check` both passed across the 19-file implementation range.
- Per plan, the final default `bash scripts/verify.sh` was not run by this executor; it remains the independent post-summary verifier transaction.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Preserved the existing recovery topology test facade**

- **Found during:** Task 2 focused node test compilation
- **Issue:** Removing the production fixed-limit constructor left existing topology unit tests without their `standard()` fixture helper.
- **Fix:** Retained a test-only `standard()` facade that delegates to `RecoveryTopologyLimits::from_policy(&PolicyConfig::default())`, so tests exercise the new policy-derived path without restoring a production constant.
- **Files modified:** `packages/open-bitcoin-node/src/network/recovery/topology.rs`
- **Commit:** `e7695ebb`

## Issues Encountered

- The first Task 3 RED attempt contained an incomplete test closure and left one CPU-bound Bun child plus an accidental duplicate timing wrapper. Only those owned processes were terminated; after correcting the test syntax, the focused RED failed immediately on the intentionally missing helper export, and subsequent GREEN runs completed normally.
- An initial formatting invocation omitted the workspace manifest path and failed before changing files. The corrected `cargo fmt --manifest-path packages/Cargo.toml --all` invocation succeeded.

## Known Stubs

None. The empty arrays in the checker are intentional mutable accumulators for temporary roots and diagnostics, not runtime or UI stubs.

## Security and Threat Review

- Explicit null preserves conservative historical uncertainty without accepting missing fields or known-future timestamps.
- Capture, decode, and topology allocation now share a finite bound derived from live policy accounting, while existing byte, transaction, identity, and checked-arithmetic gates remain intact.
- The source checker now rejects dead nested-code preflight decoys. No new endpoint, authentication path, schema version, or unplanned trust boundary was introduced.

## User Setup Required

None - no external service configuration or credentials are required.

## Next Phase Readiness

- Plan 135-09 implementation is complete and ready for the independent lifecycle-valid Phase 135 verifier transaction.
- Phase status, requirement promotion, roadmap state, and `135-VERIFICATION.md` remain owned by the parent orchestrator and verifier.

## Self-Check: PASSED

All 19 implementation, test, checker, and generated documentation artifacts exist, this summary exists, and task commits `6f496819`, `e7695ebb`, and `3bf379e7` resolve in repository history. No goal-blocking stub or unplanned threat surface remains.
