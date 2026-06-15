---
phase: 77-corruption-and-lock-recovery-hardening
plan: 04
subsystem: recovery
tags: [rust, support, live-smoke, dashboard, recovery]

requires:
  - phase: 77-corruption-and-lock-recovery-hardening
    provides: Plan 77-03 top-level status recovery evidence projection
provides:
  - Compact support recovery evidence projection sourced from status.recovery_evidence
  - Live-smoke recovery evidence report fields and allowlisted support summary projection
  - Dashboard Recovery evidence row sourced from the shared status snapshot
  - Probe-only support store-health collection without Fjall store opens
affects: [support, live-smoke, dashboard, parity-breadcrumbs]

tech-stack:
  added: []
  patterns:
    - Shared status recovery evidence is projected into support, live-smoke, and dashboard surfaces without renderer-local classification.
    - Support bundle store-health evidence is derived from status and remains probe-only.

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/support/live_smoke/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/recovery.rs
  modified:
    - docs/parity/source-breadcrumbs.json
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-cli/src/operator/support/evidence.rs
    - packages/open-bitcoin-cli/src/operator/support/live_smoke.rs
    - packages/open-bitcoin-cli/src/operator/support/render.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - scripts/run-live-mainnet-smoke.ts
    - scripts/test-run-live-mainnet-smoke.sh

key-decisions:
  - "Support, live-smoke, and dashboard projections consume `status.recovery_evidence` as the shared recovery contract and keep legacy recovery category/action only as compatibility fields."
  - "Support store-health evidence is status-derived and probe-only; support collection no longer opens Fjall stores for runtime metadata or metrics history."

patterns-established:
  - "Operator support Markdown uses a dedicated `## Recovery Evidence` section with shared action class, cause, category, and next action fields."
  - "Live-smoke reports expose compact `recoveryEvidence` plus scalar `recoveryActionClass`, `recoveryCause`, and `recoveryNextAction` fields while support summaries remain allowlisted."

requirements-completed: [REC-05, REC-06, REC-07, REC-08]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 77-2026-06-15T18-33-03
generated_at: 2026-06-15T23:37:43Z

duration: 36min
completed: 2026-06-15
---

# Phase 77 Plan 04: Recovery Evidence Projection Summary

**Shared status recovery evidence now flows into support bundles, live-smoke reports, and dashboard rows without opening damaged datadirs or duplicating classification logic.**

## Performance

- **Duration:** 36min
- **Started:** 2026-06-15T23:01:21Z
- **Completed:** 2026-06-15T23:37:43Z
- **Tasks:** 3
- **Files modified:** 12

## Accomplishments

- Added compact support JSON and Markdown recovery evidence from `OpenBitcoinStatusSnapshot.recovery_evidence`, preserving unavailable reasons.
- Updated full-sync support evidence to prefer top-level recovery evidence and keep legacy recovery category/action only as fallback compatibility context.
- Added live-smoke JSON, Markdown, shell-fixture, and support-summary projection for recovery evidence with allowlisted fields only.
- Added dashboard `Recovery evidence` rendering from the top-level status snapshot instead of parsing legacy `sync.recovery_action` prose.
- Removed support-bundle runtime metadata and metrics history Fjall opens, replacing them with status-derived probe-only evidence.

## Task Commits

Each task was committed atomically:

1. **Task 1 RED: Add failing support recovery evidence tests** - `a1da6a3` (test)
2. **Task 1 GREEN: Project support recovery evidence** - `c157865` (feat)
3. **Task 2 RED: Add failing live-smoke recovery evidence tests** - `c824953` (test)
4. **Task 2 GREEN: Project live-smoke recovery evidence** - `9a6acff` (feat)
5. **Task 3 RED: Add failing dashboard recovery evidence tests** - `2168b13` (test)
6. **Task 3 GREEN: Project dashboard recovery evidence** - `0fc8556` (feat)
7. **Cleanup: Keep recovery projection files bounded** - `4a78e44` (refactor)

## Files Created/Modified

