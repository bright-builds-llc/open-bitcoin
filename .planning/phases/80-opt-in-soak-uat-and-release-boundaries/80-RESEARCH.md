# Phase 80: Opt-In Soak UAT and Release Boundaries - Research

**Researched:** 2026-06-17  
**Domain:** Repo-native deterministic verification, opt-in operator UAT docs, parity/release-boundary closeout  
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

Source for all bullets in this section: [VERIFIED: .planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-CONTEXT.md]

### Locked Decisions

### Default Verification Boundary

- **D-01:** Add a focused Phase 80 deterministic boundary checker in the
  existing Bun/TypeScript style, with fixture tests, and wire both into
  `bash scripts/verify.sh`.
- **D-02:** The checker must prove the default verification path remains local,
  short-running, public-network-free, real-service-manager-free,
  multi-day-sleep-free, current-tip-timing-free, and free of large-disk
  allocation requirements.
- **D-03:** Guard `scripts/verify.sh` against accidental default invocation of
  live-mainnet smoke, manual peers, `--restart-after-progress`, real
  `systemctl` or `launchctl`, `-openbitcoinsync=mainnet-ibd`, multi-day sleeps,
  current-tip/release-blocking timing gates, `/proc` or `lsof` process scans,
  and large-disk stress paths.
- **D-04:** Do not add a runtime sandbox, hermetic container, or strict offline
  dependency mode as the Phase 80 proof. Those are future CI/release-engineering
  choices, not required for this closeout.

### Opt-In UAT Command Matrix

- **D-05:** Add a focused Phase 80 v1.7 UAT matrix instead of a broad v1.6-style
  scenario sweep. The matrix should cover exactly these operator workflows:
  multi-day soak lifecycle, bounded recovery drill, support-bundle generation,
  and post-failure diagnosis.
- **D-06:** Each CLI-backed workflow must provide copy-pasteable repo-local Cargo
  and Bazel command forms. Prefer
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.
- **D-07:** The matrix may reference existing Phase 75 soak lifecycle commands,
  Phase 77 recovery/status guidance, Phase 79 support-forensics commands, and
  deterministic fixture checks, but it should be one reviewer-friendly Phase 80
  entrypoint.
- **D-08:** The UAT wording must state what each workflow can prove and what it
  does not prove. Artifact existence, daemon startup, elapsed time, peer
  reachability, raw logs, stale reports, and support-bundle presence are not
  enough to prove soak stability or production readiness.

### Parity And Audit Closure

- **D-09:** Use one Phase 80 closure checker rather than a new manifest-driven
  evidence registry. The checker should require v1.7 evidence roots across
  docs, parity files, checkers, support schema anchors, and `scripts/verify.sh`
  ordering.
- **D-10:** Keep `docs/parity/source-breadcrumbs.json` and
  `scripts/check-parity-breadcrumbs.ts --check` as the required mechanism for
  new first-party Rust source or test files under `packages/open-bitcoin-*/src`
  or `packages/open-bitcoin-*/tests`.
- **D-11:** If Phase 80 adds Rust source or tests, add the required parity
  breadcrumb mapping and keep the breadcrumb checker green. If the phase stays
  docs and Bun checker only, no breadcrumb mapping change should be needed.
- **D-12:** The closure checker should assert that Phase 75 through Phase 79
  deterministic checkers remain wired before the Phase 80 checker and that the
  v1.7 roots mention VER-05, VER-06, VER-07, and REL-04.

### Release Boundary Wording

- **D-13:** Use a parity-rooted v1.7 boundary closeout. Update and guard
  README, `docs/operator/runtime-guide.md`, `docs/parity/release-readiness.md`,
  `docs/parity/README.md`, `docs/parity/checklist.md`,
  `docs/parity/index.json`, `docs/parity/deviations-and-unknowns.md`, and
  `docs/parity/catalog/operator-runtime-release-hardening.md` as needed.
- **D-14:** The v1.7 claim shape is explicit opt-in full-sync soak and recovery
  hardening: durable multi-day soak evidence, resource bounds, recovery
  diagnosis, progress guarantees, stall diagnosis, support-bundle forensics,
  opt-in UAT commands, and deterministic release-boundary checks.
- **D-15:** Preserve the non-claim list wherever Phase 80 touches docs, parity
  roots, checker constants, or status wording: inbound serving, address relay,
  block serving, transaction relay, compact block relay, production-funds wallet
  use, migration apply mode, signed packaging, Windows service support, GUI,
  hosted dashboards, public-network default checks, public-network CI,
  release-blocking live sync, automatic support-bundle upload, destructive
  repair, and broad production-node readiness.
- **D-16:** Add targeted status/output guards only where exact operator text is
  claim-bearing. Avoid broad text scans that would make legitimate historical
  non-claim wording brittle.

### the agent's Discretion

- The planner may split Phase 80 into UAT command documentation, parity/release
  root refresh, deterministic checker/test wiring, and final verification
  evidence.
- The executor may keep Phase 80 primarily in docs and Bun checker code if no
  source behavior gap is found.
- The executor may reuse the Phase 79 checker/test structure with
  v1.7-specific requirement ids, evidence paths, forbidden default-verification
  strings, and non-claim terms.

### Deferred Ideas (OUT OF SCOPE)

