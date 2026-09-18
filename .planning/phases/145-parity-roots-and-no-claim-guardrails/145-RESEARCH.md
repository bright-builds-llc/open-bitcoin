# Phase 145: Parity Roots and No-Claim Guardrails - Research

**Researched:** 2026-09-18
**Domain:** Closeout guardrails — Phase 139–144 parity-root backfill, last-gate Bun no-claim checker, UAT package, leftover-Pending flip
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Parity surface ownership

- **D-01:** Phase 145 canonically owns only CSVFY-01 and CSVFY-02. Phases
  139 through 144 remain the exactly-once owners of CACHE, FLUSH, COIN, MGR,
  HAVL, and CSOBS even when the Phase 145 checker aggregates their evidence.
- **D-02:** Do not collapse v2.3 evidence into one catalog entry.
  `docs/parity/index.json` remains the machine-readable root.
  `docs/parity/checklist.md`, `docs/parity/catalog/chainstate.md`,
  `docs/parity/catalog/p2p.md` (serve-path), and
  `docs/parity/release-readiness.md` remain the human review roots. Do not
  create a competing closeout manifest.
- **D-03:** Backfill missing machine-readable owners before closeout: add
  distinct `docs/parity/index.json` / checklist surfaces for Phases 139–144
  if they are still absent, then add one closeout surface
  `v2-3-parity-roots-and-no-claim-guardrails` that owns only CSVFY-01 and
  CSVFY-02. Prefer index/checklist rows over new catalog pages when
  `catalog/chainstate.md` already has the prose home.
- **D-04:** Promote completed `in_progress` v2.3 surfaces to `done` after
  their evidence roots are current. Preserve existing earlier-milestone
  `done` surfaces. Planning artifacts may point to canonical parity roots
  but must not become parallel sources of requirement ownership.
- **D-05:** Review `docs/parity/source-breadcrumbs.json` semantically, not
  only mechanically. Tighten or split broad or explicit-`none` groups when
  they hide a defensible v2.3 Knots coins/flush/manager/serve-path anchor,
  while preserving `none` only where no honest source anchor exists.

### Knots anchors and intentional differences

- **D-06:** Parity roots must cite these pinned Knots anchors, or document
  an intentional difference next to the claim:
  `packages/bitcoin-knots/src/coins.h`,
  `packages/bitcoin-knots/src/coins.cpp`,
  `packages/bitcoin-knots/src/validation.cpp` (FlushStateToDisk /
  GetCoinsCacheSizeState),
  `packages/bitcoin-knots/src/node/chainstate.cpp` (manager / CanFlush),
  `packages/bitcoin-knots/src/node/blockstorage.cpp` (HaveBlockData /
  serve-path). Add the matching Knots tests when they are the honest
  observable-behavior anchors.
- **D-07:** Record already-locked intentional differences instead of
  implying line-by-line source parity: Fjall per-outpoint coins instead of
  LevelDB `chainstate/`; first-party overlay occupancy instead of C++
  allocator / LevelDB SizeEstimate; leftover snapshot blobs are
  non-authoritative after the schema 1→2 one-way migration; `Pruned` stays
  reserved and is not emitted on production paths; single active
  chainstate, not assumeutxo dual-chainstate; disk-space probe may stay
  fail-open at `u64::MAX` where the node crate forbids unsafe `statvfs`.
- **D-08:** `docs/parity/catalog/chainstate.md` currently still describes
  the Phase 4 snapshot-style UTXO engine. Refresh it so disk-backed coins,
  cache-flush, manager restart, and honest availability are the current
  v2.3 claim, while the older snapshot-engine history remains labeled as
  historical rather than live coin truth.

### Last-gate no-claim checker

- **D-09:** Add a new Phase 145 Bun/TypeScript checker pair as the last
  `check-phase*` no-claim gate. Follow the Phase 138 export shape:
  `checkPhase145...(maybeRepoRoot?)` returning `string[]` failures, plus
  fixture mutation tests and an `OPEN_BITCOIN_PHASE145_REPO_ROOT` override.
- **D-10:** Wire the new pair into `scripts/verify.sh` immediately after
  Phase 144, updating the ordering comment, `VERIFY_COMMAND_ORDER` heredoc,
  and live `run_step` chain together. Phase 117 remains the v2.1 BOUND
  gate, Phase 138 remains the v2.2 closeout gate, and Phase 145 becomes the
  v2.3 closeout gate.
- **D-11:** Use a curated current claim-bearing corpus (README, runtime
  guide, parity checklist/catalog/release-readiness/production-claim-boundary/
  support-matrix, operator docs). Do not scan historical `.planning/` prose
  or milestone archives as a blocking surface.
- **D-12:** Keep Phase 138's required v2.2 D-21 sentence intact in its
  required files. Add the v2.3 scoped sentence without removing the v2.2
  sentence. Do not globally allow unscoped `historical serving`, `archive`,
  or `prune`. Keep Phase 117's archived v2.1 "package relay remains
  deferred" contract intact the same way 138 did.
- **D-13:** The aggregate checker validates all 15 v2.3 requirements
  exactly once, required Phase 139–145 parity surfaces, concrete Knots
  anchors from D-06, relevant breadcrumb groups, exact repo-local UAT
  commands, both visible and executable verifier ordering, and that
  historical `.planning/phases/` directories remain tracked.

### Allowed scoped claim vs denied overclaims

- **D-14:** The allowed scoped claim sentence, copied verbatim into
  `README.md` and `docs/operator/runtime-guide.md`, is: disk-backed
  per-outpoint coins, typed cache-flush policy, fuller chainstate-manager
  behavior for the single active chainstate, and honest stored-block
  availability that serves or reports a stored block only when the payload
  bytes are present.
- **D-15:** Companion allowed wording includes sanitized operator
  flush/recovery/cache-size/have-bytes evidence, leftover snapshot
  non-authority after one-way migration, restart from durable coins
  best-block, and hermetic default verification.
