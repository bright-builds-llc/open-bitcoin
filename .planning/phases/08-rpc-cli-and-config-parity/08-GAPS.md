---

> Lifecycle repair note: This failed verifier report is preserved as the Phase 08
> gap source. It is intentionally named `08-GAPS.md` so active lifecycle
> validation ignores it until the current gap plans execute and produce a fresh
> `08-VERIFICATION.md`.
phase: 08-rpc-cli-and-config-parity
verified: 2026-04-23T05:35:22Z
status: gaps_found
score: "7/10 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 08-2026-04-23T01-44-19
generated_at: 2026-04-23T05:35:22Z
lifecycle_validated: false
overrides_applied: 0
gaps:
  - truth: "In-scope RPC methods return compatible payloads and error semantics."
    status: failed
    reason: "`rescanblockchain` validates `start_height`/`stop_height` but always rescans the full snapshot, and `sendrawtransaction` accepts `maxfeerate`/`maxburnamount` without enforcing them."
    artifacts:
      - path: "packages/open-bitcoin-rpc/src/dispatch.rs"
        issue: "`rescan_wallet(&snapshot)` ignores the requested height window, and `submit_local_transaction(transaction)` ignores the declared safety-limit fields."
    missing:
      - "Scope wallet rescans to the requested height interval, or reject unsupported range semantics explicitly."
      - "Enforce `maxfeerate` and `maxburnamount`, or reject non-default values until the limits are implemented."
  - truth: "CLI flags, config parsing, and precedence rules match the baseline for the supported surface."
    status: failed
    reason: "The CLI always drains stdin before parsing, rejects baseline-shaped hostname forms such as `-rpcconnect=localhost`, and overwrites duplicate named parameters instead of surfacing an invalid-params error."
    artifacts:
      - path: "packages/open-bitcoin-cli/src/main.rs"
        issue: "Reads stdin unconditionally, so normal interactive invocations block until EOF."
      - path: "packages/open-bitcoin-rpc/src/config/loader.rs"
        issue: "`parse_socket_address()` only accepts `SocketAddr` or `IpAddr`, rejecting hostname inputs that the client surface should accept."
      - path: "packages/open-bitcoin-cli/src/args.rs"
        issue: "`set_named_value()` replaces earlier named arguments, masking duplicates before shared normalization."
    missing:
      - "Read stdin only when `-stdin` or `-stdinrpcpass` is present."
      - "Accept baseline-shaped hostname inputs for `-rpcconnect`."
      - "Preserve or reject duplicate named arguments before transport instead of silently overwriting them."
  - truth: "Operators can run node and wallet workflows entirely through CLI and RPC without any GUI dependency."
    status: failed
    reason: "The hermetic operator-flow suite only exercises subprocesses whose stdin is already closed, so it misses the real CLI's interactive TTY hang and overstates end-user readiness."
    artifacts:
      - path: "packages/open-bitcoin-cli/src/main.rs"
        issue: "Interactive CLI usage blocks on stdin before startup or transport work begins."
      - path: "packages/open-bitcoin-cli/tests/operator_flows.rs"
        issue: "`Command::output()` closes stdin, so the suite does not cover the blocking terminal path."
    missing:
      - "Refactor stdin handling so real terminal invocations reach startup and transport immediately when stdin flags are absent."
      - "Add a regression that exercises the normal terminal path rather than only the closed-stdin subprocess path."
---

# Phase 8: RPC, CLI, and Config Parity Verification Report

**Phase Goal:** Expose the node and wallet through operator-facing interfaces that behave compatibly with the baseline for the in-scope surface.
**Verified:** 2026-04-23T05:35:22Z
**Status:** gaps_found
**Re-verification:** No — initial verification

This verification used the repo-local guidance in `AGENTS.md`, the Bright Builds sidecar in `AGENTS.bright-builds.md`, the local `standards-overrides.md`, and the pinned Bright Builds architecture, code-shape, verification, testing, and Rust standards pages.

