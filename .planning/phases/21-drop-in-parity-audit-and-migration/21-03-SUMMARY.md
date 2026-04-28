---
phase: 21-drop-in-parity-audit-and-migration
plan: "03"
subsystem: migration-parity-audit-and-docs
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 21-2026-04-27T23-11-20
generated_at: 2026-04-28T00:35:35Z
tags:
  - migration
  - parity
  - docs
  - audit
  - ledger
key_files:
  created:
    - docs/parity/catalog/drop-in-audit-and-migration.md
  modified:
    - docs/parity/catalog/README.md
    - docs/parity/README.md
    - docs/parity/checklist.md
    - docs/parity/deviations-and-unknowns.md
    - docs/parity/index.json
    - docs/architecture/cli-command-architecture.md
    - README.md
metrics:
  completed_date: "2026-04-27"
  files_created: 1
  files_modified: 7
---

# Phase 21 Plan 03 Summary

## One-Liner

The repository now has an auditable Phase 21 drop-in migration matrix, machine-readable migration deviation records, and contributor-facing docs that point to the new dry-run planner without claiming full replacement parity.

## What Was Built

- Added `docs/parity/catalog/drop-in-audit-and-migration.md` as the dedicated
  Phase 21 audit matrix for:
  - CLI and operator command surfaces
  - RPC/config expectations
  - datadir and cookie handling
  - service behavior
  - wallet behavior
  - sync/logging and operator-doc boundaries
- Registered the new audit surface in `docs/parity/index.json` and the human
  parity catalog/README/checklist views.
- Added migration-relevant intentional deviation records to the parity ledger
  and the human-readable `docs/parity/deviations-and-unknowns.md` rollup.
- Refreshed contributor docs so the shipped migration surface is explicit:
  - `docs/architecture/cli-command-architecture.md` now includes `migrate`
  - `README.md` points to `open-bitcoin migrate plan`
  - wording stays evidence-scoped and avoids a premature full drop-in claim

## Task Commits

1. **Task 1 and Task 2: add the Phase 21 audit matrix and contributor doc pointers** — `4a2087f` `feat(21): add migration dry-run planner and parity audit`

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_flows -- --nocapture`
- `rg -n "migrate|drop-in" README.md docs/architecture/cli-command-architecture.md docs/parity/catalog/drop-in-audit-and-migration.md docs/parity/index.json`
- `bash scripts/verify.sh`

## Deviations from Plan

- The final closeout also refreshed the machine-readable checklist and broader
  parity rollup files so the new audit surface remained consistent everywhere
  the ledger is consumed.

## Self-Check: PASSED

- The new migration audit page is discoverable from the parity ledger root and
  catalog index.
- Contributor docs point to the migration planner and audit page without
  overstating parity scope.
