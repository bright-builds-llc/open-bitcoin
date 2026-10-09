---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: 2026-10-08T02:29:17.993Z
---

# Phase 158: Validated Reorg and Retained Branch Identity - Research

**Researched:** 2026-10-07 CDT / 2026-10-08 UTC
**Domain:** Validated branch transitions, immutable BASIC indexing and Fjall durability
**Confidence:** HIGH for source findings; MEDIUM for proposed integration and new work limits

<user-constraints>
## User Constraints (from CONTEXT.md)

The following decisions/discretion/deferred text is copied verbatim from the phase context. [VERIFIED: 158-CONTEXT.md]

### Locked Decisions


### Validated branch transition

- **D-01:** Extend the existing serialized staged reorg acceptance path and BASIC ordered owner. A reorg must autonomously restore branch-correct ordered work; requiring reopen or manual re-enable to resume is insufficient.
- **D-02:** Distinguish preview, accepted replacement and durable fenced replacement. Preview is not acceptance, and a success-only callback cannot capture accepted state followed by persistence failure. Preserve current mempool/chainstate ordering while preventing stale prepared index work from publishing.
- **D-03:** Identify the common ancestor by ancestry/hash, including equal-height replacements and catch-up behind the fork. Rewind active progress to the verified common prefix and derive replacement headers from that ancestor; never append from the displaced tip or select a record by height alone.
- **D-04:** Advance generation/branch identity through a trusted transition and invalidate stale prepared turns. Keep one ordered owner and bounded scheduled turns; avoid an index-owned duplicate full-history cache or a reopen/reseed workaround on each reorg.

### Retained identity and durability

- **D-05:** Keep immutable filter records keyed by block hash. Replace only the active projection/checkpoint suffix. Previously indexed displaced records and their original branch headers remain addressable after reorg, ordinary flush, actual drop and Fjall reopen.
- **D-06:** Index progress must not commit ahead of recoverable coins/chain metadata. Extend the proven validated-lineage/fence capability to accepted replacement branches; preserve accepted-unflushed behavior rather than forcing a coins flush on every connect/reorg solely to avoid crash loss.
- **D-07:** Persistence failures retain conservative protection and truthful progress. Interrupted projection/rewind writes recover a verified prefix or fail closed, without phantom advancement or destruction of immutable displaced history.

### Required inputs and refusal

- **D-08:** Validate all genuinely required retained bodies and body-bound historical undo for the reorg/index transition before preview or destructive effects. Missing body and missing undo must refuse explicitly, preserve the valid prefix and conservative protection, and leave no partial successful branch transition.
- **D-09:** Use genuine validated staged facts for replacement work, including historical and same-block spends. Never reconstruct from current coins, borrow another branch's undo/filter, skip a height, invent empty filters, silently redownload or repair. Already valid immutable records need not regenerate solely because their source payloads were subsequently pruned.
- **D-10:** Preserve index-owned reserved prune locks, generation checks and interrupted-prune startup ordering. Operator lock changes cannot weaken required reorg/index protection.

### Evidence and scope

- **D-11:** Extend continuous validated spend/fork fixtures on concrete Fjall and durable coins. Cover common-ancestor rewind, equal-height replacement, longer replacement, catch-up lag, real reopen with active/hash lookup, missing body/undo and stale-work/persistence failures. Sparse codec-valid, memory-only and source-string checks are supplementary evidence.
- **D-12:** Add required Rust source breadcrumbs and parity documentation, deterministic claim checks wired into native verification, and contributor README/UAT updates. Run the default `bash scripts/verify.sh`, independent source review, security mitigation review and formal lifecycle validation before final commit/push. UAT uses copy-pasteable repo-local Cargo/Bazel commands.

### Agent Discretion

Exact transition types, finite work limits after measurement, fault seams and plan decomposition belong to research/planning. Prefer the smallest robust change to existing core/shell seams. No new production dependency or crate. This is a headless lifecycle phase; no frontend contract is needed.

### Deferred Ideas (OUT OF SCOPE)


Filter/index RPC (159), peer serving/service bits (160), broad dashboard/support projections (161), full post-prune client integration (162). No pending todos matched. V0/type 2, BIP37, archive serving, automatic repair/import/download, public defaults and production/funds claims remain excluded.
</user-constraints>

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| CFIX-03 | A client gets branch-correct replacement filters and headers after a validated reorg and can retrieve already-indexed displaced blocks by hash; missing deep-reorg inputs cause explicit refusal rather than invented history. | Pure replacement reducer, sealed accepted transition, bounded projection replacement, branch-aware flush lineage, real Fjall failure/reopen matrix. [VERIFIED: .planning/REQUIREMENTS.md:25] |
</phase-requirements>

## Summary

Extend the serialized staged reorg transaction and existing ordered BASIC owner. The present implementation deliberately invalidates the owner and flush lineage during both preview and final absorption; ordinary replay subsequently sees stale ownership. The bounded append adapter also rejects a different hash at an existing active height. These are the two integration gaps, rather than deficiencies in BASIC encoding. [VERIFIED: packages/open-bitcoin-node/src/chainstate.rs:407-425; storage/fjall_store/filters/append.rs:135-158]

Use a pure branch replacement reducer, backed by a same-store private accepted-transition capability. Carry the common ancestor and separately verified **indexed** common prefix; preserve hash rows; shrink safe progress and strengthen protection before incompatible projection effects; replace active projections one bounded contiguous turn at a time. Preserve genuine validated lineage across accepted replacement so the normal coins/metadata owner can later seal the new durable fence. Preview, accepted dirty state, and achieved durable state must remain distinguishable. These are design recommendations derived from D-01–D-10 and existing proof boundaries. [VERIFIED: 158-CONTEXT.md; chainstate/fjall_store.rs; storage/fjall_store/filters/ownership.rs]

**Primary recommendation:** implement accepted replacement as a trusted continuation of the existing owner, with atomic generation/checkpoint/protection transition and authenticated next-height projection replacement; retain all immutable records and ordinary IfNeeded/Periodic/Always coins cadence. [VERIFIED: 158-CONTEXT.md D-01–D-07; chainstate.rs:551-563]

## Project Constraints (from AGENTS.md)

Material sources: AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md, architecture/code-shape/testing/verification/Rust standards. Project skill directories .claude/skills and .agents/skills are absent. Both active lesson files were read completely: global 5,230 bytes, repository 1,958 bytes, total 7,188 bytes / 2,397 conservative estimated tokens. [VERIFIED: filesystem reads and byte audit]

