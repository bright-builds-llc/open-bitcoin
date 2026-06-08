---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 65-2026-06-08T14-45-59
generated_at: 2026-06-08T14:46:10.823Z
---

# Phase 65: Support Bundle and Operator Review Docs - Context

**Gathered:** 2026-06-08
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 65 expands local support evidence and operator review documentation for
v1.5 unattended mainnet node operation readiness. It owns redacted support bundle
coverage for long-run sync cycles, service state, restart/recovery evidence,
peer outcomes, progress counters, stop reasons, metrics, structured logs, config
sources, and copy-pasteable repo-local review commands.

This phase does not add the Phase 66 compatibility harness wrapper, Phase 67
release-boundary closeout, public-network checks to default verification,
production-node support claims, inbound serving, relay behavior,
production-funds wallet use, migration apply mode, packaging, hosted dashboards,
Windows service integration, or GUI work.

</domain>

<decisions>

## Implementation Decisions

### Support Bundle Evidence Contract

- **D-01:** Keep support bundles as local, redacted evidence bundles, not raw
  report archives. The bundle should continue to write `support-evidence.json`
  and `support-evidence.md` under an explicit output directory and must not
  embed raw live-smoke reports, daemon stdout/stderr tails, endpoint tables,
  wallet material, credentials, raw logs, or unbounded report arrays.
- **D-02:** Extend the existing allowlisted evidence shape instead of adding a
  second support DTO. The bundle should preserve the shared
  `OpenBitcoinStatusSnapshot`, `store_health`, config path/credential metadata,
  metrics history availability, and compact live-smoke summary while adding only
  bounded v1.5 fields needed for OBS-03.
- **D-03:** The v1.5 support summary must be able to diagnose unattended
  operation from compact facts: long-run sync lifecycle/phase, configured
  targets, attempt counters, latest progress signal, stop reason, recovery
  category/action, peer health/outcomes, header/downloaded/connected progress,
  service lifecycle state, restart/resume evidence, metrics availability and
  sample counts, structured-log availability, and config sources.
- **D-04:** Missing local evidence is diagnostic evidence. If status, durable
  metadata, metrics, logs, service state, or live-smoke input is unavailable,
  render an explicit unavailable reason instead of silently omitting the section
  or substituting zero-like success values.

### Redaction And Boundaries

- **D-05:** Preserve and strengthen the allowlist posture from Phases 59, 61,
  62, and 64. Credential evidence remains metadata-only; cookie contents,
  `rpcpassword`, `rpcauth`, private keys, seed phrases, wallet files, raw local
  reports, raw peer endpoint tables, and unbounded daemon logs are never support
  evidence.
- **D-06:** Live-smoke schema v2 input remains summary-only. For Phase 65, the
  allowlist should include compact long-run and restart fields already produced
  by earlier phases, but any newly accepted keys must be individually named,
  rendered in Markdown, and covered by redaction regression tests with forbidden
  raw markers.
- **D-07:** Support bundle existence does not prove sync success. Docs must tell
  operators to interpret specific fields such as progress deltas, final status,
  stop reason, recovery category, service lifecycle, restart verdicts, and
  next-action guidance.

### Operator Review Documentation

- **D-08:** Refresh `docs/operator/runtime-guide.md` around a v1.5 operator
  review flow: deterministic repo checks first, opt-in long-run daemon review,
  service-based review, support bundle collection, and pass/fail interpretation.
  Every operator workflow must include repo-local Cargo and Bazel command forms
  rather than relying only on an installed `open-bitcoin` alias.
- **D-09:** Keep public-network and service-manager workflows explicitly opt-in
  UAT evidence. `bash scripts/verify.sh` must remain deterministic and must not
  run `run-live-mainnet-smoke`, `--manual-peer`, `--restart-after-progress`,
  `systemctl --user`, `launchctl`, or real service-manager operations.
- **D-10:** Update architecture docs only where the support evidence contract
  changes: `status-snapshot.md` for the shared snapshot/support relationship and
  `operator-observability.md` for bounded metrics/log/support evidence
  retention. Do not broaden release claims in Phase 65.

### Deterministic Verification

