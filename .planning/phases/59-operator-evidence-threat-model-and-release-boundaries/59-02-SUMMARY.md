---
phase: 59-operator-evidence-threat-model-and-release-boundaries
plan: 02
subsystem: operator-evidence
tags: [rust, operator-cli, support-bundle, live-smoke, redaction]

requires:
  - phase: 58-same-datadir-restart-and-resume-evidence
    provides: schema v2 restart/resume live-smoke evidence fields
  - phase: 59-operator-evidence-threat-model-and-release-boundaries
    provides: OBS-01 shared operator evidence consistency from Plan 59-01
provides:
  - OBS-02 redacted support-bundle projection for v1.4 live-smoke first-header, first-block, restart/resume, recovery, peer-outcome, and final-status evidence
  - Allowlisted local live-smoke summary module with manual-peer fallback removal
  - Markdown labels for compact v1.4 support evidence
affects: [operator-support-bundles, live-smoke-reports, parity-breadcrumbs, release-evidence]

tech-stack:
  added: []
  patterns:
    - Purpose-built allowlist projection from local live-smoke JSON into support evidence
    - Raw report fields stay excluded while unavailable local evidence remains visible

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/support/live_smoke.rs
    - .planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-02-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-cli/src/operator/support/render.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Live-smoke support evidence is now extracted through a dedicated allowlist module instead of helper logic embedded in support.rs."
  - "Legacy top-level live-smoke fallback no longer copies manualPeers/manual_peers into support bundles."
  - "Task commits were deferred to the final strict yolo push gate per wrapper instructions."

patterns-established:
  - "Schema v2 support summaries keep firstHeaderProgress, firstBlockProgress, restartResumeEvidence, and finalStatus as compact summarized JSON objects."
  - "Support-bundle redaction tests include raw daemon tails, endpoint tables, manual peers, cookies, wallet-like text, snapshots, and forbidden markers in the input fixture."

requirements-completed: [OBS-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 59-2026-06-05T15-10-59
generated_at: 2026-06-05T16:10:49Z

duration: 7min
completed: 2026-06-05
---

# Phase 59 Plan 02: Support Evidence Projection Summary

**Redacted support bundles now expose compact v1.4 live-smoke progress, restart, recovery, peer-outcome, and final-status evidence without embedding raw local report artifacts.**

## Performance

- **Duration:** 7 min
- **Started:** 2026-06-05T16:03:29Z
- **Completed:** 2026-06-05T16:10:49Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Extracted live-smoke support projection into `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` with the required Open Bitcoin-only parity breadcrumb.
- Added compact allowlisted summaries for `firstHeaderProgress`, `firstBlockProgress`, `restartResumeEvidence`, `recoveryDiagnosis`, `peerOutcomeSummary`, and top-level `final_status` as `finalStatus`.
- Removed `manualPeers` and `manual_peers` from the legacy top-level fallback while preserving status/no-progress/next-action compatibility.
- Extended Markdown rendering with `First header progress`, `First block progress`, `Restart/resume evidence`, `Recovery diagnosis`, and `Final status`.
- Replaced the schema-v2 support test with a v1.4 fixture that proves raw daemon tails, endpoint tables, snapshots, manual peer lists, cookies, wallet-like material, raw endpoint addresses, and secret markers stay out of JSON and Markdown.

## Task Commits

Task commits were deferred to the final strict yolo push gate per wrapper instructions. No staging, commits, or pushes were performed by this executor.

1. **Task 1: Extract allowlisted live-smoke support projection** - deferred
2. **Task 2: Render and test compact v1.4 support evidence** - deferred

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` - new allowlisted live-smoke support projection module.
- `packages/open-bitcoin-cli/src/operator/support.rs` - delegates live-smoke summaries to the new module and updates redaction-summary wording.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - renders compact v1.4 live-smoke evidence labels.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - adds v1.4 support-bundle fixture coverage and manual-peer fallback regression coverage.
- `docs/parity/source-breadcrumbs.json` - registers the new support projection source file in the Open Bitcoin-only support group.
- `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-02-SUMMARY.md` - created this execution summary.

## Decisions Made

- Kept live-smoke support extraction as pure JSON projection logic; support command file I/O remains in `support.rs`.
- Preserved summarized JSON objects in Markdown instead of expanding raw live-smoke report tables.
- Did not update `.planning/STATE.md`, `.planning/ROADMAP.md`, or `.planning/REQUIREMENTS.md` because the wrapper constrained ownership to the plan file set plus this summary.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Removed forbidden raw credential key spelling from redaction metadata**
- **Found during:** Task 2
- **Issue:** The strengthened v1.4 redaction test required support artifacts to omit the literal `rpcpassword` marker, but the bundle's redaction summary used that raw key spelling while explaining omitted credential values.
- **Fix:** Reworded the redaction summary to say `RPC password and RPC auth values`, preserving the safeguard without echoing the forbidden raw key.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle_summarizes_v1_4_live_smoke_evidence --all-features`
- **Committed in:** deferred to final strict yolo push gate

***

**Total deviations:** 1 auto-fixed (1 missing critical)
**Impact on plan:** The change tightens the intended redaction boundary without changing support-bundle semantics or adding scope.

## Issues Encountered

- Expected TDD RED failures occurred before implementation: the top-level fallback leaked manual peer keys, and Markdown initially lacked the new v1.4 evidence labels.
- `bash scripts/verify.sh` was not run because this wrapper owns only the plan file set plus summary and reserves final repo-wide verification for the strict yolo push gate.

## Verification

Passed:

- `cargo fmt --manifest-path packages/Cargo.toml --all --check`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle_preserves_top_level_live_smoke_fallback --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle_summarizes_v1_4_live_smoke_evidence --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle_keeps_missing_live_smoke_report_unavailable --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle --all-features`
- `bash -lc '! rg -n "run-live-mainnet-smoke|--restart-after-progress" scripts/verify.sh'`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-targets --all-features`
- `git diff --check -- packages/open-bitcoin-cli/src/operator/support.rs packages/open-bitcoin-cli/src/operator/support/live_smoke.rs packages/open-bitcoin-cli/src/operator/support/render.rs packages/open-bitcoin-cli/tests/operator_binary.rs docs/parity/source-breadcrumbs.json`

Acceptance scans passed for all plan-specified `rg` checks.

## Known Stubs

None. Stub-pattern scan found only intentional JSON fixture `null` values, existing empty breadcrumb arrays, and formatting strings; no UI or operator-output stub was introduced.

## Threat Flags

None. The new local live-smoke report trust-boundary logic is the planned T-59-02 allowlist mitigation; no unplanned network endpoint, auth path, file-access surface, or schema boundary was introduced.

## User Setup Required

None.

## Next Phase Readiness

OBS-02 support-bundle projection is ready for the later Phase 59 docs, threat-model, release-boundary, and final verification plans. Public-network evidence remains opt-in and outside default verification.

## Self-Check: PASSED

- Found summary file at `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-02-SUMMARY.md`.
- Found new source file at `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs`.
- Found lifecycle frontmatter and deferred-task-commit note in this summary.
- Found `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` registered in `docs/parity/source-breadcrumbs.json`.
- Commit self-check intentionally skipped because the wrapper requires no staging, commits, or pushes in this executor; task and metadata commits are deferred to the final strict yolo push gate.

***
*Phase: 59-operator-evidence-threat-model-and-release-boundaries*
*Completed: 2026-06-05*
