---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 146-2026-09-21T20-57-36
generated_at: 2026-09-21T20:57:50.113Z
---

# Phase 146: Wallet Leftover-Snapshot Cutover - Context

**Gathered:** 2026-09-21
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Wallet rescan treats durable coins and payload-present blocks as chain
truth. Leftover chainstate `"snapshot"` blobs are non-authoritative on
the wallet path, including after same-datadir reopen.

This phase delivers SNAP-01 only. It cuts the wallet consumer off leftover
snapshot bytes so a later prune cannot resurrect snapshot-as-truth. It
does not delete block or undo payloads, record have-pruned, emit `Pruned`,
encode prune policy or lock windows, advertise `NODE_NETWORK_LIMITED`, or
add prune operator surfaces. Those belong to Phases 147–151. Functional-core
crates stay I/O-free. No assumeutxo, second chainstate, archive serving,
BIP37, public defaults, or production-funds wallet claims.

</domain>

<decisions>
## Implementation Decisions

### Wallet scan authority

- **D-01:** Wallet rescan authority is the durable coins view (coins
  best-block plus the coins UTXO set) together with payload-present block
  bodies. A leftover chainstate `"snapshot"` blob must not supply tip,
  UTXOs, undo, balances, or history on the wallet path.
- **D-02:** The cutover lives in the node shell. `open-bitcoin-wallet`
  stays I/O-free and keeps consuming an assembled scan view. It must not
  open Fjall or read `StorageNamespace::Chainstate` key `"snapshot"`.
  `WalletRescanRuntime` stops using `load_chainstate_snapshot()` as scan
  truth.
- **D-03:** Same-datadir reopen with a planted leftover snapshot that
  disagrees with durable coins must not change wallet balances or history.
  The rescan job protocol stays: bounded height chunks, persisted rescan
  jobs, and checkpointed progress. Only the chain input source changes.
- **D-04:** A wallet UTXO or history entry is admissible only when its
  creating block's payload bytes are present. Coins best-block alone does
  not authorize an entry whose creating payload is missing.

### Leftover blob disposition

- **D-05:** Leave the leftover `"snapshot"` blob on disk. This phase does
  not delete it. Deletion is not required for SNAP-01 and must not be
  presented as prune.
- **D-06:** Do not restore leftover snapshot writes. `persist_progress`
  already does not call `save_chainstate_snapshot`. Schema-1 one-way
  migration may still read the blob once to seed coins (Phase 141). After
  coins exist, that read must not feed wallet rescan.
- **D-07:** `ChainstateSnapshot` may remain a pure hydrate, export, and
  test helper. Wallet rescan must not treat a helper built from the
  leftover blob as live chain truth.

### Missing payload

- **D-08:** If a height inside the current rescan chunk lacks payload
  bytes, fail that chunk closed. Do not skip the height, do not invent
  transactions, and do not fall back to leftover snapshot UTXOs or undo.
- **D-09:** A missing payload stays `Unavailable` under the Phase 143
  rule. This phase does not relabel it `Pruned`.

### Have-pruned and unlink boundary

- **D-10:** Do not delete block or undo payloads. Do not set have-pruned.
  Do not emit `Pruned` or `block_status_pruned` on production paths.
  Contributors must be able to observe that payloads remain and
  have-pruned was not invented by cutover alone.
- **D-11:** Prune mode, the 550 MiB target, the 288-block keep window, the
  10-block lock buffer, Fjall key deletion, `NODE_NETWORK_LIMITED`, and
  operator prune surfaces stay in Phases 147–151.

### Operator and parity surface

- **D-12:** Do not add prune RPC, CLI, or dashboard fields. Existing
  wallet rescan status stays. Phase 150 owns prune operator evidence.
