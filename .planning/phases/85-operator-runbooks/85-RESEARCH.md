# Phase 85: Operator Runbooks - Research

**Researched:** 2026-06-22 [VERIFIED: current_date]
**Domain:** Source-built operator runbooks, long-run monitoring, no-progress diagnosis, recovery/escalation guidance, support-bundle timeline evidence, and deterministic docs drift checks. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts]

<user_constraints>
## User Constraints (from CONTEXT.md)

All content in this block is copied from `.planning/phases/85-operator-runbooks/85-CONTEXT.md`; treat it as locked user/project context for planning. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Preflight Runbook

- **D-01:** Create one canonical operator runbook surface that starts with a
  production-boundary preflight. The preflight must send readers through the
  Phase 82 production claim boundary, Phase 83 support matrix, and Phase 84
  upgrade/rollback policy before any long-running source-built operation.
- **D-02:** The preflight should be checklist-oriented but evidence-based. It
  must require the selected datadir, explicit source revision, repo-local
  verification status, Cargo or Bazel command form, config paths, current
  status evidence, resource/disk review, service state or unavailable reason,
  wallet scope, and support-bundle availability.
- **D-03:** Every operator command example must include repo-local Cargo and
  Bazel forms. Do not rely only on an installed `open-bitcoin` alias.
- **D-04:** The preflight must distinguish review-only evidence from mutation.
  Status, support bundles, config summaries, service state, and local report
  paths are acceptable evidence; source datadir, wallet, service, and config
  mutation remains outside this phase.

### Long-Run Monitoring And No-Progress Diagnosis

- **D-05:** Long-run guidance should be organized around observable evidence:
  sync status, progress credit, no-progress reasons, recovery evidence,
  resource-bound pressure, structured logs, metrics, support-bundle summaries,
  and soak or live-smoke reports where the operator explicitly opted in.
- **D-06:** No-progress diagnosis must avoid treating elapsed time, daemon
  startup, peer reachability, raw log tails, or report existence as sufficient
  proof. The runbook should require field-level evidence and unavailable-field
  reasons.
- **D-07:** Reuse existing v1.3 through v1.7 vocabulary for progress and
  recovery outcomes: `safe_retry`, `read_only_inspection`,
  `backup_then_rebuild`, `stop_and_escalate`, resource pressure, no-progress
  diagnosis, stalled subsystem, latest stop reason, and checkpoint timeline.
- **D-08:** Keep public-network, stay-current, and multi-day soak commands
  explicitly opt-in. The runbook may tell operators how to collect those
  reports, but it must state that default `bash scripts/verify.sh` remains
  deterministic, public-network-free, service-manager-free, and multi-day-free.

### Recovery And Escalation Guidance

- **D-09:** Recovery guidance should be decision-table based. `safe_retry`
  permits a bounded retry after preserving evidence; `read_only_inspection`
  means inspect without mutation; `backup_then_rebuild` means preserve backup
  and evidence before any future operator-decided rebuild workflow; and
  `stop_and_escalate` means stop normal attempts and attach redacted evidence.
- **D-10:** Destructive repair, source datadir mutation, external wallet
  mutation, service-manager mutation, config rewrite, and automatic rebuild are
  not part of Phase 85. The runbook can point to future gates but must not
  imply those actions are currently supported.
- **D-11:** Escalation thresholds should be practical and evidence-driven:
  repeated no-progress with typed cause, unavailable critical fields, recovery
  class requiring stop/escalate, resource pressure crossing documented bounds,
  inconsistent status/support evidence, or failure to collect the minimum
  redacted support-bundle timeline.
- **D-12:** Escalation language should tell operators what to stop, what to
  preserve, what to redact, and what exact commands produced the evidence. It
  should not promise response timelines, hosted support upload, or production
  service ownership.

### Support-Bundle Timeline

- **D-13:** The support-bundle runbook should define a redacted timeline shape:
  preflight evidence, command start, status snapshots, progress or
  no-progress events, resource/recovery events, support-bundle collection,
  operator action taken, final status, and escalation decision.
- **D-14:** The minimum useful bundle should mirror the Phase 83 issue-evidence
  checklist: redacted support JSON/Markdown when available, exact command
  output, bounded log summary, config summary, service state or unavailable
  reason, resource evidence, recovery/progress evidence, sync evidence,
  version/toolchain context, platform details, and exact repo-local
  reproduction command.
- **D-15:** Privacy and safety boundaries must be explicit. Do not ask for
  wallet private material, raw wallet files, RPC cookies, rpcpassword,
  rpcauth, raw datadirs, unredacted logs, raw unbounded logs, full sensitive
  peer tables, or automatic upload.

### Documentation And Verification Shape

- **D-16:** Prefer a canonical docs/parity runbook document, linked from the
  runtime guide, production boundary, support matrix, upgrade policy,
  release-readiness page, deviations register, parity README/checklist/index,
  README, and operator runtime catalog. Avoid duplicating the full runbook in
  README.
- **D-17:** If automation is added, keep it narrow to Phase 85: required
  runbook sections, exact support terms, required repo-local command forms,
  support-bundle evidence fields, forbidden mutation/upload claims, canonical
  links, and verifier wiring.
- **D-18:** Any Phase 85 checker must be Bun-backed like the Phase 82 through
  Phase 84 checkers, deterministic, public-network-free, real-service-manager-
  free, multi-day-free, and wired into `bash scripts/verify.sh`. It must not
  become the broad Phase 88 production-claim scanner.
- **D-19:** Final closeout should run `bash scripts/verify.sh`. Focused Bun
  checker/test commands may be used during iteration, but the phase
  verification evidence should cite the repo-native verifier.

### Folded Todos

No pending todos matched Phase 85.

### the agent's Discretion

- The planner may split work into canonical runbook content, parity/root link
  updates, deterministic checker and fixture tests, verifier wiring, and
  closeout evidence.
