# Project Research Summary

**Project:** Open Bitcoin — v2.5 Prune-Aware Compact-Filter Serving (BIP157/158)
**Domain:** First-party BASIC compact-filter indexing and explicitly enabled lightweight-client serving on a single prunable chainstate
**Researched:** 2026-10-03
**Confidence:** HIGH for pinned baseline and protocol; MEDIUM for proposed implementation and recovery design

## Executive Summary

v2.5 extends the shipped v2.4 prunable node with locally generated BASIC filters, durable commitments and bounded BIP157 serving. The recommended design generates filters from validated blocks and historical spent-output scripts, retains records independently of block/undo deletion, and answers requests through branch-aware indexed lookup. BASIC/type 0 is the selected scope; Knots V0/type 2 is an explicit exclusion. Match the pinned Knots enablement contract: after valid explicit activation, advertise compact-filter capability before initial catch-up completes and serve any complete indexed range during catch-up. Capability, initial synchronization, current lag and per-request availability remain separate facts. [FEATURES.md](FEATURES.md), [ARCHITECTURE.md](ARCHITECTURE.md)

Keep the existing Rust workspace, first-party hashing/codecs, Fjall database and serialized chainstate/prune owner; add no production dependency or workspace crate. Establish exact byte/header parity first, then durable branch records and recovery invariants, then concrete prune coordination, ordinary startup/catch-up/connect/reorg consumers, authenticated RPC, peer serving and integrated evidence. Fresh or lagging index activation refuses when required body/undo history is gone; neither current coins nor leftover snapshots can reconstruct every historical spent script. No suffix-only complete index, implicit repair or redownload is acceptable. [STACK.md](STACK.md), [PITFALLS.md](PITFALLS.md)

The critical implementation risks are a startup deletion window, a filter restart cursor ahead of durable chainstate, weakening internal protection through operator lock operations, and mixing commitments from different branches. Restore and validate protection before initialization resumes a prune intent; publish safe resume progress only behind the durable coins/chain-metadata checkpoint; reserve internal protection under the actual deletion authority; preserve displaced hash-addressable records and stop-hash ancestry. Completion requires a continuous validated local chain with real spends/forks, actual Fjall block/undo deletion, production reopen and decoded local wire/RPC responses. Sparse codec-valid or memory-only fixtures are supplementary evidence. Existing v2.4 advisories remain explicit limits; public defaults, public-network CI, archive-scale serving, assumeutxo, BIP37 and production/funds claims stay deferred. [ARCHITECTURE.md](ARCHITECTURE.md), [PITFALLS.md](PITFALLS.md), [v2.4 audit](../milestones/v2.4-MILESTONE-AUDIT.md)

## Key Findings

### Recommended Stack

The existing stack is sufficient. Preserve Rust `1.94.1`/edition 2024, Fjall `3.1.4` with existing feature policy, Bazel `8.6.0`/`rules_rust 0.69.0`, Bun `1.3.9` and the pinned Knots `29.3.knots20260210` reference at `a9aee730466ac67d35a3c03ee24676be5e045878`. No dependency upgrades or installation step is needed. Same-database Fjall atomic `SyncAll` batches support filter records and metadata, but do not automatically make existing coins, index and prune effects one transaction. [STACK.md](STACK.md)

**Core technologies:**

- **Owned byte SipHash-2-4 and SHA256d:** extend the existing Wtxid-only SipHash API for arbitrary scripts while preserving BIP152 outputs; reuse SHA256d for commitments.
- **Owned CompactSize and small bit/Golomb–Rice codec:** canonical bounded encoding without importing a Rust Bitcoin library or generic bit-stream dependency.
- **Rust standard collections and `u128`:** deduplicate raw scripts before hashing, preserve mapped collisions, and implement exact multiply-high range mapping.
- **Existing Fjall database:** persist versioned block-hash records, active projection and fenced resume state; retain filters separately from the shipped soft block/undo retention target.
- **Existing runtime, transport and operator consumers:** reuse the serialized owner, periodic maintenance, authenticated RPC, peer queues, CLI/dashboard and sanitized support projection.

