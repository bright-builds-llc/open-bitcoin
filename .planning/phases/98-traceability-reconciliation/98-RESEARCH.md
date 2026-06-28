# Phase 98: Traceability Reconciliation - Research

**Researched:** 2026-06-28 [VERIFIED: local timestamp]
**Domain:** GSD traceability, parity documentation, deterministic Bun checker, milestone audit reconciliation [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md; scripts/check-phase95-network-participation-release-boundary.ts; scripts/check-phase97-inbound-metrics.ts; scripts/verify.sh]

<user_constraints>
## User Constraints (from CONTEXT.md)

The following constraints are copied from Phase 98 CONTEXT.md. [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md]

### Locked Decisions

#### Canonical Requirement Ownership

- **D-01:** Treat Phase 98 as the canonical closure phase for `INB-01`, `INB-02`, `INB-03`, `INB-04`, and `BOUND-06`.
- **D-02:** Preserve Phase 90 as historical implementation evidence for inbound listener/admission behavior, but do not leave Phase 90 as the canonical owner for `INB-01` through `INB-04` in current traceability tables.
- **D-03:** Preserve Phase 97 as the canonical closure for `INB-05` and `DOS-04`, and Phase 96 as the canonical closure for `EVICT-03`, `EVICT-04`, and `DOS-03`.
- **D-04:** Align all requirement counts to the post-gap-closure target: 28 total v1.9 requirements, 28 mapped, 28 complete, and 0 pending after Phase 98 verification passes.

#### Artifact Reconciliation

