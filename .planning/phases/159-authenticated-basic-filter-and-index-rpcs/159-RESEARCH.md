---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: 2026-10-09T16:24:07Z
---

# Phase 159: Authenticated BASIC Filter and Index RPCs - Research

**Researched:** 2026-10-09
**Domain:** Pinned Knots RPC behavior, accepted-validation provenance, Fjall point reads and owner readiness
**Confidence:** HIGH for baseline/source findings; MEDIUM for proposed new provenance/barrier design pending implementation evidence

<user-constraints>
## User Constraints (from CONTEXT.md)

The following decisions and discretion are copied verbatim from CONTEXT.md. [VERIFIED: 159-CONTEXT.md]

### Pinned request and error contract

- **D-01:** Use existing baseline-origin node-scoped authenticated RPC routing. Support positional and named blockhash/filtertype/index_name, omitted/null defaults, arity/help and pinned argument type/hash errors. Dedicated normalization must preserve framework type-check precedence rather than generic serde -32602 defaults.
- **D-02:** After framework checks, getblockfilter parses hash, resolves filter type, checks enabled index, resolves known block and connected validity, obtains readiness, then attempts immutable filter/header lookup. Unknown type is -5 `Unknown filtertype`; disabled BASIC is -1 `Index is not enabled for filtertype basic`; unknown block is -5 `Block not found`. Knots recognizes v0 but it is disabled/out of scope here; preserve its disabled error without implementing V0. Numeric names and uppercase BASIC are unknown.
- **D-03:** Successful stored lookup wins during catch-up and for retained stale/pruned blocks. Only failed lookup classifies never-connected as -5 `Filter not found. Block was not connected to active chain.`, initial indexing as -1 `Filter not found. Block filters are still in the process of being indexed.`, otherwise -32603 `Filter not found. This error is unexpected and indicates index corruption.` Backend failures/corruption never become successful empty filters or ordinary absence.
- **D-04:** Return exactly filter/header lowercase hex; header is uint256 display order, reversed from raw commitment bytes. Preserve exact names and codes for malformed hash length/hex and -3 type errors based on pinned local source.

### Shared read authority and provenance

- **D-05:** Prefer narrow typed read/query methods on existing ManagedNetworkHandle and the same configured Fjall authority. No independent RPC-owned index, duplicate full-history cache, detached owner, request-side history regeneration or implicit activation. Extract a pure classifier only if it clarifies actual decision inputs.
- **D-06:** Research and prove genuine ever-connected/scripts-valid provenance for known-but-unconnected and displaced-connected blocks, including missing filter rows after prune/reopen. Active membership, header/body/undo presence and an assumed fixture bit cannot replace accepted validation provenance. Preserve accepted-before-persistence facts and fail-closed recovery.
- **D-07:** Request reads must be bounded and measured. Current full-ancestry getters remain recovery validators; add a narrow integrity-preserving request path instead of walking entire history. Prefer authority-held point reads; prepare/read/checked-completion is discretionary only if measurements justify its complexity. Missing/corrupt predecessor/commitment evidence must not silently serve forged headers.

### Index summary and readiness

- **D-08:** getindexinfo returns only `basic block filter index` with `synced` and `best_block_height`. Omitted/null/empty selection returns all enabled in-scope indexes; exact name selects BASIC; unmatched name or disabled index returns {}. No invented txindex/coinstatsindex capability.
- **D-09:** synced is BasicIndexProgress::initially_synchronized(), preserving the initial-sync latch through later lag/reorg. best_block_height is processed progress, zero fallback, not safe durable checkpoint or current tip. Recover/reopen semantics follow pinned baseline and existing conservative durable reconciliation.
- **D-10:** Distinguish summary latch from getblockfilter readiness. Pinned BaseIndex returns promptly before initial completion, and later drains validation notifications. Research an equivalent bounded owner/barrier without waiting under the RPC context mutex for scheduled work or doing unbounded request-triggered indexing. Later pending work cannot be falsely diagnosed as corruption solely because synced is true.

### Authentication and evidence

- **D-11:** Reuse auth-before-JSON/context-lock HTTP boundary. Prove missing/wrong credentials on malformed requests disclose no index state; cookie/password success, batches/notifications and node scope follow existing transport. No credentials, datadir paths or raw backend details in public diagnostics.
- **D-12:** Prove production daemon configuration, actual shared dispatch, valid continuous-chain active/stale/pruned retrieval after real paired deletion and reopen, initial catch-up available/missing rows, later lag, known unconnected and connected absence, publication/backend corruption faults and concurrent lifecycle outcomes. Sparse codec fixtures alone cannot establish acceptance. Include focused parser/pure tests, exact result keys and negative precedence controls.
- **D-13:** Add parity breadcrumbs and scoped docs, review relevant READMEs and provide repo-local Cargo/Bazel UAT commands. Run native bash scripts/verify.sh, Bright Builds, source/security review and lifecycle validation before final commit/push. Keep production dependencies unchanged and public/default activation off.

### Agent Discretion

Exact types, provenance storage/recovery compatibility, read budget and barrier design, fixture decomposition and plan boundaries belong to research/planning. Roadmap UI hint is a generic indicator: this phase owns JSON RPC, no frontend; record the UI-gate applicability decision and do not invent a GUI/dashboard. No destructive repair, public-network verification or production/funds claims.

### Deferred Ideas (OUT OF SCOPE)

Peer compact-filter transport, broad operator/status/dashboard projections and full integrated milestone proof belong to Phases 160–162. V0, automatic repair/download, public defaults, archive-scale/funds/production claims remain deferred. No pending todos matched this phase.
</user-constraints>

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| -- | -- | -- |
| CFRP-01 | An authenticated client can call BASIC `getblockfilter` with pinned result shape, default filter type, error ordering and codes for disabled index, unknown block/type, never-connected block, indexing absence and corruption, including successful retained stale/pruned lookup. | Pinned request pipeline; accepted-validation ledger; constant-read integrity path; captured-frontier barrier; actual daemon HTTP and retained-prune evidence below. [VERIFIED: .planning/REQUIREMENTS.md] |
| CFRP-02 | An authenticated client can call `getindexinfo` with the pinned BASIC index name, `synced` and `best_block_height`, exact-name selection and empty-object absence; `synced` preserves Knots' initial-synchronization meaning. | Project existing initial-completion latch and processed endpoint; separate summary from barrier; exact selection tests. [VERIFIED: .planning/REQUIREMENTS.md] |
</phase-requirements>

