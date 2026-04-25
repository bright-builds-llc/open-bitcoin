---
quick_id: 260425-c8x
description: Migrate LOC generator to TypeScript and Bun
completed: 2026-04-25
status: completed
commit: e8d1055
---

# Quick Task 260425-c8x Summary

## Outcome

Migrated the LOC report generator from `scripts/generate-loc-report.mjs` to `scripts/generate-loc-report.ts` and made Bun the canonical runtime for repo-owned higher-level automation scripts.

Repo-owned verification paths now use Bun for LOC report generation, report freshness checks, timing helpers, and Cargo metadata parsing. The generated LOC report now records the Bun command and classifies the generator as a TypeScript/Bun script.

## Files Changed

- `.githooks/pre-commit`
- `AGENTS.md`
- `docs/metrics/lines-of-code.md`
- `scripts/check-pure-core-deps.sh`
- `scripts/generate-loc-report.ts`
- `scripts/test-check-file-lengths.sh`
- `scripts/test-generate-loc-report.sh`
- `scripts/verify.sh`

## Verification

- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check`
- `bun run scripts/generate-loc-report.ts --source=index --output=docs/metrics/lines-of-code.md --check`
- `bash scripts/test-generate-loc-report.sh`
- `bash scripts/test-check-file-lengths.sh`
- `bash scripts/check-pure-core-deps.sh`
- `shfmt -i 2 -ci -d .githooks/pre-commit scripts/verify.sh scripts/check-pure-core-deps.sh scripts/test-generate-loc-report.sh scripts/test-check-file-lengths.sh`
- `shellcheck .githooks/pre-commit scripts/verify.sh scripts/check-pure-core-deps.sh scripts/test-generate-loc-report.sh scripts/test-check-file-lengths.sh`
- `mdformat --check AGENTS.md docs/metrics/lines-of-code.md`
- `git diff --cached --check`
- `bash scripts/verify.sh`

## Residual Risks

- The TypeScript script is executed directly by Bun without a package-level `tsc` typecheck gate by design.
