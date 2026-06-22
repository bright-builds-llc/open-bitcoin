# Phase 84: Upgrade and Rollback Policy - Research

**Researched:** 2026-06-21 [VERIFIED: current_date]
**Domain:** Source-built operator upgrade policy, rollback boundaries, backup expectations, state/schema compatibility evidence, and deterministic docs drift checks. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]

<user_constraints>
## User Constraints (from CONTEXT.md) [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Upgrade Preflight Checklist

- **D-01:** Create one canonical upgrade and rollback policy under `docs/parity/`
  for source-built installs. The policy should be operator-facing, quiet, and
  explicit that v1.8 defines upgrade boundaries rather than a production
  full-node readiness claim.
- **D-02:** The pre-upgrade checklist must cover: current source revision or
  commit, repo-local verification status, binary provenance from Cargo or
  Bazel, Open Bitcoin JSONC config path, `bitcoin.conf` path, selected datadir,
  datadir ownership and free-space review, current sync/status evidence,
  support-bundle evidence when available, service state, wallet scope, and
  backup location.
- **D-03:** Operator commands in the policy must prefer repo-local forms:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`. Do not rely
  only on an installed `open-bitcoin` alias.
- **D-04:** The checklist should distinguish review-only evidence from mutation.
  Collecting status, support bundles, config summaries, or service state is
  acceptable; source datadir, wallet, service, and config mutation requires a
  future scoped plan.

### State And Schema Compatibility Outcomes

- **D-05:** Reuse the existing recovery vocabulary instead of inventing upgrade
  labels. The policy should map `clean_shutdown`, `unclean_shutdown`,
  `incompatible_schema`, `store_corruption`, `storage_lock_contention`,
  `schema_mismatch`, `corruption_marker`, `corrupt_record`, `partial_write`,
  `unreadable_namespace`, `backend_open_failure`, and the action classes
  `safe_retry`, `read_only_inspection`, `backup_then_rebuild`, and
  `stop_and_escalate`.
- **D-06:** Compatibility guidance should be decision-table based: when the
  evidence points to safe retry, retry the source-built run; when it points to
  read-only inspection, inspect and preserve evidence; when it points to
  backup-then-rebuild, preserve a backup before rebuild; when it points to
  stop-and-escalate, stop and attach redacted evidence.
- **D-07:** A schema or storage compatibility outcome is not proven by daemon
  startup, elapsed time, peer reachability, raw logs, or report existence alone.
  The policy must require field-level evidence and unavailable-field reasons
  where applicable.
- **D-08:** The policy should explicitly separate Open Bitcoin-owned durable
  store state from external Core/Knots source datadirs and wallets. External
  state remains high-value input and must not be rewritten as part of rollback
  policy.

### Rollback And Failed-Upgrade Boundary

- **D-09:** Failed-upgrade guidance should prioritize evidence preservation:
  stop the attempted upgraded process, record exact command and commit, collect
  redacted local evidence, preserve backups, and avoid repeated mutation until
  compatibility class is understood.
- **D-10:** Rollback guidance should be source-built and local-first: return to
  the previous checked-out source revision or known binary, use the same
  explicit datadir and config paths, verify with repo-local commands, and
  record the rollback evidence. Do not imply package-manager rollback, signed
  release channels, or automatic update behavior.
- **D-11:** The policy must not recommend hidden mutation of source datadirs,
  external wallets, service files, launchd/systemd state, `bitcoin.conf`, or
  Open Bitcoin JSONC config. Any mutation guidance must be explicit, future
  gated, and outside this phase unless already supported by existing docs.
- **D-12:** Destructive repair stays deferred. `backup_then_rebuild` is evidence
  and operator-decision guidance, not permission for automated destructive
  rebuild or repair.

### Release Readiness And Verification Drift Checks

- **D-13:** Link the upgrade policy from the production claim boundary, support
  matrix, release-readiness document, deviations register, parity README,
  parity checklist, parity index, README, runtime guide, and relevant parity
  catalogs without creating a second support matrix.
- **D-14:** Add a narrow Phase 84 deterministic checker only for upgrade-policy
  drift: required policy sections, exact support terms, backup/rollback
  boundaries, forbidden hidden-mutation language, required repo-local command
  forms, canonical links, and verifier wiring.
- **D-15:** The checker must be deterministic, public-network-free,
  real-service-manager-free, multi-day-free, and Bun-backed like the Phase 82
  and Phase 83 checkers. It must not become the broad all-doc production claim
  scanner owned by Phase 88.
- **D-16:** Final closeout should run `bash scripts/verify.sh`. Focused Bun
  checker/test commands may be used during iteration, but the phase verification
  evidence should cite the repo-native verifier.

### the agent's Discretion

- The planner may split work into canonical policy content, parity/root link
  updates, deterministic checker and fixture tests, verifier wiring, and
  closeout evidence.
- The executor may decide whether the upgrade policy is a new
  `docs/parity/upgrade-and-rollback-policy.md` file or a narrower filename with
  the same canonical role, as long as all entrypoints link to one source of
  truth.
- No Rust source changes are expected. If planning discovers a narrow Rust gap,
  update parity breadcrumbs for any new first-party Rust source or test files
  under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`.

### Deferred Ideas (OUT OF SCOPE)

- Migration apply mode, destructive repair, package-manager rollback, signed
  release channels, automatic update channels, production-funds wallet support,
  and automatic backup/restore remain future scoped milestones.
</user_constraints>

<phase_requirements>
## Phase Requirements [VERIFIED: .planning/REQUIREMENTS.md]

