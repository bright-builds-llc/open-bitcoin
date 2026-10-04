# Requirements: Open Bitcoin

**Defined:** 2026-10-03
**Milestone:** v2.5 Prune-Aware Compact-Filter Serving (BIP157/158)
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v2.5 Requirements

Requirements for explicitly enabled BASIC/type 0 compact-filter indexing and serving on the existing single active Fjall chainstate. Bitcoin Knots `29.3.knots20260210` is the behavioral baseline. All requirements are pending; planned scope does not claim shipped compact-filter support. Each requirement receives exactly one owning roadmap phase.

### Filter Construction

- [x] **CFIL-01**: A client can obtain the exact pinned BASIC filter bytes for a validated block using its output scripts and historical spent-output scripts, including genesis, empty filters, duplicate scripts and OP_RETURN exclusions.
- [x] **CFIL-02**: A client can obtain the exact filter hash and ancestry-dependent filter header, including the zero genesis predecessor and replacement-branch commitments.

### Index Activation

- [ ] **CFAC-01**: An operator can enable BASIC indexing through the supported Knots `blockfilterindex` forms (bare, `1`, or `basic`); omitted or `0` remains disabled, repeated/mixed forms have documented tested semantics, and explicit index activation selects durable storage without implicitly enabling networking.
- [ ] **CFAC-02**: An operator gets a non-mutating activation refusal when a fresh or saved index needs block bodies or non-genesis undo already removed by pruning; the node preserves any valid saved prefix and does not skip heights, reconstruct history from current coins, or automatically repair/download.

### Durable Index Lifecycle

- [ ] **CFIX-01**: An operator can observe ordered, bounded catch-up from retained history and ongoing indexing from ordinary validated connects; scheduled catch-up progresses without another peer message and incomplete work never reports a complete index.
- [ ] **CFIX-02**: An operator can reopen the real Fjall datadir after successful or interrupted index writes and recover valid filter records and safe progress, or receive a fail-closed diagnostic without phantom cursor advancement.
- [ ] **CFIX-03**: A client gets branch-correct replacement filters and headers after a validated reorg and can retrieve already-indexed displaced blocks by hash; missing deep-reorg inputs cause explicit refusal rather than invented history.
- [ ] **CFIX-04**: An operator can restart after index work runs ahead of a coins/chain-metadata flush without trusting a resume cursor beyond the recovered durable chainstate checkpoint; immutable filter records and active progress are reconciled to the recovered branch.

### Prune Coordination

- [ ] **CFPR-01**: An active index protects all required body/undo inputs from both manual and ordinary automatic pruning until its safe durable checkpoint permits release; operator lock set/clear cannot weaken index-owned protection, and disable/re-enable has an explicit ownership transition.
- [ ] **CFPR-02**: A client can retrieve previously indexed filters, headers and checkpoints after real paired body/undo deletion and datadir reopen, while ordinary block serving and wallet eligibility retain the shipped pruned-body restrictions.
- [ ] **CFPR-03**: An operator can reopen a datadir with interrupted prune intent without recovery deleting inputs required by the index: index protection is validated before resumed deletion, and unsafe combinations retain protection or refuse with a diagnostic.

### Peer Serving

- [ ] **CFNET-01**: An operator can explicitly enable general BASIC serving or the existing per-peer `blockfilters` permission only with the BASIC index prerequisite; `NODE_COMPACT_FILTERS` follows configured capability independently of catch-up completion, coexists truthfully with `NODE_NETWORK_LIMITED`, and does not implicitly activate listeners or public defaults.
- [ ] **CFNET-02**: A peer can receive ordered `cfilter` responses for an entirely indexed legal range of at most 1,000 blocks, selected by the requested stop block's ancestry rather than only active heights, including during catch-up.
- [ ] **CFNET-03**: A peer can receive `cfheaders` with the correct preceding header and ordered filter hashes for an entirely indexed legal range of at most 2,000 blocks, including genesis and permitted stale branches.
- [ ] **CFNET-04**: A peer can receive `cfcheckpt` headers at every positive 1,000-block interval through its permitted stop block, with correct genesis exclusion and branch ancestry.
- [ ] **CFNET-05**: Invalid compact-filter requests receive the pinned Knots disconnect/validation behavior, while absent records in an otherwise valid range never produce invented empty filters or a shortened successful response.
- [ ] **CFNET-06**: Peer serving obeys explicit work, byte and queue bounds on real inbound and outbound runtime paths, reads persisted filters instead of regenerating on request, and earns response evidence only from achieved transport writes, including partial-write failure.

### RPC and Operator Evidence

- [ ] **CFRP-01**: An authenticated client can call BASIC `getblockfilter` with pinned result shape, default filter type, error ordering and codes for disabled index, unknown block/type, never-connected block, indexing absence and corruption, including successful retained stale/pruned lookup.
- [ ] **CFRP-02**: An authenticated client can call `getindexinfo` with the pinned BASIC index name, `synced` and `best_block_height`, exact-name selection and empty-object absence; `synced` preserves Knots' initial-synchronization meaning.
- [ ] **CFOP-01**: An operator can distinguish configured capability, initial catch-up, current lag, safe durable progress, missing history and index failures consistently across existing status RPC, repo-local CLI, dashboard and redacted support evidence; retained filter growth is separate from the soft block/undo prune target.

