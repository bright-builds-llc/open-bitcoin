---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
---

# Phase 161: Shared Operator Index and Retention Evidence - Research

**Researched:** 2026-10-11 UTC
**Domain:** Existing Rust/Fjall index authority, bounded logical accounting and terminal operator evidence
**Confidence:** HIGH for existing seams; MEDIUM for proposed accounting implementation until behavioral verification

<user-constraints>
## User Constraints (from CONTEXT.md)

The following decisions, discretion and deferred scope are copied verbatim from 161-CONTEXT.md. [VERIFIED: .planning/phases/161-shared-operator-index-and-retention-evidence/161-CONTEXT.md]

### Locked Decisions

### One shared status contract

- **D-01:** Extend the existing `OpenBitcoinStatusSnapshot` and authoritative network operator snapshot with a typed BASIC section. RPC, CLI, dashboard and support must consume that projection, never independently guess state from config, service bits, RPC success or height equality. Keep existing `getindexinfo` and `getblockfilter` pinned result shapes unchanged.
- **D-02:** Show separately configured BASIC indexing, configured general peer serving and effective per-peer capability scope. Preserve Phase 160's explicit/all/implicit permission distinction; do not infer general service from one permissioned peer or infer a working index from `all` capability advertisement.
- **D-03:** Expose initial synchronization as the existing owner's latch, current lag against its accepted active target, processed endpoint and safe durable endpoint as distinct facts. Include the no-genesis/empty-prefix case without inventing height-zero completion. Later lag or reorg does not clear the earned initial-sync latch. Header height and processed rows cannot authorize durable progress.
- **D-04:** Preserve explicit availability through `FieldAvailability` and backward-compatible serde defaults. A stopped collector may report configured facts only when authoritatively known, while live progress, retention measurements and failures remain unavailable. The existing stopped JSONC config has no BASIC fields; do not infer false from absent fields, add a second resolver or claim disk configuration captures unseen daemon CLI overrides. Missing fields from an older daemon remain unavailable rather than disabled, caught up, zero bytes or healthy. Collection is read-only and must not activate/recover/index/network as a status side effect.

### Failure and retained availability semantics

- **D-05:** Use fixed low-cardinality categories for missing required body, missing required undo, persistence/backend failure, index integrity/invalid facts, resource-bound pause and lifecycle invalidation where the existing owner supplies those distinctions. Retain actionable, bounded guidance; never serialize raw backend errors, scripts, storage keys, paths or peer identifiers into this shared evidence.
- **D-06:** Keep failure/pause and retained-filter availability independent. A failed or lagged index can retain previously indexed active/stale filter records; availability evidence must describe what was actually observed and must not promise every arbitrary lookup succeeds. Integrity/backend faults must never become ordinary absence, an empty successful filter or healthy zero counts.
- **D-07:** Read coherent progress/protection/failure facts from the existing serialized chainstate/index authority, with the existing integrity/lifecycle epoch. Status must not clear sticky faults, release protection, advance a checkpoint or recreate missing history. Required body/undo refusal remains fail closed and implies no automatic repair, reconstruction or download.
- **D-08:** Preserve privacy in live shared evidence and exported support: fixed enums, booleans, heights and aggregate counts only for new BASIC facts. Redacted support agrees with the captured live snapshot, does not mutate the caller's snapshot, and excludes raw scripts/undo/filter payloads, tx/block/peer identifiers, addresses, credentials, backend keys, filesystem paths and raw lock names.

### Separate retained-index growth and prune protection

- **D-09:** Report retained BASIC record count and logical encoded filter/index bytes separately from the existing soft logical block/undo prune target. State the precise measurement basis. Include retained displaced records rather than presenting only the active prefix as total retained growth; immutable retries and reorg reuse must not double count.
- **D-10:** Collection must have explicit finite work/allocation limits and avoid a full index/history scan on every status poll. Research chooses the simplest recoverable accounting seam in existing publication/recovery authority; cached or persisted aggregates require tested freshness/error/reopen behavior. Incomplete, failed or over-budget measurements remain explicitly unavailable, never plausible partial totals. No new independent database, production crate, dependency or unbounded cache.
- **D-11:** Expose index-owned protection and its retention implication separately from growth and target accounting. Explain that protection can keep block/undo bytes above a soft target. Do not invent counterfactual physical bytes saved/blocked; show only authoritative protection bounds or measured retained facts. Preserve reserved ownership and existing prune semantics.
- **D-12:** Useful retained filters do not restore deleted block bodies, ordinary body-serving eligibility or wallet eligibility. Render this distinction in concise operator copy and documentation. The target is not a filesystem-size/disk-capacity guarantee and filter growth is not charged to it.

### Existing terminal consumers and evidence

- **D-13:** Reuse compact label/value rows and the existing terminal layout, styling, refresh cadence and explicit `Unavailable: {reason}` convention. Add a readable BASIC/index section without a new screen, new navigation, hosted frontend, chart family or controls. Preserve the existing four-band/eight-chart maximum. Generate a terminal-scoped UI contract and validate normal and narrow rendering behavior.
- **D-14:** Prove consumers agree for disabled/configured-only, initial catch-up, caught-up, earned-initial-sync plus later lag, processed-ahead-of-durable, missing body/undo, persistence/integrity/resource failure, retained stale/pruned records, protected retention, stopped collection and legacy missing fields. Use existing genuine validated-chain/Fjall fixtures and real RPC/CLI/support/dashboard callers; synthetic snapshots alone are insufficient for authoritative collection proof.
- **D-15:** Update relevant README/operator/status/parity docs and Rust parity breadcrumbs, add a narrowly scoped deterministic native claim/mutation checker where established patterns require it, and preserve historical guards. Provide copy-pasteable repo-local Cargo/Bazel UAT commands. Full `bash scripts/verify.sh`, independent source/security review, a simplification pass and lifecycle-valid formal verification gate commit/push.
- **D-16:** Preserve Phase 162 ownership and v2.5 pending claims. No BIP37/V0, filter import, wallet acceleration, reindex/repair/download, archive-scale, assumeutxo, public defaults/network CI, production service/readiness or funds-safety expansion. Status/accounting evidence does not strengthen existing v2.4 crash-window guarantees.

### Agent Discretion

Research and planning choose exact Rust enum/schema names, compact copy, bounded accounting mechanics, module seams and plan decomposition. Respect existing external schemas and established crate boundaries; prefer the simplest robust projection. No pending todos matched this phase.

## Deferred Ideas

