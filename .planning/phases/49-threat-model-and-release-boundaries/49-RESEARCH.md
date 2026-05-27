# Phase 49: Threat Model and Release Boundaries - Research

**Researched:** 2026-05-27 [VERIFIED: environment_context]
**Domain:** Security threat modeling, parity release documentation, live-evidence acceptance criteria [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]
**Confidence:** HIGH for repo seams and documentation shape; MEDIUM for exact doc/table IDs, which remain planner discretion. [VERIFIED: repo inspection, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

<user_constraints>
## User Constraints (from CONTEXT.md)

Source note: The following locked decisions and discretion text are copied from `.planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md`; claims inside this block inherit this provenance. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)

No standalone `## Deferred Ideas` section exists in the Phase 49 CONTEXT file; deferred and out-of-scope items are recorded in the CONTEXT phase boundary and in the copied decisions above. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PROOF-06 | Reviewer can validate v1.3 live-mainnet evidence with documented acceptance criteria and repo-local commands without adding public-network checks to the default `bash scripts/verify.sh` gate. [VERIFIED: .planning/REQUIREMENTS.md] | Use the artifact-first acceptance contract in `docs/parity/release-readiness.md`: deterministic `bash scripts/verify.sh`, opt-in `bun run scripts/run-live-mainnet-smoke.ts`, repo-local Cargo/Bazel support bundle commands, local report paths, and accepted blocker evidence. [VERIFIED: docs/operator/runtime-guide.md, scripts/run-live-mainnet-smoke.ts, scripts/verify.sh] |
| SEC-01 | Reviewer can inspect a v1.3 threat model covering public peer input, resource exhaustion, storage corruption, operator RPC controls, log/report redaction, and live evidence handling. [VERIFIED: .planning/REQUIREMENTS.md] | Add a reviewer-facing `docs/parity/threat-model-v1.3.md` or equivalent parity-linked document with a compact STRIDE register, assets, trust boundaries, mitigations, residual risks, and evidence links. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md; CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] |
| SEC-02 | Reviewer can inspect refreshed parity and release-readiness docs that distinguish v1.3 proven public-mainnet sync evidence from deferred inbound serving, transaction relay, production-funds wallet, migration apply mode, packaging, and unattended production-node claims. [VERIFIED: .planning/REQUIREMENTS.md] | Refresh `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, `docs/parity/deviations-and-unknowns.md`, and `docs/parity/index.json` links so the shipped boundary and deferred surfaces are discoverable from existing parity roots. [VERIFIED: docs/parity/release-readiness.md, docs/parity/checklist.md, docs/parity/deviations-and-unknowns.md, docs/parity/index.json] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Start repo-changing work through GSD workflow artifacts; this research file is produced under the Phase 49 GSD flow. [VERIFIED: AGENTS.md]
- Use `bash scripts/verify.sh` as the repo-native verification contract before marking work complete. [VERIFIED: AGENTS.md, scripts/verify.sh]
- Keep public-network checks opt-in and outside default verification. [VERIFIED: .planning/REQUIREMENTS.md, docs/operator/runtime-guide.md, scripts/verify.sh]
- Use Bun for repo-owned higher-level automation scripts; this repo has no `package.json` and no `bun install` bootstrap step. [VERIFIED: AGENTS.md, .bun-version, package.json absence]
- Prefer TypeScript for substantial repo-owned script logic and Bash for thin orchestration wrappers. [VERIFIED: AGENTS.md, Bright Builds TypeScript guidance]
- Use repo-local Cargo and Bazel commands in UAT/operator docs instead of relying only on an installed `open-bitcoin` alias. [VERIFIED: AGENTS.md, docs/operator/runtime-guide.md]
- Treat `docs/metrics/lines-of-code.md` as a tracked generated artifact that may need freshness updates after verification. [VERIFIED: AGENTS.md, scripts/verify.sh]
- Record intentional behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion parity docs. [VERIFIED: AGENTS.md, docs/parity/README.md]
- Use `git submodule update --init --recursive` if implementation or verification needs the pinned Bitcoin Knots baseline under `packages/bitcoin-knots`. [VERIFIED: AGENTS.md]
- Do not add production-path third-party Rust Bitcoin libraries; the project owns its domain model and parity surface. [VERIFIED: AGENTS.md, .planning/PROJECT.md]
- Keep functional core / imperative shell boundaries and use repo-owned verification instead of ad hoc command lists. [VERIFIED: AGENTS.md, Bright Builds architecture and verification standards at commit 05f8d7a6c9c2e157ec4f922a05273e72dab97676]
- Before committing in this Rust repo, the repo instructions require formatting, clippy, build, and tests; `bash scripts/verify.sh` is the repo-local umbrella command for completion. [VERIFIED: AGENTS.md, scripts/verify.sh]
- If new Rust source or tests are added under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update parity breadcrumbs. [VERIFIED: AGENTS.md, docs/parity/README.md]
- After substantial parity, operator-surface, workflow, or feature changes, check whether README docs need freshness updates. [VERIFIED: AGENTS.md]
- No project-local skills were found under `.claude/skills/` or `.agents/skills/`; there are no additional project skill patterns to account for. [VERIFIED: filesystem probe]
- For this phase, avoid code/runtime/support-schema expansion because Phase 49 owns documentation and reviewer traceability only. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

## Summary

Phase 49 should be planned as a documentation and deterministic validation phase, not as a runtime, support-bundle, or public-network expansion. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] The existing v1.3 surfaces already provide the raw evidence vocabulary: typed live-smoke no-progress causes, endpoint outcomes, status snapshots, peer contribution rows, resource bounds, durable recovery guidance, and redacted support evidence. [VERIFIED: scripts/run-live-mainnet-smoke.ts, docs/operator/runtime-guide.md, packages/open-bitcoin-cli/src/operator/support.rs, .planning/phases/42-live-smoke-entry-and-network-preflight/42-01-SUMMARY.md, .planning/phases/48-support-evidence-and-operator-runbooks/48-SUMMARY.md]

The primary planning move is to make the release story auditable from existing parity roots. [VERIFIED: docs/parity/README.md, docs/parity/index.json, docs/parity/checklist.md] Add a compact v1.3 STRIDE-style threat model linked from parity docs, update `release-readiness.md` from its current v1.2 framing to a v1.3 boundary matrix, and refresh checklist/deviation docs only enough for discoverability. [VERIFIED: docs/parity/release-readiness.md, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

If `docs/parity/index.json` is changed, add a small Bun/TypeScript deterministic assertion that parses JSON and checks required links/requirement IDs without touching the public network. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md, .bun-version, scripts/verify.sh] Do not add a machine-readable release-claims schema, support-bundle manifest, hosted check, checked-in live report fixture, or public-network step to `bash scripts/verify.sh`. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

**Primary recommendation:** Create `docs/parity/threat-model-v1.3.md`, update `docs/parity/release-readiness.md` with the v1.3 claim boundary and Phase 50 acceptance contract, refresh `docs/parity/checklist.md` / `docs/parity/deviations-and-unknowns.md` / `docs/parity/index.json` links, and add a deterministic Bun doc-link assertion only if JSON roots change. [VERIFIED: repo inspection, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

## Standard Stack

### Core

| Component | Version | Purpose | Why Standard |
|-----------|---------|---------|--------------|
| Markdown parity docs under `docs/parity/` | Existing repo docs surface [VERIFIED: docs/parity/README.md] | Reviewer-facing threat model, release boundary, checklist, deviations, and acceptance criteria [VERIFIED: docs/parity/README.md, docs/parity/release-readiness.md] | Existing parity ledger is the source of truth for release-readiness and intentional deviations. [VERIFIED: docs/parity/README.md, docs/parity/index.json] |
| `docs/parity/index.json` | Existing JSON root [VERIFIED: docs/parity/index.json] | Machine-readable parity root, audit paths, checklist surfaces, and release-readiness links [VERIFIED: docs/parity/index.json] | Phase 49 decisions require threat model and acceptance criteria to be reachable from this root. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| Bun TypeScript script under `scripts/` | Bun 1.3.9 [VERIFIED: .bun-version, `bun --version`] | Deterministic doc/root assertion if parity JSON changes [VERIFIED: AGENTS.md, scripts/verify.sh] | Repo-owned higher-level automation uses Bun/TypeScript, and no `package.json` install surface exists. [VERIFIED: AGENTS.md, package.json absence] |
| `bash scripts/verify.sh` | Existing repo command [VERIFIED: scripts/verify.sh] | Final deterministic verification gate [VERIFIED: AGENTS.md, scripts/verify.sh] | Repo-native verification already runs docs-adjacent generated artifact checks, Rust checks, benchmarks, Bazel smoke, and coverage without public-network sync. [VERIFIED: scripts/verify.sh] |

### Supporting

| Component | Version | Purpose | When to Use |
|-----------|---------|---------|-------------|
| Rust/Cargo workspace | Rust 1.94.1, edition 2024 [VERIFIED: rust-toolchain.toml, packages/Cargo.toml, `rustc --version`] | Only needed if planner adds or touches tests/scripts that trigger full repo verification [VERIFIED: scripts/verify.sh] | Use targeted Rust tests only if Phase 49 unexpectedly touches Rust code; the recommended plan does not require Rust implementation changes. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| Bazel/Bzlmod | Bazel 8.6.0 locally, `rules_rust` 0.69.0 [VERIFIED: `bazel --version`, MODULE.bazel] | Repo-native smoke build through `scripts/verify.sh` [VERIFIED: scripts/verify.sh] | Run through final `bash scripts/verify.sh`; do not add Bazel-only Phase 49 logic. [VERIFIED: scripts/verify.sh, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| OWASP Threat Modeling Cheat Sheet | Current official OWASP Cheat Sheet page [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] | External method reference for assets, trust boundaries, STRIDE prompts, mitigations, and review/validation [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] | Use as background framing; keep the actual threat model scoped to shipped v1.3 surfaces. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| OWASP ASVS 5.0.0 | Latest stable 5.0.0 dated May 2025 [CITED: https://github.com/OWASP/ASVS] | Security-domain checklist language for applicable categories [CITED: https://github.com/OWASP/ASVS] | Use categories as review prompts, not as a certification claim. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `docs/parity/threat-model-v1.3.md` plus release-readiness links [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] | A planning-only `49-SECURITY.md` [VERIFIED: .planning/milestones/v1.2-phases/41-audit-and-revisit-all-the-security-analyses-throughout-the-p/41-SECURITY-AUDIT.md] | Planning-only artifacts are less discoverable for reviewers starting from parity roots, and Phase 49 explicitly asks for reviewer-facing docs. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| Release-readiness boundary matrix in existing docs [VERIFIED: docs/parity/release-readiness.md] | New machine-readable release-claims schema [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] | A new schema is out of scope for Phase 49 and would add maintenance surface beyond the existing parity root. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| Existing support bundle and live-smoke artifacts [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs, scripts/run-live-mainnet-smoke.ts] | New support-bundle manifest or release validator [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] | Phase 49 must not change support schema or imply support tooling proves public-mainnet readiness. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| Deterministic Bun assertion [VERIFIED: AGENTS.md, .bun-version] | Public-network CI check [VERIFIED: .planning/REQUIREMENTS.md] | Public-network checks are explicitly out of the default verification gate for v1.3. [VERIFIED: .planning/REQUIREMENTS.md, scripts/verify.sh] |

**Installation:**

```bash
# No new packages are recommended for Phase 49. [VERIFIED: package.json absence, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]
bun --version
bash scripts/verify.sh
```

**Version verification:** No `npm view` checks are required because the recommended phase adds no npm package dependencies. [VERIFIED: package.json absence, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] Local tool versions verified: Bun 1.3.9, Cargo 1.94.1, rustc 1.94.1, Bazel 8.6.0, cargo-llvm-cov 0.8.5, Git 2.53.0, Node v24.13.0, ripgrep 15.1.0. [VERIFIED: environment availability probes]

## Architecture Patterns

### Recommended Project Structure

```text
docs/
  parity/
    threat-model-v1.3.md              # New reviewer-facing SEC-01 threat model. [VERIFIED: Phase 49 CONTEXT]
    release-readiness.md              # Refresh v1.3 claim boundary and Phase 50 acceptance contract. [VERIFIED: existing file, Phase 49 CONTEXT]
    checklist.md                      # Link threat model / acceptance criteria from human checklist. [VERIFIED: existing file, Phase 49 CONTEXT]
    deviations-and-unknowns.md        # Refresh deferred production-adjacent surfaces only as needed. [VERIFIED: existing file, Phase 49 CONTEXT]
    index.json                        # Add audit/checklist links if JSON root changes. [VERIFIED: existing file, Phase 49 CONTEXT]
