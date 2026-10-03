# Pitfalls Research

**Project:** Open Bitcoin — v2.5 Prune-Aware Compact-Filter Serving (BIP157/158)
**Domain:** BASIC compact-filter generation, durable indexing and bounded serving on the shipped single prunable chainstate
**Researched:** 2026-10-03
**Confidence:** HIGH for source-confirmed hazards/protocol; MEDIUM for proposed Fjall recovery contract until implemented

## Scope and Evidence

This document concerns new v2.5 filter behavior. Existing v2.4 wallet/prune capabilities remain shipped; their accepted advisories matter only at new index seams. Source truth is local Knots `29.3.knots20260210`, submodule commit `a9aee730466ac67d35a3c03ee24676be5e045878`, plus inspected first-party source. Official BIP157/158 specifications were checked on 2026-10-03; both were assigned 2017-05-24 and are deployed specifications. Local guidance, Bright Builds architecture/verification rules, complete active lessons, PROJECT, [FEATURES](FEATURES.md) and [archived v2.4 audit](../milestones/v2.4-MILESTONE-AUDIT.md) informed these findings. Work remains inside the GSD new-milestone workflow.

The highest-risk boundary is startup recovery before resumed deletion. First-party `DurableSyncRuntime::open_with_runtime_activation` calls `initialize` before constructing the manager; `initialize` loads locks and resumes a prune intent. Restoring index protection later in the manager would miss that deletion boundary. [N1, N2]

Phase names below are recommended work packages, not invented canonical phase numbers: **Core** (BASIC generation/codecs), **Index** (activation/persistence/recovery), **Prune** (index-owned deletion coordination), **Serving** (peer/RPC production wiring), **Evidence** (operator projection and final parity/verification). Index recovery and initial protection must precede enabling the Prune/Serving consumers.

## Critical Pitfalls

### Pitfall 1: Internally Consistent but Noncompatible GCS Bytes

**What goes wrong:** Filters and headers agree with the new implementation but differ from Knots, so clients cannot validate them against other peers.

**Why it happens:** Reusing displayed big-endian hash strings for the SipHash key; modulo reduction instead of the high half of a 128-bit product; wrong SipHash variant, P/M, unary/bit order, count prefix or final padding; deduplicating hash collisions instead of raw scripts. Self-roundtrip tests reproduce the same bug in producer and consumer.

**How to avoid:** Implement type 0, SipHash-2-4, P=19/M=784931, first 16 little-endian block-hash bytes as key, multiply-high reduction, raw-script set semantics, sorted hash values, delta Golomb-Rice coding and CompactSize count. Empty bytes are `00`. Hash/filter-header operations must use hash bytes rather than displayed hex. Use the pinned independent vectors and explicit collision/empty/count-boundary cases. [K1, K2, K8; BIP158]

**Warning signs:** Only roundtrip or same-code expected-value tests; generator samples omit known Knots hex; tests reverse hash bytes until an example passes; `hash % range` or a set of reduced hashes in production code.

**Phase to address:** Core; independently verify before any durable records are written.

### Pitfall 2: Incomplete or Invented Spent-Script Inputs

**What goes wrong:** Filters omit relevant spends, causing false negatives; a wallet might miss activity even though a filter appears validly encoded.

**Why it happens:** Reading only outputs/current UTXOs; reading coins after connect has removed spends; replacing unavailable undo with an empty list; blanket OP_RETURN exclusion across all element sources; treating a wallet snapshot as historical authority.

**How to avoid:** Build from validated block plus complete historical spent-output evidence. Knots uses block undo for every non-genesis block; genesis uses no undo. Exclude empty scripts and outputs beginning OP_RETURN, but mirror the distinct undo-script rules. Include same-block-created-and-spent scripts. Validate input/undo correspondence rather than accepting mere undo-key presence. Missing history refuses indexing, never becomes a coinbase-only filter. [K1, K3]

**Warning signs:** Generator takes only `Block`; non-genesis append succeeds with absent undo; prevout lookup consults only current coins; fixture transactions never spend anything.

**Phase to address:** Core input contract and Index activation/append validation.