- **D-13:** Touched first-party Rust source and tests under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` get
  parity breadcrumbs through `docs/parity/source-breadcrumbs.json`, using
  explicit `none` only when no defensible Knots anchor exists. Full v2.4
  prune parity roots and the no-claim checker stay in Phase 151 (GRD-01).
- **D-14:** Verification remains `bash scripts/verify.sh`. Default
  verification stays deterministic and public-network-free. Historical
  `.planning/phases/` directories stay tracked.

### Claude's Discretion

- Whether the shell assembles the existing `ChainstateSnapshot` from
  coins plus payload-present blocks, or introduces a narrower scan-input
  type, as long as D-01 through D-04 hold and the wallet crate stays
  I/O-free.
- Which existing block-store and coins-view methods supply best-block and
  payload-present bodies.
- Fixture style for a planted leftover snapshot that disagrees with coins,
  as long as reopen rescan ignores that blob for balances and history.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase contract

- `.planning/ROADMAP.md` — Phase 146 goal, SNAP-01, success criteria;
  Phases 147–151 own policy, unlink, serving, operator surfaces, and
  parity roots
- `.planning/REQUIREMENTS.md` — SNAP-01 wording; out-of-scope prune,
  archive, assumeutxo, BIP37, and production-funds claims
- `.planning/PROJECT.md` — v2.4 prune milestone; first phase cuts wallet
  rescan off leftover snapshot bytes; functional core stays I/O-free
- `.planning/reports/NEXT-MILESTONE-CANDIDATES.md` — why leftover-snapshot
  wallet cutover is Phase 146 rather than its own milestone

### Locked prior decisions

- `.planning/phases/141-durable-fjall-coins-adapter/141-CONTEXT.md` —
  leftover snapshot is non-authoritative after schema 1→2 migration;
  reopen must not copy leftover UTXOs into the live view; wallet rescan
  was the remaining consumer (D-15, D-18, D-19)
- `.planning/phases/142-manager-flush-lifecycle-and-restart/142-CONTEXT.md`
  — `persist_progress` cut off leftover snapshot writes; blob may remain
  unread and must not be live UTXO truth (D-17)
- `.planning/phases/143-honest-stored-block-availability/143-CONTEXT.md` —
  Available requires payload bytes; missing payload is `Unavailable`;
  `Pruned` stays reserved until a real prune delete (D-01, D-05, D-06)

### Current wallet read seam

- `packages/open-bitcoin-node/src/sync/wallet_rescan.rs` —
  `required_chainstate_snapshot` loads the leftover chainstate snapshot
  and `partial_chainstate_snapshot` filters that blob for each chunk
- `packages/open-bitcoin-wallet/src/wallet/scan.rs` — pure
  `rescan_chainstate` rebuilds wallet UTXOs and tip from a
  `ChainstateSnapshot`
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` —
  `load_chainstate_snapshot` / `save_chainstate_snapshot` on chainstate
  key `"snapshot"`
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` —
  `persist_progress` persists headers and runtime metadata and does not
  write the leftover snapshot

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `WalletRescanRuntime` in
  `packages/open-bitcoin-node/src/sync/wallet_rescan.rs` already owns
  enqueue, bounded chunks, resume, and checkpointed jobs. Replace the
  chain input, not the job protocol.
- `open-bitcoin-wallet` `rescan_chainstate` is a pure function of a
  `ChainstateSnapshot`. The shell can keep that signature if it assembles
  the view from coins and payload-present blocks.
- Phase 141 coins migration and Phase 142 persist cutover already made
  durable coins the live UTXO truth for chainstate reopen.
- Phase 143 payload-present classification is the honesty rule for
  "payload-present blocks."

### Established Patterns

- Functional core stays I/O-free. Fjall access stays in
  `open-bitcoin-node` storage adapters.
- Leftover `"snapshot"` blobs stay on disk and are non-authoritative
  after coins exist. Schema-1 open may still migrate that blob once.
- `Pruned` is reserved. Missing payload without a real prune delete is
  `Unavailable`.
- New or touched first-party Rust files need parity breadcrumbs.

### Integration Points

- `WalletRescanRuntime::required_chainstate_snapshot` is the wallet path
  that still calls `FjallNodeStore::load_chainstate_snapshot`.
- RPC `rescanblockchain` and restart resume both flow through that
  runtime.
- Coins best-block and block-payload reads already exist on the node
  store from Phases 141–143. This phase wires those reads into rescan.

</code_context>

<specifics>
## Specific Ideas

Same-datadir reopen is the proof: a leftover snapshot whose UTXOs or tip
disagree with durable coins must not change balances or history after
rescan. Contributors must also be able to see that block and undo
payloads were not deleted and have-pruned was not set.

</specifics>

<deferred>
## Deferred Ideas

- Pure prune policy, 550 MiB target, 288-block keep, and 10-block lock
  buffer — Phase 147
- Fjall block and undo key deletion, have-pruned, and interrupted-prune
  recovery — Phase 148
- `NODE_NETWORK_LIMITED` and honest `Pruned` versus `Unavailable` labels
  — Phase 149
- Prune RPC, CLI, dashboard, locks, and support evidence — Phase 150
- v2.4 prune parity roots and no-claim checker — Phase 151
- Deleting leftover `"snapshot"` blobs
- Temporary IBD prune target (`-pruneduringinit`, FUT-27)
- assumeutxo, archive serving, BIP37, public defaults, and
  production-funds wallet claims

</deferred>

---

*Phase: 146-wallet-leftover-snapshot-cutover*
*Context gathered: 2026-09-21*
