---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T15:04:28Z
---

# Phase 157: Safe Activation and Scheduled Index Catch-Up - Research

**Researched:** 2026-10-05
**Domain:** Pinned BASIC activation semantics, recoverable Fjall indexing, accepted-state ownership and receive-independent bounded scheduling
**Confidence:** HIGH for source findings; MEDIUM for proposed incremental proof and resource budgets

<user-constraints>
## User Constraints (from CONTEXT.md)

The following three subsections are copied verbatim from the Phase 157 context. [VERIFIED: .planning/phases/157-safe-activation-and-scheduled-index-catch-up/157-CONTEXT.md]

### Locked Decisions

### Activation and option contracts

- **D-01:** Support bare `blockfilterindex`, `1` and `basic`; omission and `0` disable. Preserve the pinned Knots repeated/mixed option selection and precedence for the BASIC-only subset, with an explicit truth table and boundary tests across supported config/daemon paths. Unknown values and excluded V0/type 2 must refuse clearly. Do not assume generic last-value-wins semantics without inspecting the pinned parser.
- **D-02:** Explicit activation selects recovered durable storage independently of sync, inbound listening or pruning. A valid datadir is required by the existing durable path. Enabling the index must not activate networking, listeners, relay or peer filter service.
- **D-03:** Configuration must reach the production startup path before interrupted prune resume can delete inputs. Reuse trusted lifecycle enable/disable transitions; establish conservative ownership before index work or dependent deletion, and stop/invalidate work before releasing ownership on disable.

### Missing history and saved prefixes

- **D-04:** Preflight the entire still-required active suffix from the recoverable validated prefix through the authoritative chainstate tip before activation mutates index ownership or resumes destructive work. Require actual bodies and validated non-genesis undo; genesis uses its proven no-undo special case. Fresh indexing needs the genesis prefix.
- **D-05:** Missing history refuses without deleting saved immutable rows/prefix, skipping heights, reconstructing scripts from current coins, downloading, reindexing or repair. Distinguish missing required body versus undo and provide bounded diagnostics. Already valid indexed history need not retain source bodies solely to resume beyond that prefix.
- **D-06:** Preserve Phase 155 recovered coins/chain-metadata fencing and Phase 156 reserved non-overridable protection. Ahead immutable rows are not safe resume authority; every persistence failure retains conservative protection and truthful progress.

### Bounded ordered owner and ordinary callers

- **D-07:** Use one ordered append owner for retained replay and live validated connects. Capture complete body-bound historical spent-output facts, including same-block spends, at the accepted state boundary. A later coins/metadata/payload persistence error must not disappear behind a success-only callback or let unsafe durable progress escape.
- **D-08:** Actual daemon startup performs a bounded first turn; an ordinary scheduled maintenance caller advances further turns while offline/idle without peer messages. Index-enabled startup must start the worker even without public sync or inbound activation. Yield between turns and use explicit bounded block and byte/work budgets; research must measure realistic fixture turns rather than invent an unbounded replay loop.
- **D-09:** While behind, validated connects enlarge the ordered backlog rather than append out-of-order headers. Revalidate branch/lifecycle identity at publication and preserve stale-work refusal. Separate initial synchronization completion, current lag, processed rows and safe durable checkpoint internally; incomplete initial work never reports completion. Branch replacement orchestration remains Phase 158.
- **D-10:** Do not add an index-owned full-history cache or duplicate chain/undo vectors. Keep decisions pure and effects in thin existing adapters; use small modules and existing typed BASIC inputs, immutable records, lifecycle generations and publication barriers.
- **D-13:** Preserve earlier Phase 142 accepted-unflushed coins behavior; do not force a coins flush on every connect merely to eliminate crash loss. Preserve Phase 153 automatic retention under its existing Periodic/Always owner, without a new deletion worker. Follow Phase 136's receive-independent injected-timer testing and Phase 127/134's short prepare/complete epochs where practical.
- **D-14:** Bound total scheduler-turn work, including validation and projection publication, not only generated record count. Existing full-prefix record/checkpoint/projection scans require explicit research and an incremental append design or an honest measured bound; retain complete integrity validation at startup/recovery.

### Evidence and finalization

- **D-11:** Prove configuration, actual daemon startup, offline scheduled multiple-turn progress, continuous consensus-validated historical/same-block spends, ordinary connect and accepted-state/persistence failures with real Fjall reopen and concrete production consumers. Include actual paired historical payload loss, fresh and saved-prefix refusal, safe resume after indexed source loss and source preservation on refusal.
- **D-12:** Document exact option semantics and resource bounds, add required source breadcrumbs, parity evidence and deterministic regression checks, review relevant README/UAT claims, and run `bash scripts/verify.sh` plus lifecycle/source/security review before final commit/push. Use repo-local Cargo/Bazel command forms in UAT. No new production dependency or crate, public-network verification, production or funds claims.

### Agent Discretion

Exact boundary types, scheduler interval/budgets after measurement, fault seams and plan decomposition belong to research/planning. Preserve existing unaffected activation surfaces and v2.4 advisories unless a narrow change is necessary for this contract. No frontend design contract is needed: option forms are CLI/config syntax, and broad dashboard/status projections belong to Phase 161.

### Deferred Ideas (OUT OF SCOPE)