- **D-16:** The checker must reject positive claims for prune-mode product
  behavior, archive-node or production-scale historical serving,
  assumeutxo/assumevalid/IBD snapshot shortcuts, compact-filter or BIP37
  serving, public serving or relay by default, public-network CI or
  release-blocking historical-serving/long-chain flush runs, production
  full-node readiness, production service operation, production-funds
  wallet safety, LevelDB `chainstate/` live import/export, and automatic
  destructive reindex or coins repair.
- **D-17:** Explicit deferred, unsupported, future-gated, no-claim, and
  opt-in-UAT wording remains valid. Negative fixtures must prove both
  rejection of overclaims and acceptance of scoped or deferred wording.

### UAT, default verification, and historical phase dirs

- **D-18:** Create
  `.planning/phases/145-parity-roots-and-no-claim-guardrails/145-UAT.md`
  as the committed UAT package. Required tests stay deterministic.
  Public-network review may be recorded as not run and must never become
  a default, CI, or release gate.
- **D-19:** UAT guidance must provide copy-pasteable repo-local Cargo and
  Bazel forms:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`, plus
  the matching `open-bitcoin-cli` / `open-bitcoind` forms where those
  binaries are the operator surface. Do not rely only on the installed
  `open-bitcoin` alias.
- **D-20:** `bash scripts/verify.sh` remains the final deterministic
  non-regression contract. Focused checker tests and commands are
  iteration aids, not alternate release criteria. Do not add
  public-network, wall-clock soak, service-manager, or production-deployment
  gates to the default verifier.
- **D-21:** Historical `.planning/phases/` directories remain tracked
  because repository verifiers consume selected evidence. This phase must
  not delete, gitignore, or archive those directories. The last-gate
  checker must fail if a verifier-referenced historical phase path used by
  `scripts/verify.sh` or existing `check-phase*` scripts is missing.

### Closeout metadata without milestone archive

- **D-22:** Flip leftover Pending v2.3 requirement rows (including
  CACHE-01, MGR-03, any still-Pending FLUSH/COIN/HAVL rows, and CSVFY-01
  through CSVFY-02) only after the checker can name their evidence. Do not
  leave leftover Pending rows that would force a later reconciliation
  phase.
- **D-23:** Reconcile ROADMAP, REQUIREMENTS, PROJECT, and STATE to agree
  that v2.3 implementation and closeout evidence are complete. Use
  `gsd-tools.cjs` mutation commands for STATE/ROADMAP updates rather than
  direct edits where the CLI owns those changes.
- **D-24:** Phase 145 may close current-milestone traceability and
  release-boundary evidence, but it must not archive v2.3 or invent a
  milestone-audit workflow; milestone completion remains
  `/gsd-complete-milestone v2.3` after this phase passes.
- **D-25:** Refresh `README.md` with a concise current-state description
  and links to the canonical evidence roots. Contributor-facing copy stays
  quiet, factual, and evidence-focused. Use
  `docs/parity/release-readiness.md` as the v2.3 release-review handoff.
  Do not introduce a disconnected changelog tree.
- **D-26:** Do not require runtime coins/flush/manager/serve behavior
  changes unless a named evidence gap has no honest existing proof. Prefer
  catalog, checker, breadcrumb, and claim-copy fixes. Preserve Phase 144's
  shared `chainstate_durability` contract; do not re-derive those facts in
  this phase.

### Claude's Discretion

The planner may choose exact checker helper names, fixture organization,
paragraph-classification implementation, the smallest honest breadcrumb or
catalog-owner splits, exact doc section placement, and whether optional
UAT items are recorded as pending or not run. Prefer small pure TypeScript
helpers with focused tests, targeted doc edits, and the existing Phase
106/117/138 patterns. Do not spend discretion on runtime behavior changes
or broader claims.

### Folded Todos

None — `todo match-phase 145` returned no matches.

### Deferred Ideas (OUT OF SCOPE)

- Milestone archival — `/gsd-complete-milestone v2.3` after this phase
  passes, not during Phase 145.
- Prune-mode product behavior, archive-node or production-scale historical
  serving, assumeutxo/assumevalid/IBD shortcuts, compact-filter or BIP37
  serving, public serving or relay defaults, public-network CI, production
  full-node readiness, production-funds wallet use, LevelDB chainstate
  import/export, automatic destructive reindex or coins repair — FUT-18
  through FUT-26.
- `getblock` product surface — still out of v2.3.
- Hosted web dashboard / GUI — project non-goal.

None — discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CSVFY-01 | Parity roots cite pinned Knots coins, flush, manager, and serve-path anchors, or document intentional differences. | Backfill six Phase 139–144 `index.json` / `checklist.md` surfaces plus closeout `v2-3-parity-roots-and-no-claim-guardrails`; refresh `catalog/chainstate.md` and the serve-path prose in `catalog/p2p.md`; cite D-06 files with honest Knots symbols (`CanFlushToDisk`, `CheckBlockDataAvailability`) and D-07 differences; semantically tighten breadcrumb groups `chainstate-engine`, `node-coins-adapter`, `node-chainstate-adapter`, `node-network-chainstate-durability-evidence`. |
| CSVFY-02 | Deterministic no-claim guardrails keep prune/archive modes, assumeutxo, compact filters, public defaults, and production readiness out of the v2.3 claim. | Copy the Phase 138 export/fixture/claim pattern; require the D-14 sentence on README + runtime-guide without removing the v2.2 D-21 sentence; deny D-16 overclaims on a curated corpus; wire after Phase 144; keep default `verify.sh` hermetic; fail if verifier-referenced historical `.planning/phases/` paths go missing. |
</phase_requirements>

## Summary

Phase 145 is a closeout and guardrail phase. The implementation already exists in Phases 139–144. What is missing is auditable machine-readable ownership, current catalog wording, a last-gate no-claim checker, a committed UAT package, leftover-Pending flips, and README / release-handoff copy that states the bounded v2.3 claim.

`docs/parity/index.json` and `docs/parity/checklist.md` contain no v2.3 surfaces today. `docs/parity/catalog/chainstate.md` still describes the Phase 4 snapshot-style UTXO engine and lists disk-backed coins as a known gap. README's chainstate row still says disk-backed databases remain follow-up depth. REQUIREMENTS still has unchecked CACHE-01, MGR-03, CSVFY-01, and CSVFY-02 even though 139/140 verification reports already mark CACHE-01 and MGR-03 satisfied. ROADMAP's coverage table is more stale than REQUIREMENTS.

**Primary recommendation:** Copy Phase 138's four-plan closeout shape. Backfill six exactly-once Phase 139–144 surfaces and one CSVFY closeout surface, add `scripts/check-phase145-parity-uat-release-boundary/` as the v2.3 last-gate checker, wire it immediately after Phase 144 (not after Phase 138), then flip leftover Pending rows only after that checker can name their evidence.

## Project Constraints (from .cursor/rules/)

`.cursor/rules/` does not exist in this repository. [VERIFIED: glob]

Repo-local constraints that the planner must honor come from `AGENTS.md` Repo-Local Guidance, `AGENTS.bright-builds.md`, `standards/core/verification.md`, `standards/languages/typescript-javascript.md`, and `standards-overrides.md` (no active exceptions):

- Use `bash scripts/verify.sh` as the repo-native verification contract. [CITED: AGENTS.md]
- UAT commands must use repo-local Cargo and Bazel forms, not only the installed `open-bitcoin` alias. [CITED: AGENTS.md]
- Bun/TypeScript owns new checker logic; do not add Python. [CITED: standards/languages/typescript-javascript.md]
- New first-party Rust source or test files need parity breadcrumbs; TypeScript checkers do not. [CITED: AGENTS.md]
- Prefer small pure functions, `maybe` names for nullish values, and Arrange/Act/Assert tests. [CITED: AGENTS.bright-builds.md]
- Treat files over ~628 lines as a split trigger. Follow Phase 138's directory-of-helpers, not Phase 144's 495-line single file. [CITED: AGENTS.bright-builds.md]
- Historical `.planning/phases/` directories stay tracked. [CITED: AGENTS.md / STATE.md]
- `standards-overrides.md` has no active exceptions. [VERIFIED: standards-overrides.md]

## Standard Stack

This phase adds no runtime libraries. Use the existing Bun/TypeScript checker toolchain.

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Bun | pin `.bun-version` `1.3.9`; local binary `1.4.2` | Run checkers and `bun:test` | Repo-owned automation runtime; no `package.json` / `bun install` [VERIFIED: `.bun-version`, `bun --version`] |
| TypeScript | Bun-native | Pure checker helpers | Existing `check-phase*` pattern [VERIFIED: `scripts/check-phase138-parity-uat-release-boundary.ts`] |
| Bitcoin Knots | `29.3.knots20260210` | Behavioral anchors | Pinned submodule under `packages/bitcoin-knots/` [CITED: PROJECT.md] |
| Rust / Cargo | `1.94.1` | Named UAT / evidence commands only | No new Rust production path [VERIFIED: `cargo --version`] |
| Bazel | `8.6.0` local | Twin UAT command forms | Required by D-19 [VERIFIED: `bazel --version`] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `node:fs` / `node:path` | Bun built-in | Corpus load, fixture write, path-escape checks | Checker and fixtures only |
| `bun:test` | Bun built-in | Mutation tests | Checker test file |
| `gsd-tools.cjs` | local GSD CLI | `requirements mark-complete`, `state update` / `state patch`, `roadmap update-plan-progress` | Final metadata reconciliation [VERIFIED: gsd-tools usage] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Phase 138 directory checker | Phase 144 single-file checker | 144 is 495 lines and already near the 628-line trigger; 138 already has claims/verifier/fixtures split. Use 138. |
| Place 145 after 138 as file-final `check-phase*` | Place 145 after 144 | File-final placement breaks Phase 138, 124, 129, 130, 131, and current-documentation-reconciliation last-gate / contiguous-sequence assertions. Honor D-10 literally. |
| New `catalog/chainstate-durability.md` | Refresh `catalog/chainstate.md` | D-03 forbids a competing catalog page when chainstate.md is already the prose home. |
| Scan all `.planning/` for overclaims | Curated current-doc corpus | D-11 forbids history-wide scans. |

**Installation:** none. Do not add npm packages.

**Version verification:** Bun pin `1.3.9` (2026 repo pin); local Bun `1.4.2`; Cargo `1.94.1`; Bazel `8.6.0`. [VERIFIED: 2026-09-18 shell]

## Architecture Patterns

### Recommended Project Structure

```
scripts/check-phase145-parity-uat-release-boundary.ts
scripts/check-phase145-parity-uat-release-boundary.test.ts
scripts/check-phase145-parity-uat-release-boundary/
├── constants.ts      # surfaces, D-14 sentence, deny list, UAT commands, Knots anchors
├── checks.ts         # checkPhase145ParityUatReleaseBoundary(maybeRepoRoot?)
├── claims.ts         # paragraph/clause classification, D-14 required, D-16 deny
├── verifier.ts       # verify.sh visible + executable order, historical phase-dir presence
└── test-fixtures.ts  # temp fixture tree + mutations
docs/parity/index.json
docs/parity/checklist.md
docs/parity/catalog/chainstate.md
docs/parity/catalog/p2p.md
docs/parity/source-breadcrumbs.json
docs/parity/release-readiness.md
docs/parity/production-claim-boundary.md
docs/parity/support-matrix.md
README.md
docs/operator/runtime-guide.md
.planning/phases/145-parity-roots-and-no-claim-guardrails/145-UAT.md
```

### Pattern 1: Phase 138 last-gate export

**What:** Thin `#!/usr/bin/env bun` entry exports `checkPhase138ParityUatReleaseBoundary(maybeRepoRoot?)` returning `string[]`, then exits 1 on failures. [VERIFIED: `scripts/check-phase138-parity-uat-release-boundary.ts`]
**When to use:** Every new closeout checker.
**Example:**

