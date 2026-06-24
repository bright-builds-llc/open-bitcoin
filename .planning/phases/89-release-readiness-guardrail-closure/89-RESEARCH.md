# Phase 89: Release Readiness Guardrail Closure - Research

**Researched:** 2026-06-24 [VERIFIED: system timestamp]  
**Domain:** docs/parity release-readiness guardrails and Bun/TypeScript deterministic checker closure [VERIFIED: 89-CONTEXT.md; scripts/check-phase87-release-readiness.ts; scripts/check-phase88-deterministic-claim-guardrails.ts]  
**Confidence:** HIGH [VERIFIED: mandatory context files, canonical parity docs, checker source, local tool availability]

<user_constraints>
## User Constraints (from CONTEXT.md)

All bullets in this section are copied from `.planning/phases/89-release-readiness-guardrail-closure/89-CONTEXT.md`; they are locked planning constraints for Phase 89. [VERIFIED: 89-CONTEXT.md]

### Locked Decisions

#### Release-Readiness Checklist Closure

- **D-01:** Add REL-02, REL-03, and REL-04 rows to the canonical
  `docs/parity/release-readiness.md` v1.8 checklist instead of leaving Phase 88
  ownership in prose below the table.
- **D-02:** Each new row must include Phase 88 evidence, focused checker/test
  commands, default verification posture, UAT/manual posture, residual risk,
  and no-claim or next-gate status.
- **D-03:** Preserve the existing release-readiness table shape and keep the
  Phase 87 checklist as the release reviewer source of truth. Do not create a
  second checklist or separate release evidence registry.
- **D-04:** Update checker expectations so the missing REL-02, REL-03, and
  REL-04 checklist rows cannot recur.

#### Deterministic Claim-Guardrail Corpus

- **D-05:** Expand the Phase 88 deterministic claim-guardrail corpus to include
  the missing canonical v1.8 policy docs:
  `docs/parity/upgrade-and-rollback-policy.md`,
  `docs/parity/operator-runbooks.md`, and
  `docs/parity/service-operation-expectations.md`.
- **D-06:** Treat those docs as first-class release-review evidence roots. A
  production-readiness or deferred-surface promotion in any of them must fail
  deterministically unless the surrounding wording is explicitly scoped,
  deferred, unsupported, opt-in UAT, historical, or a future gate.
- **D-07:** Keep the corpus curated rather than scanning all historical
  `.planning/` or milestone archive files. Phase 89 should close the audit gap
  without turning scoped historical evidence into default-verifier false
  positives.

#### Fixture And Verification Coverage

- **D-08:** Add fixture coverage proving deferred-surface promotion in the newly
  covered canonical policy docs fails the Phase 88 checker.
- **D-09:** Keep valid no-claim, deferred, unsupported, opt-in UAT, and
  outside-default-verification wording passing in the expanded corpus.
- **D-10:** Run focused Phase 87 and Phase 88 checker/test commands during
  iteration, refresh generated LOC metrics if changed, and close with the
  repo-native `bash scripts/verify.sh` gate.

#### Planning Metadata Hygiene

- **D-11:** Record whether stale planning metadata was refreshed during this gap
  closure. If it remains stale, route it explicitly to milestone closeout so the
  Phase 89 verification artifact does not leave the audit concern ambiguous.
- **D-12:** Do not over-expand Phase 89 into full milestone archival. The active
  closure target is GAP-01, GAP-02, release-readiness reviewer flow, and
  deterministic claim-guardrail flow. Broader archive wording belongs to the
  milestone closeout workflow unless required by the checker or verification
  evidence.

### Claude's Discretion

- The planner may split the work into release-readiness checklist/checker
  updates, Phase 88 corpus and fixture updates, focused verification, and
  closeout evidence.
- The executor may keep Phase 89 documentation and Bun automation only; no Rust
  source changes are expected.
- If no first-party Rust source or test files change under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, no parity
  source breadcrumb update is required.

### Deferred Ideas (OUT OF SCOPE)

- Full v1.8 milestone archival and broad narrative refresh belong to milestone
  closeout after Phase 89 passes unless needed to close the audit gap.
- Future production full-node readiness, inbound serving, relay, production
  wallet safety, migration apply mode, signed packaging, hosted dashboards, GUI
  parity, public-network CI, destructive repair, and automatic support upload
  remain future-scoped.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| REL-01 | Release reviewer can use a v1.8 release-readiness checklist that maps every production-boundary requirement to docs, UAT, deterministic checks, and residual risk. [VERIFIED: .planning/REQUIREMENTS.md] | Add REL-02, REL-03, and REL-04 rows to the existing checklist table and update Phase 87 checker constants so the table covers all REL rows. [VERIFIED: docs/parity/release-readiness.md; scripts/check-phase87-release-readiness.ts; v1.8-MILESTONE-AUDIT.md] |
