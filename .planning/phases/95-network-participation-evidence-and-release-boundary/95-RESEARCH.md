# Phase 95: Network Participation Evidence and Release Boundary - Research

**Researched:** 2026-06-27 [VERIFIED: environment_context current_date]
**Domain:** v1.9 release-boundary evidence, parity roots, deterministic Bun checkers, operator UAT docs, and support-bundle redaction [VERIFIED: .planning/phases/95-network-participation-evidence-and-release-boundary/95-CONTEXT.md]
**Confidence:** HIGH for repo-local implementation patterns; MEDIUM for exact optional public-entrypoint doc scope because Phase 95 leaves some file-selection details to the agent. [VERIFIED: .planning/phases/95-network-participation-evidence-and-release-boundary/95-CONTEXT.md]

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

## Implementation Decisions

### Deterministic Release-Boundary Checker
- **D-01:** Add one focused Phase 95 aggregate Bun checker and fixture test, wired immediately after Phase 94 in `scripts/verify.sh` and in the legacy `VERIFY_COMMAND_ORDER` block. Do not scatter Phase 95 closeout logic across Phase 90 through Phase 94 checkers unless a genuinely phase-local gap is discovered.
- **D-02:** The Phase 95 checker must prove BOUND-01, BOUND-03, BOUND-04, BOUND-05, and BOUND-06 through static, deterministic, public-network-free assertions over release/parity/operator docs, support evidence roots, `scripts/verify.sh` ordering, and requirement traceability.
- **D-03:** The checker must reject positive v1.9 claims for transaction relay, compact block relay, mempool propagation, broad/full address relay beyond the scoped Phase 92 boundary, public inbound defaults, public-network CI, production-service operation, and production full-node readiness. Valid no-claim, deferred, unsupported, future, opt-in UAT, and evidence-boundary wording must remain allowed.

### Parity Roots And Traceability
- **D-04:** Add a compact v1.9 release-boundary closeout surface inside the existing parity roots rather than creating a standalone competing manifest. `docs/parity/index.json` remains the machine-readable root; `docs/parity/checklist.md`, `docs/parity/catalog/p2p.md`, and `docs/parity/release-readiness.md` carry the human review evidence.
- **D-05:** `docs/parity/catalog/p2p.md` should include a Phase 95/v1.9 closeout rollup that cites the required Knots anchors: `packages/bitcoin-knots/src/net.cpp`, `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/src/addrman.cpp`, `packages/bitcoin-knots/src/banman.cpp`, and `packages/bitcoin-knots/src/net_permissions.cpp`, plus any already-used protocol or functional-test anchors where they clarify evidence.
- **D-06:** Requirement traceability must keep all 28 v1.9 requirements mapped exactly once across Phase 90 through Phase 95, with BOUND-01 through BOUND-06 assigned to Phase 95. Planning, summaries, verification, and any milestone audit artifacts should reference the parity roots instead of becoming separate release evidence sources.

### UAT And Non-Regression Evidence
- **D-07:** Phase 95 should use a deterministic closeout mix: focused Phase 95 checker/test commands for local iteration, full `bash scripts/verify.sh` as the repo-native non-regression proof, and operator UAT docs with copy-pasteable repo-local Cargo and Bazel commands for loopback or synthetic inbound review.
- **D-08:** UAT guidance must use explicit repo-local command forms, including `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- ...`, `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`, `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- ...`, `bazel run //packages/open-bitcoin-rpc:open_bitcoind -- ...`, `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`, and `bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- ...` where applicable.
- **D-09:** Public-network full-sync, soak, real service-manager, and live support-bundle collection remain optional operator evidence only. They must not become default verification, release-blocking CI, or wording that implies production readiness.

### Support Bundle Redaction
- **D-10:** Close BOUND-05 primarily with aggregate Phase 95 checker assertions over existing Phase 90 through Phase 94 support evidence roots. Preserve behavior unless the aggregate check exposes a real leak.
- **D-11:** The checker should assert that support evidence preserves useful inbound diagnosis across admission, permission, address, eviction/ban, misbehavior, and resource-governance surfaces while redacting or bounding raw peer addresses, endpoints, peer IDs, permission strings, config names, payload bytes, credentials, and unbounded tables.
- **D-12:** Add a narrow resource-governance support evidence assertion if needed so Phase 94 resource counters and latest decision evidence are preserved without leaking raw payload or peer material.

### the agent's Discretion
- Exact checker helper structure, fixture layout, and failure-message wording.
- Whether Phase 95 requires a new milestone audit file during execution or records closeout only through phase verification and existing parity roots.
- How to minimize brittle prose scanning while still catching forbidden release claims.
- Whether to reuse Phase 88 no-claim helper patterns directly or extract small shared local helpers inside the Phase 95 checker.

### Deferred Ideas (OUT OF SCOPE)

## Deferred Ideas

