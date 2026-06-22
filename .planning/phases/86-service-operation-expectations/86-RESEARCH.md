# Phase 86: Service Operation Expectations - Research

**Researched:** 2026-06-22 [VERIFIED: environment_context current_date]
**Domain:** Service-operation documentation, parity traceability, and deterministic Bun verification [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/check-phase85-operator-runbooks.ts; scripts/verify.sh]

<user_constraints>
## User Constraints (from CONTEXT.md) [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

All content in this section is copied from the Phase 86 context and is binding for planning. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

### Locked Decisions [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

## Implementation Decisions

### Service Support Boundary

- **D-01:** Create one canonical Phase 86 service expectation document under
  `docs/parity/`, preferably `service-operation-expectations.md`, instead of
  spreading service policy across README, runtime guide, support matrix, and
  runbooks.
- **D-02:** Preserve the exact Phase 82 support terms:
  `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred`.
  Service expectation docs must not introduce alternate terms such as
  production-ready service, production-grade service, managed service,
  package-supported, or community-supported.
- **D-03:** Classify the existing service surfaces narrowly:
  source-built daemon command forms and local status/support evidence are
  `supported`; launchd/systemd generated definition preview is `preview`; real
  user-level launchd/systemd install/start/stop/restart/status evidence is
  `opt-in UAT`; packaged service distribution, Windows service integration,
  automatic updates, production service ownership, uptime guarantees, and broad
  production full-node readiness remain `deferred`.
- **D-04:** Explicitly state that generated launchd/systemd definitions
  supervise `open-bitcoind`, not the `open-bitcoin` operator wrapper, and that
  user-level paths stay local to the operator machine. Do not imply system-wide
  service ownership or packaged distribution support.
- **D-05:** Keep service-manager mutation boundaries visible. `service preview`
  is always side-effect-free. `service install` and `service uninstall` are
  previews unless `--apply` is supplied. Starting, stopping, restarting,
  enabling, disabling, or uninstalling a real service is opt-in local UAT and
  must not be part of default verification.

### Operator Evidence And Command Forms

- **D-06:** Every operator-facing command in this phase must show repo-local
  Cargo and Bazel forms. Do not rely only on an installed `open-bitcoin` alias.
- **D-07:** Service expectation evidence should be field-based, not artifact
  existence based. Service file existence, daemon startup, elapsed time, raw log
  tail, public peer reachability, or a support bundle path is context only
  unless the expected fields and unavailable reasons are present.
- **D-08:** Required evidence areas are service lifecycle, service
  restart/resume, status JSON, sync status JSON, structured logs, bounded
  metrics, resource bounds or resource pressure, recovery category/action,
  support-bundle evidence, service log path, service manager command strings,
  generated service file path, and unavailable reasons.
- **D-09:** Carry forward the existing lifecycle labels exactly:
  `unmanaged`, `installed-stopped`, `running`, `failed`, `disabled`, and
  `unavailable-manager`. Docs and checkers should require these labels where
  service status is explained.
- **D-10:** Restart/resume evidence should be interpreted from
  `service.restart_resume` and related status fields: `same_datadir`,
  `prior_shutdown`, `durable_progress`, `stale_inflight`, `recovery_category`,
  and `next_action`. Do not treat restart command success or elapsed time as
  resume proof.
- **D-11:** Include both direct daemon and service-supervised review flows:
  direct `open-bitcoind` startup/status evidence remains source-built daemon
  operation; service commands review the local user supervisor around that
  daemon path.

### Documentation And Traceability Shape

- **D-12:** Link the canonical service expectations from the production claim
  boundary, support matrix, operator runbooks, upgrade/rollback policy,
  release-readiness page, deviations register, parity README, parity checklist,
  parity index, README, runtime guide, and operator-runtime catalog without
  duplicating the full service expectation table in each file.
- **D-13:** Update `docs/parity/production-claim-boundary.md` so its production
  service operation row points to the canonical Phase 86 expectation document
  while preserving the `deferred` production-service claim boundary.
- **D-14:** Update `docs/parity/support-matrix.md` so launchd/systemd preview
  and real lifecycle rows point to the canonical service expectation document
  without changing their existing support terms.
- **D-15:** Update `docs/parity/operator-runbooks.md` and
  `docs/operator/runtime-guide.md` only with compact pointers or clarifying
  handoffs. Keep procedural command duplication controlled by making the new
  Phase 86 document the service expectation source of truth.
- **D-16:** Register a new parity surface such as
  `v1-8-service-operation-expectations` in `docs/parity/index.json`,
  `docs/parity/checklist.md`, and `docs/parity/README.md`, with requirements
  `SVC-01` and `SVC-02` and evidence that includes the canonical document,
  runtime guide, support matrix, operator runbooks, operator-runtime catalog,
  checker, and `scripts/verify.sh`.

### Verification And Guardrails

