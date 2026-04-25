---
quick_id: 260425-avx
description: Add deterministic LOC report generator and wire it into pre-commit/verify
created: 2026-04-25
status: completed
---

# Quick Task 260425-avx Plan

## Goal

Create a deterministic first-party lines-of-code report generator, commit its Markdown output under `docs/metrics/`, and enforce freshness through both the repo-managed pre-commit hook and `scripts/verify.sh`.

## Tasks

### 1. Add LOC generator and generated report

- Files: `scripts/generate-loc-report.mjs`, `docs/metrics/lines-of-code.md`
- Action: Implement a dependency-free Node CLI with `--source=worktree|index`, `--output`, and `--check`; count first-party code/tooling while excluding vendored Knots, generated output, `.planning`, docs, and the report itself.
- Verify: Run the generator in worktree mode and confirm the report is deterministic and contains aggregate, crate, category, largest-file, and metadata sections.
- Done: `docs/metrics/lines-of-code.md` is generated from the current first-party tree.

### 2. Wire freshness into hooks and verification

- Files: `.githooks/pre-commit`, `scripts/verify.sh`
- Action: Regenerate and stage the report from the git index before pre-commit verification; add a `--check` invocation to `scripts/verify.sh`, defaulting to worktree mode with an env override for pre-commit index mode.
- Verify: Confirm `scripts/verify.sh` fails when the report is stale and passes when current.
- Done: Commits auto-include the staged-index report, while CI/manual verification rejects stale output.

### 3. Add script regression tests

- Files: `scripts/test-generate-loc-report.sh`
- Action: Add temp-repo tests covering scope exclusions, staged-index semantics, stale-check behavior, and required Markdown sections.
- Verify: Run `bash scripts/test-generate-loc-report.sh`.
- Done: Script behavior is covered without requiring external packages.

## Final Verification

- `bash scripts/test-generate-loc-report.sh`
- `bash scripts/test-check-file-lengths.sh`
- `bash scripts/verify.sh`
