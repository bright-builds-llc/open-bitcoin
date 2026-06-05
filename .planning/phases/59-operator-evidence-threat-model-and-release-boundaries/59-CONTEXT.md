---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 59-2026-06-05T15-10-59
generated_at: 2026-06-05T15:10:59.825Z
---

# Phase 59: Operator Evidence, Threat Model, and Release Boundaries - Context

**Gathered:** 2026-06-05
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 59 closes v1.4 by making the shipped operator evidence coherent across
status, dashboard, metrics, logs, RPC-facing blockchain information, live-smoke
snapshots, support evidence, docs, threat modeling, parity roots, and release
claim boundaries. It consumes Phase 54 through Phase 58 behavior and evidence;
it should not add a new sync capability, broaden public-network verification,
or claim unattended production-node readiness.

This phase owns the final v1.4 evidence packet, support-bundle allowlisting for
the new live-smoke result fields, docs and UAT command clarity, a reviewer-facing
v1.4 threat model, refreshed parity/release-readiness links, and deterministic
checks that prove public-network commands remain opt-in. It does not own inbound
serving, transaction relay, production-funds wallet use, migration apply mode,
packaging, hosted dashboards, GUI work, Windows service certification, or
unattended production operation.

</domain>

<decisions>

## Implementation Decisions

### Cross-Surface Operator Truth

- **D-01:** Treat the shared status snapshot and durable sync metadata as the
  source of truth for Phase 59 consistency checks. Status, dashboard, metrics,
  structured logs, RPC-facing blockchain info, support evidence, and live-smoke
  reports should agree on header height, downloaded block height, connected
  block height, compatibility state, progress signal, and latest error rather
  than each renderer inventing local summaries.
- **D-02:** Close OBS-01 with deterministic assertions over existing projection
  and rendering paths. Prefer focused tests and script checks that compare the
  same fixture data across renderers over broad runtime rewrites.
- **D-03:** Preserve the Phase 56-58 distinction between header progress,
  downloaded block progress, connected block progress, restart/resume evidence,
  and diagnosed blockers. Do not collapse these into a single "synced" flag or
  timing-threshold claim.

### Support Evidence Packet

- **D-04:** Extend redacted support evidence to summarize the v1.4 live-smoke
  schema v2 result fields that reviewers need: status, progress detection,
  first header progress, first block progress, restart/resume evidence,
  recovery diagnosis, selected peer outcome summaries, status snapshot
  summaries, config paths, metrics/log availability, and store health.
- **D-05:** Keep support ingestion allowlist-based and redacted. Raw live-smoke
  input, raw status snapshot arrays, daemon stdout/stderr tails, raw endpoint
  tables, manual peer lists, cookie values, wallet material, unbounded logs, and
  secrets must not be embedded in support bundles.
- **D-06:** Missing live-smoke, metrics, logs, RPC, or store evidence should
  render as unavailable with a reason. Missing optional evidence is diagnostic
  context, not a reason to hide the surface.
- **D-07:** Support bundles remain local review/troubleshooting evidence. They
  are not release validators and do not prove public-mainnet sync success by
  themselves.

### Operator Docs And UAT Commands

- **D-08:** Update operator docs with copy-pasteable repo-local commands for
  deterministic verification, manual-peer live smoke, same-datadir restart and
  resume review, support evidence collection, and pass/fail interpretation.
  Commands should prefer the repo-local Cargo and Bazel forms already required
  by repo guidance.
- **D-09:** Keep generated live-smoke reports, support bundles, daemon logs,
  metrics stores, and local datadirs out of git. Docs may reference local output
  paths and report field names, but should not check in environment-specific
  public-network artifacts.
- **D-10:** Make pass/fail copy evidence-first: explicit field names, accepted
  diagnosed-blocker paths, and next operator action. Avoid implying success
  from peer reachability, elapsed time, support-bundle existence, or daemon
  startup alone.

### Threat Model And Release Boundaries

- **D-11:** Add or refresh a reviewer-facing v1.4 threat model covering public
  peer compatibility handling, header and block input, resource bounds,
  restart/resume evidence, report redaction, support evidence, and
  operator-facing live evidence.
- **D-12:** Refresh parity roots, release-readiness docs, and checklist entries
  so reviewers can distinguish the v1.4 opt-in outbound IBD progress claim from
  deferred inbound serving, transaction relay, production wallet use, migration
  apply mode, packaging, hosted dashboard, GUI, and unattended production-node
  claims.
- **D-13:** Keep v1.3 docs as historical evidence. Add v1.4-specific surfaces or
  clearly labeled v1.4 sections instead of rewriting v1.3 claims as if they
  were the current milestone.

