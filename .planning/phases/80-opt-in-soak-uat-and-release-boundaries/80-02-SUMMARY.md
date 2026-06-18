---
phase: 80-opt-in-soak-uat-and-release-boundaries
plan: 02
subsystem: parity-release-boundaries
tags: [parity, release-readiness, v1.7, verification]

requires:
  - phase: 75-multi-day-soak-runner-and-evidence-ledger
    provides: opt-in soak runner evidence roots
  - phase: 76-disk-and-resource-bound-enforcement
    provides: resource-bound evidence roots
  - phase: 77-corruption-and-lock-recovery-hardening
    provides: recovery evidence roots
  - phase: 78-progress-guarantees-and-stall-diagnosis
    provides: progress guarantee evidence roots
  - phase: 79-diagnostics-and-support-bundle-forensics
    provides: support forensics evidence roots
  - phase: 80 plan 01
    provides: runtime-guide UAT and release-boundary framing
provides:
  - v1.7 release-readiness claim boundary matrix
  - machine-readable v1.7 parity closeout root
  - human-readable v1.7 parity checklist row
  - deferred production-adjacent non-claim register updates
  - operator-runtime Phase 80 closeout catalog row
affects: [phase-80, parity, release-readiness, verifier-roots]

tech-stack:
  added: []
  patterns:
    - existing parity roots instead of new evidence manifests
    - source breadcrumbs remain the first-party Rust traceability mechanism

key-files:
  created:
    - .planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-02-SUMMARY.md
  modified:
    - docs/parity/release-readiness.md
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/README.md
    - docs/parity/deviations-and-unknowns.md
    - docs/parity/catalog/operator-runtime-release-hardening.md

key-decisions:
  - "Use the existing parity root shape rather than creating a new evidence manifest or all-doc scanner."
  - "Make v1.7 the current source-built, explicit opt-in full-sync soak and recovery hardening claim while preserving v1.6 and older docs as historical evidence."
  - "Preserve the complete deferred production-adjacent non-claim list in release-readiness, deviations, checklist, README, and operator-runtime docs."

patterns-established:
  - "v1.7 closeout roots should point through index.json, checklist.md, release-readiness.md, source breadcrumbs, deterministic checkers, and operator docs."
  - "Future production-node, relay, wallet-production, packaging, GUI, hosted-dashboard, public-network CI, support-upload, and destructive-repair claims require separate scoped phases."