- **D-11:** Add focused Rust tests for support bundle JSON/Markdown shape and
  redaction when the support code changes. Tests should use fixtures and fake
  status evidence rather than public-network runs or real service managers.
- **D-12:** Add a Phase 65 Bun checker if docs or verification-boundary text needs
  deterministic enforcement. The checker should assert required v1.5 review
  commands and support evidence field names, and assert that default
  verification excludes public-network and real service-manager commands.
- **D-13:** Run repo-native verification before finalizing the phase. Keep any
  tracked generated LOC freshness changes if `bash scripts/verify.sh` regenerates
  them.

### the agent's Discretion

- The planner may split work by support bundle schema/redaction, docs/checker,
  and focused operator-binary tests if that keeps plans reviewable.
- The executor may add small pure support-summary helpers when they reduce
  duplication between JSON allowlisting and Markdown rendering.
- If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, parity
  breadcrumbs must be updated before committing.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 65 goal, success criteria, OBS-03/OBS-04
  requirements, and deferred Phase 66/67 boundaries.
- `.planning/REQUIREMENTS.md` - OBS-03, OBS-04, REL-03, v1.5 out-of-scope
  boundaries, and default-verification public-network exclusion.
- `.planning/PROJECT.md` - v1.5 milestone goal, current state, parity baseline,
  and production-claim limits.
- `.planning/STATE.md` - Active milestone state and prior decisions about
  deterministic verification, support evidence, and opt-in public-network UAT.

### Prior Phase Decisions And Evidence

- `.planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-CONTEXT.md`
  - Service restart/resume evidence contract, same-datadir proof, and opt-in UAT
  boundaries that Phase 65 support bundles should summarize.
- `.planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-VERIFICATION.md`
  - Passed Phase 64 evidence and restart/resume verification notes.
- `.planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md` - Service
  lifecycle labels, launchd/systemd operator review flow, and service-manager
  default-verification exclusions.
- `.planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md` - Shared sync
  truth fields, metrics/log projections, live-smoke compactness, and
  unavailable-field decisions.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` -
  Recovery category, resource pressure, allowlisted support evidence, and
  deterministic checker decisions.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-04-SUMMARY.md`
  - Completed support bundle recovery/resource evidence and redaction tests.
- `.planning/phases/60-unattended-sync-loop-control/60-CONTEXT.md` - Unattended
  loop activation, stop-reason, pause/resume, and deterministic verification
  decisions.
- `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md`
  - v1.4 support evidence redaction and release-boundary posture.

### Implementation Surfaces

