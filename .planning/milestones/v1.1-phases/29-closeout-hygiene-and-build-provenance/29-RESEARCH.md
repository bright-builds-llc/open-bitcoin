---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 29-2026-04-29T14-01-32
generated_at: 2026-04-29T14:01:32Z
---

# Phase 29 Research

## Scope

Phase 29 is the last optional cleanup left after the Phase 27 milestone audit.
The remaining tech debt is narrow:

- Bazel-built CLI status output still reports build provenance as mostly
  unavailable.
- The same Bazel path currently reports `build.version` as `0.0.0`, which is
  also operator-visible drift from the real workspace version.
- Final phase closeout should keep milestone ledgers and docs consistent without
  reopening earlier requirements.

## Audit Findings

`.planning/v1.1-MILESTONE-AUDIT-PHASE-27.md` recorded the remaining non-blocking
tech debt under Phase 24:

- "Build provenance remains effectively Cargo-only; Bazel-built CLI targets are
  likely to render those fields as `Unavailable`."

That note matches the current live baseline from:

```bash
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --network regtest --datadir="$(mktemp -d)" status --format json
```

Observed baseline:

- `build.version = "0.0.0"`
- `build.commit.state = "unavailable"`
- `build.build_time.state = "unavailable"`
- `build.target.state = "unavailable"`
- `build.profile.state = "unavailable"`

## Code Findings

### Shared status collector

`packages/open-bitcoin-cli/src/operator/status.rs` already has the right shape:

- `current_build_provenance()` reads compile-time metadata through `env!` and
  `option_env!`.
- `build_provenance_field()` keeps empty or missing values explicit as
  `Unavailable`.

That means Phase 29 does **not** need new runtime logic. It only needs truthful
env-var injection for the Bazel build path.

### Cargo path

`packages/open-bitcoin-cli/build.rs` already emits:

- `OPEN_BITCOIN_BUILD_COMMIT`
- `OPEN_BITCOIN_BUILD_TIME`
- `OPEN_BITCOIN_BUILD_TARGET`
- `OPEN_BITCOIN_BUILD_PROFILE`

for Cargo builds.

### Bazel path

`packages/open-bitcoin-cli/BUILD.bazel` currently declares:

- `rust_library(name = "open_bitcoin_cli_lib", ...)`
- `rust_binary(name = "open_bitcoin", ...)`
- `rust_binary(name = "open_bitcoin_cli", ...)`

but none of those targets currently set:

- a crate `version`
- stamped provenance env vars
- a repo-owned Bazel workspace-status command

`.bazelrc` is intentionally minimal today and has no
`--workspace_status_command` entry.

## Verification Surface Findings

`bash scripts/verify.sh` already gives useful leverage:

- it runs the repo-owned checks, Rust format/lint/build/test, smoke benchmarks,
  and a Bazel build
- it currently builds `//:cli`, which aliases the shared CLI library target

That means if the provenance fix attaches to `open_bitcoin_cli_lib`, the
existing Bazel build gate will still cover compilation of the repaired target.

What is missing today is a focused runtime proof that a Bazel-built CLI status
surface actually carries the provenance values instead of only compiling.

## Recommended Implementation Shape

1. Add a thin repo-owned workspace-status shell script that emits a stable Git
   commit value for Bazel stamping.
2. Point `.bazelrc` at that script.
3. Stamp `open_bitcoin_cli_lib` and set:
   - `version = "0.1.0"`
   - `OPEN_BITCOIN_BUILD_COMMIT = {STABLE_OPEN_BITCOIN_BUILD_COMMIT}`
   - `OPEN_BITCOIN_BUILD_TIME = {BUILD_TIMESTAMP}`
   - `OPEN_BITCOIN_BUILD_TARGET = $(TARGET_CPU)`
   - `OPEN_BITCOIN_BUILD_PROFILE = $(COMPILATION_MODE)`
4. Add a focused Bun checker that:
   - reads the workspace version from `packages/Cargo.toml`
   - reads Bazel `TARGET_CPU` and `COMPILATION_MODE`
   - runs the Bazel-built `open-bitcoin status --format json`
   - asserts version, commit, build time, target, and profile are populated
     truthfully
5. Wire that checker into `bash scripts/verify.sh`.
6. Refresh the operator-facing provenance docs and final phase ledgers.

## Risks

- `stamp = 1` on the shared CLI library is the smallest targeted fix, but it
  still makes that library sensitive to workspace status inputs.
- `TARGET_CPU` is a Bazel platform identifier, not a Rust target triple. This is
  acceptable if documented as build-system-specific provenance rather than a
  promise of Cargo-style naming.
- `BUILD_TIMESTAMP` is epoch-seconds, so the Bazel path will not match the Cargo
  ISO timestamp format. Truthfulness matters more than a forced format match.
