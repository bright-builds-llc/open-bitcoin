# Phase 138: Parity, Adversarial Pressure, Restart, and Release Guardrails - Research

**Researched:** 2026-08-22
**Domain:** Closeout guardrails — inventory-and-gap-fill of Phase 130–137 evidence, last-gate Bun claim checker, parity/UAT/benchmark wiring
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Coverage composition

- **D-01:** Prove MPVFY-01 with an inventory-and-gap-fill 4×6 matrix, not a
  new mega-harness and not by treating per-phase checkers as sufficient.
  The four methods are pinned-Knots fixtures, fake-clock scenarios,
  randomized graph-oracle tests, and failure injection. The six behaviors
  are package, rolling-fee, pressure, expiry, recovery, and retry.
- **D-02:** Map every matrix cell to a named existing Phase 130–137 Rust
  test, checker, or verification artifact. Add only the missing cells. Do
  not rewrite package, pressure, recovery, or retry runtime behavior unless
  a named cell has no honest existing proof.
- **D-03:** Phases 136 and 137 have no Bun checkers today. The Phase 138
  checker must name their runnable evidence rather than assuming
  `136-VERIFICATION.md` / `137-VERIFICATION.md` alone satisfy MPVFY-01.
- **D-04:** Restart/recovery proof stays hermetic and fake-clock. The
  required composition is checkpoint → restart install → rebuild derived
  state, reset rolling fee, preserve canonical entries and surviving local
  unbroadcast membership, remint retry timers, plus one injected
  install/write failure. Do not add process-kill, public-network, or
  wall-clock soak restart gates.
- **D-05:** Add at most a small named Rust composition case if the
  restart × package membership × retry remint × injected failure
  intersection cannot be cited from existing tests. Prefer naming existing
  131/135/136 cases first.

### Benchmark gating

- **D-06:** Use a hybrid gate. Default verification enforces documented
  work-count bounds (package member/weight limits, trim-package counts,
  clone/recompute counts, per-tick inspect/prepare budgets). Opt-in
  `--full` or UAT benches may record latency. Default `verify.sh` must not
  gain public-network or wall-clock gates.
- **D-07:** Retire the Phase 131 sustained-pressure 2-second `Instant`
  smoke gate from the default verifier path. PRESS-05 remains closed by
  the recomputation oracle plus work-count bounds, not by silicon latency.
- **D-08:** Keep `docs/parity/benchmarks.md` and
  `scripts/check-benchmark-report.ts` on the existing `threshold_free`
  contract for default smoke. Documented timing budgets belong only on the
  opt-in full/UAT path.
- **D-09:** `bun run scripts/command-timings.ts` stays local duration
  history. It is not a release or default-verify gate.

### Parity and UAT closeout

- **D-10:** Phase 138 canonically owns only MPVFY-01 through MPVFY-04.
  Phases 130–137 remain the exactly-once owners of FEEP, PRESS, PACK,
  PPKG, MPLIFE, MPDUR, IBR, and MPOBS even when the Phase 138 checker
  aggregates their evidence.
- **D-11:** Do not collapse v2.2 evidence into one catalog entry.
  `docs/parity/index.json` remains the machine-readable root.
  `docs/parity/checklist.md`, the catalog pages, and
  `docs/parity/release-readiness.md` remain the human review roots.
- **D-12:** Backfill missing machine-readable owners before closeout:
  add a Phase 136 surface for PPKG-04 and IBR-01 through IBR-04, and give
  PACK-01 through PACK-07 and PRESS-01 through PRESS-05 exact
  `index.json` / checklist owners. Prefer index/checklist rows over new
  catalog pages when catalog prose already exists.
- **D-13:** Promote completed `in_progress` v2.2 surfaces to `done`
  (`v2-2-resource-time-fee-primitives`,
  `v2-2-authoritative-cross-cache-lifecycle-integration`,
  `v2-2-snapshot-schema-checkpointing-recovery`,
  `v2-2-rpc-and-sanitized-operator-evidence`) after their evidence roots
  are current. Preserve the existing Phase 133 `done` surface.
- **D-14:** Add one closeout surface
  `v2-2-parity-uat-release-boundary` that owns only MPVFY-01 through
  MPVFY-04. Do not create a competing closeout manifest.
- **D-15:** Create
  `.planning/phases/138-parity-adversarial-pressure-restart-and-release-guardrails/138-UAT.md`
  as the committed UAT package. Required tests stay deterministic.
  Public-network review may be recorded as not run and must never become
  a default, CI, or release gate.
