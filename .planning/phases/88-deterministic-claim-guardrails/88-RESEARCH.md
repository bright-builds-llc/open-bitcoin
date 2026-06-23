# Phase 88: Deterministic Claim Guardrails - Research

**Researched:** 2026-06-23 [VERIFIED: environment current_date]
**Domain:** Bun/TypeScript deterministic documentation verifier for v1.8 release-claim boundaries [VERIFIED: .planning/phases/88-deterministic-claim-guardrails/88-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: repo-local checker, docs, config, and environment audit]

<user_constraints>
## User Constraints (from CONTEXT.md)

All content in this section is copied from `.planning/phases/88-deterministic-claim-guardrails/88-CONTEXT.md`. [VERIFIED: .planning/phases/88-deterministic-claim-guardrails/88-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Claim Scan Boundary

- **D-01:** Add a new Phase 88 Bun checker rather than extending the completed
  Phase 87 release-readiness checker. Phase 87 remains the checklist gate;
  Phase 88 owns broad deterministic claim guardrails.
- **D-02:** Use a curated release and public-operator documentation surface
  instead of a whole-docs-tree scan. Include current release roots, parity
  entrypoints, the runtime guide, and relevant parity catalog pages where
  release or operator claims are likely to be read.
- **D-03:** Do not scan `.planning/` histories, milestone archives, or every
  historical doc as a blocking default verifier surface. Historical scoped
  claims must stay discoverable without becoming false positives.
- **D-04:** Allow production-readiness and deferred-surface terms only when the
  surrounding sentence, paragraph, or table row is explicitly negative or
  scoped, such as `does not claim`, `not allowed yet`, `deferred`,
  `unsupported`, `historical`, `opt-in UAT`, `future gate`, or `outside default
  verification`.

### Evidence Gate Semantics

- **D-05:** Define production full-node readiness as disallowed for v1.8 unless
  a future milestone satisfies every required evidence gate. The only allowed
  v1.8 claim remains that Open Bitcoin defines the gates required before a
  future production full-node readiness claim.
- **D-06:** Treat existing docs as the source of truth for gates:
  `docs/parity/production-claim-boundary.md`,
  `docs/parity/support-matrix.md`, and
  `docs/parity/release-readiness.md`. Do not introduce a separate
  machine-readable v1.8 evidence manifest in this phase.
- **D-07:** A deferred-surface promotion is valid only after a future scoped
  phase names concrete evidence, a verifier or opt-in UAT command, residual
  risk, and next-gate status. Prose-only promotion must fail deterministic
  verification.
- **D-08:** Field-based evidence and named verifier roots matter; artifact
  existence, daemon startup, elapsed time, peer reachability, raw log tail,
  service file existence, support bundle path, or context-only records are not
  sufficient by themselves.

### Deferred-Surface Promotion Rules

- **D-09:** Fail positive promotion language for the Phase 82 deferred inventory,
  including inbound serving, address relay, block serving, transaction relay,
  compact block relay, production-funds wallet use or safety, migration apply
  mode, signed packaging or package-manager distribution, Windows service
  integration, hosted dashboards, GUI parity, public-network default checks,
  public-network CI, release-blocking live sync, automatic support-bundle
  upload, destructive repair, and broad production-node readiness.
- **D-10:** Cover promotion predicates such as `production-ready`,
  `production-grade`, `fully supported`, `default-verified`,
  `release-blocking`, `proven`, `GA`, `certified`, and close variants when
  attached to deferred production-adjacent surfaces.
- **D-11:** Keep exact bad-phrase denylist checks as supplemental smoke coverage,
  but do not rely only on exact strings. The checker should combine curated
  phrase matching with scoped allow rules to catch obvious paraphrases without
  blocking valid no-claim text.

### Verifier Integration And Regression Tests

- **D-12:** Add `scripts/check-phase88-deterministic-claim-guardrails.ts` and
  `scripts/check-phase88-deterministic-claim-guardrails.test.ts`, following the
  Phase 82 through Phase 87 Bun checker and fixture-test pattern.
- **D-13:** Use an `OPEN_BITCOIN_PHASE88_REPO_ROOT` override in fixture tests so
  bad release-doc and verifier wiring cases can be tested in temporary repos.
- **D-14:** Wire both `bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts`
  and `bun run scripts/check-phase88-deterministic-claim-guardrails.ts` into
  `scripts/verify.sh` immediately after the Phase 87 checker, both in the
  visible command-order heredoc and the executed `run_step` sequence.
- **D-15:** Strip the verifier command-order heredoc before checking the
  executable verifier text. A command that exists only in the heredoc must not
  satisfy Phase 88 verifier wiring.
- **D-16:** The default verifier must remain deterministic, short-running,
  public-network-free, real-service-manager-free, and multi-day-free. Fail
  verifier drift that adds commands or text such as `run-live-mainnet-smoke`,
  `systemctl`, `launchctl`, long `sleep` gates, `--restart-after-progress`,
  package-manager service commands, public-network CI/default gates,
  release-blocking live sync, automatic support upload, destructive repair, or
  broad production-node readiness.

### Folded Todos

No pending todos matched Phase 88.

### the agent's Discretion

- The planner may split the phase into checker implementation, fixture tests,
  docs/parity root updates, verifier wiring, and closeout verification.
- The executor may factor small shared helpers inside the Phase 88 checker if
  that keeps false-positive handling clear, but should avoid a broad shared
  rules registry unless it materially reduces duplication without creating a
  second source of truth.
- No Rust source changes are expected. If planning discovers a narrow Rust gap,
  update `docs/parity/source-breadcrumbs.json` for any new first-party Rust
  source or test files under `packages/open-bitcoin-*/src` or
  `packages/open-bitcoin-*/tests`.

### Deferred Ideas (OUT OF SCOPE)

None — discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| REL-02 | Deterministic verification fails if release docs claim production full-node readiness without the required v1.8 evidence gates. [VERIFIED: .planning/REQUIREMENTS.md] | Use a new Bun checker that scans curated release/operator docs for positive production-readiness contexts and permits only scoped/no-claim wording. [VERIFIED: 88-CONTEXT.md, docs/parity/production-claim-boundary.md, scripts/check-phase82-production-claim-boundary.ts] |
| REL-03 | Deterministic verification fails if docs imply deferred surfaces are production-ready, including inbound serving, relay, production-funds wallet use, migration apply mode, signed packaging, hosted dashboards, GUI parity, public-network CI, destructive repair, or automatic support-bundle upload. [VERIFIED: .planning/REQUIREMENTS.md] | Use the Phase 82 deferred inventory plus Phase 83 support-promotion invariants to fail positive promotion predicates on deferred surfaces. [VERIFIED: docs/parity/production-claim-boundary.md, docs/parity/support-matrix.md, scripts/check-phase83-support-matrix-issue-evidence.ts] |
| REL-04 | Default `bash scripts/verify.sh` runs the v1.8 release-boundary checker while keeping public-network, real service-manager, and multi-day checks opt-in. [VERIFIED: .planning/REQUIREMENTS.md] | Wire the Phase 88 test and checker immediately after Phase 87 in both the heredoc and executed `run_step` sequence, then verify executable text excludes forbidden live/service/long-run drift. [VERIFIED: 88-CONTEXT.md, scripts/check-phase87-release-readiness.ts, scripts/verify.sh] |
</phase_requirements>

## Summary

Phase 88 should be planned as a repo-local Bun/TypeScript checker plus fixture tests, not as Rust work or an external-docs crawler. [VERIFIED: 88-CONTEXT.md, AGENTS.md, scripts/check-phase82-production-claim-boundary.ts, scripts/check-phase87-release-readiness.ts] The core implementation is deterministic text and JSON validation over a curated release/operator corpus, using existing v1.8 docs as the source of truth for allowed production-related language, deferred surfaces, and evidence-gate semantics. [VERIFIED: docs/parity/production-claim-boundary.md, docs/parity/support-matrix.md, docs/parity/release-readiness.md]

The checker must be broader than Phase 82's exact overclaim smoke tests but narrower than an all-doc scanner. [VERIFIED: 88-CONTEXT.md, scripts/check-phase82-production-claim-boundary.ts] Plan for a hybrid approach: exact phrase denylist coverage, normalized context scanning over sentences/paragraphs/table rows, and explicit allow rules for negative, historical, opt-in UAT, deferred, unsupported, or future-gate contexts. [VERIFIED: 88-CONTEXT.md, docs/parity/release-readiness.md, docs/parity/support-matrix.md]

**Primary recommendation:** Implement `scripts/check-phase88-deterministic-claim-guardrails.ts` with fixture tests and verifier wiring after Phase 87; update parity roots only enough to make the new guardrail auditable, without introducing a separate evidence manifest. [VERIFIED: 88-CONTEXT.md, docs/parity/index.json, scripts/verify.sh]

## Project Constraints (from AGENTS.md)

- Use `bash scripts/verify.sh` as the repo-native verification contract, including the Bazel smoke build. [VERIFIED: AGENTS.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts and prefer TypeScript for substantial script logic. [VERIFIED: AGENTS.md]
- Keep Bash for thin orchestration wrappers and simple shell checks. [VERIFIED: AGENTS.md]
- Do not add direct repo edits outside the GSD workflow unless explicitly bypassed; this phase is already inside GSD phase research. [VERIFIED: AGENTS.md, 88-CONTEXT.md]
- Treat `docs/metrics/lines-of-code.md` as a tracked generated artifact that may change after verification regenerates it. [VERIFIED: AGENTS.md]
- Record in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md]
- Add parity breadcrumbs only for new first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`; Phase 88 expects no Rust changes. [VERIFIED: AGENTS.md, 88-CONTEXT.md]
- After substantial parity, operator-surface, or workflow changes, check whether contributor-facing README files need updates. [VERIFIED: AGENTS.md]
- Follow Bright Builds TypeScript guidance: prefer Bun in Bun-friendly repo automation, keep logic as data-in/data-out functions where practical, avoid project-defined class inheritance, and use `maybe` names for nullable internal values. [VERIFIED: AGENTS.bright-builds.md, standards/languages/typescript-javascript.md]
- Follow Bright Builds testing guidance: pure decision logic needs focused unit tests, one concern per unit test, and clear Arrange/Act/Assert sections where structure is not obvious. [VERIFIED: standards/core/testing.md]
- Follow Bright Builds verification guidance: prefer repo-owned verification entrypoints and run relevant checks before completion. [VERIFIED: standards/core/verification.md]
- No active standards override changes these rules. [VERIFIED: standards-overrides.md]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|---|---:|---|---|
| Bun runtime | 1.3.9 | Run the Phase 88 checker and `bun:test` fixture tests. [VERIFIED: .bun-version, `bun --version`] | Repo-owned automation already uses Bun/TypeScript checkers for Phases 82-87. [VERIFIED: AGENTS.md, scripts/check-phase82-production-claim-boundary.ts, scripts/check-phase87-release-readiness.ts] |
| TypeScript via Bun | Bundled with Bun | Implement deterministic text/JSON validation without adding a package manager or dependency install step. [VERIFIED: no `package.json` found, no lockfile found, existing checker imports use Bun/Node built-ins] | Existing checkers use `#!/usr/bin/env bun`, Node built-ins, exported functions, and `import.meta.main`. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts, scripts/check-phase87-release-readiness.ts] |
| `bun:test` | Bundled with Bun | Fixture-test temporary repos and failure messages. [VERIFIED: scripts/check-phase82-production-claim-boundary.test.ts, scripts/check-phase87-release-readiness.test.ts] | Existing checker tests use `afterEach`, `expect`, `test`, temp dirs, and `writeFile` fixtures. [VERIFIED: scripts/check-phase87-release-readiness.test.ts] |
| Bash `scripts/verify.sh` | repo file | Aggregate default verification and command-order contract. [VERIFIED: scripts/verify.sh] | Repo-local guidance names this as the verification contract. [VERIFIED: AGENTS.md] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|---|---:|---|---|
| Node.js | v24.13.0 | GSD tooling and general JS runtime availability context. [VERIFIED: `node --version`] | Not needed for the checker when Bun runs it, but available for GSD init and local tooling. [VERIFIED: gsd init command output, `node --version`] |
| Rust/Cargo | 1.94.1 | Full repo verification through `scripts/verify.sh`. [VERIFIED: `cargo --version`, `rustc --version`, rust-toolchain guidance in AGENTS.md] | Required when running the full verifier after implementation. [VERIFIED: scripts/verify.sh] |
| Bazel | 8.6.0 | Full verifier Bazel smoke build. [VERIFIED: `bazel --version`] | Required by full `bash scripts/verify.sh`, not by focused Phase 88 Bun tests. [VERIFIED: scripts/verify.sh] |
| cargo-llvm-cov | 0.8.5 | Full verifier coverage gate. [VERIFIED: `cargo llvm-cov --version`] | Required by full verifier in `--full` mode. [VERIFIED: scripts/verify.sh] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|---|---|---|
| New Phase 88 Bun checker | Extend Phase 87 checker | Rejected by locked decision D-01 because Phase 87 remains checklist scope and Phase 88 owns broad deterministic guardrails. [VERIFIED: 88-CONTEXT.md] |
| Curated release/operator scan | Whole-doc tree scan | Rejected by locked decisions D-02 and D-03 because `.planning/` histories and historical docs must remain discoverable without default false positives. [VERIFIED: 88-CONTEXT.md] |
| Existing canonical docs | New machine-readable v1.8 evidence manifest | Rejected by locked decision D-06 and Phase 82 checker precedent. [VERIFIED: 88-CONTEXT.md, scripts/check-phase82-production-claim-boundary.ts] |
| Deterministic string/context rules | NLP classifier or external service | The default verifier must remain deterministic, short-running, and local. [VERIFIED: 88-CONTEXT.md, scripts/verify.sh] |

**Installation:**

```bash
# No install step is needed for Phase 88 itself; Bun is already pinned and available.
bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts
bun run scripts/check-phase88-deterministic-claim-guardrails.ts
```

**Version verification:** No npm package versions need `npm view` because this repo has no `package.json`, no JS lockfile, and Phase 88 should not add external npm dependencies. [VERIFIED: `find . -maxdepth 2 -name package.json`, lockfile scan, AGENTS.md dependency policy]

## Architecture Patterns

### Recommended Project Structure

```text
scripts/
├── check-phase88-deterministic-claim-guardrails.ts        # exported checker plus CLI entrypoint [VERIFIED: 88-CONTEXT.md]
├── check-phase88-deterministic-claim-guardrails.test.ts   # Bun fixture tests [VERIFIED: 88-CONTEXT.md]
└── verify.sh                                             # heredoc and run_step wiring after Phase 87 [VERIFIED: scripts/verify.sh]

docs/parity/
├── index.json                                            # checklist/audit root entry if following prior phase pattern [VERIFIED: docs/parity/index.json, 88-CONTEXT.md]
├── checklist.md                                          # human checklist row [VERIFIED: docs/parity/checklist.md]
├── README.md                                             # compact pointer if needed [VERIFIED: docs/parity/README.md]
└── catalog/operator-runtime-release-hardening.md         # cross-phase catalog row [VERIFIED: 88-CONTEXT.md, docs/parity/catalog/operator-runtime-release-hardening.md]
```

### Pattern 1: Checker Function Shape

**What:** Export a pure checker function that returns a `string[]` of failures and accepts an optional repo-root override. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts, scripts/check-phase87-release-readiness.ts]