### Pitfall 3: Enabling a Complete-Looking Index After History Is Gone

**What goes wrong:** Enabling BASIC after pruning produces a suffix whose zero predecessor falsely looks like a complete genesis index, or an old index resumes past a gap.

**Why it happens:** Assuming all pruned datadirs are indexable; starting at prune height; trusting an old synced flag; treating current coins or borrowed peer filter headers as sufficient provenance.

**How to avoid:** Preflight required body/undo history from the valid prefix/common-ancestor boundary. New index plus missing history refuses startup; disabled index overtaken by prune also refuses reactivation. Existing complete prefix plus present required suffix can resume even if older already-indexed bodies are gone. Preserve valid records on refusal. No implicit download, destructive reindex, prefix erasure or repair. Knots startup body gate and functional prune/index test provide the baseline; undo preflight is an explicit earlier diagnostic difference. [K3, K4, K9]

**Warning signs:** Fresh index best height immediately equals prune height; “synced” only checks best>=tip; startup catches missing-history errors and continues; config enablement starts network fetch/repair.

**Phase to address:** Index activation before peer serving or new prune integration.

### Pitfall 4: Manager-Level Recovery Arrives After Startup Deletion

**What goes wrong:** A persisted prune intent deletes body/undo required by the filter index before the newly constructed runtime restores its protection.

**Why it happens:** Index recovery is placed beside network/manager construction rather than before existing initialization effects. The current durable open calls initialize at line 38; initialize resumes prune intent at lines 235–236; manager construction follows. The daemon applies runtime prune mode only after that open. [N1, N2, N6]

**How to avoid:** Make index metadata/protection validation an explicit pre-resume prerequisite. Recover coins authority as required, validate index branch/cursor/protection, then authorize resumed prune deletion, then expose ready manager/serving. Include enabled-on-open configuration in this early contract; do not rely on later `set_prune_mode`. Corrupt/absent/unverifiable protection must block affected deletion or refuse startup. Do not construct a second ad-hoc recovery owner.

**Warning signs:** New index recovery is called only after `initialize`; test creates runtime then installs a lock; no crash fixture contains both lagging index and live prune intent; disabled-index-to-enabled transition ignores startup deletion.

**Phase to address:** Index storage/recovery design first; Prune phase must prove a real production reopen with pending intent.

### Pitfall 5: Pruning Races Ahead of Filter Durability

**What goes wrong:** A worker reads/encodes a filter, advances a watermark, then deletion removes the only body/undo before filter bytes and safe recovery progress are durable.

**Why it happens:** Presence probes, append, cursor write, lock movement and prune application are treated as independent successful steps; the existing owner serializes prune but the new index uses another authority or releases it between unsafe steps.

**How to avoid:** Define one safe deletion watermark backed by durable filter bytes/hash/header and a recoverable branch-aware cursor. Protect required bodies/undo until that contract is committed. Re-check current protection in both manual and automatic application, not just planning. If generation is outside the owner, guard its input identity and verify authority/progress again before publication. Errors preserve protection and stop false progress. Test every append/commit/lock-write/deletion interruption point. Knots flushes filter files before index metadata and publishes best block after lock updates. [K3, K5; N3, N4]

**Warning signs:** Lock moves using processed_count; pruning follows an asynchronous queue acknowledgment; full verifier only exercises successful commits; automatic measurement reuse ignores changed index protection.

**Phase to address:** Index durability contract, then Prune integration; required before real deletion with indexing enabled.

### Pitfall 6: Operator Lock CRUD Weakens Internal Protection

**What goes wrong:** An operator clears or replaces the index-owned lock and the next legal prune deletes catch-up inputs.

**Why it happens:** Reusing a normal user-managed lock name without ownership policy. Current `replace_prune_lock`/`clear_prune_lock` mutate arbitrary named records under the owner. Serialization alone does not distinguish internal from operator authority. [N3]

**How to avoid:** Reserve/protect internal identity at both CRUD and deletion boundaries, or enforce index watermark independently of user locks. Reject weakening an active index guarantee; list internal protection truthfully without offering unsupported editing. Restore this invariant during pending-intent recovery too. Keep the existing buffered lock rule and soft target; missing bytes cannot be restored by adding a lock later. [K5, K6; N2, N3]

