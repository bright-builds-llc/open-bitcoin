---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: 2026-10-10T18:43:21Z
---

# Phase 160: Explicit Bounded Peer Filter Serving - Research

**Researched:** 2026-10-10 CDT
**Domain:** BASIC BIP157 codecs, permission/capability policy, branch-aware persisted reads and achieved peer transport
**Confidence:** HIGH for pinned behavior and existing seams; MEDIUM for new implementation design and measured budgets

<user-constraints>
## User Constraints (from CONTEXT.md)

The following locked decisions, discretion and exclusions are copied verbatim from finalized 160-CONTEXT.md. [VERIFIED: 160-CONTEXT.md]

### Locked Decisions

### Activation, permissions and service bits

- **D-01:** Support explicit general BASIC peer serving through a narrowly owned configuration/daemon surface anchored to pinned `peerblockfilters`. General serving and an explicitly requested `blockfilters` permission require an enabled BASIC index; refuse invalid configuration before networking starts or destructive startup effects. Index activation alone does not enable peer serving or listeners.
- **D-02:** Derive `NODE_COMPACT_FILTERS` from configured general or permitted per-peer capability before initial catch-up completes. Observe actual version messages on inbound and outbound paths. A synchronized latch, current tip equality or a successful query is not the service-bit authority. Compose truthfully with prune-derived `NODE_NETWORK_LIMITED`; do not erase the existing network-service classification.
- **D-03:** Preserve requested-versus-expanded permission provenance. Pinned Knots distinguishes `BlockFilters_Explicit` from the `all`-expanded `BlockFilters` flag; implicit whitelist expansion omits blockfilters. Explicit `blockfilters` must enforce the BASIC prerequisite. Preserve the pinned `all` corner: `all` alone is accepted with a disabled index, grants effective per-peer BlockFilters and advertises that peer's service bit; a valid request finding no backend returns without a normal response. Combining `all` with explicitly named `blockfilters` still requires BASIC. `all` never initializes a disabled index, listener, BIP37/V0 product or public default. Do not silently equate explicit and expanded flags or infer synchronized/backend availability from this permission-derived advertisement; document this precise baseline corner. With BASIC enabled, effective `all` capability still obeys finite resource limits. Existing unpermissioned/implicit/noban/download/protected peers gain no per-peer filter privilege by implication.
- **D-04:** Keep peer-serving eligibility separate from transaction relay, block-body serving and compact-block relay. Existing per-direction permission classes and actual inbound/outbound callers must consume one typed capability decision. No global service bit may be inferred from a single permissioned peer.

### Pinned validation and branch selection

- **D-05:** Preserve `PrepareBlockFilterRequest` ordering: BASIC plus the peer's advertised service capability, known/allowed stop hash, start not above stop, inclusive range within the family limit, then index lookup. Unsupported type/unadvertised capability, invalid/disallowed stop, reversed and oversized ranges disconnect as pinned. Knots V0/type2 is unsupported on this BASIC peer surface even though its local RPC recognition differs.
- **D-06:** Legal inclusive limits are 1,000 cfilters and 2,000 cfheaders entries. Cover exact limit and limit-plus-one, height arithmetic, genesis, malformed/trailing wire payloads, unknown type and combined-invalid precedence. Do not substitute RPC errors for peer wire/disconnect behavior.
- **D-07:** Resolve ranges from the requested stop block's ancestry, including retained stale branches permitted by pinned `BlockRequestAllowed`: active membership, or scripts-valid stale provenance with the pinned best-header timestamp and proof-equivalent-age tests. Hash existence, present body, current-height lookup and retained filter existence alone do not authorize stale serving. Pruned body absence must not disqualify an otherwise permitted indexed filter.
- **D-08:** cfilters return exact ordered `(type, block hash, filter bytes)` for the entire inclusive range. cfheaders returns the requested stop hash, predecessor header at start minus one (zero only for start zero), and ordered raw filter hashes. Preserve wire/raw digest byte order separately from RPC display hex.
- **D-09:** cfcheckpt returns headers at heights 1000, 2000, and subsequent positive multiples through the stop height on that branch; genesis is excluded. Stops below1000 produce an empty valid checkpoint vector. Resource bounds must account for vector bytes, ancestry/work and queued transport; do not impose the 1,000/2,000 contiguous-range caps on checkpoint stop heights.

### Persisted reads and bounded complete preparation

- **D-10:** Use the existing configured durable `ManagedNetworkHandle` authority and immutable hash-keyed records, with narrow integrity-preserving request reads. Do not create an independent filter database/full-history cache, regenerate from block bodies/current coins, mutate indexing on request, repair/download history or wait for whole-index initial synchronization.
- **D-11:** Serve already-indexed complete permitted ranges during catch-up. Resolve and validate the complete response before emitting its first wire message. A missing record/predecessor/checkpoint in an otherwise valid request yields no fabricated empty filter and no shortened successful response; pinned index lookup failure sends no normal response. Backend corruption/fault remains a distinct fail-closed internal outcome rather than ordinary absence or success.
- **D-12:** Define measured finite per-request/per-peer/aggregate work, record-read, copy/encoded-byte and queue budgets before allocating large response collections. Count ancestry traversal and point-read integrity checks, rather than only output entries. Permissioned and protected peers remain bounded. Local exhaustion behavior and any differences from Knots are explicit parity policy, not an invented protocol limit. Exact numeric defaults and narrow API design belong to research/planning.
- **D-13:** Capture branch/lifecycle identity with immutable prepared data, prevent stale plans or duplicate completion from earning live evidence, and write sockets outside the shared authority/context lock. Keep accepted/durable progress, initial synchronization and network capability distinct. Preserve existing owner/prune generation and conservative recovery rather than adding another serialized owner.

### Transport receipts and cleanup

- **D-14:** Integrate all three families into real inbound and outbound message dispatch/transport paths. A prepared plan, encoded message, enqueue, successful policy decision or passed mock alone is not a completed serving receipt.
- **D-15:** Credit message/bytes only after the real transport achieves a complete framed write. Full-range success requires every intended message; cfilter prefix writes may be recorded accurately, but a partial frame or failed later write cannot credit that frame or full range. Retry/replay cannot double-credit an already completed receipt.
- **D-16:** Queue pressure, timeout, cancellation, disconnect, partial-write failure and lifecycle invalidation abort the unwritten suffix and release reservations/in-flight state. Preserve truthful already-written-prefix evidence without continuing stale/unwritten work. Bound total buffered suffix bytes and do not hold the authority lock while waiting for transport backpressure.

### Evidence, docs and verification

