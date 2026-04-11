---
phase: 02-core-domain-and-serialization-foundations
plan: 01
subsystem: core
tags: [rust, domain, newtypes, workspace]
requires:
  - phase: 01-03
    provides: repo-native verification and pure-core policy enforcement
provides:
  - pure-core crate split for reusable primitives and codecs
  - invariant-bearing Bitcoin domain types
  - core crate re-exports for downstream consumers
affects: [consensus, chainstate, p2p, wallet, rpc, cli]
tech-stack:
  added: [open-bitcoin-primitives, open-bitcoin-codec]
  patterns: [pure-core crate split, invariant-bearing domain types]
key-files:
  created:
    - packages/open-bitcoin-primitives/Cargo.toml
    - packages/open-bitcoin-primitives/src/lib.rs
    - packages/open-bitcoin-codec/Cargo.toml
    - packages/open-bitcoin-codec/src/lib.rs
  modified:
    - packages/Cargo.toml
    - packages/open-bitcoin-core/src/lib.rs
    - scripts/pure-core-crates.txt
key-decisions:
  - "Split the pure-core surface into explicit primitives and codec crates instead of growing `open-bitcoin-core` into a monolith."
  - "Represent amounts, hashes, scripts, transactions, blocks, and message framing as checked domain types before later phases add behavior."
patterns-established:
  - "Pure-core crates are added to Cargo, Bazel, and the allowlist together."
  - "Downstream runtime crates consume domain libraries through `open-bitcoin-core` re-exports."
requirements-completed: [ARCH-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 2-2026-04-11T14-58-23
generated_at: 2026-04-11T15:25:48.127Z
duration: 7 min
completed: 2026-04-11
---

# Phase 2 Plan 01: Core Domain and Serialization Foundations Summary

**Introduced first-party pure-core primitives and codec crates, then seeded the typed Bitcoin domain model those later phases will reuse.**

## Performance

- **Duration:** 7 min
- **Started:** 2026-04-11T10:06:00-05:00
- **Completed:** 2026-04-11T10:13:28-05:00
- **Tasks:** 2
- **Files modified:** 19

## Accomplishments
- Added `open-bitcoin-primitives` and `open-bitcoin-codec` to the first-party workspace and Bazel surface.
- Implemented invariant-bearing amount, hash, script, transaction, block, and network message domain types.
- Re-exported the new pure-core surface through `open-bitcoin-core` for downstream consumption.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add the Phase 2 pure-core crate topology to Cargo and Bazel** - `5d31185` (feat)
2. **Task 2: Implement invariant-bearing primitives and shared domain structs** - `5d31185` (feat)

**Plan metadata:** pending in docs commit after state and roadmap updates

## Files Created/Modified
- `packages/open-bitcoin-primitives/src/amount.rs` - checked money-range amount type
- `packages/open-bitcoin-primitives/src/hash.rs` - fixed-width hash wrappers for tx/block identities
- `packages/open-bitcoin-primitives/src/script.rs` - byte-faithful script and witness containers
- `packages/open-bitcoin-primitives/src/transaction.rs` - reusable transaction/outpoint structures
- `packages/open-bitcoin-primitives/src/block.rs` - block and block-header structures
- `packages/open-bitcoin-primitives/src/network.rs` - message header, inventory, locator, and address domain types
- `packages/Cargo.toml` and `BUILD.bazel` - workspace and repo-root crate registration

## Decisions Made
- Used dedicated primitives and codec crates instead of pushing both responsibilities into one crate.
- Preserved wire-visible distinctions like txid vs wtxid and message command padding in the type surface.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- The repo now has reusable pure-core types for the shared Bitcoin data model.
- The codec plan can build directly on these domain types without re-litigating invariants.

---
*Phase: 02-core-domain-and-serialization-foundations*
*Completed: 2026-04-11*