BASIC uses P=19, M=784931, the first 16 raw little-endian block-hash bytes for SipHash keys, sorted delta coding and a CompactSize element count. Empty encoding is `00`. Include nonempty output scripts except outputs beginning OP_RETURN, plus nonempty historical spent scripts; do not apply the output exclusion to spent scripts. Hash encoded bytes and chain headers from the predecessor, using a zero predecessor at genesis. Independent pinned vectors must settle encoding and byte order before persistence. [STACK.md](STACK.md), [PITFALLS.md](PITFALLS.md)

### Expected Features

The 22 canonical requirements are in [REQUIREMENTS.md](../REQUIREMENTS.md); detailed feature research and exact baseline outcomes are in [FEATURES.md](FEATURES.md).

**Must have:**

- Exact BASIC bytes, filter hashes and ancestry-dependent headers from validated block/undo inputs.
- Default-off indexing and independent serving activation, including Knots boolean/named forms and explicit peer permission prerequisites.
- Contiguous genesis-to-best indexing, bounded ordinary catch-up, live connect, restart and reorg handling; refuse required missing history.
- Immutable hash-addressable records, active branch projection and a resume cursor fenced by durable chainstate.
- Startup protection before resumed pruning, reserved index-owned protection in both manual and automatic deletion, and retained filters after body/undo prune.
- All six BIP157 codecs and three serving families: inclusive maxima of 1,000 filters and 2,000 hashes; checkpoints at positive multiples of 1,000, excluding genesis.
- Complete-range availability checks before emission, Knots invalid-request disconnects, bounded reads/bytes/queues and successful-write-only completion evidence.
- Authenticated BASIC `getblockfilter` and `getindexinfo` results/errors, plus shared operator evidence with distinct capability, progress, missing-history and corruption facts.
- Continuous validated-chain proof across real Fjall prune/reopen, fork identity and actual local transport/RPC, with parity and claim guards.

**Should have:**

- Actionable redacted refusal showing the required history boundary and body/undo category.
- One authoritative progress/protection projection for prune, serving and operator consumers.
- Separate retained-index storage disclosure and clear explanation that an old matching block may require another peer.

**Defer:**

V0 filters, `scanblocks`, filter-assisted wallet workflows, peer filter import, historical recovery, assumeutxo/dual chainstate, archive-scale serving and broader production/public defaults. BIP37 stays excluded. Checkpoint caching is optional only after measurements justify a bounded cache. Filters do not restore wallet eligibility or body availability. [FEATURES.md](FEATURES.md)

### Architecture Approach

Use immutable `(BASIC, block hash)` records and a separate active height projection under the existing serialized owner. Maintain processed progress, durable records, safely resumable cursor, branch identity and initial-sync state distinctly. Catch-up and live connects share one ordered append policy; reorg rewinds to a verified common ancestor while retaining displaced records. Request ranges follow stop-hash ancestry rather than the current active height map. [ARCHITECTURE.md](ARCHITECTURE.md)

**Major components:**

1. **Primitives/codec/consensus core:** typed identities, bounded binary formats and pure generation from validated spent-script facts. Codec must not import consensus hashing; consensus must not import chainstate merely for undo.
2. **Pure index transition policy:** predecessor/branch checks, contiguous append, rewind and safe protection facts.
3. **Fjall filter adapter:** versioned records, integrity validation, active projection and explicit durable checkpoint operations in the existing database.
4. **Managed runtime owner:** serialize startup recovery, bounded catch-up, accepted connect/reorg transitions, checkpoint publication and prune/lock authority.
5. **Peer/RPC adapters:** capture bounded complete responses, write outside the authority lock and acknowledge only achieved local writes; share one durable index view.
6. **Existing operator projection:** preserve baseline RPC shapes while exposing Open Bitcoin-specific current lag, durable identity and sanitized failures through existing status/support.

Index-enabled offline startup must select durable storage and drive periodic catch-up even without peer blocks. Protection must be valid before `initialize` invokes `resume_prune_intent`. Current connect/reorg seams can change accepted in-memory state before a persistence error returns; success-only callbacks cannot define index authority. Actual callers and failure transitions must be covered. [ARCHITECTURE.md](ARCHITECTURE.md)

### Critical Pitfalls

