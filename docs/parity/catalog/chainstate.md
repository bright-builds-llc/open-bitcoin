# Chainstate And UTXO Engine

This entry tracks the Phase 4 chainstate slice implemented in Open Bitcoin
and the later v2.3 disk-backed coins, flush, manager, and honest-availability
claim. The behavioral baseline remains Bitcoin Knots `29.3.knots20260210`.

## Current v2.3 claim

The live v2.3 claim is disk-backed per-outpoint coins, typed cache-flush policy, fuller chainstate-manager behavior for the single active chainstate, and honest stored-block availability that serves or reports a stored block only when the payload bytes are present.

Leftover snapshot blobs are non-authoritative after the schema 1→2 one-way
migration. Restart tip and UTXO view come from durable coins best-block.
In the v2.3 claim, the label Pruned stayed reserved. First-party overlay occupancy is not C++ allocator /
LevelDB SizeEstimate. Fjall per-outpoint coins replace LevelDB `chainstate/`.
Disk-space probe may stay fail-open at `u64::MAX` where the node crate forbids
unsafe `statvfs`. Phase 144 `chainstate_durability` is the shared operator
contract and is not re-derived here.

Pinned Knots symbols for this claim include `FlushStateToDisk` and
`GetCoinsCacheSizeState` in `validation.cpp`, `CanFlushToDisk` in
`node/chainstate.cpp` and `validation.cpp`, and `CheckBlockDataAvailability`
in `node/blockstorage.cpp`. The locked discussion name `HaveBlockData` is not
a pinned-tree symbol; the serve-path root is `CheckBlockDataAvailability`.

## Current v2.4 claim

The current v2.4 claim is height-window prune on Fjall keys for the single active chainstate.
Status and RPC report Pruned only when have-pruned is set and the payload is gone.
A missing payload without prune stays Unavailable.
The keep window is 288 blocks (MIN_BLOCKS_TO_KEEP).
The automatic prune target floor is 550 MiB (MIN_DISK_SPACE_FOR_BLOCK_FILES).
The prune-lock buffer is 10 blocks (PRUNE_LOCK_BUFFER).
Manual prune refuses a target inside the keep window.
Have-pruned is recorded only after a durable delete.
An interrupted prune fails closed.
Open Bitcoin removes paired Fjall block and undo keys for eligible heights and does not introduce a Knots blk/rev flat-file store.
Knots UnlinkPrunedFiles deletes blk and rev flat files.
Open Bitcoin does not do that delete.
ChainstateManager::GetPruneRange max_prune is tip height minus MIN_BLOCKS_TO_KEEP.
The Knots m_snapshot_chainstate prune_start branch stays out of scope.
Pinned spellings in this claim are ParsePruneOption, PruneLockInfo, DoPruneLocksForbidPruning, FindFilesToPrune, FindFilesToPruneManual, m_have_pruned, and IsBlockPruned.

## Phase 153 automatic prune retention integration

The stable `v2-4-pure-prune-policy-and-lock-windows` surface retains
[Phase 147 verification](../../../.planning/phases/147-pure-prune-policy-and-lock-windows/147-VERIFICATION.md)
as its pure policy, PRUN-03 and LOCK-01 foundation. Phase 153 **Automatic
Prune Retention Integration** supplies the production consumer for audit gap
INT-01 and current PRUN-01/PRUN-02 closure. These two requirements are
Complete after the default full native verifier passed and
[formal Phase 153 verification](../../../.planning/phases/153-automatic-prune-retention-integration/153-VERIFICATION.md)
passed 22/22 must-haves with `lifecycle_validated: true` for lifecycle
`153-2026-10-03T04-01-54`. The [integration re-audit](../../../.planning/phases/153-automatic-prune-retention-integration/153-INTEGRATION.md)
connects 20/20 selected seams and traces 10/10 scoped flows, with no blocking
integration gap and three retained advisory items. These counts describe the
selected static production trace and recorded runtime evidence. Implementation and observed checks
are recorded in the [accounting summary](../../../.planning/phases/153-automatic-prune-retention-integration/153-01-SUMMARY.md),
[owner summary](../../../.planning/phases/153-automatic-prune-retention-integration/153-02-SUMMARY.md)
and [runtime evidence summary](../../../.planning/phases/153-automatic-prune-retention-integration/153-03-SUMMARY.md).