Phase 162 owns the complete continuous validated-chain prune/reopen/RPC/both-transports integrated proof and remaining milestone claims. All excluded surfaces in D-16 remain deferred. No pending todos matched Phase 161.
</user-constraints>

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| CFOP-01 | An operator can distinguish configured capability, initial catch-up, current lag, safe durable progress, missing history and index failures consistently across existing status RPC, repo-local CLI, dashboard and redacted support evidence; retained filter growth is separate from the soft block/undo prune target. | Shared authority projection; detailed owner failure mapping; guarded recovered accounting; production collection/consumer test matrix. [VERIFIED: .planning/REQUIREMENTS.md] |
</phase-requirements>

## Summary

Extend the existing network operator snapshot, OpenBitcoinNetworkStatusResponse and OpenBitcoinStatusSnapshot with one typed BASIC section. The actual existing wire method is **openbitcoinnetworkstatus**, without a get prefix. CLI live collection calls that method and then supplies the snapshot to human/JSON output, dashboard and support. Existing schemas have no BASIC section. [VERIFIED: packages/open-bitcoin-rpc/src/method.rs; packages/open-bitcoin-rpc/src/method/node.rs; packages/open-bitcoin-cli/src/operator/status/http.rs; packages/open-bitcoin-cli/src/operator/status.rs; packages/open-bitcoin-node/src/status.rs]

Reuse BasicIndexProgress and the private AcceptedBasicIndexFailure together: progress already separates the historical initial-sync latch, accepted target, processed endpoint, safe durable endpoint and protection. Its pause enum alone is insufficient: FactLimit and InvalidFacts currently collapse into MissingHistory. Reorg acceptance can also expose a replacement accepted target independently of the progress target; an equal height does not establish branch coherence. [VERIFIED: packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs; packages/open-bitcoin-node/src/chainstate/filter_index.rs]

**Primary recommendation:** recover a constant-size retained immutable-record aggregate inside the already-required startup integrity traversal, then update it only from existing newly inserted encoded rows after successful guarded publication. Keep the aggregate observation separate from durable checkpoint authority. Use no new persisted schema, database, dependency, background scanner or history cache. Treat this as a proposed implementation under D-10, with mandatory real-store freshness/fault/reopen tests. [VERIFIED: 161-CONTEXT.md D-09/D-10; packages/open-bitcoin-node/src/storage/fjall_store/filters.rs; packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs]

## Project Constraints (from AGENTS.md)

- Preserve pinned Knots 29.3.knots20260210 observable behavior; production code must not add Rust Bitcoin libraries. Keep pure decisions free of I/O and effects in existing shell adapters. [VERIFIED: AGENTS.md]
- Use Rust 1.94.1, Bun automation, existing Cargo workspace and Bazel/Bzlmod. No package.json or bun-install bootstrap applies. Run ad-hoc Cargo/Bazel through scripts/command-timings.ts and avoid overlapping Cargo targets. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Full bash scripts/verify.sh is the pre-commit/release contract; fast mode is iteration only. Preserve managed hooks and tracked generated docs/metrics/lines-of-code.md. Quiet/yielded commands require liveness polling, not automatic termination. [VERIFIED: AGENTS.md]
- New first-party Rust source/test files require registered parity breadcrumbs; intentional differences require docs/parity/index.json and companion evidence. Review relevant READMEs after operator/workflow changes. [VERIFIED: AGENTS.md]
- Work is within gsd-plan-phase. The strict outer wrapper defers **all** nested commits until clean native, independent source/security review, simplification and lifecycle-valid formal verification. Do not commit this research separately despite commit_docs:true. [VERIFIED: 161-CONTEXT.md D-15; parent task authorization]
- Use stable append-only task/lesson sections; only the parent owns those artifacts in this delegation. This researcher owns only 161-RESEARCH.md. [VERIFIED: AGENTS.md; parent task authorization]
- Prefer enums/newtypes, early guards, maybe_ optional names and foo.rs plus foo/ modules; meaningful pure behavior needs focused Arrange/Act/Assert tests. Do not edit managed standards or suppress checks in managed scripts. [VERIFIED: AGENTS.bright-builds.md; standards/core/architecture.md; standards/core/code-shape.md; standards/core/testing.md; standards/languages/rust.md]
- GSD frontmatter delimiters occur only at file top; use headings rather than body horizontal frontmatter markers. UAT examples must show repo-local Cargo/Bazel invocations. [VERIFIED: supplied global AGENTS instructions; AGENTS.md Repo-Local Guidance]
- standards-overrides.md has only placeholder entries; it supplies no operative exception. No project skill indexes were found under .claude/skills or .agents/skills during scouting. [VERIFIED: standards-overrides.md; filesystem discovery]

## Standard Stack

Use the existing locked stack; this is not a dependency-selection or upgrade phase. No recommended new package requires npm/crates registry installation or latest-version substitution. Versions below are repository locks, not claims of newest releases. [VERIFIED: 161-CONTEXT.md D-10/D-16; packages/Cargo.lock]

| Component | Repository version / pin | Purpose |
| --- | --- | --- |
| Rust / edition | 1.94.1 / 2024 | Existing typed domain and adapter code. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml] |
| Fjall | 3.1.10 at aa30dca811399a201e0b9595da93a4582dcb2b57 | Existing one-database guarded SyncAll publication. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/Cargo.lock] |
| serde / serde_json | 1.0.228 / 1.0.149 | Existing tagged availability and additive serde defaults. [VERIFIED: packages/Cargo.lock; packages/open-bitcoin-node/src/status.rs] |
| Ratatui / Crossterm | 0.30.0 / 0.29.0 | Existing terminal dashboard only. [VERIFIED: packages/Cargo.lock] |
| Bun / Bazel | 1.3.9 / 8.6.0 | Deterministic scripts and existing root build. [VERIFIED: .bun-version; .bazelversion] |

Fjall's pinned source describes OwnedWriteBatch as atomic across database keyspaces. Existing application code supplies SyncAll and handles failed/ambiguous publication by poisoning the shared control. Latest web docs advertise 3.1.12; do not silently adopt that version or its behavior over the actual git pin. [VERIFIED: /Users/peterryszkiewicz/.cargo/git/checkouts/fjall-3c0b80f4d3d9dab4/aa30dca/src/batch/mod.rs; packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs] [CITED: https://docs.rs/fjall/latest/fjall/struct.OwnedWriteBatch.html]

## Architecture Patterns

### Production integration map

All relative paths in this table begin under packages/. Add small sibling BASIC modules rather than enlarging central source registries. [VERIFIED: standards/core/code-shape.md; existing module layouts]

