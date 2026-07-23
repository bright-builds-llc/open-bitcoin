---
phase: 130-resource-time-and-fee-primitives
plan: "01"
subsystem: mempool
tags: [rust, mempool, resource-accounting, parity, typed-invariants]
requires:
  - phase: 129-integration-guardrails-and-milestone-reconciliation
    provides: Verified v2.1 integration and release-boundary baseline
provides:
  - Compile-time-distinct transaction vsize, accounted memory, and capacity values
  - Versioned deterministic Rust-owned memory accounting formula
  - Cached resource ledger with an independent canonical-state oracle
  - Explicit legacy vsize trimming seam without accounted-capacity enforcement
affects: [phase-131, mempool-policy, rpc-evidence, node-operators]
tech-stack:
  added: []
  patterns:
    - Checked typed resource arithmetic
    - Cached aggregate verified by independent recomputation
key-files:
  created:
    - packages/open-bitcoin-mempool/src/resource.rs
    - packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/types.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/types.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
key-decisions:
  - "Account only deterministic Rust-owned logical mempool state; exclude allocator slack, hash buckets, C++ pointer estimates, and node/network caches."
  - "Keep Phase 130 trimming exclusively on legacy_vsize_trim_limit while exposing distinct accounted usage and capacity."
  - "Map resource arithmetic failures to MempoolError::InternalInvariant at the admission and lifecycle boundary."
patterns-established:
  - "Resource roles are newtypes with explicit constructors/accessors and no cross-type arithmetic."
  - "Committed state caches MempoolResourceLedger; tests compare it with recompute_resource_ledger after every mutation class."
requirements-completed: []
requirements-addressed: [FEEP-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-23T18:26:55Z
duration: 50 min
completed: 2026-07-23
---

# Phase 130 Plan 01: Resource Accounting Primitives Summary

**Typed vsize, accounted-memory, and capacity contracts backed by checked version-1 accounting, a cached ledger, and an independent full-state oracle**

## Performance

- **Duration:** 50 min
- **Started:** 2026-07-23T17:36:16Z
- **Completed:** 2026-07-23T18:26:55Z
- **Tasks:** 2
- **Files modified:** 19

## Accomplishments

- Added non-interchangeable `TransactionVirtualSize`, `AccountedMempoolMemory`, and `MempoolCapacity` values with checked arithmetic and typed overflow errors.
- Defined version-1 logical accounting for entries, transaction elements, scripts, witness data, graph identities, and spent outpoints.
- Kept cached resource totals equal to an independently traversed oracle after admission, parent/child addition, replacement, legacy trim, and block removal.
- Migrated benchmark, node, recovery, snapshot, and RPC callers while preserving vsize-only trimming until Phase 131.

## Task Commits

1. **Task 1: Define resource values and the versioned accounting formula** - `2aa8c576`
2. **Task 2: Integrate the cached ledger and independent oracle** - `a5dd14a7`

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/resource.rs` - Resource newtypes, accounting formula, checked ledger builder, and independent oracle.
- `packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs` - Formula, overflow, cache/oracle, transition, and trim-boundary regressions.
- `packages/open-bitcoin-mempool/src/types.rs` - Typed entry vsize and distinct capacity/legacy-trim configuration.
- `packages/open-bitcoin-mempool/src/pool.rs` - Canonical cached ledger ownership and checked transition rebuilds.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Ledger-aware block removal and accounted-capacity status.
- `packages/open-bitcoin-node/src/network.rs` - Separate vsize, accounted usage, and capacity projection.
- `packages/open-bitcoin-node/src/network/types.rs` - Unambiguous managed mempool evidence fields.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - Knots-compatible `bytes`, `usage`, and `maxmempool` meanings.
- `docs/parity/source-breadcrumbs.json` - Pinned source/test anchors for both new Rust files.
- `docs/metrics/lines-of-code.md` - Hook-regenerated tracked LOC freshness.

## Decisions Made

- The accounting formula is deterministic and Rust-owned rather than a byte-for-byte C++ allocator estimate.
- Accounted capacity is reported and classified, but it does not reject or evict transactions in Phase 130.
- The oracle does not call cached-ledger mutation methods, preserving its value as a drift detector.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Migrated omitted public callers required by the typed API transition**
- **Found during:** Task 2
- **Issue:** The plan's file list omitted lifecycle implementation/tests and snapshot projection files that directly consumed removed raw fields and could not compile after the required public API migration.
- **Fix:** Migrated `pool/lifecycle.rs`, lifecycle/outcome tests, and `storage/mempool_snapshot.rs` to the typed ledger contract.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/lifecycle.rs`, `packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs`, `packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs`, `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs`
- **Verification:** Timed workspace `cargo check --workspace --all-targets` passed.
- **Committed in:** `a5dd14a7`

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Required caller migration only; no Phase 131 enforcement or pressure-policy scope was added.

## Issues Encountered

- The first Task 1 commit attempt was blocked by the repository's zero-uncovered-line hook. Focused resource tests were expanded, stale coverage data was cleaned, and the normal verified commit then passed.
- The metadata hook correctly rejected early FEEP-01 completion because Phase 130 has no lifecycle-valid `VERIFICATION.md` yet. The requirement remains addressed but pending until phase verification.

## Authentication Gates

None.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 130-02 can build fee-role newtypes on the established typed-resource pattern.
- Phase 131 has an explicit `legacy_vsize_trim_limit` removal seam for switching enforcement to accounted memory.
- No blockers remain.

## Self-Check: PASSED

- Created files and summary exist.
- Task commits `2aa8c576` and `a5dd14a7` exist.
- Typed ledger and legacy trim seam claims match the committed source.

---
*Phase: 130-resource-time-and-fee-primitives*
*Completed: 2026-07-23*
