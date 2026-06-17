---
phase: 79-diagnostics-and-support-bundle-forensics
plan: 01
subsystem: cli-operator-support
tags: [support-bundle, forensics, soak-ledger, redaction, rust]

requires:
  - phase: 75-soak-runner-and-evidence-ledger
    provides: typed soak ledger events and support bundle soak evidence
provides:
  - typed support_forensics JSON sidecar for support bundles
  - deterministic soak ledger checkpoint-chain evidence
  - redacted forensic narrative derived from typed soak evidence
affects: [support-bundle, soak-evidence, diagnostics, parity-breadcrumbs]

tech-stack:
  added: []
  patterns:
    - pure projection module fed by typed evidence
    - pub(super) support sidecar fields for sibling render/test modules
    - deterministic sha256-json-v1 evidence chain without authenticity claims

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/support/forensics.rs
    - packages/open-bitcoin-cli/src/operator/support/soak_evidence.rs
  modified:
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Forensic evidence is derived only from typed soak ledger, report, and redaction inputs."
  - "The checkpoint chain uses sha256-json-v1 as deterministic ordering/truncation evidence only, not an authenticity mechanism."
  - "Soak support collection moved to a sibling module to keep support.rs below the file-length gate."

patterns-established:
  - "Support sidecars should keep projection logic in focused sibling modules and expose pub(super) fields only where renderers/tests need typed access."
  - "Sensitive support evidence is represented through redaction metadata rather than raw source strings."

requirements-completed: [DIAG-01, DIAG-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 79-2026-06-17T13-53-04
generated_at: 2026-06-17T17:04:17Z

duration: 1h49m
completed: 2026-06-17
---

# Phase 79-01: Support Bundle Forensics Sidecar Summary

**Redacted support_forensics sidecar with typed timeline, checkpoint-chain, source, redaction, and narrative evidence**

## Performance

- **Duration:** 1h49m
- **Started:** 2026-06-17T15:15:24Z
- **Completed:** 2026-06-17T17:04:17Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added `support_forensics` to support bundle JSON with `timeline`, `checkpoint_chain`, `narrative`, `source`, `redaction`, and unavailable-state evidence.
- Implemented `sha256-json-v1` checkpoint-chain evidence for ledger ordering gaps and trailing partial bytes without claiming authenticity.
- Covered available, unavailable, sequence-gap, truncation, JSON contract, and sensitive-material redaction behavior with Phase 79 tests.
- Registered new support modules in the parity breadcrumb map and refreshed the tracked LOC artifact.

## Task Commits

1. **Task 1: Add pure support-forensics projection contract** - `775b9b6` (feat)
2. **Task 2: Wire support_forensics into bundle JSON collection** - `775b9b6` (feat)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/support/forensics.rs` - Pure typed forensics projection and narrative logic.
- `packages/open-bitcoin-cli/src/operator/support/soak_evidence.rs` - Extracted soak support evidence collection used by support bundles.
- `packages/open-bitcoin-cli/src/operator/support.rs` - Support bundle JSON now includes `support_forensics`.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Phase 79 sidecar, chain, fallback, and redaction tests.
- `docs/parity/source-breadcrumbs.json` - Breadcrumb registration for the new support modules.
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC report.

## Decisions Made

- Used `open_bitcoin_node::core::consensus::crypto::Sha256` instead of adding a dependency.
- Kept the projection data-in/data-out: no filesystem reads, clock access, raw log parsing, or renderer text parsing.
- Used conservative unavailable evidence with `collection_failed`, low confidence, and no inferred root cause when the ledger is missing.

## Deviations from Plan

### Auto-fixed Issues

**1. Production file-length gate required extracting soak evidence**
- **Found during:** Task 2 commit hook.
- **Issue:** Adding the sidecar wiring pushed `packages/open-bitcoin-cli/src/operator/support.rs` over the production file-length limit.
- **Fix:** Moved existing soak support collection types and logic into `packages/open-bitcoin-cli/src/operator/support/soak_evidence.rs` and registered the new file in parity breadcrumbs.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support.rs`, `packages/open-bitcoin-cli/src/operator/support/soak_evidence.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `bash scripts/check-file-lengths.sh`, `bun run scripts/check-parity-breadcrumbs.ts --check`, and the commit hook's full `bash scripts/verify.sh` all passed.
- **Committed in:** `775b9b6`

**Total deviations:** 1 auto-fixed file-shape issue.
**Impact on plan:** No behavior scope change; the extraction preserves the planned support bundle contract and reduces module size.

## Issues Encountered

- Initial commit hook failed on two Clippy findings. Fixed a collapsible `if` in the checkpoint walk and replaced a boolean `assert_eq!` with `assert!`.
- One focused parallel Cargo test run stopped producing output after the test binary launched. It was stopped and rerun single-threaded with `--nocapture`; all five Phase 79 tests passed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 79-02 can render the new `support_forensics` fields because the sidecar fields are `pub(super)` and the bundle JSON contract is present. The remaining work should use the typed sidecar rather than re-parsing support JSON or raw soak logs.

---
*Phase: 79-diagnostics-and-support-bundle-forensics*
*Completed: 2026-06-17*