- Runtime sandboxing or containerized offline proof for `scripts/verify.sh`.
- Signed or externally comparable support/soak artifacts.
- A reusable v1.x evidence manifest system.
- Production-node expansion, inbound serving, relay, wallet production safety,
  migration apply mode, packaging, GUI, hosted dashboards, and public-network CI.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| VER-05 | Contributor can run `bash scripts/verify.sh` without internet access, public peers, real service managers, multi-day sleeps, current-tip timing, or large disk consumption. [VERIFIED: .planning/REQUIREMENTS.md] | Implement one Phase 80 Bun checker/test pair and wire it after Phase 79 while extending the forbidden default-verification string set. [VERIFIED: scripts/verify.sh; scripts/check-phase79-diagnostics-support-bundle.ts] |
| VER-06 | Operator can run copy-pasteable repo-local Cargo and Bazel commands for opt-in multi-day soak, bounded recovery drills, support-bundle generation, and post-failure diagnosis. [VERIFIED: .planning/REQUIREMENTS.md] | Add one Phase 80 v1.7 UAT matrix to `docs/operator/runtime-guide.md` using the existing Cargo/Bazel command style. [VERIFIED: docs/operator/runtime-guide.md; AGENTS.md] |
| VER-07 | Contributor can audit parity breadcrumbs, fixtures, support bundle schemas, deterministic checkers, and operator docs for every new v1.7 source, test, and evidence surface. [VERIFIED: .planning/REQUIREMENTS.md] | Guard `docs/parity/source-breadcrumbs.json`, checker fixture coverage, typed support/soak schema anchors, deterministic checker ordering, and Phase 80 docs/parity roots. [VERIFIED: scripts/check-parity-breadcrumbs.ts; packages/open-bitcoin-cli/src/operator/support.rs; packages/open-bitcoin-cli/src/operator/support/forensics.rs] |
| REL-04 | Contributor can verify v1.7 docs and status surfaces describe only explicit opt-in soak and recovery hardening, not broad production-node readiness. [VERIFIED: .planning/REQUIREMENTS.md] | Refresh README/runtime/parity release roots and require the Phase 80 checker to preserve the explicit v1.7 claim and non-claim list. [VERIFIED: .planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-CONTEXT.md; docs/parity/release-readiness.md] |
</phase_requirements>

## Summary

Phase 80 should be planned as a closeout and guardrail phase, not a behavior-expansion phase. The locked decisions require one focused Bun/TypeScript checker with fixture tests, one focused v1.7 UAT matrix, parity-root updates, and release-boundary wording that keeps public-network and multi-day evidence opt-in. [VERIFIED: 80-CONTEXT.md; scripts/check-phase79-diagnostics-support-bundle.ts; docs/operator/runtime-guide.md]

The current repo already has the implementation pattern Phase 80 needs: Phase 75 through Phase 79 checkers assert docs, parity anchors, support/soak evidence fields, and `scripts/verify.sh` ordering, and `scripts/verify.sh` currently runs Phase 75 through Phase 79 checker tests/checkers before Rust, benchmark, Bazel, and coverage gates. [VERIFIED: scripts/verify.sh; scripts/check-phase75-soak-runner.ts; scripts/check-phase79-diagnostics-support-bundle.ts]

**Primary recommendation:** Use the existing Bun checker/test plus runtime-guide/parity-root pattern; do not add new dependencies, new manifests, runtime sandboxes, public-network default checks, or broad release-package artifacts. [VERIFIED: 80-CONTEXT.md; AGENTS.md; .planning/config.json]

## Project Constraints (from AGENTS.md)