| REL-02 | Deterministic verification fails if release docs claim production full-node readiness without required v1.8 evidence gates. [VERIFIED: .planning/REQUIREMENTS.md] | Keep Phase 88 exact production-readiness overclaim checks and expand the curated corpus to the three canonical policy docs. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; 89-CONTEXT.md] |
| REL-03 | Deterministic verification fails if docs imply deferred surfaces are production-ready. [VERIFIED: .planning/REQUIREMENTS.md] | Add Phase 88 fixture coverage for promotion prose in upgrade, runbook, and service policy docs while preserving scoped no-claim wording. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.test.ts; docs/parity/upgrade-and-rollback-policy.md; docs/parity/operator-runbooks.md; docs/parity/service-operation-expectations.md] |
| REL-04 | Default `bash scripts/verify.sh` runs the v1.8 release-boundary checker while keeping public-network, real service-manager, and multi-day checks opt-in. [VERIFIED: .planning/REQUIREMENTS.md] | Do not add new default live-network or service-manager commands; use the already-wired Phase 87 and Phase 88 checker/test steps and verify the executable `run_step` sequence. [VERIFIED: scripts/verify.sh; scripts/check-phase88-deterministic-claim-guardrails.ts] |
</phase_requirements>

## Summary

Phase 89 is a closure phase, not a new capability phase. [VERIFIED: 89-CONTEXT.md] The implementation should make Phase 88 guardrail evidence auditable from the canonical `docs/parity/release-readiness.md` checklist, then expand the existing Phase 88 curated scan corpus to include `docs/parity/upgrade-and-rollback-policy.md`, `docs/parity/operator-runbooks.md`, and `docs/parity/service-operation-expectations.md`. [VERIFIED: 89-CONTEXT.md; v1.8-MILESTONE-AUDIT.md]

The existing Phase 87 checker only requires `REL-01`, `REL-05`, and `REL-06` in `PHASE87_REQUIREMENTS`, so it cannot currently prevent the missing REL-02 through REL-04 checklist rows from recurring. [VERIFIED: scripts/check-phase87-release-readiness.ts] The existing Phase 88 checker scans a curated target list that omits the three canonical policy docs named by the audit gap. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; v1.8-MILESTONE-AUDIT.md]

**Primary recommendation:** Update the existing docs and Bun/TypeScript checkers in place; do not create a second release checklist, a new evidence registry, a whole-repo docs scanner, new Rust code, or new default live-service/public-network verification. [VERIFIED: 89-CONTEXT.md; AGENTS.md; scripts/verify.sh]

## Project Constraints

- No `.cursor/rules/` files were found, so there are no additional rule directives from that path. [VERIFIED: Glob `.cursor/rules/**` returned 0 files]
- No `.cursor/skills/` or `.agents/skills/` project skill indexes were found. [VERIFIED: Glob `.cursor/skills/**/SKILL.md`; Glob `.agents/skills/**/SKILL.md`]
- Repo-owned higher-level automation should use Bun/TypeScript; Bash should stay thin orchestration. [VERIFIED: AGENTS.md]
- `bash scripts/verify.sh` is the repo-native final verification contract; `--fast` is only for local iteration. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Default verification must remain deterministic, public-network-free, real-service-manager-free, package-manager-service-free, support-upload-free, destructive-repair-free, and multi-day-free. [VERIFIED: 88-VERIFICATION.md; scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/verify.sh]
- `workflow.nyquist_validation` is false, so no Nyquist Validation Architecture section is required for this research artifact. [VERIFIED: .planning/config.json]
- `commit_docs` is true in GSD config, but this research step only writes the research artifact unless the orchestrator handles document commits. [VERIFIED: .planning/config.json; gsd init phase-op 89]

## Standard Stack

### Core

| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| Bun | 1.3.9 | Run and test repo-owned TypeScript checker scripts. [VERIFIED: local shell `bun --version`] | AGENTS.md makes Bun the canonical runtime for repo-owned higher-level automation. [VERIFIED: AGENTS.md] |
| TypeScript scripts under `scripts/` | Repo-owned, Bun-executed | Implement deterministic document, JSON, and verifier-text checks. [VERIFIED: scripts/check-phase87-release-readiness.ts; scripts/check-phase88-deterministic-claim-guardrails.ts] | Existing v1.8 phases use focused Bun/TypeScript checkers with fixture tests and repo-root overrides. [VERIFIED: 87-CONTEXT.md; 88-CONTEXT.md] |
| `bash scripts/verify.sh` | Repo-owned | Aggregate final verification, including Phase 87 and Phase 88 test/checker steps. [VERIFIED: scripts/verify.sh] | Repo-local guidance names it as the verification contract. [VERIFIED: AGENTS.md] |

