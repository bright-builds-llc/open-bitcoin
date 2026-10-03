# Feature Research

**Project:** Open Bitcoin — v2.5 Prune-Aware Compact-Filter Serving (BIP157/158)
**Domain:** Explicit BASIC compact-filter indexing and lightweight-client serving on a single prunable chainstate
**Researched:** 2026-10-03
**Confidence:** HIGH for pinned behavior/protocol; MEDIUM for proposed recovery/integration design

## Scope and Evidence

Recommend BASIC filters only, generated from validated blocks and historical spent-output scripts, durably indexed before dependent payload deletion, and served through authenticated RPC and explicitly enabled peer transport. Reuse shipped v2.4 prune, locks, availability and wallet boundaries. These are proposed v2.5 features, not shipped capabilities.

The reference is local `packages/bitcoin-knots`, commit `a9aee730466ac67d35a3c03ee24676be5e045878` (Knots `29.3.knots20260210`). All K-source links below use that immutable pin. BIP157/158 were checked through the official Bitcoin BIPs repository and rendered specifications on 2026-10-03. Their assignment date is 2017-05-24; their current deployed specifications informed this research. No popularity or competitor-market claims are needed.

Material guidance: local AGENTS, Bright Builds sidecar, placeholder-only overrides, architecture/verification standards, both complete active lessons (7,188 bytes; 2,397 estimated tokens), and the existing GSD new-milestone workflow. Domain code stays first-party and I/O-free. [Archived v2.4 requirements](../milestones/v2.4-REQUIREMENTS.md) and [audit](../milestones/v2.4-MILESTONE-AUDIT.md) establish the shipped foundation. [Next-milestone candidates](../reports/NEXT-MILESTONE-CANDIDATES.md) is historical recommendation evidence, superseded by current PROJECT scope.

## Feature Landscape

### Table Stakes (Users Expect These)

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Deterministic BASIC bytes | Clients need identical filters for identical validated data | HIGH | Nonempty output scripts except those beginning OP_RETURN; nonempty spent-output scripts from undo; duplicate raw scripts collapse; genesis has no undo. Use pinned parameters/encoding. [K1, K2] |
| Filter hashes and chained headers | Clients need stable commitments across range segmentation | MEDIUM | Hash encoded bytes; chain each header to predecessor; zero genesis predecessor; pinned byte-order vectors. [K1, K3] |
| Separate explicit index/serving activation | Operator can index for local RPC without serving peers | MEDIUM | Both default off; serving requires BASIC; named/boolean forms and permission prerequisites tested. [K4] |
| Durable contiguous genesis-to-best index | History must survive restart and retain block identity | HIGH | Persist bytes/hash/header, active height mapping, safe progress and hash-addressable displaced records; no complete suffix-only index. [K3, K5] |
| Missing-history activation refusal | Pruned history cannot be recovered from current coins | HIGH | Check required bodies and non-genesis undo from resumable prefix/fork boundary; refuse gaps, no implicit repair/download. [K3, K4, K10] |
| Ordered bounded catch-up and ongoing indexing | Existing complete history and fresh startup both work | HIGH | Bounded batches/checkpoints and ordinary validated-connect integration; progress follows branch identity. [K5] |
| Reorg correctness and stale lookup | Height alone does not identify a chain | HIGH | Rewind to common ancestor; replacement headers use ancestor header; preserve previously indexed disconnected filters by hash. [K3, K9] |
| Index-owned protection in both prune paths | Manual/automatic deletion must not outrun durability | HIGH | Existing serialized owner and buffered locks protect bodies/undo until safely checkpointed; target stays soft. [K5, K6] |
| Retained filters after body/undo prune | Light-client filters remain useful on a pruned node | MEDIUM | Filter keys are independent of payload pruning; body status still Pruned. [K3, K10] |
| All three BIP157 request families | Clients need filters, headers and checkpoints | HIGH | Six codecs; exact Knots ranges, ancestry, absence and disconnect policies. [K7, K11] |
| BASIC getblockfilter RPC | Local clients need stable results/errors | MEDIUM | Default BASIC; hex filter/header; indexed stale lookup; disabled/unknown/not-connected/indexing/corruption distinctions. [K8, K9] |
| getindexinfo RPC | Operator needs enabled-index progress | LOW | Exact `basic block filter index` name, `synced`, `best_block_height`; exact-name selection and empty-object absence. [K8] |
| Distinct capability/readiness/availability evidence | Configured service does not prove full index catch-up | MEDIUM | Knots advertises from configuration and can serve complete indexed ranges during catch-up. [K4, K5, K7] |
| Bounded serving and redacted evidence | Filters add CPU/storage/queue demand | MEDIUM | Serve stored records; never regenerate for a request; existing byte/work/queue limits and successful-write-only evidence. [K7; BIP157 Node Operation] |
| Auditable parity and real prune/reopen proof | Core value depends on observable evidence | HIGH | Pinned vectors, real Fjall deletion/reopen, reorg/local peer/RPC fixtures and no-claim gates. [K9–K12] |