- Use `AGENTS.md` as the repo-local instruction entrypoint, then `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant `standards/` pages. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards/index.md]
- Use `git submodule update --init --recursive` to materialize the pinned Knots baseline when needed. [VERIFIED: AGENTS.md]
- Treat `rust-toolchain.toml` as the Rust source of truth; the pinned local toolchain is Rust `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code and Bazel smoke build. [VERIFIED: AGENTS.md; scripts/verify.sh]
- During UAT, provide repo-local Cargo and Bazel commands, preferring `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation scripts; prefer TypeScript for substantial script logic and Bash for thin wrappers. [VERIFIED: AGENTS.md; .planning/STACK.md]
- Do not add a `bun install` bootstrap step while this repo has no `package.json`. [VERIFIED: .planning/STACK.md; local `test -f package.json` probe]
- `bash scripts/install-git-hooks.sh` installs repo-managed hooks, and `bash scripts/verify.sh` self-heals missing local hooks outside CI. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Treat `docs/metrics/lines-of-code.md` as tracked generated output that may need freshness updates when verification regenerates it. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Record intentional in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md; docs/parity/index.json]
- If adding first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update `docs/parity/source-breadcrumbs.json` and keep `scripts/check-parity-breadcrumbs.ts --check` green. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts]
- After substantial feature, parity, operator-surface, or workflow changes, check README docs for needed updates. [VERIFIED: AGENTS.md]
- Follow functional-core / imperative-shell boundaries and keep pure business logic free of direct I/O and runtime effects. [VERIFIED: AGENTS.md; standards/core/architecture.md; .planning/ARCHITECTURE.md]
- Keep tests focused and use Arrange, Act, Assert comments for non-trivial unit tests. [VERIFIED: AGENTS.md; standards/core/testing.md]
- For Rust work before commit, run `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo build --all-targets --all-features`, and `cargo test --all-features`; the repo-native equivalent is already encoded in `bash scripts/verify.sh`. [VERIFIED: AGENTS.md; scripts/verify.sh]
- For TypeScript/Bun automation, use `maybe...` naming for nullable values and avoid class inheritance in repo-owned behavior. [VERIFIED: standards/languages/typescript-javascript.md]
- No project-local `.claude/skills/` or `.agents/skills/` directories are present. [VERIFIED: local `find .claude/skills .agents/skills` probe]

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|---|---:|---|---|
| Bun | 1.3.9 pinned and installed | Execute repo-owned TypeScript checkers and checker tests. [VERIFIED: .bun-version; `bun --version`; scripts/verify.sh] | Existing Phase 73 and Phase 75-79 deterministic checkers use Bun, and repo guidance names Bun as the automation runtime. [VERIFIED: AGENTS.md; scripts/check-phase79-diagnostics-support-bundle.ts] |
| TypeScript in `scripts/` | Repo-owned scripts, no `package.json` | Implement Phase 80 checker and fixture tests without adding package-manager state. [VERIFIED: package.json absence; .planning/STACK.md] | Existing checkers are single-file Bun/TS scripts with local fixtures. [VERIFIED: scripts/check-phase79-diagnostics-support-bundle.ts; scripts/check-phase79-diagnostics-support-bundle.test.ts] |
| Bash `scripts/verify.sh` | GNU bash 3.2.57 installed | Repo-native aggregate verifier and checker ordering point. [VERIFIED: `bash --version`; scripts/verify.sh] | AGENTS and standards require repo-native verification before done. [VERIFIED: AGENTS.md; standards/core/verification.md] |
| Rust/Cargo | 1.94.1 pinned and installed | Existing first-party code and tests if Phase 80 discovers a behavior gap. [VERIFIED: rust-toolchain.toml; `cargo --version`; `rustc --version`] | Rust is the repo's production implementation language; Phase 80 should avoid Rust changes unless necessary. [VERIFIED: AGENTS.md; 80-CONTEXT.md] |
| Bazel / rules_rust | Bazel 8.6.0 installed; `rules_rust` 0.69.0 | Verify Bazel command forms and existing smoke build. [VERIFIED: `bazel --version`; MODULE.bazel; scripts/verify.sh] | UAT docs must include Bazel forms for CLI-backed workflows. [VERIFIED: AGENTS.md; 80-CONTEXT.md] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|---|---:|---|---|
| `cargo-llvm-cov` | 0.8.5 installed | Existing coverage gate in `scripts/verify.sh`. [VERIFIED: `cargo llvm-cov --version`; scripts/verify.sh] | Required by full repo verification, not a Phase 80 implementation dependency. [VERIFIED: scripts/verify.sh] |
| `git` | 2.53.0 installed | Source-breadcrumb checker uses tracked files from git. [VERIFIED: `git --version`; scripts/check-parity-breadcrumbs.ts] | Needed for `scripts/check-parity-breadcrumbs.ts --check`. [VERIFIED: scripts/check-parity-breadcrumbs.ts] |
| `grep` | BSD grep 2.6.0 compatible installed | Existing verifier dependency. [VERIFIED: `grep --version`; scripts/verify.sh] | Required by the aggregate verifier and coverage failure scan. [VERIFIED: scripts/verify.sh] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|---|---|---|
| Bun/TS checker | Runtime sandbox or hermetic container | Explicitly out of scope for Phase 80; it would test environment policy rather than the repo's deterministic verifier boundary. [VERIFIED: 80-CONTEXT.md] |
| One closure checker | New manifest-driven evidence registry | Explicitly rejected; it would duplicate existing parity roots and source breadcrumbs. [VERIFIED: 80-CONTEXT.md; docs/parity/index.json] |
| Focused v1.7 UAT matrix | Broad v1.6-style matrix sweep | Explicitly rejected; Phase 80 should cover only multi-day soak lifecycle, bounded recovery drill, support-bundle generation, and post-failure diagnosis. [VERIFIED: 80-CONTEXT.md] |
| Typed support/soak anchors | Standalone JSON schema registry | No standalone support-bundle JSON schema file was found; current schemas are typed Rust `Serialize` structs and documented JSON/Markdown outputs. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs; packages/open-bitcoin-cli/src/operator/support/forensics.rs] |

**Installation:**

No new package install is recommended for Phase 80. [VERIFIED: 80-CONTEXT.md; .planning/STACK.md]

```bash
git submodule update --init --recursive
bash scripts/install-git-hooks.sh
bash scripts/verify.sh
```

**Version verification:** Recommended package versions were verified from local pins and installed tools, not registry freshness, because Phase 80 should add no npm/Cargo dependency. [VERIFIED: .bun-version; rust-toolchain.toml; MODULE.bazel; local version probes]

## Architecture Patterns

### Recommended Project Structure

```text
scripts/
├── check-phase80-opt-in-soak-uat-release-boundaries.ts       # Phase 80 closure checker [VERIFIED: scripts/check-phase79-diagnostics-support-bundle.ts pattern]
└── check-phase80-opt-in-soak-uat-release-boundaries.test.ts  # Fixture tests [VERIFIED: scripts/check-phase79-diagnostics-support-bundle.test.ts pattern]

