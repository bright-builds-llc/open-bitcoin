# Architecture Research

**Domain:** Open Bitcoin v2.5 BASIC compact-filter indexing and prune-aware BIP157 serving
**Researched:** 2026-10-03
**Confidence:** HIGH for traced source/baseline behavior; MEDIUM for proposed modules and commit protocol

## Recommendation and Settled Contract

Extend the existing functional core / imperative shell and single serialized chainstate/prune authority. Add owned BASIC generation, a same-database Fjall index, bounded catch-up, branch-aware lookup and transport-confirmed serving. No new production dependency, workspace crate, public default or production claim is required.

**Settled advertisement contract:** `NODE_COMPACT_FILTERS` follows explicit enablement and permitted per-peer service policy, independently of `synced`. Serve any complete indexed range during catch-up. Do not gate the service bit on catching up to tip. Fresh or lagging indexing beyond deleted historical inputs refuses; never start at prune height or implicitly repair/redownload. BASIC/type `0` is the only selected filter; V0/type `2` and BIP37 stay excluded. [K1–K4]

Material guidance: `AGENTS.md`, Bright Builds sidecar, placeholder-only overrides, architecture/verification/Rust standards, current PROJECT/ARCHITECTURE and the v2.4 audit. Existing deletion/lock authority and the audit's three accepted advisories constrain the new persistence/recovery work; they are not permission to expand unrelated cleanup.

## Standard Architecture

### System Overview

```text
Configuration / authenticated RPC / inbound and outbound peers
                         |
         Existing daemon and transport shell adapters
                         |
       ManagedNetworkHandle: one serialized authority
            |                  |                 |
       Chainstate          Filter lifecycle    Request plans
       staging/flush       catch-up/reorg      + completions
            |                  |                 |
  Pure chainstate facts -> Pure BASIC generator / BIP157 policy
            |                  |                 |
       Existing Fjall database: coins + block/undo + filter index
                              |
                 Versioned rows + branch maps + cursors
```

### Component Responsibilities

| Component | Responsibility | Boundary |
| --- | --- | --- |
| Primitives | BASIC type, filter hash/header identities | Distinct raw bytes and display encodings |
| Codec | Bit/Golomb coding, filter serialization and six BIP157 message shapes | No hashing, storage or sockets |
| Pure generator | Raw-script set, SipHash-2-4 range mapping, bytes/hash/header | Block plus validated spent-script facts in; derived records out |
| Pure index transition policy | Contiguous progress, predecessor/branch validation, rewind, protection facts | No direct Fjall or clocks |
| Fjall filter adapter | Immutable block-hash records, active projection, cursor/checkpoint, integrity checks | Same database as existing node store; explicit fallible commits |
| Managed runtime owner | Serialize index transition with connect/reorg/prune/lock mutation | Existing `Arc<Mutex<ManagedPeerNetwork>>` authority, not another deletion owner [O2] |
| Transport adapters | Resolve bounded complete responses, encode/write/acknowledge | Socket writes outside authority; no earned success until write completion [O5] |
| RPC/operator projection | Baseline filter lookup, index summary, sanitized status/support | One authoritative model with enabled, synced and available range separately |

## Live Call Chains and Required Integration

