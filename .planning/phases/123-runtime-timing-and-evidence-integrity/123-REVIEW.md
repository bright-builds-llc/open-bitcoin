---
phase: 123-runtime-timing-and-evidence-integrity
reviewed: 2026-07-16T05:19:02Z
depth: standard
diff_base: 731f70e8334bad9980f3a68a6f50e7019885fdd4
files_reviewed: 35
files_reviewed_list:
  - docs/architecture/operator-observability.md
  - docs/metrics/lines-of-code.md
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - packages/open-bitcoin-bench/src/runtime_fixtures.rs
  - packages/open-bitcoin-network/src/message.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/logging.rs
  - packages/open-bitcoin-node/src/logging/tests.rs
  - packages/open-bitcoin-node/src/metrics/block_relay.rs
  - packages/open-bitcoin-node/src/metrics/tests.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/metrics.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
  - packages/open-bitcoin-node/src/sync/session.rs
  - packages/open-bitcoin-node/src/sync/tcp.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs
  - packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs
  - packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs
  - packages/open-bitcoin-node/src/sync/types.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/inbound_listener.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - scripts/check-phase121-block-relay-metrics-log-runtime.test.ts
  - scripts/check-phase121-block-relay-metrics-log-runtime.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity.ts
  - scripts/verify.sh
findings:
  critical: 2
  warning: 2
  info: 0
  total: 4
status: issues_found
---

# Phase 123: Code Review Report

**Reviewed:** 2026-07-16T05:19:02Z
**Depth:** standard
**Files Reviewed:** 35
**Diff Base:** `731f70e8334bad9980f3a68a6f50e7019885fdd4`
**Status:** issues_found

## Summary

Reviewed the complete 35-file Phase 123 scope against the parent of the earliest Phase 123 commit, the phase context and plans, and the live daemon, sync-session, inbound-listener, evidence, persistence, and checker call chains. Repo-local guidance materially used for this review: `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/operability.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md`.

The typed post-write acknowledgements, direct same-snapshot metric/log projection, explicit idle receive outcome, parity updates, and deterministic checkers are internally consistent and pass their focused suites. However, the live sync and inbound managed networks both discard the resolved block-relay activation policy, so the tests exercise enabled states that operator configuration cannot create. The idle loop also retains the original cycle timestamp for messages received after a wake and has no path back to the daemon control loop for a silent open peer.

Verification performed:

- `bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts` passed (21 tests)
- `bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts` passed
- `bun test scripts/check-phase121-block-relay-metrics-log-runtime.test.ts` passed (13 tests)
- `bun run scripts/check-phase121-block-relay-metrics-log-runtime.ts` passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase123_ -- --nocapture` passed through the repo timing wrapper (22 tests)
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc phase123_inbound_ -- --nocapture` passed through the repo timing wrapper (7 tests)
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` passed
- `bash -n scripts/verify.sh` passed
- Scoped `git diff --check 731f70e8334bad9980f3a68a6f50e7019885fdd4..HEAD` passed

The full `bash scripts/verify.sh` contract was not rerun during this review; focused Rust and Bun coverage was used instead.

## Critical Findings

### CR-01: The Daemon Sync Network Discards Compact-Relay Activation

**Files:** `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:370`, `packages/open-bitcoin-node/src/sync.rs:77`
**Issue:** The daemon resolves an operator `runtime.block_serving` policy, but `start_daemon_sync_worker` passes only `runtime.sync.runtime` into `DurableSyncRuntime::open`. That constructor creates its authoritative network with `ManagedPeerNetwork::with_sync_limits`, whose construction path installs `BlockRelayActivationPolicy::default()` (both block serving and compact relay disabled). No production call later changes the policy. Consequently, a configured daemon cannot start the compact-download state whose idle timeout Phase 123 is meant to expire, and its authoritative metric/log snapshot cannot report enabled compact-runtime activity. The Phase 123 timing test reaches this state only by mutating the private peer manager at `runtime_timing_cases.rs:342`, while the projection test replaces the entire private network at `runtime_projection_cases.rs:39`; neither exercises production construction. This leaves HARD-02 and the live half of HARD-04 unreachable through the operator surface despite passing tests.
**Fix:** Thread the resolved `BlockRelayActivationPolicy` into `DurableSyncRuntime` construction and create `self.network` with that policy. Pass `runtime.block_serving` from `start_daemon_sync_worker`, preserving default-off behavior when the resolved policy is disabled. Rewrite the focused timing/projection setup to construct the runtime through the production activation path instead of private mutation or network replacement.

### CR-02: The Inbound Runtime Also Discards Block-Serving Activation

**File:** `packages/open-bitcoin-rpc/src/context/network.rs:75`
**Issue:** `ManagedRpcContext::from_runtime_config_with_store` receives the full `RuntimeConfig`, but builds its network with `ManagedPeerNetwork::new_with_relay_activation(config.relay, config.inbound.enabled)`. That constructor installs the default-disabled `BlockRelayActivationPolicy`; `config.block_serving` is never supplied or set later. Therefore, enabling block serving in config cannot make the live inbound listener produce a `WireNetworkMessage::Block`, so Phase 123's new successful-write acknowledgement at `inbound_listener.rs:551` is unreachable through a real configured `GetData(Block)` request. The seven focused tests invoke the acknowledgement bridge with constructed responses and assert the private counter, which verifies ordering but misses the production activation path. This leaves the inbound half of D-05/HARD-03 disconnected from live operator configuration.
**Fix:** Construct the RPC context network with `ManagedPeerNetwork::new_with_block_relay_activation`, passing `config.relay`, `config.block_serving`, and `config.inbound.enabled`. Add an integration test that builds `ManagedRpcContext` from an enabled runtime config, serves an eligible block request through the normal inbound message path, acknowledges a `Written` outcome, and observes exactly one private served write; retain a disabled-config negative case.

## Warnings

### WR-01: Messages After An Idle Wake Use The Stale Cycle Timestamp

**File:** `packages/open-bitcoin-node/src/sync.rs:388`
**Issue:** On `SyncPeerReceiveOutcome::Idle`, the loop samples the injected clock into `now_unix_seconds` and expires compact timeouts correctly. It then discards that value. A later message in the same live session still records activity with the original cycle `timestamp` at line 419 and passes that same stale timestamp into `receive_sync_message` at lines 456-461. Compact-download initialization records the supplied timestamp as its start time. For an `Idle(t=100) -> CompactBlock -> Idle(t=101)` sequence from a cycle started at `t=0`, the new download is recorded as starting at `t=0` and can expire immediately on the next wake rather than after the configured interval. Runtime activity timestamps are similarly stale after the first idle. The current test script places `CompactBlock` before its first `Idle`, so it cannot detect this ordering.
**Fix:** Maintain a session-local current timestamp, update it from the caller clock on each idle wake (or sample the injected clock for every received message), and use it for subsequent progress and network processing. Add a deterministic `Idle -> late CompactBlock -> Idle` regression test proving no fallback before a full timeout measured from actual receipt and fallback after that timeout.

### WR-02: A Silent Open Peer Can Block Sync Progress And Daemon Shutdown Indefinitely

**Files:** `packages/open-bitcoin-node/src/sync.rs:370`, `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:395`
**Issue:** The connected-session loop is bounded only by `messages_received < max_messages_per_peer`. Idle wakes deliberately do not increment that counter and always `continue`, so a TCP peer that keeps the connection open without sending a header can produce `Idle` forever. While that happens, the daemon worker remains inside `sync_until_idle_with_clock`; it cannot check `shutdown_receiver`, try another peer, persist the end-of-tick metrics/log snapshot, or return to its cadence. `DaemonSyncWorker::shutdown` sends the shutdown signal and then joins the worker, so shutdown can hang indefinitely on the silent peer. This is a remote availability and operability regression from treating read timeout as a terminal receive outcome.
**Fix:** Give the live-session maintenance loop a bounded, cancellation-aware yield policy. For example, pass a shutdown/continue callback into the session driver and return control after a bounded number or duration of idle pulses while preserving the connection only when the owning loop can resume it safely. Add a perpetual-idle transport regression proving bounded return or prompt cancellation, and a daemon-worker test proving shutdown completes while a connected peer stays silent.

***

_Reviewed: 2026-07-16T05:19:02Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