- Transaction relay and mempool propagation remain future relay scope.
- Compact block relay remains future relay scope.
- Full address relay and broader peer discovery remain future address-relay/network-participation scope.
- Public inbound serving by default, public-network CI, production-service operation, and production full-node readiness remain future release/support scope.
- Signed packaging, hosted dashboards, GUI, migration apply mode, destructive repair, production-funds wallet operation, automatic support-bundle upload, and Windows service integration remain outside v1.9.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| BOUND-01 | Release docs, parity docs, and deterministic checkers prohibit transaction relay, compact block relay, mempool propagation, production-node readiness, production-service, and public inbound default claims for v1.9. [VERIFIED: .planning/REQUIREMENTS.md] | Use one aggregate Bun checker with curated doc corpus, context-unit prose scanning, scoped allowance terms, and fixture mutation tests. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts] |
| BOUND-02 | v1.9 parity breadcrumbs and documentation cite Knots anchors for `net.cpp`, `net_processing.cpp`, `addrman.cpp`, `banman.cpp`, and `net_permissions.cpp`, or record intentional deviations. [VERIFIED: .planning/REQUIREMENTS.md] | Add a Phase 95/v1.9 closeout row in `docs/parity/catalog/p2p.md` and a matching parity index/checklist surface requiring those anchors. [VERIFIED: docs/parity/catalog/p2p.md; docs/parity/index.json; 95-CONTEXT.md] |
| BOUND-03 | Existing outbound sync, full-sync, soak, support-bundle, release-boundary, and production no-claim behavior remains non-regressed while inbound serving is added. [VERIFIED: .planning/REQUIREMENTS.md] | Wire Phase 95 immediately after Phase 94 and before pure-core checks; final proof is full `bash scripts/verify.sh`. [VERIFIED: scripts/verify.sh; 95-CONTEXT.md] |
| BOUND-04 | Operator UAT guidance includes repo-local Cargo and Bazel command forms for loopback or synthetic inbound review, not only an installed `open-bitcoin` alias. [VERIFIED: .planning/REQUIREMENTS.md] | Preserve and aggregate-check the open-bitcoind, open-bitcoin-cli, and open-bitcoin Cargo/Bazel command families already documented for Phase 90 through Phase 94. [VERIFIED: docs/operator/runtime-guide.md] |
| BOUND-05 | Support bundles redact inbound peer addresses where needed while preserving enough admission, permission, eviction, ban, and resource evidence for diagnosis. [VERIFIED: .planning/REQUIREMENTS.md] | Existing tests cover Phase 90 endpoints, Phase 91 permissions, Phase 92 address evidence, Phase 93 peer policy, and Phase 94 rendering; add a narrow resource-governance redaction assertion/sanitizer if the planner accepts the identified leak surface. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/tests.rs; packages/open-bitcoin-cli/src/operator/support/redaction.rs] |
| BOUND-06 | Requirements, roadmap, phase summaries, verification reports, and milestone audit artifacts map every v1.9 requirement exactly once. [VERIFIED: .planning/REQUIREMENTS.md] | Parse `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `docs/parity/index.json`, and `docs/parity/checklist.md` for the 28 v1.9 IDs and add the Phase 95 surface with BOUND-01 through BOUND-06 exactly once. [VERIFIED: .planning/REQUIREMENTS.md; .planning/ROADMAP.md; docs/parity/index.json; docs/parity/checklist.md] |
</phase_requirements>

## Summary

Phase 95 should be planned as a release-boundary closeout, not a runtime networking feature. [VERIFIED: 95-CONTEXT.md] The concrete implementation shape is one Bun checker plus fixture test, parity/release/operator doc updates, `scripts/verify.sh` ordering, and a small support-redaction patch only if needed for resource-governance evidence. [VERIFIED: 95-CONTEXT.md; scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts; packages/open-bitcoin-cli/src/operator/support/redaction.rs]

The strongest existing patterns are Phase 88's broad no-claim prose guardrail and Phase 94's current v1.9 checker style: fixed target files, `JSON.parse` for parity roots, context-unit scanning over paragraphs/table rows, scoped allowance terms, explicit verifier-order assertions, and mutation-based Bun fixture tests. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase88-deterministic-claim-guardrails.test.ts; scripts/check-phase94-dos-resource-governance.ts; scripts/check-phase94-dos-resource-governance.test.ts]

**Primary recommendation:** implement `scripts/check-phase95-network-participation-release-boundary.ts` plus `scripts/check-phase95-network-participation-release-boundary.test.ts`, wire both immediately after Phase 94 in `scripts/verify.sh`, update existing parity roots/docs, and add a narrow resource-governance support redaction test/sanitizer if the Phase 95 checker exposes that gap. [VERIFIED: 95-CONTEXT.md; scripts/verify.sh; packages/open-bitcoin-cli/src/operator/support/redaction.rs]

## Project Constraints (from AGENTS.md)

- Read and follow `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` when present, and task-relevant standards pages before planning/implementation/audit work. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards/index.md; standards-overrides.md]
- Use `git submodule update --init --recursive` when Knots source material must be materialized. [VERIFIED: AGENTS.md]
- Treat `rust-toolchain.toml` as the Rust source of truth; this repo pins Rust `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml; cargo --version; rustc --version]
- Use `bash scripts/verify.sh` as the repo-native verification contract; `--fast` is local iteration only and the default command remains the pre-commit/release contract. [VERIFIED: AGENTS.md; scripts/verify.sh]
- During UAT, provide copy-pasteable repo-local Cargo and Bazel commands rather than only naming the installed `open-bitcoin` alias. [VERIFIED: AGENTS.md; docs/operator/runtime-guide.md]
- Use Bun as the canonical runtime for repo-owned higher-level automation; prefer TypeScript for substantial script logic and keep Bash thin. [VERIFIED: AGENTS.md; standards/languages/typescript-javascript.md; .bun-version]
- Treat `docs/metrics/lines-of-code.md` as a tracked generated artifact that may change during hook or verifier runs. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Record in-scope behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion parity docs. [VERIFIED: AGENTS.md; docs/parity/index.json]
- When adding first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, add parity breadcrumbs through `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`; existing touched Rust files already carry breadcrumb headers. [VERIFIED: AGENTS.md; packages/open-bitcoin-cli/src/operator/support/redaction.rs; packages/open-bitcoin-cli/src/operator/support/tests.rs]
- After substantial release, parity, operator-surface, or workflow changes, check whether relevant README files need updates. [VERIFIED: AGENTS.md; README.md]
- Rust tests should use explicit Arrange, Act, Assert sections when the structure is not trivial. [VERIFIED: AGENTS.md; standards/core/testing.md; packages/open-bitcoin-cli/src/operator/support/tests.rs]
- New or substantial repo-owned TypeScript automation should use plain functions, typed data, `maybe...` naming for nullish values, and Bun tests where appropriate. [VERIFIED: standards/languages/typescript-javascript.md; scripts/check-phase94-dos-resource-governance.test.ts]
- No project-local skills exist under `.claude/skills`, `.agents/skills`, `.cursor/skills`, or `.github/skills` in this checkout. [VERIFIED: find .claude/skills .agents/skills .cursor/skills .github/skills]