**When to use:** Use this for the main Phase 88 checker so fixture tests can pass temporary repo roots without mutating the real checkout. [VERIFIED: scripts/check-phase87-release-readiness.test.ts, 88-CONTEXT.md]

**Example:**

```typescript
// Source: scripts/check-phase87-release-readiness.ts
export function checkPhase88DeterministicClaimGuardrails(
  maybeRepoRoot = process.env.OPEN_BITCOIN_PHASE88_REPO_ROOT,
): string[] {
  const failures: string[] = [];
  // read curated files, verify docs, verify parity roots, verify verifier text
  return failures;
}
```

### Pattern 2: Curated Corpus With Context Windows

**What:** Scan only the release/operator corpus named by context, then classify risky terms by surrounding sentence, paragraph, or Markdown table row. [VERIFIED: 88-CONTEXT.md]

**When to use:** Use for REL-02 and REL-03 because exact bad phrases are supplemental and broad all-doc scanning is out of scope. [VERIFIED: 88-CONTEXT.md, scripts/check-phase82-production-claim-boundary.ts]

**Recommended corpus:** `README.md`, `docs/operator/runtime-guide.md`, `docs/parity/production-claim-boundary.md`, `docs/parity/support-matrix.md`, `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, and `docs/parity/catalog/operator-runtime-release-hardening.md`. [VERIFIED: 88-CONTEXT.md]

**Example:**

```typescript
// Source: Phase 88 context plus Phase 82/83 parser patterns
function contextUnits(text: string): string[] {
  return text
    .split(/\n{2,}|(?<=[.!?])\s+/)
    .map((unit) => unit.trim())
    .filter((unit) => unit.length > 0);
}
```

### Pattern 3: Scoped Allow Rules

**What:** Allow risky terms only when the same context unit is explicitly negative or scoped, such as `does not claim`, `not allowed yet`, `deferred`, `unsupported`, `historical`, `opt-in UAT`, `future gate`, or `outside default verification`. [VERIFIED: 88-CONTEXT.md]

**When to use:** Use for valid no-claim/deferred rows in the existing docs so Phase 88 fails promotions without failing required boundary language. [VERIFIED: docs/parity/production-claim-boundary.md, docs/parity/support-matrix.md, docs/parity/release-readiness.md]

**Example:**

```typescript
// Source: 88-CONTEXT.md allow-rule decision
const ALLOW_SCOPES = [
  "does not claim",
  "not allowed yet",
  "deferred",
  "unsupported",
  "historical",
  "opt-in UAT",
  "future gate",
  "outside default verification",
] as const;
```

### Pattern 4: Executable Verifier Text

**What:** Strip the visible `VERIFY_COMMAND_ORDER` heredoc before checking that commands are actually executed. [VERIFIED: scripts/check-phase87-release-readiness.ts, 88-CONTEXT.md]

**When to use:** Use for REL-04 so a command that exists only in the heredoc cannot satisfy Phase 88 verifier wiring. [VERIFIED: 88-CONTEXT.md, scripts/check-phase87-release-readiness.test.ts]

**Example:**

```typescript
// Source: scripts/check-phase87-release-readiness.ts
function executableVerifyText(text: string): string {
  return text.replace(/^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m, "");
}
```

### Pattern 5: Parity Root Registration

**What:** If following Phase 82-87 patterns, add one surface in `docs/parity/index.json`, one audit entry, one human checklist row, and one catalog row. [VERIFIED: docs/parity/index.json, docs/parity/checklist.md, docs/parity/catalog/operator-runtime-release-hardening.md, 88-CONTEXT.md]

**When to use:** Use after the checker exists so docs can name `scripts/check-phase88-deterministic-claim-guardrails.ts`, its test, and `scripts/verify.sh` as evidence. [VERIFIED: scripts/check-phase87-release-readiness.ts, docs/parity/index.json]

### Anti-Patterns to Avoid

- **Exact-string-only scanner:** Phase 88 must catch obvious paraphrases and close variants, not only exact bad phrases. [VERIFIED: 88-CONTEXT.md]
- **All-doc-tree scanner:** Scanning `.planning/`, archives, or every historical doc is explicitly out of scope for default verification. [VERIFIED: 88-CONTEXT.md]
- **New v1.8 evidence manifest:** Existing docs are the source of truth, and Phase 82 already guards against adding a v1.8 evidence manifest. [VERIFIED: 88-CONTEXT.md, scripts/check-phase82-production-claim-boundary.ts]
- **Verifier heredoc false positive:** Checking only the visible command-order heredoc can miss missing executed `run_step` commands. [VERIFIED: scripts/check-phase87-release-readiness.ts, scripts/check-phase87-release-readiness.test.ts]
- **Runtime/live proof in default verifier:** Public-network checks, real service-manager operations, long sleeps, release-blocking live sync, support upload, or destructive repair text must fail if added to default verifier text. [VERIFIED: 88-CONTEXT.md, scripts/check-phase87-release-readiness.ts]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---|---|---|---|
| Evidence-gate source of truth | New evidence manifest or registry | Existing `production-claim-boundary.md`, `support-matrix.md`, and `release-readiness.md` | Locked decision D-06 keeps those docs authoritative. [VERIFIED: 88-CONTEXT.md] |
| Default readiness proof | Live public-network, real service-manager, or multi-day verifier work | Deterministic text/JSON checker run by `bash scripts/verify.sh` | REL-04 requires default verification to keep those checks opt-in. [VERIFIED: .planning/REQUIREMENTS.md, 88-CONTEXT.md] |
| Production-claim semantics | Ad hoc marketing-language review | Curated phrase/predicate lists plus scoped allow rules | Phase 88 must fail obvious paraphrases while preserving valid no-claim wording. [VERIFIED: 88-CONTEXT.md] |
| Markdown data extraction | Whole Markdown parser dependency | Small local table/context helpers matching existing checkers | The repo has no JS package install surface and existing checkers use lightweight local helpers. [VERIFIED: no package.json found, scripts/check-phase82-production-claim-boundary.ts, scripts/check-phase83-support-matrix-issue-evidence.ts] |
| Parity traceability | Separate docs status source | `docs/parity/index.json`, `checklist.md`, catalog row | Prior v1.8 phases register surfaces and evidence through these roots. [VERIFIED: docs/parity/index.json, docs/parity/checklist.md] |

**Key insight:** The hard part is not file discovery; it is preserving no-claim language while failing positive promotion language. [VERIFIED: 88-CONTEXT.md, docs/parity/production-claim-boundary.md, docs/parity/support-matrix.md]

## Common Pitfalls

### Pitfall 1: Exact Bad Phrases Miss Paraphrases

**What goes wrong:** A doc can avoid exact strings like `Open Bitcoin is production full-node ready` while still saying a deferred surface is `production-grade`, `fully supported`, `GA`, `certified`, or `default-verified`. [VERIFIED: 88-CONTEXT.md, scripts/check-phase82-production-claim-boundary.ts, scripts/check-phase83-support-matrix-issue-evidence.ts]
**Why it happens:** Phase 82 has exact overclaim smoke checks, but Phase 88 has broader REL-02/REL-03 guardrail scope. [VERIFIED: 88-CONTEXT.md, scripts/check-phase82-production-claim-boundary.ts]
**How to avoid:** Combine exact denylist checks with normalized context scanning for deferred surfaces plus promotion predicates. [VERIFIED: 88-CONTEXT.md]
**Warning signs:** Tests only append exact denied sentences and do not test paraphrased deferred-surface promotions. [VERIFIED: scripts/check-phase82-production-claim-boundary.test.ts, 88-CONTEXT.md]

### Pitfall 2: Valid No-Claim Text Fails The Scanner

**What goes wrong:** Required docs repeatedly mention production readiness and deferred surfaces in negative/scoped language. [VERIFIED: docs/parity/production-claim-boundary.md, docs/parity/release-readiness.md, docs/parity/support-matrix.md]
**Why it happens:** A simple term grep cannot distinguish "does not claim production full-node readiness" from a positive claim. [VERIFIED: 88-CONTEXT.md]
**How to avoid:** Inspect the sentence, paragraph, or table row and allow explicitly scoped terms such as `does not claim`, `deferred`, `unsupported`, `historical`, `opt-in UAT`, `future gate`, and `outside default verification`. [VERIFIED: 88-CONTEXT.md]
**Warning signs:** The real current docs fail immediately before any fixture mutation. [VERIFIED: scripts/check-phase87-release-readiness.test.ts pattern]

### Pitfall 3: Scanning Historical Planning Evidence

**What goes wrong:** Historical claims in `.planning/` or milestone archives become default-verifier blockers. [VERIFIED: 88-CONTEXT.md]
**Why it happens:** Whole-repo scans ignore the distinction between current release/operator surfaces and archived evidence. [VERIFIED: 88-CONTEXT.md, .planning/STATE.md]
**How to avoid:** Use the curated corpus from the context and keep historical scoped claims discoverable but non-blocking. [VERIFIED: 88-CONTEXT.md]
**Warning signs:** The checker walks directories recursively instead of reading fixed file constants. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts, scripts/check-phase87-release-readiness.ts]

### Pitfall 4: Verifier Wiring Exists Only In The Heredoc

**What goes wrong:** `scripts/verify.sh` shows the Phase 88 commands in the visible command-order block but does not execute them. [VERIFIED: 88-CONTEXT.md, scripts/check-phase87-release-readiness.test.ts]
**Why it happens:** Existing release-boundary checkers require a visible heredoc and executed `run_step` calls. [VERIFIED: scripts/verify.sh]
**How to avoid:** Strip the heredoc before checking executable text and assert Phase 88 test/checker order after Phase 87. [VERIFIED: scripts/check-phase87-release-readiness.ts, 88-CONTEXT.md]
**Warning signs:** Fixture test with heredoc-only Phase 88 commands passes. [VERIFIED: scripts/check-phase87-release-readiness.test.ts]

### Pitfall 5: Default Verification Drifts Into Opt-In UAT

**What goes wrong:** A future edit adds public-network, real service-manager, long sleep, support-upload, destructive-repair, or release-blocking live-sync behavior into default verification. [VERIFIED: 88-CONTEXT.md, docs/parity/support-matrix.md, scripts/check-phase87-release-readiness.ts]
**Why it happens:** Runtime guide UAT commands are intentionally copy-pasteable, and a naive verifier check may treat them as acceptable defaults. [VERIFIED: docs/operator/runtime-guide.md, 88-CONTEXT.md]
**How to avoid:** Check executable verifier text for forbidden command/text fragments while allowing those commands in docs as opt-in UAT. [VERIFIED: 88-CONTEXT.md, scripts/check-phase87-release-readiness.ts]
**Warning signs:** `scripts/verify.sh` executable text contains `run-live-mainnet-smoke`, `systemctl`, `launchctl`, `sleep 259200`, `--restart-after-progress`, package-manager service commands, public-network CI/default gates, support upload, or destructive repair. [VERIFIED: 88-CONTEXT.md]

## Code Examples

Verified patterns from local sources:

### Exported Checker And CLI

```typescript
// Source: scripts/check-phase87-release-readiness.ts
export function checkPhase88DeterministicClaimGuardrails(
  maybeRepoRoot = process.env.OPEN_BITCOIN_PHASE88_REPO_ROOT,
): string[] {
  const failures: string[] = [];
  return failures;
}