### Differentiators (Competitive Advantage)

These are project-specific strengths, not claims of features absent from competing nodes.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Actionable activation refusal | Operator knows why history prevents activation | LOW | Sanitized required height and body/undo category; preserve existing authority |
| One progress authority for prune and serving | Prevent misleading ready status after catch-up inputs are deleted | HIGH | Protection derives from safe durable prefix and current branch |
| Reproducible pinned parity | Contributors audit exact bytes and intentional differences | MEDIUM | First-party generator; document Fjall versus LevelDB/fltr and BASIC-only scope |
| Honest useful filters on pruned nodes | Light clients fetch commitments without a false full-body promise | MEDIUM | Explain matching old blocks may need another peer; no archive claim |

### Anti-Features (Commonly Requested, Often Problematic)

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Start at prune height and report complete | Convenient late enablement | Breaks genesis commitment and historical completeness | Refuse gaps; preserve any valid prefix |
| Rebuild from current UTXOs/wallet snapshots | Avoid obtaining old undo | Spent scripts are absent; snapshots are not authority | Historical validated bodies and spent-output evidence |
| Generate on peer request | Avoid index storage | Tiny requests repeatedly trigger large-block I/O/CPU | Persist during indexing; lookup during serving |
| Automatic destructive repair/reindex/download | Appears to self-heal | Expands data mutation authority and scope | Fail closed; explicit future recovery workflow |
| Delete filters with block/undo or count them in existing payload target | Apparent fixed disk bound | Loses historical capability and changes shipped accounting | Separate retained filter storage and growth disclosure |
| BIP37 bloom serving | Legacy client compatibility | Separate privacy/DoS scope explicitly excluded | BASIC only |
| V0 filters | Pinned Knots recognizes v0 | Second generator/index outside selected BIP158 scope | Explicit BASIC-only planned exclusion |
| scanblocks, filter-assisted wallet or new light-client wallet | Reuse index | Broadens wallet/RPC beyond serving | getblockfilter/getindexinfo and existing status/support |
| Peer filter import | Bypass missing history | Requires trust, validation and provenance policy | Locally generated filters/existing prefix only |
| Public defaults, archive scale, assumeutxo or production/funds claims | Broad rollout | Outside proven single-chainstate scope and claim gates | Explicit activation and hermetic evidence |

## Feature Dependencies

```text
BASIC bytes + hashes/headers
  -> durable records + contiguous branch-aware progress
     -> activation preflight + ordered catch-up + restart/reorg
        -> index-owned pruning protection -> real prune/reopen retention
     -> getblockfilter + getindexinfo
     -> six BIP157 codecs + complete-range lookup
        -> explicit peer/permission activation + bounded transport
           -> shared operator evidence + parity/no-claim proof

Existing v2.4 serialized prune owner/locks -> index/prune coordination
Existing peer authority/admission/queues -> bounded serving
Existing body availability -> independent block-versus-filter truth
```

### Dependency Notes

