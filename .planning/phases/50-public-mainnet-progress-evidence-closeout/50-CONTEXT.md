---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 50-2026-05-28T03-06-48
generated_at: 2026-05-28T03:09:46Z
---

# Phase 50: Public Mainnet Progress Evidence Closeout - Context

**Gathered:** 2026-05-28
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 50 closes the v1.3 public-mainnet proof milestone with auditable,
opt-in evidence. The primary path is to capture live public-mainnet header
progress, block progress, and restart/resume evidence from the same durable
datadir. The accepted fallback path, established in Phase 49, is a diagnosed
environment/network blocker that includes typed no-progress cause, endpoint
outcomes, status snapshots, and a concrete next operator action.

This phase owns local evidence capture, phase UAT, and release-readiness
closeout. It must not add public-network checks to `bash scripts/verify.sh`,
check live network fixtures into git, expand daemon scope, or claim unattended
production-node readiness, inbound serving, transaction relay, production-funds
wallet use, migration apply mode, packaging, hosted dashboard, or GUI support.

</domain>

<decisions>
## Implementation Decisions

### Evidence Strategy

- **D-01:** Use the existing opt-in `scripts/run-live-mainnet-smoke.ts` runner
  as the authoritative public-mainnet evidence source. Do not invent a second
  live-network report format for Phase 50.
- **D-02:** Run the smoke against an isolated repo-local or target-local
  public-mainnet datadir so the same durable store can be interrupted and
  reused for restart/resume evidence.
- **D-03:** Keep generated live-smoke and support-bundle artifacts local under
  `packages/target` or an explicit operator-selected output directory. Commit a
  concise phase evidence/UAT summary that points to those local artifacts and
  records the relevant fields reviewers need.
- **D-04:** Prefer successful progress evidence. Only use the diagnosed-blocker
  closeout path when bounded live attempts fail to observe progress but still
  produce typed endpoint/status diagnostics and a next action.

### Public-Network Attempt Shape

- **D-05:** Start with the repo-owned live-mainnet smoke runner and its default
  DNS/manual-peer behavior. If the first attempt does not reach useful progress
  because of endpoint discovery or connectivity, run a bounded retry with an
  explicit manual peer before treating the result as a blocker.
- **D-06:** Capture the first validated header-height increase when observed,
  including peer endpoint, source, timestamp, and before/after durable status
  from the report.
- **D-07:** Capture the first validated block connection beyond genesis or the
  configured checkpoint when observed. If block progress is not reached, record
  the explicit diagnosis from the live-smoke report instead of implying success.
- **D-08:** For restart/resume, reuse the same datadir in a second bounded
  invocation. Claim restart/resume progress only when before/after durable
  header, block, and runtime metadata remain coherent; otherwise document the
  same-datadir blocker evidence and next action.

### Support Evidence And Reviewer Packet

- **D-09:** Generate a support bundle after the live-smoke attempt when the
  operator command is available, using `--include-live-smoke-report` to embed a
  summary of the selected live-smoke JSON. The bundle remains local redacted
  support evidence, not a release validator.
- **D-10:** Add a Phase 50 UAT/evidence artifact under this phase directory with
  the commands run, artifact paths, result status, endpoint outcomes, status
  deltas, no-progress cause if any, restart/resume observation, and next action.
- **D-11:** Refresh release-readiness/parity closeout text only enough to link
  Phase 50 evidence and final v1.3 status. Avoid broad release-doc rewrites.

### Verification Posture

- **D-12:** Keep deterministic repo verification public-network-free. Required
  completion checks include repo-native verification and the Rust pre-commit
  sequence from `AGENTS.md`.
- **D-13:** Treat public-network smoke commands as explicit UAT evidence, not
  CI prerequisites. If an environment blocker is diagnosed, verification passes
  only when the blocker evidence is sufficiently specific for the next
  operator.
- **D-14:** Use repo-local Cargo and Bazel commands in UAT instructions and
  summaries rather than relying on a globally installed `open-bitcoin` alias.

### the agent's Discretion

The planner may choose exact timeout lengths, output directory names, and manual
peer retry strategy, provided the attempts are bounded and the recorded evidence
is auditable. The planner may also decide whether release-readiness updates are
needed after inspecting the live evidence outcome.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 50 goal, dependency, requirements, and success
  criteria.
- `.planning/REQUIREMENTS.md` - PROOF-03, PROOF-04, PROOF-05, and SEC-03
  requirement text.
- `.planning/PROJECT.md` - v1.3 milestone goal, project constraints, active
  scope, and explicit out-of-scope production surfaces.
- `.planning/STATE.md` - Current v1.3 state and public-network proof blocker
  posture.