**Lifecycle provenance:** Invalid. `08-CONTEXT.md` and the plan files carry `lifecycle_mode: yolo` plus `phase_lifecycle_id: 08-2026-04-23T01-44-19`, but `08-03-SUMMARY.md`, `08-04-SUMMARY.md`, and `08-05-SUMMARY.md` omit those provenance fields, so end-to-end lifecycle metadata is incomplete.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Workspace exposes first-party RPC and CLI shell crates through Cargo and Bazel, and repo-native verification sees them. | ✓ VERIFIED | `packages/Cargo.toml` lists `open-bitcoin-rpc` and `open-bitcoin-cli`; `BUILD.bazel` exports `//:rpc` and `//:cli`; `scripts/verify.sh` builds both aliases; `bazel query //:rpc` and `bazel query //:cli` both succeed. |
| 2 | RPC handlers have a shell-owned adapter seam and one typed contract/context layer over the managed node and wallet facades. | ✓ VERIFIED | `packages/open-bitcoin-rpc/src/context.rs` composes `ManagedPeerNetwork<MemoryChainstateStore>` and `ManagedWallet<MemoryWalletStore>`; `packages/open-bitcoin-rpc/src/method.rs` owns the supported method registry; `packages/open-bitcoin-node/src/network.rs` and `packages/open-bitcoin-node/src/wallet.rs` expose the projection/build helpers the RPC layer calls. |
| 3 | Authenticated local JSON-RPC transport exists with POST-only, batch, and notification handling for the supported Phase 8 slice. | ✓ VERIFIED | `packages/open-bitcoin-rpc/src/http.rs` rejects non-POST requests, enforces Basic auth, returns `204` for notifications, and routes batches through shared normalization and dispatch; `http::tests::legacy_and_json_rpc_v2_status_mapping_matches_phase_8_contract` and `http::tests::json_rpc_v2_notifications_return_no_content_and_execute` pass. |
| 4 | In-scope RPC methods return compatible payloads and error semantics. | ✗ FAILED | `packages/open-bitcoin-rpc/src/dispatch.rs:260`-`286` validates `start_height` and `stop_height` but always calls `rescan_wallet(&snapshot)` on the full snapshot; `packages/open-bitcoin-rpc/src/dispatch.rs:289`-`313` defines `maxfeerate` and `maxburnamount` in the request surface but never reads them before submission. |
| 5 | `bitcoin-cli` startup resolves config, datadir, endpoint, and auth on the client path itself through shared config loading. | ✓ VERIFIED | `packages/open-bitcoin-cli/src/startup.rs` calls `load_runtime_config_for_args(...)` and returns a client-owned `CliStartupConfig`; `startup::tests::client_startup_resolves_conf_datadir_and_auth_precedence` passes. |
| 6 | CLI flags, config parsing, and precedence rules match the baseline for the supported surface. | ✗ FAILED | `packages/open-bitcoin-cli/src/main.rs:11`-`16` always drains stdin before parsing; `packages/open-bitcoin-rpc/src/config/loader.rs:559`-`566` rejects `-rpcconnect=localhost`; `packages/open-bitcoin-cli/src/args.rs:394`-`403` overwrites duplicate named parameters; a live CLI run with `-rpcconnect=localhost` fails immediately with `invalid rpc address: localhost`; a live CLI run with duplicate `descriptor=` arguments reaches a network attempt instead of a collision error. |
| 7 | `-getinfo` stays a thin deterministic helper over real RPC methods. | ✓ VERIFIED | `packages/open-bitcoin-cli/src/getinfo.rs` builds a four-call batch over `SupportedMethod::{GetNetworkInfo, GetBlockchainInfo, GetWalletInfo, GetBalances}` and renders deterministic JSON in `--json` mode. |
| 8 | The CLI execution path issues authenticated requests with actionable failures and stable machine-readable output where promised. | ✓ VERIFIED | `packages/open-bitcoin-cli/src/client.rs` canonicalizes supported method parameters through `normalize_method_call(...)`, sends JSON-RPC over `ureq`, and renders deterministic JSON/errors through `output.rs`; `client::tests::rpc_errors_surface_exit_code_one_with_actionable_stderr` and `client::tests::getinfo_json_mode_is_stable_for_automation` pass. |
| 9 | Operators can run node and wallet workflows entirely through CLI and RPC without any GUI dependency. | ✗ FAILED | `packages/open-bitcoin-cli/tests/operator_flows.rs` proves the closed-stdin subprocess path only; the real binary in `packages/open-bitcoin-cli/src/main.rs:11`-`16` still blocks on interactive stdin before startup, so normal terminal usage is not reliable. |
| 10 | Parity docs explicitly state the supported baseline methods, Open Bitcoin extension methods, and deferred operator surfaces. | ✓ VERIFIED | `docs/parity/catalog/rpc-cli-config.md` lists the supported RPC/CLI/config slice plus deferred surfaces, and `docs/parity/index.json` indexes `rpc-cli-config`. |

