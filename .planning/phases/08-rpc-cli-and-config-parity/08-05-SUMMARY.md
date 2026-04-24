---
phase: 08-rpc-cli-and-config-parity
plan: 05
subsystem: rpc-cli-config
tags: [rpc, cli, config, operator-flows, parity]
requires:
  - phase: 08-04
    provides: "CLI startup precedence, shared config loading, and -getinfo batching"
provides:
  - "Authenticated bitcoin-cli binary over the Phase 8 HTTP transport"
  - "Hermetic CLI-driven operator-flow coverage for descriptor import through raw transaction submission"
  - "Parity ledger entry for the supported and deferred Phase 8 RPC, CLI, and config surface"
affects: [cli, rpc, docs, bazel, cargo]
tech-stack:
  added: []
  patterns:
    - "The CLI binary stays thin and routes supported method params through the shared RPC method layer before transport."
    - "Operator-flow integration tests boot an in-process TCP RPC harness over managed runtime fixtures and drive only the real CLI binary."
    - "Parity docs list supported baseline-backed methods, Open Bitcoin extension methods, and deferred surfaces explicitly."
key-files:
  created:
    - packages/open-bitcoin-cli/src/client.rs
    - packages/open-bitcoin-cli/src/client/tests.rs
    - packages/open-bitcoin-cli/src/main.rs
    - packages/open-bitcoin-cli/src/output.rs
    - packages/open-bitcoin-cli/tests/operator_flows.rs
    - docs/parity/catalog/rpc-cli-config.md
  modified:
    - packages/open-bitcoin-cli/BUILD.bazel
    - packages/open-bitcoin-cli/Cargo.toml
    - packages/open-bitcoin-cli/src/client.rs
    - packages/open-bitcoin-rpc/src/method.rs
    - docs/parity/index.json
    - packages/Cargo.lock
    - MODULE.bazel.lock
key-decisions:
  - "Keep the production CLI transport in binary-local modules over the existing args, startup, and getinfo library contracts so the Phase 8 shell stays thin."
  - "Use a hermetic TCP integration harness around managed RPC context and dispatch to prove the operator workflow through the actual CLI binary without external daemons."
  - "Fix rescanblockchain field naming in the shared RPC method type so the CLI request builder and named-argument RPC path agree on external parameter names."
requirements-completed: [CLI-01, CLI-02]
generated_by: codex
lifecycle_mode: yolo
phase_lifecycle_id: 08-2026-04-24T02-23-19
generated_at: 2026-04-22T00:00:00Z
lifecycle_repair_note: "Adopted into refreshed Phase 08 gap-closure lifecycle on 2026-04-24; original generated_at retained."
---

# Phase 08 Plan 05: CLI HTTP Execution, Operator Flow, And Parity Summary

**Working bitcoin-cli transport plus hermetic operator-flow proof over the supported Phase 8 RPC and config slice**

## Performance

- **Tasks completed:** 2
- **Files touched:** 13
- **Task commits:** `6e5470f`, `ae93c65`

## Accomplishments

- Added the `open-bitcoin-cli` binary entrypoint, a thin `ureq`-based HTTP client, deterministic RPC result rendering, and explicit exit-code `1` failures for actionable RPC and auth errors.
- Routed supported RPC calls and the `-getinfo` helper through the shared Phase 8 method layer instead of bespoke per-command transport logic, including canonical request shaping for the supported method set.
- Added hermetic integration coverage that boots a local in-process RPC harness, drives the real CLI binary through `importdescriptors -> rescanblockchain -> getbalances -> listunspent -> buildandsigntransaction -> sendrawtransaction`, and asserts explicit failures for `sendtoaddress`, `-netinfo`, and `-rpcwallet`.
- Added `docs/parity/catalog/rpc-cli-config.md` and indexed it from `docs/parity/index.json` so the supported baseline-backed methods, Open Bitcoin extension methods, supported auth/config slice, and deferred surfaces are auditable.

## Task Commits

1. **Task 1: Implement the HTTP client, CLI execution path, and operator-facing output handling** - `6e5470f`
2. **Task 2: Prove the supported headless operator flow and update the parity ledger** - `ae93c65`

## Verification

Exact targeted commands run during execution:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features client::tests::rpc_errors_surface_exit_code_one_with_actionable_stderr -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features client::tests::getinfo_json_mode_is_stable_for_automation -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --test operator_flows descriptor_rescan_balance_build_sign_and_send_roundtrip -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --test operator_flows deferred_surfaces_fail_explicitly -- --exact`
- `rg -n 'buildtransaction|buildandsigntransaction|sendtoaddress|rpcauth|rpcwhitelist|rpcwallet|getpeerinfo|-netinfo' docs/parity/catalog/rpc-cli-config.md`
- `rg -n 'rpc-cli-config' docs/parity/index.json`

Repo-mandated verification run before each task commit:

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bash scripts/verify.sh`

## Deviations from Plan

### Auto-fixed issues

1. **[Rule 3 - Blocking Issue] The operator-flow integration test needed a dev-only fixture dependency**
   - **Found during:** Task 2 RED
   - **Issue:** `packages/open-bitcoin-cli/tests/operator_flows.rs` needed repo-owned block, wallet, and descriptor fixture types, but `open-bitcoin-cli` did not depend on `open-bitcoin-node` in test builds.
   - **Fix:** Added `open-bitcoin-node` as a dev-dependency and committed the resulting `packages/Cargo.lock` plus `MODULE.bazel.lock` updates.
   - **Files modified:** `packages/open-bitcoin-cli/Cargo.toml`, `packages/Cargo.lock`, `MODULE.bazel.lock`
   - **Commit:** `ae93c65`

2. **[Rule 1 - Bug] Shared `rescanblockchain` request field names were inconsistent with the CLI HTTP path**
   - **Found during:** Task 2 GREEN
   - **Issue:** The new CLI request canonicalization surfaced that `RescanBlockchainRequest` accepted internal field names (`maybe_start_height`, `maybe_stop_height`) instead of the public RPC names (`start_height`, `stop_height`), which broke the named-argument HTTP path.
   - **Fix:** Added the missing serde renames in `packages/open-bitcoin-rpc/src/method.rs` and kept the client-side canonical request object on the public field names.
   - **Files modified:** `packages/open-bitcoin-rpc/src/method.rs`, `packages/open-bitcoin-cli/src/client.rs`
   - **Commit:** `ae93c65`

## Threat Flags

None beyond the plan’s declared CLI auth, transport, and operator-flow boundaries.

## Known Stubs

None.

## Self-Check: PASSED

- `FOUND: .planning/phases/08-rpc-cli-and-config-parity/08-05-SUMMARY.md`
- `FOUND: 6e5470f`
- `FOUND: ae93c65`