- `packages/open-bitcoin-cli/src/operator/support.rs` - Support bundle data
  model, config evidence, store health, metrics history, live-smoke evidence,
  and redaction summary.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` - Allowlisted
  live-smoke summary projection and sensitive text redaction.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Human Markdown
  support evidence rendering.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Operator binary
  support bundle regression tests for JSON/Markdown, live-smoke summaries, and
  forbidden raw marker redaction.
- `packages/open-bitcoin-cli/src/operator/status.rs` - Shared status collection
  and service/sync projection into `OpenBitcoinStatusSnapshot`.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Human and JSON
  status rendering for operator review commands.
- `packages/open-bitcoin-cli/src/operator/service.rs` - Service lifecycle state,
  command outcomes, and service status rendering.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Dashboard service
  and sync truth projection.
- `packages/open-bitcoin-node/src/status.rs` - Shared status snapshot, sync
  progress, recovery, resource pressure, service, metrics, logs, and unavailable
  field contracts.
- `packages/open-bitcoin-node/src/metrics.rs` - Bounded metrics retention and
  status samples.
- `packages/open-bitcoin-node/src/storage.rs` - Runtime metadata and recovery
  marker evidence surfaced by support bundles.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in live-smoke JSON/Markdown report
  and compact long-run/restart evidence input.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke fixture
  guard that also enforces public-network exclusion from default verification.
- `scripts/verify.sh` - Repo-native deterministic verification contract.
- `scripts/check-phase61-resource-recovery-boundaries.ts`,
  `scripts/check-phase62-sync-truth-surfaces.ts`,
  `scripts/check-phase63-service-lifecycle.ts`, and
  `scripts/check-phase64-service-restart-resume.ts` - Existing deterministic
  checker patterns for docs/default-verification boundaries.

### Operator Docs And Parity Roots

- `docs/operator/runtime-guide.md` - Operator deterministic checks, opt-in
  long-run review, service review, support bundle collection, and pass/fail
  interpretation.
- `docs/architecture/status-snapshot.md` - Shared status snapshot and support
  bundle ownership contract.
- `docs/architecture/operator-observability.md` - Metrics/log retention and
  bounded support evidence vocabulary.
- `docs/architecture/config-precedence.md` - Config source and credential
  reporting boundaries.
- `docs/parity/release-readiness.md` - Current release claim boundaries that
  Phase 65 must not broaden.
- `docs/parity/catalog/p2p.md` - Public-network and restart/resume parity
  evidence boundaries.
- `docs/parity/index.json` - Machine-readable parity roots and evidence links.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `SupportEvidenceBundle` already writes JSON and Markdown, includes redaction
  metadata, config evidence, `OpenBitcoinStatusSnapshot`, store health, metrics
  history, and live-smoke summary evidence.
- `live_smoke::summary` already supports schema v2 summary extraction for first
  header progress, first block progress, restart/resume evidence, recovery
  diagnosis, final status, and resource pressure while dropping raw snapshots,
  daemon tails, options, endpoint tables, and secrets.
- `render_support_markdown` already renders config, status, store health, and
  compact live-smoke evidence; it is the natural place to add bounded v1.5
  labels that operators need during support review.
- `operator_binary.rs` already has support bundle tests that assert redaction,
  unavailable live-smoke behavior, and forbidden raw-marker absence.
- Existing TypeScript checkers show the repo pattern for deterministic docs and
  verification-boundary enforcement.

### Established Patterns

- CLI/support layers project typed facts and compact summaries; domain truth
  stays in `open-bitcoin-node` status/sync/storage types.
- Support evidence is allowlist-based and local. Raw reports, raw daemon logs,
  endpoint tables, credentials, wallet material, and unbounded arrays stay out.
- Operator docs use repo-local Cargo and Bazel commands.
- Public-network live-smoke, manual peers, restart-after-progress, and real
  launchd/systemd operations remain opt-in UAT and outside `bash scripts/verify.sh`.
- Bun is the canonical runtime for repo-owned TypeScript checkers.

### Integration Points

- Extend support bundle summary extraction and rendering together so JSON and
  Markdown stay aligned.
- Add or update support bundle tests before broad docs work so redaction and
  field coverage are pinned.
- Add a Phase 65 checker if docs need exact command and boundary guarantees,
  and wire it into `scripts/verify.sh` only if it is deterministic and local.
- Refresh `docs/operator/runtime-guide.md` to give an end-to-end v1.5 review
  sequence while preserving existing v1.4/v1.5 boundaries.

</code_context>

<specifics>

## Specific Ideas

- Prefer an "operator review sequence" in the runtime guide: run
  `bash scripts/verify.sh`; inspect deterministic live-smoke fixtures; optionally
  run public-network long-run review; optionally run service review; collect a
  support bundle; then interpret exact fields instead of elapsed time or bundle
  existence.
- Include both command forms for support collection and status inspection:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.
- Keep the support bundle output under local paths such as
  `/tmp/open-bitcoin-support` and tell reviewers not to commit local reports,
  bundles, logs, metrics stores, or datadir contents.

</specifics>

<deferred>

## Deferred Ideas

- Phase 66 owns the operator wrapper around the public-peer compatibility
  harness and its JSON/Markdown report shape.
- Phase 67 owns final v1.5 threat-model, release-readiness, parity-root, and
  deterministic release-boundary closeout.
- Production-node support, inbound serving, transaction relay, compact block
  relay, production-funds wallet use, destructive migration apply mode, signed
  packaging, hosted dashboards, GUI work, Windows service integration, and broad
  distribution claims remain out of scope.

</deferred>

---

*Phase: 65-support-bundle-and-operator-review-docs*
*Context gathered: 2026-06-08 via yolo discussion*
