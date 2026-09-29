# Phase 151: Parity Roots and No-Claim Guardrails - Research

**Researched:** 2026-09-29
**Domain:** Closeout guardrails — Phase 146–150 parity-root backfill, last-gate Bun no-claim checker, committed UAT, leftover-Pending reconciliation
**Confidence:** HIGH
**lifecycle_mode:** yolo
**phase_lifecycle_id:** 151-2026-09-29T21-06-33

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

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

### Folded Todos

None — `todo match-phase 151` returned no matches.

### Claude's Discretion

The planner may choose exact checker helper names, fixture organization,
paragraph-classification implementation, the smallest honest breadcrumb or
catalog-owner splits, exact doc section placement, the precise Knots
symbol spellings once research confirms them, and whether optional UAT
items are recorded as pending or not run. Prefer small pure TypeScript
helpers with focused tests and the existing Phase 145 checker pattern.
Do not spend discretion on runtime behavior changes, a `blk`/`rev` store,
or broader claims.

### Deferred Ideas (OUT OF SCOPE)

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
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| GRD-01 | Parity docs cite the pinned Knots prune anchors, including the Fjall key versus `blk`/`rev` file difference, and checkers still reject archive serving, assumeutxo, BIP37, public defaults, and production-readiness claims. | Cite the frozen symbol list below in `docs/parity/catalog/chainstate.md`, `docs/parity/catalog/p2p.md`, and `docs/parity/catalog/rpc-cli-config.md`, plus index/checklist surfaces. Add `scripts/check-phase151-parity-uat-release-boundary.ts` after the Phase 145 steps in `scripts/verify.sh`. Narrow the Phase 145 `prune-mode` denial so the exact D-14 sentence and the frozen Fjall-difference sentence pass, while `archive-node`, `assumeutxo`, `bip37`, public defaults, and production-readiness stay denied. |
</phase_requirements>

## Summary

Phase 151 is a closeout and claim-guardrail phase. The 17 v2.4 requirements already have implementation owners in Phases 146–150. `docs/parity/index.json` has the v2.3 closeout surface `v2-3-parity-roots-and-no-claim-guardrails` at status `done` and contains no `v2-4` surface id. The human chainstate catalog still says `` `Pruned` stays reserved ``, and `docs/parity/catalog/p2p.md` does not yet name `NODE_NETWORK_LIMITED`. The plan should backfill six `done` parity surfaces, refresh the existing catalog prose, add one Phase 151 Bun checker cloned from the Phase 145 shape, and carve a one-sentence exception into the Phase 145 claim checker. [VERIFIED: docs/parity/index.json, docs/parity/catalog/chainstate.md, docs/parity/catalog/p2p.md, scripts/check-phase145-parity-uat-release-boundary/]

The pinned Knots tree is present at `packages/bitcoin-knots` commit `a9aee730466ac67d35a3c03ee24676be5e045878` (`v29.3.knots20260210`). Every D-06 symbol exists under the names below. The honest layout difference is Knots `UnlinkPrunedFiles` deleting `blk`/`rev` flat files versus Open Bitcoin deleting paired Fjall keys. Do not add a block-file store. [VERIFIED: packages/bitcoin-knots git submodule status; validation.h; blockstorage.cpp]

Checker insertion is immediately after the existing Phase 145 pair, in both the `VERIFY_COMMAND_ORDER` heredoc and the live `run_step` chain. `scripts/verify.sh` has no `check-phase146` through `check-phase150` steps. Inserting Phase 151 after Phase 145 preserves the Phase 145 checker's required Phase 144-then-145 order. [VERIFIED: scripts/verify.sh]

**Primary recommendation:** Clone the Phase 145 checker pair as Phase 151, insert it directly after the Phase 145 `run_step` pair, cite the frozen Knots symbols in the existing catalogs, and exempt only the exact D-14 sentence plus one frozen Fjall-versus-`blk`/`rev` sentence from the Phase 145 `prune-mode` denial.

## Project Constraints (from .cursor/rules/)

No `.cursor/rules/` directory exists in this repo. [VERIFIED: glob `.cursor/rules/**/*` returned 0 files]

Follow the loaded repo instructions instead:

- Use `bash scripts/verify.sh` as the release contract. Focused `bun test` / `bun run` of the new checker are iteration aids. [CITED: AGENTS.md Repo-Local Guidance]
- During UAT, print copy-pasteable Cargo and Bazel commands, not only the installed `open-bitcoin` alias. [CITED: AGENTS.md]
- Parity breadcrumbs for new first-party Rust sources go through `docs/parity/source-breadcrumbs.json`. This phase should not add Rust sources. [CITED: AGENTS.md]
- `standards-overrides.md` has no active exception. The table is still the placeholder row. [VERIFIED: standards-overrides.md]
- Bun/TypeScript checkers stay in `scripts/`. Do not add a Python checker. [CITED: standards/languages/typescript-javascript.md]
- `workflow.nyquist_validation` is `false`. Do not add a Nyquist test-map section. [VERIFIED: .planning/config.json]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Bun | 1.4.2 | Run `scripts/check-phase151-*.ts` and `bun test` fixtures | Existing closeout checkers are Bun entrypoints. Installed at `/Users/peterryszkiewicz/.bun/bin/bun`. [VERIFIED: `bun --version`] |
| TypeScript (Bun-native, no package.json) | repo scripts, no npm package | Checker modules under `scripts/check-phase151-parity-uat-release-boundary/` | Phase 145 already uses this layout. There is no `package.json`. [VERIFIED: AGENTS.md; scripts/check-phase145-parity-uat-release-boundary.ts] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `node:fs` / `node:path` | Node built-ins bundled with Bun | Load the curated corpus and reject paths that escape the repo root | Same as Phase 145 `checks.ts` `isInsideRepo` |
| `gsd-tools.cjs` | repo-local install at `$HOME/.cursor/get-shit-done/bin/gsd-tools.cjs` | STATE, ROADMAP progress, and requirement checkbox mutations | Only the commands listed under Architecture Patterns. Do not hand-edit fields that those commands own. |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Phase 145 checker shape | A new scanner or a Rust test | Rejected by D-09. A new scanner would drift from the fixture and no-claim rules already enforced. |
| New catalog page | Rows in `index.json` plus prose in existing catalogs | D-02 and D-03 forbid a competing closeout manifest. |

