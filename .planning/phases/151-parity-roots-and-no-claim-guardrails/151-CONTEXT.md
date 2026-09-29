---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 151-2026-09-29T21-06-33
generated_at: 2026-09-29T21:08:13.939Z
---

# Phase 151: Parity Roots and No-Claim Guardrails - Context

**Gathered:** 2026-09-29
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Parity evidence is auditable and the v2.4 claim cannot be read as archive
serving, assumeutxo, BIP37, public defaults, or production readiness.

This phase delivers GRD-01. It inventories and gap-fills Phase 146–150
parity roots, cites pinned Knots prune anchors, documents the intentional
Fjall key versus `blk`/`rev` file difference, adds a committed UAT package,
and installs a last-gate no-claim checker.

This is a closeout and guardrail phase. It must not add a `blk`/`rev`
flat-file store, archive-node serving, assumeutxo or a second chainstate,
BIP37 or compact-filter serving, public serving or relay defaults,
public-network CI as a release gate, production full-node readiness,
production-funds wallet claims, LevelDB or rust-bitcoin, txindex combined
with prune, or `-pruneduringinit`. It must not archive v2.4; milestone
completion remains `/gsd-complete-milestone v2.4` after this phase passes.
Default `bash scripts/verify.sh` stays deterministic. Historical
`.planning/phases/` directories remain tracked.

</domain>

<decisions>
## Implementation Decisions

### Parity surface ownership

- **D-01:** Phase 151 canonically owns only GRD-01. Phases 146 through 150
  remain the exactly-once owners of SNAP, PRUN, LOCK, UNLK, SERV, LABL, and
  OPER even when the Phase 151 checker aggregates their evidence.
- **D-02:** Do not collapse v2.4 evidence into one catalog entry.
  `docs/parity/index.json` remains the machine-readable root.
  `docs/parity/checklist.md`, `docs/parity/catalog/chainstate.md`,
  `docs/parity/catalog/p2p.md` (limited serving),
  `docs/parity/catalog/rpc-cli-config.md` (operator prune), and
  `docs/parity/release-readiness.md` remain the human review roots. Do not
  create a competing closeout manifest.
- **D-03:** Backfill missing machine-readable owners before closeout. Add
  distinct `docs/parity/index.json` and checklist surfaces for Phases
  146–150 when they are still absent, then add one closeout surface
  `v2-4-parity-roots-and-no-claim-guardrails` that owns only GRD-01.
  Suggested earlier surface ids, matching phase names:
  `v2-4-wallet-leftover-snapshot-cutover` (SNAP-01),
  `v2-4-pure-prune-policy-and-lock-windows` (PRUN-01, PRUN-02, PRUN-03,
  LOCK-01), `v2-4-fjall-payload-unlink-and-have-pruned` (UNLK-01, UNLK-02,
  UNLK-03), `v2-4-limited-serving-and-honest-pruned-labels` (SERV-01,
  SERV-02, SERV-03, LABL-01), and
  `v2-4-operator-prune-surfaces-and-evidence` (OPER-01, OPER-02, OPER-03,
  LOCK-02). Prefer index and checklist rows over new catalog pages when
  `catalog/chainstate.md`, `catalog/p2p.md`, or `catalog/rpc-cli-config.md`
  already has the prose home.
- **D-04:** Promote completed `in_progress` v2.4 surfaces to `done` after
  their evidence roots are current. Preserve existing earlier-milestone
  `done` surfaces, including the v2.3 closeout surface. Planning artifacts
  may point to canonical parity roots but must not become parallel sources
  of requirement ownership.
- **D-05:** Review `docs/parity/source-breadcrumbs.json` semantically, not
  only mechanically. Tighten or split broad or explicit-`none` groups when
  they hide a defensible v2.4 Knots prune, lock, unlink, have-pruned, or
  limited-serving anchor, while preserving `none` only where no honest
  source anchor exists.

### Knots anchors and the Fjall difference

