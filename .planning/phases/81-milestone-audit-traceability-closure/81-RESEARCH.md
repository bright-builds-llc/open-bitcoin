# Phase 81: Milestone Audit Traceability Closure - Research

**Researched:** 2026-06-19
**Domain:** Repo-native milestone traceability, verification artifact backfill, deterministic audit closure
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

Source for all bullets in this section: [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md]

### Locked Decisions

### Verification Artifact Repair

- **D-01:** Treat the audit failure as a traceability defect. Phase 76 and
  Phase 77 already have passed implementation, summaries, parity roots,
  deterministic checkers, and full verification evidence; Phase 81 should not
  reopen runtime behavior unless planning finds a concrete evidence mismatch.
- **D-02:** Add explicit `requirements:` frontmatter and a requirements coverage
  section to `76-VERIFICATION.md` for RES-05, RES-06, RES-07, and RES-08,
  anchored to the existing Phase 76 evidence and summaries.
- **D-03:** Add explicit `requirements:` frontmatter and a requirements coverage
  section to `77-VERIFICATION.md` for REC-05, REC-06, REC-07, and REC-08,
  anchored to the existing Phase 77 evidence and summaries.
- **D-04:** Do not edit milestone audit tooling to broaden the audit contract.
  The safer fix is to make the artifacts satisfy the existing strict audit
  expectation rather than teaching the audit to infer verification coverage
  from plans or summaries.

### Milestone Traceability Refresh

- **D-05:** Refresh `.planning/REQUIREMENTS.md` only after the verification
  artifacts explicitly name their requirement IDs. Mark completed Phase 75,
  Phase 76, Phase 77, Phase 79, and Phase 80 requirement rows according to
  their passed verification evidence; avoid premature completion wording before
  Phase 81 evidence exists.
- **D-06:** Keep `.planning/ROADMAP.md` focused on Phase 81 as the active
  gap-closure phase until its verification passes. After execution, the roadmap
  may mark Phase 81 complete and move the v1.7 milestone state out of active
  only if the audit rerun passes.
- **D-07:** Refresh `.planning/PROJECT.md` and `.planning/STATE.md` through the
  narrowest safe path so they no longer state that v1.7 completed after Phase
  80 while Phase 81 is active. Use GSD state tooling for `STATE.md` where a
  repo-owned command exists; avoid direct state mutation when tooling can record
  the session or phase transition.
- **D-08:** Preserve the v1.7 audit artifact as evidence of the gap that Phase
  81 closes. If a fresh audit report is produced, keep the old failed audit
  understandable and add or update the current audit result without erasing the
  historical reason for this phase.

### Release-Boundary Wording Cleanup

- **D-09:** Fix the stale `planned Phase 80 checker` wording in
  `docs/parity/catalog/operator-runtime-release-hardening.md` so it references
  the implemented and wired Phase 80 checker as evidence.
- **D-10:** Start with a targeted parity catalog wording refresh. Only expand
  into a broader doc sweep if searches find more stale Phase 80 current-state
  wording that would affect the v1.7 release boundary.
- **D-11:** Keep release-boundary wording tied to explicit opt-in soak and
  recovery hardening. Do not introduce or imply broad production-node,
  public-network default, release-blocking live sync, or evidence-manifest
  claims.

### Verification And Audit Closure

- **D-12:** Rerun the deterministic audit checks that previously identified the
  orphaned RES/REC IDs, then rerun `bash scripts/verify.sh` before the final
  Phase 81 verification report.
- **D-13:** Phase 81 verification should explicitly prove that RES-05 through
  RES-08 and REC-05 through REC-08 are no longer orphaned, that stale Phase 80
  wording is gone, and that the updated planning state matches the evidence.
- **D-14:** If only planning/docs/checker artifacts change, keep verification
  narrow before the full verifier: use `rg` scans for requirement IDs and stale
  wording, `gsd-tools roadmap analyze`, lifecycle validation, and focused Bun
  checkers where relevant.

### Folded Todos

No pending todos matched Phase 81.

### the agent's Discretion

- The planner may split Phase 81 into verification artifact backfill,
  milestone traceability refresh, release-boundary wording cleanup, and audit
  rerun/verification closeout.
- The executor may create a small Phase 81 deterministic checker only if it
  materially prevents the same stale traceability regression from returning.
  Do not add a brittle broad prose scanner if targeted artifact updates plus
  audit rerun are sufficient.
- The executor may update project and state narrative text narrowly, but should
  avoid noisy regeneration of historical milestone prose.

### Deferred Ideas (OUT OF SCOPE)

None - discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| RES-05 | Operator can see disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle bounds before starting a long soak. [VERIFIED: .planning/REQUIREMENTS.md] | Backfill Phase 76 verification coverage from Phase 76 summaries and checker anchors for `resource_bounds`, the full resource-kind set, status/dashboard, docs, and parity roots. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/*SUMMARY.md; scripts/check-phase76-resource-bounds.ts] |
| RES-06 | Operator can receive typed low-disk, disk-growth, compaction, log-retention, metrics-retention, and support-bundle size guidance during and after a soak. [VERIFIED: .planning/REQUIREMENTS.md] | Backfill evidence to the Phase 76 threshold and support-bound guidance already verified by the checker and full verifier. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-VERIFICATION.md; scripts/check-phase76-resource-bounds.ts] |
| RES-07 | Operator can stop or pause a soak before unsafe storage pressure while preserving durable progress and an actionable next step. [VERIFIED: .planning/REQUIREMENTS.md] | Backfill evidence to Phase 76 soak preflight, `resource_stop`, checkpoint/report, and support projection summaries. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-03-SUMMARY.md; .planning/phases/76-disk-and-resource-bound-enforcement/76-04-SUMMARY.md] |
| RES-08 | Contributor can verify resource-bound behavior with deterministic fixtures that do not require a public peer, real service manager, or large local disk allocation. [VERIFIED: .planning/REQUIREMENTS.md] | Backfill evidence to `bun test scripts/check-phase76-resource-bounds.test.ts`, `bun run scripts/check-phase76-resource-bounds.ts`, focused Cargo tests, and full `bash scripts/verify.sh`. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/76-VERIFICATION.md; scripts/verify.sh] |
| REC-05 | Operator can detect lock contention, stale lock evidence, and concurrent datadir use with no hidden mutation of the source datadir. [VERIFIED: .planning/REQUIREMENTS.md] | Backfill Phase 77 verification coverage from probe-only lock summaries, no-mutation docs, parity roots, and checker anchors. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/*SUMMARY.md; scripts/check-phase77-corruption-lock-recovery.ts] |
| REC-06 | Operator can detect corruption markers, schema mismatches, partial writes, and unreadable runtime stores with typed recovery categories. [VERIFIED: .planning/REQUIREMENTS.md] | Backfill evidence to Phase 77 classifier, Fjall recovery fixtures, status/support/soak projections, docs, and parity roots. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-VERIFICATION.md; scripts/check-phase77-corruption-lock-recovery.ts] |
| REC-07 | Operator can generate recovery evidence that separates safe retry, read-only inspection, backup-then-rebuild, and stop-and-escalate guidance. [VERIFIED: .planning/REQUIREMENTS.md] | Backfill evidence to `recovery_evidence`, action-class summaries, operator docs, support/dashboard/soak projections, and Phase 77 full verification. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-01-SUMMARY.md; .planning/phases/77-corruption-and-lock-recovery-hardening/77-06-SUMMARY.md] |
| REC-08 | Contributor can run deterministic recovery tests for lock contention, stale lock, corruption marker, schema mismatch, partial write, and storage-open failure paths. [VERIFIED: .planning/REQUIREMENTS.md] | Backfill evidence to Phase 77 Rust fixtures, checker fixture tests, checker run, and full `bash scripts/verify.sh`. [VERIFIED: .planning/phases/77-corruption-and-lock-recovery-hardening/77-VERIFICATION.md; scripts/check-phase77-corruption-lock-recovery.ts; scripts/verify.sh] |
</phase_requirements>

## Summary

Phase 81 is a traceability and evidence-closure phase, not a runtime implementation phase. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md] The failed v1.7 audit reports eight orphaned requirement IDs because RES-05 through RES-08 and REC-05 through REC-08 are absent from Phase 75-80 `*-VERIFICATION.md` requirement entries even though the Phase 76/77 summaries, parity roots, deterministic checkers, and full verification reports exist. [VERIFIED: .planning/v1.7-MILESTONE-AUDIT.md; .planning/phases/76-disk-and-resource-bound-enforcement/*SUMMARY.md; .planning/phases/77-corruption-and-lock-recovery-hardening/*SUMMARY.md]

The planner should sequence work as: backfill `76-VERIFICATION.md` and `77-VERIFICATION.md` with explicit `requirements:` frontmatter plus `### Requirements Coverage`, refresh `.planning/REQUIREMENTS.md` and `.planning/ROADMAP.md` after those IDs are explicit, update `.planning/PROJECT.md` and `.planning/STATE.md` narrowly so v1.7 is not described as complete after Phase 80 while Phase 81 is active, fix the single stale `planned Phase 80 checker` parity-catalog phrase, then rerun focused scans, GSD lifecycle/roadmap checks, and `bash scripts/verify.sh`. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md; docs/parity/catalog/operator-runtime-release-hardening.md; scripts/verify.sh]

**Primary recommendation:** Use existing verification artifacts and GSD tooling; do not change runtime code, broaden audit tooling, create a new evidence manifest, or add broad prose scanners unless focused scans prove the targeted artifact repair is insufficient. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md; /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs]

## Project Constraints (from AGENTS.md)

- Use `AGENTS.md` as the repo-local instruction entrypoint, then `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant `standards/` pages. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards/index.md]
- Use `git submodule update --init --recursive` to materialize the pinned Knots baseline when needed. [VERIFIED: AGENTS.md]
- Treat `rust-toolchain.toml` as the Rust source of truth; Rust `1.94.1` is pinned locally. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code and the Bazel smoke build. [VERIFIED: AGENTS.md; scripts/verify.sh]
- During UAT, provide repo-local Cargo and Bazel commands, preferring `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`. [VERIFIED: AGENTS.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts; prefer TypeScript for substantial script logic and Bash for thin wrappers. [VERIFIED: AGENTS.md]
- This repo has no `package.json`, and `.planning/STACK.md` says there is no `bun install` bootstrap step. [VERIFIED: rg package-manager probe; AGENTS.md]
- `bash scripts/install-git-hooks.sh` installs repo-managed hooks, and `bash scripts/verify.sh` runs hook installation outside CI through `scripts/ensure-git-hooks.sh`. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Treat `docs/metrics/lines-of-code.md` as tracked generated output that may change when verification regenerates it. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Record intentional in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md; docs/parity/index.json]
- If adding first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update `docs/parity/source-breadcrumbs.json` and keep `scripts/check-parity-breadcrumbs.ts --check` green. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts]
- After substantial feature, parity, operator-surface, or workflow changes, check README files for needed updates. [VERIFIED: AGENTS.md]
- Follow functional-core / imperative-shell boundaries and keep pure business logic free of direct I/O and runtime side effects. [VERIFIED: AGENTS.md; standards/core/architecture.md]
- Keep tests focused and use Arrange, Act, Assert comments for non-trivial unit tests. [VERIFIED: AGENTS.md; standards/core/testing.md]
- For Rust commits, repo guidance requires format, clippy, build, and test checks; the aggregate repo-native path is encoded in `bash scripts/verify.sh`. [VERIFIED: AGENTS.md; scripts/verify.sh]
- For TypeScript/Bun automation, avoid class inheritance in repo-owned behavior and use `maybe...` naming for nullish values. [VERIFIED: standards/languages/typescript-javascript.md]
- No project-local `.claude/skills/` or `.agents/skills/` skills were found. [VERIFIED: local `find .claude/skills .agents/skills` probe]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|---|---:|---|---|
| Markdown verification artifacts | Existing GSD artifact format | Backfill `76-VERIFICATION.md`, `77-VERIFICATION.md`, and create final Phase 81 verification evidence. [VERIFIED: .planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md] | Phase 80 already uses explicit `requirements:` frontmatter and a `### Requirements Coverage` table that the audit recognizes. [VERIFIED: .planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md; .planning/v1.7-MILESTONE-AUDIT.md] |
| GSD tools | Local `gsd-tools.cjs` | Load phase context, analyze roadmap state, update STATE through repo-owned commands, mark requirements complete, and validate lifecycle. [VERIFIED: `gsd-tools.cjs` command listing; `node ... init phase-op 81`; `node ... roadmap analyze`] | Repo context directs planning/state changes through GSD artifacts and tooling where available. [VERIFIED: AGENTS.md; .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md] |
| Bun / TypeScript | Bun 1.3.9 installed and pinned | Run existing deterministic checkers; add a small Phase 81 checker only if targeted artifact repair does not prevent regression. [VERIFIED: .bun-version; `bun --version`; scripts/verify.sh] | Recent Phase 76-80 artifact and release-boundary checkers use Bun/TS and fixture-root patterns. [VERIFIED: scripts/check-phase76-resource-bounds.ts; scripts/check-phase77-corruption-lock-recovery.ts; scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts] |
| `rg` / exact scans | ripgrep 15.1.0 installed | Verify requirement IDs, stale wording, and targeted traceability changes without broad prose inference. [VERIFIED: `rg --version`; .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md] | Phase 81 decisions call for `rg` scans before the full verifier when only docs/planning/checker artifacts change. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md] |
| `bash scripts/verify.sh` | Bash 3.2.57 installed; repo script current | Final aggregate verification gate after focused traceability checks. [VERIFIED: `bash --version`; scripts/verify.sh] | AGENTS and Phase 81 decisions require rerunning full repo-native verification before final Phase 81 verification evidence. [VERIFIED: AGENTS.md; .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|---|---:|---|---|
| Node.js | v24.13.0 installed | Execute `gsd-tools.cjs` commands. [VERIFIED: `node --version`; command path probe] | Use for `init`, `roadmap analyze`, `state`, `requirements mark-complete`, and lifecycle verification. [VERIFIED: /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs] |
| Rust/Cargo | 1.94.1 pinned and installed | Existing full verifier surface; not expected for Phase 81 implementation unless runtime evidence mismatch appears. [VERIFIED: rust-toolchain.toml; `cargo --version`; `rustc --version`] | Required by `bash scripts/verify.sh`; avoid new Rust edits unless a concrete evidence mismatch is found. [VERIFIED: scripts/verify.sh; .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md] |
| Bazel | 8.6.0 installed; `rules_rust` 0.69.0 | Existing full verifier surface. [VERIFIED: `bazel --version`; MODULE.bazel] | Required by `bash scripts/verify.sh`; not a Phase 81 implementation dependency. [VERIFIED: scripts/verify.sh] |
| `cargo-llvm-cov` | 0.8.5 installed | Existing coverage gate in the full verifier. [VERIFIED: `cargo llvm-cov --version`; scripts/verify.sh] | Needed only when `bash scripts/verify.sh` runs. [VERIFIED: scripts/verify.sh] |
| Git | 2.53.0 installed | Inspect status, commit if docs commit is requested, and support GSD tooling. [VERIFIED: `git --version`; clean `git status --short` probe] | Use before and after edits to detect unintended changes. [VERIFIED: AGENTS.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|---|---|---|
| Verification artifact backfill | Modify the milestone audit to infer from summaries/plans | Explicitly rejected by D-04 because it broadens the audit contract and hides missing verification coverage. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md] |
| Targeted `rg` scans | Broad all-doc prose scanner | D-10 allows broader doc sweep only if targeted searches find more stale Phase 80 current-state wording; broad prose scanners risk brittleness. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md] |
| Existing parity roots | New manifest-driven evidence registry | Explicitly out of scope for Phase 81 and Phase 80; existing roots already carry Phase 76, Phase 77, and v1.7 closeout surfaces. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md; docs/parity/index.json] |
| Existing Phase 76/77 runtime evidence | New runtime changes | D-01 says do not reopen runtime behavior unless planning finds a concrete evidence mismatch. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md] |

**Installation:**

No new dependency install is recommended for Phase 81. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md; .bun-version; rust-toolchain.toml]

```bash
git submodule update --init --recursive
bash scripts/install-git-hooks.sh
bash scripts/verify.sh
```

**Version verification:** Recommended versions were verified from local pins and local command probes because Phase 81 should add no npm or Cargo dependency. [VERIFIED: .bun-version; rust-toolchain.toml; MODULE.bazel; local version probes]

## Architecture Patterns

### Recommended Project Structure

```text
.planning/
|-- REQUIREMENTS.md                                    # Refresh completed requirement status after 76/77 verification IDs exist. [VERIFIED: .planning/REQUIREMENTS.md; 81-CONTEXT.md]
|-- ROADMAP.md                                        # Keep Phase 81 active until verification/audit pass. [VERIFIED: .planning/ROADMAP.md; 81-CONTEXT.md]
|-- PROJECT.md                                        # Narrowly correct v1.7 completion narrative while Phase 81 is active. [VERIFIED: .planning/PROJECT.md; 81-CONTEXT.md]
|-- STATE.md                                          # Prefer `gsd-tools state ...` for state updates where available. [VERIFIED: .planning/STATE.md; gsd-tools.cjs]
|-- v1.7-MILESTONE-AUDIT.md                           # Preserve failed audit as gap evidence. [VERIFIED: .planning/v1.7-MILESTONE-AUDIT.md; 81-CONTEXT.md]
`-- phases/
    |-- 76-disk-and-resource-bound-enforcement/
    |   `-- 76-VERIFICATION.md                        # Add RES requirements frontmatter and coverage. [VERIFIED: 81-CONTEXT.md]
    |-- 77-corruption-and-lock-recovery-hardening/
    |   `-- 77-VERIFICATION.md                        # Add REC requirements frontmatter and coverage. [VERIFIED: 81-CONTEXT.md]
    `-- 81-milestone-audit-traceability-closure/
        `-- 81-VERIFICATION.md                        # Record final orphan closure and full verification evidence. [VERIFIED: 81-CONTEXT.md]

docs/parity/catalog/
`-- operator-runtime-release-hardening.md              # Replace stale `planned Phase 80 checker` wording. [VERIFIED: docs/parity/catalog/operator-runtime-release-hardening.md]
```

### Pattern 1: Verification Backfill Matching Phase 80

**What:** Add explicit `requirements: [...]` frontmatter and `### Requirements Coverage` tables to Phase 76 and Phase 77 verification reports. [VERIFIED: .planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md; .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md]

**When to use:** Use this first because the milestone audit failed specifically on missing verification-file requirement entries. [VERIFIED: .planning/v1.7-MILESTONE-AUDIT.md]

**Example:**

```markdown
---
phase: 76-disk-and-resource-bound-enforcement
verified: 2026-06-15T17:37:38Z
status: passed
requirements: [RES-05, RES-06, RES-07, RES-08]
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 76-2026-06-15T13-56-15
generated_at: 2026-06-15T16:35:34Z
lifecycle_validated: true
---

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| RES-05 | 76-01, 76-02, 76-05, 76-06 | Operator can see disk, file, cache, queue, peer, in-flight, log, metric, and support-bundle bounds before starting a long soak. | SATISFIED | Existing Phase 76 summaries, parity roots, checker, focused tests, and full verifier evidence. |
```

### Pattern 2: Evidence-First Traceability Refresh

**What:** Refresh `.planning/REQUIREMENTS.md` only after Phase 76/77 verification files explicitly name the requirement IDs, then align rows to completed Phase 75, Phase 76, Phase 77, Phase 79, and Phase 80 evidence. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md; .planning/REQUIREMENTS.md]

**When to use:** Use this after verification backfill because D-05 forbids premature completion wording before the IDs are explicit. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md]

**Example:**

```bash
rg -n "requirements: \\[RES-05, RES-06, RES-07, RES-08\\]|RES-05|RES-08|Requirements Coverage" \
  .planning/phases/76-disk-and-resource-bound-enforcement/76-VERIFICATION.md
rg -n "requirements: \\[REC-05, REC-06, REC-07, REC-08\\]|REC-05|REC-08|Requirements Coverage" \
  .planning/phases/77-corruption-and-lock-recovery-hardening/77-VERIFICATION.md
node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs roadmap analyze
```

### Pattern 3: Targeted Release-Boundary Wording Cleanup

**What:** Replace the current table-row evidence phrase `planned Phase 80 checker` with wording that says the Phase 80 checker is implemented and wired. [VERIFIED: docs/parity/catalog/operator-runtime-release-hardening.md; scripts/verify.sh]

**When to use:** Use a targeted edit unless `rg` finds more current-state stale Phase 80 wording that affects the v1.7 release boundary. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md]

**Example:**

```bash
rg -n "planned Phase 80 checker|planned .*check-phase80|Phase 80 checker" \
  docs/parity/catalog/operator-runtime-release-hardening.md \
  docs/parity/release-readiness.md \
  docs/parity/index.json \
  docs/parity/checklist.md \
  .planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md
```

### Anti-Patterns to Avoid

- **Changing runtime behavior for an artifact gap:** Phase 76 and Phase 77 already passed implementation, checker, summary, parity, and full verification evidence. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md; .planning/phases/76-disk-and-resource-bound-enforcement/76-VERIFICATION.md; .planning/phases/77-corruption-and-lock-recovery-hardening/77-VERIFICATION.md]
- **Teaching the audit to infer from summaries:** The locked decision is to satisfy the strict audit expectation through explicit verification artifacts. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md]
- **Marking v1.7 shipped after Phase 80 while Phase 81 is active:** `.planning/PROJECT.md` and `.planning/STATE.md` currently still contain Phase 80-complete/v1.7-complete language that Phase 81 must narrow. [VERIFIED: .planning/PROJECT.md; .planning/STATE.md]
- **Erasing the failed audit:** The failed audit is evidence for why Phase 81 exists and should remain understandable. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md; .planning/v1.7-MILESTONE-AUDIT.md]
- **Adding a broad evidence manifest or production-node claims:** Both are out of scope for Phase 81. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---|---|---|---|
| Requirement orphan detection | A custom parser that invents new audit semantics | Explicit `requirements:` frontmatter, `### Requirements Coverage`, and focused `rg` scans against verification files. [VERIFIED: .planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md; .planning/v1.7-MILESTONE-AUDIT.md] | The audit failed because verification requirement entries were missing, not because implementation evidence was absent. [VERIFIED: .planning/v1.7-MILESTONE-AUDIT.md] |
| Milestone state updates | Broad manual rewrites of `STATE.md` | `gsd-tools state ...` commands where they cover the needed state change. [VERIFIED: gsd-tools.cjs command listing; 81-CONTEXT.md] | Phase 81 decisions explicitly prefer repo-owned state tooling for `STATE.md` where available. [VERIFIED: 81-CONTEXT.md] |
| Release evidence registry | New manifest-driven v1.7 evidence system | Existing parity roots in `docs/parity/index.json`, `checklist.md`, release-readiness docs, and phase verification reports. [VERIFIED: docs/parity/index.json; docs/parity/checklist.md; docs/parity/release-readiness.md] | New evidence manifests are out of scope and would duplicate existing roots. [VERIFIED: 81-CONTEXT.md; 80-VERIFICATION.md] |
| Resource/recovery proofs | New runtime resource or recovery behavior | Phase 76 and Phase 77 summaries, checkers, parity roots, and full verification reports. [VERIFIED: 76-VERIFICATION.md; 77-VERIFICATION.md; scripts/check-phase76-resource-bounds.ts; scripts/check-phase77-corruption-lock-recovery.ts] | Runtime behavior should be reopened only if a concrete evidence mismatch is found. [VERIFIED: 81-CONTEXT.md] |
| Stale wording search | A brittle broad prose scanner over all docs | Targeted `rg` for `planned Phase 80 checker` and related current-state strings. [VERIFIED: 81-CONTEXT.md; docs/parity/catalog/operator-runtime-release-hardening.md] | D-10 allows broadening only if targeted searches find more stale wording. [VERIFIED: 81-CONTEXT.md] |

**Key insight:** Phase 81 should close the audit by making existing evidence discoverable to the strict milestone contract, not by creating more evidence systems or changing product behavior. [VERIFIED: .planning/v1.7-MILESTONE-AUDIT.md; .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md]

## Common Pitfalls

### Pitfall 1: Updating Requirements Before Verification IDs Exist

**What goes wrong:** `.planning/REQUIREMENTS.md` can say Complete while the strict audit still sees RES/REC as orphaned because the phase verification reports still lack explicit IDs. [VERIFIED: .planning/v1.7-MILESTONE-AUDIT.md; .planning/REQUIREMENTS.md]

**Why it happens:** Summaries and parity roots already mention RES/REC IDs, so it is easy to refresh status rows before repairing the actual audit input. [VERIFIED: .planning/phases/76-disk-and-resource-bound-enforcement/*SUMMARY.md; .planning/phases/77-corruption-and-lock-recovery-hardening/*SUMMARY.md]

**How to avoid:** Backfill `76-VERIFICATION.md` and `77-VERIFICATION.md` first, then refresh `.planning/REQUIREMENTS.md`. [VERIFIED: .planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md]

**Warning signs:** `rg -n "requirements:.*RES|requirements:.*REC" .planning/phases/*/*-VERIFICATION.md` still misses Phase 76 or Phase 77. [VERIFIED: current 76/77 verification reports]

### Pitfall 2: Treating Phase 81 As The Product Owner For RES/REC

**What goes wrong:** Mapping RES-05 through REC-08 only to Phase 81 can obscure that Phase 76 and Phase 77 are the implementation phases with passed evidence. [VERIFIED: .planning/ROADMAP.md; .planning/phases/76-disk-and-resource-bound-enforcement/*SUMMARY.md; .planning/phases/77-corruption-and-lock-recovery-hardening/*SUMMARY.md]

**Why it happens:** Phase 81 lists those IDs as gap-closure requirements, but its goal is traceability closure for already implemented resource/recovery work. [VERIFIED: .planning/ROADMAP.md; 81-CONTEXT.md]

**How to avoid:** In `.planning/REQUIREMENTS.md`, align completed product requirement rows to the implementation phase evidence, while keeping Phase 81 described as the active audit gap-closure phase in the roadmap until verification passes. [VERIFIED: 81-CONTEXT.md]

**Warning signs:** RES/REC traceability says Phase 81 Pending after 76/77 verification reports explicitly list the IDs. [VERIFIED: .planning/REQUIREMENTS.md; 81-CONTEXT.md]

### Pitfall 3: Accidentally Closing v1.7 Before Phase 81 Passes

**What goes wrong:** `.planning/PROJECT.md` currently says v1.7 shipped on 2026-06-18 and `.planning/STATE.md` says milestone phase work is complete after Phase 80, which conflicts with active Phase 81. [VERIFIED: .planning/PROJECT.md; .planning/STATE.md]

**Why it happens:** Phase 81 was inserted after Phase 80 verification to close audit gaps. [VERIFIED: .planning/ROADMAP.md; .planning/v1.7-MILESTONE-AUDIT.md]

**How to avoid:** Narrow the narrative to "Phase 80 verified; Phase 81 audit traceability closure active/pending" until Phase 81 verification and audit rerun pass. [VERIFIED: 81-CONTEXT.md]

**Warning signs:** Project/state text says "No active milestone requirements are pending after Phase 80 completion" while roadmap Phase 81 remains unchecked. [VERIFIED: .planning/PROJECT.md; .planning/ROADMAP.md]

### Pitfall 4: Fixing The Stale Phase 80 Wording Too Broadly

**What goes wrong:** A broad doc sweep can rewrite historical milestone context or remove useful non-claim wording. [VERIFIED: 81-CONTEXT.md; docs/parity/catalog/operator-runtime-release-hardening.md]

**Why it happens:** The stale phrase is localized, but release-boundary docs contain many historical Phase 80 and v1.7 references. [VERIFIED: `rg` stale wording scan]

**How to avoid:** Replace the exact `planned Phase 80 checker` evidence phrase first, then run targeted scans for related stale current-state wording. [VERIFIED: 81-CONTEXT.md; docs/parity/catalog/operator-runtime-release-hardening.md]

**Warning signs:** Diffs rewrite old v1.1-v1.6 catalog rows or remove explicit non-claim lists. [VERIFIED: docs/parity/catalog/operator-runtime-release-hardening.md; docs/parity/release-readiness.md]

### Pitfall 5: Skipping Full Verification Because Changes Are Docs-Only

**What goes wrong:** Phase 81 can pass focused scans but still miss generated LOC freshness, checker wiring drift, or aggregate verifier failures. [VERIFIED: scripts/verify.sh; AGENTS.md]

**Why it happens:** The implementation surface is likely planning/docs, but the repo's done contract is the aggregate verifier. [VERIFIED: AGENTS.md; 81-CONTEXT.md]

**How to avoid:** Run focused scans and GSD checks first, then run `bash scripts/verify.sh` before writing final Phase 81 verification evidence. [VERIFIED: 81-CONTEXT.md; scripts/verify.sh]

**Warning signs:** Final verification report cites only `rg` scans and does not include `bash scripts/verify.sh`. [VERIFIED: 81-CONTEXT.md]

## Code Examples

Verified patterns from repo sources:

### Phase 80 Verification Pattern To Copy

```markdown
---
phase: 80-opt-in-soak-uat-and-release-boundaries
verified: 2026-06-18T06:09:36Z
verified_at: 2026-06-18T06:09:36Z
status: passed
requirements: [VER-05, VER-06, VER-07, REL-04]
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 80-2026-06-17T22-54-57
generated_at: 2026-06-18T06:09:36Z
lifecycle_validated: true
---

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| VER-05 | 80-03 | Contributor can run `bash scripts/verify.sh` without internet access, public peers, real service managers, multi-day sleeps, current-tip timing, or large disk consumption. | SATISFIED | `scripts/verify.sh` ordering and forbidden-string checks pass; full verifier passed. |
```

Source: `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md`. [VERIFIED: file read]

### Focused Orphan Closure Scan

```bash
rg -n "requirements: \\[RES-05, RES-06, RES-07, RES-08\\]|RES-05|RES-06|RES-07|RES-08|Requirements Coverage" \
  .planning/phases/76-disk-and-resource-bound-enforcement/76-VERIFICATION.md

