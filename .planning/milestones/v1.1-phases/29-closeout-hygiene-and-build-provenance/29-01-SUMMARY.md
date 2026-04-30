---
phase: 29-closeout-hygiene-and-build-provenance
plan: 01
status: completed
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 29-2026-04-29T14-01-32
generated_at: 2026-04-29T14:14:38Z
---

# Phase 29 Plan 01 Summary

## One-Liner

The Bazel build path now stamps truthful workspace version, commit, build-time,
target, and profile metadata into the shared CLI status collector instead of
falling back to `0.0.0` plus all-`Unavailable` build provenance.

## What Was Built

- Added `.bazelrc` workspace-status wiring so repo-local Bazel builds can read a
  checked-in status script.
- Added `scripts/open-bitcoin-workspace-status.sh` to expose the current Git
  commit as stable Bazel workspace metadata.
- Updated `packages/open-bitcoin-cli/BUILD.bazel` so `open_bitcoin_cli_lib`
  carries:
  - `version = "0.1.0"`
  - stamped commit and build-time metadata
  - Bazel `TARGET_CPU` and `COMPILATION_MODE` provenance
- Added `scripts/check-bazel-build-provenance.ts`, a focused Bun checker that
  runs the Bazel-built `open-bitcoin` binary and verifies the live status JSON
  build section against the current workspace version, Git HEAD, and Bazel
  build-mode metadata.
- Wired that focused Bazel provenance check into `bash scripts/verify.sh`.

## Task Commits

1. **Task 1: restore truthful Bazel build provenance** — Pending the
   wrapper-owned Phase 29 finalization commit.

## Verification

Passed:

- `bun run scripts/check-bazel-build-provenance.ts`

## Deviations from Plan

- None. The focused runtime checker was sufficient, so this plan did not need
  additional Rust-only unit tests beyond the existing shared status coverage.