## Project Constraints (from AGENTS.md)

- Preserve pinned Knots `29.3.knots20260210`, local commit `a9aee730466ac67d35a3c03ee24676be5e045878`; record intentional storage/resource differences in parity docs. [VERIFIED: AGENTS.md; git submodule HEAD]
- Extend existing first-party crates, functional core and effectful adapters; add no production crates/dependencies or Rust Bitcoin libraries. [VERIFIED: AGENTS.md; .planning/PROJECT.md; REQUIREMENTS.md]
- Rust `1.94.1`/2024, Bun `1.3.9`, Bazel `8.6.0` are pinned. Bun owns substantial automation; there is no package.json/install step. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml; .bun-version; .bazelversion; AGENTS.md]
- Native full verification is `bash scripts/verify.sh`, including Bazel smoke; fast mode is iteration only. Preserve hooks, generated LOC freshness, breadcrumb coverage and existing ordered historical guards. Root owns every commit after a clean strict gate. [VERIFIED: AGENTS.md; scripts/verify.sh; parent task authorization]
- Run ad-hoc Cargo/Bazel through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`; serialize shared targets and poll live resumable checks at least each 60 seconds. [VERIFIED: AGENTS.md]
- Use explicit optional naming, guard clauses, typed invariants, public API docs, visible error propagation, Arrange/Act/Assert and behavior tests; split large modules at coherent seams. [VERIFIED: AGENTS.bright-builds.md; standards/core/architecture.md; standards/core/code-shape.md; standards/core/testing.md; standards/languages/rust.md]
- Material guidance loaded: AGENTS.md, sidecar, placeholder-only overrides, standards index/architecture/code-shape/testing/verification/Rust; historical Phase 154–158 contexts, canonical milestone research and current roadmap/requirements/state. Both active lessons total 7,188 bytes/2,397 conservative tokens and were fully read; no project .claude/.agents skills directory was found. [VERIFIED: file reads and directory probes]
- The UI applicability gate is nonvisual RPC: JSON results/errors are the product surface; do not add frontend/dashboard scope. `workflow.nyquist_validation=false`, so omit Validation Architecture. Security enforcement is not disabled. [VERIFIED: 159-CONTEXT.md; .planning/config.json]

## Summary

Implement two baseline-origin node RPCs through the current daemon's shared `ManagedNetworkHandle`. The result projection is small; the hard work is preserving genuine accepted-validation history, serving records without ancestry walks, and honoring the distinction between Knots' initial-sync latch and its post-initial validation-notification drain. Existing filter/progress/HTTP code supplies the underlying primitives but does not already solve those three boundaries. [VERIFIED: rpc/context.rs; rpc/http.rs; node/network/runtime_authority/filter_index.rs; node/storage/fjall_store/filters.rs; Knots rpc/blockchain.cpp:3317; index/base.cpp:367]

Use a retained, additive per-hash provenance record minted only at real validated absorb seams; an authority-held read of a target and its immediate parent with existing codec validation; and an async captured-frontier barrier outside the RPC context mutex. Keep complete recovery validators as recovery validators. Legacy absence of newly introduced provenance cannot be called never-connected; make compatibility fail closed explicitly. The design below is prescriptive planning guidance, not a claim that these new mechanisms already exist. [VERIFIED: chainstate.rs:340–449; storage/filter_index.rs; storage/fjall_store/filters/ownership.rs; http.rs:execute_request; 159-CONTEXT.md D-05–D-10]

**Primary recommendation:** plan provenance/storage, point-read authority, parser/dispatch, asynchronous readiness, real daemon evidence, and parity/native guards as distinct dependency-ordered work units. [VERIFIED: source seams documented below]

## Standard Stack

### Core

| Component | Existing version | Purpose | Directive |
| -- | -- | -- | -- |
| Rust/first-party crates | 1.94.1 / 0.1.0 | Typed queries, validation facts and classifier | Reuse node/core/RPC crates. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml] |
| Fjall | 3.1.10, git revision aa30dca811399a201e0b9595da93a4582dcb2b57 | Same-database durable acceptance facts and filter reads | Preserve pinned revision, namespaces, atomic batches and guards. [VERIFIED: node/Cargo.toml; Cargo.lock] |
| Axum / Tokio | 0.8.9 / 1.52.1 | Existing HTTP server / async barrier adapter | Keep waits in RPC shell; node has no Tokio dependency. [VERIFIED: Cargo.lock; node/Cargo.toml; rpc/http.rs] |
| serde / serde_json | 1.0.228 / 1.0.149 | Typed result shapes / input values | Dedicated baseline normalizer before typed construction. [VERIFIED: Cargo.lock; rpc/method/normalize.rs] |

### Supporting

| Component | Existing version | Use |
| -- | -- | -- |
| std synchronization / ordered maps | Rust toolchain | Authority, shared signal revision and bounded pending acceptance; do not add node runtime dependency. [VERIFIED: node/network/runtime_authority.rs; node/Cargo.toml] |
| Bun native claim checks | 1.3.9 | Mutation-sensitive phase guard and explicit-file tests. [VERIFIED: .bun-version; scripts/verify.sh] |
| Existing getrandom / base64 | Existing lockfile | Reuse cookie creation / HTTP Basic decoding without new auth mechanisms. [VERIFIED: rpc/http.rs; Cargo.lock] |

No installation or package upgrade is required. Registry latest versions/publish dates are intentionally not recommendations: this phase locks existing versions and adds no dependencies. [VERIFIED: 159-CONTEXT.md D-13; Cargo.lock]

## Architecture Patterns

### 1. Exact pinned request pipeline

Knots first transforms named parameters, checks arity/help, collects **all argument type mismatches**, then enters the handler. A wrong filtertype type therefore wins over a malformed string blockhash; required blockhash null is a -3 type error, optional filtertype/index_name null is a default. Existing generic `normalize_request` instead maps serde errors and arity/name errors to -32602 and must not be reused unchanged. [VERIFIED: Knots rpc/server.cpp:439–560; rpc/util.cpp:657–687,960–972; rpc/method/normalize.rs]

Add `method/filter_index.rs` plus a dedicated baseline normalizer (prefer separate `method/filter_index/normalize.rs`) returning typed `GetBlockFilterRequest { block_hash, filter_type }` and `GetIndexInfoRequest { maybe_index_name }`. Extend SupportedMethod/all/origin/scope/MethodCall and `normalize_method_call`; use baseline origin and node scope. Add missing RpcErrorCode variants for -3 and -5, their roundtrips, and focused constructors rather than borrowing -32602. Proposed names/signatures are implementation recommendations. [VERIFIED: rpc/method.rs; rpc/error.rs; Knots rpc/protocol.h; CONTEXT.md D-01]

| Stage | Required behavior |
| -- | -- |
| Named conversion | Recognize blockhash/filtertype/index_name and mixed `args` array; holes become null only before a supplied later argument. Duplicate/unknown names use -8 with `Parameter X specified multiple times`, `Unknown named parameter X`, or mixed collision `Parameter X specified twice both as positional and named argument`. [VERIFIED: Knots rpc/server.cpp:439–560] |
| Arity | 1..2 for getblockfilter, 0..1 for getindexinfo; wrong arity throws generated help via runtime_error, then RPC_MISC_ERROR (-1). Do not emit generic maximum-arity -32602. [VERIFIED: Knots rpc/util.cpp:657–675, RPCHelpMan::IsValidNumArgs; rpc/server.cpp ExecuteCommand] |
| Type | -3 message `Wrong type passed:\n{\n    "Position 1 (blockhash)": "JSON value of type number is not of expected type string"\n}`; collect positions in declaration order, preserving pinned pretty formatting. Null required hash is type null, bool type bool, arrays/objects retain their JSON type names. [VERIFIED: Knots rpc/util.cpp:676–686,960–972; src/univalue/lib/univalue.cpp:218] |
| Hash | Strict 64 ASCII hex chars, accepts uppercase hex, no trim or 0x; -8 length `blockhash must be of length 64 (not N, for 'VALUE')`, else `blockhash must be hexadecimal string (not 'VALUE')`. Parse display bytes then reverse into raw BlockHash. [VERIFIED: Knots rpc/util.cpp:120–128; uint256.h:157–163] |
| Type name | omitted/null -> basic; basic supported; v0 known but disabled -> -1 `Index is not enabled for filtertype v0`; other names including BASIC/"0" -> -5 `Unknown filtertype`. [VERIFIED: Knots blockfilter.cpp g_filter_types/BlockFilterTypeByName; rpc/blockchain.cpp:3337–3352] |
| Enabled / known | Disabled basic -> -1 `Index is not enabled for filtertype basic`, before block lookup; unknown known-block lookup -> -5 `Block not found`. [VERIFIED: Knots rpc/blockchain.cpp:3348–3361] |
| Read | Obtain readiness before immutable lookup; successful stored filter + header wins irrespective of connected bit/readiness/stale status. Backend failure is not None. [VERIFIED: Knots rpc/blockchain.cpp:3365–3388; CONTEXT.md D-03] |
| Missing row | NeverConnected -> -5 `Filter not found. Block was not connected to active chain.`; connected + initial not ready -> -1 `Filter not found. Block filters are still in the process of being indexed.`; otherwise -> -32603 `Filter not found. This error is unexpected and indicates index corruption.`. [VERIFIED: Knots rpc/blockchain.cpp:3370–3387] |

Raw duplicate JSON object keys have already been collapsed by existing serde_json::Value parsing, whereas RequestParameters::Named preserves supplied tuple duplicates. Test the typed normalizer and mixed args; if claiming raw duplicate-key parity, preserve pairs at HTTP parse instead of pretending Value preserves them. This is a real existing transport limitation that needs a scoped plan decision, not a successful duplicate test through a map. [VERIFIED: rpc/http.rs from_slice<Value>; rpc/http/request.rs parse_request; rpc/method.rs RequestParameters; Knots rpc/server.cpp duplicate map construction]

The existing `dispatch::decode_hex` trims strings, so it is unsuitable unchanged for strict ParseHashV. Generic `encode_hex` emits raw-byte order; reverse only filter-header/raw BlockHash display values, never encoded filter bytes. Test asymmetric bytes instead of all-zero sentinels. [VERIFIED: rpc/dispatch/decode.rs; Knots uint256.h and rpc/blockchain.cpp]

### 2. Genuine accepted-validation provenance and legacy compatibility

**Existing gap:** HeaderEntry stores header/height/work, not BLOCK_VALID_SCRIPTS. Active chain metadata stores positions/counts, not retained displaced validation. ValidatedChainstateLineage stores a single live accepted endpoint and guarded flush authority; it is not an ever-connected history. Filter records cannot classify their own absence. [VERIFIED: network/src/header_store.rs:19–40; node/storage/snapshot_codec/chain_meta.rs; node/chainstate/fjall_store.rs:ValidatedChainstateLineage; node/storage/filter_index.rs]

Add a small versioned BlockIndex per-hash provenance envelope, e.g. `validated_block:v1:<raw_hash>`, binding hash, parent and height with explicit known-only versus scripts-valid status/coverage. Upgrade monotonically only from genuine accepted staging. Keep it independent of filter availability, active projection, pruning and BASIC disable. Publish through the same store, with private/sealed write authority rather than a public "set connected" boolean. Proposed schema/names are design choices anchored to the real missing seam. [VERIFIED: chainstate.rs commit_prepared_connect/commit_prepared_reorg; CONTEXT.md D-06]

- Ordinary connects: seal/copy the accepted identity immediately after `absorb_staged_connect`, before `persist()`. Even if later persistence fails, in-process queries must retain accepted=true. Reject further acceptance if an unresolved write would exceed the bounded pending ledger budget; do not grow a duplicate all-history cache. [VERIFIED: chainstate.rs:340–364; accepted Basic facts pattern in chainstate/filter_index.rs]
- Reorg: preserve old accepted records and add **every validated replacement position**, not only the new endpoint. Capture from staged accepted positions/receipt around `absorb_staged_reorg_with_receipt`, before `accept_basic_index_reorg` or later fallible persistence. No ledger write on a failed preview/stage. [VERIFIED: chainstate.rs:428–449; chainstate/fjall_store/reorg.rs; CONTEXT.md D-06]
- Known header-only blocks: record/recognize genuine header admission separately; no block body, undo presence, stored codec-valid bytes or test-only bit grants scripts-valid status. Known lookup should use direct `peer_manager.header_store().entry(&hash)` plus retained provenance identity, not `header_entries()` or a cloned chainstate snapshot. [VERIFIED: network.rs header_entries; network/header_sync.rs; network/src/header_store.rs:158]
- Reopen: validate ledger envelope/hash/monotonic upgrade constraints in existing exclusive startup, retaining scripts-valid facts even when recovered coins rewind or blocks become stale/pruned. Ledger describes validation history, **never** durable coins progress or prune permission. [VERIFIED: storage/fjall_store/filters/startup.rs; chainstate/fjall_store.rs; CONTEXT.md D-06/D-09]
- Legacy stores: use explicit `UnknownLegacy` when no authentic acceptance evidence exists. Do not backfill scripts-valid from current membership, body/undo, filter presence or source fixture comments. Valid immutable filters can still return success. Missing record with UnknownLegacy must fail closed/redacted, not pretend never-connected. Document this additive-layout compatibility limitation. If an implementation wants exact legacy missing-row classification, it must supply an independently sealed/revalidated accepted-history receipt; no such persistent receipt was found. [VERIFIED: inspected header/chain-meta/lineage formats; CONTEXT.md D-06]

A bounded pending identity retained after an acceptance-ledger publication failure and an additive on-disk known-only record allow new header-only versus accepted missing-row cases to be classified after reopen without an all-history memory set. The implementation must prove the failure/refusal policy and raw writer invalidation; these mechanisms are **to be built**, not verified existing behavior. [VERIFIED: existing authority/PublicationControl patterns; CONTEXT.md D-06/D-07]

### 3. Bounded immutable query path

`load_basic_filter_record(hash)` reads ancestors down to genesis then decodes back up; `maybe_basic_filter_checkpoint` also validates projection from genesis. These are O(height) recovery paths. `maybe_active_basic_filter_record` uses both; do not expose them to requests or summary reads. [VERIFIED: node/storage/fjall_store/filters.rs:74–204]

Add `filters/query.rs` with an authority/PublicationControl guarded target read and at most one immediate-parent read. Use `parse_record` for the target **and parent** to validate lengths, BASIC encoding, byte hash and own header commitment; `parse_record_fields` alone deliberately skips content commitment. Then `verify_parent`/`checkpoint_identity` binds height/hash/previous header. Only construct an owned response after these checks and budget admission. For genesis enforce the zero predecessor. [VERIFIED: storage/filter_index.rs parse_record, parse_record_fields, ParsedFilterRecord methods]

The complete startup scan already proves all immutable ancestry, including hidden fork rows, without retaining a full encoded index; trusted immutable writers maintain that proof. Make a read capability/revision depend on that recovered integrity authority and invalidate it on raw/ambiguous writes/poison. Checking just one edge without that capability does **not** prove all ancestors. If a raw clone changes a historical parent after capability acquisition, refuse/revalidate through recovery; do not silently trust a stale validation epoch. [VERIFIED: filters.rs validate_basic_filter_records; filters/ownership.rs BasicFilterAppendIdentity; filters/publication.rs PublicationControl/invalidate_append; CONTEXT.md D-07]

Recommended request outcomes: `Found(BasicFilterResult)`, `Missing { provenance, readiness }`, `Pending(BasicFilterReadBarrier)`, `Disabled`, `UnknownBlock`, and typed storage/authority error. Keep record identity + bytes/header as one result so callers cannot combine different lifecycle snapshots. One authority acquisition performs direct known/provenance lookup, progress snapshot and target/parent reads. This avoids detached index ownership and callback casting. [VERIFIED: runtime_authority.rs read/mutate and clone; CONTEXT.md D-05]

Measure fixed point-read count for genesis/height 20/height 400 and stale records. Count encoded bytes validated/copied, response expansion (2x hex), authority hold and storage elapsed; no bodies, undo, filters generated, full history snapshots or recovery scans on query. Choose numeric limits from existing valid filter bounds and measured fixtures, not an assumed latency promise. [VERIFIED: codec/filter bounds; filters.rs point-read counter; catch_up.rs BasicFilterTurnOutcome; CONTEXT.md D-07]

### 4. Summary latch versus captured-frontier readiness barrier

Knots `GetSummary` reports m_synced and m_best_block_index height or zero. `BlockUntilSyncedToCurrentChain` returns false before initial sync; once latched, it either detects current-chain ancestry already processed or drains validation-interface callbacks and returns true. Initialization recomputes m_synced from recovered index best block versus tip; it does not persist a forever-true boolean across restart. [VERIFIED: Knots index/base.cpp:81–123,367–390,415–428]

Existing BasicIndexProgress initializes the latch from processed==accepted, ORs it true on completed turns, and preserves it across ordinary target movement. Use `initially_synchronized()` and `maybe_processed_endpoint().map_or(0, |id| id.height())` for getindexinfo. Do not use safe endpoint, tip equality or configured-enabled as synced. Return exactly `{"basic block filter index":{"synced":bool,"best_block_height":number}}`. Selection is absent/null/empty=all enabled, exact case-sensitive name=one, unmatched or disabled={}. [VERIFIED: chainstate/filter_index/catch_up.rs:129–158,185–200,269–278; Knots rpc/node.cpp:409–462; CONTEXT.md D-08/D-09]

**Concrete equivalent for post-initial drain:** introduce a captured accepted frontier (generation + branch + accepted sequence/hash) and an owner-completion signal. Query captures the frontier only after observing the initial latch. It becomes ready only after ordinary scheduled work has achieved that frontier, or returns a terminal redacted failure on index failure/disable/invalidated branch/cancel. Newer accepts must not move an existing wait target forever. Initial catch-up must never wait and can return an already-present record. [VERIFIED: Knots index/base.cpp BlockUntilSyncedToCurrentChain and BlockConnected; project accepted target versus processed endpoint; CONTEXT.md D-10]

**HTTP orchestration:** normalize/hash/type/index/known checks first; briefly lock context to clone the same authority/capture a pending query; clear request-local scope and drop the context lock; asynchronously wait for the captured frontier outside both context and authority locks; reacquire/revalidate generation/branch and execute final point read. Synchronous dispatch/query returns typed Pending rather than misclassifying later lag as corruption. Keep getindexinfo synchronous. Signal completion/failure from the current scheduled owner, not a new detached worker that owns index state. [VERIFIED: rpc/http.rs execute_request holds context across dispatch; bin/open_bitcoind/coins_flush.rs existing one-second worker; CONTEXT.md D-05/D-10]

Node has no Tokio dependency. Use a std-owned revision/frontier plus bounded waiter registration or a std wait primitive exposed through the RPC async shell; do not block Tokio executor workers or use a timeout to invent an indexing/corruption classification. A runtime type-erasure seam must retain the same ManagedNetworkHandle; prefer a narrow query trait on existing storage/authority with disabled defaults for memory and explicit Fjall implementation, rather than duplicating an index inside RPC. Exact trait plumbing is planner discretion and must compile across generic dispatch/context bounds. [VERIFIED: node/Cargo.toml; rpc/context.rs ManagedRpcContext<S,V>; runtime_authority.rs; CONTEXT.md D-05/D-10]

The existing ordinary one-block accepted-unflushed path already supports indexing captured facts ahead of the durable coins checkpoint; use that mechanism when scheduled work completes the barrier. Do not force a coins flush to satisfy a read, and do not call `drive_basic_filter_index_turn` in an unbounded request loop. Reorg pending publication and terminal storage failures must wake waiters with failure/invalidation rather than hang indefinitely. [VERIFIED: node/sync/tests/filter_index/evidence/retention.rs phase157_store_retention_accepted_unflushed_reopen_loses_only_uncheckpointed_tip; catch_up.rs drive_turn; CONTEXT.md D-10]

### 5. Actual daemon and authenticated transport

Production already selects durable storage for explicit BASIC, opens configured DurableSyncRuntime, clones its authority into `ManagedRpcContext::from_runtime_config_with_network_handle`, shares that context between HTTP and other consumers, and starts the maintenance worker. Exercise this exact selection/context wiring; constructing a memory fixture context with an unrelated metrics store cannot prove it. [VERIFIED: rpc/bin/open-bitcoind.rs open_runtime_store, open_authoritative_network_runtime, serve_authoritative_runtime]

HTTP authenticates before JSON decode or context lock; password and cookie auth are already supported, with cookie mode 0600 on Unix and getrandom entropy. Reuse the handler/router, existing version/status mapping, batches/notifications and root node-scope rule. New errors must redact backend strings, paths and credentials. [VERIFIED: rpc/http.rs handle_http_request, execute_request, resolve_auth, read_or_create_cookie_password; rpc/http/request.rs; CONTEXT.md D-11]

## Recommended Dependency Plan

| Unit | Own files/seams | Required evidence / dependency |
| -- | -- | -- |
| 01 Provenance | node chainstate acceptance/reorg; storage versioned ledger; recovery | Genuine accepted receipt, failure before/after persist, retained stale/pruned/reopen, explicit legacy unknown; no fabricated promotion. |
| 02 Verified point query | node storage/filter_index codec; fjall_store/filters/query; runtime_authority/filter_index/query | 01; target/parent corruption, raw clone invalidation, fixed read count/byte/hold measurements, disabled defaults. |
| 03 Baseline parser/results | rpc method/filter_index and normalize; method.rs; error.rs | Pure exact error ordering/type aggregation/hash/name/null/arity/mixed controls; asymmetric display endian. |
| 04 Shared dispatch + barrier | rpc context/filter_index; dispatch/filter_index; HTTP async adapter; existing owner signal | 01–03; initial no-wait, later accepted-unflushed wait then success, terminal failure wake, lifecycle recapture, context lock released. |
| 05 Real daemon HTTP proof | daemon tests/filter_index plus focused fixtures; RPC HTTP tests | 01–04; actual config+opened authority+HTTP, cookie/password, batches, notifications, active/stale/prune/reopen, absent/fault matrix. |
| 06 Parity/native closeout | docs/parity roots/breadcrumbs, READMEs/UAT, scripts phase159 guard/test, verify insertion | All earlier units; phase-specific mutation controls, no-claim guard, native full verifier and source/security review before root commit/push. |

This decomposition is a recommendation from the verified seams above. Keep file ownership disjoint between parallel work; barrier and dispatch integration depend on node query types and must not be raced against unfinished APIs. [VERIFIED: cited source layout; CONTEXT.md D-12/D-13]

## Don't Hand-Roll

| Problem | Do not build | Use |
| -- | -- | -- |
| Filter/header generation on read | Historical script reconstruction or new GCS implementation | Existing immutable bytes and codec/hash validators. [VERIFIED: storage/filter_index.rs; CONTEXT.md D-05] |
| Auth/JSON-RPC transport | New server/auth/session mechanism | Existing Axum HTTP boundary and response envelopes. [VERIFIED: rpc/http.rs] |
| Recovery query | Per-request ancestry/cache refresh | Exclusive recovered integrity authority plus bounded point reads. [VERIFIED: filters/startup.rs; filters/ownership.rs] |
| Readiness | Synced==ready, request-side full catchup, independent worker owner | Captured-frontier barrier driven by current scheduled owner. [VERIFIED: Knots index/base.cpp; existing coins_flush worker] |
| Scripts validity | Body/undo/active/filter-presence heuristic | Monotonic acceptance provenance from genuine absorb receipts. [VERIFIED: Knots rpc/blockchain.cpp uses IsValid(BLOCK_VALID_SCRIPTS); context D-06] |
| Rust runtime in node | New Tokio dependency for waiter plumbing | std owner state; Tokio adapter in existing RPC shell. [VERIFIED: node/Cargo.toml; rpc/http.rs] |

## Common Pitfalls

1. **Later lag called corruption.** The initial latch remains true while an accepted tip awaits the scheduled turn. Drain the captured frontier before missing-row classification; prove accepted-unflushed facts, terminal worker faults and concurrent new targets. [VERIFIED: BasicIndexProgress observe_validated_connect; Knots BlockUntilSyncedToCurrentChain; phase157 retention test]
2. **Forged parent header passes local target validation.** parse_record_fields skips own byte/hash commitments; use parse_record for parent and a maintained recovered-integrity epoch, not only self-consistent target bytes. [VERIFIED: storage/filter_index.rs]
3. **Ledger minted from fixtures/snapshots.** Public arbitrary chain metadata/header rows are not accepted-script receipts; use real staging and explicit legacy unknown. A removed filter row must not also erase acceptance. [VERIFIED: chain_meta DTO; header_store DTO; chainstate acceptance seams]
4. **Wrong error wins.** Handler hash parsing before framework type aggregation produces -8 instead of -3 when the second arg is nonstring; disabled basic must win over unknown block after valid argument parsing. [VERIFIED: Knots rpc/util.cpp HandleRequest; rpc/blockchain.cpp]
5. **Successful empty substitute.** A BASIC empty filter has a genuine encoded representation; missing/corrupt/backend-failed bytes are not a generated empty result. [VERIFIED: storage/filter_index.rs parse_record; CONTEXT.md D-03]
6. **Legacy versus 2.0 HTTP codes drift.** Keep existing HTTP status_for_single/legacy_status_for_failure and envelope IDs when adding -3/-5 details; test both versions without rewriting unrelated transport semantics. [VERIFIED: http/request.rs; error.rs]
7. **Deadlock or stalled waiter.** Current HTTP holds context mutex through synchronous dispatch; never await scheduled maintenance there. Terminating/disabling/failing owner must signal pending callers. [VERIFIED: http.rs execute_request; coins_flush.rs; CONTEXT.md D-10]
8. **Historical checker regression.** Do not replace ordered historical guards or claim CFPR-02/CFGR complete in this phase; Phase162 still owns integrated peer proof. [VERIFIED: scripts/verify.sh; ROADMAP.md]

## Code Examples

### Existing accepted-before-fallible-persistence seam

```rust
// Source: packages/open-bitcoin-node/src/chainstate.rs:347–353
let position = self.chainstate.absorb_staged_connect(prepared.staged);
self.observe_validated_lineage(&position);
self.observe_basic_filter_acceptance(&position, prepared.maybe_filter_facts);
if let Err(error) = self.persist() {
    self.note_basic_index_failure(filter_index::AcceptedBasicIndexFailure::Persistence);
    return Err(map_persist(error));
}
```

Insert sealed ever-connected observation here, before persist; never in a success-only network callback. [VERIFIED: quoted source]

### Existing bounded local identity building blocks

```rust
// Source: packages/open-bitcoin-node/src/storage/filter_index.rs
let parsed = parse_record(&key, &bytes)?; // bounds, encoding, own byte/hash/header
parsed.verify_parent(maybe_parent.as_ref())?;
let identity = parsed.checkpoint_identity(maybe_parent.as_ref())?;
```

This sketch uses existing methods but the guarded point reader is new; it requires recovered all-ancestry authority and fully parsed parent as described above. [VERIFIED: ParsedFilterRecord APIs; filters.rs validate_basic_filter_records]

### Exact summary projection

```rust
// Sources: BasicIndexProgress APIs; pinned index/base.cpp GetSummary
let synced = progress.initially_synchronized();
let best_block_height = progress.maybe_processed_endpoint()
    .map_or(0, |endpoint| endpoint.height());
