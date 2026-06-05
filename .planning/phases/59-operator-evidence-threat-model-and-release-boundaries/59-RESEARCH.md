# Phase 59: Operator Evidence, Threat Model, and Release Boundaries - Research

**Researched:** 2026-06-05 [VERIFIED: system current_date]
**Domain:** Open Bitcoin operator evidence, support-bundle redaction, observability consistency, threat modeling, and release-boundary documentation [VERIFIED: .planning/ROADMAP.md + .planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: codebase grep + local command probes + official Bright Builds and OWASP sources]

<user_constraints>

## User Constraints (from CONTEXT.md)

The locked decisions, discretion areas, and deferred ideas below are copied from `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md`. [VERIFIED: .planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md]

### Locked Decisions

## Implementation Decisions

### Cross-Surface Operator Truth

- **D-01:** Treat the shared status snapshot and durable sync metadata as the
  source of truth for Phase 59 consistency checks. Status, dashboard, metrics,
  structured logs, RPC-facing blockchain info, support evidence, and live-smoke
  reports should agree on header height, downloaded block height, connected
  block height, compatibility state, progress signal, and latest error rather
  than each renderer inventing local summaries.
- **D-02:** Close OBS-01 with deterministic assertions over existing projection
  and rendering paths. Prefer focused tests and script checks that compare the
  same fixture data across renderers over broad runtime rewrites.
- **D-03:** Preserve the Phase 56-58 distinction between header progress,
  downloaded block progress, connected block progress, restart/resume evidence,
  and diagnosed blockers. Do not collapse these into a single "synced" flag or
  timing-threshold claim.

### Support Evidence Packet

- **D-04:** Extend redacted support evidence to summarize the v1.4 live-smoke
  schema v2 result fields that reviewers need: status, progress detection,
  first header progress, first block progress, restart/resume evidence,
  recovery diagnosis, selected peer outcome summaries, status snapshot
  summaries, config paths, metrics/log availability, and store health.
- **D-05:** Keep support ingestion allowlist-based and redacted. Raw live-smoke
  input, raw status snapshot arrays, daemon stdout/stderr tails, raw endpoint
  tables, manual peer lists, cookie values, wallet material, unbounded logs, and
  secrets must not be embedded in support bundles.
- **D-06:** Missing live-smoke, metrics, logs, RPC, or store evidence should
  render as unavailable with a reason. Missing optional evidence is diagnostic
  context, not a reason to hide the surface.
- **D-07:** Support bundles remain local review/troubleshooting evidence. They
  are not release validators and do not prove public-mainnet sync success by
  themselves.

### Operator Docs And UAT Commands

- **D-08:** Update operator docs with copy-pasteable repo-local commands for
  deterministic verification, manual-peer live smoke, same-datadir restart and
  resume review, support evidence collection, and pass/fail interpretation.
  Commands should prefer the repo-local Cargo and Bazel forms already required
  by repo guidance.
- **D-09:** Keep generated live-smoke reports, support bundles, daemon logs,
  metrics stores, and local datadirs out of git. Docs may reference local output
  paths and report field names, but should not check in environment-specific
  public-network artifacts.
- **D-10:** Make pass/fail copy evidence-first: explicit field names, accepted
  diagnosed-blocker paths, and next operator action. Avoid implying success
  from peer reachability, elapsed time, support-bundle existence, or daemon
  startup alone.

### Threat Model And Release Boundaries

- **D-11:** Add or refresh a reviewer-facing v1.4 threat model covering public
  peer compatibility handling, header and block input, resource bounds,
  restart/resume evidence, report redaction, support evidence, and
  operator-facing live evidence.
- **D-12:** Refresh parity roots, release-readiness docs, and checklist entries
  so reviewers can distinguish the v1.4 opt-in outbound IBD progress claim from
  deferred inbound serving, transaction relay, production wallet use, migration
  apply mode, packaging, hosted dashboard, GUI, and unattended production-node
  claims.
- **D-13:** Keep v1.3 docs as historical evidence. Add v1.4-specific surfaces or
  clearly labeled v1.4 sections instead of rewriting v1.3 claims as if they
  were the current milestone.

### Verification Posture

- **D-14:** Default verification remains deterministic. `bash scripts/verify.sh`
  must not invoke public-network live smoke or `--restart-after-progress`.
- **D-15:** Add or update repo-owned deterministic checks for the v1.4 release
  boundary when parity roots or threat/release docs change. Reuse the existing
  v1.3 release-boundary checker pattern if it remains the smallest robust path.
- **D-16:** Phase verification should include the repo-native aggregate
  `bash scripts/verify.sh`, targeted support/live-smoke/doc checks, and any
  relevant Rust or Bun fixtures introduced by the plans.

### the agent's Discretion

- The planner may split Phase 59 into code, docs/parity, and verification
  plans if that keeps files and risk isolated.
- The planner may decide whether v1.4 threat-model content lives in a new
  `docs/parity/threat-model-v1.4.md` file or in a clearly separated v1.4
  section, provided parity roots link to it and v1.3 historical evidence remains
  intact.
- The executor may reuse existing support bundle, release-readiness, and
  live-smoke fixtures when they already prove the v1.4 behavior, but summaries
  and verification must make the Phase 59 evidence explicit.

### Deferred Ideas (OUT OF SCOPE)

## Deferred Ideas

- Inbound peer serving, transaction relay, production-funds wallet use,
  migration apply mode, packaging, hosted dashboard, GUI work, Windows service
  support, and unattended production-node operation remain future milestones.
- Hosted support upload or support-bundle artifact validation remains future
  work; Phase 59 keeps support evidence local and redacted.
- Any public-network CI or default verification gate remains out of scope for
  v1.4.

</user_constraints>