scripts/
  check-v1.3-release-boundaries.ts    # Add only if parity JSON roots change. [VERIFIED: Phase 49 CONTEXT, AGENTS.md]
```

### Pattern 1: Compact STRIDE Register

**What:** Build one reviewer-facing threat model with assets, trust boundaries, STRIDE category, scenario, existing mitigations, evidence links, residual risk, and future gate. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md; CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html]

**When to use:** Use this for SEC-01 because Phase 49 requires a consolidated v1.3 threat model and explicitly rejects a broad formal certification artifact. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

**Example:**

```markdown
| Threat ID | STRIDE | Asset | Trust Boundary | Scenario | Mitigation / Evidence | Residual Risk / Future Gate |
| --- | --- | --- | --- | --- | --- | --- |
| V13-TM-01 | T, D | Durable sync state | Public peer input -> sync runtime -> Fjall store | A public peer sends invalid headers or blocks or triggers repeated retries. | Validation-gated contribution, invalid-data peer attribution, no active-chain advancement, bounded retries. Evidence: Phase 44/46 summaries and runtime guide. | Future inbound serving and transaction relay require a new threat model before being claimed. |
```

Source: Phase 49 requires STRIDE-style entries, and prior v1.3 phases provide the cited controls. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md, .planning/phases/44-peer-contribution-attribution/44-01-SUMMARY.md, .planning/phases/46-durable-recovery-and-invalid-data-handling/46-SUMMARY.md]

### Pattern 2: Release Claim Boundary Matrix

**What:** Put the authoritative public-mainnet and production-adjacent boundary in `docs/parity/release-readiness.md`, with each row naming the v1.3 claim, accepted evidence, explicit non-claim, future gate, and requirement. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

**When to use:** Use this for SEC-02 and PROOF-06 because the current release-readiness doc still frames the release as v1.2 and must distinguish proven v1.3 evidence from deferred surfaces. [VERIFIED: docs/parity/release-readiness.md, .planning/REQUIREMENTS.md]

**Example:**

```markdown
| Surface | v1.3 Claim | Accepted Evidence | Explicit Non-Claim | Future Gate | Requirements |
| --- | --- | --- | --- | --- | --- |
| Public-mainnet sync proof | Opt-in local evidence can show validated header/block progress, restart/resume progress, or a diagnosed blocker. | `bash scripts/verify.sh`, live-smoke JSON/Markdown, support evidence JSON/Markdown, status snapshots. | No unattended production-node readiness and no public-network default verification gate. | Phase 50 closeout or later production-node milestone. | PROOF-06, SEC-02 |
```

Source: Existing live-smoke/support artifacts and Phase 49 decisions define this boundary. [VERIFIED: scripts/run-live-mainnet-smoke.ts, packages/open-bitcoin-cli/src/operator/support.rs, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

### Pattern 3: Artifact-First Phase 50 Acceptance Contract

**What:** Document acceptance criteria as local artifact review, not as a timing threshold or public-network CI gate. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md, docs/operator/runtime-guide.md]

**When to use:** Use this for PROOF-06 because Phase 50 may close with either observed progress or a diagnosed blocker. [VERIFIED: .planning/REQUIREMENTS.md, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

**Example:**

```bash
bash scripts/verify.sh
bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet \
  --timeout-seconds=60 --poll-seconds=5 --manual-peer=HOST[:PORT]
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support \
  --include-live-smoke-report=packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  support bundle --output-dir=/tmp/open-bitcoin-support