1. **Compatible-looking but wrong filters:** self-roundtrip tests miss shared encoding errors, while current coins omit spent scripts. Use independent Knots vectors and validated undo/spend fixtures; missing undo cannot become an empty input list.
2. **Unsafe recovery or deletion progress:** manager-level protection arrives after startup deletion, or processed progress advances protection before safe durability. Validate protection before prune resume; distinguish durable rows from chainstate-fenced resume authority; faults retain protection or refuse.
3. **Operator weakening of internal protection:** ordinary named-lock CRUD can overwrite or clear a reused lock. Enforce ownership under authoritative CRUD, deletion application and recovery; an explicit disable transition must invalidate in-flight work before releasing protection.
4. **False completeness or branch mixing:** suffix bootstrap, height-only records and empty-filter fallbacks hide missing history or corrupt commitments. Preserve a verified prefix, retained stale hash records, predecessor checks and distinct absence/backend/corruption outcomes.
5. **Disconnected or unbounded serving:** codecs/helpers can pass without ordinary callers, and protocol counts do not bound total resource use. Prove actual daemon/maintenance/inbound/outbound/RPC consumers, full-range prevalidation and bytes/work/queue limits with transport failure evidence.

These synthesize pitfalls 1–12; detailed warning signs and fault boundaries are in [PITFALLS.md](PITFALLS.md).

## Implications for Roadmap

The finalized [ROADMAP.md](../ROADMAP.md) assigns all 22 requirements to nine phases, 154–162, continuing from shipped v2.4. The summaries below follow its canonical names and ownership. Startup protection and the durable cursor fence are foundation deliverables; Phase 162 owns complete post-prune client proof after RPC and all peer families exist.

### Phase 154: BASIC Generation and Commitment Parity

**Rationale:** Every durable record and client commitment depends on exact independent byte parity.

**Delivers:** Typed identities, arbitrary-byte SipHash, bounded bit/Golomb codec, raw-script input contract, deterministic BASIC generation/hash/header and pinned vectors. Include genesis, empty scripts, duplicates, mapped collisions, OP_RETURN, same-block spends and SipHash tail boundaries.

**Owns:** CFIL-01, CFIL-02. Codec allocation bounds underpin later CFNET requirements.

**Avoids:** Wrong byte order/range mapping, shared roundtrip mistakes and incomplete spent-script inputs.

### Phase 155: Recoverable Index and Pre-Prune Startup Protection

**Rationale:** Safe startup and checkpoint contracts must exist before catch-up or deletion consumes the index.

**Delivers:** Versioned Fjall records, active projection, branch-aware lookup, integrity/error taxonomy, processed-versus-resume state and chainstate checkpoint fence. Establish early protection before initialization resumes a pending prune intent, including reserved internal identity. Concrete fault/reopen fixtures prove conservative protection and reconciliation to durable coins/chain metadata.

**Owns:** CFIX-02, CFIX-04, CFPR-03. Establishes the ownership foundation enforced across mutation/deletion paths in Phase 156.

**Avoids:** Cursor ahead of durable chainstate, deletion before protection restoration, metadata reset on read failure and height-only records.

### Phase 156: Index-Owned Manual and Automatic Prune Coordination

**Rationale:** Index activation must follow enforceable protection through actual deletion and lock owners.

**Delivers:** Apply-time protection through manual and ordinary automatic pruning, safe checkpoint/protection ordering, authoritative lock CRUD refusal for reserved protection and explicit disable/re-enable ownership transitions. Reload effective protection and invalidate stale automatic measurements. Concrete Fjall faults retain required body/undo rather than release unsafe inputs.

**Owns:** CFPR-01. Carries Phase 155's startup gate and durable fence into concrete mutation/deletion paths; complete post-prune client retrieval belongs to Phase 162.

**Avoids:** Planner-only checks, cached measurements ignoring changed protection, no-op sink success and operator weakening.

### Phase 157: Safe Activation and Scheduled Index Catch-Up

**Rationale:** Recoverable storage and concrete protection precede ordinary runtime activation.

**Delivers:** Default-off BASIC forms, durable offline activation, required body/undo preflight, bounded scheduled catch-up and ordinary accepted-connect handoff. Preserve valid records on refusal. Actual startup and maintenance advance without another peer message; accepted-state/persistence-error seams cannot disappear behind success-only callbacks.

**Owns:** CFAC-01, CFAC-02, CFIX-01.

