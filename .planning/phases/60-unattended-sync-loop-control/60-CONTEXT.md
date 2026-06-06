---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 60-2026-06-06T03-04-15
generated_at: 2026-06-06T03:04:15.615Z
---

# Phase 60: Unattended Sync Loop Control - Context

**Gathered:** 2026-06-06
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 60 makes the existing explicit opt-in `open-bitcoind` mainnet sync worker
behave as a bounded unattended daemon loop after RPC binds. It owns loop policy,
stop-reason persistence, pause/resume/shutdown behavior, and deterministic
tests for retry/backoff and no-progress stop handling.

This phase does not add inbound serving, address advertisement, transaction
relay, mempool propagation, production-funds wallet claims, destructive
migration apply mode, packaging/distribution polish, or a broad production-node
claim. Public-network long-run checks remain opt-in UAT evidence and stay
outside `bash scripts/verify.sh`.

</domain>

<decisions>

## Implementation Decisions

### Loop Activation And Policy

- **D-01:** Reuse the existing explicit `sync.network_enabled = true` plus
  `sync.mode = "mainnet-ibd"` / `-openbitcoinsync=mainnet-ibd` activation as
  the opt-in setting for unattended review. Do not add an implicit default-on
  daemon mode in Phase 60.
- **D-02:** Make the daemon worker policy explicit and testable: each wake runs
  one bounded `sync_until_idle` cycle, persists the resulting durable state,
  sleeps at least the configured retry backoff, and then retries only if the
  operator has not paused or shut down the loop.
- **D-03:** Preserve the existing peer-level retry/backoff semantics. Failed or
  waiting peers must not be credited with useful progress, peer IDs must remain
  bounded by configured candidate peers per cycle, and no hot loop may occur
  when all peers are failing or waiting.

### Stop Reasons And Lifecycle

- **D-04:** Persist explicit stop reasons through durable status instead of
  relying on stderr text. Phase 60 must cover target header reached, no progress,
  max rounds, operator pause, shutdown, storage failure, resource exhaustion, and
  incompatible/failed peers through existing or additive typed status fields.
- **D-05:** Pausing the loop should produce durable lifecycle `paused` and phase
  `paused` while preserving the latest progress, last error, and next-action
  guidance from prior sync cycles when available.
- **D-06:** Clean daemon shutdown should persist lifecycle `stopped` with a
  shutdown stop reason or last error so later status/restart review can
  distinguish intentional stop from failure. Phase 64 can expand supervised
  restart evidence; Phase 60 only needs the loop boundary to be restart-safe.
- **D-07:** Storage failures and resource exhaustion should fail closed as
  lifecycle `failed`, with operator recovery guidance coming from existing
  storage/resource error mappings.

### Operator Control Surface

- **D-08:** Existing RPC/store-backed `open-bitcoin sync pause`, `resume`, and
  `status` remain the control surface. Phase 60 should make these controls
  work cleanly with the daemon loop; it should not add a second competing
  control file or require operators to edit durable metadata manually.
- **D-09:** Status wording should describe extended operator review readiness,
  not unattended production-node operation. Existing docs should be tightened
  where the new loop policy is introduced.

### Verification Posture

- **D-10:** Deterministic Rust tests should drive Phase 60. Use injectable
  loop-cycle helpers or scripted transports rather than public-network tests.
- **D-11:** Public-mainnet loop review commands may be documented as opt-in UAT,
  but `bash scripts/verify.sh` must remain deterministic and must not invoke
  live smoke, manual peers, or long-running daemon review.

### the agent's Discretion

- The planner may introduce a small daemon-loop policy type in the RPC binary if
  that keeps the implementation testable without moving network transport into
  pure-core crates.
- The planner may add an additive `SyncStopReason` variant for operator pause or
  shutdown if existing lifecycle fields are not enough to make stop reasons
  grep-visible and durable.
- The executor may keep Phase 60 code in existing files when that is the least
  risky path. If new first-party Rust files are added, parity breadcrumbs must be
  updated.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 60 goal, success criteria, and dependency on
  Phase 59.
- `.planning/REQUIREMENTS.md` - LOOP-01 through LOOP-04 and v1.5 out-of-scope
  boundaries.
- `.planning/PROJECT.md` - v1.5 milestone goal and release-boundary constraints.
- `.planning/STATE.md` - Current milestone state and prior decisions affecting
  deterministic verification and raw phase history retention.

### Prior Phase Evidence

- `.planning/phases/55-outbound-handshake-compatibility-fixes/55-CONTEXT.md` -
  Peer failure, retry, and no-credit compatibility behavior.
- `.planning/phases/55-outbound-handshake-compatibility-fixes/55-01-SUMMARY.md`
  - Completed duplicate-version and retry-backoff behavior.
- `.planning/phases/56-header-ibd-convergence/56-CONTEXT.md` - Header target
  and no-progress stop decisions.