- **D-16:** UAT guidance must provide copy-pasteable repo-local Cargo and
  Bazel forms:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`, plus
  the matching `open-bitcoin-cli` / `open-bitcoind` forms where those
  binaries are the operator surface.
- **D-17:** Flip every implemented-but-Pending v2.2 requirement row
  (PACK-01..07, PPKG-04, MPDUR-01..04, IBR-01..04, MPOBS-01..03, and
  MPVFY-01..04) only after the checker can name their evidence. Do not
  leave leftover Pending rows that would force a Phase 129-style
  reconciliation later.
- **D-18:** Reconcile ROADMAP, REQUIREMENTS, PROJECT, and STATE to agree
  that v2.2 implementation and closeout evidence are complete. Do not
  archive the milestone or invent a milestone-audit workflow.

### Claim-guardrail ownership

- **D-19:** Add a new Phase 138 Bun/TypeScript checker pair as the last
  `check-phase*` no-claim gate. Follow the Phase 117 export shape:
  `checkPhase138...(maybeRepoRoot?)` returning `string[]` failures, plus
  fixture mutation tests and an `OPEN_BITCOIN_PHASE138_REPO_ROOT` override.
- **D-20:** Wire the new pair into `scripts/verify.sh` immediately after
  the last current v2.2 checker and after Phase 117, updating the
  ordering comment, `VERIFY_COMMAND_ORDER` heredoc, and live `run_step`
  chain together. Phase 117 remains the v2.1 BOUND gate; Phase 138
  becomes the v2.2 closeout gate.
- **D-21:** The allowed scoped claim is: bounded local-package APIs,
  same-peer 1P1C assembly over ordinary transaction messages, ordinary
  transaction fanout, and initial-broadcast-retry of locally submitted
  unbroadcast members. Companion allowed wording includes persist
  canonical entries / acceptance times / surviving local unbroadcast,
  rebuild derived state and reset rolling fee on restart, and hermetic
  default verification.
- **D-22:** The checker must reject general package wire, arbitrary
  multi-parent assembly, whole-mempool rebroadcast, public/default/
  production relay, guaranteed propagation, public-network CI, and
  production-readiness / production-funds claims. Explicit deferred,
  unsupported, future-gated, no-claim, and opt-in-UAT wording remains
  valid.
- **D-23:** Use a curated current claim-bearing corpus
  (README, runtime guide, parity checklist/catalog/release-readiness/
  production-claim-boundary/support-matrix, operator docs). Do not scan
  historical `.planning/` prose or milestone archives as a blocking
  surface.
- **D-24:** Keep Phase 117's archived v2.1 "package relay remains
  deferred" contract intact. Either freeze 117's live claim corpus away
  from the new v2.2 wording or add a narrow scoped allow so live docs
  can state the D-21 claim without weakening 117's v2.1 archive wording.
  Do not globally allow unscoped `package relay`.
- **D-25:** The aggregate checker validates all 40 v2.2 requirements
  exactly once, required Phase 130–138 parity surfaces, concrete Knots
  anchors, relevant breadcrumb groups, exact repo-local UAT commands,
  work-count (not wall-clock) benchmark wiring, and both visible and
  executable verifier ordering.

### Claude's Discretion

The planner may choose exact checker helper names, fixture organization,
paragraph-classification implementation, the smallest honest breadcrumb
or catalog-owner splits, exact doc section placement, and whether
optional UAT items are recorded as pending or not run. Prefer small pure
TypeScript helpers with focused tests, targeted doc edits, and the
existing Phase 106/117 patterns. Do not spend discretion on runtime
behavior changes or broader claims.

### Deferred Ideas (OUT OF SCOPE)

- `/gsd-complete-milestone v2.2` archival — after Phase 138 verification
  passes.
- General package wire / BIP331 (FUT-12).
- Arbitrary multi-parent or cluster-mempool policy (FUT-13).
- Public/default production relay, public-network CI, guaranteed
  propagation, and production-readiness claims (FUT-15 through FUT-17).

None — discussion stayed within phase scope otherwise.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| MPVFY-01 | Package, rolling-fee, pressure, expiry, recovery, and retry behavior has deterministic pinned-Knots fixtures, fake-clock scenarios, randomized graph-oracle tests, and failure-injection coverage. | 4×6 inventory below. 23/24 cells have named existing proof. The only required new Rust work is the D-04/D-05 restart × package membership × retry remint × injected-failure composition. |
| MPVFY-02 | Package and sustained-pressure benchmarks enforce documented bounded-work and performance expectations without adding public-network or wall-clock gates to default verification. | Retire `SUSTAINED_PRESSURE_MAX_ELAPSED` in `packages/open-bitcoin-bench/src/cases/mempool.rs` and the Phase 131 checker pin. Keep `threshold_free` smoke. Enforce work-count bounds already present in Phase 132/136 tests. |
| MPVFY-03 | Parity catalogs, breadcrumbs, operator docs, and repo-local Cargo and Bazel UAT commands identify exact Knots anchors, intentional differences, and evidence boundaries for every v2.2 surface. | Backfill missing PACK/PRESS/136 owners; promote four `in_progress` surfaces; add `v2-2-parity-uat-release-boundary`; write `138-UAT.md` from the Phase 117 package; reuse Phase 137 runtime-guide command forms. |
| MPVFY-04 | Deterministic claim guardrails require the bounded local-package, same-peer 1P1C, ordinary transaction fanout, and initial-broadcast-retry wording while rejecting general package wire, whole-mempool rebroadcast, public/default/production relay, guaranteed propagation, public-network CI, and production-readiness claims. | New last-gate Bun pair after Phase 117. Keep 117's `package relay` deny. State D-21 without the bare phrase `package relay`. Update hard last-gate helpers in 124/129/130/131 and the adjacent 117→reconciliation sequence. |
</phase_requirements>

## Summary

Phase 138 is a closeout and guardrail phase. The planner should inventory existing Phase 130–137 Rust fixtures into a 4×6 MPVFY-01 matrix, add at most one named hermetic restart-composition test, retire the Phase 131 2-second `Instant` smoke gate, backfill missing parity owners, promote completed surfaces, write `138-UAT.md`, add a Phase 117-shaped last-gate Bun checker, and flip leftover Pending rows only after that checker can name evidence. Do not add runtime features, archive v2.2, or broaden claims.

The highest-risk implementation traps are not missing tests. They are existing checkers that will fail when 138 becomes last or when Pending/`in_progress` rows flip: Phase 135's hard `in_progress` + MPDUR-Pending lock, Phase 124/129/130/131 `requireFinalPhaseChecker` ("must end with Phase 117"), `check-current-documentation-reconciliation.ts` adjacent `117 → reconciliation` sequence, Phase 131's pin of `SUSTAINED_PRESSURE_MAX_ELAPSED`, and Phase 117's live `DANGEROUS_CLAIMS` entry `"package relay"`.

**Primary recommendation:** Copy the Phase 117 closeout shape (one surface, one UAT package, one last-gate checker) and spend the first plan on checker-lock updates plus the one missing restart-composition cell before flipping any Pending row.

## Project Constraints (from .cursor/rules/)

None — `.cursor/rules/` is absent in this repository. Actionable constraints come from `AGENTS.md`, `AGENTS.bright-builds.md`, and `standards/`:

- Use `bash scripts/verify.sh` as the default verification contract. [VERIFIED: AGENTS.md]
- UAT commands must be copy-pasteable Cargo and Bazel forms, not only the installed `open-bitcoin` alias. [VERIFIED: AGENTS.md]
- New first-party Rust source or test files need parity breadcrumbs via `docs/parity/source-breadcrumbs.json`. TypeScript checkers do not. [VERIFIED: AGENTS.md]
- Default verification stays hermetic: no public-network, soak, service-manager, or wall-clock release gates. [VERIFIED: AGENTS.md, 138-CONTEXT.md]
- Prefer Bun/TypeScript for substantial script logic; keep Bash as thin orchestration. [VERIFIED: AGENTS.md]
- Run ad-hoc Cargo/Bazel through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`. That tool is local history, not a release gate (D-09). [VERIFIED: AGENTS.md, 138-CONTEXT.md]
- Functional-core crates stay I/O-free; this phase must not add runtime I/O to core crates. [VERIFIED: AGENTS.md, standards/core/architecture.md]
- Do not use `unwrap()` in new Rust. [VERIFIED: user code-styling rules]

## Standard Stack

This phase adds no production runtime libraries. Use the existing closeout stack.

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Bun | repo pin `.bun-version` `1.3.9`; local runtime observed `1.3.14` | Checker + fixture test runtime | Canonical repo script runtime. [VERIFIED: `.bun-version`, `bun --version`] |
| TypeScript via Bun | repo-owned `scripts/*.ts` | Pure filesystem checkers returning `string[]` | Phase 106/117/132/135 pattern. [VERIFIED: existing checkers] |
| Rust | `1.94.1` / edition 2024 | At most one new composition test | Pinned by `rust-toolchain.toml`. [VERIFIED: `rustc --version`] |
| `node:fs` / `node:path` / `node:os` | Bun built-ins | Fixture roots and corpus reads | Phase 117 fixture pattern. [VERIFIED: `scripts/check-phase117-parity-uat-release-boundary/test-fixtures.ts`] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `bun:test` | Bun built-in | Mutation fixture tests | Every new checker pair |
| `open-bitcoin-bench` | workspace crate | Smoke + `--full` benches | Keep smoke `threshold_free`; optional UAT timings only |
| Bazel / Bazelisk | local Bazel `8.6.0` | UAT command forms | Document `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| New Phase 138 checker | Absorb MPVFY into Phase 117 | Locked out by D-19/D-20. 117 owns v2.1 BOUND only. |
| Mega Rust harness | Inventory-and-gap-fill | Locked out by D-01/D-02. |
| Wall-clock bench gate | Work-count bounds | Locked out by D-06/D-07. |
| Process-kill restart soak | Fake-clock composition | Locked out by D-04. |

**Installation:** none. No new packages.

**Version verification:** Bun pin `1.3.9`, local Bun `1.3.14`, Rust `1.94.1`, Cargo `1.94.1`, Bazel `8.6.0` observed 2026-08-22. [VERIFIED: local `command -v` probes]

## Architecture Patterns

### Recommended Project Structure

```
scripts/
├── check-phase138-parity-uat-release-boundary.ts          # CLI + re-export
├── check-phase138-parity-uat-release-boundary.test.ts     # thin entry
└── check-phase138-parity-uat-release-boundary/
    ├── checks.ts          # checkPhase138ParityUatReleaseBoundary
    ├── constants.ts       # surfaces, corpus, claims, UAT commands
    ├── matrix.ts          # 4×6 named-evidence table
    ├── claims.ts          # paragraph/clause allow-deny
    ├── verifier.ts        # verify.sh order + last-gate
    └── test-fixtures.ts   # createFixture + mutations

.planning/phases/138-parity-adversarial-pressure-restart-and-release-guardrails/
└── 138-UAT.md

