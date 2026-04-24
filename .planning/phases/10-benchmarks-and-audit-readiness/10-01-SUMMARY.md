---
phase: 10-benchmarks-and-audit-readiness
plan: "01"
subsystem: benchmarks
tags:
  - rust
  - bazel
  - benchmarks
  - serde
  - parity
requires:
  - phase-09-parity-harnesses-and-fuzzing
  - first-party-rust-workspace
provides:
  - open-bitcoin-bench crate
  - static D-01 benchmark group registry
  - Knots benchmark mapping metadata
  - bounded Instant-based runner contract
  - JSON and Markdown report schema
  - Bazel //:bench alias
affects:
  - 10-benchmarks-and-audit-readiness
  - 10-02
  - PAR-02
tech-stack:
  added:
    - serde
    - serde_json
  patterns:
    - stable Rust Instant timing
    - std::hint::black_box runner boundary
    - typed serde report serialization
    - static registry metadata before real benchmark bodies
key-files:
  created:
    - packages/open-bitcoin-bench/Cargo.toml
    - packages/open-bitcoin-bench/BUILD.bazel
    - packages/open-bitcoin-bench/src/lib.rs
    - packages/open-bitcoin-bench/src/error.rs
    - packages/open-bitcoin-bench/src/main.rs
    - packages/open-bitcoin-bench/src/registry.rs
    - packages/open-bitcoin-bench/src/report.rs
    - packages/open-bitcoin-bench/src/runner.rs
  modified:
    - packages/Cargo.toml
    - packages/Cargo.lock
    - BUILD.bazel
    - MODULE.bazel.lock
key-decisions:
  - Use a repo-owned stable-Rust benchmark harness with serde JSON/Markdown reports instead of adding Criterion or Divan.
  - Keep TDD RED runs local-only when failing commits would violate the Rust pre-commit contract.
  - Treat MODULE.bazel.lock crate-universe refreshes as task-local Bazel metadata for new workspace members.
requirements-completed:
  - PAR-02
metrics:
  duration: 10min
  completed: 2026-04-24T11:43:33Z
  tasks: 2
  files: 12
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 10-2026-04-24T10-47-33
generated_at: 2026-04-24T11:43:33Z
---

# Phase 10 Plan 01: Benchmark Foundation Summary

First-party benchmark crate with static Knots mappings, bounded Instant runner contracts, and JSON/Markdown reports.

## Accomplishments

- Added `open-bitcoin-bench` as a first-party Rust crate in the workspace.
- Defined the seven D-01 benchmark groups: consensus script, block/transaction codec, chainstate, mempool policy, network wire/sync, wallet, and RPC/CLI.
- Recorded Knots mapping markers for `VerifyScriptBench`, `DeserializeBlockTest`, `ComplexMemPool`, `RpcMempool`, `WalletBalance`, `CoinSelection`, `WalletCreateTx`, `AddrMan`, and `EvictionProtection`.
- Implemented bounded smoke/full runner configuration with no wall-clock pass/fail thresholds.
- Added typed JSON and Markdown report generation with an optional Knots source section.
- Wired the crate into Bazel through `//packages/open-bitcoin-bench:open_bitcoin_bench` and root `//:bench`.

## Task Commits

| Task | Commit | Summary |
| ---- | ------ | ------- |
| 1 | `7285671` | `feat(10-01): create benchmark crate foundation` |
| 2 | `62b0a14` | `chore(10-01): wire benchmark crate into Bazel` |

## Decisions Made

- The benchmark harness stays repo-owned and stable-Rust based for now. This keeps Phase 10 audit output deterministic and avoids adding a benchmark framework dependency before real benchmark bodies are known.
- The TDD RED step was run and observed locally, but not committed, because repo-local Rust pre-commit requirements forbid committing known-failing code.
- `MODULE.bazel.lock` was included in Task 2 because the new Cargo workspace member changed crate-universe metadata needed by Bazel.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical Functionality] Preserved Rust pre-commit contract during TDD RED**
- **Found during:** Task 1
- **Issue:** The plan requested a TDD RED commit, but `AGENTS.md` requires format, clippy, build, and tests to pass before every Rust commit.
- **Fix:** Created and ran the failing RED tests locally to prove the failure signal, then implemented the GREEN state and committed only after the required Rust verification sequence passed.
- **Files modified:** `packages/open-bitcoin-bench/src/*`, `packages/Cargo.toml`, `packages/Cargo.lock`
- **Commit:** `7285671`

**2. [Rule 3 - Blocking Issue] Committed generated Bazel crate metadata**
- **Found during:** Task 2
- **Issue:** Adding the new Cargo member caused Bazel crate-universe metadata in `MODULE.bazel.lock` to refresh.
- **Fix:** Treated the lockfile refresh as task-local generated metadata required for deterministic Bazel builds and committed it with the Bazel wiring.
- **Files modified:** `MODULE.bazel.lock`
- **Commit:** `62b0a14`

## Known Stubs

| File | Line | Stub | Reason |
| ---- | ---- | ---- | ------ |
| `packages/open-bitcoin-bench/src/registry.rs` | 230 | `metadata_case()` returns `Ok(())` | Plan 10-01 establishes benchmark contracts and Knots mappings only. Plan 10-02 is expected to replace static metadata cases with real benchmark bodies using the same registry and runner contracts. |

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench --all-features`
- `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-bench -- --list`
- `bazel build //:bench`
- `cargo fmt --all --manifest-path packages/Cargo.toml`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bash scripts/verify.sh`

All verification passed. `scripts/verify.sh` completed successfully; non-failing C warnings from third-party build dependencies were observed during Bazel/coverage output.

## Next Phase Readiness

Plan 10-02 can keep the public crate, registry, runner, report schema, and `//:bench` target stable while replacing `metadata_case()` entries with real benchmark functions.

## Self-Check: PASSED

- Found summary and key benchmark crate files.
- Found task commits `7285671` and `62b0a14`.
