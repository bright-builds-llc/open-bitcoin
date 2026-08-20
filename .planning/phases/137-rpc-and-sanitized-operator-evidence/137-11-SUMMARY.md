---
phase: 137-rpc-and-sanitized-operator-evidence
plan: 11
subsystem: docs
tags: [parity-catalog, breadcrumbs, package-rpc, operator-uat, claim-bounds]

# Dependency graph
requires:
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Plans 01-10 package projector, local admission, snapshot groups, and dual-state surfaces
provides:
  - Breadcrumb registry groups for Phase 137 projector, local package, and operator package sources
  - RPC catalog rows for testmempoolaccept/submitpackage versus openbitcoinpackage
  - Cargo and Bazel UAT command forms for package dry-run, submit, and status
  - Operator snapshot, observability, runtime-guide, and README freshness without v2.2 claim inflation
affects:
  - Phase 138 MPVFY claim-guardrail closeout
  - Contributor UAT using repo-local Cargo and Bazel forms

# Tech tracking
tech-stack:
  added: []
  patterns:
    - BaselineParity Knots names stay on open-bitcoin-cli; typed extras stay on openbitcoinpackage / open-bitcoin package
    - UAT examples use placeholder <hex> and both Cargo and Bazel labels
    - Shared status groups stay identifier-free; originating package responses own txids and fingerprints

key-files:
  created: []
  modified:
    - docs/parity/source-breadcrumbs.json
    - docs/parity/catalog/rpc-cli-config.md
    - docs/parity/index.json
    - docs/parity/service-operation-expectations.md
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
    - docs/operator/runtime-guide.md
    - README.md
    - packages/open-bitcoin-rpc/src/package_projection.rs
    - packages/open-bitcoin-rpc/src/package_projection/report_view.rs
    - packages/open-bitcoin-rpc/src/package_projection/tests.rs
    - packages/open-bitcoin-node/src/network/operator_snapshot.rs

key-decisions:
  - "Register unmapped Phase 137 sources in dedicated breadcrumb groups; leave already-mapped dispatch/method/status/policy files in existing groups to avoid duplicate mappings"
  - "Catalog submitpackage BroadcastTransaction non-throw as an intentional difference; keep getrawmempool out of Phase 137"
  - "Add a v2.2 in_progress pointer only; do not mark MPVFY complete"

patterns-established:
  - "Pattern 1: Package UAT blocks always pair cargo run --manifest-path packages/Cargo.toml with bazel run //packages/..."
  - "Pattern 2: New operator paragraphs name local admission without public/default relay or production-readiness claims"