**Installation:** none. Do not add an npm dependency.

**Version verification:** Bun `1.4.2` was printed by `bun --version` on 2026-09-29. Git `2.53.0` is installed. [VERIFIED: local commands]

## Architecture Patterns

### Recommended project structure

```text
scripts/check-phase151-parity-uat-release-boundary.ts
scripts/check-phase151-parity-uat-release-boundary.test.ts
scripts/check-phase151-parity-uat-release-boundary/
  constants.ts    # sentence, corpus, surfaces, anchors, denied topics, UAT argv
  checks.ts       # checkPhase151ParityUatReleaseBoundary(maybeRepoRoot?)
  claims.ts       # paragraph classifier; reuse Phase 145 markers
  verifier.ts     # visible heredoc order + executable run_step order + historical dirs
  test-fixtures.ts
docs/parity/index.json
docs/parity/checklist.md
docs/parity/catalog/chainstate.md
docs/parity/catalog/p2p.md
docs/parity/catalog/rpc-cli-config.md
docs/parity/release-readiness.md
docs/parity/source-breadcrumbs.json
.planning/phases/151-parity-roots-and-no-claim-guardrails/151-UAT.md
```

Export name: `checkPhase151ParityUatReleaseBoundary`. Environment override: `OPEN_BITCOIN_PHASE151_REPO_ROOT`. Return `string[]`. The CLI wrapper prints failures and exits 1, matching Phase 145. [VERIFIED: scripts/check-phase145-parity-uat-release-boundary.ts]

Read live `.planning/REQUIREMENTS.md` directly. Do not route Phase 151 through `resolvePlanningRequirementsSource` with the v2.3 archive. That helper returns `.planning/milestones/v2.3-REQUIREMENTS.md` when the live file lacks the v2.3 needle. v2.4 is still the live milestone. Archival happens only in `/gsd-complete-milestone v2.4` after this phase. [VERIFIED: scripts/source-corpus.ts; D-24]

### Pattern 1: Index surfaces are two lists

`docs/parity/index.json` has:

- `surfaces[]` objects with `name` and `status` (Phase 145 calls these top-level names).
- `checklist.surfaces[]` objects with `id`, `title`, `status`, `requirements`, `evidence`, `rationale`, `known_gaps`, `suspected_unknowns`.

Add both a top-level `name` and a checklist `id` for each of the six surface ids in D-03. Checklist ids and top-level names should be the same string. Status is `done` when the prose and requirement checkboxes exist. There is currently no `"status": "in_progress"` value anywhere in `index.json`, and no `v2-4-` id. Do not create surfaces as `in_progress` and then flip them. Preserve every existing `done` surface, including `v2-3-parity-roots-and-no-claim-guardrails`. [VERIFIED: docs/parity/index.json around the v2.3 closeout entry; grep for `v2-4-` and `"status": "in_progress"`]

Exactly-once ownership for these 17 ids, and only these:

| Surface id | Requirements |
|------------|----------------|
| `v2-4-wallet-leftover-snapshot-cutover` | SNAP-01 |
| `v2-4-pure-prune-policy-and-lock-windows` | PRUN-01, PRUN-02, PRUN-03, LOCK-01 |
| `v2-4-fjall-payload-unlink-and-have-pruned` | UNLK-01, UNLK-02, UNLK-03 |
| `v2-4-limited-serving-and-honest-pruned-labels` | SERV-01, SERV-02, SERV-03, LABL-01 |
| `v2-4-operator-prune-surfaces-and-evidence` | OPER-01, OPER-02, OPER-03, LOCK-02 |
| `v2-4-parity-roots-and-no-claim-guardrails` | GRD-01 |

REQUIREMENTS.md already has `[x]` on the first 16 and `[ ]` on GRD-01. The ROADMAP requirement table still says Pending for OPER-01, OPER-02, OPER-03, LOCK-02, and GRD-01. Flip those five cells to Complete only after the checker names their evidence. Do not reopen Phase 150. [VERIFIED: .planning/REQUIREMENTS.md; .planning/ROADMAP.md requirement table]

Mirror the same six rows in `docs/parity/checklist.md`. Status vocabulary is `planned`, `in_progress`, `done`, `deferred`, `out_of_scope`. Use `done`. [VERIFIED: docs/parity/checklist.md line 9]

### Pattern 2: Verifier insertion point

