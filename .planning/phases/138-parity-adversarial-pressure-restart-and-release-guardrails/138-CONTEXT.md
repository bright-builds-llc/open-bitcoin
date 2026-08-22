---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 138-2026-08-22T04-13-58
generated_at: 2026-08-22T04:16:49.564Z
---

# Phase 138: Parity, Adversarial Pressure, Restart, and Release Guardrails - Context

**Gathered:** 2026-08-22
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Contributors and operators have deterministic proof that the integrated v2.2
behavior matches its pinned Knots anchors, remains bounded, and does not
broaden release claims.

This phase delivers MPVFY-01 through MPVFY-04. It inventories and gap-fills
Phase 130–137 evidence, adds one v2.2 closeout parity surface, a committed
UAT package, documented work-count benchmarks, and a last-gate claim checker.

This is a closeout and guardrail phase. It must not add a general package
wire protocol, arbitrary multi-parent assembly, whole-mempool rebroadcast,
public/default/production relay, guaranteed propagation, public-network CI,
or production-readiness claims. It must not archive v2.2; milestone
completion remains `/gsd-complete-milestone v2.2` after this phase passes.

</domain>

<decisions>
## Implementation Decisions

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

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Active milestone contract

- `.planning/ROADMAP.md` — Phase 138 goal, success criteria, and
  130 → 138 execution order.
- `.planning/REQUIREMENTS.md` — MPVFY-01 through MPVFY-04, still-Pending
  PACK/PPKG-04/MPDUR/IBR/MPOBS rows, and FUT-12 through FUT-17
  exclusions.
- `.planning/PROJECT.md` — v2.2 claim boundary; PPKG-04 and IBR deferred
  to Phase 138 closeout.
- `.planning/STATE.md` — Locked v2.2 decisions: local package APIs and
  same-peer 1P1C only; persist canonical entries and reset rolling fee
  on restart; hermetic default verification.
- `.planning/CONVENTIONS.md` — Evidence-based parity claims and quiet
  operator wording.
- `AGENTS.md` — Repo-local verification, UAT Cargo/Bazel forms, and
  parity breadcrumb rules.

### Prior closeout precedents

- `.planning/phases/106-parity-traceability-uat-and-release-boundary-guardrails/106-CONTEXT.md`
  — v2.0 inventory, fixed-corpus no-claim checkers, and verify.sh
  contract.
- `.planning/phases/117-parity-traceability-uat-and-release-guardrails/117-CONTEXT.md`
  — v2.1 closeout pattern: backfill surfaces, dedicated BOUND owner,
  117-UAT.md, last-gate checker, no archive.
- `.planning/phases/117-parity-traceability-uat-and-release-guardrails/117-UAT.md`
  — Committed UAT package shape to copy.
- `.planning/phases/129-integration-guardrails-and-milestone-reconciliation/129-CONTEXT.md`
  — Why leftover Pending rows later forced a reconciliation phase; do
  not repeat that for v2.2.
- `.planning/phases/88-deterministic-claim-guardrails/88-CONTEXT.md` —
  Curated claim corpus, paragraph-aware allow/deny, no `.planning/`
  history scan.

### Implementation evidence to inventory

- `.planning/phases/130-resource-time-and-fee-primitives/130-CONTEXT.md`
  — Distinct resource/fee roles and fake-clock contracts.
- `.planning/phases/131-rolling-fee-expiry-and-descendant-eviction-core/131-CONTEXT.md`
  — Pressure trim, rolling decay, expiry, and oracle/perf bounds.
- `.planning/phases/132-typed-package-vocabulary-and-staged-admission/132-CONTEXT.md`
  — Package shape, dry-run, submit, and final-membership rules.
- `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md`
  — Bounded same-peer 1P1C over ordinary transaction messages.
- `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-CONTEXT.md`
  — Sole mutation authority and lifecycle projection.
- `.planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md`
  — Durable source-only snapshots, derived rebuild, rolling-fee reset.
- `.planning/phases/136-receive-independent-maintenance-and-transport-receipts/136-CONTEXT.md`
  — Initial-broadcast-retry, TransportWritten clear, parent-before-child
  fanout.
- `.planning/phases/137-rpc-and-sanitized-operator-evidence/137-CONTEXT.md`
  — Knots package RPC plus identifier-free shared evidence.

### Parity, benchmarks, and operator roots

- `docs/parity/index.json` — Machine-readable surface and requirement
  ownership root.
- `docs/parity/checklist.md` — Human review of surface status.
- `docs/parity/source-breadcrumbs.json` — Required Rust source anchors.
- `docs/parity/catalog/mempool-policy.md` — Package, pressure, and
  recovery catalog prose.
- `docs/parity/catalog/rpc-cli-config.md` — `testmempoolaccept` /
  `submitpackage` claim boundary.
- `docs/parity/release-readiness.md` — Release-review handoff.
- `docs/parity/production-claim-boundary.md` — Production-readiness
  deny list.
