---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 52-2026-05-31T23-48-46
generated_at: 2026-05-31T23:50:17.955Z
---

# Phase 52: Operator Evidence Cleanup - Context

**Gathered:** 2026-05-31
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 52 closes deterministic v1.3 audit tech debt D-02 and D-04 in
operator-facing evidence. It should make support bundles summarize schema v2
live-smoke `result` fields and refresh the `open-bitcoind` daemon sync
preflight wording so it matches the current opt-in sync worker. It must not
change the milestone's public-network claim boundary, add live-network checks to
`bash scripts/verify.sh`, check in generated live reports, or imply unattended
production-node readiness.

</domain>

<decisions>
## Implementation Decisions

### Support Bundle Live-Smoke Summary

- **D-01:** Treat schema v2 live-smoke reports as the primary input shape. The
  support bundle should summarize nested `result` fields for `status`,
  `progressDetected`, `maybeNoProgressCause`, `nextAction`, `headerDelta`, and
  `blockDelta` instead of reporting `summary_fields_unavailable`.
- **D-02:** Preserve the existing top-level allowlist as a compatibility
  fallback for older or hand-authored live-smoke report fixtures, but do not let
  top-level compatibility hide schema v2 nested `result` data.
- **D-03:** Keep support ingestion summary-only and redacted. Do not embed raw
  live-smoke input, raw daemon output tails, raw status snapshots, or raw
  options. Recursively sanitize any summarized strings.
- **D-04:** Markdown output should present the same allowlisted summary values
  reviewers need for audit: status, progress detection, typed no-progress cause,
  next action, header delta, and block delta.

### Deterministic Evidence Tests

- **D-05:** Add or update deterministic support-bundle tests using a schema v2
  fixture with nested `result` fields and raw secret-like report data. The tests
  must prove the nested summary is present and raw live-smoke input remains
  absent from JSON and Markdown.
- **D-06:** Keep missing-artifact and redaction behavior covered. A missing
  live-smoke report should stay a non-fatal unavailable evidence state with a
  reason.

### Daemon Preflight Truth

- **D-07:** Refresh `open-bitcoind` preflight wording to state that preflight
  opened the durable store and that the daemon will start the explicit opt-in,
  bounded mainnet sync worker when enabled.
- **D-08:** The wording must still preserve the non-claim: this is not
  unattended production-node operation or a packaged service guarantee.
- **D-09:** Add a deterministic unit assertion for the rendered preflight
  message so stale wording cannot regress silently.

### Docs And Audit References

- **D-10:** Update operator docs and v1.3 audit references only where readers
  currently have to reconcile stale support-summary or preflight wording debt.
- **D-11:** Mark the Phase 52 cleanup as deterministic debt closure after code,
  docs, and repo-native verification pass. Do not refresh live public-network
  evidence; that remains Phase 53.

### the agent's Discretion

The planner may choose exact helper names, summary key names, and Markdown
formatting, provided the JSON shape remains stable, tests cover the schema v2
nested `result` path, and the implementation stays within the existing
support/preflight modules without adding new dependencies.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Audit Debt

- `.planning/ROADMAP.md` - Phase 52 goal, requirements, gap closure, and success
  criteria.
- `.planning/REQUIREMENTS.md` - OBS-02 and OBS-03 plus v1.3 tech-debt
  follow-up traceability for D-02 and D-04.
- `.planning/v1.3-MILESTONE-AUDIT.md` - D-02 and D-04 debt descriptions and
  follow-up expectations.
- `.planning/STATE.md` - v1.3 scope boundaries and current post-Phase 51 state.

### Prior Phase Decisions

- `.planning/phases/48-support-evidence-and-operator-runbooks/48-CONTEXT.md` -
  Support evidence redaction, local bundle, and optional live-smoke artifact
  decisions.
- `.planning/phases/48-support-evidence-and-operator-runbooks/48-SUMMARY.md` -
  Existing support bundle behavior and verification evidence.
- `.planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md` -
  Release-boundary and support-evidence non-claim decisions.
- `.planning/phases/51-live-smoke-fresh-status-integration/51-CONTEXT.md` -
  Fresh live-smoke status source, schema v2 report, and opt-in evidence
  decisions.
- `.planning/phases/51-live-smoke-fresh-status-integration/51-01-SUMMARY.md` -
  Actual Phase 51 fresh-status implementation and verification commands.

### Runtime And Evidence Surfaces

- `packages/open-bitcoin-cli/src/operator/support.rs` - Support bundle evidence
  model, live-smoke summary extraction, redaction, and JSON output.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Support bundle
  Markdown and command-output rendering.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Operator binary
  support-bundle integration tests.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Daemon sync preflight,
  sync worker startup, and preflight tests.
- `scripts/run-live-mainnet-smoke.ts` - Schema v2 live-smoke report shape and
  Markdown report fields.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke regression
  fixture behavior.

### Documentation

- `docs/operator/runtime-guide.md` - Operator-facing runtime, support bundle,
  live-smoke, and known-limitation language.
- `docs/parity/release-readiness.md` - v1.3 public-mainnet evidence and
  production-node non-claims.
- `docs/parity/checklist.md` and `docs/parity/index.json` - v1.3 evidence roots
  and parity traceability.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `collect_live_smoke_evidence` already treats live-smoke reports as optional
  evidence and reports unavailable/malformed inputs without failing the whole
  support command.
- `sanitize_json_value` and `redact_sensitive_text` already provide recursive
  summary sanitization for allowlisted live-smoke values.
- `render_support_markdown` already has a dedicated Live Smoke section that can
  render richer allowlisted summary values.
- `preflight_daemon_sync`, `report_daemon_sync_preflight`, and
  `start_daemon_sync_worker` already separate durable preflight from worker
  startup in `open-bitcoind`.

### Established Patterns

- Open Bitcoin-only support code uses `none` parity breadcrumbs because there is
  no direct Bitcoin Knots source anchor.
- Rust tests in touched files should keep explicit Arrange, Act, Assert
  sections.
- Public-network live evidence stays opt-in and outside the default
  deterministic verification gate.
- Operator-facing docs prefer repo-local Cargo and Bazel commands over an
  installed alias when showing UAT workflows.

### Integration Points

- Support bundle JSON includes `live_smoke.summary`; Markdown currently prints
  that summary as one JSON value.
- Schema v2 live-smoke reports put audit-critical result values under
  `result`, with top-level `schema_version`, options, snapshots, preflight, and
  endpoint outcome data.
- `open-bitcoind` prints the preflight line before starting the daemon sync
  worker, so the wording should describe preflight plus what enabled startup
  will do next.
- Phase closeout likely needs updates to the audit artifact, roadmap, state,
  requirements tech-debt table, tracked LOC report, and phase verification
  artifacts.

</code_context>

<specifics>
## Specific Ideas

- Prefer a small `live_smoke_summary_from_result` or equivalent helper over a
  broad JSON flattening pass. The useful fields are known and should remain
  allowlisted.
- Prefer a `daemon_sync_preflight_message` helper that returns the exact stderr
  line so tests can assert the refreshed wording without invoking a process.

</specifics>

<deferred>
## Deferred Ideas

- Phase 53 owns live public-network evidence refresh and historical Phase 50
  caveat retirement. Phase 52 should not rerun or replace those live artifacts.

</deferred>

---

*Phase: 52-operator-evidence-cleanup*
*Context gathered: 2026-05-31*
