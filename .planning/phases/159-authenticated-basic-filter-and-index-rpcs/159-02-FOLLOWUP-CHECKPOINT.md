---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "02"
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T23:30:24Z"
status: resolved-rust-verified-native-pending
requirements-completed: []
---

# Plan 159-02 CLI Integration Follow-up Checkpoint

The additional CLI conversion is implemented and **all six new tests have actual GREEN evidence** from root's successful Rust recovery 4. The historical host block below is resolved for Rust verification; default native run 4 remains pending. The original 159-02-SUMMARY.md preserves the earlier 18-test pure parser/contracts receipt and now records this separate follow-up closure.

## Original Failure and Implemented Fix

Root's native Rust preflight observed E0004: `client.rs::method_call_to_json` was nonexhaustive after Plan 06 introduced `MethodCall::GetBlockFilter` and `GetIndexInfo`. That compiler failure is the original RED evidence.

Existing `packages/open-bitcoin-cli/src/client.rs` now contains explicit conversion arms:

- GetBlockFilter reverses raw `BlockHash` bytes once into lowercase display hex and emits exactly named `blockhash` and `filtertype` wire keys. BASIC maps to `basic`; recognized-disabled V0 maps to `v0` so real daemon dispatch can return its pinned disabled error.
- GetIndexInfo emits `{}` for an omitted/null normalized selector, preserving all-enabled-index semantics. A present selector emits `{"index_name": name}`, preserving empty, exact BASIC, V0 and unmatched names.
- No request Serialize derive, internal `block_hash`/`filter_type`/`maybe_index_name` wire field, wildcard match, unsupported-method drop or dependency was added.

No public reusable canonical hash-display helper exists in the dependencies: primitives expose raw bytes; node helpers are private/pub(super), and the RPC raw hex encoder is private. The conversion stays local rather than widening unrelated APIs.

## Owned Existing Files and Six New Tests

| File | New test |
| --- | --- |
| `packages/open-bitcoin-cli/src/client/tests.rs` | `phase159_filter_rpc_client_blockfilter_envelope_canonicalizes_hash_and_filter` |
| `packages/open-bitcoin-cli/src/client/tests.rs` | `phase159_filter_rpc_client_indexinfo_envelope_preserves_selectors` |
| `packages/open-bitcoin-cli/src/client/tests.rs` | `phase159_filter_rpc_client_filter_method_uses_authenticated_root_endpoint` |
| `packages/open-bitcoin-cli/src/args/tests.rs` | `phase159_filter_rpc_cli_args_accept_positional_named_and_v0` |
| `packages/open-bitcoin-cli/src/args/tests.rs` | `phase159_filter_rpc_cli_args_preserve_index_selectors` |
| `packages/open-bitcoin-cli/src/args/tests.rs` | `phase159_filter_rpc_cli_args_reject_filter_parameter_errors` |

Client envelope controls compare complete serialized V2 envelopes, including an asymmetric `001122...eeff` hash supplied in uppercase/lowercase to detect missing/double reversal; positional/named/mixed/default/null inputs; BASIC/V0 wire names; and omitted/empty/exact/unmatched selectors. The real localhost HTTP control checks authentication, the root endpoint despite `-rpcwallet`, canonical request JSON and rendered response JSON. Argument controls cover selection/default preservation and exact duplicate/collision/hash/unknown-name errors.

The three modified existing files are `client.rs` (468 lines), `client/tests.rs` (601 lines) and `args/tests.rs` (324 lines). No new Rust file or parity manifest edit is required.

## Completed Static Checks

- Scoped Rust 1.94.1 rustfmt write/check on the three owned files: passed.
- `git diff --check` on the three files: passed.
- `bun scripts/bright-builds-check.ts all`: 0 findings.
- Root reports independent follow-up source review clean. This is not runtime proof.

The main binary has `test = false` in Cargo metadata. Its `main.rs` includes `mod client;`, and `client.rs` includes `#[cfg(test)] mod tests;`, so the binary must be explicitly selected to execute these tests. Plan 08 has wired the full explicit binary suite into default native verification and extended its guard to cover the six selectors, CLI ownership and conversion route. Existing client tests use local ephemeral TCP listeners; no external network service is required.

## Historical External Host Boundary: Preserved Diagnostic Abort