- BASIC uses type 0, P=19 and M=784931. SipHash-2-4 uses the first 16 little-endian block-hash bytes; map with the high half of the 128-bit product, sort hashed values, encode delta Golomb-Rice values after CompactSize element count. Empty filter bytes are `00`. Deduplicate raw scripts, not distinct scripts whose hash-to-range values collide. Filter hash is double-SHA256(encoded filter); header is double-SHA256(filter hash || previous header), with 32 zero predecessor bytes at genesis. These details need pinned vector proof. [K1, K2; BIP158 Construction]
- Historical spent-output scripts are mandatory: Knots CustomAppend reads undo for every non-genesis block. Current UTXOs cannot replace this. [K3]
- Design durable progress and lock movement together before pruning with indexing. Knots moves the lock before publishing best-block state and flushes filter files before committing metadata. [K3, K5]
- Ranges follow the requested stop hash's ancestry, not blindly the active height map. [K3, K7]
- Filter serving must not reuse the full-body limited-window/payload gate. Knots BlockRequestAllowed accepts active-chain blocks without payload presence; filter lookup then reads the independent index. [K7]
- Filters do not restore wallet eligibility or body availability. Existing wallet replacements remain body/coins based.
- Catch-up protection may prevent reaching the soft prune target; report this rather than bypass locks.
- Review v2.4 interrupted-prune authority when restoring index progress/protection. Support counters and generic unlink defaults are not proof of filter durability; use concrete Fjall evidence, without unrelated debt cleanup.

## Activation, Readiness and Prune Contract

### Recommended Activation Matrix

| State | Observable Outcome | Evidence |
|-------|--------------------|----------|
| Fresh chainstate, BASIC enabled | Index genesis and ordinary validated connects | K5 |
| Existing complete required history, no index | Ordered genesis catch-up; incomplete status until initial catch-up | K3, K5 |
| Valid durable prefix, required suffix/fork bodies and undo present | Validate identity and resume; earlier indexed payloads may already be pruned | K4, K10 |
| No index, required body or undo absent | Refuse activation; no skipped heights, invented completeness or implicit repair | K3, K4 |
| Disabled index overtaken by pruning | Refuse reactivation when required suffix is gone | K10 |
| Corrupt prefix/header/record or unsafe interrupted-prune authority | Fail closed with corruption/Repair diagnostic | K3, K5 and archived v2.4 audit |

Knots startup checks body availability from the saved index/common-ancestor boundary; append then requires undo. Recommend undo preflight too, with a documented earlier failure timing difference. Presence preflight still does not prove valid undo; validate every append. No startup refusal may erase a valid prefix or mutate source history.

### Recommended Service Semantics

Match source-confirmed Knots semantics: explicit general BASIC serving sets NODE_COMPACT_FILTERS (`1 << 6`) after valid index activation, without requiring initial catch-up completion. In prune mode it coexists with NODE_NETWORK_LIMITED and never implies full historical block service. Explicit blockfilters permission grants the compact-filter capability only to that peer. [K4, K7]

Serve an entirely available indexed request range during catch-up. If any required record/header/hash is missing, do not produce an invented empty filter, truncated successful response or completeness claim. Knots validates the whole range before emitting and does not call BlockUntilSyncedToCurrentChain in its P2P path. A transport failure after writes start can still cause partial delivery; record only achieved writes. [K7]

Knots getindexinfo `synced` is its initial synchronization flag, not instantaneous equality with every later moving chain tip. Preserve that baseline meaning; use existing Open Bitcoin-specific status to expose current lag, durable indexed identity and availability/failure evidence. Do not redefine baseline RPC fields or append custom ones. [K5, K8]

### Pruning Invariants

1. The same authority that plans/applies both manual and ordinary automatic prune protects all required body/undo inputs.
2. Persist filter bytes/hash/header before safe progress advances; release protection only after a recoverable durable checkpoint, never from optimistic counters.
3. Restore/validate index protection before startup pruning or serving; interrupted updates preserve protection or refuse.
4. Reorg rewinds progress/protection to the common ancestor and preserves indexed displaced filters by hash. Replacement headers use that ancestor's header.
5. Covered history may then be pruned; retained filters remain retrievable while ordinary body status stays Pruned/Unavailable.
6. An operator lock clear/set cannot weaken active index-owned protection. Reserve internal protection or independently enforce the durable watermark in the owner.
7. Disabling indexing preserves stored filters but stops its guarantee for future blocks; re-enable checks the required suffix again.