Update all three together:

1. Ordering comment at `scripts/verify.sh` line 39. State that Phase 145 remains the v2.3 closeout gate and Phase 151 is the v2.4 closeout gate wired immediately after it. Keep the existing statement that Phase 138 remains the file-final `check-phase*` command.
2. `VERIFY_COMMAND_ORDER` heredoc. Current last closeout lines are:

```text
bun test scripts/check-phase145-parity-uat-release-boundary.test.ts
bun run scripts/check-phase145-parity-uat-release-boundary.ts
```

They sit at lines 131–132. The next lines are the Phase 121 pair. Insert the Phase 151 test, then the Phase 151 check, between Phase 145 and Phase 121.
3. Live `run_step` chain at lines 299–300, with the same neighbor. Phase 121 follows at line 301.

Do not insert Phase 151 between Phase 144 and Phase 145. The Phase 145 checker requires that visible and executable order. Do not add public-network, wall-clock, `run-live-mainnet-smoke`, or `command-timings.ts` as `run_step` commands. Phase 145 already fails the default verifier if those tokens appear in a `run_step`. [VERIFIED: scripts/verify.sh; scripts/check-phase145-parity-uat-release-boundary/verifier.ts `FORBIDDEN_RUN_STEP_TOKENS`]

Phase 151's own order check should require the substring sequence Phase 145 test, Phase 145 check, Phase 151 test, Phase 151 check in both the heredoc body and the executable `run_step` lines. Reuse `orderedLines`: the required strings must match the full trimmed line, so the constants and `scripts/verify.sh` must stay character-identical.

### Pattern 3: Phase 145 prune-mode exception

The denied topic is the literal substring `prune-mode`, not the word `prune`. `claims.ts` lowercases each clause and uses `String.includes`. A clause fails only when it matches a `POSITIVE_PATTERNS` entry, lacks a `NO_CLAIM_MARKERS` entry, is not a table row whose status cell is `deferred` / `unsupported` / `not allowed` / `not run`, and contains a denied topic. [VERIFIED: scripts/check-phase145-parity-uat-release-boundary/claims.ts; constants.ts `DENIED_OVERCLAIMS`]

The locked D-14 sentence does not contain `prune-mode`:

`Knots-aligned prune on the single active chainstate deletes old block and undo payloads inside a height window by removing Fjall keys, advertises NODE_NETWORK_LIMITED, and reports Pruned only after a durable delete.`

Copy that sentence verbatim into `README.md` and `docs/operator/runtime-guide.md`, and keep the v2.3 D14 sentence in both files:

`disk-backed per-outpoint coins, typed cache-flush policy, fuller chainstate-manager behavior for the single active chainstate, and honest stored-block availability that serves or reports a stored block only when the payload bytes are present.`

The D-14 sentence alone will not trip Phase 145. Current-state copy that says the node `provides` or `implements` `prune-mode` will. D-12 still requires a narrow exception so the successor claim and the Fjall difference can sit next to the word `prune-mode` when they are the exact allowed sentences.

Implement the exception this way:

- Leave `prune-mode` in `DENIED_OVERCLAIMS`.
- Before recording a `prune-mode` failure, skip that one topic when the clause contains either the exact D-14 sentence or the exact frozen difference sentence below.
- Still fail that clause when it also contains any other denied topic (`archive-node`, `assumeutxo`, `assumevalid`, `bip37`, `compact-filter`, `public serving or relay by default`, `public-network ci`, `production full-node readiness`, `production service operation`, `production-funds`, `leveldb chainstate`, `destructive reindex`).
- Add Phase 151 denied topics that Phase 145 does not already list: `blk/rev`, `pruneduringinit`, and `txindex` combined with a positive prune claim. Keep them out of the Phase 145 global allow path. Phase 151's own checker is the owner of those extra denials. Do not delete `prune-mode` from Phase 145.
- Freeze this difference sentence and use it in the chainstate catalog, README or runtime guide, and the Phase 145 exception: `Open Bitcoin removes paired Fjall block and undo keys for eligible heights and does not introduce a Knots blk/rev flat-file store.`

Give the positive clause and the denial clause separate sentences. Phase 145 treats a whole clause as non-claiming when it contains `does not`, so a single sentence that both `provides prune-mode` and `does not` add archive serving would skip every denied topic. [VERIFIED: claims.ts `hasNoClaimMarker`]

Existing negative sentences must stay, including runtime-guide and release-readiness wording such as `v2.3 does not add prune-mode product behavior`. Those pass today because `does not` is a no-claim marker. [VERIFIED: docs/operator/runtime-guide.md; docs/parity/release-readiness.md]

### Pattern 4: Claim corpus

Phase 151 `CLAIM_FILES` should be the Phase 145 list plus the operator catalog that D-11 names and Phase 145 omitted:

- `README.md`
- `docs/operator/runtime-guide.md`
- `docs/parity/checklist.md`
- `docs/parity/catalog/chainstate.md`
- `docs/parity/catalog/p2p.md`
- `docs/parity/catalog/rpc-cli-config.md`
- `docs/parity/release-readiness.md`
- `docs/parity/production-claim-boundary.md`
- `docs/parity/support-matrix.md`