Runtime reorg (158), filter/index RPC (159), peer serving/service bits (160), dashboard and support projections (161), full post-prune client proof (162). No pending todos matched. Automatic repair/import, V0/BIP37, public defaults, production/funds claims and unrelated advisory cleanup remain excluded.
</user-constraints>

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| CFAC-01 | An operator can enable BASIC indexing through the supported Knots `blockfilterindex` forms (bare, `1`, or `basic`); omitted or `0` remains disabled, repeated/mixed forms have documented tested semantics, and explicit index activation selects durable storage without implicitly enabling networking. | Preserve two-stage scalar/list resolution, extend production durable selection and carry configuration before prune resume. |
| CFAC-02 | An operator gets a non-mutating activation refusal when a fresh or saved index needs block bodies or non-genesis undo already removed by pruning; the node preserves any valid saved prefix and does not skip heights, reconstruct history from current coins, or automatically repair/download. | Preflight the recoverable required suffix before any configured enable/disable/reconcile publication or resumed deletion. |
| CFIX-01 | An operator can observe ordered, bounded catch-up from retained history and ongoing indexing from ordinary validated connects; scheduled catch-up progresses without another peer message and incomplete work never reports a complete index. | One accepted-state append owner, incremental bounded publication and injected ordinary daemon maintenance. |

Requirement descriptions are verbatim from REQUIREMENTS.md; support cells prescribe implementation from inspected seams. [VERIFIED: .planning/REQUIREMENTS.md; node/chainstate.rs; node/storage/fjall_store/filters/publication.rs; rpc/bin/open-bitcoind.rs]
</phase-requirements>

## Project Constraints (from AGENTS.md)

- Start changes through GSD; this research belongs to the active Phase 157 planning lifecycle. Honor repo-local guidance over Bright Builds defaults; overrides currently contain only a placeholder. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md]
- Preserve Knots `29.3.knots20260210`; document scoped differences in `docs/parity/index.json` and companion docs. New Rust files require source-breadcrumb registration plus checker-generated comments. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]
- Extend existing crates; no production Rust Bitcoin dependency, new production crate or dependency. Keep pure policy separate from storage, timers, filesystem and runtime effects. [VERIFIED: AGENTS.md; standards/core/architecture.md; standards/languages/rust.md]
- Use Rust 1.94.1, Rust 2024, Bazel/Bzlmod and Bun automation; no package.json bootstrap. Wrap ad-hoc Cargo/Bazel commands in `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`, and serialize shared-target Cargo work. [VERIFIED: AGENTS.md; rust-toolchain.toml; packages/Cargo.toml; scripts/command-timings.ts]
- Final gate is `bash scripts/verify.sh`, including formatter/lint/build/tests/coverage/policy/Bazel smoke and tracked LOC freshness. Full root gate and commit/push are owned by the parent strict wrapper; this researcher makes no commit. [VERIFIED: AGENTS.md; scripts/verify.sh; orchestrator task]
- Use small named modules, `foo.rs` plus `foo/`, guard clauses, typed invariants, `maybe_` optional internal names, propagated failures, and one-concern Arrange/Act/Assert behavior tests. Bright Builds checks enforce existing exact-file exceptions; do not alter managed standards/checkers. [VERIFIED: standards/core/code-shape.md; standards/core/testing.md; standards/languages/rust.md; AGENTS.bright-builds.md]
- Keep source datadir/service/config/wallet mutation dry-run-first and explicit. Update relevant READMEs/UAT claims after material changes and show repo-local Cargo/Bazel operator commands. Do not create frontend, networking activation, repair or production/funds claims. [VERIFIED: AGENTS.md; 157-CONTEXT.md]
- Both active lesson inputs were read completely; global and repository paths contain 7,188 bytes/2,397 estimated tokens in total. Project skill directories `.claude/skills` and `.agents/skills` were absent when probed. [VERIFIED: active lessons file reads and directory probes, 2026-10-05]

## Summary

The phase is integration across existing seams, not a new filter implementation. Configuration currently does not parse `blockfilterindex`; daemon durable selection checks sync/inbound/prune only. Durable open invokes `initialize`, which recovers coins, recovers saved index protection, then resumes prune intent before constructing the managed owner. Carry explicit configured index state into that boundary; enabling after runtime construction is too late. Existing trusted enable preflights history only for fresh/Disabled states and returns immediately for saved Active state, so that shortcut must also validate the required recoverable suffix. [VERIFIED: rpc/config/loader.rs; rpc/bin/open-bitcoind.rs::open_runtime_store; node/sync/open_runtime.rs; node/chainstate/flush_lifecycle.rs::initialize; node/storage/fjall_store/filters/lifecycle.rs::enable_basic_filter_index]

A record-count limit does not establish bounded catch-up. Existing publication scans immutable records, checkpoint/projection ancestry and all durable metadata; `load_basic_filter_record` itself walks to genesis and retains ancestor hashes. Use a dedicated incremental append path whose authority is a startup-verified prefix plus a maintained checkpoint/branch/generation identity, with bounded candidate and direct-predecessor checks. Retain full forest/projection/coins integrity validation at startup/recovery. Do not weaken the general reader or claim the current publisher is bounded. [VERIFIED: node/storage/fjall_store/filters.rs; filters/ownership.rs; filters/publication.rs; chainstate/filter_index.rs]

Capture historical facts at the actual accepted-state boundary before a subsequent persistence result returns. `commit_prepared_connect` absorbs staged state before `persist()?`; the mempool transaction continues its accepted patch even when persistence fails. The index must observe this accepted transition, retain conservative protection and expose failure without publishing a cursor beyond durable coins/metadata. Use the ordinary daemon timer, independently of sync or peer messages, and retain existing Periodic/Always flush cadence rather than forcing coins on every connect. [VERIFIED: node/chainstate.rs::commit_prepared_connect; node/network/lifecycle_projection/authority.rs::commit_connected_block_lifecycle_transaction; rpc/bin/open_bitcoind/coins_flush.rs; .planning/phases/142-manager-flush-lifecycle-and-restart/142-CONTEXT.md]

**Primary recommendation:** Implement configuration/pre-prune refusal first, then a pure ordered owner plus incremental proof adapter, then accepted-state and ordinary maintenance wiring, and finally continuous validated-history/fault/budget evidence. [VERIFIED: Phase 157 locked decisions and inspected integration seams]

