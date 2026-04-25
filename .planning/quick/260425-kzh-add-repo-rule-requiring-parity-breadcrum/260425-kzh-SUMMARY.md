# Quick Task 260425-kzh: Require Parity Breadcrumbs For New Rust Files - Summary

**Date:** 2026-04-25
**Status:** Complete
**Work commit:** `d4e136d`

## What Changed

- Added a repo-local `AGENTS.md` rule requiring new first-party Rust source and test files to use the parity breadcrumb convention.
- Pointed contributors at `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts` as the source of truth and enforcement path.
- Kept the explicit `none` breadcrumb available only for files without a defensible Bitcoin Knots source anchor.

## Verification

- `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 133 Rust files.
- `git diff --check` passed.
- `bash scripts/verify.sh` passed before commit.
- The pre-commit hook reran `bash scripts/verify.sh` successfully before creating `d4e136d`.

## Residual Risk

- This is a contributor-facing rule plus existing automated enforcement; files outside the checker's first-party Rust scope still depend on separate repo guidance.
