# Phase 129: Integration Guardrails and Milestone Reconciliation - Research

**Researched:** 2026-07-20
**Domain:** Deterministic Bun/TypeScript repo checkers, GSD lifecycle stage machines, planning-artifact reconciliation
**Confidence:** HIGH (nearly all findings verified by direct codebase reads in this session)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Aggregate Integration Guard

- **D-01:** Add a new deterministic checker pair `scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts` plus `scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts` with fixture-based mutation coverage, following the Phase 127/128 checker conventions (exported `checkPhase129...(maybeRepoRoot?)` returning `string[]` failures, no network or process spawning inside the checker).
- **D-02:** Wire the new pair into `scripts/verify.sh` immediately after the Phase 128 test+check steps and before the Phase 117 test+check steps, updating the ordering comment, the `VERIFY_COMMAND_ORDER` heredoc, and the live `run_step` block together. Phase 117 remains the final `check-phase*` no-claim gate; Phase 129 must not absorb it.
- **D-03:** The aggregate guard covers all six repaired seams under one fail-closed surface: shared authoritative state, local `sendcmpct` emission, production announcement invocation, live per-peer header facts, transport emission, and post-write-only evidence. Reuse the exported Phase 127/128 check functions where practical instead of duplicating anchor logic; add cross-phase assertions those checkers cannot express phase-locally.
- **D-04:** The guard names and asserts the four repaired flows explicitly (FLOW-01 durable validated block → inbound serving; FLOW-02 handshake → bilateral compact negotiation → live header-aware announcement; FLOW-03 high-bandwidth decision → successful wire emission → post-write evidence; FLOW-04 authoritative sync runtime → RPC → CLI/dashboard/support) by requiring the existing Rust production-path test anchors (`packages/open-bitcoin-rpc/tests/black_box_parity.rs`, `packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs`, `packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs`, CLI operator tests) to stay present and connected.

#### Runtime Scope

- **D-05:** No new runtime features. Research confirms OBS-01 production wiring is complete after Phases 127/128: RPC `openbitcoinnetworkstatus` reads the single shared `ManagedNetworkHandle` via `authoritative_operator_snapshot()`, and all six OBS-01 facets (activation, eligibility, negotiation, reconstruction, fallback, in-flight) project from that authority. Phase 129 closes OBS-01 with deterministic guards and verification evidence, not re-plumbing.
- **D-06:** If flow verification uncovers an actual production truthfulness defect (for example the snapshot-time mixing of live `getblocktxn_in_flight` entries into `compact_timeout_count` in fallback counters proves misleading), fix it minimally and fail closed; otherwise document the semantics and leave runtime code untouched.
- **D-07:** New or touched first-party Rust source/test files require parity breadcrumbs mapped through `docs/parity/source-breadcrumbs.json`. TypeScript checkers need no breadcrumbs.

#### Phase 124 Stage Machine Evolution

- **D-08:** Evolve `scripts/check-phase124-post-audit-gap-planning.ts` (and its dispatch in `check-phase124-milestone-closeout-reconciliation.ts`) with an explicit archive-ready post-129 stage. Today the post-audit stage hard-requires `status: gaps_found`, `29/39`, the exact GAP/FLOW inventory, and Phase 129 pending; the new stage must accept Phase 129 checked with complete plans, coverage 39/39 with 0 pending, empty gap inventories, audit `status: passed`, and archive routing — while continuing to reject any intermediate inconsistent mixture.
- **D-09:** HARD-05 ownership stays on Phase 129 in the reconciled end-state; do not revert to the legacy Phase 124 ownership encoded in the old final-audit path. Update the Phase 124 fixtures/tests to cover the new archive-ready projection.
- **D-10:** Model the reconciliation as fail-closed distinct states (consistent with the Phase 126/128 closeout precedent): gaps-open post-audit state, Phase 129 verified pre-promotion, and reconciled archive-ready. No checker may accept a state where the audit says `passed` but requirement checkboxes, coverage counts, or roadmap status disagree.

#### Requirement Closure And Milestone Reconciliation

- **D-11:** Independent verification (gsd-verifier, lifecycle-valid `129-VERIFICATION.md`) explicitly closes all 10 reassigned requirements (BSRV-03, BSRV-04, CMP-04, CMP-05, OBS-01, OBS-02, OBS-03, OBS-04, BOUND-02, HARD-05) against production-path evidence, re-attesting the 7 already Complete via Phases 127/128 and newly closing OBS-01, BOUND-02, HARD-05.
- **D-12:** Rerun the milestone audit in place: update `.planning/v2.1-MILESTONE-AUDIT.md` frontmatter to `status: passed` with full scores (requirements 39/39, integration and flows at full), empty `gaps.*`, and a refreshed body/conclusion/next-action, following the v2.0 passing-audit frontmatter shape. Do not create a companion rerun file.
- **D-13:** Reconcile all planning artifacts to agree: flip the three Pending checkboxes and traceability rows in `.planning/REQUIREMENTS.md` (39/39 Complete), update the Phase 129 row/checkbox and Next Step in `.planning/ROADMAP.md`, refresh the Current Milestone status in `.planning/PROJECT.md`, refresh `.planning/STATE.md` frontmatter/position/todos, and repair the stale `.planning/MILESTONES.md` (currently still 33/39 with a `/gsd-plan-phase 128` next step).
- **D-14:** Route the reconciled end-state to `/gsd-complete-milestone v2.1`. The archival itself (moving ROADMAP/REQUIREMENTS/AUDIT under `.planning/milestones/`) stays outside Phase 129.

#### Boundary Preservation

- **D-15:** BOUND-02 closes by demonstrating the deterministic checkers now bind to production-path evidence (production callers, shared authority, post-write evidence) rather than passing without them, while the Phase 117 final no-claim gate and the bounded v2.1 claim vocabulary remain unchanged. Package relay, bloom/filter serving, compact filter serving, public-serving defaults, production readiness, and production-funds claims remain rejected from v2.1 artifacts.
- **D-16:** Default verification remains `bash scripts/verify.sh`: deterministic, local, and public-network-free. No public-network, soak, or service-manager gates may enter the default contract.

#### Folded Todos

No pending todos matched Phase 129. The STATE.md pending todo "Plan and execute Phase 129 before rerunning the v2.1 milestone audit" is this phase itself and resolves with it.

### Claude's Discretion

The planner may choose the exact stage names and fixture shapes for the Phase 124 evolution, whether the Phase 129 checker imports 127/128 check functions or re-asserts shared corpus anchors, the split between guard plans and reconciliation plans, and the minimal set of new Rust flow-test anchors (if any) needed beyond the existing corpus. Prefer the smallest guard surface that makes an inconsistent archive claim unrepresentable.

