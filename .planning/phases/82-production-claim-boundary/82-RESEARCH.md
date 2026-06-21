# Phase 82: Production Claim Boundary - Research

**Researched:** 2026-06-21  
**Domain:** Production-readiness claim boundaries, parity documentation, evidence gates, deterministic traceability  
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

Source for all copied bullets in this section: [VERIFIED: .planning/phases/82-production-claim-boundary/82-CONTEXT.md]

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)

- Phase 88 deterministic broad-claim scanner and default-verification guardrail
  suite.
- Future production full-node readiness claim after all gates pass.
- Inbound serving, address relay, block serving, transaction relay, compact
  block relay, production-funds wallet use or safety, migration apply mode,
  signed packaging or package-manager distribution, Windows service integration,
  hosted dashboards, GUI parity, public-network CI, release-blocking live sync,
  destructive repair, and automatic support-bundle upload.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PROD-01 | Operator can read a production full-node readiness definition that separates supported, preview, opt-in UAT, unsupported, and deferred surfaces. [VERIFIED: .planning/REQUIREMENTS.md] | Put a controlled five-term glossary in one canonical `docs/parity/` boundary document and link it from entrypoints instead of scattering competing prose. [VERIFIED: 82-CONTEXT.md; docs/parity/README.md; docs/parity/release-readiness.md] |
| PROD-02 | Release reviewer can trace each allowed production-related statement to an explicit evidence gate, current status, and verification source. [VERIFIED: .planning/REQUIREMENTS.md] | Add a claim-to-evidence matrix with support term, status, evidence, verification command, UAT status, residual risk, and next gate columns. [VERIFIED: 82-CONTEXT.md; docs/parity/release-readiness.md] |
| PROD-03 | Contributor can tell which evidence is required before Open Bitcoin may claim production full-node readiness. [VERIFIED: .planning/REQUIREMENTS.md] | Include "not allowed yet" rows and a future production-readiness gate set that requires all named deferred surfaces to have scoped milestones, evidence, and support term changes before the claim is allowed. [VERIFIED: 82-CONTEXT.md; .planning/PROJECT.md] |
| PROD-04 | Operator-facing docs explicitly preserve deferred status for inbound serving, relay, production-funds wallet use, migration apply mode, signed packaging, hosted dashboards, GUI parity, public-network CI, destructive repair, and automatic support-bundle upload. [VERIFIED: .planning/REQUIREMENTS.md] | Extend `docs/parity/deviations-and-unknowns.md` and link the same inventory from release-readiness, README, runtime guide, checklist, parity README, and parity index. [VERIFIED: 82-CONTEXT.md; docs/parity/deviations-and-unknowns.md; README.md] |
</phase_requirements>

## Summary

Phase 82 should be planned as a documentation and traceability boundary, not as a production capability expansion. [VERIFIED: 82-CONTEXT.md; .planning/ROADMAP.md] The safest plan is one canonical `docs/parity/production-claim-boundary.md` document, one support-term glossary, one claim-to-evidence matrix, one deferred-surface inventory, and link updates through the existing README, runtime guide, release-readiness, checklist, parity README, parity index, and deviations register. [VERIFIED: 82-CONTEXT.md; README.md; docs/parity/README.md; docs/parity/index.json]

