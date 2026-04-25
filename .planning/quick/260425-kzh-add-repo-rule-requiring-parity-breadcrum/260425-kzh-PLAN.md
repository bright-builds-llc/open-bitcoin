# Quick Task 260425-kzh: Require Parity Breadcrumbs For New Rust Files

Created: 2026-04-25 15:06 CDT

## Goal

Add a repo-local contributor rule so new first-party Rust source and test files keep the Bitcoin Knots parity breadcrumb convention current.

## Plan

- Add a narrow `AGENTS.md` repo-local guidance bullet outside the managed Bright Builds block.
- Point the rule at `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`.
- Verify the docs change and breadcrumb checker still pass.

## Verification

- Run `bun run scripts/check-parity-breadcrumbs.ts --check`.
- Run a lightweight diff/status review.