rg -n "requirements: \\[REC-05, REC-06, REC-07, REC-08\\]|REC-05|REC-06|REC-07|REC-08|Requirements Coverage" \
  .planning/phases/77-corruption-and-lock-recovery-hardening/77-VERIFICATION.md

rg -n "planned Phase 80 checker" docs/parity/catalog/operator-runtime-release-hardening.md
```

Source: Phase 81 decisions require focused `rg` scans for requirement IDs and stale wording. [VERIFIED: 81-CONTEXT.md]

### GSD Tooling Commands

```bash
node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs roadmap analyze
node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs state json --raw
node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs state validate --raw
node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs verify lifecycle 81 --require-plans --require-verification --raw
```

Source: `gsd-tools.cjs` exposes `roadmap analyze`, `state`, and `verify lifecycle` commands. [VERIFIED: /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|---|---|---|---|
| Treat summaries and plans as enough for requirement completion | Put explicit `requirements:` frontmatter and `### Requirements Coverage` in verification reports | Existing Phase 80 verification report, 2026-06-18 [VERIFIED: 80-VERIFICATION.md] | Planner should backfill Phase 76/77 verification reports rather than changing audit inference. [VERIFIED: v1.7-MILESTONE-AUDIT.md; 81-CONTEXT.md] |
| Describe Phase 80 checker as planned in parity catalog | Describe Phase 80 checker as implemented, wired, and verified | Phase 80 verification passed 2026-06-18 [VERIFIED: 80-VERIFICATION.md; scripts/verify.sh] | Planner should update one stale evidence phrase in `operator-runtime-release-hardening.md`. [VERIFIED: docs/parity/catalog/operator-runtime-release-hardening.md] |
| Mark v1.7 complete after Phase 80 | Keep v1.7 active until Phase 81 closes audit traceability and reruns verification | Phase 81 was added after failed audit on 2026-06-19 [VERIFIED: .planning/ROADMAP.md; .planning/v1.7-MILESTONE-AUDIT.md] | Planner should narrow project/state wording until Phase 81 passes. [VERIFIED: 81-CONTEXT.md] |
| Use default verifier as product behavior proof only | Use default verifier plus artifact traceability scans as release-audit proof | Current v1.7 audit and Phase 81 decisions [VERIFIED: v1.7-MILESTONE-AUDIT.md; 81-CONTEXT.md] | Planner must include audit-oriented checks, not just runtime checker passes. [VERIFIED: 81-CONTEXT.md] |

