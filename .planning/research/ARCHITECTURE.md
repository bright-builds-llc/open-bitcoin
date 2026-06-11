# Architecture Research: v1.6 Mainnet Full-Sync Completion

**Domain:** explicit opt-in `open-bitcoind` mainnet sync-to-tip and stay-current behavior
**Researched:** 2026-06-11
**Confidence:** HIGH for integration shape, MEDIUM for exact phase sizing

## Summary

v1.6 should integrate as a completion and scale-hardening layer on the existing
`DurableSyncRuntime`, not as a separate daemon or a new architectural stack. The
existing shape is sound: pure validation, chainstate, network, and wallet rules
stay in functional-core crates; sockets, Fjall storage, scheduling, status,
logs, metrics, service controls, and support evidence remain in
`open-bitcoin-node`, `open-bitcoin-rpc`, and `open-bitcoin-cli`.

The key architectural change is that v1.6 can no longer treat sync evidence as a
bounded smoke target. It needs durable, restart-safe, mainnet-scale pipelines for
headers, block bodies, active chainstate and UTXO updates, reorg recovery, peer
work scheduling, and a truthful status contract that distinguishes:

- connected to the best known validated header chain
- caught up to the best peer-observed mainnet tip
- staying current after initial sync
- unavailable or diagnosed blocker states

The milestone should still avoid broad production-node claims. Inbound serving,
address relay, transaction relay, compact block relay, migration apply mode,
production-funds wallet use, packaging, hosted dashboards, GUI work, and
public-network default verification remain out of scope.

## Inputs And Standards

Material inputs:

- `.planning/PROJECT.md`
- `.planning/MILESTONES.md`
- `.planning/milestones/v1.5-ROADMAP.md`
- `.planning/milestones/v1.5-REQUIREMENTS.md`
- `.planning/ARCHITECTURE.md`
- `.planning/STATE.md`
- `README.md`
- `packages/README.md`
- `docs/architecture/status-snapshot.md`
- `docs/architecture/operator-observability.md`
- `docs/architecture/storage-decision.md`
- `docs/operator/runtime-guide.md`
- `docs/parity/catalog/p2p.md`
- `docs/parity/release-readiness.md`
- `docs/parity/deviations-and-unknowns.md`
- `packages/open-bitcoin-node/src/sync.rs`
- `packages/open-bitcoin-node/src/sync/runtime_state.rs`
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs`
- `packages/open-bitcoin-node/src/network.rs`
- `packages/open-bitcoin-node/src/storage/fjall_store.rs`
- `packages/open-bitcoin-node/src/status.rs`
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`
- `packages/open-bitcoin-rpc/src/dispatch/node.rs`
- `packages/open-bitcoin-network/src/peer.rs`
- `packages/open-bitcoin-network/src/header_store.rs`
- `packages/open-bitcoin-chainstate/src/engine.rs`

Repo-local guidance and Bright Builds rules materially affected this research:
preserve functional-core / imperative-shell boundaries, keep public-network UAT
outside `bash scripts/verify.sh`, use repo-owned Rust domain logic, keep parity
claims auditable through `docs/parity/`, and treat `bash scripts/verify.sh` as
the repo-native deterministic verification contract. The local
`standards-overrides.md` has no active meaningful override entries. The
canonical `standards/` markdown files referenced by `AGENTS.bright-builds.md`
were not present in this checkout, so this research relies on the repo-local
instructions plus existing architecture docs.

## Existing Integration Points

