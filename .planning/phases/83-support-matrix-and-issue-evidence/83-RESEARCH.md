# Phase 83: Support Matrix and Issue Evidence - Research

**Researched:** 2026-06-21 [VERIFIED: environment current_date]
**Domain:** Open Bitcoin v1.8 support classification, issue evidence, residual-risk documentation, and deterministic doc checking [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: local project docs and prior-phase artifacts listed in Sources]

<user_constraints>
## User Constraints (from CONTEXT.md)

All content in this block is copied from `.planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md`; treat it as locked user/project context for planning. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

### Locked Decisions

#### Implementation Decisions

##### Support Matrix Scope
- **D-01:** Phase 83 should create a support matrix for **operators, contributors, and release reviewers**, not a marketing support policy. It should be stored in the parity/operator documentation set where release reviewers already look. Suggested target: `docs/parity/support-matrix.md`, with links from release readiness and operator runtime docs.
  **Rationale:** The phase goal is practical classification and evidence, not outward-facing sales/support language.
- **D-02:** The support matrix must cover at least these environment families: **source-built install**, runtime modes, network modes, storage/datadir modes, service-supervision modes, migration/recovery/support-bundle surfaces, and explicitly deferred production surfaces.
  **Rationale:** This directly matches SUP-01 and avoids narrowing the matrix to only install/runtime.
- **D-03:** Use Phase 82 support vocabulary exactly: `supported`, `preview`, `opt-in UAT`, `unsupported`, `deferred`. Do not introduce softer synonyms such as "production-ready", "best-effort", "GA", "certified", "validated", or "fully supported".
  **Rationale:** Phase 82 intentionally created a claim boundary and exact support terms; Phase 83 should operationalize those terms, not replace them.
- **D-04:** Each support matrix row should include: **support term**, **evidence basis**, **default verification**, **opt-in UAT / manual validation**, **residual risk**, and **next gate**.
  **Rationale:** This gives contributors and reviewers enough structure to avoid accidental support broadening.

##### Issue Evidence Expectations
- **D-05:** Phase 83 should define an issue evidence checklist for support requests, including redacted support bundles, relevant logs/log paths, config summaries, service state, resource bounds/pressure evidence, sync evidence, and command/version context.
  **Rationale:** This directly satisfies SUP-02 and aligns with existing status snapshot/support bundle surfaces.
- **D-06:** The issue evidence docs must explicitly ask for the **smallest useful redacted evidence set**, not raw datadirs, private keys, wallet secrets, RPC credentials, cookies, raw unbounded logs, or automatic upload.
  **Rationale:** Existing support-bundle/redaction boundaries are local and evidence-oriented; Phase 83 should preserve that safety boundary.
- **D-07:** Issue evidence examples should use repo-local command forms for UAT/operator workflows, including Cargo and Bazel forms when relevant, not only the installed `open-bitcoin` alias.
  **Rationale:** Repo-local AGENTS guidance requires copy-pasteable Cargo and Bazel commands during UAT.
- **D-08:** The docs should explain that missing evidence can be represented as an explicit unavailable reason when the typed status surface provides one; lack of a field should not be silently treated as support proof.
  **Rationale:** This matches existing status snapshot semantics and keeps support outcomes auditable.

##### Residual Risk And Manual Validation
- **D-09:** Phase 83 must include a carried-forward residual-risk/manual-validation table spanning v1.1 through v1.7. Required entries include at least: v1.1 dashboard pseudoterminal/raw-input manual validation, v1.2 closeout without a dedicated milestone audit artifact, v1.3 diagnosed-blocker closeout, v1.4 planning traceability correction, and recurring public-network, service-manager, multi-day, support-bundle, recovery, and production-scope non-claims.
  **Rationale:** SUP-04 is explicitly about historical carried-forward risk visibility.
- **D-10:** Residual risks must distinguish between **verified deterministic behavior**, **opt-in UAT evidence**, **manual validation surface**, and **deferred/non-claim**.
  **Rationale:** Release reviewers need to know which gaps are acceptable for v1.8 versus future-phase gates.
- **D-11:** Do not convert residual risks into release blockers unless existing docs already classify them as blockers. Keep the table descriptive and gate-oriented.
  **Rationale:** The phase goal is visibility and support classification, not changing milestone readiness decisions.

##### Contributor Update Boundaries
- **D-12:** Add contributor-facing update rules for support matrix changes: new rows or promoted support levels require a concrete evidence source, verifier or opt-in UAT command, residual-risk statement, and next gate.
  **Rationale:** SUP-03 asks contributors to preserve production-boundary and deferred-surface limits.
- **D-13:** Support matrix edits must preserve Phase 82 production-boundary links and deferred-surface limits. Deferred surfaces cannot be promoted by prose-only edits.
  **Rationale:** This prevents accidental broadening of production claims.
- **D-14:** If automation is added, it should be a narrow deterministic checker for the support matrix, issue evidence checklist, residual risk table, canonical links, and exact support terms. It must not become a broad all-doc production-claim scanner.
  **Rationale:** Phase 82 already has a production boundary checker; Phase 88 owns broader scanner work.
- **D-15:** Any new checker must remain short-running and default-verification safe: no public-network access, real service-manager mutation, multi-day soak, large disk allocation, source datadir mutation, or support-bundle upload.
  **Rationale:** This preserves the repo-native `bash scripts/verify.sh` contract.

#### Folded Todos

No pending todos were folded into this discussion context. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

### the agent's Discretion
- Decide whether to create a new `docs/parity/support-matrix.md` or extend an existing doc, but prefer a single canonical reader path over scattered duplicated tables.
- Decide the exact support matrix row taxonomy, as long as it covers the environment families in D-02 and preserves Phase 82 support terms.
- Decide whether to update `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md`, `docs/operator/runtime-guide.md`, catalog docs, and README links based on existing doc structure.
- Decide whether the narrow checker is worth adding in Phase 83; if not, document why manual review is sufficient. If yes, follow existing Bun/TypeScript checker and fixture-test patterns.
- Decide exact verification commands, but include focused deterministic checks and final `bash scripts/verify.sh`.