<phase_requirements>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| OBS-01 | Operator-facing status, dashboard, metrics, structured logs, RPC-facing blockchain info, and live-smoke snapshots agree on current header height, block height, peer compatibility state, progress signal, and latest error. [VERIFIED: .planning/REQUIREMENTS.md] | Use `OpenBitcoinStatusSnapshot`, `DurableSyncState`, status renderers, dashboard projection, `SyncRunSummary`, and `scripts/run-live-mainnet-smoke.ts` status snapshots as one fixture-driven consistency surface. [VERIFIED: packages/open-bitcoin-node/src/status.rs + packages/open-bitcoin-cli/src/operator/status/render.rs + packages/open-bitcoin-cli/src/operator/dashboard/model.rs + packages/open-bitcoin-node/src/sync/types/summary.rs + scripts/run-live-mainnet-smoke.ts] |
| OBS-02 | Operator can generate a redacted v1.4 support bundle or equivalent evidence packet that summarizes diagnostics and health without raw sensitive data. [VERIFIED: .planning/REQUIREMENTS.md] | Extend the existing allowlisted support summary and Markdown renderer; tests already prove schema v2 summary redaction and missing-report unavailable behavior. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs + packages/open-bitcoin-cli/src/operator/support/render.rs + packages/open-bitcoin-cli/tests/operator_binary.rs] |
| OBS-03 | Operator docs provide copy-pasteable repo-local Cargo and Bazel commands for deterministic checks, manual-peer live smoke, restart/resume review, support evidence, and pass/fail interpretation. [VERIFIED: .planning/REQUIREMENTS.md] | Update `docs/operator/runtime-guide.md` and `docs/parity/release-readiness.md`, preserving the repo-local Cargo/Bazel command pattern from AGENTS guidance and lessons. [VERIFIED: AGENTS.md + .codex/tasks/lessons.md + docs/operator/runtime-guide.md + docs/parity/release-readiness.md] |
| SEC-01 | Reviewer can inspect a v1.4 threat-model update covering public peer compatibility handling, header/block input, resource bounds, restart/resume evidence, report redaction, and operator-facing live evidence. [VERIFIED: .planning/REQUIREMENTS.md] | Adapt the existing STRIDE-style `docs/parity/threat-model-v1.3.md` pattern to a v1.4-specific surface and map it to OWASP ASVS v5.0.0 categories where applicable. [VERIFIED: docs/parity/threat-model-v1.3.md; CITED: https://owasp.org/www-project-application-security-verification-standard/] |
| SEC-02 | Reviewer can inspect refreshed parity and release-readiness docs that distinguish the v1.4 opt-in outbound IBD progress claim from deferred surfaces. [VERIFIED: .planning/REQUIREMENTS.md] | Add a v1.4 parity/checklist/index/release-readiness root that keeps v1.3 historical and names future gates for inbound serving, relay, wallet production use, migration apply, packaging, hosted dashboard, GUI, and unattended production operation. [VERIFIED: docs/parity/index.json + docs/parity/checklist.md + docs/parity/release-readiness.md + .planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md] |
| SEC-03 | Default repo verification remains deterministic; public-network checks stay opt-in and are documented as UAT evidence rather than part of `bash scripts/verify.sh`. [VERIFIED: .planning/REQUIREMENTS.md] | Reuse the v1.3 release-boundary checker pattern and add an assertion that `scripts/verify.sh` does not contain `run-live-mainnet-smoke` or `--restart-after-progress`. [VERIFIED: scripts/check-v1.3-release-boundaries.ts + scripts/verify.sh + .planning/phases/58-same-datadir-restart-and-resume-evidence/58-VERIFICATION.md] |

</phase_requirements>

## Project Constraints (from AGENTS.md)

- Read repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant pinned Bright Builds canonical standards before planning or implementation. [VERIFIED: AGENTS.md + AGENTS.bright-builds.md; CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- No checked-in local `standards/` directory exists in this checkout; the sidecar points to the pinned upstream standards instead. [VERIFIED: `find standards -maxdepth 3 -type f -print`]
- Follow functional-core / imperative-shell boundaries; pure Bitcoin domain logic stays separate from filesystem, network, terminal, RPC, durable-store, and process effects. [VERIFIED: AGENTS.md + .planning/ARCHITECTURE.md; CITED: Bright Builds architecture standard]
- Use `bash scripts/verify.sh` as the repo-native verification contract, including Bazel smoke build, before marking substantive work complete. [VERIFIED: AGENTS.md + scripts/verify.sh]
- Keep public-network live-smoke checks opt-in and outside default `bash scripts/verify.sh`. [VERIFIED: AGENTS.md + .planning/ROADMAP.md + scripts/verify.sh]
- Use repo-local Cargo and Bazel command forms in UAT/operator docs; do not rely only on an installed `open-bitcoin` alias. [VERIFIED: AGENTS.md + .codex/tasks/lessons.md]
- Keep generated live-smoke reports, support bundles, daemon logs, metrics stores, datadirs, and local public-network artifacts out of git. [VERIFIED: 59-CONTEXT.md + docs/operator/runtime-guide.md]
- Preserve auditable Bitcoin Knots `29.3.knots20260210` parity claims and record intentional behavior differences in `docs/parity/index.json` and companion docs. [VERIFIED: AGENTS.md + .planning/PROJECT.md + docs/parity/index.json]
- When adding first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update parity breadcrumbs unless an explicit `none` breadcrumb is defensible. [VERIFIED: AGENTS.md + scripts/check-parity-breadcrumbs.ts]
- Rust work must use `cargo fmt`, `cargo clippy`, build, tests, and the repo-native aggregate path as applicable; Rust 2024 edition and Rust `1.94.1` are pinned. [VERIFIED: AGENTS.md + rust-toolchain.toml + packages/Cargo.toml]
- Bun is the canonical runtime for repo-owned TypeScript automation; this repo has no `package.json`, so do not add `bun install` setup instructions. [VERIFIED: AGENTS.md + .planning/STACK.md + `.bun-version`]
- Project skills directories `.claude/skills/` and `.agents/skills/` are absent or contain no `SKILL.md` files for this repo. [VERIFIED: `find .claude/skills .agents/skills -maxdepth 2 -name SKILL.md -print`]

## Summary

Phase 59 should be planned as a closeout and consistency phase, not as a new sync-capability phase. [VERIFIED: .planning/ROADMAP.md + 59-CONTEXT.md] The correct implementation path is to reuse existing truth sources: `OpenBitcoinStatusSnapshot`, durable sync metadata, `SyncRunSummary`, current live-smoke schema v2 report fields, and the support-bundle allowlist/redaction model. [VERIFIED: packages/open-bitcoin-node/src/status.rs + packages/open-bitcoin-node/src/sync/types/summary.rs + scripts/run-live-mainnet-smoke.ts + packages/open-bitcoin-cli/src/operator/support.rs]

The main implementation gap is that the support bundle currently allowlists only six schema v2 `result.*` fields: `status`, `progressDetected`, `maybeNoProgressCause`, `nextAction`, `headerDelta`, and `blockDelta`. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] Phase 59 needs to add compact, redacted summaries for v1.4 first-header, first-block, restart/resume, recovery diagnosis, selected peer outcome, status snapshot, metrics/log, config, and store-health evidence while continuing to exclude raw snapshots, endpoint tables, daemon stdout/stderr, manual peer lists, wallet material, cookie values, and secrets. [VERIFIED: 59-CONTEXT.md + packages/open-bitcoin-cli/tests/operator_binary.rs + docs/operator/runtime-guide.md]

The final release-boundary work should adapt the v1.3 deterministic checker into a v1.4 checker instead of relying on reviewer memory. [VERIFIED: scripts/check-v1.3-release-boundaries.ts + 59-CONTEXT.md] The checker should prove that parity roots, release-readiness docs, the v1.4 threat model, and deferred-surface wording are present, and that `scripts/verify.sh` still avoids public-network live-smoke invocation. [VERIFIED: scripts/check-v1.3-release-boundaries.ts + scripts/verify.sh]

**Primary recommendation:** Split Phase 59 into three plans: support/evidence projection, docs/parity/threat model, and deterministic release-boundary verification. [VERIFIED: 59-CONTEXT.md + codebase grep]

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust toolchain | `1.94.1` | Compile and test first-party Rust crates. | Pinned in `rust-toolchain.toml` and verified locally with `rustc --version`. [VERIFIED: rust-toolchain.toml + `rustc --version`] |
| Rust edition | `2024` | Workspace crate edition. | Defined in `packages/Cargo.toml`; new Rust work should follow this edition. [VERIFIED: packages/Cargo.toml] |
| Cargo workspace | `0.1.0` package version | First-party crates under `packages/open-bitcoin-*`. | Existing workspace boundary for node, RPC, CLI, network, and support code. [VERIFIED: packages/Cargo.toml] |
| Bun | `1.3.9` | Run repo-owned TypeScript scripts and deterministic live-smoke/release-boundary checks. | Pinned by `.bun-version` and verified locally with `bun --version`. [VERIFIED: .bun-version + `bun --version`] |
| Bazel / Bzlmod | Bazel `8.6.0`, `rules_rust` `0.69.0` | Top-level smoke build and Rust dependency import via Bzlmod. | Defined in `MODULE.bazel`; verified locally with `bazel --version`. [VERIFIED: MODULE.bazel + `bazel --version`] |
| `OpenBitcoinStatusSnapshot` | first-party | Shared status contract for CLI status, dashboard, support bundles, JSON automation, and stopped-node inspection. | Existing docs call it the sole shared status model, and code serializes OBS-01 fields. [VERIFIED: docs/architecture/status-snapshot.md + packages/open-bitcoin-node/src/status.rs] |
| `scripts/run-live-mainnet-smoke.ts` | first-party Bun script | Opt-in live-mainnet report generation with schema v2 `result.*`, status snapshots, first-progress evidence, and restart evidence. | Existing Phase 56-58 evidence and docs use it as the live evidence entrypoint. [VERIFIED: scripts/run-live-mainnet-smoke.ts + docs/operator/runtime-guide.md + 58-VERIFICATION.md] |
| `packages/open-bitcoin-cli/src/operator/support.rs` | first-party Rust | Local redacted support evidence bundle generation. | Existing code writes `support-evidence.json` and `support-evidence.md` with allowlisted live-smoke summaries. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde` / `serde_json` | `serde 1.0.228`, `serde_json 1.0.149` | Stable JSON shapes for status, support evidence, RPC, and reports. | Reuse for support-bundle model and tests. [VERIFIED: packages/Cargo.lock + packages/open-bitcoin-cli/Cargo.toml] |
| Fjall | `3.1.4` | Durable store and metrics/status metadata. | Use only through existing node-store adapters; do not add new direct shell effects in pure crates. [VERIFIED: packages/Cargo.lock + packages/open-bitcoin-node/Cargo.toml + .planning/ARCHITECTURE.md] |
| clap | `4.6.1` | Operator CLI argument surface. | Reuse only if Phase 59 changes support command flags; current context does not require a new flag. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml + 59-CONTEXT.md] |
| Ratatui / Crossterm | `ratatui 0.30.0`, `crossterm 0.29.0` | Terminal dashboard rendering. | Use existing dashboard projection tests for OBS-01; do not rewrite terminal runtime. [VERIFIED: packages/Cargo.lock + packages/open-bitcoin-cli/src/operator/dashboard/model.rs] |
| `scripts/test-run-live-mainnet-smoke.sh` | first-party Bash | Deterministic live-smoke fixture regression suite. | Use for targeted verification when live-smoke schema or docs change. [VERIFIED: scripts/test-run-live-mainnet-smoke.sh] |
| `scripts/check-v1.3-release-boundaries.ts` pattern | first-party Bun script | Deterministic docs/parity/release-boundary assertions. | Clone/adapt to `check-v1.4-release-boundaries.ts` for SEC-01 through SEC-03. [VERIFIED: scripts/check-v1.3-release-boundaries.ts + 59-CONTEXT.md] |
| `bash scripts/verify.sh` | repo-owned aggregate | Full deterministic verification contract. | Phase final verification should include it; it already invokes policy scripts, Rust checks, benchmark smoke, Bazel smoke, and v1.3 release-boundary check. [VERIFIED: scripts/verify.sh] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Extending `OpenBitcoinStatusSnapshot`/support projections | A new final-evidence runtime service | Rejected by context: Phase 59 should not add a new sync capability or aggregate runtime. [VERIFIED: 59-CONTEXT.md] |
| Allowlisted live-smoke summary extraction | Embed raw live-smoke reports in support bundles | Rejected by context and tests because raw snapshots, endpoint tables, stdout/stderr, options, peer lists, and secrets must remain excluded. [VERIFIED: 59-CONTEXT.md + packages/open-bitcoin-cli/tests/operator_binary.rs] |
| New release-boundary validation framework | Adapt v1.3 checker | The v1.3 checker already validates parity roots, docs, required wording, and `verify.sh` public-network exclusion. [VERIFIED: scripts/check-v1.3-release-boundaries.ts] |
| Live network CI gate | Opt-in UAT command documentation | Rejected by SEC-03 and repo policy; public-network checks must stay outside `bash scripts/verify.sh`. [VERIFIED: .planning/REQUIREMENTS.md + scripts/verify.sh] |

**Installation:**

```bash
# No new dependencies are recommended for Phase 59.
git submodule update --init --recursive
bash scripts/verify.sh
```

The submodule command is repo guidance for materializing the pinned Knots baseline, and the verify command is the repo-native local verification contract. [VERIFIED: AGENTS.md]

**Version verification:** This research verified the local toolchain with `rustc --version`, `cargo --version`, `cargo llvm-cov --version`, `bun --version`, `bazel --version`, `git --version`, `bash --version`, and `rg --version`. [VERIFIED: local command probes]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-cli/src/operator/
├── support.rs                 # support bundle data model, allowlist, redaction, store/live evidence collection [VERIFIED: codebase]
├── support/render.rs          # support Markdown and command-output rendering [VERIFIED: codebase]
├── status/render.rs           # human/JSON status projection from shared snapshot [VERIFIED: codebase]
└── dashboard/model.rs         # pure dashboard projection from shared snapshot [VERIFIED: codebase]
scripts/
├── run-live-mainnet-smoke.ts   # opt-in live evidence schema v2 and restart/resume report fields [VERIFIED: codebase]
├── test-run-live-mainnet-smoke.sh # deterministic live-smoke fixture checks [VERIFIED: codebase]
└── check-v1.4-release-boundaries.ts # recommended new deterministic closeout checker [VERIFIED: inferred from scripts/check-v1.3-release-boundaries.ts + 59-CONTEXT.md]
docs/parity/
├── threat-model-v1.4.md        # recommended v1.4 threat model preserving v1.3 history [VERIFIED: inferred from 59-CONTEXT.md]
├── release-readiness.md        # v1.4 evidence matrix and scoped claim boundary [VERIFIED: existing v1.3 pattern]
├── checklist.md                # human-readable parity surface root [VERIFIED: existing v1.3 pattern]
└── index.json                  # machine-readable parity root and audit entries [VERIFIED: existing v1.3 pattern]
```

### Pattern 1: Shared Snapshot As Truth Source

**What:** Status, dashboard, support evidence, metrics/logs, RPC-facing blockchain info, and live-smoke snapshots should compare values projected from `OpenBitcoinStatusSnapshot` and durable sync metadata rather than recomputing renderer-local truth. [VERIFIED: 59-CONTEXT.md + docs/architecture/status-snapshot.md]

**When to use:** Use this for OBS-01 consistency checks and any test fixture that asserts header height, downloaded block height, connected block height, progress signal, latest error, peer state, and unavailable reasons. [VERIFIED: packages/open-bitcoin-node/src/status.rs + packages/open-bitcoin-cli/src/operator/status/render.rs + packages/open-bitcoin-cli/src/operator/dashboard/model.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/status.rs and status/dashboard renderers.
// Arrange
let snapshot = test_snapshot_with_sync_progress();

// Act
let status_json = serde_json::to_value(&snapshot)?;
let dashboard = DashboardState::from_snapshot(&snapshot);

// Assert
assert_eq!(status_json["sync"]["sync_progress"]["value"]["header_height"], 840_001);
assert!(dashboard_sections_include(&dashboard, "downloaded_blocks=840000"));
assert!(dashboard_sections_include(&dashboard, "connected_blocks=840000"));
```

### Pattern 2: Allowlisted Support Evidence

**What:** Support bundles should copy selected, compact report fields and recursively redact sensitive strings inside those selected values. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs]

**When to use:** Use this for OBS-02 v1.4 summary extraction from `result.firstHeaderProgress`, `result.firstBlockProgress`, and `result.restartResumeEvidence`, plus compact peer/status/config/metrics/store summaries. [VERIFIED: scripts/run-live-mainnet-smoke.ts + 59-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-cli/src/operator/support.rs
const LIVE_SMOKE_RESULT_SUMMARY_KEYS: &[&str] = &[
    "status",
    "progressDetected",
    "maybeNoProgressCause",
    "nextAction",
    "headerDelta",
    "blockDelta",
    "firstHeaderProgress",
    "firstBlockProgress",
    "restartResumeEvidence",
];
```

The actual plan should keep this allowlist compact and should not insert raw `snapshots`, `network_preflight.endpoint_outcomes`, `daemon.stdoutTail`, `daemon.stderrTail`, or `options.manualPeers`. [VERIFIED: 59-CONTEXT.md + packages/open-bitcoin-cli/tests/operator_binary.rs + scripts/test-run-live-mainnet-smoke.sh]

### Pattern 3: Deterministic Release-Boundary Checker

**What:** Use a repo-owned Bun script that parses `docs/parity/index.json`, checks required docs/strings, and asserts that `scripts/verify.sh` does not invoke public-network live-smoke commands. [VERIFIED: scripts/check-v1.3-release-boundaries.ts]

**When to use:** Use this for SEC-01, SEC-02, and SEC-03 after adding v1.4 threat and release-readiness roots. [VERIFIED: 59-CONTEXT.md]

**Example:**

```ts
// Source: scripts/check-v1.3-release-boundaries.ts
requireContains(verifyScript, "bun run scripts/check-v1.4-release-boundaries.ts", "scripts/verify.sh");
requireNotContains(verifyScript, "run-live-mainnet-smoke", "scripts/verify.sh");
requireNotContains(verifyScript, "--restart-after-progress", "scripts/verify.sh");
```

### Pattern 4: Historical Docs Stay Historical

**What:** Add v1.4-specific sections or files instead of rewriting v1.3 claims as current v1.4 claims. [VERIFIED: 59-CONTEXT.md]

**When to use:** Use this when updating `docs/parity/threat-model-v1.3.md`, `docs/parity/release-readiness.md`, `docs/parity/checklist.md`, `docs/parity/index.json`, and `docs/parity/deviations-and-unknowns.md`. [VERIFIED: 59-CONTEXT.md + docs/parity/index.json + docs/parity/release-readiness.md]

### Anti-Patterns to Avoid

- **Single `synced` flag:** It collapses header progress, downloaded-only block progress, connected block progress, and restart/resume proof into one misleading claim. [VERIFIED: 59-CONTEXT.md + docs/architecture/status-snapshot.md]
- **Raw support report embedding:** It risks leaking secrets and contradicts the existing allowlist boundary. [VERIFIED: packages/open-bitcoin-cli/tests/operator_binary.rs + docs/operator/runtime-guide.md]
- **Public-network verification in `scripts/verify.sh`:** It violates SEC-03 and the milestone verification boundary. [VERIFIED: .planning/REQUIREMENTS.md + scripts/verify.sh]
- **Rewriting v1.3 closeout docs as v1.4 docs:** It destroys historical evidence provenance and contradicts D-13. [VERIFIED: 59-CONTEXT.md]
- **Support bundle as release validator:** Existing docs and context say support bundles are troubleshooting evidence, not proof of public-mainnet success. [VERIFIED: 59-CONTEXT.md + docs/operator/runtime-guide.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Cross-surface status truth | New final-evidence state model | `OpenBitcoinStatusSnapshot` and `DurableSyncState` | Existing code and docs already define the shared status contract for CLI, dashboard, support, and automation. [VERIFIED: docs/architecture/status-snapshot.md + packages/open-bitcoin-node/src/status.rs] |
| Support redaction | Free-form regex sanitizer over raw full reports | Existing allowlist plus `sanitize_json_value`/`redact_sensitive_text` pattern | The current tests prove raw daemon tails, snapshots, endpoint details, manual peers, and secrets are excluded. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs + packages/open-bitcoin-cli/tests/operator_binary.rs] |
| Release-boundary audit | Manual reviewer checklist only | Bun checker modeled on `scripts/check-v1.3-release-boundaries.ts` | The existing checker gives deterministic parity-root and `verify.sh` guard coverage. [VERIFIED: scripts/check-v1.3-release-boundaries.ts] |
| Public network proof in CI | Default `verify.sh` live smoke | Opt-in `bun run scripts/run-live-mainnet-smoke.ts ...` UAT docs | Public-network checks are explicitly out of default verification scope. [VERIFIED: .planning/REQUIREMENTS.md + docs/operator/runtime-guide.md] |
| Threat taxonomy from scratch | Ad hoc security headings only | Existing STRIDE register pattern plus ASVS v5.0.0 references where applicable | v1.3 threat model already uses STRIDE and OWASP ASVS is current at 5.0.0. [VERIFIED: docs/parity/threat-model-v1.3.md; CITED: https://owasp.org/www-project-application-security-verification-standard/] |

**Key insight:** The difficult Phase 59 work is not data collection; it is preserving trust boundaries while summarizing existing evidence. [VERIFIED: 59-CONTEXT.md + packages/open-bitcoin-cli/src/operator/support.rs + docs/parity/threat-model-v1.3.md]

## Common Pitfalls

### Pitfall 1: Cross-Surface Drift

**What goes wrong:** Status, dashboard, logs, metrics, RPC-facing info, support evidence, and live-smoke reports display different heights or progress signals from the same fixture. [VERIFIED: OBS-01 in .planning/REQUIREMENTS.md]

**Why it happens:** Renderers independently format local summaries instead of asserting against shared snapshot fields. [VERIFIED: 59-CONTEXT.md]

**How to avoid:** Add deterministic tests that build one snapshot/report fixture and compare status JSON, human status, dashboard rows, support JSON/Markdown, metrics/log samples, and live-smoke summary fields. [VERIFIED: packages/open-bitcoin-node/src/status.rs + packages/open-bitcoin-cli/src/operator/status/render.rs + packages/open-bitcoin-cli/src/operator/dashboard/model.rs]

**Warning signs:** New code computes `header_height`, `downloaded_block_height`, `connected_block_height`, `progress_signal`, or latest error from separate renderer-specific logic. [VERIFIED: docs/architecture/status-snapshot.md]

### Pitfall 2: Redaction Boundary Regression

**What goes wrong:** A support bundle includes raw daemon tails, endpoint tables, manual peers, status snapshots, cookie values, RPC passwords, wallet material, or unbounded logs. [VERIFIED: 59-CONTEXT.md + packages/open-bitcoin-cli/tests/operator_binary.rs]

**Why it happens:** A planner asks for "more evidence" and the executor copies raw nested live-smoke objects instead of compact summaries. [VERIFIED: 59-CONTEXT.md]

**How to avoid:** Extend the allowlist with purpose-built compact structs and add forbidden-field assertions for `stdoutTail`, `stderrTail`, `endpoint_outcomes`, `snapshots`, `manualPeers`, cookie values, wallet material, and secret-like strings. [VERIFIED: packages/open-bitcoin-cli/tests/operator_binary.rs + scripts/test-run-live-mainnet-smoke.sh]

**Warning signs:** `support-evidence.json` contains `daemon`, `options`, `network_preflight`, `snapshots`, or raw `manualPeers` keys. [VERIFIED: scripts/run-live-mainnet-smoke.ts report schema + support tests]

### Pitfall 3: Claim Boundary Inflation

**What goes wrong:** Release docs imply unattended production-node operation, inbound serving, transaction relay, production-funds wallet readiness, migration apply mode, packaging, hosted dashboard, GUI work, or public-network CI. [VERIFIED: .planning/REQUIREMENTS.md + 59-CONTEXT.md]

**Why it happens:** v1.4 outbound IBD evidence is described with broad "mainnet node" wording. [VERIFIED: .planning/ROADMAP.md]

**How to avoid:** Use an explicit v1.4 release matrix with columns for proven claim, accepted evidence, explicit non-claim, future gate, and requirement IDs. [VERIFIED: docs/parity/release-readiness.md existing v1.3 pattern]

**Warning signs:** Docs say "production ready", "full node", "unattended", "inbound", "relay", or "wallet production funds" without a future-gate qualifier. [VERIFIED: docs/parity/threat-model-v1.3.md + docs/parity/release-readiness.md]

### Pitfall 4: Live Network Enters Default Verification

**What goes wrong:** `bash scripts/verify.sh` starts calling `run-live-mainnet-smoke`, `--manual-peer`, or `--restart-after-progress`. [VERIFIED: SEC-03 in .planning/REQUIREMENTS.md]

**Why it happens:** Release-boundary checks confuse deterministic fixture coverage with opt-in UAT. [VERIFIED: .planning/ROADMAP.md]

**How to avoid:** Add a v1.4 checker assertion that `scripts/verify.sh` contains the v1.4 boundary checker but does not contain public-network live-smoke invocations. [VERIFIED: scripts/check-v1.3-release-boundaries.ts]

**Warning signs:** `rg -n "run-live-mainnet-smoke|--restart-after-progress" scripts/verify.sh` returns matches. [VERIFIED: 58-VERIFICATION.md]

### Pitfall 5: Missing Optional Evidence Hidden Instead Of Marked Unavailable

**What goes wrong:** Missing live-smoke, logs, metrics, RPC, or store evidence disappears from output. [VERIFIED: D-06 in 59-CONTEXT.md]

**Why it happens:** Renderers skip absent fields rather than preserving `Unavailable` reasons. [VERIFIED: docs/architecture/status-snapshot.md]

**How to avoid:** Reuse `FieldAvailability<T>` and `EvidenceAvailability`, and assert the unavailable state plus reason in JSON and Markdown. [VERIFIED: packages/open-bitcoin-node/src/status.rs + packages/open-bitcoin-cli/src/operator/support.rs]

**Warning signs:** Tests assert only successful evidence fields and never assert unavailable reason strings. [VERIFIED: packages/open-bitcoin-cli/tests/operator_binary.rs]

## Code Examples

Verified patterns from local sources:

### Shared Status Serialization

```rust
// Source: packages/open-bitcoin-node/src/status.rs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenBitcoinStatusSnapshot {
    pub node: NodeStatus,
    pub config: ConfigStatus,
    pub service: ServiceStatus,
    pub sync: SyncStatus,
    pub peers: PeerStatus,
    pub mempool: MempoolStatus,
    pub wallet: WalletStatus,
    pub logs: LogStatus,
    pub metrics: MetricsStatus,
    pub health_signals: Vec<HealthSignal>,
    pub build: BuildProvenance,
}
```

This is the correct shared surface for OBS-01 consistency tests. [VERIFIED: packages/open-bitcoin-node/src/status.rs]

### Support Summary Allowlist

```rust
// Source: packages/open-bitcoin-cli/src/operator/support.rs
fn live_smoke_summary_from_result(maybe_result: Option<&Value>) -> Option<Value> {
    let result = maybe_result?.as_object()?;
    let mut summary = Map::new();
    for key in LIVE_SMOKE_RESULT_SUMMARY_KEYS {
        if let Some(item) = result.get(*key) {
            summary.insert((*key).to_string(), sanitize_json_value(item));
        }
    }
    if summary.is_empty() {
        return None;
    }

    Some(Value::Object(summary))
}
```

This is the pattern to extend, not replace. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs]

### Release-Boundary Guard

```ts
// Source: scripts/check-v1.3-release-boundaries.ts
requireContains(
  verifyScript,
  "bun run scripts/check-v1.3-release-boundaries.ts",
  "scripts/verify.sh",
);
requireNotContains(verifyScript, "run-live-mainnet-smoke", "scripts/verify.sh");
```

This is the deterministic SEC-03 pattern to adapt for v1.4. [VERIFIED: scripts/check-v1.3-release-boundaries.ts]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Top-level or sparse live-smoke summaries | Schema v2 `result.*` live-smoke summary fields with fallback to older top-level fields | Phase 52 and later v1.4 phases [VERIFIED: .planning/phases/52-operator-evidence-cleanup/52-CONTEXT.md + packages/open-bitcoin-cli/src/operator/support.rs] | Support extraction should prefer nested `result` before fallback. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs] |
| Header-only progress as broad sync evidence | Separate header, downloaded-block, connected-block, and restart/resume evidence | Phases 56-58 [VERIFIED: 56-01-SUMMARY.md + 57-04-SUMMARY.md + 58-VERIFICATION.md] | Phase 59 docs must preserve the distinction and avoid a single success flag. [VERIFIED: 59-CONTEXT.md] |
| Manual threat/release review only | Deterministic release-boundary checker in default verification | Phase 49 [VERIFIED: scripts/check-v1.3-release-boundaries.ts + scripts/verify.sh] | v1.4 should add its own checker and wire it into `scripts/verify.sh`. [VERIFIED: 59-CONTEXT.md] |
| ASVS identifiers without version | ASVS guidance prefers `v<version>-<requirement>` identifiers because identifiers may change | ASVS 5.0.0 stable release, May 2025 [CITED: https://owasp.org/www-project-application-security-verification-standard/] | If Phase 59 cites ASVS requirements, cite `v5.0.0-*` identifiers or explicitly name ASVS v5.0.0. [CITED: https://owasp.org/www-project-application-security-verification-standard/] |

**Deprecated/outdated:**

- Treating v1.3 threat-model docs as the current milestone closeout is outdated for Phase 59; D-13 requires v1.4-specific surfaces while keeping v1.3 historical. [VERIFIED: 59-CONTEXT.md]
- Treating support bundles as public-mainnet proof is explicitly out of scope; support bundles remain local redacted review/troubleshooting evidence. [VERIFIED: 59-CONTEXT.md + docs/operator/runtime-guide.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Research validity window is estimated at 30 days for repo-local architecture and 7 days for external ASVS/version references. [ASSUMED] | Metadata | Planner might rely on stale external standard/version context if Phase 59 is delayed. |

All non-assumption claims in this research were verified from local files, local command probes, or cited official sources. [VERIFIED: source list below]

## Open Questions (RESOLVED)

1. **Should the v1.4 threat model be a new file or a new section?** [VERIFIED: 59-CONTEXT.md]
   - What we know: Context allows either `docs/parity/threat-model-v1.4.md` or a clearly separated v1.4 section, provided parity roots link to it and v1.3 remains historical. [VERIFIED: 59-CONTEXT.md]
   - Resolution: The planner selected a new file, `docs/parity/threat-model-v1.4.md`, for the v1.4 threat model. This keeps v1.3 immutable and gives `scripts/check-v1.4-release-boundaries.ts` a precise file target. [VERIFIED: inferred from 59-CONTEXT.md + scripts/check-v1.3-release-boundaries.ts]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust `rustc` | Rust code/tests and `scripts/verify.sh` | yes | `rustc 1.94.1` | None needed. [VERIFIED: `rustc --version`] |
| Cargo | Rust package checks | yes | `cargo 1.94.1` | None needed. [VERIFIED: `cargo --version`] |
| cargo-llvm-cov | Repo verify coverage path | yes | `0.8.5` | None needed. [VERIFIED: `cargo llvm-cov --version`] |
| Bun | TypeScript scripts and release-boundary checker | yes | `1.3.9` | None needed. [VERIFIED: `bun --version`] |
| Bazel | Repo smoke build | yes | `8.6.0` | None needed. [VERIFIED: `bazel --version`] |
| Git | Repo root detection and verify scripts | yes | `2.53.0` | None needed. [VERIFIED: `git --version`] |
| Bash | Shell wrappers and `scripts/verify.sh` | yes | GNU bash `3.2.57` | None needed. [VERIFIED: `bash --version`] |
| ripgrep | Research/planning grep checks | yes | `15.1.0` | Use `grep` if absent. [VERIFIED: `rg --version`] |

**Missing dependencies with no fallback:** None found. [VERIFIED: local command probes]

**Missing dependencies with fallback:** None found. [VERIFIED: local command probes]

## Security Domain

Security enforcement is enabled by default because `.planning/config.json` does not set `security_enforcement: false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

OWASP ASVS latest stable version is 5.0.0, and OWASP recommends version-qualified requirement identifiers because identifiers may change between versions. [CITED: https://owasp.org/www-project-application-security-verification-standard/]

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| Authentication / session-style controls | Limited | Phase 59 should not add new auth flows; it should preserve metadata-only credential reporting for RPC cookie/user-password sources. [VERIFIED: docs/architecture/config-precedence.md + packages/open-bitcoin-cli/src/operator/support.rs] |
| Access control | Limited | Support bundles remain local artifacts; do not add hosted upload or remote support access. [VERIFIED: 59-CONTEXT.md] |
| Input validation and sanitization | Yes | Parse live-smoke JSON, compact allowlisted fields, redact sensitive strings, and avoid command-shell execution for user-controlled paths. [VERIFIED: packages/open-bitcoin-cli/src/operator/support.rs + scripts/test-run-live-mainnet-smoke.sh] |
| Cryptography | No new crypto | Do not change `secp256k1` or wallet cryptographic behavior in Phase 59. [VERIFIED: packages/open-bitcoin-consensus/Cargo.toml + packages/open-bitcoin-wallet/Cargo.toml + 59-CONTEXT.md] |
| Logging, error handling, and privacy | Yes | Preserve bounded logs, unavailable reasons, redaction summaries, and no raw secret/log/report embedding. [VERIFIED: docs/architecture/operator-observability.md + packages/open-bitcoin-cli/src/operator/support.rs] |
| Configuration and secrets management | Yes | Report config paths and credential source metadata only; never copy cookie values, `rpcpassword`, `rpcauth`, wallet private material, or seed phrases. [VERIFIED: docs/architecture/config-precedence.md + packages/open-bitcoin-cli/src/operator/support.rs] |

### Known Threat Patterns for Phase 59

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Malicious or misleading public peer input appears as valid v1.4 evidence | Tampering, Denial of Service, Repudiation | Keep endpoint reachability, peer compatibility, header progress, downloaded-block progress, connected-block progress, and restart evidence as separate fields. [VERIFIED: scripts/run-live-mainnet-smoke.ts + docs/architecture/status-snapshot.md] |
| Support evidence leaks secrets or raw operator artifacts | Information Disclosure | Use allowlisted summaries and forbidden-field regression tests. [VERIFIED: packages/open-bitcoin-cli/tests/operator_binary.rs] |
| Release docs overclaim production readiness | Spoofing, Repudiation | Add a v1.4 release-boundary matrix and deterministic checker that names non-claims and future gates. [VERIFIED: docs/parity/release-readiness.md + scripts/check-v1.3-release-boundaries.ts] |
| Default verification accidentally becomes network-dependent | Denial of Service, Repudiation | Assert `scripts/verify.sh` excludes `run-live-mainnet-smoke` and `--restart-after-progress`. [VERIFIED: scripts/check-v1.3-release-boundaries.ts + scripts/verify.sh] |
| Missing live evidence is hidden | Repudiation | Render `Unavailable` with reason through `FieldAvailability` and `EvidenceAvailability`. [VERIFIED: packages/open-bitcoin-node/src/status.rs + packages/open-bitcoin-cli/src/operator/support.rs] |

## Sources

### Primary (HIGH confidence)

- `./AGENTS.md` - repo-local guidance, Rust/Bun/Bazel verification, public-network boundary, repo-local UAT commands. [VERIFIED: file read]
- `./AGENTS.bright-builds.md` - pinned Bright Builds source and highest-signal rules. [VERIFIED: file read]
- `./standards-overrides.md` - no active meaningful local override beyond placeholder table. [VERIFIED: file read]
- `.planning/STATE.md` - v1.4 accumulated decisions and Phase 59 state. [VERIFIED: file read]
- `.planning/ROADMAP.md` - Phase 59 goal and success criteria. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - OBS-01 through OBS-03 and SEC-01 through SEC-03. [VERIFIED: file read]
- `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md` - locked decisions, discretion, deferred ideas, implementation surfaces. [VERIFIED: file read]
- `packages/open-bitcoin-cli/src/operator/support.rs` and `support/render.rs` - support bundle allowlist/redaction/rendering. [VERIFIED: codebase grep/read]
- `packages/open-bitcoin-cli/src/operator/status/render.rs`, `status/sync_state.rs`, `dashboard/model.rs` - status/dashboard projections. [VERIFIED: codebase grep/read]
- `packages/open-bitcoin-node/src/status.rs`, `sync/types/summary.rs`, `sync/types/projection.rs` - shared snapshot and sync summary projection. [VERIFIED: codebase grep/read]
- `scripts/run-live-mainnet-smoke.ts` and `scripts/test-run-live-mainnet-smoke.sh` - schema v2 live evidence and deterministic fixture checks. [VERIFIED: codebase grep/read]
- `scripts/check-v1.3-release-boundaries.ts` and `scripts/verify.sh` - deterministic release-boundary guard pattern. [VERIFIED: codebase grep/read]
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`, `docs/architecture/config-precedence.md`, `docs/parity/*` - docs and parity roots. [VERIFIED: codebase grep/read]
- Phase 54-58 summaries and Phase 58 verification - prior evidence handoff. [VERIFIED: phase artifact reads]

### External Primary (HIGH confidence)

- Bright Builds standards index, architecture, code-shape, verification, testing, Rust, TypeScript/JavaScript, and operability pages at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676`. [CITED: https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]
- OWASP ASVS project page - ASVS purpose, latest stable 5.0.0, and version-qualified identifier guidance. [CITED: https://owasp.org/www-project-application-security-verification-standard/]

### Secondary (MEDIUM confidence)

- None used. [VERIFIED: source review]

### Tertiary (LOW confidence)

- None used. [VERIFIED: source review]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - versions and tools were verified from local manifests, lockfile, and command probes. [VERIFIED: rust-toolchain.toml + packages/Cargo.toml + packages/Cargo.lock + local command probes]
- Architecture: HIGH - implementation surfaces are local first-party code and docs with direct prior-phase evidence. [VERIFIED: codebase grep/read + phase summaries]
- Pitfalls: HIGH - pitfalls map directly to locked decisions, existing tests, and v1.3 release-boundary patterns. [VERIFIED: 59-CONTEXT.md + packages/open-bitcoin-cli/tests/operator_binary.rs + scripts/check-v1.3-release-boundaries.ts]
- Security: MEDIUM-HIGH - local threat model pattern and ASVS version were verified, but exact ASVS requirement IDs should be chosen during implementation if the docs cite individual controls. [VERIFIED: docs/parity/threat-model-v1.3.md; CITED: OWASP ASVS]

**Research date:** 2026-06-05 [VERIFIED: system current_date]
**Valid until:** 2026-07-05 for repo-local architecture; 2026-06-12 for external ASVS/version references. [ASSUMED]