- The executor may decide whether the canonical runbook file is named
  `operator-runbooks.md` or a narrower filename, as long as there is one
  durable source of truth and entrypoints link to it.
- No Rust source changes are expected. If planning discovers a narrow Rust gap,
  update parity breadcrumbs for any new first-party Rust source or test files
  under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`.

### Deferred Ideas (OUT OF SCOPE)

None - discussion stayed within phase scope. Phase 86 owns service operation
expectations, Phase 87 owns the release-readiness checklist, and Phase 88 owns
the broad production-claim guardrail suite.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| RUN-01 | Operator can follow a production-boundary preflight runbook before long-running source-built node operation. [VERIFIED: .planning/REQUIREMENTS.md] | Use one canonical docs/parity runbook that begins with Phase 82, Phase 83, and Phase 84 boundary links and a review-only preflight checklist covering datadir, source revision, verifier status, Cargo/Bazel command form, config paths, status, resources, service state or unavailable reason, wallet scope, and support-bundle availability. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md] |
| RUN-02 | Operator can follow long-run operation, monitoring, no-progress diagnosis, recovery, and escalation runbooks using existing v1.3 through v1.7 evidence surfaces. [VERIFIED: .planning/REQUIREMENTS.md] | Organize runbooks around `OpenBitcoinStatusSnapshot`, resource bounds, progress credit, stall/no-progress diagnosis, recovery action classes, structured logs/metrics, opt-in soak/live-smoke reports, and escalation thresholds; do not treat time, startup, reachability, raw logs, or report existence as proof. [VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/operator-observability.md; VERIFIED: docs/operator/runtime-guide.md] |
| RUN-03 | Operator can collect a redacted support-bundle timeline and identify what evidence is sufficient for support triage. [VERIFIED: .planning/REQUIREMENTS.md] | Mirror the Phase 83 issue-evidence checklist and Phase 79 support-forensics model: redacted JSON/Markdown, exact commands, bounded logs, config/service/resource/recovery/progress/sync evidence, version/toolchain/platform context, timeline events, final status, and explicit unavailable reasons. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/operator-observability.md] |
</phase_requirements>

## Summary

Phase 85 should be planned as a documentation and deterministic-checker phase, not as runtime operation, repair, upload, service-manager, migration, or wallet implementation. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: .planning/ROADMAP.md] The primary deliverable should be one canonical runbook under `docs/parity/` that turns existing v1.3 through v1.7 evidence surfaces into incident-ready operator procedures: preflight, monitor, diagnose no-progress, recover or stop, collect a support-bundle timeline, and escalate with redacted field-level evidence. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/operator-observability.md]

The main implementation risk is accidental support expansion. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md] The runbooks must preserve the Phase 82 terms, the Phase 83 issue-evidence policy, and the Phase 84 no-hidden-mutation boundary while making public-network, real service-manager, and multi-day commands opt-in only. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md; VERIFIED: docs/operator/runtime-guide.md]

**Primary recommendation:** Plan four work slices: canonical `docs/parity/operator-runbooks.md`, link and parity-root updates, `scripts/check-phase85-operator-runbooks.ts` plus fixture tests, and verifier/closeout evidence through `bash scripts/verify.sh`. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/parity/index.json; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts; VERIFIED: scripts/verify.sh]

## Project Constraints (from AGENTS.md)

- Read `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant managed standards before planning or implementation recommendations. [VERIFIED: AGENTS.md; VERIFIED: AGENTS.bright-builds.md; VERIFIED: standards/index.md]
- Use `bash scripts/verify.sh` as the repo-native verification contract; `bash scripts/verify.sh --fast` is only for local iteration and the default command remains the pre-commit and release contract. [VERIFIED: AGENTS.md; VERIFIED: docs/operator/runtime-guide.md; VERIFIED: scripts/verify.sh]
- Provide repo-local Cargo and Bazel forms for operator/UAT workflows instead of relying only on an installed `open-bitcoin` alias. [VERIFIED: AGENTS.md; VERIFIED: docs/operator/runtime-guide.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation and TypeScript for substantial script logic; keep Bash for thin orchestration wrappers. [VERIFIED: AGENTS.md; VERIFIED: standards/languages/typescript-javascript.md]
- Treat `docs/metrics/lines-of-code.md` as an intentionally tracked generated artifact that may change when verification regenerates it. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh]
- Record in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md; VERIFIED: docs/parity/README.md]
- Add parity breadcrumbs only if new first-party Rust source or test files are created under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`; Phase 85 currently expects no Rust source changes. [VERIFIED: AGENTS.md; VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]
- After substantial operator-surface, parity, or workflow documentation changes, update relevant README pointers instead of duplicating full canonical docs. [VERIFIED: AGENTS.md; VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]
- Keep functional-core/imperative-shell boundaries if code is added; Phase 85 checker logic should be pure data-in/data-out helpers with a thin file-reading shell. [VERIFIED: standards/core/architecture.md; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts]
- Unit tests should be focused and use Arrange, Act, Assert comments unless the structure is unmistakable; existing Phase 84 Bun tests follow this pattern. [VERIFIED: standards/core/testing.md; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.test.ts]
- Project-local skill directories `.claude/skills` and `.agents/skills` were not present during research. [VERIFIED: find .claude/skills .agents/skills -maxdepth 2 -name SKILL.md]

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Markdown under `docs/parity/` | Repository docs surface, no package version. [VERIFIED: docs/parity/README.md] | Canonical Phase 85 operator runbook surface. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md] | Phase 82, Phase 83, and Phase 84 v1.8 canonical roots already live under `docs/parity/`, and Phase 85 context prefers a docs/parity runbook. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md; VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md] |
| `docs/parity/index.json` and `docs/parity/checklist.md` | Repository schema/status docs, no package version. [VERIFIED: docs/parity/index.json; VERIFIED: docs/parity/checklist.md] | Register `v1-8-operator-runbooks` as the Phase 85 machine-readable and human-readable parity root. [VERIFIED: docs/parity/index.json; VERIFIED: docs/parity/checklist.md] | Phase 82 through Phase 84 use this pattern for v1.8 surfaces and checker validation. [VERIFIED: docs/parity/index.json; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts] |
| Bun | 1.3.9. [VERIFIED: .bun-version; VERIFIED: bun --version] | Run standalone TypeScript checker and Bun fixture tests. [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.test.ts] | Repo guidance makes Bun canonical for owned automation, and Phase 82 through Phase 84 checkers are Bun-backed. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh] |
| Standalone TypeScript in `scripts/` | No `package.json` and no `tsconfig*.json` were found. [VERIFIED: find . -maxdepth 3 -name package.json -print; VERIFIED: find . -maxdepth 4 -name 'tsconfig*.json' -print] | Deterministic Phase 85 docs drift checker. [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts] | Existing checkers import Node built-ins directly and run through Bun without npm install or project TypeScript config. [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.test.ts] |
| `bash scripts/verify.sh` | GNU bash 3.2.57 available locally. [VERIFIED: bash --version; VERIFIED: scripts/verify.sh] | Final repo-native verification contract and checker integration point. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh] | Repo-local guidance and runtime docs require the default verifier for closeout evidence. [VERIFIED: AGENTS.md; VERIFIED: docs/operator/runtime-guide.md] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Rust/Cargo | `cargo 1.94.1`, `rustc 1.94.1`. [VERIFIED: rust-toolchain.toml; VERIFIED: cargo --version; VERIFIED: rustc --version] | Full verifier and operator command examples. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh] | Use in runbook command examples and final verification; do not plan Rust source work unless a narrow gap is discovered. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md] |
| Bazelisk / Bazel | Bazelisk 1.28.1 using Bazel 8.6.0. [VERIFIED: bazelisk version] | Bazel operator command examples and verifier smoke build. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh] | Include alongside Cargo forms for operator commands and rely on `scripts/verify.sh` for full smoke coverage. [VERIFIED: AGENTS.md; VERIFIED: docs/operator/runtime-guide.md] |
| Node.js | v24.13.0 available locally. [VERIFIED: node --version; VERIFIED: command -v node] | GSD tooling and local JSON support when needed. [VERIFIED: node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs init phase-op 85] | Do not add Node package dependencies; use Bun for repo-owned automation. [VERIFIED: AGENTS.md; VERIFIED: find . -maxdepth 3 -name package.json -print] |
| `cargo-llvm-cov` | 0.8.5. [VERIFIED: cargo llvm-cov --version; VERIFIED: scripts/verify.sh] | Full verifier coverage step. [VERIFIED: scripts/verify.sh] | Required by default full verification, not by the Phase 85 checker itself. [VERIFIED: scripts/verify.sh] |
| `rg`, `jq`, and Git | `rg 15.1.0`, `jq 1.7.1-apple`, Git 2.53.0. [VERIFIED: rg --version; VERIFIED: jq --version; VERIFIED: git --version] | Research, parity-root inspection, and diff review. [VERIFIED: local environment audit] | Use for planning and sanity checks; do not encode as new project dependencies unless already used by repo scripts. [VERIFIED: standards/core/verification.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| New `docs/parity/operator-runbooks.md` | Add runbook sections only inside `docs/operator/runtime-guide.md` | The runtime guide already contains raw workflow material, but Phase 85 context asks for one canonical durable runbook surface and pointer links to avoid duplication. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md] |
| Narrow Phase 85 checker | Manual review only | Manual review avoids script work, but Phase 82 through Phase 84 already guard v1.8 docs drift with focused Bun checkers; Phase 85 has high-risk forbidden claims worth deterministic checks. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts; VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md] |
| Narrow Phase 85 checker | Broad production-claim scanner | A broad scanner might catch more global prose drift, but Phase 88 explicitly owns broad production-claim guardrails. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: .planning/ROADMAP.md] |
| Existing support-bundle command | Automatic support-bundle upload | Upload, consent, privacy, retention, and transport are deferred and outside v1.8 Phase 85. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md] |

**Installation:**

```bash
# No new package installation is recommended for Phase 85.
# Use the checked-in docs, standalone Bun scripts, and repo-native verifier.
```

**Version verification:** No npm packages are recommended, so `npm view` is not applicable. [VERIFIED: find . -maxdepth 3 -name package.json -print] Local versions were verified with `.bun-version`, `bun --version`, `rust-toolchain.toml`, `cargo --version`, `rustc --version`, `bazelisk version`, `bash --version`, and `cargo llvm-cov --version`. [VERIFIED: .bun-version; VERIFIED: bun --version; VERIFIED: rust-toolchain.toml; VERIFIED: cargo --version; VERIFIED: rustc --version; VERIFIED: bazelisk version; VERIFIED: bash --version; VERIFIED: cargo llvm-cov --version]

## Architecture Patterns

### Recommended Project Structure

```text
docs/
├── parity/
│   ├── operator-runbooks.md              # canonical Phase 85 runbook [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]
│   ├── production-claim-boundary.md      # pointer only; preserve Phase 82 terms [VERIFIED: docs/parity/production-claim-boundary.md]
│   ├── support-matrix.md                 # pointer only; preserve Phase 83 evidence policy [VERIFIED: docs/parity/support-matrix.md]
│   ├── upgrade-and-rollback-policy.md    # pointer only; preserve Phase 84 no-hidden-mutation boundary [VERIFIED: docs/parity/upgrade-and-rollback-policy.md]
│   ├── release-readiness.md              # v1.8 handoff pointer [VERIFIED: docs/parity/release-readiness.md]
│   ├── deviations-and-unknowns.md        # deferred/non-claim pointer [VERIFIED: docs/parity/deviations-and-unknowns.md]
│   ├── README.md                         # parity root pointer [VERIFIED: docs/parity/README.md]
│   ├── checklist.md                      # human checklist row [VERIFIED: docs/parity/checklist.md]
│   └── index.json                        # machine-readable surface/checklist/audit entries [VERIFIED: docs/parity/index.json]
├── operator/
│   └── runtime-guide.md                  # practical workflow pointer, not duplicate full runbook [VERIFIED: docs/operator/runtime-guide.md]
scripts/
├── check-phase85-operator-runbooks.ts       # exported pure checker plus thin file shell [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts]
├── check-phase85-operator-runbooks.test.ts  # Bun fixture tests [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.test.ts]
└── verify.sh                                # runs test then checker after Phase 84 [VERIFIED: scripts/verify.sh]
```

### Pattern 1: Canonical Runbook Plus Pointer Links

**What:** Put the full Phase 85 preflight, long-run monitoring, no-progress diagnosis, recovery/escalation, support-bundle timeline, and privacy boundaries in one docs/parity file, then link to it from the runtime guide, v1.8 boundary docs, parity roots, README, and operator-runtime catalog. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]

**When to use:** Use this for all RUN-01 through RUN-03 content and keep README/runtime guide additions concise pointers or workflow bridges. [VERIFIED: .planning/REQUIREMENTS.md; VERIFIED: docs/operator/runtime-guide.md]

**Example:**

```markdown
## Production-Boundary Preflight