- **D-17:** Add a narrow Phase 86 Bun checker and fixture tests only for the
  Phase 86 document set: service expectation sections, support terms, command
  forms, service lifecycle labels, restart/resume fields, required links,
  parity roots, and verifier wiring.
- **D-18:** Wire the Phase 86 checker and fixture tests into
  `bash scripts/verify.sh` immediately after Phase 85, both in the visible
  command-order block and in the executed `run_step` sequence.
- **D-19:** The Phase 86 checker must reject default verifier drift that adds
  public-network live smoke, real `systemctl`, real `launchctl`, long
  wall-clock sleeps, `--restart-after-progress`, package-manager service
  commands, Windows service claims, production service ownership, automatic
  support upload, or broad production-node readiness.
- **D-20:** Final closeout should run focused checker commands, refresh
  `docs/metrics/lines-of-code.md` if checker files are added, and then run the
  full repo-native `bash scripts/verify.sh` gate.

### Folded Todos

No pending todos matched Phase 86.

### the agent's Discretion [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

- The planner may split Phase 86 into canonical service expectation docs,
  parity/root and entrypoint links, deterministic checker plus fixture tests,
  and closeout/LOC/full-verification work.
- The executor may keep Phase 86 primarily documentation and Bun automation if
  no source behavior gap is found.
- No Rust source changes are expected. If planning discovers a narrow Rust gap,
  update `docs/parity/source-breadcrumbs.json` for any new first-party Rust
  source or test files under `packages/open-bitcoin-*/src` or
  `packages/open-bitcoin-*/tests`.

### Deferred Ideas (OUT OF SCOPE) [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

None - discussion stayed within Phase 86 scope.
</user_constraints>

<phase_requirements>
## Phase Requirements [VERIFIED: .planning/REQUIREMENTS.md]

| ID | Description | Research Support |
| --- | --- | --- |
| SVC-01 | Operator can distinguish source-built daemon operation from launchd/systemd supervision, packaged-service distribution, service-manager availability, and unsupported production-service claims. [VERIFIED: .planning/REQUIREMENTS.md] | Plan one canonical `docs/parity/service-operation-expectations.md` classification table using exact Phase 82 support terms and links from existing v1.8 docs. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |
| SVC-02 | Operator can verify service lifecycle, restart/resume, log, metric, resource-bound, and recovery expectations through repo-local Cargo and Bazel command forms. [VERIFIED: .planning/REQUIREMENTS.md] | Plan command groups for direct daemon review, service preview/install/lifecycle, status/sync status, restart/resume, support bundle, logs, metrics, resource pressure, recovery, and safe shutdown in both Cargo and Bazel forms. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; docs/operator/runtime-guide.md] |
</phase_requirements>

## Summary

Phase 86 should be planned as a documentation and Bun-automation phase centered on one canonical parity document, `docs/parity/service-operation-expectations.md`, plus compact links from the v1.8 boundary, support, runbook, upgrade, release-readiness, deviations, parity, README, runtime-guide, and catalog entrypoints. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] No Rust source changes are expected unless planning uncovers a narrow behavior gap; any new first-party Rust source or test file would require parity breadcrumb updates. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; AGENTS.md]

The implementation should reuse the Phase 85 checker structure: a fixed-target Bun checker with an exported `check...` function, fixture tests using a repo-root override, JSON parity-root validation, human-pointer deduplication checks, and `scripts/verify.sh` wiring checks that strip the legacy command-order heredoc before validating executed `run_step` commands. [VERIFIED: scripts/check-phase85-operator-runbooks.ts; scripts/check-phase85-operator-runbooks.test.ts; scripts/verify.sh]

**Primary recommendation:** Split planning into four work packages: canonical service expectations doc, parity/root plus entrypoint pointers, deterministic Phase 86 checker/tests plus verifier wiring, and closeout with focused Bun checks, LOC freshness, and full `bash scripts/verify.sh`. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/check-phase85-operator-runbooks.ts; scripts/verify.sh]

## Project Constraints (from AGENTS.md)

- Use `bash scripts/verify.sh` as the repo-native verification contract; `--fast` is only for iteration and the default command remains the pre-commit and release contract. [VERIFIED: AGENTS.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts and prefer TypeScript for substantial script logic. [VERIFIED: AGENTS.md]
- Operator-facing UAT guidance must include repo-local Cargo and Bazel command forms instead of relying only on an installed `open-bitcoin` alias. [VERIFIED: AGENTS.md]
- `docs/metrics/lines-of-code.md` is an intentionally tracked generated artifact and should be refreshed or checked when hooks or verification regenerate it. [VERIFIED: AGENTS.md]
- Intentional behavior differences from Bitcoin Knots must be recorded in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md]
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumb registration; `none` is only for files with no defensible Knots source anchor. [VERIFIED: AGENTS.md]
- Bright Builds routing requires AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, and relevant standards pages to be loaded before plan, review, implementation, or audit work. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md]
- Existing local standards require functional-core/imperative-shell separation, early returns, typed illegal-state prevention, unit tests for pure/business logic, and repo-native verification before commit. [VERIFIED: standards/core/architecture.md; standards/core/code-shape.md; standards/core/testing.md; standards/core/verification.md]
- No project-local skills were found in `.claude/skills/` or `.agents/skills/`. [VERIFIED: AGENTS.md; command: `find .claude/skills .agents/skills -maxdepth 2 -name SKILL.md`]