| ID | Description | Research Support |
|----|-------------|------------------|
| UPG-01 | Operator can follow a pre-upgrade checklist covering backups, source-built binaries, config files, datadir ownership, service state, and current sync evidence. | Use one `docs/parity/` policy with a checklist that names commit/revision, repo-local verification, Cargo/Bazel provenance, Open Bitcoin JSONC path, `bitcoin.conf`, selected datadir, ownership/free space, status/support evidence, service state, wallet scope, and backup location. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| UPG-02 | Operator can understand state and schema compatibility expectations, including when upgrade, retry, rollback, backup-then-rebuild, or stop-and-escalate guidance applies. | Reuse the Phase 77 recovery evidence vocabulary and map it through decision-table rows instead of inventing upgrade-specific terms. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/storage-decision.md] |
| UPG-03 | Operator can follow rollback and failed-upgrade guidance without hidden source datadir, wallet, service, or config mutation. | Bind failed-upgrade and rollback guidance to evidence preservation, exact command/commit capture, same explicit datadir/config paths, and no hidden source datadir, external wallet, service, or config mutation. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/support-matrix.md] |
| UPG-04 | Contributor can run deterministic checks that fail when upgrade policy docs, rollback boundaries, or backup expectations drift out of the release-readiness contract. | Follow the Phase 82/83 checker pattern: Bun TypeScript script, exported pure checker function, fixture-based Bun tests, parity-root verification, canonical-link checks, forbidden-default-command checks, and `scripts/verify.sh` wiring. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.test.ts; VERIFIED: scripts/verify.sh] |
</phase_requirements>

## Summary

Phase 84 should be planned as a documentation-policy and deterministic-drift-check phase, not as runtime migration or repair implementation. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] The canonical deliverable should be one operator-facing policy under `docs/parity/`, linked from the existing v1.8 boundary roots and relevant catalogs, with no second support matrix and no broad production-claim scanner. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/README.md; VERIFIED: docs/parity/support-matrix.md]

The main implementation risk is semantic drift: wording that treats daemon startup, elapsed time, report existence, raw logs, peer reachability, or support-bundle existence as proof of upgrade compatibility would conflict with the existing field-level evidence model. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/architecture/status-snapshot.md] The second risk is hidden mutation: the current support matrix, migration catalog, wallet catalog, storage ADR, and runtime guide repeatedly preserve source datadirs, wallets, service registrations, configs, and destructive repair as explicit/future-gated surfaces. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/parity/catalog/drop-in-audit-and-migration.md; VERIFIED: docs/parity/catalog/wallet.md; VERIFIED: docs/architecture/storage-decision.md; VERIFIED: docs/operator/runtime-guide.md]

**Primary recommendation:** Plan four work slices: canonical `docs/parity/upgrade-and-rollback-policy.md`, link/metadata updates, `scripts/check-phase84-upgrade-rollback-policy.ts` plus fixture tests, and verifier/closeout evidence through `bash scripts/verify.sh`. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts; VERIFIED: scripts/verify.sh]

## Project Constraints (from AGENTS.md)