| Evidence | Required form | Mutation status | Unavailable handling |
| --- | --- | --- | --- |
| selected datadir | exact `--datadir` path | review-only | write `Unavailable: <reason>` |
| source revision | `git rev-parse HEAD` | review-only | stop preflight until known |
```

Source: Phase 85 locked preflight decisions and Phase 84 checklist style. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md]

### Pattern 2: Evidence-First Runbook Flow

**What:** Structure the runbook in the operational order operators need: preflight, start or resume long-run review, monitor field-level status, diagnose no-progress, choose recovery action class, collect support-bundle timeline, decide escalation. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md]

**When to use:** Use this instead of organizing by historical phase number; historical v1.3 through v1.7 phase names should be supporting evidence, not the operator-facing navigation model. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/parity/release-readiness.md]

**Evidence fields to center:** `progress_credit`, `last_useful_work`, `last_peer_contribution`, `expected_progress_window`, `no_progress_threshold`, `stall_diagnosis`, `no_progress_diagnosis`, `no_progress_next_action`, `latest_stop_reason`, `resource_bounds`, `resource_pressure`, and `recovery_evidence`. [VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/operator-observability.md]

### Pattern 3: Recovery Decision Table With Non-Mutation Guardrails

**What:** Use a decision table keyed by `safe_retry`, `read_only_inspection`, `backup_then_rebuild`, and `stop_and_escalate`, with columns for required evidence, allowed action, forbidden action, and escalation bundle content. [VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/storage-decision.md]

**When to use:** Use this for RUN-02 recovery/escalation so the runbook cannot imply destructive repair, source datadir mutation, external wallet mutation, service-manager mutation, config rewrite, or automatic rebuild. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md]

**Example:**

```markdown
| Recovery class | Required evidence | Allowed action | Forbidden action |
| --- | --- | --- | --- |
| `read_only_inspection` | `recovery_evidence`, cause, evidence basis, unavailable reasons | inspect status and preserve evidence | delete locks, rewrite stores, mutate services |
```

Source: Phase 77 recovery contract and Phase 85 recovery decision. [VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/storage-decision.md; VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]

### Pattern 4: Redacted Timeline Shape

**What:** Define a minimum redacted support-bundle timeline with preflight evidence, command start, status snapshots, progress/no-progress events, resource/recovery events, support-bundle collection, operator action, final status, and escalation decision. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]

**When to use:** Use this for RUN-03 and mirror the Phase 83 issue-evidence checklist and Phase 79 support-forensics sidecar. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/operator-observability.md]

**Minimum fields:** redacted `support-evidence.json`, redacted `support-evidence.md`, exact command output, bounded log summary, config summary, service state or unavailable reason, resource evidence, recovery/progress evidence, sync evidence, version/toolchain context, platform details, exact repo-local reproduction command, and `Unavailable: <reason>` for missing critical evidence. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/operator/runtime-guide.md]

### Pattern 5: Narrow Bun Checker With Fixture Tests

**What:** Implement an exported `checkPhase85OperatorRunbooks(maybeRepoRoot)` that reads a fixed target-file list, validates canonical runbook headings, required terms, command forms, timeline fields, forbidden claims, parity index/checklist/audit entries, human pointer links, and `scripts/verify.sh` wiring. [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.test.ts]

**When to use:** Use this for deterministic RUN-01 through RUN-03 drift protection, and keep broad all-doc claim scanning for Phase 88. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: .planning/ROADMAP.md]

**Example:**

```typescript
export function checkPhase85OperatorRunbooks(
  maybeRepoRoot = process.env.OPEN_BITCOIN_PHASE85_REPO_ROOT,
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<string, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyRunbook(texts.get(RUNBOOK_PATH) ?? "", failures);
  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyHumanRoots(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  return failures;
}
```

Source: Phase 84 checker structure. [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts]

### Anti-Patterns to Avoid

- **Duplicating the full runbook across README, runtime guide, release readiness, and support matrix:** This creates drift and conflicts with the locked single durable source-of-truth decision. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]
- **Treating artifact existence as proof:** Existing docs say support bundle existence, report existence, daemon startup, elapsed time, peer reachability, and raw logs are not sufficient by themselves. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/operator/runtime-guide.md]
- **Adding public-network, real service-manager, or multi-day checks to default verification:** Default `bash scripts/verify.sh` must remain deterministic, public-network-free, service-manager-free, and multi-day-free. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: scripts/verify.sh]
- **Promising support response timelines or hosted uploads:** Phase 85 escalation can define evidence sufficiency, not hosted support ownership, response SLAs, or upload automation. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/parity/support-matrix.md]
- **Turning `backup_then_rebuild` into permission for automated destructive repair:** Existing recovery and upgrade docs keep it as evidence and operator-decision guidance only. [VERIFIED: docs/architecture/storage-decision.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Support taxonomy | New labels such as `production-ready`, `best-effort`, `GA`, or `certified`. | Phase 82 exact terms: `supported`, `preview`, `opt-in UAT`, `unsupported`, `deferred`. | Phase 82 and Phase 83 lock the vocabulary for v1.8. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md] |
| Runtime truth model | New runbook-specific proof terms or renderer-local verdicts. | `OpenBitcoinStatusSnapshot` and existing status/support/soak/live-smoke fields. | Status, dashboard, support, RPC, metrics, logs, and live-smoke projections must preserve the shared snapshot contract. [VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/operator-observability.md] |
| Recovery semantics | New recovery classes or ad hoc repair labels. | `safe_retry`, `read_only_inspection`, `backup_then_rebuild`, `stop_and_escalate`. | Phase 77 and Phase 84 already define the recovery action classes and non-mutation boundaries. [VERIFIED: docs/architecture/storage-decision.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md] |
| Support timeline collection | Automatic upload, raw datadir capture, wallet file capture, RPC credential capture, or raw unbounded logs. | Existing local redacted support bundle and Phase 83 smallest-useful-evidence checklist. | Existing docs explicitly exclude wallet private material, raw wallet files, RPC cookies, `rpcpassword`, `rpcauth`, raw datadirs, unredacted logs, raw unbounded logs, sensitive peer tables, and automatic upload. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/operator/runtime-guide.md] |
| Default verification | Live public-network checks, real `systemctl`/`launchctl` actions, long sleeps, or multi-day soak gates. | Deterministic Bun checker/tests plus final `bash scripts/verify.sh`. | The default verifier is local/deterministic and existing Phase checkers reject public-network/service-manager/multi-day drift. [VERIFIED: scripts/verify.sh; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts] |
| Docs drift automation | A broad all-doc production-claim scanner in Phase 85. | Narrow Phase 85 checker; leave broad scanner to Phase 88. | Phase 85 context and roadmap assign broad production-claim guardrails to Phase 88. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: .planning/ROADMAP.md] |
| Automation runtime | New Python, npm package, or shell parser for substantial logic. | Standalone Bun/TypeScript checker matching Phase 82 through Phase 84. | Repo guidance makes Bun canonical for substantial automation and no `package.json` exists. [VERIFIED: AGENTS.md; VERIFIED: find . -maxdepth 3 -name package.json -print] |

**Key insight:** Phase 85 is an orchestration/documentation layer over existing evidence contracts, so planner tasks should assemble, link, and guard those contracts rather than implement new runtime behavior. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/operator/runtime-guide.md]

## Common Pitfalls

### Pitfall 1: Preflight Becomes Hidden Mutation

**What goes wrong:** The runbook asks operators to fix configs, rewrite datadirs, mutate wallets, install services, or rebuild stores as part of preflight. [VERIFIED: docs/parity/upgrade-and-rollback-policy.md; VERIFIED: docs/architecture/storage-decision.md]

**Why it happens:** Operator procedures often mix evidence collection with repair steps. [ASSUMED]

**How to avoid:** Mark preflight rows as review-only evidence and route any mutation to future scoped gates. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md]

**Warning signs:** Imperatives such as "rewrite", "delete", "repair", "clear", "apply", "upload", or "rebuild automatically" appear in the runbook. [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts]

### Pitfall 2: No-Progress Diagnosis Uses Time Or Logs As Proof

**What goes wrong:** The runbook treats elapsed time, daemon startup, peer reachability, raw log tails, report existence, or support-bundle existence as sufficient evidence. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md]

**Why it happens:** These signals are easy to see, but the existing contract requires typed fields and unavailable reasons. [VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/operator/runtime-guide.md]

**How to avoid:** Require `progress_credit`, `last_useful_work`, `no_progress_diagnosis`, `stall_diagnosis`, `resource_bounds`, `recovery_evidence`, and explicit `Unavailable: <reason>` entries. [VERIFIED: docs/architecture/status-snapshot.md]

**Warning signs:** A checklist item says a run "passed" because a process started, a file exists, a peer connected, or a timer elapsed. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/parity/production-claim-boundary.md]

### Pitfall 3: Public-Network UAT Drifts Into Default Verification

**What goes wrong:** The Phase 85 checker or docs tell `bash scripts/verify.sh` to run live smoke, service-manager, multi-day, current-tip timing, or large-disk checks. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: scripts/verify.sh]

**Why it happens:** The runbooks necessarily mention opt-in UAT commands, but the default verifier must remain deterministic. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/parity/support-matrix.md]

**How to avoid:** Put opt-in commands in runbook sections with explicit non-default language and make the checker reject `run-live-mainnet-smoke`, `systemctl`, `launchctl`, and multi-day sleeps in executable verifier text. [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts]

**Warning signs:** `scripts/verify.sh` includes `run-live-mainnet-smoke`, `--manual-peer`, `systemctl`, `launchctl`, `sleep 259200`, or similar live-operation text. [VERIFIED: scripts/check-phase79-diagnostics-support-bundle.ts; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts]

### Pitfall 4: Support-Bundle Timeline Leaks Secrets

**What goes wrong:** The runbook asks for raw wallet files, private material, RPC cookies, `rpcpassword`, `rpcauth`, raw datadirs, unredacted logs, raw unbounded logs, sensitive peer tables, or automatic upload. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/operator/runtime-guide.md]

**Why it happens:** Incident templates often ask for "all logs" or "full config" unless redaction rules are explicit. [ASSUMED]

**How to avoid:** Mirror the Phase 83 smallest-useful-redacted-evidence checklist and require bounded summaries plus unavailable reasons. [VERIFIED: docs/parity/support-matrix.md]

**Warning signs:** Phrases such as "attach raw datadir", "upload bundle automatically", "include rpcpassword", or "send wallet.dat" appear in docs or fixture text. [VERIFIED: scripts/check-phase79-diagnostics-support-bundle.ts; VERIFIED: docs/operator/runtime-guide.md]

### Pitfall 5: Link Metadata Gets Out Of Sync

**What goes wrong:** The canonical runbook exists but parity roots, README, release-readiness, runtime guide, and catalog pointers do not link to it. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts]

**Why it happens:** Existing v1.8 roots require both human pointer docs and machine-readable `index.json` entries. [VERIFIED: docs/parity/index.json; VERIFIED: docs/parity/checklist.md]

**How to avoid:** Add the `v1-8-operator-runbooks` surface, checklist row, audit entry, and pointer links in the same plan slice, then guard them in the checker. [VERIFIED: docs/parity/index.json; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts]

**Warning signs:** `docs/parity/operator-runbooks.md` appears in only one file, or the `index.json` audit entry is missing RUN-01/RUN-02/RUN-03. [VERIFIED: docs/parity/index.json; VERIFIED: .planning/REQUIREMENTS.md]

## Code Examples

Verified patterns from local sources:

### Runbook Section Skeleton

```markdown
# Operator Runbooks

Surface id: `v1-8-operator-runbooks`

## Scope And Non-Claims
Use this runbook with `production-claim-boundary.md`, `support-matrix.md`, and
`upgrade-and-rollback-policy.md`. It defines source-built operator procedures,
not production full-node readiness, service ownership, destructive repair, or
automatic upload.

## Production-Boundary Preflight
| Evidence to record | How to collect it | Mutation status | Escalation use |
| --- | --- | --- | --- |

## Long-Run Monitoring
## No-Progress Diagnosis
## Recovery And Stop Decisions
## Support-Bundle Timeline
## Escalation Evidence Thresholds
## Privacy And Safety Boundaries
```

Source: Phase 85 locked docs shape and Phase 84 policy structure. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md]

### Checker Constants

```typescript
const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE85_REPO_ROOT";
const SURFACE_ID = "v1-8-operator-runbooks";
const PHASE85_REQUIREMENTS = ["RUN-01", "RUN-02", "RUN-03"] as const;
const RUNBOOK_PATH = "docs/parity/operator-runbooks.md";
const PHASE84_CHECKER_COMMAND =
  "bun run scripts/check-phase84-upgrade-rollback-policy.ts";
const PHASE85_TEST_COMMAND =
  "bun test scripts/check-phase85-operator-runbooks.test.ts";
const PHASE85_CHECKER_COMMAND =
  "bun run scripts/check-phase85-operator-runbooks.ts";
```

Source: Phase 84 checker pattern adapted to Phase 85 scope. [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts; VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]

### Verifier Wiring

```bash
run_step "test Phase 85 operator runbooks checker" bun test scripts/check-phase85-operator-runbooks.test.ts
run_step "check Phase 85 operator runbooks" bun run scripts/check-phase85-operator-runbooks.ts
```

Source: `scripts/verify.sh` Phase 82 through Phase 84 checker/test order. [VERIFIED: scripts/verify.sh]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Historical v1.3 through v1.7 docs organized evidence by phase and feature. | Phase 85 should organize by operator incident flow while citing historical evidence fields. | Phase 85 planning scope on 2026-06-22. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md] | Operators get a runbook they can follow without reading every phase history. [VERIFIED: .planning/ROADMAP.md] |
| Raw logs, elapsed time, peer reachability, and report existence could be tempting proof shortcuts. | Field-level status, support, recovery, resource, progress, and unavailable-reason evidence is required. | v1.7 and v1.8 docs preserve this boundary. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md] | Runbooks should quote typed evidence instead of artifact presence. [VERIFIED: docs/architecture/status-snapshot.md] |
| Service operation, public-network, and multi-day review can be mistaken for default readiness gates. | Public-network, real service-manager, and multi-day work stays opt-in UAT outside `bash scripts/verify.sh`. | v1.7 and v1.8 release-boundary docs. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/parity/release-readiness.md] | Phase 85 checker should guard default verifier drift. [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts] |
| Upgrade/rollback docs could imply repair or source-state mutation. | Phase 84 keeps rollback, backup, and compatibility evidence no-hidden-mutation and source-built only. | Phase 84 completed before Phase 85. [VERIFIED: .planning/STATE.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md] | Phase 85 recovery guidance must stop at inspect, preserve, retry, back up, or escalate. [VERIFIED: docs/parity/upgrade-and-rollback-policy.md] |

**Deprecated/outdated:**

- Treating support-bundle existence as triage sufficiency is outdated for this repo; Phase 83 and Phase 79 require redacted field-level support evidence and support-forensics interpretation. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/operator/runtime-guide.md]
- Treating `open-bitcoin` installed aliases as sufficient operator examples is outdated for this repo; UAT docs require repo-local Cargo and Bazel command forms. [VERIFIED: AGENTS.md; VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]
- Treating destructive repair as a current recovery option is outdated for v1.8; destructive repair remains deferred. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Operator incident templates often ask for overly broad logs unless redaction rules are explicit. | Common Pitfalls | Low planning risk; the mitigation is already verified by repo support-bundle boundaries. |
| A2 | Operator procedures often mix evidence collection with repair steps. | Common Pitfalls | Low planning risk; the mitigation is already verified by Phase 84 no-hidden-mutation boundaries. |
| A3 | Support timeline leakage of wallet or RPC secrets maps to STRIDE Information Disclosure. | Security Domain | Low planning risk; the mitigation is verified by explicit redaction and forbidden-evidence rules. |
| A4 | Runbook language that implies destructive repair or source-state mutation maps to STRIDE Tampering. | Security Domain | Low planning risk; the mitigation is verified by Phase 84 no-hidden-mutation boundaries. |
| A5 | Treating artifact existence as proof maps to STRIDE Repudiation / Tampering. | Security Domain | Low planning risk; the mitigation is verified by field-level evidence requirements. |
| A6 | Default verifier live network, service-manager, or multi-day actions map to Denial of Service / operational safety risk. | Security Domain | Low planning risk; the mitigation is verified by default-verifier boundary checks. |
| A7 | STRIDE labels in this research are threat-model mapping judgments for a docs/checker phase. | Metadata | Low planning risk; Phase 85 mitigations do not depend on exact STRIDE taxonomy. |

## Open Questions (RESOLVED)

1. **RESOLVED: Canonical filename**
   - What we know: Phase 85 context allows `operator-runbooks.md` or a narrower filename if there is one durable source of truth. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]
   - Resolution: Use `docs/parity/operator-runbooks.md` because it is literal, discoverable, and matches the requested canonical role. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]

2. **RESOLVED: Exact checker string list**
   - What we know: Phase 85 checker should guard required sections, support terms, command forms, support-bundle fields, forbidden mutation/upload claims, canonical links, and verifier wiring. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]
   - Resolution: The final checker string list is locked by the Phase 85 plans: required sections, Phase 82 support terms, repo-local Cargo/Bazel status and support-bundle command forms, D-02 preflight fields, D-05/D-06 field-level evidence, D-07 checkpoint/stalled-subsystem vocabulary, D-08 opt-in public-network/stay-current/multi-day boundaries, D-09 recovery classes, D-13 timeline labels, D-14 minimum bundle fields, D-15 forbidden sensitive evidence and upload strings, canonical links, parity roots, human pointers, non-duplication of procedural tables, and executed verifier wiring after Phase 84. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: .planning/phases/85-operator-runbooks/85-01-PLAN.md; VERIFIED: .planning/phases/85-operator-runbooks/85-03-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Bun | Phase 85 checker/test execution. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md] | yes [VERIFIED: bun --version] | 1.3.9 [VERIFIED: bun --version] | None needed. [VERIFIED: AGENTS.md] |
| Bash | `scripts/verify.sh`. [VERIFIED: scripts/verify.sh] | yes [VERIFIED: bash --version] | GNU bash 3.2.57 [VERIFIED: bash --version] | None needed. [VERIFIED: scripts/verify.sh] |
| Cargo | Repo-local command examples and full verifier. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh] | yes [VERIFIED: cargo --version] | 1.94.1 [VERIFIED: cargo --version] | None needed. [VERIFIED: rust-toolchain.toml] |
| Rustc | Full verifier and Cargo build/test surface. [VERIFIED: scripts/verify.sh] | yes [VERIFIED: rustc --version] | 1.94.1 [VERIFIED: rustc --version] | None needed. [VERIFIED: rust-toolchain.toml] |
| Bazelisk / Bazel | Bazel command examples and verifier smoke build. [VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh] | yes [VERIFIED: bazelisk version] | Bazelisk 1.28.1, Bazel 8.6.0 [VERIFIED: bazelisk version] | None needed. [VERIFIED: scripts/verify.sh] |
| `cargo-llvm-cov` | Full verifier coverage step. [VERIFIED: scripts/verify.sh] | yes [VERIFIED: cargo llvm-cov --version] | 0.8.5 [VERIFIED: cargo llvm-cov --version] | None needed for full verification. [VERIFIED: scripts/verify.sh] |
| Node.js | GSD init tooling and local JSON tooling. [VERIFIED: node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs init phase-op 85] | yes [VERIFIED: node --version] | v24.13.0 [VERIFIED: node --version] | Use Bun for repo-owned scripts. [VERIFIED: AGENTS.md] |
| `rg`, `jq`, Git | Planning inspection and sanity checks. [VERIFIED: local environment audit] | yes [VERIFIED: rg --version; VERIFIED: jq --version; VERIFIED: git --version] | `rg` 15.1.0, `jq` 1.7.1-apple, Git 2.53.0 [VERIFIED: rg --version; VERIFIED: jq --version; VERIFIED: git --version] | Use standard shell reads if unavailable. [VERIFIED: standards/core/verification.md] |

**Missing dependencies with no fallback:** None found during research. [VERIFIED: local environment audit]

**Missing dependencies with fallback:** None found during research. [VERIFIED: local environment audit]

## Security Domain

Security enforcement is enabled by default because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | Indirectly. [CITED: https://owasp.org/www-project-application-security-verification-standard/; VERIFIED: docs/operator/runtime-guide.md] | Do not request RPC cookies, `rpcpassword`, `rpcauth`, or wallet private material in support timelines. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/operator/runtime-guide.md] |
| V3 Session Management | No new session handling. [CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/; VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md] | No Phase 85 runtime auth/session implementation is planned. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md] |
| V4 Access Control | Indirectly. [CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/; VERIFIED: docs/parity/production-claim-boundary.md] | Keep hosted dashboards, automatic upload, public RPC/admin ownership, and production service ownership out of scope. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md] |
| V5 Validation, Sanitization and Encoding | Yes for checker parsing and docs guardrails. [CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts] | Parse fixed Markdown/JSON targets with Bun/TypeScript, validate required fields/links/forbidden strings, and preserve exact command forms. [VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts] |
| V6 Stored Cryptography | No new cryptography. [CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/; VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md] | Do not add crypto primitives or signed bundle claims; checkpoint-chain evidence is local ordering/truncation evidence, not signing or authenticity proof. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/status-snapshot.md] |
| V7 Error Handling and Logging | Yes. [CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/; VERIFIED: docs/operator/runtime-guide.md] | Require bounded log summaries and unavailable reasons; do not request raw unbounded logs. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/operator/runtime-guide.md] |
| V8 Data Protection | Yes. [CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/; VERIFIED: docs/parity/support-matrix.md] | Require smallest useful redacted evidence and forbid wallet/private/RPC/datadir secret material. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/operator/runtime-guide.md] |
| V14 Configuration | Yes. [CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/; VERIFIED: docs/operator/runtime-guide.md] | Record config paths and summaries without rewriting `bitcoin.conf` or Open Bitcoin JSONC config. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md] |

OWASP lists ASVS 5.0.0 as the latest stable version, and the OWASP Developer Guide lists the ASVS category names used above. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/]

### Known Threat Patterns for Phase 85

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Support timeline leaks wallet or RPC secrets. | Information Disclosure. [ASSUMED] | Checker and docs must forbid wallet private material, raw wallet files, RPC cookies, `rpcpassword`, `rpcauth`, raw datadirs, unredacted logs, raw unbounded logs, sensitive peer tables, and automatic upload. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: docs/parity/support-matrix.md] |
| Runbook implies destructive repair or source-state mutation. | Tampering. [ASSUMED] | Keep recovery guidance limited to preserve evidence, bounded retry, read-only inspection, backup-before-future-rebuild, stop, and escalate. [VERIFIED: docs/architecture/storage-decision.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md] |
| Artifact existence is treated as proof of progress or stability. | Repudiation / Tampering. [ASSUMED] | Require field-level evidence, support-forensics verdicts, status snapshot fields, and unavailable reasons. [VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/operator/runtime-guide.md] |
| Default verifier performs live network/service-manager/multi-day actions. | Denial of Service / Operational Safety. [ASSUMED] | Keep Phase 85 checker deterministic and reject forbidden live-operation command strings in executable verifier text. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/85-operator-runbooks/85-CONTEXT.md` - locked Phase 85 decisions, discretion, deferred scope, canonical refs, and existing code insights. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md]
- `.planning/REQUIREMENTS.md` - RUN-01, RUN-02, RUN-03, v1.8 out-of-scope boundaries, and traceability. [VERIFIED: .planning/REQUIREMENTS.md]
- `.planning/ROADMAP.md` - Phase 85 goal, dependencies, and success criteria. [VERIFIED: .planning/ROADMAP.md]
- `.planning/STATE.md` - Phase 85 current position and recent Phase 84 completion state. [VERIFIED: .planning/STATE.md]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and managed standards pages - repo-local workflow, Bun/Rust/testing/verification constraints. [VERIFIED: AGENTS.md; VERIFIED: AGENTS.bright-builds.md; VERIFIED: standards/core/architecture.md; VERIFIED: standards/core/code-shape.md; VERIFIED: standards/core/verification.md; VERIFIED: standards/core/testing.md; VERIFIED: standards/languages/typescript-javascript.md; VERIFIED: standards/languages/rust.md]
- `docs/parity/production-claim-boundary.md` - Phase 82 support terms, claim-to-evidence matrix, and deferred surfaces. [VERIFIED: docs/parity/production-claim-boundary.md]
- `docs/parity/support-matrix.md` - Phase 83 support classification, issue-evidence checklist, redaction boundaries, and contributor update rules. [VERIFIED: docs/parity/support-matrix.md]
- `docs/parity/upgrade-and-rollback-policy.md` - Phase 84 pre-upgrade, recovery, rollback, failed-upgrade, and no-hidden-mutation boundaries. [VERIFIED: docs/parity/upgrade-and-rollback-policy.md]
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, and `docs/architecture/operator-observability.md` - operator command forms, status/evidence fields, resource/recovery/progress semantics, support-bundle forensics, and opt-in UAT boundaries. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/operator-observability.md]
- `scripts/check-phase82-production-claim-boundary.ts`, `scripts/check-phase83-support-matrix-issue-evidence.ts`, `scripts/check-phase84-upgrade-rollback-policy.ts`, and matching tests - narrow Bun checker patterns. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.test.ts]
- `scripts/verify.sh` - repo-native verifier command order and integration point. [VERIFIED: scripts/verify.sh]