## Standard Stack

### Core

| Tool / Library | Version | Purpose | Why Standard |
| --- | --- | --- | --- |
| Bun | 1.3.9 [VERIFIED: command `bun --version`] | Run Phase 86 TypeScript checker and fixture tests. [VERIFIED: AGENTS.md; scripts/check-phase85-operator-runbooks.ts] | Repo-local guidance makes Bun the canonical runtime for higher-level automation scripts. [VERIFIED: AGENTS.md] |
| TypeScript checker under `scripts/` | Repo-owned source, no package install needed [VERIFIED: scripts/check-phase85-operator-runbooks.ts; AGENTS.md] | Validate service expectation docs, parity roots, required links, forbidden drift, and verifier wiring. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | Phase 82 through Phase 85 already use narrow deterministic Bun checkers wired into `scripts/verify.sh`. [VERIFIED: scripts/verify.sh; scripts/check-phase85-operator-runbooks.ts] |
| Bash `scripts/verify.sh` | Repo-owned script [VERIFIED: scripts/verify.sh] | Default verification gate and checker ordering surface. [VERIFIED: AGENTS.md; scripts/verify.sh] | Repo-local guidance defines it as the source-of-truth verification contract. [VERIFIED: AGENTS.md] |
| Cargo / Rust | Cargo 1.94.1 and rustc 1.94.1 [VERIFIED: command `cargo --version`; command `rustc --version`] | Provide repo-local operator command examples and normal repo verification. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md] | Rust `1.94.1` is pinned by repo guidance and the operator command forms are required for UAT docs. [VERIFIED: AGENTS.md] |
| Bazel | 8.6.0 [VERIFIED: command `bazel --version`] | Provide repo-local Bazel command examples and full verifier smoke builds. [VERIFIED: AGENTS.md; scripts/verify.sh] | Repo-local UAT guidance requires Bazel command forms where applicable. [VERIFIED: AGENTS.md] |

### Supporting