- **D-06:** Parity roots must cite these pinned Knots anchors, or document
  an intentional difference next to the claim:
  `packages/bitcoin-knots/src/validation.h` (`MIN_BLOCKS_TO_KEEP`,
  `MIN_DISK_SPACE_FOR_BLOCK_FILES`),
  `packages/bitcoin-knots/src/validation.cpp` (`GetPruneRange` and the
  manual versus automatic height checks),
  `packages/bitcoin-knots/src/node/blockmanager_args.cpp`
  (`ParsePruneOption`),
  `packages/bitcoin-knots/src/node/blockstorage.h` (`PruneLockInfo`),
  `packages/bitcoin-knots/src/node/blockstorage.cpp`
  (`DoPruneLocksForbidPruning`, `PRUNE_LOCK_BUFFER`, `FindFilesToPrune`,
  unlink / `m_have_pruned`), and the protocol plus RPC anchors for
  `NODE_NETWORK_LIMITED`, `pruneblockchain`, and the `getblockchaininfo`
  prune quartet. Add the matching Knots tests when they are the honest
  observable-behavior anchors. Researcher confirms the exact symbol names
  in the pinned tree before the checker freezes them.
- **D-07:** Record the already-locked layout difference instead of implying
  file-level source parity: eligible heights lose paired Fjall block and
  undo keys. Open Bitcoin does not introduce a Knots `blk`/`rev` flat-file
  store. The difference is a documented parity difference, not a second
  block store.
- **D-08:** Refresh `docs/parity/catalog/chainstate.md` so the current v2.4
  claim is height-window prune on Fjall keys, with v2.3 disk-backed coins
  and honest availability labeled as the foundation. `Pruned` is no longer
  a reserved unused label: status and RPC report `Pruned` only when
  have-pruned is set and the payload is gone. A missing payload without
  prune stays `Unavailable`.

### Last-gate no-claim checker

- **D-09:** Add a new Phase 151 Bun/TypeScript checker pair as the last
  `check-phase*` no-claim gate. Follow the Phase 145 export shape:
  `checkPhase151...(maybeRepoRoot?)` returning `string[]` failures, plus
  fixture mutation tests and an `OPEN_BITCOIN_PHASE151_REPO_ROOT` override.
- **D-10:** Wire the new pair into `scripts/verify.sh` immediately after
  the Phase 145 closeout steps, updating the ordering comment,
  `VERIFY_COMMAND_ORDER` heredoc, and live `run_step` chain together.
  Phase 145 remains the v2.3 closeout gate. Phase 151 becomes the v2.4
  closeout gate. Do not reorder unrelated earlier-milestone checkers.
  Phases 146–150 did not add their own `scripts/verify.sh` steps; this
  phase does not backfill a checker per earlier phase.
- **D-11:** Use a curated current claim-bearing corpus (README, runtime
  guide, parity checklist, chainstate / p2p / rpc-cli catalogs,
  release-readiness, production-claim-boundary, support-matrix, operator
  docs). Do not scan historical `.planning/` prose or milestone archives
  as a blocking surface.
- **D-12:** Keep Phase 145's required v2.3 D14 sentence intact in
  `README.md` and `docs/operator/runtime-guide.md`. Add the v2.4 scoped
  prune sentence without removing the v2.3 sentence. Phase 145 currently
  treats positive `prune-mode` wording as a denied overclaim. Update that
  checker only far enough to accept the exact successor v2.4 prune
  sentence and its documented Fjall-versus-`blk`/`rev` difference. Keep
  denying archive serving, assumeutxo, BIP37, public defaults, and
  production-readiness claims. Do not globally allow unscoped `archive`,
  `assumeutxo`, or `production readiness`.
- **D-13:** The aggregate checker validates all 17 v2.4 requirements
  exactly once, required Phase 146–151 parity surfaces, concrete Knots
  anchors from D-06, the Fjall-versus-`blk`/`rev` difference, relevant
  breadcrumb groups, exact repo-local UAT commands, both visible and
  executable verifier ordering, and that historical `.planning/phases/`
  directories remain tracked.