## Standard Stack

Use locked installed project dependencies without upgrades; no new package is recommended, so latest npm registry versions/publish dates are not relevant to this Rust integration phase. Versions below are the authoritative Cargo lock, not claims about ecosystem latest releases. [VERIFIED: packages/Cargo.lock; 157-CONTEXT.md D-12]

| Component | Pinned version | Purpose |
| --- | --- | --- |
| Rust / Cargo | 1.94.1 / edition 2024 | First-party core/node/RPC implementation. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml] |
| Fjall | 3.1.10, git `aa30dca811399a201e0b9595da93a4582dcb2b57` | Existing same-database durable batches. [VERIFIED: packages/Cargo.lock; local pinned Fjall src/batch/mod.rs] |
| Tokio | 1.52.1 | Existing daemon asynchronous shell; reuse current thread/timer ownership. [VERIFIED: packages/Cargo.lock; rpc/bin/open-bitcoind.rs] |
| serde / serde_json | 1.0.228 / 1.0.149 | Existing config/storage shapes. [VERIFIED: packages/Cargo.lock] |
| jsonc-parser | 0.32.3 | Existing JSONC boundary where applicable; do not replace baseline option resolver with it. [VERIFIED: packages/Cargo.lock; rpc/config/loader.rs] |
| Bun / Bazel | 1.3.9 / 8.6.0 declared pins | Existing automation/checks and smoke build. [VERIFIED: .bun-version; .bazelversion] |

**Installation:** none for phase features. Materialize the pinned submodule using `git submodule update --init --recursive` when required by environment, and use the existing verifier/bootstrap contract. [VERIFIED: AGENTS.md]

## Architecture Patterns

### Exact pinned option resolver

Knots first resolves scalar `GetArg`: empty or `1` enables BASIC directly; `0` disables directly; any other scalar switches to `GetArgs`, validates every merged name and selects recognized types. CLI scalar selection is the last value after the most recent boolean negation; config scalar selection is the first remaining value. The named-mode list normally includes CLI, active network section and default-section values, including lower-priority config values. Thus “CLI overrides config” is true for selecting scalar mode but does not discard config list entries in named mode. [VERIFIED: Knots init.cpp:1076; common/args.cpp::GetArg/GetArgs; common/settings.cpp::GetSetting/GetSettingsList/SettingsSpan]

Implement a narrow pure resolver retaining ordered per-source occurrences and distinguishing string `"0"` from boolean false created by `no...` negation. Do not change generic RPC-config precedence as an incidental cleanup. Register `blockfilterindex` in known CLI/config keys and retain repeated occurrences instead of collapsing into an Option<bool>. [VERIFIED: Knots common/args.cpp::InterpretValue; common/settings.cpp; rpc/config/loader.rs::CliSettings/FileSettings/collect_file_settings]

The following truth table is derived directly from pinned source, not a live Knots process run. Each sequence is in written order; “refuse” means named-mode validation sees a non-name. [VERIFIED: same pinned resolver functions]

| Source / values | Selected scalar | BASIC subset result |
| --- | --- | --- |
| Omitted | default `0` | disabled |
| Single bare / empty / `1` / `basic` | corresponding value | enabled |
| Single `0` | `0` | disabled |
| CLI `0,basic` | `basic` | refuse `0` |
| CLI `basic,0` | `0` | disabled |
| CLI `1,basic` or `empty,basic` | `basic` | refuse `1` or empty |
| CLI `basic,1` or `basic,empty` | `1` or empty | enabled |
| CLI `basic,basic` | `basic` | enabled, idempotent type selection |
| Config `0,basic` | first `0` | disabled |
| Config `basic,0` or `basic,1` | first `basic` | refuse `0` or `1` |
| Config `1,basic` or `empty,basic` | first `1` or empty | enabled |
| CLI `basic` + config `0` or unknown | CLI `basic` | refuse lower-priority config value |
| CLI `1` + config unknown | CLI `1` | enabled, list bypassed |
| CLI unknown + config `1` | CLI unknown | refuse unknown |
| CLI `-noblockfilterindex` | boolean false → scalar `0` | disabled; prior CLI list reset |
| CLI `basic,no...` | boolean false | disabled |
| CLI `no...,basic` + config `basic` | `basic` | enabled; earlier CLI discarded, config revived |
| CLI `no...,basic` + config `0` | `basic` | refuse revived `0` |
| CLI `-noblockfilterindex=0` | boolean true → scalar `1` | enabled with pinned double-negative warning semantics |
| Active network section `0` + default `basic` | section `0` | disabled |
| Active network section `basic` + default `0` | section `basic` | refuse default `0` |

Pinned CLI accepts single/double dash (`--blockfilterindex`), strips exactly one extra dash, and supports equals values; a separate next token is not the same syntax. Existing Rust parser uses `trim_start_matches('-')`, so boundary tests must intentionally state the supported parity subset. Unknown or excluded V0/type 2 as the selected single/named-list value must refuse clearly. Earlier bad strings bypassed by a final scalar `0/1/empty` follow pinned selection; do not impose eager all-values rejection that changes mixed semantics. Knots recognizes `v0`; its exclusion is a documented scope difference, not “Knots calls v0 unknown.” [VERIFIED: Knots common/args.cpp::ParseParameters/InterpretValue; init.cpp:1076; blockfilter.cpp::g_filter_types; 157-CONTEXT.md D-01]

### Pre-prune configured activation and non-mutation