| Tool | Version | Purpose | When to Use |
| --- | --- | --- | --- |
| Git | 2.53.0 [VERIFIED: command `git --version`] | Capture source revision in operator evidence and support issue context. [VERIFIED: docs/parity/operator-runbooks.md; docs/parity/upgrade-and-rollback-policy.md] | Include in docs as evidence to record, not as a new implementation dependency. [VERIFIED: docs/parity/operator-runbooks.md] |
| cargo-llvm-cov | 0.8.5 [VERIFIED: command `cargo llvm-cov --version`] | Required by full `bash scripts/verify.sh`. [VERIFIED: scripts/verify.sh] | Planner should expect full closeout to need this tool available. [VERIFIED: scripts/verify.sh; command `cargo llvm-cov --version`] |
| ripgrep | 15.1.0 [VERIFIED: command `rg --version`] | Fast local source/doc inspection during implementation. [VERIFIED: command `rg --version`] | Use for targeted implementation checks, not as checked-in dependency. [VERIFIED: command `rg --version`] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
| --- | --- | --- |
| Bun TypeScript checker [VERIFIED: AGENTS.md] | Python script [ASSUMED] | Do not use Python for new repo-owned automation because repo guidance requires Bun/TypeScript for substantial scripts. [VERIFIED: AGENTS.md; standards/languages/typescript-javascript.md] |
| One canonical parity document [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | Duplicate service tables across README/runtime/support/runbooks [ASSUMED] | Duplicating the table conflicts with the locked Phase 86 decision to keep the new service document as source of truth. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |
| Narrow Phase 86 checker [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | Broad all-doc production-claim scanner [ASSUMED] | Broad production-claim guardrails belong to Phase 88, not Phase 86. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; .planning/ROADMAP.md] |
| Repo-local Cargo and Bazel command pairs [VERIFIED: AGENTS.md] | Installed `open-bitcoin` alias only [ASSUMED] | Alias-only examples violate repo-local UAT guidance and Phase 86 D-06. [VERIFIED: AGENTS.md; .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |

**Installation:** No new npm, Cargo, or system packages should be planned for Phase 86; use existing repo-local Bun scripts and docs. [VERIFIED: AGENTS.md; .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

```bash
# No new package installation expected for Phase 86.
bun test scripts/check-phase86-service-operation-expectations.test.ts
bun run scripts/check-phase86-service-operation-expectations.ts
```

**Version verification:** `npm view` is not applicable because no npm package is recommended; local tool versions were verified through `bun --version`, `cargo --version`, `rustc --version`, `bazel --version`, and `cargo llvm-cov --version`. [VERIFIED: command outputs in research session]

## Architecture Patterns

### Recommended Project Structure

```text
docs/
  parity/
    service-operation-expectations.md        # canonical Phase 86 service expectation root [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
    production-claim-boundary.md             # compact Phase 86 pointer, no duplicated table [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
    support-matrix.md                        # support rows point to canonical service doc [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
    operator-runbooks.md                     # compact service-expectation handoff [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
    upgrade-and-rollback-policy.md           # compact service mutation-boundary handoff [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
    release-readiness.md                     # register SVC-01/SVC-02 handoff [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
    deviations-and-unknowns.md               # deferred service-production boundaries [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
    README.md                                # parity entrypoint pointer [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
    checklist.md                             # human parity checklist row [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
    index.json                               # machine parity surface and audit key [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
    catalog/
      operator-runtime-release-hardening.md  # catalog row pointer [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
docs/operator/runtime-guide.md               # compact pointer, keep practical commands [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
README.md                                    # compact operator preview pointer [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
scripts/
  check-phase86-service-operation-expectations.ts       # narrow fixed-target checker [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
  check-phase86-service-operation-expectations.test.ts  # fixture tests [VERIFIED: scripts/check-phase85-operator-runbooks.test.ts]
  verify.sh                                            # checker/test wired after Phase 85 [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/verify.sh]
docs/metrics/lines-of-code.md                          # tracked generated freshness surface if regenerated [VERIFIED: AGENTS.md]
```

### Pattern 1: Canonical Service Expectation Table

**What:** Create one table-driven parity document with service surface, support term, what it proves, Cargo command evidence, Bazel command evidence, default verification status, opt-in UAT status, residual risk, and next gate. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

**When to use:** Use for SVC-01 and SVC-02 because the phase must distinguish source-built daemon operation, service supervision, distribution limits, and evidence command forms. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

**Example skeleton:**

```markdown
| Service surface | Support term | What evidence proves | Cargo command evidence | Bazel command evidence | Default verification | Opt-in UAT | Residual risk | Next gate |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Direct source-built `open-bitcoind` operation | `supported` | Selected datadir status, sync status, logs/metrics/resource fields, and unavailable reasons | `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -datadir=<path>` plus `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=<path> status --format json` | `bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -datadir=<path>` plus `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=<path> status --format json` | deterministic docs/checker only | public-network and long-run review remain opt-in | startup alone is not proof | future production-readiness gate |
```

Source: Phase 86 D-03, D-06, D-07, D-08, and runtime-guide service/status command forms. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; docs/operator/runtime-guide.md]

### Pattern 2: Pointer-Only Entrypoint Updates

**What:** Existing docs should link to `service-operation-expectations.md` without copying the full service expectation table. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

**When to use:** Use for README, runtime guide, production boundary, support matrix, runbooks, upgrade policy, release readiness, deviations, parity README, checklist, index JSON, and operator-runtime catalog. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

**Example rule:** Require `service-operation-expectations.md` in human roots and fail if the canonical table header appears in pointer files. [VERIFIED: scripts/check-phase85-operator-runbooks.ts]

### Pattern 3: Deterministic Checker With Fixture Tests

**What:** Follow Phase 85's fixed-target checker structure: read a known file set, accumulate failures, validate canonical doc content, parse `docs/parity/index.json`, validate human roots, and verify `scripts/verify.sh` executable wiring. [VERIFIED: scripts/check-phase85-operator-runbooks.ts]

**When to use:** Use because Phase 86 D-17 and D-18 require a narrow Bun checker plus fixture tests wired into the verifier after Phase 85. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

**Code pattern:**

```typescript
const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE86_REPO_ROOT";
const SURFACE_ID = "v1-8-service-operation-expectations";
const AUDIT_KEY = "v1_8_service_operation_expectations";
const PHASE86_REQUIREMENTS = ["SVC-01", "SVC-02"] as const;
const SERVICE_DOC_PATH = "docs/parity/service-operation-expectations.md";

// Pattern source: Phase 85 checker uses repo-root override, surface id,
// audit key, exact requirements, target files, parity JSON parsing, and
// verifier wiring checks. [VERIFIED: scripts/check-phase85-operator-runbooks.ts]
```

### Pattern 4: Executable Verifier Wiring Check

**What:** The checker must strip the legacy `VERIFY_COMMAND_ORDER` heredoc before validating executed `run_step` command order. [VERIFIED: scripts/check-phase85-operator-runbooks.ts; scripts/verify.sh]

**When to use:** Use so Phase 86 commands cannot exist only in the visible heredoc while missing from the executed verification path. [VERIFIED: scripts/check-phase85-operator-runbooks.test.ts]

```typescript
function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}
```

Source: Existing Phase 85 checker. [VERIFIED: scripts/check-phase85-operator-runbooks.ts]

### Anti-Patterns to Avoid

- **Inventing new support terms:** Service docs must use only `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred`. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; docs/parity/production-claim-boundary.md]
- **Treating service command success as resume proof:** Restart/resume evidence must come from `service.restart_resume` fields, not elapsed time or command exit status alone. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; docs/operator/runtime-guide.md]
- **Adding live service-manager or public-network work to default verification:** `scripts/verify.sh` must stay deterministic, public-network-free, real-service-manager-free, and multi-day-free. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/verify.sh; docs/parity/operator-runbooks.md]
- **Duplicating the canonical table across entrypoints:** Phase 86 D-12 and D-15 require one service expectation source of truth and compact handoffs elsewhere. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
- **Using alias-only operator commands:** Every operator-facing command in this phase needs repo-local Cargo and Bazel forms. [VERIFIED: AGENTS.md; .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
| --- | --- | --- | --- |
| Support vocabulary | New service maturity labels | Phase 82 terms exactly: `supported`, `preview`, `opt-in UAT`, `unsupported`, `deferred` [VERIFIED: docs/parity/production-claim-boundary.md] | Alternate terms would contradict Phase 86 D-02. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |
| Service manager implementation | New launchd/systemd adapters or service owner | Existing service docs and existing operator service command surface [VERIFIED: docs/operator/runtime-guide.md; scripts/check-phase63-service-lifecycle.ts] | Phase 86 documents expectations and must not add a new service manager. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |
| Production claim scanner | Broad all-doc production-readiness scanner | Narrow Phase 86 checker only [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | Broad deterministic claim guardrails belong to Phase 88. [VERIFIED: .planning/ROADMAP.md; .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |
| Evidence interpretation | Raw artifact/path/log-tail checks | Field-based status, sync, service, log, metric, resource, recovery, support, and unavailable-reason checks [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; docs/operator/runtime-guide.md] | Artifact existence alone is explicitly insufficient. [VERIFIED: docs/parity/support-matrix.md; docs/parity/operator-runbooks.md] |
| Package/service distribution | Package-manager service commands or signed distribution policy | Deferred surface rows and source-built command evidence [VERIFIED: docs/parity/production-claim-boundary.md; docs/parity/support-matrix.md] | Signed packaging, package-manager distribution, Windows service, automatic updates, and production service ownership are deferred. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |

**Key insight:** The hard part of Phase 86 is preventing service expectation wording from becoming an implicit production-service claim; field-based evidence, exact support terms, and deterministic checker guardrails are the control points. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; docs/parity/production-claim-boundary.md; docs/parity/support-matrix.md]

## Common Pitfalls

### Pitfall 1: Default Verifier Drift

**What goes wrong:** The Phase 86 checker or `scripts/verify.sh` accidentally includes public-network live smoke, real `systemctl`, real `launchctl`, long sleeps, `--restart-after-progress`, package-manager service commands, Windows service claims, or automatic support upload. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
**Why it happens:** Service expectations can look operational, but real service-manager and public-network evidence is opt-in UAT rather than default verification. [VERIFIED: docs/parity/support-matrix.md; docs/operator/runtime-guide.md]
**How to avoid:** Have the Phase 86 checker reject those strings in executable verifier text, not just in the legacy heredoc. [VERIFIED: scripts/check-phase85-operator-runbooks.ts; scripts/check-phase85-operator-runbooks.test.ts]
**Warning signs:** The planned verifier diff adds `systemctl`, `launchctl`, `run-live-mainnet-smoke`, `sleep 259200`, or `--restart-after-progress`. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/check-phase85-operator-runbooks.ts]

### Pitfall 2: Service Expectation Table Sprawl

**What goes wrong:** README, runtime guide, support matrix, and runbooks each get partial service tables with slightly different terms or evidence rules. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
**Why it happens:** Many existing entrypoints need a Phase 86 pointer. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
**How to avoid:** Keep only `docs/parity/service-operation-expectations.md` table-driven and make other files compact handoffs. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
**Warning signs:** Pointer files include the canonical table header or long command matrices instead of a link. [VERIFIED: scripts/check-phase85-operator-runbooks.ts]

### Pitfall 3: Command Forms Missing One Build Path

**What goes wrong:** Operator evidence examples show Cargo but omit Bazel, or use only an installed alias. [VERIFIED: AGENTS.md; .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
**Why it happens:** Existing docs contain many practical command examples and are easy to copy incompletely. [VERIFIED: docs/operator/runtime-guide.md]
**How to avoid:** Define command groups with paired Cargo and Bazel forms for direct daemon, service lifecycle, status, sync status, support bundle, log/metric/resource review, recovery, and safe shutdown. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; docs/operator/runtime-guide.md]
**Warning signs:** The checker fixture can remove one command form without failing. [VERIFIED: scripts/check-phase85-operator-runbooks.test.ts]

### Pitfall 4: Restart/Resume Proof Collapses Into Start/Stop Proof

**What goes wrong:** Docs imply `service restart` success or elapsed time proves durable resume. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
**Why it happens:** The service lifecycle command flow and restart/resume evidence live near each other in the runtime guide. [VERIFIED: docs/operator/runtime-guide.md]
**How to avoid:** Require `service.restart_resume.same_datadir`, `prior_shutdown`, `durable_progress`, `stale_inflight`, `recovery_category`, and `next_action` fields in docs and checker. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/check-phase64-service-restart-resume.ts]
**Warning signs:** The service doc mentions restart evidence but omits `same_datadir` or `stale_inflight`. [VERIFIED: scripts/check-phase64-service-restart-resume.ts]

