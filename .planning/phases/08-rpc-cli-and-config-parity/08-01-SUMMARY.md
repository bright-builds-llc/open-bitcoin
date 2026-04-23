---
phase: 08-rpc-cli-and-config-parity
plan: 01
subsystem: infra
tags: [rpc, cli, cargo, bazel, verification]
requires: []
provides:
  - "First-class `open-bitcoin-rpc` Cargo and Bazel package shell"
  - "First-class `open-bitcoin-cli` Cargo and Bazel package shell"
  - "Repo-native verification coverage for `//:rpc` and `//:cli`"
affects: [08-02, 08-03, rpc, cli]
tech-stack:
  added: [axum, base64, tokio, serde, serde_json, clap, ureq]
  patterns:
    - "Library-first shell crates with thin `lib.rs` readiness markers"
    - "Root Bazel alias smoke verification for new first-party packages"
key-files:
  created:
    - packages/open-bitcoin-rpc/Cargo.toml
    - packages/open-bitcoin-rpc/BUILD.bazel
    - packages/open-bitcoin-rpc/src/lib.rs
    - packages/open-bitcoin-cli/Cargo.toml
    - packages/open-bitcoin-cli/BUILD.bazel
    - packages/open-bitcoin-cli/src/lib.rs
  modified:
    - packages/Cargo.toml
    - packages/Cargo.lock
    - BUILD.bazel
    - scripts/verify.sh
key-decisions:
  - "Keep both new Phase 8 crates as `rust_library` shells with `crate_ready` markers until later plans add typed contracts, transport, and CLI behavior."
  - "Declare only shell-layer dependencies needed for later Phase 8 work and keep `/wallet/<name>`, `rpcauth`, and runtime behavior out of this scaffold plan."
  - "Fail fast through root Bazel aliases in `scripts/verify.sh` so missing RPC or CLI target ownership cannot hide until later interface plans."
patterns-established:
  - "New operator-facing surfaces start as first-class Cargo workspace members plus root Bazel aliases before behavior lands."
  - "Phase scaffolds document intentional placeholder exports in the summary so later plans replace them deliberately."
requirements-completed: [RPC-01, CLI-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 08-2026-04-23T01-44-19
generated_at: 2026-04-23T03:42:58Z
duration: 5m 29s
completed: 2026-04-23
---

# Phase 08 Plan 01: Shell Package and Verification Foundation Summary

**First-class `open-bitcoin-rpc` and `open-bitcoin-cli` shell crates with Cargo/Bazel ownership and repo-native verification coverage**

## Performance

- **Duration:** 5m 29s
- **Started:** 2026-04-23T03:37:49Z
- **Completed:** 2026-04-23T03:42:58Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Added `open-bitcoin-rpc` and `open-bitcoin-cli` as first-class Cargo workspace members and root Bazel aliases.
- Created thin shell manifests, Bazel targets, and `lib.rs` crate roots without leaking typed contracts, transport, or CLI behavior into this plan.
- Extended `bash scripts/verify.sh` so `//:rpc` and `//:cli` fail early in the repo-native smoke path.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add first-class RPC and CLI packages to Cargo and Bazel** - `ec96f24` (feat)
2. **Task 2: Wire repo-native verification to the new shell targets** - `22b2ed2` (chore)

## Files Created/Modified

- `packages/Cargo.toml` - Adds the RPC and CLI shell crates as workspace members.
- `packages/Cargo.lock` - Locks the shell-layer dependency graph for the new crates.
- `BUILD.bazel` - Exposes root `rpc` and `cli` aliases for the new shell packages.
- `packages/open-bitcoin-rpc/{Cargo.toml,BUILD.bazel,src/lib.rs}` - Defines the RPC shell package, Bazel target, and readiness marker export.
- `packages/open-bitcoin-cli/{Cargo.toml,BUILD.bazel,src/lib.rs}` - Defines the CLI shell package, Bazel target, and readiness marker export.
- `scripts/verify.sh` - Extends the repo-native Bazel smoke build to include `//:rpc` and `//:cli`.

## Decisions Made

- Kept both new crates as `rust_library` shells rather than introducing binaries or runtime behavior in this scaffolding plan.
- Limited the dependency surface to shell-layer libraries identified in Phase 8 research and deferred wallet-path routing, multi-user auth, and transport behavior.
- Put the new root aliases directly into `scripts/verify.sh` so missing build ownership breaks immediately during Phase 8 instead of surfacing in later plans.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- Bazel regenerated `MODULE.bazel.lock` during verification because the new crate manifests changed the crate-universe graph. That file was kept out of scope and out of commits to honor the requested Phase 08 Plan 01 file boundary.

## Known Stubs

- `packages/open-bitcoin-rpc/src/lib.rs:5` - `crate_ready()` is an intentional placeholder export for the shell scaffold; later Phase 8 plans will replace it with typed RPC contracts and transport wiring.
- `packages/open-bitcoin-cli/src/lib.rs:5` - `crate_ready()` is an intentional placeholder export for the shell scaffold; later Phase 8 plans will replace it with CLI argument parsing and client behavior.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Later Phase 8 plans can add typed RPC contracts, server transport, and CLI behavior without reopening workspace ownership or repo-native verification wiring.

No code blockers were introduced by this plan. The only incidental follow-up is the out-of-scope `MODULE.bazel.lock` drift produced by Bazel during verification.

## Self-Check: PASSED

- `FOUND: .planning/phases/08-rpc-cli-and-config-parity/08-01-SUMMARY.md`
- `FOUND: packages/open-bitcoin-rpc/src/lib.rs`
- `FOUND: packages/open-bitcoin-cli/src/lib.rs`
- `FOUND: ec96f24`
- `FOUND: 22b2ed2`