| Area | Current component | v1.6 integration |
| --- | --- | --- |
| Daemon activation | `open-bitcoind` starts an explicit opt-in sync worker through `start_daemon_sync_worker` | Keep this entrypoint. Extend the worker policy from bounded review loop to full-sync/stay-current loop while preserving opt-in activation and shutdown controls. |
| Runtime orchestration | `DurableSyncRuntime` owns Fjall store, managed peer network, sync config, peer backoff, and in-flight blocks | Keep this as the shell boundary. Split internals into scheduler, work-queue, connect, and evidence modules as complexity grows. |
| Pure network behavior | `open-bitcoin-network::PeerManager`, `HeaderStore`, wire messages, compatibility harness | Extend peer/header planning in pure network types where behavior is deterministic and I/O-free. Keep DNS, sockets, clocks, retries, and persistence in node shell. |
| Validation and chainstate | `open-bitcoin-consensus` and `open-bitcoin-chainstate::Chainstate` | Add scale-safe pure APIs where needed, but do not add storage or network effects to pure crates. |
| Durable storage | `FjallNodeStore` keyspaces for headers, block index, chainstate, wallet, metrics, runtime, schema | Evolve from whole JSON snapshots for critical mainnet paths to record-keyed durable indexes and incremental chainstate persistence. |
| Status truth | `OpenBitcoinStatusSnapshot`, `DurableSyncState`, `SyncStatus` | Extend the existing status model with best-known-tip, IBD completion, stay-current, and tip-evidence fields. Do not create renderer-local definitions. |
| RPC | `getblockchaininfo`, `openbitcoinsyncstatus`, sync pause/resume | Keep compatibility-shaped fields truthful. Add Open Bitcoin-specific detail through existing extension sync status rather than overloading baseline fields with unsupported claims. |
| CLI/dashboard/support | `open-bitcoin status`, dashboard, support bundle, compatibility wrapper, live-smoke script | Reuse shared status. Add v1.6 full-sync/stay-current UAT reporting and support summaries with redaction and bounded evidence. |
| Release evidence | parity docs, threat model, release-boundary checker | Add v1.6 parity roots and a deterministic checker that guards the narrower sync-to-tip claim and preserves deferred surfaces. |

## New Components

### 1. Best-Known Tip Evidence Model

Add a typed status/runtime model for the best known mainnet tip. This should be
observational, not a magic oracle.

Likely fields:

- `best_known_header_height`
- `best_known_header_hash`
- `best_known_chain_work`
- `best_known_source_count`
- `peer_reported_start_heights`
- `tip_observed_at_unix_seconds`
- `connected_to_best_known_tip`
- `caught_up_to_best_known_tip`
- `stay_current_state`
- `maybe_tip_evidence_unavailable_reason`

This model should make a careful distinction between "validated local best
header", "connected local chainstate", and "current mainnet tip according to
compatible peers observed during this opt-in run." If v1.6 wants a stronger
human UAT claim, the evidence runner can optionally compare against an
operator-supplied expected height/hash or a controlled Knots peer, but default
verification should not depend on external services.

### 2. Durable Sync Work Queue

The current runtime tracks `inflight_blocks` in memory and clears stale work on
restart. For full sync, the runtime needs a durable, idempotent work queue or
block-state projection:

- missing best-chain block ranges
- requested block hashes and peer attribution
- downloaded block bodies
- connected block hashes/heights
- notfound, invalid, malformed, duplicate, disconnected, and non-extending
  outcomes
- retry eligibility and next peer action

This can live in `open-bitcoin-node::sync` and persist through `FjallNodeStore`.
It should not make stale per-peer socket state durable as if a dead peer can be
resumed. Durable state should describe recoverable work, not live connections.

### 3. Full-Sync Scheduler

Add a scheduler below the daemon worker and above peer sessions. It should own
the mode transitions:

- header bootstrap
- block body catch-up
- connect/reconcile
- reorg resolution
- no-progress diagnosis
- caught-up steady polling
- shutdown/pause/recovery

This scheduler can remain synchronous/thread-owned at first because
`open-bitcoind` already runs a sync worker thread. The important boundary is
that scheduling decisions are typed and testable without public-network sockets.

### 4. Incremental Durable Chainstate Store

Mainnet full sync needs a durable chainstate that is not a whole
`ChainstateSnapshot` JSON rewrite after every block. Add record-keyed storage
for:

