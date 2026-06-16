---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 78-2026-06-16T14-21-42
generated_at: 2026-06-16T14:21:42.637Z
---

# Phase 78: Progress Guarantees and Stall Diagnosis - Context

**Gathered:** 2026-06-16
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 78 makes long-run soak progress truthful and stall diagnosis actionable.
It owns PROG-01 through PROG-04: progress credit only after validated, durably
connected active-chain progress or explicit stay-current evidence; expected
progress windows, last useful work, last peer contribution, stalled subsystem,
and no-progress threshold evidence; typed distinction between public-network
reachability, incompatible peers, slow peers, stalled validation, storage
pressure, at-tip waiting, and local shutdown; and deterministic tests for false
progress, stale in-flight work, peer rotation, at-tip waiting, and validation
stalls.

This phase consumes Phase 75 soak ledger/report semantics, Phase 76 resource
bound evidence, Phase 77 recovery evidence, and the earlier Phase 69/70
stay-current and no-progress status contracts. It must not add public-network
or multi-day wall-clock checks to `bash scripts/verify.sh`, claim production
node readiness, implement Phase 79 support-bundle forensics, add inbound
serving or relay scope, or hide destructive datadir mutation behind progress
or recovery wording.

</domain>

<decisions>

## Implementation Decisions

### Progress Credit Contract

- **D-01:** Treat validated, durably connected active-chain progress as the
  only normal source of soak progress credit. Header downloads, peer messages,
  queued block requests, in-flight work, report generation, and retries may be
  evidence, but they must not advance the credited progress watermark by
  themselves.
- **D-02:** Explicit stay-current evidence may also count as useful work when
  the node is already at the best-known validated tip. That evidence must be
  derived from the existing `StayCurrentStatus`, `BestKnownTipStatus`, peer-tip
  agreement, freshness threshold, and durable status projection rather than a
  renderer-local string.
- **D-03:** Preserve Phase 70's active-chain rule: a better header branch is
  not credited as active-chain progress until its blocks are available,
  consensus-validated, connected, and durably persisted. Branch competition
  should continue to report awaiting bodies or reorg progress without claiming
  the replacement active tip early.
- **D-04:** Store progress-credit evidence as typed shared status and soak
  checkpoint fields. A checkpoint should be able to show the credited
  validated height/hash/work, the evidence kind that justified credit, the
  source timestamp, and why non-credit activity was rejected when relevant.

### Stall Diagnosis Evidence

- **D-05:** Extend the existing shared status contracts instead of building a
  soak-only stall model. `SyncProgress`, `SyncProgressSignal`,
  `last_successful_progress_unix_seconds`, `StayCurrentStatus`,
  `NoProgressDiagnosis`, `SyncResourcePressure`, `RecoveryEvidenceSnapshot`,
  peer outcomes, and reconcile progress are the starting contract.
- **D-06:** Phase 78 should add or derive explicit fields for expected progress
  window, no-progress threshold, last useful work, last peer contribution,
  stalled subsystem, and diagnosis confidence/evidence basis. Missing evidence
  must remain an unavailable field with a reason, not an omitted value.
- **D-07:** Diagnosis should distinguish public-network reachability,
  incompatible peers, slow or stalled peers, peer failures exhausted, stale
  in-flight cleanup, branch competition awaiting bodies, stalled validation,
  storage/resource pressure, current-at-tip waiting, operator stop, and local
  shutdown. Reuse existing peer failure and recovery categories where they are
  precise enough; add narrowly scoped typed variants or evidence fields only
  when the current labels cannot express PROG-03 truthfully.
- **D-08:** Storage/resource pressure and recovery evidence outrank peer retry
  advice. If Phase 76/77 evidence says the selected datadir is blocked,
  diagnosis should point to storage/resource action rather than telling the
  operator to wait for or rotate peers.

### Soak Ledger And Operator Surfaces

- **D-09:** Carry progress-guarantee and stall fields through the Phase 75
  datadir-owned soak ledger checkpoint and report projection. Reports remain
  projections; the ledger and shared status are the durable source of truth.