**Avoids:** Suffix-only activation, missing ordinary consumers, incomplete spent-script inputs, unbounded owner hold and phantom complete progress.

### Phase 158: Validated Reorg and Retained Branch Identity

**Rationale:** Replacement and retained stale commitments must follow validated ancestry before lookup surfaces expose them.

**Delivers:** Common-ancestor rewind, branch-correct replacement headers, immutable displaced hash records and reconciled active projection/protection across real forks and Fjall reopen. Missing deep-reorg inputs refuse without current-coins substitution or hidden redownload.

**Owns:** CFIX-03.

**Avoids:** Height-only identity, stale-record loss, wrong predecessor headers, preview-only branch publication and mixed-branch responses.

### Phase 159: Authenticated BASIC Filter and Index RPCs

**Rationale:** Authenticated clients need pinned lookup behavior from the shared durable branch authority.

**Delivers:** BASIC `getblockfilter` result/error ordering and codes, retained stale/pruned lookup and available records during catch-up. Preserve exact `getindexinfo` name/shape/selection and initial-synchronization meaning; custom current lag/failure evidence stays in existing Open Bitcoin status/support.

**Owns:** CFRP-01, CFRP-02.

**Avoids:** Detached RPC indexes, empty success on backend failure, known V0 misclassified as unknown to Knots and synced redefined as instantaneous tip equality.

### Phase 160: Explicit Bounded Peer Filter Serving

**Rationale:** Peers need complete durable branch lookup and proven prune/recovery behavior before advertised serving becomes real.

**Delivers:** Six wire codecs, all three request families, exact Knots invalid-request/stale-stop policy, complete-range absence behavior, scoped service bits/permissions, bytes/work/queue budgets and inbound/outbound write acknowledgements. Observe capability before initial catch-up completes and successful complete older ranges during catch-up. A gap produces no invented partial success; limited-body depth gates do not restrict retained filters.

**Owns:** CFNET-01, CFNET-02, CFNET-03, CFNET-04, CFNET-05, CFNET-06.

**Avoids:** Synced-gated advertisement, global permission escalation, body/filter availability conflation, mixed branches, allocation amplification and enqueue credited as serving.

### Phase 161: Shared Operator Index and Retention Evidence

**Rationale:** Existing operator consumers must report the same capability, progress, availability and failures as the actual runtime.

**Delivers:** Shared sanitized status RPC, repo-local CLI, terminal dashboard and support projection. Distinguish configured capability, initial catch-up, current lag, safe durable identity and missing-history/backend failures. Show retained-index growth separately from the soft block/undo prune target; useful filters do not restore bodies or wallet eligibility.

**Owns:** CFOP-01.

**Avoids:** Enabled labeled complete, inconsistent consumer facts, sensitive support output and total-disk promises.

### Phase 162: Real Prune Retention and Integrated Parity Proof

**Rationale:** Complete post-prune client evidence depends on authenticated RPC and all three peer families already existing.

**Delivers:** One hermetic continuous validated local-chain scenario covering historical spends, actual startup/scheduled catch-up/connect, shallow reorg/stale lookup, real manual/automatic Fjall paired prune, production reopen, RPC and decoded inbound/outbound wire families. Prove retained filters/headers/checkpoints after deletion while body/wallet restrictions remain. Concrete durable-coins faults cover pending intent, cursor/checkpoint/protection/delete and partial-write boundaries. Add pinned parity deviations/breadcrumbs, repo-local UAT commands and native release/claim checks.

**Owns:** CFPR-02, CFGR-01, CFGR-02. Earlier phases own their behavior tests; this phase owns complete post-prune client retrieval and cross-component proof.

**Avoids:** Helper-only success, sparse/memory fixture overclaims, incomplete client-family proof, production promises and removal of historical claim checks.

### Phase Ordering Rationale

- Exact bytes precede recoverable records and early protection; concrete manual/automatic protection in Phase 156 precedes activation/catch-up in 157.
- Phase 158 establishes validated branch replacement and retained stale identity before RPC in 159 and peer serving in 160.
- Wire codec preparation can overlap earlier work, but serving activation depends on branch-complete durable lookup and prune/recovery proof.
- Continuous validated-chain fixtures begin in Phase 154 and expand through ordinary lifecycle in 157–158. Phase 162 proves complete post-prune RPC and all peer families after their implementations exist; fixture feasibility cannot wait until closeout.
- Existing v2.4 behavior remains the foundation. Accepted interrupted-prune Repair refusal, generic-sink advisory and counter undercount do not become index progress authority or unrelated cleanup obligations.