**Warning signs:** Test updates the index lock through public replace; clear returns true for reserved identity; protection exists only in a UI disable state; direct store/resume path bypasses owner policy.

**Phase to address:** Index ownership policy and Prune/operator integration.

### Pitfall 7: Cursor Commits Ahead of Recoverable Chainstate

**What goes wrong:** After a crash, filter progress claims a chain tip that durable coins/chain metadata do not establish, or a reorg checkpoint claims the replacement branch while durable authority still names the old one.

**Why it happens:** Assuming an atomic filter batch also commits coins/chain metadata; equating stored record existence or announced tip with an authoritative restart checkpoint.

**How to avoid:** Separate computed records from safely resumable progress. Persist branch identity and verify it against recoverable coins/chain metadata; clamp/rewind only through a verified replay contract, otherwise refuse. Extra unclaimed records can remain by hash, but cannot authorize deleted history or synced status. Knots Rewind explicitly avoids a commit that would run ahead of flushed chainstate, and ChainStateFlushed validates ancestry before committing. Treat Fjall coordination as its own documented design, not a copied Knots transaction guarantee. [K5; N1, N2]

**Warning signs:** Cursor advances in every append regardless of chain flush; test compares heights without hashes; restart trusts index metadata before recovered coins best-block; only clean shutdown is exercised.

**Phase to address:** Index checkpoint/recovery phase; verify again across actual reorg and interrupted prune.

### Pitfall 8: Active Heights Leak into Stale-Branch Responses

**What goes wrong:** getblockfilter for an old connected block returns a replacement filter; cfheaders/cfilters/checkpoints silently combine different branch commitments.

**Why it happens:** Storing only height-keyed data; deleting displaced filters; retaining cached headers keyed by height; starting replacement headers from old tip rather than common ancestor; assembling response while chain identity changes.

**How to avoid:** Index records by block identity, map active height to hash, retain displaced indexed records by hash and anchor every range to stop-hash ancestry. Rewind header/progress/protection to common ancestor. Pin request ancestry/view for assembly and guard changed authority before effects. Apply Knots stale-chain eligibility, not every known header and not blanket active-only rejection. Previously connected stale differs from never connected. [K3, K7, K10]

**Warning signs:** Only linear chain fixtures; cache keyed `height`; stale RPC rejected as unconnected; response uses current tip for a stale stop hash; checkpoints match a different fork.

**Phase to address:** Index reorg plus Serving range lookup; continuous validated fork proof required.

### Pitfall 9: Corruption or Backend Failure Becomes an Empty Filter

**What goes wrong:** Missing/corrupt bytes become legitimate `00`, ready progress or empty successful range, permanently lying to clients.

**Why it happens:** `unwrap_or_default`, broad catch/fallback, assuming any failed metadata read means a missing key, or treating absent-after-ready the same as initial catch-up.

**How to avoid:** Preserve distinct NotEnabled, UnknownBlock, NeverConnected, CatchingUp, MissingRequiredHistory, Corruption and backend failure facts. Validate stored block identity/hash/header and prefix continuity; bounds-check decoded records. Knots CustomInit distinguishes absent position metadata from read failure; ReadFilterFromDisk checks checksum; getblockfilter distinguishes indexing from corruption. Propagate storage faults and preserve protection; do not silently clear index data. [K3, K11]

**Warning signs:** All errors mapped to None; an unavailable record yields one-byte zero filter; restart erases metadata and “rebuilds”; counters advance despite failed durable writes.

**Phase to address:** Index parsing/recovery and RPC/Serving taxonomy.

### Pitfall 10: Protocol Limits Are Mistaken for Total Resource Bounds

**What goes wrong:** Legal requests exhaust memory/I/O or block progress, despite correct 1,000-filter and 2,000-hash range checks.

**Why it happens:** Tiny tests; count limits without encoded-byte accounting; unbounded queued request batches; whole-index reloads; applying a filters cap to checkpoints whose response scales with stop height.

