---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 75-2026-06-14T22-59-23
generated_at: 2026-06-14T23:04:34.486Z
---

# Phase 75: Multi-Day Soak Runner and Evidence Ledger - Context

**Gathered:** 2026-06-14
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 75 starts v1.7 by giving operators an explicit opt-in full-sync soak
workflow with bounded inputs, durable run identity, resumable evidence, typed
stop outcomes, and deterministic synthetic coverage. It owns SOAK-01 through
SOAK-04 only: multi-day soak execution, operator bounds, stop-reason evidence,
and public-network-free synthetic tests.

This phase may define the operator command, ledger schema, report projection,
and first deterministic coverage for the soak lifecycle. It must not move
public-network or multi-day wall-clock checks into `bash scripts/verify.sh`,
claim broad production-node readiness, add inbound serving or relay scope,
perform destructive datadir repair, automatically upload support bundles, or
pre-build the deeper Phase 76 through Phase 79 resource, corruption, progress,
and forensic diagnosis scope.

</domain>

<decisions>

## Implementation Decisions

### Operator Invocation And Bounds

- **D-01:** Make `open-bitcoin soak` the stable operator-facing entrypoint for
  Phase 75. Keep it explicit opt-in, surfaced through repo-local Cargo and
  Bazel command forms, and separate from default verification.
- **D-02:** Use a layered contract: `open-bitcoin soak` owns argument parsing,
  run identity, ledger/report paths, resume mode, and final operator output;
  `open-bitcoind` and Open Bitcoin runtime/config inputs remain authoritative
  for daemon-owned sync bounds such as network, target height, peer policy, and
  runtime stop behavior.
- **D-03:** Treat `scripts/run-live-mainnet-smoke.ts` as a compatibility,
  fixture, or opt-in evidence helper rather than the durable soak product
  surface. Reuse its report, preflight, and deterministic fixture lessons where
  useful, but do not grow it into the primary multi-day soak runner.
- **D-04:** The soak command should accept or derive bounded elapsed time,
  target height, datadir, network, peer policy, disk budget, and stop conditions
  without creating hidden public-network defaults or implicit source-datadir
  mutation.

### Durable Evidence Ledger

- **D-05:** Use a hybrid evidence model: a small datadir-owned run index or
  current-run pointer anchors durable identity and resume ownership, while a
  typed append-only JSONL event ledger records started, checkpoint, resume,
  stop, and verdict events.
- **D-06:** Derive shareable JSON and Markdown reports from the ledger. Reports
  are operator artifacts, not the source of truth; stale or moved reports must
  not be mistaken for current durable state.
- **D-07:** Support bundles may include a compact, redacted soak summary derived
  from the ledger, but support bundles are projections only. They must not
  become the primary ledger and must not embed raw daemon logs, raw reports,
  wallet material, credentials, unbounded peer tables, or automatic uploads.
- **D-08:** Ledger writes should be typed, versioned, bounded, and resilient to
  partial/interrupted runs. Planning should define atomic write behavior,
  retention or compaction boundaries, and how the run index detects the latest
  resumable run.

### Run Outcome And Resume Taxonomy

- **D-09:** Add a soak-owned run outcome vocabulary for Phase 75 rather than
  overloading `SyncStopReason` or `SyncRecoveryCategory`. Required final labels
  are clean completion, diagnosed blocker, operator stop, resource stop,
  recovery stop, and unexpected termination.
- **D-10:** Every soak outcome must carry source evidence from existing shared
  contracts where possible: `SyncStopReasonStatus` for bounded sync stops,
  `SyncRecoveryCategory` for recovery or resource classes,
  `NoProgressDiagnosis` for blocker detail, `EvidenceVerdictSummary` for proof
  versus diagnosed blocker, and process/cancellation facts for operator stop or
  unexpected termination.
