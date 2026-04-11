---
phase: 01-workspace-baseline-and-guardrails
plan: 02
subsystem: infra
tags: [bazel, bzlmod, rules_rust]
requires:
  - phase: 01-01
    provides: first-party Cargo workspace and crate layout under packages/
provides:
  - root Bazelisk/Bzlmod bootstrap files
  - rules_rust toolchain registration
  - repo-root Bazel aliases for the core and node crates
affects: [verification, ci, workspace-bootstrap]
tech-stack:
  added: [Bazel 8.6.0 pin, rules_rust 0.69.0]
  patterns: [Bzlmod-first bootstrap, root Bazel aliases for first-party crates]
key-files:
  created:
    - .bazelversion
    - MODULE.bazel
    - MODULE.bazel.lock
    - .bazelrc
    - BUILD.bazel
    - packages/open-bitcoin-core/BUILD.bazel
    - packages/open-bitcoin-node/BUILD.bazel
  modified:
    - .gitignore
key-decisions:
  - "Pinned Bazel to 8.6.0 and registered rules_rust 0.69.0 via Bzlmod."
  - "Exposed repo-root aliases so contributors can target the first-party crates from //:core and //:node."
patterns-established:
  - "Bzlmod and MODULE.bazel are the source of truth for Bazel dependencies."
  - "First-party Rust crates are visible from the repo root through small alias targets."
requirements-completed: [ARCH-01]
generated_by: gsd-execute-plan
lifecycle_mode: interactive
phase_lifecycle_id: 01-2026-04-11T11-36-20
generated_at: 2026-04-11T12:08:08Z
duration: 1 min
completed: 2026-04-11
---

# Phase 1 Plan 02: Workspace, Baseline, and Guardrails Summary

**Bootstrapped a Bzlmod-based Bazel surface with `rules_rust` toolchains and repo-root aliases for the core and node crates.**

## Performance

- **Duration:** 1 min
- **Started:** 2026-04-11T12:07:36Z
- **Completed:** 2026-04-11T12:08:08Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- Added the root Bazel files needed for a pinned Bazelisk/Bzlmod workflow
- Registered `rules_rust` and a Rust 1.85.0 / edition 2024 toolchain
- Exposed the first-party crates from the repo root through `//:core` and `//:node`

## Task Commits

Each task was committed atomically:

1. **Task 1: Pin Bazelisk/Bzlmod root configuration** - `44475be` (chore)
2. **Task 2: Expose the first-party crates as Bazel targets** - `05b12c6` (feat)

**Plan metadata:** pending in docs commit after state and roadmap updates

## Files Created/Modified
- `.bazelversion` - pins Bazel `8.6.0`
- `MODULE.bazel` - Bzlmod root with `rules_rust` and the Rust toolchain declaration
- `MODULE.bazel.lock` - records the resolved module graph
- `.bazelrc` - repo-wide Bazel defaults
- `BUILD.bazel` - root aliases for `//:core` and `//:node`
- `packages/open-bitcoin-core/BUILD.bazel` - Bazel library target for the pure-core crate
- `packages/open-bitcoin-node/BUILD.bazel` - Bazel library target for the node crate
- `.gitignore` - ignores Bazel-generated output symlinks

## Decisions Made
- Treated `MODULE.bazel` and `MODULE.bazel.lock` as first-class repo artifacts for reproducible Bzlmod resolution.
- Added root aliases instead of asking contributors to memorize deep package labels for the initial crates.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Ignored Bazel output symlinks as part of the root bootstrap**
- **Found during:** Task 1 (Pin Bazelisk/Bzlmod root configuration)
- **Issue:** The first Bazel run created `bazel-*` output symlinks that would otherwise remain as noisy untracked files in the worktree.
- **Fix:** Added `bazel-*` to `.gitignore` during the root bootstrap commit.
- **Files modified:** `.gitignore`
- **Verification:** `git status --short` no longer reports the Bazel output symlinks after the ignore rule was committed
- **Committed in:** `44475be` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** No scope change. The ignore rule keeps the Bazel bootstrap usable for repeated local runs.

## Issues Encountered
- The first `bazel build //:core //:node` incurred a long one-time `rules_rust` toolchain download. After the cache was warm, the verification build completed quickly.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Bazel can now address the first-party crates from the repo root.
- The phase is ready for the pure-core enforcement and repo-native verification scripts in Plan 03.

---
*Phase: 01-workspace-baseline-and-guardrails*
*Completed: 2026-04-11*