- Follow pinned Knots 29.3.knots20260210; materialized submodule HEAD is a9aee730466ac67d35a3c03ee24676be5e045878. Register new Rust source breadcrumbs and auditable behavior/layout differences. [VERIFIED: AGENTS.md; git -C packages/bitcoin-knots rev-parse HEAD]
- Keep pure policy in existing functional-core crates; effectful authority/Fjall work stays in node adapters. No new production dependency/crate or Rust Bitcoin library. [VERIFIED: AGENTS.md; standards/core/architecture.md; 158-CONTEXT.md]
- Use Rust 1.94.1, Bun 1.3.9, Bazel/Bzlmod and repo-owned TypeScript automation; no package.json/bootstrap npm install. [VERIFIED: rust-toolchain.toml; .bun-version; AGENTS.md]
- Use foo.rs plus foo/ modules, checked arithmetic, maybe_ optional names, guard clauses; avoid unwrap and swallowed errors; test one concern with Arrange/Act/Assert. Split growing modules rather than putting every transition in chainstate.rs. [VERIFIED: supplied global instructions; standards/languages/rust.md; standards/core/code-shape.md; standards/core/testing.md]
- Use bash scripts/verify.sh as default final contract, including breadcrumbs, claims, pure-core coverage and Bazel smoke. Preserve hooks and tracked generated LOC freshness. Ad hoc Cargo/Bazel commands must use scripts/command-timings.ts and never overlap the same target. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Follow GSD lifecycle; root's strict wrapper defers all Git finalization until full verification, source/security reviews and lifecycle validation pass. This researcher must not commit despite config commit_docs=true. [VERIFIED: 158-CONTEXT.md; parent task]
- Update relevant README/parity/UAT claims and supply explicit Cargo/Bazel commands. No frontend, RPC/peer product expansion, hidden repair/download, public defaults or production/funds claims. [VERIFIED: AGENTS.md; active repository lessons; 158-CONTEXT.md D-12 and Deferred Ideas]
- Preserve append-only task/lesson blocks, frontmatter delimiter discipline and managed standards ownership. Research ownership is only this file. [VERIFIED: supplied global instructions; AGENTS.bright-builds.md; parent task]

## Standard Stack

Reuse pinned dependencies; this phase does not select/install packages, so npm latest-version discovery is inapplicable. Versions below are repository requirements rather than upgrade advice. [VERIFIED: 158-CONTEXT.md; AGENTS.md]

| Component | Verified version/pin | Purpose | Why use it |
| --- | --- | --- | --- |
| Rust/Cargo | 1.94.1; compiler e408947bf, Cargo 29ea6fb6a | Existing core/shell and tests | Pinned toolchain; installed versions matched. [VERIFIED: rustc --version; cargo --version; rust-toolchain.toml] |
| Fjall | git revision aa30dca811399a201e0b9595da93a4582dcb2b57 | Same database, immutable records and atomic SyncAll batches | Existing concrete durable store; no second engine. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; filters/append.rs] |
| Bun | repository pin 1.3.9 | Claims, fixture automation, timed commands | Existing native verifier. Recovered binary at /tmp/open-bitcoin-bun-1.3.9-fresh/bun reports 1.3.9. [VERIFIED: .bun-version; bun --version] |
| First-party chainstate/core | workspace Rust 2024 | Pure progress, validated staging, undo/input binding | Existing domain APIs, no added crate. [VERIFIED: packages/Cargo.toml; open-bitcoin-chainstate/src/engine/stage.rs] |
| Existing serde/serde_json | manifest 1.0.228 / 1.0.149 | Existing evidence/metadata shapes where needed | Keep current schema/codec ownership. [VERIFIED: packages/open-bitcoin-node/Cargo.toml] |

Installation: no dependency addition. Preserve existing lockfiles and pinned Fjall revision. [VERIFIED: 158-CONTEXT.md Agent Discretion]

## Architecture Patterns

### Concrete seams and dependency order

All paths below are under packages/ unless prefixed otherwise. Existing APIs were inspected; proposed modules/APIs are recommendations, not implemented capabilities. [VERIFIED: source reads listed in Sources]

| Order | Responsibility | Existing seam | Planned change |
| --- | --- | --- | --- |
| 1 | Pure replacement facts/progress | open-bitcoin-chainstate/src/filter_index/catch_up.rs and lifecycle.rs | Add checked active-to-active generation renewal and reorg reducer; test branch/height/ancestry/refusal semantics. [VERIFIED: existing APIs; D-03/D-04] |
| 2 | Genuine staged fork facts | open-bitcoin-chainstate/src/engine.rs, engine/stage.rs and lib.rs | First make ALL five currently public StagedChainstateReorg fields private; only successful stage_reorg constructs it. Add borrowed replacement position/undo/old-tip/ancestor accessors and sealed absorb receipt, with external compile-fail and genuine stage/preview/absorb evidence. [VERIFIED: engine.rs:57-63 public fields; stage.rs:37 transition accessor; proposed privacy is required Plan01 work, not current capability] |
| 3 | Guarded replacement publication and sealed bridge | open-bitcoin-node/src/storage/fjall_store/filters/{ownership,publication,append}.rs; chainstate/fjall_store.rs | In Wave2 implement private same-store ValidatedBasicFilterReorg authorization from already tracked lineage plus genuinely absorbed Plan01 receipt, guarded CompletedBasicFilterReorg publication and lineage confirmation. Include real recovered-manager success/fault tests here before Wave3 orchestration. [VERIFIED: current guarded adapter and private lineage seams; proposed exact signatures in 158-02-PLAN.md] |
| 4 | Accepted owner and flush lineage | open-bitcoin-node/src/chainstate.rs; chainstate/{filter_index,fjall_store}.rs | Extend PreparedChainstateReorg; preserve preview versus acceptance; update owner/lineage before subsequent persist result can fail. Seal accepted replacement flush, never reseed from raw tip. [VERIFIED: current absorption/persist order] |
| 5 | Ordinary production consumers | open-bitcoin-node/src/network/mempool_lifecycle.rs; network/runtime_authority.rs; sync/block_reconcile.rs | Preflight before preview/mempool effects; retain existing mempool ordering; direct and staged reorg use one owner transition. Existing scheduled catch-up resumes autonomously. [VERIFIED: reorg_to_branch and reconcile call chains] |
| 6 | Behavioral evidence | open-bitcoin-node/src/sync/tests/filter_index/{recovery,catch_up/fixtures,evidence}.rs | Extend continuous validated fork histories, faults, prune invariants and actual close/reopen. [VERIFIED: existing ValidatedHistory/TurnHistory] |
| 7 | Claims/finalization | docs/parity/, source-breadcrumbs.json, scripts/verify.sh, READMEs and UAT | Phase-specific Bun checker/tests plus deterministic verifier wiring, source/security review and lifecycle gate. [VERIFIED: D-12; native verifier existing phase guards] |

