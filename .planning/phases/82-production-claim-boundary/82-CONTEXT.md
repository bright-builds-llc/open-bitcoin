---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 82-2026-06-21T12-38-13
generated_at: 2026-06-21T12:38:38.140Z
---

# Phase 82: Production Claim Boundary - Context

**Gathered:** 2026-06-21
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 82 defines the production full-node readiness claim boundary before the
project makes any production-readiness statement. It should give operators,
contributors, and release reviewers one vocabulary for supported, preview,
opt-in UAT, unsupported, and deferred surfaces; one evidence-gate model for
allowed production-related statements; and one deferred-surface inventory that
future phases can reuse.

This phase should not claim production full-node readiness, add inbound serving
or relay support, make production-funds wallet workflows supported, enable
migration apply mode, add signed packaging, make public-network checks part of
default verification, or implement the Phase 88 deterministic guardrail suite.
</domain>

<decisions>
## Implementation Decisions

### Production Vocabulary

- **D-01:** Define exactly five support terms for v1.8 docs and release
  language: `supported`, `preview`, `opt-in UAT`, `unsupported`, and
  `deferred`. Avoid alternate near-synonyms such as production-ish,
  production-grade, beta-supported, or ready enough.
- **D-02:** Treat `supported` as evidence-backed source-built behavior that
  default verification and documented UAT can substantiate today. Treat
  `preview` as shipped but not support-committed. Treat `opt-in UAT` as
  explicit operator-run evidence outside default verification. Treat
  `unsupported` as available only for local experimentation or historical
  compatibility without support expectation. Treat `deferred` as not shipped or
  not safe to rely on until a future milestone names gates and evidence.
- **D-03:** State that v1.8 is a boundary-setting milestone, not the production
  readiness milestone. The allowed near-term claim is that Open Bitcoin defines
  gates required before any future production full-node readiness claim.
- **D-04:** Keep the language quiet and operator-facing. This is a release
  control surface, not marketing copy.

### Evidence Gate Model

- **D-05:** Add a claim-to-evidence matrix that maps each allowed
  production-related statement to a required support term, current status,
  evidence sources, verification command, UAT status, residual risk, and next
  required gate.
- **D-06:** A statement is allowed only when the matrix names concrete evidence.
  Evidence can be deterministic verification, passed phase verification,
  parity roots, operator docs, support bundles, opt-in UAT artifacts, or
  milestone audit artifacts. Artifact existence by itself is not enough.
- **D-07:** Include explicit "not allowed yet" rows for production full-node
  readiness, production service operation, relay/inbound serving, production
  wallet use, migration apply mode, signed distribution, hosted dashboards,
  public-network CI, destructive repair, and automatic support upload.
- **D-08:** Keep Phase 82 evidence gates readable in docs first. A
  machine-readable parity/index entry is useful for traceability, but the broad
  default-verification blocker belongs to Phase 88 unless planning finds a
  narrow local check needed to keep Phase 82 internally consistent.

### Deferred Surface Inventory

- **D-09:** Preserve the exact deferred production-adjacent inventory from
  v1.8 requirements and previous release-boundary docs: inbound serving,
  address relay, block serving, transaction relay, compact block relay,
  production-funds wallet use, production-funds wallet safety, migration apply
  mode, signed packaging or package-manager distribution, Windows service
  integration, hosted dashboards, GUI parity, public-network default checks,
  public-network CI, release-blocking live sync, automatic support-bundle
  upload, destructive repair, and broad production-node readiness.
- **D-10:** For each deferred surface, record why it is deferred and which
  future evidence gate would be needed before the support term can change.
- **D-11:** Keep historical v1.3 through v1.7 scoped claims discoverable as
  evidence, but do not rewrite them into current production support. Those
  milestones remain source-built, opt-in evidence surfaces.

### Documentation Shape

- **D-12:** Prefer one canonical production boundary document under
  `docs/parity/`, linked from README, runtime guide, release readiness,
  checklist, parity README, parity index, and deviations register.
- **D-13:** Update `docs/parity/release-readiness.md` with a v1.8 production
  claim boundary section rather than replacing the v1.7 release-readiness
  history.
