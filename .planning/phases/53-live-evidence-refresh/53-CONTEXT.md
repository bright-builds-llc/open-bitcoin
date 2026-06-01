---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 53-2026-06-01T02-51-22
generated_at: 2026-06-01T02:51:22Z
---

# Phase 53: Live Evidence Refresh - Context

**Gathered:** 2026-06-01
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 53 refreshes opt-in public-mainnet evidence after the Phase 51
fresh-status live-smoke fix and Phase 52 support-summary cleanup. The phase
owns rerunning or superseding the Phase 44 contribution UAT and Phase 50
selected closeout evidence so v1.3 can archive without unresolved historical
artifact caveats.

This phase must keep generated live-smoke and support-bundle reports local,
must not add public-network checks to `bash scripts/verify.sh`, and must not
broaden the milestone claim into unattended production-node operation, inbound
serving, transaction relay, production-funds wallet use, migration apply mode,
packaging, hosted dashboard, or GUI support.
</domain>

<decisions>
## Implementation Decisions

### Evidence Refresh Strategy

- **D-01:** Use the existing opt-in `scripts/run-live-mainnet-smoke.ts` runner
  as the authoritative evidence source. Do not add a second live-evidence
  format or a new default verification path.
- **D-02:** Run Phase 53 evidence into fresh phase-specific local output paths
  under `packages/target`, with a phase-specific datadir and report directory
  unless a same-datadir reuse is explicitly needed for restart/resume evidence.
- **D-03:** Start with a bounded default live-smoke attempt. If endpoint
  discovery, TCP reachability, handshake, timeout, or no-progress evidence is
  inconclusive, run one bounded manual-peer retry before selecting the closeout
  report.
- **D-04:** Prefer observed header/block progress when the public network allows
  it. When bounded attempts do not observe progress, accept a fresh
  diagnosed-blocker report only if it includes schema v2 `result` fields,
  endpoint outcomes, fresh-status snapshots from `openbitcoinsyncstatus`, a
  typed no-progress cause, and a concrete next operator action.

### Contribution And Historical Caveat Closeout

- **D-05:** Supersede the Phase 44 optional public-network contribution UAT only
  with fresh Phase 53 evidence that either shows useful peer contribution rows
  or records why the live network did not allow contribution observation in this
  environment.
- **D-06:** Supersede the Phase 50 historical selected report caveat with Phase
  53 evidence generated after the Phase 51 fresh-status fix. Do not rewrite
  history or edit generated Phase 50 report paths as if they had fresh-status
  snapshots.
- **D-07:** If Phase 53 still closes through a diagnosed blocker, make the
  milestone audit language distinguish accepted environmental no-progress from
  stale-artifact debt. The old caveat should no longer be framed as unresolved
  if the new selected report uses fresh-status evidence.
- **D-08:** If Phase 53 observes progress, update the reviewer-facing docs to
  state exactly which header/block/restart/contribution claims are proven and
  leave all non-goals intact.

### Support Evidence And Reviewer Packet

- **D-09:** Generate a redacted support bundle for the selected Phase 53
  live-smoke report when the support command is available, relying on Phase 52
  schema v2 nested `result` summaries. The support bundle remains local
  reviewer context, not proof by itself.
- **D-10:** Create a committed Phase 53 UAT/evidence artifact that records exact
  repo-local commands, local artifact paths, selected report fields, endpoint
  outcomes, fresh-status snapshot evidence, contribution rows or blocker reason,
  support-bundle summary, requirement verdicts, and next operator action.
- **D-11:** Update parity roots, release readiness, requirements tech-debt
  traceability, roadmap, state, and milestone audit only enough to close D-01
  and D-03 with the actual Phase 53 outcome.

### Verification Posture

- **D-12:** Keep deterministic verification public-network-free. Required
  commit-gating verification remains the repo-native `bash scripts/verify.sh`
  plus the applicable Rust pre-commit checks from repo guidance.
- **D-13:** Treat live public-network commands as explicit UAT evidence. A
  public-network blocker is acceptable only when the selected report is fresh,
  typed, auditable, and actionable.
- **D-14:** Use copy-pasteable repo-local Cargo and Bazel commands in UAT and
  final operator instructions instead of relying on an installed
  `open-bitcoin` alias.

### the agent's Discretion

The planner may choose exact timeout lengths, output directory names, manual
peer candidates, and whether a second same-datadir invocation is useful after
the first selected report. Keep attempts bounded and avoid checking generated
live artifacts into git.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Audit Debt

- `.planning/ROADMAP.md` - Phase 53 goal, requirements, gap closure, and
  success criteria.
- `.planning/REQUIREMENTS.md` - PEER-03, PROOF-03, PROOF-04, PROOF-05, OBS-02,
  SEC-03, and v1.3 tech-debt follow-up rows for D-01 and D-03.
- `.planning/v1.3-MILESTONE-AUDIT.md` - Remaining D-01 and D-03 debt
  descriptions and archive routing.
- `.planning/STATE.md` - Current v1.3 state, public-network boundaries, and
  pending archive todo.

### Prior Phase Decisions And Evidence