- UTXO records by `OutPoint`
- undo data by connected block hash
- active chain positions by height and hash
- best connected tip pointer
- block validity and connection status

The pure `open-bitcoin-chainstate` crate can continue to own validation and
transition rules, but the shell should persist deltas or apply a storage-backed
view. If the pure engine needs a new API, prefer a typed `ChainstateDelta` or
connect result that lets the shell write exact changes without leaking Fjall
into the core crate.

### 5. v1.6 Evidence Runner And Boundary Checker

Extend, or create a sibling of, `scripts/run-live-mainnet-smoke.ts` for
full-sync/stay-current UAT. The report should track:

- initial datadir and config
- opt-in activation mode
- peer tip evidence
- header catch-up
- downloaded block catch-up
- connected chainstate catch-up
- restart/resume checkpoints
- no-progress and recovery diagnoses
- stay-current observation window
- final claim verdict

Add `scripts/check-v1.6-release-boundaries.ts` so deterministic verification can
check docs, parity roots, and forbidden public-network defaults without running
public-network sync.

## Modified Components

### `open-bitcoin-node/src/sync.rs`

Refactor before adding substantial behavior. The file already owns transport,
runtime state, block reconciliation, progress projection, resolver, and wallet
rescan exports. v1.6 should split new work into modules such as:

- `scheduler.rs`
- `work_queue.rs`
- `tip_evidence.rs`
- `connect_pipeline.rs`
- `steady_state.rs`
- `evidence.rs`

Keep `DurableSyncRuntime` as the public facade exported by `open-bitcoin-node`.

### `open-bitcoin-node/src/storage/*`

Change the mainnet-critical paths from snapshot-shaped persistence to indexed
records. `FjallNodeStore` already has the right namespace shape, but the current
header and chainstate writes are too coarse for mainnet full sync:

- `save_header_entries` rewrites all header entries.
- `save_chainstate_snapshot` stores the whole active chain, UTXO set, and undo
  map as a single versioned JSON value.
- block bodies are individually keyed, which is a good pattern to preserve.

Add schema-versioned record codecs and migration/reindex behavior before making
the full-sync claim.

### `open-bitcoin-chainstate`

The current pure engine is valuable but mainnet scale exposes two issues:

- `connect_block_with_current_time` clones the full UTXO map before applying a
  block.
- script verification flags are currently selected by the runtime as
  `ScriptVerifyFlags::P2SH`, which is unlikely to be sufficient for truthful
  full mainnet validation.

Add height-aware validation flag selection and incremental transition APIs as
pure domain behavior. Keep database reads/writes outside this crate.

### `open-bitcoin-network`

Extend pure peer/header planning where necessary:

- continue `getheaders` while peers report higher start heights or new headers
- express best-chain block request windows as pure decisions
- preserve `notfound` and invalid-data attribution
- keep compatibility harness coverage for early protocol behavior
- avoid adding socket, DNS, clock, or storage effects

The existing `HeaderSyncPolicy::HeadersOnly` and `HeadersAndBlocks` shape can be
reused or clarified, but daemon sync should still route block downloads through
the node shell so durable storage and chainstate connection remain controlled.

### `open-bitcoin-rpc`

Update `getblockchaininfo` so `blocks`, `headers`, `verificationprogress`,
`initialblockdownload`, and `warnings` remain truthful with the new completion
states. Continue exposing richer Open Bitcoin-specific state through
`openbitcoinsyncstatus`, pause, and resume. Do not imply inbound serving or relay
readiness through RPC warnings or version strings.

### `open-bitcoin-cli`

Extend `status`, dashboard, `sync status`, and support rendering to consume the
shared v1.6 status fields. Add operator UAT commands or documented script
commands with both Cargo and Bazel forms. Keep output quiet, field-driven, and
copy-pasteable.

### Docs And Parity Roots