- **D-17:** Prove observed version bits, prerequisite/refusal matrix, explicit/all/implicit permissions, active/recent-stale/disallowed stale branches, exact range limits, checkpoint intervals, catch-up available/missing ranges, corruption, concurrent lifecycle change and cleanup through production callers. Use deterministic wire fixtures plus genuine validated-chain concrete Fjall/durable-coins histories. Independent pinned Knots helpers/source supply expected bytes and contracts.
- **D-18:** Include successful and forced partial/failed transport writes on both actual inbound and outbound adapter paths; source-string checks or testing only a standalone serving helper cannot establish CFNET-06. Use loopback/injected production transport seams and deterministic clocks; default verification remains hermetic/public-network-free.
- **D-19:** Register Rust source/test parity breadcrumbs, update scoped parity/docs and relevant README/UAT, and include copy-pasteable repo-local Cargo/Bazel commands. Native `bash scripts/verify.sh`, independent source/security review, explicit simplification pass and lifecycle-valid final verification gate the outer commit/push.
- **D-20:** Roadmap UI hint is protocol/header/operator terminology. No frontend/browser/UI-SPEC contract applies; this phase changes the existing headless config/protocol surface. Broader status/CLI/dashboard/support index projections remain Phase161. Record only minimal shared truthful transport evidence necessary to prove this phase.


### Agent Discretion

Research/planning may choose exact config field names within the pinned/general-serving vocabulary, typed capabilities, codec/module boundaries, measured finite budgets, transport receipt implementation, fixtures and plan decomposition. Preserve existing dependencies/crates and pure-core versus effectful-shell layering. Preserve the explicit/all/implicit baseline distinction captured above; do not expand BIP37/V0, whitelist compatibility or public defaults.

### Deferred Ideas (OUT OF SCOPE)

Broad operator/status/dashboard/support rollout (Phase161); complete production-startup/prune/reopen/authenticated-RPC/all-peer-family integrated client proof (Phase162). V0/type2, BIP37, archive-scale claims, filter import, wallet acceleration, automatic repair/download, public defaults/network CI, production service/readiness and funds safety remain excluded. No pending todos matched Phase160.
</user-constraints>

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| -- | -- | -- |
| CFNET-01 | An operator can explicitly enable general BASIC serving or the existing per-peer `blockfilters` permission only with the BASIC index prerequisite; `NODE_COMPACT_FILTERS` follows configured capability independently of catch-up completion, coexists truthfully with `NODE_NETWORK_LIMITED`, and does not implicitly activate listeners or public defaults. | Explicit/all provenance matrix, config refusal before runtime open, shared baseline service composition and observed handshake tests. |
| CFNET-02 | A peer can receive ordered `cfilter` responses for an entirely indexed legal range of at most 1,000 blocks, selected by the requested stop block's ancestry rather than only active heights, including during catch-up. | Dedicated immediate complete-range preparation; one guarded persisted lookup boundary; branch ancestry and ordered frames. |
| CFNET-03 | A peer can receive `cfheaders` with the correct preceding header and ordered filter hashes for an entirely indexed legal range of at most 2,000 blocks, including genesis and permitted stale branches. | Correct previous header, raw digest serialization, strict inclusive limit and stale authorization. |
| CFNET-04 | A peer can receive `cfcheckpt` headers at every positive 1,000-block interval through its permitted stop block, with correct genesis exclusion and branch ancestry. | floor(stop/1000) vector; work/byte admission distinct from contiguous range cap. |
| CFNET-05 | Invalid compact-filter requests receive the pinned Knots disconnect/validation behavior, while absent records in an otherwise valid range never produce invented empty filters or a shortened successful response. | Ordered semantic gates; short/trailing payload nuance; complete-before-send absence/corruption controls. |
| CFNET-06 | Peer serving obeys explicit work, byte and queue bounds on real inbound and outbound runtime paths, reads persisted filters instead of regenerating on request, and earns response evidence only from achieved transport writes, including partial-write failure. | Request-scoped reservation/affine receipts; real inbound/outbound integration; measured complete accounting and abort-suffix executor tests. |

Descriptions are verbatim REQUIREMENTS.md; support entries are research recommendations derived from the sources below. [VERIFIED: REQUIREMENTS.md CFNET-01–06; 160-CONTEXT.md]
</phase-requirements>

## Project Constraints (from AGENTS.md)

- Preserve pinned Knots `29.3.knots20260210` externally observable in-scope behavior; record intentional differences in docs/parity/index.json and companion docs. [VERIFIED: AGENTS.md]
- Preserve functional core/imperative shell: pure wire/range/proof/resource policy in existing first-party core/network crates; storage, mutexes, sockets and runtime effects in node/RPC adapters. No production Rust Bitcoin library, new crate or dependency is needed or authorized. [VERIFIED: AGENTS.md; 160-CONTEXT.md discretion; parent scope]
- Use Rust 1.94.1/2024 and the existing Bazel/Bzlmod build; use Bun for substantive repo automation, no package-install bootstrap. Cargo/Bazel commands must run through command-timings.ts, serially against the target directory. [VERIFIED: AGENTS.md; rust-toolchain.toml; packages/Cargo.toml]
- Run the full default bash scripts/verify.sh contract; fast mode is iteration only. Preserve hooks, regenerate tracked docs/metrics/lines-of-code.md, register every new first-party Rust source/test parity breadcrumb. Review relevant README/UAT and provide copy-pasteable repo-local Cargo/Bazel commands. [VERIFIED: AGENTS.md]
- Keep errors visible; use typed fallible boundaries, early returns, maybe_ option names, no unwrap, pure focused Arrange/Act/Assert tests. Split oversized modules rather than extending already crowded entrypoints. [VERIFIED: supplied global guidance; standards/core/{architecture,code-shape,testing}.md; standards/languages/rust.md]
- GSD workflow is active; outer wrapper owns all commits/pushes only after clean phase verification. Researcher owns only this file. No project .claude/skills or .agents/skills directory was found; standards-overrides.md contains placeholders only. Parent loaded active lessons and synchronized origin/main. [VERIFIED: filesystem probes; standards-overrides.md; parent task]
- Relevant local sources loaded: AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md, architecture/code-shape/verification/testing/Rust pages. Config explicitly disables Nyquist validation, so no Validation Architecture section is emitted. Security is enabled by absence of false. [VERIFIED: inspected instructions; .planning/config.json]

## Summary

Use a distinct typed filter-serving request flow under the existing ManagedNetworkHandle, then transport immutable prepared responses outside both authority and RPC context locks. Reuse the existing Fjall bounded point-read integrity machinery; do not loop the RPC readiness API, regenerate filters, or create another history owner. The existing RPC query deliberately returns Pending after initial synchronization when the accepted frontier outruns processing; peer requests instead need immediate already-indexed completeness. [VERIFIED: node/network/runtime_authority/filter_index/query.rs query_network; 160-CONTEXT.md D-10–13]

The largest integration traps are concrete: the network enum has no compact-filter variants or named service bit; inbound permission effects discard requested-token provenance; prune service updates overwrite the entire service set; outbound sync receives but drops inbound_response_plan; ordinary inbound response failures drop the remaining vector. Existing affine announcement executors provide the correct achieved-prefix/abort-suffix shape, but their message kinds, mempool lifecycle binding and global 128-effect ledger are unsuitable unchanged for a 1,000-filter request. [VERIFIED: network/message.rs; inbound/permissions.rs; node/network/limited_serve.rs; node/sync/session.rs:399–415; rpc/inbound_listener/connection_runtime.rs; node/network/announcement_transport.rs; lifecycle_effects.rs]