Do not scan `.planning/`, milestone archives, or historical phase prose as a blocking claim surface. `PROJECT.md` is reconciled by D-23 but is not a claim-checker file. When refreshing PROJECT current-state copy, say Fjall key deletion, not "delete old block files", and keep the existing archive / assumeutxo / production deferrals. The milestone goal sentence currently says the node can delete old block files. That sentence is outside the blocking corpus, and this phase should correct it in place so the handoff agrees with D-07. [VERIFIED: .planning/PROJECT.md Current Milestone goal; Phase 145 `CLAIM_FILES`]

### Pattern 5: Breadcrumbs to tighten

Breadcrumbs are file paths, not symbol names. Symbol names belong in the catalogs and in the Phase 151 checker constants. Do not replace a file-path breadcrumb with a symbol string. [VERIFIED: docs/parity/source-breadcrumbs.json]

| Group | Current breadcrumbs | Required edit |
|-------|---------------------|---------------|
| `chainstate-prune` | validation.h, validation.cpp, blockmanager_args.cpp, blockstorage.h, blockstorage.cpp | Already the five D-06 policy files. Leave the group intact. |
| `node-fjall-prune-unlink` | validation.cpp, blockstorage.cpp, blockstorage.h, validation.h | Already cites unlink and have-pruned files. Leave it. |
| `node-prune-records` | blockstorage.cpp, blockstorage.h | Sufficient for have-pruned records. Leave it. |
| `network-limited-serve` | protocol.h, init.cpp, net_processing.cpp, validation.h | Sufficient. Leave it. |
| `node-limited-serve` | init.cpp, protocol.h, net_processing.cpp, blockstorage.cpp | Sufficient for advertisement and `IsBlockPruned`. Leave it. |
| `operator-prune-projection` | rpc/blockchain.cpp, chain.h | Sufficient for the quartet projection. Leave it. |
| `cli-operator-prune` | only `packages/bitcoin-knots/src/bitcoin-cli.cpp` | Add `packages/bitcoin-knots/src/rpc/blockchain.cpp`. The shipped commands call `pruneblockchain`, `listprunelocks`, `setprunelock`, and `clearprunelock`. `bitcoin-cli.cpp` alone hides that RPC anchor. |

Do not add an explicit `none` breadcrumb for these groups. Do not retarget unrelated empty groups (`node-storage-contract` and the other empty groups are mempool, snapshot, and CLI contracts, not v2.4 prune anchors). [VERIFIED: source-breadcrumbs.json labels listed above]

The closeout checklist surface `upstream.sources` must include the file paths in the frozen anchor table. Phase 145 enforces that pattern for its own surface only. Phase 151 should do the same for `v2-4-parity-roots-and-no-claim-guardrails`. [VERIFIED: checks.ts `checkSurfaceOwnership`]

### Pattern 6: Metadata commands the CLI owns

Use this binary: `node "$HOME/.cursor/get-shit-done/bin/gsd-tools.cjs"`.

| Change | Command | Do not |
|--------|---------|--------|
| Mark GRD-01 checked | `requirements mark-complete GRD-01` | Hand-edit the checkbox before the checker can name the evidence. |
| Phase 151 progress-table row after plans and summaries exist | `roadmap update-plan-progress 151` | Expect this command to edit the requirement-coverage table. It updates the phase progress row from plan/summary counts. [VERIFIED: gsd-tools.cjs header; lib/roadmap.cjs `cmdRoadmapUpdatePlanProgress`] |
| Session continuity | `state record-session --stopped-at "..." --resume-file ".planning/ROADMAP.md"` | Omit `--resume-file`. The implementation stores `None` when the flag is absent. [VERIFIED: gsd-tools.cjs `record-session` branch] |
| Current focus / status fields | `state patch --<field> <value>` or `state update <field> <value>` | Blindly rewrite STATE.md session YAML. |

The requirement-coverage rows `| OPER-01 |` through `| GRD-01 |` are not phase-number rows, so `roadmap update-plan-progress` will not change their Pending cells. Edit those five status cells in `.planning/ROADMAP.md` directly after evidence exists. Also replace the next-step line that says Phase 151 remains unplanned. [VERIFIED: ROADMAP.md lines 170–201; roadmap.cjs table regex keys off the phase number]

Do not call `phase complete` or any milestone-archive command in this phase.

### Anti-patterns to avoid

- **A checker per Phase 146–150.** D-10 forbids backfilling `check-phase146` through `check-phase150`.
- **Global removal of `prune-mode` from Phase 145.** Unscoped positive `prune-mode` must still fail. The existing fixture appends `Open Bitcoin provides prune-mode product behavior.` and expects a failure. [VERIFIED: scripts/check-phase145-parity-uat-release-boundary.test.ts]
- **Scanning `.planning/` for claims.** Historical phase prose will mention deferred and superseded wording and will false-fail.
- **Runtime prune, serving, or a `blk`/`rev` store.** No honest evidence gap requires it. Catalog, checker, breadcrumb, and claim copy are the fix.
- **Putting the allowed sentence and `does not` in one clause.** That lets the no-claim marker hide a real overclaim.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Last-gate claim scanner | A new regex engine or a repo-wide grep | Phase 145 paragraph, clause, positive-pattern, and no-claim-marker functions | Table rows, backticks, and `deferred` status cells are already specified. Copy them. |
| Parity ownership ledger | A second JSON catalog or closeout manifest | `docs/parity/index.json` plus `docs/parity/checklist.md` | D-02. |
| Knots prune behavior | A new height planner, unlink path, or block file writer | Document `UnlinkPrunedFiles` as the Knots behavior and Fjall key deletion as the locked difference | D-07 and D-26. |
| Requirement checkbox automation | A custom markdown rewriter for every planning file | `requirements mark-complete` for GRD-01 | The CLI already owns that checkbox flip. |
| Milestone archive | A Phase 151 audit or archive script | `/gsd-complete-milestone v2.4` after this phase passes | D-24. |

