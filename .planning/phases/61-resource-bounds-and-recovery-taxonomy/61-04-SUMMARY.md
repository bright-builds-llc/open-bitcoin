---
phase: 61-resource-bounds-and-recovery-taxonomy
plan: 04
subsystem: operator-evidence
tags: [live-smoke, support-bundle, recovery-taxonomy, resource-bounds, redaction, rust, typescript]

# Dependency graph
requires:
  - phase: 61-resource-bounds-and-recovery-taxonomy
    provides: typed recovery categories and status projections from plans 61-01 and 61-02
  - phase: 59-operator-evidence-threat-model-and-release-boundaries
    provides: support evidence allowlist and redaction contract
provides:
  - live-smoke reports with Phase 61 recovery labels and compact final-status resource pressure
  - support bundle summaries that retain only allowlisted recovery/resource evidence
  - deterministic fixture coverage proving public-network live smoke stays outside default verification
affects: [operator-support, live-smoke, support-evidence, v1.5-uat, phase-65-support-bundle]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - support evidence keeps nested live-smoke fields behind explicit allowlists
    - live-smoke final status converts status JSON resource pressure into compact camelCase report fields

key-files:
  created:
    - .planning/phases/61-resource-bounds-and-recovery-taxonomy/61-04-SUMMARY.md
  modified:
    - scripts/run-live-mainnet-smoke.ts
    - scripts/test-run-live-mainnet-smoke.sh
    - packages/open-bitcoin-cli/src/operator/support/live_smoke.rs
    - packages/open-bitcoin-cli/src/operator/support/render.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Use the ten Phase 61 recovery labels in live-smoke diagnosis output instead of the older v1.4 category set."
  - "Expose support bundle resource pressure through explicit allowlisted keys only, leaving raw peer/report material omitted."
  - "Keep public-network live smoke opt-in and absent from scripts/verify.sh."

patterns-established:
  - "Final status support evidence can add compact nested fields by using a dedicated summarizer and key list."
  - "Live-smoke resource bounds flow from snake_case status JSON into camelCase report/support evidence."

requirements-completed: [RR-01, RR-02, RR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 61-2026-06-06T03-43-41
generated_at: 2026-06-06T14:25:57Z

# Metrics
duration: 31m 15s
completed: 2026-06-06
---

# Phase 61 Plan 04: Live-Smoke and Support Evidence Summary

**Phase 61 recovery labels and compact resource-pressure facts now flow through opt-in live-smoke reports and redacted support bundles.**

## Performance

- **Duration:** 31m 15s
- **Started:** 2026-06-06T13:54:42Z
- **Completed:** 2026-06-06T14:25:57Z
- **Tasks:** 2
- **Files modified:** 6 implementation/generated files, plus this summary

## Accomplishments

- Replaced live-smoke recovery diagnosis output with the ten Phase 61 category labels, including storage lock/backend split and operator cancellation.
- Added compact final-status `recoveryCategory` and `resourcePressure` report fields without adding raw daemon tails, endpoint arrays, or peer history to support-facing summaries.
- Extended support bundle JSON and Markdown to preserve only allowlisted recovery/resource evidence while keeping Phase 59 redaction expectations intact.
- Confirmed `scripts/verify.sh` still contains no public-network live-smoke, manual-peer, or restart-after-progress invocation.

## Task Commits

Each completed task was committed atomically with normal hooks:

1. **Task 1: Align live-smoke recovery diagnosis labels and compact final-status bounds** - `cafd96c` (`feat`)
2. **Task 2: Allowlist compact recovery and resource fields in support evidence** - `f3cb3e4` (`feat`)

TDD RED failure evidence was captured before implementation, but failing RED commits were not created because this run required normal hooks and no `--no-verify`.

## Files Created/Modified

- `scripts/run-live-mainnet-smoke.ts` - Uses Phase 61 recovery labels and adds compact `ResourcePressureSummary` parsing from status JSON.
- `scripts/test-run-live-mainnet-smoke.sh` - Adds deterministic fixtures for the new recovery labels, compact resource pressure, and verify-script exclusion.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` - Adds final-status recovery category and allowlisted nested resource-pressure extraction.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Renders `Recovery category` and `Resource pressure` labels in support Markdown.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Renames and extends the support bundle fixture for Phase 61 recovery/resource evidence and forbidden raw markers.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC report after source changes.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-04-SUMMARY.md` - Records plan completion and verification.

## Decisions Made

- Phase 61 category names are the cross-surface contract for live-smoke report comparison and support evidence.
- Support evidence keeps `resourcePressure` behind a dedicated `RESOURCE_PRESSURE_KEYS` allowlist rather than copying the whole object.
- Final plan checks record the serial `bash scripts/test-run-live-mainnet-smoke.sh` pass; one concurrent run with Cargo returned a bare failure, so the fixture should not be treated as a parallel-safe command.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed generated live-smoke fixture JSON quoting**
- **Found during:** Task 1 (Align live-smoke recovery diagnosis labels and compact final-status bounds)
- **Issue:** The new shell fixture initially wrote the recovery category value with invalid JSON quoting, causing the deterministic script test to fail before the production parsing path could be verified.
- **Fix:** Corrected the fixture JSON generation so `sync.recovery_category.value` emits a normal JSON string.
- **Files modified:** `scripts/test-run-live-mainnet-smoke.sh`
- **Verification:** `bash scripts/test-run-live-mainnet-smoke.sh` passed.
- **Committed in:** `cafd96c`

**2. [Rule 3 - Blocking] Regenerated tracked LOC report required by repo verification**
- **Found during:** Task 1 and Task 2 verification
- **Issue:** `bash scripts/verify.sh` failed immediately when `docs/metrics/lines-of-code.md` was stale after source edits.
- **Fix:** Ran `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` and included the refreshed tracked artifact in the related task commits.
- **Files modified:** `docs/metrics/lines-of-code.md`
- **Verification:** `bash scripts/verify.sh` passed after regeneration and in normal commit hooks.
- **Committed in:** `cafd96c`, `f3cb3e4`

***

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking verification freshness issue)
**Impact on plan:** No scope expansion; both fixes were required for deterministic verification and tracked-generated-file freshness.

