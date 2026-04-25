# Quick Task 260425-e1c: Fix CI Bun provisioning - Summary

**Date:** 2026-04-25
**Status:** Complete
**Work commit:** `e51c539`

## What Changed

- Added `.bun-version` with Bun `1.3.9` as the pinned runtime source of truth.
- Updated GitHub CI to install Bun with `oven-sh/setup-bun@v2` from
  `.bun-version` before running `bash scripts/verify.sh`.
- Added a CI `bun --version` check before repo verification.
- Documented the local Bun prerequisite in `CONTRIBUTING.md`.
- Refreshed `docs/metrics/lines-of-code.md` for the CI workflow change.

## Verification

- `bun --version` matched `.bun-version` (`1.3.9`).
- `git diff --check` passed.
- `bash scripts/verify.sh` passed.
- The repo-managed pre-commit hook reran `bash scripts/verify.sh` successfully
  before creating `e51c539`.

## Residual Risk

- CI still needs to run on GitHub Actions to confirm runner-side Bun setup, but
  the workflow now provisions Bun before the existing verification step.