docs/
├── operator/runtime-guide.md                                # Focused v1.7 UAT matrix [VERIFIED: 80-CONTEXT.md]
└── parity/
    ├── release-readiness.md                                 # v1.7 boundary closeout [VERIFIED: 80-CONTEXT.md]
    ├── index.json                                           # Machine-readable root [VERIFIED: docs/parity/index.json]
    ├── checklist.md                                         # Human checklist root [VERIFIED: docs/parity/checklist.md]
    ├── README.md                                            # Parity entrypoint [VERIFIED: docs/parity/README.md]
    ├── deviations-and-unknowns.md                           # Deferred scope [VERIFIED: docs/parity/deviations-and-unknowns.md]
    └── catalog/operator-runtime-release-hardening.md        # Phase 75-80 runtime catalog [VERIFIED: docs/parity/catalog/operator-runtime-release-hardening.md]
```

### Pattern 1: Closure Checker With Fixture Root

**What:** Add a Bun script that reads repo files, asserts required anchors, parses structured JSON where structure matters, rejects forbidden default-verifier strings, and exits non-zero with concrete failures. [VERIFIED: scripts/check-phase79-diagnostics-support-bundle.ts; scripts/check-v1.6-release-boundaries.ts]

**When to use:** Use this for Phase 80 because the locked decision is a deterministic docs/parity/verifier boundary gate. [VERIFIED: 80-CONTEXT.md]

**Example:**

```typescript
// Source: scripts/check-phase79-diagnostics-support-bundle.ts and scripts/check-v1.6-release-boundaries.ts
const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE80_REPO_ROOT";
const maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV];
const REPO_ROOT =
  maybeRepoRoot === undefined ? path.resolve(import.meta.dir, "..") : path.resolve(maybeRepoRoot);

function requireContains(text: string, needle: string, label: string, failures: string[]): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function requireNotContains(text: string, needle: string, label: string, failures: string[]): void {
  if (text.includes(needle)) {
    failures.push(`${label} must not contain Phase 80 forbidden text: ${needle}`);
  }
}
```

### Pattern 2: Verify Wiring Order

**What:** Add `bun test scripts/check-phase80-...test.ts` and `bun run scripts/check-phase80-...ts` immediately after the Phase 79 checker in `scripts/verify.sh`, then have the Phase 80 checker assert Phase 75 through Phase 79 checker order remains before it. [VERIFIED: scripts/verify.sh; 80-CONTEXT.md]

**When to use:** This satisfies VER-05 and D-12 by making default verification fail if the closure gate is omitted or moved before prerequisite phase checkers. [VERIFIED: 80-CONTEXT.md; .planning/REQUIREMENTS.md]

**Example:**

```typescript
// Source: scripts/check-phase79-diagnostics-support-bundle.ts
const PHASE79_CHECKER_COMMAND = "bun run scripts/check-phase79-diagnostics-support-bundle.ts";
const PHASE80_TEST_COMMAND =
  "bun test scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts";
const PHASE80_CHECKER_COMMAND =
  "bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts";

function verifyVerifyScript(failures: string[]): void {
  const lines = readText("scripts/verify.sh", failures)
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
  const phase79Index = lines.indexOf(PHASE79_CHECKER_COMMAND);
  if (lines.indexOf(PHASE80_TEST_COMMAND) !== phase79Index + 1) {
    failures.push("scripts/verify.sh must run the Phase 80 checker test after Phase 79");
  }
  if (lines.indexOf(PHASE80_CHECKER_COMMAND) !== phase79Index + 2) {
    failures.push("scripts/verify.sh must run the Phase 80 checker after its test");
  }
}
```

### Pattern 3: Focused UAT Matrix, Not Scattered Commands

**What:** Put one Phase 80 v1.7 UAT matrix in `docs/operator/runtime-guide.md` with four rows: multi-day soak lifecycle, bounded recovery drill, support-bundle generation, and post-failure diagnosis. [VERIFIED: 80-CONTEXT.md; docs/operator/runtime-guide.md]

**When to use:** Use this instead of editing older Phase 73/v1.6 matrices into a broader sweep. [VERIFIED: 80-CONTEXT.md]

**Example command style:**

```bash
# Source: AGENTS.md and docs/operator/runtime-guide.md
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  --network mainnet \
  soak report --run-id <run-id>

bazel run //packages/open-bitcoin-cli:open_bitcoin -- \
  --datadir=/tmp/open-bitcoin-mainnet \
  --network mainnet \
  soak report --run-id <run-id>