## Standard Stack

### Core

| Library/Tool | Version | Purpose | Why Standard |
|--------------|---------|---------|--------------|
| Bun | 1.3.9 pinned and installed | Run repo-owned TypeScript checkers and `bun:test` fixture tests. | Existing Phase 88 through Phase 94 deterministic checkers use Bun, and repo-local guidance names Bun as canonical for substantial automation scripts. [VERIFIED: .bun-version; bun --version; AGENTS.md; scripts/check-phase94-dos-resource-governance.ts] |
| TypeScript under Bun | Bundled with Bun execution surface | Implement the Phase 95 aggregate static checker. | Existing scripts are TypeScript files run directly by Bun with typed constants, fixed target corpora, and exported checker functions. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts] |
| Bash `scripts/verify.sh` | GNU bash 3.2.57 locally | Wire checker/test ordering and prove full non-regression. | Repo guidance makes this the native verification contract, and existing release-boundary checkers inspect its executable path plus `VERIFY_COMMAND_ORDER` heredoc. [VERIFIED: AGENTS.md; bash --version; scripts/verify.sh] |
| Rust/Cargo | Rust/Cargo 1.94.1 pinned and installed | Add or verify narrow support-bundle redaction behavior if needed. | Existing support rendering/redaction and tests are first-party Rust under `open-bitcoin-cli`, with Rust 2024 workspace metadata. [VERIFIED: rust-toolchain.toml; cargo --version; packages/Cargo.toml; packages/open-bitcoin-cli/src/operator/support/redaction.rs] |
| Bazel/Bazelisk command surface | Bazel 8.6.0 pinned and installed | Provide repo-local Bazel UAT command forms and full smoke build through default verifier. | Repo guidance requires Bazel command forms for operator UAT and `scripts/verify.sh` runs Bazel in full mode. [VERIFIED: .bazelversion; bazel --version; AGENTS.md; scripts/verify.sh] |

### Supporting

| Library/Tool | Version | Purpose | When to Use |
|--------------|---------|---------|-------------|
| `JSON.parse` | Runtime built-in | Validate `docs/parity/index.json` structure without ad hoc text parsing. | Use for parity root, checklist surface, audit entry, and requirement arrays. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts] |
| `rg` | Available in repo workflow | Targeted source/doc inspection during planning and verification. | Use for local developer investigation, not as the Phase 95 checker core when JSON structure is available. [VERIFIED: AGENTS.md; research command outputs] |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::support::tests::...` | Cargo 1.94.1 | Fast focused support-redaction verification if Rust support code changes. | Use before full `bash scripts/verify.sh` when adding the narrow BOUND-05 support assertion. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/tests.rs; cargo --version] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| One aggregate Phase 95 checker | Patch Phase 90 through Phase 94 checkers | Locked decision D-01 rejects scattering closeout logic unless a phase-local gap is discovered. [VERIFIED: 95-CONTEXT.md] |
| Fixed-file curated corpus | Repository-wide docs scan | Existing checkers intentionally avoid broad `.planning`/history scans to reduce false positives and keep default verification deterministic. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts] |
| Existing parity roots | New standalone evidence manifest | Locked decision D-04 keeps `docs/parity/index.json` as the machine-readable root and avoids a competing manifest. [VERIFIED: 95-CONTEXT.md] |
| Shared support status/redaction | New Phase 95 support artifact format | Existing support bundles already project `OpenBitcoinStatusSnapshot.peers.inbound`; Phase 95 should close gaps there rather than add a parallel support contract. [VERIFIED: docs/architecture/status-snapshot.md; packages/open-bitcoin-cli/src/operator/support/tests.rs] |

**Installation:**

```bash
# No new npm, Cargo, or Bazel dependencies are recommended for Phase 95.
```

**Version verification:** Recommended tools were verified from pinned repo files and local commands, not npm registry package additions. [VERIFIED: .bun-version; rust-toolchain.toml; .bazelversion; bun --version; cargo --version; rustc --version; bazel --version]

## Architecture Patterns

### Recommended Project Structure

```text
scripts/
  check-phase95-network-participation-release-boundary.ts       # aggregate fixed-file checker [VERIFIED: scripts/check-phase94-dos-resource-governance.ts]
  check-phase95-network-participation-release-boundary.test.ts  # mutation fixture tests [VERIFIED: scripts/check-phase94-dos-resource-governance.test.ts]
docs/parity/
  index.json                    # machine-readable Phase 95 surface/checklist/audit root [VERIFIED: docs/parity/index.json]
  checklist.md                  # human-readable Phase 95 surface row [VERIFIED: docs/parity/checklist.md]
  release-readiness.md          # v1.9 closeout matrix and reviewer commands [VERIFIED: docs/parity/release-readiness.md]
  production-claim-boundary.md  # optional stale-wording cleanup for v1.9 inbound nuance [VERIFIED: docs/parity/production-claim-boundary.md]
  support-matrix.md             # optional support-term wording cleanup if checker corpus includes it [VERIFIED: docs/parity/support-matrix.md]
  catalog/p2p.md                # Phase 95/v1.9 closeout rollup and Knots anchors [VERIFIED: docs/parity/catalog/p2p.md]
