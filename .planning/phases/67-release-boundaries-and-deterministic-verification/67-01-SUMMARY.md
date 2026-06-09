---
phase: 67-release-boundaries-and-deterministic-verification
plan: 01
subsystem: parity-release-verification
tags: [release-boundaries, parity-docs, deterministic-verification, bun, v1.5]

# Dependency graph
requires:
  - phase: 60-unattended-sync-loop-control
    provides: bounded unattended sync loop control evidence
  - phase: 61-resource-bounds-and-recovery-taxonomy
    provides: resource pressure and recovery category evidence
  - phase: 62-long-run-sync-truth-surfaces
    provides: shared long-run sync truth surfaces
  - phase: 63-service-supervision-lifecycle
    provides: launchd/systemd user service lifecycle evidence
  - phase: 64-service-restart-and-same-datadir-resume-evidence
    provides: service restart/resume evidence
  - phase: 65-support-bundle-and-operator-review-docs
    provides: redacted support evidence and operator review docs
  - phase: 66-compatibility-harness-operator-wrapper
    provides: operator compatibility wrapper evidence
provides:
  - v1.5 threat model and release-boundary matrix
  - machine-readable and human-readable v1.5 parity roots
  - deterministic v1.5 release-boundary checker wired into verify.sh
  - Phase 67 verification artifact for REL-01 through REL-04
affects: [parity-docs, operator-runtime-guide, verification, v1.5-closeout]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Bun parity-root checker for deterministic release-boundary docs
    - Evidence-first release readiness matrix with explicit non-claims

key-files:
  created:
    - docs/parity/threat-model-v1.5.md
    - scripts/check-v1.5-release-boundaries.ts
    - .planning/phases/67-release-boundaries-and-deterministic-verification/67-VERIFICATION.md
  modified:
    - docs/parity/release-readiness.md
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/README.md
    - docs/parity/catalog/p2p.md
    - docs/parity/deviations-and-unknowns.md
    - docs/operator/runtime-guide.md
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md

key-decisions:
  - "v1.5 is documented as source-built, explicit opt-in unattended mainnet operator-review readiness, not production-node readiness."
  - "Public-network live smoke, manual peers, restart-after-progress, and real service-manager commands remain opt-in UAT outside default verification."
  - "The deterministic checker guards v1.5 parity roots and default-verification exclusions."

patterns-established:
  - "Release-boundary closeouts add a versioned threat model, release-readiness matrix, parity roots, and a Bun checker wired into verify.sh."
  - "Historical v1.3/v1.4 evidence stays linked but is not promoted into the current v1.5 claim."

requirements-completed: [REL-01, REL-02, REL-03, REL-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 67-2026-06-09T00-30-52
generated_at: 2026-06-09T01:28:27.000Z

# Metrics
duration: 44min
completed: 2026-06-09
---

# Phase 67: Release Boundaries and Deterministic Verification Summary

**v1.5 unattended-operation release boundaries are auditable through versioned parity docs and a deterministic Bun checker in `verify.sh`.**

## Performance

- **Duration:** 44 min
- **Started:** 2026-06-09T00:44:07.096Z
- **Completed:** 2026-06-09T01:28:27.000Z
- **Tasks:** 4
- **Files modified:** 13

## Accomplishments

- Added `docs/parity/threat-model-v1.5.md` with STRIDE, ASVS L1, evidence acceptance, release-boundary matrix, requirements traceability, and residual risks for REL-01 through REL-04.
- Updated release readiness, parity roots, checklist, README, P2P catalog, deviations register, and runtime guide so reviewers can distinguish v1.5 operator-review readiness from deferred production-adjacent claims.
- Added `scripts/check-v1.5-release-boundaries.ts` and wired it into `scripts/verify.sh` to guard v1.5 roots and keep public-network/service-manager UAT out of default verification.
- Refreshed `docs/metrics/lines-of-code.md` and recorded Phase 67 verification evidence.

## Task Commits

Task changes are being committed together after the clean final verification pass because this yolo run executed the single plan inline.

## Files Created/Modified

- `docs/parity/threat-model-v1.5.md` - Current v1.5 scoped threat model and release-boundary companion.
- `docs/parity/release-readiness.md` - Current v1.5 readiness verdict and claim boundary matrix.
- `docs/parity/index.json` - Machine-readable v1.5 parity surface and audit roots.
- `docs/parity/checklist.md` - Human-readable v1.5 checklist surface.
- `docs/parity/README.md` - Parity entrypoint pointing current closeout evidence at v1.5.
- `docs/parity/catalog/p2p.md` - v1.5 P2P/operator-review boundary and deferred P2P non-claims.
- `docs/parity/deviations-and-unknowns.md` - v1.5 deferred-surface and non-claim wording.
- `docs/operator/runtime-guide.md` - Operator-facing v1.5 checker and review-sequence guidance.
- `scripts/check-v1.5-release-boundaries.ts` - Deterministic Bun checker for v1.5 release-boundary drift.
- `scripts/verify.sh` - Default verification now runs the v1.5 checker.
- `docs/metrics/lines-of-code.md` - Refreshed tracked generated line-count artifact.
- `.planning/phases/67-release-boundaries-and-deterministic-verification/67-VERIFICATION.md` - Verification evidence for REL-01 through REL-04.

## Decisions Made

- v1.5 closeout language uses "source-built, explicit opt-in extended unattended mainnet operator review readiness" and avoids production-node readiness wording.
- Default verification remains deterministic; public-network live-smoke, manual-peer probing, restart-after-progress, `systemctl --user`, and `launchctl` remain opt-in UAT only.
- The v1.5 checker validates both parity roots and forbidden default-verification strings to make documentation drift fail locally.

## Deviations from Plan

None - plan executed as written. The tracked LOC report required refresh after adding the Phase 67 artifacts, which the plan anticipated.

## Issues Encountered

- Initial `bash scripts/verify.sh` stopped on a stale `docs/metrics/lines-of-code.md`; regenerated it with the repo-owned LOC script and reran verification successfully.
- The first checker draft had two assertions that were too sensitive to Markdown line wrapping; adjusted them to assert stable phrase fragments instead.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 67 closes the v1.5 release-boundary and deterministic-verification surface. The milestone is ready for milestone completion or any separate verification/UAT workflow the operator wants to run.

---
*Phase: 67-release-boundaries-and-deterministic-verification*
*Completed: 2026-06-09*