Prescribed order: recover coins authority; read/validate/reconcile the saved index in memory; determine still-required suffix; validate all required bodies and body-bound undo; only then publish lifecycle/protection/reconciliation as needed; only then permit resume_prune_intent; construct owner; run one bounded first turn; start ordinary maintenance. Separate inspection from publication when current recovery would publish reconciliation before missing-history refusal. Missing history must leave saved rows/checkpoint/generation/locks/intent and source payloads unchanged, apart from independently required coins recovery. Explicitly prove which storage keys changed. [VERIFIED: initialize and filters/startup.rs/lifecycle.rs; 157-CONTEXT.md D-03–D-06]

The suffix begins at safe recoverable prefix + 1, or genesis for no prefix. Extra conservative lock coverage can begin earlier than this suffix; it must not force re-reading already indexed pruned bodies. Existing enable preflights `protection`, which may deliberately preserve stronger interrupted-disable retention; split required-input cursor from retained lock coverage. Active saved state also needs suffix preflight. Fresh genesis needs a body but no undo; every non-genesis block needs validated undo even if it contains only coinbase. [VERIFIED: filters/lifecycle.rs:111,152; FilterCheckpoint::input_protection; BasicFilterInputs::from_historical; 157-CONTEXT.md D-04–D-05]

An empty datadir cannot currently construct `VerifiedChainstateFence`: coins B and nonempty metadata are required. Specify a safe empty-awaiting-genesis state if enabled daemon startup must support an empty store; otherwise fail explicitly without inventing an indexed genesis. This is a planning boundary to settle using existing normal genesis bootstrapping, not a reason to fetch history. [VERIFIED: chainstate/filter_index.rs::VerifiedChainstateFence::new; 157-CONTEXT.md D-02,D-04]

### One owner, separate accepted and durable progress

Use a small pure index-owner state with configured lifecycle/generation, initially-synchronized flag, processed prefix, accepted target and safe durable prefix. Initial synchronization becomes true only once processing reaches the accepted target under the same identity; later connects can add lag without redefining historical initial completion. Store record existence does not prove safe resume. [VERIFIED: Knots index/base.cpp::BlockConnected/ChainStateFlushed; Phase 157 D-07–D-09; existing FilterCheckpoint/IndexWorkIdentity]

Expose complete `StagedChainstateConnect::basic_filter_inputs(block)` through the opaque prepared boundary, or consume it in a narrow prepared index fact. Record accepted target/facts in the same infallible absorb transaction before fallible persistence. When behind, enlarge target and let ordered replay consume the next height; when caught up, use the identical append reducer. A live connect persistence error must still change accepted progress/failure state but must not advance durable cursor or relax protection. Avoid an unbounded pending script/body queue; use existing accepted chainstate/body/undo owners and a bounded prepared turn. [VERIFIED: chainstate/engine/stage.rs; node/chainstate.rs::PreparedChainstateConnect/commit_prepared_connect; network/lifecycle_projection/authority.rs; Phase 157 D-07,D-10,D-13]

Inspect both staged network acceptance and ordinary direct ManagedChainstate connect paths. DurableSyncRuntime's requested-block post-processing persists body only after a successful Connected disposition; a success-only sync hook misses earlier accepted changes and later `save_block` errors. A rejected staged connect must not enqueue work. [VERIFIED: node/chainstate.rs::connect_block_with_current_time; network/lifecycle_projection/authority.rs; sync/block_response.rs::record_block_disposition]

### Incremental append proof, mandatory for bounded turns

The worker must not call the current all-purpose publisher chain on each turn. These paths scale with prior history, even when candidates are capped: [VERIFIED: files listed below]

| Hidden work | Existing implementation | Required worker replacement |
| --- | --- | --- |
| Durable fence reload/validation | `verify_basic_filter_fence` reads all chain metadata; `VerifiedChainstateFence::new` walks positions and creates an identity HashSet | Owner-maintained verified durable checkpoint identity, with explicit invalidation on successful metadata publication and failed/ambiguous persistence; compare current coins heads/B and trusted metadata identity without full vector reload. |
| Immutable forest scan | `validate_basic_filter_records` | Full once at recovery; validate candidate envelopes/commitments/local predecessor edges and conflicts only per append. |
| Checkpoint/projection scan | `maybe_basic_filter_checkpoint` validates all active heights; `scan_basic_filter_checkpoint` traverses saved prefix | Read bounded state + endpoint via existing `bounded_basic_filter_checkpoint`; require equality with startup-verified maintained append proof. |
| Projection publication | `prepare_basic_filter_projection` loops genesis through endpoint | Append only consecutive suffix projections; exact old endpoint and matching candidate parent/header are preconditions. |
| “Single record” read | `load_basic_filter_record` walks to genesis, stores ancestor hashes, decodes ancestry again | Internal bounded record decode with explicit already-verified direct predecessor identity; compare existing candidate bytes/identity locally for idempotence. Preserve full reader for recovery/general callers. |
| Token creation/completion | `maybe_basic_filter_work/check_basic_filter_work_guarded` reverify full fence | Carry same-store publication capability, lifecycle generation, old append endpoint and durable identity; revalidate these bounded identities at completion. |

Introduce a separate proof/capability whose construction requires complete recovery validation, and whose only extension is one atomic successful contiguous append. It must be invalidated when branch/lifecycle identity changes or publication poisons the store. Candidate local checks alone cannot authorize prune release. No caller-selected hash/height and no unverified raw metadata read may create this capability. Keep reconciliation/full validation as a separate recovery path. This is the prescriptive design supported by existing ownership capability and local-edge verification APIs; implementation and fault tests must prove its invariants. [VERIFIED: filters/ownership.rs::BasicFilterWorkToken/bounded_basic_filter_checkpoint; filter_index.rs::verify_filter_record_predecessor; publication.rs::finish_basic_filter_batch; Phase 157 D-06,D-09,D-14]

### Ordinary maintenance