- Prefer `AGENTS.md` as the repo-local instruction entrypoint, then read `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant managed standards pages before planning or implementation recommendations. [VERIFIED: AGENTS.md; VERIFIED: AGENTS.bright-builds.md]
- Use `bash scripts/verify.sh` as the repo-native verification contract; `--fast` is local iteration only and the default command remains the pre-commit/release contract. [VERIFIED: AGENTS.md]
- Operator/UAT commands must use explicit repo-local Cargo and Bazel forms instead of relying only on an installed `open-bitcoin` alias. [VERIFIED: AGENTS.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts, prefer TypeScript for substantial script logic, and keep Bash for thin orchestration wrappers. [VERIFIED: AGENTS.md; VERIFIED: standards/languages/typescript-javascript.md]
- Treat `docs/metrics/lines-of-code.md` as an intentionally tracked generated artifact that may change when verification regenerates it. [VERIFIED: AGENTS.md; VERIFIED: README.md]
- Record in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md]
- Add parity breadcrumbs only if new first-party Rust source or test files are created under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`; Phase 84 currently expects no Rust source changes. [VERIFIED: AGENTS.md; VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]
- After substantial operator-surface or workflow changes, check whether README and relevant docs need updates. [VERIFIED: AGENTS.md]
- Keep functional-core/imperative-shell boundaries and pure business logic unit-tested when implementation touches code; the Phase 84 checker should keep decision logic in pure data-in/data-out helper functions with a thin file-reading shell. [VERIFIED: standards/core/architecture.md; VERIFIED: standards/core/testing.md; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]
- Unit tests should be focused and use clear Arrange, Act, Assert structure; the existing Phase 83 Bun tests use that structure. [VERIFIED: standards/core/testing.md; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.test.ts]
- Project-local skill directories `.claude/skills` and `.agents/skills` were not present during research. [VERIFIED: find .claude/skills .agents/skills -maxdepth 2 -name SKILL.md]

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Markdown under `docs/parity/` | Repository docs surface | Canonical operator-facing upgrade policy and parity-link root. | Phase 84 locks one canonical policy under `docs/parity/`, and v1.8 Phase 82/83 roots already live there. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md] |
| `docs/parity/index.json` and `docs/parity/checklist.md` | Repository schema | Machine-readable and human-readable parity roots for the new Phase 84 surface. | Phase 82/83 checkers already validate surface/checklist/audit entries and evidence arrays through these roots. [VERIFIED: docs/parity/index.json; VERIFIED: docs/parity/checklist.md; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts] |
| Bun | 1.3.9 | Run TypeScript checkers and Bun fixture tests. | `.bun-version` pins Bun 1.3.9, `bun --version` returned 1.3.9, and `scripts/verify.sh` runs the Phase 82/83 Bun checker/test pattern. [VERIFIED: .bun-version; VERIFIED: bun --version; VERIFIED: scripts/verify.sh] |
| TypeScript scripts in `scripts/` | No `package.json`; Bun-native | Deterministic docs drift checker. | The repo has no `package.json`, no `tsconfig*.json`, and the existing Phase 82/83 checkers are standalone TypeScript files run directly by Bun. [VERIFIED: find . -maxdepth 2 -name package.json -print; VERIFIED: find . -maxdepth 3 -name 'tsconfig*.json' -print; VERIFIED: scripts/check-phase82-production-claim-boundary.ts; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Rust/Cargo | 1.94.1 | Repo-native verification and existing operator binary command examples. | Use in final `bash scripts/verify.sh` and operator policy command forms, but do not plan Rust source work unless a narrow gap is discovered. [VERIFIED: rust-toolchain.toml; VERIFIED: cargo --version; VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| Bazel | 8.6.0 | Repo-local Bazel operator command examples and full verifier smoke path. | Use explicit `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...` forms in operator docs and rely on `scripts/verify.sh` for full Bazel smoke verification. [VERIFIED: bazel --version; VERIFIED: AGENTS.md; VERIFIED: scripts/verify.sh] |
| Pinned Bitcoin Knots baseline | `29.3.knots20260210` | Behavioral reference for datadir, wallet, config, service, and destructive-recovery caution. | Use as source context for operator policy boundaries, not as permission to implement Core/Knots destructive recovery behavior. [VERIFIED: AGENTS.md; VERIFIED: README.md; VERIFIED: packages/bitcoin-knots/doc/files.md; VERIFIED: packages/bitcoin-knots/src/init.cpp] |
| OWASP ASVS | 5.0.0 latest stable | Security mapping vocabulary if the planner needs ASVS references. | Use versioned ASVS identifiers when adding security notes because OWASP says identifiers can change and recommends `v<version>-...` references. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| One `docs/parity/` policy | Add upgrade sections across runtime guide, support matrix, release-readiness, and README only | This would violate the locked single-source-of-truth decision and make drift harder for a narrow checker to detect. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| Bun TypeScript checker | Shell `grep` checker | Existing Phase 82/83 drift checks parse Markdown/JSON and enforce ordering/content invariants in TypeScript; shell would be weaker for structured parity-root checks. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts] |
| Source-built rollback guidance | Package-manager rollback or signed channel rollback | Package-manager rollback, signed releases, and automatic update channels are explicitly deferred for Phase 84. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/production-claim-boundary.md] |
| Diagnosis-only `backup_then_rebuild` guidance | Automated destructive rebuild/repair | Destructive repair is deferred, and `backup_then_rebuild` is an operator-decision class rather than permission for automated repair. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/architecture/storage-decision.md] |

**Installation:**

```bash
# No new package installation is recommended for Phase 84.
# Use the repo-pinned Bun, Rust/Cargo, and Bazel surfaces already required by scripts/verify.sh.
```

**Version verification:** No npm packages are recommended, so `npm view` is not applicable. [VERIFIED: find . -maxdepth 2 -name package.json -print] Bun was verified from `.bun-version` and `bun --version`; Rust was verified from `rust-toolchain.toml`, `cargo --version`, and `rustc --version`; Bazel was verified with `bazel --version`. [VERIFIED: .bun-version; VERIFIED: bun --version; VERIFIED: rust-toolchain.toml; VERIFIED: cargo --version; VERIFIED: rustc --version; VERIFIED: bazel --version]

## Architecture Patterns

### Recommended Project Structure

```text
docs/
├── parity/
│   ├── upgrade-and-rollback-policy.md    # canonical Phase 84 operator policy [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]
│   ├── production-claim-boundary.md      # pointer only, no duplicate policy [VERIFIED: docs/parity/production-claim-boundary.md]
│   ├── support-matrix.md                 # pointer only, no second support matrix [VERIFIED: docs/parity/support-matrix.md]
│   ├── release-readiness.md              # v1.8 handoff pointer [VERIFIED: docs/parity/release-readiness.md]
│   ├── deviations-and-unknowns.md        # deferred/non-claim pointer [VERIFIED: docs/parity/deviations-and-unknowns.md]
│   ├── README.md                         # parity root pointer [VERIFIED: docs/parity/README.md]
│   ├── checklist.md                      # human checklist row [VERIFIED: docs/parity/checklist.md]
│   └── index.json                        # machine-readable surface/checklist/audit entries [VERIFIED: docs/parity/index.json]
├── operator/
│   └── runtime-guide.md                  # practical command pointer, not duplicate policy [VERIFIED: docs/operator/runtime-guide.md]
scripts/
├── check-phase84-upgrade-rollback-policy.ts       # exported pure checker + thin file shell [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]
├── check-phase84-upgrade-rollback-policy.test.ts  # Bun fixture tests [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.test.ts]
└── verify.sh                                      # runs test then checker after Phase 83 [VERIFIED: scripts/verify.sh]
```

### Pattern 1: Canonical Policy Plus Pointers