| Layer | Exact integration files | Required change |
| --- | --- | --- |
| Domain progress / private faults | open-bitcoin-chainstate/src/filter_index/catch_up.rs; open-bitcoin-node/src/chainstate/filter_index.rs | Pure projection from owner progress plus detailed failure; no storage or network in domain decisions. [VERIFIED: source files] |
| Retained observation | open-bitcoin-node/src/storage/fjall_store/filters.rs; filters/startup.rs; filters/append.rs; filters/publication.rs; filters/publication/epoch.rs | O(1) aggregate, startup recovery, achieved delta, invalidation. Audit compatibility publishers and reorg/lifecycle paths too. [VERIFIED: source files] |
| Raw invalidation | open-bitcoin-node/src/storage/fjall_store/payload_usage.rs | Raw basic_filter key put/remove already invalidate integrity before effects; revoke aggregates there through the common control method. [VERIFIED: source file] |
| Shared authority | open-bitcoin-node/src/network/operator_snapshot.rs; network/types.rs; network/runtime_authority.rs | Capture BASIC alongside other network facts under one authority guard; no second BASIC read after releasing it. [VERIFIED: source files] |
| Wire schema / dispatch | open-bitcoin-rpc/src/context/inbound_status.rs; method/node.rs; dispatch/node.rs | Pass projection through AuthoritativeOperatorSnapshot and openbitcoinnetworkstatus. Preserve baseline filter/index RPC shapes. [VERIFIED: source files; 161-CONTEXT.md D-01] |
| CLI collection | open-bitcoin-cli/src/operator/status.rs; operator/status/http.rs; operator/config.rs | Copy server BASIC facts; handle missing old-server field, failed RPC and stopped input explicitly. [VERIFIED: source files] |
| Shared status / human output | open-bitcoin-node/src/status.rs; open-bitcoin-cli/src/operator/status/render.rs | Defaulted BASIC contract and compact label/value rendering. [VERIFIED: source files] |
| Dashboard | open-bitcoin-cli/src/operator/dashboard/model.rs; dashboard/app.rs; sibling model module | Consume the snapshot; preserve bands/charts/cadence and verify clipping. [VERIFIED: source files; 161-CONTEXT.md D-13] |
| Support | open-bitcoin-cli/src/operator/support.rs; support/redaction.rs; support/render.rs | Capture existing snapshot, sanitize BASIC unavailable reasons, render bounded facts and guidance. [VERIFIED: source files] |

### Recommended typed contract

Use a dedicated status/basic_filter_index.rs containing serde-facing projection types and pure mapping helpers. Keep configured capability, progress, observed retained records and protection in independent availability fields; one unavailable measurement must not erase trustworthy configured facts. This follows the existing PruneOperatorStatus / ChainstateDurabilityEvidence seams. [VERIFIED: packages/open-bitcoin-node/src/status/prune_operator.rs; packages/open-bitcoin-node/src/status/chainstate_durability.rs; 161-CONTEXT.md D-01/D-04/D-06]

Recommended fields: configured indexing, configured general serving, fixed per-peer capability policy/scope, operational state/failure category, initial_sync_complete, accepted_height, optional processed_height, optional safe_durable_height, lag_blocks, input-protection boundary, and retained-record observation. Internally compare generation, branch hashes, publication identity and durable fence, but export **no new hashes or identifiers**. A missing endpoint is None, not height zero. [VERIFIED: 161-CONTEXT.md D-02/D-03/D-08; packages/open-bitcoin-node/src/chainstate/filter_index.rs; packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs]

Use FieldAvailability defaults at both wire/shared snapshot boundaries. For absent old-daemon fields, use a fixed unavailable reason, not false/zero/default healthy. For an enabled fresh prefix with accepted genesis and no processed row, lag is 1 and the endpoint is absent. For earned initial sync followed by later lag, preserve true plus nonzero lag. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs; 161-CONTEXT.md D-03/D-04]

### Configured capability without inference

BasicPeerServingCapability already contains general_configured, effective_permission and explicit_permission_requested. Reuse its policy, including the all-only exception: per-peer capability may exist with no enabled backend; implicit permission expansion must not confer filter capability. Use fixed scope classes or aggregate counters maintained at existing peer lifecycle transitions; do not allocate a peer table or iterate all sessions on each status poll. [VERIFIED: packages/open-bitcoin-network/src/filter_serving/capability.rs; packages/open-bitcoin-network/src/inbound/permissions.rs; 161-CONTEXT.md D-02/D-10]

General configured service currently reaches local service flags through network/limited_serve.rs. Retain the resolved indexing configuration as a narrow shared-authority fact at existing construction/startup if necessary; a backend error must not turn configured=true into configured=false. Do not derive index configuration from a successful lookup or synchronization latch. [VERIFIED: packages/open-bitcoin-node/src/network/limited_serve.rs; packages/open-bitcoin-rpc/src/config.rs; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; 161-CONTEXT.md D-01/D-02]

**Stopped-collector gap:** OpenBitcoinConfig JSONC contains prune but has no BASIC/peer-serving fields. Actual blockfilterindex/peerblockfilters resolve in config/loader.rs from baseline config and daemon CLI options into RuntimeConfig. The smallest honest stopped behavior is unavailable capability when the collector has no authoritative resolution; it may show configured facts only from inputs actually resolved. If adding configured-only collection, reuse that resolver in bounded read-only mode and state that unseen daemon CLI overrides are unavailable. Do not add a duplicate parser or infer disabled from JSONC omission. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; config/loader.rs; config/loader/blockfilter.rs; packages/open-bitcoin-cli/src/operator/config.rs; 161-CONTEXT.md D-04]

### Exact failure taxonomy and precedence

| Existing private input | Recommended public category | Bounded guidance |
| --- | --- | --- |
| RequiredBody | missing_required_body | Required historical body unavailable; preserve protection; no automatic reconstruction/download. [VERIFIED: node/src/chainstate/filter_index.rs; 161-CONTEXT.md D-05/D-07] |
| RequiredUndo | missing_required_undo | Required historical undo unavailable; preserve protection. [VERIFIED: same sources] |
| Persistence | persistence_backend_failure | Safe progress has not advanced from this failed effect; follow existing restart/diagnostic workflow. [VERIFIED: same source; storage/fjall_store/filters/publication.rs] |
| InvalidFacts | index_integrity_failure | Invalid evidence cannot authorize progress; preserve state and investigate. [VERIFIED: node/src/chainstate/filter_index.rs] |
| FactLimit | resource_bound_pause | Local accounting/input cap refused work; no invented missing-history claim. [VERIFIED: same source] |
| Invalidated | lifecycle_invalidated | Captured generation/branch authority is invalid; observe ordinary owner recovery, never status-triggered repair. [VERIFIED: same source; 161-CONTEXT.md D-07] |

Inspect detailed failure before the coarser BasicIndexPause. For a store-read error classify StorageError variants through fixed typed categories; never inspect/serialize arbitrary Display text to decide a public machine class. Keep failed operational state and retained records independent. Observed trustworthy retained rows do not imply a working arbitrary lookup; unobserved backend corruption cannot be excluded by a constant-size status query. [VERIFIED: packages/open-bitcoin-node/src/chainstate/filter_index.rs; packages/open-bitcoin-node/src/storage.rs; packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs; 161-CONTEXT.md D-05/D-06]

