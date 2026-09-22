---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 147-2026-09-22T12-05-43
generated_at: 2026-09-22T12:07:11.819Z
---

# Phase 147: Pure Prune Policy and Lock Windows - Context

**Gathered:** 2026-09-22
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Operators and later unlink get Knots-aligned prune mode, height-window,
and lock-buffer decisions with no disk I/O in core.

This phase delivers PRUN-01, PRUN-02, PRUN-03, and LOCK-01. It adds typed
prune mode (disabled, manual-only, or automatic target of at least 550 MiB)
and pure functions that decide which heights may be pruned. Automatic plans
keep the last 288 blocks and do not start before the network prune-after
height. Manual prune refuses a target inside that keep window. A prune lock
forbids deleting the locked height range plus a 10-block buffer. Those
decisions live in `open-bitcoin-chainstate` and must not open Fjall or the
filesystem.

This phase does not delete block or undo payloads, set have-pruned, emit
`Pruned`, advertise `NODE_NETWORK_LIMITED`, or add RPC, CLI, or dashboard
prune commands. Those belong to Phases 148–151. It does not introduce
`blk`/`rev` files, a new crate, assumeutxo, a second chainstate, BIP37,
`-pruneduringinit`, txindex, public prune defaults, or production-readiness
claims.

</domain>

<decisions>
## Implementation Decisions

### Typed prune mode

- **D-01:** Prune mode is a typed enum in `open-bitcoin-chainstate`:
  disabled, manual-only, or automatic with a target in MiB. The external
  Knots integer contract is `0` disabled, `1` manual-only
  (`PRUNE_TARGET_MANUAL`), and `N >= 550` automatic target MiB. The decision
  core does not keep a raw integer as its working state.
- **D-02:** An automatic target below 550 MiB is a typed config refusal.
  Values other than `0`, `1`, or `>= 550` are the same refusal. 550 MiB is
  `MIN_DISK_SPACE_FOR_BLOCK_FILES`, not a tunable default.
- **D-03:** The default mode is disabled. Enabling prune is explicit. This
  phase does not parse JSONC, clap flags, or RPC. It exposes the typed
  shapes and pure decisions that a later operator surface (Phase 150) will
  feed. Success criterion 1 is satisfied by those typed config/policy
  shapes, not by a new operator command.

### Automatic height window

- **D-04:** Automatic planning keeps the last 288 blocks
  (`MIN_BLOCKS_TO_KEEP`). Heights inside that trailing window are never
  prune candidates.
- **D-05:** Automatic planning does not start before the network
  prune-after height. That height is an injected chain-params input, not
  a mainnet constant baked into the function and not an operator override.
  Before the tip reaches that height, the automatic plan is empty. That
  empty plan is not a failure.
- **D-06:** The pure planner may return a candidate height list only from
  caller-injected facts: tip height, prune-after height, mode, and
  per-height payload byte sizes. It compares injected usage with the
  automatic target and stops at the keep window, prune-after gate, and
  lock protections. It does not stat disk, list Fjall keys, or choose
  files by walking storage.

### Manual keep-window refusal

- **D-07:** Manual prune takes an explicit target height. A target inside
  the 288-block keep window is a typed refusal. The comparison must match
  pinned Knots `GetPruneRange` / manual prune height checks. Do not invent
  a looser or stricter window.
- **D-08:** Manual mode does not invent an automatic byte-budget plan.
  When the target height is legal, the plan is the heights up to that
  target that also sit outside the keep window, at or after the
  prune-after rule Knots uses for manual prune, and outside lock
  protections. Researcher must cite the Knots manual-versus-automatic
  difference rather than assuming they share one formula.

### Lock buffer

- **D-09:** A prune lock is an injected inclusive height range plus a
  caller-supplied name or id. This phase does not persist locks, list
  them, or add an operator setter. Phase 150 owns LOCK-02.
- **D-10:** Protected heights are the locked range expanded by
  `PRUNE_LOCK_BUFFER = 10` in the direction and inclusivity Knots uses in
  `DoPruneLocksForbidPruning`. Both automatic and manual plans must omit
  those heights. A plan that would otherwise delete a protected height
  must leave it out, not fail the whole prune solely because one lock
  overlaps part of the range, unless Knots itself refuses the call.
  Researcher pins that Knots behavior before the planner codes it.
- **D-11:** Lock decisions are pure. No Fjall, filesystem, or RPC access
  in `open-bitcoin-chainstate`.

### Plan result and later phases

- **D-12:** The phase output is a pure plan or a typed refusal: eligible
  heights, an empty not-yet plan, or a refusal reason (below-minimum
  target, manual height inside the keep window, prune disabled when a
  manual prune is requested). It does not delete payloads, set
  have-pruned, or emit `Pruned`.
- **D-13:** Do not extend `FlushMode` or perform flush in this phase.
  Phase 148 owns unlink order, including any flush-before-delete fact.
  A non-empty candidate list is enough for that later phase to consume.
- **D-14:** Single active chainstate only. No assumeutxo snapshot-base
  prune-start branch and no second chainstate.

### Operator, parity, and verification boundary

- **D-15:** Do not add prune RPC, CLI, or dashboard fields. Phase 150 owns
  operator surfaces. Do not advertise `NODE_NETWORK_LIMITED` (Phase 149).
