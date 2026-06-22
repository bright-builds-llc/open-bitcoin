---
phase: 84-upgrade-and-rollback-policy
plan: 01
subsystem: docs
tags: [upgrade-policy, rollback, parity, support-evidence]

requires:
  - phase: 82-production-claim-boundary
    provides: Production claim boundary and Phase 82 support terms
  - phase: 83-support-matrix-and-issue-evidence
    provides: Support matrix, issue evidence, and redaction boundaries
provides:
  - Canonical Phase 84 upgrade and rollback policy
  - Source-built pre-upgrade checklist for operator evidence
  - State/schema compatibility table using existing recovery vocabulary
  - Failed-upgrade and rollback guidance with no-hidden-mutation boundaries
affects: [operator-runbooks, service-expectations, release-readiness, deterministic-claim-guardrails]

tech-stack:
  added: []
  patterns:
    - Field-level operator evidence before upgrade or rollback decisions
    - Existing recovery vocabulary reused for upgrade compatibility decisions

key-files:
  created:
    - docs/parity/upgrade-and-rollback-policy.md
    - .planning/phases/84-upgrade-and-rollback-policy/84-01-SUMMARY.md
  modified: []

key-decisions:
  - "Keep Phase 84 upgrade and rollback guidance source-built and local-first."
  - "Use existing recovery categories, causes, and action classes instead of new upgrade-specific labels."
  - "Forbid hidden mutation and keep destructive repair deferred."

patterns-established:
  - "Upgrade policy rows classify evidence before action."
  - "Rollback guidance preserves exact datadir/config paths and repo-local commands."

requirements-completed: [UPG-01, UPG-02, UPG-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 84-2026-06-21T21-33-46
generated_at: 2026-06-22T00:23:49Z

duration: 4 min
completed: 2026-06-22
---

# Phase 84 Plan 01: Upgrade And Rollback Policy Summary

**Source-built upgrade policy with pre-upgrade evidence, recovery-vocabulary compatibility tables, and rollback boundaries that forbid hidden mutation.**

## Performance

- **Duration:** 4 min
- **Started:** 2026-06-22T00:19:37Z
- **Completed:** 2026-06-22T00:23:49Z
- **Tasks:** 3
- **Files modified:** 1

## Accomplishments

- Created `docs/parity/upgrade-and-rollback-policy.md` with surface id `v1-8-upgrade-rollback-policy`.
- Added the complete UPG-01 pre-upgrade checklist with required Cargo and Bazel command forms.
- Added UPG-02 state/schema compatibility guidance using `safe_retry`, `read_only_inspection`, `backup_then_rebuild`, and `stop_and_escalate`.
- Added UPG-03 failed-upgrade and rollback guidance that forbids hidden mutation and keeps destructive repair deferred.

## Task Commits

Each task was committed atomically:

1. **Task 1: Create policy shell and pre-upgrade checklist** - `fff382c` (docs)
2. **Task 2: Add state and schema compatibility decision tables** - `74c36b6` (docs)
3. **Task 3: Add failed-upgrade, rollback, and deferred mutation boundaries** - `86d0aec` (docs)

**Plan metadata:** committed separately after summary self-check.

## Files Created/Modified

- `docs/parity/upgrade-and-rollback-policy.md` - Canonical Phase 84 upgrade and rollback policy.
- `.planning/phases/84-upgrade-and-rollback-policy/84-01-SUMMARY.md` - Execution summary and self-check artifact.

## Decisions Made

- Used the existing Phase 82 support terms and Phase 77 recovery vocabulary instead of introducing upgrade-specific labels.
- Kept rollback source-built and local-first, with exact datadir/config path reuse.
- Treated source datadirs, external wallets, service files, launchd/systemd state, `bitcoin.conf`, and Open Bitcoin JSONC config as future-scoped mutation surfaces.

## Deviations from Plan

None - plan executed exactly as written.

## Known Stubs

None - the policy text is complete for UPG-01, UPG-02, and UPG-03; UPG-04 remains assigned to later Phase 84 verifier work.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Verification

- `rg -n 'Surface id: \`v1-8-upgrade-rollback-policy\`|## Pre-Upgrade Checklist|current source revision or commit|repo-local verification status|binary provenance from Cargo or Bazel|Open Bitcoin JSONC config path|bitcoin.conf path|selected datadir|datadir ownership and free-space review|current sync/status evidence|support-bundle evidence when available|service state|wallet scope|backup location' docs/parity/upgrade-and-rollback-policy.md`
- `rg -n 'cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=&lt;path&gt; status --format json|bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=&lt;path&gt; status --format json|bash scripts/verify.sh|review-only evidence' docs/parity/upgrade-and-rollback-policy.md`
- `rg -n '## State And Schema Compatibility Decision Table|clean_shutdown|unclean_shutdown|storage_lock_contention|incompatible_schema|schema_mismatch|store_corruption|corruption_marker|corrupt_record|partial_write|unreadable_namespace|backend_open_failure|safe_retry|read_only_inspection|backup_then_rebuild|stop_and_escalate' docs/parity/upgrade-and-rollback-policy.md`
- `rg -n '## Evidence That Is Not Sufficient|daemon startup|elapsed time|peer reachability|raw logs|report existence alone|Unavailable: &lt;reason&gt;|Open Bitcoin-owned durable store state|external Core/Knots source datadirs and wallets' docs/parity/upgrade-and-rollback-policy.md`
- `rg -n '## Failed Upgrade Guidance|stop the attempted upgraded process|record exact command and commit|collect redacted local evidence|preserve backups|avoid repeated mutation until the compatibility class is understood|## Rollback Guidance|return to the previous checked-out source revision or known binary|use the same explicit datadir and config paths|verify with repo-local commands|record rollback evidence' docs/parity/upgrade-and-rollback-policy.md`
- `rg -n 'package-manager rollback|signed release channels|automatic update behavior|Phase 84 does not recommend hidden mutation of source datadirs, external wallets, service files, launchd/systemd state, bitcoin.conf, or Open Bitcoin JSONC config.|Destructive repair remains deferred.|backup_then_rebuild is evidence and operator-decision guidance, not permission for automated destructive rebuild or repair.' docs/parity/upgrade-and-rollback-policy.md`
- `rg -n 'v1-8-upgrade-rollback-policy|UPG-01|UPG-02|UPG-03|UPG-04|Phase 84 does not recommend hidden mutation' docs/parity/upgrade-and-rollback-policy.md`
- `git diff --check -- docs/parity/upgrade-and-rollback-policy.md`

## Next Phase Readiness

Ready for 84-02 to add canonical links and metadata without duplicating this policy.

## Self-Check: PASSED

- Found `docs/parity/upgrade-and-rollback-policy.md`.
- Found `.planning/phases/84-upgrade-and-rollback-policy/84-01-SUMMARY.md`.
- Found task commits `fff382c`, `74c36b6`, and `86d0aec`.
- `git diff --check -- .planning/phases/84-upgrade-and-rollback-policy/84-01-SUMMARY.md` passed.

---
*Phase: 84-upgrade-and-rollback-policy*
*Completed: 2026-06-22*