- **D-14:** Update README only enough to point readers to the v1.8 boundary and
  avoid stale v1.7-as-current wording. Do not duplicate the full matrix there.
- **D-15:** Keep `docs/parity/deviations-and-unknowns.md` as the durable
  deferred-surface register. Phase 82 may add a v1.8 section that names the
  support level and required future gate for each deferred surface.

### Verification And Traceability

- **D-16:** Run and cite `bash scripts/verify.sh` for phase closeout. If the
  implementation is docs-only plus parity JSON, focused Markdown/JSON scans may
  be used during iteration, but the final verification still uses the repo
  contract.
- **D-17:** If Phase 82 adds Bun/TypeScript automation, follow the existing
  checker/test pattern from Phase 80 and keep it deterministic,
  public-network-free, service-manager-free, timing-stable, and short-running.
- **D-18:** No Rust source changes are expected. If planning discovers a narrow
  Rust gap, update parity breadcrumbs for new first-party Rust source or test
  files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`.
- **D-19:** UAT examples in any operator-facing docs must use repo-local Cargo
  and Bazel command forms, not the installed `open-bitcoin` alias alone.

### Folded Todos

No pending todos matched Phase 82.

### the agent's Discretion

- The planner may split work into boundary vocabulary, evidence-gate matrix,
  deferred-surface registry, parity/root link updates, README/runtime-guide
  pointer refresh, and verification closeout.
- The executor may add a small machine-readable parity root or local checker if
  it materially improves Phase 82 traceability without duplicating Phase 88.
- The executor may keep Phase 82 primarily in documentation and parity metadata
  if no source behavior or deterministic guardrail gap is required.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 82 goal, success criteria, and dependencies.
- `.planning/REQUIREMENTS.md` - PROD-01 through PROD-04, future production
  capability requirements, and v1.8 out-of-scope table.
- `.planning/PROJECT.md` - active v1.8 boundary-setting goal, current project
  posture, core value, and production-claim constraints.
- `.planning/STATE.md` - current milestone state and accumulated v1.8
  decisions.
- `AGENTS.md` - repo-local verification, UAT command, parity breadcrumb, GSD,
  generated artifact, and workflow rules.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - local standards override registry.
- `standards/core/architecture.md` - functional-core and illegal-state rules.
- `standards/core/code-shape.md` - code shape, script, and naming rules.
- `standards/core/verification.md` - sync-first and repo-native verification
  requirements.
- `standards/core/testing.md` - unit-test and Arrange/Act/Assert expectations.
- `standards/languages/rust.md` - Rust module, option naming, invariant, and
  verification guidance.
- `standards/languages/typescript-javascript.md` - Bun/TS automation guidance.

### Prior Release Boundary Decisions

- `.planning/phases/67-release-boundaries-and-deterministic-verification/67-CONTEXT.md`
  - v1.5 release-boundary checker posture, deferred-surface wording, and
  default-verification exclusions.
- `.planning/phases/74-release-boundaries-parity-and-documentation/74-CONTEXT.md`
  - v1.6 release-claim shape, non-claim list, release-readiness matrix, and
  deterministic checker posture.
- `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-CONTEXT.md`
  - v1.7 opt-in UAT matrix, non-claim list, release-boundary wording, and
  checker/test pattern.
- `.planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md`
  - audit-traceability repair posture and narrow planning-state refresh
  decisions.
- `.planning/milestones/v1.7-MILESTONE-AUDIT.md` - passed v1.7 audit and final
  traceability context.

### Current Docs And Checkers

- `README.md` - contributor/operator entrypoint requiring current v1.8 boundary
  pointer and no stale production-readiness wording.
- `docs/operator/runtime-guide.md` - operator command surface, known
  limitations, opt-in UAT posture, and support-bundle boundaries.
- `docs/parity/release-readiness.md` - current v1.7 release-readiness handoff
  to extend with a v1.8 production claim boundary section.
- `docs/parity/deviations-and-unknowns.md` - current deferred-surface register
  to extend with v1.8 support terms and future gates.
- `docs/parity/index.json` - machine-readable parity root for discoverability.
- `docs/parity/checklist.md` - human-readable parity checklist root.
- `docs/parity/README.md` - parity entrypoint requiring a v1.8 boundary link.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - accumulated
  operator-runtime and release-hardening evidence catalog.
- `docs/parity/catalog/p2p.md` - current P2P release boundaries and deferred
  inbound/relay surfaces.
- `docs/parity/catalog/chainstate.md` - current chainstate release boundaries
  and production-node non-claims.
- `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` and
  `scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts` - nearest
  deterministic release-boundary checker and fixture-test pattern.
- `scripts/check-v1.6-release-boundaries.ts` - earlier milestone
  release-boundary checker pattern.
- `scripts/verify.sh` - repo-native verification contract.

### External Orientation

- `https://sre.google/sre-book/evolving-sre-engagement-model/` - Google SRE
  production-readiness review framing: production responsibility follows
  explicit readiness review and support acceptance.