The current repo already has the release-boundary pattern Phase 82 needs: v1.3 through v1.7 sections preserve historical claims, current docs avoid public-network default gates, and Phase 80 shows the narrow Bun checker pattern when traceability needs automation. [VERIFIED: docs/parity/release-readiness.md; docs/parity/deviations-and-unknowns.md; scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts] External release-readiness references support this shape: production responsibility follows explicit readiness review and support acceptance, maturity claims should be evidence-backed, and Bitcoin release processes keep release notes, candidate testing, signing, and maintenance support separate from broad claims. [CITED: https://sre.google/sre-book/evolving-sre-engagement-model/; https://contribute.cncf.io/projects/lifecycle/; https://github.com/bitcoin/bitcoin/blob/master/doc/release-process.md; https://bitcoincore.org/en/lifecycle/]

**Primary recommendation:** Use docs and parity metadata first; add a Phase 82 Bun checker only if planning identifies a narrow consistency check that materially improves PROD-02 or PROD-03 without duplicating Phase 88. [VERIFIED: 82-CONTEXT.md; scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts]

## Project Constraints (from AGENTS.md)

- Use `AGENTS.md` as the repo-local instruction entrypoint, then `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant `standards/` pages. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards/index.md]
- Use `git submodule update --init --recursive` to materialize the pinned Knots baseline when needed. [VERIFIED: AGENTS.md]
- Treat `rust-toolchain.toml` as the Rust source of truth; the pinned local toolchain is Rust `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml; rustc --version]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including the Bazel smoke build. [VERIFIED: AGENTS.md; scripts/verify.sh]
- During UAT, provide repo-local Cargo and Bazel commands, preferring `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts; prefer TypeScript for substantial script logic and Bash for thin wrappers. [VERIFIED: AGENTS.md; .planning/STACK.md]
- Do not add a `bun install` bootstrap step while this repo has no `package.json`. [VERIFIED: .planning/STACK.md; local package-manager file probe]
- `bash scripts/install-git-hooks.sh` installs repo-managed hooks, and `bash scripts/verify.sh` self-heals missing local hooks outside CI. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Treat `docs/metrics/lines-of-code.md` as tracked generated output that may change when hooks or verification refresh it. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Record intentional in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md; docs/parity/index.json]
- If adding first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update `docs/parity/source-breadcrumbs.json` and keep `scripts/check-parity-breadcrumbs.ts --check` green. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts]
- After substantial feature, parity, operator-surface, or workflow changes, check whether relevant README files need updates. [VERIFIED: AGENTS.md]
- Follow functional-core / imperative-shell boundaries and keep pure business logic free of direct I/O and runtime effects. [VERIFIED: AGENTS.md; standards/core/architecture.md; .planning/ARCHITECTURE.md]
- Keep tests focused and use Arrange, Act, Assert comments for non-trivial unit tests. [VERIFIED: AGENTS.md; standards/core/testing.md]
- For TypeScript/Bun automation, use `maybe...` naming for nullable values and avoid class inheritance in repo-owned behavior. [VERIFIED: standards/languages/typescript-javascript.md]
- No project-local `.claude/skills/` or `.agents/skills/` directories were found. [VERIFIED: local project skill probe]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|---|---:|---|---|
| Markdown docs under `docs/parity/` | n/a | Canonical production-boundary document, release-readiness section, deviations register, parity README, and checklist links. [VERIFIED: 82-CONTEXT.md; docs/parity/README.md] | Existing release-boundary phases preserve historical claims in docs/parity and use these files as reviewer roots. [VERIFIED: docs/parity/release-readiness.md; docs/parity/checklist.md] |
| `docs/parity/index.json` | repo schema, no standalone version | Machine-readable parity root for the new v1.8 production boundary surface and audit entry. [VERIFIED: docs/parity/index.json; 82-CONTEXT.md] | The repo already uses top-level surfaces, checklist surfaces, and audit entries for v1.3 through v1.7 boundary evidence. [VERIFIED: docs/parity/index.json] |
| Bun / TypeScript in `scripts/` | Bun 1.3.9 | Optional narrow checker and fixture tests if docs plus parity metadata are insufficient. [VERIFIED: .bun-version; bun --version; 82-CONTEXT.md] | Existing release-boundary checkers use Bun/TypeScript and are wired through `scripts/verify.sh`. [VERIFIED: scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts; scripts/verify.sh] |
| Bash `scripts/verify.sh` | Bash 3.2.57 available | Final phase verification and aggregate repo contract. [VERIFIED: bash --version; scripts/verify.sh; AGENTS.md] | Repo-local guidance and Bright Builds verification rules prefer the repo-owned aggregate verifier. [VERIFIED: AGENTS.md; standards/core/verification.md] |
| Cargo/Rust | Rust 1.94.1 | Existing verifier surface and conditional Rust change path only. [VERIFIED: rust-toolchain.toml; cargo --version; rustc --version] | Phase 82 expects no Rust source changes, but full verification still builds and tests Rust. [VERIFIED: 82-CONTEXT.md; scripts/verify.sh] |
| Bazel | 8.6.0 | Existing smoke build and required UAT command form when docs show CLI-backed workflows. [VERIFIED: bazel --version; scripts/verify.sh; AGENTS.md] | AGENTS requires repo-local Bazel command forms for UAT examples. [VERIFIED: AGENTS.md] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|---|---:|---|---|
| `jq` | 1.7.1 | Focused JSON sanity checks over `docs/parity/index.json`. [VERIFIED: jq --version; docs/parity/index.json] | Use during iteration if parity JSON is edited. [VERIFIED: 82-CONTEXT.md] |
| `rg` | 15.1.0 | Focused scans for support terms, deferred surfaces, and stale v1.7-current wording. [VERIFIED: rg --version; local scans] | Use during iteration; avoid broad production-word failure gates. [VERIFIED: 82-CONTEXT.md; docs/parity/release-readiness.md] |
| `cargo-llvm-cov` | 0.8.5 | Full `scripts/verify.sh` coverage gate. [VERIFIED: cargo llvm-cov --version; scripts/verify.sh] | Required by full verifier, not a Phase 82 implementation dependency. [VERIFIED: scripts/verify.sh] |
| `git` | 2.53.0 | Source breadcrumb and tracked-file verification support. [VERIFIED: git --version; scripts/check-parity-breadcrumbs.ts] | Needed if Rust files are added or full verification runs. [VERIFIED: AGENTS.md; scripts/verify.sh] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|---|---|---|
| One canonical boundary doc | Put all content only in `release-readiness.md` | Rejected by discussion because one canonical doc plus links avoids duplicated matrices while preserving reviewer discoverability. [VERIFIED: 82-DISCUSSION-LOG.md; 82-CONTEXT.md] |
| Docs and parity metadata first | Add a Phase 82 checker by default | Checker is allowed only if it materially improves traceability; broad guardrails belong to Phase 88. [VERIFIED: 82-CONTEXT.md] |
| Existing parity roots | New evidence-manifest system | Existing roots already serve machine/human traceability; Phase 82 context names parity index as useful but not a new registry. [VERIFIED: docs/parity/index.json; 82-CONTEXT.md] |
| Existing five-term vocabulary | Ad hoc synonyms such as beta-supported or production-grade | Locked decisions reject alternate near-synonyms because support language is the control surface. [VERIFIED: 82-CONTEXT.md] |
| Documentation and metadata | Rust source/status wording changes | No Rust changes are expected, and adding Rust would trigger parity breadcrumb work. [VERIFIED: 82-CONTEXT.md; AGENTS.md] |

**Installation:**

No new npm, Bun, Cargo, or Bazel dependency is recommended for Phase 82. [VERIFIED: 82-CONTEXT.md; .planning/STACK.md; local package-manager file probe]

```bash
git submodule update --init --recursive
bash scripts/install-git-hooks.sh
bash scripts/verify.sh
```

**Version verification:** No `npm view` command is needed because the standard stack adds no npm package. [VERIFIED: 82-CONTEXT.md; local package-manager file probe] Tool versions were verified from local pins and command probes. [VERIFIED: .bun-version; rust-toolchain.toml; bun --version; cargo --version; bazel --version]

## Architecture Patterns

### Recommended Project Structure

```text
docs/
|-- parity/
|   |-- production-claim-boundary.md                 # canonical Phase 82 boundary doc [ASSUMED]
|   |-- release-readiness.md                         # add v1.8 section, preserve v1.7 history [VERIFIED: 82-CONTEXT.md]
|   |-- deviations-and-unknowns.md                   # durable deferred-surface register [VERIFIED: 82-CONTEXT.md]
|   |-- index.json                                   # machine-readable v1.8 surface and audit entry [VERIFIED: docs/parity/index.json]
|   |-- checklist.md                                 # human-readable v1.8 surface row [VERIFIED: docs/parity/checklist.md]
|   `-- README.md                                    # parity entrypoint link [VERIFIED: docs/parity/README.md]
|-- operator/
|   `-- runtime-guide.md                             # pointer only, no duplicated full matrix [VERIFIED: 82-CONTEXT.md]
`-- ...
README.md                                           # pointer and stale v1.7-current wording refresh [VERIFIED: 82-CONTEXT.md]

scripts/
`-- check-phase82-production-claim-boundary.ts       # optional narrow checker only if needed [VERIFIED: 82-CONTEXT.md; scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts]
```

### Pattern 1: Controlled Support-Term Glossary

**What:** Define exactly `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred`, then require claim-bearing docs to use those terms instead of synonyms. [VERIFIED: 82-CONTEXT.md]  
**When to use:** Use this at the top of the canonical boundary doc so operators and reviewers start from one vocabulary. [VERIFIED: 82-CONTEXT.md; .planning/ROADMAP.md]  
**Example:**

```markdown
<!-- Source: .planning/phases/82-production-claim-boundary/82-CONTEXT.md -->
| Term | Meaning in v1.8 release language |
| --- | --- |
| `supported` | Evidence-backed source-built behavior that default verification and documented UAT substantiate today. |
| `preview` | Shipped behavior without a support commitment. |
| `opt-in UAT` | Explicit operator-run evidence outside default verification. |
| `unsupported` | Local experimentation or historical compatibility without support expectation. |
| `deferred` | Not shipped or not safe to rely on until a future milestone names gates and evidence. |
```

### Pattern 2: Claim-To-Evidence Matrix

**What:** Map each allowed statement to support term, current status, evidence sources, verification command, UAT status, residual risk, and next required gate. [VERIFIED: 82-CONTEXT.md]  
**When to use:** Use this as the core of `PROD-02` and `PROD-03`. [VERIFIED: .planning/REQUIREMENTS.md; 82-CONTEXT.md]  
**Example:**

```markdown
<!-- Source: .planning/phases/82-production-claim-boundary/82-CONTEXT.md -->
| Statement | Support term | Current status | Evidence sources | Verification command | UAT status | Residual risk | Next required gate |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Open Bitcoin defines gates required before a future production full-node readiness claim. | `supported` | allowed | `docs/parity/production-claim-boundary.md`, `docs/parity/index.json`, `bash scripts/verify.sh` closeout | `bash scripts/verify.sh` | docs/parity verification only | Gates are not yet satisfied | Phase 87/88 release-readiness and guardrail closure |
| Open Bitcoin is production full-node ready. | `deferred` | not allowed yet | Deferred-surface inventory and future gate list | none today | no accepted UAT gate today | Broad production claim would overstate support | Future production-readiness milestone after every gate passes |
```

### Pattern 3: Deferred-Surface Inventory With Future Gates

**What:** Preserve the full v1.8 deferred production-adjacent inventory and add why-deferred plus future-gate columns. [VERIFIED: 82-CONTEXT.md; .planning/REQUIREMENTS.md]  
**When to use:** Use this in the canonical doc and summarize or link it from `deviations-and-unknowns.md`. [VERIFIED: 82-CONTEXT.md; docs/parity/deviations-and-unknowns.md]  
**Example:**

```markdown
<!-- Source: .planning/phases/82-production-claim-boundary/82-CONTEXT.md -->
| Surface | Support term | Why deferred | Required future gate |
| --- | --- | --- | --- |
| Inbound serving, address relay, block serving, transaction relay, compact block relay | `deferred` | Requires separate P2P capability and public-network evidence milestones. | Scoped P2P production milestone, public-network evidence, deterministic guardrails, and release-readiness acceptance. |
| Production-funds wallet use and safety | `deferred` | Requires separate threat model, audit, and evidence milestone. | Wallet-production threat model, safety evidence, support policy, and operator docs. |
```

### Pattern 4: Parity Root Entry Instead Of New Registry

**What:** Add `v1-8-production-claim-boundary` to `docs/parity/index.json` top-level surfaces, checklist surfaces, and audit keys, then mirror it in `docs/parity/checklist.md`. [VERIFIED: 82-CONTEXT.md; docs/parity/index.json; docs/parity/checklist.md]  
**When to use:** Use this for machine-readable traceability; do not invent a separate evidence registry. [VERIFIED: 82-CONTEXT.md; docs/parity/index.json]  
**Example:**

```json
{
  "name": "v1-8-production-claim-boundary",
  "status": "done"
}
```

```json
{
  "id": "v1-8-production-claim-boundary",
  "title": "v1.8 Production Claim Boundary",
  "status": "done",
  "requirements": ["PROD-01", "PROD-02", "PROD-03", "PROD-04"],
  "evidence": [
    "docs/parity/production-claim-boundary.md",
    "docs/parity/release-readiness.md",
    "docs/parity/deviations-and-unknowns.md",
    "README.md",
    "docs/operator/runtime-guide.md",
    "scripts/verify.sh"
  ]
}
```

### Pattern 5: Optional Narrow Checker

**What:** If planning decides automation is needed, add one Bun checker with fixture tests that parses `docs/parity/index.json`, checks required support terms and matrix anchors in targeted docs, and rejects only exact Phase 82 overclaim strings. [VERIFIED: 82-CONTEXT.md; scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts]  
**When to use:** Use only when docs plus parity metadata cannot give enough confidence for PROD-02/PROD-03; leave broad all-doc production-claim scanning to Phase 88. [VERIFIED: 82-CONTEXT.md; .planning/ROADMAP.md]  
**Example:**

```typescript
// Source: scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts
const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE82_REPO_ROOT";

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function requireNotContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) {
    failures.push(`${label} must not contain Phase 82 forbidden text: ${needle}`);
  }
}
```

### Anti-Patterns to Avoid

- **Synonym drift:** Do not introduce `production-grade`, `beta-supported`, `ready enough`, or similar terms because the five-term vocabulary is locked. [VERIFIED: 82-CONTEXT.md]
- **Broad all-doc scanner in Phase 82:** It can collide with legitimate historical v1.3-v1.7 text and duplicates Phase 88. [VERIFIED: 82-CONTEXT.md; docs/parity/release-readiness.md]
- **README as source of truth:** README should point to the boundary and avoid stale wording, not duplicate the matrix. [VERIFIED: 82-CONTEXT.md; README.md]
- **Artifact-existence proof:** Reports, bundles, daemon startup, elapsed time, or peer reachability are evidence only when the matrix names the accepted fields and status. [VERIFIED: 82-CONTEXT.md; docs/operator/runtime-guide.md]
- **Hidden support expansion:** Do not change a deferred surface to `supported` or `preview` without a future milestone, evidence gate, and support term update. [VERIFIED: 82-CONTEXT.md; .planning/REQUIREMENTS.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---|---|---|---|
| Production support vocabulary | Ad hoc support/status names | Locked five-term glossary | The vocabulary itself is the release control surface for PROD-01. [VERIFIED: 82-CONTEXT.md; .planning/REQUIREMENTS.md] |
| Evidence registry | New standalone manifest | `docs/parity/index.json`, checklist, release-readiness, and deviations register | Existing parity roots already provide machine/human traceability. [VERIFIED: docs/parity/index.json; docs/parity/checklist.md] |
| Production-readiness proof | Public-network CI, multi-day default gates, real service-manager checks, or live release-blocking sync | Claim-to-evidence matrix plus explicit future gates | v1.8 defines gates before production readiness and keeps public-network/default verifier expansion out of scope. [VERIFIED: .planning/REQUIREMENTS.md; 82-CONTEXT.md] |
| Broad claim scanner | Phase 82 all-doc production term scanner | Optional narrow checker over targeted files, or defer to Phase 88 | Broad default guardrail suite is explicitly deferred to Phase 88. [VERIFIED: 82-CONTEXT.md; .planning/ROADMAP.md] |
| Release marketing copy | Persuasive readiness narrative | Quiet operator-facing release control language | D-04 requires quiet operator-facing language, not marketing copy. [VERIFIED: 82-CONTEXT.md] |
| Rust behavior changes | New source/status behavior to support docs | Documentation and parity metadata unless a narrow gap is found | Phase 82 expects no Rust source changes and Rust changes trigger breadcrumb work. [VERIFIED: 82-CONTEXT.md; AGENTS.md] |

**Key insight:** The planner should prevent claim inflation, not prove production readiness. [VERIFIED: 82-CONTEXT.md; .planning/PROJECT.md]

## Common Pitfalls

### Pitfall 1: Collapsing Support Terms

**What goes wrong:** Docs use `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred` interchangeably. [VERIFIED: 82-CONTEXT.md]  
**Why it happens:** Prior release docs use scoped prose rather than a single v1.8 glossary. [VERIFIED: docs/parity/release-readiness.md; docs/parity/deviations-and-unknowns.md]  
**How to avoid:** Put the glossary in the canonical boundary doc and use exact terms in matrix rows. [VERIFIED: 82-CONTEXT.md]  
**Warning signs:** New text contains `production-grade`, `beta-supported`, `production-ish`, or `ready enough`. [VERIFIED: 82-CONTEXT.md]

### Pitfall 2: Letting "Allowed Statement" Become A Production Claim

**What goes wrong:** The allowed near-term statement that Open Bitcoin defines production gates becomes a claim that the gates are already satisfied. [VERIFIED: 82-CONTEXT.md; .planning/PROJECT.md]  
**Why it happens:** v1.7 shipped strong opt-in soak evidence, but production-node readiness remains deferred. [VERIFIED: .planning/PROJECT.md; .planning/milestones/v1.7-MILESTONE-AUDIT.md]  
**How to avoid:** Include "not allowed yet" rows for production full-node readiness and every production-adjacent surface named in D-07/D-09. [VERIFIED: 82-CONTEXT.md]  
**Warning signs:** README or release-readiness text says v1.8 is production full-node ready, production service ready, or production wallet safe. [VERIFIED: 82-CONTEXT.md; README.md]

### Pitfall 3: Treating Artifact Existence As Evidence

**What goes wrong:** A report, support bundle, or milestone artifact is cited without naming what it proves. [VERIFIED: 82-CONTEXT.md]  
**Why it happens:** Historical docs link many artifacts, and artifact paths can look authoritative by themselves. [VERIFIED: docs/parity/release-readiness.md; docs/operator/runtime-guide.md]  
**How to avoid:** Matrix evidence cells must name concrete evidence and current status; artifact existence by itself is not enough. [VERIFIED: 82-CONTEXT.md]  
**Warning signs:** Matrix rows say only "bundle exists", "report generated", "daemon starts", or "phase passed". [VERIFIED: docs/operator/runtime-guide.md; 82-CONTEXT.md]

### Pitfall 4: Duplicating The Full Matrix In Entry Points

**What goes wrong:** README, runtime guide, release-readiness, parity README, and deviations all carry divergent copies of the same matrix. [VERIFIED: 82-CONTEXT.md]  
**Why it happens:** Multiple entrypoints need discoverability. [VERIFIED: README.md; docs/parity/README.md; docs/operator/runtime-guide.md]  
**How to avoid:** Make the canonical doc authoritative and use pointers or compact summaries elsewhere. [VERIFIED: 82-CONTEXT.md]  
**Warning signs:** README contains a large claim-to-evidence matrix or terms that do not match the canonical doc. [VERIFIED: 82-CONTEXT.md]

### Pitfall 5: Forgetting The Full Deferred Inventory

**What goes wrong:** Docs mention only inbound serving, relay, and wallet while omitting packaging, Windows service integration, hosted dashboards, GUI parity, public-network CI, destructive repair, or support upload. [VERIFIED: 82-CONTEXT.md; .planning/REQUIREMENTS.md]  
**Why it happens:** Different prior milestone docs list slightly different subsets. [VERIFIED: docs/parity/deviations-and-unknowns.md; docs/parity/release-readiness.md]  
**How to avoid:** Use D-09 as the source list and add a future evidence gate for each item. [VERIFIED: 82-CONTEXT.md]  
**Warning signs:** The deferred table has category-only rows without each named surface. [VERIFIED: 82-CONTEXT.md]

### Pitfall 6: Building Phase 88 Early

**What goes wrong:** Phase 82 grows into a broad deterministic production-claim scanner and default verifier guardrail suite. [VERIFIED: 82-CONTEXT.md; .planning/ROADMAP.md]  
**Why it happens:** PROD-02 and PROD-03 need traceability, and automation can look like the fastest proof. [VERIFIED: .planning/REQUIREMENTS.md; scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts]  
**How to avoid:** Add a checker only for narrow local consistency; leave overbroad production-readiness and deferred-surface claim failures to Phase 88. [VERIFIED: 82-CONTEXT.md; .planning/ROADMAP.md]  
**Warning signs:** The proposed checker scans every markdown file for generic words like `production`, `ready`, or `supported`. [VERIFIED: 82-CONTEXT.md; docs/parity/release-readiness.md]

## Code Examples

Verified patterns from local sources:

### Boundary Document Skeleton

```markdown
<!-- Source: .planning/phases/82-production-claim-boundary/82-CONTEXT.md -->
# Production Claim Boundary

