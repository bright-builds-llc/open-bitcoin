---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 76-2026-06-15T13-56-15
generated_at: 2026-06-15T13:58:16.426Z
---

# Phase 76: Disk and Resource Bound Enforcement - Context

**Gathered:** 2026-06-15
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 76 makes long-run resource limits visible, enforceable, and testable for
explicit opt-in soak workflows before storage pressure turns a run into an
unsafe or opaque failure. It owns RES-05 through RES-08: preflight and status
visibility for disk, file, cache, queue, peer, in-flight, log, metric, and
support-bundle bounds; typed runtime guidance for low disk, disk growth,
compaction, log retention, metrics retention, and support-bundle size pressure;
operator pause or stop before unsafe pressure while preserving durable progress;
and deterministic resource-bound fixtures.

This phase consumes Phase 75's `open-bitcoin soak` command, `SoakBounds`,
ledger, report projection, and support evidence. It must not redefine soak run
identity, move public-network or multi-day wall-clock checks into
`bash scripts/verify.sh`, add destructive datadir repair or pruning, claim broad
production-node readiness, or implement the deeper Phase 77 corruption/lock,
Phase 78 progress/stall, or Phase 79 support-forensics scope.

</domain>

<decisions>

## Implementation Decisions

### Resource Inventory And Bound Surfaces

- **D-01:** Extend the existing shared typed contracts instead of creating a
  separate renderer-local resource model. `SyncResourcePressure` remains the
  network/in-flight resource envelope, while Phase 76 may add typed adjacent
  bound evidence for disk usage, file counts, cache or queue bounds, metrics
  retention, log retention, soak ledger/report footprints, and support-bundle
  size pressure.
- **D-02:** Surface bounds before and during a soak through the same shared
  status path that CLI status, dashboard, RPC status, support evidence, and
  soak reports already consume. Every field should be either available with
  measured/configured values or explicitly unavailable with a reason.
- **D-03:** Treat the Phase 75 `SoakBounds.disk_budget_bytes` value as the
  operator's explicit soak budget. Compare it against measured datadir,
  metrics, log, soak-ledger/report, and support-evidence footprints where those
  footprints are available. Missing measurements should produce typed
  unavailable evidence, not silent success.
- **D-04:** Keep bound evidence compact and allowlisted. Do not copy raw daemon
  logs, raw metrics stores, raw support bundles, raw live-smoke reports, wallet
  material, credentials, or unbounded peer tables into soak reports or support
  evidence.

### Enforcement And Operator Stop Policy

- **D-05:** Resource enforcement should be dry-run-first in preflight and
  evidence-first at runtime. Preflight should refuse obviously unsafe starts
  such as zero or already-exceeded budgets, missing datadirs that cannot be
  assessed, and unavailable required resource paths when the operator requested
  enforcement.
- **D-06:** Runtime pressure should produce typed warning and stop decisions
  instead of vague "sync failed" text. Use explicit threshold states such as
  normal, warning, and stop-required; planner may choose exact numeric defaults,
  but the defaults must be documented, tested, and configurable or derived from
  the operator's explicit disk budget.
- **D-07:** When resource pressure requires stopping a soak, record a
  `resource_stop` soak outcome with source evidence from shared status,
  recovery category, no-progress/resource diagnosis when available, and the
  resource-bound snapshot that triggered the decision. Durable progress and the
  run id must remain resumable under the Phase 75 same-run resume rules.
- **D-08:** Operator pause and stop guidance should prefer existing
  `open-bitcoin sync pause`, `open-bitcoin sync resume`, and `open-bitcoin soak`
  resume semantics before adding new control surfaces. If a new flag or command
  is needed, it must be explicit, non-destructive, and documented with
  repo-local Cargo and Bazel forms.

### Retention, Compaction, And Support Evidence

- **D-09:** Metrics and structured logs already have bounded retention policies;
  Phase 76 should expose their configured policy, current footprint or
  unavailable reason, and pressure classification rather than duplicating the
  retention engines.
- **D-10:** Support-bundle pressure is a first-class bound. Support evidence
  should report projected bundle size, compact summary availability, omitted
  raw artifacts, and any size pressure that would make bundle generation unsafe
  or misleading.
- **D-11:** Compaction and cleanup guidance must be advice, not hidden mutation.
  The system may tell an operator to free disk, rotate or prune configured logs,
  reduce retention, move output paths, or retry after clearing space. It must
  not silently delete, compact, repair, prune, or relocate user data.
- **D-12:** Resource guidance should preserve storage-first precedence from
  Phase 71. Low disk, storage pressure, backend write failures, and resource
  exhaustion outrank peer retry advice and should map to
  `SyncRecoveryCategory::ResourceExhaustion` or a more precise typed category
  only when planning proves the existing taxonomy is insufficient.

### Deterministic Verification

- **D-13:** RES-08 must be proven with deterministic fixtures, not public peers,
  real service managers, large local disk allocations, or multi-day sleeps.
  Tests should use small temp directories, synthetic file metadata, scripted
  status collectors, fake resource probes, and explicit timestamps.