These Fjall recovery recommendations are MEDIUM confidence until proved. Knots uses separate fltr files/LevelDB metadata, buffered locks and safe rewind/flush constraints; identical storage transactions are not claimed. [K3, K5, K6]

## Explicit Option and RPC Parity

| Surface | Pinned Knots | v2.5 Contract |
|---------|--------------|---------------|
| blockfilterindex omitted/0 | Default disabled | Match; stored records need not be destroyed |
| Bare blockfilterindex, =1, =basic | BASIC enabled | Match all forms through config/help/CLI |
| Repeated named options | Collects valid types; pinned names basic and v0 | Idempotent repeated BASIC; V0 planned exclusion; test mixed boolean/name precedence |
| Unknown index name | Startup unknown-value error | Match truly unknown names; do not claim Knots considers v0 unknown |
| peerblockfilters | Default false; requires BASIC; general bit | Match; no implicit listener/index activation |
| Explicit blockfilters permission | Requires BASIC; per-peer bit even if general serving off | Activate existing permission with prerequisite; do not escalate global capability |
| getblockfilter(blockhash, filtertype?) | BASIC default; hex filter/header | Match BASIC shape, hash parsing and error ordering |
| Unknown filter type | -5, Unknown filtertype | Match genuinely unknown type; scoped V0 outcome documented |
| Index disabled | -1, Index is not enabled for filtertype basic | Match; precedes unknown-block lookup |
| Unknown block with index enabled | -5, Block not found | Match |
| Missing filter, never-connected block | -5, Filter not found. Block was not connected to active chain. | Match; header-only knowledge insufficient |
| Missing filter, initial catch-up incomplete | -1, Filter not found. Block filters are still in the process of being indexed. | Match; available older record can succeed during catch-up |
| Missing filter, index ready | -32603, Filter not found. This error is unexpected and indicates index corruption. | Match category; no empty success |
| Indexed formerly connected stale block | Successful hash lookup | Match; stale differs from never connected |
| getindexinfo(index_name?) | basic block filter index -> synced, best_block_height; exact-name filter; absent/unknown -> {} | Match; custom failure/lag evidence goes to existing status/support |

Sources: K4 activation/help; K8 RPC source; K9 stale/error tests. Numeric codes were confirmed in pinned rpc/protocol.h:36–42; verify CLI conversion paths during implementation. This is BASIC parity, not all Knots optional-index RPC parity.

## MVP Definition

### Launch With (v2.5)

- [ ] BASIC generation/hash/header parity against pinned vectors.
- [ ] Default-off activation and strict missing-body/undo refusal.
- [ ] Durable contiguous progress, bounded catch-up, ordinary connect and restart/reorg correctness.
- [ ] Index protection through both prune paths plus retained filters after real prune/reopen.
- [ ] Six codecs and all three bounded serving families with exact Knots validation/service/permission policy.
- [ ] BASIC RPCs, shared operator evidence and deterministic end-to-end parity/claim gates.

### Add After Validation (Later Scoped Work)

- [ ] Checkpoint-header caching only when measurements justify it; bound the cache.
- [ ] More filter storage diagnostics if independent retained-index growth needs them.
- [ ] Explicit historical recovery/import only under separate provenance and mutation requirements.

### Future Consideration

- [ ] V0, scanblocks and filter-assisted wallet workflows.
- [ ] Assumeutxo/dual chainstate, archive-scale serving and broader public/production claims after existing gates.
- [ ] BIP37 remains excluded; no automatic follow-up promise.

## Proposed Atomic User-Observable Requirements

Candidate statements for orchestration, not canonical IDs; each should get one roadmap owner.