Reuse the one-second injected-wait shell in `coins_flush.rs` or an equivalently small dedicated index maintenance module owned alongside it. Index-enabled durable selection must start maintenance with sync and inbound disabled. Drive bounded index work on startup and every elapsed idle tick; wait/yield between turns. Keep existing Periodic/Always coins and automatic-prune owner; no extra deletion worker and no forced coins flush on each connect. Tests should call the same maintenance body with injected waits and zero peer-message receives. [VERIFIED: rpc/bin/open_bitcoind/coins_flush.rs; retry.rs; .planning/phases/136-receive-independent-maintenance-and-transport-receipts/136-CONTEXT.md; Phase 142 D-01,D-02; Phase 157 D-08,D-13]

Prefer prepare/complete epochs if generation work under the authority is measurably costly: capture only bounded body/undo/input facts with identity, generate outside authority, then reacquire authority → publication and validate identity before append. If an initial implementation generates under authority, it must still meet tested block/input/operation bounds and document measured hold time. Stale lifecycle/fork completion refuses; runtime branch replacement orchestration stays Phase 158. [VERIFIED: Phase 157 D-09,D-10,D-13; filters lifecycle/ownership lock order]

## Resource Budgets and Measurement-Ready Evidence

No per-turn wall-time benchmark was run during research, and the current implementation has no production catch-up turn to benchmark. Do not invent a performance target or present fixed candidate counts as measured latency. The planner must include the following execution task before resource constants/docs become final. [VERIFIED: no scheduler implementation in inspected source; orchestrator direction to defer benchmark]

1. Extend `sync/tests/filter_index/recovery.rs::ValidatedHistory` from three genuinely accepted blocks to a parameterized continuous chain longer than several candidate turns. Preserve historical and same-block spends; the current helper uses coinbase maturity 1 and local synthetic validated headers, not a public-mainnet chain. Add representative larger transaction/script/undo inputs, including boundary-sized legal payloads. [VERIFIED: ValidatedHistory::new]
2. Measure at several prefix lengths (e.g. 16/128/512 fixture blocks), cold/warm reopen, and first/subsequent turns. Record body/undo bytes read, script elements/bytes hashed and sorted, candidate filter bytes, rows decoded/read/written, persistence batches, elapsed turn and authority hold time. Prefix length samples are test design choices, not product limits. [VERIFIED: inspected hidden scans; Phase 157 D-14]
3. Choose explicit production block, aggregate input/work and encoded-output budgets from this evidence, within current append ceilings: at most 128 candidates and aggregate encoded bytes at most `MAX_SIZE + 128 * RECORD_OVERHEAD`. Add script/item/byte accounting before expensive processing, not only post-encode size checks. Specify a singleton policy for any legal block larger than a normal aggregate budget so it neither spins forever nor silently overruns the published bound. [VERIFIED: filters/publication.rs::prepare_basic_filter_records; BasicFilterInputs; Phase 157 D-08,D-14]
4. Prove deterministic operation bounds are independent of prior prefix length: instrument I/O/decodes, and show each turn reads only bounded candidates plus constant predecessor/checkpoint facts. Include completion validation and projection publication in counters. Wall time is reported evidence; fsync latency is not guaranteed by a count cap. [VERIFIED: local pinned Fjall src/batch/mod.rs::commit; inspected worker call graph; Phase 157 D-14]
5. Feed results into `157-UAT.md`/parity docs and tests. Use injected tick/clock to prove repeated idle progress and responsiveness between turns; no sleep-dependent latency assertion. [VERIFIED: Phase 136 injected scheduling contract; Phase 157 D-11,D-12]

The full required-history activation preflight and full integrity recovery remain startup-wide work; stream inputs with bounded memory and report that separately from bounded first replay turn. Do not claim bounded startup total latency when D-04 requires checking the entire suffix. [VERIFIED: Phase 157 D-04,D-08,D-14; filters/lifecycle.rs::preflight_basic_filter_history]

## Don't Hand-Roll

| Problem | Use existing solution | Rationale/source |
| --- | --- | --- |
| BASIC bytes/hash/header | `BasicFilterInputs`, `StoredFilterRecord::generate`, typed predecessor identity | Already parity-proved Phase 154; spent history is body-bound. [VERIFIED: Phase 154 context; storage/filter_index.rs] |
| Durable transactions | Fjall OwnedWriteBatch with SyncAll | Pinned source supports atomic writes across same database keyspaces; this does not atomically include separately written coins. [VERIFIED: pinned Fjall src/batch/mod.rs; filters/publication.rs] |
| Lifecycle authority | Existing generation/token and reserved lock owner | Foreign/stale work and operator lock replacement already refuse. [VERIFIED: filters/ownership.rs; runtime_authority/filter_index.rs; Phase 156 context] |
| Historical spent scripts | Staged validation undo / retained validated undo | Current UTXOs omit spent and same-block-spent outputs. [VERIFIED: ValidatedHistory assertions; BIP158 contents; Knots undo.h] |
| New scheduler/deletion service | Existing ordinary maintenance plus serialized owner | Current worker already wakes without network receives and owns normal flush cadence. [VERIFIED: coins_flush.rs; Phase 153/136 decisions] |

## Common Pitfalls