- **D-11:** Resume rules should be explicit. Clean completion should close the
  run and not resume as the same run. Operator, resource, and recovery stops may
  resume only through an explicit same-run resume record with preserved datadir
  and run identity. Unexpected termination should resume as interrupted-run
  recovery evidence, never as a clean stop.
- **D-12:** Keep the soak vocabulary shallow in Phase 75. Later phases own
  deeper resource-bound classification, corruption/lock recovery detail,
  progress guarantees, and support-bundle forensics.

### Deterministic Synthetic Coverage

- **D-13:** Use mixed deterministic coverage, with Rust tests as the canonical
  behavioral proof. Reuse `DurableSyncRuntime`, scripted transport/resolver,
  explicit timestamps or scripted clocks, durable reopen fixtures, and
  synthetic long-chain patterns to prove long-run control flow without public
  peers or wall-clock multi-day waits.
- **D-14:** Add a thin operator-level harness only for user-facing command and
  report behavior: argument validation, run identity paths, interrupted/resumed
  report behavior, and final output.
- **D-15:** Add a focused Bun checker when docs, report fixtures, parity roots,
  or default-verification boundaries need auditing. The checker should follow
  Phase 68 through Phase 74 patterns and remain local, short-running,
  public-network-free, service-manager-free, and timing-stable.
- **D-16:** Avoid timer-virtualization complexity unless planning proves it is
  needed. The existing sync paths already accept explicit timestamps in many
  places, so scripted clocks and deterministic fixture inputs are the preferred
  first path.

### Operator Guidance And Scope Boundaries

- **D-17:** Operator docs must describe what the soak evidence proves and what
  it does not prove. A soak run can prove bounded opt-in full-sync soak
  behavior, durable resume evidence, or diagnosed blocker evidence; it does not
  prove inbound serving, relay, production-funds wallet safety, migration apply
  mode, signed packages, GUI readiness, hosted dashboards, or broad
  production-node readiness.
- **D-18:** UAT commands should use repo-local Cargo and Bazel forms for
  operator CLI workflows. Avoid bare installed-alias instructions unless the
  user explicitly asks for them.
- **D-19:** If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update
  `docs/parity/source-breadcrumbs.json` and keep the breadcrumb checker green.

### the agent's Discretion

- The planner may split Phase 75 into operator CLI contract, soak ledger/domain
  model, report/support summary projection, deterministic runtime tests,
  operator docs, and checker/parity closeout.
- The executor may add small pure domain types for soak bounds, run identity,
  event records, and final outcomes when they make illegal states
  unrepresentable.
- The executor may keep the first support-bundle integration minimal: expose a
  redacted soak summary only after the ledger/report source of truth is typed
  and tested.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 75 goal, dependency on Phase 74, success
  criteria, and v1.7 phase sequence.
- `.planning/REQUIREMENTS.md` - SOAK-01 through SOAK-04, v1.7 out-of-scope
  boundaries, and traceability.
- `.planning/PROJECT.md` - v1.7 milestone goal, current state, pinned Knots
  baseline, functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - Phase 75 readiness, accumulated v1.7 decisions, and
  pending todo to define the soak-runner and evidence-ledger shape.
- `AGENTS.md` - Repo-local GSD workflow, Rust, parity breadcrumb, UAT command,
  generated artifact, and verification requirements.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Local standards override registry.
- `standards/core/architecture.md` - Functional-core, parse-at-boundaries, and
  illegal-state modeling rules.
- `standards/core/code-shape.md` - Early-return, optional naming, script, and
  file-size guidance.
- `standards/core/verification.md` - Sync-first and repo-native verification
  requirements.
- `standards/core/testing.md` - Unit-test and Arrange/Act/Assert expectations.
- `standards/languages/rust.md` - Rust module, option naming, invariant, and
  verification guidance.
- `standards/languages/typescript-javascript.md` - Bun/TS automation and
  nullish naming guidance.

### Prior Full-Sync Decisions And Evidence

- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md`
  - Validated active-chain progress, durable persistence, no-credit peer
  outcomes, and deterministic verification posture.
- `.planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md`
  - Best-known tip, stay-current state, bounded daemon wake cycles, and
  public-network UAT boundaries.
- `.planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md`
  - Reorg/no-progress diagnosis, peer rotation, stale in-flight cleanup, and
  typed next actions.
- `.planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md`
  - Resource pressure, durable restart/resume matrix, storage recovery
  precedence, and synthetic long-chain verification.
- `.planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md`
  - Shared truth contract, cross-surface alignment, redacted support evidence,
  and support verdicts.
- `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-CONTEXT.md`
  - Opt-in UAT command matrix, deterministic default-verification exclusions,
  and evidence auditability.
- `.planning/phases/74-release-boundaries-parity-and-documentation/74-CONTEXT.md`
  - v1.6 release claim shape, non-claim list, release-boundary checker posture,
  and repo-local UAT command guidance.

### Implementation And Verification Surfaces

- `packages/open-bitcoin-cli/src/operator.rs` - Existing operator subcommand
  shape and natural place to add a `soak` operator entrypoint.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` - Operator command
  dispatch and config resolution surface.
- `packages/open-bitcoin-cli/src/operator/support.rs` - Support bundle command
  entrypoint and redacted local evidence surface.
