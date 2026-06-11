---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 69-2026-06-11T15-13-14
generated_at: 2026-06-11T15:13:14.807Z
---

# Phase 69: Tip Tracking and Stay-Current Operation - Context

**Gathered:** 2026-06-11
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 69 defines the best-known mainnet tip evidence contract and turns the
post-catch-up daemon loop into truthful stay-current operation. Operators should
be able to see where the best-known tip came from, how fresh it is, how peers
agree or disagree, and whether `open-bitcoind` is still catching up, current at
the best-known tip, stale, recovering, or making no progress.

This phase owns tip evidence, freshness classification, peer agreement evidence,
and continued header/block detection after catch-up. It does not own broad
branch-competition/reorg recovery, peer-rotation expansion, long-run resource
proof, cross-surface observability closeout, default public-network checks,
inbound serving, relay, production-wallet claims, migration apply mode,
packaging, Windows service support, GUI work, hosted dashboards, or broad
production-node claims.

</domain>

<decisions>

## Implementation Decisions

### Best-Known Tip Evidence

- **D-01:** Best-known tip evidence must be a first-class typed status contract,
  not a renderer-specific string. It should include source, height, hash,
  cumulative work, timestamp, freshness, and peer agreement evidence.
- **D-02:** Prefer a deterministic peer-derived tip model: use the validated
  header store and current peer outcomes to derive best-known tip evidence.
  Do not introduce a trusted external tip oracle, checkpoint shortcut,
  assumevalid shortcut, assumeutxo shortcut, centralized peer, or public API
  dependency for the v1.6 sync-to-tip claim.
- **D-03:** Peer agreement evidence should be bounded and auditable. It should
  expose enough detail to tell whether peers agree with the best-known tip,
  lag behind it, disagree with it, or provided no useful tip evidence, without
  requiring operators to inspect raw wire transcripts.
- **D-04:** Persist enough tip evidence in runtime metadata to remain coherent
  across restart and peer rotation. Fresh peer observations may update evidence,
  but restart should not collapse prior durable tip state into "unknown" when
  the store has enough validated header and runtime metadata.

### Stay-Current State Model

- **D-05:** Add or refine a shared stay-current status classification that
  distinguishes `initial_catch_up`, `current_at_best_known_tip`, `stale_tip`,
  `recovering`, and `no_progress`. This classification should be computed in
  core sync/status code and reused by CLI, RPC, dashboard/status JSON, logs, and
  support evidence in later phases.
- **D-06:** Current-at-tip means the validated active-chain height/hash/work is
  at the best-known validated peer tip and the tip evidence is fresh enough for
  the configured deterministic policy. Downloaded-only or headers-only progress
  must not satisfy current-at-tip.
- **D-07:** Stale-tip and no-progress are different states. Stale-tip means the
  best-known tip evidence is old or lacks fresh peer agreement; no-progress
  means work remains or evidence is insufficient and the daemon is not making
  useful progress. Both should carry operator-facing next-action context.
- **D-08:** Recovering remains reserved for typed recovery contexts already
  present in the runtime, such as storage, unclean restart, or peer failure
  recovery. Phase 70 may expand recovery detail, but Phase 69 should not flatten
  recovery into stale-tip or no-progress.

### Runtime Loop Behavior

- **D-09:** After catch-up, `open-bitcoind` should continue bounded daemon wake
  cycles that request fresh headers and needed blocks, validate and connect new
  active-chain blocks, persist progress, and refresh tip evidence.
- **D-10:** Preserve the bounded opt-in daemon posture from v1.5 and Phase 68:
  no hot loops, no unbounded peer fanout, no default public-network verification,
  and no claim that the daemon is a production full node.
- **D-11:** When a daemon wake observes no new work but evidence remains fresh
  and connected progress equals the best-known tip, report stay-current success
  rather than a generic no-progress warning.