docs/parity/
├── index.json             # backfill + promote + new surface
├── checklist.md
├── catalog/mempool-policy.md
├── catalog/rpc-cli-config.md
├── release-readiness.md
├── benchmarks.md          # work-count default; timings opt-in only
└── service-operation-expectations.md

packages/open-bitcoin-node/src/network/tests/
└── <existing recovery/maintenance module>  # at most one composition test
```

Discretion recommendation: follow Phase 117's directory split (`claims.ts`, `test-fixtures.ts`, `verifier.ts`) rather than a single 500-line root. [VERIFIED: `scripts/check-phase117-parity-uat-release-boundary/`]

### Pattern 1: Phase 117 export / fixture / env override

**What:** Pure function, optional repo-root, env override, `string[]` failures, temp fixture corpus.
**When to use:** The new Phase 138 pair (D-19).
**Example:**

```typescript
// Source: scripts/check-phase117-parity-uat-release-boundary.ts
export function checkPhase117ParityUatReleaseBoundary(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE117_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  // ... filesystem-only checks ...
  return failures;
}
```

Required Phase 138 mirrors:

| Piece | Use this exact shape |
|-------|----------------------|
| Export | `checkPhase138ParityUatReleaseBoundary(maybeRepoRoot?: string): string[]` |
| Env | `OPEN_BITCOIN_PHASE138_REPO_ROOT` |
| CLI | `if (import.meta.main)` print failures and `process.exit(1)` |
| Fixtures | `createFixture({ maybeMutate })` writing the curated corpus into a temp root |
| Tests | Arrange / Act / Assert; one concept per mutation |

Phase 132 already uses `OPEN_BITCOIN_PHASE132_REPO_ROOT`. Phase 135's live export takes `maybeRepoRoot` but has **no** env override — do not copy that omission. [VERIFIED: `scripts/check-phase132-typed-package-staged-admission/checks.ts`, `scripts/check-phase135-snapshot-recovery.ts`]

### Pattern 2: verify.sh dual-surface lockstep

**What:** Visible `VERIFY_COMMAND_ORDER` heredoc and executable `run_step` chain must stay identical.
**When to use:** Every checker-order change.

Current tail after Phase 135 [VERIFIED: `scripts/verify.sh` lines 425–433 and 587–595]:

```
bun test scripts/check-phase135-snapshot-recovery.test.ts
bun run scripts/check-phase135-snapshot-recovery.ts
bun test scripts/check-phase117-parity-uat-release-boundary.test.ts
bun run scripts/check-phase117-parity-uat-release-boundary.ts
bun test scripts/check-current-documentation-reconciliation.test.ts
bun run scripts/check-current-documentation-reconciliation.ts
```

**Required insertion (D-20):** after Phase 117 and before current-documentation-reconciliation, because 138 must be the last `check-phase*` gate while remaining after 117.

```
...135 test/check
...117 test/check
bun test scripts/check-phase138-parity-uat-release-boundary.test.ts
bun run scripts/check-phase138-parity-uat-release-boundary.ts
...current-documentation-reconciliation test/check
```

Update the comment at `scripts/verify.sh:304-307` that still says "Phase 117 remains the final changed-path release-boundary and no-claim gate" and "The current-documentation reconciliation test/check pair runs immediately after that final Phase 117 gate."

`orderedLines` in Phase 117 is a **subsequence** check, so 117 → 138 → PURE_CORE still satisfies 117 → PURE_CORE. [VERIFIED: `scripts/check-phase117-parity-uat-release-boundary.ts` `orderedLines`]

`check-current-documentation-reconciliation.ts` is an **adjacent string** check (`visible.includes(VISIBLE_SEQUENCE)` where the sequence is `117 test\n117 check\nreconciliation test\nreconciliation check`). Inserting 138 between 117 and reconciliation **breaks** it unless that sequence is updated. [VERIFIED: `scripts/check-current-documentation-reconciliation.ts:36-47,325-330`]

### Pattern 3: Inventory-and-gap-fill matrix, not a mega-harness

**What:** The Phase 138 checker holds a 4×6 table of `{method, behavior, artifact, symbol}` and fails if any cell is missing or the named file/symbol is absent.
**When to use:** MPVFY-01 (D-01, D-02, D-25).

Do not spawn `cargo test` from the checker. Name the filter and assert the source still contains the symbol, the same way Phase 131 asserts `sustained_pressure_oracle` and `rolling_fee_restarts_at_zero` exist as text. [VERIFIED: `scripts/check-phase131-rolling-fee-expiry-pressure.ts:169-176`]

### Anti-Patterns to Avoid

- **Absorbing MPVFY into Phase 117:** 117 uniquely owns BOUND-01..05 and v2.1 surfaces. [VERIFIED: `REQUIREMENTS_BY_SURFACE` in Phase 117 checker]
- **Scanning `.planning/` history as a blocking claim corpus:** D-23. 117 already remaps live `REQUIREMENTS.md` to `.planning/milestones/v2.1-REQUIREMENTS.md` when the live file is not v2.1. [VERIFIED: `v21RequirementsSource`]
- **Writing "Phase 138 is complete/done/shipped" on surfaces Phase 134 scans:** Phase 134 forbids that regex. Keep "Phase 138 owns MPVFY" in 134 `known_gaps`. [VERIFIED: `scripts/check-phase134-authoritative-lifecycle/scope.ts:26-30,86-91`]
- **Flipping MPDUR Pending before relaxing Phase 135:** Phase 135 currently requires `in_progress` plus `- [ ] **MPDUR-0X**` and checklist `| MPDUR-0X | Pending |`. [VERIFIED: `scripts/check-phase135-snapshot-recovery.ts:474-501`]
- **Saying "supports package relay" in 117 CLAIM_FILES:** 117 denies the substring `package relay` on positive clauses. Use D-21 wording that never contains that phrase. [VERIFIED: `DANGEROUS_CLAIMS` in Phase 117 checker]
- **Leaving PACK/PRESS/PPKG-04/IBR without `index.json` owners:** D-12 / D-25 / D-17. Those IDs have catalog prose but no machine-readable surface owner today.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Closeout checker | New framework or result alias | Phase 117 `string[]` + fixture mutations | D-19; 129 already reused this contract |
| Claim classification | Whole-file deny | Phase 117 paragraph/clause + no-claim markers | Mixed "deferred, while X is supported" sentences |
| UAT package | Ad-hoc checklist | Copy `117-UAT.md` | D-15 |
| Restart soak | Process-kill / wall-clock | Fake-clock composition on existing 135/136 seams | D-04 |
| Graph oracle | proptest / new RNG crate | Existing seeded generators | `generated_steps(0x1341_1000_5eed)` and `generated_graph_recomputation_oracle_covers_twenty_five_sparse_additions` already exist |
| Benchmark schema | New report format | `scripts/check-benchmark-report.ts` `threshold_free` | D-08 |
| Cargo/Bazel UAT forms | Installed alias only | Phase 137 runtime-guide blocks | D-16, AGENTS.md |
| Breadcrumb engine | Custom TS parser | `scripts/check-parity-breadcrumbs.ts` + `source-breadcrumbs.json` | Repo contract |

**Key insight:** Closeout phases fail when they rewrite working runtime or leave leftover Pending rows. Inventory, name, guard, then flip.

## MPVFY-01 4×6 Evidence Inventory

Methods: pinned-Knots fixtures, fake-clock scenarios, randomized/seeded graph-oracle, failure injection.
Behaviors: package, rolling-fee, pressure, expiry, recovery, retry.

Status key: **HAVE** = honest named proof exists. **GAP** = add at most the D-05 composition (or a checker-named citation if a later read finds one).

| Method \ Behavior | Package | Rolling-fee | Pressure | Expiry | Recovery | Retry |
|-------------------|---------|-------------|----------|--------|----------|-------|
| Pinned-Knots fixtures | HAVE | HAVE | HAVE | HAVE | HAVE | HAVE |
| Fake-clock scenarios | HAVE | HAVE | HAVE | HAVE | HAVE | HAVE |
| Seeded graph-oracle | HAVE | HAVE | HAVE | HAVE | HAVE | HAVE |
| Failure injection | HAVE | HAVE | HAVE | HAVE | HAVE | HAVE |
| **D-04 composition** | — | — | — | — | **GAP** | **GAP** |

The 24 method×behavior cells are covered. The locked D-04/D-05 intersection (restart × package membership × retry remint × injected failure in **one** named case) is not.

### Named existing cells

#### Package

| Method | File | Symbol |
|--------|------|--------|
| Pinned-Knots | `packages/open-bitcoin-mempool/src/package/tests/empty_package_is_rejected.rs` | `package_fingerprint_matches_knots_fixed_vector`, `package_above_maximum_count_is_rejected`, `package_above_maximum_weight_is_rejected` |
| Pinned-Knots | `packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases/dry_run_submit_valid_parent_invalid_child_partial_acceptance_and_lifecyc.rs` | partial-acceptance / dry-run≡submit cases |
| Pinned-Knots | `packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases/*.rs` | residual-group, TRUC, reconsiderable cases |
| Fake-clock | Package admission uses injected `AdmissionContext` / `PolicyTime`; no wall clock | Phase 132 checker pins PACK-01..07 shape/time-free evaluation |
| Graph-oracle | `packages/open-bitcoin-mempool/src/pool/tests/prospective_oracle_cases.rs` | `generated_graph_recomputation_oracle_covers_twenty_five_sparse_additions` |
| Graph-oracle | `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/oracle.rs` | `generated_steps` includes `Step::Package`; `fixed_seed_generated_oracle_detects_each_corrupted_target_exactly` |
| Failure injection | `packages/open-bitcoin-mempool/src/pool/tests/prospective_failure_cases.rs` | staged overlay / trim / commit failures |
| Failure injection | `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases/newly_present_becomes_post_trim_absent_and_dry_run_submit_reports_are_eq.rs` | `package_rbf_replacement_rollback_*` |
| Failure injection | `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/oracle.rs` | `every_injected_preflight_failure_preserves_the_complete_aggregate` |
| Work-count (MPVFY-02) | same parity file | `package_trim_count_for_test() == 1`, `full_clone_count_for_test() == 0`, `full_recompute_count_for_test() == 0` |

#### Rolling-fee

| Method | File | Symbol |
|--------|------|--------|
| Pinned-Knots | `packages/open-bitcoin-mempool/src/pool/tests/rolling_fee_cases.rs` | `rolling_fee_decay_twelve_hour_halflife_at_high_occupancy`, `..._six_hour_...`, `..._three_hour_...`, `rolling_fee_decay_zeros_below_incremental_half` |
| Fake-clock | same | `rolling_fee_decay_does_not_run_before_block_after_bump`, `rolling_fee_decay_skips_updates_within_ten_seconds` (PolicyTime, not `Instant`) |
| Graph-oracle | `packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs` | `sustained_pressure_oracle_agrees_across_fill_trim_block_decay_expiry_refill_reorg` asserts rolling after decay |
| Failure injection | `packages/open-bitcoin-mempool/src/fee/rolling.rs` | `set_rolling_fee_rate_updates_inject_seam` plus lifecycle decay-gate tests |
| Restart (partial) | `sustained_pressure_cases.rs` | `rolling_fee_restarts_at_zero_without_durability` — **not** the D-04 composition |

#### Pressure

| Method | File | Symbol |
|--------|------|--------|
| Pinned-Knots | `packages/open-bitcoin-mempool/src/pool/tests/pressure_cases.rs` | `accounted_capacity_trim_evicts_until_usage_within_capacity`, `pressure_bump_uses_descendant_package_feerate_plus_incremental`, `pressure_removes_victim_and_descendants_with_roles` |
| Fake-clock | `sustained_pressure_cases.rs` | full fill/trim/block/decay/expiry/refill/reorg sequence on PolicyTime |
| Graph-oracle | `sustained_pressure_cases.rs` + `prospective_oracle_cases.rs` | oracle + 25-sparse generated graph trim |
| Graph-oracle | `lifecycle_projection_cases/oracle.rs` | `Step::Pressure` in `generated_steps` |
| Failure injection | `pressure_internal_cases.rs`, `prospective_failure_cases.rs` | trim / overlay failure paths |
| Work-count | bench + tests | 24 admit/trim cycles; one final package trim |

#### Expiry

| Method | File | Symbol |
|--------|------|--------|
| Pinned-Knots | `packages/open-bitcoin-mempool/src/pool/tests/expiry_cases.rs` | `expiry_removes_aged_entry_and_descendants`, `expiry_retains_fresh_known_entries`, `expiry_skips_legacy_unknown_without_inventing_time`, `expiry_emits_mempool_removal_cause_expiry` |
| Fake-clock | same + `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases/lifecycle_cases.rs` | `expiry_uses_injected_time_without_sleeping` |
| Graph-oracle | `sustained_pressure_cases.rs` + `oracle.rs` `Step::Expiry` | expiry inside generated/oracle sequences |
| Failure injection | `packages/open-bitcoin-mempool/src/pool/tests/prepared_maintenance_cases.rs` | `expiry_preparation_is_pure_and_orders_descendants_before_ancestors` |
| Cross-cache | `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs` | `expiry_removes_descendants_from_every_projection_and_advances_once` |

#### Recovery

| Method | File | Symbol |
|--------|------|--------|
| Pinned-Knots | `packages/open-bitcoin-node/src/network/tests/recovery_cases.rs` | `recovery_topology_orders_parent_before_child_independent_of_stored_order` |
| Pinned-Knots | `packages/open-bitcoin-node/src/network/tests/recovery_cases/metadata.rs` | `recovery_metadata_managed_local_requested_preserves_facts_and_fanout` |
| Fake-clock | `recovery_cases/staging.rs` | `prepare_mempool_recovery_at(..., PolicyTime::from_unix_seconds(...))` |
| Membership | `recovery_cases/staging.rs` | `recovery_keeps_unbroadcast_only_for_the_exact_surviving_member` |
| Failure injection | `recovery_cases/staging.rs` | `every_injected_recovery_install_validation_failure_preserves_the_exact_aggregate` (`RecoveryInstallFailureGuard::inject`) |
| Failure injection | `recovery_cases/staging.rs` | `recovery_install_rejects_a_chainstate_change_after_preparation` |
| Timer remint-to-None | `packages/open-bitcoin-node/src/network/tests/maintenance_tick_cases.rs` | `recovery_install_clears_due_time_and_cursor` |

#### Retry

| Method | File | Symbol |
|--------|------|--------|
| Pinned-Knots | `packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs` | `retry_cycle_length_is_ten_minutes_plus_injected_jitter`, production inspect=256 / prepare=32 |
| Fake-clock | `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/retry.rs` | `retry_worker_fires_maintenance_tick_on_elapsed_without_sleep`, `retry_worker_shutdown_stops_before_next_tick`, `retry_worker_jitter_unavailable_does_not_use_silent_constant_jitter` |
| Fake-clock | `maintenance_tick_cases.rs` | `maintenance_tick_remints_due_time_from_injected_context`, `maintenance_tick_respects_inspect_256_and_prepare_32_with_leftover_cursor` |
| Graph/subset oracle | Phase 136 VERIFICATION names | `expected_unbroadcast_members` subset oracle; `maintenance_tick_walks_only_unbroadcast_members` |
| Failure injection | `retry_worker_jitter_unavailable_*`; injected recovery install (above) | jitter/install failures, not the full D-04 composition |
| Fanout | `packages/open-bitcoin-node/src/network/tests/package_fanout_cases.rs` | `package_fanout_*` parent-before-child |

### The one missing composition (D-04 / D-05)

Cite these separately — they do **not** jointly prove the intersection:

1. `rolling_fee_restarts_at_zero_without_durability` — restart rolling fee, no package/unbroadcast/retry/inject.
2. `recovery_keeps_unbroadcast_only_for_the_exact_surviving_member` — surviving unbroadcast, no remint/inject.
3. `recovery_install_clears_due_time_and_cursor` — remint-to-`None` on install, no package members, no inject.
4. `maintenance_tick_remints_due_time_from_injected_context` — remint due time, no recovery.
5. `every_injected_recovery_install_validation_failure_preserves_the_exact_aggregate` — inject, no package/retry remint.

**Add at most one named Rust test** that: builds a source-only snapshot with a local-package (or parent+child) plus surviving unbroadcast → `prepare_mempool_recovery_at` / `install_mempool_recovery` → asserts rebuilt derived state, rolling fee at restart baseline, canonical + surviving unbroadcast preserved, retry timer reminted to `None` then due-time from injected context → one `RecoveryInstallFailureGuard::inject` (or snapshot-write inject) that leaves the aggregate unchanged. Place it next to `recovery_cases/staging.rs` or `maintenance_tick_cases.rs`. Add a breadcrumb. Do not add process-kill or wall-clock soak.

### Phase 136 / 137 runnable evidence (D-03)

Do **not** treat VERIFICATION.md as sufficient. The Phase 138 checker must name commands such as:

```bash
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib retry
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib maintenance_tick
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib package_fanout
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib retry_worker
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib package
```

136-VERIFICATION.md already lists the exact test names to pin as source symbols (`local_requested_admission_inserts_unbroadcast_member`, `maintenance_tick_walks_only_unbroadcast_members`, four `package_fanout_*`, five `getdata_tx_receipt_*`, four `retry_worker_*`). [VERIFIED: `.planning/phases/136-receive-independent-maintenance-and-transport-receipts/136-VERIFICATION.md`]

137-VERIFICATION.md names `project_testmempoolaccept` / `project_submitpackage`, dual-state identifier-free proofs, and Knots RPC codes `-8` / `-22` / `-25`. [VERIFIED: `.planning/phases/137-rpc-and-sanitized-operator-evidence/137-VERIFICATION.md`]

There is no `scripts/check-phase136*` or `scripts/check-phase137*`. [VERIFIED: glob]

## Bun Checker Inventory

| Phase | Export | Env override | Fixture tests | verify.sh today |
|-------|--------|--------------|---------------|-----------------|
| 106 | `checkPhase106ParityUatReleaseBoundary(maybeRepoRoot?)` | implicit repo root | `scripts/check-phase106-parity-uat-release-boundary.test.ts` | before 117 historically; not last |
| 117 | `checkPhase117ParityUatReleaseBoundary(maybeRepoRoot?)` | `OPEN_BITCOIN_PHASE117_REPO_ROOT` | directory `test-fixtures.ts` + `claims.ts` | last `check-phase*` after 135 |
| 130 | `checkPhase130ResourceTimeFeePrimitives` | `OPEN_BITCOIN_PHASE130_REPO_ROOT` | yes | before 131 |
| 131 | `checkPhase131RollingFeeExpiryPressure` | `OPEN_BITCOIN_PHASE131_REPO_ROOT` | yes | before 132 |
| 132 | `checkPhase132TypedPackageStagedAdmission` | `OPEN_BITCOIN_PHASE132_REPO_ROOT` | yes | before 133 |
| 133 | `checkPhase133...` | yes | yes | before 134 |
| 134 | `checkPhase134AuthoritativeLifecycle` + `check-phase134-apply-boundaries.ts` | yes | split tests | before 135 |
| 135 | `checkPhase135SnapshotRecovery(maybeRepoRoot)` | **none** | yes | last v2.2 checker; then 117 |
| 136 | **none** | — | — | not wired |
| 137 | **none** | — | — | not wired |
| 138 | **add** | `OPEN_BITCOIN_PHASE138_REPO_ROOT` | add | after 117 |

Phase 117 CLAIM_FILES (live, not archives) [VERIFIED: `scripts/check-phase117-parity-uat-release-boundary.ts:69-79`]:

- `README.md`
- `docs/operator/runtime-guide.md`
- `docs/architecture/status-snapshot.md`
- `docs/architecture/operator-observability.md`
- `docs/parity/catalog/p2p.md`
- `docs/parity/release-readiness.md`
- `docs/parity/production-claim-boundary.md`
- `docs/parity/deviations-and-unknowns.md`
- `docs/parity/support-matrix.md`

Phase 117 `DANGEROUS_CLAIMS` includes `"package relay"` as a substring. Positive verbs (`supports`, `provides`, `enables`, …) plus that substring fail unless a no-claim marker (`deferred`, `does not`, `remain deferred`, …) is in the same clause. [VERIFIED: lines 126–178, 396–418]

Current live wording already uses the safe form: README P2P row says "package relay … remain deferred". D-21 can be stated as "bounded local-package APIs and same-peer 1P1C over ordinary transaction messages" **without** the words `package relay`. That is the smallest D-24 fix: keep 117's deny list; do not freeze the live corpus; do not globally allow `package relay`. Add a Phase 138 mutation that D-21 wording is allowed by both 117 and 138.

117 fixtures seed "Package relay … remain deferred" (`test-fixtures.ts:79`). Keep that seed. Do not change 117 to read live v2.2 REQUIREMENTS.md — it already remaps to the v2.1 archive.

## Last-gate assertions that must change when 138 is last

### Hard last-gate (`requireFinalPhaseChecker` — last `check-phase*` must be 117)

These **will fail** if 138 is appended after 117:

| File | Failure text |
|------|----------------|
| `scripts/check-phase124-milestone-closeout-reconciliation.ts` | `P124 visible/executable verifier final gate must end with bun run scripts/check-phase117-parity-uat-release-boundary.ts` |
| `scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts` | `P129 final gate ... must end with ...117...` |
| `scripts/check-phase130-resource-time-fee-primitives.ts` | `P130 final gate ... must end with ...117...` |
| `scripts/check-phase131-rolling-fee-expiry-pressure.ts` | `P131 final gate ... must end with ...117...` |

Matching tests assert the same string (`check-phase130-*.test.ts:250`, `check-phase131-*.test.ts:139`, `check-phase129-*.test.ts:135`).

**Planner action:** change these helpers to require the last `check-phase*` command to be the Phase 138 **check** (not 117), and keep 117 immediately before 138. Do not weaken "117 still present".

### Adjacent-sequence (will fail if 138 is inserted between 117 and reconciliation)

| File | Contract |
|------|----------|
| `scripts/check-current-documentation-reconciliation.ts` | `VISIBLE_SEQUENCE` / `EXECUTABLE_SEQUENCE` is exact `117 test + 117 check + reconciliation test + reconciliation check` |

**Planner action:** insert the 138 pair into those sequences: `117 → 138 → reconciliation`.

### Subsequence / filtered-token (OK if 138 is after 117 and not in their needle list)

| File | Contract |
|------|----------|
| Phase 117 `checkVerifier` | 116 → 117 → PURE_CORE subsequence |
| Phase 126 / 127 / 128 | own pair precedes 117 |
| Phase 132 `checkVerifierWiring` | 131 → 132 → 117 subsequence |
| Phase 133 `checkVerifierWiring` | 132 → 133 → 117, counted twice (heredoc + run_step) |
| Phase 134 | filtered tokens `133 → 134 → 117` twice |
| Phase 135 | filtered tokens `134 → 135 → 117` twice |

No change required unless a needle accidentally matches `check-phase138`.

### Not last-gate, but Phase 134/135 will fail on D-13/D-17 flips

| File | Lock | Required update **before** flipping rows |
|------|------|------------------------------------------|
| `scripts/check-phase135-snapshot-recovery.ts` | Forces top-level name `"v2 snapshot schema, checkpointing, and recovery"` `in_progress`, checklist `v2-2-snapshot-schema-checkpointing-recovery` `in_progress`, human "In progress", `MPDUR-01 through MPDUR-04 remain pending`, REQUIREMENTS `- [ ] **MPDUR-0X**`, checklist `\| MPDUR-0X \| Pending \|` | Relax to `done` / Complete after evidence is current. Keep deferred-claim denies. |
| `scripts/check-phase134-authoritative-lifecycle/scope.ts` | If MPLIFE is still `- [ ]`, surfaces must stay `in_progress`. `known_gaps` must still contain `"phase 138 owns"`. Forbids `"phase 138 is complete/done/shipped"` on scanned claim surfaces. | Flip MPLIFE to `[x]` first or in the same change as promoting the 134 surface. Keep the phrase "Phase 138 owns" in 134 known_gaps (MPVFY). Never write "Phase 138 is complete" on 134 claim surfaces. |

Phase 132 pins PACK-01..07 in catalog/source but does **not** lock REQUIREMENTS.md Pending boxes. Safe to flip PACK after the 138 checker names evidence.

## Phase 131 2s Instant smoke gate (D-07)

**Location:** `packages/open-bitcoin-bench/src/cases/mempool.rs`

- `SUSTAINED_PRESSURE_MAX_ELAPSED = Duration::from_millis(2_000)`
- `let started = Instant::now();` … `if elapsed > SUSTAINED_PRESSURE_MAX_ELAPSED { return Err(...) }`
- Case id `mempool-policy.sustained-pressure-trim`, N=24 cycles

**Checker pin:** `scripts/check-phase131-rolling-fee-expiry-pressure.ts` `checkPress05OracleAndBench` requires the bench file to contain `SUSTAINED_PRESSURE_MAX_ELAPSED` and the catalog to mention `sustained-pressure-trim`.

**Catalog pin:** `docs/parity/catalog/mempool-policy.md` "Sustained-pressure bounds (PRESS-05)" currently says "2s wall-time ceiling under the default verifier".

**Do not drop PRESS-05 oracle/work-count evidence:**

- Keep `sustained_pressure_oracle_agrees_across_fill_trim_block_decay_expiry_refill_reorg`
- Keep `recompute_resource_ledger` assertions
- Keep `rolling_fee_restarts_at_zero_without_durability` as a cited cell (not the D-04 composition)
- Keep N=24 loop and "one retained entry / rolling fee bumped" checks
- Replace the `Instant` ceiling with work-count assertions (trim cycles, retained count, oracle agreement)
- Update the Phase 131 checker to require those work-count symbols instead of `SUSTAINED_PRESSURE_MAX_ELAPSED`
- Rewrite catalog PRESS-05 to say default smoke is work-count / `threshold_free`; timings are `--full` / UAT only

`scripts/check-benchmark-report.ts` already requires `report.profile.threshold_free === true` and smoke mode `debug`. Do not add latency fields to that contract. [VERIFIED: lines 134–152]

`scripts/run-benchmarks.sh --full` builds `--release` and remains opt-in. Document any timing budget only on that path (D-08).

## Parity surfaces (D-12, D-13, D-14)

### Top-level `docs/parity/index.json` `surfaces[]` today

| name | status |
|------|--------|
| `v2-2-resource-time-fee-primitives` | `in_progress` → promote `done` |
| `v2-2-package-aware-download-orphan-bridge` | `done` — keep |
| `v2-2-authoritative-cross-cache-lifecycle-integration` | `in_progress` → promote `done` |
| `v2 snapshot schema, checkpointing, and recovery` | `in_progress` → promote `done` (checklist id is already `v2-2-snapshot-schema-checkpointing-recovery`) |
| `v2-2-rpc-and-sanitized-operator-evidence` | `in_progress` → promote `done` |
| **missing** `v2-2-typed-package-staged-admission` or equivalent PACK owner | **add** (D-12). Catalog prose already exists in `docs/parity/catalog/mempool-policy.md` "Typed Package Vocabulary and Staged Admission". Prefer a checklist/index row, not a new catalog page. |
| **missing** PRESS owner | **add** (D-12). Catalog PRESS-01..05 prose already exists. Can be `v2-2-rolling-fee-expiry-pressure` owning PRESS-01..05. |
| **missing** Phase 136 surface | **add** `v2-2-receive-independent-maintenance-and-transport-receipts` owning PPKG-04, IBR-01..04 |
| **missing** closeout | **add** `v2-2-parity-uat-release-boundary` owning only MPVFY-01..04 |

### Requirement owners today (checklist / index)

| IDs | Owner today | Action |
|-----|-------------|--------|
| FEEP-01..05 | `v2-2-resource-time-fee-primitives` | Promote `done` |
| PRESS-01..05 | **none** | Add surface + checklist row |
| PACK-01..07 | **none** (catalog prose only) | Add surface + checklist row |
| PPKG-01..03 | `v2-2-package-aware-download-orphan-bridge` `done` | Keep |
| PPKG-04 | **none** | Phase 136 surface |
| MPLIFE-01..04 | `v2-2-authoritative-cross-cache-lifecycle-integration` | Promote `done` after 134 lock allows |
| MPDUR-01..04 | `v2-2-snapshot-schema-checkpointing-recovery` | Promote `done` after 135 lock update |
| IBR-01..04 | **none** | Phase 136 surface |
| MPOBS-01..03 | `v2-2-rpc-and-sanitized-operator-evidence` | Promote `done` |
| MPVFY-01..04 | **none** | New closeout surface |

### Breadcrumb groups the 138 checker should require (D-25)

Existing groups to name, not recreate [VERIFIED: `docs/parity/source-breadcrumbs.json` labels]:

- `mempool-resource-accounting`, `mempool-entry-context`, `mempool-lifecycle`, `mempool-package-policy`, `mempool-package-parity-closure`, `mempool-staged-admission`, `mempool-package-fee-policy`, `network-initial-broadcast-retry-inputs`, `node-initial-broadcast-retry`, `node-package-admission-bridge`, `node-local-package-admission`, `node-mempool-recovery-topology`, `node-mempool-checkpoint-coordinator`, `rpc-package-projection`, `cli-operator-package`, `bench-mempool-policy`

New Rust composition test must join the recovery or retry group that owns its source offender.

### Knots anchors to require on the closeout surface

From CONTEXT and existing v2.2 surfaces:

- `packages/bitcoin-knots/src/txmempool.cpp`
- `packages/bitcoin-knots/src/policy/packages.cpp`
- `packages/bitcoin-knots/src/rpc/mempool.cpp`
- `packages/bitcoin-knots/src/net_processing.cpp`
- plus already-cited `txpackage_tests.cpp`, `mempool_limit.py`, `mempool_persist.py`, `p2p_opportunistic_1p1c.py`

## UAT command forms (D-15, D-16)

Copy `.planning/phases/117-parity-traceability-uat-and-release-guardrails/117-UAT.md`: required deterministic tests with expected/result/evidence; optional public-network review `status: not run`; no default/CI/release gate.

Phase 137 already published the operator forms in `docs/operator/runtime-guide.md` "Phase 137 Package RPC And Operator Review" and `docs/parity/service-operation-expectations.md` (lines 94–103). Reuse these exact strings in `138-UAT.md` and require the Phase 138 checker to find them in the runtime guide (same pattern as Phase 117 `REQUIRED_RUNTIME_COMMANDS`):

```bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -datadir=/tmp/open-bitcoin-mainnet testmempoolaccept '["<hex>"]'
bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -datadir=/tmp/open-bitcoin-mainnet testmempoolaccept '["<hex>"]'
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -datadir=/tmp/open-bitcoin-mainnet submitpackage '["<hex>"]'
bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -datadir=/tmp/open-bitcoin-mainnet submitpackage '["<hex>"]'
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet package dry-run --hex '<hex>'
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet package dry-run --hex '<hex>'
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet package submit --hex '<hex>'
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet package submit --hex '<hex>'
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet status --format json
```

Also require the Phase 138 test/check pair and `bash scripts/verify.sh`. Optional public-network: record `not run`.

Discretion: record optional UAT as `not run`, matching 117-UAT.md, not `pending`.

## Pending-row flip order (D-17) — do not leave leftovers

Flip only after the 138 checker names evidence. Suggested order:

1. Update Phase 135 parity lock and Phase 134 pending→`in_progress` lock.
2. Update last-gate helpers (124/129/130/131) and reconciliation adjacent sequence.
3. Retire Instant gate + Phase 131 PRESS-05 pin + catalog sentence.
4. Backfill index/checklist owners; add closeout surface still `in_progress`.
5. Land checker + 138-UAT.md + composition test.
6. Flip REQUIREMENTS.md boxes: PACK-01..07, PPKG-04, MPDUR-01..04, IBR-01..04, MPOBS-01..03, MPVFY-01..04. Also flip MPLIFE if still `[ ]` in the checkbox list (traceability table already says Complete for FEEP/PRESS/PPKG-01..03/MPLIFE; checkbox section at top still has MPDUR/IBR/MPOBS/PACK pending).
7. Promote surfaces to `done`.
8. Reconcile ROADMAP / PROJECT / STATE. PROJECT.md still says "PPKG-04 and IBR-01 through IBR-04 stay Pending until Phase 138 closeout" and "Phase 137 is next". [VERIFIED: `.planning/PROJECT.md:19`]
9. Do **not** archive. Do **not** run `/gsd-complete-milestone v2.2`.

## Common Pitfalls

### Pitfall 1: Last-gate helpers still require 117

**What goes wrong:** Default verify fails in 124/129/130/131 after 138 is appended.
**Why it happens:** `requireFinalPhaseChecker` takes the last `bun test|run scripts/check-phase\d+` line.
**How to avoid:** Update those four checkers and their mutation tests in the same plan that wires verify.sh.
**Warning signs:** failure text `must end with bun run scripts/check-phase117-...`

### Pitfall 2: Adjacent 117 → reconciliation sequence

**What goes wrong:** `check-current-documentation-reconciliation.ts` fails.
**Why it happens:** `visible.includes("117 test\\n117 check\\nreconciliation test\\nreconciliation check")`.
**How to avoid:** Extend both sequences to include 138.
**Warning signs:** `verifier visible reconciliation order must immediately follow Phase 117`

### Pitfall 3: Phase 135 Pending lock

**What goes wrong:** Promoting MPDUR / snapshot surface fails Phase 135.
**Why it happens:** `PHASE135_DIAGNOSTICS.parity` hard-codes `in_progress` and Pending.
**How to avoid:** Change 135 first, then flip.
**Warning signs:** `P135 parity: evidence stays in progress and MPDUR requirements stay pending`

### Pitfall 4: Phase 117 `package relay` substring

**What goes wrong:** README or runtime-guide "supports package relay" fails 117.
**Why it happens:** `DANGEROUS_CLAIMS` is substring match on positive clauses.
**How to avoid:** Write D-21 without those two words; keep "package relay remains deferred" for FUT-12.
**Warning signs:** `forbidden positive Phase 117 claim: package relay`

### Pitfall 5: Phase 134 "Phase 138 is complete"

**What goes wrong:** 134 scope checker fails during closeout docs.
**Why it happens:** `/\bphase 138 (?:is )?(?:implemented|complete|done|shipped)\b/`
**How to avoid:** Say "Phase 138 owns MPVFY-01 through MPVFY-04" / "Phase 138 closes the v2.2 claim boundary".
**Warning signs:** `P134 scope: Phase 135-138 and broad relay/readiness claims must remain deferred`

### Pitfall 6: Treating VERIFICATION.md as MPVFY-01 proof

**What goes wrong:** D-03 violated; 136/137 cells look covered but are not executable from the checker.
**How to avoid:** Pin file + Rust symbol (+ optional cargo filter string as documentation).
**Warning signs:** checker only greps `.planning/phases/136-*/136-VERIFICATION.md`

### Pitfall 7: Retiring Instant and losing PRESS-05

**What goes wrong:** Oracle or work-count evidence deleted with the 2s gate.
**Why it happens:** Gate and oracle live in the same bench function.
**How to avoid:** Delete only `Instant` / `SUSTAINED_PRESSURE_MAX_ELAPSED`; keep N=24, membership, rolling bump, and the Rust oracle test.

### Pitfall 8: Leftover Pending rows

**What goes wrong:** Phase 129-style reconciliation later (D-17).
**Why it happens:** Surfaces promoted while REQUIREMENTS.md checkboxes stay `[ ]`.
**How to avoid:** Checker asserts every v2.2 ID is `[x]` and maps to exactly one phase/surface before closeout is `done`.

## Code Examples

### Phase 138 checker skeleton

```typescript
// Source: scripts/check-phase117-parity-uat-release-boundary.ts (copy shape)
export function checkPhase138ParityUatReleaseBoundary(maybeRepoRoot?: string): string[] {
  const repoRoot = path.resolve(
    maybeRepoRoot ?? process.env.OPEN_BITCOIN_PHASE138_REPO_ROOT ?? DEFAULT_REPO_ROOT,
  );
  const failures: string[] = [];
  const texts = loadCorpus(repoRoot, failures);
  checkMatrixEvidence(texts, failures);
  checkSurfaceOwnership(texts, failures); // 40 IDs exactly once; MPVFY only on closeout
  checkClaims(texts, failures);           // D-21 required; D-22 denied
  checkVerifier(texts, failures);         // 135 → 117 → 138; 138 last check-phase*
  checkBenchmarks(texts, failures);       // threshold_free; no Instant 2s pin
  checkUatCommands(texts, failures);
  return failures;
}
```

### Allowed vs denied claims

```typescript
// Discretion recommendation — keep 117 deny intact; 138 requires D-21 phrases
const REQUIRED_SCOPED_CLAIMS = [
  "bounded local-package",
  "same-peer 1p1c",
  "ordinary transaction",
  "initial-broadcast-retry",
] as const;