### Supporting

| Tool | Version | Purpose | When to Use |
|------|---------|---------|-------------|
| Git | 2.53.0 | Inspect diff status and support closeout hygiene if needed. [VERIFIED: local shell `git --version`] | Use for normal repository state inspection; do not use destructive commands. [VERIFIED: developer instructions] |
| Cargo | 1.94.1 | Final verifier runs Rust format, clippy, build, tests, and coverage. [VERIFIED: local shell `cargo --version`; scripts/verify.sh] | Use only through `bash scripts/verify.sh` for this phase unless debugging focused verifier failures. [VERIFIED: AGENTS.md; scripts/verify.sh] |
| Bazel | 8.6.0 | Final verifier runs Bazel smoke builds in full mode. [VERIFIED: local shell `bazel --version`; scripts/verify.sh] | No Phase 89 code should directly add Bazel targets; Bazel is a final verification dependency. [VERIFIED: scripts/verify.sh; 89-CONTEXT.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Existing Phase 87 checker | A new Phase 89 checker | A new checker would create a second gate for the same checklist and violates D-03; extend the existing checker instead. [VERIFIED: 89-CONTEXT.md; scripts/check-phase87-release-readiness.ts] |
| Existing Phase 88 checker | Whole `docs/` or `.planning/` scanner | Whole-tree scanning risks false positives in scoped historical evidence; D-07 requires a curated corpus. [VERIFIED: 89-CONTEXT.md; 88-CONTEXT.md] |
| In-place parity roots | New evidence manifest | Phase 88 explicitly avoided a separate machine-readable evidence manifest; existing canonical docs and parity roots remain authoritative. [VERIFIED: 88-CONTEXT.md; docs/parity/index.json] |

**Installation:** No new packages should be installed for Phase 89. [VERIFIED: 89-CONTEXT.md; AGENTS.md]

```bash
# No install step. Use existing repo tools:
bun test scripts/check-phase87-release-readiness.test.ts
bun run scripts/check-phase87-release-readiness.ts
bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts
bun run scripts/check-phase88-deterministic-claim-guardrails.ts
bash scripts/verify.sh
```

**Version verification:** Tool availability was checked locally: Bun 1.3.9, Node v24.13.0, Cargo 1.94.1, Bazel 8.6.0, Git 2.53.0, and GNU Bash 3.2.57 are available. [VERIFIED: local shell tool availability audit]

## Architecture Patterns

### Recommended Project Structure

```text
docs/parity/
  release-readiness.md                    # Add REL-02, REL-03, REL-04 checklist rows. [VERIFIED: 89-CONTEXT.md]
  index.json                              # Refresh v1-8-release-readiness-checklist and claim-guardrail evidence if checker exact arrays require it. [VERIFIED: scripts/check-phase87-release-readiness.ts; scripts/check-phase88-deterministic-claim-guardrails.ts]
  checklist.md                            # Keep human-readable evidence rows coherent with index.json. [VERIFIED: docs/parity/checklist.md]

scripts/
  check-phase87-release-readiness.ts       # Require REL-02, REL-03, REL-04 rows and Phase 88 evidence. [VERIFIED: scripts/check-phase87-release-readiness.ts]
  check-phase87-release-readiness.test.ts  # Add regression for missing REL-02 through REL-04. [VERIFIED: scripts/check-phase87-release-readiness.test.ts]
  check-phase88-deterministic-claim-guardrails.ts       # Add missing canonical policy docs to curated corpus/evidence. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts]
  check-phase88-deterministic-claim-guardrails.test.ts  # Add corpus-specific deferred-promotion failures and scoped-passing cases. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.test.ts]

.planning/phases/89-release-readiness-guardrail-closure/
  89-VERIFICATION.md                       # Record focused checks, final verifier, and metadata hygiene decision. [VERIFIED: 89-CONTEXT.md]
```

### Pattern 1: Extend the Checklist, Not the Release Surface

**What:** Add three rows to the existing `## v1.8 Release Readiness Checklist` table for REL-02, REL-03, and REL-04. [VERIFIED: docs/parity/release-readiness.md; 89-CONTEXT.md]

**When to use:** Use this pattern for GAP-01 and the release-readiness review flow gap. [VERIFIED: v1.8-MILESTONE-AUDIT.md]

**Implementation notes:** Keep the current table header and row vocabulary; each new row should name Phase 88 evidence, `bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts`, `bun run scripts/check-phase88-deterministic-claim-guardrails.ts`, `bash scripts/verify.sh`, opt-in/manual posture, residual risk, and no-claim or next-gate status. [VERIFIED: 89-CONTEXT.md; docs/parity/release-readiness.md]

### Pattern 2: Make the Phase 87 Checker Enforce the Added Rows

**What:** Update `PHASE87_REQUIREMENTS` to include REL-02, REL-03, and REL-04, and update required command/evidence checks so Phase 88 commands are visible from the checklist. [VERIFIED: scripts/check-phase87-release-readiness.ts; 89-CONTEXT.md]

**When to use:** Use this when closing D-04 so missing rows cannot recur. [VERIFIED: 89-CONTEXT.md]

**Example from current checker:**

```typescript
const PHASE87_REQUIREMENTS = [
  "PROD-01",
  // ...
  "REL-01",
  "REL-05",
  "REL-06",
] as const;
```

The planner should direct the executor to add REL-02, REL-03, and REL-04 here and mirror that exact requirement list in fixture parity-index text. [VERIFIED: scripts/check-phase87-release-readiness.ts; scripts/check-phase87-release-readiness.test.ts]

### Pattern 3: Expand the Curated Phase 88 Corpus In Place

**What:** Add the three missing policy docs to `TARGET_FILES`, `POINTER_FILES`, and `REQUIRED_EVIDENCE` where appropriate, then update parity roots and fixture constants to match. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase88-deterministic-claim-guardrails.test.ts; 89-CONTEXT.md]