- **Active means preflight already happened:** saved Active shortcut currently returns early. Activation must preflight missing suffix after recovery and before destructive startup resume. [VERIFIED: filters/lifecycle.rs:83; Phase 157 D-04]
- **Protection equals first needed input:** interrupted-disable stronger lock can cover already-indexed pruned history. Preserve stronger protection but preflight only actual required suffix. [VERIFIED: filters/lifecycle.rs:102–111; Phase 157 D-05]
- **Success callback equals accepted transition:** state absorbs before fallible persistence, and body save follows disposition. Hook accepted state, preserve errors, and separately track durable progress. [VERIFIED: chainstate.rs; lifecycle_projection/authority.rs; sync/block_response.rs]
- **128 means bounded:** prefix/forest/fence/ancestor scans dominate growing history. Replace all hot-path scans listed above, including token and reader internals. [VERIFIED: filters.rs; filters/ownership.rs; filters/publication.rs]
- **Flush per connect fixes fencing:** contradicts the allowed accepted-unflushed window. Records can exist ahead; safely fenced cursor/protection must wait for existing durable checkpoint success. [VERIFIED: Phase 142 D-01,D-02; Phase 155 D-04,D-05]
- **Counter or tip means synced:** processed prefix, initial completion, accepted lag and durable resume authority differ. Never initialize synced=true from saved filter height alone. [VERIFIED: Knots base.cpp; Phase 157 D-09]
- **Refusal after reconciliation is non-mutating:** a saved prefix/generation/lock change before history detection violates refusal evidence. Split inspect/preflight from effect publication and compare exact keys after real reopen. [VERIFIED: startup.rs reconciliation publication; Phase 157 D-05,D-11]
- **Fixture-only worker is production wiring:** prove daemon durable selection, early startup first turn and actual elapsed idle maintenance body with sync/inbound off. [VERIFIED: rpc/bin/open-bitcoind.rs; Phase 157 D-08,D-11]

## Code Examples

Current verified accepted-state seam (excerpt; new index fact capture belongs before the fallible persist, inside this accepted boundary): [VERIFIED: node/chainstate.rs::commit_prepared_connect]

```rust
let position = self.chainstate.absorb_staged_connect(prepared.staged);
self.persist().map_err(map_persist)?;
Ok(position)
```

Current verified receive-independent worker pattern; tests inject `Wait` into the same loop rather than waiting for peers: [VERIFIED: rpc/bin/open_bitcoind/coins_flush.rs::coins_flush_worker_loop]

```rust
match wait(Duration::from_secs(TICK_SECS)) {
    CheckpointWait::Elapsed => drive_periodic(&handle, &store),
    CheckpointWait::Shutdown => return drive_always(&handle, &store),
}
```

Current verified durable batch pattern, retained by the incremental append adapter: [VERIFIED: filters/publication.rs]

```rust
let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
// Add bounded immutable suffix, active projections, state and owned protection.
self.finish_basic_filter_batch(batch, control)
```

### Verification and fault seams for plans

Plan targeted node `filter_index` tests, config option boundary tests, daemon startup/maintenance tests, and a new phase Bun checker/mutation suite wired into the native verifier. Nyquist validation is explicitly false, so no separate Validation Architecture is included. These are execution verification recommendations, not tests run by the researcher. [VERIFIED: .planning/config.json; scripts/verify.sh; existing filter/config/daemon test modules]

- Reuse `sync/tests/filter_index/recovery.rs::ValidatedHistory` and actual Fjall reopen. Extend genuine validation rather than `support_blocks` sparse fixtures. [VERIFIED: inspected fixture]
- Reuse BeforeRecords, BeforeCheckpoint, BeforeProtection, AfterCommit, BeforeChainMeta, lifecycle before/after enable/disable/release faults. BeforeChainMeta occurs after real coins write; reopen must refuse incompatible B/metadata without erasing rows. [VERIFIED: filters/publication.rs::FilterPublicationFault; sync/tests/filter_index/faults.rs]
- Add narrow test-only payload persistence failure at the real writer and accepted-state observer assertions. Ensure rejected validation adds nothing; accepted/persist-failed state remains observed; failed index append retains protection; identical retry/reopen does not duplicate rows or advance phantom cursor. [VERIFIED: existing accepted and persistence seams; Phase 157 D-07,D-11]
- Prove actual paired prune removes a required body/undo, then fresh and saved-prefix configured activation refuses unchanged; cover missing body and missing undo separately, safe resume after indexed-source deletion, saved Active startup suffix failure, interrupted prune intent preservation, and conservative interrupted-disable lock. [VERIFIED: storage/fjall_store/prune.rs; Phase 157 D-04,D-05,D-11]
- Prove foreign/stale generation/frontier/fence/branch work refuses; idle index-only startup/maintenance yields multiple strict prefixes; larger accepted tip while behind cannot append out of order. [VERIFIED: ownership.rs; Phase 157 D-08,D-09]

Example targeted command for execution, serialized through the existing wrapper: [VERIFIED: AGENTS.md; scripts/command-timings.ts; existing test filter names]

```bash
bun run scripts/command-timings.ts run --key phase157-filter-index -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node filter_index -- --nocapture
```

Final verification remains `bash scripts/verify.sh`, then lifecycle/source/security review and strict root Git gate. [VERIFIED: Phase 157 D-12; AGENTS.md]

## State of the Art

| Prior implemented state | This phase's required behavior | Evidence |
| --- | --- | --- |
| Internal lifecycle enable/disable and pre-prune saved recovery | Explicit configured production startup with complete suffix preflight | [VERIFIED: Phase 156; Phase 157 D-01–D-06] |
| Full scans used for recovery and unused future-worker publication | Full recovery plus separate incremental bounded append capability | [VERIFIED: filters/publication.rs dead-code annotations and scans; Phase 157 D-14] |
| Existing IfNeeded/Periodic/Always coins cadence | Index observes accepted progress without changing cadence | [VERIFIED: Phase 142; coins_flush.rs; Phase 157 D-13] |

No ecosystem deprecation/upgrade is required; scope is extending pinned first-party integration. [VERIFIED: Phase 157 D-12]

## Assumptions Log

No training-only facts are used. New proof/capability design and measurement recipes are explicitly recommendations derived from code and locked requirements; budget values and performance guarantees are not asserted. [VERIFIED: cited source inventory and Phase 157 context]

| # | Claim | Section | Risk if Wrong |
| --- | --- | --- | --- |
| — | No `[ASSUMED]` factual claims | — | Execution must still measure budgets and validate proposed proof invariants. |