**Key insight:** The observable proof is a deterministic checker over current docs plus the existing Phase 145 gate. Re-implementing prune would create a second behavior source the catalogs would then have to re-prove.

## Common Pitfalls

### Pitfall 1: Phase 145 rejects unscoped `prune-mode` even after the D-14 sentence lands

**What goes wrong:** README current-state copy says the project provides prune-mode, and `bash scripts/verify.sh` fails in the Phase 145 check before Phase 151 runs.
**Why it happens:** `DENIED_OVERCLAIMS` includes `prune-mode`, and positive verbs include `provides`, `implements`, and `adds`.
**How to avoid:** Keep the D-14 sentence free of the hyphenated token, or include that exact sentence in the clause. Add the narrow exception. Keep a fixture that still rejects `Open Bitcoin provides prune-mode product behavior.` when the exact sentence is absent.
**Warning signs:** Failure text `forbidden positive Phase 145 claim: prune-mode`.

### Pitfall 2: Inserting Phase 151 between Phase 144 and Phase 145

**What goes wrong:** Phase 145 reports `visible and executable order must be Phase 144 then Phase 145`.
**Why it happens:** `orderedLines` searches forward from Phase 144. A Phase 151 line between them does not by itself break that search, but replacing the Phase 145 lines or updating only the heredoc does.
**How to avoid:** Add two lines after the Phase 145 pair in both the heredoc and the `run_step` list. Update the line 39 comment in the same commit.
**Warning signs:** `verify.sh` heredoc and `run_step` list disagree.

### Pitfall 3: Stale `Pruned` reserved wording

**What goes wrong:** Chainstate catalog still says `` `Pruned` stays reserved `` after LABL-01 shipped, so GRD-01 cites a claim the code no longer makes.
**Why it happens:** `docs/parity/catalog/chainstate.md` still heads its current claim as v2.3 only. [VERIFIED: chainstate.md lines 7–13]
**How to avoid:** Keep the v2.3 sentence as the foundation section. Add a v2.4 section that states height-window Fjall prune, `Pruned` only after durable have-pruned plus a missing payload, and `Unavailable` when the payload is missing without prune.
**Warning signs:** The catalog and `docs/operator/runtime-guide.md` disagree about `block_status_pruned`.

### Pitfall 4: `state record-session` clears the resume file

**What goes wrong:** STATE.md `Resume file` becomes `None`.
**Why it happens:** `cmdStateRecordSession` defaults `resume_file` to `None` when `--resume-file` is omitted.
**How to avoid:** Always pass `--stopped-at` and `--resume-file`.
**Warning signs:** Session Continuity no longer points at ROADMAP or the phase directory.

### Pitfall 5: Treating the ROADMAP progress command as the requirement table

**What goes wrong:** OPER-01 through GRD-01 stay Pending, which D-22 says would force another reconciliation phase.
**Why it happens:** `roadmap update-plan-progress` rewrites the phase progress row only.
**How to avoid:** Directly set those five coverage-table statuses to Complete after the checker names the evidence, and use the CLI for the GRD-01 checkbox and the phase progress row.
**Warning signs:** REQUIREMENTS.md says Complete while ROADMAP still says Pending.

### Pitfall 6: Citing Knots `blk`/`rev` deletion as if Open Bitcoin did it

**What goes wrong:** A catalog sentence implies file-level source parity or a second block store.
**Why it happens:** `UnlinkPrunedFiles` really does `fs::remove` on block and undo flat files. That is the Knots anchor, not the Open Bitcoin substrate.
**How to avoid:** Put the frozen difference sentence next to the `UnlinkPrunedFiles` citation.
**Warning signs:** Docs mention creating `blk?????.dat` or `rev?????.dat` under the datadir.

### Pitfall 7: `GetPruneRange` snapshot branch looks like in-scope work

**What goes wrong:** A planner copies the snapshot-chain `prune_start = snapshot base + 1` branch into the product.
**Why it happens:** `ChainstateManager::GetPruneRange` special-cases `m_snapshot_chainstate`. That path is assumeutxo / a second chainstate and stays deferred under FUT-21.
**How to avoid:** Document the branch as Knots behavior that this milestone does not implement. The in-scope height rule is `max_prune = tip - 288` and `prune_end = min(caller height, max_prune)`.
**Warning signs:** A task mentions `m_snapshot_chainstate` or IBD snapshot chainstate.

## Code Examples

### Frozen Knots symbols

Pinned tree: `packages/bitcoin-knots` at `a9aee730466ac67d35a3c03ee24676be5e045878`, tag label `v29.3.knots20260210`. [VERIFIED: `git submodule status packages/bitcoin-knots`]