Update these after substantial implementation changes:

- `docs/architecture/status-snapshot.md`
- `docs/architecture/operator-observability.md`
- `docs/operator/runtime-guide.md`
- `docs/parity/catalog/p2p.md`
- `docs/parity/deviations-and-unknowns.md`
- `docs/parity/release-readiness.md`
- `docs/parity/index.json`
- `docs/parity/checklist.md`

## Durable Data Flow

Recommended durable flow:

```text
open-bitcoind opt-in config
    -> FjallNodeStore open and schema/recovery preflight
    -> DurableSyncRuntime open
    -> load header index, block index, chainstate tip, runtime metadata
    -> scheduler decides header/block/connect/steady work
    -> transport resolves and connects compatible outbound peers
    -> peer messages update pure network/header state
    -> requested block bodies are validated, saved, and attributed
    -> connect pipeline applies contiguous best-chain blocks
    -> chainstate deltas, undo data, active-chain tip, and runtime state persist
    -> status, metrics, logs, RPC, dashboard, support, and UAT read one snapshot
```

Storage ownership should be:

| Namespace | v1.6 role |
| --- | --- |
| `headers` | record-keyed validated headers, parent links, height index, best header tip, chain work |
| `block_index` | block body records, body availability, validity/connect status, best-chain membership, reindex metadata |
| `chainstate` | durable UTXO records, undo records, active-chain positions, best connected tip |
| `runtime` | lifecycle, pause flag, stop reason, recovery marker, best-known-tip evidence, scheduler checkpoint |
| `metrics` | bounded header/downloaded/connected/stay-current samples |
| `wallet` | unchanged except wallet freshness should continue comparing against durable connected tip |
| `schema` | version and migration/reindex guards |

Atomicity matters most at connect boundaries. A connected block should not be
reported unless its active-chain position, UTXO changes, undo data, block status,
and runtime tip projection are all durable enough for same-datadir restart
evidence. If Fjall batch support is insufficient across all affected keyspaces,
the architecture needs a small journal or recovery marker for interrupted
connect operations.

## Validation And Connect Flow

Full-sync connect flow should be:

1. Accept peer only after baseline-compatible outbound handshake and service-bit
   checks.
2. Validate every received header with PoW, contextual time, retarget, and
   parent/chain-work checks before it can enter the best header index.
3. Schedule missing block bodies only for the selected best-header chain and
   within bounded per-peer/global in-flight limits.
4. For each block response, verify that it was requested, matches a known
   best-chain header, decodes correctly, and is not a duplicate/no-credit body.
5. Save the block body under its hash before connect attempts.
6. Connect only contiguous blocks whose parent is the active tip, using
   height-aware consensus/script flags and a durable UTXO/undo update.
7. On a better-work branch, load durable disconnect blocks and replacement
   branch bodies, apply undo data, connect the replacement branch, and update
   active-chain indexes atomically or with recoverable journal markers.
8. Persist status after each meaningful progress point so restart evidence can
   show header, downloaded, and connected progress without duplicate connects.

The existing `block_reconcile::reconcile_best_chain` is the right starting
point. v1.6 should harden it for large branches, partial block availability,
interrupted connect operations, and reorgs beyond smoke-sized cases.

## Peer And Scheduler Flow

The daemon worker should keep its explicit opt-in lifecycle and pause/resume
contract. Inside that worker, scheduling should become phase-aware:

```text
preflight
    -> recover interrupted storage/connect work
    -> collect compatible peers
    -> header catch-up until best-known header tip stabilizes
    -> block catch-up for best-known chain
    -> connect/reconcile until connected == downloaded == best-known header
    -> mark IBD-complete-to-best-known-tip
    -> steady polling for new headers/inv
    -> download/connect new best-chain blocks
    -> preserve caught-up or diagnosed state across restart
```

Peer behavior should remain bounded:

- cap outbound peer slots
- cap per-peer and global block in-flight counts
- keep retry/backoff typed and visible
- rotate away from peers that stall, send invalid data, or fail to provide
  missing blocks
- preserve useful-progress attribution by peer
- avoid durable claims about live socket state after restart

At tip, "no progress" needs different semantics than during IBD. A no-progress
cycle while connected height equals best known header height can be a steady
state, not a failure. A no-progress cycle while headers or blocks remain missing
is a diagnosed blocker. This distinction should be encoded in the scheduler and
status model, not inferred by renderers.

## Status And Support Evidence Flow

Extend `OpenBitcoinStatusSnapshot` rather than adding parallel report-only
fields. Recommended additions under `sync`:

- `completion`: unavailable, ibd, connected_to_best_known_tip, staying_current,
  blocked
- `best_known_tip`: height, hash, chain work, observed peer count, observed time
- `tip_lag`: headers remaining, downloaded blocks remaining, connected blocks
  remaining
- `stay_current`: last new-header time, last connected block time, observation
  window, verdict
- `latest_connect_checkpoint`: height, hash, persisted_at
- `latest_reorg`: optional depth, disconnected count, connected count, result
- `evidence_boundary`: explicit label such as `opt_in_mainnet_full_sync_review`

Existing consumers should keep using the same shared snapshot:

- CLI human and JSON status
- dashboard model
- `openbitcoinsyncstatus`
- `getblockchaininfo` warnings and IBD fields
- metrics samples
- structured logs
- service status and restart/resume evidence
- support bundle
- v1.6 UAT report

Support bundles should summarize v1.6 state and include links or paths to local
UAT artifacts, but they should not embed raw daemon logs, raw peer tables,
credentials, wallet material, or full local report archives.

## Likely Phase Order

Suggested phase order, continuing after v1.5 Phase 67:

1. **Phase 68: v1.6 Sync Completion Contract And Boundaries**
   Define requirements, status fields, tip evidence semantics, non-claims, and
   deterministic release-boundary checker scaffolding.

2. **Phase 69: Durable Mainnet-Scale Storage Shape**
   Add record-keyed header/block/chainstate indexes, schema migration/reindex
   paths, interrupted-write recovery, and reopen tests. This should land before
   long-run sync depends on it.

3. **Phase 70: Height-Aware Validation And Incremental Connect**
   Harden consensus flag selection, chainstate transition APIs, UTXO/undo
   persistence, and connect idempotence for mainnet-scale block application.

4. **Phase 71: Full-Sync Scheduler And Work Queue**
   Add phase-aware scheduling, durable missing-block work, peer rotation, block
   request windows, and no-progress classification for IBD versus steady state.

5. **Phase 72: Reorg, Recovery, And Restart At Scale**
   Prove interrupted connect recovery, reorg handling from durable undo data,
   same-datadir restart, no duplicate connects, and recovery guidance.

6. **Phase 73: Stay-Current Behavior**
   Add steady-state polling after catch-up, fresh header/block connect behavior,
   peer churn handling, and status that distinguishes steady caught-up from
   blocked no-progress.

7. **Phase 74: Operator Evidence And UAT Harness**
   Extend live-mainnet evidence into full-sync/stay-current reports, support
   bundle summaries, Cargo/Bazel operator commands, and deterministic fixtures.

8. **Phase 75: v1.6 Threat Model, Parity Roots, And Release Closeout**
   Update parity docs, threat model, release readiness, README wording, and
   deterministic checker. Keep public-network full-sync UAT outside default
   verification.

If Phase 69 or Phase 70 uncovers large storage or validation gaps, split them
before scheduler work. Scheduler evidence is only meaningful after durable
connect state is credible.

## Test Strategy Implications

Default verification must remain deterministic and public-network-free. The test
shape should be layered:

| Layer | Tests |
| --- | --- |
| Pure consensus and chainstate | Height-aware flags, contextual block validation, UTXO connect/disconnect deltas, undo correctness, reorg selection, invalid block rejection |
| Pure network | Header locator continuation, best-chain selection, getdata planning, `notfound` attribution, compatibility transcripts, peer message-order edge cases |
| Node storage | Record-keyed header/block/chainstate round trips, schema mismatch, interrupted connect journal, corruption markers, reindex/repair guidance, same-datadir reopen |
| Sync scheduler | Synthetic peers for header catch-up, block catch-up, no-progress during IBD, steady no-progress at tip, peer rotation, resource limits, pause/resume/shutdown |
| Integration | Deterministic long synthetic chain with multiple header batches, block bodies, restart midway, reorg branch, and support/status assertions |
| RPC/CLI/dashboard | Shared status fixture tests so all renderers agree on completion, tip lag, recovery, and unavailable reasons |
| Scripts/docs | v1.6 release-boundary checker, live UAT report parser tests, support bundle redaction tests |
| Opt-in UAT | Public-mainnet full-sync/stay-current command that writes local JSON/Markdown reports outside `bash scripts/verify.sh` |

Add benchmarks or measured smoke checks for storage write amplification and
connect throughput. Do not make elapsed-time thresholds release gates unless a
later policy explicitly does that.

## Architecture Risks

### Whole-Snapshot Storage Will Not Scale

The current Fjall adapter stores headers and chainstate through large versioned
JSON snapshots. That is acceptable for earlier evidence targets but risky for
mainnet full sync. v1.6 should address this before claiming sync-to-tip.

### Chainstate Connect Clones The UTXO Set

`Chainstate::connect_block_with_current_time` clones the UTXO map before
applying each block. That protects invariants but is likely too expensive for
mainnet. The pure core needs an incremental transition shape that remains
testable and rollback-safe.

### Validation Flags Need Mainnet Activation Semantics

The runtime currently uses a static `ScriptVerifyFlags::P2SH`. A truthful
mainnet full-sync claim needs height/time-aware validation behavior aligned with
the pinned Knots baseline for in-scope consensus rules.

### "Current Tip" Is Not A Local Absolute Fact

Without an external oracle, the node can prove it is connected to the best known
validated chain from observed compatible peers, not that it has found a global
absolute tip. v1.6 status and UAT language must define current-tip evidence
precisely and avoid overclaiming.

### IBD No-Progress And Steady At-Tip Look Similar

The current stop reasons focus on target reached, no progress, max rounds,
pause, and shutdown. v1.6 needs scheduler-level distinction between "blocked
while behind" and "steady because connected equals best known tip."

### Reorg Recovery Needs Durable Undo Guarantees

The pure chainstate supports reorg behavior, and the runtime has a
`reconcile_best_chain` path, but full-sync reorg recovery requires durable block
bodies and undo data for disconnect/reconnect across restarts. This is a
storage and atomicity risk.

### Long-Lived Peer Orchestration Can Blur Scope

Stay-current behavior requires continued outbound sync work, but not inbound
serving, relay, bans, address advertisement, or production uptime promises.
Docs, status labels, and release checkers should keep that boundary visible.

### Public-Network Evidence Is Inherently Flaky

The implementation should close through deterministic synthetic coverage plus
explicit opt-in UAT artifacts. `bash scripts/verify.sh` should continue to avoid
public-network, manual-peer, and real service-manager requirements.

## Bottom Line

v1.6 should be built as a sequence of hardening phases around the existing
daemon sync architecture:

1. define the truthful completion contract
2. make storage and chainstate mainnet-scale
3. make validation/connect height-aware and durable
4. add a full-sync scheduler and work queue
5. prove restart, reorg, and stay-current behavior
6. surface evidence through the existing status/support/parity channels

This preserves the functional-core / imperative-shell boundary while turning
the existing opt-in bounded review workflow into an auditable sync-to-best-known
mainnet tip and stay-current workflow.