| Existing path | Traced call chain | Required v2.5 change |
| --- | --- | --- |
| Durable startup | `open-bitcoind::open_runtime_store` → `open_authoritative_network_runtime` → `DurableSyncRuntime::open_with_runtime_activation` → `initialize` → recovered coins/chain metadata/undo → `ManagedNetworkHandle` | Index-enabled offline mode must select durable storage too. Validate/reconcile filter protection before recovery can resume pruning, before workers/listeners start. [O1, O3] |
| Outbound sync receive | `sync/session.rs` → `handle.receive_sync_message` → `ManagedPeerNetwork::receive_sync_message` → peer actions / `process_actions` | Filter requests must route here as well as through inbound handling. Live block acceptance must feed the index transition, not only status/report code. [O2, O4] |
| Connected block | `mempool_lifecycle` → `prepare_connect_block_with_current_time` → sealed mempool lifecycle → `commit_connected_block_lifecycle_transaction` → `commit_prepared_connect` → absorb + persist | Extract generated undo from the validated staged connect before consumption, then notify/process index work under the same owner. Preserve existing chain/mempool commit failure semantics. [O4] |
| Reorg | `reorg_to_branch` → `prepare_reorg` → preview + mempool transition → `commit_prepared_reorg` | Reconcile filter frontier and rewind active projection; retain old branch records; never publish an index branch based only on the preview. [O4, K5] |
| Ordinary maintenance | daemon `start_coins_flush_worker` → `flush_cycle` → `handle.flush_coins` → `automatic_prune::flush` | Drive a bounded index step or its checkpoint here; idle/offline index catch-up needs a real consumer even without new peer blocks. Index-enabled startup must not bypass this worker. [O6] |
| Manual/automatic delete | authority-owned `flush_applying_prune_plan` / ordinary flush → reload locks → prefix writes → paired unlink → coins/meta completion | Make filter protection and persistence part of the actual apply/recovery gates. Proposed planner-only protection is insufficient. [O6, O7] |
| Inbound serving | listener frame read → `resolve_inbound_wire_responses` → `ManagedRpcContext::prepare_inbound_wire_message` → `receive_message_for_durable_serving` → action translation → response plan → `resolve` → bounded write → acknowledgement | Add a filter response plan/reader independent of body presence. Carry branch/range identity and completion evidence through resolution/write. [O5] |
| Outbound response write | sync receive result → session `send_all_for_peer` | Use the same filter plan/completion semantics; prove writes rather than assuming inbound plumbing covers it. [O4] |

**Important existing seam:** startup `initialize` calls `resume_prune_intent` before it sets `ReadyToFlush`. Filter protection must already be durable and validated at that point; attaching an index after runtime construction leaves a recovery deletion window. **Important failure seam:** connect absorbs validated state before its persistence result returns, and reorg installs a preview before final commit. A filter hook must track accepted transitions and durable checkpoints explicitly; an ordinary success-only callback can miss state changes after a persistence error. [O3, O4]

## Recommended Project Structure

Proposed paths below are new modules, not current implemented capability. Preserve `foo.rs` plus `foo/` convention and existing crate graph.

```text
packages/
  open-bitcoin-primitives/src/compact_filter.rs       typed identities
  open-bitcoin-codec/src/compact_filter.rs            filter/bit codec
  open-bitcoin-codec/src/compact_filter/              bounded helpers
  open-bitcoin-codec/src/filter_messages.rs           BIP157 payloads
  open-bitcoin-consensus/src/compact_filter.rs         pure generation
  open-bitcoin-consensus/src/crypto/siphash.rs          byte API extension
  open-bitcoin-chainstate/src/filter_index.rs          pure cursor/rewind facts
  open-bitcoin-network/src/compact_filter.rs           request/serve policy
  open-bitcoin-node/src/storage/filter_index.rs        versioned records
  open-bitcoin-node/src/storage/fjall_store/filters.rs  atomic adapter
  open-bitcoin-node/src/network/filter_index.rs        lifecycle/catch-up
  open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  open-bitcoin-rpc/src/dispatch/compact_filters.rs     baseline lookup RPCs
```

Modify existing daemon config/startup, storage namespaces/schema, chainstate staging/flush, network message dispatch/service flags/permissions, inbound response plans and sync session send paths. Extend shared status/support and existing harness/benchmarks; do not build a separate dashboard, index service or database.

Codec currently depends only on primitives while consensus depends on codec. Keep hash computation outside codec. Consensus should accept spent-script facts, avoiding a dependency back to chainstate merely for `BlockUndo`. `StagedChainstateConnect` already carries public undo; `PreparedChainstateConnect` is opaque and currently exposes only position, so add a narrow validated-input accessor or prepared filter projection at that seam. [O4, O8]

## Architectural Patterns

### Immutable Records, Branch-Aware Projection

