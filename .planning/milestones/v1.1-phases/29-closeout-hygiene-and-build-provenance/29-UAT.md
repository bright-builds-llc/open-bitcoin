---
status: complete
phase: 29-closeout-hygiene-and-build-provenance
source: [29-01-SUMMARY.md, 29-02-SUMMARY.md]
started: 2026-05-06T12:14:18.178Z
updated: 2026-05-06T12:22:15.564Z
---

## Current Test

[testing complete]

## Tests

### 1. Bazel Status Build Provenance
expected: From the repo root, run `temp_dir=$(mktemp -d)` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --network regtest --datadir "$temp_dir" status --format json`; the JSON `build.version` matches the workspace version in `packages/Cargo.toml`, `build.commit` is available and equals `git rev-parse HEAD`, `build.build_time` is a non-empty UTC timestamp, and `build.target` plus `build.profile` are available Bazel metadata instead of `0.0.0` or all-unavailable provenance.
result: pass
verified: `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --network regtest --datadir "$temp_dir" status --format json` returned `build.version = "0.1.0"`, `build.commit = "3899a8ab71a60e508ae713a7ed2eb9379c855426"`, non-empty UTC `build.build_time`, `build.target = "darwin_arm64"`, and `build.profile = "fastbuild"`.

### 2. Repo-Native Verification Includes Provenance Check
expected: Running `bash scripts/verify.sh` from the repo root succeeds and includes the focused Bazel provenance checker, with `bun run scripts/check-bazel-build-provenance.ts` passing as part of the verification contract.
result: pass
verified: `bash scripts/verify.sh` completed successfully in 2m 30.739s, and `bun run scripts/check-bazel-build-provenance.ts` printed `Bazel build provenance check passed.`

### 3. Operator Documentation Describes Cargo And Bazel Provenance
expected: `docs/operator/runtime-guide.md` and `docs/architecture/status-snapshot.md` describe the shared `status` and `dashboard` build section as compile-time truthful across Cargo and Bazel local builds, with build-system-specific provenance strings rather than a normalized enum or all-unavailable fallback.
result: pass
verified: `docs/operator/runtime-guide.md` states the build section stays compile-time truthful across Cargo and Bazel local builds, and `docs/architecture/status-snapshot.md` documents Cargo `TARGET`/`PROFILE`, Bazel `TARGET_CPU`/`COMPILATION_MODE`, and build-system-specific provenance semantics.

## Summary

total: 3
passed: 3
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
