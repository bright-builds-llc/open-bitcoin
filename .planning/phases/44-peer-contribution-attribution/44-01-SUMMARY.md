---
phase: 44
plan: 01
subsystem: sync-runtime
tags: [peer-telemetry, sync, live-smoke, rust, bun]
requires:
  - phase: 43
    provides: outbound peer resilience and retry-backoff visibility
provides:
  - validation-gated per-peer header and block contribution accounting
  - failed-peer activity retention for connected invalid-data outcomes
  - live-smoke runtime peer contribution evidence rows
  - operator guidance for activity versus useful contribution
affects: [phase-45-resource-bounds, phase-50-public-mainnet-proof]
tech-stack:
  added: []
  patterns:
    - split sync peer activity accounting from useful contribution accounting
    - render durable peer telemetry as support evidence without public-network verification
key-files:
  created:
    - .planning/phases/44-peer-contribution-attribution/44-01-SUMMARY.md
    - .planning/phases/44-peer-contribution-attribution/44-REVIEW.md
    - .planning/phases/44-peer-contribution-attribution/44-REVIEW-FIX.md
  modified:
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/progress.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - scripts/run-live-mainnet-smoke.ts
    - scripts/test-run-live-mainnet-smoke.sh
    - docs/operator/runtime-guide.md
key-decisions:
  - "Count useful peer contribution only after the existing sync path accepts headers or preserves blocks."
  - "Preserve connected failed-peer activity and failure reason separately from useful contribution."
  - "Keep live-smoke contribution evidence derived from durable runtime telemetry, not endpoint preflight results."
patterns-established:
  - "Peer activity (`messages_processed`, last activity) is diagnostic evidence, not useful sync progress."
  - "Live-smoke Markdown can add support tables while keeping the opt-in public-network boundary."