The [Fjall accounting adapter](../../../packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs)
uses one guarded snapshot to sum actual block and encoded-undo value lengths
across both complete payload prefixes. Protected, recent and nonactive
payloads count in total; candidates refer only to current active height/hash
pairs. Either present mate counts, and both-absent pairs are omitted. Coins,
metadata, support records and leftover snapshot sizes are excluded. Checked
arithmetic and accounting errors refuse without inventing facts. Clone-shared
mutation guards invalidate measured revisions for complete write attempts,
including equal-size replacements and errors.

The [ordinary serialized owner](../../../packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune.rs)
passes these facts, configured mode, current tip, network prune-after height
and current durable locks to the unchanged pure planner. It preserves the
288-block keep window, ten-block lock buffer and legal 550 MiB minimum.
The target is soft: protected/nonactive usage may keep retained bytes above
target and never permits protected deletion. Lock read-modify-persist and
operator-status reads share the authority. Acquisition order is authority,
automatic state, then storage payload guard; nested flush effects run after
the storage guard is released.

The existing daemon Periodic worker ticks every second; exact measurement of
changed Periodic inputs coalesces for 60 seconds independently of coins-write
deadlines. First eligible activity and Always measure immediately. Equal
reusable revisions with unchanged tip/mode/locks skip non-Always scans; no
cached deletion plan is applied by a deferred cycle. None, disabled,
manual-only, under-target, short-chain and fully protected cases do not
delete. A nonempty automatic plan requests the existing full Always coins
and chain-metadata checkpoint, even when no coins write was due. Idle full
checkpoints carry current coins best-block without replacing a newer overlay
tip. Existing paired-unlink receipts own cache/undo cleanup on success and
later failure; counters and have-pruned remain earned by committed deletes.

Pinned Knots [blockstorage.cpp](../../../packages/bitcoin-knots/src/node/blockstorage.cpp)
roots are `CalculateCurrentUsage` (line 885) and `FindFilesToPrune` (line
387); [validation.cpp](../../../packages/bitcoin-knots/src/validation.cpp)
`FlushStateToDisk` (line 3070) forces writing for `fFlushForPrune`. Intentional
accounting difference: Knots sums file-info `nSize + nUndoSize` and applies
flat-file chunk/IBD buffering, while Open Bitcoin measures logical live Fjall
payload values per hash through its existing planner. The measurement is
neither physical disk allocation nor a promise of immediate space reclamation.
No temporary IBD target, flat-file layout or new budget rule is introduced.

The default-suite [genuine runtime regression](../../../packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs)
`automatic_prune_genuine_ordinary_retention_and_reopen` invokes the same
ordinary cycle helper as the daemon against its production offline durable
owner. It uses codec-valid sparse history, 235 large nonactive block/undo
pairs and six small active pairs, with actual serialized accounting:

| Observation | Result |
| --- | --- |
| Automatic target | 550 MiB = 576,716,800 bytes |
| Initial retained logical bytes | 578,359,864; nonactive 578,358,970 and active 894 |
| First ordinary committed deletes | Heights 1, 500 and 713; 447 bytes |
| Protected active survivors | Heights 510, 714 and 1001 |
| First retained total after deletion | 578,359,417; still above the soft target |
| Initial earned counters | One batch, three heights, last height 713 |
| Final genuine scenario duration | 17.891221375 seconds; harness 18.08 seconds |

At tip 1001, height 713 is eligible while 714 is recent; a committed lock at
520 protects 510 through its buffer. Nonactive retained bytes keep the
target unreachable. Repeated activity earns no duplicate counters. Both
wallet adapters succeed with retained creating payloads, then refuse older
creating payloads after automatic deletion while preserving saved wallets
and truthful Failed/checkpoint evidence. In-memory serving and operator
dispatch corroborate actual Pruned, Unknown and explicitly injected
Unavailable outcomes, configured target and earned counters without sockets.