- `.planning/phases/44-peer-contribution-attribution/44-CONTEXT.md` - Useful
  peer contribution semantics and live-smoke contribution evidence
  expectations.
- `.planning/phases/50-public-mainnet-progress-evidence-closeout/50-CONTEXT.md`
  - Progress-or-diagnosed-blocker evidence strategy and local artifact posture.
- `.planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md` -
  Historical selected Phase 50 report, handshake blocker, fresh-status
  amendment, and support-summary amendment.
- `.planning/phases/51-live-smoke-fresh-status-integration/51-CONTEXT.md` -
  Fresh `openbitcoinsyncstatus` polling decisions.
- `.planning/phases/51-live-smoke-fresh-status-integration/51-01-SUMMARY.md` -
  Implemented fresh-status runner behavior and verification evidence.
- `.planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md` - Schema v2
  support-summary and preflight wording decisions.
- `.planning/phases/52-operator-evidence-cleanup/52-01-SUMMARY.md` - Implemented
  schema v2 support summary behavior and Phase 52 amendments.

### Runtime And Evidence Surfaces

- `scripts/run-live-mainnet-smoke.ts` - Opt-in public-mainnet smoke runner,
  schema v2 report shape, endpoint diagnostics, fresh-status snapshots, and
  Markdown rendering.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic regression for
  live-smoke report generation and fresh-status polling.
- `packages/open-bitcoin-cli/src/operator/support.rs` - Redacted support
  evidence bundle and schema v2 live-smoke summary extraction.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Support evidence
  Markdown rendering.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Daemon sync preflight
  and opt-in sync worker startup.
- `packages/open-bitcoin-node/src/sync/` - Durable sync runtime, status, and
  peer contribution source.

### Reviewer-Facing Docs

- `docs/operator/runtime-guide.md` - Operator live-smoke, support-bundle,
  troubleshooting, and non-claim language.
- `docs/parity/release-readiness.md` - v1.3 evidence acceptance contract,
  Phase 50 closeout, Phase 51/52 amendments, and release boundaries.
- `docs/parity/checklist.md` - Human-readable parity surface rows and known
  gaps.
- `docs/parity/index.json` - Machine-readable parity roots, evidence paths,
  known gaps, and suspected unknowns.
- `docs/parity/threat-model-v1.3.md` - Live evidence handling, support
  redaction, and release-claim boundary.
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `scripts/run-live-mainnet-smoke.ts` already accepts `--datadir`,
  `--manual-peer`, `--output-dir`, `--timeout-seconds`, and `--poll-seconds`,
  writes schema v2 JSON/Markdown reports, and records `result.status`,
  `result.progressDetected`, `result.maybeNoProgressCause`,
  `result.nextAction`, `result.headerDelta`, and `result.blockDelta`.
- The live-smoke runner now calls `openbitcoinsyncstatus` for per-poll
  snapshots and records the status command in the generated report.
- Phase 52 support bundles summarize schema v2 nested `result` fields and keep
  raw live-smoke inputs, daemon tails, raw snapshots, raw options, and endpoint
  tables out of redacted support evidence.
- Existing parity roots already point to Phase 50, Phase 51, and the live-smoke
  runner; Phase 53 can update those rows rather than inventing new doc roots.

### Established Patterns

- Public-network evidence is opt-in, bounded, local, and never part of default
  deterministic verification.
- Generated reports under `packages/target` are not checked into git; committed
  UAT files summarize paths and selected fields.
- Reviewer-facing docs preserve explicit non-goals and do not imply broader
  production-node readiness.
- Rust tests use explicit Arrange, Act, Assert sections; repo-owned scripts use
  Bun TypeScript and thin Bash wrappers.

### Integration Points

- Phase 53 UAT should parse the selected live-smoke JSON with structured tooling
  and copy only audit-critical fields into committed Markdown.
- Support evidence can be generated with the repo-local Cargo CLI command and
  `--include-live-smoke-report` pointing at the selected Phase 53 report.
- Parity and audit updates should be outcome-driven: progress evidence, fresh
  diagnosed blocker, or explicit operator/environment blocker.
</code_context>

<specifics>
## Specific Ideas

- Use phase-specific local paths such as
  `packages/target/phase53-mainnet-datadir` and
  `packages/target/live-mainnet-smoke-reports/phase53-*` so the Phase 53
  evidence is clearly separate from the historical Phase 50 artifacts.
- If manual peers are needed, prefer recording both the attempted peer list and
  the endpoint outcome summary in the committed UAT rather than checking in the
  generated report.
- If the selected Phase 53 report still records `handshake_failure`, `timeout`,
  or another no-progress cause, close the old stale-artifact caveat by pointing
  to fresh-status snapshots and next action, not by claiming progress.
</specifics>

<deferred>
## Deferred Ideas

- Future production-node, inbound-serving, transaction-relay, packaging,
  hosted-dashboard, GUI, artifact-validator, or broader release-gate work
  remains outside v1.3.
</deferred>

---

*Phase: 53-live-evidence-refresh*
*Context gathered: 2026-06-01*
