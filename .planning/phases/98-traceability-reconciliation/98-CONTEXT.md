---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 98-2026-06-28T19-19-22
generated_at: 2026-06-28T19:20:44.567Z
---

# Phase 98: Traceability Reconciliation - Context

**Gathered:** 2026-06-28
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 98 reconciles v1.9 requirements, roadmap, phase summaries, verification reports, deterministic checker evidence, and milestone audit artifacts after the Phase 96 and Phase 97 gap-closure work. It closes the remaining stale traceability gaps so v1.9 can show exact, consistent 28/28 requirement coverage across planning, verification, and audit surfaces.

This phase is documentation, planning-state, checker, and audit reconciliation work. It must not add or expand runtime network behavior, inbound public defaults, transaction relay, compact block relay, mempool propagation, broad address relay, production-service claims, or production full-node readiness.

</domain>

<decisions>
## Implementation Decisions

### Canonical Requirement Ownership

- **D-01:** Treat Phase 98 as the canonical closure phase for `INB-01`, `INB-02`, `INB-03`, `INB-04`, and `BOUND-06`.
- **D-02:** Preserve Phase 90 as historical implementation evidence for inbound listener/admission behavior, but do not leave Phase 90 as the canonical owner for `INB-01` through `INB-04` in current traceability tables.
- **D-03:** Preserve Phase 97 as the canonical closure for `INB-05` and `DOS-04`, and Phase 96 as the canonical closure for `EVICT-03`, `EVICT-04`, and `DOS-03`.
- **D-04:** Align all requirement counts to the post-gap-closure target: 28 total v1.9 requirements, 28 mapped, 28 complete, and 0 pending after Phase 98 verification passes.

### Artifact Reconciliation

