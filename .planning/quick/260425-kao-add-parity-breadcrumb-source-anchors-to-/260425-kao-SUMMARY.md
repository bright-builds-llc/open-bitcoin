# Quick Task 260425-kao: Parity Breadcrumb Source Anchors - Summary

**Date:** 2026-04-25
**Status:** Complete
**Work commit:** `d2b67e3`

## What Changed

- Added parity breadcrumb comments to all 133 first-party Rust source and test files under `packages/open-bitcoin-*/src` and `packages/open-bitcoin-*/tests`.
- Added `docs/parity/source-breadcrumbs.json` as the machine-readable breadcrumb source of truth.
- Added `scripts/check-parity-breadcrumbs.ts` with `--write` and `--check` modes, and wired the check into `scripts/verify.sh`.
- Added a local VS Code-compatible extension under `.vscode/extensions/open-bitcoin-parity-breadcrumb-links/` so VS Code/Cursor can resolve `packages/bitcoin-knots/...` comment paths through a `DocumentLinkProvider`.
- Updated `docs/parity/README.md` and regenerated `docs/metrics/lines-of-code.md`.

## Verification

- `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 133 Rust files.
- `cargo fmt --manifest-path packages/Cargo.toml --all --check` passed.
- `node --check .vscode/extensions/open-bitcoin-parity-breadcrumb-links/extension.js` passed.
- `git diff --check` passed.
- `bash scripts/verify.sh` passed before commit.
- The pre-commit hook reran `bash scripts/verify.sh` successfully before creating `d2b67e3`.

## Residual Risk

- Raw repo-relative source-comment paths are not a documented default VS Code/Cursor link contract, so clickability depends on enabling the checked-in local workspace extension.