Persist record identity `(BASIC, block hash)` containing filter bytes/hash/header/predecessor. Maintain active height→block-hash projection and branch-aware lookups separately. Raw-script inputs come from the validated block and undo; genesis has no undo. Cache bounded records if useful, but durable absence remains authoritative. Keep completed filter records when bodies/undo are pruned. [K3, K5]

Knots keeps disconnected filter rows accessible by hash and looks up ranges along the stop block's ancestry. Active height alone cannot answer a stale-branch request. Validate parent/header chaining and capture the full requested branch before emitting anything. Retained filter records do not make block bodies available and must not change wallet rescan eligibility or `Pruned` labels. [K5, O9]

### Progress Separate from Durable Resume Authority

Maintain explicit processed frontier, committed filter rows, durable resume cursor, current active branch identity and synced state. Do not collapse them into one height/counter. A record can be durable before the chain checkpoint catches up; it must not make the durable resume cursor claim chainstate that will disappear on restart. Knots explicitly warns against committed index state ahead of flushed chainstate. [K6]

Use the existing successful coins/chain-metadata checkpoint as the fence for publishing a durable index cursor. If records have been written ahead, reopen derives the active projection/frontier from the recovered chain anchor, preserving immutable branch records without trusting stale active mappings. Header/coins/checkpoint errors leave a conservative lock/frontier; status exposes the failure instead of fabricating progress. Protect the earliest needed history until a safe committed transition permits release.

### Atomic Index Commit and Protection

Within the shared owner, prepare bounded records and branch changes, then commit records/projection/cursor/protection through a same-database atomic `SyncAll` batch where its chain checkpoint preconditions hold. If protection is a separate effect, write filter/checkpoint proof first and advance protection second. Never advance the lock first. A crash may retain extra payloads but must not delete required unindexed block/undo. Existing paired deletes are separately atomic; design the cross-operation protocol rather than claiming a transaction across all current effects. [O7, F1]

Reserve internal filter-index lock identity and owner. Operator set/clear must reject attempts to overwrite/remove internal protection. An explicit disable transition stops catch-up and invalidates in-flight work before releasing protection; ordinary status may show sanitized owned-lock counts. Current public named-lock CRUD allows general replacement/clear, so this control is new behavior needing a real production guard. Catch-up/manual/automatic/recovery deletion must all use that guard. [O7]

### Bounded Work under One Authority

Process a fixed batch of blocks/bytes per catch-up turn and yield between batches. Revalidate branch identity and durable inputs at each turn. Initial historical replay and live connects share one append policy; while catch-up is behind, new connects enlarge the backlog instead of creating out-of-order headers. Do not duplicate full-chain metadata/undo into another index-owned cache. The current runtime already loads undo at reopen; this milestone should not add a second unbounded copy. [O1, K6]

Use existing periodic shell scheduling to call the owner; an index task may prepare work concurrently only if branch/checkpoint identity is revalidated before commit. Network/socket writes must never hold the authority lock. These are design recommendations; batch budgets require phase profiling and tests.

## Data Flow and Recovery

1. **Startup:** parse default-off index and serving activation → open Fjall for index-enabled durable/offline mode → verify record schema/checksums/cursor and internal protection → existing coins/prune recovery → recover chain anchor → reconcile filter branch/cursor → preflight block+undo availability → start bounded catch-up and existing listeners/workers. Fresh/lagged missing history refuses without implicit repair. [O1, O3, K4]
2. **Connect:** validate/stage block+undo → prepare pure filter inputs → commit accepted chain lifecycle → append in order or queue indexed backlog → commit index rows → use chain checkpoint fence to advance resume cursor/protection → derive status. Genesis uses the zero preceding header. [O4, K3, K6]
3. **Disconnect/reorg:** capture validated transition → find common ancestor → preserve old hash rows → rewind projection/header under authority → index replacement branch in order → commit fenced cursor/protection. A deeper-than-retained required input fails explicitly; no replacement fabricated from current coins. [K5, K6]
4. **Prune:** actual planner reloads internal protection → filter records needed by deletion already committed → existing prefix and paired deletes → existing checkpoint/recovery protocol → filter rows remain readable. Deleted-body receipts still own cache cleanup. [O6, O7, O9]
5. **Serve:** validated peer/request → BASIC/per-peer services/stop ancestry/range policy → bounded complete index lookup → all requested responses prepared → encode → socket write → acknowledge written prefix or abort. Missing records yield Knots lookup behavior, not shortened ranges or `notfound` substitutes. [K2, K7, O5]
6. **Reopen:** recover the real Fjall store after each fault boundary; reconcile to durable chain anchor and validate predecessor/header/rows/protection. `synced` is recomputed, not restored from a hopeful cached boolean. Existing finish-or-Repair prune refusal remains permitted and visible. [O3, K6, O9]