**Score:** 7/10 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/Cargo.toml` | Workspace membership for the Phase 8 interface crates | ✓ VERIFIED | Both `open-bitcoin-rpc` and `open-bitcoin-cli` are workspace members. |
| `BUILD.bazel` | Root Bazel aliases for the Phase 8 interface crates | ✓ VERIFIED | Exports `//:rpc` and `//:cli`. |
| `scripts/verify.sh` | Repo-native verification reaches the Phase 8 shell targets | ✓ VERIFIED | Builds `//:core //:node //:rpc //:cli`. |
| `packages/open-bitcoin-rpc/src/context.rs` | Managed RPC composition root over node and wallet seams | ✓ VERIFIED | Wraps managed network and wallet facades and exposes RPC-facing helpers. |
| `packages/open-bitcoin-rpc/src/method.rs` | Typed supported-method registry and request/response contracts | ✓ VERIFIED | Enumerates the supported baseline-backed methods plus the two Open Bitcoin extension methods. |
| `packages/open-bitcoin-rpc/src/dispatch.rs` | Typed method execution over the managed RPC context | ⚠️ HOLLOW | Wired to real managed state, but `rescanblockchain` ignores the requested range and `sendrawtransaction` ignores declared safety-limit fields. |
| `packages/open-bitcoin-rpc/src/http.rs` | POST-only authenticated JSON-RPC transport | ✓ VERIFIED | Parses envelopes once, normalizes params through shared metadata, and routes into dispatch. |
| `packages/open-bitcoin-cli/src/args.rs` | Baseline-shaped CLI parsing for the supported slice | ⚠️ HOLLOW | Handles supported flags and deferred surfaces, but duplicate named arguments are silently overwritten before shared validation. |
| `packages/open-bitcoin-cli/src/startup.rs` | Explicit client-side startup and auth resolution | ⚠️ HOLLOW | Reuses the shared loader correctly, but hostname forms such as `localhost` fail inside the loader. |
| `packages/open-bitcoin-cli/src/main.rs` | Real CLI binary entrypoint | ✗ FAILED | Unconditionally drains stdin before parsing args or resolving startup, breaking normal terminal invocation. |
| `packages/open-bitcoin-cli/src/getinfo.rs` | Thin `-getinfo` helper over real RPC methods | ✓ VERIFIED | Uses shared method metadata, not a bespoke local status path. |
| `packages/open-bitcoin-cli/src/client.rs` | Thin authenticated HTTP client for the supported RPC surface | ✓ VERIFIED | Uses shared normalization and JSON-RPC request building for supported methods. |
| `packages/open-bitcoin-cli/tests/operator_flows.rs` | Hermetic end-to-end operator-flow coverage | ⚠️ PARTIAL | Proves the happy path, but only through `Command::output()` subprocesses whose stdin is already closed. |
| `docs/parity/catalog/rpc-cli-config.md` | Auditable supported-versus-deferred interface ledger | ✓ VERIFIED | Lists supported baseline-backed methods, extension methods, and deferred surfaces explicitly. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `packages/Cargo.toml` | `BUILD.bazel` | Workspace members and root aliases for RPC/CLI shells | WIRED | Cargo and Bazel both expose first-class `open-bitcoin-rpc` and `open-bitcoin-cli` targets. |
| `scripts/verify.sh` | `BUILD.bazel` | Repo-native verification builds the Phase 8 aliases | WIRED | `bazel build //:core //:node //:rpc //:cli` is in the repo verify script. |
| `packages/open-bitcoin-rpc/src/context.rs` | `packages/open-bitcoin-node/src/network.rs` | Managed RPC context depends on managed network helpers | WIRED | `ManagedRpcContext` calls `chainstate_snapshot`, `mempool_info`, `network_info`, and `submit_local_transaction`. |
| `packages/open-bitcoin-rpc/src/context.rs` | `packages/open-bitcoin-node/src/wallet.rs` | Managed RPC context depends on managed wallet helpers | WIRED | `ManagedRpcContext` calls `wallet_info`, `balance`, `utxos`, `import_descriptor`, `rescan_chainstate`, and build/sign helpers. |
| `packages/open-bitcoin-rpc/src/http.rs` | `packages/open-bitcoin-rpc/src/dispatch.rs` | HTTP transport normalizes params once and dispatches typed calls | WIRED | `handle_http_request(...)` flows through `normalize_method_call(...)` into `dispatch(...)`. |
| `packages/open-bitcoin-cli/src/startup.rs` | `packages/open-bitcoin-rpc/src/config.rs` | Client startup reuses shared config loading | WIRED | `resolve_startup_config(...)` calls `load_runtime_config_for_args(...)`, but inherits the loader's hostname restriction. |
| `packages/open-bitcoin-cli/src/getinfo.rs` | `packages/open-bitcoin-rpc/src/method.rs` | `-getinfo` batches real supported methods | WIRED | Uses `SupportedMethod` variants for all four batch calls. |
| `packages/open-bitcoin-cli/src/main.rs` | `packages/open-bitcoin-cli/src/startup.rs` | CLI binary should reach startup before request dispatch | PARTIAL | The call chain exists through `client::run_cli(...)`, but unconditional stdin draining blocks startup on normal terminal invocation. |
| `packages/open-bitcoin-cli/tests/operator_flows.rs` | `packages/open-bitcoin-rpc/src/http.rs` | Hermetic operator-flow tests drive the real CLI against the real RPC harness | WIRED | The harness boots the real HTTP layer and subprocess CLI, but only with closed stdin. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| `packages/open-bitcoin-rpc/src/context.rs` | `network_info`, `mempool_info`, `wallet_*` | `ManagedPeerNetwork` and `ManagedWallet` facades in `open-bitcoin-node` | Yes | ✓ FLOWING |
| `packages/open-bitcoin-rpc/src/dispatch.rs` | RPC payload fields for node, wallet, rescan, and transaction submission methods | `ManagedRpcContext` projections over the managed node/wallet state | Partial | ⚠️ HOLLOW — core data flows are real, but the rescan range and `sendrawtransaction` safety-limit inputs are disconnected from execution. |
| `packages/open-bitcoin-cli/src/client.rs` | `result` plus batch responses | Real HTTP JSON-RPC calls through `post_json(...)` and `extract_result(...)` | Yes | ✓ FLOWING |
| `packages/open-bitcoin-cli/tests/operator_flows.rs` | Roundtrip CLI outputs | Real CLI binary against an in-process RPC harness | Partial | ⚠️ MISLEADING — the subprocess path closes stdin, so it does not exercise the blocking interactive path in `main.rs`. |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Root RPC alias exists | `bazel query //:rpc` | Returns `//:rpc` | ✓ PASS |
| Root CLI alias exists | `bazel query //:cli` | Returns `//:cli` | ✓ PASS |
| RPC binary builds | `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --bin open-bitcoind` | Build succeeds | ✓ PASS |
| CLI binary builds | `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --bin open-bitcoin-cli` | Build succeeds | ✓ PASS |
| `sendrawtransaction` happy-path regression still passes | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features dispatch::tests::sendrawtransaction_returns_txid_and_maps_rejections -- --exact` | `1 passed` | ✓ PASS |
| HTTP status mapping regression still passes | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features http::tests::legacy_and_json_rpc_v2_status_mapping_matches_phase_8_contract -- --exact` | `1 passed` | ✓ PASS |
| CLI named/positional collision regression still passes | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features args::tests::named_arguments_reject_positional_collisions -- --exact` | `1 passed` | ✓ PASS |
| Hermetic operator flow still passes | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --test operator_flows descriptor_rescan_balance_build_sign_and_send_roundtrip -- --exact` | `1 passed` | ✓ PASS |
| Live CLI accepts baseline-style hostname input | `printf '' | cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features -- -rpcconnect=localhost getnetworkinfo` | Fails immediately with `invalid rpc address: localhost` | ✗ FAIL |
| Live CLI rejects duplicate named parameters before transport | `printf '' | cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features -- -rpcconnect=127.0.0.1:9 -rpcuser=alice -rpcpassword=secret -named deriveaddresses 'descriptor=\"...\"' 'descriptor=\"...\"'` | Falls through to `Could not connect to the server 127.0.0.1:9` instead of a duplicate-parameter error | ✗ FAIL |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| `RPC-01` | `08-01`, `08-02`, `08-03` | In-scope RPC methods, result payloads, and error semantics match the pinned baseline. | ✗ BLOCKED | The supported surface exists, but `packages/open-bitcoin-rpc/src/dispatch.rs:260`-`286` ignores `rescanblockchain` range semantics and `packages/open-bitcoin-rpc/src/dispatch.rs:289`-`313` ignores `sendrawtransaction` safety limits. |
| `CLI-01` | `08-01`, `08-02`, `08-03`, `08-04`, `08-05` | In-scope CLI commands, config-file parsing, and option precedence match the pinned baseline. | ✗ BLOCKED | `packages/open-bitcoin-cli/src/main.rs:11`-`16` blocks on stdin, `packages/open-bitcoin-rpc/src/config/loader.rs:559`-`566` rejects `localhost`, and `packages/open-bitcoin-cli/src/args.rs:394`-`403` overwrites duplicate named args. |
| `CLI-02` | `08-05` | Operators can run the node and wallet headlessly through CLI and RPC surfaces only. | ✗ BLOCKED | The hermetic roundtrip test passes, but it uses `Command::output()` in `packages/open-bitcoin-cli/tests/operator_flows.rs:510`-`530`, which closes stdin and misses the real terminal hang in `packages/open-bitcoin-cli/src/main.rs:11`-`16`. |