- **D-05:** Update `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, and `.planning/STATE.md` so their requirement status, phase tables, current-state narrative, and next-step guidance no longer carry stale Phase 96/97-era pending counts.
- **D-06:** Update `.planning/v1.9-MILESTONE-AUDIT.md` through the Phase 98 work so it no longer reports the pre-Phase-97 21/28 status, missing Phase 97/98 evidence, or stale next action.
- **D-07:** Add explicit cross-phase ownership notes to verification artifacts where historical evidence and canonical ownership now differ. Prefer small, targeted notes over rewriting older summaries wholesale.
- **D-08:** Keep `docs/parity/checklist.md` and `docs/parity/index.json` evidence surfaces intact unless they make canonical ownership claims. Evidence roots may cite historical phase surfaces; ownership tables must use the current canonical map.

#### Deterministic Checker And Verification

- **D-09:** Add a focused Phase 98 Bun checker and fixture test that asserts exact requirement ownership, no stale pending counts, aligned audit status, current verification cross-references, and verifier wiring.
- **D-10:** Wire the Phase 98 checker immediately after the Phase 97 checker in both executable `scripts/verify.sh` order and any visible verifier-order documentation block.
- **D-11:** Use existing Phase 95, Phase 96, Phase 97, and Phase 81 traceability-closure patterns instead of inventing a broad repository scan. The checker should use fixed, explicit files and stable failure messages.
- **D-12:** Final verification for the phase must include `bash scripts/verify.sh`, plus a fresh Phase 98 verification report proving the milestone audit can be rerun without orphaned, partial, or stale requirement-status findings.

#### Folded Todos

No pending todos matched this phase.

### Claude's Discretion

The planner may choose exact checker helper names, fixture construction, and how much historical verification prose to annotate, provided the final state is exact, deterministic, and easy to audit. Prefer localized edits, fixed-file checker inputs, and explicit evidence notes over broad churn in historical phase artifacts.

### Deferred Ideas (OUT OF SCOPE)

Transaction relay, compact block relay, mempool propagation, full address relay, public inbound serving by default, public-network CI, signed packaging, hosted dashboards, GUI work, migration apply mode, production service operation, production-funds wallet operation, automatic support-bundle upload, and production full-node readiness remain outside Phase 98 and outside v1.9.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| INB-01 | Operators can enable inbound peer serving only through explicit config or CLI controls, with inbound serving disabled by default unless a later release boundary says otherwise. [VERIFIED: .planning/REQUIREMENTS.md] | Phase 90 remains the historical implementation evidence, while Phase 98 must mark current canonical traceability complete for `INB-01` and note the historical/canonical split. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md; .planning/phases/98-traceability-reconciliation/98-CONTEXT.md] |
| INB-02 | The daemon can bind and listen on configured interfaces with deterministic preflight and diagnostic errors when disabled, unavailable, unsafe, or already in use. [VERIFIED: .planning/REQUIREMENTS.md] | Phase 90 verification already records deterministic preflight evidence, and Phase 98 must reconcile the active requirement row from Pending to Complete with Phase 98 canonical closure. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md; .planning/REQUIREMENTS.md] |
| INB-03 | The node admits inbound peers through typed connection records, handshake lifecycle state, duplicate/self-connection protections, and inbound/outbound counters. [VERIFIED: .planning/REQUIREMENTS.md] | Phase 90 verification already records typed inbound peer and counter evidence, and Phase 98 must make the audit see current completion rather than missing Phase 98 artifacts. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md; .planning/v1.9-MILESTONE-AUDIT.md] |
| INB-04 | The node enforces configurable inbound connection caps, reserved slots, and protected peer handling without starving the existing outbound sync workflow. [VERIFIED: .planning/REQUIREMENTS.md] | Phase 90 verification already records cap/reserved-slot evidence, and Phase 98 should close only traceability and verification notes without changing runtime admission behavior. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md; .planning/phases/98-traceability-reconciliation/98-CONTEXT.md] |
| BOUND-06 | Requirements, roadmap, phase summaries, verification reports, and milestone audit artifacts map every v1.9 requirement exactly once. [VERIFIED: .planning/REQUIREMENTS.md] | Phase 98 must update planning tables, audit status, selected verification notes, and a new deterministic checker so the final state is 28 total, 28 mapped, 28 complete, and 0 pending. [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md; .planning/ROADMAP.md; scripts/check-phase95-network-participation-release-boundary.ts] |
</phase_requirements>

## Summary

Phase 98 is documentation, planning-state, checker, and audit reconciliation work only; it must not add runtime network behavior or broaden v1.9 network participation claims. [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md] The current repository already contains the historical implementation and verification evidence for Phase 90 inbound listener/admission, Phase 96 peer-policy runtime bridge, and Phase 97 inbound metrics, but active traceability and audit surfaces still report stale pending or missing evidence for the final v1.9 closure. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md; .planning/phases/96-peer-policy-runtime-bridge/96-VERIFICATION.md; .planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md; .planning/v1.9-MILESTONE-AUDIT.md]

The implementation should follow the Phase 81 precedent: preserve historical failed-audit context only where useful, add current closure evidence, use focused exact scans/checkers, and gate completion on `bash scripts/verify.sh`. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-RESEARCH.md; .planning/phases/81-milestone-audit-traceability-closure/81-VERIFICATION.md] For Phase 98, the checker should be narrower than a repository-wide prose scan and stricter than the Phase 95 checker in the areas that now drifted: active planning counts, audit current status, selected cross-phase verification notes, and verifier order. [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md; scripts/check-phase95-network-participation-release-boundary.ts]

**Primary recommendation:** Update the active traceability artifacts first, add `scripts/check-phase98-traceability-reconciliation.ts` plus a Bun fixture test over a curated corpus, wire it after Phase 97 in `scripts/verify.sh`, then write `98-VERIFICATION.md` with focused scan evidence and the final full verifier result. [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md; scripts/check-phase97-inbound-metrics.ts; scripts/verify.sh]

## Project Constraints

- Use `AGENTS.md` as the repo-local instruction entrypoint, then `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant standards pages before research/planning/implementation. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts, and prefer TypeScript for substantial script logic. [VERIFIED: AGENTS.md; standards/languages/typescript-javascript.md]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including the Bazel smoke build. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Treat `docs/metrics/lines-of-code.md` as tracked generated output if verification refreshes it. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Keep functional-core / imperative-shell boundaries, early returns, focused tests, Arrange/Act/Assert comments, and no class inheritance in repo-owned TypeScript behavior. [VERIFIED: standards/core/architecture.md; standards/core/code-shape.md; standards/core/testing.md; standards/languages/typescript-javascript.md]
- No project-local `.cursor/rules/`, `.cursor/skills/`, or `.agents/skills/` indexes were found for this repository. [VERIFIED: Glob .cursor/rules/**; Glob .cursor/skills/**/SKILL.md; Glob .agents/skills/**/SKILL.md]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|---|---:|---|---|
| Bun / TypeScript | Bun 1.3.9 | Implement and run the Phase 98 deterministic checker and fixture test. [VERIFIED: .bun-version; local `bun --version`; AGENTS.md] | Existing Phase 95, Phase 96, and Phase 97 checkers use Bun/TypeScript exported functions, fixed corpora, and mutation fixtures. [VERIFIED: scripts/check-phase95-network-participation-release-boundary.ts; scripts/check-phase96-peer-policy-runtime-bridge.ts; scripts/check-phase97-inbound-metrics.ts] |
| Markdown GSD artifacts | Existing repository format | Update requirements, roadmap, state, audit, selected verification notes, and Phase 98 verification. [VERIFIED: .planning/REQUIREMENTS.md; .planning/ROADMAP.md; .planning/STATE.md; .planning/v1.9-MILESTONE-AUDIT.md] | Phase 81 closed a similar audit traceability gap by updating explicit Markdown artifacts instead of broadening audit logic. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-RESEARCH.md; .planning/phases/81-milestone-audit-traceability-closure/81-VERIFICATION.md] |
| `scripts/verify.sh` | Repo-owned Bash verifier | Final phase gate and checker wiring surface. [VERIFIED: scripts/verify.sh] | Repo guidance names it as the default verification contract, and prior phase checkers are already ordered there. [VERIFIED: AGENTS.md; scripts/verify.sh] |
| Ripgrep focused scans | ripgrep 15.1.0 | Verify stale text removal and exact closure markers during implementation and final verification. [VERIFIED: local `rg --version`] | Phase 81 used exact scans for stale wording and orphan closure without adding broad prose scanners. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-VERIFICATION.md] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|---|---:|---|---|
| Node.js | v24.13.0 | Run `gsd-tools.cjs` for lifecycle/state/roadmap checks if needed. [VERIFIED: local `node --version`; `node "$HOME/.cursor/get-shit-done/bin/gsd-tools.cjs" init phase-op 98`] | Use for GSD lifecycle validation and repo planning-state checks. [VERIFIED: phase-op init output] |
| Bash | 3.2.57 | Execute `scripts/verify.sh` and thin shell orchestration. [VERIFIED: local `bash --version`; scripts/verify.sh] | Use for the repo-native verifier only; keep checker logic in TypeScript. [VERIFIED: AGENTS.md; standards/core/code-shape.md] |
| Cargo / Rust | cargo 1.94.1, rustc 1.94.1 | Existing full verifier surface. [VERIFIED: rust-toolchain.toml; local `cargo --version`; local `rustc --version`; scripts/verify.sh] | Needed because full `bash scripts/verify.sh` runs Cargo format, clippy, build, tests, and coverage. [VERIFIED: scripts/verify.sh] |
| Bazel | 8.6.0 | Existing full verifier smoke build surface. [VERIFIED: local `bazel --version`; scripts/verify.sh] | Needed only through full `bash scripts/verify.sh`. [VERIFIED: AGENTS.md; scripts/verify.sh] |
| cargo-llvm-cov | 0.8.5 | Existing pure-core coverage gate in full verification. [VERIFIED: local `cargo llvm-cov --version`; scripts/verify.sh] | Needed only through full `bash scripts/verify.sh`. [VERIFIED: scripts/verify.sh] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|---|---|---|
| Focused Phase 98 checker | Broad repository-wide prose scanner | Broad scans would be brittle against harmless historical prose; the locked decision requires fixed explicit files and stable failure messages. [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md] |
| Existing planning/audit/parity roots | New traceability manifest | A new manifest would create a parallel evidence source and contradict the existing Phase 95/Phase 81 pattern. [VERIFIED: .planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md; .planning/phases/81-milestone-audit-traceability-closure/81-RESEARCH.md] |
| Targeted verification notes | Rewriting historical summaries wholesale | Historical Phase 90 and Phase 95 evidence should remain auditable; only current ownership notes need to change. [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md] |
| Docs/checker reconciliation | Runtime network changes | Runtime feature work is explicitly out of scope for Phase 98. [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md] |

