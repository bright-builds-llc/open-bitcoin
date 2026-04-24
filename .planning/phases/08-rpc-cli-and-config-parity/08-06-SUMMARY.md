---
phase: 08-rpc-cli-and-config-parity
plan: 06
subsystem: rpc
tags: [rpc, dispatch, cookie-auth, security, gap-closure]
requires:
  - phase: 08-05
    provides: "Completed Phase 8 RPC/CLI/config implementation with verifier gaps preserved in 08-GAPS.md"
provides:
  - "Explicit rejection of unsupported rescanblockchain height ranges"
  - "Explicit rejection of unenforced sendrawtransaction safety-limit parameters"
  - "Strong random cookie-auth credential creation with owner-only Unix file mode"
affects: [08-07, 08-08, rpc, config, security]
key-files:
  modified:
    - packages/open-bitcoin-rpc/Cargo.toml
    - packages/open-bitcoin-rpc/BUILD.bazel
    - packages/Cargo.lock
    - packages/open-bitcoin-rpc/src/dispatch.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-rpc/src/http.rs
    - packages/open-bitcoin-rpc/src/http/tests.rs
requirements-completed: [RPC-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 08-2026-04-24T02-23-19
generated_at: 2026-04-24T09:18:46Z
duration: in-progress-session
completed: 2026-04-24
---

# Phase 08 Plan 06: RPC Dispatcher And Cookie-Auth Gap Closure Summary

## Accomplishments

- `rescanblockchain` now accepts only omitted heights or the explicit full active snapshot range and rejects partial/out-of-bounds ranges before wallet mutation.
- `sendrawtransaction` now rejects explicit `maxfeerate` and `maxburnamount` values before decoding or mempool submission because Phase 8 does not enforce those limits.
- Cookie-auth creation now uses direct `getrandom` entropy and creates new Unix cookie files with owner-read/write mode.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features dispatch::tests::rescanblockchain_rejects_partial_height_ranges_without_rescanning -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features dispatch::tests::sendrawtransaction_rejects_unenforced_fee_limits_before_mempool_submission -- --exact`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features http::tests::cookie_auth_creates_owner_only_file_with_random_secret -- --exact`

## Handoff

- CLI hostname, duplicate named-parameter, and open-stdin gaps remain assigned to `08-07`.
- Parity documentation, final full verification, and human verification notes remain assigned to `08-08`.
- This interactive run has not created task-level git commits yet; commit finalization should happen after all Phase 08 gap plans pass verification.
