---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 145-2026-09-18T03-36-29
generated_at: 2026-09-18T03:38:24.949Z
---

# Phase 145: Parity Roots and No-Claim Guardrails - Context

**Gathered:** 2026-09-18
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Parity evidence is auditable and the v2.3 claim cannot be read as prune,
assumeutxo, archive, or production readiness.

This phase delivers CSVFY-01 and CSVFY-02. It inventories and gap-fills
Phase 139–144 parity roots, cites pinned Knots coins, flush, manager, and
serve-path anchors (or documents intentional differences), adds a committed
UAT package, and installs a last-gate no-claim checker.

This is a closeout and guardrail phase. It must not add prune/archive product
modes, assumeutxo/assumevalid/IBD snapshot shortcuts, compact-filter or BIP37
serving, public/default serving or relay, public-network CI, production
readiness, production-funds wallet use, LevelDB or rust-bitcoin, or a new
`getblock` product. It must not archive v2.3; milestone completion remains
`/gsd-complete-milestone v2.3` after this phase passes. Default
`bash scripts/verify.sh` stays deterministic. Historical `.planning/phases/`
directories remain tracked.

</domain>

<decisions>
## Implementation Decisions

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

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Active milestone contract

- `.planning/ROADMAP.md` — Phase 145 goal, CSVFY-01, CSVFY-02, success
  criteria, 139 → 145 execution order
- `.planning/REQUIREMENTS.md` — CSVFY-01, CSVFY-02, leftover Pending
  CACHE-01 / MGR-03 rows, FUT-18 through FUT-26 exclusions
- `.planning/PROJECT.md` — v2.3 claim boundary; prune/archive, assumeutxo,
  compact filters, public defaults, and production claims stay deferred
- `.planning/STATE.md` — Locked v2.3 decisions: storage-first coins,
  honest availability, historical `.planning/phases/` stay tracked
- `.planning/CONVENTIONS.md` — Evidence-based parity claims and quiet
  operator wording
- `AGENTS.md` — Repo-local verification, UAT Cargo/Bazel forms, parity
  breadcrumb rules, and historical phase-directory tracking
- `AGENTS.bright-builds.md` — Bright Builds workflow, architecture,
  testing, and verification defaults
- `standards/core/verification.md` — Repo-native verification; prefer
  `bash scripts/verify.sh`
- `standards/core/architecture.md` — Functional core / imperative shell
- `standards/core/code-shape.md` — Control flow and file-size triggers
- `standards/core/testing.md` — Arrange/Act/Assert unit-test shape
- `standards/languages/typescript-javascript.md` — Bun/TypeScript checker
  rules
- `standards-overrides.md` — No active exceptions

### Locked prior closeout precedents

- `.planning/phases/138-parity-adversarial-pressure-restart-and-release-guardrails/138-CONTEXT.md`
  — Last-gate checker, D-21 sentence, curated corpus, no milestone archive
- `.planning/phases/117-parity-traceability-uat-and-release-guardrails/117-CONTEXT.md`
  — v2.1 BOUND closeout pattern
- `.planning/phases/106-parity-traceability-uat-and-release-boundary-guardrails/106-CONTEXT.md`
  — v2.0 inventory, fixed-corpus no-claim checkers, verify.sh contract
- `.planning/phases/144-operator-flush-and-availability-evidence/144-CONTEXT.md`
  — CSOBS surfaces already shipped; Phase 145 owns no-claim guardrails
- `.planning/phases/143-honest-stored-block-availability/143-CONTEXT.md`
  — Reserved `Pruned`, payload-byte probe, no getblock
- `.planning/phases/142-manager-flush-lifecycle-and-restart/142-CONTEXT.md`
  — CanFlush, ordered flush, restart from coins best-block
- `.planning/phases/141-durable-fjall-coins-adapter/141-CONTEXT.md`
  — Per-outpoint Fjall coins, leftover snapshot non-authority
- `.planning/phases/140-pure-flush-policy-and-typed-decisions/140-CONTEXT.md`
  — Injectable Flush/Sync/RefuseDiskSpace decisions
- `.planning/phases/139-coins-view-cache-contract-and-engine-apply/139-CONTEXT.md`
  — Pure CoinsView / CoinsCache; no Fjall in core

### Parity, release, and operator roots