### Deferred Ideas (OUT OF SCOPE)

- The actual `/gsd-complete-milestone v2.1` archival run (move ROADMAP/REQUIREMENTS/AUDIT under `.planning/milestones/`) happens after Phase 129 routes there.
- Refactoring the 1,505-line `scripts/check-phase124-milestone-gap-closure.ts` remains non-blocking maintainability debt unless the stage evolution requires touching it anyway.
- Package relay, bloom/filter serving, compact filters, public relay defaults, public-network CI, archive-node claims, production full-node readiness, production-funds wallet use, migration apply mode, packaging, hosted services, and GUI work remain outside v2.1.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| OBS-01 | RPC and shared network status report block-serving activation, serving eligibility, compact negotiation, reconstruction, fallback, and in-flight compact-block state truthfully. | Verified wiring: `authoritative_operator_snapshot()` in `packages/open-bitcoin-rpc/src/context/inbound_status.rs` reads the shared `ManagedNetworkHandle`; `open_bitcoin_network_status` in `packages/open-bitcoin-rpc/src/dispatch/node.rs` projects it (already guarded field-by-field by the Phase 127 checker). Closure = deterministic guard + `129-VERIFICATION.md` evidence, plus the D-06 fallback-counter decision (see Pitfall 11). |
| BOUND-02 | Deterministic checkers prevent package relay, bloom/filter serving, compact filter serving, public-serving-default, production-readiness, and production-funds claims from entering v2.1 artifacts. | Closure = demonstrating checkers bind to production-path evidence: the new Phase 129 aggregate guard names all four flows and six seams (D-03/D-04), the Phase 117 gate stays final and unchanged (verified: `requireFinalPhaseChecker` in the Phase 124 dispatcher plus Phase 117's own verifier-scope checks), and verify.sh stays public-network-free (Phase 117 `FORBIDDEN_DEFAULT_GATES` scan of every `run_step`). |
| HARD-05 | Roadmap, requirement coverage, phase status, and the final milestone audit agree and route v2.1 directly to archival. | Closure = archive-ready stage in `check-phase124-post-audit-gap-planning.ts` (D-08), field-level reconciliation of six artifacts (exact end-state documented below), and in-place audit rerun to `status: passed` following the verified v2.0 frontmatter precedent. |
</phase_requirements>

## Summary

Phase 129 is a checkers-and-reconciliation phase with almost no runtime Rust work. The three hard problems are all sequencing problems, not implementation problems: (1) the new Phase 129 aggregate checker must slot into a verify.sh ordering contract that three other checkers already parse; (2) the Phase 124 post-audit stage machine currently *hard-fails* on every artifact state Phase 129 will pass through (plan counts, Next Step routes, checked requirements, audit scores), so the stage machine must be evolved in the same commits that mutate the planning artifacts — the pre-commit hook runs the full verifier on every commit; and (3) requirement promotion is gated by two independent checkers (`check-active-milestone-verification-traceability.ts` and the Phase 124 dispatcher) that enforce "verification before promotion" ordering.

I read every relevant checker end-to-end. The stage-detection logic is favorable: `isPostAuditGapPlanningStage` keys on `#### Phase 127/128/129` roadmap headings which remain present until archival, so the archive-ready stage lives *inside* `verifyPostAuditGapPlanningStage`; and the legacy final-audit path (which would demand HARD-05 owned by Phase 124) is unreachable as long as the `#### Phase 125:`/`#### Phase 126:` roadmap headings remain, because `isPhase124GapClosureStage` stays true and the dispatcher's `!gapClosureStage && finalStage` guard never fires. Do not delete those headings.

**Primary recommendation:** Split into (a) an aggregate-guard plan that adds the Phase 129 checker pair + verify.sh wiring while the repo is still in today's "Phase 128 complete / Phase 129 pending" state, and (b) reconciliation plan(s) that evolve the Phase 124 stage machine, run the verifier/audit rerun, and flip all artifacts — with every artifact mutation co-committed with its matching checker-state change so verify.sh stays green at every commit.

## Project Constraints (from AGENTS.md / AGENTS.bright-builds.md)

No `.cursor/rules/` directory exists; `AGENTS.md` (always-applied) plus `AGENTS.bright-builds.md` and the managed `standards/` pages govern. `standards-overrides.md` has no active overrides (verified: template placeholder only). Directives that materially constrain this phase:

- `bash scripts/verify.sh` is the pre-commit and release contract; the pre-commit hook (`.githooks/pre-commit`, verified) regenerates `docs/metrics/lines-of-code.md`, `git add`s it, and runs the full verifier on every commit.
- Bun is the canonical runtime for repo-owned automation; substantial checker logic stays in TypeScript (`standards/languages/typescript-javascript.md`). Prefix nullable bindings with `maybe`; early returns over nesting; files >~628 lines and functions >~161 lines are refactor triggers (relevant: `check-phase124-post-audit-gap-planning.ts` is already 690 lines — the archive-ready evolution may need a module split, mirroring how gap-closure logic lives in its own imported file).
- Parity breadcrumbs are required only for `packages/open-bitcoin-*/(src|tests)/**/*.rs` (verified regex in `scripts/check-parity-breadcrumbs.ts:113`); TypeScript checkers are exempt.
- Run ad-hoc Cargo/Bazel through `bun run scripts/command-timings.ts run --key <key> -- <command>`; do not overlap Cargo jobs on one target dir.
- Record intentional Knots behavior differences in `docs/parity/index.json` + companion docs.
- Treat stale LOC report updates as required freshness changes, not noise.
- After substantial workflow changes, check whether README files need updates (Phase 117's claim scanner also lints README wording — keep claim vocabulary bounded).
- Unit tests follow Arrange/Act/Assert with one concern per test (`standards/core/testing.md`), matching the existing `test.each` mutation-table style in checker tests.

## Standard Stack

No new dependencies. Everything needed already exists in-repo. [VERIFIED: codebase]

### Core

| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| Bun | 1.3.9 (verified locally) | Checker runtime + `bun test` mutation suites | Repo-canonical for automation; all `check-phase*` pairs use it |
| TypeScript (Bun-native) | n/a | Checker implementation | Established `check-phaseN-*.ts` convention |
| `node:fs` / `node:path` | stdlib | Corpus loading, fixture temp dirs | Used by every existing checker; no network, no spawning |
| gsd-tools (`~/.cursor/get-shit-done/bin/gsd-tools.cjs`) | installed | `init phase-op`, `commit`, lifecycle validation | GSD workflow contract |

### Supporting (existing modules to reuse)

| Module | Purpose | When to Use |
|--------|---------|-------------|
| `scripts/check-phase127-authoritative-network-state-unification.ts` → `checkPhase127AuthoritativeNetworkStateUnification(maybeRepoRoot?)` | FLOW-01/FLOW-04 seam anchors (shared authority, durable serving, operator projection) | Import from the Phase 129 checker (D-03); see composition tradeoffs below |
| `scripts/check-phase128-production-compact-announcement-transport.ts` → `checkPhase128ProductionCompactAnnouncementTransport(maybeRepoRoot?)` + exported `PHASE128_TARGET_FILES` | FLOW-02/FLOW-03 seam anchors (sendcmpct, durable trigger, live facts, transport writes, post-write evidence) | Same |
| `scripts/rust-source-invariants.ts` | Structural Rust assertions (`rustFunction`, `rustLetInitializers`, …) | Only if Phase 129 adds Rust-structural assertions beyond 127/128 reuse |
| `scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts` → `createFixture(tempRoots, options)` | Synthetic staged planning-corpus fixtures | Extend with a Phase 129 stage option for the stage-machine tests |
| `scripts/check-phase124-milestone-closeout-lifecycle.ts` / `check-phase124-milestone-gap-closure.ts` | Lifecycle-frontmatter and completed-phase-125/126 assertions the dispatcher already calls in the post-audit branch | Leave intact; the post-audit branch keeps calling them |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Importing 127/128 check functions | Re-asserting a small set of shared corpus anchors in the 129 checker | Import gives full seam coverage for free and stays in sync automatically, but the 129 mutation-test fixture must materialize the *union* of both target corpora plus the `open_bitcoind/` helper directory (see Pitfall 8). Re-assertion keeps the fixture small but duplicates anchors that can drift. Recommended: import both functions and pass the same `repoRoot`; build the fixture from the union file list. |
| Extending `check-phase124-post-audit-gap-planning.ts` in place | New sibling module (e.g. `check-phase124-archive-ready.ts`) imported by the dispatcher | The file is 690 lines; adding a full archive-ready stage will push it past the ~628-line refactor trigger. The gap-closure precedent (separate imported module) supports a sibling module. Either is acceptable; a sibling module is cleaner. |

**Installation:** none. No `npm`/`bun install` step exists in this repo (no `package.json`). [VERIFIED: AGENTS.md + repo root]

## Architecture Patterns

### Verified current state of the key surfaces

**verify.sh triple ordering contract** (verified at `scripts/verify.sh`):

1. **Ordering comment** — lines 295–309. Prose sentences like "Phase 123 is followed by Phase 124…". Several *older* checkers assert exact sentences from this comment (verified: `check-phase100/101/102/110/111` `requireContains(text, "Phase X is followed by Phase Y", …)`), but **no checker asserts a Phase 126/127/128 comment sentence** — the 126/127/128 checkers assert only the heredoc + run_step lines. So append a new sentence for Phase 129; never rewrite or delete existing sentences.
2. **`VERIFY_COMMAND_ORDER` heredoc** — Phase 128 lines at 412–413, Phase 117 lines at 414–415. Insert the Phase 129 test+check lines between line 413 (`bun run scripts/check-phase128-production-compact-announcement-transport.ts`) and line 414 (`bun test scripts/check-phase117-parity-uat-release-boundary.test.ts`).
3. **Live `run_step` block** — Phase 128 steps at lines 558–559, Phase 117 steps at 560–561. Insert the two Phase 129 `run_step` lines between 559 and 560.

Exact new lines (following the 127/128 label conventions):

```bash
# heredoc additions (between current lines 413 and 414)
bun test scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts
bun run scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts

# run_step additions (between current lines 559 and 560)
run_step "test Phase 129 integration guardrails and milestone reconciliation checker" bun test scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts
run_step "check Phase 129 integration guardrails and milestone reconciliation" bun run scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts
```

Constraints on those lines (all verified):
- `requireFinalPhaseChecker` (Phase 124 dispatcher, lines 501–509) requires the *last* line matching `bun (test|run) scripts/check-phase\d+` in both the heredoc and run_step text to be the Phase 117 check — the Phase 129 pair must precede Phase 117, which insertion between 128 and 117 satisfies.
- The Phase 124 dispatcher's `verifyVerifierOrder` asserts the subsequence [123 test/check, 124 test/check, active-traceability test/check, 117 test/check] and exact occurrence counts of the Phase 124 commands (2 each). Inserting Phase 129 lines does not disturb either.
- Phase 117's `checkVerifier` scans every logical `run_step` command for `FORBIDDEN_DEFAULT_GATES` tokens (`public-network`, `wall-clock`, `soak` outside `scripts/check-phase*`, `systemd`, …). The Phase 129 step labels/commands above contain none.
- Phase 127's and 128's own `checkVerifier` assert their subsequences ([126 check, 127 test, 127 check, 117 test] and [127 check, 128 test, 128 check, 117 test]) — both remain satisfied.

**Phase 127/128 checker convention** (verified): single exported `checkPhaseN…(maybeRepoRoot?: string): string[]`, `DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..")`, optional `OPEN_BITCOIN_PHASEN_REPO_ROOT` env fallback, `loadCorpus` over a `TARGET_FILES` const, pure string/structural assertions, self-scan for forbidden tokens built with string concatenation (`"fetch" + "("` etc.) so the checker doesn't match itself, and an `import.meta.main` CLI footer. Mutation tests clone the *live repo files* for each target into a temp dir, apply one `replace(...)` mutator per case, and assert the exact failure string (Arrange/Act/Assert with `test.each`).

Caveat for composition: `TARGET_FILES` is **not exported** from the Phase 127 checker (only Phase 128 exports `PHASE128_TARGET_FILES`), and the Phase 127 checker additionally reads every non-test `.rs` file under `packages/open-bitcoin-rpc/src/bin/open_bitcoind/` via `productionDaemonHelperSources()` beyond its target list. A Phase 129 fixture that composes both checkers must export/duplicate the 127 list and copy that helper directory (currently `inbound_metrics.rs`, `sync_seed.rs`, `runtime_control.rs`, and any peers — copy the whole directory minus `tests.rs`).

**Phase 124 dispatcher stage detection** (verified, `check-phase124-milestone-closeout-reconciliation.ts` lines 105–171):

```
postAuditGapPlanningStage = roadmap contains "#### Phase 127|128|129: <name>"   ← true now, true after reconciliation
gapClosureStage           = audit has "status: gaps_found" OR roadmap has "#### Phase 125:" OR "#### Phase 126:"  ← stays true via 125/126 headings even after audit flips to passed
dispatch: postAudit branch wins (if/else if); legacy final-audit path requires !gapClosureStage → unreachable
always-on: verifyNoClaimBoundary(corpus), verifyVerifierOrder(verify.sh, activeTraceabilityRequired)
post-audit branch also calls: verifyCompletedGapClosureLifecycleArtifacts (125/126 stay 4 plans + 4 summaries + verification) and verifyPhase124CloseoutLifecycle (124-VERIFICATION frontmatter pinned to lifecycle id 124-2026-07-16T20-19-53)
```

**Current post-audit assertions that must gain an archive-ready projection** (verified, `check-phase124-post-audit-gap-planning.ts`):

| Function | Today (Phase 128 complete, 129 pending) | Archive-ready end-state |
|---|---|---|
| `verifyRequirementProjection` | checked/Complete counts = 36; OBS-01/BOUND-02/HARD-05 must be **unchecked/Pending**; owners pinned 127/128/129 | counts = 39; all three checked/Complete; owners unchanged (HARD-05 stays Phase 129 per D-09) |
| `verifyCoverage` | REQUIREMENTS `Complete: 36` / ROADMAP `Satisfied: 36`, `Pending integration gap closure: 3` | `Complete: 39` / `Satisfied: 39`, `Pending integration gap closure: 0` |
| `verifyRoadmapTopology` | Phase 129 row `- [ ]` and section `**Plans:** 0 plans`; execution order `126 -> 127 -> 128 -> 129`; phase dirs exist | Phase 129 row `- [x]`, section `**Plans:** N/N plans complete`; order line unchanged |
| `verifyAudit` | hard-requires `status: gaps_found`, `requirements: "29/39"`, `phases: "17/17"`, `integration: "9/13"`, `flows: "7/11"`, and exactly one `- id: X` frontmatter entry for each of the 10 requirements + GAP-01..03 + FLOW-01..04 | `status: passed`, full scores, empty `gaps:` block (`requirements: []`, `integration: []`, `flows: []`), zero `- id:` gap entries |
| `verifyRouting` | roadmap `## Next Step\n\nRun \`/gsd-plan-phase 129\`.`; audit `## Next Action\n\nRun \`/gsd-plan-phase 127\``; ROADMAP+STATE contain `/gsd-plan-phase 129` | all routing surfaces carry `/gsd-complete-milestone v2.1` with stale `/gsd-plan-phase` / `/gsd-execute-phase` routes absent (mirror `verifyFinalRoute`/`verifyPromotedRoute` precedent); add MILESTONES.md and PROJECT.md to the archive-ready routing set (they are unguarded in today's `complete` stage — that is exactly how MILESTONES.md went stale) |
| (new) Phase 129 lifecycle | none | model on `verifyPhase127Lifecycle`: exact plan set, contiguous summary prefix, ≤1 verification with `status: passed`, `generated_by` chain (`gsd-discuss-phase` context → `gsd-plan-phase` plans → `gsd-execute-plan` summaries → `gsd-verifier` verification; verified against 127/128 artifact frontmatter), promotion only with verification present |

**Fail-closed stage modeling (D-10):** three externally distinguishable states, detected from artifacts, mirroring `Phase126CloseoutStage`:
1. **gaps-open post-audit** — today's assertions (all current behavior preserved bit-for-bit, including the intermediate Phase 129 planned/executing sub-states the plan adds as it goes);
2. **Phase 129 verified pre-promotion** — `129-VERIFICATION.md` exists with `status: passed`, but REQUIREMENTS still 36/39 and audit still `gaps_found`;
3. **reconciled archive-ready** — everything in the right column above, simultaneously. Any mixture (e.g. audit `passed` but a Pending checkbox, or 39/39 without lifecycle-valid verification) must fail.

**Requirement promotion ordering** (verified, `check-active-milestone-verification-traceability.ts`): flipping OBS-01/BOUND-02/HARD-05 to checked+Complete requires, at the same commit: (a) a lifecycle-valid Phase 129 `*-SUMMARY.md` whose frontmatter `requirements-completed:` lists those IDs (exactly one such field per summary; inline `[A, B]` or block list — both verified formats in 128 summaries), and (b) a lifecycle-valid `129-VERIFICATION.md` with `status: passed` **and `lifecycle_validated: true`** whose text contains each requirement ID token. Checklist state and traceability status must flip together (the checker rejects inconsistent pairs). Promotion therefore happens *after* the gsd-verifier runs — the Phase 126 precedent ("promote exactly the six requirements only after the independent verifier and lifecycle gates pass").

**Phase 117 interactions** (verified): `traceabilityStage()` returns `post-audit-gap-planning` whenever any `| X | Phase 127/128/129 |` row exists — true before and after reconciliation. In that stage it requires `| OBS-01 | Phase 129 |` and `| BOUND-02 | Phase 129 |` rows exactly once (status column not parsed — Pending→Complete flips are invisible to it). HARD-* requirements are outside Phase 117's requirement set entirely. Phase 117 never parses the milestone audit. No Phase 117 changes are needed; it must remain byte-identical as the final gate (D-15).

**Milestone audit rerun shape (D-12):** the v2.0 precedent frontmatter (verified) is:

```yaml
---
milestone: v2.1
milestone_name: Block Serving and Compact Block Relay Boundary
audited: <fresh ISO timestamp>
status: passed
scores:
  requirements: "39/39"
  phases: "20/20"        # active roadmap now lists 20 phases (110–129); the gaps_found audit predates 127–129 completion at "17/17"
  integration: "13/13"
  flows: "11/11"
gaps:
  requirements: []
  integration: []
  flows: []
tech_debt: [...]          # see Open Question 2
nyquist:
  enabled: false
  compliant_phases: []
  partial_phases: []
  missing_phases: []
  overall: skipped
---
```

Body: refreshed Result/Definition-of-Done/coverage tables/conclusion, `## Next Action` → `Run \`/gsd-complete-milestone v2.1\``, and no stale `/gsd-plan-phase` or `/gsd-execute-phase` routes anywhere in the file. Body prose is scanned by `verifyNoClaimBoundary` (CLAIM_TOPICS + positive-verb regex) — keep deferred-surface sentences in no-claim form ("remain deferred", "does not", table rows with `deferred` status).

### Field-level reconciled end-state per artifact (D-13)

| Artifact | Required end-state fields |
|---|---|
| `.planning/REQUIREMENTS.md` | 3 checkboxes `[x]` (OBS-01 line 49, BOUND-02 line 58, HARD-05 line 69); traceability rows → `Complete` (lines 141/147/155); coverage block → `Complete: 39`, `Pending integration gap closure: 0`; refresh the trailing `*Last updated:*` line |
| `.planning/ROADMAP.md` | Current Status prose; milestone bullet (line 34) → complete wording; Phase 129 row (line 61) → `- [x] … (completed <date>)`; Phase 129 section → `**Plans:** N/N plans complete` + plan list; progress table row 129 → `N/N | Complete | date`; coverage → `Satisfied: 39`, `Pending integration gap closure: 0`; `## Next Step` → `Run \`/gsd-complete-milestone v2.1\`.` Keep `#### Phase 125:`/`#### Phase 126:` headings and the `126 -> 127 -> 128 -> 129` order line intact |
| `.planning/PROJECT.md` | Current State paragraph (Next action → `/gsd-complete-milestone v2.1`); Current Milestone **Status:** paragraph → reconciled/archive-ready wording; preserve the two exact sentences the Phase 128 checker pins (`v2.1 does not imply public relay defaults, …` and `package relay, bloom/filter serving, … deferred`) |
| `.planning/STATE.md` | frontmatter `status`, `stopped_at`, `progress` (note pre-existing drift: `total_plans: 66` vs `completed_plans: 68` — reconcile to truthful counts including Phase 129 plans), Current Position, `Next action: Run \`/gsd-complete-milestone v2.1\`.`, resolve the "Plan and execute Phase 129…" pending todo, decisions log entry |
| `.planning/MILESTONES.md` | v2.1 section: 33/39 → 39/39, "Phases 110 through 127" → through 129, plan totals, audit projection ("three integration gaps" → passed rerun), `**What's next:** Run \`/gsd-complete-milestone v2.1\`.` (currently `/gsd-plan-phase 128` — the flagship stale artifact) |
| `.planning/v2.1-MILESTONE-AUDIT.md` | Full in-place rewrite per the frontmatter shape above; keep filename (in-place supersession precedent verified: "Milestone audits are superseded in place") |
| `docs/parity/index.json` | Only if wording changes: the Phase 127 checker requires the literal substrings `"Phase 128 retains"` and `"Phase 129 retains"` (verified at index.json lines 3022–3023) — do not reword those entries out of existence |

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| FLOW-01/04 seam assertions | New Rust-structural anchors for shared authority/projection | `checkPhase127AuthoritativeNetworkStateUnification(repoRoot)` import | 538 lines of structural assertions incl. `rust-source-invariants` parsing; re-deriving guarantees drift |
| FLOW-02/03 seam assertions | New sendcmpct/transport/post-write anchors | `checkPhase128ProductionCompactAnnouncementTransport(repoRoot)` import | Same; also exports `PHASE128_TARGET_FILES` for fixture building |
| Frontmatter/lifecycle parsing | New YAML parsing | Copy the established `extractFrontmatter`/`exactScalar` helpers already duplicated per checker (repo convention is per-file duplication, not a shared module — follow it unless the planner deliberately extracts one) | Exact regex semantics (single frontmatter block, exactly-one-key) are load-bearing for fail-closed behavior |
| Staged planning-corpus fixtures | A new fixture system for stage tests | Extend `createFixture` in `check-phase124-milestone-closeout-reconciliation.fixtures.ts` with `maybePhase129Stage` + archive-ready projections | It already models requirements/roadmap/state/audit/project/milestones per stage and creates phase dirs + lifecycle artifacts |
| Ordered-subsequence checks | New ordering logic | The `orderedLines` / `requireOrdered` / `visibleCommandOrder` helpers (copy per convention) | Consistent trim-exact-line matching semantics |
| Commits | raw `git commit` | `node "$HOME/.cursor/get-shit-done/bin/gsd-tools.cjs" commit "<msg>" --files ...` per GSD flow; hook runs verify automatically | Keeps lifecycle metadata and hook behavior consistent |

**Key insight:** in this repo the checkers *are* the product for this phase. The failure mode is not "missing library" but "two checkers disagreeing about one artifact for one commit" — every reuse above reduces the number of places that encode the same expectation.

## Common Pitfalls

### Pitfall 1: Every planning-artifact edit trips the current post-audit checker at commit time
**What goes wrong:** `check-phase124-post-audit-gap-planning.ts` currently hard-requires Phase 129 row `- [ ]`, section `**Plans:** 0 plans`, roadmap Next Step `Run \`/gsd-plan-phase 129\`.`, checked-count 36, audit `status: gaps_found`. The pre-commit hook runs the full verifier, so the *first* commit that adds `129-01-PLAN.md` roadmap counts, or later changes Next Step, fails verify.
**Why it happens:** the stage machine is deliberately fail-closed and was extended commit-by-commit during Phases 127/128 (verified precedent: `phase128LifecycleStage` accepts only the exact states 128 actually passed through: `0 plans` → `3/4 plans executed` → `4/4 plans complete`, plus the custom `PHASE128_EXECUTION_ROUTE` roadmap text).
**How to avoid:** every plan that mutates ROADMAP/REQUIREMENTS/STATE/audit must co-commit the matching stage-machine extension (checker + fixtures + test). Sequence each plan's tasks so checker acceptance lands in the same commit as (or immediately before) the artifact mutation. Decide up front which intermediate states Phase 129 will legally occupy (e.g. `planned` → `executing` → `verified pre-promotion` → `archive-ready`) and teach the checker exactly those.
**Warning signs:** local `bash scripts/verify.sh` failing with `P124 post-audit …` messages on an artifact-only commit.

### Pitfall 2: The archive-ready stage must live inside the post-audit branch, not beside it
**What goes wrong:** adding a new top-level dispatch branch keyed on `status: passed` would never be reached correctly: `isPostAuditGapPlanningStage` (roadmap `#### Phase 127/128/129` headings) stays true after reconciliation and the dispatcher checks it first.
**How to avoid:** detect archive-ready *within* `verifyPostAuditGapPlanningStage` (or a sibling module it calls), keyed on consistent artifact evidence (audit `status: passed` + Phase 129 lifecycle complete), and fail on mixtures. Keep `isPostAuditGapPlanningStage` itself unchanged.

### Pitfall 3: The legacy final-audit path re-activates if Phase 125/126 headings disappear
**What goes wrong:** `verifyFinalAudit` (dispatcher lines 148–159) runs when `!gapClosureStage && finalStage` (HARD-05 checked). It demands `phases: "15/15"`, `tech_debt: []`, `## Resolved Hardening Debt`, and (via `verifyRequirementOwnership` in the non-stage branch) HARD-05 owned by Phase 124 — all wrong for the reconciled end-state. It is currently unreachable only because `isPhase124GapClosureStage` sees `#### Phase 125:`/`#### Phase 126:` in the roadmap.
**How to avoid:** reconciliation must not remove the Phase 125/126 phase-detail headings from ROADMAP.md (they naturally remain until archival). Add a fixture/test asserting the reconciled corpus still dispatches to the post-audit branch. Optionally retire the dead legacy path while touching the dispatcher (planner discretion; D-09 forbids reverting HARD-05 ownership either way).

### Pitfall 4: The audit's `## Next Action` is pinned to `/gsd-plan-phase 127` today
**What goes wrong:** `verifyRouting` requires the audit to contain `## Next Action\n\nRun \`/gsd-plan-phase 127\`` *in all current sub-stages*. The audit rerun replaces this with the archive route — one more flip that must land in the same commit as the stage-machine change accepting it.
**Warning signs:** `P124 post-audit audit route is missing …` failures.

### Pitfall 5: MILESTONES.md is unguarded in the current stage — and that's the drift this phase must end
**What goes wrong:** at `phase128Stage === "complete"` the routing check covers only ROADMAP+STATE, which is why MILESTONES.md still says 33/39 and `/gsd-plan-phase 128` without failing verify.
**How to avoid:** include `.planning/MILESTONES.md` and `.planning/PROJECT.md` in the archive-ready routing/consistency assertions (route present, stale routes absent, count agreement), fulfilling the CONTEXT "guard against recurrence" intent.

### Pitfall 6: Promotion before verification breaks two checkers at once
**What goes wrong:** flipping REQUIREMENTS checkboxes before `129-VERIFICATION.md` exists fails `check-active-milestone-verification-traceability.ts` (no activation summary / no lifecycle-valid verification token coverage) and the evolved Phase 124 stage machine (D-10 mixture rejection).
**How to avoid:** strict order within the final plan: write summaries with `requirements-completed` → run gsd-verifier → only then flip checkboxes/traceability/coverage + audit + routing in the reconciliation commit(s). `129-VERIFICATION.md` frontmatter must include `status: passed`, `lifecycle_validated: true`, `generated_by: gsd-verifier`, `lifecycle_mode: yolo`, `phase_lifecycle_id: 129-2026-07-20T19-28-06` (matching 129-CONTEXT.md), and must name all 10 reassigned requirement IDs (D-11) — the traceability checker needs at minimum OBS-01/BOUND-02/HARD-05 tokens; naming all 10 also satisfies D-11's re-attestation.

### Pitfall 7: verify.sh comment prose is parsed by older checkers
**What goes wrong:** rewriting the ordering comment breaks `check-phase100/101/102/110/111` which `requireContains` exact sentences (e.g. `"Phase 108 is followed by Phase 110"`).
**How to avoid:** append-only edit to the comment block (e.g. add "Phase 128 is followed by Phase 129. Phase 129 precedes the final Phase 117 gate."); never reflow existing sentences.

### Pitfall 8: Fixture corpus strategy — live-clone vs synthetic
**What goes wrong:** Phase 127/128 mutation tests clone *live repo files* (`readFileSync(REPO_ROOT/…)`) into temp dirs. If the Phase 129 checker's corpus includes volatile planning artifacts (ROADMAP/REQUIREMENTS/audit), a live-clone "passes with complete corpus" test bakes in whatever stage the repo is in at that commit — the test then breaks on the *next* reconciliation commit.
**How to avoid:** split responsibilities. (a) The Phase 129 *aggregate seam guard* should target stable Rust/scripts corpus (plus flow-test anchors) and can use the live-clone fixture pattern; if it composes the 127/128 checkers it must copy the union of `PHASE128_TARGET_FILES` + the (to-be-exported) 127 target list + the whole `packages/open-bitcoin-rpc/src/bin/open_bitcoind/` directory (the 127 checker recursively reads it outside its target list). (b) The *reconciliation/stage* assertions belong in the Phase 124 stage machine, whose tests use the synthetic `createFixture` builder — extend it with archive-ready projections rather than cloning the live corpus.
**Warning signs:** checker tests green locally but failing right after an unrelated reconciliation commit.

### Pitfall 9: Deterministic-scope self-checks match naive source text
**What goes wrong:** the 127/128 checkers scan their own source for `fetch(`, `Bun.spawn`, `node:child_process`, `http://`, `https://`; writing those tokens literally in the Phase 129 checker (even in comments or failure strings) trips the guard if Phase 129 adopts the same self-scan (it should, per convention).
**How to avoid:** build forbidden-token lists with string concatenation exactly as 127/128 do (`"fetch" + "("`).

### Pitfall 10: LOC report and hook runtime tax on many small commits
**What goes wrong:** every commit regenerates `docs/metrics/lines-of-code.md` (hook `git add`s it) and runs full verify — measured history: median 194s, recent runs ~312s, p90 678s (verified via `command-timings.ts report --key verify-full`). A 10-commit execution plan costs 0.5–2 hours of verification alone; a stale LOC doc in a commit is a required freshness change, not noise.
**How to avoid:** batch related artifact+checker changes into single commits where atomicity allows; expect LOC diffs in commits that add checker files; never bypass the hook.

### Pitfall 11: OBS-01 truthfulness nuance (D-06 decision point)
**What goes wrong:** `fallback_counters()` in `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` (lines ~344–356) adds the count of live `getblocktxn_in_flight` entries to `compact_timeout_count` at snapshot time, on top of the durable increments recorded on real timeout cleanup (`record_cleanup`, lines ~241–256 — Timeout cause increments both `compact_download_timeout_count` and `fallback.compact_timeout_count`). Net effect: the *fallback* facet's `compact_timeout_count` transiently counts in-flight (not timed-out) requests. [VERIFIED: codebase]
**How to avoid:** the plan must make the D-06 call explicit: either document the semantics (e.g. in the verification report + parity/observability docs, "compact_timeout_count includes live in-flight getblocktxn entries at snapshot time") or minimally fix (stop mixing; expose in-flight via the in-flight facet only). A fix touches Rust → breadcrumbs (D-07), tests, and possibly the Phase 127 checker's field-level projection assertions — check `inbound_status.rs` and status contracts before choosing. Do not silently ignore this: FLOW-04 verification (D-11) attests truthfulness.

### Pitfall 12: Phase 129 checker asserting planning artifacts creates a circular update problem
**What goes wrong:** if the new Phase 129 checker itself pins reconciled-state strings (e.g. `Satisfied: 39`), then during the gaps-open window verify fails; if it pins gaps-open strings, reconciliation breaks it.
**How to avoid:** keep the Phase 129 checker's planning-artifact surface stage-aware or minimal; let the evolved Phase 124 stage machine own artifact-state assertions (it already has the stage plumbing). Recommended split: Phase 129 checker = seams + flows + verify.sh wiring + boundary tokens (stage-independent); Phase 124 stage machine = artifact/lifecycle/routing states.

## Code Examples

All verified against the current repo.

### Phase 129 checker skeleton (composition variant)

```typescript
// scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts
#!/usr/bin/env bun
import path from "node:path";
import { checkPhase127AuthoritativeNetworkStateUnification } from "./check-phase127-authoritative-network-state-unification";
import { checkPhase128ProductionCompactAnnouncementTransport } from "./check-phase128-production-compact-announcement-transport";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");

export function checkPhase129IntegrationGuardrailsAndMilestoneReconciliation(
  maybeRepoRoot?: string,
): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ??
      process.env.OPEN_BITCOIN_PHASE129_REPO_ROOT ??
      DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  // Six seams via composed upstream guards (D-03).
  failures.push(...checkPhase127AuthoritativeNetworkStateUnification(repoRoot));
  failures.push(...checkPhase128ProductionCompactAnnouncementTransport(repoRoot));
  // Cross-phase additions the upstream guards cannot express (D-04):
  // named FLOW-01..04 anchors, verify.sh Phase 129 wiring, bounded-claim tokens.
  checkNamedFlows(repoRoot, failures);
  checkVerifierWiring(repoRoot, failures);
  return failures;
}
```

Note: composed failures carry `P127`/`P128` prefixes — acceptable, but the mutation test must then assert those exact upstream strings, and the fixture needs both corpora (Pitfall 8). The dedup risk (127/128 already run standalone in verify.sh moments earlier) is only ~1–2s of repeat work; determinism is unaffected.

### Named flow anchors (stage-independent, live-clone-safe)

```typescript
const FLOW_ANCHORS = [
  // FLOW-01 + FLOW-04 production composition proof
  ["packages/open-bitcoin-rpc/tests/black_box_parity.rs",
   "phase127_production_composition_shares_sync_serving_and_operator_authority"],
  // FLOW-02 + FLOW-03 transport proof
  ["packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
   "production_announcement_transport_cases_fanout_uses_live_peer_facts"],
  ["packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs",
   "production_announcement_transport_cases_partial_failure_credits_only_prefix_and_redacts"],
] as const; // all three test names verified present in the current corpus
```

CLI operator surfaces for FLOW-04: `packages/open-bitcoin-cli/tests/operator_flows.rs` and `operator_binary.rs` exist (verified); dashboard/support test-module anchors are already asserted inside the Phase 127 checker (`dashboard_model_block_relay_rows_surface_shared_status_contract`, `support_bundle_renders_block_relay_evidence_from_shared_projection`, …) so composition covers them.

### Archive-ready audit assertion sketch (Phase 124 evolution)

```typescript
function verifyArchiveReadyAudit(audit: string, failures: string[]): void {
  for (const line of [
    "status: passed",
    'requirements: "39/39"',
    "gaps:\n  requirements: []\n  integration: []\n  flows: []",
  ]) {
    requireContains(audit, line, "archive-ready audit", failures);
  }
  for (const stale of ["status: gaps_found", "- id: GAP-0", "- id: FLOW-0"]) {
    requireAbsent(audit, stale, "archive-ready audit", failures);
  }
  requireContains(
    audit,
    "## Next Action\n\nRun `/gsd-complete-milestone v2.1`",
    "archive-ready audit route",
    failures,
  );
  for (const staleRoute of ["/gsd-plan-phase", "/gsd-execute-phase"]) {
    requireAbsent(audit, staleRoute, "archive-ready audit stale route", failures);
  }
}
```

(`- id:` matching targets frontmatter list entries only; historical prose mentions of "GAP-01" without the `- id: ` prefix stay legal, matching how `verifyAudit` counts today.)

### Fixture extension shape

```typescript
// check-phase124-milestone-closeout-reconciliation.fixtures.ts
type Phase129FixtureStage = "planned" | "executing" | "verified_pre_promotion" | "archive_ready";
type FixtureOptions = { /* existing */; maybePhase129Stage?: Phase129FixtureStage };
// archive_ready projects: requirements 39/39, roadmap [x]+N/N, audit passed/empty gaps,
// STATE/PROJECT/MILESTONES archive route, plus addPhase129Artifacts(files, stage)
// (context gsd-discuss-phase, plans gsd-plan-phase, summaries gsd-execute-plan with
//  requirements-completed, verification gsd-verifier + lifecycle_validated: true).
```

## State of the Art

Not an ecosystem-facing phase; the relevant "state of the art" is the repo's own newest precedent, which this phase should copy rather than innovate on:

| Old Approach | Current Approach | Where Established | Impact |
|--------------|------------------|-------------------|--------|
| Loosening checker assertions during transitions | Explicit enumerated fail-closed stages | Phases 125/126/128 stage machines | D-10 mandates the same for 129 |
| Companion "rerun" audit files | In-place audit supersession | v2.0 → v2.1 audit handling | D-12 mandates in-place |
| Requirement promotion by executor | Promotion only after independent gsd-verifier | Phase 126 precedent | Ordering constraint in Pitfall 6 |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Rerun audit full scores are `phases: "20/20"`, `integration: "13/13"`, `flows: "11/11"` (denominators from the current audit's own tables; numerator = full) | Architecture Patterns | Low — D-12 says "full scores"; if the audit rerun re-derives different denominators (e.g. keeps 17 phases for the original audit scope), the archive-ready checker assertion must match whatever the rerun writes. Keep the checker assertion authored in the same commit as the audit rerun. |
| A2 | `tech_debt:` in the passed audit may remain non-empty (carrying the 1,505-line checker debt) — the legacy `tech_debt: []` requirement lives only in the unreachable final-audit path | Open Questions | Low — v2.0 precedent used `tech_debt: []`; either is representable, planner should pick one and pin it in the archive-ready stage |
| A3 | Phase 129 plan/summary artifacts will use `generated_by: gsd-plan-phase` / `gsd-execute-plan` like Phases 127/128 (verified for 127/128; assumed the same generator versions produce 129's) | Architecture Patterns | Low — the new lifecycle assertions should be authored after the first real plan file exists, so drift self-corrects |
| A4 | gsd-verifier will produce `129-VERIFICATION.md` with `lifecycle_validated: true` (it did for 127/128) | Pitfall 6 | Low — if absent, the traceability checker fails; re-run verifier with lifecycle validation |

## Open Questions (RESOLVED)

All three questions were resolved during planning; the adopted answers are recorded inline and implemented by the committed plans.

1. **Which intermediate Phase 129 states does the stage machine legalize?**
   - What we know: 127 modeled a rich ladder (0/4 → n/4 → 4/4); 128 modeled only the three states it actually passed through, with a custom roadmap route string for the executing window.
   - What's unclear: the Phase 129 plan count (planner's discretion) and whether execution updates ROADMAP plan counts per-plan or only at checkpoints.
   - Recommendation: follow the 128 minimal-ladder approach — legalize exactly the states the plan structure will pass through, extending the checker in the same commits (Pitfall 1). Decide the plan count first, then hard-code it like `verifyPhase127Lifecycle` does.
   - **RESOLVED (Plan 129-03):** the stage machine legalizes exactly three fail-closed states per D-10 — gaps-open (current post-audit pins), Phase 129 verified pre-promotion, and reconciled archive-ready — with mixture rejection between them. Plan count is fixed at 4, hard-coded in the lifecycle assertions.
2. **`tech_debt` in the passed audit:** keep the two current entries (1,505-line file, cross-cutting-verification note — the latter is arguably *resolved by this phase* and should be closed or reworded) or empty the list per v2.0 precedent. Recommendation: mark the cross-cutting-verification debt resolved (Phase 129 delivers exactly those guards), carry only the 1,505-line maintainability item, and pin whichever shape is chosen in the archive-ready assertion.
   - **RESOLVED (Plans 129-03/129-04):** the passed audit carries only the 1,505-line `check-phase124-milestone-gap-closure.ts` maintainability entry; the cross-cutting-verification debt is recorded as resolved by Phase 129's guards. The archive-ready assertion pins this exact shape.
3. **D-06 fallback-counter call** (document vs. fix) — genuinely open until flow verification runs; both paths are researched (Pitfall 11) with the fix path carrying breadcrumb + checker-projection follow-through.
   - **RESOLVED (Plan 129-02):** adopted as an evidence-driven decision procedure executed inside Plan 02 — flow verification first; if the mixed counter misreports, apply the minimal Rust fix with parity breadcrumbs and regression tests; otherwise document the semantics in `docs/parity/`. Both outcomes are legal plan endpoints.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Bun | checkers, tests, hooks | ✓ | 1.3.9 | — |
| Cargo/Rust | full verify (`cargo test`, clippy, fmt, llvm-cov) | ✓ | cargo 1.94.1 | — |
| Bazel | verify smoke build | ✓ | 8.6.0 | — |
| gsd-tools.cjs | init/commit/lifecycle | ✓ | installed (`~/.cursor/get-shit-done/bin`) | — |
| Git hooks | pre-commit verify + LOC regen | ✓ | `.githooks/pre-commit` installed, runs `OPEN_BITCOIN_LOC_REPORT_SOURCE=index bash scripts/verify.sh` | `bash scripts/install-git-hooks.sh` self-heals |
| cargo-llvm-cov | full verify coverage step | ✓ (verify.sh `require_command` would fail otherwise; recent verify-full runs pass) | — | — |

**Missing dependencies:** none.

## Security Domain

`security_enforcement` is absent from `.planning/config.json` (= enabled), so this section is included; the phase's attack surface is minimal — local deterministic file-reading checkers and Markdown edits, no network, no auth, no crypto.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | — |
| V3 Session Management | no | — |
| V4 Access Control | no | — |
| V5 Input Validation | yes (narrowly) | Checkers parse repo-local text with anchored regexes and exact-string matching (existing convention); fail closed on malformed frontmatter — preserve the "exactly one frontmatter block / exactly one key" strictness, don't loosen it |
| V6 Cryptography | no | — |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Checker gains network/process reach (supply-chain / nondeterminism) | Tampering/Elevation | Keep the forbidden-token self-scan (`fetch(`, `Bun.spawn`, `node:child_process`, URLs) in the Phase 129 checker per 127/128 convention; D-16 keeps verify.sh public-network-free |
| Guard bypass via inconsistent-state windows | Repudiation | D-10 fail-closed mixtures; mutation tests for every flipped assertion |
| Weakening the no-claim boundary while editing claim-scanned docs | Tampering | `verifyNoClaimBoundary` + Phase 117 claim scanner stay unchanged; audit/PROJECT/README edits keep no-claim markers |

## Sources

### Primary (HIGH confidence — direct file reads this session)
- `scripts/verify.sh` (full read; triple-contract line anchors 295–309 / 310–419 / 452–566)
- `scripts/check-phase124-post-audit-gap-planning.ts` (full read, 690 lines)
- `scripts/check-phase124-milestone-closeout-reconciliation.ts` (full read, 591 lines)
- `scripts/check-phase124-milestone-closeout-lifecycle.ts` (full read)
- `scripts/check-phase124-milestone-gap-closure.ts` (targeted reads: stage detection, completed-artifact checks, stage types)
- `scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts` (createFixture options + stage projections)
- `scripts/check-phase127-authoritative-network-state-unification.ts` + `.test.ts` (full/targeted)
- `scripts/check-phase128-production-compact-announcement-transport.ts` + `.test.ts` (full/targeted)
- `scripts/check-phase117-parity-uat-release-boundary.ts` (full read)
- `scripts/check-active-milestone-verification-traceability.ts` (full read)
- `scripts/check-parity-breadcrumbs.ts` (scope regex)
- `.planning/{REQUIREMENTS,ROADMAP,STATE,PROJECT,MILESTONES}.md`, `.planning/v2.1-MILESTONE-AUDIT.md`, `.planning/milestones/v2.0-MILESTONE-AUDIT.md`, `.planning/config.json`
- Phase 127/128 lifecycle artifacts (`*-PLAN.md`, `*-SUMMARY.md`, `128-VERIFICATION.md` frontmatter)
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` (fallback-counter mixing sites), Rust flow-test anchor greps
- Local tool probes: `bun --version`, `cargo --version`, `bazel --version`, `.githooks/pre-commit`, `command-timings.ts report --key verify-full`

### Secondary (MEDIUM)
- Phase 127/128 CONTEXT/VERIFICATION documents (locked-decision provenance, cited by 129-CONTEXT)

### Tertiary (LOW)
- None. No web research was needed; the domain is entirely repo-local.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new dependencies; conventions verified in 6+ existing checker pairs
- Architecture (stage machine, verify.sh anchors, checker interactions): HIGH — every assertion cross-read in source with line anchors
- Pitfalls: HIGH for mechanics (all reproduced from code); MEDIUM only for the audit-rerun score denominators (A1) and gsd generator behavior for 129 artifacts (A3/A4)
- OBS-01 runtime claims: HIGH — D-05 wiring and the D-06 counter nuance both re-verified in source this session

**Research date:** 2026-07-20
**Valid until:** state-sensitive — findings describe the repo at Phase-128-complete state; re-verify current-stage assertions (Pitfall 1 table, ROADMAP line numbers) if any commit lands between research and planning. Stable for ~30 days otherwise.