**When to use:** Use this for GAP-02 and the deterministic claim-guardrail flow gap. [VERIFIED: v1.8-MILESTONE-AUDIT.md]

**Example from current checker:**

```typescript
const TARGET_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  // missing canonical policy docs belong here for Phase 89
  "scripts/verify.sh",
] as const;
```

### Pattern 4: Preserve Fixture Repositories and Env Overrides

**What:** Use the existing temporary-repository fixture pattern with `OPEN_BITCOIN_PHASE87_REPO_ROOT` and `OPEN_BITCOIN_PHASE88_REPO_ROOT`. [VERIFIED: scripts/check-phase87-release-readiness.test.ts; scripts/check-phase88-deterministic-claim-guardrails.test.ts]

**When to use:** Use this to prove missing rows fail, bad promotion prose fails in each newly covered policy doc, and scoped no-claim/deferred wording still passes. [VERIFIED: 89-CONTEXT.md]

### Anti-Patterns to Avoid

- **Second checklist:** Do not create another release-readiness checklist or registry; `docs/parity/release-readiness.md` remains canonical. [VERIFIED: 89-CONTEXT.md]
- **Whole-history scanning:** Do not scan all `.planning/` or milestone archives in default verification; Phase 89 must avoid historical false positives. [VERIFIED: 89-CONTEXT.md; 88-CONTEXT.md]
- **Default live-service checks:** Do not add public-network live smoke, real `systemctl` or `launchctl`, multi-day sleeps, package-manager service commands, destructive repair, or support-upload checks to default verification. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/verify.sh]
- **Rust source churn:** Do not change first-party Rust source or tests unless planning discovers an unexpected narrow gap; no Rust changes are expected. [VERIFIED: 89-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Release checklist enforcement | A new ad hoc Markdown parser or duplicate checklist registry | Existing `checkPhase87ReleaseReadiness` constants and normalized substring helpers [VERIFIED: scripts/check-phase87-release-readiness.ts] | Existing checker already validates table header, requirements, canonical roots, commands, pointers, parity JSON, and verifier order. [VERIFIED: scripts/check-phase87-release-readiness.ts] |
| Deferred-surface claim scanning | A broad regex over the whole repository | Existing `checkPhase88DeterministicClaimGuardrails` curated corpus plus context-unit scanning [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts] | Existing checker already splits prose/table rows, applies scoped allow terms, and rejects unscoped production/deferred promotions. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts] |
| Verifier order checks | Trusting the legacy `VERIFY_COMMAND_ORDER` heredoc | Existing executable verifier text stripping in Phase 87/88 checkers [VERIFIED: scripts/check-phase87-release-readiness.ts; scripts/check-phase88-deterministic-claim-guardrails.ts] | Both checkers remove the heredoc before checking executed `run_step` commands. [VERIFIED: scripts/check-phase87-release-readiness.ts; scripts/check-phase88-deterministic-claim-guardrails.ts] |
| Evidence roots | A new machine-readable evidence manifest | Existing `docs/parity/index.json`, `docs/parity/checklist.md`, and canonical parity docs [VERIFIED: docs/parity/index.json; docs/parity/checklist.md; 88-CONTEXT.md] | Phase 88 explicitly kept canonical docs as the source of truth and avoided a new manifest. [VERIFIED: 88-CONTEXT.md] |

