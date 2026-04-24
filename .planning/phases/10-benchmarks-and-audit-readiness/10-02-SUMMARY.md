---
phase: 10-benchmarks-and-audit-readiness
plan: 02
subsystem: benchmarking
tags: [rust, cargo, bazel, benchmarks, parity, audit]

requires:
  - phase: 10-01
    provides: Benchmark crate foundation, registry, runner, and report contracts
provides:
  - Executable benchmark cases for all seven D-01 groups
  - Deterministic benchmark fixtures for consensus, codec, chainstate, mempool, network, and wallet paths
  - Smoke/full benchmark CLI report writing with optional Knots metadata paths
affects: [benchmarking, parity, audit-readiness, bazel, cargo]

tech-stack:
  added: [open-bitcoin-bench first-party crate dependencies on CLI, RPC, wallet, node-path crates]
  patterns: [public-API benchmark composition, clone-reset stateful fixtures, metadata-only optional external paths]

key-files:
  created:
    - packages/open-bitcoin-bench/src/cases.rs
    - packages/open-bitcoin-bench/src/cases/consensus.rs
    - packages/open-bitcoin-bench/src/cases/codec.rs
    - packages/open-bitcoin-bench/src/cases/chainstate.rs
    - packages/open-bitcoin-bench/src/cases/mempool.rs
    - packages/open-bitcoin-bench/src/cases/network.rs
    - packages/open-bitcoin-bench/src/cases/wallet.rs
    - packages/open-bitcoin-bench/src/cases/rpc_cli.rs
    - packages/open-bitcoin-bench/src/fixtures.rs
  modified:
    - packages/open-bitcoin-bench/src/main.rs
    - packages/open-bitcoin-bench/src/registry.rs
    - packages/open-bitcoin-bench/src/report.rs
    - packages/open-bitcoin-bench/BUILD.bazel
    - packages/open-bitcoin-bench/Cargo.toml
    - packages/Cargo.lock
    - MODULE.bazel.lock
    - packages/open-bitcoin-codec/BUILD.bazel

key-decisions:
  - "Benchmark cases compose existing public first-party APIs instead of widening production visibility."
  - "Optional Knots JSON/bin inputs are recorded as report metadata only and are not read during default smoke execution."
  - "The benchmark CLI writes JSON and Markdown reports by default while retaining optional stdout report formatting for compatibility."

patterns-established:
  - "Stateful benchmark cases clone prepared fixtures before mutation so repeated iterations keep the same preconditions."
  - "Bazel compile_data/filegroup is used for checked-in codec fixture bytes consumed by benchmark include_str! calls."