## Open Questions (RESOLVED)

All four questions are resolved at the planning-contract level below. Referenced plans specify required implementation and acceptance evidence; these resolutions do not claim implementation or measured results already exist. [VERIFIED: 157-01,02,04,05,07,08,10-PLAN.md]

1. **RESOLVED — Numeric budgets and authority hold time:** Plan 07 Tasks 2–3 adopt the measurement recipe above: continuous validated history, prefixes 16/128/512, first/subsequent turns, cold/warm reopen and complete-operation accounting. Task 3 runs `phase157_measure_turns -- --ignored --nocapture`, chooses actual block/input-work/output constants and legal singleton absolute limits from observed evidence, then reruns deterministic bounds tests with the production defaults. Numeric values are intentionally an execution acceptance gate: Plan 07 cannot complete without nonzero measurement evidence, constants consumed by the actual driver and passing default-bound tests. Plan 10 publishes earned values and separates chain-wide startup/preflight from bounded replay turns. No unresolved budget-design choice blocks planning, and no measurement result is fabricated here. [VERIFIED: 157-07-PLAN.md Tasks 2–3; 157-10-PLAN.md Task 2]
2. **RESOLVED — Enabled empty store:** Plan 02 Task 1 selects explicit `validated genesis history required` refusal before index mutation. It does not create an awaiting-genesis index or synthesize an anchor. Plan 10 Task 2 records this as a narrow deliberate Knots fresh-start parity difference and documents the retained, already-validated genesis/history prerequisite through the existing node's normal validated-history path, without inventing a bootstrap/import/repair command. Plan 08 proves this behavior through actual configured daemon startup. [VERIFIED: 157-02-PLAN.md Task 1; 157-08-PLAN.md Task 1; 157-10-PLAN.md Task 2]
3. **RESOLVED — Durable identity without rereading full metadata:** Plans 04–05 choose private recovery-minted `BasicFilterAppendProof`, binding the store incarnation/publication identity, lifecycle generation, exact verified append endpoint, active branch identity and maintained verified durable checkpoint/revision. Only complete integrity recovery plus suffix preflight can mint it; no caller-selected hash/height can refresh it. Plan 04 Task 2 requires every concrete production B/H and chain-meta writer, including replay/limited coins batch paths and `save_chain_meta`, to invalidate proof before ambiguous mutation and refresh only after trusted completed publication proves matching coins B, no interrupted H and compatible validated ancestry. Untrusted writers invalidate and require recovery; all store clones share invalidation. Prepare/completion compare bounded current H/B, maintained metadata identity, store/generation/frontier/branch/revision under existing authority/publication guards. Disable/re-enable, incompatible or partial writes and poisoned/ambiguous commits invalidate authority. Plan 05 extends proof only through achieved atomic contiguous suffix publication, preserving conservative safe checkpoint/protection and full recovery/general-reader validation. [VERIFIED: 157-04-PLAN.md interfaces and Tasks 1–2; 157-05-PLAN.md interfaces and Tasks 1–2]
4. **RESOLVED — Configuration surface and durable selection:** Plan 01 selects CLI and `bitcoin.conf`/includeconf/active network-section syntax only, with no new JSONC index field. `BasicFilterIndexSetting::{Unspecified, Disabled, Basic}` retains explicit presence and exact repeated scalar/list semantics. Plan 08 selects durable inspection for explicit BASIC or explicit `0` when a valid datadir exists; enabled without a valid datadir refuses. Omission with no other durable trigger stays transient, performs no index work/deletion and leaves saved protection untouched until an actual durable startup. Any actual durable daemon startup, including one selected by existing sync/inbound/prune triggers, maps omission/`0` to Disabled before prune resume. Plan 10 documents these exact boundaries. [VERIFIED: 157-01-PLAN.md interfaces and Task 2; 157-08-PLAN.md Task 1; 157-10-PLAN.md Task 2]

## Environment Availability

| Dependency | Available | Version / evidence | Fallback |
| --- | --- | --- | --- |
| Rust / Cargo | yes | rustc 1.94.1; pin matches | Existing native workflow. [VERIFIED: rustc --version; rust-toolchain.toml] |
| Bun | pinned fallback available | Default PATH 1.4.2; .bun-version 1.3.9; verified fallback 1.3.9 | Prefix execution checks with `PATH=/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64:$PATH`. [VERIFIED: bun --version; .bun-version; parent toolchain preflight] |
| Bazel launcher | present | /opt/homebrew/bin/bazel; declared Bazel 8.6.0 | Verifier smoke resolves actual tool; no build run by research. [VERIFIED: command -v bazel; .bazelversion] |
| Node | yes | v24.13.0 | Used by GSD tooling, not a new production runtime. [VERIFIED: node --version] |
| Pinned Knots | yes | inspected source baseline | Hermetic local fixture/oracle; no network process needed. [VERIFIED: packages/bitcoin-knots/src reads] |
| Fjall pinned source | yes | Cargo git checkout aa30dca | Existing lock/source; no external storage service. [VERIFIED: Cargo.lock and ~/.cargo/git/checkouts/fjall-*/aa30dca/src/batch/mod.rs] |

No external daemon, database service, credential or public network is required for this phase's hermetic evidence. No benchmark/build was launched during research to avoid overlapping root-owned Cargo work. [VERIFIED: phase scope; orchestrator instructions]

## Security Domain

Security enforcement is enabled by default because config has no false setting. Use ASVS 5.0.0 labels consistent with Phase 156; V2 here means Validation and Business Logic, not the older ASVS authentication chapter. No ASVS certification claim is made. [VERIFIED: .planning/config.json; 156-SECURITY.md; official ASVS v5.0.0 directory listing]