- `docs/parity/source-breadcrumbs.json` - Registers the new Rust child modules for parity breadcrumb checks.
- `packages/open-bitcoin-cli/src/operator/support.rs` - Adds compact recovery evidence and status-only store-health collection.
- `packages/open-bitcoin-cli/src/operator/support/evidence.rs` - Prefers top-level recovery evidence in full-sync support summaries.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` - Summarizes allowlisted recovery evidence from live-smoke reports.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke/tests.rs` - Holds live-smoke summary tests split out of the production module.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Renders the support Markdown `## Recovery Evidence` section.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Covers support JSON, Markdown, unavailable evidence, full-sync preference, and probe-only store health.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Adds the dashboard `Recovery evidence` row.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/recovery.rs` - Centralizes dashboard recovery category and evidence formatting.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Covers available and unavailable dashboard recovery evidence rows.
- `scripts/run-live-mainnet-smoke.ts` - Adds final-status recovery evidence fields and Markdown rendering.
- `scripts/test-run-live-mainnet-smoke.sh` - Adds deterministic recovery evidence fixtures and assertions.

## Decisions Made

- Reused `status.recovery_evidence` as the single recovery evidence source for support, live-smoke, and dashboard projection.
- Kept legacy `sync.recovery_category` and `sync.recovery_action` compatibility fields, but did not parse legacy prose for cause or action class.
- Treated support store-health collection as status projection only, with explicit unavailable reasons when runtime metadata or metrics history is not already present.
- Kept live-smoke opt-in; no `scripts/verify.sh` wiring was added.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib support_recovery_evidence_ --all-features`
- `bash scripts/test-run-live-mainnet-smoke.sh`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib live_smoke_recovery_evidence_ --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib dashboard_recovery_evidence_ --all-features`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- Negative scan: no `FjallNodeStore::open` in `packages/open-bitcoin-cli/src/operator/support.rs`.
- Negative scan: no live-smoke wiring in `scripts/verify.sh`.
- Negative scan: no dashboard `sync.recovery_action` parsing for cause or action class.
- Line-count check: touched production Rust files are below the local trigger.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Repo Rule Compliance] Split recovery projection support to keep production Rust files bounded**
- **Found during:** Final verification cleanup
- **Issue:** The initial implementation left several touched production Rust files over the repo line-count trigger.
- **Fix:** Moved live-smoke tests into `support/live_smoke/tests.rs`, moved dashboard recovery formatting into `dashboard/model/recovery.rs`, simplified support store-health collection to status-only functions, and added parity breadcrumbs for the new Rust files.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support.rs`, `packages/open-bitcoin-cli/src/operator/support/tests.rs`, `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs`, `packages/open-bitcoin-cli/src/operator/support/live_smoke/tests.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`, `packages/open-bitcoin-cli/src/operator/dashboard/model/recovery.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Focused plan tests, parity breadcrumb check, negative scans, and `wc -l` line-count check passed.
- **Committed in:** `4a78e44`

---

**Total deviations:** 1 auto-fixed (Rule 2)
**Impact on plan:** The deviation reduced file size and strengthened the probe-only support collection path without changing the requested user-facing behavior.

## Issues Encountered

- A stale dashboard helper call remained after moving recovery formatting into a child module. The focused support test build caught it, and the call site was corrected before rerunning all focused checks.

## Known Stubs

None. The stub scan found only format strings, null guards, and deterministic shell fixture mechanics; no placeholder data or unwired UI/support data source was introduced.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 77-04 leaves support, live-smoke, and dashboard surfaces aligned on the shared recovery evidence contract. Follow-on recovery work can consume the same top-level status evidence without adding new support-bundle datadir probes or renderer-local recovery classification.

---
*Phase: 77-corruption-and-lock-recovery-hardening*
*Completed: 2026-06-15*

## Self-Check: PASSED

- Found summary file at `.planning/phases/77-corruption-and-lock-recovery-hardening/77-04-SUMMARY.md`.
- Verified task and cleanup commits exist: `a1da6a3`, `c157865`, `c824953`, `9a6acff`, `2168b13`, `0fc8556`, `4a78e44`.
- Confirmed no failed self-check marker remains.