### Coherent capture, reorg and poison behavior

1. Acquire the existing network authority, obtain manager progress, detailed failure and maybe_basic_index_accepted_target, then acquire publication control in existing authority -> publication order. Capture only one BASIC observation. [VERIFIED: packages/open-bitcoin-node/src/network/runtime_authority.rs; packages/open-bitcoin-node/src/chainstate/filter_index.rs; storage/fjall_store/filters/lifecycle.rs]
2. Check current lifecycle generation/read-integrity and branch/reorg state against existing authenticated identities; do not establish a new append/recovery proof or walk ancestry for status. PreviewFrozen and AcceptedReplacement are not ordinary processed progression. When a coherent active-branch endpoint cannot be established, report its progress unavailable/lifecycle-invalidated rather than subtracting unrelated equal-height endpoints. [VERIFIED: chainstate/filter_index.rs; chainstate/filter_reorg.rs; storage/fjall_store/filters/ownership.rs; 161-CONTEXT.md D-03/D-07]
3. Safe progress must come from the owner's already-earned safe endpoint and compatible current fence, never batch byte counts, headers or retained total. Retained rows can be ahead of safe progress or on displaced branches. [VERIFIED: chainstate/filter_index.rs; storage/fjall_store/filters/append.rs; 155-CONTEXT.md D-04/D-05; 158-CONTEXT.md D-05/D-06]
4. Publication poison returns an error from filter_publication_guard before inspecting old control values. Convert BASIC-specific errors into unavailable BASIC evidence while other coherent snapshot groups remain usable; do not bypass the guard to read stale healthy counters. If the outer network mutex is poisoned, preserve existing RPC failure semantics and let CLI produce unavailable live evidence. [VERIFIED: storage/fjall_store/filters/publication.rs; packages/open-bitcoin-rpc/src/dispatch/node.rs; packages/open-bitcoin-cli/src/operator/status.rs]
5. Status collection never mutates checkpoints, counters, sticky faults, read epochs, locks or ownership. Repeated status calls must not readmit raw-invalidated data. A real query discovering integrity/backend failure must remain a failed query; any newly added observed-fault latch belongs to its existing error path, not to a status repair pass. [VERIFIED: 161-CONTEXT.md D-04/D-06/D-07; storage/fjall_store/filters/query.rs]

## Recoverable Bounded Retained Accounting

### Measurement basis

Recommend three scalar totals: retained_record_count, encoded_filter_payload_bytes, and retained_record_key_value_bytes. The last includes each immutable record's exact key plus versioned envelope and encoded filter value. Include every unique retained record, active, displaced and ahead/unclaimed. Exclude active projections, state/owner metadata, prune-lock map, other keyspaces, WAL/SST/compaction/allocator/filesystem overhead. Label it explicitly **logical retained BASIC record key/value bytes**, not total database size or a physical capacity promise. This narrow basis satisfies separate growth disclosure without coupling to mutable projection/lock metadata. [VERIFIED: 161-CONTEXT.md D-09/D-10/D-12; packages/open-bitcoin-node/src/storage/filter_index.rs; storage/fjall_store/filters.rs]

The codec envelope is 170 bytes. A key is RECORD_PREFIX plus 64 hex characters; count its actual byte length rather than duplicating a magic key-size formula. Count payload length from already-validated borrowed data or validated value length minus the envelope; avoid decoding/filter-copying again. Immutable record count is not processed height plus one. [VERIFIED: packages/open-bitcoin-node/src/storage/filter_index.rs; storage/fjall_store/filters.rs; storage/fjall_store/filters/append.rs]

### Recovery and publication mechanics

- Add an internal scan result/accumulator to the existing validate_basic_filter_records traversal, not a second traversal. Existing startup always performs a final full integrity scan after applying Enabled/Disabled/PreserveSaved mode, including disabled stores with retained rows. Populate the shared aggregate only after the complete validation succeeds. Parent/active-projection validations remain intact. [VERIFIED: storage/fjall_store/filters.rs::validate_basic_filter_records; filters/startup.rs::configure_basic_filter_index_before_prune]
- Existing scan has no publication lock acquisition of its own; callers may already hold that lock. Return the summary to the guarded caller rather than locking the same mutex recursively. Preserve a compatibility wrapper if historical guards/reference callers need the old function shape. [VERIFIED: filters.rs; filters/startup.rs; filters/lifecycle.rs; filters/publication.rs]
- Put only constant-size aggregate state in PublicationControl, shared by store clones. Reopen recomputes from actual rows and restores availability; no schema change, migration or persisted aggregate authority is needed. Overflow, measurement budget exhaustion, invalidation and corruption have explicit non-success states. [VERIFIED: fjall_store.rs; filters/publication.rs; 161-CONTEXT.md D-10]
- prepare_basic_filter_append already compares existing encoded rows and places **only missing records** into prepared.records. Sum those already-encoded key/value lengths before effects, with checked arithmetic. Identical retry/reorg reuse contributes zero. Do not perform another row decode, query or payload clone just for accounting. [VERIFIED: filters/append.rs::prepare_basic_filter_append_guarded_positions]
- Compute prospective totals before effects, but install them only after finish_basic_filter_batch returns success while still under publication control. A successful batch followed by injected AfterCommit failure leaves totals unavailable; reopen measures the actually committed forest. Never publish prospective totals on error or independently advance processed/safe progress. [VERIFIED: filters/append.rs::complete_basic_filter_append; filters/publication.rs::finish_basic_filter_batch; filters/publication.rs::after_commit_fault]
- Audit every immutable publisher: append.rs, persist_basic_filter_records and publish_basic_filter_checkpoint_guarded in publication.rs, and initialization/lifecycle/reorg paths. Compatibility publishers that can rewrite identical rows must compute a new-row-only delta using their existing comparison pass. If a path cannot prove its delta under current admission budgets, revoke observation rather than return a partial total or trigger a request scan. Metadata-only batches preserve record totals. [VERIFIED: filters/publication.rs; filters/append.rs; filters/lifecycle.rs; filters/reorg.rs]
- Do **not** clear record totals in invalidate_append: ordinary coins/metadata/checkpoint revisions call it despite leaving immutable rows unchanged. Clear/revoke on invalidate_read_integrity before public raw basic_filter writes, on poison/ambiguous batch outcome, or on checked accounting overflow. Disable advances integrity capture epoch but does not delete records; preserve/rebind current aggregate observation without reviving old read capabilities. [VERIFIED: filters/publication/epoch.rs; filters/publication.rs; fjall_store/payload_usage.rs; fjall_store/coins.rs; filters/lifecycle.rs]
- If a raw write occurs between preparation/completion, existing append proof revision checks must reject stale work before totals install. A later trusted small append cannot readmit an unknown forest. Only complete existing recovery validation may restore aggregate trust. [VERIFIED: filters/append.rs; filters/ownership.rs; fjall_store/payload_usage.rs; 161-CONTEXT.md D-07/D-10]