if (import.meta.main) {
  const failures = checkPhase88DeterministicClaimGuardrails();
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exitCode = 1;
  }
}
```

### Fixture Test Shape

```typescript
// Source: scripts/check-phase87-release-readiness.test.ts
test("fails_when_phase88_verify_commands_exist_only_in_legacy_heredoc", async () => {
  // Arrange
  const root = await createFixture({ maybeVerifyScript: heredocOnlyVerifyText() });

  // Act
  const failures = checkPhase88DeterministicClaimGuardrails(root);

  // Assert
  expect(failures.join("\n")).toContain("verifier-order");
});
```

### Markdown Table Row Context

```typescript
// Source: scripts/check-phase83-support-matrix-issue-evidence.ts
function splitMarkdownRow(line: string): string[] {
  return line
    .split("|")
    .slice(1, -1)
    .map((cell) => cell.trim());
}
```

### Verifier Heredoc Stripping

```typescript
// Source: scripts/check-phase87-release-readiness.ts
function executableVerifyText(text: string): string {
  return text.replace(/^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m, "");
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|---|---|---|---|
| Exact overclaim smoke checks in Phase 82. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts] | Hybrid exact denylist plus scoped context scanning for obvious paraphrases. [VERIFIED: 88-CONTEXT.md] | Phase 88 context gathered 2026-06-23. [VERIFIED: 88-CONTEXT.md] | Planner should include both exact bad-phrase fixtures and positive-promotion paraphrase fixtures. [VERIFIED: 88-CONTEXT.md] |
| Phase 87 release-readiness checklist checker. [VERIFIED: scripts/check-phase87-release-readiness.ts] | Separate Phase 88 broad deterministic claim guardrail checker. [VERIFIED: 88-CONTEXT.md] | Locked decision D-01. [VERIFIED: 88-CONTEXT.md] | Do not overload the completed Phase 87 checker. [VERIFIED: 88-CONTEXT.md] |
| Visible command-order checks can be fooled by heredoc-only entries. [VERIFIED: scripts/check-phase82-production-claim-boundary.test.ts] | Strip heredoc and assert executable `run_step` order. [VERIFIED: scripts/check-phase87-release-readiness.ts] | Existing Phase 87 pattern. [VERIFIED: scripts/check-phase87-release-readiness.ts] | Phase 88 should copy the executable-text pattern. [VERIFIED: 88-CONTEXT.md] |
| Historical docs remain release evidence. [VERIFIED: .planning/STATE.md, docs/parity/release-readiness.md] | Default claim scanner only blocks curated current release/operator surfaces. [VERIFIED: 88-CONTEXT.md] | Locked decisions D-02 and D-03. [VERIFIED: 88-CONTEXT.md] | Avoid false positives from `.planning/` and milestone archives. [VERIFIED: 88-CONTEXT.md] |