- `https://contribute.cncf.io/projects/lifecycle/` - CNCF lifecycle framing for
  maturity levels and adoption expectations.
- `https://github.com/bitcoin/bitcoin/blob/master/doc/release-process.md` -
  Bitcoin Core release-process reference for release-note and release-evidence
  discipline.
- `https://bitcoincore.org/en/lifecycle/` - Bitcoin Core software lifecycle
  reference for maintenance/support terminology.
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `docs/parity/release-readiness.md` already contains scoped historical
  readiness verdicts and claim-boundary matrices through v1.7.
- `docs/parity/deviations-and-unknowns.md` already preserves deferred
  production-adjacent surfaces and is the natural durable register.
- `docs/parity/index.json`, `docs/parity/checklist.md`, and
  `docs/parity/README.md` are the existing machine-readable and human parity
  roots for release reviewers.
- `docs/operator/runtime-guide.md` already documents source-built operator
  workflows, known limitations, support bundles, and opt-in UAT commands.
- `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` demonstrates
  the current Bun checker pattern for release-boundary evidence, required
  strings, forbidden default-verification paths, and parity roots.

### Established Patterns

- Release-boundary phases add current milestone sections while preserving
  historical v1.3 through v1.7 evidence.
- Default verification remains deterministic, local, public-network-free,
  real-service-manager-free, timing-stable, and short-running.
- Operator-facing UAT commands use explicit repo-local Cargo and Bazel forms.
- Broad production-node, inbound serving, relay, production-funds wallet,
  migration apply, packaging, GUI, hosted-dashboard, public-network CI,
  destructive repair, and automatic support-upload claims remain non-claims
  until a future milestone changes their evidence gates.
- Evidence language is field- and artifact-specific. Daemon startup, elapsed
  time, peer reachability, report existence, or bundle existence alone do not
  prove readiness.

### Integration Points

- Add or update a canonical v1.8 production boundary doc under `docs/parity/`.
- Link the boundary from README, runtime guide, release readiness, checklist,
  parity README, parity index, deviations register, and relevant catalog pages.
- If a Phase 82 checker is added, wire it near existing release-boundary checks
  in `scripts/verify.sh` without running public-network or service-manager work.
- Update `.planning` artifacts through GSD tooling where a safe tool exists;
  avoid broad manual state rewrites.
</code_context>

<specifics>
## Specific Ideas

- Use a stable surface id such as `v1-8-production-claim-boundary` for parity
  roots if a new checklist/index entry is added.
- Use short status labels in matrices: `allowed`, `allowed-with-opt-in-uat`,
  `preview-only`, `unsupported`, and `deferred`.
- Keep "production full-node readiness" as a gated future claim with a visible
  no-claim row until all required gates pass in a later milestone.
- Prefer one support-term glossary plus one evidence-gate matrix over scattered
  prose paragraphs.
</specifics>

<deferred>
## Deferred Ideas

- Phase 88 deterministic broad-claim scanner and default-verification guardrail
  suite.
- Future production full-node readiness claim after all gates pass.
- Inbound serving, address relay, block serving, transaction relay, compact
  block relay, production-funds wallet use or safety, migration apply mode,
  signed packaging or package-manager distribution, Windows service integration,
  hosted dashboards, GUI parity, public-network CI, release-blocking live sync,
  destructive repair, and automatic support-bundle upload.
</deferred>

---

*Phase: 82-production-claim-boundary*
*Context gathered: 2026-06-21*