### Finite limits for planning

Existing limits below are verified. New observation limits are proposed local policy under delegated D-10 discretion, not claims of measured latency or production archive scale. Make new limits injectable for exact-limit/over-limit tests. [VERIFIED: 161-CONTEXT.md D-10/D-16; packages/open-bitcoin-chainstate/src/filter_index/catch_up/budget.rs]

| Operation | Bound / policy | Planning consequence |
| --- | --- | --- |
| Status BASIC capture | Propose at most 8 bounded metadata reads, zero record/history/body/undo traversals, zero filter payload copies, and at most 4 KiB serialized BASIC section | Instrument point reads and verify repeated polls have constant work independent of retained count. This bounds the new BASIC group, not existing prune/other status collectors. [VERIFIED: existing query/owner seams; proposed D-10 policy] |
| Startup observation | Propose up to 2,000,000 immutable rows and 256 GiB cumulative record key/value bytes | Stop aggregation when its budget is exceeded; return unavailable measurement. Existing mandatory integrity validation still completes or fails under its existing contract. Do not weaken recovery or reject a valid index solely because optional observation exceeded its budget. [VERIFIED: filters.rs; 161-CONTEXT.md D-10; proposed policy] |
| Observation allocation | Three checked u64 counters plus fixed state; no per-record collection | Borrow each existing validator row; no cache or extra complete encoded index. [VERIFIED: filters.rs; 161-CONTEXT.md D-10; proposed policy] |
| Normal index turn | 8 blocks; 1 MiB encoded; 512 record operations; 256 projection operations; 1,000,000 checkpoint operations | Reuse existing accounting; counter delta must not secretly add uncharged storage queries. [VERIFIED: node/src/chainstate/filter_reorg/preflight.rs::production_budget] |
| Publisher absolute candidates | 128; encoded envelope ceiling 0x02000000 + 128*170 | Do not create a new unbounded publisher through accounting. [VERIFIED: chainstate/src/filter_index/catch_up/budget.rs] |
| Singleton envelope | 0x02000000 + 170 bytes | Existing record admission remains authoritative; new accounting must handle its exact length. [VERIFIED: same source; node/src/storage/filter_index.rs] |
| Poll failure policy | Fixed unavailable category; no retry scan, activation or history acquisition | A budget failure is never successful zero/partial growth. [VERIFIED: 161-CONTEXT.md D-04/D-07/D-10] |

Actual end-to-end/lock-hold timings must be measured during implementation using existing measurement fixtures; no Rust tests/builds were run by this researcher. Structural budgets avoid brittle elapsed-time assertions in the default hermetic gate. [VERIFIED: parent task constraint; existing sync/tests/filter_index/reorg/measurements.rs]

### Protection versus target

Expose IndexInputProtection::FromHeight(h) or explicit height-space exhaustion from current owner facts; show the reserved protection bound without the raw lock name. Existing lock rendering includes the ten-block helper buffer and caps the final range; do not invent a second arithmetic policy in CLI. Keep soft target bytes on existing PruneOperatorStatus. Protection can retain required bodies/undo above target; filter-record bytes remain separately disclosed. [VERIFIED: packages/open-bitcoin-chainstate/src/filter_index.rs; prune/locks.rs; node/src/status/prune_operator.rs; 153-CONTEXT.md D-01/D-02; 161-CONTEXT.md D-11]

Automatic retention already measures actual body/undo values including nonactive/protected bytes and coalesces exact scans by revision and a 60-second interval. Its state stores a completed identity, not a ready public snapshot of all measured totals. Do not add another full body/undo scan just to estimate bytes held by BASIC; expose only the authoritative protection bound unless reusing an already-earned measurement. [VERIFIED: node/src/storage/fjall_store/payload_usage.rs; network/runtime_authority/automatic_prune.rs]

## Don't Hand-Roll

| Problem | Use existing mechanism | Why |
| --- | --- | --- |
| Initial sync / lag / safe progress | BasicIndexProgress plus actual accepted/reorg owner | Height equality alone loses empty-prefix, latch and branch distinctions. [VERIFIED: chainstate/filter_index/catch_up.rs; node/chainstate/filter_index.rs] |
| Per-peer capability policy | BasicPeerServingCapability / PeerPermissionSet | Preserve explicit/all/implicit provenance and prerequisites. [VERIFIED: network/filter_serving/capability.rs] |
| Freshness ownership | PublicationControl and existing epoch/revision guards | Store clones must observe raw invalidation and poison consistently. [VERIFIED: fjall_store.rs; filters/publication/epoch.rs] |
| Availability shape | FieldAvailability plus serde defaults | Older daemon/stopped evidence must not become zero or disabled. [VERIFIED: node/status.rs; 161-CONTEXT.md D-04] |
| Duplicate record identity | Existing prepare append comparison | Avoid a duplicate hash set or second record decode/copy. [VERIFIED: filters/append.rs] |
| Support redaction | Existing cloned-snapshot sanitization and fixed reasons | Keep operator facts aligned without exporting arbitrary error strings. [VERIFIED: cli/operator/support/redaction.rs] |

## Common Pitfalls

1. **Coarse pause taxonomy:** FactLimit/InvalidFacts currently map to MissingHistory. Test detailed failure mapping separately; retain the detailed failure getter. [VERIFIED: node/src/chainstate/filter_index.rs]
2. **Success-only totals:** committed-after-error rows survive reopen; totals from prospective batches are not achieved evidence. Test AfterCommit faults and actual drop/reopen. [VERIFIED: filters/publication.rs; filters/startup.rs]
3. **Invalidation too broad:** clearing totals on every invalidate_append loses valid growth after ordinary coins flush. Test normal checkpoint/flush with unchanged record totals. [VERIFIED: fjall_store/coins.rs; filters/publication.rs]
4. **Invalidation too narrow:** raw key mutation revokes read integrity before effects. A cached count cannot remain healthy after that, even if a later append is otherwise valid. [VERIFIED: fjall_store/payload_usage.rs; 161-CONTEXT.md D-07]
5. **Store poison erases fault:** filter_publication_guard returns before exposing values. Downgrade only the BASIC group, with typed fixed diagnostics, rather than inventing zero, bypassing poison or stopping all otherwise-available groups. [VERIFIED: filters/publication.rs; rpc/dispatch/node.rs; 161-CONTEXT.md D-04/D-06]
6. **Counting active height as retained rows:** displaced and ahead rows survive. Include forest rows once; projections/reorg metadata do not change immutable count. [VERIFIED: filters.rs; filters/reorg.rs; 158-CONTEXT.md D-05]
7. **New dashboard rows silently clipped:** app slices sections into existing 35/35/30 columns and does not automatically allocate more height for appended sections; minimum interactive height is 25. Verify actual TestBackend cells at normal/narrow geometries, not model rows alone. [VERIFIED: cli/operator/dashboard/app.rs; app/tests.rs]
8. **Stale legacy copy:** existing support render/inbound.rs still describes blockfilter permissions as inactive Phase 91 evidence. Update only BASIC-related current wording, preserving unrelated relay/bloom exclusions and historical guards. [VERIFIED: cli/operator/support/render/inbound.rs; 161-CONTEXT.md D-15/D-16]
9. **Wrong wire command:** use openbitcoinnetworkstatus, not getopenbitcoinnetworkstatus. [VERIFIED: rpc/src/method.rs; cli/operator/status/http.rs]