const DENIED_OVERCLAIMS = [
  "general package wire",
  "arbitrary multi-parent",
  "whole-mempool rebroadcast",
  "public relay by default",
  "production relay",
  "guaranteed propagation",
  "public-network ci",
  "production full-node readiness",
  "production-funds",
] as const;
```

### Work-count pins already in-tree

```rust
// Source: packages/open-bitcoin-mempool/src/pool/tests/package_parity_cases/...rs
assert_eq!(full_clone_count, 0, "zero clone");
assert_eq!(full_recompute_count, 0, "zero recompute");
assert_eq!(package_trim_count_for_test(), 1);
```

```rust
// Source: packages/open-bitcoin-node/src/network/tests/maintenance_tick_cases.rs
// maintenance_tick_respects_inspect_256_and_prepare_32_with_leftover_cursor
assert_eq!(first.prepared_count, 32);
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Phase 106 v2.0 closeout last-gate | Phase 117 v2.1 BOUND last-gate after later v2.1 checkers | 2026-07 | 138 repeats this: 117 stays BOUND; 138 becomes last `check-phase*` |
| Phase 131 2s Instant smoke | Work-count + oracle; timings opt-in | this phase (D-07) | Default verify stays hermetic and silicon-independent |
| Per-phase Pending until "verification" | Flip all leftover Pending at 138 after named evidence | D-17, lesson from Phase 129 | Avoids a reconciliation phase |
| "package relay remains deferred" as v2.1 archive truth | Still true for FUT-12; v2.2 claim is D-21 without that phrase | this phase (D-21/D-24) | 117 stays valid |