requirements-completed: [VER-07, REL-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 80-2026-06-17T22-54-57
generated_at: 2026-06-18T03:05:36Z

duration: 53m15s
completed: 2026-06-18
---

# Phase 80 Plan 02: v1.7 Parity Roots Summary

**v1.7 release-boundary parity roots for source-built opt-in full-sync soak and recovery hardening**

## Performance

- **Duration:** 53m15s
- **Started:** 2026-06-18T02:12:21Z
- **Completed:** 2026-06-18T03:05:36Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Added the current v1.7 release-readiness matrix with the complete SOAK, RES, REC, PROG, DIAG, VER, and REL traceability set.
- Added the `v1-7-full-sync-soak-recovery-release-boundaries` machine and human parity roots.
- Kept the v1.7 claim scoped to source-built, explicit opt-in full-sync soak and recovery hardening, with production-adjacent non-claims explicit.
- Preserved source breadcrumbs and deterministic checker references as the audit path instead of adding a new evidence manifest.

## Task Commits

1. **Task 1: Define v1.7 release-readiness claim boundaries** - `f3d7d1d` (docs)
2. **Task 2: Update machine and human parity roots for v1.7 closeout** - `6c59922` (docs)

## Files Created/Modified

- `docs/parity/release-readiness.md` - Added the current v1.7 claim-boundary matrix, all 24 v1.7 requirement IDs, and non-claim boundaries.
- `docs/parity/deviations-and-unknowns.md` - Added the v1.7 deferred production-adjacent scope and suspected unknowns.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - Added the Phase 80 operator-runtime closeout row.
- `docs/parity/index.json` - Added the v1.7 closeout surface and audit metadata labels.
- `docs/parity/checklist.md` - Added the human checklist row for the v1.7 closeout root.
- `docs/parity/README.md` - Pointed the parity entrypoint at Phase 80 as the current v1.7 closeout root.
- `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-02-SUMMARY.md` - Recorded this execution.

## Decisions Made

- Used one new closeout surface in `docs/parity/index.json`, matching the existing parity root shape and avoiding a duplicate registry.
- Did not create `docs/parity/threat-model-v1.7.md`; the plan scoped this work to release-boundary roots, not a new threat model.
- Kept source breadcrumbs as the Rust source/test traceability mechanism instead of adding a new manifest.
- Left `.planning/STATE.md` untouched and unstaged because the orchestrator owns that dirty file.

## Verification

- `rg -n "v1.7 Full-Sync Soak and Recovery Hardening Claim Boundary Matrix|Phase 80 opt-in soak UAT and release boundaries|SOAK-01|RES-05|REC-05|PROG-01|DIAG-01|VER-05|VER-06|VER-07|REL-04" docs/parity/release-readiness.md docs/parity/catalog/operator-runtime-release-hardening.md` - passed.
- `rg -n "inbound serving|address relay|block serving|transaction relay|compact block relay|production-funds wallet|migration apply mode|signed packaging|Windows service support|GUI|hosted dashboards|public-network CI|release-blocking live sync|automatic support-bundle upload|destructive repair|production-node readiness" docs/parity/deviations-and-unknowns.md docs/parity/release-readiness.md` - passed.
- `bun -e 'JSON.parse(await Bun.file("docs/parity/index.json").text()); console.log("json ok")'` - passed.
- `rg -n "v1-7-full-sync-soak-recovery-release-boundaries|v1_7_release_boundaries|phase80_opt_in_soak_uat_release_boundaries|VER-05|VER-06|VER-07|REL-04" docs/parity/index.json docs/parity/checklist.md docs/parity/README.md` - passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed.
- `jq` uniqueness checks for the new top-level surface and checklist surface - both returned `1`.
- `test ! -e docs/parity/threat-model-v1.7.md` - passed.
- `cargo fmt --manifest-path packages/Cargo.toml --all` - passed before each task commit.
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` - passed before each task commit.
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` - passed before each task commit.
- `cargo test --manifest-path packages/Cargo.toml --all-features` - passed before each task commit.
- `bash scripts/verify.sh` - passed end to end in 17m43s, including parity checkers, Rust tests, benchmark smoke, Bazel smoke build, and coverage/test phases.

## Simplification Pass

The parity diff was checked against D-09 through D-13. No new registry, no new manifest, no broad all-doc scanner, and no v1.7 threat-model file were added. The only new machine-readable closeout root is `v1-7-full-sync-soak-recovery-release-boundaries`.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

A supplemental evidence-path existence probe found that the plan-required `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` and `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md` paths are not present in this working tree yet. Because Task 2 explicitly required those references and both files are outside the owned file set, no scope-expanding file creation was done.

## Known Stubs

None. Stub scan matched only existing prose that says shipped service/status/dashboard docs are not placeholders; it did not identify a functional stub in the files changed by this plan.

## Threat Flags

None. This plan changed release/parity documentation only and introduced no new endpoint, authentication path, file-access adapter, schema boundary, or runtime trust boundary.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

The v1.7 closeout roots now point reviewers through the release-readiness matrix, machine root, human checklist, parity README, deviations register, operator-runtime catalog, source breadcrumbs, deterministic checkers, and `scripts/verify.sh`. Later work can add Phase 80-specific verification artifacts or production-adjacent claims only through separate scoped plans.

## Self-Check: PASSED

- Found summary file: `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-02-SUMMARY.md`
- Found task commits: `f3d7d1d`, `6c59922`
- Confirmed `.planning/STATE.md` remains unstaged and orchestrator-owned.

---
*Phase: 80-opt-in-soak-uat-and-release-boundaries*
*Completed: 2026-06-18*