**Key insight:** Phase 89 should strengthen existing deterministic gates and evidence roots, not introduce parallel release-governance machinery. [VERIFIED: 89-CONTEXT.md; v1.8-MILESTONE-AUDIT.md]

## Common Pitfalls

### Pitfall 1: Fixing Prose Without Updating Checkers

**What goes wrong:** REL-02 through REL-04 rows get added to `release-readiness.md`, but the Phase 87 checker still only requires REL-01, REL-05, and REL-06. [VERIFIED: scripts/check-phase87-release-readiness.ts]

**Why it happens:** The current checker constants predate Phase 89 and exactly encode the Phase 87 scope. [VERIFIED: scripts/check-phase87-release-readiness.ts; 87-VERIFICATION.md]

**How to avoid:** Update checker constants, fixture expected arrays, and parity roots in the same plan as the checklist rows. [VERIFIED: scripts/check-phase87-release-readiness.ts; scripts/check-phase87-release-readiness.test.ts]

**Warning signs:** New checklist rows pass manual review but a fixture removing REL-02, REL-03, or REL-04 does not fail. [VERIFIED: 89-CONTEXT.md]

### Pitfall 2: Expanding the Corpus Without Evidence Roots

**What goes wrong:** `TARGET_FILES` includes the new policy docs, but `REQUIRED_EVIDENCE`, `POINTER_FILES`, parity index evidence, or checklist rows omit them. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; docs/parity/index.json]

**Why it happens:** Phase 88 tracks target scanning, human pointers, and parity root evidence as related but separate arrays. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts]

**How to avoid:** Update `TARGET_FILES`, `POINTER_FILES`, `REQUIRED_EVIDENCE`, test fixture constants, and parity roots together. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase88-deterministic-claim-guardrails.test.ts]

**Warning signs:** The checker scans a doc but parity evidence for `v1-8-deterministic-claim-guardrails` does not list that doc. [VERIFIED: docs/parity/index.json; scripts/check-phase88-deterministic-claim-guardrails.ts]

### Pitfall 3: False Positives From Valid Scoped Wording

**What goes wrong:** The expanded checker rejects legitimate text saying a surface is deferred, unsupported, outside default verification, opt-in UAT, historical, or future-gated. [VERIFIED: 89-CONTEXT.md; scripts/check-phase88-deterministic-claim-guardrails.ts]

**Why it happens:** The new policy docs heavily mention deferred surfaces, but they do so as scoped non-claims. [VERIFIED: docs/parity/upgrade-and-rollback-policy.md; docs/parity/operator-runbooks.md; docs/parity/service-operation-expectations.md]

**How to avoid:** Add both failing promotion fixtures and passing scoped fixtures for the newly covered docs. [VERIFIED: 89-CONTEXT.md; scripts/check-phase88-deterministic-claim-guardrails.test.ts]

**Warning signs:** Existing canonical policy docs fail immediately after being added to `TARGET_FILES`. [VERIFIED: docs/parity/upgrade-and-rollback-policy.md; docs/parity/operator-runbooks.md; docs/parity/service-operation-expectations.md]

### Pitfall 4: Accidentally Broadening Default Verification

**What goes wrong:** A closeout edit adds live public-network checks, real service-manager commands, or multi-day timing gates to `scripts/verify.sh`. [VERIFIED: .planning/REQUIREMENTS.md; scripts/check-phase88-deterministic-claim-guardrails.ts]

**Why it happens:** REL-04 is about default verifier coverage, but it explicitly keeps live/public/service/multi-day checks opt-in. [VERIFIED: .planning/REQUIREMENTS.md; 88-VERIFICATION.md]

**How to avoid:** Keep Phase 89 to existing Phase 87/88 deterministic checker steps; only update `scripts/verify.sh` if a checker constant or command-order assertion truly requires it. [VERIFIED: scripts/verify.sh; 89-CONTEXT.md]