## Exact Baseline Lookup Outcomes

| Surface / condition | Required outcome |
| --- | --- |
| Global `-peerblockfilters` without BASIC index | Configuration error; global service bit otherwise follows enablement, independent of synced. [K1] |
| Explicit `blockfilters` permission | Requires BASIC index; peer initialization adds the service bit for permitted peers. Audit implicit/all permissions separately against pinned behavior. [K1] |
| Unsupported type, unknown/disallowed stop, start above stop, oversized range | Knots disconnects; inclusive caps 1,000 filters/2,000 hashes. Stop policy accepts active blocks or script-valid sufficiently recent stale blocks under both time/work-age checks. [K2] |
| Valid filter range partly absent | No range response; no shortened prefix. All filter rows are loaded before emission. [K7] |
| `getcfheaders` predecessor missing or any hash missing | No response; genesis predecessor is zero. [K7] |
| `getcfcheckpt` checkpoint missing | No response; return headers at positive 1,000-block intervals, excluding genesis. [K7] |
| Complete indexed range while catch-up incomplete | Serve normally; no global synced gate. [K2, K7] |
| `getblockfilter`: unknown filter name / block unknown | `-5`, respectively `Unknown filtertype` / `Block not found`. [K8] |
| `getblockfilter`: index disabled | `-1`, `Index is not enabled for filtertype basic`. [K8] |
| Lookup absent for block never script-valid/connected | `-5`, `Filter not found. Block was not connected to active chain.` [K8] |
| Lookup absent for connected block while index lagging | `-1`, `Filter not found. Block filters are still in the process of being indexed.` [K8] |
| Lookup absent for connected block while synced | `-32603`, `Filter not found. This error is unexpected and indicates index corruption.` [K8] |
| Lookup succeeds while index lagging / retained stale or pruned block | Return filter hex and header hex; synced is consulted to explain absence, not deny successful lookup. [K8] |

Scope models BASIC only. Knots locally recognizes V0 as well; do not accidentally add its generator/RPC/serving capability. Record the scoped unsupported V0 boundary in parity artifacts rather than claim all Knots filter types.

## Dependency Build Order

| Order | Deliverable | Depends on / integration proof |
| --- | --- | --- |
| 1 | Typed identities, byte SipHash, bit codec, BASIC generator/commitments | Pinned byte/header vectors and SipHash tails; no storage |
| 2 | Index schema, atomic adapter, branch lookup and cursor/checkpoint policy | Generation; real Fjall commit/failure/reopen proof |
| 3 | Startup/catch-up/connect/reorg/checkpoint owner integration | Storage + staged undo; demonstrate actual daemon/idle consumer and fenced progress |
| 4 | Reserved protection and prune/recovery coordination | Durable cursor policy + same runtime owner; real deletes, reopen and refusal after lost history |
| 5 | Wire codecs/policy, per-peer advertisement, RPC lookup and transport completions | Complete durable branch lookup; inbound and outbound wire/acknowledgement proof |
| 6 | Shared operator/support evidence and release guardrails | Real lifecycle and writes; one deterministic integrated scenario, parity roots, final native verification |

Wire shape work can run alongside storage, but public-facing activation cannot precede durable complete-range and prune/recovery proof. No phase may count only library existence or a static support flag as runtime completion.