```ts
// Source: scripts/check-phase138-parity-uat-release-boundary.ts
export { checkPhase145ParityUatReleaseBoundary } from "./check-phase145-parity-uat-release-boundary/checks.ts";
import { checkPhase145ParityUatReleaseBoundary } from "./check-phase145-parity-uat-release-boundary/checks.ts";

if (import.meta.main) {
  const failures = checkPhase145ParityUatReleaseBoundary();
  if (failures.length > 0) {
    console.error("Phase 145 parity UAT release boundary check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 145 parity UAT release boundary validated.");
}
```

Repo-root resolution must be `maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE145_REPO_ROOT ?? DEFAULT_REPO_ROOT`, and fixture loads must reject paths that escape the repo (`path.relative` must not start with `..`). [VERIFIED: Phase 138 `checks.ts` `isInsideRepo`]

### Pattern 2: Exactly-once surface ownership

**What:** Top-level `index.json` `surfaces[]` name+status rows plus `checklist.surfaces[]` id/requirements/status/upstream/evidence. Phase 138 requires exactly one owner per requirement ID. [VERIFIED: `checkSurfaceOwnership` in Phase 138 `checks.ts`]
**When to use:** Backfill and closeout rows.

Prescribed v2.3 surface map:

| Surface id / top-level name | Requirements | Phase owner |
|---|---|---|
| `v2-3-coins-view-cache-contract` | CACHE-01 | 139 |
| `v2-3-pure-flush-policy-and-typed-decisions` | FLUSH-01, MGR-03 | 140 |
| `v2-3-durable-fjall-coins-adapter` | COIN-01, CSOBS-03 | 141 |
| `v2-3-manager-flush-lifecycle-and-restart` | MGR-01, MGR-02, FLUSH-02 | 142 |
| `v2-3-honest-stored-block-availability` | HAVL-01, HAVL-02, HAVL-03 | 143 |
| `v2-3-operator-flush-availability-evidence` | CSOBS-01, CSOBS-02 | 144 |
| `v2-3-parity-roots-and-no-claim-guardrails` | CSVFY-01, CSVFY-02 | 145 |