## Support Terms

| Term | v1.8 Meaning |
| --- | --- |
| `supported` | Evidence-backed source-built behavior that default verification and documented UAT substantiate today. |
| `preview` | Shipped behavior without support commitment. |
| `opt-in UAT` | Explicit operator-run evidence outside default verification. |
| `unsupported` | Local experimentation or historical compatibility without support expectation. |
| `deferred` | Not shipped or not safe to rely on until future gates and evidence exist. |

## Claim-To-Evidence Matrix

| Statement | Support term | Current status | Evidence sources | Verification command | UAT status | Residual risk | Next required gate |
| --- | --- | --- | --- | --- | --- | --- | --- |

## Deferred Production-Adjacent Surfaces

| Surface | Support term | Why deferred | Required future gate |
| --- | --- | --- | --- |
```

### Parity Index Surface Pattern

```json
{
  "audit": {
    "v1_8_production_claim_boundary": {
      "path": "production-claim-boundary.md",
      "status": "done",
      "requirements": ["PROD-01", "PROD-02", "PROD-03", "PROD-04"],
      "evidence": [
        "docs/parity/production-claim-boundary.md",
        "docs/parity/release-readiness.md",
        "docs/parity/deviations-and-unknowns.md",
        "docs/parity/checklist.md",
        "docs/parity/README.md",
        "README.md",
        "docs/operator/runtime-guide.md",
        "scripts/verify.sh"
      ]
    }
  }
}
```

Source: `docs/parity/index.json` existing v1.7 audit and checklist surface pattern. [VERIFIED: docs/parity/index.json]

### Optional Checker Failure Accumulation

```typescript
// Source: scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts
export function checkPhase82ProductionClaimBoundary(
  maybeRepoRoot = process.env.OPEN_BITCOIN_PHASE82_REPO_ROOT,
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];

  verifyBoundaryDoc(repoRoot, failures);
  verifyParityIndex(repoRoot, failures);
  verifyClaimEntryPoints(repoRoot, failures);

  return failures;
}
```

### Focused Iteration Scans

```bash
# Source: .planning/phases/82-production-claim-boundary/82-CONTEXT.md
rg -n "supported|preview|opt-in UAT|unsupported|deferred" \
  docs/parity/production-claim-boundary.md \
  docs/parity/release-readiness.md \
  docs/parity/deviations-and-unknowns.md