### Research Flags

**Needs deeper phase research:**

- **155 — recovery/checkpoint protocol:** trace early startup, recoverable coins/metadata anchors, cross-keyspace atomicity, schema compatibility and conservative protection publication.
- **156 — prune/protection ownership:** settle reserved CRUD, apply-time manual/automatic checks, measurement invalidation and safe disable/re-enable release ordering.
- **157 — activation/lifecycle:** settle validated undo access, accepted-state/persistence-error tracking, required prefix/suffix boundary, mixed/repeated options and measured offline scheduler budgets.
- **158 — reorg/recovery:** settle common-ancestor rewind, retained stale identity under real faults and deep-reorg missing-input refusal.
- **159 — RPC parity:** settle known-but-excluded V0 outcomes, exact error ordering and initial-sync projection through the shared context.
- **160 — peer policy/resources:** settle implicit/all permissions, stale eligibility, existing-peer failure/reorg behavior, checkpoint byte/work/queue budgets and transport cleanup.

**Established patterns; skip standalone research unless new evidence changes scope:**

- **154 — generation:** pinned Knots/BIP vectors and traced owned APIs define the algorithm. Final placement must respect the crate graph.
- **161 — operator projection:** reuse existing CLI/dashboard/status/support and sanitization patterns while preserving baseline RPC meanings.
- **162 — parity/integrated evidence:** reuse native verification, breadcrumbs, claim guards and repo-local UAT conventions. Continuous-chain, real durable-coins/Fjall, all-family client and transport-fault proof remains mandatory engineering work.

## Confidence Assessment

| Area | Confidence | Notes |
| --- | --- | --- |
| Stack | HIGH baseline / MEDIUM integration | Existing pins and Fjall 3.1.4 atomic-batch APIs are confirmed; final schema/commit protocol is proposed. |
| Features | HIGH baseline / MEDIUM recovery policy | Pinned options, RPC errors, message limits, service timing and stale behavior are source-confirmed; internal protection and early undo refusal need implementation/parity documentation. |
| Architecture | HIGH traced seams / MEDIUM design | Actual startup/connect/reorg/prune/transport consumers are traced; safe cursor fence, scheduling and cross-operation protocol need concrete proof. |
| Pitfalls | HIGH hazards / MEDIUM mitigations | Encoding/history/startup/branch hazards follow source and audit evidence; crash behavior cannot be established by research alone. |

**Overall confidence:** MEDIUM for implementation readiness; HIGH for the scoped behavioral baseline. No shipping, hardware-resilience or production-capacity conclusion follows from this research.

### Gaps to Address

- **Schema/reopen compatibility:** decide additive namespace/envelope versus existing store schema 2 bump and compatibility handling before writes.
- **Recovery protocol:** define exactly which durable chain checkpoint fences the resume cursor, how extra branch records/projections reconcile, and when replay versus explicit refusal is allowed.
- **Startup and ownership:** specify enabled-on-open pre-resume contract, conservative protection at each fault boundary, reserved CRUD enforcement and safe disable/re-enable.
- **Activation boundaries/options:** settle common-ancestor/checkpoint input boundary, undo validation, mixed 0/1/basic ordering and deliberate V0 startup/RPC outcome.
- **Resource governance:** choose measured catch-up batches and response byte/work/queue limits; checkpoint responses scale with height and do not inherit the 1,000/2,000 range caps.
- **Deep reorg:** required missing body/undo must fail safely; retained filters alone cannot replay a branch or justify hidden redownload.
- **Evidence feasibility:** build continuous validated spend/fork fixtures and actual durable-coins reopen faults early. Distinguish verified local writes from remote receipt and concrete Fjall from memory fixtures.
- **Known v2.4 limits:** preserve permitted finish-or-Repair refusal and support undercount; do not infer index durability from unlink summaries or generic sink defaults.

## Sources

### Primary: HIGH Baseline Confidence