- `.planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md` - Completed
  `sync_until_idle` stop-reason and target-height evidence.
- `.planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md` -
  Block progress and invalid/no-credit peer outcome decisions.
- `.planning/phases/57-block-download-and-connect-progress/57-04-SUMMARY.md` -
  Completed block progress and runtime resource-bound evidence.
- `.planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md`
  - Same-datadir restart/resume boundaries and recovery diagnosis.
- `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md`
  - Operator evidence, release-boundary, and deterministic verification posture.
- `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-VERIFICATION.md`
  - Passed v1.4 closeout evidence and residual boundary notes.

### Implementation Surfaces

- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Daemon startup,
  preflight, worker loop, durable state seeding, and tests.
- `packages/open-bitcoin-rpc/src/config.rs` - Daemon sync mode contract and
  runtime config surface.
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` - JSONC sync activation
  and bounds schema.
- `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` - Config
  validation and sync runtime overrides.
- `packages/open-bitcoin-rpc/src/context.rs` - RPC/store-backed pause, resume,
  and status control.
- `packages/open-bitcoin-node/src/sync.rs` - `DurableSyncRuntime`,
  `sync_until_idle`, peer retry/backoff, and stop-reason behavior.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Durable sync state,
  control state, metrics/log persistence, and resource pressure projection.
- `packages/open-bitcoin-node/src/sync/types.rs` - `SyncRuntimeConfig`,
  `SyncStopReason`, `SyncRuntimeError`, peer outcome, and failure reason types.
- `packages/open-bitcoin-node/src/sync/types/projection.rs` - Sync phase names
  for stop reasons and peer waiting.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Status, metrics, and
  structured-log projection of summaries and stop reasons.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Scripted transport/resolver
  fixtures for deterministic sync loop behavior.
- `packages/open-bitcoin-node/src/status.rs` - Durable lifecycle and control
  status contracts.
- `docs/operator/runtime-guide.md` - Operator sync activation, pause/resume,
  resource-bound, and release-boundary guidance.

### Baseline Anchors

- `packages/bitcoin-knots/src/bitcoind.cpp` - Daemon startup anchor.
- `packages/bitcoin-knots/src/init.cpp` - Shutdown/datadir lifecycle anchor.
- `packages/bitcoin-knots/src/net.cpp` - Peer connection/retry lifecycle anchor.
- `packages/bitcoin-knots/src/net_processing.cpp` - Peer sync, invalid data,
  and no-credit progress attribution anchor.
- `packages/bitcoin-knots/src/headerssync.cpp` - Header sync stop/progress
  behavior anchor.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `open-bitcoind` already starts a background thread when daemon sync is
  enabled and stores a `DaemonSyncControl` in the RPC context.
- `DurableSyncRuntime::sync_until_idle` already stops on target header, no
  progress, and max rounds, then persists durable state through
  `record_until_idle_stop`.
- `DurableSyncRuntime` already tracks per-peer backoff with waiting outcomes and
  no-credit failure paths.
- `DaemonSyncControl::store_backed` already lets CLI/RPC pause and resume the
  durable control flag without direct file editing.
- `DurableSyncState` already carries lifecycle, phase, progress signal, last
  error, recovery action, peer telemetry, and resource pressure.

### Established Patterns

- Sync logic lives in `open-bitcoin-node`; `open-bitcoind` owns process/thread
  orchestration.
- Operator docs must use repo-local Cargo/Bazel commands for workflows.
- Deterministic verification must avoid public-network commands.
- Status fields should report unavailable or typed states instead of inventing
  broad success flags.

### Integration Points

- The daemon loop should call a small policy helper from
  `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` so tests can run one
  cycle without launching Axum or sleeping indefinitely.
- Additive stop reasons should flow through `SyncStopReason::label`,
  `SyncStopReason::message`, `sync_phase_name`, structured logs, and durable
  status.
- If pause or shutdown uses an additive stop reason, tests should assert both
  the stop reason label and the durable lifecycle/phase fields.

</code_context>

<specifics>

## Specific Ideas

No additional user-specific requests beyond the v1.5 milestone prompt. Use the
standard Open Bitcoin approach: opt-in, bounded, auditable, deterministic by
default, and clear about deferred production-node scope.

</specifics>

<deferred>

## Deferred Ideas

- Service supervisor restart policy, clean-vs-unclean supervised restart
  evidence, and same-datadir service resume are Phase 63 and Phase 64 work.
- Long-run metrics/log/support-bundle cycle summaries are Phase 62 and Phase 65
  work.
- Compatibility harness operator wrapper is Phase 66 work.
- Production-node, inbound-serving, relay, production-funds wallet, destructive
  migration apply, and packaging/distribution claims remain future milestones.

</deferred>

---

*Phase: 60-unattended-sync-loop-control*
*Context gathered: 2026-06-06*