**Deprecated/outdated:**

- `SUSTAINED_PRESSURE_MAX_ELAPSED` / 2s Instant default-verify gate
- "Phase 117 remains the final changed-path … gate" comment in `verify.sh`
- PROJECT.md "Phase 137 is next" / "PPKG-04 and IBR stay Pending until Phase 138"
- Catalog PRESS-05 "2s wall-time ceiling under the default verifier"
- Phase 135 "MPDUR remain pending" / `in_progress` lock after closeout

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Seeded/generated oracles (`generated_steps`, 25-sparse prospective graph) satisfy MPVFY-01 "randomized graph-oracle" without adding proptest. | Matrix | If discuss-phase meant a statistical fuzzer, one more generated-graph cell would be needed. Existing seeded generators are the in-repo meaning of "randomized". [ASSUMED] |
| A2 | Inserting 138 between 117 and current-documentation-reconciliation is the intended D-20 order. | Verifier | If 138 were placed after reconciliation it would still be last `check-phase*`, but D-20 says after 117; reconciliation's adjacent check would then stay valid without edits. Planner should still update reconciliation if following the recommended order. [ASSUMED] |

**If this table is empty:** All claims in this research were verified or cited — no user confirmation needed.

A1/A2 are low risk and within Claude's Discretion. Do not block planning.