### Allowed scoped claim versus denied overclaims

- **D-14:** The allowed scoped claim sentence, copied verbatim into
  `README.md` and `docs/operator/runtime-guide.md`, is: Knots-aligned
  prune on the single active chainstate deletes old block and undo
  payloads inside a height window by removing Fjall keys, advertises
  `NODE_NETWORK_LIMITED`, and reports `Pruned` only after a durable
  delete.
- **D-15:** Companion allowed wording includes the intentional
  Fjall-versus-`blk`/`rev` difference, the 288-block keep, the 550 MiB
  minimum automatic target, the 10-block lock buffer, manual-prune
  refusal inside the keep window, have-pruned only after a durable
  delete, fail-closed interrupted prune, sanitized prune support
  evidence, and hermetic default verification.
- **D-16:** The checker must reject positive claims for archive-node or
  production-scale historical serving, assumeutxo, assumevalid, or a
  second chainstate, BIP37 bloom serving, compact-filter serving,
  public serving or relay by default, public-network CI as a release
  gate, production full-node readiness, production service operation,
  production-funds wallet safety, a `blk`/`rev` flat-file store, LevelDB
  `chainstate/` live import or export, automatic destructive reindex,
  txindex combined with prune, and `-pruneduringinit`.
- **D-17:** Explicit deferred, unsupported, future-gated, no-claim, and
  opt-in-UAT wording remains valid. Negative fixtures must prove both
  rejection of overclaims and acceptance of the scoped v2.4 prune
  sentence plus deferred wording.

### UAT, default verification, and historical phase dirs

- **D-18:** Create
  `.planning/phases/151-parity-roots-and-no-claim-guardrails/151-UAT.md`
  as the committed UAT package. Required tests stay deterministic.
  Public-network review may be recorded as not run and must never become
  a default, CI, or release gate.
