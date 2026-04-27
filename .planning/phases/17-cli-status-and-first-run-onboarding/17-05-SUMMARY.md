---
phase: 17-cli-status-and-first-run-onboarding
plan: "05"
subsystem: cli-operator-runtime
tags: [cli, status, onboarding, jsonc, config, bazel]

requires:
  - phase: 17-cli-status-and-first-run-onboarding
    provides: "operator config resolution, read-only detection, and shared status rendering"
provides:
  - "actual open-bitcoin operator binary"
  - "runtime dispatch for status, config paths, and onboarding"
  - "idempotent JSONC-only onboarding write path"
  - "end-to-end operator binary coverage for stopped and fake live status"
affects: [CLI-03, CLI-04, CLI-05, CLI-06, CLI-07, OBS-01, OBS-02, MIG-02, operator]

tech-stack:
  added:
    - "Cargo and Bazel open-bitcoin binary target"
  patterns:
    - "operator commands return typed stdout/stderr/exit-code outcomes"
    - "onboarding plans are pure until the approved JSONC write shell runs"
    - "HTTP status RPC adapter lives under the status module and is exercised through hermetic fake RPC tests"

key-files:
  created:
    - packages/open-bitcoin-cli/src/bin/open-bitcoin.rs
    - packages/open-bitcoin-cli/src/operator/status/http.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
  modified:
    - README.md
    - docs/architecture/cli-command-architecture.md
    - docs/parity/source-breadcrumbs.json
    - packages/open-bitcoin-cli/Cargo.toml
    - packages/open-bitcoin-cli/BUILD.bazel
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/config.rs
    - packages/open-bitcoin-cli/src/operator/onboarding.rs
    - packages/open-bitcoin-cli/src/operator/onboarding/tests.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/src/operator/status.rs

key-decisions:
  - "Kept open-bitcoin-cli as the compatibility client and added open-bitcoin as a separate operator binary."
  - "Non-interactive onboarding requires explicit approval to create config and force-overwrite to replace existing config."
  - "Onboarding writes only open-bitcoin.jsonc and stores Open Bitcoin-owned answers under onboarding, metrics, logs, and migration settings."
  - "Service and dashboard commands return explicit Phase 18 and Phase 19 boundary messages."

patterns-established:
  - "Binary integration tests use env!(\"CARGO_BIN_EXE_open-bitcoin\") with temp dirs and local fake RPC only."
  - "Core/Knots detection is rendered as read-only evidence with source paths and uncertainty language."
  - "Runtime dispatch stays thin; HTTP status RPC details live under operator/status/http.rs."

requirements-completed: [OBS-01, OBS-02, CLI-03, CLI-04, CLI-05, CLI-06, CLI-07, MIG-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 17-2026-04-26T23-56-00
generated_at: 2026-04-27T01:43:31Z

duration: 16min
completed: 2026-04-27
---

# Phase 17 Plan 05 Summary

**Actual open-bitcoin operator binary with status, config path discovery, and idempotent first-run onboarding**

## Performance

- **Duration:** 16 min
- **Started:** 2026-04-27T01:27:37Z
- **Completed:** 2026-04-27T01:43:31Z
- **Tasks:** 3
- **Files modified:** 14

## Accomplishments

- Added the `open-bitcoin` Cargo/Bazel binary that parses `OperatorCli`, executes typed runtime outcomes, and preserves `open-bitcoin-cli` compatibility behavior.
- Wired `status`, `config paths`, and `onboard` through the runtime, including stopped-node status and fake live RPC status collection.
- Added pure onboarding planning plus an approved write shell that writes only `open-bitcoin.jsonc`, is idempotent by default, and requires `--force-overwrite` before replacing an existing config.
- Added hermetic binary tests for stopped status JSON, fake running RPC status JSON, human no-color status, config paths, and non-interactive onboarding create/rerun/force behavior.
- Updated README and CLI architecture docs for the now-executable operator surface.

## Task Commits

1. **Plan 17-05 implementation and docs** - pending final wrapper commit after phase verification.

## Decisions Made

- Used a separate `open-bitcoin` binary instead of overloading the compatibility client entrypoint.
- Kept status RPC failures as unreachable snapshots rather than process failures, preserving support-oriented output for partially configured nodes.
- Split the real HTTP status RPC adapter into `operator/status/http.rs` during the simplification pass so `runtime.rs` remains command dispatch focused and below the repo size guideline.
- Added the missing Bazel `serde` dependency for the existing `open_bitcoin_cli` target so both CLI binaries build under Bazel.

## Deviations from Plan

### Auto-fixed Issues

**1. Runtime file-size simplification**
- **Found during:** Simplification pass after targeted verification.
- **Issue:** `packages/open-bitcoin-cli/src/operator/runtime.rs` was just over the repo size guideline because it also owned the HTTP status RPC adapter.
- **Fix:** Moved the adapter to `packages/open-bitcoin-cli/src/operator/status/http.rs`, exported it from `status.rs`, and registered the file in parity breadcrumbs.
- **Verification:** `wc -l` reports `runtime.rs` at 411 lines; targeted operator tests, binary tests, breadcrumb check, Bazel CLI build, and the full Rust gate pass.

**2. Contributor-facing docs were stale**
- **Found during:** README/operator docs freshness check required by the plan.
- **Issue:** `docs/architecture/cli-command-architecture.md` still described operator execution as later work, and README only showed the compatibility RPC client preview.
- **Fix:** Updated architecture docs for executable `status`, `config paths`, and `onboard`, and added README examples for the operator binary.
- **Verification:** Docs diff reviewed and `bash scripts/verify.sh` remains the final repo-native verification gate.

## Issues Encountered

- The fake RPC test fixture initially read an accepted socket as nonblocking on macOS and returned an unreachable snapshot. The fixture now tolerates `WouldBlock` and timeouts while keeping production status behavior unchanged.
- Bazel initially failed `open_bitcoin_cli` because `src/client.rs` imports `serde` directly; adding the explicit Bazel dep aligned the BUILD target with Cargo.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_flows` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- `bazel build //packages/open-bitcoin-cli:open_bitcoin //packages/open-bitcoin-cli:open_bitcoin_cli` passed.
- Rust pre-commit gate passed after the simplification pass: `cargo fmt --all`, clippy with `-D warnings`, all-target build, and `cargo test --all-features`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 18 can implement service lifecycle effects against a working operator binary and status/config surface. Phase 19 can consume the same status snapshot and metrics/log paths for the dashboard.

## Self-Check: PASSED

- `open-bitcoin status` works for stopped nodes and fake live RPC evidence.
- `open-bitcoin onboard` is idempotent and does not write `bitcoin.conf`.
- Core/Knots detection evidence appears with source paths and uncertainty language.
- `open-bitcoin-cli` compatibility tests still pass.

---
*Phase: 17-cli-status-and-first-run-onboarding*
*Completed: 2026-04-27*
