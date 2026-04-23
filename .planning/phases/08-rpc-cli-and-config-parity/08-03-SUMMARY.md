---
phase: 08-rpc-cli-and-config-parity
plan: 03
subsystem: api
tags: [rpc, http, config, auth, axum]
requires:
  - phase: 08-02
    provides: "Typed Phase 8 RPC contracts and the managed RPC context seam"
provides:
  - "Repo-owned bitcoin.conf loading with includeconf, datadir precedence, and auth resolution"
  - "Typed RPC normalization and dispatch for the supported Phase 8 method surface"
  - "POST-only authenticated JSON-RPC transport plus the open-bitcoind entrypoint"
affects: [rpc, cli, config, http, wallet, mempool]
tech-stack:
  added: []
  patterns:
    - "Typed method normalization before dispatch"
    - "Shared managed RPC context reused across config, dispatch, and transport"
    - "Axum transport kept thin over shell-owned config and dispatch layers"
key-files:
  created:
    - packages/open-bitcoin-rpc/src/config/loader.rs
    - packages/open-bitcoin-rpc/src/dispatch.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-rpc/src/http.rs
    - packages/open-bitcoin-rpc/src/http/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  modified:
    - packages/open-bitcoin-rpc/BUILD.bazel
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/config/tests.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/tests.rs
    - packages/open-bitcoin-rpc/src/error.rs
    - packages/open-bitcoin-rpc/src/lib.rs
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-rpc/src/method/tests.rs
key-decisions:
  - "Treat the repo-local Rust verification contract as workspace-manifest scoped because the repo root has no Cargo.toml."
  - "Use a child module at packages/open-bitcoin-rpc/src/config/loader.rs so the config implementation stays under the repo's 628-line production file limit."
  - "Map mempool and chainstate submission failures through RPC error code -26 while keeping legacy HTTP failures on 404 or 500 and JSON-RPC 2.0 failures on 200."
patterns-established:
  - "HTTP transport parses requests once, normalizes params through shared method metadata, and delegates only typed calls into dispatch."
  - "ManagedRpcContext should expose narrow shell wrappers for test fixtures and transport or dispatcher needs instead of leaking raw subsystem internals."
requirements-completed: [RPC-01, CLI-01]
generated_by: codex
generated_at: 2026-04-22T00:00:00Z
---

# Phase 08 Plan 03: RPC Server Transport Summary

**Shared runtime config loading, typed Phase 8 RPC dispatch, and authenticated open-bitcoind transport**

## Performance

- **Tasks completed:** 3
- **Files touched:** 15
- **Task commits:** `9881bea`, `73550dc`, `1634edd`

## Accomplishments

- Implemented the supported `bitcoin.conf` slice in `open-bitcoin-rpc`, including `includeconf`, `datadir` precedence, chain selection, and cookie-versus-user/password auth resolution.
- Added typed RPC method normalization and dispatch for the honest Phase 8 surface: node info, mempool info, wallet info, descriptor import, rescan, list or balance reads, raw transaction submission, and the Open Bitcoin build or sign extensions.
- Added a POST-only axum transport with HTTP Basic auth, legacy versus JSON-RPC 2.0 status handling, batch or notification support, and a thin `open-bitcoind` binary that starts from `load_runtime_config()`.

## Task Commits

1. **Task 1: Implement the repo-owned shared config parser and local-operator auth resolution** - `9881bea`
2. **Task 2: Implement the typed dispatcher and the exact supported method handlers** - `73550dc`
3. **Task 3: Add the authenticated axum transport and open-bitcoind entrypoint** - `1634edd`

## Verification

Exact targeted commands run during execution:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features config::tests::conf_cannot_be_set_in_configuration_files -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features config::tests::rpcpassword_with_hash_is_rejected -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features config::tests::cli_datadir_overrides_config_datadir -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features config::tests::auth_resolution_prefers_cookie_when_password_is_empty -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features context::tests::managed_rpc_context_builds_from_runtime_config -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features dispatch::tests::node_info_methods_return_documented_phase_8_fields -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features dispatch::tests::deriveaddresses_returns_expected_addresses_for_supported_descriptors -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features dispatch::tests::getwalletinfo_returns_supported_field_subset -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features dispatch::tests::wallet_descriptor_and_rescan_methods_update_wallet_views -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features dispatch::tests::sendrawtransaction_returns_txid_and_maps_rejections -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features dispatch::tests::buildandsigntransaction_returns_deterministic_hex_and_fee -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features method::tests::ranged_descriptors_and_deferred_methods_fail_explicitly -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features method::tests::named_params_normalize_and_reject_duplicate_or_colliding_keys -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features http::tests::legacy_and_json_rpc_v2_status_mapping_matches_phase_8_contract -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features http::tests::json_rpc_v2_notifications_return_no_content_and_execute -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features http::tests::mixed_version_batches_are_accepted -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features http::tests::post_only_transport_rejects_unauthenticated_requests -- --exact`
- `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --bin open-bitcoind`

Repo-mandated verification run before each task commit:

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`

Hook-managed verification also passed on each commit, including `bash scripts/verify.sh`.

## Deviations from Plan

### Auto-fixed execution issues

1. **[Rule 3 - Blocking Issue] Workspace Rust checks had to run against `packages/Cargo.toml`**
   - **Found during:** Task 1 verification
   - **Issue:** The repo root has no `Cargo.toml`, so the literal root `cargo fmt --all` or `cargo clippy ...` invocations fail immediately.
   - **Fix:** Ran the required Rust sequence against the workspace manifest at `packages/Cargo.toml`, preserving the mandated order and full workspace coverage.

2. **[Rule 3 - Blocking Issue] Config loader exceeded the repo’s production Rust file limit**
   - **Found during:** Task 1 commit hook verification
   - **Issue:** `packages/open-bitcoin-rpc/src/config.rs` exceeded the enforced 628-line file limit.
   - **Fix:** Split the implementation into `config.rs` plus `config/loader.rs` while keeping the public config surface stable.

## Threat Flags

None beyond the plan’s declared transport and config boundaries.

## Known Stubs

None.

## Self-Check: PASSED

- `FOUND: .planning/phases/08-rpc-cli-and-config-parity/08-03-SUMMARY.md`
- `FOUND: 9881bea`
- `FOUND: 73550dc`
- `FOUND: 1634edd`