docs/operator/
  runtime-guide.md              # aggregate v1.9 loopback/synthetic UAT commands [VERIFIED: docs/operator/runtime-guide.md]
packages/open-bitcoin-cli/src/operator/support/
  redaction.rs                  # only if resource-governance redaction gap is fixed [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs]
  tests.rs                      # narrow BOUND-05 regression assertion if Rust support changes [VERIFIED: packages/open-bitcoin-cli/src/operator/support/tests.rs]
scripts/verify.sh               # Phase 95 test/checker wiring immediately after Phase 94 [VERIFIED: scripts/verify.sh]
README.md                       # check/update public entrypoint if v1.9 release boundary changes visible status [VERIFIED: AGENTS.md; README.md]
```

### Pattern 1: Aggregate Fixed-File Checker

**What:** Read a curated corpus into a `Map`, validate structured JSON with `JSON.parse`, scan human docs by paragraph/table-row context units, and return `string[]` failures from an exported checker function. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts]

**When to use:** Use this for BOUND-01, BOUND-03, BOUND-04, BOUND-05, and BOUND-06 because those requirements are static docs/root/verifier/support-boundary assertions. [VERIFIED: 95-CONTEXT.md; .planning/REQUIREMENTS.md]

**Example:**

```typescript
export function checkPhase95NetworkParticipationReleaseBoundary(
  options: CheckPhase95Options = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = readTargetFiles(repoRoot, failures);

  verifyParityIndex(texts.get("docs/parity/index.json") ?? "", failures);
  verifyNoClaimBoundary(texts, failures);
  verifyUatCommands(texts.get("docs/operator/runtime-guide.md") ?? "", failures);
  verifySupportRedactionBoundary(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyRequirementTraceability(texts, failures);

  return failures;
}
```

Source: adapted from existing exported checker style. [VERIFIED: scripts/check-phase94-dos-resource-governance.ts]

### Pattern 2: Scoped No-Claim Scanning

**What:** Split prose into context units, allow explicit no-claim/deferred/future/opt-in wording, and fail only positive claim markers for forbidden surfaces. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts]

**When to use:** Use this for transaction relay, compact block relay, mempool propagation, broad/full address relay, public inbound defaults, public-network CI, production-service operation, and production full-node readiness. [VERIFIED: 95-CONTEXT.md]

**Implementation note:** Keep allowed terms at least as broad as Phase 94's `does not`, `do not`, `not`, `no`, `without`, `outside`, `remain outside`, `remains outside`, `deferred`, `future`, `not claim`, `not claiming`, `no-claim`, and `non-claim`; add `opt-in UAT`, `unsupported`, `evidence-boundary`, and `release-boundary` because D-03 explicitly allows those forms. [VERIFIED: scripts/check-phase94-dos-resource-governance.ts; 95-CONTEXT.md]

### Pattern 3: Verifier Ordering Must Check Both Heredoc and Executable Path

**What:** Existing checkers remove the `VERIFY_COMMAND_ORDER` heredoc to inspect executed `run_step` lines and separately validate that the heredoc preserves legacy command order. [VERIFIED: scripts/check-phase94-dos-resource-governance.ts; scripts/verify.sh]

**When to use:** Phase 95 must be inserted after Phase 94 in both the heredoc and executable `run_step` path. [VERIFIED: 95-CONTEXT.md; scripts/verify.sh]

**Required order:**

```text
bun test scripts/check-phase94-dos-resource-governance.test.ts
bun run scripts/check-phase94-dos-resource-governance.ts
bun test scripts/check-phase95-network-participation-release-boundary.test.ts
bun run scripts/check-phase95-network-participation-release-boundary.ts
bash scripts/check-pure-core-deps.sh
```

[VERIFIED: scripts/verify.sh; 95-CONTEXT.md]

### Pattern 4: Mutation Fixture Tests

**What:** Build a temporary fixture with all target files present, assert the pass case, then mutate one concern at a time to prove failure messages for missing roots, forbidden claims, missing UAT commands, verifier drift, and support-redaction leaks. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.test.ts; scripts/check-phase94-dos-resource-governance.test.ts]

**When to use:** Use this to keep the Phase 95 checker small, deterministic, and resistant to false positives. [VERIFIED: standards/core/testing.md; scripts/check-phase94-dos-resource-governance.test.ts]

### Pattern 5: Support Redaction Closes Gaps in Shared Status, Not Rendered Prose Only

**What:** Support bundles call `support_status_for_bundle`, which currently redacts inbound endpoints, permission evidence, address evidence, and peer-policy evidence before JSON/Markdown rendering. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs]

**Gap:** `support_status_for_bundle` does not currently call a resource-governance sanitizer for `latest_resource_governance_decision`, even though that event has free-form `outcome`, `reason`, `label`, `source`, `message`, and `next_action` strings. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs; packages/open-bitcoin-node/src/status/inbound.rs]

**When to use:** If the Phase 95 BOUND-05 checker/test injects `127.0.0.1:18444`, `peer_id=`, `payload_bytes`, `raw_permission`, `credential`, or `secret` into resource-governance decision fields, add `redact_inbound_resource_governance_evidence` and a test in `packages/open-bitcoin-cli/src/operator/support/tests.rs`. [VERIFIED: 95-CONTEXT.md; packages/open-bitcoin-cli/src/operator/support/tests.rs]

### Recommended Plan Slices

1. Checker/test skeleton and fixture: create the Phase 95 Bun checker/test with pass fixture, forbidden-claim mutations, missing-UAT mutations, missing-root mutations, verifier-order mutations, and support-redaction mutation. [VERIFIED: scripts/check-phase94-dos-resource-governance.test.ts; 95-CONTEXT.md]
2. Parity and release docs: add the v1.9 closeout surface to `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/catalog/p2p.md`, and `docs/parity/release-readiness.md`. [VERIFIED: 95-CONTEXT.md; docs/parity/index.json; docs/parity/checklist.md]
3. Operator UAT closeout: add a compact Phase 95/v1.9 review section to `docs/operator/runtime-guide.md` that points to Phase 90 through Phase 94 command families and includes all required Cargo/Bazel binary forms. [VERIFIED: 95-CONTEXT.md; docs/operator/runtime-guide.md]
4. Support redaction gap: add a narrow Rust test and sanitizer for resource-governance decision fields only if the checker confirms the current redaction boundary is insufficient. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs; packages/open-bitcoin-cli/src/operator/support/tests.rs]
5. Verifier wiring and final proof: wire the Phase 95 checker/test after Phase 94 and run focused Bun tests/checker, focused Rust support test if changed, then full `bash scripts/verify.sh`. [VERIFIED: scripts/verify.sh; AGENTS.md]

### Anti-Patterns to Avoid

- **Do not add runtime network behavior:** Phase 95 is evidence/docs/checker work and must not expand transaction relay, compact blocks, mempool propagation, public inbound defaults, service operation, or production readiness. [VERIFIED: 95-CONTEXT.md]
- **Do not scan all `.planning` history in the checker:** Historical plans contain deliberate future-scope phrases and will create false positives; use active requirements/roadmap plus current public docs/parity roots. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts]
- **Do not count commands that exist only in `VERIFY_COMMAND_ORDER`:** Existing checkers explicitly strip the heredoc before proving executable `run_step` wiring. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts]
- **Do not add public-network, service-manager, long-running, or multi-day commands to default verification:** Existing release-boundary docs and `scripts/verify.sh` keep those opt-in only. [VERIFIED: docs/operator/runtime-guide.md; docs/parity/release-readiness.md; scripts/verify.sh]
- **Do not add a new evidence manifest:** The locked decision keeps `docs/parity/index.json` as the machine-readable root. [VERIFIED: 95-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Parity root validation | Regex over JSON text | `JSON.parse` plus typed unknown checks | Existing checkers validate arrays/status/requirements/evidence structurally. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts] |
| Release claim classifier | Natural-language parser or broad AI-style classifier | Explicit forbidden surfaces, positive markers, and scoped allowance terms | Existing deterministic checks stay explainable and fixture-testable. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts] |
| Support-bundle evidence model | New Phase 95 support schema | `OpenBitcoinStatusSnapshot.peers.inbound` plus existing support redaction/rendering | Status snapshot is the shared source for CLI, RPC, metrics, logs, and support. [VERIFIED: docs/architecture/status-snapshot.md; packages/open-bitcoin-cli/src/operator/support/tests.rs] |
| UAT command matrix | Installed alias-only examples | Repo-local Cargo and Bazel command forms | Repo guidance and D-08 require explicit Cargo/Bazel forms. [VERIFIED: AGENTS.md; 95-CONTEXT.md] |
| Non-regression proof | New ad hoc command bundle | Focused Phase 95 checker/test plus full `bash scripts/verify.sh` | `scripts/verify.sh` is the repo-native contract and already runs prior release-boundary checks. [VERIFIED: AGENTS.md; scripts/verify.sh] |
| Requirement traceability | Separate spreadsheet or manifest | `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `docs/parity/index.json`, and `docs/parity/checklist.md` | Locked decisions require existing parity roots and exactly-once mapping. [VERIFIED: 95-CONTEXT.md; .planning/REQUIREMENTS.md; .planning/ROADMAP.md] |

**Key insight:** Phase 95 is a boundary-policing problem; custom runtime probes or new manifests would increase scope while making the actual release claim less auditable. [VERIFIED: 95-CONTEXT.md; docs/parity/index.json; scripts/verify.sh]

## Common Pitfalls

### Pitfall 1: Positive Claim False Positives From No-Claim Text

**What goes wrong:** A checker fails valid sentences such as "does not claim transaction relay" because it only searches forbidden terms. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts]