**Primary recommendation:** Implement protocol/capability policy → exact branch authorization → complete bounded persisted preparation → request-scoped receipt/reservation → both actual socket executors, with genuine accepted-chain and forced-write-failure proof at each integration seam. [VERIFIED: 160-CONTEXT.md; inspected runtime/transport paths]

## Standard Stack

### Core

| Component | Version/pin | Purpose / instruction |
| -- | -- | -- |
| Rust/std | 1.94.1; 2024 edition | Existing pure policy, checked arithmetic, owned/affine tokens and sync outbound sockets. [VERIFIED: rustc probe; rust-toolchain.toml; workspace manifest] |
| First-party network/codec/consensus/node/RPC crates | workspace 0.1.0 | Extend existing wire model and adapter seams; no new production crates. [VERIFIED: packages/Cargo.toml; parent scope] |
| Fjall | 3.1.10, git aa30dca811399a201e0b9595da93a4582dcb2b57 | Existing authoritative immutable filter records and publication integrity guard. [VERIFIED: packages/Cargo.lock; storage/fjall_store/filters/query.rs] |
| Tokio | 1.52.1 | Existing asynchronous inbound listener transport/context lock. [VERIFIED: packages/Cargo.lock; inbound_listener/connection_runtime.rs] |
| serde / serde_json | 1.0.228 / 1.0.149 | Existing configuration representation only; peer protocol remains binary. [VERIFIED: packages/Cargo.lock; rpc/config.rs] |
| Bitcoin Knots | 29.3.knots20260210, a9aee730466ac67d35a3c03ee24676be5e045878 | Behavioral authority and independent test helpers. [VERIFIED: submodule HEAD; source handlers] |

### Supporting

| Component | Pin | Use |
| -- | -- | -- |
| Bun | .bun-version 1.3.9 | Deterministic oracle/check scripts and timed commands. PATH currently resolves 1.4.2; verified 1.3.9 fallback exists. [VERIFIED: probes] |
| Bazel | .bazelversion 8.6.0 | Existing full verifier smoke build. [VERIFIED: .bazelversion; AGENTS.md] |
| Pinned Python messages.py | same Knots revision | Independent expected wire bytes through existing narrow test-oracle pattern; orchestration stays Bun. [VERIFIED: Knots test_framework/messages.py:1736–1884; AGENTS.md compatibility exception] |

**Installation/version policy:** No dependency installation or upgrade. These are verified checkout pins, not latest-registry recommendations; npm registry version/publish checks do not apply to this Rust-only extension. [VERIFIED: parent scope; packages/Cargo.lock; no package.json]

### Alternatives Considered

| Instead of | Could use | Decision |
| -- | -- | -- |
| Immediate batch preparation | Existing singleton basic_filter_query loop | Reject: separate lock snapshots can mix branches and API may wait for readiness; reuse guarded reader, not RPC wait semantics. [VERIFIED: query.rs] |
| Request-scoped capability | One PeerEmission per filter | Reject unchanged: generic pending ledger 128, unsupported CF kinds and mempool lifecycle binding. [VERIFIED: announcement_transport.rs; lifecycle_effects.rs] |
| Header-derived exact proof facts | Existing chain_work field or elapsed height proxy | Reject: HeaderStore score is constant 1 and u128; Knots uses exact target-derived proof. [VERIFIED: header_store.rs header_work_score; Knots chain.cpp] |

## Architecture Patterns

### Recommended file seams

These are proposed modules and ownership boundaries, not claims that the new APIs exist. [VERIFIED: existing files and 160-CONTEXT.md discretion]

