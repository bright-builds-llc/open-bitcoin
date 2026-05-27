---
phase: 49-threat-model-and-release-boundaries
plan: 02
subsystem: parity-verification
tags: [parity-roots, bun, verification, release-boundaries]
requires:
  - phase: 49-01
    provides: v1.3 threat model and release-readiness claim boundary
provides:
  - parity root links for v1.3 threat model and release boundaries
  - deterministic Bun release-boundary assertion
  - verification wiring without public-network checks
affects: [phase-50-public-mainnet-progress-evidence-closeout, scripts-verify]
tech-stack:
  added: []
  patterns:
    - parse parity JSON roots with Bun and Node standard library only
    - assert documentation invariants without network access
key-files:
  created:
    - scripts/check-v1.3-release-boundaries.ts
  modified:
    - docs/parity/checklist.md
    - docs/parity/index.json
    - docs/parity/README.md
    - docs/parity/deviations-and-unknowns.md
    - scripts/verify.sh
key-decisions:
  - "Use v1-3-threat-model-release-boundaries as the parity checklist surface id."
  - "Use v1_3_threat_model and v1_3_release_boundaries as parity audit keys."
  - "Wire only deterministic docs/root assertions into scripts/verify.sh."
patterns-established:
  - "Parity root changes that affect release claims get a local script assertion rather than a public-network gate."
requirements-completed: [PROOF-06, SEC-01, SEC-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 49-2026-05-27T21-24-44
generated_at: 2026-05-27T22:22:08Z
duration: 22 min
completed: 2026-05-27
---

# Phase 49 Plan 02: Parity Roots And Deterministic Boundary Assertion Summary

**Parity roots now link Phase 49 threat and release-boundary artifacts, with local verification proving those links stay present**

## Performance

- **Duration:** 22 min
- **Started:** 2026-05-27T22:00:00Z
- **Completed:** 2026-05-27T22:22:08Z
- **Tasks:** 2
- **Files modified:** 6 docs/script files plus this summary

## Accomplishments

- Added `v1-3-threat-model-release-boundaries` to the human and JSON parity roots.
- Linked the v1.3 threat model from `docs/parity/README.md`, `docs/parity/checklist.md`, `docs/parity/index.json`, and `docs/parity/release-readiness.md`.
- Added `v1_3_threat_model` and `v1_3_release_boundaries` audit entries to `docs/parity/index.json`.
- Added `scripts/check-v1.3-release-boundaries.ts` to assert requirement traces, document links, non-claim language, and the absence of `run-live-mainnet-smoke` from `scripts/verify.sh`.
- Wired the new assertion into `bash scripts/verify.sh` immediately after parity breadcrumb checks.

## Task Commits

The strict wrapper for this run commits only after phase-level verification.

1. **Task 1: Link Phase 49 from parity roots and deferred-surface docs** - pending final verification commit
2. **Task 2: Add deterministic release-boundary assertion to verification** - pending final verification commit

## Files Created/Modified

- `docs/parity/checklist.md` - Adds the Phase 49 checklist surface and requirement/evidence links.
- `docs/parity/index.json` - Adds parity surface and audit roots for the v1.3 threat model and release boundaries.
- `docs/parity/README.md` - Adds the threat model to the parity file index.
- `docs/parity/deviations-and-unknowns.md` - Adds explicit v1.3 non-claims and support-bundle boundary language.
- `scripts/check-v1.3-release-boundaries.ts` - Adds deterministic parity-root and release-boundary assertions.
- `scripts/verify.sh` - Runs the new local assertion without adding public-network checks.

## Decisions Made

- The deterministic assertion checks docs and JSON roots only; it intentionally does not inspect live reports, support bundles, cookies, wallets, logs, or public peers.
- The parity root remains a checklist/audit index, not a machine-readable release-claims schema.
- `scripts/verify.sh` remains public-network-free and fails if `run-live-mainnet-smoke` is added.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Verification Evidence

- `jq empty docs/parity/index.json`
- `bun run scripts/check-v1.3-release-boundaries.ts`
- `rg -n "v1-3-threat-model-release-boundaries|threat-model-v1\.3\.md|PROOF-06|SEC-01|SEC-02" docs/parity/checklist.md docs/parity/index.json docs/parity/README.md docs/parity/deviations-and-unknowns.md`
- `rg -n "inbound serving|transaction relay|production-funds|migration apply mode|packaging|hosted/public dashboard|GUI|unattended production-node" docs/parity/deviations-and-unknowns.md`
- `rg -n "check-v1\.3-release-boundaries\.ts" scripts/verify.sh`
- `if rg -n "run-live-mainnet-smoke" scripts/verify.sh; then exit 1; fi`

## Next Phase Readiness

Phase 50 can now start from the parity root, inspect the v1.3 threat model and release boundary, and capture public-mainnet progress or diagnosed-blocker evidence using the documented acceptance contract.

---
*Phase: 49-threat-model-and-release-boundaries*
*Completed: 2026-05-27*
