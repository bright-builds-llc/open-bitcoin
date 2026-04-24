---
phase: 08-rpc-cli-and-config-parity
verified: 2026-04-24T09:26:58Z
status: passed
score: "10/10 phase truths verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 08-2026-04-24T02-23-19
generated_at: 2026-04-24T09:26:58Z
lifecycle_validated: true
---

# Phase 8: RPC, CLI, and Config Parity Verification Report

**Phase Goal:** Expose the node and wallet through operator-facing interfaces that behave compatibly with the baseline for the in-scope surface.
**Verified:** 2026-04-24T09:26:58Z
**Status:** passed
**Re-verification:** Yes — validates the gap closures preserved in `08-GAPS.md`

This verification used the repo-local guidance in `AGENTS.md`, the Bright Builds sidecar in `AGENTS.bright-builds.md`, `standards-overrides.md`, and the pinned Bright Builds architecture, code-shape, verification, testing, and Rust standards pages.

## Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Workspace exposes first-party RPC and CLI shell crates through Cargo and Bazel, and repo-native verification sees them. | VERIFIED | `packages/Cargo.toml`, `BUILD.bazel`, and `bash scripts/verify.sh` all cover the RPC and CLI crates. |
| 2 | RPC handlers have a shell-owned adapter seam and one typed contract/context layer over the managed node and wallet facades. | VERIFIED | `packages/open-bitcoin-rpc/src/context.rs`, `packages/open-bitcoin-rpc/src/method.rs`, and dispatcher tests cover the managed seam. |
| 3 | Authenticated local JSON-RPC transport exists with POST-only, batch, notification, cookie-auth, and Basic-auth handling for the supported Phase 8 slice. | VERIFIED | `packages/open-bitcoin-rpc/src/http.rs` and `http::tests::cookie_auth_creates_owner_only_file_with_random_secret` passed. |
| 4 | In-scope RPC methods return compatible payloads and explicit error semantics. | VERIFIED | `rescanblockchain` now rejects unsupported partial ranges, and `sendrawtransaction` now rejects explicit `maxfeerate` and `maxburnamount` values before dispatch. |
| 5 | `bitcoin-cli` startup resolves config, datadir, endpoint, and auth on the client path itself through shared config loading. | VERIFIED | `startup::tests::client_startup_resolves_conf_datadir_and_auth_precedence` and hostname startup coverage passed. |
| 6 | CLI flags, config parsing, and precedence rules match the supported baseline-shaped surface. | VERIFIED | Hostname `-rpcconnect` inputs preserve explicit `-rpcport` over embedded-port over chain-default precedence; server bind validation remains socket-only. |
| 7 | `-getinfo` stays a thin deterministic helper over real RPC methods. | VERIFIED | Existing `getinfo` and client tests passed under full `cargo test --all-features`. |
| 8 | The CLI execution path issues authenticated requests with actionable failures and stable machine-readable output where promised. | VERIFIED | `client::tests::rpc_errors_surface_exit_code_one_with_actionable_stderr` and `getinfo_json_mode_is_stable_for_automation` passed. |
| 9 | Operators can run node and wallet workflows entirely through CLI and RPC without GUI dependency or hidden stdin blocking. | VERIFIED | `operator_flows::normal_cli_without_stdin_flags_does_not_wait_for_open_stdin` passed with an open stdin pipe, and the roundtrip operator flow passes with full-snapshot rescan semantics. |
| 10 | Parity docs explicitly state the supported baseline methods, Open Bitcoin extension methods, rejected gap semantics, and deferred operator surfaces. | VERIFIED | `docs/parity/catalog/rpc-cli-config.md` now anchors closed gap semantics and deferred entries for `sendtoaddress`, `rpcauth`, `rpcwhitelist`, `rpcwallet`, `getpeerinfo`, and `-netinfo`. |

## Closed Gaps

| Gap Source | Status | Evidence |
| --- | --- | --- |
| `rescanblockchain` accepted unsupported range-shaped inputs while rescanning the full snapshot. | CLOSED | `dispatch::tests::rescanblockchain_rejects_partial_height_ranges_without_rescanning` passed; operator flow uses omitted heights for full active-snapshot rescans. |
| `sendrawtransaction` exposed `maxfeerate` and `maxburnamount` without enforcing them. | CLOSED | `dispatch::tests::sendrawtransaction_rejects_unenforced_fee_limits_before_mempool_submission` passed. |
| Cookie auth file creation relied on weak fallback behavior and default permissions. | CLOSED | `http::tests::cookie_auth_creates_owner_only_file_with_random_secret` passed; docs and summary record expected `__cookie__:<64 lowercase hex chars>` and `0600` mode. |
| `-rpcconnect=localhost` and other hostname endpoints failed before transport. | CLOSED | `config::tests::rpcconnect_accepts_hostnames_and_preserves_port_precedence` and `startup::tests::client_startup_preserves_rpcconnect_hostnames` passed. |
| Duplicate named CLI parameters were overwritten before shared normalization. | CLOSED | `method::tests::named_params_distinguish_duplicate_keys_from_positional_collisions` and `args::tests::named_arguments_reject_duplicate_keys_before_transport` passed. |
| The real CLI binary drained stdin unconditionally and could hang with open stdin. | CLOSED | `args::tests::stdin_requirement_detection_matches_stdin_flags` and `operator_flows::normal_cli_without_stdin_flags_does_not_wait_for_open_stdin` passed. |

## Requirements Coverage

| Requirement | Status | Evidence |
| --- | --- | --- |
| `RPC-01`: In-scope RPC methods, result payloads, and error semantics match the pinned baseline for the supported surface. | COMPLETE | RPC dispatch, HTTP, and full repo verification passed. |
| `CLI-01`: In-scope CLI commands, config-file parsing, and option precedence match the pinned baseline for the supported surface. | COMPLETE | Hostname endpoint, duplicate named-parameter, startup, and client tests passed. |
| `CLI-02`: Operators can run the node and wallet headlessly through CLI and RPC surfaces only. | COMPLETE | Operator flow and open-stdin subprocess regressions passed. |

## Verification Commands

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bash scripts/verify.sh`
- `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 08 --require-plans`
- `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" phase-plan-index 08`

## Human Notes

No blocking human verification remains. Optional operator spot checks are recorded in `08-08-SUMMARY.md` for a real terminal no-stdin invocation and cookie file inspection.

## Lifecycle

`08-CONTEXT.md`, all eight plans, all eight summaries, and this verification report carry `lifecycle_mode: yolo` and `phase_lifecycle_id: 08-2026-04-24T02-23-19`. The previous failed verifier report remains preserved as `08-GAPS.md` for audit history.