- `docs/parity/support-matrix.md` — Supported versus deferred surfaces.
- `docs/parity/benchmarks.md` — `threshold_free` default-smoke contract.
- `docs/parity/service-operation-expectations.md` — Repo-local Cargo and
  Bazel operator command forms.
- `docs/operator/runtime-guide.md` — Current operator wording that must
  stay scoped.
- `README.md` — Contributor-facing current-state claim.

### Verification wiring

- `scripts/verify.sh` — Default deterministic contract; Phase 117 is
  currently last after Phase 135.
- `scripts/check-phase117-parity-uat-release-boundary.ts` — Last-gate
  export/fixture pattern to copy, not absorb.
- `scripts/check-parity-breadcrumbs.ts` — Breadcrumb verification.
- `scripts/run-benchmarks.sh` — Opt-in bench runner.
- `scripts/check-benchmark-report.ts` — Benchmark report schema and
  `threshold_free` check.

### Pinned Bitcoin Knots behavior

- `packages/bitcoin-knots/src/txmempool.cpp` — Rolling minimum fee,
  expiry, and descendant-score trim.
- `packages/bitcoin-knots/src/policy/packages.cpp` — Package limits and
  validation order.
- `packages/bitcoin-knots/src/rpc/mempool.cpp` — `testmempoolaccept` and
  `submitpackage`.
- `packages/bitcoin-knots/src/net_processing.cpp` — Opportunistic
  same-peer 1P1C over ordinary transaction messages and unbroadcast
  retry.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `scripts/check-phase117-parity-uat-release-boundary.ts` — Exported
  `checkPhase117ParityUatReleaseBoundary(maybeRepoRoot?)` returning
  `string[]`; curated corpus; mutation fixtures.
- `scripts/check-phase106-parity-uat-release-boundary.ts` — Earlier
  closeout checker to reuse for corpus and UAT-command assertions.
- `scripts/check-phase132-typed-package-staged-admission.ts` — Existing
  v2.2 package claim/parity pins.
- `scripts/check-phase135-snapshot-recovery.ts` — Existing recovery and
  restart-contract pins.
- `scripts/check-benchmark-report.ts` — Default-smoke `threshold_free`
  enforcement.
- Phase 131 sustained-pressure oracle tests — Work-count and
  recomputation evidence; also the 2s `Instant` smoke gate to retire
  from default verify.
- Phase 136 retry-worker fake-clock tests — 10-to-15-minute injected
  cycles and recovery remint.

### Established Patterns

- Closeout phases inventory existing evidence, add a mutation-tested Bun
  checker, refresh parity/UAT/docs, and leave runtime behavior alone.
- Checkers export a pure function, take an optional repo-root override,
  and never spawn the network or the default verifier.
- `verify.sh` visible `VERIFY_COMMAND_ORDER` and executable `run_step`
  chain must stay in lockstep.
- Default verification is hermetic: no public-network, soak,
  service-manager, or wall-clock release gates.
- New first-party Rust source or test files need parity breadcrumbs.
  TypeScript checkers do not.

### Integration Points

- `scripts/verify.sh` — Insert the Phase 138 test+check pair after Phase
  117 so 138 is the last `check-phase*` gate.
- `docs/parity/index.json` and `docs/parity/checklist.md` — Backfill 136
  / PACK / PRESS owners, promote completed surfaces, add the MPVFY
  closeout surface.
- `README.md`, `docs/parity/release-readiness.md`, and
  `docs/operator/runtime-guide.md` — State the D-21 scoped claim without
  tripping Phase 117's archived package-relay deny unless 117's corpus
  is frozen or narrowly allowed.
- `.planning/REQUIREMENTS.md` — Flip implemented-Pending rows only after
  named evidence exists.
- Existing Phase 130–137 Rust fixtures — Inventory targets for the 4×6
  MPVFY-01 matrix.

</code_context>

<specifics>
## Specific Ideas

- Follow Phase 117 so closely that a reviewer can diff closeout
  artifacts: one new surface, one UAT package, one last-gate checker,
  copy-pasteable Cargo/Bazel commands, no archive.
- The truthful v2.2 sentence is narrow: local package APIs and bounded
  same-peer 1P1C over ordinary transaction messages, plus initial
  broadcast retry of local unbroadcast members. Do not say "package
  relay" without those qualifiers.
- Restart proof is a fake-clock composition, not a crash-soak: rebuild
  derived state, reset rolling fee, keep canonical entries and local
  unbroadcast, remint retry timers.

</specifics>

<deferred>
## Deferred Ideas

- `/gsd-complete-milestone v2.2` archival — after Phase 138 verification
  passes.
- General package wire / BIP331 (FUT-12).
- Arbitrary multi-parent or cluster-mempool policy (FUT-13).
- Public/default production relay, public-network CI, guaranteed
  propagation, and production-readiness claims (FUT-15 through FUT-17).

None — discussion stayed within phase scope otherwise.

</deferred>

---

*Phase: 138-parity-adversarial-pressure-restart-and-release-guardrails*
*Context gathered: 2026-08-22*