```

Source: These command forms already exist in the operator guide and Phase 49 context. [VERIFIED: docs/operator/runtime-guide.md, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

### Pattern 4: Deterministic Parity Root Assertion

**What:** If `docs/parity/index.json` changes, add a small Bun script that parses the JSON root and checks required requirement IDs, doc links, and no public-network fixture references. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md, AGENTS.md]

**When to use:** Use this when implementation updates machine-readable parity roots because D-13 requires a scriptable assertion for linked docs and traceability. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

**Example:**

```typescript
#!/usr/bin/env bun

import { readFileSync } from "node:fs";

const index = JSON.parse(readFileSync("docs/parity/index.json", "utf8"));
const serialized = JSON.stringify(index);
for (const required of ["PROOF-06", "SEC-01", "SEC-02"]) {
  if (!serialized.includes(required)) {
    throw new Error(`missing requirement trace: ${required}`);
  }
}
for (const path of ["threat-model-v1.3.md", "release-readiness.md", "checklist.md"]) {
  if (!serialized.includes(path)) {
    throw new Error(`missing parity link: ${path}`);
  }
}
```

Source: JSON parsing uses Node/Bun standard APIs and matches existing Bun script patterns. [VERIFIED: scripts/check-parity-breadcrumbs.ts, scripts/check-benchmark-report.ts, AGENTS.md]

### Anti-Patterns to Avoid

- **Expanding support artifacts:** Do not add a support-bundle manifest or treat `support-evidence.json` as a release validator. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]
- **Adding live checks to default verification:** Do not add public-network runs, hosted CI network checks, or checked-in live-mainnet report fixtures to `bash scripts/verify.sh`. [VERIFIED: .planning/REQUIREMENTS.md, scripts/verify.sh, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]
- **Blurring endpoint reachability with sync proof:** Do not call DNS/TCP reachability or support-bundle presence successful public-mainnet progress. [VERIFIED: scripts/run-live-mainnet-smoke.ts, docs/operator/runtime-guide.md]
- **Creating a broad security certification artifact:** Keep the threat model scoped to v1.3 shipped surfaces and explicit future gates. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]
- **Duplicating the status truth model:** Use `OpenBitcoinStatusSnapshot` and existing report artifacts as evidence links instead of inventing new status DTOs. [VERIFIED: docs/architecture/status-snapshot.md, packages/open-bitcoin-cli/src/operator/support.rs]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Threat model method | A custom taxonomy with unreviewable categories [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] | Compact STRIDE register with assets, trust boundaries, mitigations, residual risks, and evidence links [VERIFIED: Phase 49 CONTEXT] | STRIDE is already the locked Phase 49 structure, and OWASP documents it as a structured prompt for threat identification. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md; CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] |
| Live-mainnet evidence collection | A new runner or network probe [VERIFIED: scripts/run-live-mainnet-smoke.ts] | `bun run scripts/run-live-mainnet-smoke.ts` [VERIFIED: scripts/run-live-mainnet-smoke.ts, docs/operator/runtime-guide.md] | Existing runner already writes JSON/Markdown evidence with endpoint outcomes, status snapshots, no-progress causes, next action, and peer rows. [VERIFIED: scripts/run-live-mainnet-smoke.ts] |
| Support evidence packaging | New support schema or release validator [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] | Existing `open-bitcoin support bundle` outputs [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs, docs/operator/runtime-guide.md] | Phase 48 support bundle is local redacted evidence only and Phase 49 must not change that boundary. [VERIFIED: .planning/phases/48-support-evidence-and-operator-runbooks/48-SUMMARY.md, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| Release claims traceability | Separate machine-readable release-claims schema [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] | Existing parity docs and `docs/parity/index.json` links [VERIFIED: docs/parity/README.md, docs/parity/index.json] | D-05 explicitly rejects a separate release-claims schema in this phase. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| Secret/log redaction policy | Raw archive plus best-effort grep redaction [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] | Existing allowlisted support fields and redaction summary [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs, packages/open-bitcoin-cli/tests/operator_binary.rs] | OWASP logging guidance warns against recording passwords, keys, access tokens, and sensitive data directly in logs. [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html] |

**Key insight:** Phase 49 should document and link the controls already built in Phases 42-48; custom validators, schemas, network gates, or runtime changes would expand the shipped support boundary instead of clarifying it. [VERIFIED: .planning/phases/42-live-smoke-entry-and-network-preflight/42-01-SUMMARY.md, .planning/phases/48-support-evidence-and-operator-runbooks/48-SUMMARY.md, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

## Common Pitfalls

### Pitfall 1: Over-Claiming Production Readiness

**What goes wrong:** Docs imply unattended production-node readiness, production-funds wallet support, packaging readiness, inbound serving, transaction relay, migration apply mode, hosted/public surfaces, or GUI support. [VERIFIED: .planning/REQUIREMENTS.md, docs/operator/runtime-guide.md]

**Why it happens:** Current release-readiness text is still v1.2-oriented, while v1.3 adds stronger live evidence and hardening language that could be mistaken for a production claim. [VERIFIED: docs/parity/release-readiness.md, .planning/ROADMAP.md]

**How to avoid:** Use a boundary matrix with explicit `v1.3 claim`, `accepted evidence`, `explicit non-claim`, `future gate`, and `requirement` columns. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

**Warning signs:** The words `production`, `unattended`, `packaged`, `relay`, `inbound`, `wallet funds`, `apply mode`, `hosted`, or `GUI` appear without a matching non-claim or future gate. [VERIFIED: .planning/REQUIREMENTS.md, docs/operator/runtime-guide.md]

### Pitfall 2: Making Public Network Checks Part of Default Verification

**What goes wrong:** A plan adds live-mainnet smoke execution, DNS/TCP probes, or public peer checks to `bash scripts/verify.sh`. [VERIFIED: .planning/REQUIREMENTS.md, scripts/verify.sh]

**Why it happens:** PROOF-06 requires documented acceptance commands, but the same requirement forbids adding public-network checks to the default gate. [VERIFIED: .planning/REQUIREMENTS.md]

**How to avoid:** Add only deterministic doc/root checks to default verification; keep `bun run scripts/run-live-mainnet-smoke.ts` as an opt-in operator command documented for Phase 50. [VERIFIED: scripts/verify.sh, docs/operator/runtime-guide.md]

**Warning signs:** `scripts/verify.sh` gains `run-live-mainnet-smoke`, DNS seed calls, manual peer calls, or live report fixture dependencies. [VERIFIED: scripts/verify.sh, scripts/run-live-mainnet-smoke.ts]

### Pitfall 3: Treating Support Bundles as Proof

**What goes wrong:** Docs claim that generating `support-evidence.json` proves public-mainnet readiness. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs, docs/operator/runtime-guide.md]

**Why it happens:** Support bundles can include live-smoke summaries, status snapshots, and store-health evidence, which are useful but not sufficient by themselves. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs, docs/operator/runtime-guide.md]

**How to avoid:** State that the support bundle is local redacted evidence and must be paired with Phase 50 live-smoke/status artifacts for public-mainnet acceptance. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

**Warning signs:** A support-bundle path is listed under `accepted evidence` without a live-smoke report, status snapshots, or explicit blocker diagnosis context. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

### Pitfall 4: Leaking Secrets Through Evidence Examples

**What goes wrong:** Examples include RPC passwords, cookies, private wallet material, raw logs, or raw live-smoke tails. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs, packages/open-bitcoin-cli/tests/operator_binary.rs]

**Why it happens:** Evidence docs often copy realistic commands and reports, and support inputs can contain secret-like strings. [VERIFIED: packages/open-bitcoin-cli/tests/operator_binary.rs]

**How to avoid:** Use placeholder values, document metadata-only credential handling, and keep live-smoke support ingestion allowlisted. [VERIFIED: docs/operator/runtime-guide.md, packages/open-bitcoin-cli/src/operator/support.rs]

**Warning signs:** `rpcpassword=`, `rpcauth=`, `__cookie__`, `private_key`, `xprv`, or raw log blocks appear in new docs or fixtures. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs]

### Pitfall 5: Breaking Parity Root Discoverability

**What goes wrong:** The threat model exists but is not reachable from `docs/parity/index.json`, `docs/parity/checklist.md`, or `docs/parity/release-readiness.md`. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

**Why it happens:** It is easy to add a standalone doc without updating the existing parity entrypoints. [VERIFIED: docs/parity/README.md]

**How to avoid:** Update the parity root and human checklist links together, and add a deterministic assertion when JSON roots change. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]

**Warning signs:** `rg -n "threat-model-v1.3|SEC-01|PROOF-06|SEC-02" docs/parity/index.json docs/parity/checklist.md docs/parity/release-readiness.md` misses any root. [VERIFIED: repo inspection]

## Code Examples

Verified patterns from project sources:

### Deterministic Parity Link Assertion

```typescript
#!/usr/bin/env bun

