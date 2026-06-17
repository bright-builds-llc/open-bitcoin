---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 79-2026-06-17T13-53-04
generated_at: 2026-06-17T13:53:04.288Z
---

# Phase 79: Diagnostics and Support Bundle Forensics - Context

**Gathered:** 2026-06-17
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 79 turns the Phase 75 through Phase 78 long-run evidence into a redacted
"what happened" support story for failed or degraded explicit opt-in soaks. It
owns DIAG-01 through DIAG-04: a support bundle timeline and checkpoint chain,
resource-pressure, recovery-event, peer-outcome, and final-verdict evidence;
agreement between CLI status, dashboard status, RPC status, metrics, structured
logs, live-smoke reports, and support bundles; concise failure narratives with
likely cause, evidence basis, next action, and outcome meaning; and
deterministic checks for redaction, size bounds, timeline ordering, and
cross-surface consistency.

This phase consumes the existing `OpenBitcoinStatusSnapshot`, soak ledger and
report projection, support bundle, resource-bound evidence, recovery evidence,
and progress-guarantee/stall evidence. It must not redefine soak run identity,
move public-network or multi-day wall-clock checks into `bash scripts/verify.sh`,
add external telemetry dependencies, automatically upload support bundles,
perform destructive datadir mutation or repair, or claim broad production-node
readiness.

</domain>

<decisions>

## Implementation Decisions

### Forensic Timeline And Checkpoint Chain

- **D-01:** Build the Phase 79 timeline from typed, redacted, structured
  evidence rather than parsing raw logs or freeform renderer text. The primary
  source is the datadir-owned soak ledger and its checkpoint status fields,
  enriched with shared status, resource-bound, recovery, progress-guarantee,
  peer-outcome, live-smoke summary, and support-bundle provenance where
  available.
- **D-02:** Treat a hash-linked checkpoint chain as the preferred support
  contract when it can be implemented compactly and deterministically. The
  chain should prove event ordering and truncation/missing-evidence detection,
  not authenticity. Do not add signing or external trust roots in this phase.
- **D-03:** Support reconstruction for pre-Phase-79 or incomplete evidence only
  as an explicit fallback with missing-evidence markers. Fallback evidence must
  stay conservative and must not infer causes from elapsed time, bundle
  existence, raw log snippets, or stale report files.
- **D-04:** Timeline entries should be compact and bounded: run start/resume,
  checkpoints, stops, verdicts, resource pressure, recovery events, peer
  contribution or failure aggregates, progress/stall facts, and source paths.
  Raw daemon logs, raw live-smoke reports, wallet material, credentials, and
  unbounded peer tables remain excluded.

### Shared Diagnostic Contract

- **D-05:** Keep node/runtime diagnostic truth in the existing shared status
  contract. Extend `OpenBitcoinStatusSnapshot` or adjacent status types only
  when the evidence is live or durable node truth that CLI, dashboard, RPC,
  metrics, logs, live-smoke, and support surfaces should all preserve.
- **D-06:** Add a support-forensics sidecar only for bundle-specific evidence:
  source ledger/report paths, source event counts, redaction summary, bundle
  size/projection facts, checkpoint-chain validation, and comparison metadata.
  The sidecar may derive from status and soak evidence but must not become an
  alternate source of truth for runtime classifications.
- **D-07:** Renderers should format typed diagnostics; they must not reclassify
  causes, verdicts, resource states, recovery actions, or stalled subsystems
  from strings. Missing evidence should stay machine-readable as unavailable
  reasons or explicit missing-evidence entries.
- **D-08:** Metrics and structured logs should project bounded labels and counts
  from the same diagnostic contract, not embed high-cardinality diagnostic
  objects or unbounded timelines. Keep field names stable and deterministic.

### Failure Narrative And Final Verdict

- **D-09:** Use a compact domain verdict for the support-bundle narrative rather
  than a CI-style pass/fail or full postmortem format. Required outcomes should
  distinguish at least `soak_stable`, `blocker_diagnosed`, `inconclusive`, and
  `collection_failed` when bundle generation cannot support analysis.