- **D-19:** UAT guidance must provide copy-pasteable repo-local Cargo and
  Bazel forms for the prune status, manual prune, and prune-lock
  commands Phase 150 actually shipped:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`, plus
  the matching `open-bitcoin-cli` / `open-bitcoind` forms where those
  binaries are the operator surface. Researcher confirms the exact argv
  from the shipped CLI before the UAT file freezes it. Do not rely only
  on the installed `open-bitcoin` alias.
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

- **D-22:** Flip leftover Pending v2.4 requirement rows, including GRD-01
  and any ROADMAP table rows that still say Pending for OPER-01, OPER-02,
  OPER-03, or LOCK-02, only after the checker can name their evidence.
  `.planning/REQUIREMENTS.md` already marks OPER and LOCK-02 Complete;
  reconcile the ROADMAP table to that evidence instead of reopening
  Phase 150 implementation. Do not leave leftover Pending rows that would
  force a later reconciliation phase.
- **D-23:** Reconcile ROADMAP, REQUIREMENTS, PROJECT, and STATE to agree
  that v2.4 implementation and closeout evidence are complete. Use
  `gsd-tools.cjs` mutation commands for STATE and ROADMAP updates rather
  than direct edits where the CLI owns those changes.
- **D-24:** Phase 151 may close current-milestone traceability and
  release-boundary evidence, but it must not archive v2.4 or invent a
  milestone-audit workflow. Milestone completion remains
  `/gsd-complete-milestone v2.4` after this phase passes.
- **D-25:** Refresh `README.md` with a concise current-state description
  and links to the canonical evidence roots. Contributor-facing copy stays
  quiet, factual, and evidence-focused. Use
  `docs/parity/release-readiness.md` as the v2.4 release-review handoff.
  Do not introduce a disconnected changelog tree.
- **D-26:** Do not require runtime prune, serving, or operator behavior
  changes unless a named evidence gap has no honest existing proof.
  Prefer catalog, checker, breadcrumb, and claim-copy fixes. Preserve
  Phase 145's v2.3 D14 sentence and Phase 150's shipped operator
  surfaces. Do not re-derive prune policy, unlink, or serving in this
  phase.

### Claude's Discretion

The planner may choose exact checker helper names, fixture organization,
paragraph-classification implementation, the smallest honest breadcrumb or
catalog-owner splits, exact doc section placement, the precise Knots
symbol spellings once research confirms them, and whether optional UAT
items are recorded as pending or not run. Prefer small pure TypeScript
helpers with focused tests and the existing Phase 145 checker pattern.
Do not spend discretion on runtime behavior changes, a `blk`/`rev` store,
or broader claims.

### Folded Todos

None — `todo match-phase 151` returned no matches.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Active milestone contract

- `.planning/ROADMAP.md` — Phase 151 goal, GRD-01, success criteria, and
  the 146 → 151 execution order. The requirement table still lists
  OPER-01 through LOCK-02 as Pending; REQUIREMENTS.md already marks them
  Complete.
- `.planning/REQUIREMENTS.md` — GRD-01, the Fjall-versus-`blk`/`rev`
  difference, and FUT-19 through FUT-27 exclusions
- `.planning/PROJECT.md` — v2.4 prune milestone; archive, assumeutxo,
  BIP37, public defaults, and production claims stay deferred
- `.planning/STATE.md` — Current progress and locked prior decisions
- `.planning/CONVENTIONS.md` — Evidence-based parity claims and quiet
  operator wording
- `AGENTS.md` — Repo-local verification, UAT Cargo and Bazel forms,
  parity breadcrumb rules, and historical phase-directory tracking
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

### Locked prior closeout and prune decisions

- `.planning/phases/145-parity-roots-and-no-claim-guardrails/145-CONTEXT.md`
  — v2.3 last-gate checker, D14 sentence, curated corpus, no milestone
  archive. Phase 145 still denies positive `prune-mode` claims.
- `.planning/phases/150-operator-prune-surfaces-and-evidence/150-CONTEXT.md`
  — Operator prune surfaces already shipped; Phase 151 owns GRD-01
- `.planning/phases/149-limited-serving-and-honest-pruned-labels/149-CONTEXT.md`
  — `NODE_NETWORK_LIMITED` and honest `Pruned` versus `Unavailable`
- `.planning/phases/148-fjall-payload-unlink-and-have-pruned/148-CONTEXT.md`
  — Paired Fjall delete, have-pruned after durable success, fail-closed
  interrupted prune
- `.planning/phases/147-pure-prune-policy-and-lock-windows/147-CONTEXT.md`
  — 288 keep, 550 MiB minimum, 10-block lock buffer, pure planner
- `.planning/phases/146-wallet-leftover-snapshot-cutover/146-CONTEXT.md`
  — Leftover snapshot bytes stay non-authoritative
- `.planning/phases/138-parity-adversarial-pressure-restart-and-release-guardrails/138-CONTEXT.md`
  — Earlier last-gate pattern Phase 145 followed

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
- `docs/operator/runtime-guide.md`

### Existing checker and verifier integration

- `scripts/check-phase145-parity-uat-release-boundary.ts` and
  `scripts/check-phase145-parity-uat-release-boundary/` — primary closeout
  template, including the current `prune-mode` denial in `constants.ts`
- `scripts/verify.sh` — Phase 145 is the current v2.3 closeout gate.
  Phases 146–150 did not add `check-phase146` through `check-phase150`
  steps.

### Bitcoin Knots anchors

- `packages/bitcoin-knots/src/validation.h` — `MIN_BLOCKS_TO_KEEP` (288)
  and `MIN_DISK_SPACE_FOR_BLOCK_FILES` (550 MiB)
- `packages/bitcoin-knots/src/validation.cpp` — prune range and manual
  versus automatic height checks
- `packages/bitcoin-knots/src/node/blockmanager_args.cpp` —
  `ParsePruneOption`
- `packages/bitcoin-knots/src/node/blockstorage.h` — `PruneLockInfo`
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — prune locks,
  file unlink, and have-pruned

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- Phase 145 checker and test directory is the closest aggregate
  parity, UAT, and release-boundary template. It already checks fixed
  corpus, ownership, Knots anchors, UAT commands, verifier order, and
  overclaims.
- `scripts/verify.sh` places the Phase 145 test and check immediately
  after Phase 144. That is the insertion point for Phase 151.
- `docs/parity/index.json` has the v2.3 closeout surface
  `v2-3-parity-roots-and-no-claim-guardrails` and no `v2-4` surface yet.
- Phase 145 `DENIED_OVERCLAIMS` includes `prune-mode`. A positive v2.4
  prune sentence in README or the runtime guide will fail that checker
  until the successor exception in D-12 lands.

### Established Patterns

- Closeout phases add one aggregate Bun checker and test pair rather
  than rewriting every completed phase checker.
- Claim checks use curated current docs and mutation fixtures, not
  history-wide scans.
- Machine-readable parity surfaces and the human checklist mirror
  exactly-once requirement ownership.
- Previous-milestone required sentences stay in their required files
  when README current-state copy advances.
- Public-network review stays optional and untracked. Default
  verification stays deterministic and local.
- Functional-core crates stay I/O-free. This phase should not add
  runtime I/O to satisfy a documentation gap.

### Integration Points

- Extend v2.4 entries in `docs/parity/index.json`,
  `docs/parity/checklist.md`, and the existing chainstate, p2p, and
  rpc-cli catalog pages.
- Tighten relevant groups in `docs/parity/source-breadcrumbs.json` where
  semantic prune anchors are missing.
- Add the Phase 151 checker pair and wire it after Phase 145 in
  `scripts/verify.sh`.
- Adjust the Phase 145 checker so the exact v2.4 prune sentence is
  allowed and archive, assumeutxo, BIP37, public-default, and
  production-readiness claims stay denied.
- Refresh README, release-readiness, production-boundary, and runtime-guide
  wording around the bounded v2.4 claim.
- Commit the Phase 151 UAT package and reconcile current-milestone
  metadata after verification evidence is current.

</code_context>

<specifics>
## Specific Ideas

Preferred release wording: "Knots-aligned prune on the single active
chainstate deletes old block and undo payloads inside a height window by
removing Fjall keys, advertises NODE_NETWORK_LIMITED, and reports Pruned
only after a durable delete."

Keep the v2.3 sentence in the same files: "disk-backed per-outpoint coins,
typed cache-flush policy, fuller chainstate-manager behavior for the single
active chainstate, and honest stored-block availability that serves or
reports a stored block only when the payload bytes are present."

Required operator forms include Cargo and Bazel invocations of the shipped
Phase 150 prune status, manual prune, and prune-lock commands. A
public-network UAT item may truthfully say "not run" without turning
Phase 151 verification into a failure.

Do not delete historical `.planning/phases/` directories. Do not archive
the milestone from this phase. Do not add `blk` or `rev` files.

</specifics>

<deferred>
## Deferred Ideas

- Milestone archival — `/gsd-complete-milestone v2.4` after this phase
  passes, not during Phase 151.
- Temporary IBD prune target (`-pruneduringinit`) — FUT-27.
- Archive-node or production-scale historical serving — FUT-19.
- Compact-filter serving (BIP157/158). BIP37 bloom serving stays excluded
  — FUT-20.
- assumeutxo, assumevalid, or a second chainstate — FUT-21.
- LevelDB `chainstate/` live import or export — FUT-22.
- Automatic destructive reindex or coins repair — FUT-23.
- Public serving or relay by default — FUT-24.
- Public-network CI as a release gate — FUT-25.
- Production full-node readiness, production service operation, or
  production-funds wallet safety — FUT-26.
- txindex combined with prune — out of scope for this milestone.
- Hosted web dashboard or GUI — project non-goal.

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 151-parity-roots-and-no-claim-guardrails*
*Context gathered: 2026-09-29*