## Code Examples

These are verified existing patterns; new names/types must be introduced by the plan rather than assumed to already exist. [VERIFIED: files cited below]

### Existing progress facts

~~~rust
// Source: packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs
let initial_sync_complete = progress.initially_synchronized();
let lag_blocks = progress.current_lag();
let maybe_processed = progress.maybe_processed_endpoint();
let maybe_safe_durable = progress.maybe_safe_durable_endpoint();
// Before using lag on reorg: compare the separately captured accepted target
// and internal branch/generation facts; never compare only exported heights.
~~~

### Existing additive availability

~~~rust
// Source: packages/open-bitcoin-node/src/status.rs
#[serde(default)]
pub chainstate_durability: FieldAvailability<ChainstateDurabilityEvidence>;
// Mirror this availability/default pattern for BASIC at BOTH schema boundaries.
~~~

### Existing duplicate filtering

~~~rust
// Source: packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
match self.maybe_bounded_basic_filter_append_row(&key, &mut work, Some(maximum_work))? {
    Some(existing) => {
        if existing != bytes {
            return Err(index_corruption("conflicting immutable BASIC record"));
        }
    }
    None if maybe_next != Some(id.height()) => {
        return Err(index_corruption("missing BASIC retry row"));
    }
    None => prepared.records.push((key, bytes)),
}
// Count a prospective delta from prepared.records; install only after success.
~~~

## Behavioral Validation Strategy

The Nyquist Validation Architecture section is intentionally omitted because workflow.nyquist_validation is explicitly false. The mandatory phase/native behavioral verification below still applies. [VERIFIED: .planning/config.json; 161-CONTEXT.md D-14/D-15]

| Concern | Existing fixture / test seam | Required new proof |
| --- | --- | --- |
| Pure status semantics | node/src/status/tests; chainstate/src/filter_index/catch_up/tests | Empty prefix vs genesis; initial latch with later lag; processed ahead of safe; each detailed failure; unavailable defaults. [VERIFIED: existing test directories; 161-CONTEXT.md D-14] |
| Actual authority collection | node/src/sync/tests/filter_index/startup/configured.rs; catch_up/fixtures.rs; reorg/fixtures.rs | Genuine validated Fjall histories invoke production operator_snapshot, not a hand-built BASIC snapshot alone. [VERIFIED: source files] |
| Recovered totals | node/src/storage/fjall_store/filters/tests; sync/tests/filter_index/reorg/retention.rs | Active/stale/ahead rows, disabled reopen, identical retry, corruption, over-limit and checked overflow; exact totals independently computed from fixture keys/values. [VERIFIED: existing tests and publisher seams] |
| Fault freshness | filters/tests/append/fencing.rs; sync/tests/filter_index/faults.rs; startup/faults.rs | Raw clone mutation, before/after commit failure, stale work, generation changes, normal coins flush and real all-handle reopen; no healthy stale totals. [VERIFIED: source files] |
| RPC/schema/auth | rpc/src/dispatch/tests/network_status_schema.rs; http/tests; bin/open_bitcoind/tests/filter_index | Exact additive keys, configured capability despite backend failure, unauthorized request does not disclose BASIC facts, baseline filter/index shapes unchanged. [VERIFIED: existing tests; 159-CONTEXT.md D-11] |
| CLI collection/legacy | cli/src/operator/status/tests/snapshot.rs; status/tests/prune_snapshot.rs; tests/status_bootstrap.rs | Same RPC facts in snapshot; old JSON missing BASIC; unavailable method/server; stopped no fabricated capabilities. [VERIFIED: source files] |
| Dashboard/render | cli/src/operator/dashboard/model/tests; dashboard/app/tests.rs; status/render/tests | Match captured facts, all key states visible; real terminal normal/narrow buffers; at most eight charts/no new controls. [VERIFIED: source files; 161-CONTEXT.md D-13] |
| Support/export | cli/src/operator/support/tests; support/redaction.rs; support/render | JSON/Markdown agree with captured facts; input clone unchanged; malicious reason strings with paths/scripts/hashes/ANSI identifiers do not escape. [VERIFIED: source files; 161-CONTEXT.md D-08] |
| Protection/body negatives | node/src/sync/tests/filter_index/reorg/retention.rs; rpc/src/bin/open_bitcoind/tests/filter_index/rpc/retention.rs | Growth survives actual paired prune/reopen; existing body-serving and wallet eligibility remain unavailable/refused; do not claim the whole Phase 162 flow. [VERIFIED: source files; 161-CONTEXT.md D-12/D-16] |
| Native claims | scripts/check-phase144-operator-flush-availability-evidence.ts; check-phase160-peer-filter-serving.ts and mutation tests | Narrow Phase 161 checker pins production edges, assertive tests, redaction, read-only/budget boundaries; mutation controls fail removed wiring/false-zero/stale-total changes. Keep historical guards intact. [VERIFIED: scripts/verify.sh; existing checker tests] |