**How to avoid:** Match inclusive legal maxima and reject before subtraction/allocation when start>stop or stop/type invalid. Bound bytes, pending work, queue slots and decoded allocation as well as counts. Keep persisted lookup, bounded catch-up and bounded optional cache. getcfcheckpt emits all positive multiples of 1,000 through stop, so derive its resource cost from chain height/transport limits; document any local backpressure differences. Never compute filters from bodies on request. [K7; BIP157]

**Warning signs:** Per-request legal checks but no queue budget; Vector capacity comes from untrusted count; stop-start underflows; checkpoint path reuses 2,000 range cap; catch-up holds authority for all history.

**Phase to address:** Core boundary codecs, Serving governance and Index bounded work.

### Pitfall 11: Configuration, Completeness and Body Availability Collapse

**What goes wrong:** The bit is delayed until full sync against selected Knots semantics, or configured bit/synced is reported as proof of complete current coverage; pruned nodes are unable to serve retained filters because body gates are reused.

**Why it happens:** One ready boolean controls configuration, initial catch-up, durable cursor, per-request availability and body service; inert per-peer permission gains global effect accidentally.

**How to avoid:** Match Knots configured NODE_COMPACT_FILTERS and available complete-range serving during catch-up. Keep initial synced, current lag/branch identity, durable progress, request availability and body availability distinct. A lookup gap produces no P2P response, not a fabricated empty/truncated success; invalid type/hash/range follows Knots disconnect policy. The BASIC bit may coexist with NODE_NETWORK_LIMITED. Permission-only enabling stays per peer. Do not change getindexinfo baseline meaning to instantaneous tip equality. [K4, K5, K7, K11]

**Warning signs:** Service flag derives from current cursor==tip; bits imply archive history; status says complete because option is true; old-body prune window rejects cfilters; test only checks a configured struct, not version bytes.

**Phase to address:** Index status model and Serving activation/projectors.

### Pitfall 12: Helper Success Has No Ordinary Production Consumer

**What goes wrong:** Generator/index/prune/network helper tests pass while daemon catch-up never runs, ordinary connects never append, authenticated RPC uses a detached in-memory index, or peer responses never reach the socket.

**Why it happens:** Static wiring or direct helper fixtures substitute for the actual chain lifecycle; a response plan/enqueue is credited as served. v2.4 previously had phase-level passes while audit found missing ordinary automatic retention and wallet integration. Its closure evidence is historical precedent, not proof new filters are wired. [A1]

**How to avoid:** Trace config through durable open, ordinary connect/flush, catch-up, pending-prune recovery, RPC shared context, request dispatch and real local transport. Use one authoritative index projection. Extend consuming success/failure receipts; successful `write_all` earns local transport completion, not proof a remote client received/validated bytes. Test failed write/queued suffix cleanup/replayed receipt. Reopen actual production Fjall, validate filters across real prune and fork, and inspect actual callers beyond helpers. [N1, N4–N7]

**Warning signs:** Only MemoryCoinsView/in-memory index fixtures; direct `append` bypasses daemon; fresh feature has no production call-site; served count increases on prepare; UAT names an installed alias without repo-local commands.