- **D-10:** CLI status, dashboard status, RPC status, soak reports, live-smoke
  summaries, metrics/log summaries, and support evidence should consume the
  same typed progress/stall contract. Phase 78 may update the surfaces needed
  to prove PROG-01 through PROG-04, while Phase 79 owns the broader
  "what happened" support-bundle narrative.
- **D-11:** Operator wording should stay quiet and actionable: identify the
  stalled subsystem, the evidence basis, and the next action. Avoid vague
  "sync failed" text, false "making progress" language, and production-node
  readiness claims.
- **D-12:** Local shutdown and operator stop should be separate from network,
  peer, validation, and storage stalls. A clean local stop should not be
  reported as public-network failure or validation stall.

### Deterministic Verification

- **D-13:** PROG-04 must be proven with deterministic Rust tests for the core
  decision logic: false-progress prevention, stale in-flight cleanup, peer
  rotation/backoff, at-tip waiting, validation stalls, and storage/resource
  precedence. Use synthetic chain and scripted peer/status fixtures rather than
  public peers or wall-clock multi-day sleeps.
- **D-14:** Keep pure classifiers and progress-credit decisions easy to unit
  test. Tests should focus on one concern, use Arrange/Act/Assert comments when
  setup is non-trivial, and avoid driving behavior through renderer strings.
- **D-15:** Add a focused Bun checker only for docs, parity roots, phase
  artifact anchors, required field names, and default-verification exclusions
  that Rust tests cannot prove. Keep the checker public-network-free,
  service-manager-free, and short-running.
- **D-16:** If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update
  `docs/parity/source-breadcrumbs.json` and keep
  `scripts/check-parity-breadcrumbs.ts` green.

### the agent's Discretion

- The planner may split Phase 78 across progress-credit domain/status types,
  no-progress/stall classifier extensions, soak checkpoint/report projection,
  operator surface rendering, deterministic fixtures, docs, and checker/parity
  closeout.
- The executor may add compact typed structs/enums for progress-credit evidence,
  stalled subsystem, threshold/window evidence, and no-progress basis when they
  make illegal states unrepresentable.
- The executor may preserve existing `NoProgressDiagnosis` labels and add
  adjacent evidence fields when that is simpler and less disruptive than
  expanding enum labels.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 78 goal, dependency on Phase 77, success
  criteria, and Phase 79/80 boundaries.
- `.planning/REQUIREMENTS.md` - PROG-01 through PROG-04, v1.7 out-of-scope
  boundaries, and traceability.
- `.planning/PROJECT.md` - v1.7 milestone goal, current state, pinned Knots
  baseline, functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - current focus, accumulated v1.7 decisions, and
  default-verification constraints.
- `AGENTS.md` - repo-local GSD workflow, Rust, parity breadcrumb, UAT command,
  generated artifact, and verification requirements.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - local standards override registry.
- `standards/core/architecture.md` - functional-core, parse-at-boundaries, and
  illegal-state modeling rules.
- `standards/core/code-shape.md` - early-return, optional naming, script, and
  file-size guidance.
- `standards/core/verification.md` - sync-first and repo-native verification
  requirements.
- `standards/core/testing.md` - unit-test and Arrange/Act/Assert expectations.
- `standards/languages/rust.md` - Rust module, option naming, invariant, and
  verification guidance.
- `standards/languages/typescript-javascript.md` - Bun/TS automation and
  nullish naming guidance.

### Prior Phase Decisions And Evidence

- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md`
  - active-chain validation and durable progress-credit boundaries.
- `.planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md`
  - best-known tip evidence, stay-current states, and freshness thresholds.
- `.planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md`
  - no-progress diagnosis, peer rotation, stale in-flight cleanup, and
  branch-competition rules.
- `.planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md`
  - storage/resource precedence and durable restart/resume evidence.
- `.planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md`
  - shared truth contract, cross-surface status/support alignment, and redacted
  evidence boundaries.
- `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-CONTEXT.md`
  - opt-in UAT command matrix and deterministic default-verification
  exclusions.
- `.planning/phases/74-release-boundaries-parity-and-documentation/74-CONTEXT.md`
  - release-claim and non-claim wording.
- `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md`
  - `open-bitcoin soak`, durable run ledger, checkpoint/report projection, and
  soak outcome taxonomy.
- `.planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md`
  - resource-bound status surfaces, resource-stop semantics, and deterministic
  fixture policy.
- `.planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md`
  - recovery evidence, recovery-stop semantics, probe-only status/support
  boundaries, and deterministic recovery fixtures.
- `.planning/phases/77-corruption-and-lock-recovery-hardening/77-VERIFICATION.md`
  - passed Phase 77 evidence and current readiness for Phase 78.

### Implementation And Verification Surfaces

- `packages/open-bitcoin-node/src/status.rs` - `SyncProgress`,
  `SyncProgressSignal`, `SyncStatus`, `StayCurrentStatus`,
  `NoProgressDiagnosis`, `SyncResourcePressure`, `BestKnownTipStatus`,
  `SyncReconcileProgressStatus`, and `RecoveryEvidenceSnapshot` integration.
- `packages/open-bitcoin-node/src/sync/progress.rs` - peer contribution,
  no-credit block response handling, no-progress classification, and next-action
  mapping.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - durable status
  projection, `last_successful_progress_unix_seconds` carry-forward, best-tip
  evidence, stay-current classification, no-progress diagnosis, and resource
  pressure projection.
- `packages/open-bitcoin-node/src/sync.rs` - durable sync runtime orchestration,
  current-at-tip handling, stop conditions, and progress summary production.
- `packages/open-bitcoin-node/src/sync/types.rs` and
  `packages/open-bitcoin-node/src/sync/types/summary.rs` - sync runtime
  summaries, peer outcomes, stop reasons, progress signal, lag, metrics/log
  projections, and shared status data.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs` and
  `packages/open-bitcoin-node/src/sync/block_response.rs` - active-chain
  extension, branch competition, block response attribution, stale in-flight
  work, and no-credit block paths.
- `packages/open-bitcoin-node/src/network.rs` - managed peer network and block
  connect dispositions used to distinguish validation, peer, and storage
  outcomes.
- `packages/open-bitcoin-node/src/recovery.rs` and
  `packages/open-bitcoin-node/src/status/recovery.rs` - typed recovery evidence
  and stable recovery category labels.
- `packages/open-bitcoin-node/src/sync/tests.rs` and
  `packages/open-bitcoin-node/src/sync/tests/soak.rs` - deterministic scripted
  peer, transport, storage, resource, and soak fixtures.
- `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` - checkpoint schema
  for carrying progress-guarantee and stall evidence.
- `packages/open-bitcoin-cli/src/operator/soak/runtime.rs` - checkpoint
  collection loop and outcome evaluation.
- `packages/open-bitcoin-cli/src/operator/soak/report.rs` - operator report
  projection from checkpoint fields.
- `packages/open-bitcoin-cli/src/operator/soak/outcome.rs` - soak outcome
  classification using support, recovery, resource, and no-progress evidence.
- `packages/open-bitcoin-cli/src/operator/status/render.rs`,
  `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`,
  `packages/open-bitcoin-cli/src/operator/sync.rs`,
  `packages/open-bitcoin-cli/src/operator/support/evidence.rs`, and
  `packages/open-bitcoin-rpc/src/method/node.rs` - current operator, dashboard,
  support, and RPC consumers of status/progress evidence.
- `scripts/check-phase70-reorg-recovery.ts`,
  `scripts/check-phase75-soak-runner.ts`,
  `scripts/check-phase76-resource-bounds.ts`, and
  `scripts/check-phase77-corruption-lock-recovery.ts` - recent checker patterns
  for deterministic phase closeout.
- `scripts/verify.sh` - repo-native deterministic verification contract.
- `scripts/check-parity-breadcrumbs.ts` and
  `docs/parity/source-breadcrumbs.json` - required breadcrumb mechanism for new
  first-party Rust source or test files.

### Operator Docs And Parity Roots

- `docs/operator/runtime-guide.md` - soak command forms, resource/recovery
  guidance, progress/status descriptions, and opt-in proof boundaries.