**Installation:**

No new dependency installation is recommended for Phase 98 because all required local tools are already available. [VERIFIED: local version probes; .bun-version; rust-toolchain.toml]

```bash
bun test scripts/check-phase98-traceability-reconciliation.test.ts
bun run scripts/check-phase98-traceability-reconciliation.ts
bash scripts/verify.sh
```

**Version verification:** No `npm view` checks are needed because Phase 98 should not add npm packages. [VERIFIED: AGENTS.md; .planning/phases/98-traceability-reconciliation/98-CONTEXT.md]

## Exact Current Stale Surfaces

| Surface | Current stale state | Required Phase 98 outcome |
|---|---|---|
| `.planning/REQUIREMENTS.md` | `INB-01`, `INB-02`, `INB-03`, `INB-04`, and `BOUND-06` are mapped to Phase 98 with `Pending`, and coverage still reports 7 pending gap-closure requirements after Phase 96. [VERIFIED: .planning/REQUIREMENTS.md] | Mark those five Phase 98 rows complete after Phase 98 verification, report 28 total / 28 mapped / 28 complete / 0 pending, and refresh the footer. [VERIFIED: 98-CONTEXT.md] |
| `.planning/ROADMAP.md` | Current state says Phase 96 is complete while Phase 97/98 close gaps, still reports 21 complete and 7 pending, and the next-step block still points to `/gsd-plan-phase 96`. [VERIFIED: .planning/ROADMAP.md] | Reflect Phase 97 completion, Phase 98 active/complete as appropriate, 28/28 complete after verification, and a current next step. [VERIFIED: 98-CONTEXT.md; .planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md] |
| `.planning/STATE.md` | The by-phase table still maps Phase 90 to `INB-01` through `INB-05` and Phase 95 to `BOUND-01` through `BOUND-06`, while Phase 98 is only ready to plan. [VERIFIED: .planning/STATE.md] | Preserve Phase 90 and Phase 95 as historical evidence while making current canonical ownership match Phase 96/97/98 gap-closure ownership. [VERIFIED: 98-CONTEXT.md] |
| `.planning/v1.9-MILESTONE-AUDIT.md` | Frontmatter and body still report `requirements: "21/28"`, `status: gaps_found`, Phase 97/98 missing verification, stale Phase 97 next action, and unresolved `INT-03` / `FLOW-03`. [VERIFIED: .planning/v1.9-MILESTONE-AUDIT.md] | Update current audit status/evidence so it no longer reports the pre-Phase-97 state, and include Phase 98 closure evidence for `INT-03-traceability-reconciliation` and `FLOW-03-phase-completion-to-traceability`. [VERIFIED: 98-CONTEXT.md] |
| `docs/parity/release-readiness.md` | The BOUND-06 row says v1.9 requirement traceability stays exactly once across Phase 90 through Phase 95 and references older Plan 02/Plan 04 verifier wording. [VERIFIED: docs/parity/release-readiness.md] | Update the BOUND-06 row to Phase 90 through Phase 98 traceability and Phase 98 checker/verification evidence without changing deferred-scope no-claim boundaries. [VERIFIED: 98-CONTEXT.md] |
| `scripts/check-phase95-network-participation-release-boundary.test.ts` | Fixture text still says requirement traceability is exactly once across Phase 90 through Phase 95 and carries a stale pending count fixture line. [VERIFIED: scripts/check-phase95-network-participation-release-boundary.test.ts] | Update fixture text only as needed so future checker fixtures do not preserve stale current-state claims. [VERIFIED: 98-CONTEXT.md] |
| `scripts/verify.sh` | Visible and executable order stops at Phase 97 before pure-core checks. [VERIFIED: scripts/verify.sh] | Add Phase 98 test/checker commands immediately after Phase 97 in the heredoc and `run_step` order, and update the ordering comment. [VERIFIED: 98-CONTEXT.md] |
| Selected verification reports | Phase 90 still reads as direct evidence for `INB-01` through `INB-05`, Phase 95 says `BOUND-06` is satisfied by Phase 95, and Phase 97 is the fresh canonical evidence for `INB-05`/`DOS-04`. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md; .planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md; .planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md] | Add localized ownership notes: Phase 90 is historical implementation evidence for `INB-01` through `INB-04`; Phase 98 is canonical closure for those and `BOUND-06`; Phase 97 is canonical closure for `INB-05` and `DOS-04`. [VERIFIED: 98-CONTEXT.md] |