| Symbol | Exact spelling and location | Observable value or behavior |
|--------|-----------------------------|------------------------------|
| `MIN_BLOCKS_TO_KEEP` | `static const unsigned int MIN_BLOCKS_TO_KEEP = 288;` in `src/validation.h` line 71 | Keep window of 288 blocks. |
| `MIN_DISK_SPACE_FOR_BLOCK_FILES` | `static const uint64_t MIN_DISK_SPACE_FOR_BLOCK_FILES = 550 * 1024 * 1024;` in `src/validation.h` line 82 | Automatic target floor of 550 MiB. |
| `Chainstate::FlushStateToDisk` | `validation.cpp` lines 3070–3106 | Manual versus automatic split: `nManualPruneHeight > 0` calls `FindFilesToPruneManual` with `std::min(last_prune, nManualPruneHeight)`; otherwise `FindFilesToPrune`. |
| `ChainstateManager::GetPruneRange` | `validation.cpp` lines 6910–6934 | `max_prune = Height() - MIN_BLOCKS_TO_KEEP`; `prune_end = min(last_height_can_prune, max_prune)`. The snapshot-chain `prune_start` branch stays out of scope. |
| `ParsePruneOption` | `util::Result<uint64_t> ParsePruneOption(const int64_t nPruneArg, const std::string_view opt_name)` in `src/node/blockmanager_args.cpp` line 20 | `0` disables, `1` returns `BlockManager::PRUNE_TARGET_MANUAL`, and a target below `MIN_DISK_SPACE_FOR_BLOCK_FILES` errors. |
| `PruneLockInfo` | `struct PruneLockInfo` in `src/node/blockstorage.h` line 104 | Fields `desc`, `height_first`, `height_last`, `temporary`. |
| `PRUNE_LOCK_BUFFER` | `static constexpr int PRUNE_LOCK_BUFFER{10};` in `src/node/blockstorage.cpp` line 222 | 10-block buffer. |
| `BlockManager::DoPruneLocksForbidPruning` | `blockstorage.cpp` line 317 | Uses `PRUNE_LOCK_BUFFER` on both ends of the lock. |
| `BlockManager::FindFilesToPruneManual` | `blockstorage.cpp` line 335 | Manual file selection after `GetPruneRange`. |
| `BlockManager::FindFilesToPrune` | `blockstorage.cpp` line 387 | Automatic selection. Returns immediately when `Height() <= PruneAfterHeight()`. |
| `m_have_pruned` | `bool m_have_pruned = false;` in `blockstorage.h` line 419 | Set true in `FlushStateToDisk` only after a non-empty prune set, alongside `WriteFlag("prunedblockfiles", true)` (`validation.cpp` lines 3107–3111). |
| `BlockManager::UnlinkPrunedFiles` | `blockstorage.cpp` line 896 | Knots deletes flat files: `m_block_file_seq.FileName` and `m_undo_file_seq.FileName`. This is the `blk`/`rev` anchor. Open Bitcoin does not do this. |
| `BlockManager::IsBlockPruned` | `blockstorage.cpp` line 711 | `m_have_pruned && !(BLOCK_HAVE_DATA) && nTx > 0`. |
| `BlockManager::ScanAndUnlinkAlreadyPrunedFiles` | `blockstorage.cpp` line 679 | No-op when `m_have_pruned` is false. |
| `NODE_NETWORK_LIMITED` | `(1 << 10)` in `src/protocol.h` line 327 | Comment: serving the last 288 blocks, BIP159. |
| `g_local_services` | `src/init.cpp` line 977 starts as `NODE_NETWORK_LIMITED \| NODE_WITNESS` | Non-prune mode adds `NODE_NETWORK` at lines 2071–2075. Prune mode does not add `NODE_NETWORK`. `init.cpp` lines 1106–1108 reject prune combined with `-txindex`. |
| `NODE_NETWORK_LIMITED_MIN_BLOCKS` | `static const unsigned int NODE_NETWORK_LIMITED_MIN_BLOCKS = 288;` in `src/net_processing.cpp` line 123 | Serve window. The ignore-getdata check adds a two-block buffer at line 2295. |
| `pruneblockchain` | `static RPCHelpMan pruneblockchain()` in `src/rpc/blockchain.cpp` line 1213 | Refuses when `IsPruneMode()` is false. Returns `GetPruneHeight(...).value_or(-1)` at line 1271. |
| `GetPruneHeight` | `std::optional<int> GetPruneHeight(...)` in `blockchain.cpp` line 1027 | Last pruned height, not the info-field plus one. |
| Prune quartet | `getblockchaininfo` in `blockchain.cpp` lines 1762–1765 and 1802–1810 | Keys `pruned`, `pruneheight` (`GetPruneHeight + 1`, or `0`), `automatic_pruning`, `prune_target_size`. `automatic_pruning` is `GetPruneTarget() != PRUNE_TARGET_MANUAL`. |
| `listprunelocks` / `setprunelock` | `blockchain.cpp` lines 1052 and 1113, registered at 4129–4130 | Knots lock RPC. `clearprunelock` is the Open Bitcoin extension already shipped in Phase 150 and already listed in `rpc-cli-config.md`. |
| Unit test | `packages/bitcoin-knots/src/test/blockmanager_tests.cpp` | `blockmanager_scan_unlink_already_pruned_files` sets `m_have_pruned` before `ScanAndUnlinkAlreadyPrunedFiles`. |
| Functional test | `packages/bitcoin-knots/test/functional/feature_pruning.py` | Exercises `-prune=550`, the 288-block undo window, and `pruneblockchain` versus `getblockchaininfo.pruneheight`. |