### Pattern 1: Pure exact branch replacement

Recommended input facts: old accepted (height,hash), new accepted (height,hash), common ancestor position, separately verified indexed common record (or Empty), old progress, and achieved new generation/branch/protection. The pure type is policy data, not a store token. [VERIFIED: BasicIndexProgress and AcceptedIndexTarget have no storage authority; D-03/D-04]

Use this reducer contract:

1. Verify the old accepted endpoint still matches the prepared transition; require nonempty genuine replacement or explicitly represent disconnect-only transitions. Verify replacement parent chain joins the ancestor; do not infer branch identity from equal/different heights. [VERIFIED: engine.rs stage_reorg and existing disconnect-only network tests]
2. Let P be processed, S safe, C validated fork ancestor, I indexed common endpoint. I is the highest **already processed** prefix shared with the replacement. If P precedes C, I=P, not C; if P lies above C, I=C only after the ancestor's immutable row/header and lineage are verified; Empty stays Empty. [VERIFIED: D-03; existing contiguous progress invariant]
3. New processed endpoint is I. New safe endpoint is the old safe endpoint if still below/on the shared prefix, otherwise I or the verified shared safe endpoint; never advance safe during acceptance. Compare hashes at equal heights. [VERIFIED: D-03/D-06; catch_up.rs endpoint invariants]
4. Set accepted target to the genuine new tip, stable branch incarnation to the replacement tip hash and generation to checked next. Drop stale next-height facts; retain the original initially_synchronized milestone while computing current lag from the replacement endpoint. An owner never initially synchronized must not become synchronized solely because a reorg shortened its backlog. [VERIFIED: catch_up.rs current initial-sync monotonic behavior; D-04/D-09 from 157]
5. Protection is at least FromHeight(new_safe+1), and may remain stronger. Generation exhaustion, invalid ancestor/endpoint, weak protection and stale transition refuse without reducing old state. [VERIFIED: lifecycle.rs IndexGeneration and catch_up.rs errors/protection]

Add pure tests for fully indexed fork, index behind fork, index within displaced suffix, safe below fork, equal-height different hash, empty index, disabled owner, malformed same-height identity, stale prepared work, generation overflow and stronger retained locks. [VERIFIED: required reducer branches derived from D-03–D-10]

### Pattern 2: Preview is a fenced preparation epoch

prepare_reorg validates the complete fork, then install_prepared_reorg_preview installs staged overlay/metadata for mempool work; commit_prepared_reorg absorbs staged state and calls persist(IfNeeded). Therefore a mempool error and a post-acceptance persistence error are different dispositions even if both return Err. [VERIFIED: chainstate.rs:387-425; network/mempool_lifecycle.rs:138-179; engine/stage.rs:83-96]

Recommended transaction sequence:

- Stage and preflight all required inputs and resource limits before preview, durable index ownership changes or mempool effects. Capture old owner/lineage and sealed staged reorg facts. [VERIFIED: D-02/D-08; existing opaque PreparedChainstateReorg]
- Freeze/fence prepared index work for the preview epoch; use existing serialized authority plus private preview state. Do not replace accepted target or mint replacement flush lineage based solely on preview. Do not call ordinary scheduler against preview state. [VERIFIED: D-02; ManagedNetworkHandle::mutate serialization]
- Preserve existing chain/mempool ordering. On failure before absorption, retain old accepted identity and conservative protection; if preview mutated live state, make its suspension explicit so no later turn mistakes the preview tip for acceptance. Do not invent rollback semantics absent from the current mempool contract. [VERIFIED: engine/stage.rs preview mutation; existing mempool_lifecycle failure tests; D-02]
- At genuine absorption, record accepted replacement and validated lineage **before** calling any later persistence effect. Atomically renew durable index generation, rewind checkpoint/protection to verified shared prefix under publication guard, then install the resulting accepted append identity. A persistence error pauses the accepted owner while keeping new accepted target and facts visible. [VERIFIED: D-02/D-06/D-07; current commit_prepared_connect acceptance-before-persist pattern]
- A fault after the durable batch but before returning its achieved result cannot justify in-memory advancement. Existing poisoned publication/reopen refusal is valid for ambiguous storage failure; it must not become the normal reorg resume mechanism. [VERIFIED: finish_basic_filter_batch and AfterCommit faults; D-01/D-07]

Use a private state enum/capability to distinguish PreviewFrozen, AcceptedReplacement and DurablyFencedReplacement. Names are planner discretion; transitions and authority are mandatory. [VERIFIED: D-02; standards architecture invariant guidance]

### Pattern 3: Bound projection replacement, retain immutable rows

Every record is already stored under basic_filter:v1:record:<blockhash>, with parent hash and branch header. Active mappings are basic_filter:v1:active:<height>. Existing checkpoint readers mask projections above the durable endpoint, and the legacy checkpoint publisher rewrites a prefix without erasing immutable records. [VERIFIED: storage/filter_index.rs keys and fields; filters.rs:173-194; publication.rs:348-418]

Do **not** call the legacy full-prefix publisher/enable/recovery loader for every runtime reorg: it validates the full record forest and prefix, scans projection rows and reloads durable metadata. Its 128 changed-projection cap does not bound total scan work. [VERIFIED: publication.rs:365-375, prepare_basic_filter_projection; filters/lifecycle.rs]

Recommended minimal design: lower the durable checkpoint to the shared safe prefix and keep it authoritative; mask the displaced suffix immediately. Permit overwrite of a conflicting projection only for the exact next height of a sealed replacement append proof, after verifying accepted canonical hash, predecessor/header and immutable retry equality. Replace at most the ordinary turn's admitted records. Leave unreachable suffix rows as ignored storage until replacement catches up; no eager whole-suffix deletion is needed for this contract. Do not treat raw projection rows beyond the current frontier as active lookup authority. [VERIFIED: existing masking/retry mechanics; D-04/D-05; design recommendation]