**What:** Put the upgrade/rollback decision tables, checklist, backup boundaries, failed-upgrade steps, and forbidden mutation rules in one canonical `docs/parity/` file, then link to it from the existing v1.8 roots and relevant catalog pages. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

**When to use:** Use this whenever a Phase 84 requirement would otherwise duplicate support classification, release-boundary, or operator command text across multiple docs. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/parity/README.md]

**Example:**

```markdown
## State And Schema Compatibility Decision Table

| Evidence observed | Compatibility class | Allowed next action | Forbidden hidden mutation | Escalation evidence |
| --- | --- | --- | --- | --- |
| `recovery_evidence.action_class` is `read_only_inspection` | inspect and preserve | inspect selected Open Bitcoin datadir read-only | deleting lock artifacts, changing stores, mutating source datadirs | redacted status/support evidence plus unavailable-field reasons |
```

Source: Phase 84 decision-table requirement and Phase 77 recovery terms. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/operator/runtime-guide.md]

### Pattern 2: Field-Level Evidence, Not Artifact Existence

**What:** Require concrete fields and unavailable-field reasons for compatibility outcomes, including recovery category/action/cause, sync heights, support-bundle fields, config paths, service state, version/commit/toolchain context, and exact repo-local command. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/architecture/status-snapshot.md]

**When to use:** Use this in pre-upgrade, failed-upgrade, rollback, backup-then-rebuild, and escalation sections. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

**Example:**

```markdown
Do not treat daemon startup, elapsed time, peer reachability, raw logs, report
existence, or support-bundle existence as compatibility proof. Record the field
that supports the decision or write `Unavailable: <reason>`.
```

Source: Existing evidence rules. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md]

### Pattern 3: Narrow Bun Checker With Pure Policy Functions

**What:** Implement `checkPhase84UpgradeRollbackPolicy(maybeRepoRoot)` as the exported pure entrypoint that reads a fixed target-file list, validates the canonical policy, validates parity metadata roots, validates links, rejects forbidden mutation/proof language, and validates `scripts/verify.sh` order. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]

**When to use:** Use this for UPG-04 drift protection; do not broaden it into Phase 88 all-doc production-claim scanning. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

**Example:**

```typescript
export function checkPhase84UpgradeRollbackPolicy(
  maybeRepoRoot = process.env.OPEN_BITCOIN_PHASE84_REPO_ROOT,
): string[] {
  const repoRoot =
    maybeRepoRoot === undefined ? DEFAULT_REPO_ROOT : path.resolve(maybeRepoRoot);
  const failures: string[] = [];
  const texts = new Map<string, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyPolicySections(texts.get(POLICY_PATH) ?? "", failures);
  verifyParityRoots(texts.get("docs/parity/index.json") ?? "", failures);
  verifyCanonicalLinks(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  return failures;
}
```

Source pattern: Phase 83 checker shape. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]

### Pattern 4: Fixture Tests Prove Checker Failures

**What:** Use temporary fixture roots with file maps, replacement/omission options, and `OPEN_BITCOIN_PHASE84_REPO_ROOT` to prove both passing fixtures and targeted drift failures. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.test.ts]

**When to use:** Use one focused test per invariant: missing checklist section, missing recovery term, forbidden hidden mutation language, missing repo-local command form, parity-root mismatch, and verify wiring drift. [VERIFIED: standards/core/testing.md; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.test.ts]

**Example:**

```typescript
test("fails_when_policy_omits_backup_then_rebuild_boundary", async () => {
  // Arrange
  const root = await createFixture({
    maybeOmission: {
      file: "docs/parity/upgrade-and-rollback-policy.md",
      needle: "backup_then_rebuild",
    },
  });

  // Act
  const failures = checkPhase84UpgradeRollbackPolicy(root);

  // Assert
  expect(failures.join("\n")).toContain("backup_then_rebuild");
});
```

Source pattern: Phase 83 fixture tests. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.test.ts]

### Anti-Patterns to Avoid

- **Creating alternate support labels:** Use exactly the Phase 82 terms where support classification appears; do not add labels such as beta, best-effort, or production-grade. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]
- **Duplicating the support matrix:** Link to `docs/parity/support-matrix.md`; do not reproduce the support matrix columns in Phase 84 entrypoints. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]
- **Treating rollback as package management:** Phase 84 is source-built and local-first; signed channels, automatic update channels, package-manager rollback, and installed aliases remain outside scope. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]
- **Turning `backup_then_rebuild` into destructive repair:** The action class means preserve a backup before an operator-decided rebuild outside this phase; it does not authorize automated repair. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/storage-decision.md]
- **Adding live checks to default verification:** `scripts/verify.sh` must stay public-network-free, real-service-manager-free, and multi-day-free for this phase. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/release-readiness.md; VERIFIED: scripts/verify.sh]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Support taxonomy | New upgrade support labels | Phase 82 terms: `supported`, `preview`, `opt-in UAT`, `unsupported`, `deferred` | The v1.8 boundary already defines the terms and Phase 83 checks for drift. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts] |
| Recovery compatibility vocabulary | New upgrade-state labels | Existing recovery categories, causes, and action classes | Runtime/status/storage docs and code already expose stable labels for clean/unclean shutdown, schema, corruption, lock contention, and action classes. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs] |
| Evidence sufficiency | "Started successfully", "has logs", "has report", or "peer reachable" | Field-level evidence plus `Unavailable: <reason>` | Existing v1.8 docs explicitly reject artifact existence and raw signals as proof by themselves. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md] |
| Rollback mechanism | Package-manager rollback, signed release channel, auto-update channel | Source revision or known local binary plus explicit datadir/config paths | Phase 84 locks source-built rollback and defers signed/package/automatic channels. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| Backup automation | Automatic backup or restore | Operator checklist and backup-location evidence | Automatic backup/restore is deferred and wallet/source data are high-value state. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/catalog/wallet.md] |
| Destructive repair | Lock deletion, recovery marker clearing, store repair, compaction, reindex, source datadir rewrite | Diagnosis-only policy with future gates | Existing runtime and storage docs explicitly exclude these automatic actions. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/storage-decision.md] |
| Drift checker | Broad all-doc production-claim scanner | Narrow Phase 84 checker scoped to policy sections, terms, boundaries, commands, links, and verifier wiring | Phase 88 owns broad deterministic claim guardrails. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/release-readiness.md] |