## Architecture Patterns

### Recommended Project Structure

```text
.planning/
|-- REQUIREMENTS.md                                      # Active 28/28 requirement status and exact ownership. [VERIFIED: .planning/REQUIREMENTS.md]
|-- ROADMAP.md                                          # Phase 98 current state, success criteria, and traceability table. [VERIFIED: .planning/ROADMAP.md]
|-- STATE.md                                            # Current milestone and phase table narrative. [VERIFIED: .planning/STATE.md]
|-- v1.9-MILESTONE-AUDIT.md                             # Current audit result plus closure evidence. [VERIFIED: .planning/v1.9-MILESTONE-AUDIT.md]
`-- phases/98-traceability-reconciliation/
    |-- 98-01-PLAN.md                                   # Requirements/roadmap/state reconciliation. [VERIFIED: .planning/ROADMAP.md]
    |-- 98-02-PLAN.md                                   # Audit, parity, verification-note, and checker consistency. [VERIFIED: .planning/ROADMAP.md]
    |-- 98-03-PLAN.md                                   # Final verification and audit rerun readiness. [VERIFIED: .planning/ROADMAP.md]
    `-- 98-VERIFICATION.md                              # Final proof with focused scans and full verifier. [VERIFIED: 98-CONTEXT.md]

docs/parity/
`-- release-readiness.md                                # BOUND-06 current traceability row. [VERIFIED: docs/parity/release-readiness.md]

scripts/
|-- check-phase98-traceability-reconciliation.ts         # Fixed-corpus checker. [VERIFIED: 98-CONTEXT.md]
|-- check-phase98-traceability-reconciliation.test.ts    # Bun mutation fixture tests. [VERIFIED: 98-CONTEXT.md]
`-- verify.sh                                           # Verifier wiring after Phase 97. [VERIFIED: scripts/verify.sh]
```

### Pattern 1: Fixed-Corpus Checker

**What:** Read explicit target files, return stable failure strings, and expose an exported check function plus CLI main. [VERIFIED: scripts/check-phase95-network-participation-release-boundary.ts; scripts/check-phase97-inbound-metrics.ts]

**When to use:** Use this for Phase 98 because the decisions require deterministic checks over selected traceability artifacts, not a broad repository scan. [VERIFIED: 98-CONTEXT.md]

**Example:**

```typescript
// Source: scripts/check-phase97-inbound-metrics.ts [VERIFIED]
export function checkPhase97InboundMetrics(
  options: CheckPhase97Options = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  return failures;
}
```

### Pattern 2: Exact Ownership Map Separate From Evidence Roots

**What:** Keep a canonical current ownership map for `REQUIREMENTS.md` and `ROADMAP.md`, while allowing parity evidence rows to cite historical implementation surfaces if they are not canonical ownership tables. [VERIFIED: 98-CONTEXT.md; scripts/check-phase95-network-participation-release-boundary.ts]

**When to use:** Use this to preserve Phase 90 and Phase 95 evidence while making Phase 98 the canonical closure owner for `INB-01` through `INB-04` and `BOUND-06`. [VERIFIED: 98-CONTEXT.md]

**Example current map for Phase 98 checker:**

```typescript
// Source: Phase 98 research synthesis from current CONTEXT and Phase 95 checker [VERIFIED]
const REQUIREMENT_PHASE_ASSIGNMENTS = {
  "INB-01": 98,
  "INB-02": 98,
  "INB-03": 98,
  "INB-04": 98,
  "INB-05": 97,
  "EVICT-03": 96,
  "EVICT-04": 96,
  "DOS-03": 96,
  "DOS-04": 97,
  "BOUND-06": 98,
} as const;
```

### Pattern 3: Mutation Fixtures For Stale Traceability

**What:** Build a complete temporary corpus, mutate one stale condition at a time, and assert the checker fails with the relevant requirement/gap label. [VERIFIED: scripts/check-phase95-network-participation-release-boundary.test.ts; scripts/check-phase97-inbound-metrics.test.ts]