There is no separate `blockmanager_args` unit test file in this pin. `feature_pruning.py` is the honest behavior test for `ParsePruneOption`'s 550 MiB floor: it expects `Prune configured below the minimum of 550 MiB`. [VERIFIED: feature_pruning.py line 127]

`PRUNE_TARGET_MANUAL` is `std::numeric_limits<uint64_t>::max()` in `blockstorage.h` line 375. Cite it next to `ParsePruneOption` when explaining `-prune=1`.

### Frozen UAT argv

Phase 150 shipped operator subcommands, not a separate prune-status binary. Prune status is `open-bitcoin status`. Manual prune is `open-bitcoin prune run <height-or-timestamp>`. Locks are `open-bitcoin prune lock list|set|clear`. The parser tests use height `10` and lock name `ibd` with heights 1 and 2. Knots' RPC example uses `pruneblockchain 1000`. Freeze the example height `1000` and the example lock name `ibd` in UAT so the strings are stable. They are examples, not a live prune request. [VERIFIED: packages/open-bitcoin-cli/src/operator/prune.rs; operator/tests/routing/prune.rs; packages/open-bitcoin-cli/BUILD.bazel targets `open_bitcoin`, `open_bitcoin_cli`; packages/open-bitcoin-rpc/BUILD.bazel target `open_bitcoind`]

Operator status:

```text
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format human
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format human
bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json
```

Manual prune:

```text
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- prune run 1000
bazel run //packages/open-bitcoin-cli:open_bitcoin -- prune run 1000
```

Prune locks:

```text
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- prune lock list
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- prune lock set --name ibd --height-first 1 --height-last 2
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- prune lock clear --name ibd
bazel run //packages/open-bitcoin-cli:open_bitcoin -- prune lock list
bazel run //packages/open-bitcoin-cli:open_bitcoin -- prune lock set --name ibd --height-first 1 --height-last 2
bazel run //packages/open-bitcoin-cli:open_bitcoin -- prune lock clear --name ibd
```

RPC and daemon, using the same preview host tuple Phase 145 already publishes:

```text
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -datadir=/tmp/open-bitcoin-preview
bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -datadir=/tmp/open-bitcoin-preview
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview getblockchaininfo
bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview getblockchaininfo
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview pruneblockchain 1000
bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview pruneblockchain 1000
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview listprunelocks
bazel run //packages/open-bitcoin-cli:open_bitcoin_cli -- -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview listprunelocks
```

Also document `setprunelock` and `clearprunelock` with the same Cargo and Bazel prefix. `clearprunelock` is the Open Bitcoin extension; keep that label. Public-network UAT is one line: `not run`. Do not add it to `scripts/verify.sh`.

Phase 151 also requires these verifier commands inside `151-UAT.md`:

```text
bun test scripts/check-phase151-parity-uat-release-boundary.test.ts
bun run scripts/check-phase151-parity-uat-release-boundary.ts
bash scripts/verify.sh
```

### Historical phase path check

Copy Phase 145 `checkHistoricalPhasePaths`. It scans `scripts/verify.sh` and every `scripts/check-phase*.ts` for `\.planning/phases/[A-Za-z0-9._-]+` and fails when the directory is missing. Do not gitignore or delete those directories. [VERIFIED: verifier.ts]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| v2.3 closeout denies positive `prune-mode` and treats `Pruned` as reserved | v2.4 allows one scoped prune sentence and reports `Pruned` only after a durable delete | Phase 149 shipped labels; Phase 151 records the claim | Phase 145 needs a one-sentence exception, not a topic deletion |
| Knots unlinks `blk`/`rev` files | Open Bitcoin deletes paired Fjall keys | Locked in the v2.4 roadmap | Document the difference beside `UnlinkPrunedFiles` |
| No v2.4 rows in `docs/parity/index.json` | Six `done` surfaces, GRD-01 owned only by the closeout row | This phase | Exactly-once checker will fail until the rows exist |

**Deprecated/outdated:**