**Deprecated/outdated:**
- Treating artifact existence, daemon startup, elapsed time, peer reachability, raw log tail, service file existence, support-bundle path, or context-only records as sufficient proof is explicitly invalid for this milestone. [VERIFIED: 88-CONTEXT.md, docs/parity/production-claim-boundary.md, docs/parity/release-readiness.md]
- Treating public-network, real service-manager, or multi-day checks as default verifier requirements is explicitly outside v1.8. [VERIFIED: .planning/REQUIREMENTS.md, docs/parity/support-matrix.md, docs/operator/runtime-guide.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|---|---|---|

All claims in this research were verified against repo files, local command output, or GSD/user-provided phase context; no `[ASSUMED]` claims are intentionally present. [VERIFIED: full research source set]

## Open Questions (RESOLVED)

1. **Should Phase 88 add a parity root surface and audit entry, or only a checker?** [VERIFIED: 88-CONTEXT.md]
   - What we know: Context says `docs/parity/index.json`, `docs/parity/checklist.md`, and the operator-runtime catalog should record a Phase 88 parity surface if implementation follows prior phase patterns. [VERIFIED: 88-CONTEXT.md]
   - What's unclear: The exact surface id and audit key are not locked in context. [VERIFIED: 88-CONTEXT.md]
   - Recommendation: Use `v1-8-deterministic-claim-guardrails` and `v1_8_deterministic_claim_guardrails` to match existing v1.8 naming. [VERIFIED: docs/parity/index.json]
   - RESOLVED: Add both the parity root surface and audit entry, using `v1-8-deterministic-claim-guardrails` and `v1_8_deterministic_claim_guardrails`. This keeps Phase 88 auditable through the same parity roots as prior v1.8 phases without introducing a new evidence manifest. [VERIFIED: 88-CONTEXT.md, 88-01-PLAN.md, 88-02-PLAN.md]

2. **How broad should the curated corpus be beyond canonical refs?** [VERIFIED: 88-CONTEXT.md]
   - What we know: Context names release roots, parity entrypoints, runtime guide, and relevant parity catalog pages; canonical refs name the concrete files. [VERIFIED: 88-CONTEXT.md]
   - What's unclear: Whether P2P/chainstate/wallet/drop-in catalogs should be included in Phase 88 default scan. [VERIFIED: 88-CONTEXT.md]
   - Recommendation: Start with the files listed in canonical refs for Phase 88 and do not add subsystem catalogs unless fixture or current-doc inspection exposes a current public-operator claim there. [VERIFIED: 88-CONTEXT.md, scripts/check-phase82-production-claim-boundary.ts]
   - RESOLVED: Use the Phase 88 canonical release/operator corpus only: README, runtime guide, production claim boundary, support matrix, release readiness, deviations, parity index/checklist/README, operator-runtime catalog, and `scripts/verify.sh`. Do not add subsystem catalogs or recursive docs scans unless a later scoped phase expands the current public-operator surface. [VERIFIED: 88-CONTEXT.md, 88-01-PLAN.md, 88-02-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|---|---|---:|---|---|
| Bun | Phase 88 checker and tests | yes | 1.3.9 | None needed. [VERIFIED: `.bun-version`, `bun --version`] |
| Bash | `scripts/verify.sh` | yes | GNU bash 3.2.57 | None needed. [VERIFIED: `bash --version`, scripts/verify.sh] |
| Git | `scripts/verify.sh` requirements and worktree review | yes | 2.53.0 | None needed. [VERIFIED: `git --version`, scripts/verify.sh] |
| grep | `scripts/verify.sh` requirements | yes | BSD grep 2.6.0-FreeBSD | None needed. [VERIFIED: `grep --version`, scripts/verify.sh] |
| Rust/Cargo | full repo verifier | yes | 1.94.1 | Focused Bun tests can run without full verifier, but closeout should use `bash scripts/verify.sh`. [VERIFIED: `cargo --version`, `rustc --version`, scripts/verify.sh] |
| cargo-llvm-cov | full repo verifier coverage | yes | 0.8.5 | Use `bash scripts/verify.sh --fast` only for iteration, not final contract. [VERIFIED: `cargo llvm-cov --version`, AGENTS.md, scripts/verify.sh] |
| Bazel | full repo verifier smoke build | yes | 8.6.0 | Use focused Bun tests for iteration; final default verifier still needs Bazel. [VERIFIED: `bazel --version`, AGENTS.md, scripts/verify.sh] |

**Missing dependencies with no fallback:** None found for Phase 88 planning. [VERIFIED: environment probe commands]

**Missing dependencies with fallback:** None found for Phase 88 planning. [VERIFIED: environment probe commands]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---|---:|---|
| V2 Authentication | no | Phase 88 does not add auth or identity flows; it scans tracked docs and scripts. [VERIFIED: 88-CONTEXT.md] |
| V3 Session Management | no | Phase 88 does not add session state. [VERIFIED: 88-CONTEXT.md] |
| V4 Access Control | no | Phase 88 does not add authorization decisions. [VERIFIED: 88-CONTEXT.md] |
| V5 Input Validation | yes | Parse `docs/parity/index.json` with `JSON.parse`, validate expected arrays/strings, and treat file contents as untrusted text scanned through deterministic rules. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts, scripts/check-phase87-release-readiness.ts] |
| V6 Cryptography | no | Phase 88 does not add cryptographic operations. [VERIFIED: 88-CONTEXT.md] |

### Known Threat Patterns for Bun/TypeScript Doc Checkers

| Pattern | STRIDE | Standard Mitigation |
|---|---|---|
| False release claim accepted because wording evades exact strings | Repudiation | Use curated context scanning plus exact denylist tests. [VERIFIED: 88-CONTEXT.md] |
| Valid no-claim docs rejected, leading contributors to remove necessary boundary language | Denial of Service | Allow scoped/negative contexts in sentence, paragraph, or row windows. [VERIFIED: 88-CONTEXT.md, docs/parity/release-readiness.md] |
| Default verifier silently stops executing the checker | Tampering | Strip heredoc and assert executed `run_step` order after Phase 87. [VERIFIED: scripts/check-phase87-release-readiness.ts, 88-CONTEXT.md] |
| Default verifier gains live public-network, service-manager, or multi-day behavior | Denial of Service | Fail forbidden executable verifier text. [VERIFIED: 88-CONTEXT.md, scripts/check-phase87-release-readiness.ts] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/88-deterministic-claim-guardrails/88-CONTEXT.md` - locked Phase 88 decisions, canonical refs, checker/test/wiring constraints.
- `.planning/REQUIREMENTS.md` - REL-02, REL-03, REL-04 requirement text and traceability.
- `.planning/ROADMAP.md` - Phase 88 goal, dependency, and success criteria.
- `.planning/STATE.md` - current v1.8 planning state and deterministic-verifier boundary reminders.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/languages/typescript-javascript.md` - repo-local and Bright Builds workflow constraints.
- `docs/parity/production-claim-boundary.md`, `docs/parity/support-matrix.md`, `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md` - canonical v1.8 claim/evidence/deferred-surface sources.
- `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, `README.md`, `docs/operator/runtime-guide.md`, `docs/parity/catalog/operator-runtime-release-hardening.md` - curated public/release/operator surfaces.
- `scripts/check-phase82-production-claim-boundary.ts`, `scripts/check-phase82-production-claim-boundary.test.ts`, `scripts/check-phase83-support-matrix-issue-evidence.ts`, `scripts/check-phase87-release-readiness.ts`, `scripts/check-phase87-release-readiness.test.ts`, `scripts/verify.sh` - local checker, fixture, and verifier wiring patterns.
- Local commands: `bun --version`, `node --version`, `bash --version`, `git --version`, `cargo --version`, `rustc --version`, `bazel --version`, `cargo llvm-cov --version`, `grep --version`.

### Secondary (MEDIUM confidence)

- None. This phase is codebase-local and did not require web or ecosystem discovery. [VERIFIED: 88-CONTEXT.md]

### Tertiary (LOW confidence)

- None. [VERIFIED: source review]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Bun/TypeScript, `bun:test`, and `scripts/verify.sh` are directly established by repo instructions, pinned files, environment probes, and existing checkers. [VERIFIED: AGENTS.md, .bun-version, scripts/check-phase87-release-readiness.ts]
- Architecture: HIGH - Phase 82-87 checkers provide direct local implementation patterns, and Phase 88 context locks the new checker/test/wiring approach. [VERIFIED: 88-CONTEXT.md, scripts/check-phase82-production-claim-boundary.ts, scripts/check-phase87-release-readiness.ts]
- Pitfalls: HIGH - Pitfalls are directly called out in locked decisions and existing checker fixture failures. [VERIFIED: 88-CONTEXT.md, scripts/check-phase87-release-readiness.test.ts]

**Research date:** 2026-06-23 [VERIFIED: environment current_date]
**Valid until:** 2026-07-23 for repo-local patterns unless Phase 88 context or v1.8 docs change first. [VERIFIED: .planning/STATE.md]