### Deferred Ideas (OUT OF SCOPE)
- Broad production-claim scanner across every markdown/code file. Phase 88 owns this.
- New runtime support-bundle collection behavior beyond documentation/checking of existing evidence expectations.
- New service-manager integration, public-network live checks, multi-day soak automation, current-tip timing gates, or release-blocking live-sync checks.
- Packaging/signing/installers, GUI support, hosted dashboards, automatic support upload, destructive recovery/repair, migration apply mode, production funds/wallet support, or broad production full-node readiness.
- Changing Phase 82 support terms or promoting any deferred surface to supported based only on documentation edits.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SUP-01 | Operators can classify source-built install, runtime, network, storage, and service-supervision environments by support level. [VERIFIED: .planning/REQUIREMENTS.md] | Use the Phase 82 exact support terms and a row schema with evidence basis, default verification, opt-in UAT/manual validation, residual risk, and next gate. [VERIFIED: docs/parity/production-claim-boundary.md] |
| SUP-02 | Operators can identify support information expected for issue reports, including redacted bundles, logs, config summaries, service state, resource evidence, and sync evidence. [VERIFIED: .planning/REQUIREMENTS.md] | Reuse support-bundle/status snapshot fields and redaction rules from runtime and architecture docs; require explicit unavailable reasons instead of silent proof. [VERIFIED: docs/operator/runtime-guide.md; docs/architecture/status-snapshot.md] |
| SUP-03 | Contributors can update the matrix while preserving production-boundary and deferred-surface limits. [VERIFIED: .planning/REQUIREMENTS.md] | Add contributor update rules and a narrow deterministic checker that rejects unsupported support terms, prose-only promotions, missing evidence, and missing Phase 82 links. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts] |
| SUP-04 | Release reviewers can see residual risks and manual validation surfaces carried forward from v1.1 through v1.7. [VERIFIED: .planning/REQUIREMENTS.md] | Build a residual-risk/manual-validation table from milestone audits, release readiness, and milestone history; keep entries descriptive and gate-oriented. [VERIFIED: .planning/milestones/v1.1-MILESTONE-AUDIT.md; .planning/MILESTONES.md; .planning/milestones/v1.3-MILESTONE-AUDIT.md; .planning/milestones/v1.4-MILESTONE-AUDIT.md; docs/parity/release-readiness.md] |
</phase_requirements>

## Summary

Phase 83 should add a single canonical support-matrix reader path, preferably `docs/parity/support-matrix.md`, and link it from release-readiness, operator-runtime, parity index/checklist, and relevant catalog docs instead of scattering duplicate tables. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md; docs/parity/production-claim-boundary.md; docs/parity/README.md] The matrix should operationalize the Phase 82 terms `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred`, with no additional labels and no production full-node readiness claim. [VERIFIED: docs/parity/production-claim-boundary.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

The support matrix needs to classify environment families, not individual anecdotes: source-built installation, local runtime, public-network or regtest network mode, datadir/storage and resource-bound behavior, launchd/systemd service supervision, migration dry-run/apply boundaries, support-bundle/recovery surfaces, wallet/production-funds boundaries, and explicitly deferred production surfaces. [VERIFIED: .planning/REQUIREMENTS.md; docs/operator/runtime-guide.md; docs/parity/catalog/operator-runtime-release-hardening.md; docs/parity/catalog/drop-in-audit-and-migration.md; docs/parity/catalog/wallet.md] Each row should name evidence basis, default verification, opt-in UAT/manual validation, residual risk, and next gate so contributors cannot promote support through wording alone. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

Add a narrow Bun/TypeScript checker in Phase 83. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; scripts/check-phase82-production-claim-boundary.test.ts; scripts/verify.sh] The checker should validate exact support terms, required table sections, issue-evidence redaction boundaries, residual-risk entries, canonical links, and parity-index/checklist registration; it should not scan every doc for production claims because Phase 88 owns that broader scanner. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md; docs/parity/catalog/operator-runtime-release-hardening.md]

**Primary recommendation:** Create `docs/parity/support-matrix.md`, register it under `docs/parity/index.json` as a Phase 83 parity surface, add a focused checker/test pair under `scripts/`, wire those commands into `bash scripts/verify.sh` after the Phase 82 checker, and finish with the repo-native `bash scripts/verify.sh`. [VERIFIED: AGENTS.md; scripts/verify.sh; docs/parity/index.json; scripts/check-phase82-production-claim-boundary.ts]

## Project Constraints (from AGENTS.md)

- Use `AGENTS.md` as the repo-local instruction source and follow referenced Bright Builds standards. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards/index.md]
- Use `bash scripts/verify.sh` as the repo-native verification contract; `--fast` is only for local iteration. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts and prefer TypeScript for substantial script logic. [VERIFIED: AGENTS.md; standards/languages/typescript-javascript.md]
- During UAT documentation, provide repo-local Cargo and Bazel command forms, especially `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`. [VERIFIED: AGENTS.md]
- Preserve externally observable Bitcoin Knots `29.3.knots20260210` behavior for in-scope surfaces and keep parity evidence auditable through `docs/parity/`. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Keep production code free of existing Rust Bitcoin libraries and keep dependencies minimal. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Treat `docs/metrics/lines-of-code.md` as intentionally tracked generated output; expect it to change if verification regenerates it. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Do not add first-party Rust source or tests without adding required parity breadcrumbs; Phase 83 should not need Rust changes. [VERIFIED: AGENTS.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]
- Before substantial parity/operator/workflow changes, check whether relevant README files need pointer updates. [VERIFIED: AGENTS.md]
- Use `git submodule update --init --recursive` only if materializing the pinned Knots baseline is needed; Phase 83 documentation/checker work should not require it. [VERIFIED: AGENTS.md]

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|-------------------|---------|---------|--------------|
| Markdown under `docs/parity/` and `docs/operator/` | Repository docs, no package version [VERIFIED: docs/parity/README.md; docs/operator/runtime-guide.md] | Canonical human-readable support matrix, issue-evidence checklist, and residual-risk table. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] | Phase 82 and parity docs already use these roots for claim boundaries and release-review evidence. [VERIFIED: docs/parity/production-claim-boundary.md; docs/parity/release-readiness.md] |
| `docs/parity/index.json` plus `docs/parity/checklist.md` | Repository schema/status docs, no package version [VERIFIED: docs/parity/index.json; docs/parity/checklist.md] | Machine-readable and reviewer-readable registration for the Phase 83 support-matrix surface. [VERIFIED: docs/parity/README.md] | Phase 82 registered `v1-8-production-claim-boundary` through the parity index/checklist pattern. [VERIFIED: docs/parity/index.json; docs/parity/checklist.md; .planning/phases/82-production-claim-boundary/82-02-SUMMARY.md] |
| Bun / TypeScript checker | Bun `1.3.9` available locally [VERIFIED: `bun --version`] | Deterministic validation of support terms, tables, redaction boundaries, canonical links, and residual-risk entries. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts] | Repo instructions make Bun canonical for owned automation, and Phase 82 already established a checker/test fixture pattern. [VERIFIED: AGENTS.md; scripts/check-phase82-production-claim-boundary.test.ts] |
| `bash scripts/verify.sh` | GNU bash `3.2.57(1)-release` available locally [VERIFIED: `bash --version`; scripts/verify.sh] | Final repo-native verification contract and integration point for any Phase 83 checker. [VERIFIED: AGENTS.md; scripts/verify.sh] | The repo requires this script before marking work complete. [VERIFIED: AGENTS.md] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| Cargo / Rust | `cargo 1.94.1`, `rustc 1.94.1` available locally [VERIFIED: `cargo --version`; `rustc --version`; rust-toolchain.toml] | Full repo verification and repo-local UAT command examples. [VERIFIED: AGENTS.md; scripts/verify.sh] | Use in documented operator commands and final verification, not for new Phase 83 implementation unless checker/docs require no Rust edits. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] |
| Bazel / Bazelisk path | `bazel 8.6.0` available locally [VERIFIED: `bazel --version`] | Repo-local Bazel operator command examples and Bazel smoke build inside verifier. [VERIFIED: AGENTS.md; scripts/verify.sh] | Include alongside Cargo forms for UAT docs. [VERIFIED: AGENTS.md] |
| `cargo-llvm-cov` | `0.8.5` available locally [VERIFIED: `cargo llvm-cov --version`] | Coverage gate inside `scripts/verify.sh`. [VERIFIED: scripts/verify.sh] | Required by full verification, not by the Phase 83 checker itself. [VERIFIED: scripts/verify.sh] |
| `rg` / `jq` / `git` | `rg 15.1.0`, `jq 1.7.1-apple`, `git 2.53.0` available locally [VERIFIED: local version probes] | Focused research, diff review, and ad hoc local validation. [VERIFIED: local environment audit] | Use for planning and sanity checks; avoid encoding them as new project dependencies unless already used. [VERIFIED: standards/core/verification.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| New `docs/parity/support-matrix.md` | Extend `docs/parity/release-readiness.md` only | A single existing doc reduces files, but it makes the matrix harder to find and risks mixing support policy, release readiness, and historical risk into one dense page. [VERIFIED: docs/parity/release-readiness.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] |
| Bun/TypeScript narrow checker | Manual doc review only | Manual review is simpler in Phase 83, but a checker directly guards the highest-risk failure mode: accidental term drift or deferred-surface promotion. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] |
| Narrow checker | Broad all-doc support scanner | A broad scanner would better catch global prose drift, but Phase 88 explicitly owns broad production-scope scanning and Phase 83 should stay narrow. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md; docs/parity/catalog/operator-runtime-release-hardening.md] |
| Existing support terms | Add `best-effort` from requirement prose | The locked Phase 82 vocabulary forbids new labels; planner should map any `best-effort` requirement wording into `preview`, `opt-in UAT`, `unsupported`, or `deferred` as appropriate. [VERIFIED: .planning/REQUIREMENTS.md; docs/parity/production-claim-boundary.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] |