The first stage proves successful production durable checkpoints and reopen;
a conflicting leftover snapshot cannot resurrect undo or replace durable
truth. A later, separate fixture uses `MetadataFaultStore` to delegate real
Fjall accounting, locks, writes, paired unlink and counters, but refuses only
metadata persistence and uses a fixture owner with `MemoryCoinsView`.
Replenishing height 1 earns one more actual delete before this injected error;
retry and final production reopen retain absence with two batches/four height
deletions. This later stage is effect injection, not a hardware fault or a
second production durable-coins checkpoint proof.

Repeat the genuine and small owner regressions sequentially from the repo root:

```bash
bun run scripts/command-timings.ts run --key phase153-genuine-retention -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind automatic_prune_genuine_ordinary_retention_and_reopen --all-features -- --test-threads=1 --nocapture
bun run scripts/command-timings.ts run --key phase153-owner-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib automatic_prune --all-features -- --test-threads=1
bash scripts/verify.sh
```

Related lifecycle, daemon, RPC, wallet and strict affected Clippy checks
passed in the linked summaries. The default full verifier then passed with
exit 0 in 45m47.953s, including the genuine default-suite regression, strict
workspace lint/build/tests, benchmark smoke, Bazel and pure-core coverage.
[Plan 04 evidence](../../../.planning/phases/153-automatic-prune-retention-integration/153-04-SUMMARY.md)
records the earlier failures and final pass. Sparse codec-valid history does not prove consensus validation
of a continuous chain, hardware resilience, public-network behavior or
production readiness. Concurrent direct-library wallet probe/save atomicity,
archive serving, assumeutxo, BIP37, public defaults and production-funds
wallet claims remain outside this evidence.

## Phase 152 post-prune wallet rescan eligibility

The stable `v2-4-wallet-leftover-snapshot-cutover` surface owns SNAP-01.
[Phase 146 verification](../../../.planning/phases/146-wallet-leftover-snapshot-cutover/146-VERIFICATION.md)
is its historical durable-coins/leftover-cutover foundation. Phase 152
**Post-Prune Wallet Rescan Eligibility** owns audit gap INT-02 closure.
Implementation and targeted node/RPC tests are recorded in the
[node summary](../../../.planning/phases/152-post-prune-wallet-rescan-eligibility/152-01-SUMMARY.md)
and [RPC summary](../../../.planning/phases/152-post-prune-wallet-rescan-eligibility/152-02-SUMMARY.md).
SNAP-01 is Complete after the full default native verifier passed and
[Phase 152 verification](../../../.planning/phases/152-post-prune-wallet-rescan-eligibility/152-VERIFICATION.md)
verified 11/11 must-haves with `status: passed` and
`lifecycle_validated: true` for lifecycle `152-2026-10-03T00-26-52`.
Historical Phase 146 evidence is retained alongside this new closure proof.

Open Bitcoin stages its existing pure full UTXO replacement on a copy of the
saved wallet. The requested start/chunk controls progress and stop height;
it does not exclude older matching coins from the replacement. The shared
[eligibility helper](../../../packages/open-bitcoin-node/src/wallet_registry/rescan.rs)
checks requested heights and every selected candidate's creating height,
including heights before start, before the adapter saves a replacement.
It resolves each required height explicitly, probes distinct hashes once,
excludes entries above the replacement height, and does not require
unrelated old coins' payloads. Node/store authority uses guarded durable
coins and best-block; live RPC authority retains the manager admission
view of durable parent coins plus pending overlay updates, with a separate
durable head-marker check. Leftover snapshot bytes supply neither authority
nor fallback, even when still present after reopen.

This stricter gate is the project full-replacement contract. Pinned Knots
[transactions.cpp::rescanblockchain](../../../packages/bitcoin-knots/src/wallet/rpc/transactions.cpp)
checks `hasBlocks` over its requested range (line 922), then calls
[wallet.cpp::ScanForWalletTransactions](../../../packages/bitcoin-knots/src/wallet/wallet.cpp)
(line 2000), which scans and updates transactions incrementally. Knots does
not implement Open Bitcoin's full UTXO replacement algorithm.

