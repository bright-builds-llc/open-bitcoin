---
phase: 39
phase_name: "Operator Sync Observability and Control"
generated_by: gsd-execute-phase-inline
lifecycle_mode: yolo
phase_lifecycle_id: "39-2026-05-02T11-46-08"
generated_at: "2026-05-23T01:59:36Z"
status: passed
lifecycle_validated: true
---

# Phase 39 Verification

## Result

Passed. The Phase 39 gap is closed at the deterministic regression level, through a local live-shape daemon check, and through the user-rerun live mainnet UAT: live operator sync control now goes through authenticated daemon RPC, the daemon remains the single process owner of the Fjall store, status/pause/resume do not wait on the busy sync worker thread, offline direct-store control still works when no daemon is reachable, and auth failures cannot be bypassed by falling back to store mutation.

## Commands

| Command | Result | Notes |
| --- | --- | --- |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc sync_control_methods_are_open_bitcoin_node_extensions -- --nocapture` | Passed | New RPC methods normalize as node-scoped Open Bitcoin extensions and reject extra params. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoin_sync_rpc_control_updates_daemon_runtime_metadata -- --nocapture` | Passed | Dispatch routes status, pause, and resume through the daemon sync-control handle. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoin_sync_rpc_control_uses_daemon_store_backend -- --nocapture` | Passed | Store-backed daemon control responds without a sync-worker channel and persists pause/resume metadata through the daemon-opened store handle. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli sync_control_uses_live_rpc_when_datadir_store_is_locked --test operator_binary -- --nocapture` | Passed | Held Fjall store lock plus fake local RPC proves CLI does not perform second-process store opens for live control. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_sync_pause_and_resume_update_durable_control_state --test operator_binary -- --nocapture` | Passed | Offline direct-store pause/resume compatibility remains intact. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli sync_control_auth_failure_does_not_fallback_to_store --test operator_binary -- --nocapture` | Passed | Reachable daemon auth failure is terminal and does not fall back to direct store mutation. |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | Passed | Formatting clean after implementation. |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | Passed | Full workspace lint passed. |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | Passed | Full workspace build passed. |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | Passed | Full workspace tests and doctests passed. |
| `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` | Passed | Refreshed tracked LOC report after verification reported staleness. |
| `bash scripts/verify.sh` | Passed | Repo-native verification passed after LOC refresh, including Bazel smoke and coverage checks. |
| `git diff --check` | Passed | No whitespace or conflict-marker issues. |
| Local live-shape daemon check against `/tmp/open-bitcoin-mainnet-uat-codex-timeout` | Passed | Started `open-bitcoind`, then `sync status`, `sync pause`, JSON `sync status`, and `sync resume` all returned without timeout or `FjallError: Locked`; paused status showed `sync_control.paused: true`. |
| User-rerun live mainnet UAT against `/tmp/open-bitcoin-mainnet-uat` | Passed | User reported that the documented live `open-bitcoind` mainnet sync-control steps passed after rebuilding from this working tree. |

## Evidence

- [`packages/open-bitcoin-rpc/src/method.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/method.rs) registers `openbitcoinsyncstatus`, `openbitcoinsyncpause`, and `openbitcoinsyncresume` as Open Bitcoin extension methods.
- [`packages/open-bitcoin-rpc/src/context.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/context.rs) defines channel-backed and store-backed daemon sync-control backends used by RPC dispatch.
- [`packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs) starts the sync worker with a store-backed control handle cloned from the daemon process's open Fjall database.
- [`packages/open-bitcoin-node/src/storage/fjall_store.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store.rs) permits in-process store handle cloning so daemon RPC control does not open the datadir from another process.
- [`packages/open-bitcoin-cli/src/operator/runtime/support.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/runtime/support.rs) attempts authenticated local RPC before direct store access and treats reachable daemon failures as terminal.
- [`packages/open-bitcoin-cli/tests/operator_binary.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/tests/operator_binary.rs) covers the reported Fjall `Locked` failure and the auth-failure no-fallback rule.

## Residual Risks

- The daemon sync worker remains an opt-in operator review path, not a production-node claim.