### Parity and Integrated Proof

- [ ] **CFGR-01**: A contributor can audit pinned BASIC/protocol/RPC parity and intentional Fjall, BASIC-only and local resource-policy differences through source breadcrumbs, parity roots and deterministic native claim checks that retain the milestone's excluded surfaces.
- [ ] **CFGR-02**: A contributor can reproduce a hermetic continuous validated-chain scenario through actual startup, scheduled catch-up, connect/reorg, real Fjall paired prune, reopen, RPC and both peer transport paths, including missing-history refusal and durability/write failures; sparse codec-valid fixtures alone do not satisfy this proof.

## Future Requirements

Deferred and not mapped to the v2.5 roadmap. FUT-20 compact-filter serving is promoted into the scoped requirements above; BIP37 remains excluded.

- **FUT-19**: Archive-node or production-scale historical serving.
- **FUT-21**: Assumeutxo, assumevalid, IBD snapshot shortcuts or a second chainstate.
- **FUT-22**: Live Knots/Core LevelDB `chainstate/` import or export.
- **FUT-23**: Automatic destructive reindex or coins repair.
- **FUT-24**: Public serving or relay enabled by default.
- **FUT-25**: Public-network runs become default CI or release-blocking gates.
- **FUT-26**: Production full-node readiness, production service operation or production-funds wallet safety claims.
- **FUT-27**: Temporary initial-download prune target (`-pruneduringinit`).
- **FUT-28**: Knots V0 filter generation/indexing and its additional local RPC scope.
- **FUT-29**: Historical filter import or explicit missing-history recovery under separately defined provenance and mutation policy.
- **FUT-30**: `scanblocks`, filter-assisted wallet acceleration or a new lightweight-client wallet.

## Out of Scope

| Feature | Reason |
| -- | -- |
| BIP37 bloom serving | Separate privacy/DoS surface; BASIC compact filters are the selected light-client capability. |
| Knots V0/type 2 filters | BIP158 BASIC/type 0 is selected; the pinned Knots local V0 extension needs a separate generator/index scope and explicit parity exclusion. |
| Partial-history index advertised as complete | Historical commitments require a validated genesis prefix; already-pruned gaps must refuse activation. |
| Implicit reindex, redownload or destructive repair | Missing-history and corruption diagnostics do not authorize source-history mutation or acquisition. |
| Filter import from peers | Trust, provenance and historical reconstruction policy are separate work. |
| New filter-assisted wallet behavior | Existing wallet eligibility stays coins/body based; this milestone serves filters rather than adding a new wallet product. |
| New production crates or Rust Bitcoin dependencies | Extend existing first-party crates and Fjall; retain the current Rust/Bazel/Bun stack. |
| Knots LevelDB/fltr storage layout | Match observable behavior on Fjall and document layout/durability differences rather than add a second storage engine. |
| Archive-scale serving, assumeutxo, public defaults or production/funds claims | Existing capability, activation and evidence gates remain independent. |
| Public-network default CI or release-blocking live sync | Native verification stays deterministic and hermetic; optional network UAT remains explicit. |
| Unrelated v2.4 advisory cleanup | Address a retained advisory only when required for the selected index/prune contract. |

## Traceability

All 22 current v2.5 requirements map to exactly one owning phase. CFIL-01 and CFIL-02 are Complete after Phase 154's lifecycle-valid verification and full native pass; the remaining 20 requirements are Pending.

| Requirement | Phase | Status |
| -- | -- | -- |
| CFIL-01 | Phase 154 | Complete |
| CFIL-02 | Phase 154 | Complete |
| CFIX-02 | Phase 155 | Pending |
| CFIX-04 | Phase 155 | Pending |
| CFPR-03 | Phase 155 | Pending |
| CFPR-01 | Phase 156 | Pending |
| CFAC-01 | Phase 157 | Pending |
| CFAC-02 | Phase 157 | Pending |
| CFIX-01 | Phase 157 | Pending |
| CFIX-03 | Phase 158 | Pending |
| CFRP-01 | Phase 159 | Pending |
| CFRP-02 | Phase 159 | Pending |
| CFNET-01 | Phase 160 | Pending |
| CFNET-02 | Phase 160 | Pending |
| CFNET-03 | Phase 160 | Pending |
| CFNET-04 | Phase 160 | Pending |
| CFNET-05 | Phase 160 | Pending |
| CFNET-06 | Phase 160 | Pending |
| CFOP-01 | Phase 161 | Pending |
| CFPR-02 | Phase 162 | Pending |
| CFGR-01 | Phase 162 | Pending |
| CFGR-02 | Phase 162 | Pending |

Coverage: 22/22 requirements mapped; 0 unmapped; 0 duplicate owners.

______________________________________________________________________

*Requirements defined: 2026-10-03*
*Last updated: 2026-10-03 after v2.5 research-backed scope selection*