**Phase to address:** Each component's production wiring; Evidence phase verifies complete ordinary flows before completion.

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Height-only filter index | Easy linear lookup | Loses branch identity and stale parity | Never for durable production path |
| Best-effort internal lock movement | Keeps progress moving on error | Deletes irrecoverable catch-up input | Never |
| Restore protection after manager open | Minimal code change | Startup prune recovery runs first | Never |
| Current coins as historical script source | Reuses adapter | Filters omit spends | Never |
| Fixed oversized filter cache | Fast repeated lookup | Unsupported memory bound | Only explicitly bounded measured cache |
| No checkpoint cache initially | Fewer moving parts | More reads for repeated checkpoint requests | Acceptable with existing work/byte/queue limits and measurements |
| Retain unclaimed records by hash | Safer interrupted append/reorg recovery | Extra index storage | Acceptable if identity/integrity verified; no progress or disk-bound claim |
| Sparse codec-valid pruning fixture | Cheap legal deletion setup | Does not prove valid spends/continuous ancestry | Supplement real validated-chain tests, never replace them |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Durable open -> initialize -> prune resume | Index validation added only to later manager | Validate enabled index protection before any resumed deletion. [N1, N2] |
| Operator locks -> prune owner | Reserve name only in CLI | Enforce in authoritative CRUD, application and resume. [N2, N3] |
| Automatic measurement memoization | Index protection changes but cached measurement stays reusable | Include/reload effective index protection under owner. [N4] |
| Pending prune -> cache cleanup | Filter availability inferred from body cache | Independent durable filter lookup; retain receipt-owned body/undo eviction. [N3, N4] |
| RPC and daemon contexts | Construct a separate ephemeral index | One shared authority/projection across durable runtime and authenticated RPC |
| Inbound/outbound transport | Add codecs with no request dispatch/send caller | Extend live local sessions, failures/receipts and completion facts. [N5, N7] |
| Knots options | Pretend v0 is an unknown Knots type | Explicit BASIC-only exclusion; truly unknown versus deferred type distinction. [K1, K4] |

## Performance Traps

Thresholds here are concrete triggers, not guessed production-scale capacity claims.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Generate for each peer | CPU and body reads grow per request | Persist once, lookup only | First repeated request for large block |
| Count-only queue bound | Memory grows despite legal 1,000-block ranges | Account encoded bytes and pending request work | Several simultaneous maximal requests |
| Whole-history catch-up under owner | Prune/RPC/peer progress stalls | Bounded batches and publication points | First large pre-existing history |
| Whole-index reload on reopen | Memory/startup work scales with filter history | Metadata/prefix validation and bounded reads | History exceeds small fixtures |
| Unbounded checkpoint cache/vector | Legal height drives excessive allocation | Derive count/encoded size and bounded cache | High stop height or repeated distinct branches |
| Assume prune target covers filters | Reported target fits while physical usage grows | Separate index accounting; preserve soft logical payload contract | Filters retained after repeated body deletion |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Falsely complete index across missing history | HIGH: clients accept incomplete coverage | Genesis prefix, strict activation refusal, no skipped history |
| Public lock operations weaken internal guarantee | HIGH: permanent history loss | Owned protection enforced below UI/API |
| Decode arithmetic/allocation from unchecked fields | HIGH: panic or resource exhaustion | Cheap-first parse, checked arithmetic, byte/count/work limits |
| Dynamic filter generation on peer request | HIGH: computation/I/O amplification | Indexed lookup only |
| Known header accepted as validated branch | HIGH: serving unvalidated/incoherent data | Pinned connected/stale eligibility and identity checks |
| Implicit repair or peer filter import | HIGH: unauthorized mutation/unverified provenance | Fail closed; future explicit workflow |
| Per-peer permission activates global/public serving | MEDIUM: scope escalation | Resolved peer-local capability and BASIC prerequisite |
| Support evidence includes raw paths/peer identifiers/scripts | MEDIUM: data exposure | Existing sanitization; categories/counters/bounded heights only |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Enabled labeled complete | Operator misreads capability | Show configured, initial catch-up, current lag and durable progress separately |
| Missing undo called not synced forever | Operator waits for impossible progress | Actionable missing-history refusal category; no implicit repair |
| Full prune target promise | Operator assumes total disk bound | Explain soft payload target, retained index and stalled protection |
| Serving filters implies serving matching old blocks | Light client retries impossible body request | Keep filter/body availability independent; another peer may hold body |
| Baseline RPC shape gains custom diagnostics | Breaks clients/parity | Preserve getblockfilter/getindexinfo; use existing Open Bitcoin status/support |
| UAT uses installed alias only | Contributor cannot reproduce checkout | Copy-pasteable Cargo/Bazel repo-local commands per AGENTS |

## "Looks Done But Isn't" Checklist