```

### Pattern 4: Parity Roots As Evidence Registry

**What:** Update existing parity roots instead of adding a new registry: `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md`, and `docs/parity/catalog/operator-runtime-release-hardening.md`. [VERIFIED: 80-CONTEXT.md; docs/parity/index.json; docs/parity/checklist.md]

**When to use:** Use this for VER-07 and REL-04 because the repo already treats parity roots as the machine/human audit surface. [VERIFIED: AGENTS.md; docs/parity/release-readiness.md]

### Anti-Patterns to Avoid

- **Broad text scans over all docs:** They will hit legitimate historical v1.3-v1.6 wording and create brittle false positives. Use targeted files and exact claim-bearing anchors. [VERIFIED: 80-CONTEXT.md; docs/parity/release-readiness.md]
- **Artifact-existence proof:** Do not describe support bundles, reports, daemon startup, peer reachability, elapsed time, or raw logs as proof of soak stability. [VERIFIED: 80-CONTEXT.md; docs/operator/runtime-guide.md]
- **Default public-network proof:** Do not add `run-live-mainnet-smoke`, `--manual-peer`, `--restart-after-progress`, mainnet IBD activation, service-manager calls, process-table scans, multi-day sleeps, current-tip timing gates, or large-disk stress to `scripts/verify.sh`. [VERIFIED: 80-CONTEXT.md; scripts/check-phase79-diagnostics-support-bundle.ts]
- **New release-manifest layer:** Do not add a v1.x evidence manifest; use existing parity roots and source breadcrumbs. [VERIFIED: 80-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---|---|---|---|
| Evidence registry | New manifest-driven v1.7 registry | `docs/parity/index.json`, checklist, README, release-readiness, catalog pages | Existing roots are already machine/human audit surfaces and the user rejected a new manifest. [VERIFIED: 80-CONTEXT.md; docs/parity/index.json] |
| Offline proof | Sandbox/container/strict offline dependency mode | Deterministic checker over `scripts/verify.sh` contents and repo docs | Locked decision excludes runtime sandboxing for this phase. [VERIFIED: 80-CONTEXT.md] |
| Support-bundle schema registry | Custom JSON schema generator | Typed Rust `Serialize` structs plus checker anchors | Current bundle schema is represented by `SupportEvidenceBundle`, `SupportForensicsEvidence`, `SoakReportProjection`, and `SoakLedgerEventEnvelope`. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs; packages/open-bitcoin-cli/src/operator/support/forensics.rs; packages/open-bitcoin-cli/src/operator/soak/report.rs; packages/open-bitcoin-cli/src/operator/soak/ledger.rs] |
| Long-run verification | Multi-day sleeps or current-tip timing gates | Opt-in UAT commands plus deterministic fixture/checker coverage | Default verification must stay short-running and public-network-free. [VERIFIED: .planning/REQUIREMENTS.md; scripts/verify.sh] |
| Real service checks | Direct `systemctl`, `launchctl`, `/proc`, or `lsof` checks in default verifier | Existing fake/deterministic docs and checker assertions | Phase 80 must keep real service-manager and process-table evidence out of the default gate. [VERIFIED: 80-CONTEXT.md; scripts/check-phase77-corruption-lock-recovery.ts] |
| Broad production readiness wording | Marketing/release claim prose | Explicit opt-in soak and recovery hardening boundary language | v1.7 out of scope excludes broad production-node readiness and production-funds wallet safety. [VERIFIED: .planning/REQUIREMENTS.md; .planning/ROADMAP.md] |

**Key insight:** Phase 80 is mostly about preventing evidence inflation. The project already has typed evidence and deterministic checkers; planning should connect them, not invent new release infrastructure. [VERIFIED: 80-CONTEXT.md; docs/parity/threat-model-v1.6.md]

## Common Pitfalls

### Pitfall 1: Treating UAT Artifacts As Proof

**What goes wrong:** A plan says a support bundle, report file, daemon startup, peer reachability, or elapsed runtime proves soak stability. [VERIFIED: 80-CONTEXT.md; docs/operator/runtime-guide.md]  
**Why it happens:** Prior matrices contain many useful commands, but Phase 79 explicitly tightened interpretation to typed `support_forensics` verdicts and evidence basis. [VERIFIED: docs/operator/runtime-guide.md; scripts/check-phase79-diagnostics-support-bundle.ts]  
**How to avoid:** Require each UAT row to state "Evidence proves" and "Does not prove" and make the checker anchor those phrases. [VERIFIED: scripts/check-phase73-uat-verification.ts; 80-CONTEXT.md]  
**Warning signs:** Matrix rows mention "ran for N days", "bundle exists", "daemon started", or "peer reachable" without validated durable progress, forensics verdict, or next-action evidence. [VERIFIED: docs/operator/runtime-guide.md]

### Pitfall 2: Letting Opt-In Commands Enter `scripts/verify.sh`

**What goes wrong:** Public-network smoke, manual peers, mainnet IBD activation, real service managers, process scans, multi-day sleeps, current-tip timing gates, or large-disk paths become default verification. [VERIFIED: 80-CONTEXT.md]  
**Why it happens:** Older operator docs correctly document opt-in public-network/service commands, and a broad copy can accidentally paste them into the verifier. [VERIFIED: docs/operator/runtime-guide.md; scripts/verify.sh]  
**How to avoid:** Extend `FORBIDDEN_VERIFY_STRINGS` from Phase 79 with Phase 80's complete forbidden list and add fixture tests that fail on each class. [VERIFIED: scripts/check-phase79-diagnostics-support-bundle.ts; 80-CONTEXT.md]  
**Warning signs:** `scripts/verify.sh` contains `run-live-mainnet-smoke`, `--manual-peer`, `--restart-after-progress`, `systemctl`, `launchctl`, `openbitcoinsync=mainnet-ibd`, `sleep 86400`, `lsof`, `/proc/`, or current-tip/release-blocking wording. [VERIFIED: scripts/check-phase79-diagnostics-support-bundle.ts; 80-CONTEXT.md]

### Pitfall 3: Creating a Brittle Release-Wording Scanner

**What goes wrong:** A checker scans all docs for words like "production" or "mainnet" and fails on historical non-claim text. [VERIFIED: 80-CONTEXT.md; docs/parity/release-readiness.md]  
**Why it happens:** The repo preserves v1.3-v1.6 historical claim text in the same docs tree. [VERIFIED: .planning/PROJECT.md; docs/parity/release-readiness.md]  
**How to avoid:** Assert exact v1.7 anchors in target files and exact non-claim lists where Phase 80 touches claim-bearing wording. [VERIFIED: 80-CONTEXT.md]  
**Warning signs:** Checker failures point at historical sections unrelated to Phase 80 or require rewriting shipped milestone history. [VERIFIED: docs/parity/release-readiness.md]

### Pitfall 4: Forgetting Conditional Rust Breadcrumbs

**What goes wrong:** New Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` lack parity breadcrumbs. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts]  
**Why it happens:** Phase 80 is likely docs/checker-only, so a small Rust follow-up could be added late and miss the established breadcrumb mechanism. [VERIFIED: 80-CONTEXT.md]  
**How to avoid:** Plan a conditional task: if Rust files are added, update `docs/parity/source-breadcrumbs.json`, add header blocks, and run `bun run scripts/check-parity-breadcrumbs.ts --check`. [VERIFIED: AGENTS.md; scripts/check-parity-breadcrumbs.ts]  
**Warning signs:** `git diff --name-only` includes `packages/open-bitcoin-*/src/*.rs` or `packages/open-bitcoin-*/tests/*.rs` without a corresponding breadcrumb mapping update. [VERIFIED: scripts/check-parity-breadcrumbs.ts]