**Installation:**

No new packages should be installed for Phase 83. [VERIFIED: AGENTS.md; scripts/check-phase82-production-claim-boundary.ts] Use the checked-in toolchain and existing commands:

```bash
bun test scripts/check-phase83-support-matrix-issue-evidence.test.ts
bun run scripts/check-phase83-support-matrix-issue-evidence.ts
bash scripts/verify.sh
```

These command names are recommended if the checker is added. [VERIFIED: scripts/verify.sh; scripts/check-phase82-production-claim-boundary.test.ts]

**Version verification:** Local probes completed before writing this research. [VERIFIED: local environment audit]

```bash
bun --version                 # 1.3.9
bash --version | head -1      # GNU bash, version 3.2.57(1)-release
cargo --version               # cargo 1.94.1
rustc --version               # rustc 1.94.1
bazel --version               # bazel 8.6.0
cargo llvm-cov --version      # cargo-llvm-cov 0.8.5
```

## Architecture Patterns

### Recommended Project Structure

```text
docs/
├── parity/
│   ├── support-matrix.md              # Phase 83 canonical support matrix and issue evidence
│   ├── production-claim-boundary.md   # Phase 82 locked terms and deferred-surface boundary
│   ├── release-readiness.md           # Link to matrix plus reviewer-facing summary
│   ├── deviations-and-unknowns.md     # Residual-risk/deferred-surface cross-reference
│   ├── index.json                     # Register v1-8-support-matrix-issue-evidence
│   ├── checklist.md                   # Reviewer checklist row for the new surface
│   └── catalog/                       # Catalog pointers where support rows depend on surface evidence
├── operator/
│   └── runtime-guide.md               # Operator issue-evidence and UAT command pointer
scripts/
├── check-phase83-support-matrix-issue-evidence.ts
├── check-phase83-support-matrix-issue-evidence.test.ts
└── verify.sh
```

This structure keeps the support matrix in the parity/release-review path while linking operator workflow details from the runtime guide. [VERIFIED: docs/parity/README.md; docs/operator/runtime-guide.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

### Pattern 1: One Canonical Matrix With Phase 82 Terms

**What:** Create a single support matrix whose rows use only `supported`, `preview`, `opt-in UAT`, `unsupported`, or `deferred`, and whose columns include support term, evidence basis, default verification, opt-in UAT/manual validation, residual risk, and next gate. [VERIFIED: docs/parity/production-claim-boundary.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

**When to use:** Use for all SUP-01 classifications and for contributor updates under SUP-03. [VERIFIED: .planning/REQUIREMENTS.md]

**Planning detail:** Recommended row families are source-built install/toolchain, repo-local CLI/RPC/status runtime, regtest/local deterministic runtime, public-network mainnet activation/sync/soak, storage/datadir and resource-bound behavior, service supervision through launchd/systemd, migration dry-run/apply, support-bundle and support-forensics evidence, recovery behavior, wallet/non-production wallet boundaries, production funds, hosted dashboard/GUI, automatic support upload, destructive repair, and packaged/signed install. [VERIFIED: docs/operator/runtime-guide.md; docs/architecture/operator-observability.md; docs/parity/catalog/operator-runtime-release-hardening.md; docs/parity/catalog/drop-in-audit-and-migration.md; docs/parity/catalog/wallet.md]

### Pattern 2: Issue Evidence As a Redacted Checklist, Not a Collection Feature

**What:** Document expected issue evidence as a checklist of existing evidence surfaces: redacted support bundle, bounded logs or log paths, config summary, service state, resource bounds or pressure, sync status, command form, binary/version/build context, and explicit unavailable reasons. [VERIFIED: docs/operator/runtime-guide.md; docs/architecture/status-snapshot.md; docs/architecture/operator-observability.md]

**When to use:** Use for SUP-02 and link it from operator docs so issue reporters can gather evidence without exposing secrets. [VERIFIED: .planning/REQUIREMENTS.md; docs/operator/runtime-guide.md]

**Example:**

```markdown
### Issue Evidence Checklist

- Redacted support bundle: `support-evidence.json` and `support-evidence.md` from `support bundle`.
- Logs: bounded log path or relevant excerpt only; do not attach raw unbounded logs.
- Config: typed config summary or unavailable reason; do not include RPC cookies, `rpcpassword`, or `rpcauth`.
- Service: typed service state such as `running`, `failed`, or `unavailable-manager`.
- Resources: `resource_bounds` or `resource_pressure` fields when available.
- Sync: snapshot sync fields, support verdict, or explicit unavailable reason.
```

This example is derived from current runtime-guide support-bundle and redaction boundaries. [VERIFIED: docs/operator/runtime-guide.md; docs/architecture/status-snapshot.md]

### Pattern 3: Residual-Risk Register With Historical Source Links

**What:** Add a table that makes v1.1-v1.7 carried-forward risks visible without changing their blocker status. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

**When to use:** Use for SUP-04 and release-review closeout. [VERIFIED: .planning/REQUIREMENTS.md]

**Required rows:** v1.1 pseudoterminal/raw-input dashboard manual validation, v1.2 no dedicated milestone audit artifact, v1.3 diagnosed-blocker closeout, v1.4 planning traceability correction, and recurring public-network/service-manager/multi-day/support-bundle/recovery/production-scope non-claims. [VERIFIED: .planning/milestones/v1.1-MILESTONE-AUDIT.md; .planning/MILESTONES.md; .planning/milestones/v1.3-MILESTONE-AUDIT.md; .planning/milestones/v1.4-MILESTONE-AUDIT.md; docs/parity/release-readiness.md]

### Pattern 4: Narrow Deterministic Checker

**What:** Add a Phase 83 checker that reads only the known docs, parses the support matrix and residual-risk table, validates exact term allowlists, validates required evidence checklist/redaction language, and checks parity registration/linkage. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts]

**When to use:** Use if Phase 83 adds a canonical doc and parity metadata, which this research recommends. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

**Example checker skeleton:**

```typescript
const SUPPORT_TERMS = ["supported", "preview", "opt-in UAT", "unsupported", "deferred"] as const;

function assertOnlyKnownSupportTerms(rows: MatrixRow[]): void {
  for (const row of rows) {
    if (!SUPPORT_TERMS.includes(row.supportTerm as (typeof SUPPORT_TERMS)[number])) {
      fail(`Unsupported support term in row "${row.surface}": ${row.supportTerm}`);
    }
  }
}
```

This pattern mirrors the Phase 82 allowlist/checker style. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts]