- [ ] **Generator:** Independent pinned vectors, not just self-roundtrip, prove exact bytes/hash/header.
- [ ] **Spent scripts:** Continuous validated spends, including same-block spends, prove undo inputs; absence refuses.
- [ ] **Activation:** First enable after prune and disabled-index overtaken cases fail without prefix deletion or mutation.
- [ ] **Catch-up:** Actual daemon startup/ordinary connect consumes bounded index work; existing records can serve while incomplete.
- [ ] **Recovery:** Index protection is valid before initialize resumes a real pending prune intent.
- [ ] **Durability:** Fault points between bytes/cursor/chain checkpoint/lock movement cannot release unsafe history.
- [ ] **Internal lock:** Actual authenticated replace/clear cannot weaken active index-owned protection.
- [ ] **Pruning:** Both ordinary automatic and manual concrete Fjall paths retain required inputs and preserve indexed filters.
- [ ] **Reorg:** Replacement and indexed stale RPC/P2P ranges/checkpoints follow their own ancestry after reopen.
- [ ] **Serving:** 1,000/1,001 and 2,000/2,001 boundaries, checkpoint heights, invalid disconnects and missing no-reply cases tested.
- [ ] **Resources:** Legal requests remain bounded by bytes/work/queue, not just range count.
- [ ] **Transport:** Real local send success/failure and partial-prefix failure earn only correct achieved effects.
- [ ] **Status:** Baseline fields remain Knots-shaped; shared operator evidence distinguishes capability/progress/body presence.
- [ ] **Claims:** No BIP37, V0 support, archive-scale, assumeutxo, public defaults/network CI, implicit repair or production/funds claims.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Wrong encoding before public serving/prune | MEDIUM | Disable affected serving, preserve evidence, fix generator and validate pinned vectors; rebuild only through deliberate authorized scope |
| Missing body/undo for required prefix/suffix | HIGH | Refuse index activation and explain required history; no mutation/download; plan explicit recovery separately |
| Interrupted index/protection checkpoint | MEDIUM/HIGH | Restore verified conservative protection and replay only available validated inputs; otherwise refuse before prune resume |
| Cursor ahead of recovered authority | HIGH | Validate ancestry and replay/rewind contract; retain unclaimed records by identity; refuse if history is absent |
| Corrupt filter/index read | HIGH | Surface corruption/backend failure; preserve protection/data; no automatic empty filter or reset |
| Reorg deeper than retained body/undo | HIGH | Stop/refuse affected indexing safely; explicit future recovery required; no hidden redownload |
| Queue/write failure | LOW | Abort unwritten suffix, release pending budgets/capabilities, record only local successful prefix effects; no fabricated receipt |

Recovery recommendations require phase-specific implementation proof. No new destructive operation is authorized by this research.

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| 1 Encoding | Core | Independent vectors plus byte-order/collision/empty boundaries |
| 2 Script inputs | Core + Index | Validated spends/same-block spends; missing/corrupt undo refusal |
| 3 Missing history | Index | Fresh after prune and disabled-prefix overtaken; no mutation |
| 4 Startup deletion ordering | Index before Prune | Production reopen containing incomplete index and live prune intent |
| 5 Durability/deletion race | Index + Prune | Append/checkpoint/protection/delete fault matrix in real Fjall |
| 6 Operator weakening | Prune + operator integration | Actual RPC lock CRUD cannot bypass owner/recovery gate |
| 7 Ahead-of-authority cursor | Index | Dirty shutdown/reorg with coins/metadata checkpoint lag |
| 8 Branch mixing | Index + Serving | Active/stale ranges/checkpoints under reorg and reopen |
| 9 Read/corruption fallback | Index + RPC/Serving | Error taxonomy and no fabricated empty success |
| 10 Resource exhaustion | Core + Serving | Boundary arithmetic, maximal/repeated requests and backpressure |
| 11 Collapsed readiness | Index + Serving/Evidence | Wire bit during catch-up, available-range success, gap no reply |
| 12 Missing production caller | Every owning phase + Evidence | Actual startup/connect/prune/reopen/RPC/local socket complete flows |

## Sources

Source-confirmed statements are HIGH confidence. Preventive designs and recovery steps are MEDIUM confidence until their named verification exists. No unverified ecosystem claims are used.

