---
phase: 08-rpc-cli-and-config-parity
plan: 07
subsystem: cli-config
tags: [cli, config, stdin, gap-closure]
requires:
  - phase: 08-05
    provides: "Completed Phase 8 RPC/CLI/config implementation with verifier gaps preserved in 08-GAPS.md"
  - phase: 08-06
    provides: "RPC dispatcher and cookie-auth gap closure"
provides:
  - "Hostname-preserving RPC client endpoint parsing"
  - "Duplicate named CLI parameter rejection before HTTP transport"
  - "Parser-owned stdin gating for the real CLI binary"
affects: [08-08, cli, config, rpc-client]
key-files:
  modified:
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/config/loader.rs
    - packages/open-bitcoin-rpc/src/config/loader/rpc_address.rs
    - packages/open-bitcoin-rpc/src/config/tests.rs
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-rpc/src/method/tests.rs
    - packages/open-bitcoin-cli/src/startup.rs
    - packages/open-bitcoin-cli/src/startup/tests.rs
    - packages/open-bitcoin-cli/src/args.rs
    - packages/open-bitcoin-cli/src/args/tests.rs
    - packages/open-bitcoin-cli/src/main.rs
    - packages/open-bitcoin-cli/tests/operator_flows.rs
requirements-completed: [CLI-01, CLI-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 08-2026-04-24T02-23-19
generated_at: 2026-04-24T09:24:00Z
duration: in-progress-session
completed: 2026-04-24
---

# Phase 08 Plan 07: CLI Config And Stdin Gap Closure Summary

## Accomplishments

- RPC client config now preserves hostname endpoints through `RpcClientEndpoint` while keeping server bind addresses socket-only.
- `-named` CLI parsing now keeps repeated keys long enough for shared method normalization to reject duplicates before HTTP transport.
- The real `open-bitcoin-cli` binary now reads stdin only when `-stdin` or `-stdinrpcpass` is enabled.
- Added an open-stdin subprocess regression so normal no-stdin-flag invocations cannot hang while waiting for EOF.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features config::tests::rpcconnect_accepts_hostnames_and_preserves_port_precedence -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features startup::tests::client_startup_preserves_rpcconnect_hostnames -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features method::tests::named_params_distinguish_duplicate_keys_from_positional_collisions -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features args::tests::named_arguments_reject_duplicate_keys_before_transport -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features args::tests::stdin_requirement_detection_matches_stdin_flags -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --test operator_flows normal_cli_without_stdin_flags_does_not_wait_for_open_stdin -- --exact`

## Handoff

- Parity documentation, final full verification, and human verification notes remain assigned to `08-08`.
- This interactive run has not created task-level git commits yet; commit finalization should happen after all Phase 08 gap plans pass verification.