- **D-14:** Rust tests are the canonical behavior proof for pure resource-bound
  decisions, status projections, soak runtime stop behavior, and retention
  classification. Keep tests one concern per test and use Arrange/Act/Assert
  comments when setup is non-trivial.
- **D-15:** Add a focused Bun checker only for docs, phase artifact anchors,
  parity roots, default-verification boundaries, and generated LOC freshness
  that Rust tests do not prove. Follow the Phase 71 through Phase 75 checker
  style and keep the checker public-network-free and service-manager-free.
- **D-16:** If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update
  `docs/parity/source-breadcrumbs.json` and keep
  `scripts/check-parity-breadcrumbs.ts` green.

### the agent's Discretion

- The planner may split Phase 76 across resource-bound domain/status types,
  soak preflight/runtime enforcement, retention/support pressure projection,
  deterministic fixtures, operator docs, and checker/parity closeout.
- The executor may add small pure helper types for budget thresholds, measured
  footprints, pressure states, and next-action guidance when they make illegal
  states unrepresentable.
- The executor may keep existing enum variants and add precise evidence fields
  when that is simpler than adding a new taxonomy. Add new variants only when
  the current labels cannot express RES-05 through RES-08 clearly.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 76 goal, dependency on Phase 75, success
  criteria, and deferred Phase 77 through Phase 80 boundaries.
- `.planning/REQUIREMENTS.md` - RES-05 through RES-08, v1.7 out-of-scope
  boundaries, and traceability.
- `.planning/PROJECT.md` - v1.7 milestone goal, current state, pinned Knots
  baseline, functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - Phase 76 readiness, accumulated v1.7 decisions, and
  pending todo to decide the disk/resource enforcement shape.
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

### Prior Phase Decisions And Evidence

- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` -
  earlier resource pressure, recovery category, and support-evidence compactness
  decisions.
- `.planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md`
  - `SyncResourcePressure`, storage-first recovery precedence, restart/resume
  evidence, synthetic long-chain verification, and retention proof.
- `.planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md`
  - shared status truth contract, cross-surface alignment, and redacted support
  evidence boundaries.
- `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-CONTEXT.md`
  - opt-in UAT command matrix and default-verification exclusions.
- `.planning/phases/74-release-boundaries-parity-and-documentation/74-CONTEXT.md`
  - scoped release-claim language and release-boundary checker posture.
- `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md`
  - `open-bitcoin soak`, `SoakBounds`, durable ledger/report shape, outcome
  taxonomy, support summary projection, and deferred Phase 76 resource scope.
- `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-VERIFICATION.md`
  - passed Phase 75 evidence and any residual Phase 76 risks.

### Implementation And Verification Surfaces

- `packages/open-bitcoin-node/src/status.rs` - `OpenBitcoinStatusSnapshot`,
  `SyncStatus`, `SyncResourcePressure`, `FieldAvailability`, recovery,
  no-progress, metrics, logs, and support-facing status contracts.
- `packages/open-bitcoin-node/src/status/recovery.rs` - stable
  `SyncRecoveryCategory` labels and resource/storage recovery categories.
- `packages/open-bitcoin-node/src/metrics.rs` - `MetricRetentionPolicy`,
  bounded metric series, disk usage metric kind, and metrics status projection.
- `packages/open-bitcoin-node/src/logging.rs` and
  `packages/open-bitcoin-node/src/logging/` - `LogRetentionPolicy`, structured
  log retention planning, pruning, writer behavior, and log status projection.
- `packages/open-bitcoin-node/src/storage.rs` and
  `packages/open-bitcoin-node/src/storage/fjall_store.rs` - storage errors,
  low-disk/resource recovery mapping, metadata, and durable status integration.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - durable status
  projection, resource pressure, metrics/log persistence, and recovery
  precedence.
- `packages/open-bitcoin-node/src/sync/types.rs` - sync runtime config, bounds,
  stop reasons, runtime errors, and recovery mapping.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - shared status,
  metrics, structured logs, resource pressure, and summary contracts.
- `packages/open-bitcoin-node/src/sync/tests.rs` and
  `packages/open-bitcoin-node/src/sync/tests/soak.rs` - deterministic runtime,
  resource, restart/resume, and soak fixtures.
- `packages/open-bitcoin-cli/src/operator/soak.rs` - `SoakBounds`, peer policy,
  stop conditions, and soak command entrypoint.
- `packages/open-bitcoin-cli/src/operator/soak/runtime.rs` - bounded soak loop,
  status collection, resource/recovery stop mapping, and resume validation.
- `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` - run index,
  append-only event ledger, report paths, and atomic writes.
- `packages/open-bitcoin-cli/src/operator/soak/report.rs` - report projection
  shape and redaction guard.
- `packages/open-bitcoin-cli/src/operator/soak/tests.rs` and
  `packages/open-bitcoin-cli/tests/operator_binary.rs` - operator-level soak
  fixtures and binary flow coverage.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` - sync
  pause/resume/status command behavior and resource-pressure support text.
- `packages/open-bitcoin-cli/src/operator/status/render.rs`,
  `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`, and
  `packages/open-bitcoin-cli/src/operator/status/service_status.rs` - human,
  JSON, dashboard, and service status resource-pressure renderers.