- **BIP157:** [Official specification](https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki), New Messages and Node Operation; [rendered specification](https://bips.dev/157/), accessed 2026-10-03.
- **BIP158:** [Official specification](https://github.com/bitcoin/bips/blob/master/bip-0158.mediawiki), GCS/BASIC contents/construction and test vectors; [rendered specification](https://bips.dev/158/), accessed 2026-10-03.
- **K1:** [Pinned blockfilter.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/blockfilter.cpp#L21-L103), hashing/encoding/types; [elements/header functions](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/blockfilter.cpp#L190-L282).
- **K2:** [BASIC constants](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/blockfilter.h#L89-L90).
- **K3:** [Index initialization/commit/checksum](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/blockfilterindex.cpp#L116-L191); [undo append/rewind](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/blockfilterindex.cpp#L268-L355); lookup/ancestry lines 358–507.
- **K4:** [Option/service activation](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/init.cpp#L1075-L1105); explicit permission prerequisite lines 2358–2362; [startup history gate](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/init.cpp#L2456-L2496).
- **K5:** [Commit/rewind authority warning](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/base.cpp#L227-L263); BlockConnected/ChainStateFlushed lines 279–364; synced wait/summary/protection publication lines 367–447.
- **K6:** [Buffered lock/deletion checks](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/node/blockstorage.cpp#L317-L359), automatic lock check line 445.
- **K7:** [All peer filter requests](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/net_processing.cpp#L3156-L3320); range constants 153–155; per-peer service permission 1551–1553; BlockRequestAllowed 1865–1871.
- **K8:** [Pinned filter vectors](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/test/data/blockfilters.json); [generator tests](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/test/blockfilter_tests.cpp).
- **K9:** [Prune retention/disable/resume/refusal test](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/test/functional/feature_index_prune.py#L66-L154).
- **K10:** [RPC stale lookup/errors](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/test/functional/rpc_getblockfilter.py#L22-L62); [P2P stale/bounds/disconnect tests](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/test/functional/p2p_blockfilters.py#L140-L273).
- **K11:** [getblockfilter readiness/corruption distinctions](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/blockchain.cpp#L3317-L3392); [getindexinfo](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/node.cpp#L409-L462).
- **N1:** [Current durable open](../../packages/open-bitcoin-node/src/sync/open_runtime.rs), lines 30–46: initialize before manager; chain metadata/undo hydration follows.
- **N2:** [Current initialize](../../packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs), lines 201–240: coins recovery, load locks, resume intent, ReadyToFlush; [prune resume](../../packages/open-bitcoin-node/src/storage/fjall_store/prune.rs), lines 154–219: supplied-lock/active-hash/coins-tip checks and delete.
- **N3:** [Current prune authority/lock CRUD](../../packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs), lines 60–116: effective lock reload, named replace/clear; cache eviction lines 18–40.
- **N4:** [Automatic retention](../../packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune.rs), lines 132–175: durable locks/revision measurement key; [prune apply](../../packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply.rs), lines 43–124: apply-time lock/keep/active-hash classification and paired unlink.
- **N5:** [Consuming transport completion](../../packages/open-bitcoin-node/src/sync/session/emission_terminal.rs), lines 22–53: send then acknowledge, abort suffix on failure; [TCP write](../../packages/open-bitcoin-node/src/sync/tcp.rs), lines 74–88: encode/write_all; [served-effect classification](../../packages/open-bitcoin-node/src/network/block_serving.rs), lines 124–126.
- **N6:** [Daemon authoritative open](../../packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs), lines 300–330: runtime open before prune network/mode configuration.
- **N7:** [Inbound block-serving completion](../../packages/open-bitcoin-rpc/src/context/inbound_wire.rs), Written outcome around line 331; extend the actual caller rather than only pure request helpers.
- **A1:** [Archived v2.4 audit](../milestones/v2.4-MILESTONE-AUDIT.md), original integration gaps, closure evidence, accepted stale-metadata/generic-sink/counter advisories and fixture limits; [feature decisions](FEATURES.md).

*Pitfalls research for: v2.5 Prune-Aware Compact-Filter Serving*
*Researched: 2026-10-03*