requirements-completed: [PAR-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 10-2026-04-24T10-47-33
generated_at: 2026-04-24T12:04:23Z

duration: 19min
completed: 2026-04-24
---

# Phase 10 Plan 02: Benchmark Cases Summary

**Deterministic Rust benchmark cases for consensus, codec, chainstate, mempool, network, wallet, and RPC/CLI surfaces with smoke JSON/Markdown report output**

## Performance

- **Duration:** 19 min
- **Started:** 2026-04-24T11:45:30Z
- **Completed:** 2026-04-24T12:04:23Z
- **Tasks:** 2
- **Files modified:** 19 source/build files plus this summary

## Accomplishments

- Added executable benchmark cases for all seven D-01 groups: `consensus-script`, `block-transaction-codec`, `chainstate`, `mempool-policy`, `network-wire-sync`, `wallet`, and `rpc-cli`.
- Built deterministic shared fixtures, including clone-reset stateful chainstate/mempool/wallet inputs, checked-in codec bytes, and in-memory network/RPC contexts.
- Updated the benchmark CLI to run `--smoke`/`--full`, write JSON and Markdown reports, reject unknown flags, and record optional Knots JSON/bin paths as metadata only.

## Task Commits

1. **Task 1: Add deterministic fixtures and core benchmark cases** - `d86eaa6` (feat)
2. **Task 2: Add wallet and RPC/CLI cases plus CLI report execution** - `01dd611` (feat)

## Files Created/Modified

- `packages/open-bitcoin-bench/src/fixtures.rs` - Shared deterministic fixture builder for benchmark cases.
- `packages/open-bitcoin-bench/src/cases/*.rs` - Group-specific executable benchmark cases.
- `packages/open-bitcoin-bench/src/main.rs` - CLI mode parsing, output directory handling, report writes, and optional Knots metadata flags.
- `packages/open-bitcoin-bench/src/report.rs` - Optional Knots source metadata schema and Markdown rendering.
- `packages/open-bitcoin-bench/src/registry.rs` - Replaced wallet/RPC metadata placeholders with executable case slices.
- `packages/open-bitcoin-bench/BUILD.bazel`, `packages/open-bitcoin-bench/Cargo.toml`, `packages/Cargo.lock`, `MODULE.bazel.lock` - Build graph updates for first-party benchmark dependencies.
- `packages/open-bitcoin-codec/BUILD.bazel` - Public filegroup for codec testdata used as benchmark compile-time data.

## Decisions Made

- Kept all benchmark construction inside `open-bitcoin-bench` through existing public APIs rather than adding new production support hooks.
- Did not widen `open-bitcoin-cli` to expose binary-internal render helpers; the RPC/CLI case covers the available public CLI parse, RPC normalize, and dispatch path.
- Treated optional Knots input paths as report metadata only, matching D-03 and avoiding default external binary or JSON dependencies.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added required build dependencies and lockfile updates**
- **Found during:** Task 1 and Task 2
- **Issue:** The plan listed benchmark source files, but executable cases require linking the relevant first-party crates and refreshing Cargo/Bazel lock metadata.
- **Fix:** Updated `open-bitcoin-bench` Cargo/Bazel deps, `packages/Cargo.lock`, and `MODULE.bazel.lock`.
- **Files modified:** `packages/open-bitcoin-bench/Cargo.toml`, `packages/open-bitcoin-bench/BUILD.bazel`, `packages/Cargo.lock`, `MODULE.bazel.lock`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench --all-features`, `bazel build //:bench`, `bash scripts/verify.sh`
- **Committed in:** `d86eaa6`, `01dd611`

**2. [Rule 3 - Blocking] Made codec fixture bytes available in Bazel sandbox**
- **Found during:** Task 1
- **Issue:** Bazel could not resolve `include_str!` paths for codec fixture files unless those files were declared in the build graph.
- **Fix:** Added a public codec `testdata` filegroup and benchmark `compile_data` reference.
- **Files modified:** `packages/open-bitcoin-codec/BUILD.bazel`, `packages/open-bitcoin-bench/BUILD.bazel`
- **Verification:** `bazel build //:bench`, `bash scripts/verify.sh`
- **Committed in:** `d86eaa6`

### Process Adjustments

- TDD RED checks were run locally and left uncommitted because `AGENTS.md` requires all pre-commit checks to pass before every commit. Each task was committed after GREEN verification passed.

**Total deviations:** 2 auto-fixed blocking issues plus 1 AGENTS.md process adjustment.
**Impact on plan:** All changes were required for executable benchmark cases and repo-native verification; no scope outside benchmark/report/build wiring was added.

## Issues Encountered

- Bazel target labels initially referenced non-existent crate target names; corrected to the repo's `_lib` labels and verified with `bazel build //:bench`.
- `RpcFailure` has structured details instead of `Display`; the benchmark RPC/CLI case now records public detail messages when mapping failures into `BenchError`.

## Known Stubs

None. The remaining generated `TODO` strings detected during stub scanning are inside `MODULE.bazel.lock` generated crate-universe metadata, not benchmark behavior or UI/data-source stubs.

## Threat Flags

None. The new CLI file writes and optional Knots paths were already covered by the plan threat model; no new network endpoints, auth paths, or trust-boundary surfaces were introduced.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench --all-features` - passed
- `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-bench -- --smoke --output-dir packages/target/benchmark-reports` - passed and wrote JSON/Markdown reports
- `node -e "const fs=require('fs'); const report=JSON.parse(fs.readFileSync('packages/target/benchmark-reports/open-bitcoin-bench-smoke.json','utf8')); if (!Array.isArray(report.groups) || report.groups.length < 7) process.exit(1); console.log(report.groups.length);"` - passed with `7`
- `bash scripts/verify.sh` - passed

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 10-03 can consume structured smoke benchmark reports from `packages/target/benchmark-reports` and rely on all D-01 groups being present without requiring Knots execution by default.

## Self-Check: PASSED

- Summary file exists.
- Created benchmark case and fixture files exist.
- Task commits `d86eaa6` and `01dd611` exist in git history.

---
*Phase: 10-benchmarks-and-audit-readiness*
*Completed: 2026-04-24*