requirements-completed: [PEER-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: "44-2026-05-25T16-03-34"
generated_at: "2026-05-25T16:39:31Z"
duration: 11 min
completed: 2026-05-25
---

# Phase 44 Plan 01: Validation-Gated Peer Contribution Evidence Summary

**Validation-gated peer contribution telemetry with live-smoke support evidence and operator interpretation guidance**

## Performance

- **Duration:** 11 min
- **Started:** 2026-05-25T16:28:12Z
- **Completed:** 2026-05-25T16:39:31Z
- **Tasks:** 2
- **Files modified:** 7 implementation/docs files plus GSD artifacts

## Accomplishments

- Split peer activity accounting from useful contribution accounting in the durable sync runtime.
- Moved per-peer header and block contribution increments behind existing accepted sync handling.
- Preserved last activity, accepted contribution, capabilities when available, and failure reason for connected peers that later fail.
- Added deterministic `peer_contribution_` regressions for accepted contribution, invalid-header zero credit, and waiting/stalled zero credit.
- Extended live-smoke JSON and Markdown reports with runtime peer contribution rows from durable `recent_peers`.
- Documented validation-gated contribution semantics in the operator runtime guide.

## Task Commits

The wrapper for this run requires commit/push only after clean phase verification, so task-level commits were intentionally deferred until the final strict gate.

1. **Task 1: Gate runtime peer contribution on accepted sync data** - pending final verification commit
2. **Task 2: Surface contribution evidence in live smoke reports and docs** - pending final verification commit

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/progress.rs` - Adds explicit activity and contribution helpers, plus failed-outcome conversion for partial progress.
- `packages/open-bitcoin-node/src/sync.rs` - Wires contribution increments after accepted sync handling and preserves failed connected-peer activity.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Uses Unix-second retry backoff conversion for peer retry scheduling.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Adds PEER-03 regressions for accepted, invalid, stalled, and waiting peer outcomes.
- `scripts/run-live-mainnet-smoke.ts` - Carries durable per-peer contribution fields into JSON and Markdown reports.
- `scripts/test-run-live-mainnet-smoke.sh` - Extends deterministic live-smoke fixtures and assertions for contribution rows.
- `docs/operator/runtime-guide.md` - Explains validation-gated contribution counters and live-smoke contribution evidence.

## Decisions Made

- Duplicate but accepted headers or blocks count under Phase 44 because the phase gates on existing sync acceptance, not novelty or peer scoring.
- Failed connected peers keep accepted contribution and activity evidence, while pre-connect failures remain zero-activity outcomes.
- Live-smoke report contribution rows are generated from durable runtime peer telemetry only; preflight endpoint rows remain separate diagnostics.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Refreshed best heights and capabilities on failed connected peer outcomes**
- **Found during:** Task 1 diff review
- **Issue:** A connected peer that accepted valid data and then failed could preserve counters without refreshing summary best heights or capabilities.
- **Fix:** On connected-peer failure, refresh best heights in the summary and preserve capabilities when the peer manager has them.
- **Files modified:** `packages/open-bitcoin-node/src/sync.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features peer_contribution`
- **Committed in:** pending final verification commit

**2. [Code Review - Critical] Hardened live-smoke command detection**
- **Found during:** GSD code review
- **Issue:** Environment override values were interpolated into a shell command while checking command availability.
- **Fix:** Passed the command as a positional shell argument and added a regression that proves shell metacharacters are not executed.
- **Files modified:** `scripts/run-live-mainnet-smoke.ts`, `scripts/test-run-live-mainnet-smoke.sh`
- **Verification:** `bash scripts/test-run-live-mainnet-smoke.sh`

**3. [Code Review - Warning] Fixed stalled peer counts and retry timestamp units**
- **Found during:** GSD code review
- **Issue:** Stalled peers were included in connected peer counts, and retry backoff milliseconds were used as Unix seconds in runtime timestamps.
- **Fix:** Count only connected outcomes as connected peers and convert retry backoff milliseconds to rounded-up seconds for idle-loop and peer backoff scheduling.
- **Files modified:** `packages/open-bitcoin-node/src/sync.rs`, `packages/open-bitcoin-node/src/sync/runtime_state.rs`, `packages/open-bitcoin-node/src/sync/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features backoff`

**4. [Code Review - Warning/Info] Tightened live-smoke operator input handling**
- **Found during:** GSD code review
- **Issue:** Partial binary overrides skipped required builds, and numeric parsing accepted trailing junk.
- **Fix:** Skip builds only when both binary overrides are present and require full decimal strings for ports and positive integer options.
- **Files modified:** `scripts/run-live-mainnet-smoke.ts`, `scripts/test-run-live-mainnet-smoke.sh`
- **Verification:** `bash scripts/test-run-live-mainnet-smoke.sh`

**Total deviations:** 4 auto-fixed (1 local bug, 1 critical review issue, 2 warning/info review clusters).
**Impact on plan:** The fixes strengthen the planned contribution accounting and live-smoke evidence path without widening Phase 44 beyond PEER-03. The existing smoke-runner file-size refactor remains deferred as a separate maintainability concern.

## Issues Encountered

- Plan verification initially found an unresolved research question about duplicate accepted data. It was resolved in `44-RESEARCH.md` by recording the chosen Phase 44 semantics: count data accepted by the existing sync path and defer novelty/deduplication policy.

## User Setup Required

None - no external service configuration required.

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features peer_contribution` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features backoff` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features contextual_invalid_headers_fail_with_typed_invalid_data` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features mixed_peer_failures_rotate_to_replacement_without_corrupting_state` passed.
- `bash scripts/test-run-live-mainnet-smoke.sh` passed.
- `bun run scripts/run-live-mainnet-smoke.ts --help` passed.
- `git diff --check` passed.

## Next Phase Readiness

Phase 45 can rely on peer contribution counters that distinguish useful accepted sync progress from idle, waiting, stalled, or failed activity. Resource-bound and peer scoring policy work remains explicitly out of Phase 44.

*Phase: 44-peer-contribution-attribution*
*Completed: 2026-05-25*