- `packages/open-bitcoin-cli/src/operator/support.rs`,
  `packages/open-bitcoin-cli/src/operator/support/evidence.rs`, and
  `packages/open-bitcoin-cli/src/operator/support/render.rs` - support bundle
  command, support verdicts, compact resource evidence, redaction, and rendering.
- `scripts/check-phase71-resource-restart.ts` - prior resource/restart checker
  pattern.
- `scripts/check-phase72-observability-evidence.ts` - cross-surface evidence
  checker pattern.
- `scripts/check-phase75-soak-runner.ts` and
  `scripts/check-phase75-soak-runner.test.ts` - current phase checker and test
  style for soak artifacts, docs, support evidence, parity roots, and default
  verification boundaries.
- `scripts/verify.sh` - repo-native deterministic verification contract.
- `scripts/check-parity-breadcrumbs.ts` and
  `docs/parity/source-breadcrumbs.json` - required breadcrumb mechanism for new
  first-party Rust source or test files.

### Operator Docs And Parity Roots

- `docs/operator/runtime-guide.md` - runtime resource bounds, Phase 75 soak
  commands, opt-in UAT command forms, support-bundle collection, and release
  boundary language.
- `docs/architecture/status-snapshot.md` - shared status truth contract,
  resource-pressure fields, metrics/log projections, and Phase 75 soak ledger
  vocabulary.
- `docs/architecture/operator-observability.md` - metrics/log retention,
  resource and recovery vocabulary, support evidence boundaries, and Phase 75
  soak evidence ledger notes.
- `docs/architecture/storage-decision.md` - durable storage, low-disk backend
  guidance, and no-hidden-mutation recovery boundary.
- `docs/parity/release-readiness.md`, `docs/parity/index.json`,
  `docs/parity/checklist.md`, `docs/parity/README.md`, and
  `docs/parity/catalog/p2p.md` - parity roots that may need Phase 76 evidence
  discoverability if planning changes parity-facing surfaces.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- Phase 75 added `packages/open-bitcoin-cli/src/operator/soak.rs` with
  `SoakBounds`, `SoakRunId`, stop conditions, and explicit disk budget input.
- Phase 75 added a durable soak ledger and projection under
  `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` and
  `packages/open-bitcoin-cli/src/operator/soak/report.rs`.
- `OpenBitcoinStatusSnapshot`, `SyncStatus`, `SyncResourcePressure`, metrics,
  logs, and recovery categories already provide a shared status contract for
  operator surfaces.
- `MetricRetentionPolicy` and `LogRetentionPolicy` already encode bounded
  metrics and structured-log retention.
- `open-bitcoin sync pause` and `open-bitcoin sync resume` already exist as
  explicit operator controls.
- Existing Phase 71 through Phase 75 Bun checkers provide a repeatable pattern
  for artifact anchors and default-verification boundary checks.

### Established Patterns

- Default verification stays deterministic, public-network-free,
  service-manager-free, timing-stable, free of multi-day sleeps, and free of
  large local disk allocations.
- Operator evidence should be typed, compact, and shared across surfaces rather
  than renderer-local prose.
- Storage/resource blockers take precedence over peer retry advice and must not
  trigger hidden mutation.
- Support evidence is local, redacted, and allowlisted. Raw logs, credentials,
  wallet material, raw reports, and unbounded tables stay out.
- New first-party Rust source or test files under package source/test trees need
  parity breadcrumbs.

### Integration Points

- Add resource-bound domain helpers in pure modules before wiring filesystem
  probes or status collection through runtime/operator adapters.
- Extend status/resource projection once, then render through CLI status,
  dashboard, support evidence, soak reports, and docs.
- Enforce soak resource stops in the bounded soak loop so the ledger records the
  triggering evidence and remains resumable.
- Verify behavior with small fake resource probes, temp files, scripted status
  snapshots, and deterministic timestamps.

</code_context>

<specifics>

## Specific Ideas

- The recommended shape is a typed "resource-bound snapshot" consumed by status,
  soak reports, and support evidence, with unavailable reasons for measurements
  that cannot be collected.
- Preflight should distinguish "cannot safely assess", "within budget",
  "approaching limit", and "stop required" instead of flattening everything to
  pass/fail.
- Support-bundle size pressure should be explicit because support evidence is a
  user-visible artifact and can otherwise become a hidden unbounded output path.
- Existing `sync pause/resume` controls should be reused for operator control
  guidance before adding new commands.

</specifics>

<deferred>

## Deferred Ideas

- Corruption markers, schema mismatch, stale locks, partial writes, and
  storage-open recovery detail belongs to Phase 77.
- False-progress prevention, stalled subsystem diagnosis, peer contribution
  windows, and no-progress thresholds belong to Phase 78.
- Full support-bundle forensic timeline, failure narrative, and cross-surface
  final verdict forensics belong to Phase 79.
- Opt-in multi-day soak UAT command closeout and v1.7 release-boundary wording
  belongs to Phase 80.

</deferred>

---

*Phase: 76-disk-and-resource-bound-enforcement*
*Context gathered: 2026-06-15*