**Deprecated/outdated:**

- The phrase `planned Phase 80 checker` is outdated because `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` exists and `scripts/verify.sh` runs it after Phase 79. [VERIFIED: docs/parity/catalog/operator-runtime-release-hardening.md; scripts/verify.sh]
- The statement that no active milestone requirements are pending after Phase 80 completion is outdated while Phase 81 remains active. [VERIFIED: .planning/PROJECT.md; .planning/ROADMAP.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|---|---|---|
| A1 | Research validity is estimated through 2026-07-19 or until Phase 81 execution changes target artifacts. [ASSUMED] | Metadata | Planner might rely on stale artifact state if Phase 81 files change before planning. |

## Open Questions (RESOLVED)

1. **Which exact command should rerun the full milestone audit?**
   - What we know: `gsd-tools.cjs` exposes `audit-uat`, `roadmap analyze`, `verify lifecycle`, `requirements mark-complete`, and `milestone complete`, but no direct `milestone audit` subcommand was found in the local command listing. [VERIFIED: /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs]
   - What's unclear: The prior `.planning/v1.7-MILESTONE-AUDIT.md` may have been produced by the `/gsd-audit-milestone` skill rather than a direct `gsd-tools.cjs` subcommand. [VERIFIED: .planning/v1.7-MILESTONE-AUDIT.md; available skills list]
   - Resolution: Use the exact GSD milestone audit command, `/gsd-audit-milestone`, for v1.7 if it is available in the executor environment. If it is not callable there, the deterministic fallback scans listed in Plan 81-04 are the authoritative rerun evidence for Phase 81, together with `gsd-tools roadmap analyze`, `gsd-tools state validate --raw`, and Task 2 full `bash scripts/verify.sh`. [VERIFIED: 81-CONTEXT.md; .planning/phases/81-milestone-audit-traceability-closure/81-04-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|---|---|---|---|---|
| Bun | Existing checkers and optional Phase 81 checker | yes | 1.3.9 | Missing Bun blocks checker work; install per repo toolchain policy. [VERIFIED: .bun-version; `bun --version`] |
| Node.js | GSD tools | yes | v24.13.0 | Missing Node blocks GSD tooling; use installed Node path. [VERIFIED: `node --version`; command path probe] |
| `rg` | Focused artifact scans | yes | 15.1.0 | Fallback to `grep`, but repo guidance prefers `rg`. [VERIFIED: `rg --version`; developer instructions] |
| Bash | `scripts/verify.sh` | yes | 3.2.57 | No fallback for repo-native verifier. [VERIFIED: `bash --version`; scripts/verify.sh] |
| Cargo | Full verifier | yes | 1.94.1 | No fallback for Rust workspace verification. [VERIFIED: `cargo --version`; scripts/verify.sh] |
| Rustc | Full verifier | yes | 1.94.1 | No fallback for Rust build/test verification. [VERIFIED: `rustc --version`; rust-toolchain.toml] |
| Bazel | Full verifier | yes | 8.6.0 | No fallback for Bazel smoke build in `scripts/verify.sh`. [VERIFIED: `bazel --version`; scripts/verify.sh] |
| cargo-llvm-cov | Full verifier coverage gate | yes | 0.8.5 | No fallback in current `scripts/verify.sh`. [VERIFIED: `cargo llvm-cov --version`; scripts/verify.sh] |
| Git | Status/diff and GSD commit path | yes | 2.53.0 | No practical fallback for repo workflow. [VERIFIED: `git --version`] |

**Missing dependencies with no fallback:** None found. [VERIFIED: command path probes]

**Missing dependencies with fallback:** None found. [VERIFIED: command path probes]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---|---|---|
| V2 Authentication | no | Phase 81 should not add auth, remote admin, wallet-production, or hosted-upload surfaces. [VERIFIED: 81-CONTEXT.md] |
| V3 Session Management | no | Phase 81 should not add sessions or service identity flows. [VERIFIED: 81-CONTEXT.md] |
| V4 Access Control | no | Phase 81 should not add new runtime access-control decisions. [VERIFIED: 81-CONTEXT.md] |
| V5 Input Validation | yes | Use exact requirement ID scans, structured JSON parsing for parity roots when needed, and GSD lifecycle validation instead of loose prose inference. [VERIFIED: scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts; 81-CONTEXT.md] |
| V6 Cryptography | no | Phase 81 should not add cryptographic behavior or signed packaging. [VERIFIED: 81-CONTEXT.md] |

### Known Threat Patterns for Phase 81

| Pattern | STRIDE | Standard Mitigation |
|---|---|---|
| Requirement traceability tampering or omission | Tampering / Repudiation | Require explicit `requirements:` frontmatter and coverage tables in Phase 76/77 verification reports, then scan for all RES/REC IDs. [VERIFIED: v1.7-MILESTONE-AUDIT.md; 81-CONTEXT.md] |
| Erasing the failed audit trail | Repudiation | Preserve `.planning/v1.7-MILESTONE-AUDIT.md` as historical gap evidence and add/update current results separately. [VERIFIED: 81-CONTEXT.md] |
| Default verifier scope creep | Denial of Service | Keep public-network, service-manager, multi-day, current-tip, and large-disk work outside `bash scripts/verify.sh`. [VERIFIED: 81-CONTEXT.md; scripts/verify.sh] |
| Accidental production-readiness claim | Spoofing / Repudiation | Keep v1.7 wording scoped to source-built, explicit opt-in full-sync soak and recovery hardening. [VERIFIED: 81-CONTEXT.md; docs/parity/release-readiness.md] |
| Raw evidence or support artifact expansion | Information Disclosure | Do not add raw logs, raw support bundles, automatic uploads, or new runtime evidence capture as part of Phase 81. [VERIFIED: 81-CONTEXT.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md` - locked Phase 81 decisions, discretion, deferred scope, canonical references. [VERIFIED: file read]
- `.planning/v1.7-MILESTONE-AUDIT.md` - failed audit root cause, orphaned RES/REC IDs, stale traceability notes, and audit evidence commands. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 81 goal, success criteria, Phase 75-80 completion state, active v1.7 scope. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - current requirement descriptions and stale traceability table. [VERIFIED: file read]
- `.planning/PROJECT.md` and `.planning/STATE.md` - current v1.7 narrative/state drift after Phase 80. [VERIFIED: file read]
- `.planning/phases/76-disk-and-resource-bound-enforcement/76-VERIFICATION.md` and summaries - passed Phase 76 evidence and missing explicit `requirements:` frontmatter. [VERIFIED: file read; `rg` scan]
- `.planning/phases/77-corruption-and-lock-recovery-hardening/77-VERIFICATION.md` and summaries - passed Phase 77 evidence and missing explicit `requirements:` frontmatter. [VERIFIED: file read; `rg` scan]
- `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md` - local verification artifact pattern with explicit requirements coverage. [VERIFIED: file read]
- `docs/parity/catalog/operator-runtime-release-hardening.md` - stale `planned Phase 80 checker` wording. [VERIFIED: file read; `rg` scan]
- `scripts/check-phase76-resource-bounds.ts`, `scripts/check-phase77-corruption-lock-recovery.ts`, `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts`, and `scripts/verify.sh` - checker and verifier patterns. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/core/*.md`, and `standards/languages/*.md` - repo and standards constraints. [VERIFIED: file read]

### Secondary (MEDIUM confidence)

- `/Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs` - available GSD command surface for state, roadmap, requirements, milestone, and lifecycle operations. [VERIFIED: local source grep]

### Tertiary (LOW confidence)

- None. [VERIFIED: no web or unverified third-party sources used]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - all tools are repo-local, pinned, or locally probed, and no new dependency is recommended. [VERIFIED: .bun-version; rust-toolchain.toml; MODULE.bazel; local version probes]
- Architecture: HIGH - Phase 81 is constrained to existing GSD verification-artifact, parity-root, and repo-native verification patterns. [VERIFIED: 81-CONTEXT.md; 80-VERIFICATION.md; scripts/verify.sh]
- Pitfalls: HIGH - pitfalls come from the failed audit and current artifact drift found in local files. [VERIFIED: v1.7-MILESTONE-AUDIT.md; .planning/REQUIREMENTS.md; .planning/PROJECT.md; .planning/STATE.md]

**Research date:** 2026-06-19
**Valid until:** 2026-07-19 or until Phase 81 execution changes the target artifacts. [ASSUMED]
