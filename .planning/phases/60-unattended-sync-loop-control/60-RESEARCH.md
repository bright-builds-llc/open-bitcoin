---
phase: 60-unattended-sync-loop-control
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 60-2026-06-06T03-04-15
generated_at: 2026-06-06T03:04:15.615Z
status: complete
---

# Phase 60 Research: Unattended Sync Loop Control

## RESEARCH COMPLETE

**Question:** What do we need to know to plan Phase 60 well?

Phase 60 is a codebase-local daemon orchestration phase. The existing runtime
already has durable sync state, pause/resume control, peer backoff, target/no
progress/max-round stop reasons, metrics/log persistence, and deterministic
scripted sync tests. The missing part is an explicit, restart-safe daemon loop
policy that can be tested without starting RPC or sleeping forever.

## Relevant Existing Behavior

- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` starts RPC, preflights
  daemon sync, spawns `daemon_sync_worker`, and attaches a store-backed
  `DaemonSyncControl` to the RPC context.
- The current worker loops forever, reads pause state, runs
  `sync_until_idle`, persists a state for the summary, logs errors to stderr,
  and sleeps for `max(retry_backoff_ms, 1000)` milliseconds.
- `DurableSyncRuntime::sync_until_idle` already:
  - runs bounded sync rounds;
  - stops on `SyncStopReason::TargetHeaderReached`;
  - stops on `SyncStopReason::NoProgress`;
  - stops on `SyncStopReason::MaxRoundsReached`;
  - persists stop-reason status through `record_until_idle_stop`.
- `DurableSyncRuntime` already persists `SyncLifecycleState::Paused`,
  `Recovering`, `Active`, `Failed`, and `Stopped` through
  `durable_sync_state_from_summary`.
- Peer-level retry/backoff already uses `PeerRetryState`, `record_waiting_outcome`,
  and failure/no-credit paths in `record_outcome`.

## Recommended Implementation

Use one plan with three focused changes:

1. Add an explicit daemon-loop policy/result helper in `open-bitcoind.rs`.
   This helper should run one finite cycle and return a typed decision such as
   `sleep and retry`, `paused`, `stopped`, or `failed`. Tests can invoke the
   helper directly with deterministic runtime state and scripted transports.
2. Add additive stop reasons where needed for operator pause and shutdown.
   Existing target/no-progress/max-round reasons should stay intact.
3. Tighten docs to describe the explicit stop/retry/backoff policy and the
   release boundary: extended operator review only, not production-node
   operation.

Avoid moving daemon worker orchestration into `open-bitcoin-node` for this
phase. The node crate owns sync domain/runtime behavior, while the RPC binary
owns process lifetime, sleeps, stderr, and daemon threading. A small local
policy helper keeps the shell boundary clear.

## Validation Architecture

### Unit Tests

- `open-bitcoind` tests should cover policy message/preflight and finite helper
  behavior:
  - enabled preflight mentions "unattended review loop" and the minimum
    backoff/sleep policy;
  - paused loop cycle persists lifecycle `paused` and phase `paused`;
  - shutdown loop cycle persists lifecycle `stopped` and stop reason
    `shutdown_requested`;
  - failed loop cycle persists lifecycle `failed` and the error text.

### Runtime Tests

- `packages/open-bitcoin-node/src/sync/tests.rs` should cover additive stop
  reason projection and existing retry/backoff behavior:
  - target/no-progress stop reasons still map to existing labels and phases;
  - peer retry backoff creates waiting peer outcomes with zero useful
    contribution;
  - resource-limit/storage errors surface failed lifecycle and recovery action.

### Docs/Boundary Checks

- `docs/operator/runtime-guide.md` should describe the loop policy in terms of
  explicit activation, bounded cycles, stop reasons, pause/resume, clean
  shutdown, retry/backoff, and opt-in UAT.
- Default verification must not invoke public-network live smoke or long-run
  service checks.

## Risks And Mitigations

- **Risk:** A loop helper could accidentally hide errors and continue forever.
  **Mitigation:** Persist lifecycle `failed`, durable last error, and structured
  stop reason before sleeping/retrying.
- **Risk:** Pause/resume could race with the daemon thread.
  **Mitigation:** Re-read durable control before each cycle and before retry
  sleeps; Phase 60 only needs bounded loop semantics, not service-level
  supervised restart guarantees.
- **Risk:** New stop reasons could break existing phase names.
  **Mitigation:** Add variants additively and update projection/log tests for
  exact labels.
- **Risk:** Docs could imply production readiness.
  **Mitigation:** Keep "extended operator review" wording and preserve
  production-node/inbound/relay/wallet/migration exclusions.

## Files To Plan Against

- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`
- `packages/open-bitcoin-node/src/sync.rs`
- `packages/open-bitcoin-node/src/sync/runtime_state.rs`
- `packages/open-bitcoin-node/src/sync/types.rs`
- `packages/open-bitcoin-node/src/sync/types/projection.rs`
- `packages/open-bitcoin-node/src/sync/types/summary.rs`
- `packages/open-bitcoin-node/src/sync/tests.rs`
- `docs/operator/runtime-guide.md`