import { readFileSync } from "node:fs";

const index = JSON.parse(readFileSync("docs/parity/index.json", "utf8"));
const serialized = JSON.stringify(index);
for (const required of ["PROOF-06", "SEC-01", "SEC-02"]) {
  if (!serialized.includes(required)) {
    throw new Error(`missing requirement trace: ${required}`);
  }
}
for (const path of ["threat-model-v1.3.md", "release-readiness.md"]) {
  if (!serialized.includes(path)) {
    throw new Error(`missing required parity doc link: ${path}`);
  }
}
```

Source: Existing scripts use Bun/TypeScript and Node standard-library file reads for repo assertions. [VERIFIED: scripts/check-parity-breadcrumbs.ts, scripts/check-benchmark-report.ts, AGENTS.md]

### Threat Model Row

```markdown
| V13-TM-05 | I | Support and live evidence artifacts | Local evidence -> reviewer | A shared bundle leaks RPC credentials, wallet private material, or raw logs. | Support bundle includes credential metadata only, redaction summary, allowlisted live-smoke summary fields, and redaction regression tests. | Future hosted support upload would need a new threat model and access-control design. |
```

Source: Support bundle redaction is implemented and tested; hosted support upload is not in scope. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs, packages/open-bitcoin-cli/tests/operator_binary.rs, .planning/REQUIREMENTS.md]

### Release Boundary Row

```markdown
| Transaction relay | No v1.3 shipped claim. | Deferred-surface docs and parity checklist. | v1.3 does not claim mempool transaction relay or propagation behavior. | Future PRODNODE/relay milestone with parity and resource-governance evidence. | SEC-02 |
```

Source: Transaction relay is explicitly out of v1.3 scope. [VERIFIED: .planning/REQUIREMENTS.md, docs/operator/runtime-guide.md]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| One-time threat model stored as internal planning notes [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] | Maintained, reviewable threat model tied to assets, trust boundaries, mitigations, and validation [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] | OWASP guidance treats threat modeling as part of the SDLC rather than a one-off activity. [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] | Phase 49 should create a reviewer-facing parity doc instead of only a planning artifact. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| Broad release-security certification language [VERIFIED: .planning/milestones/v1.2-phases/41-audit-and-revisit-all-the-security-analyses-throughout-the-p/41-SECURITY-AUDIT.md] | Scoped release boundary matrix with explicit non-claims and future gates [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] | v1.3 milestone was created after v1.2 did not close public-mainnet progress proof. [VERIFIED: .planning/PROJECT.md, .planning/STATE.md] | Reviewers can audit what is proven without expanding support boundary. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| ASVS 4-era category names in older templates [CITED: https://github.com/OWASP/ASVS] | ASVS 5.0.0 categories include V1 Encoding, V2 Validation and Business Logic, V4 API and Web Service, V6 Authentication, V8 Authorization, V11 Cryptography, V13 Configuration, V14 Data Protection, V16 Security Logging and Error Handling [CITED: https://github.com/OWASP/ASVS] | ASVS 5.0.0 was released in May 2025. [CITED: https://github.com/OWASP/ASVS] | Use current ASVS categories as review prompts and avoid claiming formal certification. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |

**Deprecated/outdated:**

- The current `docs/parity/release-readiness.md` v1.2 framing is outdated for Phase 49 planning and must be refreshed for v1.3. [VERIFIED: docs/parity/release-readiness.md, .planning/ROADMAP.md]
- A Phase 41-style planning-security audit is insufficient for SEC-01 because SEC-01 asks for a reviewer-inspectable v1.3 threat model covering the expanded public-mainnet hardening scope. [VERIFIED: .planning/REQUIREMENTS.md, .planning/milestones/v1.2-phases/41-audit-and-revisit-all-the-security-analyses-throughout-the-p/41-SECURITY-AUDIT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| - | No unverified assumptions were used. [VERIFIED: source review and environment probes] | All sections | Low; planner can proceed from verified repo and official-source evidence. [VERIFIED: source review and environment probes] |

## Open Questions (RESOLVED)

1. **Exact checklist surface ID**
   - What we know: Existing `docs/parity/index.json` has checklist surfaces and audit roots, and Phase 49 requires parity-root discoverability. [VERIFIED: docs/parity/index.json, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]
   - Resolution: Use `v1-3-threat-model-release-boundaries` as the stable checklist surface ID and `v1_3_threat_model` / `v1_3_release_boundaries` as parity audit keys. [VERIFIED: docs/parity/index.json naming patterns]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Bun | Deterministic docs/root assertion and existing scripts [VERIFIED: AGENTS.md, scripts/verify.sh] | Yes [VERIFIED: `bun --version`] | 1.3.9 [VERIFIED: `.bun-version`, `bun --version`] | None needed. [VERIFIED: environment probe] |
| Rust/Cargo | Final repo verification [VERIFIED: scripts/verify.sh] | Yes [VERIFIED: `cargo --version`] | cargo 1.94.1 [VERIFIED: `cargo --version`] | None needed. [VERIFIED: environment probe] |
| rustc | Final repo verification [VERIFIED: scripts/verify.sh] | Yes [VERIFIED: `rustc --version`] | rustc 1.94.1 [VERIFIED: `rustc --version`, rust-toolchain.toml] | None needed. [VERIFIED: environment probe] |
| Bazel | Final repo verification [VERIFIED: scripts/verify.sh] | Yes [VERIFIED: `bazel --version`] | 8.6.0 [VERIFIED: `bazel --version`] | None needed. [VERIFIED: environment probe] |
| cargo-llvm-cov | Final repo verification coverage gate [VERIFIED: scripts/verify.sh] | Yes [VERIFIED: `cargo llvm-cov --version`] | 0.8.5 [VERIFIED: `cargo llvm-cov --version`] | None needed. [VERIFIED: environment probe] |
| Git | GSD commit and repo scripts [VERIFIED: scripts/verify.sh] | Yes [VERIFIED: `git --version`] | 2.53.0 [VERIFIED: `git --version`] | None needed. [VERIFIED: environment probe] |
| ripgrep | Research/search and optional implementation checks [VERIFIED: repo workflow instructions] | Yes [VERIFIED: `rg --version`] | 15.1.0 [VERIFIED: `rg --version`] | Use `grep` if unavailable. [VERIFIED: scripts/verify.sh requires grep] |

**Missing dependencies with no fallback:** None found. [VERIFIED: environment probes]

**Missing dependencies with fallback:** None found. [VERIFIED: environment probes]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V1 Encoding and Sanitization | Yes, for Markdown/JSON report examples and doc validator output. [CITED: https://github.com/OWASP/ASVS; VERIFIED: scripts/run-live-mainnet-smoke.ts] | Escape or avoid untrusted report text in Markdown examples; keep live-smoke raw tails out of support summaries. [VERIFIED: scripts/run-live-mainnet-smoke.ts, packages/open-bitcoin-cli/src/operator/support.rs] |
| V2 Validation and Business Logic | Yes, for public peer evidence interpretation and claim-boundary rules. [CITED: https://github.com/OWASP/ASVS; VERIFIED: .planning/REQUIREMENTS.md] | Treat validated header/block progress separately from endpoint reachability and support-bundle availability. [VERIFIED: .planning/phases/44-peer-contribution-attribution/44-01-SUMMARY.md, docs/operator/runtime-guide.md] |
| V4 API and Web Service | Yes, for local JSON-RPC operator controls and `getblockchaininfo` evidence. [CITED: https://github.com/OWASP/ASVS; VERIFIED: docs/operator/runtime-guide.md] | Keep operator RPC controls local, authenticated, and documented as not a public hosted surface. [VERIFIED: docs/operator/runtime-guide.md, .planning/milestones/v1.2-phases/41-audit-and-revisit-all-the-security-analyses-throughout-the-p/41-SECURITY-AUDIT.md] |
| V5 File Handling | Yes, for local evidence files and support bundle paths. [CITED: https://github.com/OWASP/ASVS; VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] | Write local JSON/Markdown artifacts only, avoid source datadir mutation, and report unavailable artifacts with reasons. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs, .planning/phases/48-support-evidence-and-operator-runbooks/48-CONTEXT.md] |
| V6 Authentication | Yes, for local RPC credential evidence. [CITED: https://github.com/OWASP/ASVS; VERIFIED: docs/architecture/config-precedence.md] | Report credential source metadata only; do not expose cookie contents or passwords. [VERIFIED: docs/architecture/config-precedence.md, packages/open-bitcoin-cli/src/operator/support.rs] |
| V7 Session Management | No browser/session-management surface is shipped in Phase 49. [CITED: https://github.com/OWASP/ASVS; VERIFIED: .planning/REQUIREMENTS.md] | No Phase 49 control required beyond not adding hosted/public web surfaces. [VERIFIED: .planning/REQUIREMENTS.md] |
| V8 Authorization | Yes, for operator pause/resume/status and offline mutation boundaries. [CITED: https://github.com/OWASP/ASVS; VERIFIED: .planning/phases/45-runtime-resource-bounds-and-store-coordination/45-SUMMARY.md] | Preserve live-RPC-first control and second-writer diagnostics; Phase 49 documents, not changes, this control. [VERIFIED: .planning/phases/45-runtime-resource-bounds-and-store-coordination/45-SUMMARY.md] |
| V11 Cryptography | No new crypto is added in Phase 49. [CITED: https://github.com/OWASP/ASVS; VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] | Do not add cryptographic claims or hand-roll cryptographic controls. [VERIFIED: AGENTS.md, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| V12 Secure Communication | Limited to documenting current local RPC and public-peer evidence boundaries. [CITED: https://github.com/OWASP/ASVS; VERIFIED: docs/operator/runtime-guide.md] | Keep public peer operation opt-in and local RPC bound to documented operator workflows. [VERIFIED: docs/operator/runtime-guide.md] |
| V13 Configuration | Yes, for JSONC sync knobs, manual peers, and config precedence. [CITED: https://github.com/OWASP/ASVS; VERIFIED: docs/architecture/config-precedence.md] | Keep Open Bitcoin-only sync settings in `open-bitcoin.jsonc` and document precedence. [VERIFIED: docs/architecture/config-precedence.md] |
| V14 Data Protection | Yes, for support and live evidence redaction. [CITED: https://github.com/OWASP/ASVS; VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] | Preserve redaction of RPC secrets, wallet private material, raw logs, and raw live-smoke inputs. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |
| V15 Secure Coding and Architecture | Yes, for docs validator and no runtime expansion. [CITED: https://github.com/OWASP/ASVS; VERIFIED: AGENTS.md] | Use repo-owned Bun script patterns, no new dependencies, and no runtime/schema expansion. [VERIFIED: AGENTS.md, .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |
| V16 Security Logging and Error Handling | Yes, for log/report redaction and evidence diagnosis. [CITED: https://github.com/OWASP/ASVS; CITED: https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html] | Preserve typed no-progress causes, next actions, bounded tails, and redaction boundaries. [VERIFIED: scripts/run-live-mainnet-smoke.ts, packages/open-bitcoin-cli/src/operator/support.rs] |
| V17 WebRTC | No WebRTC surface exists in Phase 49. [CITED: https://github.com/OWASP/ASVS; VERIFIED: .planning/REQUIREMENTS.md] | No control required. [VERIFIED: .planning/REQUIREMENTS.md] |

### Known Threat Patterns for v1.3 Public-Mainnet Evidence

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Public peer sends invalid headers or blocks. [VERIFIED: .planning/phases/46-durable-recovery-and-invalid-data-handling/46-SUMMARY.md] | Tampering, Denial of Service [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] | Validation-gated contribution, invalid-data peer attribution, no active-chain advancement, and recovery guidance. [VERIFIED: .planning/phases/44-peer-contribution-attribution/44-01-SUMMARY.md, .planning/phases/46-durable-recovery-and-invalid-data-handling/46-SUMMARY.md] |
| Public peer failures exhaust runtime resources. [VERIFIED: .planning/phases/45-runtime-resource-bounds-and-store-coordination/45-SUMMARY.md] | Denial of Service [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] | Bounded outbound targets, message caps, sync rounds, block in-flight caps, bounded metrics, and bounded logs. [VERIFIED: .planning/phases/45-runtime-resource-bounds-and-store-coordination/45-SUMMARY.md, docs/operator/runtime-guide.md] |
| Store corruption or second writer confuses durable evidence. [VERIFIED: .planning/phases/45-runtime-resource-bounds-and-store-coordination/45-SUMMARY.md, .planning/phases/46-durable-recovery-and-invalid-data-handling/46-SUMMARY.md] | Tampering, Denial of Service [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] | Offline mutating controls refuse unclean active owners, and status separates storage recovery guidance from peer guidance. [VERIFIED: .planning/phases/45-runtime-resource-bounds-and-store-coordination/45-SUMMARY.md, docs/operator/runtime-guide.md] |
| Operator RPC controls are spoofed or misapplied. [VERIFIED: .planning/milestones/v1.2-phases/41-audit-and-revisit-all-the-security-analyses-throughout-the-p/41-SECURITY-AUDIT.md] | Spoofing, Elevation of Privilege, Tampering [CITED: https://learn.microsoft.com/en-us/windows-hardware/drivers/driversecurity/threat-modeling-for-drivers] | Reachable daemon auth failures are terminal, mutations route through daemon sync control, and offline fallback is limited. [VERIFIED: .planning/milestones/v1.2-phases/41-audit-and-revisit-all-the-security-analyses-throughout-the-p/41-SECURITY-AUDIT.md] |
| Logs or reports leak secrets. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] | Information Disclosure [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html] | Metadata-only credential evidence, allowlisted live-smoke summary fields, omitted raw logs, and redaction regression tests. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs, packages/open-bitcoin-cli/tests/operator_binary.rs] |
| Release docs overstate support boundary. [VERIFIED: docs/parity/release-readiness.md, .planning/REQUIREMENTS.md] | Spoofing, Repudiation [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html] | Claim boundary matrix, explicit non-claims, future gates, and requirement-to-phase traceability. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md` - locked Phase 49 decisions, boundary, and verification posture. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - PROOF-06, SEC-01, SEC-02, v1.3 out-of-scope surfaces, and traceability. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 49/50 success criteria and v1.3 milestone scope. [VERIFIED: file read]
- `.planning/PROJECT.md` and `.planning/STATE.md` - current milestone decisions and public-network proof blockers. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, and `standards-overrides.md` - repo-local workflow, verification, and Bright Builds constraints. [VERIFIED: file read]
- Bright Builds standards at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676` - architecture, code shape, verification, testing, Rust, and TypeScript guidance. [CITED: https://github.com/bright-builds-llc/bright-builds-rules/tree/05f8d7a6c9c2e157ec4f922a05273e72dab97676]
- `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, `docs/parity/deviations-and-unknowns.md`, `docs/parity/index.json`, and `docs/parity/README.md` - existing parity roots and current v1.2-oriented release wording. [VERIFIED: file read]
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, and `docs/architecture/config-precedence.md` - operator boundaries, status model, observability, config, and redaction guidance. [VERIFIED: file read]
- `scripts/run-live-mainnet-smoke.ts`, `packages/open-bitcoin-cli/src/operator/support.rs`, and `packages/open-bitcoin-cli/src/operator/support/render.rs` - live-smoke and support evidence contracts. [VERIFIED: file read]
- Phase 42-48 contexts/summaries and v1.2 Phase 40/41 security closeouts - prior controls and residual-risk boundaries. [VERIFIED: file read]