### Pitfall 5: Misplacing v1.7 Release Boundary

**What goes wrong:** README and parity docs continue to describe v1.6 as the current release boundary or jump to broad v1.7 production readiness. [VERIFIED: README.md; docs/parity/release-readiness.md]  
**Why it happens:** `README.md` and `docs/parity/release-readiness.md` currently center the release boundary on v1.6, while v1.7 sections are partial Phase 75-79 scoped inserts. [VERIFIED: README.md; docs/parity/release-readiness.md]  
**How to avoid:** Add a scoped v1.7 boundary closeout that says "explicit opt-in full-sync soak and recovery hardening" and lists the non-claims. [VERIFIED: 80-CONTEXT.md; .planning/ROADMAP.md]  
**Warning signs:** Current docs say "v1.6 current release boundary" after Phase 80 or use "production-node readiness" as a positive claim. [VERIFIED: README.md; docs/parity/release-readiness.md]

## Code Examples

Verified patterns from local sources:

### Checker Failure Accumulation

```typescript
// Source: scripts/check-phase79-diagnostics-support-bundle.ts
function main(): void {
  const failures: string[] = [];

  verifyParityCoverage(failures);
  verifyVerifyScript(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 80 opt-in soak UAT and release boundaries");
}
```

### Fixture Test Harness

```typescript
// Source: scripts/check-phase79-diagnostics-support-bundle.test.ts
test("fails_when_verify_order_or_default_boundaries_drift", async () => {
  // Arrange
  const root = await createFixture({
    maybeVerifyScript: [
      PHASE80_TEST_COMMAND,
      PHASE80_CHECKER_COMMAND,
      PHASE79_CHECKER_COMMAND,
      "bun run scripts/run-live-mainnet-smoke.ts",
    ].join("\n"),
  });

  // Act
  const result = runChecker(root);

  // Assert
  expect(result.exitCode).not.toBe(0);
});
```

### Structured Parity Index Check

```typescript
// Source: scripts/check-v1.6-release-boundaries.ts
type ChecklistSurface = {
  evidence?: unknown;
  id?: unknown;
  requirements?: unknown;
  status?: unknown;
};

function requireArrayIncludes(value: unknown, label: string, required: string): void {
  if (!Array.isArray(value)) {
    throw new Error(`${label} must be an array`);
  }
  if (!value.includes(required)) {
    throw new Error(`${label} missing required value: ${required}`);
  }
}
```

### Support Schema Anchors To Guard

```text
Source: packages/open-bitcoin-cli/src/operator/support.rs
SupportEvidenceBundle:
- generated_at_unix_seconds
- generated_by
- output
- redaction
- config
- status
- recovery_evidence
- store_health
- live_smoke
- full_sync_evidence
- soak_evidence
- support_forensics
- resource_bound_evidence
```

```text
Source: packages/open-bitcoin-cli/src/operator/support/forensics.rs
SupportForensicsEvidence:
- timeline
- checkpoint_chain
- narrative
- source
- redaction
- maybe_unavailable_reason
```

## State of the Art