### Pitfall 5: Parity Root Registration Incomplete

**What goes wrong:** The canonical doc exists but parity index, checklist, README, release-readiness, or audit key do not discover it. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
**Why it happens:** Previous phases require both human and machine roots. [VERIFIED: docs/parity/index.json; docs/parity/checklist.md; docs/parity/README.md]
**How to avoid:** Add `v1-8-service-operation-expectations` and `v1_8_service_operation_expectations` with SVC-01/SVC-02 and evidence paths, then checker-validate both. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/check-phase85-operator-runbooks.ts]
**Warning signs:** `docs/parity/index.json` has a checklist surface but no audit key, or checklist has the row but no README pointer. [VERIFIED: scripts/check-phase85-operator-runbooks.ts]

## Code Examples

### Phase 86 Checker Constants

```typescript
const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE86_REPO_ROOT";
const SURFACE_ID = "v1-8-service-operation-expectations";
const AUDIT_KEY = "v1_8_service_operation_expectations";
const PHASE86_REQUIREMENTS = ["SVC-01", "SVC-02"] as const;
const SERVICE_DOC_PATH = "docs/parity/service-operation-expectations.md";
```

Use this pattern because Phase 85 uses the same constant shape for surface id, audit key, exact requirements, and canonical doc path. [VERIFIED: scripts/check-phase85-operator-runbooks.ts]