| Candidate | Observable Requirement | Decisive Evidence |
|-----------|------------------------|-------------------|
| CFIL-01 | Obtain exact pinned BASIC bytes for validated blocks and historical spent scripts | Genesis/empty/duplicates/OP_RETURN/same-block spends and pinned vectors |
| CFIL-02 | Obtain correct filter hash and ancestry-dependent header | Zero predecessor, vectors and replacement branch |
| CFAC-01 | Explicitly enable BASIC with supported Knots forms; default disabled | Absent/0/bare/1/basic config/CLI matrix |
| CFAC-02 | Startup refuses missing required body or non-genesis undo | First enable after prune; disabled prefix overtaken; no skip/mutation |
| CFIX-01 | See ordered catch-up and ordinary validated-connect progress | Complete existing history/live connect; bounded work, honest incomplete state |
| CFIX-02 | Restart preserves valid records and resumes safe progress | Real Fjall reopen and interrupted append/checkpoint replay or refusal |
| CFIX-03 | Reorg exposes replacement headers and indexed stale lookup | Actual shallow fork, stale RPC/P2P ancestry and reopen |
| CFPR-01 | Manual and automatic prune protect incomplete index inputs | Stalled/failing indexing in both real paths; lock CRUD cannot bypass |
| CFPR-02 | Retrieve indexed filters after bodies/undo are pruned | RPC/all three P2P families after deletion/reopen; body still refused |
| CFNET-01 | Explicit serving/permission requires BASIC and advertises scoped capability | Disabled/general/per-peer-only version matrix including prune mode |
| CFNET-02 | Receive ordered cfilters for complete legal range of at most 1,000 blocks | Inclusive 1,000/1,001 boundary, bytes/branch and absence |
| CFNET-03 | Receive preceding header and hashes for at most 2,000 blocks | Genesis/nonzero predecessor and 2,000/2,001 boundary |
| CFNET-04 | Receive positive 1,000-height checkpoints through stop block | 0/999/1,000/1,001/2,000 and stale branch |
| CFNET-05 | Invalid requests disconnect per Knots; missing ranges yield no false success | Unknown type/hash, disallowed stale, start>stop, oversize and catch-up absence |
| CFNET-06 | Serving obeys work/byte/queue limits and records only successful writes | Queue pressure, local budgets, transport failure and bounded work |
| CFRP-01 | Authenticated BASIC getblockfilter matches results/errors | Disabled/type/hash/not-connected/indexing/corruption/stale |
| CFRP-02 | Authenticated getindexinfo matches BASIC shape/exact selection | Disabled/unknown/name selection/initial catch-up/current |
| CFOP-01 | Distinguish configured/catching-up/ready/history-gap/corruption in existing consumers | Shared RPC status, repo-local CLI/dashboard and redacted support |
| CFGR-01 | Audit pinned parity/deviations through deterministic checks and unchanged claim boundaries | Parity index/docs/breadcrumbs, native verifier, no-claim/local UAT |

Knots protocol maxima, disconnections and lookup absence behavior come from K7/K11. Local resource bounds beyond those maxima are deliberate engineering policy and need documentation.

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| BASIC byte/hash/header parity | HIGH | MEDIUM | P1 |
| Missing-history activation refusal | HIGH | MEDIUM | P1 |
| Durable branch/progress/prune coordination | HIGH | HIGH | P1 |
| Filter retention after real prune/reopen | HIGH | MEDIUM | P1 |
| Three bounded serving families | HIGH | HIGH | P1 |
| BASIC RPC/status/support and parity proof | HIGH | MEDIUM | P1 |
| Checkpoint cache optimizations | MEDIUM | LOW | P2 |
| V0, scanblocks and wallet acceleration | MEDIUM | HIGH | P3 |
| Historical import/recovery | MEDIUM | HIGH | P3 |

P1 is required for v2.5; P2 needs measured benefit; P3 stays later scope.

## Baseline Feature Comparison

| Feature | BIP157/158 | Pinned Knots | Open Bitcoin |
|---------|------------|--------------|--------------|
| Production | Deterministic indexed filters | BASIC plus local V0 generator | BASIC-only first-party behavior |
| Pruned history | Allows prune after filter persistence | Prunable index/progress locks; required-gap refusal | Existing Fjall owner plus durable index protection |
| Invalid requests | Legal ranges/nonresponse expectations | Disconnects unsupported/unadvertised type, invalid hash/range | Match Knots beyond BIP minimum |
| Initial catch-up | Startup indexing | Configured bit; available ranges before full catch-up | Match with separate progress/availability evidence |
| Stale chain | Stop-hash ancestry defines branch | Displaced records by hash; stale eligibility | Preserve indexed stale records and branch-correct responses |
| Store | No mandated database layout | LevelDB metadata plus fltr files | Existing Fjall; document layout/durability difference |

## Open Decisions and Phase Research Flags