| Old Approach | Current Phase 80 Approach | When Changed | Impact |
|---|---|---|---|
| Broad v1.6 full-sync UAT matrix | Focused v1.7 matrix for multi-day soak lifecycle, bounded recovery drill, support-bundle generation, and post-failure diagnosis | Phase 80 context, 2026-06-17 | Planner should not expand the matrix to daemon activation, live smoke, service restart, and status sweep unless needed as references. [VERIFIED: 80-CONTEXT.md; docs/operator/runtime-guide.md] |
| Release-boundary checker for v1.6 only | Phase 80 closure checker that guards v1.7 roots and prior Phase 75-79 checker ordering | Phase 80 context, 2026-06-17 | Planner should add one new checker/test pair and wire it after Phase 79. [VERIFIED: 80-CONTEXT.md; scripts/verify.sh] |
| Phase-local scoped claims through Phase 79 | v1.7 closeout claim across full-sync soak and recovery hardening | Phase 80 roadmap scope | Planner should update README and parity roots so v1.7 becomes discoverable without claiming production readiness. [VERIFIED: .planning/ROADMAP.md; README.md; docs/parity/release-readiness.md] |
| Support bundle as compact redacted evidence | Support bundle with `support_forensics`, forensic timeline, checkpoint chain, narrative, redaction, and size/cross-surface checks | Phase 79 verification, 2026-06-17 | Planner should guard existing typed anchors, not invent a standalone schema registry. [VERIFIED: 79-VERIFICATION.md; packages/open-bitcoin-cli/src/operator/support/forensics.rs] |

**Deprecated/outdated:**

- Treating v1.6 release boundary as the current final claim after Phase 80 is outdated once Phase 80 lands; it should become historical context while v1.7 describes scoped opt-in soak/recovery hardening. [VERIFIED: README.md; docs/parity/release-readiness.md; 80-CONTEXT.md]
- Treating public-network checks, multi-day sleeps, or real service-manager commands as default verification is explicitly out of scope. [VERIFIED: .planning/REQUIREMENTS.md; 80-CONTEXT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|---|---|---|
| A1 | The 30-day validity window is an operational freshness estimate for repo-internal research. [ASSUMED] | Metadata | If the repo changes sooner, rerun research before planning against stale anchors. |
| A2 | Whether the planner should add a new v1.7 threat model is unclear because the locked file list omits it. [ASSUMED] | Open Questions | If the user intended a threat model, the plan may need one extra docs/checker task. |

## Open Questions

1. **Should Phase 80 create a new `docs/parity/threat-model-v1.7.md`?**
   - What we know: The locked Phase 80 file list includes README, runtime guide, release-readiness, parity README/checklist/index/deviations, and operator runtime catalog, but it does not list a new v1.7 threat model. [VERIFIED: 80-CONTEXT.md]
   - What's unclear: Whether the planner should add a new threat model anyway as release-closeout polish. [ASSUMED]
   - Recommendation: Do not plan a new threat-model file unless implementation reveals a concrete claim-bearing security gap; keep Phase 80 to the locked parity-rooted closeout files. [VERIFIED: 80-CONTEXT.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|---|---|---:|---|---|
| Bun | Phase 80 checker/test and existing verify script | yes | 1.3.9 | Missing Bun blocks checker work; install per repo/toolchain policy. [VERIFIED: `.bun-version`; `bun --version`] |
| Cargo | Existing Rust verification if touched and full verify | yes | 1.94.1 | No fallback for Rust changes. [VERIFIED: `cargo --version`; scripts/verify.sh] |
| Rustc | Existing Rust build/test if touched and full verify | yes | 1.94.1 | No fallback for Rust changes. [VERIFIED: `rustc --version`; rust-toolchain.toml] |
| Bazel | Existing smoke build and documented UAT command form | yes | 8.6.0 | If Bazel unavailable, Cargo command forms still document operator CLI, but full `scripts/verify.sh` remains blocked. [VERIFIED: `bazel --version`; scripts/verify.sh] |
| cargo-llvm-cov | Full default verification coverage gate | yes | 0.8.5 | No fallback in `scripts/verify.sh`; install if missing. [VERIFIED: `cargo llvm-cov --version`; scripts/verify.sh] |
| git | Source breadcrumb checker | yes | 2.53.0 | No fallback for `scripts/check-parity-breadcrumbs.ts --check`. [VERIFIED: `git --version`; scripts/check-parity-breadcrumbs.ts] |
| bash | `scripts/verify.sh` | yes | 3.2.57 | No fallback; verify script is Bash. [VERIFIED: `bash --version`; scripts/verify.sh] |
| grep | Coverage scan in verify | yes | BSD grep 2.6.0 compatible | No fallback in `scripts/verify.sh`. [VERIFIED: `grep --version`; scripts/verify.sh] |

**Missing dependencies with no fallback:** None found in the local environment. [VERIFIED: local command probes]

**Missing dependencies with fallback:** None found in the local environment. [VERIFIED: local command probes]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---|---:|---|
| V2 Authentication | no | Phase 80 adds no auth surface; preserve no production-funds wallet, remote admin, or hosted upload claim. [VERIFIED: 80-CONTEXT.md; docs/parity/threat-model-v1.6.md] |
| V3 Session Management | no | No session or browser surface is involved. [VERIFIED: 80-CONTEXT.md; .planning/ROADMAP.md] |
| V4 Access Control | no | No new privileged service-manager behavior should enter default verification. [VERIFIED: 80-CONTEXT.md; scripts/check-phase79-diagnostics-support-bundle.ts] |
| V5 Input Validation | yes | Parse structured parity JSON where structure matters, and keep docs checks as exact anchored strings. [VERIFIED: scripts/check-v1.6-release-boundaries.ts; scripts/check-phase79-diagnostics-support-bundle.ts] |
| V6 Cryptography | limited | Do not treat `sha256-json-v1` checkpoint chain as authenticity, signing, or an external trust root. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/forensics.rs; docs/operator/runtime-guide.md] |
| Logging and error privacy | yes | Preserve redaction boundaries for credentials, wallet material, raw logs, raw live-smoke input, raw options, and endpoint tables. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs; docs/operator/runtime-guide.md; scripts/check-phase79-diagnostics-support-bundle.ts] |