Run targeted tests through the timing wrapper with actual test-name filters; do not claim a target completed until output proves it. Examples for planned Phase161-named tests: [VERIFIED: AGENTS.md Repo-Local Guidance; packages/*/Cargo.toml]

~~~bash
bun run scripts/command-timings.ts run --key phase161-node -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase161 --all-features
bun run scripts/command-timings.ts run --key phase161-rpc -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc phase161 --all-features
bun run scripts/command-timings.ts run --key phase161-cli -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase161 --all-features
bun test ./scripts/check-phase161-operator-index-evidence.test.ts
bash scripts/verify.sh
~~~

The proposed Phase161 checker/test file does not exist yet. Add it to both relevant verification branches in scripts/verify.sh without weakening legacy checks. Final source review/security review, simplification and formal lifecycle verification must follow a clean full native run before outer finalization. [VERIFIED: scripts/verify.sh; 161-CONTEXT.md D-15; parent task]

Operator UAT should include repo-local paired examples; use existing status JSON/human and support/dashboard commands after checking actual argument spelling. [VERIFIED: AGENTS.md Repo-Local Guidance; cli/operator/status/render.rs]

~~~bash
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json
~~~

## Security Domain

Security enforcement is not explicitly disabled in config, so include a phase threat/mitigation table and independent security verification. This section scopes controls; it is not an ASVS certification claim. ASVS 5.0 reorganized chapters, so use versioned names rather than the older template's V2-authentication/V5-input labels. [VERIFIED: .planning/config.json] [CITED: https://owasp.org/projects/asvs]

| ASVS 5.0 category | Applicability | Existing / planned control |
| --- | --- | --- |
| V2 Validation and Business Logic | Yes | Fixed enums, coherent source identities, checked counters and explicit collection limits. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x11-V2-Validation-and-Business-Logic.md] |
| V6 Authentication | Existing authenticated RPC | Reuse existing auth-first boundary; no new unauthenticated diagnostics route. [VERIFIED: rpc/src/http.rs; 159-CONTEXT.md D-11] [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x15-V6-Authentication.md] |
| V8 Authorization | Yes | Read-only existing node scope; new projection cannot release reserved prune ownership. [VERIFIED: rpc/src/method.rs; 156-CONTEXT.md D-01/D-02] [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x17-V8-Authorization.md] |
| V14 Data Protection | Yes | No new scripts/filter payloads, hashes, peer IDs, raw locks, paths or credentials; fixed BASIC labels and sanitized unavailable reasons. [VERIFIED: 161-CONTEXT.md D-08] [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x23-V14-Data-Protection.md] |
| V16 Security Logging and Error Handling | Yes | Keep raw backend errors off shared output; unavailable evidence on poison/fault; no silent success or false zeros. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x25-V16-Security-Logging-and-Error-Handling.md] |
| Session / cryptography controls | No new session or cryptographic mechanism | Preserve existing RPC credential and filter-commitment implementations; this phase adds typed observation only. [VERIFIED: 161-CONTEXT.md D-01/D-16] |

| Threat | STRIDE | Concrete mitigation / proof |
| --- | --- | --- |
| Forged healthy summary after raw mutation, failed publication or stale branch | Tampering | Shared publication invalidation, internal identity checks, unavailable observations; raw clone/AfterCommit/reorg tests. [VERIFIED: actual epoch/publication seams; proposed D-07/D-10 tests] |
| Repeated status causes history scans/filter cloning | Denial of service | Constant-size group, finite metadata reads/serialized bytes, zero request record traversal; structural work assertions. [VERIFIED: 161-CONTEXT.md D-10; proposed limits above] |
| Diagnostic exports raw backend keys/paths/scripts/IDs | Information disclosure | Typed categories and fixed guidance; sanitize arbitrary unavailable reason at support boundary; malicious-value tests. [VERIFIED: 161-CONTEXT.md D-05/D-08; support/redaction.rs] |
| Collection activates index or relaxes protection | Elevation of privilege / tampering | Read-only authority method; unchanged raw store/protection after repeated polls; no activation/recovery call edges. [VERIFIED: 161-CONTEXT.md D-04/D-07] |
| Support snapshot changes captured live facts | Repudiation / tampering | Compare original snapshot before/after exported cloned view; retain scalar facts, redact only unsafe metadata. [VERIFIED: support/redaction.rs; 161-CONTEXT.md D-08] |

## State of the Art

| Existing foundation | Phase 161 change | Boundary |
| --- | --- | --- |
| Phase 159 summary returns synced latch and processed best height | Separate richer Open Bitcoin status projection | Preserve exact getindexinfo/getblockfilter results. [VERIFIED: rpc/src/dispatch/filter_index.rs; 161-CONTEXT.md D-01] |
| Phase 160 explicit/all/implicit peer capability | Report fixed capability scope without peer identifiers | Do not infer working backend from service advertisement. [VERIFIED: network/src/filter_serving/capability.rs; 160-CONTEXT.md D-03] |
| Exclusive full integrity recovery plus bounded append | Constant-size recovered logical growth observation | Do not change durable resume authority/crash guarantees. [VERIFIED: filters/startup.rs; filters/append.rs; 161-CONTEXT.md D-16] |

BIP157 supports treating filters as separately persisted indexes and permits pruning block data after storing filters. Its generic reindex/checkpoint-cache recommendations do not override this project's locked fail-closed/no-repair/no-full-history-cache policy. Pinned local Knots remains the behavioral source. [CITED: https://bips.dev/157/] [VERIFIED: 161-CONTEXT.md D-07/D-10/D-16; 160-CONTEXT.md]

## Environment Availability

| Dependency | Available | Version / evidence | Fallback |
| --- | --- | --- | --- |
| Rust | Yes | rustc --version: 1.94.1; toolchain pin matches. [VERIFIED: environment probe] | None needed |
| Bun on default PATH | Wrong version | 1.4.2 vs .bun-version 1.3.9. [VERIFIED: environment probe; .bun-version] | Prefix PATH with /tmp/open-bitcoin-bun-1.3.9-fresh; executable reports 1.3.9 |
| Pinned Bun | Yes | /tmp/open-bitcoin-bun-1.3.9-fresh/bun --version: 1.3.9. [VERIFIED: environment probe] | Re-probe before execute; temporary paths can disappear |
| Bazel | Yes | bazel --version: 8.6.0. [VERIFIED: environment probe] | Existing repo wrapper |
| Knots baseline | Yes | submodule HEAD a9aee730466ac67d35a3c03ee24676be5e045878. [VERIFIED: git -C packages/bitcoin-knots rev-parse HEAD] | Existing submodule bootstrap |
| Pinned Fjall source | Yes | Cargo git checkout aa30dca present; batch source inspected. [VERIFIED: filesystem source read] | Cargo's existing locked dependency preparation |

No external database/service or public network is required for the proposed tests. Source research did not execute Cargo, native verification or claim successful new tests. The worktree has parent-owned planning/task changes; preserve them. [VERIFIED: 161-CONTEXT.md D-16; environment/source inspection; git status --short; parent task]

## Assumptions Log

No unverified package capabilities, compliance requirements or performance guarantees are used as locked facts. Proposed numeric observation budgets and accounting/schema names are explicit design recommendations within delegated discretion and must be proved by the planned tests; no measured latency result is asserted. [VERIFIED: 161-CONTEXT.md Agent Discretion; this research's tool record]

## Open Questions (RESOLVED)

These are resolved planning choices with implementation evidence still required; none asserts that unimplemented behavior or new performance measurements already pass. [VERIFIED: 161-CONTEXT.md D-10/D-13/D-14; 161-UI-SPEC.md approval and Render and Cross-Surface Verification Contract]

1. **Terminal fit — resolved by approved UI contract.** Use the BASIC index section first in the existing right-column group, followed by Logs and Health and then Prune. Preserve the four bands, 35/35/30 split, eight-chart maximum and 25-row safety threshold. The contract fixes all 19 full-output rows and prioritizes State, Init sync, Lag, Processed, Durable, Failure and Retained in narrow interactive output. Prove actual visible cells and adjacent semantic values through real Ratatui TestBackend draws at 160x55 and 80x30, plus 80x25, 80x24 and 40x30 safety cases. The approved contract resolves grouping; rendering tests remain mandatory execution work. [VERIFIED: 161-UI-SPEC.md Layout and Component Inventory, BASIC row order, Render and Cross-Surface Verification Contract, Checker Sign-Off; packages/open-bitcoin-cli/src/operator/dashboard/app.rs]
2. **Stopped configuration provenance — resolved as unavailable when unknown.** Reuse already-authoritative resolved inputs where present. Otherwise Indexing, Peer serving and effective Peer scope stay unavailable alongside live progress, failures and measurements. Do not add another resolver or new config syntax, and never infer disabled from absent BASIC fields in OpenBitcoinConfig JSONC. Cover both supplied-known configuration and unknown stopped configuration in collector/legacy-response tests. [VERIFIED: 161-UI-SPEC.md Copywriting Contract; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-cli/src/operator/config.rs; 161-CONTEXT.md D-04]
3. **Timing and bounded work — resolved as an explicit evidence obligation, not a latency promise.** Use the finite observation limits in this research and retain existing index-turn admission limits. Planning must include proposed tests named phase161_status_capture_has_constant_metadata_work and phase161_retained_accounting_preserves_append_budget in existing node filter-index measurement/test seams. Compare genuine small and growing Fjall histories, assert zero status-side record/history/body/undo traversal and payload copying, assert metadata reads stay within the chosen finite ceiling, and verify publication accounting adds no uncharged reads/decodes/copies. Record measured authority-hold and storage elapsed observations from these actual callers in the implementation summary; elapsed time is diagnostic evidence, not a brittle hermetic pass threshold. The names are proposed new tests, not existing verified symbols. Run them through the established timing wrapper and retain full native verification as the final gate. [VERIFIED: 161-CONTEXT.md D-10/D-14/D-15; packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs; packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs; AGENTS.md Repo-Local Guidance; proposed test design under delegated discretion]
4. **Observed corruption — resolved as bounded observation with fail-closed errors.** Describe recovered/achieved retained evidence only, without promising every arbitrary lookup succeeds. Known integrity/backend errors remain explicit unavailable/failure facts, and raw invalidation or publication poison cannot preserve a healthy stale aggregate. Prove raw clone mutation, before/after-commit faults, query errors and real reopen using the behavioral matrix above. An O(1) status projection is not a proactive whole-forest corruption scan; do not add such a scan or strengthen hardware/crash guarantees to resolve this limit. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs; filters/publication.rs; fjall_store/payload_usage.rs; 161-CONTEXT.md D-06/D-07/D-10/D-16; 161-UI-SPEC.md State coexistence and privacy]

No unresolved planning blocker remains in this section. Implementation, independent review and lifecycle verification must supply the stated evidence before completion. [VERIFIED: 161-CONTEXT.md D-14/D-15; parent task scope]

## Sources

### Primary — local pinned source and contracts

- .planning/phases/161-shared-operator-index-and-retention-evidence/161-CONTEXT.md; .planning/REQUIREMENTS.md; current STATE/PROJECT sections — locked scope and lifecycle. [VERIFIED: file reads]
- Phase 144, 150, 153, 155–160 CONTEXT.md files at the exact canonical paths in 161-CONTEXT.md — shared snapshot/privacy, logical target, recovery/fence, protection, catch-up, reorg, RPC and capability inheritance. Phase 150 path is **150-operator-prune-surfaces-and-evidence**, not an inferred variant. [VERIFIED: file reads]
- AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md; standards/index.md; architecture/code-shape/operability/testing/verification/Rust standards — material repository constraints. [VERIFIED: file reads]
- packages/bitcoin-knots/src/index/base.cpp: GetSummary and prune-lock update; src/index/blockfilterindex.cpp: retained branch records; src/rpc/node.cpp: getindexinfo; src/node/blockstorage.cpp: protection; exact local pin listed above. [VERIFIED: pinned source searches]
- First-party source files named beside findings, especially catch_up.rs, chainstate/filter_index.rs, publication/epoch.rs, append.rs, startup.rs, payload_usage.rs and production status/consumer chain. [VERIFIED: source reads/searches]
- docs/architecture/status-snapshot.md and operator-observability.md; docs/parity/catalog/basic-compact-filters.md; scripts/verify.sh and historical checkers — vocabulary, scope and verification. [VERIFIED: source section/search reads]

### Primary — official external references

- https://bips.dev/157/ — protocol background and separately persisted filters; original GitHub URL returned 503, official rendered BIP was read. [CITED: https://bips.dev/157/]
- https://owasp.org/projects/asvs and official v5.0.0 chapter URLs above — versioned category and threat-review vocabulary only. [CITED: https://owasp.org/projects/asvs]
- https://docs.rs/fjall/latest/fjall/struct.OwnedWriteBatch.html — atomic batch background; pinned source verified separately because version-specific web fetch was unavailable. [CITED: https://docs.rs/fjall/latest/fjall/struct.OwnedWriteBatch.html]

Context7 was not available in this tool inventory. No third-party search result is relied on for API/security facts. Package/latest registry lookups are unnecessary because adding/upgrading packages is excluded. [VERIFIED: tool discovery; 161-CONTEXT.md D-10/D-16]

## Metadata

| Area | Confidence | Reason |
| --- | --- | --- |
| Existing stack/seams | HIGH | Locked manifests, actual source and pinned baseline inspected. [VERIFIED: sources above] |
| Projection/failure semantics | HIGH | Current owner, pause mapping, accepted target and consumer schemas inspected. [VERIFIED: sources above] |
| Proposed accounting | MEDIUM | Reuses proven scan/dedup/publication seams but new deltas/fault budgets require implementation tests. [VERIFIED: sources above] |
| Terminal fit/performance | MEDIUM | Existing layout/budgets verified; new rendering/hold-time evidence pending. [VERIFIED: dashboard/app.rs; no implementation tests run] |

**Research date:** 2026-10-11 UTC. **Validity:** tied to the current source revision and lifecycle; recheck changed authority/publisher seams before execution. **Commit:** intentionally deferred to outer strict finalization. [VERIFIED: lifecycle context; parent task]