**When to use:** Use this for stale pending counts, stale Phase 90/95 ownership rows, stale audit 21/28 status, missing Phase 98 verifier wiring, and absent verification cross-reference notes. [VERIFIED: 98-CONTEXT.md]

**Example:**

```typescript
// Source: scripts/check-phase95-network-participation-release-boundary.test.ts [VERIFIED]
test("fails when gap-closure traceability maps requirements to stale phases", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        ".planning/REQUIREMENTS.md",
        "| INB-05 | Phase 97 | Pending |",
        "| INB-05 | Phase 90 | Pending |",
      );
    },
  });

  // Act
  const failures = checkPhase95NetworkParticipationReleaseBoundary({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("BOUND-06");
});
```

### Anti-Patterns to Avoid

- **Changing runtime networking behavior:** Phase 98 is docs/checker/audit reconciliation only. [VERIFIED: 98-CONTEXT.md]
- **Reassigning parity evidence roots as if they were canonical ownership tables:** `docs/parity/checklist.md` and `docs/parity/index.json` can remain evidence roots unless they make ownership claims. [VERIFIED: 98-CONTEXT.md]
- **Hiding stale current state inside test fixtures:** Fixture text is checked-in reviewer context and should not keep stale "Phase 90 through Phase 95" or stale pending-count claims after Phase 98. [VERIFIED: scripts/check-phase95-network-participation-release-boundary.test.ts; 98-CONTEXT.md]
- **Erasing historical evidence wholesale:** Phase 90 and Phase 95 reports are useful history and should receive small notes rather than rewrites. [VERIFIED: 98-CONTEXT.md]
- **Completing roadmap/state before verification:** Phase 81 gated completion-state edits on focused checks and `bash scripts/verify.sh`; Phase 98 should follow the same sequencing. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-04-PLAN.md; .planning/phases/81-milestone-audit-traceability-closure/81-VERIFICATION.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---|---|---|---|
| Requirement ownership validation | A new audit inference engine | A fixed TypeScript checker with a literal requirement-phase map and exact target files. [VERIFIED: 98-CONTEXT.md; scripts/check-phase95-network-participation-release-boundary.ts] | The gap is stale artifact reconciliation, not missing runtime evidence or missing parser infrastructure. [VERIFIED: .planning/v1.9-MILESTONE-AUDIT.md] |
| Audit closure evidence | A separate traceability manifest | Update `.planning/v1.9-MILESTONE-AUDIT.md` and `98-VERIFICATION.md`. [VERIFIED: 98-CONTEXT.md; .planning/phases/81-milestone-audit-traceability-closure/81-VERIFICATION.md] | Existing GSD audit and verification artifacts are the planner/archiver surfaces. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-RESEARCH.md] |
| Verifier wiring proof | Manual README-only instructions | `scripts/verify.sh` visible heredoc plus executable `run_step` checks. [VERIFIED: scripts/check-phase95-network-participation-release-boundary.ts; scripts/verify.sh] | Prior checkers already guard against commands appearing only in documentation blocks. [VERIFIED: scripts/check-phase95-network-participation-release-boundary.test.ts] |
| Stale-text detection | Broad all-doc scanners | Curated stale phrase checks over `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md`, `.planning/v1.9-MILESTONE-AUDIT.md`, selected verification reports, `docs/parity/release-readiness.md`, Phase 95 checker fixture, and `scripts/verify.sh`. [VERIFIED: 98-CONTEXT.md; rg stale-surface audit] | Broad scanners risk failing on harmless historical context; fixed files match the locked scope. [VERIFIED: 98-CONTEXT.md] |

**Key insight:** The phase should make current traceability exact without pretending the historical implementation phases did not happen. [VERIFIED: 98-CONTEXT.md; .planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md]

## Common Pitfalls

### Pitfall 1: Marking Requirements Complete Before Phase 98 Verification Exists

**What goes wrong:** `.planning/REQUIREMENTS.md` can show 28 complete while the audit still reports missing Phase 98 verification. [VERIFIED: .planning/REQUIREMENTS.md; .planning/v1.9-MILESTONE-AUDIT.md]

**Why it happens:** Phase 90, Phase 95, Phase 96, and Phase 97 already contain substantial evidence, so it is tempting to update status rows before the canonical Phase 98 report exists. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md; .planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md; .planning/phases/96-peer-policy-runtime-bridge/96-VERIFICATION.md; .planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md]

**How to avoid:** Sequence Plan 98-03 so `98-VERIFICATION.md` is written after the Phase 98 checker and `bash scripts/verify.sh` pass, then update final completion state. [VERIFIED: 98-CONTEXT.md; .planning/phases/81-milestone-audit-traceability-closure/81-04-PLAN.md]

**Warning signs:** The audit still mentions "Phase 98 has no plan, summary, or verification artifact yet" after requirements rows are marked complete. [VERIFIED: .planning/v1.9-MILESTONE-AUDIT.md]

### Pitfall 2: Treating Historical Evidence As Current Canonical Ownership

**What goes wrong:** Phase 90 remains listed as the current owner for `INB-01` through `INB-04`, or Phase 95 remains listed as the current owner for `BOUND-06`. [VERIFIED: .planning/STATE.md; 98-CONTEXT.md]