- **D-12:** When new headers are observed after catch-up, the runtime should
  transition back through catch-up behavior until the corresponding blocks are
  downloaded, validated, connected, persisted, and reflected in the shared
  progress fields added in Phase 68.

### Operator Surface Boundaries

- **D-13:** Status evidence must preserve the Phase 68 separation among header
  height, downloaded block height, connected block height, validated active-chain
  height, cumulative work, and tip freshness. Phase 69 should add tip/stay-current
  meaning without hiding those lower-level counters.
- **D-14:** Operator wording should explain whether evidence proves caught-up,
  stay-current, stale, recovering, or blocked behavior. Avoid production-node,
  inbound-serving, relay, production-wallet, migration-apply, packaging, GUI,
  hosted-dashboard, or broad readiness phrasing.
- **D-15:** Public-mainnet stay-current review remains opt-in UAT evidence.
  Default verification must remain deterministic, public-network-free,
  service-manager-free, timing-stable, and short-running.

### Verification Posture

- **D-16:** Deterministic tests should prove best-known tip projection,
  peer-agreement classification, stale-tip classification, current-at-tip
  classification, post-catch-up header/block progress, and restart persistence
  of coherent tip evidence.
- **D-17:** Add a focused deterministic checker when docs or status contracts
  need release-boundary guardrails. Keep `bash scripts/verify.sh` as the final
  repo-native verification contract.
- **D-18:** New first-party Rust source or test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` must receive
  parity breadcrumb coverage through `docs/parity/source-breadcrumbs.json` and
  `scripts/check-parity-breadcrumbs.ts`.

### the agent's Discretion

- The planner may split implementation by status-domain types, runtime summary
  projection, daemon-loop stay-current behavior, and deterministic docs/checker
  coverage if that keeps commits reviewable.
- The executor may use a small pure helper module for tip freshness and
  stay-current classification when it reduces duplicated logic across runtime
  summary, durable metadata, status JSON, and tests.
- The executor may choose conservative freshness thresholds as explicit config
  defaults, provided deterministic tests avoid wall-clock flakiness and operator
  docs describe the policy.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 69 goal, dependency on Phase 68, success
  criteria, and deferred Phase 70 through Phase 74 boundaries.
- `.planning/REQUIREMENTS.md` - TIP-01 through TIP-03, v1.6 out-of-scope table,
  and default-verification public-network exclusion.
- `.planning/PROJECT.md` - v1.6 milestone goal, explicit opt-in full-sync claim,
  pinned Knots baseline, functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - Current milestone state and recent decisions.
- `AGENTS.md` - Repo-local workflow, Rust, parity breadcrumb, and verification
  requirements.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Current local standards override registry.

### Prior Phase Decisions And Evidence

- `.planning/phases/60-unattended-sync-loop-control/60-CONTEXT.md` - Bounded
  daemon wake loop, pause/resume control, and deterministic verification posture.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` -
  Recovery categories, resource pressure, no-hot-loop expectations, and
  operator next-action boundaries.
- `.planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md` - Shared
  sync truth fields for progress signal, lag, metrics, logs, live-smoke, and
  support surfaces.
- `.planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-CONTEXT.md`
  - Same-datadir restart/resume evidence and stale in-flight cleanup posture.
- `.planning/phases/65-support-bundle-and-operator-review-docs/65-CONTEXT.md`
  - Redacted support evidence and repo-local operator command guidance.
- `.planning/phases/67-release-boundaries-and-deterministic-verification/67-CONTEXT.md`
  - Release-boundary wording, deterministic checker posture, and deferred
  production scopes.
- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md`
  - Validated active-chain progress credit and durable persistence contract.
- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-VERIFICATION.md`
  - Passed Phase 68 evidence and residual risks deferred to Phase 69.

### Implementation Surfaces

- `packages/open-bitcoin-node/src/status.rs` - Shared operator status contracts,
  sync lifecycle, progress, lag, recovery, resource pressure, and serialization.