### Verification Posture

- **D-14:** Default verification remains deterministic. `bash scripts/verify.sh`
  must not invoke public-network live smoke or `--restart-after-progress`.
- **D-15:** Add or update repo-owned deterministic checks for the v1.4 release
  boundary when parity roots or threat/release docs change. Reuse the existing
  v1.3 release-boundary checker pattern if it remains the smallest robust path.
- **D-16:** Phase verification should include the repo-native aggregate
  `bash scripts/verify.sh`, targeted support/live-smoke/doc checks, and any
  relevant Rust or Bun fixtures introduced by the plans.

### the agent's Discretion

- The planner may split Phase 59 into code, docs/parity, and verification
  plans if that keeps files and risk isolated.
- The planner may decide whether v1.4 threat-model content lives in a new
  `docs/parity/threat-model-v1.4.md` file or in a clearly separated v1.4
  section, provided parity roots link to it and v1.3 historical evidence remains
  intact.
- The executor may reuse existing support bundle, release-readiness, and
  live-smoke fixtures when they already prove the v1.4 behavior, but summaries
  and verification must make the Phase 59 evidence explicit.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 59 goal, success criteria, and milestone
  closeout boundary.
- `.planning/REQUIREMENTS.md` - OBS-01 through OBS-03 and SEC-01 through
  SEC-03.
- `.planning/PROJECT.md` - v1.4 current state, out-of-scope surfaces, and core
  parity/architecture constraints.
- `.planning/STATE.md` - Current phase state and accumulated operator-evidence
  decisions.

### Prior Phase Evidence

- `.planning/phases/54-peer-compatibility-baseline-and-diagnostic-harness/54-CONTEXT.md`
  - Compatibility diagnostics and baseline-comparison scope.
- `.planning/phases/54-peer-compatibility-baseline-and-diagnostic-harness/54-01-SUMMARY.md`
  - Completed deterministic peer compatibility harness evidence.
- `.planning/phases/55-outbound-handshake-compatibility-fixes/55-CONTEXT.md`
  - Connected-handshake and no-credit compatibility decisions.
- `.planning/phases/55-outbound-handshake-compatibility-fixes/55-01-SUMMARY.md`
  - Completed outbound handshake compatibility behavior.
- `.planning/phases/56-header-ibd-convergence/56-CONTEXT.md` - Header progress,
  first-header evidence, and no-progress diagnosis decisions.
- `.planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md` - Completed
  deterministic header convergence and fresh status evidence.
- `.planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md` -
  Block download, connect, and first-block evidence decisions.
- `.planning/phases/57-block-download-and-connect-progress/57-04-SUMMARY.md` -
  Completed first-block live-smoke evidence and documentation.
- `.planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md`
  - Restart/resume evidence, support-bundle handoff, and recovery diagnosis
  decisions.
- `.planning/phases/58-same-datadir-restart-and-resume-evidence/58-VERIFICATION.md`
  - Passed Phase 58 evidence and remaining Phase 59 handoff.

### Historical Operator Evidence And Release Boundaries

- `.planning/phases/48-support-evidence-and-operator-runbooks/48-CONTEXT.md` -
  Support evidence redaction and local bundle decisions.
- `.planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md` -
  v1.3 threat-model and release-boundary approach to adapt for v1.4.
- `.planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md` - Schema v2
  support-summary decisions and daemon preflight wording.
- `.planning/phases/53-live-evidence-refresh/53-CONTEXT.md` - Fresh live
  evidence refresh strategy and local artifact posture.
- `.planning/milestones/v1.3-MILESTONE-AUDIT.md` - Prior milestone audit
  closeout pattern and residual-boundary language.

### Implementation Surfaces

- `packages/open-bitcoin-cli/src/operator/support.rs` - Support bundle evidence
  model, live-smoke summary extraction, metrics/log/store evidence, and
  redaction.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Support evidence
  Markdown rendering.
- `packages/open-bitcoin-cli/src/operator/status.rs` - Operator status command
  routing and shared snapshot use.
- `packages/open-bitcoin-cli/src/operator/status/sync_state.rs` - Sync status
  projection for operator output.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Human and JSON
  status rendering.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Dashboard model
  projection from shared status data.
- `packages/open-bitcoin-cli/src/operator/dashboard/app.rs` - Dashboard runtime
  rendering and action state.
- `packages/open-bitcoin-rpc/src/context.rs` - RPC-facing sync status,
  pause/resume, and blockchain info integration points.
- `packages/open-bitcoin-node/src/status.rs` - Shared status snapshot and sync
  state fields.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Sync summary,
  progress signal, health, logs, metrics, and recovery output.