Keep an explicit constant-size reorg authorization in append identity if required to distinguish legitimate suffix replacement from corruption. Never simply remove the conflict check for generic workers. Validate store identity, generation, branch, old frontier/revision and parent edge during both preparation and guarded completion. An identical old block revisited on a later reorg may reuse its existing hash row; a conflicting record with the same hash remains corruption. [VERIFIED: BasicFilterAppendProof private store binding; append.rs immutable equality and suffix refusal]

This phase selects checkpoint masking plus authenticated bounded next-height overwrite, with no physical whole-suffix deletion and no new durable rewind cursor. Existing startup integrity/protection validation remains before prune resume. The choice avoids adding partial deletion/recovery state and keeps new index publication independent of unrelated history. [RESOLVED DESIGN: D-04/D-07/D-10; existing TurnWork and checkpoint masking; Plans02/06]

### Pattern 4: Branch-aware genuine validated flush lineage

ValidatedChainstateLineage is minted only by from_recovered_chainstate after checking same-store coins and durable proof. Its observe currently supports consecutive connects only. PendingValidatedFlush requires same store, endpoint, generation/branch and exact own-publication revision+2; confirm_basic_filter_flush additionally matches completed coins/metadata receipts and actual H/B. Preserve these anti-forgery checks. [VERIFIED: chainstate/fjall_store.rs:42-161; filters/ownership.rs:139-201]

Plan01 first seals every currently public staged-reorg field, captures old endpoint/common ancestry during genuine staging and mints a private non-Clone AcceptedChainstateReorg only after absorption; private receipt fields alone cannot authenticate forgeable staged input. Existing external node caller uses transition(), so its borrowed read can remain compatible. Add external raw-construction/field-mutation compile-fail proof and real stage/absorb tests. [VERIFIED: engine.rs:57-63,338-407; engine/stage.rs; node/chainstate.rs:403; proposed privacy/receipt, not implemented capability]

The chosen sealed bridge is implemented in Wave2 alongside storage, not deferred to Wave3: ValidatedChainstateLineage::authorize_reorg(&mut self, &AcceptedChainstateReorg, &PreparedBasicFilterReorg) -> Result<ValidatedBasicFilterReorg, StorageError> verifies already tracked old endpoint, store/proof/revision/generation/branch and staged ancestor, then records genuine accepted endpoint with private nonflushable pending state. FjallNodeStore::complete_basic_filter_reorg(prepared, authorization) -> Result<CompletedBasicFilterReorg, StorageError> consumes that token, repeats guarded checks and mints achieved capability only after SyncAll success. ValidatedChainstateLineage::confirm_reorg(&mut self, &CompletedBasicFilterReorg) -> Result<(), StorageError> verifies pending/store/achieved generation/branch/revision before normal flush preparation. All token fields stay private, without Clone/raw/test-only constructors; failure retains pending/nonflushable lineage. Wave3 consumes these exact methods for owner/normal-flush orchestration. Real recovered same-store managers, genuine stage/absorb and production bridge calls make Wave2 success/fault tests reachable without Wave3 or injected lineage. [RESOLVED DESIGN: D-02/D-06/D-07; exact interfaces in 158-02-PLAN.md; existing provenance model in chainstate/fjall_store.rs]

Important fence case: accepted replacement can equal or exceed the old durable height while old durable coins still identify the displaced branch. earned_basic_filter_safe_checkpoint currently selects a row at durable height and refuses a differing hash. Extend the capability to record whether the durable fence remains on the accepted branch, plus the verified shared safe prefix. While durable fence is displaced, processing replacement records must leave safe at the shared prefix instead of selecting a new row by old fence height or refusing every turn. After a successful ordinary replacement coins+metadata receipt, promote the fence to the replacement endpoint and earn checkpoint only from its actual hash/ancestry. [VERIFIED: append.rs earned_basic_filter_safe_checkpoint; D-03/D-06]

Preserve exact-tip release policy unless separately justified: the current implementation only releases newly processed history after reaching the authenticated durable tip. Reorg need not force coins writes or weaken locks for intermediate progress. Normal Periodic/Always/automatic prune remains the durability/deletion owner. [VERIFIED: 157-TURN-MEASUREMENTS.md release cadence; chainstate.rs flush_applying_plan; D-06/D-10]

### Pattern 5: Required-input classification and immutable reuse

| Input case | Required proof/effect |
| --- | --- |
| Disconnected block needed by consensus rewind | Actual body and genuine block-bound undo must be present/valid before preview. Indexed filter availability cannot disconnect coins. [VERIFIED: stage_reorg/apply_disconnect; Knots DisconnectTip; D-08] |
| Replacement block | Supplied validated body plus new undo generated by stage_reorg; do not demand an already durable undo for a brand-new accepted block. Expose borrowed staged undo before consuming stage. [VERIFIED: engine.rs:362-381; D-09] |
| Lagging common-prefix height with no valid indexed row | Actual retained body and non-genesis historical undo bound to exact canonical block; genesis special case preserved. [VERIFIED: catch_up generate_records; D-08/D-09] |
| Already verified indexed row whose source payload was pruned | Reuse immutable bytes/header after verifying exact block identity, ancestry and integrity; do not regenerate or add a blanket retained-history requirement. This does not exempt inputs needed to disconnect chainstate. [VERIFIED: D-09; existing full forest integrity and source-loss saved-prefix tests] |
| Missing required body or undo | Distinct bounded diagnostic; refuse before acceptance/destructive effects, preserve old prefix/hash rows and covering lock. No different-branch undo, current coins, skip, empty filter or hidden redownload. [VERIFIED: D-08/D-09] |

Production sync/block_reconcile loads replacement and displaced bodies before reorg_to_branch; missing replacement bodies currently yield BranchCompetitionAwaitingBodies while displaced missing body errors. Preserve ordinary network competition semantics, but ensure an actual required reorg/index input refusal is explicit and cannot silently become a successful transition. Direct authority callers and reopened in-memory undo caches also need coverage: deleting a durable undo mate must not be concealed by retained in-memory data when that durable mate is a genuinely required input. [VERIFIED: sync/block_reconcile.rs:213-273; D-08/D-11]