Closeout `upstream.sources` must include the five D-06 files. Closeout `upstream.tests` should include `packages/bitcoin-knots/src/test/blockmanager_tests.cpp` for serve-path availability. [VERIFIED: Knots `blockmanager_tests.cpp` uses `CheckBlockDataAvailability`]

Copy the Phase 138 closeout schema: `id`, `title`, `status`, `requirements`, `evidence`, `rationale`, `upstream.sources`, `upstream.tests`, `known_gaps`, `suspected_unknowns`. [VERIFIED: `docs/parity/index.json` `v2-2-parity-uat-release-boundary`]

### Pattern 3: Curated claim corpus + paragraph clauses

**What:** Phase 138 splits markdown into paragraphs and table cells, then requires a positive-claim verb plus a denied topic, unless a no-claim marker or table status of `deferred` / `unsupported` / `not run` is present. [VERIFIED: `scripts/check-phase138-parity-uat-release-boundary/claims.ts`]
**When to use:** CSVFY-02 overclaim rejection.

Required D-14 sentence, verbatim, in `README.md` and `docs/operator/runtime-guide.md`:

> disk-backed per-outpoint coins, typed cache-flush policy, fuller chainstate-manager behavior for the single active chainstate, and honest stored-block availability that serves or reports a stored block only when the payload bytes are present.

Keep the v2.2 D-21 sentence intact in those same files:

> bounded local-package APIs, same-peer 1P1C assembly over ordinary transaction messages, ordinary transaction fanout, and initial-broadcast-retry of locally submitted unbroadcast members

[VERIFIED: README status block already contains the v2.2 sentence; Phase 138 `constants.ts` `D21_SENTENCE` / `D21_REQUIRED_FILES`]

### Pattern 4: verify.sh triple update

**What:** Ordering comment, `VERIFY_COMMAND_ORDER` heredoc, and live `run_step` chain must stay in lockstep. [VERIFIED: `scripts/verify.sh` lines 291–305, 394–395, 560–561]
**When to use:** Wiring Phase 145.

Insert immediately after the Phase 144 pair and before Phase 121:

```
bun test scripts/check-phase144-operator-flush-availability-evidence.test.ts
bun run scripts/check-phase144-operator-flush-availability-evidence.ts
bun test scripts/check-phase145-parity-uat-release-boundary.test.ts
bun run scripts/check-phase145-parity-uat-release-boundary.ts
bun test scripts/check-phase121-block-relay-metrics-log-runtime.test.ts
```

Do **not** insert Phase 145 after Phase 138. Phase 138, Phase 124, Phase 129, Phase 130, Phase 131, and `check-current-documentation-reconciliation.ts` currently require Phase 138 to remain the last `check-phase*` command and require the contiguous `117 → 138 → reconciliation` sequence. [VERIFIED: Phase 138 `verifier.ts` `lastPhaseCommand !== PHASE138_CHECK`; reconciliation `VISIBLE_SEQUENCE`]

Interpret D-09 "last `check-phase*` no-claim gate" as the last **v2.3** no-claim gate. Phase 138 remains the file-final `check-phase*` and the v2.2 closeout gate. Update the `verify.sh` ordering comment to say that, so later phases do not re-learn the wrong placement.

Phase 145 `verifier.ts` must assert:

1. Visible and executable: Phase 144 test/check, then Phase 145 test/check.
2. Phase 117 BOUND and Phase 138 closeout commands remain present.
3. Default `run_step` lines do not contain `public-network`, `wall-clock`, `run-live-mainnet-smoke`, or `command-timings.ts` as a gate.
4. Do not assert that Phase 145 is the last `check-phase*` in the whole file.

Phase 144's current order check only requires 116 then 144; inserting 145 after 144 keeps that passing. [VERIFIED: `check-phase144-operator-flush-availability-evidence.ts` `checkVerifierOrder`]

### Anti-Patterns to Avoid

- **Collapsing v2.3 into one catalog page:** D-02 / D-03 forbid it. Refresh `chainstate.md` and add index/checklist rows.
- **Scanning `.planning/` or milestone archives for claims:** D-11. Historical prose will false-positive on prune/archive/assumeutxo discussion.
- **Rewriting `chainstate_durability`:** D-26. Name the existing contract; do not re-derive fields.
- **Archiving v2.3 from this phase:** D-24. Only `/gsd-complete-milestone v2.3` after the phase passes.
- **Making Phase 145 the file-final `check-phase*`:** breaks five historical last-gate contracts. Wire after 144.
- **Flipping CACHE-01 / MGR-03 before the checker names them:** recreates the Phase 129 leftover-Pending problem. Follow Phase 138 Plan 04.
- **Inventing a Knots `HaveBlockData` symbol:** that identifier is not in the pinned tree. Cite `blockstorage.cpp` plus `CheckBlockDataAvailability`. [VERIFIED: `rg HaveBlockData packages/bitcoin-knots` empty]
- **Leaving ROADMAP coverage Pending after REQUIREMENTS is `[x]`:** ROADMAP currently still lists FLUSH-01, COIN-01, CSOBS-03, and HAVL-01..03 as Pending even though REQUIREMENTS already checks them. [VERIFIED: REQUIREMENTS.md vs ROADMAP.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Closeout checker shape | New result type, CLI framework, or shared mega-harness | Phase 138 `string[]` export + directory helpers | Already mutation-tested and wired into verify.sh |
| Claim paragraph parsing | Regex-over-whole-file | Phase 138 `markdownParagraphs` / `claimClauses` / `NO_CLAIM_MARKERS` | Table cells and deferred rows need clause splits |
| Requirement activation | Direct REQUIREMENTS edit first | SUMMARY `requirements-completed` + lifecycle-valid VERIFICATION + `requirements mark-complete` | `check-active-milestone-verification-traceability` activates IDs from SUMMARY frontmatter [VERIFIED: `lifecycle.ts` `activatedRequirementIds`] |
| STATE / ROADMAP progress | Hand-edit progress bars | `gsd-tools.cjs` `state update` / `state patch` / `roadmap update-plan-progress` | D-23; CLI owns those fields [VERIFIED: gsd-tools usage] |
| Historical phase-dir presence | Recreate missing archives | Fail the checker if a referenced path is absent or untracked | D-21; verifiers consume those files |
| Runtime coins/flush/manager/serve fixes | New Rust behavior | Catalog, breadcrumb, checker, and claim-copy | D-26; evidence already exists in 139–144 |

**Key insight:** This phase's job is to make existing evidence auditable and to stop overclaims. Custom runtime work is out of scope unless a named evidence gap has no honest existing proof. No such runtime gap was found.

## Common Pitfalls

### Pitfall 1: File-final last-gate collision

**What goes wrong:** Inserting Phase 145 after Phase 138 makes `check-phase138`, `check-phase124`, `check-phase129`, `check-phase130`, `check-phase131`, and `check-current-documentation-reconciliation` fail. [VERIFIED: those scripts assert `must end with bun run scripts/check-phase138-parity-uat-release-boundary.ts` or a contiguous 117-138-reconciliation string]
**Why it happens:** Phase 138 copied Phase 117's "I am last" assertion and several later v2.2 checkers pinned that fact.
**How to avoid:** Wire 145 immediately after 144. Treat "last-gate" as last v2.3 no-claim gate. Do not retarget those historical last-gate assertions.
**Warning signs:** `must end with bun run scripts/check-phase138-parity-uat-release-boundary.ts` or `verifier visible reconciliation order must immediately follow Phase 117`.

### Pitfall 2: Leftover Pending without SUMMARY activation

**What goes wrong:** `requirements mark-complete CACHE-01` fails `check-active-milestone-verification-traceability` unless a lifecycle-valid SUMMARY lists CACHE-01 in `requirements-completed` and a lifecycle-valid VERIFICATION names the ID. [VERIFIED: 139/140 summaries left `requirements-completed: []` specifically to avoid this; 139/140 VERIFICATION.md already have `lifecycle_validated: true` and name the IDs]
**Why it happens:** Intermediate plans cannot activate IDs before phase verification exists; later closeout never came back to flip them.
**How to avoid:** In the final plan, put CACHE-01, MGR-03, CSVFY-01, and CSVFY-02 in `145-*-SUMMARY.md` `requirements-completed` (activation walks every phase corpus). Then run `requirements mark-complete`. Also reconcile ROADMAP rows that are already `[x]` in REQUIREMENTS (FLUSH-01, COIN-01, CSOBS-03, HAVL-01..03).
**Warning signs:** `activated requirement CACHE-01 is missing lifecycle-valid active-phase verification coverage` or hook failure on `requirements mark-complete`.

### Pitfall 3: Snapshot-era catalog left as live truth

**What goes wrong:** `catalog/chainstate.md` Coverage still lists "node-side in-memory snapshot persistence" and Known gaps still say "disk-backed coins databases, cache-flush policy, and assumeutxo flows". README chainstate notes say disk-backed databases remain follow-up depth. [VERIFIED: `docs/parity/catalog/chainstate.md` lines 18–19, 186–189; README line 69]
**Why it happens:** v2.3 never backfilled the Phase 4 page.
**How to avoid:** Add a labeled **Current v2.3 claim** section above historical Phase 4/70–79 text. Keep older snapshot-engine history, but mark it historical. Move leftover snapshot blobs to D-07 non-authority. Keep assumeutxo in gaps as deferred, not as a missing v2.3 deliverable.
**Warning signs:** "snapshot persistence that keeps storage outside the pure chainstate core" appearing as current coverage.

### Pitfall 4: HaveBlockData name vs pinned symbol

**What goes wrong:** A checker or catalog claims Knots exports `HaveBlockData`. That identifier is absent from `packages/bitcoin-knots`. The honest serve-path symbol is `BlockManager::CheckBlockDataAvailability`. Open Bitcoin `has_block` rustdoc already says "Analogous to Knots `HaveBlockData` / `CheckBlockDataAvailability`". [VERIFIED: `rg HaveBlockData` empty; `blockstorage.cpp:736`; `packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs` lines 12–13]
**Why it happens:** D-06 uses the discussion name from Phase 143/144 context.
**How to avoid:** Require the D-06 **file** `packages/bitcoin-knots/src/node/blockstorage.cpp` on the closeout surface. In catalog prose, write `HaveBlockData` only as the locked discussion name and immediately name `CheckBlockDataAvailability` as the pinned symbol. Cite `packages/bitcoin-knots/src/test/blockmanager_tests.cpp`. Same treatment for D-06 `CanFlush` → Knots `CanFlushToDisk` in `node/chainstate.cpp` and `validation.cpp`.
**Warning signs:** A required-anchor list that includes a non-existent `HaveBlockData` token as if it were a path or symbol.

### Pitfall 5: Removing the v2.2 D-21 sentence while refreshing README

**What goes wrong:** Phase 138 fails if README or runtime-guide lose the v2.2 sentence. Phase 117 still denies unscoped `package relay` on its claim corpus. [VERIFIED: Phase 138 test `fails_when_the_d21_sentence_is_missing`; Phase 117 `DANGEROUS_CLAIMS` includes `package relay`]
**Why it happens:** README current-state copy is one status block.
**How to avoid:** Add a v2.3 current-state sentence that includes the D-14 wording. Keep the existing v2.2 sentence. Keep v2.1 default-off block-serving history. Do not globally allow `historical serving`, `archive`, or `prune`.
**Warning signs:** Phase 138 checker `missing required D-21 sentence` or Phase 117 `forbidden positive Phase 117 claim: package relay`.

### Pitfall 6: Public-network or history-scan false positives

**What goes wrong:** Optional public-network UAT recorded as a gap, or `.planning/` historical prune/assumeutxo prose treated as a live overclaim.
**Why it happens:** Broad corpus + missing no-claim markers.
**How to avoid:** 145-UAT.md records public-network review as `not run` with "this does not create a gap". Claim corpus is the D-11 current-doc list only. `future`, `future-gated`, `deferred`, `not run`, and `without broadening` stay valid markers, same as Phase 138.
**Warning signs:** Default verify depending on live peers; checker failures on archived CONTEXT.md text.

### Pitfall 7: Competing closeout manifest

**What goes wrong:** A new `docs/parity/v2.3-closeout.md` or planning-owned requirement table becomes a second source of truth.
**Why it happens:** Convenience during inventory.
**How to avoid:** `index.json` + `checklist.md` + existing catalog pages + `release-readiness.md` only. Planning files may link to those roots.
**Warning signs:** A new markdown file that lists CACHE/FLUSH/COIN/MGR/HAVL/CSOBS IDs as if it owned them.

## Code Examples

Verified patterns from this repository:

### Closeout surface row

```json
{
  "id": "v2-3-parity-roots-and-no-claim-guardrails",
  "title": "v2.3 Parity Roots and No-Claim Guardrails",
  "status": "in_progress",
  "requirements": ["CSVFY-01", "CSVFY-02"],
  "evidence": [
    ".planning/phases/145-parity-roots-and-no-claim-guardrails/145-UAT.md",
    "scripts/check-phase145-parity-uat-release-boundary.ts",
    "docs/parity/release-readiness.md",
    "docs/parity/catalog/chainstate.md",
    "docs/parity/index.json"
  ],
  "upstream": {
    "sources": [
      "packages/bitcoin-knots/src/coins.h",
      "packages/bitcoin-knots/src/coins.cpp",
      "packages/bitcoin-knots/src/validation.cpp",
      "packages/bitcoin-knots/src/node/chainstate.cpp",
      "packages/bitcoin-knots/src/node/blockstorage.cpp"
    ],
    "tests": [
      "packages/bitcoin-knots/src/test/blockmanager_tests.cpp"
    ]
  }
}
```

Source shape: `docs/parity/index.json` `v2-2-parity-uat-release-boundary`. [VERIFIED]

### Required UAT commands to pin

Copy Phase 144's already-published operator twins, then add the Phase 145 pair:

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format human
bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-chainstate-durability-support
bazel run //packages/open-bitcoin-cli:open_bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-chainstate-durability-support
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -datadir=/tmp/open-bitcoin-preview
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview getblockchaininfo
bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview getblockchaininfo
bun test scripts/check-phase145-parity-uat-release-boundary.test.ts
bun run scripts/check-phase145-parity-uat-release-boundary.ts
bash scripts/verify.sh
```

[VERIFIED: runtime-guide Phase 144 section; README operator preview already publishes the daemon / `open-bitcoin-cli` forms]

### Historical phase-dir check

```ts
// Source: Phase 145 verifier recommendation, following Phase 138 isInsideRepo
function checkHistoricalPhasePaths(repoRoot: string, corpusTexts: string[], failures: string[]): void {
  const referenced = new Set<string>();
  const pattern = /\.planning\/phases\/[A-Za-z0-9._-]+/g;
  for (const text of corpusTexts) {
    for (const match of text.matchAll(pattern)) referenced.add(match[0]);
  }
  for (const relative of referenced) {
    const absolute = path.resolve(repoRoot, relative);
    if (!isInsideRepo(repoRoot, absolute) || !existsSync(absolute)) {
      failures.push(`missing verifier-referenced historical phase path ${relative}`);
    }
  }
}
```

Use `scripts/verify.sh` plus the `scripts/check-phase*.ts` corpus as the reference set. Do not require every historical phase directory in the tree — only paths those verifiers already name. [CITED: CONTEXT D-21]

### Denied-overclaim fixture

```ts
// Source: scripts/check-phase138-parity-uat-release-boundary.test.ts
test("fails_when_a_positive_prune_mode_claim_is_added", () => {
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin provides prune-mode product behavior.");
    },
  });
  const failures = checkPhase145ParityUatReleaseBoundary(root).join("\n");
  expect(failures).toContain("prune-mode");
});