- [STACK.md](STACK.md), [FEATURES.md](FEATURES.md), [ARCHITECTURE.md](ARCHITECTURE.md), [PITFALLS.md](PITFALLS.md) — detailed inspected-source citations and proposed integration decisions, researched 2026-10-03.
- [PROJECT.md](../PROJECT.md), [v2.5 requirements](../REQUIREMENTS.md), [v2.4 requirements](../milestones/v2.4-REQUIREMENTS.md), [v2.4 audit](../milestones/v2.4-MILESTONE-AUDIT.md) — selected v2.5 scope, shipped foundation, accepted advisories and evidence limits.
- [Pinned Knots blockfilter.cpp](../../packages/bitcoin-knots/src/blockfilter.cpp), [Golomb–Rice](../../packages/bitcoin-knots/src/util/golombrice.h), [byte vectors](../../packages/bitcoin-knots/src/test/data/blockfilters.json), [generator tests](../../packages/bitcoin-knots/src/test/blockfilter_tests.cpp) — BASIC inputs, encoding and commitments at pin `a9aee730466ac67d35a3c03ee24676be5e045878`.
- [Knots filter index](../../packages/bitcoin-knots/src/index/blockfilterindex.cpp), [base index](../../packages/bitcoin-knots/src/index/base.cpp), [startup](../../packages/bitcoin-knots/src/init.cpp), [blockstorage](../../packages/bitcoin-knots/src/node/blockstorage.cpp) — retention, progress, chainstate fence, activation/refusal and locks.
- [Knots peer policy](../../packages/bitcoin-knots/src/net_processing.cpp), [getblockfilter](../../packages/bitcoin-knots/src/rpc/blockchain.cpp), [getindexinfo](../../packages/bitcoin-knots/src/rpc/node.cpp) — exact serving/RPC behavior.
- [Knots P2P tests](../../packages/bitcoin-knots/test/functional/p2p_blockfilters.py), [RPC tests](../../packages/bitcoin-knots/test/functional/rpc_getblockfilter.py), [prune/index tests](../../packages/bitcoin-knots/test/functional/feature_index_prune.py) — branch, absence, range, restart and missing-history oracle.
- [Official BIP157](https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki), [Official BIP158](https://github.com/bitcoin/bips/blob/master/bip-0158.mediawiki) — normative wire and generation contracts, checked by source researchers against pinned Knots.
- [Fjall 3.1.4 Database](https://docs.rs/fjall/3.1.4/fjall/struct.Database.html), [OwnedWriteBatch](https://docs.rs/fjall/3.1.4/fjall/struct.OwnedWriteBatch.html) — same-database atomic batches and explicit durability APIs.
- [Durable runtime open](../../packages/open-bitcoin-node/src/sync/open_runtime.rs), [flush initialization](../../packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs), [prune authority](../../packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs), [automatic prune](../../packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune.rs), [concrete Fjall prune](../../packages/open-bitcoin-node/src/storage/fjall_store/prune.rs) — existing startup/deletion authority.
- [Chain lifecycle](../../packages/open-bitcoin-node/src/chainstate.rs), [mempool lifecycle](../../packages/open-bitcoin-node/src/network/mempool_lifecycle.rs), [sync session](../../packages/open-bitcoin-node/src/sync/session.rs), [inbound wire](../../packages/open-bitcoin-rpc/src/context/inbound_wire.rs) — validated inputs, actual consumers and completion seams.
- [AGENTS.md](../../AGENTS.md), [Bright Builds sidecar](../../AGENTS.bright-builds.md), [overrides](../../standards-overrides.md), [architecture standards](../../standards/core/architecture.md), [verification standards](../../standards/core/verification.md) — material workflow, functional-core boundaries and native verification requirements.

### Proposed Design: MEDIUM Confidence

Same-database record layout, early startup recovery gate, fenced cursor/protection protocol, reserved internal ownership, batch/resource budgets and fault recovery are recommendations synthesized from the primary sources. They require phase design and concrete evidence; no identical Knots storage transaction or hardware-failure guarantee is claimed.

### Tertiary

No low-confidence market, competitor or ecosystem claim is used. Context7 was unavailable to the stack researcher; inspected pinned source and version-specific official documentation supplied evidence.

***
*Research completed: 2026-10-03*
*Ready for requirements and roadmap: yes; proposed invariants require phase-specific implementation proof*