## Open Questions

1. **Exact PACK/PRESS surface IDs**
   - What we know: catalog prose exists; no index/checklist owner.
   - What's unclear: `v2-2-typed-package-staged-admission` vs `v2-2-package-admission`; PRESS as `v2-2-rolling-fee-expiry-pressure` vs attaching PRESS to the Phase 131 catalog section only.
   - Recommendation: Use `v2-2-typed-package-staged-admission` (PACK-01..07) and `v2-2-rolling-fee-expiry-pressure` (PRESS-01..05). Discretion.

2. **Phase 135 top-level `name` vs `id` mismatch**
   - What we know: top-level `surfaces[].name` is `"v2 snapshot schema, checkpointing, and recovery"` while checklist `id` is `v2-2-snapshot-schema-checkpointing-recovery`. Phase 135 checker keys the top-level by the human name.
   - What's unclear: whether to normalize the top-level name to the id when promoting `done`.
   - Recommendation: Keep the human name so Phase 135's lookup does not break; only change the status lock.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Bun | Checker + tests | ✓ | pin 1.3.9 / local 1.3.14 | — |
| Rust/Cargo | Composition test + UAT forms | ✓ | 1.94.1 | — |
| Bazel | UAT command forms | ✓ | 8.6.0 | Document form even if unused in default verify |
| `bash scripts/verify.sh` | Phase gate | ✓ | repo script | — |
| Public network | Optional UAT | n/a | — | Record `not run` |