- **D-10:** Pair every final verdict with four concise fields: likely cause,
  evidence basis, next action, and confidence. These fields must be derived
  from typed evidence and should avoid overclaiming root cause when evidence is
  partial.
- **D-11:** Map existing Phase 75 outcome labels into narrative outcomes without
  changing their meaning. `clean_completion` can support `soak_stable` only
  when progress/stay-current evidence and final status support it;
  `diagnosed_blocker`, resource/recovery stops, stall diagnosis, or peer
  failure evidence can support `blocker_diagnosed`; missing or conflicting
  evidence remains `inconclusive`.
- **D-12:** Operator wording should stay quiet, dense, and actionable. The
  narrative should answer what happened, what evidence supports that reading,
  what the operator should do next, and which claim the run did or did not
  prove. It must not imply production-node readiness, inbound serving, relay,
  wallet safety, migration apply safety, or public-network CI coverage.

### Redaction, Size Bounds, And Deterministic Verification

- **D-13:** Make the default proof a deterministic typed contract checker plus
  focused Rust support-bundle tests. The checker should verify required DIAG
  requirement ids, phase-plan frontmatter, docs, parity roots, support field
  anchors, redaction guards, timeline ordering, size-bound wording, and
  `scripts/verify.sh` ordering.
- **D-14:** Focused Rust tests should prove support-bundle JSON and Markdown
  contain the forensic timeline, checkpoint chain, narrative verdict, evidence
  basis, next action, redaction summary, size/projection facts, and
  cross-surface status agreement from deterministic fixtures.
- **D-15:** Seed forbidden sensitive material in tests and checkers where useful:
  RPC cookie contents, `rpcpassword`, `rpcauth`, wallet material, raw daemon
  stdout/stderr tails, raw live-smoke input, raw logs, raw options, and
  endpoint tables must not appear in support output.
- **D-16:** Opt-in public-network or multi-day forensic validation may be
  documented for UAT, but it stays outside `bash scripts/verify.sh` and outside
  the commit/push gate. Default verification remains public-network-free,
  service-manager-free, short-running, and free of large disk allocations.

### the agent's Discretion

- The planner may split Phase 79 across shared diagnostic domain types,
  support-forensics projection, support bundle rendering, live-smoke/status
  cross-surface consistency, deterministic fixtures, operator docs, and checker
  closeout.
- The executor may add compact enums and structs for forensic timeline entries,
  checkpoint-chain evidence, narrative verdicts, evidence confidence, and
  missing-evidence markers when they make illegal states unrepresentable.
- The executor may start with checkpoint-chain validation over the existing soak
  event sequence and canonical serialized fields. It should avoid external
  telemetry dependencies and avoid cryptographic signing unless a later phase
  explicitly asks for signed comparable artifacts.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 79 goal, dependency on Phase 78, success
  criteria, and Phase 80 boundary.
- `.planning/REQUIREMENTS.md` - DIAG-01 through DIAG-04, v1.7 out-of-scope
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

- `.planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md`
  - shared status truth contract, cross-surface alignment, and redacted support
  evidence boundaries.
- `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-CONTEXT.md`
  - opt-in UAT command matrix and deterministic default-verification
  exclusions.
- `.planning/phases/74-release-boundaries-parity-and-documentation/74-CONTEXT.md`
  - release-claim and non-claim wording.
- `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md`
  - `open-bitcoin soak`, durable run ledger, checkpoint/report projection,
  soak outcome taxonomy, and support summary projection.
- `.planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md`
  - resource-bound status surfaces, support-bundle size pressure, resource-stop
  semantics, and deterministic fixture policy.
- `.planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md`
  - recovery evidence, recovery-stop semantics, probe-only status/support
  boundaries, and deterministic recovery fixtures.
- `.planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md`
  - progress-credit, stall-diagnosis, last-peer-contribution, resource/recovery
  precedence, and deferred Phase 79 narrative scope.
- `.planning/phases/78-progress-guarantees-and-stall-diagnosis/78-VERIFICATION.md`
  - passed Phase 78 evidence and current readiness for Phase 79.

### Implementation And Verification Surfaces

