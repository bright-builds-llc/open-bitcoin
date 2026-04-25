# Quick Task 260425-csn: Refresh README parity status and docs freshness guidance - Summary

**Date:** 2026-04-25
**Status:** Complete
**Work commit:** `37202da`

## What Changed

- Rewrote the root README around the parity-ledger source of truth, including a near-top parity status matrix, Open Bitcoin differentiators, operator-preview commands, future-work summary, and updated parity links.
- Refreshed `packages/README.md`, `docs/parity/README.md`, and `docs/parity/catalog/README.md` to match the current crate layout and ledger status.
- Added repo-local guidance in `AGENTS.md` to check relevant README files after substantial feature, parity, operator-surface, or workflow changes.

## Verification

- `rg -n "Phase 1|scaffold|not yet runnable|planned" README.md packages/README.md docs/parity/README.md docs/parity/catalog/README.md` returned no matches.
- `git diff --check` passed.
- `bash scripts/verify.sh` passed.
- Commit hook reran `bash scripts/verify.sh` successfully before creating `37202da`.

## Residual Risk

- `.planning/REQUIREMENTS.md` and `.planning/ROADMAP.md` still contain older status rows by design; the README now cites the parity ledger and `.planning/STATE.md` as current status sources.