Avoid a replacement-body/undo cache proportional to full historical chain. Borrow the supplied branch and staged next_undo_by_block, reuse existing chainstate undo ownership and durable payloads, and retain only the bounded next-height live facts where necessary. Preflight/capture refusal must occur before allocating extra cloned vectors. [VERIFIED: D-04/D-09; stage_reorg existing undo map; MAX_ACCEPTED_FACT_BYTES]

## Finite Work and Measurement Gate

### Known static and measured evidence

These are existing verified ceilings/measurements, not Phase 158 runtime results. [VERIFIED: catch_up/budget.rs; 157-TURN-MEASUREMENTS.md]

| Work | Existing bound/evidence | Planning consequence |
| --- | --- | --- |
| Normal scheduler turn | 8 blocks; 1 MiB body; 4 MiB undo; 16 MiB clone/allocation; 4 Mi item units; 32 Mi byte units; 1 MiB envelopes; 512 record, 1,000,000 structural/checkpoint and 256 projection operations | Preserve complete ledger through reorg-aware admission/completion. [VERIFIED: production_budget] |
| Hard publisher envelope | 128 candidates; aggregate 33,576,192 bytes; singleton 33,554,602 bytes | Bound before encoding/hashing/clone; these are not wall-clock promises. [VERIFIED: MAX_SIZE=0x02000000 and budget.rs constants; arithmetic] |
| Accepted fact retention | 384 MiB / 1,000,000 raw script items | Do not multiply this by arbitrary replacement length; borrow stage facts or bound aggregate before cloning. [VERIFIED: chainstate/filter_index.rs] |
| Native body row | Reject above 4,000,000 bytes before allocating decoder | Reuse existing body probes and bounded scanner. [VERIFIED: filters/turn_inputs.rs; 157-TURN-MEASUREMENTS.md] |
| Recorded Phase 157 turns | Prefixes 16/128/512: subsequent 8-block holds 8.602–9.461ms, 40 indexed point reads, 70 record and 36 projection operations; legal 989,871-byte singleton hold 43.230ms | Reference baseline only; existing measurements do not cover reorg acceptance/rewind. [VERIFIED: 157-TURN-MEASUREMENTS.md] |

The current pure stage_reorg clones the active-chain vector, undo map and confirmed-txid map; preview clones them again. The phase must not falsely claim the whole pre-existing reorg is O(fork depth) or constant memory. The **new index work** can remain constant-size for branch publication plus O(admitted turn records), without a new full-history copy. Missing-input preflight is at least O(required payload bytes) and staging retains its existing O(history) ownership costs. [VERIFIED: engine.rs:347-351; engine/stage.rs:83-95; design complexity from inspected loops]

Derive indexed common prefix from captured old processed/safe endpoints and the staged verified ancestor using existing position ancestry. Only local endpoint/parent record checks should be required after initial forest validation; do not call the public full-ancestry load_basic_filter_record in turn code. Predecessor loads need explicit envelope bounds because a single stored filter can approach MAX_SIZE. [VERIFIED: filters.rs:67-101; append.rs read_basic_filter_local_identity; D-04]

### Mandatory implementation measurement

Before locking any new reorg-specific numeric cap, add an explicit ignored measurement using continuous validated histories at prefixes 16/128/512, fixed equal-height and longer fork depths (e.g. 1/8/32), index positions before/within/after the fork, and real Fjall stores. Measure total acceptance, index-specific preflight, generation/rewind publication, first and subsequent replacement turns separately. Record actual indexed point reads, projection reads/writes, bytes reserved/encoded, batches, authority hold and storage completion, plus old/new safe and processed identities. Proposed sample sizes are research recommendations, not verified timings. [VERIFIED: D-04/D-11; existing measurement harness pattern]

Require deterministic assertions that new index-only transition work does not scale with unrelated prefix length, each projection turn stays within complete TurnWork, and one-over/exhaustion refuses before effects. Distinguish full startup/staging/recovery work from incremental scheduled work; no arbitrary hard latency ceiling or timing assertions. If a new bounded preflight cap is necessary, quantify admitted bytes/operations and refuse before preview when exceeded; do not invent a cap that unnecessarily forbids retained reorgs. [VERIFIED: D-08; 157 measurement disclaimer and admission model]

No Phase 158 timing was run during research. Root initially reported pinned Bun stalled at macOS first launch and instructed static research; the runtime subsequently recovered and its version was confirmed. Implementation measurements remain a concrete execution gate rather than a missing tool blocker. [VERIFIED: parent runtime audit/messages; bun --version]

## Don't Hand-Roll

| Problem | Don't build | Use instead |
| --- | --- | --- |
| BASIC/historical input reconstruction | Fresh encoder or script lookup from current UTXOs | Existing BasicFilterInputs::from_historical, staged undo and StoredFilterRecord::generate. [VERIFIED: stage.rs; storage/filter_index.rs] |
| Durable authorization | Boolean accepted flag, public tip-derived proof | Existing same-store private proof/lineage/receipt machinery extended for reorg. [VERIFIED: ownership.rs; chainstate/fjall_store.rs] |
| Full fork index cache | Duplicate chain/undo vectors or map of every header | Existing chainstate metadata, immutable rows and constant-size accepted owner. [VERIFIED: D-04; AcceptedBasicIndexOwner] |
| Parallel worker/deletion owner | New thread that bypasses chainstate serialization | Existing ManagedNetworkHandle::mutate and scheduled turn. [VERIFIED: runtime_authority and catch_up.rs] |
| Transaction durability | Manual sequence advancing checkpoint/lock before rows | Existing SyncAll batch, revision fencing and poison handling. [VERIFIED: append/publication.rs] |

## Common Pitfalls