- **D-16:** New first-party Rust source and tests under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` get
  parity breadcrumbs through `docs/parity/source-breadcrumbs.json`. Cite
  the pinned Knots prune anchors where a defensible source line exists.
  Full v2.4 prune parity docs and the no-claim checker stay in Phase 151
  (GRD-01).
- **D-17:** Verification remains `bash scripts/verify.sh`. Default
  verification stays deterministic and public-network-free. No new
  production crate or third-party library.

### Claude's Discretion

- Module split inside `open-bitcoin-chainstate` (`prune.rs` versus a
  `prune/` folder) as long as the crate stays I/O-free and the public
  decision API is typed.
- Exact integer comparisons for "last 288" and the 10-block buffer, once
  research quotes the pinned Knots functions. The constants 288, 10, and
  550 MiB are locked; off-by-one interpretation is not.
- Whether per-height sizes are a `BTreeMap` or a sorted slice, as long as
  the planner stays pure and does not read storage.
- Fixture style for tip, prune-after, locks, and injected byte sizes.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase contract

- `.planning/ROADMAP.md` — Phase 147 goal, PRUN-01, PRUN-02, PRUN-03,
  LOCK-01, and success criteria; Phases 148–151 own unlink, serving,
  operator surfaces, and parity roots
- `.planning/REQUIREMENTS.md` — PRUN-01..03 and LOCK-01 wording; FUT-27
  `-pruneduringinit` stays deferred; txindex+prune, archive, assumeutxo,
  BIP37, and public defaults stay out of scope
- `.planning/PROJECT.md` — v2.4 prune milestone; functional core stays
  I/O-free; prune must not resurrect snapshot-as-truth
- `.planning/research/STACK.md` — no new crate; pure height window and
  lock check in `open-bitcoin-chainstate`; Fjall unlink is later
- `.planning/research/FEATURES.md` — Knots `-prune` `0` / `1` / `>=550`
  contract
- `.planning/research/PITFALLS.md` — keep-window and lock-buffer failure
  modes if unlink ships without these decisions

### Pinned Knots anchors

- `packages/bitcoin-knots/src/validation.h` — `MIN_BLOCKS_TO_KEEP` (288)
  and `MIN_DISK_SPACE_FOR_BLOCK_FILES` (550 MiB)
- `packages/bitcoin-knots/src/validation.cpp` — `GetPruneRange` and the
  manual versus automatic prune height checks
- `packages/bitcoin-knots/src/node/blockmanager_args.cpp` —
  `ParsePruneOption`, `nPruneTarget`, `PRUNE_TARGET_MANUAL`
- `packages/bitcoin-knots/src/node/blockstorage.h` — `PruneLockInfo`
- `packages/bitcoin-knots/src/node/blockstorage.cpp` —
  `DoPruneLocksForbidPruning`, `PRUNE_LOCK_BUFFER` (10),
  `FindFilesToPrune`

### Existing Open Bitcoin seams

- `packages/open-bitcoin-chainstate/src/lib.rs` — I/O-free chainstate
  crate; new prune decisions belong here
- `packages/open-bitcoin-chainstate/src/coins/flush.rs` — existing
  `FlushMode` stays unchanged in this phase
- `packages/open-bitcoin-network/src/block_serving.rs` —
  `BlockServingDataAvailability::Pruned` stays reserved until a real
  delete in a later phase
- `.planning/phases/146-wallet-leftover-snapshot-cutover/146-CONTEXT.md`
  — wallet rescan already ignores leftover snapshot bytes; this phase
  must not invent have-pruned or payload deletion

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `open-bitcoin-chainstate` is already the I/O-free home for flush and
  recovery decisions (`decide_flush`, `FlushMode`). Prune policy should
  follow that pattern: injected facts in, typed decision out.
- `FlushMode::{None, IfNeeded, Periodic, Always}` already exists. This
  phase must not add a prune flush mode.
- `BlockServingDataAvailability::Pruned` is reserved and documented as
  not a product signal until prune actually deletes payloads.

### Established Patterns

- Functional core stays free of Fjall, filesystem, and network I/O.
  Adapters inject heights, byte sizes, and lock ranges.
- Knots-aligned constants live next to the decision they govern, with
  parity breadcrumbs pointing at the pinned Knots file.
- Missing payload without a recorded prune stays `Unavailable`. This
  phase must not relabel anything `Pruned`.
- No new production crate. Extend `open-bitcoin-chainstate`.

### Integration Points

- Later Phase 148 unlink should consume the candidate height list from
  this planner and perform Fjall deletes itself.
- Later Phase 150 operator surfaces should construct the typed prune
  mode and pass lock ranges in. They should not reimplement the window
  math.
- Chain params (or the node shell) supply `prune_after_height` per
  network. The pure function does not look up the network.

</code_context>

<specifics>
## Specific Ideas

Match pinned Knots `29.3.knots20260210` numbers and refusal rules:
288-block keep, 10-block lock buffer, 550 MiB minimum automatic target,
and `0` / `1` / `>=550` mode parsing. Map file-granularity Knots prune
onto height decisions because Open Bitcoin stores per-hash Fjall
payloads. Document that layout difference later in Phase 151; do not
invent `blk`/`rev` files here.

No specific UI or operator copy in this phase.

</specifics>

<deferred>
## Deferred Ideas

- Fjall block and undo key deletion, have-pruned, and interrupted-prune
  recovery — Phase 148 (UNLK-01..03).
- `NODE_NETWORK_LIMITED` advertisement and honest `Pruned` versus
  `Unavailable` labels — Phase 149 (SERV-01..03, LABL-01).
- RPC, CLI, dashboard prune status, manual prune commands, and lock
  list/set — Phase 150 (OPER-01..03, LOCK-02).
- Full prune parity docs and no-claim checkers — Phase 151 (GRD-01).
- Temporary IBD prune target (`-pruneduringinit`) — FUT-27.
- Combining txindex with prune — out of scope; Knots marks those modes
  incompatible, and this milestone does not add txindex.
- Flush-before-unlink orchestration (`fFlushForPrune`) — Phase 148.

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 147-pure-prune-policy-and-lock-windows*
*Context gathered: 2026-09-22*