**Orphaned requirements:** None. `RPC-01`, `CLI-01`, and `CLI-02` are the only Phase 8 requirements in `.planning/REQUIREMENTS.md`, and all three appear in the plan frontmatter.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| `packages/open-bitcoin-rpc/src/dispatch.rs` | 281 | Requested rescan window validated but ignored | 🛑 Blocker | `rescanblockchain` can claim a bounded scan while rebuilding wallet state from the full snapshot. |
| `packages/open-bitcoin-rpc/src/dispatch.rs` | 297 | Declared transaction-safety fields unused | 🛑 Blocker | `sendrawtransaction` accepts `maxfeerate` and `maxburnamount` inputs without enforcing them. |
| `packages/open-bitcoin-cli/src/main.rs` | 14 | Unconditional stdin drain before argument parsing | 🛑 Blocker | Ordinary terminal invocations block until EOF, so the real CLI path is not operator-ready. |
| `packages/open-bitcoin-cli/src/args.rs` | 394 | Duplicate named parameters overwritten | ⚠️ Warning | Duplicate named args can bypass shared invalid-params handling and reach transport as a mutated request. |
| `packages/open-bitcoin-rpc/src/config/loader.rs` | 559 | Hostname-only `rpcconnect` rejected | ⚠️ Warning | Baseline-shaped hostname inputs such as `localhost` fail before any network call. |
| `packages/open-bitcoin-rpc/src/http.rs` | 427 | Cookie auth file created with default perms and predictable fallback secret | ⚠️ Warning | Local RPC cookie auth can be weaker than expected on systems where `/dev/urandom` is unavailable or the process umask is permissive. |