```

The wrapper name/fields must be exactly the pinned JSON names, with no hash/checkpoint/lag extras. [VERIFIED: Knots rpc/node.cpp SummaryToJSON]

## Evidence and Verification Design

Nyquist scaffolding is disabled, but behavioral evidence remains required. Use native Rust tests plus Bun mutation guards; no new framework. Root must run full repo-native verification before final commit/push. [VERIFIED: config.json; AGENTS.md; parent scope]

| Behavior | Concrete fixture/test seam | Essential controls |
| -- | -- | -- |
| Request precedence | rpc/method/tests/filter_index.rs (new) | named/positional/mixed, missing/null, both wrong types, malformed hash+wrong type, v0/basic/BASIC/"0", arity/help |
| Authority/read bounds | runtime_authority/filter_index/query/tests.rs (new) | genesis/deep/stale point count, bytes budget, no body/undo/generation; malformed target/parent/missing parent; epoch invalidation |
| Acceptance | chainstate + storage ledger tests (new) | rejected stage no upgrade; accepted persistence failure still live true; every replacement accepted; stale/pruned retained ledger; absence != false for legacy |
| Summary/readiness | owner plus HTTP tests (new) | initial latch false with present row succeeds; initially missing row -1; later lag summary true, pending query waits, then succeeds; no coins flush forced |
| Actual daemon | bin/open_bitcoind/tests/filter_index/rpc.rs (new) | runtime config parser -> open_runtime_store -> open_authoritative_network_runtime -> shared context -> actual handle_http_request/router |
| Retained prune/reopen | continuous 400+ block local validated fixture | prune height20 through flush_applying_prune_plan; assert actual deleted hashes, body=None, undo=None, have_pruned; drop all store/runtime handles; reopen configured runtime; HTTP bytes/header unchanged |
| Stale retained | existing ForkFixture pattern, small branch fixture | genuine validated reorg, old hash exact bytes/header before/after reopen; remove stale row without removing provenance then distinguish connected absence |
| HTTP auth/scope | existing router/password/cookie helpers | malformed unauthorized 401 empty without state/lock; wrong credential same; cookie and password success; wallet URI rejection; batch/notification IDs |
| Fault/lifecycle | existing publication fault seams plus query faults | before records/checkpoint/protection and after commit poison; backend read error distinct from absent; concurrent disable/reorg invalidates or recaptures; no successful empty result |

These are required new evidence cases, not claims that tests exist yet. Source-backed reusable fixtures: `node/sync/tests/filter_index/reorg/fixtures.rs` and `retention.rs`; `node/sync/tests/filter_index/evidence/retention.rs`; `rpc/bin/open_bitcoind/tests/filter_index/fixtures.rs` builds a continuous staged/committed chain with historical and same-block spends; `rpc/http/tests` supplies transport patterns. Avoid making cfg(test)-private node fixtures a production dependency or treating sparse storage fixtures as accepted chain evidence. [VERIFIED: cited fixture files; CONTEXT.md D-12]

Use manual paired prune for a small deterministic RPC retention test; automatic prune needs the existing real threshold/volume fixture and should be reused only if it adds evidence, not duplicated merely to increase count. Phase159 owns RPC post-prune proof; Phase162 retains the all-peer integrated CFPR-02 gate. [VERIFIED: phase157 evidence/retention.rs automatic fixture; ROADMAP.md]

Proposed focused commands (test names are planned, not currently existing): [VERIFIED: workspace package names; AGENTS.md timing contract]

```bash
bun run scripts/command-timings.ts run --key phase159-node -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase159 -- --nocapture
bun run scripts/command-timings.ts run --key phase159-rpc -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc phase159 -- --nocapture
bun test ./scripts/check-phase159-filter-rpcs.test.ts
bun scripts/bright-builds-check.ts all
bash scripts/verify.sh
```

## State of the Art

| Existing approach | Phase159 approach | Reason |
| -- | -- | -- |
| Recovery ancestry getter used internally | Separate bounded serving query | Current getter is O(height); recovery remains comprehensive. [VERIFIED: filters.rs] |
| Header/active metadata identity | Explicit accepted provenance history | Missing rows and stale reopens need genuine scripts-valid facts. [VERIFIED: inspected DTOs; Knots IsValid] |
| Initial synced latch only | Latch for summary, captured-frontier drain for read readiness | Matches pinned BaseIndex separation. [VERIFIED: base.cpp; BasicIndexProgress] |
| Generic serde normalizer | Dedicated pinned framework normalization | Current generic errors lose -3/type-order and -8 named contract. [VERIFIED: normalize.rs; Knots util/server] |

No ecosystem migration, deprecated dependency replacement or frontend change is part of this phase. [VERIFIED: CONTEXT.md scope; existing lockfile]

## Assumptions Log

No unverified factual assumptions are promoted to locked decisions. Proposed new types/schema/barrier plumbing are clearly recommendations and need implementation evidence; numeric query/wait budgets remain to be measured. Legacy SCRIPTS-valid history could not be recovered from inspected existing formats without inference. [VERIFIED: source inspection described above]

## Open Questions

All five questions are **RESOLVED FOR PLANNING** below. These outcomes close design selection; they do not claim implementation or measured verification is complete. [VERIFIED: 159-01–08-PLAN.md; 159-PLAN-CHECK.md research-resolution finding]

1. **Legacy acceptance history — RESOLVED FOR PLANNING.** Select explicit `ValidationProvenance::UnknownLegacy` and fail-closed missing-row behavior. Only genuinely empty-store coverage and trusted new header/accepted adapters may establish NeverConnected/ScriptsValid; do not infer historical validation from headers, snapshots, body/undo, active membership or filter presence. Valid stored lookup can still succeed. Plan159-01 Tasks1–2 define/recover the coverage ledger; Plan159-03 Tasks1–3 wire genuine acceptance and prove legacy/reopen/failure controls; Plan159-06 Task1 maps UnknownLegacy absence to fixed redacted internal failure; Plan159-08 Task1 documents compatibility. Exact historical backfill is excluded from this selected implementation because no authentic existing retained receipt was found. [VERIFIED: header/chain-meta/lineage formats; 159-01-PLAN.md; 159-03-PLAN.md; 159-06-PLAN.md; 159-08-PLAN.md; CONTEXT.md D-06]
2. **Async adapter/generic bounds — RESOLVED FOR PLANNING.** Select the narrow object-safe erased query backend accessor/hook on existing `ChainstateStore`, with a default disabled result for memory stores and concrete Fjall implementation; generic `ManagedNetworkHandle` continues to own serialization and read the same configured store. No downcast, specialization, new generic RPC-bound propagation, detached owner or independent RPC cache. Plan159-04 Task2 owns the backend/query/summary contract and opaque captured-frontier carrier; Plan159-05 Tasks1–2 make that carrier operational through a bounded std Future/waker adapter and existing owner completion/failure signals; Plan159-06 Tasks1–3 wire prepare/await/finish through the same context authority outside the HTTP context lock. [VERIFIED: 159-04-PLAN.md interfaces/Task2; 159-05-PLAN.md; 159-06-PLAN.md; CONTEXT.md D-05/D-10]
3. **Numeric read budget — RESOLVED FOR PLANNING.** Select codec/generated-bound derivation followed by concrete execution measurements, not an arbitrary small cap or latency promise. Plan159-04 Task1 implements checked target/parent envelope, validation/copy and response-expansion admission; Task3 measures genesis/20/400/stale and a legitimate maximum singleton, counts target and parent separately, confirms 2x hexadecimal expansion, and finalizes named numeric ceilings with exact-boundary/one-over controls. Preserve valid singleton admission and report actual counts/bytes/timings/ceilings in its SUMMARY; Plan159-08 Task3 includes the measured limits in final evidence. **Concrete measurements and resulting numeric values are pending execution**, while the procedure and owning tasks are selected. [VERIFIED: storage/filter_index.rs MAX_SIZE/RECORD_OVERHEAD; 159-04-PLAN.md Tasks1/3; 159-08-PLAN.md Task3; CONTEXT.md D-07]
4. **Raw duplicate names — RESOLVED FOR PLANNING.** Select scoped duplicate-preserving serde Deserialize visitors for these two methods at the HTTP boundary before `Value` maps collapse parameter pairs, retaining current behavior for unrelated methods. Plan159-02 Task2 supplies exact tuple/name/mixed normalization; Plan159-06 Task2 preserves raw single/batch parameter-object pairs and routes them to that normalizer; Task3 proves exact -8 duplicate/collision errors through authenticated HTTP with existing version/ID/notification semantics. Normalizer-only tests do not satisfy this selected transport requirement. [VERIFIED: http.rs current Value parsing; Knots rpc/server.cpp; 159-02-PLAN.md Task2; 159-06-PLAN.md Tasks2–3]
5. **Help serialization — RESOLVED FOR PLANNING.** Select complete pinned help-string fixtures for getblockfilter/getindexinfo, derived from the pinned method definitions and RPCHelpMan::ToString formatting; wrong arity returns -1 with the complete corresponding string. Plan159-02 Task2 owns fixtures and whole-string comparison, together with all arity/type/hash ordering controls; Plan159-06 Tasks1/3 register and verify the completed behavior through actual dispatch/HTTP; Plan159-08 Task2 guards exact parser/help evidence. A signature-only or substring assertion is insufficient. Fixture generation/comparison remains execution work and is not claimed as already verified. [VERIFIED: Knots rpc/util.cpp ToString/HandleRequest; rpc/blockchain.cpp getblockfilter; rpc/node.cpp getindexinfo; 159-02-PLAN.md Task2; 159-06-PLAN.md; 159-08-PLAN.md Task2; CONTEXT.md D-01]

## Environment Availability

| Dependency | Available | Version/evidence | Planner action |
| -- | -- | -- | -- |
| Node | Yes | 24.13.0 executable probe | GSD tooling available. [VERIFIED: node --version] |
| Bun | Yes | 1.3.9 at /tmp/open-bitcoin-bun-1.3.9-fresh/bun; PATH also contains ~/.bun/bin/bun | Use pinned runtime; root handles PATH. [VERIFIED: command -v; explicit --version] |
| Rust/Cargo | Executables found | rustup shims, toolchain 1.94.1 pinned | No Cargo build/test run by researcher; root owns serialized verification. [VERIFIED: command -v; rust-toolchain.toml; parent scope] |
| Bazel/Bazelisk | Executables found | Homebrew paths, .bazelversion 8.6.0 | No Bazel invocation by researcher; native root gate verifies. [VERIFIED: command -v; .bazelversion] |
| Knots baseline | Yes | HEAD a9aee730466ac67d35a3c03ee24676be5e045878 | Local source authoritative, no live-node/public-network dependency. [VERIFIED: git rev-parse] |
| External service | Not required | Hermetic Fjall + loopback fixtures | No credential provisioning or public endpoint. [VERIFIED: CONTEXT.md scope; fixture patterns] |

No missing external dependency blocks research. ASVS official source was consulted only for category mapping, not to introduce compliance, auth replacement or retention requirements. [VERIFIED: environment probes; security sources below]

## Security Domain

Security enforcement is enabled by absence of an explicit false. ASVS category numbering below refers explicitly to **4.0.3**, matching the workflow template; it is an applicability checklist, not a claim of current ASVS certification or the newest release. [VERIFIED: config.json; CITED: https://raw.githubusercontent.com/OWASP/ASVS/v4.0.3/4.0/OWASP%20Application%20Security%20Verification%20Standard%204.0.3-en.pdf]

| ASVS category | Applies to phase | Existing control / required evidence |
| -- | -- | -- |
| V2 Authentication | Yes | Reuse password/cookie HTTP auth before parsing/readiness; missing/wrong credentials disclose no state. [VERIFIED: http.rs; CITED: official ASVS4.0.3 PDF above] |
| V3 Session Management | No new session mechanism | HTTP Basic credentials are checked per request; cookie here is a credential file, not a browser session. Preserve existing lifecycle. [VERIFIED: http.rs resolve_auth/authorized; CITED: official ASVS4.0.3 PDF above] |
| V4 Access Control | Yes | Node scope root URI and shared trusted owner; raw client cannot mint accepted receipt/publication capability. [VERIFIED: http.rs validate_scope; CITED: https://raw.githubusercontent.com/OWASP/ASVS/v4.0.3/4.0/en/0x12-V4-Access-Control.md] |
| V5 Input Validation | Yes | Typed allowlisted args, strict hex, bounded envelopes/response allocations, safe JSON serialization. [VERIFIED: source normalizer/codec; CITED: https://raw.githubusercontent.com/OWASP/ASVS/v4.0.3/4.0/en/0x13-V5-Validation-Sanitization-Encoding.md] |
| V6 Stored Cryptography | Existing primitives only | Reuse tested project SHA256d commitment validator/getrandom cookie path; add no new crypto/auth scheme. Public filter commitments are not secret encryption. [VERIFIED: storage/filter_index.rs; http.rs; CITED: https://raw.githubusercontent.com/OWASP/ASVS/v4.0.3/4.0/en/0x14-V6-Cryptography.md] |

| Threat | STRIDE | Required mitigation |
| -- | -- | -- |
| Unauthenticated index oracle | Information disclosure | Authentication before JSON/context/barrier; malformed unauthorized control. [VERIFIED: http.rs; D-11] |
| Forged/missing record/header | Tampering | Target+parent commitment validation plus recovered integrity epoch and raw-writer invalidation. [VERIFIED: codec and PublicationControl] |
| Fake connected state from body/fixture | Spoofing/Tampering | Sealed accepted receipt ledger, monotonic upgrade and explicit legacy unknown. [VERIFIED: D-06; acceptance seams] |
| Height-dependent request work / stalled lock | Denial of service | Fixed point reads, checked byte accounting, async captured frontier outside locks, failure signaling. [VERIFIED: current O(height) getter; D-07/D-10] |
| Backend path/error leakage | Information disclosure | Fixed public messages; internal typed detail stays internal, auth/debug formatting reviewed. [VERIFIED: D-11; existing error adapters] |
| Reorg/disable interleaves with read | Tampering | Same-authority capture and generation/branch recheck; no successful mixed snapshot. [VERIFIED: runtime_authority.rs; lifecycle generation patterns] |

## Sources

### Primary — HIGH confidence

All local source claims were checked against this checkout/pin; links below provide immutable upstream attribution. [VERIFIED: git submodule HEAD; cited local reads]

- [Pinned getblockfilter](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/blockchain.cpp#L3317): handler, exact result/error order.
- [Pinned getindexinfo](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/node.cpp#L409): summary name/selection.
- [Pinned RPC framework](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/util.cpp#L657): arity/help, aggregate type validation; ParseHashV line120; MatchesType line960.
- [Pinned named conversion](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/server.cpp#L439): names, holes, mixed args, duplicates.
- [Pinned BaseIndex](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/base.cpp#L367): readiness; init line81; BlockConnected line278; summary line415.
- [Pinned filter lookup](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/blockfilterindex.cpp#L436): stored filter/header lookup.
- Local node source: chainstate.rs; chainstate/filter_index.rs; chainstate/fjall_store.rs and reorg.rs; network/runtime_authority/filter_index/catch_up.rs; storage/filter_index.rs; storage/fjall_store/filters.rs and ownership/publication/startup modules.
- Local RPC source: context.rs/context/network.rs; method.rs/method/normalize.rs; dispatch.rs/dispatch/decode.rs; error.rs; http.rs/http/request.rs; bin/open-bitcoind.rs and coins_flush.rs.
- Canonical refs: 154–159 contexts; ROADMAP/REQUIREMENTS/PROJECT/STATE; milestone research ARCHITECTURE/PITFALLS/FEATURES; repository instructions/standards; scripts/verify.sh; docs/parity/source-breadcrumbs.json.
- Official OWASP ASVS 4.0.3 PDF and V4/V5/V6 raw pages cited in Security Domain; mapped only to this RPC/storage scope.

### Secondary / Tertiary

None used for implementation claims. Context7 is not exposed in this session; no new third-party API is needed, and the local pinned source outranks current web behavior for Bitcoin parity. [VERIFIED: available tool metadata; CONTEXT.md canonical pin]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — checked pins, lockfile and current executable availability. [VERIFIED: environment/source probes]
- Baseline RPC behavior: HIGH — direct pinned handler, framework and index source. [VERIFIED: listed Knots sources]
- Architecture: MEDIUM — seams verified; new provenance ledger/read capability/frontier barrier require implementation/fault evidence. [VERIFIED: inspected existing seams; proposals above]
- Pitfalls: HIGH — explicit current source differences and reusable failure/retention fixtures. [VERIFIED: listed files]
- Legacy absence design: RESOLVED FOR PLANNING — explicit UnknownLegacy and redacted fail-closed absence; historical exact backfill is not inferred. Implementation/reopen evidence remains pending execution. [VERIFIED: inspected DTOs/lineage; 159-01/03/06-PLAN.md; resolved question1]

**Research date:** 2026-10-09
**Valid until:** source/pin change; otherwise 2026-11-08 for stable local architecture. [VERIFIED: fixed baseline and current checkout scope]

No source edits, Cargo/Bazel commands, state/roadmap mutations or commits were performed by this researcher; root owns strict finalization. [VERIFIED: task execution scope]
