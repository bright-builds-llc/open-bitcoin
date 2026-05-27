---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 49-2026-05-27T21-24-44
generated_at: 2026-05-27T21:29:02.646Z
---

# Phase 49: Threat Model and Release Boundaries - Context

**Gathered:** 2026-05-27
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 49 gives reviewers a release-facing v1.3 threat model, explicit shipped
claim boundaries, and acceptance criteria for the final public-mainnet evidence
closeout. It should make the security posture and non-claims auditable before
Phase 50 captures live evidence.

This phase owns documentation and reviewer traceability only. It must not add a
new public-network verification gate, change support bundle schema, expand the
daemon runtime, or claim inbound serving, transaction relay, production-funds
wallet use, migration apply mode, packaging, hosted/public web surfaces, GUI
work, or unattended production-node readiness.

</domain>

<decisions>
## Implementation Decisions

### Threat Model Scope

- **D-01:** Create a consolidated, reviewer-facing v1.3 scoped threat model
  instead of a planning-only security note or a broad formal certification
  artifact.
- **D-02:** Cover the SEC-01 domains directly: public peer input, resource
  exhaustion, storage corruption, operator RPC controls, log/report redaction,
  and live evidence handling.
- **D-03:** Use a compact STRIDE-style register with assets, trust boundaries,
  mitigations, residual risks, and evidence links. Every threat entry should
  stay tied to shipped v1.3 surfaces or explicit future gates.

### Release Claim Boundaries

- **D-04:** Keep the authoritative claim boundary in the parity and release
  readiness docs. The boundary should name the proven v1.3 claim, accepted
  evidence, explicit non-claim, future gate, and related requirement for each
  public-mainnet or production-adjacent surface.
- **D-05:** Refresh checklist/deviation docs only enough to make the same
  boundary discoverable. Do not create a separate support-bundle manifest or
  machine-readable release-claims schema in this phase.
- **D-06:** Keep operator-facing limitation language consistent with
  `docs/operator/runtime-guide.md`: source-built, opt-in, local evidence,
  deterministic default verification, and no production-node or production-funds
  claim.

### Live Evidence Acceptance Criteria

- **D-07:** Document an artifact-first reviewer contract for Phase 50. The
  acceptance path should use existing commands and artifacts: `bash
  scripts/verify.sh`, `bun run scripts/run-live-mainnet-smoke.ts`, repo-local
  Cargo/Bazel support bundle commands, live-smoke JSON/Markdown, support
  evidence JSON/Markdown, and status snapshots.
- **D-08:** Phase 50 evidence may close either with observed header/block and
  restart/resume progress or with a diagnosed environment/network blocker that
  includes typed no-progress cause, endpoint outcomes, status snapshots, and a
  next operator action.
- **D-09:** Public-network checks remain opt-in and outside
  `bash scripts/verify.sh`. Phase 49 should not add hosted CI network checks or
  checked-in live-report fixtures.

### Reviewer Traceability

- **D-10:** Add docs-first traceability in the existing parity/release-readiness
  review path. Map PROOF-06, SEC-01, and SEC-02 to roadmap phases, evidence
  docs, support artifacts, and deferred claims without changing runtime or
  support tooling.
- **D-11:** Link the threat model and acceptance criteria from the existing
  parity roots so reviewers can start at `docs/parity/index.json`,
  `docs/parity/checklist.md`, or `docs/parity/release-readiness.md`.
- **D-12:** Treat the Phase 48 support bundle as local redacted evidence only.
  It should not become a release validator and should not imply that support
  tooling proves public-mainnet readiness.

### Verification Posture

- **D-13:** Verification for this phase should be deterministic documentation
  validation plus repo-native checks. If JSON parity roots are changed, add or
  update a scriptable assertion that the new docs are linked and requirements
  remain traceable.
- **D-14:** Run `bash scripts/verify.sh` before completion. If only docs and
  planning artifacts change, no public-network command is required for Phase 49.

### the agent's Discretion

The planner may decide whether the traceability matrix lives directly in
`docs/parity/release-readiness.md` or in a dedicated linked parity document if
the table becomes too large. The planner may also choose exact threat IDs and
section names, provided the resulting docs are concise, linkable, and
machine-checkable where the repo already has JSON roots.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 49 goal, dependency, requirements, and success
  criteria.
- `.planning/REQUIREMENTS.md` - PROOF-06, SEC-01, and SEC-02 requirement text.
- `.planning/PROJECT.md` - v1.3 milestone goal, project constraints, active
  scope, and explicit out-of-scope production surfaces.
- `.planning/STATE.md` - Current v1.3 state, prior decisions, and public
  network proof blockers.

### Prior Phase Decisions

- `.planning/phases/42-live-smoke-entry-and-network-preflight/42-CONTEXT.md` -
  Opt-in live-smoke entrypoint, typed no-progress causes, manual-peer path, and
  deterministic verification posture.
- `.planning/phases/43-outbound-peer-resilience/43-CONTEXT.md` - Peer failure,
  retry/backoff, waiting state, and no production-node claim boundary.
- `.planning/phases/44-peer-contribution-attribution/44-CONTEXT.md` - Useful
  peer contribution semantics and live-smoke peer evidence expectations.
- `.planning/phases/45-runtime-resource-bounds-and-store-coordination/45-CONTEXT.md`
  - Runtime resource bounds and second-writer store coordination decisions.
- `.planning/phases/46-durable-recovery-and-invalid-data-handling/46-CONTEXT.md`
  - Durable progress, invalid-data attribution, and recovery guidance
  decisions.