**Key insight:** The phase is about preserving operator decision boundaries, not implementing new state transitions; custom automation around backup, restore, service mutation, source datadir mutation, or destructive recovery would move the phase outside its locked scope. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/support-matrix.md]

## Runtime State Inventory

> Phase 84 is policy-only, but upgrade and rollback planning directly concerns runtime state; these categories identify what remains outside code/doc edits after the repository is updated. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | Open Bitcoin-owned durable store state lives under the selected datadir and exposes schema/recovery evidence through status/support fields; external Core/Knots datadirs and wallets are separate high-value inputs. [VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/storage-decision.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: packages/bitcoin-knots/doc/files.md] | Code edit: none expected. Policy task: explicitly separate Open Bitcoin-owned durable store state from external Core/Knots datadirs/wallets and document no hidden external rewrite. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| Live service config | Existing launchd/systemd service state may exist outside git and real service-manager lifecycle is `opt-in UAT`; service previews are documented but real manager mutation is outside default verification. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/operator/runtime-guide.md; VERIFIED: packages/bitcoin-knots/doc/init.md] | Code edit: none expected. Policy task: record service state as evidence and forbid hidden launchd/systemd mutation in upgrade/rollback guidance. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| OS-registered state | User-level service registrations can embed binary, config, datadir, and restart behavior, and upstream service examples show command-line service paths can override config-file values. [VERIFIED: packages/bitcoin-knots/contrib/init/bitcoind.service; VERIFIED: packages/bitcoin-knots/doc/init.md] | Code edit: none expected. Policy task: require service-state capture and make any service-file or supervisor mutation explicit/future-gated. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| Secrets/env vars | Support docs classify wallet private material, raw wallet files, RPC cookies, `rpcpassword`, `rpcauth`, raw datadirs, and unredacted logs as forbidden attachments. [VERIFIED: docs/parity/support-matrix.md] | Code edit: none expected. Policy task: preserve redaction and "Do Not Attach" boundaries when asking for rollback/escalation evidence. [VERIFIED: docs/parity/support-matrix.md] |
| Build artifacts | Source-built Cargo and Bazel command forms are canonical for operator evidence; installed aliases or old binaries can drift from the checked-out source revision. [VERIFIED: AGENTS.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: README.md] | Code edit: Phase 84 checker/verifier wiring only. Policy task: require current source revision/commit and Cargo/Bazel binary provenance before upgrade/rollback decisions. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |

**Nothing found in category:** No database migration, external service API patch, OS registration rewrite, secret rename, or installed package cleanup is required by Phase 84 itself because the locked scope is documentation plus deterministic drift checks. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

## Common Pitfalls

### Pitfall 1: Proving Compatibility From Startup Or Artifacts

**What goes wrong:** A policy says "if the upgraded daemon starts" or "if a support bundle exists" then schema/storage compatibility is proven. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md]

**Why it happens:** Startup, logs, reports, and support bundles are easy to observe, but existing docs require field-level evidence and unavailable-field reasons. [VERIFIED: docs/parity/support-matrix.md]

**How to avoid:** Make every compatibility decision row name the exact evidence fields or require `Unavailable: <reason>`. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/architecture/status-snapshot.md]

**Warning signs:** Prose relies on "started", "elapsed", "reachable", "logs show", "report exists", or "bundle generated" without naming fields. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md]

### Pitfall 2: Inventing Upgrade-Specific Recovery Labels

**What goes wrong:** The policy introduces labels such as upgrade-blocked, retryable-upgrade, rollback-needed, or rebuild-required instead of reusing existing recovery terms. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

**Why it happens:** Upgrade policy sounds distinct from recovery policy, but current status/storage surfaces already define stable categories, causes, and action classes. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/storage-decision.md]

**How to avoid:** Use a decision table that maps existing labels to allowed operator actions. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

**Warning signs:** The checker fixture does not fail when `safe_retry`, `read_only_inspection`, `backup_then_rebuild`, or `stop_and_escalate` disappears. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.test.ts]

### Pitfall 3: Hidden Mutation Through Helpful Rollback Text

**What goes wrong:** Rollback guidance tells operators to clear locks, reindex, rewrite config, restore wallets, disable services, or rebuild stores without an explicit future-gated plan. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

**Why it happens:** Upstream Core/Knots exposes destructive or mutating actions such as reindex and wallet migration/backup operations, and those can be mistaken for Open Bitcoin Phase 84 policy. [VERIFIED: packages/bitcoin-knots/src/init.cpp; VERIFIED: packages/bitcoin-knots/doc/managing-wallets.md]

**How to avoid:** Phrase Phase 84 rollback as "preserve evidence, stop, return to previous source revision or known binary, use same explicit paths, verify, record evidence" and keep mutation out of scope. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

