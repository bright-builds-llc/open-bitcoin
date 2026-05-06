---
phase: 24-wallet-aware-live-status-and-build-provenance
plan: "04"
subsystem: build-provenance
requirements-completed: [OBS-01, OBS-02, DASH-01]
generated_by: gsd-execute-phase
lifecycle_mode: gap_closure
phase_lifecycle_id: 24-uat-gap-build-time-format
generated_at: 2026-05-06T02:19:14Z
completed: 2026-05-06
---

# Phase 24 Plan 04 Summary

## One-Liner

Build-time provenance now reports ISO-8601 UTC timestamps consistently across
Cargo and Bazel `status --format json` output, with a Bazel checker regression
guard covering the operator-facing JSON contract.

## What Was Built

- Normalized Cargo `SOURCE_DATE_EPOCH` build provenance to ISO-8601 UTC instead
  of exposing raw epoch seconds.
- Switched the Bazel CLI status stamp from `{BUILD_TIMESTAMP}` to a repo-owned
  `OPEN_BITCOIN_BUILD_TIME` stable status key.
- Updated the workspace status script to emit the repo-owned build-time key in
  ISO-8601 UTC format.
- Tightened the Bazel build-provenance checker so `build_time.value` must match
  the normalized UTC timestamp shape.
- Recorded the UAT gap resolution and promoted the UAT command-copy rule into
  repo guidance after the earlier verification correction.
- Refreshed the tracked LOC report after the repo-native verification contract
  required it.

## Task Commits

1. **Task 1: normalize Cargo and Bazel build-time provenance** — Pending the
   wrapper-owned Phase 24 gap-closure finalization commit.

## Verification

Passed:

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::status::tests::build_provenance_from_inputs_marks_present_fields_available -- --exact`
- `bun run scripts/check-bazel-build-provenance.ts`
- `SOURCE_DATE_EPOCH=1778032597 cargo run --quiet --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json | jq -r '.build.build_time.value'`
- `cargo run --quiet --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json | jq -r '.build.build_time.value'`
- `bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json | jq -r '.build.build_time.value'`
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md`
- `bash scripts/verify.sh`

Observed UAT values:

- Fixed epoch Cargo build: `2026-05-06T01:56:37Z`
- Fresh Cargo build: `2026-05-06T02:11:59Z`
- Fresh Bazel build: `2026-05-06T02:12:44Z`

## Deviations from Plan

- The active GSD lifecycle tooling does not index archived v1.1 phases, so the
  gap closure was executed directly against the archived Phase 24 artifacts
  without updating active v1.2 state.
- `bash scripts/verify.sh` required a LOC report refresh before the final clean
  pass, which is expected for this repo's tracked generated metrics artifact.

## Self-Check: PASSED

- Cargo and Bazel now expose the same build-time JSON shape.
- The UAT gap is recorded as fixed with concrete copy-pasteable Cargo and Bazel
  reproduction commands.
- Repo-native verification passed after all code, script, metrics, and planning
  artifact updates.