**Warning signs:** `scripts/verify.sh` executable text includes `run-live-mainnet-smoke`, `systemctl`, `launchctl`, long `sleep`, `--restart-after-progress`, `public-network CI`, support upload, destructive repair, or broad production-node readiness text. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts]

### Pitfall 5: Leaving Planning Metadata Ambiguous

**What goes wrong:** Phase 89 closes the audit gaps but does not say whether stale `REQUIREMENTS.md`, `ROADMAP.md`, `STATE.md`, or `PROJECT.md` metadata was refreshed or deferred to milestone closeout. [VERIFIED: v1.8-MILESTONE-AUDIT.md; 89-CONTEXT.md]

**Why it happens:** The audit separated critical integration gaps from non-blocking closeout hygiene. [VERIFIED: v1.8-MILESTONE-AUDIT.md]

**How to avoid:** Add an explicit Phase 89 verification note: refreshed now, or routed to milestone closeout. [VERIFIED: 89-CONTEXT.md]

**Warning signs:** Phase 89 verification says the guardrails passed but never mentions stale planning metadata. [VERIFIED: 89-CONTEXT.md; v1.8-MILESTONE-AUDIT.md]

## Code Examples

Verified patterns from existing source:

### Existing Phase 87 Requirement Gate

```typescript
const PHASE87_REQUIREMENTS = [
  "PROD-01",
  "PROD-02",
  "PROD-03",
  "PROD-04",
  "SUP-01",
  "SUP-02",
  "SUP-03",
  "SUP-04",
  "UPG-01",
  "UPG-02",
  "UPG-03",
  "UPG-04",
  "RUN-01",
  "RUN-02",
  "RUN-03",
  "SVC-01",
  "SVC-02",
  "REL-01",
  "REL-05",
  "REL-06",
] as const;
```

Source: `scripts/check-phase87-release-readiness.ts`; Phase 89 should add `REL-02`, `REL-03`, and `REL-04` to close GAP-01. [VERIFIED: scripts/check-phase87-release-readiness.ts; v1.8-MILESTONE-AUDIT.md]

### Existing Phase 88 Corpus Gate

```typescript
const TARGET_FILES = [
  "README.md",
  "docs/operator/runtime-guide.md",
  "docs/parity/production-claim-boundary.md",
  "docs/parity/support-matrix.md",
  "docs/parity/release-readiness.md",
  "docs/parity/deviations-and-unknowns.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
  "scripts/verify.sh",
] as const;
```

Source: `scripts/check-phase88-deterministic-claim-guardrails.ts`; Phase 89 should add the upgrade policy, operator runbooks, and service expectations docs to the curated corpus. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; 89-CONTEXT.md]

### Existing Scoped Allow Rule

```typescript
const ALLOWED_SCOPE_TERMS = [
  "does not claim",
  "not allowed yet",
  "deferred",
  "unsupported",
  "historical",
  "opt-in UAT",
  "future gate",
  "outside default verification",
  "defines gates",
  "future milestone",
  "does not prove",
  "does not add",
  "without claiming",
  "without internet access",
  "no public-network",
  "remain outside",
  "remains outside",
] as const;
```

Source: `scripts/check-phase88-deterministic-claim-guardrails.ts`; fixtures should prove these scoped terms still permit valid no-claim wording in the newly covered policy docs. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; 89-CONTEXT.md]

## State of the Art

| Old Approach | Current Phase 89 Approach | When Changed | Impact |
|--------------|---------------------------|--------------|--------|
| Phase 87 checklist prose says Phase 88 owns REL-02 through REL-04 outside the table. [VERIFIED: docs/parity/release-readiness.md] | Add REL-02, REL-03, and REL-04 as first-class table rows with evidence, verification, UAT/manual posture, residual risk, and no-claim or next-gate status. [VERIFIED: 89-CONTEXT.md] | Phase 89 gap closure. [VERIFIED: .planning/ROADMAP.md] | Release reviewers can audit all REL rows from one checklist. [VERIFIED: v1.8-MILESTONE-AUDIT.md] |
| Phase 88 scans a curated public release/operator corpus that omits three canonical policy docs. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts] | Add `upgrade-and-rollback-policy.md`, `operator-runbooks.md`, and `service-operation-expectations.md` to the curated corpus. [VERIFIED: 89-CONTEXT.md] | Phase 89 gap closure. [VERIFIED: .planning/ROADMAP.md] | Deferred-surface promotion in canonical policy docs fails deterministically. [VERIFIED: 89-CONTEXT.md] |
| Planning metadata hygiene is noted as audit tech debt. [VERIFIED: v1.8-MILESTONE-AUDIT.md] | Phase 89 verification must record whether metadata was refreshed or routed to milestone closeout. [VERIFIED: 89-CONTEXT.md] | Phase 89 closeout. [VERIFIED: 89-CONTEXT.md] | The audit concern is not left ambiguous. [VERIFIED: 89-CONTEXT.md] |