- **Height equality hides branch replacement.** Equal-height tips and a lagged index both defeat height-only rewind. Verify endpoint/parent hashes and use indexed common prefix. [VERIFIED: D-03; Knots ReadFilterHeader/LookupOne]
- **Preview becomes acceptance.** Preview physically installs chainstate metadata; a later mempool error must not grant replacement owner/flush authority. Cover this separately from post-acceptance persist Err. [VERIFIED: stage.rs; D-02]
- **Success-only hook loses accepted error state.** Absorption precedes persist. Update accepted owner before persist can fail and retain explicit paused new target. [VERIFIED: chainstate.rs; D-02/D-07]
- **Old durable fence equals new replacement height.** Existing exact-tip earning selects height first; require branch compatibility/shared-prefix fallback until own replacement flush. [VERIFIED: append.rs; D-06]
- **Generation churn makes genuine lineage stale.** Renew store proof, pure owner and private lineage together; preserve unchanged store binding and exact two-write receipts. [VERIFIED: ownership.rs; fjall_store.rs]
- **Broad recovery helper destroys turn bounds.** The legacy publisher scans full history even with small changed-projection limit. Use local bounded publication, retain full integrity at startup. [VERIFIED: publication.rs:365-375; D-04]
- **Accepting arbitrary conflicting projection weakens corruption refusal.** Only sealed reorg next-height replacement may overwrite; same hash with different immutable bytes remains an error. [VERIFIED: append.rs conflict checks; D-05]
- **Cached undo hides real missing retained mate.** Tests must drop/reopen after actual durable pair removal or probe the true store boundary when required. Memory-only loss tests are supplementary. [VERIFIED: D-11; existing history_loss.rs evidence]
- **Reopen used as normal recovery mechanism.** Successful acceptance must continue via ordinary maintenance; reopen is crash/fault evidence only. [VERIFIED: D-01]
- **Filter retention mistaken for block availability.** Retained stale hashes do not make bodies/undo present or change prune/wallet/service labels. [VERIFIED: .planning/PROJECT.md scope; D-10]

## Code Examples

The following are current verified API patterns; the proposed reducer/acceptance APIs in Architecture Patterns are deliberately not presented as compiled code. [VERIFIED: inspected source]

### Genuine staged connect facts, extend to reorg borrowing

~~~rust
// Source: packages/open-bitcoin-chainstate/src/engine/stage.rs
BasicFilterInputs::from_historical(
    block,
    &self.position,
    Some(HistoricalBlockUndo {
        block_hash: self.position.block_hash,
        undo: &self.undo,
    }),
)
~~~

A reorg accessor should borrow the position and generated undo associated with the exact replacement block from next_undo_by_block. Apply this before stage consumption; do not expose a constructor that confers provenance on arbitrary BlockUndo. [VERIFIED: engine.rs stage_reorg; stage.rs connect accessor; D-09]

### Prepared generation/frontier and achieved publication

~~~rust
// Source: packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
let prepared_progress = current.prepare_turn().map_err(index_corruption)?;
let prepared = store.prepare_basic_filter_append(proof, &records)?;
let achieved = store.complete_basic_filter_append(prepared)?;
manager.complete_basic_index_turn(prepared_progress, &identities)
    .map_err(index_corruption)?;
manager.confirm_basic_index_checkpoint(achieved.safe_checkpoint, achieved.protection)
    .map_err(index_corruption)?;
~~~

Preserve the actual driver's preflight/trial checks and empty-record handling omitted from this short excerpt. Apply owner reduction only after achieved storage publication. [VERIFIED: catch_up.rs drive_turn]

### Reproducible implementation commands

~~~bash
PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH \
  bun run scripts/command-timings.ts run --key phase158-reorg-node \
  -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase158

PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH \
  bun run scripts/command-timings.ts run --key phase158-reorg-core \
  -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate phase158
~~~

These commands are planned test selectors after new tests exist and pinned Bun preflight succeeds; no run is claimed. For explicit measurements add the registered measurement test name plus -- --ignored --nocapture. Full final contract remains bash scripts/verify.sh under pinned PATH. [VERIFIED: AGENTS.md command policy; D-12]

## State of the Art

| Current source behavior | Required Phase 158 behavior | Evidence |
| --- | --- | --- |
| Reorg clears lineage/invalidates owner | Genuine accepted branch renews both, ordinary catch-up resumes | [VERIFIED: chainstate.rs:407-425; D-01/D-06] |
| Append refuses any conflicting active projection | Sealed replacement authority can replace exact next-height projection while immutable row equality remains strict | [VERIFIED: append.rs; D-05] |
| Flush lineage observes only next-height connects | Genuine staged accepted replacement advances branch lineage, exact own coins/meta receipt remains required | [VERIFIED: fjall_store.rs; D-06] |
| Runtime reorg proof deferred after Phase 157 | Continuous validated fork, missing input, faults and real reopen prove CFIX-03 | [VERIFIED: ROADMAP Phase 158; PROJECT current state] |

Pinned Knots CustomRewind preserves displaced entries by copying height rows to hash rows, restores ancestor header, and BaseIndex::Rewind deliberately avoids committing ahead of flushed chainstate. Knots CustomAppend requires non-genesis undo; LookupOne verifies height-row hash before hash fallback. Open Bitcoin already stores hash rows first, so retention requires preserving those rows rather than recreating Knots fltr layout. [VERIFIED: pinned index/blockfilterindex.cpp:265-285,336-374; index/base.cpp:247-264]