1. **Crash/lock ordering:** Determine smallest Fjall batch/checkpoint contract and internal protection ownership. Prove no prune window between append/progress/protection; no assumption that coins, index and prune intent share one atomic transaction.
2. **Required suffix boundary:** Match conservative Knots common-ancestor/checkpoint-body preflight, then add undo checks. Define prefix integrity/schema and replay policy.
3. **V0 outcome:** Explicit BASIC-only exclusion is selected. Decide known-but-unenabled RPC versus unsupported startup wording; document difference from actual Knots names.
4. **Mixed/repeated options:** Exhaustively test 0/1/basic ordering and current config/CLI conversion before claiming parser parity.
5. **Checkpoint bounds:** getcfcheckpt uses all positive 1,000-height checkpoints; its range has no 1,000/2,000 cap. Derive message/queue/work policy from chain height/transport bounds, documenting local refusal/backpressure differences.
6. **Deep reorg beyond retained data:** Refuse/fail safely if body/undo is missing; no hidden redownload or unsupported deep-reorg claim.
7. **Fixture quality:** Use continuous validated local chains for ancestry/spend evidence; sparse codec-valid prune fixtures alone cannot prove those behaviors. Default gates remain hermetic.

Service timing is resolved: match configuration advertisement and available-range serving during catch-up; do not require full sync before the bit or an available response. Active index-owned protection must not be weakened by operator lock operations.

## Sources

All K links point to the verified local pin. Source behavior is HIGH confidence; proposed Fjall integration/recovery is MEDIUM until implementation evidence.

- **BIP157:** [Official specification](https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki), New Messages/Node Operation; [rendered specification](https://bips.dev/157/), accessed 2026-10-03.
- **BIP158:** [Official specification](https://github.com/bitcoin/bips/blob/master/bip-0158.mediawiki), GCS/BASIC construction/signaling/test vectors; [rendered specification](https://bips.dev/158/), accessed 2026-10-03.
- **K1:** [blockfilter.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/blockfilter.cpp#L21-L103), types/hash/encoding; [elements/headers](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/blockfilter.cpp#L190-L282).
- **K2:** [BASIC constants](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/blockfilter.h#L89-L90).
- **K3:** [Filter-index storage](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/blockfilterindex.cpp#L19-L43); CustomInit/Commit lines 116–174; [append/rewind](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/blockfilterindex.cpp#L268-L355); lookup/range lines 358–507.
- **K4:** [Option parsing/services](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/init.cpp#L1075-L1105); help lines 549–552/584; permission prerequisite lines 2358–2362; [history startup gate](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/init.cpp#L2456-L2496); defaults src/index/blockfilterindex.h:17 and src/net_processing.h:35.
- **K5:** [BaseIndex init/catch-up](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/base.cpp#L79-L217); rewind 245–263; BlockUntilSynced 367–390; [summary/protection publication](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/base.cpp#L415-L447).
- **K6:** [Prune locks/manual checks](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/node/blockstorage.cpp#L317-L359); automatic gate line 445; [prunable filter index](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/blockfilterindex.h#L48).
- **K7:** [Peer filter requests](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/net_processing.cpp#L3156-L3320); maxima 153–155; BlockRequestAllowed 1865–1871; per-peer permission 1551–1553; bit src/protocol.h:321–323.
- **K8:** [getblockfilter](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/blockchain.cpp#L3317-L3392); [getindexinfo shape/selection](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/node.cpp#L409-L462); [numeric RPC errors](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/protocol.h#L36-L42).
- **K9:** [RPC active/stale/errors test](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/test/functional/rpc_getblockfilter.py#L22-L62).
- **K10:** [Prune/index retention/resume/refusal test](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/test/functional/feature_index_prune.py#L66-L154).
- **K11:** [P2P ancestry/hash/disconnect test](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/test/functional/p2p_blockfilters.py#L140-L273).
- **K12:** [Pinned byte vectors](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/test/data/blockfilters.json); [generator tests](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/test/blockfilter_tests.cpp); [index tests](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/test/blockfilter_index_tests.cpp).

*Feature research for: v2.5 Prune-Aware Compact-Filter Serving*
*Researched: 2026-10-03*