test("accepts_deferred_prune_wording", () => {
  const root = createFixture({
    maybeMutate(files) {
      append(files, "README.md", "Open Bitcoin does not add prune-mode product behavior.");
    },
  });
  expect(checkPhase145ParityUatReleaseBoundary(root)).toEqual([]);
});
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Phase 4 snapshot UTXO engine as live catalog claim | Disk-backed per-outpoint Fjall coins + leftover snapshot non-authority | Phases 141–142 (2026-09) | `chainstate.md` must stop presenting snapshot blobs as live UTXO truth |
| Phase 106/117/138 file-final closeout gate | New closeout after previous closeout; older gate stays present | 117 then 138 (2026-08) | 145 is the v2.3 closeout but must not steal 138's file-final pin |
| Leftover Pending rows left for a later reconciliation phase | Flip in the last-gate plan after the checker names evidence | Phase 138 Plan 04 / Phase 129 lesson | D-22 exists to avoid another Phase 129 |
| Mechanical breadcrumb presence | Semantic review of coins/flush/manager/serve-path groups | D-05 | `node-storage-contract` stays `none` for Fjall/Open-Bitcoin-only files; do not force a fake Knots cite |

**Deprecated/outdated:**

- Treating leftover snapshot persist as CACHE-01 completion. Phase 139 verification already forbids that. [CITED: 139-VERIFICATION.md]
- README "start future work with `/gsd-new-milestone`" as current-state copy. v2.3 is the active milestone. Refresh current-state; do not invent a changelog tree. [VERIFIED: README lines 21–24]
- `catalog/chainstate.md` Known gap "disk-backed coins databases, cache-flush policy" as a live missing deliverable. Those are shipped; assumeutxo stays deferred (FUT-21).

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| — | — | — | — |