- `docs/parity/catalog/chainstate.md` line 13, `` `Pruned` stays reserved ``, is stale relative to LABL-01.
- README chainstate row still says prune/archive modes remain deferred as a present claim. Archive, assumeutxo, and production claims remain deferred. The scoped prune sentence replaces the prune half of that deferral. [VERIFIED: README.md line 76]
- PROJECT.md current-state text still says operator prune surfaces remain Phase 150, and the milestone goal still says "delete old block files". Refresh both during D-23 without archiving the milestone. [VERIFIED: .planning/PROJECT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Example UAT height `1000` and lock name `ibd` are acceptable fixtures because the shipped parser accepts any `i64` height and any lock name. | Frozen UAT argv | Low. The commands stay copy-pasteable if the planner picks different example numbers, as long as Cargo and Bazel forms both appear. |

All other factual claims in this research were checked against the repo in this session.

## Open Questions

1. **Should `setprunelock` UAT show Knots' JSON desc payload or the Phase 150 CLI flags?**
   - What we know: The shipped operator CLI is `prune lock set --name --height-first --height-last`. Knots' help example passes a JSON object with `desc` and `height`. `clearprunelock` is an Open Bitcoin extension.
   - What's unclear: Nothing about the shipped argv. The question is only how much Knots help text to quote.
   - Recommendation: Freeze the CLI flag form in UAT. Mention the RPC method names. Do not copy the Knots JSON help example as the required command string.

2. **Does any v2.4 surface already exist under a different id?**
   - What we know: `rg` for `v2-4-` in `docs/parity/index.json` returned no matches. Checklist grep for v2.4 requirement ids found only the v2.3 closeout row in the sampled area; the requirement ids SNAP-01 through GRD-01 are not surface owners in the checklist yet.
   - What's unclear: None that blocks planning. A planner who adds a second id for the same requirement will fail the exactly-once check.
   - Recommendation: Use the six D-03 ids and no others.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Bun | Checker tests and `scripts/verify.sh` | ✓ | 1.4.2 | — |
| git | Submodule identity and historical path checks | ✓ | 2.53.0 | — |
| `packages/bitcoin-knots` checkout | D-06 symbol confirmation | ✓ | `a9aee730` (`v29.3.knots20260210`) | — |
| Cargo / Bazel | Copy-paste UAT, not a phase build step | Not probed | — | Document the commands. Do not make a live prune, daemon, or Bazel build part of Phase 151's default gate. |

**Missing dependencies with no fallback:**

- None for the checker and docs work.

**Missing dependencies with fallback:**

- Cargo and Bazel were not executed. They are UAT documentation, not implementation dependencies.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | This phase adds no auth surface. |
| V3 Session Management | no | No session state. |
| V4 Access Control | no | No new operator privilege. The checker enforces claim scope, which is release integrity rather than a runtime access check. |
| V5 Input Validation | yes | Reuse Phase 145 `isInsideRepo` before reading any `OPEN_BITCOIN_PHASE151_REPO_ROOT` path. Reject path escape, missing files, and invalid JSON. Do not shell out to user-controlled paths. |
| V6 Cryptography | no | No crypto. Do not invent a hasher for claim text. |

### Known Threat Patterns for the closeout checker

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Positive production-readiness, archive, assumeutxo, or BIP37 wording in current docs | Spoofing / tampering of the release claim | Curated corpus plus denied-topic clauses. Negative and deferred markers stay allowed. |
| `OPEN_BITCOIN_PHASE151_REPO_ROOT` pointing outside the repo | Tampering / information disclosure | `path.relative` escape check copied from Phase 145. |
| A `does not` later in the same sentence laundering a positive overclaim | Tampering | Keep allowed and denied claims in separate sentences. Fixtures must cover both acceptance of D-14 and rejection of archive / assumeutxo / BIP37 / public defaults / production readiness. |
| Default verifier gaining a public-network or wall-clock gate | Denial of verifiability | Keep `FORBIDDEN_RUN_STEP_TOKENS` behavior. Public-network UAT stays `not run`. |

## Sources

### Primary (HIGH confidence)

- Pinned Knots tree `packages/bitcoin-knots` at `a9aee730466ac67d35a3c03ee24676be5e045878` — `validation.h`, `validation.cpp`, `node/blockmanager_args.cpp`, `node/blockstorage.h`, `node/blockstorage.cpp`, `protocol.h`, `init.cpp`, `net_processing.cpp`, `rpc/blockchain.cpp`, `src/test/blockmanager_tests.cpp`, `test/functional/feature_pruning.py`
- `scripts/check-phase145-parity-uat-release-boundary.ts` and `scripts/check-phase145-parity-uat-release-boundary/` — export shape, `prune-mode` denial, verifier order, historical phase paths
- `scripts/verify.sh` lines 39, 131–132, and 299–300 — insertion point
- `docs/parity/index.json` — no `v2-4` surface; v2.3 closeout `done`
- `packages/open-bitcoin-cli/src/operator/prune.rs` and `operator/tests/routing/prune.rs` — shipped argv
- `node "$HOME/.cursor/get-shit-done/bin/gsd-tools.cjs"` command list and `lib/roadmap.cjs` — which planning edits the CLI owns
- `.planning/config.json` — `nyquist_validation: false`

### Secondary (MEDIUM confidence)

- None. Planning-command behavior was confirmed in the local gsd-tools source, not by a web page.

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — Bun and the Phase 145 layout are already in the repo.
- Architecture: HIGH — insertion point, symbol spellings, and CLI argv were read from the tree.
- Pitfalls: HIGH — Phase 145 tests and claim matching were read; the `record-session` default was read from gsd-tools source.

**Research date:** 2026-09-29
**Valid until:** 2026-10-29 (stable checker and pinned Knots tree; re-check if the submodule commit moves)

## Planner Checklist

- Add the six `done` index and checklist surfaces. GRD-01's only owner is `v2-4-parity-roots-and-no-claim-guardrails`.
- Refresh chainstate, p2p, and rpc-cli prose. Do not add a new catalog page.
- Add `rpc/blockchain.cpp` to the `cli-operator-prune` breadcrumb group.
- Add the Phase 151 checker pair and wire it immediately after Phase 145 in comment, heredoc, and `run_step`.
- Exempt only the exact D-14 sentence and the frozen Fjall-difference sentence from the Phase 145 `prune-mode` topic.
- Commit `151-UAT.md` with the argv blocks above and a public-network line of `not run`.
- After the checker is green, mark GRD-01 complete, fix the five ROADMAP Pending cells, and reconcile PROJECT and STATE without archiving v2.4.