- `.planning/phases/47-operator-sync-truth-surfaces/47-CONTEXT.md` - Shared
  operator status truth model and deterministic verification posture.
- `.planning/phases/48-support-evidence-and-operator-runbooks/48-CONTEXT.md` -
  Support evidence, redaction, optional live-smoke artifact, and Phase 49/50
  handoff decisions.
- `.planning/phases/48-support-evidence-and-operator-runbooks/48-SUMMARY.md` -
  Implemented support bundle outputs, redaction coverage, docs updates, and
  residual risk.

### Parity And Release Docs

- `docs/parity/release-readiness.md` - Current readiness verdict, evidence
  commands, live-mainnet evidence posture, security analysis audit, and reviewer
  checklist.
- `docs/parity/deviations-and-unknowns.md` - Intentional deviations, deferred
  surfaces, suspected unknowns, and follow-up triggers.
- `docs/parity/checklist.md` - Human-readable parity surface checklist and
  evidence root.
- `docs/parity/index.json` - Machine-readable parity root, deviations, and
  catalog inventory.
- `docs/parity/README.md` - Parity documentation entrypoint.

### Operator And Evidence Surfaces

- `docs/operator/runtime-guide.md` - Mainnet sync activation, live-smoke
  commands, support bundle commands, redaction boundaries, benchmark modes, and
  known limitations.
- `docs/architecture/status-snapshot.md` - Shared status snapshot, support
  bundle embedding, progress semantics, metrics/logs, and unavailable-field
  rules.
- `docs/architecture/operator-observability.md` - Metrics/log retention and
  public-network verification boundary.
- `docs/architecture/config-precedence.md` - Config source ownership,
  precedence, and credential metadata-only reporting.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in live-mainnet smoke artifact
  writer and no-progress report source.
- `packages/open-bitcoin-cli/src/operator/support.rs` - Support bundle
  collection and output contract.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Support evidence
  Markdown rendering.

### Prior Security Evidence

- `.planning/milestones/v1.2-phases/40-live-mainnet-smoke-docs-and-parity-closeout/40-SECURITY.md`
  - Previous live-mainnet closeout security notes.
- `.planning/milestones/v1.2-phases/41-audit-and-revisit-all-the-security-analyses-throughout-the-p/41-SECURITY-AUDIT.md`
  - v1.2 security audit result and deferred future threat-model triggers.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- The Phase 48 support bundle already writes `support-evidence.json` and
  `support-evidence.md` with config evidence, `OpenBitcoinStatusSnapshot`,
  store-health availability, redaction metadata, and allowlisted live-smoke
  summary fields.
- `scripts/run-live-mainnet-smoke.ts` already writes local JSON/Markdown
  reports with endpoint outcomes, status snapshots, typed no-progress causes,
  daemon output tails, and peer contribution rows.
- `docs/operator/runtime-guide.md` already contains repo-local Cargo and Bazel
  command examples, support bundle usage, and v1.3 known limitations.
- `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, and
  `docs/parity/index.json` already form the reviewer entrypoint pattern.

### Established Patterns

- Default verification remains deterministic and public-network-free.
- Generated live-smoke, support, and benchmark reports are local evidence under
  `packages/target` or an operator-selected output directory, not checked-in
  release fixtures.
- Public-facing claims are evidence-based and explicit about non-goals.
- Parity docs name deferred surfaces instead of silently omitting them.
- Redaction preserves metadata and typed status while excluding secrets, wallet
  private material, raw credential values, and unbounded raw logs.

### Integration Points

- Add or update reviewer-facing parity docs for the v1.3 threat model,
  acceptance criteria, and claim boundary matrix.
- Link new docs from `docs/parity/release-readiness.md`,
  `docs/parity/checklist.md`, and `docs/parity/index.json`.
- Update `docs/parity/deviations-and-unknowns.md` if deferred or suspected
  unknown language needs v1.3-specific refresh.
- Update `docs/operator/runtime-guide.md` only if its limitations or evidence
  commands drift from the threat/release docs.
- Add deterministic documentation/root validation when changing
  machine-readable parity roots.

</code_context>

<specifics>
## Specific Ideas

- A `docs/parity/v1.3-threat-model.md` document should be enough if it includes
  threat register, evidence acceptance criteria, release boundary matrix, and
  traceability sections. Split only if readability suffers.
- Each release-boundary row should state: "v1.3 proves", "evidence", "does not
  claim", "future gate", and "requirements/phases".
- Phase 50 acceptance criteria should explicitly accept a diagnosed blocker when
  live public-network progress cannot be observed, as long as the report
  includes typed cause, endpoint outcomes, status snapshots, and next action.
- Avoid wording like "production ready", "full node replacement", or
  "unattended mainnet node" unless it appears in an explicit non-claim.

</specifics>

<deferred>
## Deferred Ideas

- Offline live-report validation outside `bash scripts/verify.sh` may be useful
  in a future phase if reviewers need a machine-checkable artifact validator.
- Hosted/manual CI public-network evidence remains out of scope unless a future
  release-management phase deliberately accepts flake, cost, privacy, and
  artifact-retention tradeoffs.
- Production-node readiness, inbound serving, transaction relay,
  production-funds wallet behavior, migration apply mode, packaging, hosted
  dashboards, and GUI work remain future milestone scopes.

</deferred>

---

*Phase: 49-threat-model-and-release-boundaries*
*Context gathered: 2026-05-27*