### Human Verification Required After Fixes

### 1. Interactive CLI Terminal Path

**Test:** Run `packages/target/debug/open-bitcoin-cli getnetworkinfo` from a real terminal without piping stdin.
**Expected:** The command should proceed immediately to config resolution or RPC failure, not wait for EOF.
**Why human:** TTY blocking behavior differs from the closed-stdin subprocess path used by the current automated tests.

### 2. Cookie Auth File Exposure

**Test:** Start cookie-auth RPC mode and inspect the created cookie file's permissions and contents on disk.
**Expected:** The secret should be strong and the cookie file should be owner-only readable.
**Why human:** OS-level file mode and local-user exposure are not covered by the current unit suite.

### Gaps Summary

Phase 8 has the right structural foundation: the RPC and CLI crates are first-class workspace targets, the managed node/wallet seam is real, the typed method layer exists, the HTTP transport works, the CLI client can issue requests, the happy-path operator flow is exercised, and the parity ledger is present. The phase goal still is not achieved because critical compatibility behavior is missing in the live tree, not just in the plan wording.

Two RPC semantics are materially incomplete: `rescanblockchain` does not honor its requested height range, and `sendrawtransaction` exposes `maxfeerate` and `maxburnamount` without enforcing them. On the CLI side, the real binary blocks on stdin before it even parses arguments, baseline-shaped hostname inputs such as `-rpcconnect=localhost` are rejected, and duplicate named parameters are silently overwritten. The current test suite proves the happy path, but it also contains a misleading success signal: `operator_flows.rs` uses `Command::output()`, which closes stdin and therefore cannot catch the interactive hang in the shipped CLI binary.

---

_Verified: 2026-04-23T05:35:22Z_
_Verifier: the agent (gsd-verifier)_