### Known Threat Patterns for Phase 80

| Pattern | STRIDE | Standard Mitigation |
|---|---|---|
| Evidence inflation: artifact existence treated as proof | Spoofing / Repudiation | UAT matrix must separate "Evidence proves" from "Does not prove", and checker must anchor typed evidence wording. [VERIFIED: 80-CONTEXT.md; docs/operator/runtime-guide.md] |
| Default verifier drift to public-network or service-manager work | Repudiation / Denial of Service | Phase 80 checker rejects forbidden `scripts/verify.sh` strings and verifies ordering after Phase 79. [VERIFIED: 80-CONTEXT.md; scripts/check-phase79-diagnostics-support-bundle.ts] |
| Support bundle data leakage | Information Disclosure | Guard typed redaction strings and Phase 79 forbidden output strings. [VERIFIED: scripts/check-phase79-diagnostics-support-bundle.ts; packages/open-bitcoin-cli/src/operator/support.rs] |
| Checkpoint chain overclaim | Spoofing / Repudiation | Docs and checker should state checkpoint-chain evidence is ordering/truncation evidence only. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/forensics.rs; docs/operator/runtime-guide.md] |
| Broad production-node claim leakage | Elevation of Privilege / Repudiation | Preserve v1.7 non-claim list across README, runtime guide, parity roots, checker constants, and claim-bearing status wording. [VERIFIED: 80-CONTEXT.md; docs/parity/threat-model-v1.6.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-CONTEXT.md` - locked Phase 80 decisions, discretion, deferred scope, canonical references. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - VER-05, VER-06, VER-07, REL-04 and v1.7 out-of-scope list. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 80 goal, success criteria, dependency on Phase 79, active v1.7 scope. [VERIFIED: file read]
- `.planning/STATE.md` - Phase 79 complete, Phase 80 pending, accumulated v1.7 decisions. [VERIFIED: file read]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/` pages - repo and Bright Builds rules. [VERIFIED: file read]
- `scripts/verify.sh` - aggregate verification contract and current checker order. [VERIFIED: file read]
- `scripts/check-phase79-diagnostics-support-bundle.ts` and `.test.ts` - closest Phase 80 checker/test pattern. [VERIFIED: file read]
- `scripts/check-v1.6-release-boundaries.ts` - release-boundary checker and structured parity index pattern. [VERIFIED: file read]
- `docs/operator/runtime-guide.md`, `README.md`, `docs/parity/release-readiness.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`, `docs/parity/deviations-and-unknowns.md`, `docs/parity/catalog/operator-runtime-release-hardening.md` - target docs and parity roots. [VERIFIED: file read / rg]
- `packages/open-bitcoin-cli/src/operator/support.rs`, `support/forensics.rs`, `soak/report.rs`, `soak/ledger.rs` - support bundle and soak schema anchors. [VERIFIED: file read]
- `.planning/config.json` - `workflow.nyquist_validation` is `false`, so Validation Architecture section is skipped. [VERIFIED: file read]

### Secondary (MEDIUM confidence)

- `.planning/phases/75-79/*-VERIFICATION.md` - prior phase passed evidence and residual risks. [VERIFIED: file read for Phase 75, 76, 78, 79; Phase 77 inferred from roadmap/checker roots]
- Local tool probes for Bun, Cargo, Rustc, Bazel, git, grep, bash, and cargo-llvm-cov versions. [VERIFIED: command output]

### Tertiary (LOW confidence)

- None. No web-only or unverified ecosystem claims were used. [VERIFIED: research scope]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - all stack items are repo-pinned or locally probed, and Phase 80 adds no new dependencies. [VERIFIED: .bun-version; rust-toolchain.toml; MODULE.bazel; local probes]
- Architecture: HIGH - Phase 80 decisions exactly match existing Phase 73 and Phase 75-79 checker/docs/parity patterns. [VERIFIED: 80-CONTEXT.md; scripts/check-phase79-diagnostics-support-bundle.ts; docs/operator/runtime-guide.md]
- Pitfalls: HIGH - pitfalls are drawn from locked decisions, existing checker forbidden lists, and prior threat/release docs. [VERIFIED: 80-CONTEXT.md; scripts/check-phase79-diagnostics-support-bundle.ts; docs/parity/threat-model-v1.6.md]

**Research date:** 2026-06-17  
**Valid until:** 2026-07-17 or until Phase 80 implementation changes the target docs/checker surfaces. [ASSUMED]