## Deterministic Integration and Crash Proof

| Evidence case | Required observable proof |
| --- | --- |
| Generator oracle | Exact BASIC bytes/hash/header from pinned fixtures, duplicate scripts, spent scripts, OP_RETURN/empty rules, SipHash tails and bit boundaries |
| Actual connect/reorg | Validated runtime blocks/undo reach the owner; branch headers and hash lookups remain correct after real disconnect/replacement |
| Ordinary catch-up | Enable index on retained history through production startup; bounded scheduled work advances without requiring another inbound block |
| Real prune + reopen + wire | Generate/index before real Fjall paired delete; prove bodies/undo absent, filters present; reopen production durable runtime; handshake and decode all six wire messages, including old filter ranges |
| Catch-up service | Observe advertised service while synced=false; complete indexed range succeeds, crossing an unindexed range sends no partial response |
| Stale branch / branch race | Hash/ancestor selection survives equal-height replacements; allowed stale range succeeds, disallowed stop disconnects; captured response never mixes branches |
| Missing-history refusal | Fresh and lagged index activation after real deletion refuses; no cursor jump, zero-header suffix or hidden redownload |
| Reserved lock | Authenticated operator cannot overwrite/clear internal lock; disable/re-enable and restart preserve intended ownership; automatic/manual/recovery paths obey it |
| Transport failure | Budget refusal and partial socket-write failure earn no full-range success; only successfully written prefix is counted, with cleanup and retry evidence |
| Corruption/failed batch | Missing bytes, wrong predecessor/header/hash, mixed cursor mapping and failed commit surface distinct errors; no phantom synced/cursor/protection advancement |

Inject failures before/after: filter batch, protection commit, paired delete, coins/meta checkpoint, cursor publication, reorg rewind, and response write. Reopen after persisted boundaries using concrete Fjall and durable coins. If a fixture substitutes `MemoryCoinsView` or sparse codec-valid blocks, state that limitation; it does not prove a continuous consensus chain or a production durable-coins crash. Preserve the v2.4 allowed interrupted-prune Repair refusal, generic-sink advisory and support-counter crash undercount; filter progress must use records/checkpoints rather than deletion summary counters. [O9]

## Scaling Considerations

| Pressure | Initial architecture | Optimize only with evidence |
| --- | --- | --- |
| Historical catch-up | Bounded batch reads/generation; one owner; no full-history copying | Prepare outside lock with branch revalidation if measured lock duration hurts live paths |
| Serving many ranges | Per-peer request/output-byte limits; complete lookup before output | Bounded shared record cache or chunked lookup that still verifies completeness before first emission |
| Persistent filter growth | Separate visible index-byte accounting; no implicit filter pruning | Future explicit retention mode requires a separate BIP157 availability contract |
| Chain transition/storage failures | Immutable branch rows and conservative resume fence/protection | Optimize write cadence after fault/reopen tests, not before |

## Anti-Patterns

- Gating `NODE_COMPACT_FILTERS` on synced contradicts the settled Knots contract; advertise explicit enablement and show sync separately.
- Indexing from current UTXOs or regenerating on each request loses spent scripts and breaks after pruning; generate with validated undo and retain records.
- Height-only index keys mix branches after reorg; retain block-hash identity and predecessor commitments.
- Lock advancement or persisted cursor before durable proof can destroy needed history; use conservative commit/checkpoint ordering.
- Registering protection after `initialize` misses recovery deletes; make startup recovery ordering explicit.
- Activating only codecs, helpers or static permission labels leaves the feature disconnected; prove actual daemon maintenance, transport reads/writes and reopen.
- Applying the limited-body 288+2 window to filters unnecessarily denies retained historical records; keep availability policies distinct.
- Restoring `synced` from status counters or treating a generic no-op sink as index success creates false authority; require concrete records and checks.

## Sources

All paths are repo-relative and were inspected on 2026-10-03. HIGH confidence means source-confirmed existing behavior; proposed file layout, scheduling budgets and persistence protocol remain MEDIUM until implementation verification.