- `docs/architecture/status-snapshot.md` - shared status snapshot, progress,
  stay-current, no-progress, resource, recovery, and soak vocabulary.
- `docs/architecture/operator-observability.md` - metrics/log/support evidence
  boundaries and cross-surface observability vocabulary.
- `docs/parity/index.json`, `docs/parity/checklist.md`,
  `docs/parity/README.md`, `docs/parity/catalog/p2p.md`,
  `docs/parity/catalog/chainstate.md`, and
  `docs/parity/catalog/operator-runtime-release-hardening.md` - parity roots
  that may need Phase 78 evidence discoverability.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `SyncStatus` in `packages/open-bitcoin-node/src/status.rs` already exposes
  availability-aware sync progress, progress signal, lag, last successful
  progress timestamp, best-known tip, stay-current, no-progress diagnosis,
  resource pressure, and reconcile progress.
- `classify_no_progress` and `no_progress_next_action` in
  `packages/open-bitcoin-node/src/sync/progress.rs` already centralize the
  Phase 70 no-progress decision path and should be extended rather than
  duplicated in renderers.
- `DurableSyncRuntime::project_status` in
  `packages/open-bitcoin-node/src/sync/runtime_state.rs` is the current bridge
  from sync summaries into shared status, including carry-forward of
  `last_successful_progress_unix_seconds`, best-known tip projection,
  stay-current classification, and no-progress projection.
- `SoakCheckpointStatus` in
  `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` already carries
  recovery, no-progress, resource-bound, validated-active-chain, and
  best-known-tip checkpoint fields.
- The existing Phase 75 through Phase 77 Bun checkers provide a pattern for
  guarding docs, parity roots, field names, and default-verification
  exclusions without replacing Rust behavior tests.

### Established Patterns

- Shared status fields use `FieldAvailability<T>` so unavailable evidence is
  explicit and machine-readable. Phase 78 should follow that pattern for any
  new threshold/window/stalled-subsystem evidence.
- Pure classifiers feed thin status, soak, support, dashboard, and docs
  projection layers. Renderer-local parsing or prose-based classification is
  discouraged.
- Public-network and multi-day soak evidence stays opt-in UAT. Default
  verification must remain deterministic, short-running, service-manager-free,
  and public-network-free.
- Progress claims are intentionally conservative: active-chain progress is
  credited after validated durable connection, while best-known-tip and
  stay-current evidence explain at-tip waits.

### Integration Points

- Add progress-credit and stall evidence near `SyncStatus` and
  `DurableSyncRuntime::project_status`, then pass compact labels or structured
  fields through soak checkpoints and operator surfaces.
- Extend `classify_no_progress` or introduce a neighboring pure classifier for
  PROG-03 distinctions that are not currently represented by
  `NoProgressDiagnosis`.
- Update soak report rendering only after the typed checkpoint/status fields
  exist, so the report remains a projection of shared evidence.
- Add deterministic Rust tests around pure classifiers and runtime projection,
  then supplement with a checker for docs/parity/default-verification anchors.

</code_context>

<specifics>

## Specific Ideas

No user-provided freeform specifics were supplied in this yolo run. The
recommended path is to keep Phase 78 additive, typed, and shared-status-first,
with Rust tests proving behavior and docs/checkers proving operator wording and
parity discoverability.

</specifics>

<deferred>

## Deferred Ideas

- Phase 79 owns the richer redacted "what happened" support-bundle narrative,
  timeline reconstruction, and cross-surface forensic story. Phase 78 should
  expose the typed facts Phase 79 will later narrate.
- Phase 80 owns opt-in multi-day soak UAT command closeout, final v1.7 release
  boundary wording, and audit of public-network exclusions.
- Future production-node readiness, inbound serving, relay, production-wallet,
  migration-apply, packaging, GUI, hosted-dashboard, scheduled public soak
  monitors, and signed comparable soak artifacts remain outside Phase 78.

</deferred>

---

*Phase: 78-progress-guarantees-and-stall-diagnosis*
*Context gathered: 2026-06-16*
