---
quick_id: 260425-avx
description: Add deterministic LOC report generator and wire it into pre-commit/verify
completed: 2026-04-25
status: completed
commit: 9b09df8
---

# Quick Task 260425-avx Summary

## Outcome

Added a deterministic, dependency-free Node LOC report generator and checked in the generated report at `docs/metrics/lines-of-code.md`.

The pre-commit hook now regenerates the report from the staged index, stages the report, and runs `scripts/verify.sh` in index mode. Manual and CI verification use `scripts/verify.sh`, which checks the report in worktree mode by default.

## Files Changed

- `.githooks/pre-commit`
- `docs/metrics/lines-of-code.md`
- `scripts/generate-loc-report.mjs`
- `scripts/test-generate-loc-report.sh`
- `scripts/verify.sh`

## Verification

- `node --check scripts/generate-loc-report.mjs`
- `shfmt -i 2 -ci -d .githooks/pre-commit scripts/verify.sh scripts/test-generate-loc-report.sh`
- `shellcheck .githooks/pre-commit scripts/verify.sh scripts/test-generate-loc-report.sh`
- `mdformat --check docs/metrics/lines-of-code.md`
- `bash scripts/test-generate-loc-report.sh`
- `node scripts/generate-loc-report.mjs --source=index --output=docs/metrics/lines-of-code.md --check`
- `node scripts/generate-loc-report.mjs --source=worktree --output=docs/metrics/lines-of-code.md --check`
- `git diff --cached --check`
- `bash scripts/verify.sh`

## Residual Risks

- Comment-only LOC is detectable for common first-party formats and should be treated as an estimate for mixed or uncommon file syntaxes.
