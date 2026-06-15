---
phase: 75-multi-day-soak-runner-and-evidence-ledger
plan: 04
subsystem: operator-support-evidence
tags: [rust, operator-cli, support-bundle, soak, redaction]

requires:
  - phase: 75-multi-day-soak-runner-and-evidence-ledger
    provides: Plan 75-01 datadir-owned soak ledger and report projection contracts
  - phase: 75-multi-day-soak-runner-and-evidence-ledger
    provides: Plan 75-02 operator-facing soak command writes latest run artifacts
provides:
  - Compact redacted soak support evidence in support bundle JSON
  - Support Markdown soak evidence section with run id, outcome, sequence, and source paths
  - Unit and binary coverage for available, unavailable, and raw-local-evidence exclusion behavior
affects: [phase-75, operator-cli, support-evidence, soak-ledger]

tech-stack:
  added: []
  patterns:
    - Support bundles derive soak summaries from the selected datadir run index and ledger only
    - Shareable support output uses allowlisted projection fields instead of raw ledger or report bodies

key-files:
  created:
    - .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-04-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-cli/src/operator/support/render.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs

key-decisions:
  - "Treat absent datadir, missing run index, empty latest run, unreadable ledger, malformed ledger, and mismatched index ledger path as unavailable soak evidence."
  - "Expose report paths in support output without reading raw JSON or Markdown report bodies."

patterns-established:
  - "Support soak summary fields are `state`, `maybe_run_id`, `maybe_final_outcome`, `maybe_latest_sequence`, source ledger path, report paths, and unavailable reason."
  - "Support Markdown renders `## Soak Evidence` with stable labels for operator review."

requirements-completed: [SOAK-01, SOAK-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 75-2026-06-14T22-59-23
generated_at: 2026-06-15T02:39:34Z

duration: 18 min
completed: 2026-06-15
---

# Phase 75 Plan 04: Support Bundle Soak Summary Projection

**Support bundles now include compact ledger-derived soak evidence while excluding raw ledger, report, log, credential, wallet, and peer-table material.**

## Performance

- **Duration:** 18 min
- **Started:** 2026-06-15T02:21:33Z
- **Completed:** 2026-06-15T02:39:34Z
- **Tasks:** 1
- **Files modified:** 5

## Accomplishments

- Added `SoakSupportEvidence` to support bundle JSON, derived from the selected datadir's latest indexed soak ledger.
- Added a `## Soak Evidence` Markdown section with stable state, run, outcome, source ledger, report path, and latest sequence labels.
- Added unit and binary tests for available evidence, unavailable evidence, and redaction boundaries.

## Task Commits

TDD produced two commits for the single task:

1. **RED: Add failing soak support summary tests** - `fec36fd` (`test`)
2. **GREEN: Add soak support summary projection** - `3ae4a16` (`feat`)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/support.rs` - Adds compact soak evidence collection from datadir run index and ledger projection.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Renders the compact soak evidence section in support Markdown.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Adds unit coverage for available, unavailable, and redacted soak support evidence.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Adds binary support bundle coverage for Phase 75 soak summary output.
- `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-04-SUMMARY.md` - Records this plan outcome.

## Decisions Made

- Missing or unreadable soak evidence returns `state = "unavailable"` with reason `soak ledger unavailable`; the support bundle does not infer state from stale report files.
- The collector rejects a latest run whose indexed ledger path does not match the selected datadir-owned run layout.
- Report paths are exposed as metadata only; raw report bodies are not read into support evidence.

## Verification

- RED: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib phase75_soak_support_ --all-features` failed on missing `collect_soak_support_evidence`, `soak_outcome_label`, and `soak_evidence`.
- `cargo fmt --manifest-path packages/Cargo.toml --all` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib phase75_soak_support_ --all-features` passed: 4 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_support_bundle_includes_phase75_soak_summary --all-features` passed: 1 test.
- Acceptance `rg` checks for support symbols, Markdown labels, test names, and redaction assertion terms passed.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- The first binary GREEN run hardcoded a latest sequence of `5`, while the actual soak runner fixture produced `8`; the test was corrected to assert the sequence reported in support JSON is rendered consistently in Markdown.
- Running the two focused Cargo checks in parallel caused temporary Cargo lock waits; both checks completed successfully after waiting.

## Known Stubs

None. Stub-pattern scan only matched existing format-string and test-fixture credential literals, not introduced placeholder behavior or unwired data.

## Threat Flags

None - the new soak-ledger-to-support trust boundary was covered by the plan threat model. The implementation reads only the selected datadir run index and expected latest run ledger path, and support output stays allowlisted.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 75-05. Support bundles can now surface compact soak evidence without becoming the ledger source of truth or carrying raw local artifacts.

## Self-Check: PASSED

- Summary file exists: `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-04-SUMMARY.md`
- Task commit exists: `fec36fd`
- Task commit exists: `3ae4a16`

---
*Phase: 75-multi-day-soak-runner-and-evidence-ledger*
*Completed: 2026-06-15*