- `README.md`
- `docs/parity/README.md`, `docs/parity/index.json`,
  `docs/parity/checklist.md`, `docs/parity/source-breadcrumbs.json`
- `docs/parity/catalog/chainstate.md`, `docs/parity/catalog/p2p.md`,
  `docs/parity/catalog/rpc-cli-config.md`,
  `docs/parity/catalog/operator-runtime-release-hardening.md`,
  `docs/parity/catalog/verification-harnesses.md`
- `docs/parity/release-readiness.md`,
  `docs/parity/production-claim-boundary.md`,
  `docs/parity/deviations-and-unknowns.md`, `docs/parity/support-matrix.md`
- `docs/operator/runtime-guide.md`,
  `docs/architecture/status-snapshot.md`,
  `docs/architecture/operator-observability.md`

### Existing checker and verifier integration

- `scripts/check-phase138-parity-uat-release-boundary.ts` and
  `scripts/check-phase138-parity-uat-release-boundary/` — primary closeout
  template
- `scripts/check-phase117-parity-uat-release-boundary.ts` — v2.1 last-gate
- `scripts/check-phase144-operator-flush-availability-evidence.ts` — current
  last v2.3 checker; Phase 145 wires immediately after it
- `scripts/check-parity-breadcrumbs.ts`, `scripts/verify.sh`

### Bitcoin Knots anchors

- `packages/bitcoin-knots/src/coins.h`,
  `packages/bitcoin-knots/src/coins.cpp` — CCoinsView / CCoinsViewCache,
  DIRTY/FRESH, BatchWrite/Flush
- `packages/bitcoin-knots/src/validation.cpp` — FlushStateToDisk,
  GetCoinsCacheSizeState
- `packages/bitcoin-knots/src/node/chainstate.cpp` — manager flush,
  CanFlush, coins-db init
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — HaveBlockData vs
  missing payload

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- Phase 138 checker/test directory: closest aggregate parity/UAT/release-boundary
  template with fixed-corpus, ownership, anchor, command, verifier-order, and
  overclaim checks.
- Phase 144 checker: current last v2.3 `check-phase*` pair in `verify.sh`.
- `docs/parity/index.json` and `docs/parity/checklist.md`: no v2.3 surfaces
  exist yet — Phases 139–144 still need backfill before the closeout row.
- `docs/parity/catalog/chainstate.md`: still snapshot-era; must be refreshed
  so leftover snapshot blobs are not live UTXO truth.

### Established Patterns

- Closeout phases add one aggregate Bun checker/test pair rather than rewriting
  every completed phase checker.
- Claim checks use curated current docs and mutation fixtures, not history-wide
  scans.
- Machine-readable parity surfaces and the human checklist mirror exactly-once
  requirement ownership.
- Public-network review stays optional and untracked; default verification
  stays deterministic and local.
- Previous-milestone D-21 sentences stay in their required files when README
  current-state copy advances.

### Integration Points

- Extend v2.3 entries in `docs/parity/index.json`, `docs/parity/checklist.md`,
  and `docs/parity/catalog/chainstate.md` (serve-path also in `catalog/p2p.md`
  where HaveBlockData already lives).
- Tighten relevant groups in `docs/parity/source-breadcrumbs.json` where
  semantic coins/flush/manager/serve-path anchors are missing.
- Add the Phase 145 checker pair and wire it after Phase 144 in
  `scripts/verify.sh`.
- Refresh README, release-readiness, production-boundary, runtime-guide, and
  chainstate catalog wording around the bounded v2.3 claim.
- Commit the Phase 145 UAT package and reconcile current-milestone metadata
  after verification evidence is current.

</code_context>

<specifics>
## Specific Ideas

Preferred release wording: "disk-backed per-outpoint coins, typed cache-flush
policy, fuller chainstate-manager behavior for the single active chainstate,
and honest stored-block availability that serves or reports a stored block
only when the payload bytes are present."

Required operator forms include
`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
and
`bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`,
plus explicit daemon and RPC CLI forms where the UAT workflow needs them.

A public-network UAT item may truthfully say "not run" without turning
Phase 145 verification into a failure.

Do not delete historical `.planning/phases/` directories. Do not archive
the milestone from this phase.

</specifics>

<deferred>
## Deferred Ideas

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

</deferred>

---

*Phase: 145-parity-roots-and-no-claim-guardrails*
*Context gathered: 2026-09-18*
