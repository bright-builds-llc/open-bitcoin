---
phase: 144-operator-flush-and-availability-evidence
plan: 04
subsystem: observability
tags: [chainstate-durability, csobs, docs, checker, breadcrumbs, verify]

requires:
  - phase: 144-operator-flush-and-availability-evidence
    provides: Dedicated chainstate_durability FieldAvailability field on OpenBitcoinStatusSnapshot
provides:
  - "status-snapshot, operator-observability, and runtime-guide document chainstate_durability with Cargo/Bazel UAT commands"
  - "Deterministic Phase 144 cross-surface checker wired after Phase 116 in verify.sh"
  - "Parity breadcrumbs cover every Plan 01-03 first-party Rust path"
affects:
  - 145-csvfy
  - operator-docs
  - verify-wiring

tech-stack:
  added: []
  patterns:
    - "Phase 144 checker copies the Phase 116 corpus/fixture pattern"
    - "Forbidden new-field needles allow wrapped must-not rustdoc via a previous-line window"

key-files:
  created:
    - scripts/check-phase144-operator-flush-availability-evidence.ts
    - scripts/check-phase144-operator-flush-availability-evidence.test.ts
  modified:
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
    - docs/operator/runtime-guide.md
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined each task RED and GREEN into one hook-passing feat/docs commit because pre-commit runs verify.sh"
  - "Inserted the Phase 144 bun test/run pair immediately after Phase 116, before Phase 121"
  - "Did not add a CSVFY or Phase 145 no-claim checker; Phase 145 still owns those guardrails"
  - "Breadcrumb JSON needed no new groups; Plan 01-03 files already belonged to existing groups"

patterns-established:
  - "Pattern 1: Docs grep for chainstate_durability, cache_size, last_flush_reason, have-bytes, and Unavailable:"
  - "Pattern 2: New-field forbidden needles fail unless a nearby no-claim marker is present"
  - "Pattern 3: UAT commands are repo-local Cargo and Bazel twins, not only the open-bitcoin alias"

requirements-completed: [CSOBS-01, CSOBS-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 144-2026-09-17T11-23-17
generated_at: 2026-09-17T17:28:57Z

duration: 30min
completed: 2026-09-17
---

# Phase 144 Plan 04: Docs, Checker, Breadcrumbs, And Verify Wiring Summary

**Operator docs now describe the shared `chainstate_durability` contract, and a deterministic Bun checker proves RPC/CLI/dashboard/metrics/logs/support agree on CSOBS-01 and CSOBS-02 — wired into `verify.sh` after Phase 116, with no prune/archive/public-default/production-readiness closeout.**

## Performance

- **Duration:** 30 min
- **Started:** 2026-09-17T16:59:11Z
- **Completed:** 2026-09-17T17:28:57Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- `status-snapshot.md`, `operator-observability.md`, and `runtime-guide.md` document `chainstate_durability` as a dedicated `FieldAvailability` field, `cache_size` OK/LARGE/CRITICAL occupancy, last real flush reason, have-bytes versus do-not, and fail-closed / interrupted-without-B tip omission.
- Runtime guide includes copy-pasteable repo-local Cargo and Bazel `status --format` and `support bundle` commands. Public-network review stays opt-in UAT only.
- `scripts/check-phase144-operator-flush-availability-evidence.ts` requires CSOBS-01/CSOBS-02, locked symbols and behavior tests, redaction needles, and Cargo/Bazel commands. Forbidden `getblock` / `pruned` claims fail on the new-field corpus.
- `verify.sh` runs the checker test and check immediately after the Phase 116 pair in both the visible bun block and the `run_step` block. Breadcrumb coverage for Plan 01-03 Rust paths was already complete.

## Task Commits

Each task was committed atomically:

1. **Task 1: Document the shared field and repo-local UAT commands** - `c812c0a3` (docs)
2. **Task 2: Checker, breadcrumbs, and verify.sh** - `792f1df7` (feat)

**Plan metadata:** pending docs commit after this summary.

_Note: Combined RED+GREEN in one hook-passing commit per task because pre-commit runs `verify.sh`, matching Phases 140–143 and 144-01/02/03._

## Files Created/Modified

- `docs/architecture/status-snapshot.md` - Top-level field row plus Phase 144 contract, locked names, and fail-closed tip rule
- `docs/architecture/operator-observability.md` - Seven MetricKind serde names, `CHAINSTATE_DURABILITY_LOG_SOURCE`, recursive support redaction
- `docs/operator/runtime-guide.md` - Have-bytes versus do-not inspection subsection with Cargo/Bazel commands and the locked Next action
- `scripts/check-phase144-operator-flush-availability-evidence.ts` - Cross-surface CSOBS checker
- `scripts/check-phase144-operator-flush-availability-evidence.test.ts` - Live-repo pass plus CSOBS/getblock/verify.sh fixtures
- `scripts/verify.sh` - Phase 144 bun test/run after Phase 116
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC

## Decisions Made

- Combined RED+GREEN because hooks run the workspace verifier; a RED-only commit cannot pass pre-commit.
- Insert Phase 144 immediately after Phase 116 rather than at the end of the bun list so D-21 stays auditable.
- Leave CSVFY-01 / CSVFY-02 and production-readiness claims to Phase 145.
- Do not invent a new breadcrumb group; the Plan 01-03 files already sit in `node-status-contract`, `node-chainstate-adapter`, `node-network-chainstate-durability-evidence`, `node-observability-contracts`, and the CLI operator groups.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Nearby-line no-claim window for wrapped rustdoc**
- **Found during:** Task 2
- **Issue:** `status/chainstate_durability.rs` splits `must not include pruned` / `getblock` across two rustdoc lines. A single-line scan treated the continuation as a positive claim.
- **Fix:** Treat the previous line plus current line as the no-claim window so wrapped "must not" comments pass and injected standalone `getblock` / `pruned` still fail.
- **Files modified:** `scripts/check-phase144-operator-flush-availability-evidence.ts`
- **Verification:** `bun test scripts/check-phase144-operator-flush-availability-evidence.test.ts`
- **Committed in:** `792f1df7` (Task 2)

***

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Necessary for the live-repo pass without weakening the injected-needle fail test. No CSVFY checker, no getblock section, no prune/archive/production-readiness claims. Combined RED+GREEN was an allowed plan exception, not a deviation.

## Issues Encountered

- First Task 1 commit attempt failed a pre-tool hook when the message used a HEREDOC. Retried with `git commit -F` and hooks on. No `--no-verify`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 145 can add CSVFY no-claim guardrails and parity-root citations without reopening the CSOBS field contract.
- CSOBS-01 and CSOBS-02 are implemented across status, RPC, CLI, dashboard, metrics, logs, support, docs, and the checker. Lifecycle-valid phase verification remains the phase-level closeout.

***
*Phase: 144-operator-flush-and-availability-evidence*
*Completed: 2026-09-17*

## Self-Check: PASSED