requirements-completed: [MPOBS-01, MPOBS-02, MPOBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-20T00:04:00Z

# Metrics
duration: 4min
completed: 2026-08-20
---

# Phase 137 Plan 11: RPC Catalog, Breadcrumbs, And Operator Docs Summary

**Parity catalog, breadcrumbs, and operator docs now describe Knots `testmempoolaccept`/`submitpackage` plus `openbitcoinpackage` with copy-pasteable Cargo and Bazel UAT forms and no public/default-relay or production-readiness claims.**

## Performance

- **Duration:** 4 min
- **Started:** 2026-08-19T23:59:46Z
- **Completed:** 2026-08-20T00:04:00Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Registered dedicated Knots-anchored breadcrumb groups for `package_projection.rs`, `local_package.rs`, and `operator/package.rs`, and verified 795 first-party Rust files through `check-parity-breadcrumbs.ts`.
- Catalogued the BaselineParity versus typed-extension split, the intentional `submitpackage` non-throw after accept, and that `getrawmempool` is out of Phase 137.
- Added Phase 137 Cargo and Bazel UAT forms for `open-bitcoind -server=1`, baseline package RPC, `open-bitcoin package dry-run|submit`, and `status --format json`.
- Documented identifier-free snapshot groups (`resources`, `fee_floors`, `pressure`, `eviction`, `checkpoint`, `recovery`, `retry`, `admission`), the `mempool_policy` metric/log source, and README freshness without Phase 138 closeout.

## Task Commits

Each task was committed atomically:

1. **Task 1: Register breadcrumbs and catalog the package RPC split** - `a5c987b5` (docs)
2. **Task 2: Refresh operator docs and contributor README wording** - `b742fecf` (docs)

**Plan metadata:** this commit (docs: complete plan)

## Files Created/Modified

- `docs/parity/source-breadcrumbs.json` - Added `rpc-package-projection`, `node-local-package-admission`, and `cli-operator-package` groups
- `docs/parity/catalog/rpc-cli-config.md` - Package RPC/CLI catalog rows and intentional-difference wording
- `docs/parity/index.json` - Phase 137 `in_progress` pointer; MPVFY left for Phase 138
- `docs/parity/service-operation-expectations.md` - Cargo and Bazel package UAT command block
- `docs/architecture/status-snapshot.md` - Mempool group ownership including `MempoolFeeFloorsGroup`
- `docs/architecture/operator-observability.md` - `MetricKind` / `mempool_policy` log source and support next-action sentence
- `docs/operator/runtime-guide.md` - Package dry-run/submit and baseline CLI forms
- `README.md` - Package RPC status line; removed stale "Phase 137 surfaces remain deferred" wording
- `packages/open-bitcoin-rpc/src/package_projection.rs` and children - Added `protocol.h` breadcrumb to match the new group
- `packages/open-bitcoin-node/src/network/operator_snapshot.rs` - Synced stale breadcrumb block to the existing adapter group

## Decisions Made

- Dedicated breadcrumb groups only for unmapped Phase 137 sources. `dispatch/package.rs` and `method/package.rs` stay under `rpc-surface` patterns, which already include `mempool.cpp` and `protocol.h`. Status/policy renderer and metrics/log modules stay in existing `none` groups.
- `docs/parity/index.json` received a `v2-2-rpc-and-sanitized-operator-evidence` `in_progress` pointer because v2.2 surfaces already list RPC catalog evidence. MPVFY is not marked complete.
- UAT examples use placeholder `<hex>` only.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Left already-mapped Phase 137 sources in existing groups**
- **Found during:** Task 1 (Register breadcrumbs and catalog the package RPC split)
- **Issue:** Adding dedicated groups for `dispatch/package.rs`, `method/package.rs`, `status/mempool_groups.rs`, and `mempool_policy` renderers/metrics/logs would duplicate mappings already owned by `rpc-surface` patterns or existing `none` groups.
- **Fix:** Added groups only for the six unmapped files the checker reported. Existing Knots/`none` registrations already cover the rest.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun scripts/check-parity-breadcrumbs.ts` passed for 795 files
- **Committed in:** `a5c987b5` (Task 1 commit)

**2. [Rule 3 - Blocking] Synced stale `operator_snapshot.rs` breadcrumbs**
- **Found during:** Task 1 verification
- **Issue:** `operator_snapshot.rs` was already listed under `node-network-adapter` but its comment block was stale, so the checker failed.
- **Fix:** Replaced the comment block with the group's Knots targets.
- **Files modified:** `packages/open-bitcoin-node/src/network/operator_snapshot.rs`
- **Verification:** Breadcrumb checker passed
- **Committed in:** `a5c987b5` (Task 1 commit)

**3. [Rule 2 - Missing Critical] Added `protocol.h` to projector breadcrumbs**
- **Found during:** Task 1
- **Issue:** Source comments listed only `mempool.cpp`; the plan required `mempool.cpp` plus `protocol.h` for the projector family.
- **Fix:** Added `protocol.h` to the new group and the three projector source comments.
- **Files modified:** `docs/parity/source-breadcrumbs.json`, `packages/open-bitcoin-rpc/src/package_projection.rs`, `packages/open-bitcoin-rpc/src/package_projection/report_view.rs`, `packages/open-bitcoin-rpc/src/package_projection/tests.rs`
- **Verification:** Breadcrumb checker passed
- **Committed in:** `a5c987b5` (Task 1 commit)

---

**Total deviations:** 3 auto-fixed (2 blocking, 1 missing critical)
**Impact on plan:** Required for breadcrumb checker correctness. No claim inflation or scope creep.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 137 documentation and UAT forms are registered. Phase 138 can own MPVFY claim-guardrail mutation tests, adversarial pressure, and production-readiness closeout.
- Do not treat this plan as public/default relay, network-wide delivery, or production-readiness proof.

---
*Phase: 137-rpc-and-sanitized-operator-evidence*
*Completed: 2026-08-20*

## Self-Check: PASSED