### Prior Phase Decisions

- `.planning/phases/42-live-smoke-entry-and-network-preflight/42-CONTEXT.md` -
  Opt-in live-smoke runner, typed no-progress causes, manual-peer path, and
  deterministic verification boundary.
- `.planning/phases/44-peer-contribution-attribution/44-CONTEXT.md` - Useful
  peer contribution semantics and live-smoke peer evidence expectations.
- `.planning/phases/46-durable-recovery-and-invalid-data-handling/46-CONTEXT.md`
  - Durable header/block progress, invalid-data attribution, and recovery
  guidance decisions.
- `.planning/phases/47-operator-sync-truth-surfaces/47-CONTEXT.md` - Shared
  operator status truth model and deterministic verification posture.
- `.planning/phases/48-support-evidence-and-operator-runbooks/48-CONTEXT.md` -
  Support evidence, redaction, optional live-smoke artifact, and Phase 50
  handoff decisions.
- `.planning/phases/48-support-evidence-and-operator-runbooks/48-SUMMARY.md` -
  Implemented support bundle outputs, redaction coverage, docs updates, and
  residual risk.
- `.planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md` -
  Release-boundary and Phase 50 progress-or-blocker acceptance decisions.
- `.planning/phases/49-threat-model-and-release-boundaries/49-01-SUMMARY.md` -
  v1.3 threat model implementation details.
- `.planning/phases/49-threat-model-and-release-boundaries/49-02-SUMMARY.md` -
  Parity-root release boundary assertions and closeout wiring.

### Parity, Release, And Operator Docs

- `docs/parity/threat-model-v1.3.md` - Phase 50 acceptance contract, threat
  boundaries, and live evidence handling.
- `docs/parity/release-readiness.md` - Current readiness verdict, evidence
  commands, live-mainnet evidence posture, and reviewer checklist.
- `docs/parity/checklist.md` - Human-readable parity surface checklist and
  evidence root.
- `docs/parity/index.json` - Machine-readable parity root, deviations, and
  catalog inventory.
- `docs/parity/deviations-and-unknowns.md` - Deferred surfaces, suspected
  unknowns, and follow-up triggers.
- `docs/operator/runtime-guide.md` - Mainnet sync activation, live-smoke
  commands, support bundle commands, redaction boundaries, and known
  limitations.
- `docs/architecture/status-snapshot.md` - Shared status snapshot, durable
  progress semantics, support bundle embedding, and unavailable-field rules.

### Evidence And Runtime Surfaces

- `scripts/run-live-mainnet-smoke.ts` - Opt-in public-mainnet smoke report
  writer, endpoint diagnostics, status snapshots, progress/no-progress result,
  and Markdown rendering.
- `packages/open-bitcoin-cli/src/operator/support.rs` - Support bundle
  collection and output contract.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Support evidence
  Markdown rendering.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Local daemon binary
  used by the live-mainnet smoke runner.
- `packages/open-bitcoin-node/src/sync/` - Durable sync runtime and peer
  contribution/status source.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `scripts/run-live-mainnet-smoke.ts` already writes JSON and Markdown reports
  under `packages/target/live-mainnet-smoke-reports` by default.
- Smoke reports include `result.status`, `result.progressDetected`,
  `result.headerDelta`, `result.blockDelta`, `result.maybeNoProgressCause`,
  `result.nextAction`, endpoint outcomes, before/after status snapshots,
  runtime peer telemetry, and daemon output tails.
- The report status enum already distinguishes `passed`, `preflight_failed`,
  `runtime_failed`, `no_progress`, and `cancelled`.
- Phase 48 support bundle generation already writes `support-evidence.json` and
  `support-evidence.md` with redacted config evidence, status snapshot, store
  health, and optional live-smoke summary fields.

### Established Patterns

- Public-network evidence is opt-in, local, and not part of the default
  deterministic verifier.
- Generated live-smoke/support artifacts are local evidence under
  `packages/target` or an explicit output directory, not checked-in fixtures.
- Release claims remain evidence-based and explicit about non-goals.
- Peer activity is not useful contribution unless headers or blocks are accepted
  by sync handling.
- Durable status keeps validated headers, downloaded blocks, and connected
  blocks distinct across operator surfaces.

### Integration Points

- Run the live-mainnet smoke runner from the repo root with explicit datadir and
  output directory arguments.
- Inspect generated JSON with structured tooling and summarize only the fields
  needed for review.
- Generate a support bundle with a repo-local Cargo command if the selected
  live-smoke artifact should be embedded into support evidence.
- Update release-readiness/parity docs only if the final evidence outcome
  changes the v1.3 closeout status.

</code_context>