jq '.audit.v1_8_production_claim_boundary.requirements' docs/parity/index.json
```

## State Of The Art

| Old Approach | Current Approach | When Changed | Impact |
|---|---|---|---|
| Historical scoped release claims through v1.7 | v1.8 boundary-setting milestone before any production full-node readiness statement | v1.8 roadmap, 2026-06-20 | Planner should preserve v1.3-v1.7 evidence as historical and make v1.8 the current claim-boundary pointer. [VERIFIED: .planning/ROADMAP.md; .planning/PROJECT.md] |
| Release-readiness prose plus non-claim lists | Controlled support vocabulary plus claim-to-evidence matrix | Phase 82 context, 2026-06-21 | Planner should define exact support terms and matrix rows, not rely on scattered prose. [VERIFIED: 82-CONTEXT.md] |
| v1.7 boundary rooted in existing parity docs and Phase 80 checker | v1.8 boundary rooted in a canonical production boundary doc plus parity metadata | Phase 82 context, 2026-06-21 | Planner should add a canonical doc under `docs/parity/` and link it through existing roots. [VERIFIED: 82-CONTEXT.md; docs/parity/index.json] |
| Production responsibility assumed from operational success | Production responsibility follows explicit readiness review, standards, evidence, training/docs, and acceptance | Google SRE PRR model | This supports treating v1.8 as gate definition rather than readiness acceptance. [CITED: https://sre.google/sre-book/evolving-sre-engagement-model/] |
| Maturity implied by feature completeness | Maturity is tied to adoption, stability, security practices, and production-readiness evidence | CNCF lifecycle framing | This supports separate support terms and no current broad production claim. [CITED: https://contribute.cncf.io/projects/lifecycle/] |
| Release wording as informal prose | Release notes, candidate testing, signed tags/build attestations, and maintenance support are separate release controls | Bitcoin Core release process and lifecycle | This supports keeping signed distribution, package-manager distribution, and support lifecycle as deferred gates. [CITED: https://github.com/bitcoin/bitcoin/blob/master/doc/release-process.md; https://bitcoincore.org/en/lifecycle/] |

**Deprecated/outdated:**

- Treating the v1.7 source-built opt-in soak/recovery claim as the current final production boundary after Phase 82 would be stale. [VERIFIED: README.md; .planning/PROJECT.md; 82-CONTEXT.md]
- Treating public-network full-sync, multi-day soak, real service-manager work, or public-network CI as default verification remains out of scope. [VERIFIED: .planning/REQUIREMENTS.md; 82-CONTEXT.md; scripts/verify.sh]
- Treating ASVS references as certification claims is out of scope; existing threat models use ASVS v5.0.0 as reviewer vocabulary only. [VERIFIED: docs/parity/threat-model-v1.5.md; docs/parity/threat-model-v1.6.md; CITED: https://owasp.org/www-project-application-security-verification-standard/]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|---|---|---|
| A1 | The recommended canonical filename is `docs/parity/production-claim-boundary.md`. [ASSUMED] | Architecture Patterns | If the planner chooses another name, update link targets and parity metadata consistently. |
| A2 | The 30-day validity window is a practical freshness estimate for repo-internal research. [ASSUMED] | Metadata | If the repo changes sooner, rerun focused research before planning against stale anchors. |

## Open Questions (RESOLVED)

1. **Should Phase 82 add a narrow Bun checker?**
   - What we know: Phase 82 decisions prefer docs first and reserve broad default guardrails for Phase 88. [VERIFIED: 82-CONTEXT.md]
   - What's unclear: Whether docs plus `docs/parity/index.json` and focused scans are enough for PROD-02/PROD-03 closeout. [VERIFIED: .planning/REQUIREMENTS.md; 82-CONTEXT.md]
   - Recommendation: Plan docs and parity metadata first; add a checker only if the plan needs automated proof that support terms, the matrix, and the v1.8 parity surface remain present. [VERIFIED: 82-CONTEXT.md; scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts]
   - RESOLVED: Phase 82 adds the narrow checker in Plan 04, limited to targeted docs/parity anchors, exact support terms, the v1.8 parity surface, D-09 deferred surfaces, and verifier ordering. This resolves PROD-02/PROD-03 traceability without building the Phase 88 broad production-claim scanner. [VERIFIED: .planning/phases/82-production-claim-boundary/82-04-PLAN.md]

2. **Should the canonical doc include full future-gate details for Phase 83 through Phase 88?**
   - What we know: Phase 82 must define production readiness gates and deferred surface future gates, while later phases own support matrix, upgrade policy, runbooks, service expectations, release checklist, and broad guardrails. [VERIFIED: .planning/ROADMAP.md; 82-CONTEXT.md]
   - What's unclear: How much detail belongs in Phase 82 versus forward links to later phase placeholders. [VERIFIED: .planning/ROADMAP.md]
   - Recommendation: Name gate categories and evidence expectations now, but avoid writing future phase runbooks or support policies in full. [VERIFIED: 82-CONTEXT.md; .planning/ROADMAP.md]
   - RESOLVED: Phase 82 uses Plan 01's gate-category approach: the canonical boundary names support terms, allowed and not-allowed rows, evidence categories, residual risks, and next required gates, while avoiding full Phase 83 through Phase 88 runbooks or support-policy expansion. [VERIFIED: .planning/phases/82-production-claim-boundary/82-01-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|---|---|---:|---|---|
| Bun | Optional Phase 82 checker/test and existing verifier | yes | 1.3.9 | Missing Bun blocks checker work and `scripts/verify.sh`. [VERIFIED: .bun-version; bun --version; scripts/verify.sh] |
| Cargo | Existing Rust verification if full verifier runs | yes | 1.94.1 | No fallback for Rust verification. [VERIFIED: cargo --version; scripts/verify.sh] |
| Rustc | Existing Rust build/test if full verifier runs | yes | 1.94.1 | No fallback for Rust verification. [VERIFIED: rustc --version; rust-toolchain.toml] |
| Bazel | Existing smoke build and documented UAT command form | yes | 8.6.0 | Cargo forms can document equivalent CLI workflows, but full verifier remains blocked without Bazel. [VERIFIED: bazel --version; AGENTS.md; scripts/verify.sh] |
| cargo-llvm-cov | Full verifier coverage gate | yes | 0.8.5 | No fallback in `scripts/verify.sh`. [VERIFIED: cargo llvm-cov --version; scripts/verify.sh] |
| bash | `scripts/verify.sh` | yes | 3.2.57 | No fallback; verifier is Bash. [VERIFIED: bash --version; scripts/verify.sh] |
| grep | Existing verifier coverage scan | yes | BSD grep 2.6.0 compatible | No fallback in `scripts/verify.sh`. [VERIFIED: grep --version; scripts/verify.sh] |
| git | Source breadcrumb checker and repo state | yes | 2.53.0 | No fallback for breadcrumb checks. [VERIFIED: git --version; scripts/check-parity-breadcrumbs.ts] |
| jq | Focused JSON inspection during iteration | yes | 1.7.1 | Bun/TypeScript checker can parse JSON if jq is unavailable. [VERIFIED: jq --version; scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts] |
| ripgrep | Focused docs scans during iteration | yes | 15.1.0 | Use `grep` for narrower scans if `rg` is unavailable. [VERIFIED: rg --version; grep --version] |

**Missing dependencies with no fallback:** None found in the local environment. [VERIFIED: local command probes]

**Missing dependencies with fallback:** None found in the local environment. [VERIFIED: local command probes]

## Security Domain

### Applicable ASVS Areas

This mapping uses OWASP ASVS v5.0.0 as reviewer vocabulary only; it is not a certification claim and does not expand Phase 82 scope. [VERIFIED: docs/parity/threat-model-v1.5.md; docs/parity/threat-model-v1.6.md; CITED: https://owasp.org/www-project-application-security-verification-standard/]

| ASVS / Security Area | Applies | Standard Control |
|---|---:|---|
| Authentication | no | Phase 82 adds no authentication surface, remote admin, hosted upload, or production-funds wallet support. [VERIFIED: 82-CONTEXT.md; .planning/REQUIREMENTS.md] |
| Session Management | no | Phase 82 adds no browser session or hosted dashboard surface. [VERIFIED: 82-CONTEXT.md; .planning/REQUIREMENTS.md] |
| Access Control | limited | Preserve local-only and source-built boundaries; do not imply production service operation, remote administration, or hosted support upload. [VERIFIED: 82-CONTEXT.md; docs/parity/threat-model-v1.6.md] |
| Input Validation / Sanitization | yes | Parse structured parity JSON with JSON tooling or Bun, and keep support terms as exact controlled values. [VERIFIED: docs/parity/index.json; scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts; 82-CONTEXT.md] |
| Cryptography / Signing | limited | Treat signed packaging and package-manager distribution as deferred; do not imply signed release artifacts exist. [VERIFIED: 82-CONTEXT.md; .planning/REQUIREMENTS.md] |
| Logging / Error Privacy | limited | Preserve support-bundle privacy boundaries and keep automatic support upload deferred. [VERIFIED: 82-CONTEXT.md; docs/parity/deviations-and-unknowns.md] |
| Secure Deployment Limits | yes | State that v1.8 defines gates before production full-node readiness and does not claim production service operation. [VERIFIED: 82-CONTEXT.md; .planning/PROJECT.md] |

### Known Threat Patterns For Phase 82

| Pattern | STRIDE | Standard Mitigation |
|---|---|---|
| Evidence inflation: artifact existence treated as readiness proof | Spoofing / Repudiation | Matrix rows must name concrete evidence, current status, residual risk, and next gate. [VERIFIED: 82-CONTEXT.md] |
| Vocabulary drift broadens support | Repudiation | Use exactly five support terms and reject near-synonyms in claim-bearing docs. [VERIFIED: 82-CONTEXT.md] |
| Deferred surface disappears from docs | Repudiation / Elevation of Privilege | Preserve D-09 inventory in the canonical doc and deviations register with future gates. [VERIFIED: 82-CONTEXT.md] |
| Phase 82 checker duplicates Phase 88 | Denial of Service / Repudiation | Keep any checker narrow and leave broad default-verification blockers to Phase 88. [VERIFIED: 82-CONTEXT.md; .planning/ROADMAP.md] |
| README overstates current readiness | Spoofing / Repudiation | README should point to the canonical v1.8 boundary and avoid duplicating the full matrix. [VERIFIED: 82-CONTEXT.md; README.md] |
| Signed distribution overclaim | Spoofing / Tampering | Keep signed packaging and package-manager distribution as `deferred` until future release-engineering gates exist. [VERIFIED: 82-CONTEXT.md; .planning/REQUIREMENTS.md; CITED: https://github.com/bitcoin/bitcoin/blob/master/doc/release-process.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/82-production-claim-boundary/82-CONTEXT.md` - locked decisions, discretion, deferred ideas, canonical references, reusable assets, and implementation constraints. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - PROD-01 through PROD-04 and v1.8 out-of-scope table. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 82 goal, success criteria, and Phase 88 separation. [VERIFIED: file read]
- `.planning/PROJECT.md` - v1.8 boundary-setting milestone, production-claim constraints, and historical milestone posture. [VERIFIED: file read]
- `.planning/STATE.md` - current milestone state and accumulated v1.8 decisions. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, and relevant `standards/` pages - repo and Bright Builds workflow constraints. [VERIFIED: file read]
- `README.md`, `docs/operator/runtime-guide.md`, `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md`, `docs/parity/README.md`, `docs/parity/checklist.md`, `docs/parity/index.json`, `docs/parity/catalog/p2p.md`, `docs/parity/catalog/chainstate.md`, and `docs/parity/catalog/operator-runtime-release-hardening.md` - target docs and existing release-boundary patterns. [VERIFIED: file read / rg]
- `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts`, `scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts`, `scripts/check-v1.6-release-boundaries.ts`, and `scripts/verify.sh` - checker/test and verifier patterns. [VERIFIED: file read]
- Local tool probes for Bun, Cargo, Rustc, Bazel, cargo-llvm-cov, bash, grep, git, jq, and ripgrep. [VERIFIED: command output]
- Google SRE PRR chapter, CNCF lifecycle page, Bitcoin Core release process, Bitcoin Core lifecycle page, and OWASP ASVS page. [CITED: https://sre.google/sre-book/evolving-sre-engagement-model/; https://contribute.cncf.io/projects/lifecycle/; https://github.com/bitcoin/bitcoin/blob/master/doc/release-process.md; https://bitcoincore.org/en/lifecycle/; https://owasp.org/www-project-application-security-verification-standard/]

### Secondary (MEDIUM confidence)

- `.planning/phases/67-release-boundaries-and-deterministic-verification/67-CONTEXT.md`, `.planning/phases/74-release-boundaries-parity-and-documentation/74-CONTEXT.md`, `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-CONTEXT.md`, `.planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md`, and `.planning/milestones/v1.7-MILESTONE-AUDIT.md` - historical release-boundary and audit patterns. [VERIFIED: file read]
- `.planning/STACK.md`, `.planning/ARCHITECTURE.md`, and `.planning/CONVENTIONS.md` - repo stack, architecture, and convention summaries. [VERIFIED: file read]

### Tertiary (LOW confidence)

- None. [VERIFIED: research scope]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - all tools are repo-pinned, locally probed, or already used by `scripts/verify.sh`, and no new dependencies are recommended. [VERIFIED: .bun-version; rust-toolchain.toml; scripts/verify.sh; local probes]
- Architecture: HIGH - Phase 82 decisions match existing docs/parity root patterns, and the only naming assumption is logged. [VERIFIED: 82-CONTEXT.md; docs/parity/index.json; Assumptions Log]
- Pitfalls: HIGH - pitfalls are drawn from locked decisions, requirements, current docs, and prior release-boundary contexts. [VERIFIED: 82-CONTEXT.md; .planning/REQUIREMENTS.md; docs/parity/release-readiness.md]
- External framing: HIGH - all external references are official project or standards pages. [CITED: https://sre.google/sre-book/evolving-sre-engagement-model/; https://contribute.cncf.io/projects/lifecycle/; https://github.com/bitcoin/bitcoin/blob/master/doc/release-process.md; https://bitcoincore.org/en/lifecycle/; https://owasp.org/www-project-application-security-verification-standard/]

**Research date:** 2026-06-21  
**Valid until:** 2026-07-21 or until Phase 82 implementation changes target docs/checker surfaces. [ASSUMED]