**Deprecated/outdated:** Treating Phase 88 ownership prose below the checklist as sufficient REL-02 through REL-04 reviewer evidence is outdated after the milestone audit. [VERIFIED: v1.8-MILESTONE-AUDIT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | ASVS category names in the Security Domain section follow the GSD template taxonomy rather than a session-verified ASVS document. [ASSUMED] | Security Domain | Low implementation risk because Phase 89 does not add authentication, session, cryptography, or runtime security features; planner should not treat this as external compliance evidence. |

## Open Questions (RESOLVED)

1. **Should stale planning metadata be refreshed during Phase 89 or explicitly deferred to milestone closeout?** [VERIFIED: 89-CONTEXT.md; v1.8-MILESTONE-AUDIT.md]
   - What we know: D-11 requires Phase 89 closeout to record the decision. [VERIFIED: 89-CONTEXT.md]
   - What's unclear: The phase can either do a narrow refresh or route the work to milestone closeout. [VERIFIED: 89-CONTEXT.md]
   - Recommendation: Plan a closeout task that first checks whether checker/doc edits require metadata changes; if not, write an explicit verification note routing broader archive narrative to milestone closeout. [VERIFIED: 89-CONTEXT.md]
   - **RESOLVED:** Phase 89 verification must record the decision explicitly. Refresh only metadata that is required by the checker/doc edits; otherwise route broader archive narrative to milestone closeout. [VERIFIED: 89-CONTEXT.md; 89-03-PLAN.md]

2. **Should `v1-8-release-readiness-checklist` parity root requirements expand to include REL-02 through REL-04?** [VERIFIED: docs/parity/index.json; scripts/check-phase87-release-readiness.ts]
   - What we know: The Phase 87 checker currently exact-matches parity root requirements for the release-readiness surface. [VERIFIED: scripts/check-phase87-release-readiness.ts]
   - What's unclear: The planner must decide whether to update the existing surface requirements array or add a Phase 89-specific audit entry. [VERIFIED: docs/parity/index.json; 89-CONTEXT.md]
   - Recommendation: Update the existing release-readiness surface requirements/evidence to include REL-02 through REL-04 and Phase 88 evidence, because D-03 keeps the Phase 87 checklist as the release reviewer source of truth. [VERIFIED: 89-CONTEXT.md]
   - **RESOLVED:** Expand the existing `v1-8-release-readiness-checklist` parity root and Phase 87 checker expectations to include REL-02, REL-03, REL-04, and Phase 88 evidence rather than adding a parallel Phase 89 release checklist root. [VERIFIED: 89-CONTEXT.md; 89-01-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Bun | Phase 87/88 checker tests and scripts | yes | 1.3.9 [VERIFIED: local shell] | None needed. |
| Node | GSD tooling and possible repo scripts | yes | v24.13.0 [VERIFIED: local shell] | Use Bun for repo automation where possible. [VERIFIED: AGENTS.md] |
| Cargo | Final verifier Rust checks | yes | 1.94.1 [VERIFIED: local shell; rust-toolchain pinned in AGENTS.md] | None needed. |
| Bazel | Final verifier smoke build | yes | 8.6.0 [VERIFIED: local shell] | None needed. |
| Git | Repository state and closeout inspection | yes | 2.53.0 [VERIFIED: local shell] | None needed. |
| Bash | `scripts/verify.sh` | yes | GNU Bash 3.2.57 [VERIFIED: local shell] | None needed. |

**Missing dependencies with no fallback:** None found for the planned Phase 89 work. [VERIFIED: local shell availability audit]

**Missing dependencies with fallback:** None found for the planned Phase 89 work. [VERIFIED: local shell availability audit]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication [ASSUMED] | no | Phase 89 changes docs and deterministic local checkers only; no auth surface is in scope. [VERIFIED: 89-CONTEXT.md] |
| V3 Session Management [ASSUMED] | no | No session state is introduced by Phase 89. [VERIFIED: 89-CONTEXT.md] |
| V4 Access Control [ASSUMED] | no | No runtime authorization boundary is changed by Phase 89. [VERIFIED: 89-CONTEXT.md] |
| V5 Input Validation [ASSUMED] | yes | Validate Markdown/JSON/verifier text through existing Bun checker functions and fixture tests. [VERIFIED: scripts/check-phase87-release-readiness.ts; scripts/check-phase88-deterministic-claim-guardrails.ts] |
| V6 Cryptography [ASSUMED] | no | No cryptographic behavior or key material handling changes are in scope. [VERIFIED: 89-CONTEXT.md] |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Overbroad production-readiness claim in release docs | Spoofing / Repudiation [ASSUMED] | Phase 88 exact overclaim and scoped context-unit checks over curated docs. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts] |
| Deferred-surface promotion through policy prose | Tampering [ASSUMED] | Add missing policy docs to Phase 88 target corpus and test bad promotion prose. [VERIFIED: 89-CONTEXT.md] |
| Verifier command appearing only in legacy heredoc | Repudiation [ASSUMED] | Strip `VERIFY_COMMAND_ORDER` before checking executable `run_step` text. [VERIFIED: scripts/check-phase87-release-readiness.ts; scripts/check-phase88-deterministic-claim-guardrails.ts] |
| False release evidence from artifact existence alone | Information integrity risk [ASSUMED] | Checklist and policy docs require field-based evidence, unavailable reasons, canonical roots, and deterministic checker output. [VERIFIED: docs/parity/release-readiness.md; docs/parity/production-claim-boundary.md] |

## Sources

### Primary (HIGH confidence)

- `AGENTS.md` - repo-local Bun, verification, UAT command, generated artifact, and parity breadcrumb guidance. [VERIFIED: ReadFile]
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing. [VERIFIED: ReadFile]
- `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/languages/typescript-javascript.md`, `standards/languages/rust.md` - architecture, code-shape, testing, verification, and language-specific constraints. [VERIFIED: ReadFile]
- `.planning/phases/89-release-readiness-guardrail-closure/89-CONTEXT.md` - locked Phase 89 decisions and scope. [VERIFIED: ReadFile]
- `.planning/v1.8-MILESTONE-AUDIT.md` - GAP-01, GAP-02, release-readiness flow gap, deterministic claim-guardrail flow gap, and metadata hygiene notes. [VERIFIED: ReadFile]
- `docs/parity/release-readiness.md` - canonical v1.8 release-readiness checklist and current missing REL rows. [VERIFIED: ReadFile]
- `scripts/check-phase87-release-readiness.ts` and `.test.ts` - Phase 87 checker constants, parity root checks, fixture patterns, and verifier-order checks. [VERIFIED: ReadFile]
- `scripts/check-phase88-deterministic-claim-guardrails.ts` and `.test.ts` - curated corpus, scoped allow rules, deferred-surface promotion checks, fixture patterns, and verifier-order checks. [VERIFIED: ReadFile]
- `scripts/verify.sh` - final default verification sequence and existing Phase 87/88 wiring. [VERIFIED: ReadFile]

### Secondary (MEDIUM confidence)

- Local tool availability audit - Bun, Node, Cargo, Bazel, Git, and Bash versions available on this machine. [VERIFIED: Shell]
- `docs/parity/upgrade-and-rollback-policy.md`, `docs/parity/operator-runbooks.md`, `docs/parity/service-operation-expectations.md` - newly covered policy doc targets and their existing scoped non-claim wording. [VERIFIED: ReadFile]
- `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, and `docs/parity/catalog/operator-runtime-release-hardening.md` - current parity roots and evidence rows that likely need coherence updates. [VERIFIED: ReadFile]

### Tertiary (LOW confidence)

- ASVS category naming and STRIDE mapping in the Security Domain section are template-level guidance not externally verified in this session. [ASSUMED]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - repo-local instructions and local tool probes agree on Bun, Bash, Cargo, Bazel, and verifier usage. [VERIFIED: AGENTS.md; scripts/verify.sh; local shell]
- Architecture: HIGH - phase context locks in extending existing checklist/checkers rather than introducing new surfaces. [VERIFIED: 89-CONTEXT.md; scripts/check-phase87-release-readiness.ts; scripts/check-phase88-deterministic-claim-guardrails.ts]
- Pitfalls: HIGH - audit gaps and current checker constants directly identify the failure modes. [VERIFIED: v1.8-MILESTONE-AUDIT.md; scripts/check-phase87-release-readiness.ts; scripts/check-phase88-deterministic-claim-guardrails.ts]

**Research date:** 2026-06-24 [VERIFIED: system timestamp]  
**Valid until:** 2026-07-01 for checker/planning details; re-read current files after any Phase 89 implementation or milestone closeout changes. [ASSUMED]