- **D-05:** Update `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, and `.planning/STATE.md` so their requirement status, phase tables, current-state narrative, and next-step guidance no longer carry stale Phase 96/97-era pending counts.
- **D-06:** Update `.planning/v1.9-MILESTONE-AUDIT.md` through the Phase 98 work so it no longer reports the pre-Phase-97 21/28 status, missing Phase 97/98 evidence, or stale next action.
- **D-07:** Add explicit cross-phase ownership notes to verification artifacts where historical evidence and canonical ownership now differ. Prefer small, targeted notes over rewriting older summaries wholesale.
- **D-08:** Keep `docs/parity/checklist.md` and `docs/parity/index.json` evidence surfaces intact unless they make canonical ownership claims. Evidence roots may cite historical phase surfaces; ownership tables must use the current canonical map.

### Deterministic Checker And Verification

- **D-09:** Add a focused Phase 98 Bun checker and fixture test that asserts exact requirement ownership, no stale pending counts, aligned audit status, current verification cross-references, and verifier wiring.
- **D-10:** Wire the Phase 98 checker immediately after the Phase 97 checker in both executable `scripts/verify.sh` order and any visible verifier-order documentation block.
- **D-11:** Use existing Phase 95, Phase 96, Phase 97, and Phase 81 traceability-closure patterns instead of inventing a broad repository scan. The checker should use fixed, explicit files and stable failure messages.
- **D-12:** Final verification for the phase must include `bash scripts/verify.sh`, plus a fresh Phase 98 verification report proving the milestone audit can be rerun without orphaned, partial, or stale requirement-status findings.

### Folded Todos

No pending todos matched this phase.

### the agent's Discretion

The planner may choose exact checker helper names, fixture construction, and how much historical verification prose to annotate, provided the final state is exact, deterministic, and easy to audit. Prefer localized edits, fixed-file checker inputs, and explicit evidence notes over broad churn in historical phase artifacts.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Workflow Rules

- `AGENTS.md` - repo-local GSD workflow, verification, parity breadcrumb, generated artifact, and UAT command rules.
- `AGENTS.bright-builds.md` - Bright Builds workflow, architecture, testing, code-shape, and verification expectations.
- `standards/core/architecture.md` - functional core / imperative shell and domain-type defaults.
- `standards/core/code-shape.md` - readable control flow and script shape guidance.
- `standards/core/testing.md` - focused unit tests and Arrange/Act/Assert expectations.
- `standards/core/verification.md` - repo-native verification and clean commit gate rules.
- `standards/languages/typescript-javascript.md` - Bun-backed TypeScript checker guidance.
- `standards/languages/rust.md` - Rust guidance if any verification notes touch Rust source or tests.

### Phase Scope And Gap Evidence

- `.planning/ROADMAP.md` - Phase 98 goal, requirements, gap-closure IDs, plan outline, and success criteria.
- `.planning/REQUIREMENTS.md` - v1.9 requirement status, traceability table, and `BOUND-06` exact-once traceability requirement.
- `.planning/STATE.md` - current milestone position, current phase, and carry-forward workflow constraints.
- `.planning/v1.9-MILESTONE-AUDIT.md` - stale audit findings for `INT-03-traceability-reconciliation` and `FLOW-03-phase-completion-to-traceability` that Phase 98 must close.

### Prior v1.9 Context And Evidence

- `.planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md` - original inbound listener/admission implementation boundary and historical evidence roots for `INB-01` through `INB-04`.
- `.planning/phases/95-network-participation-evidence-and-release-boundary/95-CONTEXT.md` - release-boundary and traceability checker decisions, including the expected exact ownership map.
- `.planning/phases/96-peer-policy-runtime-bridge/96-CONTEXT.md` - canonical closure for peer-policy runtime bridge requirements and duplicate ownership guardrails.
- `.planning/phases/97-inbound-metrics-sample-production/97-CONTEXT.md` - canonical closure for retained inbound metrics sample production.
- `.planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md` - historical Phase 90 verification evidence that needs a canonical-ownership note.
- `.planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md` - Phase 95 release-boundary verification that needs `BOUND-06` ownership alignment.
- `.planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md` - latest completed gap-closure verification evidence.

### Checker And Documentation Patterns

- `scripts/check-phase95-network-participation-release-boundary.ts` - existing target ownership map and traceability checker pattern.
- `scripts/check-phase95-network-participation-release-boundary.test.ts` - stale traceability fixture and mutation-test precedent.
- `scripts/check-phase96-peer-policy-runtime-bridge.ts` - duplicate canonical ownership guard pattern.
- `scripts/check-phase97-inbound-metrics.ts` - latest phase checker structure and public-network-free verification precedent.
- `scripts/check-phase97-inbound-metrics.test.ts` - latest Bun fixture style.
- `scripts/verify.sh` - repo-native verification order and checker wiring contract.
- `docs/parity/release-readiness.md` - BOUND-06 release-readiness row and exact traceability language.
- `docs/parity/checklist.md` - parity evidence checklist that may remain an evidence surface.
- `docs/parity/index.json` - machine-readable parity evidence root that may remain an evidence surface.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `scripts/check-phase95-network-participation-release-boundary.ts` already encodes the intended post-gap-closure ownership map for v1.9 requirements.
- Phase 95 through Phase 97 checker/test files already show the preferred Bun script pattern: fixed file inputs, explicit assertions, mutation tests, and verifier-order checks.
- Phase 81 milestone audit traceability closure is the planning precedent for backfilling verification reports, refreshing requirements and roadmap artifacts, rerunning audit, and writing a final verification report.

### Established Patterns

- Traceability checkers should be deterministic, public-network-free, and based on curated file sets rather than repository-wide prose scans.
- Historical phase summaries and verification reports can remain evidence records when annotated clearly; current ownership tables should carry the canonical post-gap-closure map.
- Default verification remains `bash scripts/verify.sh`, with Bun checkers wired as repo-owned script steps.
- Generated metrics documentation such as `docs/metrics/lines-of-code.md` may refresh during verification and should be treated as intentional tracked output.

### Integration Points

- Add `scripts/check-phase98-traceability-reconciliation.ts` and a paired `.test.ts` fixture.
- Wire the Phase 98 checker after Phase 97 in `scripts/verify.sh`.
- Update `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md`, `.planning/v1.9-MILESTONE-AUDIT.md`, selected phase verification reports, and `docs/parity/release-readiness.md` to align with the exact 28/28 map.
- Create Phase 98 plan, summary, and verification artifacts under `.planning/phases/98-traceability-reconciliation/`.

</code_context>

<specifics>
## Specific Ideas

- Treat stale Phase 90 ownership as an evidence-vs-canonical-owner problem, not as a reason to erase Phase 90 history.
- Align the roadmap gap ID wording with the milestone audit naming so `INT-03-phase-90-requirements-status` and `INT-03-traceability-reconciliation` do not appear to be separate unresolved gaps.
- Keep the Phase 98 checker narrow enough that future harmless historical prose does not fail verification, but strict enough to catch stale pending counts, stale next steps, and stale "Phase 90 through Phase 95 only" exact-traceability claims.
- Use the final Phase 98 verification report as the human-readable audit bridge showing requirements, roadmap, summaries, verification, and milestone audit artifacts agree.

</specifics>

<deferred>
## Deferred Ideas

Transaction relay, compact block relay, mempool propagation, full address relay, public inbound serving by default, public-network CI, signed packaging, hosted dashboards, GUI work, migration apply mode, production service operation, production-funds wallet operation, automatic support-bundle upload, and production full-node readiness remain outside Phase 98 and outside v1.9.

</deferred>

---

*Phase: 98-traceability-reconciliation*
*Context gathered: 2026-06-28*