### Anti-Patterns to Avoid

- **Adding `best-effort` as a sixth support level:** Requirement prose includes best-effort language, but Phase 83 context and Phase 82 lock the vocabulary to five exact terms. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md; docs/parity/production-claim-boundary.md]
- **Treating artifact existence as support proof:** Existing support docs state that bundle/report existence alone is not proof; typed verdict fields and evidence basis matter. [VERIFIED: docs/operator/runtime-guide.md; docs/architecture/operator-observability.md]
- **Making public-network, real service-manager, or multi-day checks default verification:** Existing docs keep those as opt-in UAT and Phase 83 context forbids moving them into default verification. [VERIFIED: docs/operator/runtime-guide.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]
- **Duplicating contradictory matrices across docs:** Use one canonical matrix and link from release-readiness/runtime/catalog docs. [VERIFIED: docs/parity/README.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]
- **Building a broad all-doc scanner now:** Phase 88 owns broad production-scope scanning. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Support taxonomy | New labels such as `best-effort`, `GA`, `certified`, or `production-ready` | Phase 82 terms only: `supported`, `preview`, `opt-in UAT`, `unsupported`, `deferred` | The production boundary already defines the vocabulary and Phase 83 must preserve it. [VERIFIED: docs/parity/production-claim-boundary.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] |
| Issue evidence collection | Automatic upload, raw datadir capture, wallet file capture, RPC credential capture, raw unbounded logs | Existing local redacted support bundle and typed status snapshot evidence | Runtime docs explicitly exclude secrets, wallet material, raw datadirs, raw unbounded logs, and automatic upload. [VERIFIED: docs/operator/runtime-guide.md] |
| Support-proof semantics | "File exists, therefore support claim is proven" | Typed support verdicts, snapshot fields, unavailable reasons, residual-risk statements | Existing docs require field-specific evidence and mark missing/unavailable data explicitly. [VERIFIED: docs/operator/runtime-guide.md; docs/architecture/status-snapshot.md] |
| Automation runtime | New Python or shell parser for substantial logic | Bun/TypeScript checker with fixture tests | Repo instructions make Bun canonical for substantial automation, and Phase 82 uses this exact pattern. [VERIFIED: AGENTS.md; scripts/check-phase82-production-claim-boundary.ts; scripts/check-phase82-production-claim-boundary.test.ts] |
| Production-scope enforcement | Broad all-doc scanner in Phase 83 | Narrow Phase 83 checker plus existing Phase 82 checker; leave broad scanner to Phase 88 | Phase 83 context explicitly defers broad scanner work. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] |
| Runtime validation | Live public-network, real service-manager mutation, multi-day soak, or large disk allocation in default verify | Deterministic doc/checker tests plus opt-in UAT command references | Existing release/operator docs keep these outside default verification. [VERIFIED: docs/operator/runtime-guide.md; docs/parity/release-readiness.md] |

**Key insight:** Phase 83 is a claim-boundary and evidence-routing phase, not a runtime feature phase; custom collection, broad scanning, or new labels would increase risk more than they improve support clarity. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md; docs/parity/production-claim-boundary.md]

## Common Pitfalls

### Pitfall 1: New Support Words Drift Into Docs

**What goes wrong:** A contributor adds `best-effort`, `validated`, `GA`, `certified`, or `production-ready` to describe a row. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md; docs/parity/production-claim-boundary.md]

**Why it happens:** SUP-01 requirement text mentions best-effort, while Phase 82 locked a different exact vocabulary. [VERIFIED: .planning/REQUIREMENTS.md; docs/parity/production-claim-boundary.md]

**How to avoid:** Use a checker allowlist for the five terms and explain that requirement prose maps into the locked terms. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts]

**Warning signs:** Matrix terms outside the five allowed strings or release-readiness prose that implies production full-node readiness. [VERIFIED: docs/parity/production-claim-boundary.md]

### Pitfall 2: Support Rows Lack Evidence Basis

**What goes wrong:** A row says a surface is supported or preview but does not name a verifier, doc source, UAT command, or residual risk. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

**Why it happens:** Support tables are easy to write as policy prose instead of auditable evidence. [VERIFIED: docs/parity/production-claim-boundary.md]

**How to avoid:** Require columns for evidence basis, default verification, opt-in UAT/manual validation, residual risk, and next gate. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

**Warning signs:** Empty cells, generic "tested" language, or no link to parity/operator docs. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts]

### Pitfall 3: Deferred Surfaces Get Promoted By Prose

**What goes wrong:** Packaging, GUI, hosted dashboards, automatic upload, destructive repair, migration apply mode, production funds, or broad production full-node readiness get described as supported. [VERIFIED: docs/operator/runtime-guide.md; docs/parity/production-claim-boundary.md; docs/parity/catalog/drop-in-audit-and-migration.md; docs/parity/catalog/wallet.md]

**Why it happens:** Support docs can accidentally conflate "documented limitation" with "supported surface." [VERIFIED: docs/parity/deviations-and-unknowns.md; docs/parity/release-readiness.md]

**How to avoid:** Put deferred/non-claim rows in the matrix and require next gates before promotion. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

**Warning signs:** Phrases like "production-ready full node" or "automatic support upload" outside deferred/unsupported contexts. [VERIFIED: docs/parity/production-claim-boundary.md]

### Pitfall 4: Issue Templates Ask For Sensitive Data

**What goes wrong:** Docs request wallet private material, raw wallet files, RPC cookies, `rpcpassword`, `rpcauth`, raw datadirs, or raw unbounded logs. [VERIFIED: docs/operator/runtime-guide.md]

**Why it happens:** Support workflows often ask for "everything" when exact evidence fields are not defined. [VERIFIED: docs/operator/runtime-guide.md; docs/architecture/status-snapshot.md]

**How to avoid:** Ask for the smallest useful redacted set and explicitly list forbidden evidence. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md; docs/operator/runtime-guide.md]

**Warning signs:** Instructions to attach the entire datadir, service logs without bounds, or credential-bearing config files. [VERIFIED: docs/operator/runtime-guide.md]

### Pitfall 5: Checker False Positives From `scripts/verify.sh` Heredoc Text