**Why it happens:** Phase 90 and Phase 95 genuinely produced passed evidence before the later gap-closure phases changed canonical ownership. [VERIFIED: .planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md; .planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md]

**How to avoid:** Use explicit note language such as "historical implementation evidence" for Phase 90 and "historical release-boundary evidence" for Phase 95 while keeping active ownership rows mapped to Phase 98. [VERIFIED: 98-CONTEXT.md]

**Warning signs:** A current traceability table says `| INB-01 | Phase 90 |` or `| BOUND-06 | Phase 95 |` after Phase 98 closes. [VERIFIED: .planning/STATE.md; 98-CONTEXT.md]

### Pitfall 3: Breaking Parity Evidence Exact-Once Checks

**What goes wrong:** Removing `INB-01` through `INB-04` from the Phase 90 parity evidence surface or `BOUND-06` from the Phase 95 parity evidence surface can break the existing Phase 95 parity exact-once expectations. [VERIFIED: docs/parity/index.json; docs/parity/checklist.md; scripts/check-phase95-network-participation-release-boundary.ts]

**Why it happens:** The same requirement IDs appear in parity evidence roots and active planning ownership tables, but Phase 98 decisions distinguish evidence roots from ownership tables. [VERIFIED: 98-CONTEXT.md]

**How to avoid:** Leave `docs/parity/checklist.md` and `docs/parity/index.json` intact unless they make canonical ownership claims, and make the Phase 98 checker read active ownership from planning/audit/verification artifacts. [VERIFIED: 98-CONTEXT.md]

**Warning signs:** `bun run scripts/check-phase95-network-participation-release-boundary.ts` fails after editing parity roots for Phase 98. [VERIFIED: scripts/check-phase95-network-participation-release-boundary.ts]

### Pitfall 4: Forgetting Verifier Order In One Surface

**What goes wrong:** The Phase 98 command is added to the visible `VERIFY_COMMAND_ORDER` block but not executable `run_step`, or vice versa. [VERIFIED: scripts/verify.sh; scripts/check-phase95-network-participation-release-boundary.test.ts]

**Why it happens:** `scripts/verify.sh` intentionally carries visible legacy command lines and executable timed `run_step` calls. [VERIFIED: scripts/verify.sh]

**How to avoid:** Make the Phase 98 checker assert both visible and executable order: Phase 97 test, Phase 97 checker, Phase 98 test, Phase 98 checker, then pure-core checks. [VERIFIED: 98-CONTEXT.md; scripts/check-phase97-inbound-metrics.ts; scripts/verify.sh]

**Warning signs:** `rg -n "check-phase97-inbound-metrics|check-phase98-traceability-reconciliation|check pure-core dependencies" scripts/verify.sh` shows commands out of order. [VERIFIED: scripts/verify.sh]

## Code Examples

### Curated File Reader

```typescript
// Source: scripts/check-phase97-inbound-metrics.ts [VERIFIED]
function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`P97 missing required corpus file: ${relativePath}`);
    return "";
  }
  return readFileSync(absolutePath, "utf8");
}
```

### Ordered Verifier Assertion

```typescript
// Source: scripts/check-phase97-inbound-metrics.ts [VERIFIED]
function requireOrdered(
  text: string,
  needles: readonly string[],
  label: string,
  failures: string[],
): void {
  let cursor = -1;
  for (const needle of needles) {
    const index = text.indexOf(needle);
    if (index === -1) {
      failures.push(`${label} missing ${needle}`);
      continue;
    }
    if (index <= cursor) {
      failures.push(`${label} has ${needle} out of order`);
      continue;
    }
    cursor = index;
  }
}
```

### Phase 98 Suggested Checker Targets