**Why it happens:** Prose contains forbidden surfaces in both valid no-claim wording and invalid promotional wording. [VERIFIED: docs/parity/catalog/p2p.md; docs/operator/runtime-guide.md]

**How to avoid:** Reuse context-unit scanning with scoped allowance terms before checking positive markers. [VERIFIED: scripts/check-phase94-dos-resource-governance.ts]

**Warning signs:** Tests only include positive forbidden strings and no "allows scoped no-claim" fixtures. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.test.ts]

### Pitfall 2: Stale v1.8 Production Boundary Wording

**What goes wrong:** Public release docs keep saying broad "inbound serving" is deferred, even after v1.9 adds explicit opt-in inbound serving evidence. [VERIFIED: docs/parity/production-claim-boundary.md; docs/parity/support-matrix.md; docs/parity/catalog/p2p.md]

**Why it happens:** v1.8 documents predate v1.9 and use broad labels for production-adjacent deferred surfaces. [VERIFIED: docs/parity/production-claim-boundary.md; docs/parity/support-matrix.md]

**How to avoid:** Update public-facing wording to distinguish "opt-in bounded inbound listener/admission evidence exists" from "public inbound defaults, production network participation, and production full-node readiness remain deferred." [VERIFIED: 95-CONTEXT.md; docs/operator/runtime-guide.md]

