# Chainstate And UTXO Engine

This entry tracks the Phase 4 chainstate slice implemented in Open Bitcoin.
The behavioral baseline remains Bitcoin Knots `29.3.knots20260210`.

## Coverage

- explicit UTXO entries carrying output, coinbase, creation-height, and
  creation-median-time-past metadata
- pure-core active-chain snapshots and per-block undo payloads
- direct block connect using the existing consensus validators plus derived
  spend contexts from the current UTXO view
- direct tip disconnect that removes created outputs, restores spent inputs in
  reverse order, and rewinds the active tip
- explicit reorg application over disconnect and reconnect paths
- deterministic best-tip preference by cumulative work, then height, then block
  hash for repo-owned fixtures
- node-side in-memory snapshot persistence that keeps storage outside the pure
  chainstate core

## Knots sources

- [`packages/bitcoin-knots/src/coins.h`](../../../packages/bitcoin-knots/src/coins.h)
- [`packages/bitcoin-knots/src/coins.cpp`](../../../packages/bitcoin-knots/src/coins.cpp)
- [`packages/bitcoin-knots/src/validation.cpp`](../../../packages/bitcoin-knots/src/validation.cpp)
- [`packages/bitcoin-knots/src/node/blockstorage.cpp`](../../../packages/bitcoin-knots/src/node/blockstorage.cpp)

## Knots behaviors mirrored here

- unspendable outputs do not enter the spendable UTXO view
- connect spends inputs before it adds outputs at the connected height
- disconnect removes created outputs before replaying undo in reverse order
- connect rejects BIP30-style output overwrites instead of silently replacing
  live coins
- best-chain preference is work-first even though Open Bitcoin uses a stable
  hash tie-break for deterministic fixtures instead of Knots' pointer-identity
  fallback

## Phase 70 branch and reorg recovery claim

Phase 70 keeps branch replacement deterministic by selecting candidate branches
by cumulative work, then height, then hash for the stable final tie-breaker.
The sync runtime waits for replacement branch block bodies before changing the
active chain, reuses `Chainstate::reorg` through the managed chainstate adapter,
persists the resulting active-chain snapshot, and exposes bounded latest
evidence through `sync.latest_reorg`.

That bounded latest evidence includes common ancestor height/hash, disconnected
count, connected count, final active height/hash, and whether the transition was
fully persisted. Missing active-chain block bodies, missing undo data, malformed
stored chainstate, or storage persistence failures remain storage recovery
blockers rather than peer retry claims.

## Phase 71 resource and storage-pressure claim

Phase 71 extends the local restart/resume evidence with storage-pressure
classification. Low-disk backend failures surface through
`StorageRecoveryAction::FreeDisk`, map to
`SyncRecoveryCategory::ResourceExhaustion`, and tell the operator:
`Free disk space for the selected datadir, then retry sync.` The claim remains
diagnostic and bounded; it does not add automatic chainstate repair,
block serving, production-funds wallet claims, migration apply mode, signed
packaging, Windows service support, GUI, hosted dashboards, or broad
production-node readiness.

## Phase 72 active-chain evidence claim

Phase 72 adds observability/support evidence only. Connected and validated
active-chain height, hash, and work now flow into operator status, support
evidence, live-smoke summaries, metrics, and structured logs so reviewers can
distinguish downloaded block bodies from durably persisted active-chain
progress.

This evidence does not add inbound serving, address relay, block serving,
transaction relay, compact block relay, production-funds wallet claims,
migration apply mode, signed packaging, Windows service support, GUI, hosted
dashboards, or broad production-node readiness.

## VER-02 deterministic coverage map

Phase 73 makes the deterministic chainstate coverage boundary auditable through
existing local tests and checker anchors. The VER-02 map covers:

- UTXO/undo persistence
- block connect/disconnect/reorg across restart
- best-chain header selection
- peer response failures
- crash recovery as durable reopen
- duplicate connect prevention
- resource bounds

This coverage map is local verification evidence only. It does not add block
serving, transaction relay, compact block relay, production-funds wallet
claims, migration apply mode, signed packaging, Windows service support, GUI,
hosted dashboards, or broad production-node readiness.

## v1.6 full-sync completion release boundary

Phase 74 uses the Phase 68 through Phase 73 chainstate evidence as part of the
source-built, explicit opt-in full-sync completion claim. The accepted
chainstate evidence is validated active-chain progress, durable UTXO/undo and
block-index state, same-datadir restart/resume continuity, reorg persistence,
duplicate-connect prevention, and deterministic coverage for resource-bounded
long-chain behavior.