- **O1:** `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:301–380,134–145`; `packages/open-bitcoin-node/src/sync/open_runtime.rs:30–85` — durable startup and workers.
- **O2:** `packages/open-bitcoin-node/src/network/runtime_authority.rs:65–132,280–330,536–545`; `network.rs:364–445` — shared authority and receive paths.
- **O3:** `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:209–239` — coins recovery and prune-intent resume before readiness.
- **O4:** `packages/open-bitcoin-node/src/chainstate.rs:183–204,295–380`; `network/mempool_lifecycle.rs:113–178`; `network/lifecycle_projection/authority.rs:545–571`; `sync/session.rs:399–447`; `sync/block_response.rs:117–172`; `packages/open-bitcoin-chainstate/src/engine.rs:48–62` — staged undo, connect/reorg, persistence and transport flow.
- **O5:** `packages/open-bitcoin-rpc/src/context.rs:384–429`; `context/inbound_wire.rs:111–174,258–338`; `inbound_listener/connection_runtime.rs:200–265,575–604`; node `network/action_translation.rs:76–113` — plan/resolve/write/acknowledge seam.
- **O6:** `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs:68–90,161–171`; node `network/runtime_authority/automatic_prune.rs:132–173`; `chainstate/flush_lifecycle.rs:338–391,554–566` — real ordinary maintenance and deletion ordering.
- **O7:** node `network/runtime_authority/prune_flush.rs:60–115`; `storage/fjall_store/prune.rs:79–103,211–244`; `chainstate/fjall_store.rs:130–135` — lock CRUD, concrete deletes and recovery.
- **O8:** consensus/codec/network Cargo manifests; `packages/open-bitcoin-consensus/src/crypto/siphash.rs:45–87`; chainstate `types.rs:15–40` — crate graph and reusable hashing/undo.
- **O9:** `.planning/milestones/v2.4-MILESTONE-AUDIT.md`, Actual retention and limits / Advisory debt; `.planning/PROJECT.md`, Current Milestone and Current State — accepted caveats and claim boundaries.
- **K1:** Knots `src/init.cpp:1075–1104,2358–2361`; `src/net_processing.cpp:1548–1555` — global enablement and per-peer permission advertisement.
- **K2:** Knots `src/net_processing.cpp:86,1865–1871,3156–3206` — exact preparation/disconnect and stale-stop policy.
- **K3:** Knots `src/blockfilter.cpp:187–224,258–282`; `src/index/blockfilterindex.cpp:268–285` — generation, undo and commitments.
- **K4:** Knots `src/init.cpp:2485–2496`; `test/functional/feature_index_prune.py:123–155` — missing-history startup refusal.
- **K5:** Knots `src/index/blockfilterindex.cpp:336–370,476–510` — rewind, branch rows and complete range lookup.
- **K6:** Knots `src/index/base.cpp:227–261,274–319,322–388,430–446`; `blockfilterindex.cpp:145–165` — processed/committed progress, flush fence and protection ordering.
- **K7:** Knots `src/net_processing.cpp:3219–3315`; `src/index/blockfilterindex.h:20`; `test/functional/p2p_blockfilters.py` — missing-range no-response, counts and checkpoints.
- **K8:** Knots `src/rpc/blockchain.cpp:3340–3391`; `src/rpc/protocol.h:34–42`; `src/rpc/node.cpp:422–458` — exact RPC lookup outcomes and index summary.
- **F1:** [Fjall 3.1.4 OwnedWriteBatch](https://docs.rs/fjall/3.1.4/fjall/struct.OwnedWriteBatch.html) — atomic writes across same-database keyspaces and durability selection; checked alongside concrete existing adapter use.
- **Normative specifications:** [Official BIP157](https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki), [Official BIP158](https://github.com/bitcoin/bips/blob/master/bip-0158.mediawiki); generation/wire contracts cross-checked against pinned Knots `a9aee730466ac67d35a3c03ee24676be5e045878`.

*Active v2.5 architecture research; earlier v2.4 research remains recoverable in Git.*
