---
phase: 08-rpc-cli-and-config-parity
plan: 04
subsystem: cli
tags: [cli, config, getinfo, parity]
requires:
  - phase: 08-03
    provides: "Shared runtime config loading plus typed Phase 8 method normalization"
provides:
  - "Baseline-shaped bitcoin-cli parsing for stdin, named args, deferred surfaces, and supported startup flags"
  - "Explicit client-side config, endpoint, and auth precedence resolution over the shared RPC runtime-config loader"
  - "Thin -getinfo batch and deterministic rendering contract over the supported Phase 8 RPC methods"
affects: [cli, rpc, config, bazel]
tech-stack:
  added: []
  patterns:
    - "CLI parsing validates supported method shapes through the shared RPC normalizer instead of inventing CLI-only semantics."
    - "Client startup resolves config and auth by forwarding only the relevant CLI flags into the shared runtime-config loader."
    - "-getinfo stays transport-free by batching SupportedMethod calls and rendering the typed RPC response structs directly."
key-files:
  created:
    - packages/open-bitcoin-cli/src/args.rs
    - packages/open-bitcoin-cli/src/args/tests.rs
    - packages/open-bitcoin-cli/src/startup.rs
    - packages/open-bitcoin-cli/src/startup/tests.rs
    - packages/open-bitcoin-cli/src/getinfo.rs
    - packages/open-bitcoin-cli/src/getinfo/tests.rs
    - packages/open-bitcoin-rpc/src/config/loader/rpc_address.rs
  modified:
    - packages/open-bitcoin-cli/src/lib.rs
    - packages/open-bitcoin-cli/Cargo.toml
    - packages/open-bitcoin-cli/BUILD.bazel
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/config/loader.rs
    - packages/open-bitcoin-rpc/src/config/tests.rs
    - packages/Cargo.lock
    - MODULE.bazel.lock
key-decisions:
  - "Expose an arg-driven shared runtime-config entrypoint so bitcoin-cli can reuse config and auth precedence without reaching through open-bitcoind startup."
  - "Keep CLI named-argument handling thin by parsing CLI input locally, then delegating supported-method validation to open-bitcoin-rpc::normalize_method_call."
  - "Treat --json as the only accepted -getinfo helper argument and serialize the typed RPC response structs directly for deterministic automation output."
patterns-established:
  - "Supported CLI methods can validate early without transport by normalizing RequestParameters through the shared RPC method registry."
  - "Bazel crate-universe lock updates are task-local generated artifacts when workspace Cargo manifests change and should be committed with the owning task."
requirements-completed: [CLI-01]
generated_by: codex
lifecycle_mode: yolo
phase_lifecycle_id: 08-2026-04-24T02-23-19
generated_at: 2026-04-23T04:53:13Z
lifecycle_repair_note: "Adopted into refreshed Phase 08 gap-closure lifecycle on 2026-04-24; original generated_at retained."
---

# Phase 08 Plan 04: CLI Startup and GetInfo Summary

**bitcoin-cli startup precedence, supported flag parsing, and deterministic -getinfo batching**

## Performance

- **Tasks completed:** 2
- **Files touched:** 15
- **Task commits:** `ed945c0`, `d78fc40`

## Accomplishments

- Added a real `open-bitcoin-cli` parsing surface for the supported Phase 8 slice, including `-named`, `-stdin`, `-stdinrpcpass`, `-rpcconnect`, `-rpcport`, `-rpcuser`, `-rpcpassword`, `-rpccookiefile`, `-getinfo`, and `-color`, while failing `-netinfo` and `-rpcwallet` explicitly as deferred surfaces.
- Added `startup.rs` so the client path resolves config-file location, datadir, endpoint, and auth precedence itself by forwarding only the relevant CLI flags into the shared `open-bitcoin-rpc` config loader.
- Added a thin `-getinfo` contract that freezes the helper as a four-call batch over `getnetworkinfo`, `getblockchaininfo`, `getwalletinfo`, and `getbalances`, plus deterministic `--json` rendering for automation and a human dashboard renderer for operators.

## Task Commits

1. **Task 1: Implement explicit bitcoin-cli startup, precedence, and helper-flag parsing** - `ed945c0`
2. **Task 2: Implement the -getinfo helper batch and deterministic rendering contract** - `d78fc40`

## Verification

Exact targeted commands run during execution:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features args::tests::stdinrpcpass_is_consumed_before_stdin_arguments -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features args::tests::invalid_rpc_ports_fail_before_request_dispatch -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features startup::tests::client_startup_resolves_conf_datadir_and_auth_precedence -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features args::tests::named_arguments_reject_positional_collisions -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features args::tests::deferred_cli_surfaces_fail_with_actionable_errors -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features getinfo::tests::getinfo_builds_expected_four_call_batch_and_rejects_extra_args -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features getinfo::tests::getinfo_json_mode_is_stable_for_automation -- --exact`

Repo-mandated verification run before each task commit:

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`

Hook-managed verification also passed on both task commits, including `bash scripts/verify.sh`.

## Deviations from Plan

### Auto-fixed execution issues

1. **[Rule 3 - Blocking Issue] Workspace Rust checks had to run against `packages/Cargo.toml`**
   - **Found during:** Task 1 verification
   - **Issue:** The repo root has no `Cargo.toml`, so the literal root `cargo fmt --all` and related commands fail immediately.
   - **Fix:** Ran the required Rust sequence against the workspace manifest at `packages/Cargo.toml`, preserving the mandated order and full workspace coverage.

2. **[Rule 3 - Blocking Issue] The shared config loader was missing the client-facing entrypoint and correct client port precedence**
   - **Found during:** Task 1 implementation
   - **Issue:** `open-bitcoin-cli` needed to reuse shared config loading, but `open-bitcoin-rpc` only exposed environment-driven startup and the client path did not honor `rpcport` over embedded `rpcconnect` ports.
   - **Fix:** Added `load_runtime_config_for_args`, corrected client port precedence, and moved the new RPC address parsing helper into `config/loader/rpc_address.rs` to keep the production file below the repo’s line limit.

3. **[Rule 3 - Blocking Issue] Bazel crate-universe lockfiles changed after the CLI package dependency wiring**
   - **Found during:** Task 2 verification
   - **Issue:** `MODULE.bazel.lock` regenerated after the CLI crate started depending on the shared RPC crate and the updated workspace manifests fed Bazel’s crate-universe inputs.
   - **Fix:** Committed the generated lockfile update with the task so the workspace stays reproducible after `scripts/verify.sh`.

## Threat Flags

None beyond the plan’s declared CLI-input, auth, and helper-batching boundaries.

## Known Stubs

None.

## Self-Check: PASSED

- `FOUND: .planning/phases/08-rpc-cli-and-config-parity/08-04-SUMMARY.md`
- `FOUND: ed945c0`
- `FOUND: d78fc40`