**Warning signs:** The policy uses verbs like delete, clear, repair, compact, reindex, rewrite, restore, disable, uninstall, replace, upload, or import without a future-gate caveat. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/storage-decision.md]

### Pitfall 4: Turning The Checker Into Phase 88

**What goes wrong:** The Phase 84 checker scans all docs for broad production-readiness claims or tries to enforce the entire support matrix. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/release-readiness.md]

**Why it happens:** Phase 82/83/84 docs are adjacent to release-readiness and support-claim language, but broad claim guardrails are explicitly Phase 88 scope. [VERIFIED: docs/parity/release-readiness.md]

**How to avoid:** Limit the checker to Phase 84 policy sections, exact terms, required command forms, forbidden hidden-mutation language, canonical links, parity roots, and verifier wiring. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

**Warning signs:** The checker reads every Markdown file, performs generic forbidden-word scans, or duplicates Phase 83 matrix enforcement. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]

### Pitfall 5: Relying On Installed Aliases

**What goes wrong:** Operator steps mention only `open-bitcoin ...`, making it impossible to connect evidence to the checked-out source revision. [VERIFIED: AGENTS.md]

**Why it happens:** Installed aliases are shorter, but the repo policy requires explicit Cargo and Bazel command forms for UAT/operator workflows. [VERIFIED: AGENTS.md; VERIFIED: docs/parity/support-matrix.md]

**How to avoid:** Include both repo-local command forms where the policy asks the operator to inspect status or collect support evidence. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/parity/support-matrix.md]

**Warning signs:** The checker does not require `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --` and `bazel run //packages/open-bitcoin-cli:open_bitcoin --`. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]

## Code Examples

Verified patterns from official repo sources:

### Required Command-Form Check

```typescript
requireNormalizedContains(
  policy,
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --",
  "upgrade policy",
  failures,
);
requireNormalizedContains(
  policy,
  "bazel run //packages/open-bitcoin-cli:open_bitcoin --",
  "upgrade policy",
  failures,
);
```

Source: Phase 83 checker requires exact repo-local Cargo and Bazel support-bundle command forms. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]

### Parity Root Requirements Check

```typescript
const PHASE84_REQUIREMENTS = ["UPG-01", "UPG-02", "UPG-03", "UPG-04"] as const;

function requireExactRequirements(value: unknown, label: string, failures: string[]): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} parity root requirements must be an array`);
    return;
  }

  const actual = JSON.stringify(value);
  const expected = JSON.stringify(PHASE84_REQUIREMENTS);
  if (actual !== expected) {
    failures.push(`${label} parity root requirements mismatch: expected ${expected}, got ${actual}`);
  }
}
```

Source pattern: Phase 83 checker exact requirements validation. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]

### Verify Script Ordering Check

```typescript
const PHASE83_CHECKER_COMMAND =
  "bun run scripts/check-phase83-support-matrix-issue-evidence.ts";
const PHASE84_TEST_COMMAND =
  "bun test scripts/check-phase84-upgrade-rollback-policy.test.ts";
const PHASE84_CHECKER_COMMAND =
  "bun run scripts/check-phase84-upgrade-rollback-policy.ts";

