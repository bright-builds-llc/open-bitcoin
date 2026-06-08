---
phase: 65-support-bundle-and-operator-review-docs
plan: 02
subsystem: operator-docs
tags: [support-bundle, operator-review, verification, parity, docs]
requires:
  - phase: 65-support-bundle-and-operator-review-docs
    plan: 01
    provides: support bundle service and availability evidence labels
provides:
  - v1.5 operator review runbook with repo-local Cargo and Bazel commands
  - Deterministic Phase 65 support review checker
  - Default verification guard for public-network and service-manager exclusions
affects: [phase-65, docs, verification, parity, v1.5-uat]
tech-stack:
  added: []
  patterns: [bun-deterministic-checker, opt-in-uat-boundaries]
key-files:
  created:
    - scripts/check-phase65-support-review.ts
    - .planning/phases/65-support-bundle-and-operator-review-docs/65-VERIFICATION.md
  modified:
    - docs/operator/runtime-guide.md
    - docs/architecture/operator-observability.md
    - docs/parity/catalog/p2p.md
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Keep v1.5 public-network and real service-manager review as opt-in UAT outside default verification."
patterns-established:
  - "Phase-specific Bun checkers guard both required docs wording and forbidden default-verification commands."
requirements-completed: [OBS-03, OBS-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 65-2026-06-08T14-45-59
generated_at: 2026-06-08T15:36:00.000Z
duration: 28min
completed: 2026-06-08
---

# Phase 65: Support Bundle and Operator Review Docs Summary

**Operator review docs and deterministic Phase 65 verification boundaries are complete.**

## Performance

- **Duration:** 28 min
- **Completed:** 2026-06-08T15:36:00Z
- **Tasks:** 3
- **Files modified:** 7

## Accomplishments

- Added `scripts/check-phase65-support-review.ts` and wired it into `scripts/verify.sh` after the Phase 64 checker.
- Added a `v1.5 operator review` runbook with repo-local Cargo and Bazel forms for deterministic checks, sync status, status snapshot, service status/restart, and support bundle collection.
- Documented support bundle interpretation around `support-evidence.json`, `support-evidence.md`, `live_smoke.summary.finalStatus`, `restartResumeEvidence`, `status.service.restart_resume`, `status.metrics`, `status.logs`, unavailable reasons, and opt-in UAT boundaries.
- Added parity wording for v1.5 support bundle/operator review evidence without broadening production-node, service, or release claims.
- Refreshed the tracked LOC report after adding the checker.

## Verification

- `bun run scripts/check-phase65-support-review.ts` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle --all-features` passed.
- `bash scripts/verify.sh` passed.

## Decisions Made

- The checker asserts required docs/source evidence and explicitly rejects public-network live-smoke, manual peer, restart-after-progress, `systemctl --user`, and `launchctl` commands in default verification.
- Operator docs frame public-network and real service-manager review as opt-in UAT evidence, not default deterministic verification.

## Deviations from Plan

None.

## Issues Encountered

- `bash scripts/verify.sh` initially failed on stale `docs/metrics/lines-of-code.md`; the tracked report was regenerated and verification passed afterward.

## User Setup Required

None.

## Next Phase Readiness

Phase 66 can add the compatibility harness operator wrapper while preserving the default verification and release-boundary constraints established here.

---
*Phase: 65-support-bundle-and-operator-review-docs*
*Completed: 2026-06-08*