Missing active-chain metadata, absent requested/creating payloads and
payload read errors refuse before replacement persistence. The saved wallet
balances, UTXOs, tip and MTP, and the job's actual successful checkpoint and
next height are preserved. A requested start never fabricates progress;
missing stop metadata never fabricates a target hash. An identified job
records Failed with safe boundary/category and known height/hash, excluding
raw backend paths and snapshot contents. Failure-save errors propagate
visibly. Pending freshness reflects the actual checkpoint relative to its
target: Partial below the target, Fresh when already scanned through it,
and Scanning when the checkpoint is unknown. The changed-target Failed-save
regression proves the persisted Pending job remains Partial if recording
Failed itself errors; refusal never changes the saved wallet/checkpoint.

Each resumed node chunk and later durable RPC request reevaluates
eligibility. A previously successful scan is not permanent permission after
pruning. A refused later scan preserves the last successful wallet. Durable
`rescan_wallet(snapshot)` delegates to the same guarded range path, so a
caller-supplied snapshot cannot bypass eligibility; local fixture behavior
continues to use its supplied snapshot.

| Executable evidence | Behavior and fixture |
| --- | --- |
| [Shared helper tests](../../../packages/open-bitcoin-node/src/wallet_registry/rescan/tests.rs) | Eight tests cover older creating heights, unrelated coins, absent requested/creating metadata, safe probe errors, deduplication and replacement-height filtering. |
| [Node runtime eligibility tests](../../../packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime/eligibility.rs) | Real paired deletion with retained coins/best-block and have-pruned; midrange and in-range refusal; successful first chunk then prune/resume after dropping and reopening stores; conflicting leftover retained but ignored; retained-payload and unrelated-old-coin success controls. Literal H=[new,old] with missing B proves interrupted authority refusal; private callbacks exercise probe and Failed-save errors. |
| [Durable RPC eligibility tests](../../../packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs) and [fixtures](../../../packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility/fixtures.rs) | `FlushPersistSink::commit_paired_unlink` really removes both payload mates from the RPC store, leaves durable coins/best-block, sets have-pruned and retains other mates. Full wallet equality, saved checkpoint, later request/reopen and conflicting leftover assertions cover refusal and retained/unrelated controls. Missing-target and direct-helper tests close invented metadata and snapshot bypasses. RPC authority/read failures are privately injected typed errors through the real refusal/persistence handler; they are not physical H/B fixtures. |

The node executor observed a real pre-change RED: pruning the older matching
creating block still returned Complete/Fresh through height 3. Shared helper
tests passed 8/8, final node eligibility passed 10/10, coins migration passed
15/15 and node Clippy passed. The RPC executor observed its real durable RED:
the old adapter accepted start=3/stop=3 after paired deletion at creating
height 1. The final RPC matrix passed 19/19, including changed-target
Failed-save Pending/Partial readback, unknown-checkpoint Scanning, and a
real prune/refusal whose checkpoint already meets the target and correctly
remains Fresh. Freshness compatibility passed 4/4; construction passed 5/5,
range dispatch 1/1, node wallet tests 39/39 and final review-fix RPC Clippy
passed. The full default `bash scripts/verify.sh` then passed with exit 0 in
34m46.545s, including ordered workspace verification, coverage, benchmarks
and Bazel smoke. The linked lifecycle-valid report corroborates runtime
requested/creating-height checks and all three truthful freshness branches.
Historical source-string checks do not substitute for these runtime assertions.

From the repository root, require Bun on PATH to match `.bun-version`
(1.3.9), then use the timing wrapper to repeat the behavioral checks
sequentially; do not overlap Cargo jobs:

```bash
bun --version # must match .bun-version (1.3.9)
bun run scripts/command-timings.ts run --key phase152-node-wallet-rescan -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib wallet_rescan_runtime::eligibility --all-features
bun run scripts/command-timings.ts run --key phase152-node-shared-eligibility -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib wallet_registry::rescan --all-features
bun run scripts/command-timings.ts run --key phase152-rpc-post-prune -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib context::tests::rescan_eligibility --all-features
bash scripts/verify.sh
```

The owner trace shows HTTP RPC dispatch serializes through the context
mutex; manual prune uses the network authority mutation mutex. At Phase 152
verification, automatic `flush_coins` supplied an empty prune plan; the
Phase 153 ordinary consumer is described above. Separately exported
node/store callers share no atomic rescan probe/save transaction. This
evidence proves eligibility after completed pruning and fresh resume/reopen
probes; it does not guarantee payload presence at save under arbitrary
concurrent direct-library pruning. A new concurrent owner or stronger claim
requires replanning. Phase 153 requirement closure is Complete under the
formal report linked above; it does not expand this concurrency contract.
Incremental wallet scanning, snapshot deletion, repair, archive serving,
assumeutxo, public defaults and production-funds claims remain deferred.

## Historical Phase 4 snapshot-engine coverage

The following bullets are historical Phase 4 snapshot-engine coverage, not live v2.3 coin truth.

- explicit UTXO entries carrying output, coinbase, creation-height, and
  creation-median-time-past metadata
- historical Phase 4 pure-core active-chain snapshots and per-block undo payloads
- direct block connect using the existing consensus validators plus derived
  spend contexts from the current UTXO view
- direct tip disconnect that removes created outputs, restores spent inputs in
  reverse order, and rewinds the active tip
- explicit reorg application over disconnect and reconnect paths
- deterministic best-tip preference by cumulative work, then height, then block
  hash for repo-owned fixtures
- historical Phase 4 / later-snapshot node-side in-memory snapshot persistence
  that keeps storage outside the pure chainstate core

## Knots sources

- [`packages/bitcoin-knots/src/coins.h`](../../../packages/bitcoin-knots/src/coins.h)
- [`packages/bitcoin-knots/src/coins.cpp`](../../../packages/bitcoin-knots/src/coins.cpp)
- [`packages/bitcoin-knots/src/validation.cpp`](../../../packages/bitcoin-knots/src/validation.cpp)
- [`packages/bitcoin-knots/src/node/chainstate.cpp`](../../../packages/bitcoin-knots/src/node/chainstate.cpp)
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
- flush and cache-size decisions follow `FlushStateToDisk` /
  `GetCoinsCacheSizeState` and manager readiness follows `CanFlushToDisk`
- stored-block serve/report availability follows
  `CheckBlockDataAvailability`; `HaveBlockData` is the locked discussion name
  only

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

The Phase 84 upgrade policy is
[`docs/parity/upgrade-and-rollback-policy.md`](../upgrade-and-rollback-policy.md).
Chainstate schema/storage compatibility decisions use field-level evidence and
do not imply destructive repair.

## First-party implementation

- [`packages/open-bitcoin-chainstate/src/engine.rs`](../../../packages/open-bitcoin-chainstate/src/engine.rs)
- [`packages/open-bitcoin-chainstate/src/types.rs`](../../../packages/open-bitcoin-chainstate/src/types.rs)
- [`packages/open-bitcoin-chainstate/tests/parity.rs`](../../../packages/open-bitcoin-chainstate/tests/parity.rs)
- [`packages/open-bitcoin-node/src/chainstate.rs`](../../../packages/open-bitcoin-node/src/chainstate.rs)

## Known gaps

- assumeutxo, assumevalid, and IBD snapshot shortcuts remain deferred FUT-21,
  not a missing v2.3 deliverable
- prune/archive product modes, compact-filter serving, public serving or relay
  defaults, and production readiness remain deferred
- mempool repair and disconnected-transaction pools during reorg
- header-chain validation beyond the shipped active-chain and single-manager
  durability slice

## Follow-up triggers

Update this entry when later phases add mempool-coupled spend views,
header-chain work calculation, or disk-backed persistence that materially
changes the external chainstate behavior.