**If this table is empty:** All claims in this research were verified or cited — no user confirmation needed.

D-06's discussion name `HaveBlockData` is not a pinned-tree symbol. That is a verified absence, not an assumption. The planner should cite the locked file and the honest symbol `CheckBlockDataAvailability`.

## Open Questions

1. **Exact top-level surface name strings for Phases 139–144**
   - What we know: D-03 requires distinct index/checklist surfaces; the closeout id is locked as `v2-3-parity-roots-and-no-claim-guardrails`.
   - What's unclear: Discretion allows the smallest honest names.
   - Recommendation: Use the table in Pattern 2. Do not reuse the Phase 135-style human title that diverges from the checklist id.

2. **Whether `node-storage-contract` should gain a HaveBlockData cite**
   - What we know: `fjall_store/blocks.rs` `has_block` is the payload-byte probe and currently sits in the `none` `node-storage-contract` group. D-05 says split when a defensible v2.3 serve-path anchor is hidden. Phase 143 recorded "node-storage-contract breadcrumbs stay none; HaveBlockData lives in has_block rustdoc". [CITED: STATE.md Phase 143 decision]
   - What's unclear: Split vs keep `none` plus rustdoc.
   - Recommendation: Smallest honest split: move `fjall_store/blocks.rs` and `fjall_store/tests/block_presence.rs` into a dedicated group citing `node/blockstorage.cpp` / `CheckBlockDataAvailability`. Leave remaining Fjall snapshot/lock files on `none`.

3. **How much ROADMAP coverage-table repair is in scope**
   - What we know: REQUIREMENTS already checks FLUSH-01, COIN-01, CSOBS-03, HAVL-01..03; ROADMAP still lists them Pending.
   - What's unclear: Whether Phase 145 should rewrite the whole ROADMAP coverage table or only leftover rows D-22 names.
   - Recommendation: Reconcile every v2.3 coverage-table Status cell to match REQUIREMENTS after the flip. D-23 requires ROADMAP and REQUIREMENTS to agree. Do not mark Phase 145 itself complete in that same task; execute-phase owns that after verification. [CITED: Phase 138-04-PLAN.md Task 2]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Bun | Checker + tests | ✓ | local 1.4.2 / pin 1.3.9 | — |
| Cargo | Named UAT command strings only | ✓ | 1.94.1 | — |
| Bazel | Named UAT command strings only | ✓ | 8.6.0 | — |
| Git | Historical phase-dir tracked check | ✓ | 2.53.0 | — |
| Node | `gsd-tools.cjs` metadata mutations | ✓ | v24.13.0 | — |
| Bitcoin Knots submodule | D-06 anchors | ✓ | files present under `packages/bitcoin-knots/src` | — |
| New npm packages | — | n/a | — | Do not add |

**Missing dependencies with no fallback:** none.

**Missing dependencies with fallback:** none.

Step 2.6 ran because the phase depends on Bun, git, and the Knots tree. No blocking installs.

## Security Domain

