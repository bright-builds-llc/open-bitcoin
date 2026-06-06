---
phase: 62-long-run-sync-truth-surfaces
plan: 03
subsystem: observability
tags: [typescript, bun, live-smoke, operator-reports, phase-62]

requires:
  - phase: 62-long-run-sync-truth-surfaces
    provides: Phase 62 shared sync truth contract from plan 62-01
provides:
  - Live-smoke JSON snapshots with Phase 62 configured targets, attempt counters, progress signal, stop reason, recovery, and resource-pressure fields
  - Compact live-smoke Markdown snapshot and final-status rendering without persisted raw daemon tails
  - Deterministic fixture coverage for the Phase 62 live-smoke truth report contract
affects: [live-smoke, operator-observability, support-evidence, phase-62]

tech-stack:
  added: []
  patterns:
    - Single TypeScript camelCase mapping layer from typed `metadata.maybe_sync_state.sync` FieldAvailability values
    - Persist daemon output diagnosis as counts and observed flags instead of raw stdout/stderr tails

key-files:
  created:
    - .planning/phases/62-long-run-sync-truth-surfaces/62-03-SUMMARY.md
  modified:
    - scripts/run-live-mainnet-smoke.ts
    - scripts/test-run-live-mainnet-smoke.sh
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Map live-smoke report fields only from typed durable sync status JSON, keeping camelCase differences isolated to the TypeScript report layer."
  - "Write deterministic fixture reports under packages/target/live-mainnet-smoke-reports so the plan acceptance commands inspect generated JSON and Markdown directly."
  - "Keep daemon output evidence compact by persisting exit/signal plus stdout/stderr observed flags and line counts, not raw tail text."

patterns-established:
  - "Phase 62 live-smoke unavailable fields retain explicit `maybe...UnavailableReason` fields for report and Markdown rendering."
  - "Live-smoke fixture verification now checks both generated JSON contract fields and Markdown table headings."

requirements-completed: [OBS-01, OBS-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 62-2026-06-06T19-46-48
generated_at: 2026-06-06T22:26:28Z

duration: 50m
completed: 2026-06-06
---

# Phase 62 Plan 03: Live-Smoke Truth Report Summary

**Live-smoke JSON and Markdown reports now project Phase 62 sync truth fields compactly from typed durable status**

## Performance

- **Duration:** 50m
- **Started:** 2026-06-06T21:36:00Z
- **Completed:** 2026-06-06T22:26:28Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Added `ConfiguredTargetsSummary`, `AttemptCountersSummary`, and `StopReasonSummary` plus helper mappings from typed `FieldAvailability` status JSON.
- Extended live-smoke snapshots, restart progress evidence, and final status with configured targets, attempt counters, progress signal, stop reason, recovery action/category, resource pressure, latest error reasons, and bounded counters.
- Reworked Markdown snapshots and final durable status to show compact Phase 62 rows and explicit unavailable text for missing facts.
- Removed persisted raw daemon stdout/stderr tails from JSON and Markdown reports, replacing them with output observed flags and line counts.
- Updated deterministic fixtures and assertions to validate generated JSON/Markdown without public-network access.

## Task Commits

1. **Task 1: Extend the live-smoke TypeScript truth mapping** - `cbe2845` (feat)
2. **Task 2: Render compact Phase 62 live-smoke Markdown snapshots** - `cbe2845` (feat)

## Files Created/Modified

- `scripts/run-live-mainnet-smoke.ts` - Adds the Phase 62 typed mapping layer, compact report fields, Markdown snapshot/final-status rendering, and daemon output summary persistence.
- `scripts/test-run-live-mainnet-smoke.sh` - Expands deterministic status fixtures and generated-report assertions for Phase 62 fields and raw-tail exclusion.
- `docs/metrics/lines-of-code.md` - Hook-regenerated tracked LOC report.
- `.planning/phases/62-long-run-sync-truth-surfaces/62-03-SUMMARY.md` - Execution summary.

## Decisions Made

- Kept report casing conversion in the live-smoke TypeScript mapping layer and did not parse human output, structured logs, or Markdown for truth fields.
- Wrote deterministic test reports to `packages/target/live-mainnet-smoke-reports` so the plan's explicit acceptance checks validate the generated artifacts.
- Preserved compact daemon diagnostics with line counts and observed flags, avoiding persisted raw output material.

## Verification

- `bash scripts/test-run-live-mainnet-smoke.sh` - passed.
- `bash -c 'bash scripts/test-run-live-mainnet-smoke.sh >/tmp/phase62-live-smoke-test.log 2>&1; ! rg -n "stdoutTail|stderrTail|Daemon Output Tail" packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.md'` - passed.
- `bun --eval 'const report = await Bun.file("packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json").json(); if (!report.final_status?.configuredTargets || !report.snapshots?.[0]?.progressSignal) throw new Error("Phase 62 live-smoke truth fields missing");'` - passed.
- `rg -n "ConfiguredTargetsSummary|AttemptCountersSummary|StopReasonSummary|configuredTargetsFromValue|attemptCountersFromValue|stopReasonFromValue|unavailableReasonFromFieldAvailability|progressSignal|latestStopReason" scripts/run-live-mainnet-smoke.ts` - passed.
- `rg -n "Signal \\| Configured Targets \\| Attempts|Latest Stop Reason|Unavailable: no sync status snapshots captured|Final Durable Status|Daemon Output Summary" scripts/run-live-mainnet-smoke.ts` - passed.
- `bash -n scripts/test-run-live-mainnet-smoke.sh` - passed.
- `git diff --check -- scripts/run-live-mainnet-smoke.ts scripts/test-run-live-mainnet-smoke.sh` - passed.
- Normal commit hooks passed; `bash scripts/verify.sh` completed in 7m 41.794s during the implementation commit.

## Deviations from Plan

None - plan executed as written.

## Issues Encountered

- Normal hooks regenerated `docs/metrics/lines-of-code.md`; it was included in the implementation commit as the repo-local guidance treats it as a tracked generated artifact.
- `.planning/ROADMAP.md`, `.planning/STATE.md`, and `.planning/config.json` were already modified by orchestration before this executor. Per the prompt, they were left unstaged and unmodified, so state/roadmap advancement was not run for this plan.

## Known Stubs

None. Stub scan findings were limited to existing guard/default assignments and intentional nullable-field handling in the report mapping, not UI/report stubs.

## Threat Surface

No new network endpoint, authentication path, schema boundary, or unbounded report store was introduced. The existing status JSON to local report boundary was narrowed by typed field mapping and raw daemon tail removal.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 62-04 can rely on live-smoke JSON and Markdown reports carrying the same Phase 62 configured target, attempt, progress, recovery, pressure, and block evidence vocabulary as shared status.

## Self-Check: PASSED

- FOUND: `scripts/run-live-mainnet-smoke.ts`
- FOUND: `scripts/test-run-live-mainnet-smoke.sh`
- FOUND: `docs/metrics/lines-of-code.md`
- FOUND: `cbe2845` (`feat(62-03): compact live smoke truth reports`)

---
*Phase: 62-long-run-sync-truth-surfaces*
*Completed: 2026-06-06*