| Responsibility | Existing seam | Recommended change |
| -- | -- | -- |
| Wire types/codec | network/src/message.rs; message/cursor.rs; peer/message_dispatch.rs | Add six families in dedicated message/filter.rs plus tests; add COMPACT_FILTERS = 1<<6; preserve unknown raw type until semantic gate. |
| Pure range/capability/resource policy | network/src/inbound/permissions.rs; network/src/header_store.rs | Add requested-token accessor and typed per-peer capability; separate filter_serving modules for ordered checks/proof-age/resource arithmetic. |
| Startup config | rpc/src/config/{blockfilter.rs,loader/blockfilter.rs,loader.rs}; bin/open-bitcoind.rs | Narrow peerblockfilters CLI/bitcoin.conf resolution and early index prerequisite; publish capability to opened authority before any handshake. |
| Service composition | node/src/network/limited_serve.rs; network/src/peer.rs and peer/message_dispatch.rs | Compose prune baseline plus general filter bit; apply per-peer permission overlay at both version sends, snapshot own advertised capability for request validation. |
| Durable read preparation | node/src/network/runtime_authority/filter_index/{query.rs,new serving.rs}; storage/fjall_store/filters/query.rs | One authority turn and publication guard; immediate bounded batch reads; bind exact branch/stop/peer session/generation and reserve output before returning owned frames. |
| Filter receipt ledger | node/src/network/{new filter_transport.rs,runtime_authority/*}; existing lifecycle_effects patterns | Private, non-Clone request ticket with next sequence, reservation and completed-prefix accounting; no mempool generation dependency. |
| Actual inbound caller | rpc/src/context.rs; context/inbound_wire.rs; inbound_listener/connection_runtime.rs | Consume new filter response plan and execute outside locks; terminal helper explicitly aborts every suffix reservation. |
| Actual outbound caller | node/src/sync/session.rs; sync/tcp.rs; sync/session/emission_terminal.rs | Explicitly consume response plan after receive_sync_message; transport all three families via real send; receipts after complete framed write only. |

### Pattern 1: Preserve configured, permissioned and backend states separately

Use the following matrix for startup and actual version messages. Explicitly requested means requested_tokens contains BlockFilters, not expanded permissions contains it; combining all and explicitly named blockfilters still refuses without BASIC. Implicit Knots whitelist expansion adds mempool/noban/addr and optional relay flags, never blockfilters. [VERIFIED: Knots init.cpp:1097–1104,2358–2363; net_permissions.h:48–55; net_permissions.cpp; net.cpp:606–613]

| Configuration | BASIC disabled | BASIC enabled | Own advertised CF capability |
| -- | -- | -- | -- |
| Index only / no general serving / ordinary peer | Accepted | Accepted | False |
| Explicit peerblockfilters | Refuse before networking/startup effects | Accepted | General true for both directions |
| Explicitly named blockfilters | Refuse even combined with all | Accepted | True only for matching peer |
| all alone | Accepted; no backend; legal request yields no normal response | Accepted, finite bounds | True only for matching peer |
| implicit / noban / download / protected only | Accepted | Accepted | False unless general serving independently enabled |

Matrix follows pinned permission/startup behavior and locked D-01–04. No per-peer capability may set the global service bit or start a listener. [VERIFIED: Knots InitializeNode; 160-CONTEXT.md]

Existing ParsedPeerPermissionClass explicitly rejects outbound direction and PeerPermissionClassRegistry only exposes resolve_inbound. Do not describe outbound permission configuration as already supported. General serving must work for outbound production callers. Preserve existing class compatibility scope; carry a direction-aware capability through peer state, and make any narrowly expanded permission adapter explicit in plans rather than silently widening every permission. Current InboundPermissionDecision carries effect labels, not the original PeerPermissionSet, so the filter-specific decision/provenance must cross admission as a typed fact. [VERIFIED: network/inbound/permissions.rs:ParsedPeerPermissionClass,validate_inbound_class_permissions,inbound_decision; peer.rs:add_outbound_peer]

Compose services centrally: existing set_prune_mode resets both node local_config and PeerManager local_config to advertised_service_flags(mode). Add general capability without losing NETWORK_LIMITED/WITNESS; both setters and both handshake paths must consume the same composition. Capture per-peer own advertised services, not remote version.services or current global backend state, for request gate 1. [VERIFIED: node/network/limited_serve.rs:24–28; network/peer.rs:475–489; peer/message_dispatch.rs:handle_version; Knots PrepareBlockFilterRequest]

### Pattern 2: Ordered wire semantics, including malformed requests

Pinned semantic gate order is exactly: BASIC plus peer.m_our_services CF bit → known stop AND BlockRequestAllowed → start<=stop → stop-start<family maximum → GetBlockFilterIndex. First four failures disconnect; absent backend after valid gates returns without response. Lookup range/header failure also returns without a normal response. A checkpoint uses start=0 and UINT32_MAX maximum, not 1000/2000. [VERIFIED: Knots net_processing.cpp:3156–3316]

Short **complete framed payloads** throw during DataStream extraction and the enclosing ProcessMessages catches/logs exceptions without directly setting fDisconnect. The three handlers do not enforce end-of-payload, so trailing bytes after valid fields are ignored at this handler boundary. Distinguish these from broken framing, stream EOF or an oversized envelope, which are transport-level concerns. Add request-specific parse outcomes/caller handling preserving pinned semantics; if generic strict Cursor::finish is retained, record strict trailing-data refusal as an intentional parity difference with explicit test and docs rather than asserting Knots does it. [VERIFIED: Knots ProcessMessages:5042–5058; ProcessGetCF handlers; network/message/cursor.rs finish; 160-CONTEXT.md D-05–06]

The request type must retain u8 filter_type so type2/unknown reaches the ordered unsupported-type gate; do not parse unsupported types into a generic codec error before the peer policy. Keep start and stop as u32/raw BlockHash; compute stop-start only after reverse check. [VERIFIED: Knots handler extraction/PrepareBlockFilterRequest; network Cursor methods]

### Pattern 3: Exact branch authorization without body dependency

Knots BlockRequestAllowed permits active membership immediately. A stale stop requires SCRIPTS-valid provenance, a best header, signed best_header.time-stop.time <2,592,000 seconds, and signed proof-equivalent age <2,592,000 seconds. The strict inequality, signed future case, tip-based denominator and both tests matter. Present body/undo, filter existence or active-height projection are not substitutes; pruned bodies do not enter this filter eligibility test. [VERIFIED: Knots net_processing.cpp:1865–1871; chain.cpp:131–161]

Use Phase159 genuine validation provenance for stale eligibility: only ScriptsValid authorizes that branch; UnknownLegacy/NeverConnected cannot be promoted from filter rows or snapshots. Traverse stop-parent identity with checked decreasing heights. Resolve both range and checkpoint locations from that stop; never use active_key(height) for a stale request. [VERIFIED: node/chainstate/validation_history.rs; storage/validation_history.rs; 160-CONTEXT.md D-07,D-10]

**Critical implementation gap:** HeaderStore.chain_work is u128 and header_work_score returns 1; ancestor_at_height is a linear parent walk, and best_tip uses the simplified score. No inspected existing exact GetBlockProofEquivalentTime helper was found. Do not reuse simplified work as baseline evidence. Add a narrow exact proof-work helper in the existing consensus/network layer using compact_target_bytes and fixed limbs; preserve Knots arithmetic/sign/saturation with pinned vectors. Maintain exact serving header facts/best-proof-header identity incrementally at trusted header admission and reconstruct during existing recovery, or bound/cancel common ancestry when computing differences. Avoid per-request whole-history recomputation and avoid replacing unrelated chainstate scoring as incidental scope. The planner must choose and own the exact metadata/helper seam. [VERIFIED: network/header_store.rs; consensus/crypto.rs compact_target_bytes; block/difficulty.rs existing fixed-limb patterns; Knots chain.cpp; parent scope]

Exact reference formula: proof=(~target/(target+1))+1; signed elapsed=floor(abs(to_work-from_work)*target_spacing/proof(best_header)), saturated to signed INT64_MAX when result exceeds63 bits. C++ uses 256-bit arithmetic; implement and test its bounded arithmetic behavior, not floats, u128 saturation, block-count times600 or best active tip time. [VERIFIED: Knots chain.cpp:131–161; arith_uint256.h]

### Pattern 4: Complete persisted preparation under one authority

Extend the erased query backend boundary already exposed by ChainstateStore.maybe_basic_filter_query_store. Within one authority turn, resolve eligibility/branch, enter one with_basic_filter_query publication guard, compare enabled/generation/read-integrity state, and prepare every required record/header before returning any wire output. Reuse BasicFilterQueryReader.maybe_record and BasicFilterRecordView.identity/encoded_bytes/work, preserving target+parent commitment validation and known header identity agreement. Do not loop basic_filter_query through separate authority acquisitions or await its RPC readiness barrier. [VERIFIED: runtime_authority/filter_index/query.rs; storage/fjall_store/filters/query.rs; chainstate store adapter]

For cfilters, stage ordered exact bytes for every inclusive record. For cfheaders, read the start-1 predecessor unless start0 and then ordered record.filter_hash values; previous header is zero only at start0. For cfcheckpt, count=floor(stop_height/1000), read branch headers1000,2000,...; below1000 is a valid empty vector. Missing target means no normal response; missing predecessor inside an existing record or commitment mismatch is corruption, not ordinary missing. Preserve separate Disabled/Missing/ResourceRefused/StorageFailure/Invalidated outcomes internally; none becomes an empty cfilter or shortened success. [VERIFIED: Knots ProcessGetCF*; guarded point reader missing parent error; 160-CONTEXT.md D-08–12]

Immutable prepared bytes may outlive the read guard, but live completion/reservation authority must bind authority incarnation, index generation, captured branch/stop identity and peer session generation. Check validity before every write and upon completion; lifecycle invalidation aborts unwritten work without inventing full-range success. Record physical achieved-prefix evidence separately from live successful completion if a lifecycle changes during a successful write. [VERIFIED: 160-CONTEXT.md D-13–16; existing affine emission freshness/completion patterns]

### Pattern 5: Measured finite resource model

Every request must preflight checked counts/byte formulas and reserve a bounded peer/aggregate slot before materialization. Charge actual target+parent reads, lifecycle/provenance metadata reads, ancestry links and proof operations, parsed/hash bytes, logical copies, framed output and queued suffix bytes. Existing BasicFilterQueryWork counts one/two record reads and SHA input/padded copies; its response_hex_bytes is an RPC projection and must not govern raw wire admission. [VERIFIED: storage/fjall_store/filters/query.rs work/charge_hash_work; 160-CONTEXT.md D-12]

| Family | Wire accounting (excluding no extra fields) | Work that still needs charging |
| -- | -- | -- |
| cfilter | Per frame24 + type1 + hash32 + CompactSize(n) + n | All range ancestry/record+parent validation; total series bytes; copies and duplicated encoder buffers. |
| cfheaders | 24+1+32+32+CompactSize(k)+32*k; k<=2000 | Complete record validation can read large bytes even though response is small; predecessor. |
| cfcheckpt | 24+1+32+CompactSize(floor(h/1000))+32*floor(h/1000) | Sparse outputs still need stop-to-checkpoint ancestry and each target+parent integrity. |

Formulas follow local framing and pinned message serializers; BIP157 independently specifies the fields. [VERIFIED: network/message.rs encode_wire; Knots messages.py:1736–1884; CITED: https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki]

**Planning choice:** Use named policy fields with one active preparation/send request per peer and a bounded aggregate request count; one request reservation owns the whole suffix, with next-message cursor and actual retained bytes, rather than one generic lifecycle ledger entry per cfilter. Select byte/work defaults from concrete measured fixtures and make the selection an execution acceptance gate. Required fixtures: 1/20/400/1000 cfilters, 2000 cfheaders, checkpoint stop>2000 plus large sparse ancestry, recent stale fork, varying-difficulty proof, maximum admitted wire singleton and one-over budget. Record cold/warm guarded hold and total duration, structural work and peak logical retained copies; observed timings are not latency guarantees. Bounds/refusal are explicit local resource policy in parity docs, not protocol-invalid ranges or a checkpoint-height cap. [VERIFIED: 160-CONTEXT.md D-12; existing phase159 query measurement pattern; lifecycle ledger128 cap]

The index accepts up to32 MiB encoded filter data; Phase159 intentionally admitted this codec capacity without claiming it is consensus-generated. Such a stored singleton exceeds the current outbound32 MiB payload envelope after CF metadata, and Knots receive envelope is4,000,000 bytes. Establish the filter-serving frame limit explicitly and distinguish generic storage capacity from wire-admissible response bytes; do not shrink storage or claim all codec-capacity fixtures are valid production peer payloads. Count refusal before response encoding/allocation and prove genuinely generated legal large filters still fit. [VERIFIED: codec/compact_size.rs MAX_SIZE; phase159 159-04-SUMMARY.md; node/sync/tcp.rs:97; Knots net.h MAX_PROTOCOL_MESSAGE_LENGTH and net.cpp:789–790]

### Pattern 6: Both physical transports, affine receipts and cleanup

| Path | Exact existing seam | Required behavior |
| -- | -- | -- |
| Inbound prepare | rpc/context.rs:385–420; context/inbound_wire.rs:258–301 | Prepare via shared authority; return owned filter plan; release context/authority before I/O. |
| Inbound write | inbound_listener/connection_runtime.rs:212–267; resource_runtime.rs:390–421 | Only Written after full frame earns message/bytes; timeout/error/partial frame aborts current and suffix. |
| Outbound prepare | node/sync/session.rs:399–415 | Currently consumes only sync_result.outbound; explicitly route new filter plan instead of dropping inbound_response_plan. |
| Outbound write | node/sync/tcp.rs:74–88; session.rs:556–583 | write_all success is complete frame; failed partial write earns zero for that frame and closes/aborts remaining series. |
| Cleanup template | node/sync/session/emission_terminal.rs; rpc connection_runtime.rs execute_inbound_emissions | Validate target, complete once, preserve written prefix, abort all remaining capabilities on encoding/rejection/disconnect/completion failure. |

These seams and behavior were independently traced by the transport explorer. Generic acknowledge_wire_message_written currently records block-relay evidence only; it cannot prove CF request completion. Add filter-specific receipt finalization with request identity and expected frame sequence/length; private non-Clone tokens prevent caller-forged/replayed completion. A passing enqueue/mock is insufficient. [VERIFIED: explorer source trace; node/network/block_relay_evidence.rs:373–375; announcement_transport.rs affine types]

Do not reuse generic PeerEffectCapability freshness unchanged: it binds mempool generation. Filter receipt freshness must use index/chain/peer authority and must release reservation even when stale. Ensure normal return, early error, cancellation/shutdown and dropped plan have explicit cleanup (RAII or a checked terminal owner); a failed current frame cannot leave the unsent suffix in the128-entry generic effect map. [VERIFIED: runtime_authority/lifecycle.rs:301–304; lifecycle_effects.rs; 160-CONTEXT.md D-15–16]

### Anti-Patterns to Avoid

- Global bit from a permissioned peer, bit from synced latch, or later prune setter erasing the bit. [VERIFIED: pinned InitializeNode/init.cpp; node limited_serve.rs]
- Timestamp-only stale permission, constant/u128 work or current-height projection selecting stale records. [VERIFIED: pinned BlockRequestAllowed; HeaderStore limitations]
- Sending cfilter1 before validating filter1000; silent filter omission or fabricated zero/empty data. [VERIFIED: pinned LookupFilterRange before send; D-11]
- Revalidating full stored ancestry on every point read or holding authority while awaiting backpressure. [VERIFIED: current guarded point-reader design; D-12–13]
- Crediting successful preparation, queue insertion or a partly written frame; retry double credit. [VERIFIED: achieved-write pattern; D-14–16]

## Don't Hand-Roll

| Problem | Do not build | Use instead |
| -- | -- | -- |
| BASIC generation / hashes | Request-time block/current-coins generator | Existing persisted BasicFilterRecordView and project commitment verification. [VERIFIED: storage reader; D-10] |
| Binary primitive codec | New endian/hash/display conversions or ad hoc varints | Existing Cursor/read_compact_size/write_compact_size, raw hash bytes and canonical codec tests. [VERIFIED: message/cursor.rs; codec exports] |
| Validation provenance | Public boolean or infer from body/filter/active height | Genuine Phase159 acceptance ledger and exact ScriptsValid provenance. [VERIFIED: chainstate/validation_history.rs] |
| Runtime ownership | Independent reader/cache/worker | Existing ManagedNetworkHandle and publication guard. [VERIFIED: runtime_authority.rs; D-10,D-13] |
| Completion accounting | Counter increments on enqueue | Existing affine receipt/executor architecture with new filter-specific request semantics. [VERIFIED: announcement_transport.rs; emission_terminal.rs] |

**Key insight:** New policy/proof helpers are necessary where the exact baseline is missing, but storage integrity, validation evidence, serialization primitives and actual transport ownership already have authorities; reuse them rather than introduce competing ones. [VERIFIED: inspected modules above]

## Common Pitfalls

### Gate ordering erased by decoding or disabled-backend shortcuts

Unsupported types and unadvertised capability must win before stop/range/backend checks. all-without-backend still disconnects invalid requests, while valid requests return without response. Keep raw filter type through decoding and do not return Disabled before semantic validation. [VERIFIED: Knots PrepareBlockFilterRequest]

### Checkpoint output mistaken for contiguous range

floor(stop/1000) is the number of headers, not a1000/2000 stop-height maximum. A tiny checkpoint response may still require many parent links. Use ancestry/work refusal with explicit policy, not an oversized-range protocol disconnect. [VERIFIED: Knots ProcessGetCFCheckPt; D-09,D-12]

### Integrity “optimization” bypasses validated bytes

Even cfheaders/checkpoints must preserve existing read-integrity and commitment guarantees. Reading header fields with parse_record_fields alone would skip encoded-filter/hash checks provided by parse_record. Optimize repeated checks only through a proof-preserving guarded batch API and count all remaining work. [VERIFIED: storage/filter_index.rs parse_record versus parse_record_fields; reader query.rs]

### Lifecycle invalidation loses achieved prefix or leaks reservation

A frame may physically finish while reorg/disable invalidates live authority. Separate truthful achieved frames from complete current-range success, terminate suffix and release all slots/bytes. Reject duplicate/foreign completion. [VERIFIED: D-13–16; affine emission cleanup contracts]

### Only one production direction tested

Outbound receive currently drops response plans, and inbound ordinary failures do not explicitly abort the suffix. Prove each family via real inbound and outbound caller, not a standalone helper. [VERIFIED: source seams; explorer findings; D-17–18]

## Code Examples

Existing verified point-read operations expose immutable bytes and commitments; proposed batch glue should remain inside the existing authority/publication boundary. [VERIFIED: query.rs BasicFilterRecordView methods]

```rust
// Existing API: node/storage/fjall_store/filters/query.rs
let identity = record.identity();
let raw_filter = record.encoded_bytes();
let header = record.filter_header();
let measured = record.work();
// New peer glue must preserve identity.height/parent/hash agreement,
// charge measured work and frame expansion before taking ownership.
```


Independent pinned serializer pattern for the two request shapes (uses existing Python helper in test-only oracle; production stays Rust). [VERIFIED: Knots messages.py:1736–1760,1839–1860]

```python
# Source: pinned test/functional/test_framework/messages.py
getcfilters_payload = (
    filter_type.to_bytes(1, "little")
    + start_height.to_bytes(4, "little")
    + ser_uint256(stop_hash)
)
getcfcheckpt_payload = filter_type.to_bytes(1, "little") + ser_uint256(stop_hash)
```


Existing successful-prefix terminal shape is the integration reference: validate target → encode/write complete frame → consume completion capability; any failed frame aborts it and every remaining emission. Filter request executor must additionally bind sequence and finish a range only at expected total. [VERIFIED: node/sync/session/emission_terminal.rs:22–53,104–149; rpc execute_inbound_emissions]

## Evidence and Test Strategy

Nyquist is explicitly false, so this is implementation evidence guidance rather than a generated Validation Architecture. Tests should use existing cargo framework and deterministic loopback/fault seams. [VERIFIED: config.json; existing tests]

| Area | Required evidence | Existing fixture/seam |
| -- | -- | -- |
| Config + CFNET-01 | General/explicit/all/all+explicit/implicit matrix with BASIC disabled/enabled; reject before startup effects; actual inbound/outbound version before catch-up; coexist NETWORK_LIMITED; index-only no capability/listener | rpc/config/tests; bin/open_bitcoind/tests/filter_index; network permission tests; limited-serve tests |
| Wire + CFNET-02–05 | Pinned exact payload/frame bytes; raw digest order; genesis/predecessor;1000/1001 and2000/2001; checkpoint0/999/1000/1001/2000/3001; unsupported/V0 plus unknown stop/reversed combined precedence | messages.py; network/message/tests; Knots p2p_blockfilters.py |
| Branch + CFNET-02–04 | Continuous genuinely validated spending chain and accepted fork; correct retained stale branch bytes; header-only/UnknownLegacy/too-old timestamp/too-old proof; strict30d boundaries; nonuniform bits and future timestamp | node/sync/tests/filter_index/reorg/fixtures.rs; phase159 validation history; rpc daemon fixtures |
| Completeness + CFNET-05 | Catch-up complete prefix succeeds while synced=false; crossing missing row yields zero messages; cfheaders predecessor/checkpoint absent; target/parent corruption and backend fault distinct | guarded QueryRecordFault, BeforeQueryRead; phase159 catch_up/rpc_faults |
| Transport + CFNET-06 | All families over both actual paths; forced second cfilter frame failure gives one prefix/no complete range; partial first frame earns zero; queue/timeouts/cancel/disconnect/duplicate/stale/foreign tickets release suffix | node/sync/tests/production_announcement_transport_cases.rs; rpc/inbound_listener/tests/announcement_successful_prefix.rs |
| Resource + CFNET-06 | Exact admission and one-over for request/peer/aggregate bytes/work; metadata and ancestry charges; permissioned/protected peers bounded; no generation/body/undo reads; locks free during blocked write | query measurement/fault hooks; real loopback/injected production transport |

Sources in the final column exist; new Phase160 cases remain implementation work, not achieved evidence. Reuse genuine accepted continuous histories and durable coins/Fjall reopen; raw seeded snapshots can be explicit negative/legacy controls only. Avoid cfg(test)-private cross-crate fixture dependencies. [VERIFIED: phase159 159-07-SUMMARY.md; node/rpc cited fixtures]

Proposed focused commands use future phase160 test names, then the unchanged native gate. Cargo/Bazel remain serialized by command-timings. [VERIFIED: AGENTS.md; existing package names]

```bash
bun run scripts/command-timings.ts run --key phase160-network -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase160 -- --nocapture
bun run scripts/command-timings.ts run --key phase160-node -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase160 -- --nocapture
bun run scripts/command-timings.ts run --key phase160-rpc -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc phase160 -- --nocapture
bun scripts/bright-builds-check.ts all
bash scripts/verify.sh
```


## State of the Art

| Current checkout | Required Phase160 behavior | Evidence |
| -- | -- | -- |
| blockfilters is inactive label | Conditional per-peer CF capability, preserving all provenance | permissions.rs; pinned init/net_permissions |
| No BIP157 wire variants | Six typed families and exact bytes | message.rs; messages.py |
| Singleton RPC readiness/point query | Immediate complete bounded peer batch | runtime query.rs; pinned ProcessGetCF* |
| Simplified header score | Exact stale-serving proof-age facts | header_store.rs; Knots chain.cpp |
| Block/tx affine write receipts | Filter request sequence/range receipts on both callers | announcement_transport.rs; session/inbound executor |

No third-party migration or dependency deprecation is part of this phase. [VERIFIED: parent scope; locked discretion]

## Runtime State Inventory

This is an additive serving extension, not a rename/migration, but it touches existing runtime state; the inventory prevents accidental startup or durable-owner changes. [VERIFIED: scope and inspected adapters]

| Category | Items found | Action required |
| -- | -- | -- |
| Stored data | Existing BASIC records/lifecycle, validation-history ledger and header/chain metadata | Reuse unchanged durable authority; no filter-record migration. Exact proof facts may be reconstructed from trusted header bits; any persisted schema change needs explicit plan/evidence. [VERIFIED: filter/validation codecs; recommendation above] |
| Live service config | Repo-local daemon CLI/bitcoin.conf/JSONC surfaces; no external service inspected or required | Add narrow explicit serving config; preserve default off; no claim of modifying external running daemon configuration. [VERIFIED: config loader; D-01] |
| OS-registered state | No service-manager or OS-registration changes in selected scope | None for research/phase; do not activate listener/service implicitly. [VERIFIED: CONTEXT.md scope] |
| Secrets/env vars | No new secret/env contract in BASIC peer serving | None; reuse existing runtime launch environment. [VERIFIED: selected config scope] |
| Build artifacts | Existing Cargo/Bazel products and tracked LOC report | Native build/test rebuilds; refresh LOC through existing hook/verifier. [VERIFIED: AGENTS.md] |

## Assumptions Log

No training-only factual claim is promoted to a decision. All factual findings are source-tagged; proposed APIs, exact proof metadata shape and numeric defaults are explicitly recommendations pending implementation evidence. No fabricated performance measurements or consensus-generated maximum filter size are claimed. [VERIFIED: sources and proposal labels above]

## Open Questions (RESOLVED)

1. **Exact stale proof facts:** Select the narrow fixed-limb proof helper plus incremental/recovered best-proof-header metadata seam. Current u128/constant score is insufficient. Require independent vectors and changing-difficulty/stale boundary tests; do not leave this as a timestamp-only fallback. [VERIFIED: inspected header_store/consensus; pinned chain.cpp]
2. **Numeric resource defaults:** Select finite request/peer/aggregate fields now, with measured final values and exact/one-over controls as an execution gate; document wire singleton capacity separately from index codec capacity. This is an acceptance task, not permission to omit bounds until later. [VERIFIED: D-12; phase159 measurement source]
3. **Short/trailing payload policy:** Prefer exact pinned handler behavior; if broader strict decoder constraints force a difference, explicitly document/test it and preserve semantic precedence. No normal response to truncation, no invented protocol disconnect claim. [VERIFIED: pinned exception/handler source]
4. **Directional permissions:** General serving must reach both directions; explicitly own any minimum outward capability plumbing because current class parser rejects out. Do not expand all unrelated outbound permissions as incidental work. [VERIFIED: permissions.rs; D-04]
5. **Achieved-but-invalidated completion:** Choose a typed terminal outcome separating physical prefix evidence, current complete-range success and released reservations. Require lifecycle-change-during-write proof. [VERIFIED: D-13–16; affine transport source]

These design selections are resolved by executable plans; implementation measurements and verification remain required evidence, not achieved results.

1. **RESOLVED — exact proof:** Plan160-04 owns private fixed256-bit ProofWork and pinned signed equivalent-time arithmetic; Plan160-05 attaches derived exact cumulative work/best-proof-header facts to existing HeaderStore admission/recovery. No persisted schema/cache or global synthetic fork-choice rewrite; no per-request full-history recomputation.
2. **RESOLVED — resources:** Plan160-06 fixes payload4000000; suffix8MiB/request; peer1 and aggregate4 requests/32MiB suffix; ancestry8192,proof/header8192,record reads4096,metadata256; envelope64MiB,hash-padded128MiB,logical-copy64MiB/request; active frame4000024/peer and8MiB combined aggregate. Metadata was adjusted from128 to256 after Plan05 established conservative stale138 (sealed128 provenance identities plus10 bookkeeping), versus active6. Actual guarded-path structural counters and exact/one-over fixtures validate these before transport activation; bounded policy revisions require paired test/docs changes. Legal1000/2000 and checkpoint3001 success is mandatory; larger checkpoint ancestry may be a documented local refusal, never an invented protocol height cap.
3. **RESOLVED — malformed payloads:** Plan160-01 preserves pinned short-complete extraction as nonfatal no-response and ignores valid-field trailing bytes only for the three CF request shapes. Framing/EOF/checksum remain separate existing transport failures.
4. **RESOLVED — direction:** Plan160-02 retains the current outbound permission-class grammar rejection; a single typed capability and own-advertised-services snapshot serves both directions, with configured general serving supplying actual outbound capability. Plans160-03/09/10 wire real startup/handshakes/transports without widening unrelated whitelist permissions.
5. **RESOLVED — invalidated writes:** Plan160-07 owns private one-use request/session/sequence/length-bound receipts, physical achieved-prefix evidence separate from live complete-range success, and drop/cancel-safe suffix release. Cancellation drops actual queued Vec suffix buffers; already-handed frame bytes and the peer/request slot stay charged until completion/drop,with late writes physical-only. Plans160-09/10 prove these on both production executors and real loopback paths; Plan160-11 supplies independent concrete evidence.
6. **RESOLVED — publication epoch supplement:** Execution exposed a same-generation integrity ABA regression (recovery revived captured token0). Plan160-13 now exclusively owns six existing store files for an opaque checked shared monotonic epoch,raw put/delete,both recovery invalidations and pre-write disable. Overflow permanently fails closed; clones share invalidation; trusted append preserves captures; currentness uses no backend I/O. Plan160-07 depends on13 and consumes the epoch contract without store edits. This split preserves D-13/D-16/CFNET-06 at full fidelity; T-160-37 registers the concrete ABA/overflow trust boundary.

7. **RESOLVED — mixed pending governance supplement:** Concrete inspection found inbound aggregate queue values are connection-local,outbound Tcp.send encodes+writes without outgoing byte accounting,and outboxes bound count only. Plan160-14extends the SAME Plan07FilterTransportOwner.registry with affine generic retained/scratch guards and internal atomic CFactive admission,removing caller snapshots. Combined8MiB applies in both admission orders; generic descriptors128/peer1024aggregate bound maps. Inbound keeps its preexisting1MiB/128 local checks; outbound receives an explicit NEW local4000000payload+24 envelope,source-grounded by pinned envelope and existing MAX_BLOCK_WEIGHT4000000 so valid block bodies fit. Count encoder scratch before allocation and preserve exact guarded bytes through batch suffix/blocked write/drop. Plan160-10requires actual Tcp exact encoded-send override with no re-encode; any scripted default is nonphysical. Plans09/10/11prove cross-direction mixed orders,barrier race,exact8MiB/one-over and eventual cleanup through actual transports. This is intentional local resource policy,not a Knots protocol claim; T-160-38owns mitigation.
8. **RESOLVED — pure dispatch ownership supplement:** Adding ServeBasicFilter exposed admission_bridge's exhaustive relay match. Plan160-15owns exact pure PeerAction/request dispatch,three managed carrier variants,canonical BasicFilterPrepareOutcome/BasicFilterServeUnavailable definitions in filter_transport.rs and explicit relay-only ignore. Plan08imports these definitions and exclusively resolves durable intents; no reverse serving reexport or15→08cycle. Plan15precedes08; T-160-39covers this boundary. No locked decision or deferred product scope changes.

9. **RESOLVED — independent fixture parallelism refinement:** Plan160-16extracts only the original11oracle/generator task into five disjoint Bun/Python/golden paths,with two focused pin/orchestration and authentic rendering/idempotence tasks. It runs wave10after14alongside09/10without touching adapter files or running Cargo/native Rust proof. T-160-31moves uniquely to16. Plan11remains wave11and depends on09/10/16,retaining every original concrete Rust/provenance/mixed transport assertion in nine owned paths. Source-bounded wire/resource facts,all requirements/exclusions and12wave12final gate remain unchanged.

## Environment Availability

| Dependency | Available | Version / evidence | Fallback/action |
| -- | -- | -- | -- |
| Rust/Cargo | Yes | rustc1.94.1; Cargo shim available | Root owns timed serialized verification. [VERIFIED: command -v; rustc --version] |
| Bun | PATH mismatch; pinned fallback yes | PATH1.4.2, required1.3.9; /tmp/open-bitcoin-bun-1.3.9-fresh/bun reports1.3.9 | Use pinned fallback PATH for checks. [VERIFIED: executable probes] |
| Node | Yes |24.13.0 | GSD tooling available. [VERIFIED: node --version] |
| Bazel/Bazelisk | Found | /opt/homebrew/bin; .bazelversion8.6.0 | Native smoke gate verifies; researcher launched no Bazel. [VERIFIED: probes] |
| Pinned Knots | Yes | a9aee730466ac67d35a3c03ee24676be5e045878 | Local source/test helper authority. [VERIFIED: git submodule HEAD] |
| External service/public network | Not required | Hermetic loopback/Fjall tests authorized | No credentials or public daemon needed. [VERIFIED: D-18 and current fixture patterns] |

No blocking missing dependency found. No Cargo/Bazel tests were run by researcher; evidence listed as required remains future work. [VERIFIED: task execution scope]

## Security Domain

Use ASVS5.0 category numbering rather than the outdated template V2-auth/V3-session/V4-access labels. Official V2 and V8 chapters were fetched; this is a scoped threat applicability map, not a certification/compliance claim. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x11-V2-Validation-and-Business-Logic.md; https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x17-V8-Authorization.md]

| Category | Applies | Standard control in this phase |
| -- | -- | -- |
| V2 Validation and Business Logic | Yes | Ordered typed wire gates, complete preparation, finite peer/aggregate work and byte reservations. [CITED: official V2 above; VERIFIED: D-05–12] |
| V8 Authorization | Yes | Own advertised capability, requested/all provenance and stale ScriptsValid branch eligibility; recheck lifecycle/peer session authority. [CITED: official V8 above; VERIFIED: D-01–07,D-13] |
| Authentication/session mechanisms | No new mechanism | Existing permission classes/handshake lifecycle; BASIC protocol adds no credential/session scheme. [VERIFIED: existing peer/config surface; phase scope] |
| Cryptography | Existing only | Reuse SHA256d commitments and target decoding; narrow exact arithmetic helper needs parity vectors, no new crypto library. [VERIFIED: storage parse_record; consensus crypto; pinned proof formula] |

| Threat | STRIDE | Mitigation/evidence |
| -- | -- | -- |
| Unadvertised/all-expanded privilege confused with explicit grant | Elevation | Preserve requested/effective tokens and peer own services; matrix tests before startup. [VERIFIED: pinned permissions/init] |
| Header-only or legacy stale block treated validated | Spoofing/Tampering | Genuine ScriptsValid ledger plus both exact stale-age checks. [VERIFIED: pinned BlockRequestAllowed; validation history] |
| Huge ancestry or small response causing huge record reads | Denial of service | Complete measured ancestry/point-validation/copy work and peer/aggregate quotas before allocation. [VERIFIED: point reader and D-12] |
| Queue pressure or failure leaks suffix reservations | Denial of service | Affine request terminal owner; cancellation/failure releases all unwritten bytes/slots. [VERIFIED: cleanup template; D-16] |
| False completion, duplicate replay, stale lifecycle | Repudiation/Tampering | Complete framed writes only, request/sequence/session/generation-bound one-use receipts. [VERIFIED: D-13–16; affine capability source] |
| Backend diagnostic leakage | Information disclosure | Typed fixed sanitized outcomes; no raw paths/scripts/credentials in peer/log/status success surface. [VERIFIED: existing query fault markers; scoped error patterns] |

## Sources

### Primary — HIGH confidence

- [Pinned request gates and handlers](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/net_processing.cpp#L3156): semantic order, limits, complete lookup, checkpoint ancestry; local BlockRequestAllowed line1865 and InitializeNode line1539.
- [Pinned proof arithmetic](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/chain.cpp#L131): exact proof/equivalent-time signs and saturation.
- [Pinned initialization](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/init.cpp#L1097): general CF prerequisite; explicit permission at2358.
- [Pinned permissions](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/net_permissions.h#L48): all/explicit distinction; net_permissions.cpp parser; net.cpp implicit expansion606 and transport max789.
- [Pinned wire oracle](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/test/functional/test_framework/messages.py#L1736): six message serializers; p2p_blockfilters.py behavior fixtures.
- [BIP157 specification](https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki): independent field/response-order cross-check; local pinned Knots wins behavioral differences.
- Official OWASP ASVS5.0 V2/V8 raw chapters linked in Security Domain: scoped category verification.
- First-party network: message.rs/cursor.rs, peer.rs/message_dispatch.rs, header_store.rs, inbound/permissions.rs, block_serving.rs, limited_serve.rs.
- First-party node: network/runtime_authority.rs and filter_index/query.rs/readiness, storage/filter_index.rs and fjall_store/filters/query.rs, validation_history adapters, announcement_transport.rs/lifecycle_effects.rs, limited_serve.rs, sync/session.rs/tcp.rs/emission_terminal.rs.
- First-party RPC: config.rs/loader.rs/loader/blockfilter.rs/open_bitcoin_runtime.rs, bin/open-bitcoind.rs, context.rs/context/inbound_wire.rs, inbound_listener/connection_runtime.rs/resource_runtime.rs and successful-prefix tests.
- Planning/evidence: finalized160-CONTEXT.md, ROADMAP/REQUIREMENTS/PROJECT/STATE, prior157–159 context/research,159-04-SUMMARY and159-07-SUMMARY; AGENTS/sidecar/standards and config pins.
- Transport explorer independently traced actual socket callers, missing outbound plan consumption, supported emission kinds,128 ledger cap, completion and suffix-cleanup behavior. [VERIFIED: explorer report source references]

### Secondary / Tertiary

None used for implementation claims. Context7 is not exposed; no new third-party API is required. Local pinned source is authoritative for parity, while official BIP/ASVS sources supply cross-checks. [VERIFIED: available tool inventory; fixed pin]

## Metadata

- Standard stack: HIGH — lockfile/toolchain pins and executable probes checked. [VERIFIED: sources above]
- Baseline semantics: HIGH — direct pinned handler, initialization, permissions, proof and exception-boundary source. [VERIFIED: listed Knots files]
- Architecture: MEDIUM — exact existing seams checked; new proof metadata/resource/receipt implementation still needs execution and fault evidence. [VERIFIED: current code gaps]
- Pitfalls: HIGH — concrete contradictory current code paths and independent transport trace. [VERIFIED: source/explorer findings]
- Research date:2026-10-10. Valid until source/pin/context changes; otherwise stable-source review by2026-11-09. [VERIFIED: fixed source scope; planning validity recommendation]
- Researcher changed only this file; no source edits, build/test runs, state changes, commits or pushes. Outer wrapper owns clean finalization. [VERIFIED: task scope]