### Secondary (MEDIUM confidence)

- OWASP Threat Modeling Cheat Sheet - threat-model process, STRIDE use, mitigations, and review/validation prompts. [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html]
- OWASP ASVS repository - ASVS 5.0.0 release and current category names. [CITED: https://github.com/OWASP/ASVS]
- OWASP Logging Cheat Sheet - sensitive data exclusion and log verification guidance. [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html]
- Microsoft Learn STRIDE page - STRIDE category definitions and data-flow threat modeling structure. [CITED: https://learn.microsoft.com/en-us/windows-hardware/drivers/driversecurity/threat-modeling-for-drivers]

### Tertiary (LOW confidence)

- None. [VERIFIED: source review]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - no new packages are needed, and required repo tools were verified locally. [VERIFIED: environment probes, package.json absence]
- Architecture: HIGH - the phase is constrained to existing parity docs, release-readiness docs, live-smoke artifacts, and support evidence. [VERIFIED: .planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md]
- Pitfalls: HIGH - pitfalls are directly tied to locked decisions and existing docs/code boundaries. [VERIFIED: .planning/REQUIREMENTS.md, docs/operator/runtime-guide.md, packages/open-bitcoin-cli/src/operator/support.rs]
- External security framing: MEDIUM - OWASP/Microsoft sources are current enough for method/category framing, but Phase 49 is not a formal compliance/certification exercise. [CITED: https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html, https://github.com/OWASP/ASVS]

**Research date:** 2026-05-27 [VERIFIED: environment_context]
**Valid until:** 2026-06-26 for repo-specific phase planning; re-check OWASP ASVS and repo docs if planning occurs after that date. [VERIFIED: current_date; CITED: https://github.com/OWASP/ASVS]