Observed timed command, started 2026-10-09T20:24:08.082Z:

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
bun run scripts/command-timings.ts run --key phase159-cli-filter-client-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli phase159_filter_rpc_client
```

At the final inspection around 20:35:26 UTC it remained active after more than eleven minutes, **before the CLI build script entered Rust main**:

- Exec session: `4757`, now completed with Cargo exit 101 after the explicitly authorized diagnostic abort below.
- Timing wrapper PID: `92686`; Cargo PID: `92691`.
- CLI `build-script-build` PID: `93108`, path `packages/target/debug/build/open-bitcoin-cli-b1497436526aa722/build-script-build`.
- Process state `S`; build script CPU time `0:00.00`; Cargo CPU time `0:00.24`.
- Initial sample `/tmp/open-bitcoin-phase159-cli-client-red.sample`: 807/807 samples at `_dyld_start`, 96 KiB footprint, no Rust frames. Fresh final sample `/tmp/open-bitcoin-phase159-cli-client-final.sample`: 799/799 samples at the same `_dyld_start`, 96 KiB footprint, no Rust frames.
- Read-only macOS policy-log query for current PID/build variant over the preceding two minutes returned no entries. No definitive current policy denial was observed.
- Timing record now reports `outcome: failure`, `exitStatus: 101`, `endedAt: 2026-10-09T20:35:40.499Z`, duration 692,417 ms; record path `.local/open-bitcoin-dev/command-timings/phase159-cli-filter-client-red/2026-10-09T20-24-08.082Z-96d86ceb-e924-4be6-8bdb-2ade47ef4878.json`. This raw wrapper outcome denotes the diagnostic abort, not a compiler/test failure.

The command was polled at intervals below 60 seconds. After the fresh final sample confirmed the unchanged pre-main state, root explicitly instructed a targeted diagnostic stop. Only the stalled build-script child PID 93108 received `SIGTERM`; neither Cargo nor other processes were killed. Cargo then reported its custom build command terminated by signal 15 and exited 101; the timing wrapper completed normally. A final process inventory found all three attempt PIDs gone, releasing the build slot. No timeout, cache cleaning, signing, xattr, or system-policy change was performed. **This is an external execution block and explicit diagnostic abort, not evidence of a code failure or pass.**

The key includes `-red` because it was launched before the implementation; the source was fixed while the process was paused before compilation of the client. It never reached that compilation or a test harness. Do not claim that command as RED; root's original E0004 is the observed failure.

## Historical Verification Queue

Session `4757` completed after the targeted diagnostic abort. At that time root instructed this executor to start no further builds/tests pending recovery of the external host execution boundary. The following commands were the recorded queue, not additional runs claimed by this executor:

Recorded continuation commands with the pinned PATH:

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
bun run scripts/command-timings.ts run --key phase159-cli-filter-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-targets --all-features -- -D warnings
bun run scripts/command-timings.ts run --key phase159-cli-filter-client -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --all-features
bun run scripts/command-timings.ts run --key phase159-cli-filter-args -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib --all-features args::
bun run scripts/command-timings.ts run --key phase159-validation-history-store-rename -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase159_validation_history_store_
```

The last selector was root's requested three-test check of the canonical `runtime_state/tests.rs` rename. Root subsequently ran full workspace verification, which covers those tests; no separate focused invocation of that selector is claimed here.

## Resolution: Actual Rust Recovery 4 Passed

Root's timed `phase159-native-rust-recovery` attempt started at 2026-10-09T23:00:24.065Z and ended at 23:25:36.767Z with **exit 0**, duration 1,512,702 ms. The persisted timing record was read during this update: `.local/open-bitcoin-dev/command-timings/phase159-native-rust-recovery/2026-10-09T23-00-24.065Z-0cca7b01-5e69-4178-8f24-573cbc697283.json`.

- Full workspace formatting, strict Clippy across all targets/features, and build passed.
- Workspace tests/doctests: **3,857 passed**, 3 ignored.
- CLI library suite: **365 passed**, including all three new argument controls listed above; these tests are included in the workspace total.
- Explicit `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --all-features`: **7 passed**, including all three new client controls listed above.
- All six new tests therefore executed and passed; the E0004 integration defect has actual GREEN proof, separate from the prior loader diagnostic abort.

The request-helper/envelope and authenticated node-root routing assertions are actual executed tests. The pinned CLI command forms used by README/UAT remain coherent with this conversion. Root reports the follow-up source review clean (91 checks) and 611 guard tests passed.

At this update, **default native run 4 is still running in root session 19945**. No default native pass is claimed. Root still owns completion of that native gate, formal verification, final source/security/lifecycle review, requirement completion and authorized Git finalization. `requirements-completed` remains empty. This executor performed no additional Cargo/test run and changed only the two execution artifacts during this resolution update; no staging, commit, push or STATE/ROADMAP/config mutation was performed.
