---
phase: 01-workspace-baseline-and-guardrails
plan: 01
subsystem: infra
tags: [git-submodule, cargo-workspace, rust]
requires: []
provides:
  - pinned Bitcoin Knots baseline under packages/bitcoin-knots
  - first-party Cargo workspace under packages/
  - pure-core and node crate split
affects: [bazel, verification, contributor-workflow]
tech-stack:
  added: [git submodule, cargo workspace]
  patterns: [read-only reference baseline, pure-core-vs-shell crate split]
key-files:
  created:
    - .gitmodules
    - packages/Cargo.toml
    - packages/README.md
    - packages/open-bitcoin-core/Cargo.toml
    - packages/open-bitcoin-core/src/lib.rs
    - packages/open-bitcoin-node/Cargo.toml
    - packages/open-bitcoin-node/src/lib.rs
  modified: []
key-decisions:
  - "Pinned Knots as a tracked submodule at packages/bitcoin-knots instead of a copied source snapshot."
  - "Started the first-party Rust workspace with a pure-core crate and a shell/runtime crate."
patterns-established:
  - "Reference baseline stays read-only and separate from first-party implementation code."
  - "Shell/runtime crates may depend on pure-core crates, but not vice versa."
requirements-completed: [REF-01]
generated_by: gsd-execute-plan
lifecycle_mode: interactive
phase_lifecycle_id: 01-2026-04-11T11-36-20
generated_at: 2026-04-11T11:55:40Z
duration: 1 min
completed: 2026-04-11
---

# Phase 1 Plan 01: Workspace, Baseline, and Guardrails Summary

**Pinned the Knots baseline as a Git submodule and created the first pure-core/runtime Cargo workspace split under `packages/`.**

## Performance

- **Duration:** 1 min
- **Started:** 2026-04-11T11:55:17Z
- **Completed:** 2026-04-11T11:55:40Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- Added Bitcoin Knots as a pinned submodule at `packages/bitcoin-knots`
- Created the first-party Cargo workspace under `packages/`
- Split the initial first-party Rust code into `open-bitcoin-core` and `open-bitcoin-node`

## Task Commits

Each task was committed atomically:

1. **Task 1: Vendor the Knots baseline as a pinned submodule** - `e03bac6` (chore)
2. **Task 2: Scaffold the first-party Cargo workspace and initial crate split** - `9438de5` (feat)

**Plan metadata:** pending in docs commit after state and roadmap updates

## Files Created/Modified
- `.gitmodules` - registers the pinned Knots baseline as a tracked submodule
- `packages/Cargo.toml` - first-party Rust workspace manifest
- `packages/README.md` - documents the vendor-vs-first-party package layout
- `packages/open-bitcoin-core/Cargo.toml` - pure-core crate manifest
- `packages/open-bitcoin-core/src/lib.rs` - initial pure-core crate root
- `packages/open-bitcoin-node/Cargo.toml` - shell/runtime crate manifest
- `packages/open-bitcoin-node/src/lib.rs` - initial node crate root

## Decisions Made
- Used a tracked git submodule for the Knots baseline instead of a copied snapshot.
- Started the Rust workspace with the pure-core vs shell split required by the project architecture.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Switched from a full-history submodule clone to a shallow pinned clone before registration**
- **Found during:** Task 1 (Vendor the Knots baseline as a pinned submodule)
- **Issue:** A full `git submodule add` spent too long unpacking the entire Knots history and risked stalling Phase 1 execution.
- **Fix:** Replaced it with a shallow clone pinned to `v29.3.knots20260210`, then registered the existing repo as the tracked submodule and absorbed the git dir.
- **Files modified:** `.gitmodules`, `packages/bitcoin-knots`
- **Verification:** `git submodule status packages/bitcoin-knots` and `git -C packages/bitcoin-knots describe --tags --exact-match`
- **Committed in:** `e03bac6` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** No scope change. The shallow pinned clone reduced fetch cost while preserving the required submodule outcome.

## Issues Encountered
- The initial full-history submodule fetch was too expensive for the large Knots repository, so the vendoring flow was narrowed to the exact pinned baseline tag.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- The repo now has the baseline and first-party Rust package layout that Bazel bootstrap can target next.
- Wave 2 can build directly on the new workspace manifests and package paths.

---
*Phase: 01-workspace-baseline-and-guardrails*
*Completed: 2026-04-11*