BIP157 defines double-SHA256(filter_hash || previous_filter_header), with zero predecessor at genesis; branch-specific predecessor is therefore essential. [CITED: https://bips.dev/157/]

## Runtime State Inventory

This is an existing lifecycle refactor; inventory is scoped to the inspected repo and test/runtime seams, not an assertion about unknown live deployments. [VERIFIED: parent task scope; PROJECT]

| Category | Items found | Action required |
| --- | --- | --- |
| Stored data | BASIC record/active/state/owner keys and reserved prune lock in existing Fjall BlockIndex; coins H/B and chain metadata separately written | Preserve immutable rows/key format, extend trusted transitions; if adding durable reorg markers, version/parse and test old stores. No blanket data rewrite. [VERIFIED: filter_index.rs; ownership.rs; publication.rs] |
| Live service config | No external service setting is required by the traced headless reorg path | No external service mutation planned; actual live service inventory was not attempted. [VERIFIED: runtime_authority/reorg and sync callers] |
| OS registrations | No service-name/path rename in phase scope | None; OS registrations were not inspected, so no global absence claim. [VERIFIED: 158-CONTEXT.md scope] |
| Secrets/env | No secret/env-name change required | Preserve configuration; no secret content read. [VERIFIED: inspected reorg/proof APIs] |
| Build artifacts | Existing Rust workspace/Bazel targets; new modules/tests require normal builds and source breadcrumbs | Existing build/check workflow, no installed package rename. [VERIFIED: AGENTS.md; packages/Cargo.toml] |

## Evidence and Failure Matrix

All rows below are required planned evidence derived from D-02–D-11. Existing fixtures validate actual spends with custom maturity=1 and easy local proof of work; label this software evidence accurately, not public-mainnet consensus operation. [VERIFIED: ValidatedHistory and TurnHistory fixture implementations]

| Scenario | Required observations |
| --- | --- |
| Equal-height validated replacement | New accepted hash, common prefix/header, generation change, stale prepared turn refusal, replacement filter/header, no append from displaced tip |
| Longer continuous replacement | More than one admitted turn; subsequent offline/idle turns finish automatically; new connects enlarge the same ordered backlog |
| Catch-up before fork | Preserve processed endpoint before ancestor; index shared lag first; do not manufacture ancestor checkpoint |
| Catch-up inside displaced suffix | Rewind only verified indexed portion; discard stale next-height facts; replacement header joins ancestor |
| Accepted-unflushed fork | Old recoverable coins fence remains; new processed records may advance, safe prefix/protection do not; ordinary later successful flush seals replacement |
| Required body loss | Actual source mate removal and explicit refusal before preview/acceptance; immutable rows/old progress/locks unchanged |
| Required undo loss | Actual correct-branch mate removal; body present; refusal cannot be bypassed with cache, same-shape other branch undo or current coins |
| Preview/mempool failure | No new accepted BASIC lineage, no unsafe scheduler work/release; disposition separate from absorbed persistence failure |
| Body/undo/coins/meta failure after acceptance | Accepted replacement remains visible and paused; safe/protection conservative; reopened old prefix or explicit fail-closed H/B mismatch |
| Rewind/protection/projection before-effect failure | No partial successful transition, no hash-row loss; generation/frontier stale work refused |
| AfterCommit ambiguity | No invented in-memory achieved result; real drop/reopen finds verified prefix or fails closed and retains immutable branch bytes |
| Ordinary flush + real close/reopen | Active height lookup follows replacement; every displaced hash returns exactly its original filter bytes/hash/header/parent |
| Return to earlier indexed branch | Reuse immutable equal records even after source pruning where not needed for consensus disconnect; no conflict weakening |
| Prune ownership | Operator cannot clear/weaken reserved lock; preplanned manual/automatic and resumed intent respect new generation/protection |

Fixtures must drop runtime, manager, store, proofs and clone handles before reopening the same path; a fresh handle sharing the existing Arc is not actual reopen. Test actual production ManagedNetworkHandle::reorg_to_branch and sync reconciliation, not only raw fixture writes. [VERIFIED: D-11; existing real-reopen fixture patterns]

## Assumptions Log

| # | Claim | Section | Risk if wrong |
| --- | --- | --- | --- |
| None | No training-only factual claim is used. Proposed architecture/budgets are explicitly recommendations backed by traced constraints; new performance results remain unmeasured. | All | Planner must implement and validate recommendations before recording capabilities. |

## Open Questions

1. **RESOLVED — Preview failure disposition.** Use private PreviewFrozen suspension under existing serialized authority; preview neither changes accepted BASIC endpoint nor mints accepted/flush authority. Since preview changes live overlay/metadata, failure before absorption preserves old accepted index identity and conservative protection but leaves scheduler/prepared publications suspended. Do not claim rollback of chainstate/mempool. Only successful genuine accepted completion clears suspension through the manager transition; post-absorb errors instead retain visible new accepted identity with a persistence pause. Concrete durable evidence in Plans03/06 must prove both dispositions separately. [VERIFIED source premise: engine/stage.rs; reorg_reject_evidence.rs; RESOLVED DESIGN: D-02/D-07]
2. **RESOLVED — Sealed capability contract.** First seal ALL StagedChainstateReorg fields in Plan01, with read-only accessors and external compile-fail proof; constructor solely genuine staging, receipt solely absorption. In Plan02 implement tracked lineage authorize_reorg -> private non-Clone ValidatedBasicFilterReorg -> guarded SyncAll CompletedBasicFilterReorg -> confirm_reorg, using exact signatures in that plan. No raw/public/test-only constructor or injected lineage. Append identity stores only fixed-size replacement next-height permission, displaced durable-fence identity and verified shared-safe prefix. Only achieved same-store publication enables replacement projection; normal successful own coins/metadata receipt later joins durable fence. Plan02 owns bridge success/fault tests; Plan03 owns orchestration. [VERIFIED source premise: public engine.rs stage fields; private fjall_store.rs lineage; RESOLVED DESIGN: D-02/D-05/D-06]
3. **RESOLVED — Measurement-before-new-cap policy; execution measurements remain UNMEASURED.** Reuse existing complete finite TurnWork/envelope bounds and checked accounting while building this phase. No additional reorg-specific numeric cap may be finalized from guesses or Phase157 timings. Plan06 runs explicit continuous validated concrete prefixes/fork-depth/index-position measurements, reports pre-existing stage/preview history costs separately, and tests deterministic exact/one-over/exhaustion before effects. Only if observations justify a new cap, amend the exact owning budget paths and prove pre-preview refusal before accepting that cap or its documentation. Timing results/calibration are pending execution gates, not unresolved architecture and not experiments already run. [VERIFIED source premise: existing budgets/157 evidence; RESOLVED DESIGN: D-04/D-08/D-11]
4. **RESOLVED — Checkpoint masking, no durable rewind cursor.** Select existing authoritative checkpoint masking plus bounded authenticated projection overwrite. Retain immutable hash rows and ignored incompatible active suffix rows; do not eagerly delete/scan the whole suffix. No new versioned rewind cursor or unfinished-rewind marker is needed for this design. Startup still validates recovered index protection before resuming prune intent, and Plans02/05/06 prove fault/reopen masking and retained hash identity. [VERIFIED source premise: existing checkpoint masking; RESOLVED DESIGN: D-04/D-05/D-07/D-10]

All four design questions are resolved. Phase158 timings, deterministic counters, limit calibration if needed, API compile-fail proof and concrete behavior remain planned execution evidence; this revision claims no completed experiments or shipped capability.

## Environment Availability

| Dependency | Available | Version/status | Fallback |
| --- | --- | --- | --- |
| Rust/Cargo | Yes | Both 1.94.1, confirmed by commands | None needed. [VERIFIED: rustc/cargo --version] |
| Bun matching pin | Yes | /tmp/open-bitcoin-bun-1.3.9-fresh/bun reports 1.3.9 after first-launch recovery | Use task-local PATH prefix for scripts and timed Cargo/Bazel. [VERIFIED: .bun-version; parent audit; bun --version] |
| Bazel/Bazelisk | Commands present | /opt/homebrew/bin/bazel and bazelisk; version not executed here | Default verifier owns smoke/version preflight. [VERIFIED: command -v] |
| Knots source | Yes | Exact a9aee730466ac67d35a3c03ee24676be5e045878 | No baseline source fallback needed. [VERIFIED: git submodule HEAD] |
| Fjall library/store | Existing production dependency | Pinned git revision; concrete test paths present; no test run claimed here | Implementation uses existing Cargo workspace. [VERIFIED: node manifest and fixture sources] |

No missing external dependency currently blocks planning. The earlier Bun launch issue recovered; execution still must run new measurements and the full native verifier. No dependency/service/credential installation is proposed. [VERIFIED: parent runtime recovery message; local bun --version; phase scope]

## Security Domain

security_enforcement is absent from config, so security domain is included; workflow.nyquist_validation=false means no Validation Architecture section. ASVS numbering below explicitly uses the **4.0.3 taxonomy requested by the research template**, not an assertion that it is the latest ASVS release or a compliance certification. Its input/resource/business-logic categories were verified against official versioned source. [VERIFIED: .planning/config.json; OWASP official v4.0.3 source listing] [CITED: https://owasp.org/projects/asvs]

| ASVS category | Applies to changed surface | Standard control / evidence |
| --- | --- | --- |
| V2 Authentication | No new authentication interface | Existing RPC/peer activation is outside this phase; preserve caller boundaries. [VERIFIED: D-12 Deferred Ideas; runtime authority scope] |
| V3 Session Management | No new user session lifecycle | Do not add public endpoints/session state. [VERIFIED: phase scope] |
| V4 Access Control | Yes, internal capabilities/prune ownership | Private same-store transition proofs, unforgeable genuine lineage, serialized authority; reserved lock cannot be weakened by operator CRUD. [VERIFIED: ownership.rs; D-10] |
| V5 Input Validation | Yes | Strong staged domain facts, exact parent/hash/header checks, bounded decode/envelopes and checked counters. [VERIFIED: stage/append/input code] [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v4.0.3/4.0/en/0x13-V5-Validation-Sanitization-Encoding.md] |
| V6 Cryptography | Existing commitments only | Reuse verified first-party BASIC/hash/header implementation; no new cryptography. [VERIFIED: D-09; StoredFilterRecord::generate] |
| V11 Business Logic | Yes | Explicit preview/accepted/durable states, no phantom achievements; atomic index/protection transition and failure tests. [VERIFIED: current proof boundaries] [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v4.0.3/4.0/en/0x19-V11-BusLogic.md] |
| V12 Files/Resources | Yes, disk input and bounded work | Full integrity startup, envelope admission before decode/clone, complete turn ledger, conservative retention under failures. [VERIFIED: input/work/publisher code] [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v4.0.3/4.0/en/0x20-V12-Files-Resources.md] |

| Threat | STRIDE | Required mitigation |
| --- | --- | --- |
| Stale old-branch prepared publication | Tampering | Renew generation/branch/revision under shared publication guard; compare at completion. [VERIFIED: D-04; append proof model] |
| Preview/raw tip impersonates genuine accepted lineage | Spoofing / Elevation | Opaque staged acceptance capability, private lineage transition; reject generic reconstruction/foreign receipts. [VERIFIED: fjall_store provenance tests; D-02/D-06] |
| Checkpoint/protection outruns recoverable replacement coins | Tampering / Denial | Shared safe prefix until exact completed own coins+metadata proof; locks retain required inputs. [VERIFIED: D-06/D-07; receipt model] |
| Missing body/undo concealed by different branch/current coins | Tampering | Explicit source-bound preflight, no substitution; durable loss fixtures. [VERIFIED: D-08/D-09] |
| Unbounded reorg-index scans/duplicate caches | Denial | Constant-size transition facts and bounded projection turns; measure new index work versus prefix size. [VERIFIED: D-04; existing work ledger] |
| Persistence error reported as achieved branch/index state | Repudiation / Tampering | Accepted versus achieved status and paused failure; AfterCommit/reopen controls, preserve immutable bytes. [VERIFIED: D-02/D-07/D-11] |

## Sources

### Primary (HIGH confidence)

- Local pinned Knots a9aee730466ac67d35a3c03ee24676be5e045878: src/index/base.cpp (rewind, flush callback), src/index/blockfilterindex.cpp (historical undo, header rewind, stale hash lookup), src/validation.cpp (DisconnectTip/ConnectTip/ActivateBestChainStep), src/undo.h (complete spent output metadata). [VERIFIED: source reads and git HEAD]
- [BIP157 specification](https://bips.dev/157/) — ancestry commitments checked during this research; local pinned Knots remains behavioral authority. [CITED: https://bips.dev/157/]
- Existing first-party engine/stage, catch_up/lifecycle, node chainstate/fjall_store/filter owner, Fjall filters append/publication/ownership/lifecycle/startup and sync/network callers cited in sections. [VERIFIED: source reads]
- Phase154–157 contexts, Phase158 context, current PROJECT/REQUIREMENTS/STATE/ROADMAP, milestone research architecture/features/pitfalls and Phase157 turn measurement artifact. [VERIFIED: local planning reads]
- Repo instructions/standards, complete active lessons, verifier and breadcrumb manifest; no project skill directories. [VERIFIED: local filesystem audit]
- [OWASP ASVS](https://owasp.org/projects/asvs) and official versioned 4.0.3 V5/V11/V12 files linked in Security Domain. [CITED: official OWASP sources]

### Secondary / Tertiary

No community/blog/training-only evidence is required. No new package recommendation/version registry claim was made. [VERIFIED: research source inventory]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — manifest/toolchain/submodule confirmed; Bun runtime limitation explicit. [VERIFIED: environment audit]
- Architecture: MEDIUM — exact existing boundaries are HIGH confidence; proposed sealed replacement integration requires implementation/behavior proof. [VERIFIED: source tracing; recommendations above]
- Pitfalls: HIGH — current preview invalidation, suffix conflict and exact-height fence failure are visible source paths. [VERIFIED: chainstate/append/ownership source]
- New performance bounds: MEDIUM pending measurement — prior turn ceilings measured/documented; no new reorg timing or all-payload claim. [VERIFIED: Phase157 measurement artifact; no Phase158 run]

**Research date:** 2026-10-07 CDT / 2026-10-08 UTC.
**Valid until:** retrace on source change; 30-day source review horizon is an operational recommendation, not a dependency guarantee.
**Git:** no commit/push; strict parent wrapper owns verified finalization. [VERIFIED: parent task]