- `packages/open-bitcoin-node/src/status.rs` - `OpenBitcoinStatusSnapshot`,
  `SyncStatus`, `FieldAvailability`, peer telemetry, recovery evidence,
  resource bounds, and progress-guarantee status contracts.
- `packages/open-bitcoin-node/src/status/progress_guarantee.rs` - progress
  credit, rejected progress activity, peer contribution, stalled subsystem,
  confidence, evidence basis, and next-action types.
- `packages/open-bitcoin-node/src/status/recovery.rs` - stable
  `SyncRecoveryCategory` labels.
- `packages/open-bitcoin-node/src/status/resource_bounds.rs` - resource-bound
  kind, pressure, support-bundle bound, and usage contracts.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - durable status
  projection, resource/recovery/progress carry-forward, and shared status
  construction.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - compact metrics,
  structured log, resource pressure, peer contribution, and summary projection.
- `packages/open-bitcoin-cli/src/operator/support.rs` - support bundle command,
  bundle JSON shape, support-soak evidence collection, redaction summary, and
  current support evidence bundle struct.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - support Markdown
  and command-output rendering.
- `packages/open-bitcoin-cli/src/operator/support/evidence.rs` - full-sync
  support evidence and typed support verdict derivation.
- `packages/open-bitcoin-cli/src/operator/support/progress_guarantee.rs` -
  compact Phase 78 progress and stall support summaries.
- `packages/open-bitcoin-cli/src/operator/support/resource_bounds.rs` - compact
  support resource-bound evidence and projected bundle-size fields.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` and
  `packages/open-bitcoin-cli/src/operator/support/live_smoke/tests.rs` -
  allowlisted live-smoke summary projection and redaction tests.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - support bundle
  JSON/Markdown fixture coverage and natural home for Phase 79 forensic tests.
- `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` - datadir-owned soak
  run index, JSONL event envelope, checkpoint schema, and event-size bound.
- `packages/open-bitcoin-cli/src/operator/soak/report.rs` - report projection,
  report redaction guard, latest checkpoint rendering, and source ledger paths.
- `packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs` - checkpoint
  status projection from shared status snapshots.
- `packages/open-bitcoin-cli/src/operator/soak/tests.rs` and
  `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs` - soak ledger,
  report, recovery, resource, and progress fixture coverage.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` and
  `packages/open-bitcoin-cli/src/operator/dashboard/model/sync_section.rs` -
  dashboard status projection from shared status.
- `packages/open-bitcoin-rpc/src/method/node.rs` - RPC-facing status consumer
  that must preserve shared diagnostic fields if touched.
- `scripts/run-live-mainnet-smoke.ts` and `scripts/test-run-live-mainnet-smoke.sh`
  - opt-in live-smoke report and deterministic fixture validation; useful as
  summary-only input, not as the default Phase 79 proof.
- `scripts/check-phase75-soak-runner.ts`,
  `scripts/check-phase76-resource-bounds.ts`,
  `scripts/check-phase77-corruption-lock-recovery.ts`, and
  `scripts/check-phase78-progress-guarantees.ts` - recent phase checker style,
  verifier ordering checks, and public-network boundary guards.
- `scripts/verify.sh` - repo-native deterministic verification contract.
- `scripts/check-parity-breadcrumbs.ts` and
  `docs/parity/source-breadcrumbs.json` - required breadcrumb mechanism for new
  first-party Rust source or test files.

### Operator Docs And Parity Roots

- `docs/operator/runtime-guide.md` - support-bundle command forms, Phase 75
  through Phase 78 evidence guidance, redaction boundaries, UAT command
  surface, and non-claim wording.
- `docs/architecture/status-snapshot.md` - shared status snapshot, support
  bundle, soak ledger, resource, recovery, progress, and cross-surface truth
  vocabulary.
- `docs/architecture/operator-observability.md` - observability, metrics/log,
  support evidence, resource, recovery, progress, and compact evidence
  boundaries.
- `docs/architecture/config-precedence.md` - credential-source metadata and
  support evidence redaction expectations.