```typescript
// Source: Phase 98 research synthesis from current artifacts [VERIFIED]
const TARGET_FILES = [
  ".planning/REQUIREMENTS.md",
  ".planning/ROADMAP.md",
  ".planning/STATE.md",
  ".planning/v1.9-MILESTONE-AUDIT.md",
  ".planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md",
  ".planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md",
  ".planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md",
  ".planning/phases/98-traceability-reconciliation/98-VERIFICATION.md",
  "docs/parity/release-readiness.md",
  "scripts/check-phase95-network-participation-release-boundary.test.ts",
  "scripts/verify.sh",
] as const;
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|---|---|---|---|
| Phase 90 owned all inbound admission requirements in active tables. [VERIFIED: .planning/STATE.md; docs/parity/checklist.md] | Phase 90 remains historical implementation evidence, while Phase 98 becomes canonical closure for `INB-01` through `INB-04`. [VERIFIED: 98-CONTEXT.md] | Phase 98 discussion on 2026-06-28. [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md] | Active tables must distinguish evidence from canonical ownership. [VERIFIED: 98-CONTEXT.md] |
| Phase 95 owned `BOUND-06` in closeout evidence. [VERIFIED: .planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md] | Phase 98 becomes canonical closure for `BOUND-06`, while Phase 95 remains historical release-boundary evidence for BOUND rows. [VERIFIED: 98-CONTEXT.md] | Phase 98 discussion on 2026-06-28. [VERIFIED: .planning/phases/98-traceability-reconciliation/98-CONTEXT.md] | Verification notes and release-readiness text need targeted clarification. [VERIFIED: 98-CONTEXT.md; docs/parity/release-readiness.md] |
| Phase 97 and Phase 98 were missing in the audit. [VERIFIED: .planning/v1.9-MILESTONE-AUDIT.md] | Phase 97 verification exists and reports `INB-05` / `DOS-04` completion. [VERIFIED: .planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md] | Phase 97 verification on 2026-06-28. [VERIFIED: .planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md] | Audit needs current Phase 97 evidence and final Phase 98 evidence before v1.9 can show 28/28. [VERIFIED: 98-CONTEXT.md] |

**Deprecated/outdated:**

- `21/28`, `7 pending`, and stale `/gsd-plan-phase 96` guidance are outdated after Phase 97 and should not remain in current v1.9 status after Phase 98 verification. [VERIFIED: .planning/ROADMAP.md; .planning/v1.9-MILESTONE-AUDIT.md; 98-CONTEXT.md]
- "Requirement traceability stays exactly once across Phase 90 through Phase 95" is outdated as a current v1.9 closure claim after Phase 98. [VERIFIED: docs/parity/release-readiness.md; scripts/check-phase95-network-participation-release-boundary.test.ts; 98-CONTEXT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|---|---|---|

All claims in this research were verified against local repository artifacts or local tool probes; no `[ASSUMED]` claims are used. [VERIFIED: sources listed below]

## Open Questions (RESOLVED)

1. **Should `INT-04-peer-policy-log-emission-edge` remain a residual audit caveat or block v1.9 archive?** [VERIFIED: .planning/v1.9-MILESTONE-AUDIT.md]
   - What we know: The v1.9 audit names INT-04 as a medium observability edge for automatic `inbound_peer_policy` structured-log emission. [VERIFIED: .planning/v1.9-MILESTONE-AUDIT.md]
   - What's unclear: Phase 98 scope names `INT-03` and `FLOW-03`, not INT-04. [VERIFIED: .planning/ROADMAP.md; 98-CONTEXT.md]
   - Recommendation: Do not implement INT-04 in Phase 98; record it as a residual caveat unless the user explicitly expands scope. [VERIFIED: 98-CONTEXT.md]
   - RESOLVED: `INT-04-peer-policy-log-emission-edge` remains a residual non-blocking audit caveat outside Phase 98 scope. Phase 98 closes `INT-03-traceability-reconciliation` and `FLOW-03-phase-completion-to-traceability` only. [VERIFIED: .planning/ROADMAP.md; 98-CONTEXT.md]

2. **Should the audit artifact preserve its original failed snapshot or become a fully current report?** [VERIFIED: .planning/v1.9-MILESTONE-AUDIT.md; .planning/phases/81-milestone-audit-traceability-closure/81-VERIFICATION.md]
   - What we know: Phase 98 decision D-06 says the audit should no longer report pre-Phase-97 21/28 status or missing Phase 97/98 evidence. [VERIFIED: 98-CONTEXT.md]
   - What's unclear: The exact formatting can be a full rewrite or an appended closure rerun plus updated frontmatter. [VERIFIED: 98-CONTEXT.md]
   - Recommendation: Update frontmatter/current result to the new 28/28 state and preserve the old gap evidence only as historical context if needed. [VERIFIED: 98-CONTEXT.md; .planning/phases/81-milestone-audit-traceability-closure/81-VERIFICATION.md]
   - RESOLVED: The audit artifact should become a current Phase 98 closeout report showing 28/28 requirement coverage after verification, while preserving historical failed-gap context only where it remains useful for explaining what Phase 98 closed. [VERIFIED: 98-CONTEXT.md; .planning/phases/81-milestone-audit-traceability-closure/81-VERIFICATION.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|---|---|---:|---|---|
| Bun | Phase 98 checker and tests | yes | 1.3.9 | None needed. [VERIFIED: local `bun --version`; .bun-version] |
| Node.js | GSD tools | yes | v24.13.0 | None needed. [VERIFIED: local `node --version`] |
| Bash | `scripts/verify.sh` | yes | 3.2.57 | None needed. [VERIFIED: local `bash --version`] |
| ripgrep | Focused stale-surface scans | yes | 15.1.0 | Use TypeScript checker assertions if shell scans are unavailable. [VERIFIED: local `rg --version`] |
| Cargo | Full verifier | yes | 1.94.1 | None for final verification. [VERIFIED: local `cargo --version`; rust-toolchain.toml] |
| rustc | Full verifier | yes | 1.94.1 | None for final verification. [VERIFIED: local `rustc --version`; rust-toolchain.toml] |
| Bazel | Full verifier smoke build | yes | 8.6.0 | None for final verification. [VERIFIED: local `bazel --version`; scripts/verify.sh] |
| cargo-llvm-cov | Full verifier coverage gate | yes | 0.8.5 | None for full verification. [VERIFIED: local `cargo llvm-cov --version`; scripts/verify.sh] |

**Missing dependencies with no fallback:** None found for Phase 98 planning. [VERIFIED: local version probes]

**Missing dependencies with fallback:** None found for Phase 98 planning. [VERIFIED: local version probes]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---|---|---|
| V2 Authentication | no | No authentication surface changes are in Phase 98. [VERIFIED: 98-CONTEXT.md] |
| V3 Session Management | no | No session-management surface changes are in Phase 98. [VERIFIED: 98-CONTEXT.md] |
| V4 Access Control | no | No runtime access-control surface changes are in Phase 98. [VERIFIED: 98-CONTEXT.md] |
| V5 Input Validation | yes | Parse JSON roots with `JSON.parse`, use exact Markdown row/string checks, and fail closed with stable errors. [VERIFIED: scripts/check-phase95-network-participation-release-boundary.ts] |
| V6 Cryptography | no | No cryptographic behavior changes are in Phase 98. [VERIFIED: 98-CONTEXT.md] |

### Known Threat Patterns for Traceability Reconciliation

| Pattern | STRIDE | Standard Mitigation |
|---|---|---|
| Stale requirement ownership appears complete but maps to the wrong phase. [VERIFIED: .planning/STATE.md; .planning/v1.9-MILESTONE-AUDIT.md] | Tampering / Repudiation | Fixed canonical ownership map in the Phase 98 checker and exact rows in `REQUIREMENTS.md` / `ROADMAP.md`. [VERIFIED: 98-CONTEXT.md] |
| Historical evidence is erased, making prior verification unauditable. [VERIFIED: 98-CONTEXT.md] | Repudiation | Add small historical/canonical notes instead of rewriting older summaries wholesale. [VERIFIED: 98-CONTEXT.md] |
| Broad scanners fail on harmless archived prose. [VERIFIED: 98-CONTEXT.md] | Denial of Service | Limit scanner/checker inputs to curated files and current-state phrases. [VERIFIED: 98-CONTEXT.md] |
| Verification command is documented but not executed. [VERIFIED: scripts/check-phase95-network-participation-release-boundary.test.ts; scripts/verify.sh] | Spoofing / Repudiation | Assert both `VERIFY_COMMAND_ORDER` and executable `run_step` order. [VERIFIED: scripts/check-phase95-network-participation-release-boundary.ts] |
| Docs/checker work accidentally expands v1.9 claims. [VERIFIED: 98-CONTEXT.md] | Elevation of Privilege | Preserve no-claim language for relay, public defaults, production service operation, and production full-node readiness. [VERIFIED: 98-CONTEXT.md; docs/parity/release-readiness.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/98-traceability-reconciliation/98-CONTEXT.md` - locked decisions, phase boundary, canonical references, and checker requirements. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - active v1.9 requirements, pending rows, and traceability coverage. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 98 description, success criteria, stale current-state counts, and traceability table. [VERIFIED: file read]
- `.planning/STATE.md` - current milestone status and stale by-phase ownership table. [VERIFIED: file read]
- `.planning/v1.9-MILESTONE-AUDIT.md` - gap evidence for `INT-03` and `FLOW-03`, stale audit status, and Phase 97/98 missing evidence. [VERIFIED: file read]
- `scripts/check-phase95-network-participation-release-boundary.ts` and `.test.ts` - existing exact ownership map, fixed-corpus checker style, and mutation fixture patterns. [VERIFIED: file read]
- `scripts/check-phase96-peer-policy-runtime-bridge.ts` and `scripts/check-phase97-inbound-metrics.ts` - latest checker style and verifier-order patterns. [VERIFIED: file read]
- `scripts/verify.sh` - visible and executable verification order and full repo-native verification contract. [VERIFIED: file read]
- `.planning/phases/81-milestone-audit-traceability-closure/81-RESEARCH.md` and `81-VERIFICATION.md` - closest milestone-audit traceability closure precedent. [VERIFIED: file read]
- `.planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md`, `.planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md`, `.planning/phases/96-peer-policy-runtime-bridge/96-VERIFICATION.md`, and `.planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md` - cross-phase evidence and ownership-note targets. [VERIFIED: file read]