This release boundary does not add block serving, transaction relay, compact
block relay, production-funds wallet safety, migration apply mode, signed
packaging, Windows service support, GUI parity, hosted dashboards,
public-network CI, release-blocking live sync, or broad production-node
readiness. Public-network evidence remains opt-in UAT outside
`bash scripts/verify.sh`.

## Phase 75 soak ledger and chainstate evidence

The `phase75-multi-day-soak-runner-evidence-ledger` surface uses shared
chainstate status evidence when a soak checkpoint or verdict references
validated active-chain progress. Reviewers should continue to distinguish
validated active-chain height, hash, and work from downloaded-only block bodies
or elapsed runtime.

Phase 75 does not add block serving, transaction relay, compact block relay,
production-funds wallet safety, migration apply mode, signed packages, GUI
readiness, hosted dashboards, public-network CI, release-blocking live sync, or
broad production-node readiness. The soak ledger is evidence over existing
sync and chainstate facts, not a new chainstate-manager claim.

## Phase 78 progress guarantee chainstate boundary

The `phase78-progress-guarantees-stall-diagnosis` surface uses chainstate facts
for PROG-01, PROG-02, PROG-03, and PROG-04. `progress_credit` is valid only
when the runtime has validated, connected, and durably persisted active-chain
height/hash/work, or when `current_at_best_known_tip` evidence proves the
connected active-chain tip matches the fresh best-known validated tip.

Downloaded-only block bodies, header-only branches, report generation, and
peer contribution evidence remain diagnostics until chainstate connection and
durable persistence happen. Better header branches can explain
`branch_competition_awaiting_bodies` or `stall_diagnosis`, but they do not
replace the active tip or credit progress before the replacement bodies are
available and validated.

## Phase 79 diagnostics support-bundle forensics boundary

The `phase79-diagnostics-support-bundle-forensics` surface uses chainstate and
sync status facts for DIAG-01, DIAG-02, DIAG-03, and DIAG-04. `support_forensics`
may render a forensic timeline, checkpoint chain, failure narrative, likely cause, evidence basis, next action, confidence, redaction, size bounds,
timeline ordering, and cross-surface consistency, but those fields remain a
support-bundle projection over existing validated active-chain, downloaded
block, resource, recovery, and stall evidence.

The sidecar does not credit chainstate progress by itself. Support bundle
existence, elapsed time, peer reachability, daemon startup, raw logs, or stale
reports do not prove soak stability, chainstate safety, inbound serving, relay,
production-funds wallet use, migration apply mode, packaging, GUI, hosted
dashboards, public-network default checks, multi-day default gates, automatic support-bundle upload, or production-node readiness.

## v1.8 production claim boundary

The v1.8 production claim boundary is
[`docs/parity/production-claim-boundary.md`](../production-claim-boundary.md).
Validated chainstate evidence remains historical support for scoped sync,
soak, recovery, and diagnostics claims. It does not satisfy broad
production-node readiness, destructive repair, public-network CI, or
release-blocking live sync gates by itself.

The Phase 83 support matrix is
[`docs/parity/support-matrix.md`](../support-matrix.md). Chainstate
sync/recovery/resource evidence supports scoped source-built evidence only; it
does not satisfy broad production-node readiness, destructive repair,
public-network CI, or release-blocking live sync gates.

## First-party implementation

- [`packages/open-bitcoin-chainstate/src/engine.rs`](../../../packages/open-bitcoin-chainstate/src/engine.rs)
- [`packages/open-bitcoin-chainstate/src/types.rs`](../../../packages/open-bitcoin-chainstate/src/types.rs)
- [`packages/open-bitcoin-chainstate/tests/parity.rs`](../../../packages/open-bitcoin-chainstate/tests/parity.rs)
- [`packages/open-bitcoin-node/src/chainstate.rs`](../../../packages/open-bitcoin-node/src/chainstate.rs)

## Known gaps

- disk-backed coins databases, cache-flush policy, and assumeutxo flows
- mempool repair and disconnected-transaction pools during reorg
- header-chain validation and full node chainstate-manager behavior beyond this
  phase's active-chain slice

## Follow-up triggers

Update this entry when later phases add mempool-coupled spend views,
header-chain work calculation, or disk-backed persistence that materially
changes the external chainstate behavior.