- `packages/open-bitcoin-node/src/sync.rs` - `DurableSyncRuntime`, daemon sync
  cycles, pause/resume behavior, and public sync entrypoints.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Durable status
  projection, best-height helpers, runtime metadata persistence, and restart
  state derivation.
- `packages/open-bitcoin-node/src/sync/types.rs` - Sync runtime config,
  summaries, stop reasons, peer outcomes, runtime errors, and exported types.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - `SyncRunSummary`
  status, metrics, progress signal, lag, structured-log, and peer projection.
- `packages/open-bitcoin-node/src/sync/progress.rs` - Peer progress accounting,
  accepted/no-credit block outcomes, retry backoff, and activity timestamps.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs` - Stored block
  reconciliation and active-chain connect behavior after block bodies arrive.
- `packages/open-bitcoin-node/src/sync/block_response.rs` - Block response
  handling and no-credit peer outcomes.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Deterministic scripted
  transport, resolver, storage, status, metrics, and restart fixtures.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` - Runtime metadata
  DTOs and compatibility handling for newly persisted tip evidence.
- `packages/open-bitcoin-cli/src/operator/sync.rs` - Operator CLI sync status,
  pause, and resume rendering.
- `packages/open-bitcoin-rpc/src/context.rs` - RPC sync control bridge and
  durable metadata fallback.
- `packages/open-bitcoin-rpc/src/method/node.rs` - RPC method response shapes
  that expose `RuntimeMetadata`.
- `docs/operator/runtime-guide.md` - Current daemon sync, status, recovery,
  resource, and opt-in UAT operator guidance.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `SyncRunSummary::sync_status` already projects a shared `SyncStatus` for
  runtime, RPC, CLI, metrics, and log consumers.
- `SyncProgressSignal`, `SyncLagStatus`, `SyncRecoveryCategory`, and
  `FieldAvailability<T>` already provide the shape for typed operator states
  without renderer-specific interpretation.
- `DurableSyncRuntime::refresh_summary_progress` and
  `DurableSyncRuntime::durable_sync_state_for_summary` are the natural adapter
  points for deriving and persisting tip/stay-current evidence from validated
  headers and connected active-chain progress.
- Existing deterministic `sync::tests` fixtures cover scripted peers, runtime
  reopening, status projection, metrics, logs, storage failures, and no-credit
  peer outcomes.

### Established Patterns

- Public-network behavior is opt-in UAT and excluded from `bash scripts/verify.sh`.
- Operator status fields prefer explicit availability wrappers and typed enums
  over ad hoc strings.
- Runtime metadata changes require backward-compatible storage DTO updates and
  deterministic reopen tests.
- Progress credit is validated active-chain progress, not headers-only or
  downloaded-only progress.

### Integration Points

- Add shared tip/stay-current types in or near `status.rs`, then project them
  from sync summary/runtime state so RPC and CLI inherit the same contract.
- Extend summary/log/metric projection only where the new evidence changes the
  operator truth contract; broader cross-surface observability is Phase 72.
- Extend docs and deterministic checkers only for Phase 69 truth-boundary claims.

</code_context>

<specifics>

## Specific Ideas

- Treat "current" as a precise evidence state: connected active-chain progress
  is at the best-known validated peer tip and tip evidence is fresh.
- Keep stale-tip and no-progress separate so operators can tell whether the
  problem is old tip evidence or lack of useful work.
- Prefer bounded peer agreement rows/counts over raw transcript dumps.

</specifics>

<deferred>

## Deferred Ideas

- Reorg handling, branch competition, broader peer rotation, and detailed
  no-progress recovery belong to Phase 70.
- Long-run resource-bound proof and restart/resume resource-pressure behavior
  belong to Phase 71.
- Full cross-surface support evidence unification belongs to Phase 72.
- Public-mainnet stay-current UAT command expansion belongs to Phase 73.

</deferred>

---

*Phase: 69-tip-tracking-and-stay-current-operation*
*Context gathered: 2026-06-11*