const phase83Index = executableText.indexOf(PHASE83_CHECKER_COMMAND);
const phase84TestIndex = executableText.indexOf(PHASE84_TEST_COMMAND);
const phase84CheckerIndex = executableText.indexOf(PHASE84_CHECKER_COMMAND);
if (!(phase83Index !== -1 && phase84TestIndex > phase83Index && phase84CheckerIndex > phase84TestIndex)) {
  failures.push("verifier-order requires executed Phase 84 test and checker after Phase 83 checker");
}
```

Source pattern: Phase 83 checker validates executed checker order after Phase 82 and strips the legacy command-order heredoc before checking executable text. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts; VERIFIED: scripts/verify.sh]

### Forbidden Default Verification Text

```typescript
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-" + "smoke",
  "system" + "ctl",
  "launch" + "ctl",
  "sleep " + "259200",
] as const;
```

Source pattern: Phase 83 keeps default verification public-network-free, real-service-manager-free, and multi-day-free. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts; VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Treating historical sync/soak evidence as production-readiness momentum | v1.8 explicitly defines evidence gates before any production full-node readiness claim | Phase 82, 2026-06-21 [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: .planning/PROJECT.md] | Phase 84 must describe upgrade boundaries without broadening production-readiness language. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| Support docs scattered across runtime/catolog pages | One canonical support matrix with pointers from entrypoints and catalogs | Phase 83, 2026-06-21 [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/parity/README.md] | Phase 84 should mirror the single-source pattern and avoid a second support matrix. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| Recovery guidance as prose-only status text | Shared `recovery_evidence` field with action classes, causes, compatibility categories, evidence basis, unavailable reason, and next action | Phase 77 [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/operator-observability.md] | Phase 84 can map upgrade compatibility outcomes to existing labels without new runtime code. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| Support bundles or reports as sufficient proof | Field-level evidence and `Unavailable: <reason>` required | Phase 83 [VERIFIED: docs/parity/support-matrix.md] | The upgrade policy should reject daemon startup, elapsed time, peer reachability, raw logs, and report existence alone. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| Manual-only docs drift review | Narrow Bun checkers with fixture tests and `scripts/verify.sh` wiring | Phases 82 and 83 [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts; VERIFIED: scripts/verify.sh] | Phase 84 should add a similarly narrow deterministic checker for UPG-04. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |

**Deprecated/outdated:**

- Unversioned ASVS category references are risky because OWASP ASVS 5.0.0 states identifiers can change and recommends versioned references. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS]
- Package-manager rollback, signed releases, automatic update channels, migration apply mode, destructive repair, production-funds wallet support, and automatic backup/restore are deferred for this phase. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]
- Broad all-doc production-claim scanning is Phase 88 scope, not Phase 84 checker scope. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/release-readiness.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|

All claims in this research were verified from the phase context, project files, local command probes, or cited official OWASP ASVS sources; no `[ASSUMED]` claims are intentionally present. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; CITED: https://owasp.org/www-project-application-security-verification-standard/]

## Open Questions (RESOLVED)

1. **Exact policy filename**
   - What we know: The executor may choose `docs/parity/upgrade-and-rollback-policy.md` or a narrower filename with the same canonical role. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]
   - RESOLVED: Use `docs/parity/upgrade-and-rollback-policy.md` because it matches the phase name, is explicit for link targets, and is the path selected by the Phase 84 plans. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-01-PLAN.md]

2. **Whether to add Rust checks**
   - What we know: No Rust source changes are expected. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]
   - RESOLVED: Do not plan Rust work. Keep Phase 84 to docs, parity metadata, Bun checker/test coverage, verifier wiring, and the existing contingency that any newly discovered first-party Rust source/test file must add parity breadcrumbs. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-04-PLAN.md; VERIFIED: AGENTS.md]

3. **How much runtime-guide detail to duplicate**
   - What we know: The phase requires a canonical policy and links from the runtime guide; existing runtime guide already contains command forms, recovery vocabulary, status evidence, support-bundle collection, service state, and opt-in UAT material. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]
   - RESOLVED: Keep runtime-guide updates pointer-first. The canonical policy should include the minimum command snippets needed for pre-upgrade/status/support evidence and link to the runtime guide for broader workflows. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-03-PLAN.md; VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/parity/support-matrix.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Git | Source revision, rollback source checkout, sync check, commit workflow | yes | 2.53.0 | None needed. [VERIFIED: git --version] |
| Bun | Phase 84 checker and fixture tests | yes | 1.3.9 | None needed; repo pins Bun and has no npm package surface. [VERIFIED: .bun-version; VERIFIED: bun --version; VERIFIED: find . -maxdepth 2 -name package.json -print] |
| Cargo | Repo-local operator command forms and `scripts/verify.sh` | yes | 1.94.1 | None needed. [VERIFIED: cargo --version; VERIFIED: rust-toolchain.toml] |
| Rust compiler | `scripts/verify.sh` Rust format/lint/build/test path | yes | 1.94.1 | None needed. [VERIFIED: rustc --version; VERIFIED: rust-toolchain.toml] |
| Bazel | Repo-local Bazel operator command forms and full verifier smoke path | yes | 8.6.0 | None needed. [VERIFIED: bazel --version; VERIFIED: scripts/verify.sh] |
| `package.json` / npm dependency surface | Not required by Phase 84 | no | none | Use Bun direct script execution; this matches existing repo pattern. [VERIFIED: find . -maxdepth 2 -name package.json -print; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts] |
| `tsconfig*.json` | Not required by existing checker pattern | no | none | Existing Bun checker scripts run without a repo TypeScript config. [VERIFIED: find . -maxdepth 3 -name 'tsconfig*.json' -print; VERIFIED: scripts/verify.sh] |

**Missing dependencies with no fallback:**

- None. [VERIFIED: command probes listed above]

**Missing dependencies with fallback:**

- No `package.json`, Bun lockfile, or `tsconfig*.json` exists; fallback is the established direct `bun run scripts/*.ts` and `bun test scripts/*.test.ts` pattern. [VERIFIED: find . -maxdepth 2 -name package.json -print; VERIFIED: find . -maxdepth 3 -name 'bun.lock*' -print; VERIFIED: find . -maxdepth 3 -name 'tsconfig*.json' -print; VERIFIED: scripts/verify.sh]

## Security Domain

OWASP ASVS latest stable version is 5.0.0, and OWASP recommends versioned requirement references because identifiers can change. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS]

### Applicable ASVS Categories

| ASVS Category / Topic | Applies | Standard Control |
|---------------|---------|-----------------|
| Authentication/session management | no | Phase 84 does not add authentication or session features; preserve existing RPC credential redaction boundaries. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/support-matrix.md] |
| Access control / privileged operations | yes | Do not imply hidden service-manager, datadir, wallet, or config mutation; all mutation remains explicit and future-gated. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/support-matrix.md] |
| Input validation / command injection | yes | The checker should parse fixed repo files and compare literal command forms; it should not execute operator commands extracted from docs. [VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts; VERIFIED: scripts/verify.sh] |
| Sensitive data protection | yes | Keep `Do Not Attach` boundaries for wallet private material, raw wallet files, RPC cookies, `rpcpassword`, `rpcauth`, raw datadirs, unredacted logs, raw unbounded logs, and full sensitive peer tables. [VERIFIED: docs/parity/support-matrix.md] |
| Error handling and logging | yes | Require compatibility decisions to preserve field-level evidence and unavailable-field reasons rather than raw logs or artifact existence. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/architecture/status-snapshot.md] |
| Cryptography | no new crypto | Phase 84 should not add cryptographic protocols, signing, package authenticity, wallet-production safety, or signed release channels. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: docs/parity/production-claim-boundary.md] |

### Known Threat Patterns for Phase 84

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Support evidence leaks secrets or high-value local data | Information Disclosure | Keep redacted local evidence only and preserve `Do Not Attach` boundaries. [VERIFIED: docs/parity/support-matrix.md] |
| Rollback prose causes hidden datadir, wallet, service, or config mutation | Tampering / Elevation of Privilege | Require explicit future-scoped plans for mutation and state that Phase 84 is evidence/policy only. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| Operator cannot prove which source revision or binary was used | Repudiation | Pre-upgrade and rollback checklists must capture source revision/commit, repo-local verification status, and Cargo/Bazel provenance. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md] |
| Default verification gains public-network, real-service-manager, or multi-day work | Denial of Service / Reliability | Phase 84 checker and `scripts/verify.sh` wiring must remain deterministic and local. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: scripts/verify.sh] |
| Compatibility outcome inferred from logs/startup/report existence | Spoofing / Repudiation | Require field-level evidence, unavailable-field reasons, and typed recovery labels. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/architecture/status-snapshot.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md` - locked Phase 84 decisions, scope, integration points, and deferred work. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md]
- `.planning/REQUIREMENTS.md` - UPG-01 through UPG-04 requirement definitions. [VERIFIED: .planning/REQUIREMENTS.md]
- `.planning/STATE.md`, `.planning/PROJECT.md`, `.planning/ROADMAP.md` - v1.8 boundary-setting posture, phase dependencies, and current milestone state. [VERIFIED: .planning/STATE.md; VERIFIED: .planning/PROJECT.md; VERIFIED: .planning/ROADMAP.md]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and standards pages under `standards/` - repo-local verification, command-form, Bun/TS, testing, and workflow constraints. [VERIFIED: AGENTS.md; VERIFIED: AGENTS.bright-builds.md; VERIFIED: standards/core/verification.md; VERIFIED: standards/languages/typescript-javascript.md]
- `docs/parity/production-claim-boundary.md`, `docs/parity/support-matrix.md`, `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md`, `docs/parity/README.md`, `docs/parity/checklist.md`, `docs/parity/index.json` - current v1.8 boundary/support roots and link targets. [VERIFIED: docs/parity/production-claim-boundary.md; VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/parity/release-readiness.md; VERIFIED: docs/parity/deviations-and-unknowns.md; VERIFIED: docs/parity/README.md; VERIFIED: docs/parity/checklist.md; VERIFIED: docs/parity/index.json]
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/architecture/storage-decision.md`, `docs/architecture/cli-command-architecture.md` - operator command forms, recovery vocabulary, evidence semantics, storage recovery boundaries, and migration/config mutation boundaries. [VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/status-snapshot.md; VERIFIED: docs/architecture/operator-observability.md; VERIFIED: docs/architecture/storage-decision.md; VERIFIED: docs/architecture/cli-command-architecture.md]
- `scripts/check-phase82-production-claim-boundary.ts`, `scripts/check-phase83-support-matrix-issue-evidence.ts`, `scripts/check-phase83-support-matrix-issue-evidence.test.ts`, `scripts/verify.sh` - deterministic checker, fixture-test, and verifier wiring patterns. [VERIFIED: scripts/check-phase82-production-claim-boundary.ts; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.test.ts; VERIFIED: scripts/verify.sh]
- `packages/bitcoin-knots/doc/files.md`, `packages/bitcoin-knots/doc/init.md`, `packages/bitcoin-knots/doc/managing-wallets.md`, `packages/bitcoin-knots/src/init.cpp`, `packages/bitcoin-knots/contrib/init/bitcoind.service` - pinned baseline context for datadir, wallet, service, config, and reindex mutation semantics. [VERIFIED: packages/bitcoin-knots/doc/files.md; VERIFIED: packages/bitcoin-knots/doc/init.md; VERIFIED: packages/bitcoin-knots/doc/managing-wallets.md; VERIFIED: packages/bitcoin-knots/src/init.cpp; VERIFIED: packages/bitcoin-knots/contrib/init/bitcoind.service]

### Secondary (MEDIUM confidence)

- OWASP ASVS official project page and GitHub README - ASVS 5.0.0 stable version and versioned-reference guidance. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS]

### Tertiary (LOW confidence)

- None. [VERIFIED: research source log]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - local versions and command availability were verified with repo files and command probes; no new external package is recommended. [VERIFIED: .bun-version; VERIFIED: rust-toolchain.toml; VERIFIED: bun --version; VERIFIED: cargo --version; VERIFIED: rustc --version; VERIFIED: bazel --version]
- Architecture: HIGH - Phase 84 integration points are locked by context and match Phase 82/83 existing docs/checker patterns. [VERIFIED: .planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md; VERIFIED: scripts/check-phase83-support-matrix-issue-evidence.ts]
- Pitfalls: HIGH - each pitfall is directly supported by existing v1.8 docs, runtime/storage docs, or pinned Knots baseline behavior. [VERIFIED: docs/parity/support-matrix.md; VERIFIED: docs/operator/runtime-guide.md; VERIFIED: docs/architecture/storage-decision.md; VERIFIED: packages/bitcoin-knots/src/init.cpp]

**Research date:** 2026-06-21 [VERIFIED: current_date]
**Valid until:** 2026-07-21 for repo-local planning guidance; re-check before execution if Phase 82/83 docs, `scripts/verify.sh`, `.bun-version`, `rust-toolchain.toml`, or support-matrix roots change. [VERIFIED: .planning/config.json; VERIFIED: scripts/verify.sh; VERIFIED: .bun-version; VERIFIED: rust-toolchain.toml]