- `packages/open-bitcoin-cli/src/operator/support/evidence.rs` - Full-sync
  support evidence and verdict derivation pattern.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` - Allowlisted
  live-smoke summary projection pattern.
- `packages/open-bitcoin-cli/tests/operator_flows.rs` - Operator flow
  integration-test surface.
- `packages/open-bitcoin-node/src/sync.rs` - Durable sync runtime orchestration,
  target stop handling, and current-at-tip stop handling.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Runtime metadata,
  progress persistence, metrics/log projection, and durable state projection.
- `packages/open-bitcoin-node/src/sync/types.rs` - `SyncRunSummary`,
  `SyncStopReason`, and sync runtime types.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Shared sync status,
  stop reason, metrics, and structured log projection.
- `packages/open-bitcoin-node/src/sync/types/recovery.rs` - Existing recovery
  category mappings to preserve rather than overload.
- `packages/open-bitcoin-node/src/status.rs` - Shared status snapshot and
  durable sync state contracts.
- `packages/open-bitcoin-node/src/status/recovery.rs` - Stable sync recovery
  labels.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Existing scripted transport,
  durable reopen, synthetic long-chain, resource, and stop-reason tests.
- `scripts/run-live-mainnet-smoke.ts` - Existing opt-in public-mainnet runner
  and report fixture pattern to reuse without making it the soak product
  surface.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke fixture
  validation that must remain public-network-free.
- `scripts/verify.sh` - Repo-native deterministic verification contract.
- `scripts/check-phase73-uat-verification.ts` and
  `scripts/check-v1.6-release-boundaries.ts` - Recent checker patterns for
  opt-in UAT and default-verification boundary guards.
- `scripts/check-parity-breadcrumbs.ts` and
  `docs/parity/source-breadcrumbs.json` - Required breadcrumb mechanism for new
  first-party Rust source or test files.
- `docs/operator/runtime-guide.md` - Operator guide and authoritative
  copy-pasteable UAT command surface.
- `docs/architecture/status-snapshot.md` - Shared status, stop reason,
  progress, stay-current, no-progress, and recovery vocabulary.
- `docs/architecture/operator-observability.md` - Metrics/log/support evidence
  retention and compact evidence boundaries.
- `docs/parity/release-readiness.md`, `docs/parity/index.json`,
  `docs/parity/checklist.md`, and `docs/parity/README.md` - Parity roots that
  may need Phase 75 evidence discoverability if planning changes parity-facing
  surfaces.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `open-bitcoin` already owns operator workflows through Clap subcommands in
  `packages/open-bitcoin-cli/src/operator.rs` and runtime dispatch in
  `packages/open-bitcoin-cli/src/operator/runtime.rs`.
- `DurableSyncRuntime`, `RuntimeMetadata`, `DurableSyncState`, and
  `SyncRunSummary` already persist and project sync lifecycle, stop reason,
  progress, metrics, logs, and recovery evidence.
- `OpenBitcoinStatusSnapshot` is the shared status model for CLI status,
  dashboard, RPC-facing status, metrics/log projections, live-smoke snapshots,
  and support bundles.
- `support/evidence.rs` already derives typed full-sync support verdicts from
  status and live-smoke evidence, and `support/live_smoke.rs` already provides
  an allowlisted summary-only live-smoke projection.
- `scripts/run-live-mainnet-smoke.ts` already covers opt-in daemon spawn,
  polling, manual peers, preflight checks, disk checks, status snapshots,
  restart evidence, JSON/Markdown output, and deterministic fixture validation.
- Phase 68 through Phase 74 checkers show the local Bun pattern for explicit
  required paths, source/test/doc needles, ordered verify wiring, and forbidden
  default public-network/service-manager strings.

### Established Patterns

- Default verification stays deterministic, public-network-free,
  service-manager-free, timing-stable, and free of multi-day wall-clock waits.
- Operator evidence must distinguish headers, downloaded blocks, connected
  blocks, validated active-chain progress, best-known tip, stay-current state,
  stop reason, recovery category, no-progress diagnosis, and unavailable
  reasons rather than collapsing them into success-like prose.
- Storage and resource blockers take precedence over peer retry advice and must
  not trigger hidden mutation.
- Support evidence is compact, redacted, allowlisted, and local. Raw daemon
  tails, secrets, raw live-smoke reports, wallet material, and unbounded peer
  tables stay out.
- Public-network full-sync and multi-day soak evidence remain opt-in UAT until
  a future phase explicitly changes that boundary.

### Integration Points

- Add the soak operator surface near existing `SyncCommand` and
  `SupportCommand` flows, then keep parsing/validation at the CLI boundary.
- Add soak domain/run types in a pure module before wiring file, process, or
  storage effects through operator/runtime adapters.
- Reuse shared status and sync runtime projections for outcome source evidence
  instead of reclassifying stop and recovery states in report rendering.
- Add deterministic Rust coverage near existing sync runtime tests, and split
  new test modules if file size or responsibility boundaries become clearer.
- Add a focused TypeScript checker only for artifact, docs, fixture, parity, and
  default-verification boundaries that Rust tests do not prove.

</code_context>

<specifics>

## Specific Ideas

- Advisor mode selected the layered `open-bitcoin soak` contract over making
  `scripts/run-live-mainnet-smoke.ts` the durable product surface.
- Advisor mode selected the hybrid datadir index plus append-only JSONL ledger
  over report-only or support-bundle-only evidence.
- Advisor mode selected a soak-owned outcome vocabulary that wraps existing
  shared evidence over extending `SyncStopReason` directly.
- Advisor mode selected mixed deterministic coverage with Rust as the canonical
  behavior proof, a thin operator harness for CLI/report flow, and a Bun checker
  for docs/fixtures/default-verification boundaries.

</specifics>

<deferred>

## Deferred Ideas

- Scheduled public-network soak monitors remain future SOAK-05 scope.
- Signed externally comparable soak result artifacts remain future SOAK-06
  scope.
- Deep disk/resource bound enforcement belongs to Phase 76.
- Corruption and lock recovery hardening belongs to Phase 77.
- Progress guarantees and stall diagnosis belongs to Phase 78.
- Full support-bundle forensics and failure narratives belong to Phase 79.

</deferred>

---

*Phase: 75-multi-day-soak-runner-and-evidence-ledger*
*Context gathered: 2026-06-14*