**Warning signs:** Rows such as "Open Bitcoin supports relay/inbound serving" stay unchanged without a v1.9 qualifier. [VERIFIED: docs/parity/production-claim-boundary.md]

### Pitfall 3: Resource-Governance Support Decision Leak

**What goes wrong:** A support bundle could preserve raw endpoint, peer id, payload, credential, or permission material inside `latest_resource_governance_decision`. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs; packages/open-bitcoin-node/src/status/inbound.rs]

**Why it happens:** Existing support redaction sanitizes endpoints, permission evidence, address evidence, and peer-policy evidence, but does not sanitize resource-governance decision fields. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs]

**How to avoid:** Add one sanitizer for `InboundResourceGovernanceEvent` fields plus a test that injects forbidden raw resource material and confirms JSON/Markdown redaction. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/tests.rs; 95-CONTEXT.md]

**Warning signs:** `support_status_for_bundle` still calls only four inbound redaction helpers and not a resource-governance helper. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs]

### Pitfall 4: Requirement Double Mapping

**What goes wrong:** BOUND-01 through BOUND-06 appear in multiple phase surfaces or v1.9 totals no longer equal 28. [VERIFIED: .planning/REQUIREMENTS.md; .planning/ROADMAP.md]

**Why it happens:** Phase 95 touches cross-phase closeout docs, so it is easy to copy prior requirement IDs into the new surface instead of referencing them as evidence. [VERIFIED: docs/parity/index.json; 95-CONTEXT.md]

**How to avoid:** The Phase 95 parity surface should list exactly BOUND-01 through BOUND-06; prior Phase 90 through Phase 94 surfaces keep their original INB/PERM/ADDR/EVICT/DOS IDs. [VERIFIED: docs/parity/index.json; .planning/REQUIREMENTS.md]

**Warning signs:** The Phase 95 checklist row includes INB, PERM, ADDR, EVICT, or DOS IDs as requirements instead of evidence references. [VERIFIED: docs/parity/checklist.md]

### Pitfall 5: Verifier Drift Into Public Network Or Service Operations

**What goes wrong:** Default verification starts running listener exposure, public-network CI, systemd/launchd, live sync, or multi-day soak commands. [VERIFIED: 95-CONTEXT.md; scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts]

**Why it happens:** UAT commands are copy-pasteable and can be mistakenly promoted into `scripts/verify.sh`. [VERIFIED: docs/operator/runtime-guide.md; scripts/verify.sh]

**How to avoid:** Phase 95 checker should fail executable verifier text containing `systemctl`, `launchctl`, public-network listener phrases, wildcard listener commands, public-network CI, long sleeps, or release-blocking live sync strings. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts]

**Warning signs:** `scripts/verify.sh` gets `0.0.0.0`, mainnet listener, service-manager, or public-network command text outside comments/heredoc. [VERIFIED: scripts/check-phase94-dos-resource-governance.ts]

## Code Examples

### Context Units For Prose/Table Scans

```typescript
function contextUnits(text: string): string[] {
  const units: string[] = [];
  for (const block of text.replaceAll("\r\n", "\n").split(/\n\s*\n/)) {
    const lines = block
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0);
    if (lines.length === 0) {
      continue;
    }
    const tableRows = lines.filter((line) => line.startsWith("|") && !/^\|\s*-/.test(line));
    if (tableRows.length > 0) {
      units.push(...tableRows.map(normalizeWhitespace));
      units.push(...sentenceUnits(lines.filter((line) => !line.startsWith("|")).join(" ")));
      continue;
    }
    units.push(...sentenceUnits(lines.join(" ")));
  }
  return units.map(normalizeWhitespace).filter((unit) => unit.length > 0);
}
```

Source: existing Phase 94 checker pattern. [VERIFIED: scripts/check-phase94-dos-resource-governance.ts]

### Verifier Heredoc Stripping

```typescript
function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}
```

Source: existing release-boundary checker pattern. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts]

### Narrow Resource-Governance Redaction Shape

```rust
fn redact_inbound_resource_governance_evidence(
    inbound: &mut FieldAvailability<InboundPeerServingStatus>,
) {
    let FieldAvailability::Available(evidence) = inbound else {
        return;
    };
    let FieldAvailability::Available(event) =
        &mut evidence.latest_resource_governance_decision
    else {
        return;
    };
    event.outcome = sanitized_resource_governance_text(&event.outcome);
    event.reason = sanitized_resource_governance_text(&event.reason);
    event.label = sanitized_resource_governance_text(&event.label);
    event.source = sanitized_resource_governance_text(&event.source);
    event.message = sanitized_resource_governance_text(&event.message);
    event.next_action = sanitized_resource_governance_text(&event.next_action);
}
```