**What goes wrong:** A checker searches the entire verifier file and misreads legacy heredoc/sample command text as executed verification. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; scripts/verify.sh]

**Why it happens:** `scripts/verify.sh` contains both executable `run_step` calls and usage/help heredoc content. [VERIFIED: scripts/verify.sh]

**How to avoid:** Reuse the Phase 82 pattern that strips or isolates heredoc content before checking executable command order. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts]

**Warning signs:** Checker failures pointing at help text rather than actual `run_step` calls. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts]

### Pitfall 6: Historical Risks Are Omitted Because They Are Non-Blocking

**What goes wrong:** v1.1-v1.7 residual risks disappear from the support matrix because they are not current blockers. [VERIFIED: .planning/milestones/v1.1-MILESTONE-AUDIT.md; .planning/MILESTONES.md]

**Why it happens:** Release docs often optimize for the current phase and can hide carried-forward manual validation surfaces. [VERIFIED: .planning/MILESTONES.md; docs/parity/release-readiness.md]

**How to avoid:** Add a residual-risk table that distinguishes deterministic verification, opt-in UAT, manual validation, and deferred/non-claim. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

**Warning signs:** No mention of v1.1 pseudoterminal dashboard validation, v1.2 no dedicated audit artifact, v1.3 diagnosed-blocker closeout, or v1.4 traceability correction. [VERIFIED: .planning/milestones/v1.1-MILESTONE-AUDIT.md; .planning/MILESTONES.md; .planning/milestones/v1.3-MILESTONE-AUDIT.md; .planning/milestones/v1.4-MILESTONE-AUDIT.md]

## Code Examples

Verified patterns from repository sources:

### Support Term Allowlist

```typescript
const SUPPORT_TERMS = ["supported", "preview", "opt-in UAT", "unsupported", "deferred"] as const;
type SupportTerm = (typeof SUPPORT_TERMS)[number];

function isSupportTerm(value: string): value is SupportTerm {
  return SUPPORT_TERMS.includes(value as SupportTerm);
}
```

Source pattern: Phase 82 checker uses explicit allowlists for support terms and production-boundary claims. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts]

### Markdown Table Validation Shape

```typescript
type MatrixRow = {
  surface: string;
  supportTerm: string;
  evidenceBasis: string;
  defaultVerification: string;
  optInUat: string;
  residualRisk: string;
  nextGate: string;
};

function assertNoBlankMatrixCells(rows: MatrixRow[]): void {
  for (const row of rows) {
    for (const [field, value] of Object.entries(row)) {
      if (value.trim() === "") {
        fail(`Support matrix row "${row.surface}" has blank ${field}`);
      }
    }
  }
}
```

Source pattern: Phase 82 checker parses markdown tables and fails on missing required evidence/columns. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts]

### Fixture Test Mutation Pattern

```typescript
test("rejects a non-Phase-82 support term", async () => {
  const fixture = await makeFixture();
  await replaceInFile(
    fixture.path("docs/parity/support-matrix.md"),
    "| Source-built install | supported |",
    "| Source-built install | best-effort |",
  );

  expect(() => runPhase83Check(fixture.root)).toThrow(/best-effort/);
});
```

Source pattern: Phase 82 tests mutate temporary fixture files to assert targeted checker failures. [VERIFIED: scripts/check-phase82-production-claim-boundary.test.ts]

### Repo-Local UAT Command Forms For Docs

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support

bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support
```

Source: AGENTS requires repo-local Cargo and Bazel command forms, and the runtime guide already documents these support-bundle forms. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| General release-readiness prose for production boundaries | Dedicated production-claim boundary with exact support terms and deferred-surface inventory | Phase 82 [VERIFIED: .planning/phases/82-production-claim-boundary/82-01-SUMMARY.md; docs/parity/production-claim-boundary.md] | Phase 83 must operationalize existing terms instead of redefining support. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] |
| Artifact presence as evidence | Typed support verdicts, status snapshot fields, unavailable reasons, and local redacted bundle summaries | Phases 72, 79, and 82 [VERIFIED: docs/operator/runtime-guide.md; docs/architecture/status-snapshot.md; docs/parity/production-claim-boundary.md] | Support matrix rows should cite evidence fields and limitations, not only artifact paths. [VERIFIED: docs/operator/runtime-guide.md] |
| Public-network and real service-manager checks as tempting verification targets | Opt-in UAT surfaces outside default verification | Phases 73, 80, and 82 [VERIFIED: docs/operator/runtime-guide.md; docs/parity/release-readiness.md; docs/parity/catalog/operator-runtime-release-hardening.md] | Phase 83 checker and docs must keep default verification public-network-free and service-manager-free. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] |
| Broad production scanner in current phase | Narrow Phase 83 checker plus future Phase 88 broad scanner | Phase 83 context [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] | Plan a focused checker only. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] |
| ASVS 4.x category IDs as stable shorthand | ASVS latest stable version 5.0.0, with version-prefixed requirement identifiers recommended | ASVS 5.0.0 stable release in May 2025 [CITED: https://github.com/OWASP/ASVS; https://owasp.org/www-project-application-security-verification-standard/] | Security notes should avoid over-specific ASVS IDs unless versioned. [CITED: https://github.com/OWASP/ASVS] |

**Deprecated/outdated:**

- Treating support-bundle existence as proof is outdated for this repo; existing docs require typed verdicts and field-specific evidence. [VERIFIED: docs/operator/runtime-guide.md]
- Treating public-network live sync as release-blocking default verification is explicitly out of scope for current docs. [VERIFIED: docs/operator/runtime-guide.md; docs/parity/release-readiness.md]
- Adding a new support term for "best effort" would contradict Phase 82 and Phase 83 decisions. [VERIFIED: docs/parity/production-claim-boundary.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

## Concrete Files Likely To Change

| File | Expected Change | Why |
|------|-----------------|-----|
| `docs/parity/support-matrix.md` | New canonical support matrix, issue evidence checklist, contributor update rules, residual-risk table. | Directly satisfies SUP-01 through SUP-04. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] |
| `docs/parity/index.json` | Add `v1-8-support-matrix-issue-evidence` surface with requirements SUP-01 through SUP-04 and evidence paths. | Keeps parity roots auditable. [VERIFIED: docs/parity/index.json; docs/parity/README.md] |
| `docs/parity/checklist.md` | Add reviewer checklist row for the new surface. | Checklist already tracks Phase 82 parity/release surfaces. [VERIFIED: docs/parity/checklist.md] |
| `docs/parity/README.md` | Add link/description for the support matrix. | README is the parity ledger entrypoint. [VERIFIED: docs/parity/README.md] |
| `docs/parity/release-readiness.md` | Link to support matrix and summarize reviewer residual-risk path without duplicating the table. | Release reviewers already use this doc for readiness evidence. [VERIFIED: docs/parity/release-readiness.md] |
| `docs/parity/deviations-and-unknowns.md` | Link support-matrix residual risks and deferred/non-claim handling. | Existing deviations doc owns unknowns/deferred limits. [VERIFIED: docs/parity/deviations-and-unknowns.md] |
| `docs/operator/runtime-guide.md` | Add or adjust pointer to issue evidence checklist and repo-local UAT command usage. | Operators use this doc for support bundles, service, sync, and UAT flows. [VERIFIED: docs/operator/runtime-guide.md; AGENTS.md] |
| `docs/parity/catalog/operator-runtime-release-hardening.md` | Add Phase 83 support-matrix evidence note. | Catalog already tracks Phase 72/73/79/80/82 support and boundary evidence. [VERIFIED: docs/parity/catalog/operator-runtime-release-hardening.md] |
| `docs/parity/catalog/p2p.md`, `docs/parity/catalog/chainstate.md`, `docs/parity/catalog/wallet.md`, `docs/parity/catalog/drop-in-audit-and-migration.md` | Add targeted pointers only where support rows depend on those deferred or preview surfaces. | These catalogs contain current deferred/support-limiting facts. [VERIFIED: docs/parity/catalog/p2p.md; docs/parity/catalog/chainstate.md; docs/parity/catalog/wallet.md; docs/parity/catalog/drop-in-audit-and-migration.md] |
| `README.md` | Optional light pointer to support matrix if user-facing support wording changes. | Repo guidance says check README after parity/operator workflow changes. [VERIFIED: AGENTS.md; README.md] |
| `scripts/check-phase83-support-matrix-issue-evidence.ts` | New narrow deterministic checker. | Recommended to guard Phase 83 drift. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts] |
| `scripts/check-phase83-support-matrix-issue-evidence.test.ts` | Fixture tests for checker failures. | Phase 82 uses test coverage for checker behavior. [VERIFIED: scripts/check-phase82-production-claim-boundary.test.ts] |
| `scripts/verify.sh` | Add `bun test` and `bun run` steps after Phase 82 checker. | Final verification contract should include deterministic checker if added. [VERIFIED: scripts/verify.sh] |
| `docs/metrics/lines-of-code.md` | May update if verifier regenerates tracked LOC metrics. | Repo treats this as tracked generated output. [VERIFIED: AGENTS.md; scripts/verify.sh] |

## Support Matrix Row Recommendations

| Environment Family | Recommended Term | Evidence Basis | Default Verification | Opt-In / Manual Surface | Next Gate |
|--------------------|------------------|----------------|----------------------|-------------------------|-----------|
| Source-built repo checkout, toolchain, hooks, first-party verifier | `supported` | Rust/Bun/Bazel toolchain docs and `scripts/verify.sh` contract. [VERIFIED: AGENTS.md; rust-toolchain.toml; scripts/verify.sh] | `bash scripts/verify.sh` and focused checker/test commands. [VERIFIED: scripts/verify.sh] | Local environment installation differences remain operator-managed. [VERIFIED: AGENTS.md] | Keep verifier green across release. [VERIFIED: AGENTS.md] |
| Repo-local operator command forms through Cargo and Bazel | `supported` for documented source-built UAT forms | AGENTS and runtime guide command examples. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md] | Deterministic docs/checker validation of examples. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts] | Actual public-network outcomes remain opt-in UAT. [VERIFIED: docs/operator/runtime-guide.md] | Keep Cargo and Bazel examples synchronized. [VERIFIED: AGENTS.md] |
| Local deterministic status/config/RPC/support-bundle surfaces | `supported` or `preview` per existing evidence; do not overclaim production readiness | Status snapshot, runtime guide, and support-bundle docs. [VERIFIED: docs/architecture/status-snapshot.md; docs/operator/runtime-guide.md] | Existing unit/integration verifier plus Phase 83 doc checker. [VERIFIED: scripts/verify.sh] | Missing live fields should be explicit unavailable reasons. [VERIFIED: docs/architecture/status-snapshot.md] | Promote only with concrete verifier evidence. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] |
| Public-network mainnet activation, full-sync, stay-current, soak | `opt-in UAT` | Runtime guide Phase 73 and Phase 80 UAT matrices. [VERIFIED: docs/operator/runtime-guide.md] | Not default verification. [VERIFIED: docs/operator/runtime-guide.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] | Operator-run public-network evidence and bounded reports. [VERIFIED: docs/operator/runtime-guide.md] | Future field evidence before support promotion. [VERIFIED: docs/parity/release-readiness.md] |
| launchd/systemd user service supervision | `preview` for side-effect-free previews; `opt-in UAT` for real service-manager lifecycle | Runtime guide service integration section. [VERIFIED: docs/operator/runtime-guide.md] | Deterministic config/docs tests only. [VERIFIED: scripts/verify.sh] | Real service-manager start/stop/restart/status review remains opt-in. [VERIFIED: docs/operator/runtime-guide.md] | More platform validation before broader support. [VERIFIED: docs/operator/runtime-guide.md] |
| Storage/datadir resource bounds and recovery evidence | `preview` or `opt-in UAT` depending on row; destructive repair is `deferred` | Operator observability resource/recovery evidence and runtime guide. [VERIFIED: docs/architecture/operator-observability.md; docs/operator/runtime-guide.md] | Deterministic tests for typed evidence and docs. [VERIFIED: scripts/verify.sh] | Large disk, live recovery, and long-running stress remain opt-in/out of default. [VERIFIED: docs/operator/runtime-guide.md] | Field evidence and recovery gates before support promotion. [VERIFIED: docs/parity/release-readiness.md] |
| Migration dry-run | `supported` or `preview` only for dry-run/read-only behavior, based on existing docs | Drop-in audit and migration catalog. [VERIFIED: docs/parity/catalog/drop-in-audit-and-migration.md] | Deterministic migration dry-run checks where already in verifier. [VERIFIED: scripts/verify.sh] | Source service mutation and apply mode not included. [VERIFIED: docs/parity/catalog/drop-in-audit-and-migration.md] | Separate future apply-mode plan. [VERIFIED: docs/parity/catalog/drop-in-audit-and-migration.md] |
| Migration apply, source service mutation, source datadir rewrite | `deferred` | Migration docs explicitly keep apply/mutation out of scope. [VERIFIED: docs/parity/catalog/drop-in-audit-and-migration.md] | None. [VERIFIED: docs/parity/catalog/drop-in-audit-and-migration.md] | None in v1.8. [VERIFIED: docs/parity/catalog/drop-in-audit-and-migration.md] | Future phase only. [VERIFIED: docs/parity/catalog/drop-in-audit-and-migration.md] |
| Wallet production funds and advanced wallet surfaces | `deferred` for production funds/advanced gaps; `preview` only for documented non-production wallet slice if included | Wallet catalog lists current gaps and production-funds limits. [VERIFIED: docs/parity/catalog/wallet.md] | Existing wallet deterministic tests only. [VERIFIED: scripts/verify.sh; docs/parity/catalog/wallet.md] | No production funds support. [VERIFIED: docs/parity/catalog/wallet.md; docs/operator/runtime-guide.md] | Future wallet phases. [VERIFIED: docs/parity/catalog/wallet.md] |
| Packaged/signed install, GUI, hosted dashboard, automatic upload, destructive repair, broad production full-node readiness | `deferred` | Runtime guide and Phase 82 boundary list these as limitations/non-claims. [VERIFIED: docs/operator/runtime-guide.md; docs/parity/production-claim-boundary.md] | None. [VERIFIED: docs/operator/runtime-guide.md] | None in v1.8. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] | Future scoped phases only. [VERIFIED: docs/operator/runtime-guide.md] |

These are planning recommendations, not final support claims; implementation should cite the exact docs/evidence row by row. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| None | All factual claims in this research are tied to local verified files, local version probes, or cited OWASP ASVS sources. | All sections | No user confirmation needed for factual claims; final support-row wording still needs implementation review. [VERIFIED: local sources listed in Sources] |

## Open Questions (RESOLVED)

1. **Should `docs/parity/support-matrix.md` be a new parity surface ID?** [VERIFIED: docs/parity/index.json]
   - What we know: Phase 82 registered production-claim boundary evidence in `docs/parity/index.json` and `docs/parity/checklist.md`. [VERIFIED: docs/parity/index.json; docs/parity/checklist.md]
   - What was unclear before resolution: The exact surface ID name is not locked. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]
   - Recommendation: Use `v1-8-support-matrix-issue-evidence` with requirements SUP-01 through SUP-04. [VERIFIED: .planning/REQUIREMENTS.md; docs/parity/index.json]
   - RESOLVED: Accept the recommendation to use `v1-8-support-matrix-issue-evidence` with requirements SUP-01 through SUP-04. [VERIFIED: .planning/REQUIREMENTS.md; docs/parity/index.json]

2. **How much should `README.md` change?** [VERIFIED: README.md; AGENTS.md]
   - What we know: Repo instructions say to check README files after substantial parity/operator workflow changes. [VERIFIED: AGENTS.md]
   - What was unclear before resolution: The support matrix may be more reviewer/operator-facing than top-level README-facing. [VERIFIED: README.md; docs/parity/README.md]
   - Recommendation: Add only a lightweight pointer if top-level support wording already exists or would otherwise become misleading. [VERIFIED: AGENTS.md; README.md]
   - RESOLVED: Accept the recommendation to add only a lightweight pointer if top-level support wording already exists or would otherwise become misleading. [VERIFIED: AGENTS.md; README.md]

3. **Is the checker required for Phase 83?** [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]
   - What we know: Automation is optional by context, but the highest-risk Phase 83 failure modes are deterministic text/metadata drift. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md; scripts/check-phase82-production-claim-boundary.ts]
   - What was unclear before resolution: Planner may choose doc-only implementation if scope must stay smaller. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]
   - Recommendation: Add the narrow checker because it is short-running, follows Phase 82 precedent, and directly protects SUP-03. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; .planning/REQUIREMENTS.md]
   - RESOLVED: Accept the recommendation to add the narrow checker because it is short-running, follows Phase 82 precedent, and directly protects SUP-03. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; .planning/REQUIREMENTS.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Bun | Phase 83 checker/test and repo automation | Yes [VERIFIED: `bun --version`] | `1.3.9` [VERIFIED: `bun --version`] | Manual review only, but not recommended. [VERIFIED: AGENTS.md] |
| Bash | `scripts/verify.sh` | Yes [VERIFIED: `bash --version`] | GNU bash `3.2.57(1)-release` [VERIFIED: `bash --version`] | None needed. [VERIFIED: scripts/verify.sh] |
| Cargo | Full verify and UAT command docs | Yes [VERIFIED: `cargo --version`] | `1.94.1` [VERIFIED: `cargo --version`; rust-toolchain.toml] | None for final verify. [VERIFIED: AGENTS.md] |
| Rust compiler | Full verify | Yes [VERIFIED: `rustc --version`] | `1.94.1` [VERIFIED: `rustc --version`; rust-toolchain.toml] | None for final verify. [VERIFIED: AGENTS.md] |
| Bazel | Bazel smoke build and UAT command docs | Yes [VERIFIED: `bazel --version`] | `8.6.0` [VERIFIED: `bazel --version`] | Cargo command examples still available, but repo requires Bazel smoke in verifier. [VERIFIED: AGENTS.md; scripts/verify.sh] |
| `cargo-llvm-cov` | Coverage section of full verifier | Yes [VERIFIED: `cargo llvm-cov --version`] | `0.8.5` [VERIFIED: `cargo llvm-cov --version`] | None for full verifier. [VERIFIED: scripts/verify.sh] |
| `git` | Diff review and repository operations | Yes [VERIFIED: `git --version`] | `2.53.0` [VERIFIED: `git --version`] | None needed. [VERIFIED: local environment audit] |
| `rg` | Research and focused local scans | Yes [VERIFIED: `rg --version`] | `15.1.0` [VERIFIED: `rg --version`] | `grep`, but `rg` is available. [VERIFIED: local environment audit] |
| `jq` | JSON inspection if needed | Yes [VERIFIED: `jq --version`] | `1.7.1-apple` [VERIFIED: `jq --version`] | Bun/Node JSON parsing. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts] |

**Missing dependencies with no fallback:** None found for Phase 83 research and planned docs/checker work. [VERIFIED: local environment audit]

**Missing dependencies with fallback:** None found. [VERIFIED: local environment audit]

## Security Domain

Security enforcement is enabled because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

OWASP ASVS is the official application security verification standard, and the current stable ASVS release is 5.0.0 from May 2025; ASVS identifiers are version-sensitive, so use versioned references if exact requirements are later cited. [CITED: https://owasp.org/www-project-application-security-verification-standard/; https://github.com/OWASP/ASVS]

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | No new auth implementation [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] | Do not request RPC credentials, cookies, `rpcpassword`, `rpcauth`, wallet secrets, or private keys as issue evidence. [VERIFIED: docs/operator/runtime-guide.md] |
| V3 Session Management | No hosted session surface in Phase 83 [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] | Keep hosted dashboard and automatic upload deferred. [VERIFIED: docs/operator/runtime-guide.md; docs/parity/production-claim-boundary.md] |
| V4 Access Control | No new access-control implementation [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] | Do not broaden local support evidence into hosted/public support collection. [VERIFIED: docs/operator/runtime-guide.md] |
| V5 Input Validation | Yes, for checker parsing of local markdown/JSON docs [VERIFIED: scripts/check-phase82-production-claim-boundary.ts] | Use exact allowlists, JSON parsing, required-column validation, and fail-closed checks for required terms/links. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts] |
| V6 Cryptography | No new cryptography [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md] | Do not imply checkpoint chains or support bundles are cryptographic authenticity proofs unless existing docs say so. [VERIFIED: docs/architecture/operator-observability.md; docs/operator/runtime-guide.md] |

### Known Threat Patterns for Phase 83

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Support request discloses secrets or wallet material | Information Disclosure | Issue-evidence checklist must forbid private keys, raw wallet files, RPC cookies, `rpcpassword`, `rpcauth`, raw datadirs, and raw unbounded logs. [VERIFIED: docs/operator/runtime-guide.md] |
| Prose broadens production support beyond evidence | Elevation of Privilege / Tampering with policy boundary | Checker allowlists exact support terms and validates deferred-surface rows plus Phase 82 links. [VERIFIED: docs/parity/production-claim-boundary.md; scripts/check-phase82-production-claim-boundary.ts] |
| Operator treats incomplete support bundle as proof | Repudiation / Tampering | Require typed verdicts, unavailable reasons, and evidence basis in matrix/checklist. [VERIFIED: docs/operator/runtime-guide.md; docs/architecture/status-snapshot.md] |
| Default verifier becomes flaky or costly | Denial of Service | Keep Phase 83 checks deterministic, local, public-network-free, service-manager-free, multi-day-free, and large-disk-free. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md; docs/operator/runtime-guide.md] |
| Checker accepts malformed docs | Tampering | Parse JSON structurally, validate markdown table headers/columns, and fail on blank cells or unknown terms. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts] |

## Verification Recommendations

Use focused checks during implementation and full repo verification before handoff. [VERIFIED: AGENTS.md; scripts/verify.sh]

1. Run the new checker test if added: `bun test scripts/check-phase83-support-matrix-issue-evidence.test.ts`. [VERIFIED: scripts/check-phase82-production-claim-boundary.test.ts]
2. Run the new checker if added: `bun run scripts/check-phase83-support-matrix-issue-evidence.ts`. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts]
3. Run the existing Phase 82 checker/test to ensure Phase 83 did not weaken the production boundary: `bun test scripts/check-phase82-production-claim-boundary.test.ts` and `bun run scripts/check-phase82-production-claim-boundary.ts`. [VERIFIED: scripts/verify.sh]
4. Run documentation-focused scans for forbidden support drift, such as `rg "best-effort|production-ready|GA|certified|validated|fully supported" docs README.md`. [VERIFIED: docs/parity/production-claim-boundary.md; .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]
5. Run final repo-native verification: `bash scripts/verify.sh`. [VERIFIED: AGENTS.md; scripts/verify.sh]

Do not add public-network, real service-manager, multi-day soak, large-disk allocation, source datadir mutation, or support-bundle upload to default verification. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md; docs/operator/runtime-guide.md]

## Sources

### Primary (HIGH confidence)

- `AGENTS.md` - repo-local instructions, Bright Builds routing, verification contract, Bun automation, parity docs, UAT command forms. [VERIFIED: AGENTS.md]
- `AGENTS.bright-builds.md` and `standards/` - managed workflow, architecture, code shape, verification, testing, Rust, and TypeScript guidance. [VERIFIED: AGENTS.bright-builds.md; standards/index.md; standards/core/architecture.md; standards/core/code-shape.md; standards/core/verification.md; standards/core/testing.md; standards/languages/rust.md; standards/languages/typescript-javascript.md]
- `.planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md` - locked Phase 83 decisions, discretion, and out-of-scope items. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]
- `.planning/REQUIREMENTS.md` - SUP-01 through SUP-04. [VERIFIED: .planning/REQUIREMENTS.md]
- `.planning/ROADMAP.md` - Phase 83 scope, dependency, and success criteria. [VERIFIED: .planning/ROADMAP.md]
- `.planning/STATE.md` - current project state and recent phase history. [VERIFIED: .planning/STATE.md]
- `.planning/phases/82-production-claim-boundary/82-CONTEXT.md` and `82-*-SUMMARY.md` - Phase 82 locked boundary and built artifacts. [VERIFIED: .planning/phases/82-production-claim-boundary/82-CONTEXT.md; .planning/phases/82-production-claim-boundary/82-01-SUMMARY.md; .planning/phases/82-production-claim-boundary/82-02-SUMMARY.md; .planning/phases/82-production-claim-boundary/82-03-SUMMARY.md; .planning/phases/82-production-claim-boundary/82-04-SUMMARY.md]
- `docs/parity/production-claim-boundary.md` - exact support terms, deferred surfaces, and production claim boundary. [VERIFIED: docs/parity/production-claim-boundary.md]
- `docs/operator/runtime-guide.md` - support bundles, redaction boundaries, service supervision, opt-in UAT, and limitations. [VERIFIED: docs/operator/runtime-guide.md]
- `docs/architecture/status-snapshot.md` and `docs/architecture/operator-observability.md` - typed status fields, unavailable reasons, resource/recovery/support forensics evidence. [VERIFIED: docs/architecture/status-snapshot.md; docs/architecture/operator-observability.md]
- `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md` - parity/release-review evidence roots. [VERIFIED: docs/parity/release-readiness.md; docs/parity/deviations-and-unknowns.md; docs/parity/index.json; docs/parity/checklist.md; docs/parity/README.md]
- `docs/parity/catalog/operator-runtime-release-hardening.md`, `p2p.md`, `chainstate.md`, `wallet.md`, `drop-in-audit-and-migration.md` - cataloged support, deferred, preview, and non-claim surfaces. [VERIFIED: docs/parity/catalog/operator-runtime-release-hardening.md; docs/parity/catalog/p2p.md; docs/parity/catalog/chainstate.md; docs/parity/catalog/wallet.md; docs/parity/catalog/drop-in-audit-and-migration.md]
- `.planning/milestones/v1.1-MILESTONE-AUDIT.md`, `.planning/MILESTONES.md`, `.planning/milestones/v1.3-MILESTONE-AUDIT.md`, `.planning/milestones/v1.4-MILESTONE-AUDIT.md`, `.planning/RETROSPECTIVE.md` - v1.1-v1.7 residual-risk history. [VERIFIED: local milestone docs]
- `scripts/check-phase82-production-claim-boundary.ts`, `scripts/check-phase82-production-claim-boundary.test.ts`, and `scripts/verify.sh` - checker architecture and verifier integration pattern. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; scripts/check-phase82-production-claim-boundary.test.ts; scripts/verify.sh]

### Secondary (MEDIUM confidence)

- OWASP ASVS project page - ASVS purpose and official project location. [CITED: https://owasp.org/www-project-application-security-verification-standard/]
- OWASP ASVS GitHub repository - ASVS 5.0.0 stable release date and identifier-versioning guidance. [CITED: https://github.com/OWASP/ASVS]

### Tertiary (LOW confidence)

- None. [VERIFIED: this research used local project docs and official OWASP sources]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - repo instructions, existing Phase 82 checker, and local tool probes agree. [VERIFIED: AGENTS.md; scripts/check-phase82-production-claim-boundary.ts; local environment audit]
- Architecture: HIGH - Phase 83 context locks the scope, and Phase 82 artifacts define the support vocabulary and boundary. [VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md; docs/parity/production-claim-boundary.md]
- Pitfalls: HIGH - pitfalls are derived from explicit locked constraints, runtime redaction rules, Phase 82 checker behavior, and milestone audit records. [VERIFIED: docs/operator/runtime-guide.md; scripts/check-phase82-production-claim-boundary.ts; local milestone docs]
- Security: MEDIUM - Phase 83 is docs/checker-only, so ASVS mapping is scoped to redaction, support evidence, and local parser controls rather than application runtime controls. [CITED: https://github.com/OWASP/ASVS; VERIFIED: .planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md]

**Research date:** 2026-06-21 [VERIFIED: environment current_date]
**Valid until:** 2026-07-21 for local project guidance, or sooner if Phase 82/83 boundary docs or OWASP ASVS version change. [VERIFIED: local source timestamps not revalidated beyond current session; CITED: https://github.com/OWASP/ASVS]