**Missing dependencies with no fallback:** none.

**Missing dependencies with fallback:** none.

Step 2.6: no blocking external services. This is code/docs/checker work plus at most one hermetic Rust test.

## Security Domain

`security_enforcement` is not set to `false` in `.planning/config.json` (absent = enabled). This phase is a documentation/checker closeout, not a new auth or crypto surface.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | — |
| V3 Session Management | no | — |
| V4 Access Control | no | — |
| V5 Input Validation | yes | Checker path + JSON parse fail-closed; no network |
| V6 Cryptography | no | Do not hand-roll; no new crypto |

### Known Threat Patterns for closeout checkers

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Claim overreach / social-engineering of operators | Spoofing / Elevation | Paragraph-aware deny list; curated corpus; no `.planning/` history scan |
| Path traversal via repo-root override | Tampering | Resolve repo root; read only allow-listed relative files |
| Nondeterministic verify (wall-clock, public net) | Denial of Service / Tampering | Filesystem-only checker; no spawn/fetch; retire Instant gate |
| Fixture leftover dirs | Information Disclosure | `afterEach` `rmSync` temp roots (Phase 117 pattern) |

## Sources

### Primary (HIGH confidence)

- `138-CONTEXT.md` — locked D-01..D-25
- `scripts/check-phase117-parity-uat-release-boundary.ts` + `claims.ts` + `test-fixtures.ts` — export, corpus, DANGEROUS_CLAIMS, orderedLines
- `scripts/verify.sh` — current 135 → 117 → reconciliation order
- `scripts/check-phase124|129|130|131-*.ts` — hard last-gate
- `scripts/check-current-documentation-reconciliation.ts` — adjacent 117 sequence
- `scripts/check-phase134-authoritative-lifecycle/scope.ts` — in_progress / "phase 138 owns" / forbidden "phase 138 is complete"
- `scripts/check-phase135-snapshot-recovery.ts` — MPDUR Pending lock
- `scripts/check-phase131-rolling-fee-expiry-pressure.ts` — PRESS-05 Instant pin
- `packages/open-bitcoin-bench/src/cases/mempool.rs` — 2s Instant gate
- `scripts/check-benchmark-report.ts` — `threshold_free`
- `docs/parity/index.json` — v2.2 surfaces and missing owners
- `docs/parity/checklist.md` — human rows
- Named Rust tests listed in the matrix (mempool, node, network, rpc)
- `117-UAT.md`, `docs/operator/runtime-guide.md` Phase 137 section, `docs/parity/service-operation-expectations.md`
- `136-VERIFICATION.md`, `137-VERIFICATION.md` — runnable symbol lists, not sufficient alone
- `.planning/config.json` — `nyquist_validation: false`