Source: recommended by analogy to existing address and peer-policy redaction helpers; exact helper names are planner discretion. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs; packages/open-bitcoin-node/src/status/inbound.rs]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| v1.8 broad production-claim boundary forbids inbound serving claims entirely. [VERIFIED: docs/parity/production-claim-boundary.md] | v1.9 has opt-in inbound listener, permissions, address-boundary, peer-policy, and resource-governance evidence, while public defaults and production readiness stay deferred. [VERIFIED: .planning/ROADMAP.md; docs/parity/catalog/p2p.md] | v1.9 Phase 90 through Phase 94 completed before Phase 95. [VERIFIED: .planning/STATE.md; .planning/ROADMAP.md] | Phase 95 must update wording to avoid both overclaiming and stale underclaiming. [VERIFIED: 95-CONTEXT.md] |
| Phase-specific checkers prove local slices. [VERIFIED: scripts/check-phase90-inbound-listener-admission.ts; scripts/check-phase94-dos-resource-governance.ts] | Phase 95 should aggregate closeout evidence across all v1.9 slices without scattering logic. [VERIFIED: 95-CONTEXT.md] | Locked in Phase 95 D-01. [VERIFIED: 95-CONTEXT.md] | Planner should create one checker/test plan, not five phase-checker edits. [VERIFIED: 95-CONTEXT.md] |
| Support redaction summary covered inbound endpoints, permissions, address evidence, and peer policy. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs] | Resource-governance evidence now exists and needs the same redaction boundary if free-form fields can carry raw material. [VERIFIED: packages/open-bitcoin-node/src/status/inbound.rs; packages/open-bitcoin-cli/src/operator/support/tests.rs] | Phase 94 added resource-governance evidence before Phase 95. [VERIFIED: docs/parity/catalog/p2p.md; packages/open-bitcoin-cli/src/operator/support/tests.rs] | Add a narrow BOUND-05 guard instead of rewriting support bundles. [VERIFIED: 95-CONTEXT.md] |

**Deprecated/outdated:**

- Treating all "inbound serving" as deferred is outdated for v1.9 docs; the precise deferred claims are public inbound defaults, broad/full address relay, production-service operation, production network participation, and production full-node readiness. [VERIFIED: docs/parity/production-claim-boundary.md; .planning/ROADMAP.md; 95-CONTEXT.md]
- Treating support-bundle resource-governance rendering as sufficient redaction proof is incomplete because rendering tests do not inject raw resource material into resource-governance decision fields. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/tests.rs; packages/open-bitcoin-cli/src/operator/support/redaction.rs]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | README, `docs/parity/support-matrix.md`, and `docs/parity/production-claim-boundary.md` should be treated as likely but planner-confirmed public-entrypoint updates, not mandatory locked files, because Phase 95 D-04 names `index.json`, `checklist.md`, `catalog/p2p.md`, and `release-readiness.md` as the human/machine parity roots. [ASSUMED] | Architecture Patterns | If wrong, public release wording may remain stale or the checker may miss an overclaim/underclaim in public entrypoints. |
| A2 | The research remains valid until 2026-07-27 unless Phase 95 planning changes the locked checker/file scope. [ASSUMED] | Metadata | If wrong, planner may rely on stale local checker or doc scope after the phase evolves. |

## Open Questions (RESOLVED)

1. **RESOLVED: Should Phase 95 update public entrypoints beyond the locked parity roots?** [VERIFIED: 95-CONTEXT.md; README.md; docs/parity/production-claim-boundary.md; docs/parity/support-matrix.md; 95-03-PLAN.md]
   - What we know: D-04 locks the main parity roots to `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/catalog/p2p.md`, and `docs/parity/release-readiness.md`. [VERIFIED: 95-CONTEXT.md]
   - What's unclear: README, production-claim-boundary, and support-matrix still use v1.8-oriented inbound wording and may need a v1.9 qualifier. [VERIFIED: README.md; docs/parity/production-claim-boundary.md; docs/parity/support-matrix.md]
   - Resolution: Plan 95-03 includes README, production-claim-boundary, and support-matrix review/update work so public entrypoints cannot stay stale while the locked parity roots carry v1.9 evidence. [VERIFIED: 95-03-PLAN.md]

2. **RESOLVED: Should the Phase 95 checker require final GSD summary/verification artifacts?** [VERIFIED: .planning/REQUIREMENTS.md; 95-CONTEXT.md; 95-04-PLAN.md]
   - What we know: BOUND-06 names summaries, verification reports, and milestone audit artifacts, but those artifacts may not exist until later in Phase 95 execution. [VERIFIED: .planning/REQUIREMENTS.md; GSD lifecycle from init phase-op 95]
   - What's unclear: Requiring future GSD artifacts in `scripts/verify.sh` can make default verification fail before those artifacts are created. [VERIFIED: scripts/verify.sh; 95-CONTEXT.md]
   - Resolution: Plan 95-04 scopes the automated checker to stable release-boundary roots, while final GSD summary and verification artifacts remain manual lifecycle evidence produced after implementation. [VERIFIED: 95-04-PLAN.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Bun | Phase 95 checker and fixture test | yes | 1.3.9 pinned/installed | None needed; required by repo automation. [VERIFIED: .bun-version; bun --version; AGENTS.md] |
| Bash | `scripts/verify.sh` | yes | GNU bash 3.2.57 | None needed. [VERIFIED: bash --version; scripts/verify.sh] |
| Cargo | Focused Rust support test and full verifier | yes | 1.94.1 | None needed. [VERIFIED: cargo --version; rust-toolchain.toml] |
| Rustc | Rust support code compile/test | yes | 1.94.1 | None needed. [VERIFIED: rustc --version; rust-toolchain.toml] |
| Bazel | UAT command docs and full verifier smoke build | yes | 8.6.0 | None needed. [VERIFIED: bazel --version; .bazelversion; scripts/verify.sh] |
| cargo-llvm-cov | Full verifier coverage gate | yes | 0.8.5 | Use `bash scripts/verify.sh --fast` only for local iteration, not final proof. [VERIFIED: cargo llvm-cov --version; scripts/verify.sh; AGENTS.md] |
| Git | GSD/docs commit and verifier hook checks | yes | 2.53.0 | None needed. [VERIFIED: git --version; scripts/verify.sh] |

**Missing dependencies with no fallback:** None found for Phase 95 research and planned local verification. [VERIFIED: environment availability commands]

**Missing dependencies with fallback:** None found; `--fast` exists for local iteration but is not the final contract. [VERIFIED: scripts/verify.sh; AGENTS.md]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | Phase 95 should not add auth/session behavior; keep RPC credential material out of support evidence. [VERIFIED: 95-CONTEXT.md; packages/open-bitcoin-cli/src/operator/support/redaction.rs] |
| V3 Session Management | no | No session management surface is planned. [VERIFIED: 95-CONTEXT.md] |
| V4 Access Control | yes, documentation boundary only | Preserve public-network and production-service deferrals; do not promote default listener exposure. [VERIFIED: 95-CONTEXT.md; docs/operator/runtime-guide.md] |
| V5 Input Validation | yes | Use structured JSON parsing for parity roots and explicit allow/deny lists for claim and redaction checks. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts] |
| V6 Cryptography | no | Phase 95 should not add cryptographic behavior. [VERIFIED: 95-CONTEXT.md] |
| V7 Error Handling and Logging | yes | Failure messages should identify missing roots, forbidden claims, verifier drift, and redaction leaks without exposing sensitive values. [VERIFIED: scripts/check-phase94-dos-resource-governance.ts; packages/open-bitcoin-cli/src/operator/support/redaction.rs] |
| V14 Configuration | yes | Preserve explicit opt-in listener configuration and do not turn public endpoints into defaults. [VERIFIED: docs/operator/runtime-guide.md; 95-CONTEXT.md] |

