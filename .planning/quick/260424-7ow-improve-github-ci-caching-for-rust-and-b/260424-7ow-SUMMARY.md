# Quick Task 260424-7ow: Improve GitHub CI Caching For Rust And Bazel Verification

**Date:** 2026-04-24
**Status:** Completed
**Implementation commit:** c8a16bd

## Changes

- Added `Swatinem/rust-cache@v2` to the GitHub CI workflow with the repository's
  `packages -> target` Cargo workspace mapping.
- Restricted Rust cache saves to pushes on `main` and disabled `~/.cargo/bin`
  caching so installed CI tools remain action-owned.
- Enabled Bazelisk, repository, and disk caching through the existing
  `bazel-contrib/setup-bazel@0.19.0` step with the same push-to-main save
  policy.

## Verification

- Passed: `actionlint .github/workflows/ci.yml`
- Passed: `bash scripts/verify.sh`
- Passed: `git diff --check`
- Passed again in the implementation commit pre-commit hook: `bash scripts/verify.sh`