### Verifier Order Pattern

```bash
bun test scripts/check-phase86-service-operation-expectations.test.ts
bun run scripts/check-phase86-service-operation-expectations.ts
```

Add these both to the visible `VERIFY_COMMAND_ORDER` block and to executed `run_step` calls immediately after Phase 85. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/verify.sh]

### Required Forbidden Drift Set

```typescript
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "systemctl",
  "launchctl",
  "sleep 259200",
  "--restart-after-progress",
  "brew services",
  "system service",
  "Windows service",
  "automatic support-bundle upload",
  "production service ownership",
] as const;
```

The exact list should be tightened during implementation, but it must cover the drift classes named in Phase 86 D-19. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
| --- | --- | --- | --- |
| Treat daemon startup, elapsed time, raw logs, or bundle paths as useful proof. [VERIFIED: docs/parity/support-matrix.md; docs/parity/operator-runbooks.md] | Require field-level evidence and unavailable reasons. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; docs/operator/runtime-guide.md] | Established across v1.7/v1.8 evidence docs. [VERIFIED: docs/parity/release-readiness.md; docs/parity/operator-runbooks.md] | Phase 86 docs should require status, sync, service, log, metric, resource, recovery, and support fields. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |
| Use runtime guide as the only service source. [VERIFIED: docs/operator/runtime-guide.md] | Add one canonical `docs/parity/service-operation-expectations.md` and link to it. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | Locked for Phase 86. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | Reduces duplicated support-policy drift across entrypoints. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |
| Real service-manager checks in default verification. [VERIFIED: scripts/check-phase63-service-lifecycle.ts; scripts/check-phase64-service-restart-resume.ts] | Keep real launchd/systemd lifecycle as opt-in UAT and default verification as deterministic text/docs/source checks. [VERIFIED: docs/parity/support-matrix.md; scripts/verify.sh] | Established before Phase 86 and locked in D-19. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | Prevents local machine mutation and nondeterministic CI behavior. [VERIFIED: docs/operator/runtime-guide.md; scripts/verify.sh] |
| Alias-oriented operator examples. [VERIFIED: docs/operator/runtime-guide.md] | Repo-local Cargo and Bazel command forms for operator workflows. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md] | Repo-local guidance and Phase 86 D-06. [VERIFIED: AGENTS.md; .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | Operators and reviewers can reproduce evidence from the checkout. [VERIFIED: AGENTS.md] |
| Checker command present only in the legacy visible order block. [VERIFIED: scripts/verify.sh] | Checker also verifies executed `run_step` order after stripping the heredoc. [VERIFIED: scripts/check-phase85-operator-runbooks.ts; scripts/check-phase85-operator-runbooks.test.ts] | Established by Phase 85 checker pattern. [VERIFIED: scripts/check-phase85-operator-runbooks.ts] | Prevents false verifier-wiring confidence. [VERIFIED: scripts/check-phase85-operator-runbooks.test.ts] |

**Deprecated/outdated for Phase 86:**

- Alias-only `open-bitcoin` examples are out of pattern for operator-facing Phase 86 evidence. [VERIFIED: AGENTS.md; .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
- Production-service, package-supported, managed-service, and community-supported labels are forbidden alternate support terms. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
- Live public-network, real service-manager, long wall-clock, and automatic upload default checks are out of scope. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/verify.sh]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
| --- | --- | --- | --- |
| A1 | The alternatives row mentions Python, table duplication, broad scanner, and alias-only command forms only as rejected alternatives inferred from locked decisions. [ASSUMED] | Standard Stack | Low: planner should treat the locked decisions, not these examples, as binding. |
| A2 | The STRIDE labels for production-service wording drift are an analyst classification, not a project-standard taxonomy. [ASSUMED] | Security Domain | Low: mitigation still follows locked Phase 86 wording guardrails even if the label changes. |
| A3 | The STRIDE label for default verifier service-manager mutation is an analyst classification. [ASSUMED] | Security Domain | Low: mitigation still follows deterministic verifier and no-mutation constraints. |
| A4 | The STRIDE label for sensitive support artifacts is an analyst classification. [ASSUMED] | Security Domain | Low: mitigation still follows verified redaction and local-only support bundle boundaries. |
| A5 | The STRIDE labels for artifact-presence-only checking are analyst classifications. [ASSUMED] | Security Domain | Low: mitigation still follows verified field-based evidence requirements. |
| A6 | The 30-day validity window is a planning freshness estimate. [ASSUMED] | Metadata | Low: planner can re-run research if Phase 86 context, verifier wiring, or v1.8 scope changes sooner. |

## Open Questions (RESOLVED)

1. **Should Phase 86 include Rust source changes? RESOLVED: no Rust work by default; only add Rust source/test changes if implementation discovers a narrow source behavior gap, and update parity breadcrumbs if that happens.** [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
   - What we know: No Rust source changes are expected and the phase can stay docs plus Bun automation if no source behavior gap is found. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
   - What's unclear: Implementation may discover a narrow source behavior gap while writing the service expectation doc or checker. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]
   - Recommendation: Plan no Rust work by default, but include a contingency that any new first-party Rust source/test file updates parity breadcrumbs. [VERIFIED: AGENTS.md; .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
| --- | --- | --- | --- | --- |
| Bun | Phase 86 checker/test and repo verification [VERIFIED: AGENTS.md; scripts/verify.sh] | yes [VERIFIED: command `bun --version`] | 1.3.9 [VERIFIED: command `bun --version`] | None needed. [VERIFIED: command `bun --version`] |
| Bash | `scripts/verify.sh` [VERIFIED: scripts/verify.sh] | yes [VERIFIED: command `bash --version`] | GNU bash 3.2.57 [VERIFIED: command `bash --version`] | None needed. [VERIFIED: command `bash --version`] |
| Cargo | Repo-local commands and verifier [VERIFIED: AGENTS.md; scripts/verify.sh] | yes [VERIFIED: command `cargo --version`] | 1.94.1 [VERIFIED: command `cargo --version`] | None needed. [VERIFIED: command `cargo --version`] |
| rustc | Rust toolchain for verifier [VERIFIED: AGENTS.md; scripts/verify.sh] | yes [VERIFIED: command `rustc --version`] | 1.94.1 [VERIFIED: command `rustc --version`] | None needed. [VERIFIED: command `rustc --version`] |
| Bazel | Repo-local Bazel command forms and full verifier smoke build [VERIFIED: AGENTS.md; scripts/verify.sh] | yes [VERIFIED: command `bazel --version`] | 8.6.0 [VERIFIED: command `bazel --version`] | None needed. [VERIFIED: command `bazel --version`] |
| cargo-llvm-cov | Full `bash scripts/verify.sh` coverage step [VERIFIED: scripts/verify.sh] | yes [VERIFIED: command `cargo llvm-cov --version`] | 0.8.5 [VERIFIED: command `cargo llvm-cov --version`] | None needed for full closeout. [VERIFIED: command `cargo llvm-cov --version`] |
| Git | Revision evidence and normal repo workflow [VERIFIED: docs/parity/operator-runbooks.md; standards/core/verification.md] | yes [VERIFIED: command `git --version`] | 2.53.0 [VERIFIED: command `git --version`] | None needed. [VERIFIED: command `git --version`] |

**Missing dependencies with no fallback:** None found for Phase 86 planning. [VERIFIED: environment availability commands]

**Missing dependencies with fallback:** None found for Phase 86 planning. [VERIFIED: environment availability commands]

## Security Domain

OWASP ASVS latest stable is version 5.0.0 dated May 2025, and OWASP recommends versioned requirement identifiers because unversioned identifiers follow the latest content. [CITED: https://github.com/OWASP/ASVS] Phase 86 is a docs/checker phase and should not add authentication, session, access-control, or cryptography behavior. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md]

### Applicable Security Categories

| Security Category | Applies | Standard Control |
| --- | --- | --- |
| Authentication | no [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | Do not add auth/session behavior in Phase 86. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |
| Session Management | no [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | No session state is in scope for the service expectation doc/checker. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |
| Access Control | limited [VERIFIED: docs/operator/runtime-guide.md; docs/parity/support-matrix.md] | Preserve local user-level service boundaries and avoid system-wide service ownership claims. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; docs/operator/runtime-guide.md] |
| Input Validation / Command Injection | yes [CITED: https://github.com/OWASP/ASVS; VERIFIED: scripts/check-phase85-operator-runbooks.ts] | Keep checker targets fixed, parse JSON with `JSON.parse`, and do not generate shell commands from untrusted input. [VERIFIED: scripts/check-phase85-operator-runbooks.ts] |
| Cryptography | no [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | No new crypto, signing, package provenance, or credential-handling implementation is in scope. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |
| Logging / Privacy | yes [VERIFIED: docs/parity/support-matrix.md; docs/parity/operator-runbooks.md] | Preserve redaction boundaries for RPC cookies, passwords, wallet material, raw datadirs, unredacted logs, and automatic support upload. [VERIFIED: docs/parity/support-matrix.md; docs/parity/operator-runbooks.md] |

### Known Threat Patterns for Phase 86

| Pattern | STRIDE | Standard Mitigation |
| --- | --- | --- |
| Docs imply production service support or broad production-node readiness. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | Spoofing / Tampering [ASSUMED] | Checker rejects forbidden service ownership, Windows service, package-manager, automatic upload, and broad readiness wording in Phase 86 targets. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |
| Default verifier mutates local service manager state. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/verify.sh] | Tampering [ASSUMED] | Checker rejects executable `systemctl`, `launchctl`, package-manager service commands, long sleeps, and public-network live smoke. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/check-phase85-operator-runbooks.ts] |
| Support guidance asks for sensitive raw artifacts. [VERIFIED: docs/parity/support-matrix.md; docs/parity/operator-runbooks.md] | Information Disclosure [ASSUMED] | Preserve "do not attach" redaction list and local-only support bundle boundary. [VERIFIED: docs/parity/support-matrix.md; docs/parity/operator-runbooks.md] |
| Checker validates artifact presence but not fields. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] | Repudiation / Tampering [ASSUMED] | Require lifecycle labels, restart/resume fields, service log path, service manager command strings, generated service file path, and unavailable reasons. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/86-service-operation-expectations/86-CONTEXT.md` - locked Phase 86 scope, decisions, command evidence, traceability, and verification requirements. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - SVC-01, SVC-02, and v1.8 out-of-scope boundaries. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 86 goal, dependencies, success criteria, and Phase 88 separation. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/*` - repo-local and Bright Builds workflow, verification, testing, Rust, and TypeScript constraints. [VERIFIED: file read]
- `docs/operator/runtime-guide.md` - service lifecycle, restart/resume, status/sync status, logs, metrics, resource, support-bundle, Cargo/Bazel command forms, and non-claim boundaries. [VERIFIED: file read]
- `docs/parity/production-claim-boundary.md`, `support-matrix.md`, `operator-runbooks.md`, `upgrade-and-rollback-policy.md`, `release-readiness.md`, `deviations-and-unknowns.md`, `checklist.md`, `README.md`, `index.json`, and `catalog/operator-runtime-release-hardening.md` - v1.8 parity roots and service/support/production-boundary context. [VERIFIED: file read]
- `scripts/check-phase63-service-lifecycle.ts`, `scripts/check-phase64-service-restart-resume.ts`, `scripts/check-phase85-operator-runbooks.ts`, `scripts/check-phase85-operator-runbooks.test.ts`, and `scripts/verify.sh` - existing checker/test/verifier patterns. [VERIFIED: file read]
- Local version commands for Bun, Cargo, rustc, Bazel, Bash, Git, cargo-llvm-cov, ripgrep, Node, and jq. [VERIFIED: command outputs]

### Secondary (MEDIUM confidence)

- OWASP ASVS GitHub README - latest stable ASVS version and versioned identifier guidance used only for security-domain framing. [CITED: https://github.com/OWASP/ASVS]

### Tertiary (LOW confidence)

- None. [VERIFIED: this research artifact]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - tool versions were checked locally and no new dependencies are recommended. [VERIFIED: command outputs; AGENTS.md]
- Architecture: HIGH - Phase 86 decisions and Phase 82-85 patterns explicitly define the doc/checker shape. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/check-phase85-operator-runbooks.ts]
- Pitfalls: HIGH - failure modes are named in locked decisions and existing checkers already enforce adjacent boundaries. [VERIFIED: .planning/phases/86-service-operation-expectations/86-CONTEXT.md; scripts/check-phase63-service-lifecycle.ts; scripts/check-phase64-service-restart-resume.ts; scripts/check-phase85-operator-runbooks.ts]

**Research date:** 2026-06-22 [VERIFIED: environment_context current_date]
**Valid until:** 2026-07-22 for repo-internal planning assumptions, unless v1.8 scope, `scripts/verify.sh`, or Phase 86 context changes first. [ASSUMED]