> Required because `security_enforcement` is not set to `false` in `.planning/config.json`.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | No new auth surface |
| V3 Session Management | no | No sessions |
| V4 Access Control | no | Docs/checker only |
| V5 Input Validation | yes | Resolve `maybeRepoRoot` / `OPEN_BITCOIN_PHASE145_REPO_ROOT` with `path.resolve` + `isInsideRepo`; reject `..` escapes [VERIFIED: Phase 138 `checks.ts`] |
| V6 Cryptography | no | Do not hand-roll crypto; no new crypto |

### Known Threat Patterns for closeout checkers

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Repo-root path escape via env override | Tampering | `isInsideRepo` before `readFileSync` |
| Overclaim spoof in README / release docs | Spoofing | Curated corpus + D-16 deny list + no-claim markers |
| False-Complete requirement rows | Tampering | Flip only after the checker names evidence; then require `[x]` / `done` |
| Milestone-archive elevation | Elevation of Privilege | D-24; do not run `/gsd-complete-milestone v2.3` |
| Historical-phase deletion | Repudiation | D-21 presence + tracked-path check |
| Identifier leakage in UAT examples | Information Disclosure | Reuse sanitized `/tmp/open-bitcoin-*` paths; no peer IDs, coin dumps, or credentials beyond the existing preview user |

## Prescribed Plan Shape

Use four plans, matching Phase 138:

1. **145-01 — Inventory and parity-root backfill.** Add the six Phase 139–144 surfaces as `in_progress`, add the closeout surface as `in_progress` owning only CSVFY-01/02, refresh `catalog/chainstate.md` (current v2.3 claim + historical snapshot section), add serve-path honesty to `catalog/p2p.md` beside the existing reserved-`Pruned` / missing-payload prose, and do the D-05 breadcrumb split for `has_block` if the planner takes that discretion. No requirement flips.
2. **145-02 — Last-gate checker and mutation fixtures.** Create the Phase 145 pair. Cover ownership, Knots anchors, breadcrumbs, D-14 sentence, D-16 overclaims, deferred-wording acceptance, UAT command presence, 144-then-145 verifier order, 117/138 presence, and missing historical phase paths. Do not wire `verify.sh` yet if that would fail on missing docs; prefer landing the checker against fixtures first, then live corpus in Plan 03.
3. **145-03 — Claim copy, UAT package, verify.sh wiring.** Write the D-14 sentence into README and runtime-guide without removing the v2.2 sentence. Add `145-UAT.md` with required deterministic tests and `not run` public-network review. Refresh `release-readiness.md` as the v2.3 handoff, plus production-claim-boundary / support-matrix no-claim rows. Wire verify.sh after Phase 144. Update the ordering comment.
4. **145-04 — Flip leftover Pending rows and reconcile metadata.** After `bun run scripts/check-phase145-parity-uat-release-boundary.ts` can name evidence: activate CACHE-01 / MGR-03 / CSVFY-01 / CSVFY-02 via SUMMARY `requirements-completed`, `requirements mark-complete`, promote all seven v2.3 surfaces to `done`, reconcile ROADMAP coverage (including already-implemented FLUSH/COIN/HAVL/CSOBS rows), and use `gsd-tools` for STATE/ROADMAP progress. Do not archive. Tighten the checker so leftover `- [ ] **CSVFY-01**` or an `in_progress` closeout surface fails.

Do not add a fifth plan for runtime coins/flush/manager/serve work. No such honest gap was found.

## Sources

### Primary (HIGH confidence)

- `.planning/phases/145-parity-roots-and-no-claim-guardrails/145-CONTEXT.md` — locked D-01..D-26
- `scripts/check-phase138-parity-uat-release-boundary/` — export, ownership, claims, verifier, fixtures
- `scripts/check-phase144-operator-flush-availability-evidence.ts` — current last v2.3 checker; 116-then-144 order
- `scripts/verify.sh` — Phase 144 at lines 394–395 / 560–561; Phase 138 at 429–430 / 595–596
- `docs/parity/index.json` — no v2.3 surfaces; `v2-2-parity-uat-release-boundary` schema
- `docs/parity/catalog/chainstate.md` — snapshot-era live wording
- `docs/parity/source-breadcrumbs.json` — `chainstate-engine`, `node-coins-adapter`, `node-chainstate-adapter`, `node-storage-contract` (`none`), `node-network-chainstate-durability-evidence`
- `packages/bitcoin-knots/src/{coins.h,coins.cpp,validation.cpp,node/chainstate.cpp,node/blockstorage.cpp}` — D-06 files present; `FlushStateToDisk`, `GetCoinsCacheSizeState`, `CanFlushToDisk`, `CheckBlockDataAvailability`
- `scripts/check-active-milestone-verification-traceability/lifecycle.ts` — SUMMARY activation rule
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/verification.md`, `standards/languages/typescript-javascript.md`

### Secondary (MEDIUM confidence)

- `.planning/phases/138-parity-adversarial-pressure-restart-and-release-guardrails/138-04-PLAN.md` — leftover-Pending flip after named evidence
- `.planning/phases/139-*/139-VERIFICATION.md` and `140-*/140-VERIFICATION.md` — CACHE-01 / MGR-03 already SATISFIED
- `packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs` — `has_block` analog comment
- `docs/parity/catalog/p2p.md` Phase 111 section — reserved `block_status_pruned`, missing-payload `NotFound`

### Tertiary (LOW confidence)

- None. Web search was not used; this is a repo-local closeout.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — existing Bun checker toolchain, versions probed
- Architecture: HIGH — Phase 138/144/verify.sh/index.json read in this session
- Pitfalls: HIGH — last-gate collisions, leftover-Pending activation, HaveBlockData symbol gap, and snapshot catalog drift are all file-backed

**Research date:** 2026-09-18
**Valid until:** 2026-10-18 (stable closeout pattern; re-check only if `verify.sh` last-gate pins change)