- `docs/parity/index.json`, `docs/parity/checklist.md`,
  `docs/parity/README.md`, `docs/parity/catalog/p2p.md`,
  `docs/parity/catalog/chainstate.md`, and
  `docs/parity/catalog/operator-runtime-release-hardening.md` - parity roots
  that may need Phase 79 evidence discoverability.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `OpenBitcoinStatusSnapshot` already serves as the shared truth model for CLI
  status, dashboard, RPC-facing status, metrics projections, structured logs,
  live-smoke summaries, and support bundles.
- `packages/open-bitcoin-cli/src/operator/support.rs` already writes
  `support-evidence.json` and `support-evidence.md` with config evidence,
  redaction metadata, status, recovery evidence, store health, live-smoke
  summary, full-sync evidence, soak evidence, and resource-bound evidence.
- `SoakLedgerEventEnvelope`, `SoakLedgerEvent`, and `SoakCheckpointStatus`
  already provide a sequence-numbered, versioned, datadir-owned event stream
  with bounded event bytes and atomic run-index updates.
- `SoakReportProjection` already derives latest checkpoint, stop, verdict,
  resume count, checkpoint count, source ledger path, and final outcome from
  ledger events.
- Phase 76 and Phase 78 support modules already summarize resource bounds,
  progress credit, and stall diagnosis in support bundles.
- Existing phase checkers provide a repeatable Bun pattern for required paths,
  docs needles, parity roots, plan frontmatter, verifier ordering, and
  forbidden public-network/service-manager strings.

### Established Patterns

- Default verification stays deterministic, public-network-free,
  service-manager-free, timing-stable, free of multi-day sleeps, and free of
  large local disk allocations.
- Status, support, dashboard, RPC, logs, metrics, live-smoke, and soak reports
  consume shared typed status evidence instead of renderer-local string
  classification.
- Support evidence is compact, local, redacted, and allowlisted. Raw logs,
  credentials, wallet material, raw reports, raw options, and unbounded tables
  stay out.
- Resource and recovery evidence take precedence over peer retry advice when
  storage or datadir safety is at stake.
- New first-party Rust source or test files under package source/test trees
  need parity breadcrumbs.

### Integration Points

- Add pure forensic/narrative types close to the support or shared status
  domain, then wire the support bundle as a thin projector from status, soak
  ledger/report, resource, recovery, progress, and live-smoke summaries.
- Keep support-bundle-specific provenance and redaction evidence inside a
  support-forensics sidecar rather than polluting live node status with
  bundle-only paths.
- Update support Markdown and JSON together so the human and machine surfaces
  carry the same verdict, timeline, checkpoint-chain, evidence-basis, and
  missing-evidence facts.
- Add deterministic Rust tests first for the pure projection and support
  rendering, then add a focused Bun checker for docs/parity/verification
  anchors that Rust tests cannot prove.

</code_context>

<specifics>

## Specific Ideas

- Advisor mode selected a structured soak timeline and checkpoint-chain model
  over raw log reconstruction. Raw reconstruction is allowed only as a clearly
  labeled fallback with missing-evidence markers.
- Advisor mode selected a shared-contract-first approach: status owns runtime
  truth, while the support bundle may add a forensics sidecar for provenance,
  redaction, source paths, event counts, and chain validation.
- Advisor mode selected a compact domain verdict plus four-field narrative
  instead of a verbose postmortem or generic CI conclusion.
- Advisor mode selected typed deterministic checker coverage plus focused Rust
  support-bundle tests as the default proof, leaving opt-in live or multi-day
  forensic validation to UAT.

</specifics>

<deferred>

## Deferred Ideas

- External telemetry export, OpenTelemetry adoption, and high-cardinality
  diagnostic object export are deferred unless a future milestone deliberately
  standardizes on an external observability surface.
- Cryptographically signed or externally comparable support/soak artifacts
  remain future SOAK-06 scope.
- Opt-in multi-day soak UAT command closeout and final v1.7 release-boundary
  wording belong to Phase 80.
- Inbound serving, relay, production-funds wallet claims, migration apply mode,
  packaging, GUI, hosted dashboards, scheduled public soak monitors, and broad
  production-node readiness remain outside Phase 79.

</deferred>

---

*Phase: 79-diagnostics-and-support-bundle-forensics*
*Context gathered: 2026-06-17*