### Secondary (MEDIUM confidence)

- OWASP ASVS project page - latest stable ASVS 5.0.0 and standard purpose. [CITED: https://owasp.org/www-project-application-security-verification-standard/]
- OWASP Developer Guide ASVS page - ASVS category names used in the Security Domain section. [CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/]

### Tertiary (LOW confidence)

- Common operator-template behavior around broad log requests and evidence/repair mixing is based on general operational experience and is explicitly marked in the assumptions log. [ASSUMED]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - The phase uses existing repo docs, Bun TypeScript checkers, and `scripts/verify.sh`; all relevant local tool versions were probed. [VERIFIED: .planning/phases/85-operator-runbooks/85-CONTEXT.md; VERIFIED: bun --version; VERIFIED: cargo --version; VERIFIED: bazelisk version; VERIFIED: scripts/verify.sh]
- Architecture: HIGH - Phase 82 through Phase 84 provide current local patterns for canonical docs/parity roots, pointer docs, parity metadata, and narrow checker wiring. [VERIFIED: docs/parity/index.json; VERIFIED: scripts/check-phase84-upgrade-rollback-policy.ts; VERIFIED: scripts/verify.sh]
- Pitfalls: HIGH for repo-specific pitfalls and LOW for two general operator-template explanations - the mitigations are verified by local docs, while the generic causes are assumed and logged. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/parity/upgrade-and-rollback-policy.md; ASSUMED]
- Security domain: MEDIUM - ASVS categories were verified from official OWASP pages, while STRIDE labels are threat-model mapping judgments for a docs/checker phase. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://devguide.owasp.org/en/03-requirements/05-asvs/; ASSUMED]

**Research date:** 2026-06-22 [VERIFIED: current_date]
**Valid until:** 2026-07-22 for local docs/checker architecture; re-check ASVS/source docs sooner if security policy or v1.8 scope changes. [VERIFIED: .planning/config.json; CITED: https://owasp.org/www-project-application-security-verification-standard/]