### Secondary (MEDIUM confidence)

- Phase 132/133/126/127/128 verifier subsequence behavior inferred from `orderedLines` / filtered tokens (verified by reading those functions; not re-run)

### Tertiary (LOW confidence)

- A1 seeded-oracle ≡ "randomized" (flagged in Assumptions Log)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new libraries; versions probed
- Architecture: HIGH — existing closeout pattern and exact file locks read
- Pitfalls: HIGH — last-gate, 135 lock, 117 claim collision, Instant pin all confirmed in source

**Research date:** 2026-08-22
**Valid until:** 2026-09-21 (30 days; repo-local closeout, not a fast-moving ecosystem)

## Planner task sketch (non-binding)

1. **Wave 0 — unlock flips:** relax Phase 135 parity lock; update 124/129/130/131 last-gate; extend reconciliation sequence; update `verify.sh` comment.
2. **Wave 1 — MPVFY-01/02 evidence:** add the one restart composition test + breadcrumb; retire Instant gate; retarget Phase 131 PRESS-05 pin and catalog sentence.
3. **Wave 2 — MPVFY-03 owners:** backfill PACK/PRESS/136 surfaces; add `v2-2-parity-uat-release-boundary`; write `138-UAT.md`; refresh runtime-guide/README with D-21 wording (no bare `package relay`).
4. **Wave 3 — MPVFY-04 checker:** new Bun pair, fixtures, wire after 117; assert 40-requirement exactly-once ownership, Knots anchors, breadcrumbs, UAT commands, work-count bench wiring, verifier order.
5. **Wave 4 — close the ledger:** flip Pending rows; promote surfaces to `done`; reconcile ROADMAP/PROJECT/STATE; run `bash scripts/verify.sh`. Do not archive.