## Issues Encountered

- The Task 2 RED test failed at the intended assertion: `finalStatus.recoveryCategory` was `Null` before the support allowlist was updated.
- A final parallel run of `bash scripts/test-run-live-mainnet-smoke.sh` alongside the Cargo support test returned a bare failure; rerunning the shell fixture serially passed. Final evidence uses the serial pass.

## Verification

Focused plan checks:

- `bash scripts/test-run-live-mainnet-smoke.sh` - passed after Task 1 and passed again serially after Task 2.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle_summarizes_phase61_resource_recovery_evidence --all-features` - failed in RED as expected, then passed after Task 2.
- `if rg -n "run-live-mainnet-smoke|--manual-peer|--restart-after-progress" scripts/verify.sh; then exit 1; fi` - passed.
- `rg -n "clean_shutdown|unclean_shutdown|incompatible_schema|storage_lock_contention|storage_backend_failure|operator_cancellation|ResourcePressureSummary|recoveryCategory|resourcePressure" scripts/run-live-mainnet-smoke.ts` - passed.
- `rg -n "incompatible_schema|storage_lock_contention|maxBlocksInFlightTotal|recoveryCategory" scripts/test-run-live-mainnet-smoke.sh` - passed.
- `rg -n "RESOURCE_PRESSURE_KEYS|recoveryCategory|resourcePressure|maxBlocksInFlightTotal|targetOutboundPeers" packages/open-bitcoin-cli/src/operator/support/live_smoke.rs packages/open-bitcoin-cli/tests/operator_binary.rs` - passed.
- `rg -n "Recovery category|Resource pressure" packages/open-bitcoin-cli/src/operator/support/render.rs packages/open-bitcoin-cli/tests/operator_binary.rs` - passed.

Repo verification before Task 2 commit:

- `cargo fmt --manifest-path packages/Cargo.toml --all` - passed.
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` - passed.
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` - passed.
- `cargo test --manifest-path packages/Cargo.toml --all-features` - passed.
- `bash scripts/verify.sh` - passed after LOC regeneration.

Normal commit hooks:

- Task 1 commit hook ran `bash scripts/verify.sh` and passed.
- Task 2 commit hook ran `bash scripts/verify.sh` and passed.

## Stub Scan

No blocking stubs found. The scan matched existing null/empty guard logic, redaction fixture strings, and unavailable-state support evidence terms; none are placeholders or unwired mock data.

## Threat Surface Scan

No unplanned threat flags. The plan's local live-smoke report to support bundle boundary was mitigated by explicit field allowlists and regression tests for raw daemon tails, endpoint tables, manual peers, cookie-like text, wallet-like text, `rpcpassword`, and raw endpoint addresses.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 65 support bundle work can consume the compact `recoveryCategory` and `resourcePressure` shape without expanding raw live-smoke report exposure. Public-network long-run review remains opt-in UAT evidence and outside default verification.

## Self-Check: PASSED

- Found summary and all six modified implementation/generated files.
- Found task commits `cafd96c` and `f3cb3e4` in git history.

***
*Phase: 61-resource-bounds-and-recovery-taxonomy*
*Completed: 2026-06-06*