- `packages/open-bitcoin-node/src/sync/types/projection.rs` - Durable sync
  status projection and storage-health mapping.
- `scripts/run-live-mainnet-smoke.ts` - Live-smoke schema v2 result fields,
  first-header, first-block, restart/resume, and recovery diagnosis evidence.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke fixture
  checks.
- `scripts/check-v1.3-release-boundaries.ts` - Existing deterministic
  release-boundary checker pattern.

### Reviewer-Facing Docs And Parity Roots

- `docs/operator/runtime-guide.md` - Operator status, live-smoke,
  restart/resume, support bundle, and known-limitation guidance.
- `docs/architecture/status-snapshot.md` - Shared status snapshot contract and
  unavailable-field behavior.
- `docs/architecture/operator-observability.md` - Metrics/log retention and
  public-network verification boundary.
- `docs/architecture/config-precedence.md` - Config source ownership and
  metadata-only credential reporting.
- `docs/parity/README.md` - Parity documentation entrypoint.
- `docs/parity/checklist.md` - Human-readable parity surface inventory and
  known gaps.
- `docs/parity/index.json` - Machine-readable parity root, evidence paths, and
  known gaps.
- `docs/parity/release-readiness.md` - Release-readiness matrix and evidence
  command surface.
- `docs/parity/threat-model-v1.3.md` - Historical v1.3 threat-model pattern and
  boundary matrix.
- `docs/parity/catalog/p2p.md` - v1.4 outbound IBD, live-smoke, and
  restart/resume parity wording.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- Support bundle generation already writes `support-evidence.json` and
  `support-evidence.md`, collects config paths, shared status, durable store
  evidence, metrics history, and allowlisted live-smoke summaries.
- Live-smoke reports already expose schema v2 `result.firstHeaderProgress`,
  `result.firstBlockProgress`, and `result.restartResumeEvidence` fields with
  typed diagnoses.
- Operator status and dashboard surfaces already consume shared status data
  rather than independent node-state models.
- The v1.3 release-boundary checker already demonstrates a scriptable pattern
  for verifying parity roots, threat-model docs, release-readiness docs, and
  deferred-surface wording.

### Established Patterns

- Public-network live evidence is opt-in UAT and stays outside
  `bash scripts/verify.sh`.
- Operator output is quiet, work-focused, and evidence-first.
- Missing live fields should remain visible as unavailable with reasons.
- Support evidence is allowlisted and redacted; raw local artifacts stay local.
- Docs that include UAT commands should use repo-local Cargo and Bazel command
  forms, not just an installed alias.

### Integration Points

- Extend support live-smoke summary extraction and Markdown rendering to cover
  Phase 56-58 result fields without embedding raw report data.
- Add deterministic fixture coverage for support summaries that include header,
  block, restart, recovery, and peer outcome data.
- Add or update parity/release-boundary checks to make v1.4 docs and machine
  roots traceable to OBS-01 through OBS-03 and SEC-01 through SEC-03.
- Update runtime guide and parity docs with final v1.4 pass/fail interpretation
  and claim boundaries.

</code_context>

<specifics>

## Specific Ideas

- Consider a new `docs/parity/threat-model-v1.4.md` and a `scripts/check-v1.4-release-boundaries.ts`
  checker if that is clearer than overloading the v1.3 artifacts.
- Add support-bundle fixture input with schema v2 `firstHeaderProgress`,
  `firstBlockProgress`, and `restartResumeEvidence`, including secret-like raw
  fields that must be absent from output.
- Add a compact operator evidence matrix in `docs/parity/release-readiness.md`
  that maps each Phase 54-58 evidence surface to the final v1.4 claim.
- Ensure docs include exact commands for:
  `bash scripts/verify.sh`,
  `bash scripts/test-run-live-mainnet-smoke.sh`,
  `bun run scripts/run-live-mainnet-smoke.ts --manual-peer ...`,
  `bun run scripts/run-live-mainnet-smoke.ts --restart-after-progress ...`,
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`,
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.

</specifics>

<deferred>

## Deferred Ideas

- Inbound peer serving, transaction relay, production-funds wallet use,
  migration apply mode, packaging, hosted dashboard, GUI work, Windows service
  support, and unattended production-node operation remain future milestones.
- Hosted support upload or support-bundle artifact validation remains future
  work; Phase 59 keeps support evidence local and redacted.
- Any public-network CI or default verification gate remains out of scope for
  v1.4.

</deferred>

---

*Phase: 59-operator-evidence-threat-model-and-release-boundaries*
*Context gathered: 2026-06-05*