### Secondary (MEDIUM confidence)

- `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, `docs/parity/index.json`, and `docs/parity/catalog/p2p.md` - parity evidence roots and stale/current no-claim language. [VERIFIED: file reads and focused rg]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/languages/typescript-javascript.md`, and `standards/languages/rust.md` - workflow, verification, and script/testing constraints. [VERIFIED: file read]

### Tertiary (LOW confidence)

- None. No web or unverified third-party research was needed because this phase is codebase-internal reconciliation. [VERIFIED: research scope and local artifacts]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - all tools are repo-standard and locally available. [VERIFIED: AGENTS.md; local version probes]
- Architecture: HIGH - locked decisions and existing checker patterns define the implementation shape. [VERIFIED: 98-CONTEXT.md; scripts/check-phase95-network-participation-release-boundary.ts; scripts/check-phase97-inbound-metrics.ts]
- Pitfalls: HIGH - each pitfall is visible in current stale artifacts or prior Phase 81/95 checker behavior. [VERIFIED: rg stale-surface audit; .planning/phases/81-milestone-audit-traceability-closure/81-VERIFICATION.md]

**Research date:** 2026-06-28 [VERIFIED: local timestamp]
**Valid until:** 2026-07-05, because this is active milestone state and can change as soon as Phase 98 planning/execution starts. [VERIFIED: .planning/STATE.md]