| ASVS 5 category | Applicability | Standard control for this phase |
| --- | --- | --- |
| V2 Validation and Business Logic | direct | Parse ordered options and storage bytes into typed state; enforce suffix, generation, continuity and work budgets. [CITED: github.com/OWASP/ASVS/tree/v5.0.0/5.0/en] |
| V6 Authentication | preserved boundary | Existing RPC authentication; no public lifecycle mutation RPC added. [VERIFIED: rpc/config.rs; Phase 157 scope] |
| V7 Session Management | no new session surface | Existing auth/worker lifecycle; test shutdown ownership. [VERIFIED: coins_flush.rs; Phase 157 scope] |
| V8 Authorization | direct | Same-store owner capability and reserved prune protection; reject foreign/stale work. [CITED: github.com/OWASP/ASVS/tree/v5.0.0/5.0/en; VERIFIED: filters/ownership.rs] |
| V11 Cryptography | existing BIP commitments | Reuse established owned SHA256d/SipHash and typed BASIC identities. [VERIFIED: Phase 154; BIP158] |
| V13 Configuration / V15 Secure Coding / V16 Error Handling | direct | Default off, no networking side effects, explicit failure and conservative progress; bounded redacted diagnostics. [VERIFIED: official ASVS v5.0.0 names; Phase 157 D-01,D-02,D-05,D-12] |

| Threat | STRIDE | Required mitigation |
| --- | --- | --- |
| Forged/stale append capability | Spoofing / Elevation | Mint proof only after full recovery; compare store/generation/frontier/branch/durable identity at publication. [VERIFIED: existing token pattern; Phase 157 D-09] |
| Lost history or early lock release | Tampering | Preflight before mutation/deletion; atomically commit proof before relaxation; preserve prefix and intent on refusal. [VERIFIED: Phase 157 D-03–D-06] |
| Full-history hot-path scans / oversized script work | Denial of service | Instrument and bound all turn work, including decode/publication/token verification; yield through ordinary timer. [VERIFIED: inspected hidden scans; Phase 157 D-14] |
| Accepted state disappears after persistence error | Repudiation / Tampering | Infallible accepted-state observation plus separate durable/failure state; real writer faults and reopen. [VERIFIED: commit_prepared_connect; Phase 157 D-07] |
| Raw scripts or paths leaked by diagnostics | Information disclosure | Fixed/bounded category-height diagnostics; no raw historical script/credential logging. [VERIFIED: Phase 157 D-05; later CFOP-01 redaction requirement] |

## Sources

### Primary (HIGH confidence)

All abbreviated first-party paths above are relative to `packages/open-bitcoin-<crate>/src/`; Knots paths are relative to `packages/bitcoin-knots/src/`. Source inspection took place on 2026-10-05. [VERIFIED: session source reads]

- `packages/bitcoin-knots/src/init.cpp`, `common/args.cpp`, `common/settings.cpp`, `blockfilter.cpp`, `index/base.cpp`, `index/blockfilterindex.cpp`, `undo.h`: pinned option resolution, history gate, accepted/committed semantics and spent facts.
- `packages/open-bitcoin-rpc/src/config.rs`, `config/loader.rs`, `bin/open-bitcoind.rs`, `bin/open_bitcoind/coins_flush.rs`: real configuration, durable startup and receive-independent maintenance.
- `packages/open-bitcoin-node/src/chainstate.rs`, `chainstate/flush_lifecycle.rs`, `network/lifecycle_projection/authority.rs`, `network/runtime_authority/filter_index.rs`, `sync/open_runtime.rs`, `sync/block_response.rs`: accepted boundary, pre-prune recovery, runtime ownership and post-connect persistence.
- `packages/open-bitcoin-node/src/storage/fjall_store/filters.rs` plus `filters/{startup,lifecycle,ownership,publication}.rs`: recovery, bounded owner loader, hidden scans and fault seams.
- `packages/open-bitcoin-chainstate/src/filter_index.rs`, `filter_index/{recovery,lifecycle}.rs`, `engine/stage.rs`: typed ancestry, fence and historical input boundary.
- `packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs`, `faults.rs`: continuous validated spent history and concrete real-coins metadata failure.
- Local pinned Fjall checkout `aa30dca/src/batch/mod.rs`: atomic same-database batches and explicit durability.
- [Official BIP158](https://bips.dev/158/): historical spent-output contents, BASIC type and parameters; assigned 2017-05-24, deployed specification, read during this research.
- [Official ASVS 5.0.0 corpus](https://github.com/OWASP/ASVS/tree/v5.0.0/5.0/en): verified category names from official GitHub directory API and V2/V8 chapters.
- AGENTS/standards, Phase 154–157 contexts, active REQUIREMENTS/ROADMAP/PROJECT/STATE, v2.5 research architecture/features/pitfalls, Phase 142/136 constraints and `scripts/verify.sh`: scope and workflow evidence.

### Secondary / Tertiary

None used for technical claims. Context7 tools were not available; pinned implementation, installed dependency source and official normative sources provide primary evidence. Some docs.rs and initially guessed ASVS URLs failed, so those URLs are not evidence. [VERIFIED: tool catalog and fetch outcomes]

## Metadata

| Area | Confidence | Reason |
| --- | --- | --- |
| Standard stack | HIGH | Exact project lock, tool pins and installed source checked. |
| Existing parser/lifecycle/accepted seams | HIGH | Concrete pinned and production source inspected. |
| Incremental bounded proof design | MEDIUM | Prescriptive extension of current capability model; requires implementation fault/scaling proof. |
| Resource numbers | MEDIUM | Existing ceilings verified; final turn budgets explicitly measurement-pending. |

**Valid until:** 2026-11-04 for internal source findings, or any earlier change to pinned code, lifecycle contracts or underlying append/flush paths. This is a review horizon recommendation, not a performance guarantee. [VERIFIED: research scope]