### Known Threat Patterns for Phase 95

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Release overclaim promotes unsafe production use | Spoofing / Elevation of Privilege | Deterministic no-claim checker over curated public release/operator docs. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; 95-CONTEXT.md] |
| Support bundle leaks peer address, peer id, payload, permission, or credential material | Information Disclosure | Redact shared inbound status before support JSON/Markdown rendering and add resource-governance sanitizer if needed. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs; 95-CONTEXT.md] |
| Default verifier starts using public network or service-manager state | Repudiation / Denial of Service | Verify `scripts/verify.sh` executable text excludes public-network/service-manager/long-running commands. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts] |
| Requirement traceability drift hides unimplemented work | Tampering | Parse active requirements/roadmap/parity roots for exact 28-ID v1.9 mapping. [VERIFIED: .planning/REQUIREMENTS.md; .planning/ROADMAP.md; docs/parity/index.json] |

## Sources

### Primary (HIGH confidence)

- `AGENTS.md` - repo-local verification, Bun, Rust, parity, UAT, and GSD workflow directives. [VERIFIED: AGENTS.md]
- `AGENTS.bright-builds.md` and standards pages - workflow, verification, testing, code shape, Rust, and TypeScript rules. [VERIFIED: AGENTS.bright-builds.md; standards/core/verification.md; standards/core/testing.md; standards/core/code-shape.md; standards/languages/rust.md; standards/languages/typescript-javascript.md]
- `.planning/phases/95-network-participation-evidence-and-release-boundary/95-CONTEXT.md` - locked Phase 95 decisions and deferred scope. [VERIFIED: 95-CONTEXT.md]
- `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md` - BOUND requirements, Phase 95 success criteria, v1.9 28/28 traceability, and current project state. [VERIFIED: .planning/REQUIREMENTS.md; .planning/ROADMAP.md; .planning/STATE.md]
- `scripts/check-phase88-deterministic-claim-guardrails.ts` and `.test.ts` - broad release no-claim checker and fixture pattern. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase88-deterministic-claim-guardrails.test.ts]
- `scripts/check-phase94-dos-resource-governance.ts` and `.test.ts` - current v1.9 checker pattern, verifier-order checks, and no-claim scanning. [VERIFIED: scripts/check-phase94-dos-resource-governance.ts; scripts/check-phase94-dos-resource-governance.test.ts]
- `scripts/verify.sh` - default verification contract and exact Phase 95 insertion point. [VERIFIED: scripts/verify.sh]
- `docs/parity/catalog/p2p.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/release-readiness.md`, `docs/parity/production-claim-boundary.md` - parity/release roots and current v1.9/v1.8 boundary wording. [VERIFIED: listed files]
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md` - operator UAT commands and shared status/support evidence contracts. [VERIFIED: listed files]
- `packages/open-bitcoin-cli/src/operator/support/tests.rs`, `packages/open-bitcoin-cli/src/operator/support/redaction.rs`, `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs`, `packages/open-bitcoin-node/src/status/inbound.rs` - support redaction/rendering and inbound status contracts. [VERIFIED: listed files]

### Secondary (MEDIUM confidence)

- None; no external web sources were needed because the phase is repo-local and locked to existing tooling. [VERIFIED: 95-CONTEXT.md]

### Tertiary (LOW confidence)

- Assumption A1 about optional README/support-matrix/production-boundary file scope. [ASSUMED]
- Assumption A2 about research validity window. [ASSUMED]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - versions and tool choices are pinned in repo files and verified locally. [VERIFIED: .bun-version; rust-toolchain.toml; .bazelversion; local version commands]
- Architecture: HIGH - checker/test/verifier patterns are already implemented in Phase 88 and Phase 94. [VERIFIED: scripts/check-phase88-deterministic-claim-guardrails.ts; scripts/check-phase94-dos-resource-governance.ts]
- Pitfalls: HIGH for verifier/no-claim/support-redaction risks because they are visible in current files; MEDIUM for optional public-entrypoint doc scope because D-04 names a narrower locked root set. [VERIFIED: scripts/verify.sh; packages/open-bitcoin-cli/src/operator/support/redaction.rs; 95-CONTEXT.md]

**Research date:** 2026-06-27 [VERIFIED: environment_context current_date]
**Valid until:** 2026-07-27 for repo-local patterns, or sooner if Phase 95 planning changes the locked checker/file scope. [ASSUMED]
