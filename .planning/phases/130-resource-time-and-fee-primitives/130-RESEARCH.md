---
phase: 130-resource-time-and-fee-primitives
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-23T15:05:17Z
status: complete
---

# Phase 130: Resource, Time, and Fee Primitives - Research

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Resource accounting

- **D-01:** Represent transaction virtual size, accounted mempool memory, and configured capacity as distinct domain values; no numeric field may stand for more than one concept.
- **D-02:** Add a deterministic Rust-owned accounted-memory ledger with a documented accounting formula, cached aggregate, and recomputation oracle. It estimates owned mempool structures rather than imitating C++ allocator behavior.
- **D-03:** Keep Phase 130 limited to the accounting contract and truthful evidence. Phase 131 will use accounted memory for capacity enforcement, trimming, parity tolerances, and performance thresholds.
- **D-04:** Preserve Knots-compatible RPC meaning: `getmempoolinfo.bytes` is total transaction vsize, `usage` is accounted memory, and `maxmempool` is configured accounted-memory capacity.

#### Fee-floor vocabulary

- **D-05:** Wrap the shared fee-rate representation in semantic role types for the static relay floor, incremental relay fee, and rolling mempool floor. Derive the effective admission floor rather than storing another mutable fee state.
- **D-06:** For individual admission and operator reporting, effective admission is the maximum of the static relay floor and rolling mempool floor.
- **D-07:** Eligible package aggregates may satisfy the rolling mempool floor, but ordinary members must still satisfy the static relay floor individually. Preserve only explicitly pinned Knots exceptions, including enforced-TRUC behavior selected by later package planning.
- **D-08:** Incremental relay fee remains a replacement and pressure-bump input; it is not an independent ordinary admission threshold.

#### Entry metadata and explicit inputs

- **D-09:** Canonical mempool entries carry a typed acceptance timestamp plus typed origin and relay-intent metadata. Retry eligibility requires local origin, relay requested, and continued authoritative membership.
- **D-10:** Live admission samples acceptance time in the shell, recovery restores the persisted original acceptance time, and genuine reorg reacceptance receives the event's explicit current time. Recovery must not guess missing origin.
- **D-11:** Use operation-specific immutable contexts for admission, pressure, block, reorg, and retry decisions. Each context carries only the explicit time, block, occupancy, or jitter values relevant to that operation.
- **D-12:** Clocks and randomness stay in imperative-shell adapters. Pure mempool and network policy never reads wall-clock time or randomness directly.

#### Typed lifecycle outcomes

- **D-13:** Define one cache-agnostic semantic `MempoolLifecycleDelta` for committed consequences, separate from validation or admission attempt results.
- **D-14:** The delta records admitted members, final post-transition membership, and typed removals that distinguish cause from direct-versus-descendant role. Causes cover replacement, expiry, pressure, block confirmation or conflict, reorg consequences, and retry-state clearing where applicable.
- **D-15:** Stable enum-derived labels are the only shared metrics, log, and support-evidence projection. Transaction identities and detailed member results remain confined to authenticated direct responses.
- **D-16:** Phase 130 defines semantic facts and ordering/deduplication invariants. Phase 134 projects those facts through `ManagedNetworkHandle` into serving, fanout, retry, persistence, compact-reconstruction, and evidence state without reclassifying outcomes.

### Claude's Discretion

- Exact Rust type and module names, provided the semantic roles remain compile-time distinct.
- The documented components of the deterministic accounted-memory formula and its internal cache layout.
- Whether operation contexts are structs or newtypes, provided they remain narrow and prevent irrelevant or invalid input combinations.
- Exact lifecycle-delta collection types and ordering representation, provided final membership, deterministic ordering, and deduplication are explicit.

### Deferred Ideas (OUT OF SCOPE)

- Accounted-memory enforcement, descendant-package eviction, rolling-fee bump and decay, and parity tolerance benchmarks — Phase 131.
- Package admission, package RBF, TRUC, and ephemeral-dust execution semantics — Phase 132.
- Complete lifecycle-delta projection across every dependent cache — Phase 134.
- Durable checkpoint schema and recovery implementation — Phase 135.
- Receive-independent retry scheduling and transport-receipt clearing — Phase 136.

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| FEEP-01 | Operator evidence distinguishes transaction virtual size from accounted mempool memory usage and its configured capacity. | Use three newtypes, one cached resource ledger, an independent recomputation oracle, and corrected `getmempoolinfo` projection. [VERIFIED: `.planning/REQUIREMENTS.md:13`; `packages/bitcoin-knots/src/rpc/mempool.cpp:882-895`] |
| FEEP-02 | Node policy distinguishes the configured static relay floor, incremental relay fee, rolling mempool floor, and effective admission floor without allowing package fees to bypass the wrong boundary. | Keep `FeeRate` as arithmetic representation, wrap each role, derive effective admission, and encode static-versus-package rules before Phase 132. [VERIFIED: `.planning/REQUIREMENTS.md:14`; `packages/bitcoin-knots/src/validation.cpp:754-768,1097-1112`] |
| FEEP-03 | Mempool entries carry explicit acceptance time and typed local-origin and relay-request metadata needed by expiry, recovery, and initial broadcast retry. | Add canonical entry metadata and require operation-specific admission contexts from live, recovery, and reorg adapters. [VERIFIED: `.planning/REQUIREMENTS.md:15`; `packages/bitcoin-knots/src/kernel/mempool_entry.h:90-97,125-154`; `packages/bitcoin-knots/src/node/mempool_persist.cpp:135-173`] |
| FEEP-04 | Pure mempool and network policy accepts explicit time, block, occupancy, and jitter inputs without reading wall-clock time or randomness directly. | Define narrow immutable contexts; current pure crates contain no `SystemTime` or `UNIX_EPOCH` reads, while current node/network APIs already pass integer timestamps. [VERIFIED: code search in `packages/open-bitcoin-mempool`; `packages/open-bitcoin-node/src/network/admission_bridge.rs:45-90`; `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:89-96`] |
| FEEP-05 | Admission, replacement, expiry, pressure eviction, block connection, reorg, and retry clearing use stable typed outcomes suitable for deterministic lifecycle and operator evidence. | Separate attempt result from committed `MempoolLifecycleDelta`, split removal cause from role, and make labels enum-derived and bounded. [VERIFIED: `.planning/REQUIREMENTS.md:17`; `packages/bitcoin-knots/src/kernel/mempool_removal_reason.h:10-20`; existing split summaries in `packages/open-bitcoin-mempool/src/outcome.rs` and `pool/lifecycle.rs`] |

</phase_requirements>

## Summary

Phase 130 is a contract correction, not pressure-policy implementation. Open Bitcoin currently stores vsize as `usize`, uses the same `max_mempool_virtual_size` value for reporting and trimming, reports `getmempoolinfo.usage` as vsize, has no entry acceptance metadata, and lets the node bridge re-derive cache effects from `MempoolOutcome`. [VERIFIED: `packages/open-bitcoin-mempool/src/types.rs:78-115,136-175`; `pool.rs:32-45,454-470`; `packages/open-bitcoin-rpc/src/dispatch/node.rs:98-113`; `packages/open-bitcoin-node/src/network/admission_bridge.rs:291-312`]

The pinned Knots baseline keeps total transaction vsize, dynamic memory usage, configured memory capacity, entry time, fee roles, and removal causes as separate concepts. Its RPC maps `bytes` to total vsize, `usage` to dynamic memory, `maxmempool` to capacity, and `mempoolminfee` to the maximum of rolling and static floors. [VERIFIED: `packages/bitcoin-knots/src/txmempool.h:323-329`; `txmempool.cpp:1182-1186,1245-1267`; `kernel/mempool_entry.h:90-97`; `rpc/mempool.cpp:882-895`]

**Primary recommendation:** introduce compile-time-distinct resource and fee roles, canonical admission metadata, narrow explicit operation contexts, and one deterministic lifecycle delta in the pure crates; then correct the existing authoritative node/RPC evidence without implementing Phase 131 pressure behavior or Phase 134 complete cache projection. [RECOMMENDED]

## Project Constraints

- No `.cursor/rules/` files exist, so there are no additional Cursor-rule directives to merge. [VERIFIED: repository glob on 2026-07-23]
- Keep policy and lifecycle decisions in `open-bitcoin-mempool` or `open-bitcoin-network`; clocks, randomness, storage, RPC, transport, and serialization remain shell effects. [VERIFIED: `AGENTS.md:61-68,86-92`; `standards/core/architecture.md:5-10`; `standards/languages/rust.md:162-195`]
- Make illegal states unrepresentable with Rust newtypes/enums, use `foo.rs` plus `foo/` for new multi-file modules, avoid `unwrap()`, prefix optional internal names with `maybe_`, and unit-test pure logic with focused Arrange/Act/Assert tests. [VERIFIED: `AGENTS.bright-builds.md:45-68`; `standards/core/testing.md:5-120`; `standards/languages/rust.md:5-31,69-160`]
- Add parity breadcrumb registration for every new first-party Rust source/test file and keep intentional Knots differences in `docs/parity/`. [VERIFIED: `AGENTS.md:45-46`; `docs/parity/source-breadcrumbs.json:1-12`]
- Use Rust `1.94.1`, edition 2024, and no third-party Rust Bitcoin production dependency. [VERIFIED: `rust-toolchain.toml:1-4`; `packages/Cargo.toml:19-23`; `AGENTS.md:61-67`]
- Run ad-hoc Cargo/Bazel work through `scripts/command-timings.ts`; the final repository contract is `bash scripts/verify.sh`. [VERIFIED: `AGENTS.md:40,48-49`]
- Local standards materially informing this research are `standards/core/architecture.md`, `code-shape.md`, `testing.md`, `verification.md`, and `standards/languages/rust.md`; `standards-overrides.md` has no substantive active override. [VERIFIED: local standards files; `standards-overrides.md:5-16`]

## Standard Stack

### Core

| Technology | Version | Purpose | Why Standard |
| --- | --- | --- | --- |
| Rust | `1.94.1`, edition 2024 | Newtypes, immutable contexts, deterministic ledgers, lifecycle enums | Repository-pinned language and toolchain; no toolchain change is needed. [VERIFIED: `rust-toolchain.toml`; `packages/Cargo.toml`] |
| `open-bitcoin-mempool` | workspace `0.1.0` | Canonical entry metadata, resources, fee roles, admission and lifecycle facts | It already owns the accepted graph, fee checks, trimming, and lifecycle summaries. [VERIFIED: `packages/open-bitcoin-mempool/src/pool.rs`; `types.rs`; `pool/lifecycle.rs`] |
| `open-bitcoin-network` | workspace `0.1.0` | Retry decision context and relay-intent eligibility vocabulary | It already owns bounded transaction fanout and deferred rebroadcast decisions with injected timestamps. [VERIFIED: `packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs`] |
| `open-bitcoin-node` | workspace `0.1.0` | Shell-sampled time, admission origin, authoritative projection, narrow snapshot handoff | It already owns the admission bridge and sole runtime mutation authority. [VERIFIED: `packages/open-bitcoin-node/src/network/admission_bridge.rs`; `runtime_authority.rs`] |
| Bitcoin Knots | `v29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878` | Behavioral and naming baseline | The vendored submodule resolves exactly to the pinned tag and commit. [VERIFIED: local git commands on 2026-07-23] |

### Supporting

| Facility | Version | Purpose | When to Use |
| --- | --- | --- | --- |
| Standard-library collections | Rust `1.94.1` | Deterministic `BTreeMap`/`BTreeSet` output and canonical maps | Use ordered collections at delta/evidence boundaries; retain existing maps for canonical lookup. [VERIFIED: current use in `pool/lifecycle.rs` and `relay_fanout.rs`] |
| Serde/JSON snapshot codec | existing workspace dependency | Narrow metadata DTO compatibility | Touch only if Phase 130 must preserve acceptance metadata through the already-existing snapshot; do not build Phase 135 checkpoint scheduling. [VERIFIED: `packages/open-bitcoin-node/src/storage/snapshot_codec.rs:122-134,546-618`] |
| Existing Bun checker pattern | Bun `1.3.9` available | Deterministic phase guard and mutation fixtures | Use only for parity/evidence guardrails and verifier wiring. [VERIFIED: local `bun --version`; prior Phase 129 context/checker pattern] |
| Bazel/Bzlmod | Bazel `8.6.0` available | Top-level smoke build | Existing `glob(["src/**/*.rs"])` targets automatically include new Rust modules. [VERIFIED: local `bazel --version`; package `BUILD.bazel` files] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
| --- | --- | --- |
| Semantic fee/resource newtypes | Renamed primitive fields | Rejected: names do not prevent passing capacity where vsize is expected. [VERIFIED: locked D-01/D-05; `standards/core/architecture.md:107-142`] |
| Rust-owned logical accounting formula | Port Knots `memusage::DynamicUsage` byte-for-byte | Rejected by D-02: C++ container/allocator estimates are implementation-specific; preserve observable meanings, not allocator identity. [VERIFIED: `130-CONTEXT.md` D-02; `packages/bitcoin-knots/src/txmempool.cpp:1182-1186`] |
| One semantic lifecycle delta | Keep node-side reclassification from `MempoolOutcome` | Rejected: the current bridge independently decides replacement/eviction/cache effects and cannot represent final membership once package and pressure transitions arrive. [VERIFIED: `packages/open-bitcoin-node/src/network/admission_bridge.rs:291-379`; locked D-13/D-16] |
| Explicit input contexts | Clock/random traits in pure crates | Rejected: traits still permit hidden effect acquisition and allow irrelevant input combinations; immutable operation data is narrower. [VERIFIED: locked D-11/D-12; `standards/core/architecture.md:5-10`] |

**Installation:** no new Cargo package, system service, or database is required. [VERIFIED: existing workspace manifests and `.planning/research/STACK.md:7-18`]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-mempool/src/
├── fee.rs                         # FeeRate arithmetic plus semantic role wrappers
├── resource.rs                    # Vsize, accounted-memory, capacity, formula, ledger
├── context.rs                     # Admission/pressure/block/reorg immutable contexts
├── types.rs                       # MempoolEntry and PolicyConfig consume the new types
├── outcome.rs                     # Validation/admission attempt result only
├── pool.rs                        # State transition orchestration
└── pool/lifecycle.rs              # MempoolLifecycleDelta, removal cause/role, ordering
packages/open-bitcoin-network/src/peer/transaction_relay/
└── retry.rs                       # RetryDecisionContext and injected jitter value only
packages/open-bitcoin-node/src/network/
├── admission_bridge.rs            # Samples/provides origin, intent, acceptance time
└── mempool_lifecycle.rs           # Consumes semantic delta without reclassification
```

This layout follows the repository's required `foo.rs` plus `foo/` module convention and keeps each semantic cluster out of the already 300-line `types.rs` and 585-line `pool.rs`. [VERIFIED: `standards/languages/rust.md:5-31`; current file lengths from `ReadFile`]

### Pattern 1: Distinct Resource Values and One Ledger

Use `TransactionVirtualSize`, `AccountedMempoolMemory`, and `MempoolCapacity` as non-interchangeable newtypes. `MempoolResourceLedger` should cache total vsize and accounted memory; capacity belongs to policy configuration, not the mutable ledger. [RECOMMENDED]

The accounted-memory formula should be versioned and based on owned logical structures:

1. entry-map key plus `MempoolEntry` fixed size;
2. input/output element storage;
3. script and witness payload bytes plus witness item headers;
4. parent/child identity payloads;
5. spent-outpoint index key/value payloads.

Use `size_of` for first-party fixed structs and collection lengths for dynamic payloads; exclude allocator capacity, hash-table bucket slack, C++ pointer estimates, and dependent network caches. Use checked addition and return a typed invariant error on overflow. [RECOMMENDED]

This formula matches D-02's Rust-owned estimate and is independently recomputable from canonical entries and `spent_outpoints`. Knots itself documents its dynamic usage as an estimate with a hard-coded multi-index overhead, so byte identity is not a sound target. [VERIFIED: `130-CONTEXT.md` D-02; `packages/bitcoin-knots/src/txmempool.cpp:1182-1186`]

### Pattern 2: Transitional Capacity Without Semantic Overload

Phase 130 must not silently switch trimming to accounted memory because Phase 131 owns enforcement. Preserve the current vsize trim temporarily under an explicitly legacy/internal name, add the real `MempoolCapacity`, report the real capacity through `maxmempool`, and label accounted-capacity enforcement as deferred until Phase 131. [RECOMMENDED]

Do not keep one field named `max_mempool_virtual_size` and reinterpret it as both a vsize trim limit and memory capacity. That would directly violate D-01 and make FEEP-01 evidence false. [VERIFIED: locked D-01/D-03/D-04; current use in `pool.rs:454-470` and `pool/lifecycle.rs:92-108`]

### Pattern 3: Fee Representation Versus Fee Role

Keep `FeeRate` for arithmetic and introduce transparent semantic wrappers:

```rust
pub struct StaticRelayFeeRate(FeeRate);
pub struct IncrementalRelayFeeRate(FeeRate);
pub struct RollingMempoolFeeRate(FeeRate);
pub struct EffectiveAdmissionFeeRate(FeeRate);

pub fn effective_admission_fee_rate(
    static_floor: StaticRelayFeeRate,
    rolling_floor: RollingMempoolFeeRate,
) -> EffectiveAdmissionFeeRate;
```

The effective value is derived with `max(static, rolling)` and never stored as independent mutable state. Incremental relay fee remains an input to replacement and later pressure bumps. [VERIFIED: locked D-05 through D-08; Knots projection in `rpc/mempool.cpp:893-895`]

Current ordinary admission checks only `min_relay_feerate`, while replacement separately consumes `incremental_relay_feerate`; those call sites are the direct migration seams. [VERIFIED: `packages/open-bitcoin-mempool/src/pool.rs:110,248-258,353-368`]

### Pattern 4: Metadata Is Canonical, Adapter Facts Are Explicit

Use a canonical metadata value on every accepted entry:

```rust
pub struct MempoolEntryMetadata {
    pub accepted_at: MempoolAcceptanceTime,
    pub origin: MempoolOrigin,
    pub relay_intent: RelayIntent,
}

pub enum MempoolOrigin {
    Local,
    Peer,
    Reorg,
    RecoveryUnknown,
}
```

`RecoveryUnknown` is a fail-closed compatibility state, not a guess: it must never qualify for retry. New-format recovery should restore the original typed timestamp, origin, and relay intent; genuine reorg reacceptance should use the explicit reorg event time and `Reorg` origin. [RECOMMENDED; VERIFIED constraint: locked D-09/D-10]

The current live peer path already receives `timestamp`, and the current local path has `submit_local_transaction_outcome_at`; however the mempool API discards both and the convenience local method injects `0`. Remove the zero-time convenience from production use. [VERIFIED: `packages/open-bitcoin-node/src/network/admission_bridge.rs:45-59,189-210`]

Keep Phase 130's durable work narrow: add metadata fields and compatibility classification only as needed to prevent current recovery from fabricating time/origin. Do not add checkpoint cadence, topological replay, unbroadcast persistence, or recovery scheduling; those remain Phase 135. [RECOMMENDED; VERIFIED boundary: `130-CONTEXT.md` deferred ideas]

### Pattern 5: Operation-Specific Contexts

Define separate immutable values rather than one broad `PolicyContext`:

```rust
pub struct AdmissionContext {
    pub metadata: MempoolEntryMetadata,
}

pub struct PressureDecisionContext {
    pub observed_at: PolicyTime,
    pub usage: AccountedMempoolMemory,
    pub capacity: MempoolCapacity,
}

pub struct BlockLifecycleContext {
    pub connected_at: PolicyTime,
    pub height: u32,
}

pub struct ReorgLifecycleContext {
    pub occurred_at: PolicyTime,
}

pub struct RetryDecisionContext {
    pub observed_at: PolicyTime,
    pub jitter: RetryJitter,
}
```

These contexts establish compile-time boundaries now; Phase 131, 134, and 136 add behavior behind them. Pure crates must not add `SystemTime`, `getrandom`, Tokio timers, or hidden defaults. [VERIFIED: locked D-11/D-12; no current `SystemTime`/`UNIX_EPOCH` matches in pure mempool; current shell timestamp flow in `admission_bridge.rs`]

### Pattern 6: Attempt Result Plus Committed Delta

Keep `MempoolOutcome` as the validation/admission attempt vocabulary. Return committed state separately:

```rust
pub struct MempoolTransition {
    pub outcome: MempoolOutcome,
    pub delta: MempoolLifecycleDelta,
}

pub struct MempoolLifecycleRemoval {
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub cause: MempoolRemovalCause,
    pub role: MempoolRemovalRole,
}
```

`MempoolRemovalCause` should cover replacement, expiry, pressure, block confirmation, block conflict, and reorg. `MempoolRemovalRole::{Direct, Descendant}` replaces the current mistake where `Descendant` is treated as a competing reason. Retry clearing should be a separate typed consequence in the same delta, because clearing relay intent is not itself removal from the mempool. [RECOMMENDED; VERIFIED current problem: `packages/open-bitcoin-mempool/src/pool/lifecycle.rs:16-32`]

Delta invariants should be:

- admitted members preserve transition/topological order;
- removals are deduplicated by identity and ordered deterministically;
- direct role wins over descendant role for the same cause;
- final membership is recorded for every affected identity after all mutation/trimming;
- retry clears contain exactly one fact per identity per delta, with deterministic cause precedence `LifecycleRemoval > TransportWritten > EligibleServe`;
- exact duplicate retry-clear facts collapse, while inconsistent txid/wtxid identity pairs are rejected;
- shared labels derive only from enums;
- detailed identities stay internal or in authenticated direct responses.

[VERIFIED constraint: locked D-13 through D-16]

### Dependency Order

1. Resource and fee newtypes plus ledger/oracle.
2. Entry metadata and explicit contexts, including narrow live/recovery/reorg adapters.
3. `MempoolLifecycleDelta` production and migration of current node consumers.
4. Corrected RPC/operator evidence, parity docs, breadcrumbs, and deterministic guard.

[RECOMMENDED]

## Existing Implementation Seams

| Seam | Current State | Phase 130 Action |
| --- | --- | --- |
| `open-bitcoin-mempool/src/types.rs` | `FeeRate`, raw `usize` vsize/capacity, entry without time/origin/intent. [VERIFIED: lines 27-175] | Move semantic clusters to focused modules; make entry/config fields typed. |
| `open-bitcoin-mempool/src/pool.rs` | Cached total vsize, full recomputation, vsize trimming, ordinary static fee check. [VERIFIED: lines 32-45,79-147,454-580] | Add resource ledger/oracle and transition return; retain explicitly legacy vsize enforcement until 131. |
| `pool/lifecycle.rs` | Pressure summary overloads vsize as capacity and removal reason conflates descendant role. [VERIFIED: lines 16-108] | Replace summary fields with typed resources/fees and produce semantic delta. |
| `outcome.rs` | Stable attempt labels but no committed final-membership fact. [VERIFIED: lines 12-198] | Keep as attempt vocabulary; do not overload it with lifecycle projection. |
| `node/network/admission_bridge.rs` | Has peer/local timestamps and origin context but discards them before entry creation; re-derives effects from outcome. [VERIFIED: lines 45-75,189-223,291-379] | Build typed admission context and consume returned delta. |
| `node/network/mempool_lifecycle.rs` | Block/reorg paths manually translate summary/outcome into cache cleanup. [VERIFIED: lines 20-101] | Provide explicit block/reorg contexts and consume semantic cause/role facts without renaming them. |
| `node/storage/mempool_snapshot.rs` | Snapshot omits acceptance time/origin/intent and replays through ordinary admission. [VERIFIED: lines 13-100] | Add only the narrow metadata/compatibility seam required for truthful recovery; leave complete Phase 135 recovery design deferred. |
| `rpc/dispatch/node.rs` | `usage == bytes`, `maxmempool` is vsize limit, and `mempoolminfee == minrelaytxfee`. [VERIFIED: lines 98-113] | Project bytes/usage/capacity separately and add static/incremental/rolling/effective evidence. |

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
| --- | --- | --- | --- |
| Allocator parity | A C++ Boost/container memory emulator | Versioned Rust logical accounting formula | Knots' own formula is an implementation estimate; D-02 explicitly selects Rust-owned accounting. [VERIFIED: `txmempool.cpp:1182-1186`; locked D-02] |
| Time abstraction | Global clock singleton or trait object inside pure policy | Typed timestamp/context sampled by shell | Keeps tests deterministic and prevents hidden wall-clock access. [VERIFIED: locked D-11/D-12] |
| Randomness abstraction | RNG state in mempool/network policy | Shell-sampled `RetryJitter` value | Pinned retry samples every cycle, but Phase 136 owns scheduling. [VERIFIED: `net_processing.cpp:1562-1579`; phase boundary] |
| Fee state | Another mutable effective floor | Pure `max(static, rolling)` derivation | Effective admission is derived by decision. [VERIFIED: locked D-05/D-06] |
| Lifecycle evidence translation | String parsing or per-cache reason maps | Enum-derived labels from one delta | Prevents reclassification drift and dynamic labels. [VERIFIED: locked D-13 through D-16] |
| New dependency | Graph, cache, time, or memory-size crate | Existing first-party types and standard library | Current structures provide every required capability. [VERIFIED: manifests and existing source] |

## Common Pitfalls

### Pitfall 1: Changing Enforcement in the Contract Phase

**What goes wrong:** accounted usage starts driving eviction before Phase 131's descendant eviction, rolling-floor bump, tolerances, and performance tests exist. [VERIFIED boundary: locked D-03 and deferred Phase 131 scope]

**How to avoid:** add truthful accounted usage and capacity now, retain a plainly named legacy vsize enforcement seam, and make the operator evidence say accounted enforcement is not active until 131. [RECOMMENDED]

### Pitfall 2: Preserving a Misleading RPC Shape

**What goes wrong:** `bytes`, `usage`, and `maxmempool` remain numerically equal even though they claim different meanings. This is the current implementation. [VERIFIED: `packages/open-bitcoin-rpc/src/dispatch/node.rs:104-112`]

**How to avoid:** map them from typed ledger/capacity fields and test unequal values with a transaction whose accounted usage exceeds its vsize. Add `incrementalrelayfee`; expose raw rolling and derived effective values through a clearly documented Open Bitcoin extension while keeping Knots field meanings intact. [RECOMMENDED; VERIFIED baseline: `rpc/mempool.cpp:889-895`]

### Pitfall 3: Treating Incremental Relay Fee as Admission Floor

**What goes wrong:** ordinary transactions are rejected against `max(static, incremental, rolling)` even though incremental fee is for replacement and pressure bumping. [VERIFIED: locked D-08; current replacement use in `pool.rs:248-258`]

**How to avoid:** make wrapper types incompatible and omit incremental fee from the effective-admission constructor. [RECOMMENDED]

### Pitfall 4: Guessing Recovery Origin or Time

**What goes wrong:** restart makes every recovered transaction look newly accepted or locally originated, corrupting expiry and privacy-sensitive retry eligibility. The current snapshot lacks both fields. [VERIFIED: `storage/mempool_snapshot.rs:13-20,56-100`]

**How to avoid:** preserve known metadata, classify legacy absence explicitly, and make unknown origin ineligible for retry. Never substitute restart time or `Local`. [RECOMMENDED; VERIFIED constraint: locked D-10]

### Pitfall 5: Calling Descendant a Removal Cause

**What goes wrong:** a descendant of a replacement, pressure victim, expiry root, or conflict loses the actual cause, so later cache/evidence projections disagree. [VERIFIED: current enum in `pool/lifecycle.rs:16-32`; Knots causes in `kernel/mempool_removal_reason.h:13-20`]

**How to avoid:** store `cause` and `role` independently, deduplicate identities, and test direct/descendant precedence. [RECOMMENDED]

### Pitfall 6: Full Phase 134 Projection Creep

**What goes wrong:** Phase 130 attempts to synchronize every serving, fanout, peer, compact, retry, persistence, and support cache before package/pressure semantics exist. [VERIFIED boundary: locked D-16 and deferred Phase 134 scope]

**How to avoid:** produce the semantic delta and migrate current consumers enough to prove the seam; explicitly defer complete cross-cache coverage to 134. [RECOMMENDED]

### Pitfall 7: Dynamic or Sensitive Evidence

**What goes wrong:** txids, wtxids, peer IDs, free-form reasons, or fee values become metric/log labels. Existing relay evidence uses fixed labels and aggregate counts. [VERIFIED: `packages/open-bitcoin-node/src/network/relay_fanout.rs:44-80,479-533`; locked D-15]

**How to avoid:** expose identities only in authenticated direct responses; shared status uses fixed enum labels and counts. [RECOMMENDED]

## Code Examples

### Accounted-Memory Recompute Oracle

```rust
pub fn recompute_accounted_memory(
    entries: &HashMap<Txid, MempoolEntry>,
    spent_outpoints: &HashMap<OutPoint, Txid>,
) -> Result<AccountedMempoolMemory, ResourceAccountingError> {
    // Sum documented first-party logical structure components with checked math.
    // This is intentionally independent of the cached ledger update path.
}
```

This mirrors Knots' cached-versus-recomputed invariant pattern without copying its allocator formula. Knots checks cached inner usage against a fresh sum in its consistency check. [VERIFIED: `packages/bitcoin-knots/src/txmempool.cpp:807-895`]

### Final Membership in a Transition

```rust
pub struct MempoolMemberState {
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub membership: FinalMempoolMembership,
}

pub enum FinalMempoolMembership {
    Present,
    Absent,
}
```

Use an ordered, deduplicated collection of affected members rather than cloning the entire pool into every delta. [RECOMMENDED]

### Shell-Sampled Admission

```rust
let context = AdmissionContext::local(
    MempoolAcceptanceTime::from_unix_seconds(now_unix_seconds)?,
    RelayIntent::Requested,
);
let transition = managed_mempool.submit_transaction(transaction, context, policy_facts)?;
```

The current bridge already receives the timestamp; this change carries it into the pure transition rather than discarding it. [VERIFIED: `packages/open-bitcoin-node/src/network/admission_bridge.rs:45-59,198-210`]

## State of the Art

| Old/Current Approach | Phase 130 Approach | Impact |
| --- | --- | --- |
| One raw `usize` represents vsize and reported capacity. [VERIFIED: `types.rs:78-115`; `pool/lifecycle.rs:68-108`] | Three resource newtypes plus ledger | Compile-time semantic separation and truthful RPC evidence. |
| One `FeeRate` type is used directly for static and incremental roles; rolling is only `Deferred`. [VERIFIED: `types.rs:27-63,79-115`; `pool/lifecycle.rs:54-76`] | Shared arithmetic type wrapped by role types; effective derived | Later package/pressure code cannot bypass the wrong boundary accidentally. |
| Admission timestamps exist at node/network call sites but not on entries. [VERIFIED: `admission_bridge.rs:45-59`; `types.rs:136-175`] | Canonical entry metadata from explicit context | Expiry, recovery, and retry can use one source fact. |
| Attempt outcomes and lifecycle summaries are separate but incomplete and node adapters reclassify them. [VERIFIED: `outcome.rs`; `pool/lifecycle.rs`; `admission_bridge.rs`] | Attempt outcome plus committed semantic delta | Phase 134 receives a stable projection contract. |
| `getmempoolinfo.usage` equals vsize. [VERIFIED: `rpc/dispatch/node.rs:104-112`] | `bytes=vsize`, `usage=accounted`, `maxmempool=capacity` | Matches pinned field meaning. [VERIFIED: Knots `rpc/mempool.cpp:889-895`] |

## Final Plan Ownership

The corrective plan set uses the following exact owners and commit-safe dependency boundaries:

1. **Plan 130-01** owns distinct resource values, versioned accounted-memory formula, cached ledger, independent oracle, and the explicitly legacy vsize enforcement seam.
2. **Plan 130-02** owns semantic fee roles, effective-floor derivation, and ordinary/member-versus-package aggregate boundary contracts.
3. **Plan 130-03** owns canonical entry metadata plus admission, pressure, block, and reorg input contexts.
4. **Plan 130-04** owns `MempoolLifecycleDelta`, cause/role/final-membership facts, and the exactly-one retry-clear invariant with `LifecycleRemoval > TransportWritten > EligibleServe` precedence.
5. **Plan 130-05** owns managed peer/local admission contexts and admission-side delta consumption while introducing fail-closed compatibility APIs.
6. **Plan 130-06** migrates node-owned callers but deliberately retains the no-time compatibility method because `open-bitcoin-rpc` still calls it; a workspace check proves this intermediate commit compiles.
7. **Plan 130-07** owns block/reorg contexts and lifecycle consumers, including its serialized changes to `runtime_authority.rs`.
8. **Plan 130-08** owns current snapshot metadata compatibility, fail-closed legacy decode, and recovery replay.
9. **Plan 130-10** owns the independent pure-network retry time/jitter vocabulary and may execute after Plans 130-03 and 130-04.
10. **Plan 130-11** executes after Plan 130-07, owns `runtime_authority.rs`, migrates the remaining RPC compatibility caller, and only then removes the no-time methods in the same workspace-compilable commit.
11. **Plan 130-09** owns authoritative operator/RPC resource and fee evidence after Plans 130-08 and 130-11.
12. **Plan 130-12** owns parity catalog/index/breadcrumb reconciliation and concrete updates to all three README surfaces.
13. **Plan 130-13** owns the `string[]` Phase 129-style checker contract, independent three-README stale-wording mutations, default verifier wiring, and full repository verification.

This ownership keeps claims explicit that accounted enforcement/rolling behavior are Phase 131 and complete cross-cache projection is Phase 134. [VERIFIED boundary: locked D-03/D-16]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
| --- | --- | --- | --- | --- |
| Rust compiler | Pure domain implementation/tests | ✓ | `1.94.1` | — [VERIFIED: local command] |
| Cargo | Crate tests and workspace checks | ✓ | `1.94.1` | — [VERIFIED: local command] |
| Bun | Command timing and optional checker tests | ✓ | `1.3.9` | — [VERIFIED: local command] |
| Bazel/Bazelisk path | Workspace smoke build | ✓ | Bazel `8.6.0` | — [VERIFIED: local command] |
| Pinned Knots submodule | Parity anchors/fixtures | ✓ | `v29.3.knots20260210` at `a9aee730...` | — [VERIFIED: local git command] |

**Missing dependencies with no fallback:** none. [VERIFIED: environment audit on 2026-07-23]

**Missing dependencies with fallback:** none. [VERIFIED: environment audit on 2026-07-23]

## Validation Architecture

Nyquist validation is currently disabled, but this section is included by explicit Phase 130 instruction. [VERIFIED: `.planning/config.json:15-20`; user request]

### Test Framework

| Property | Value |
| --- | --- |
| Framework | Built-in Rust unit/integration tests under Cargo `1.94.1`; Bun tests for optional structural checker. [VERIFIED: workspace source/tests and local tools] |
| Config file | `packages/Cargo.toml`; package targets use existing manifests. [VERIFIED: workspace manifest] |
| Quick run command | `bun run scripts/command-timings.ts run --key phase130-mempool-test -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool --lib` |
| Full suite command | `bash scripts/verify.sh` [VERIFIED: `AGENTS.md:40`] |

### Phase Requirements to Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
| --- | --- | --- | --- | --- |
| FEEP-01 | Vsize, accounted usage, and capacity differ and oracle equals cache after add/replace/remove. | unit + RPC integration | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool resource` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc get_mempool` | ❌ Wave 0 focused resource tests; RPC test module exists |
| FEEP-02 | Static/incremental/rolling/effective roles cannot be interchanged; effective is max(static, rolling); incremental is excluded. | unit + compile-time API review | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool fee` | ❌ Wave 0 focused fee tests |
| FEEP-03 | Live/local/peer/reorg/recovery metadata preserves time/origin/intent; unknown recovery never becomes local retry. | unit + node integration | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node admission_bridge` | Existing admission tests need extension |
| FEEP-04 | Pure contexts take explicit values; no clock/random reads exist in pure crates. | unit + deterministic source guard | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool context` plus optional Phase 130 checker | ❌ Wave 0 context tests/guard |
| FEEP-05 | Delta ordering/dedup/final membership and cause-versus-role remain stable across admission, replacement, block, reorg, expiry/pressure fixtures, and retry-clear facts. | unit + managed integration | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool lifecycle` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node mempool_lifecycle` | Existing lifecycle tests need extension |

### Required Boundary Cases

- Accounted formula: empty pool, one witness transaction, multiple scripts/witness items, parent/child edge, replacement, block removal, checked-overflow fixture, and cached-versus-recomputed equality. [RECOMMENDED]
- Fee roles: rolling below/equal/above static, zero rolling baseline, incremental greater than both but excluded from admission, and package eligibility API that cannot waive static member checks. [RECOMMENDED; VERIFIED baseline rule: `validation.cpp:1097-1112`]
- Metadata: local relay requested/not requested, peer origin, genuine reorg timestamp, recovered known metadata, legacy missing origin, and duplicate attempt that must not rewrite canonical acceptance metadata. [RECOMMENDED]
- Lifecycle: replacement direct victim plus descendant, expiry root plus descendant, pressure root plus descendant, block confirmed versus block conflict, reorg removal, retry clear, duplicate affected identities, and final present/absent state. [RECOMMENDED]
- RPC: construct unequal vsize/accounted/capacity values and assert exact field mapping; assert static, incremental, rolling, and effective fee evidence separately. [RECOMMENDED]

### Sampling Rate

- **Per task commit:** affected package command through `scripts/command-timings.ts`. [VERIFIED: `AGENTS.md:48`]
- **Per wave merge:** mempool, node, and RPC package tests serially; do not overlap Cargo jobs against the same target directory. [VERIFIED: `AGENTS.md:48-49`]
- **Phase gate:** `bash scripts/verify.sh` must pass, including parity breadcrumb and Bazel smoke checks. [VERIFIED: `AGENTS.md:40,45-46`]

### Wave 0 Gaps

- [ ] Focused resource-accounting tests and independently structured recomputation oracle.
- [ ] Focused fee-role/effective-floor tests.
- [ ] Context construction and no-hidden-clock/randomness guard coverage.
- [ ] Lifecycle delta ordering, deduplication, cause/role, and final-membership tests.
- [ ] RPC fixture where `bytes != usage != maxmempool`.
- [ ] Breadcrumb entries for every new Rust file.

## Verification Commands

```bash
bun run scripts/command-timings.ts run --key phase130-mempool-test -- \
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool --lib

bun run scripts/command-timings.ts run --key phase130-node-test -- \
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node

bun run scripts/command-timings.ts run --key phase130-rpc-test -- \
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc

bun run scripts/command-timings.ts run --key phase130-bazel-mempool -- \
  bazel build //packages/open-bitcoin-mempool:open_bitcoin_mempool_lib

bash scripts/verify.sh
```

These commands follow the repository's cooperative timing/lock and final verification contracts. [VERIFIED: `AGENTS.md:40,48-49`; package `BUILD.bazel` targets]

## Security Domain

### Phase-Level ASVS L1 Control Matrix

This matrix is the blocking ASVS L1 applicability contract for every Phase 130 plan. It is intentionally scoped to a headless Rust domain/node/RPC phase: browser, session, transport, upload, and cryptographic controls are not treated as applicable when the phase introduces no such surface. Each plan threat model must cite the row IDs it owns, execute the listed verification, and treat a failed applicable row as a commit blocker.

| Matrix row | ASVS 4.0.3 L1 family | Applicability | Concrete Phase 130 mitigation | Owner plans/tasks | Blocking verification |
| --- | --- | --- | --- | --- | --- |
| ASVS-130-V1 | V1 Architecture, Design and Threat Modeling | Applicable | Preserve the typed functional-core/imperative-shell boundary, one managed mutation authority, explicit trust boundaries, and workspace-compilable public API transitions. | Plans 01–11, every Rust task; Plan 13 verifier task | Each Rust task must inventory workspace callers and the exact timed `cargo check --workspace --all-targets` gate below must exit 0; Plan 13 runs `bash scripts/verify.sh`. |
| ASVS-130-V2 | V2 Authentication | Not applicable | Phase 130 adds no authentication mechanism or credential flow; the existing authenticated direct-RPC boundary is preserved, not redesigned. | Plans 05, 09, 11 document preservation only | Source/integration assertions keep detailed identities in the existing direct response and out of shared evidence. |
| ASVS-130-V3 | V3 Session Management | Not applicable | The headless node phase creates no browser or server session lifecycle. | None | Scope audit confirms no session/token/cookie changes in `files_modified`. |
| ASVS-130-V4 | V4 Access Control | Applicable to the existing RPC evidence boundary | Only the existing authenticated direct response may carry transaction/member detail; shared status remains aggregate and identity-free. | Plan 05 Task 1; Plan 09 Task 1; Plan 11 Task 1; Plan 12 Tasks 1–2; Plan 13 Task 1 | Node/RPC serialization tests, forbidden-identity scans, parity/no-claim checks, and checker mutations must pass. |
| ASVS-130-V5 | V5 Validation, Sanitization and Encoding | Applicable | Parse resource, fee, time, origin, relay intent, lifecycle, jitter, and snapshot values into closed typed contracts; use checked arithmetic and all-or-none durable decoding. | Plans 01–04; Plan 08 Tasks 1–2; Plan 10 Task 1 | Focused boundary/overflow/identity-conflict/jitter/codec tests plus each owning Rust task's all-target workspace check must pass. |
| ASVS-130-V6 | V6 Stored Cryptography | Not applicable | No key, secret, hashing, signature, or cryptographic storage behavior changes; no cryptographic dependency is added. | None | Manifest and changed-path audit confirms no crypto surface or dependency change. |
| ASVS-130-V7 | V7 Error Handling and Logging | Applicable | Overflow, identity conflict, clock conversion, malformed snapshot, and jitter failures remain typed and fail closed; shared labels derive only from fixed enums. | Plan 01 Tasks 1–2; Plan 04 Tasks 1–2; Plan 08 Task 2; Plan 09 Task 1; Plan 10 Task 1; Plan 11 Task 1; Plan 13 Task 1 | Focused typed-error tests, fixed-label/source scans, mutation tests, and full verifier must pass. |
| ASVS-130-V8 | V8 Data Protection | Applicable | Treat local origin, relay intent, acceptance metadata, and transaction identities as sensitive; preserve exact source facts while excluding identities from aggregate evidence and docs. | Plans 03, 05–09, 11–13 | Metadata/recovery tests, shared-serialization negative assertions, parity checks, README/no-claim checks, and checker identity-leak mutation must pass. |
| ASVS-130-V9 | V9 Communications | Not applicable | No transport protocol, TLS, socket authentication, or public relay behavior is introduced; retry scheduling/receipts remain Phase 136. | Plan 10 and Plan 12 document deferral only | Source/parity checks reject transport, scheduler, public/default relay, and guaranteed-propagation claims. |
| ASVS-130-V10 | V10 Malicious Code | Not applicable | No new third-party dependency, dynamic code loading, plugin, or executable content path is introduced. | Plans 01–11 preserve the existing dependency set | Cargo manifest/diff audit and repository verifier confirm no dependency or dynamic-execution addition. |
| ASVS-130-V11 | V11 Business Logic | Applicable | Enforce non-interchangeable resource/fee roles, immutable metadata, explicit operation inputs, deterministic lifecycle ordering/deduplication, and truthful compatibility boundaries. | Plans 01–11, all tasks | Focused business-logic tests and the all-target workspace check at every Rust commit boundary must pass. |
| ASVS-130-V12 | V12 File and Resources | Not applicable | The phase adds no upload, archive extraction, user-selected path, or downloadable file surface. Existing trusted local snapshot bytes are validated under V5 rather than treated as an upload feature. | Plan 08 documents this boundary | Snapshot corruption tests pass; changed-path audit confirms no new path/upload/archive surface. |
| ASVS-130-V13 | V13 API and Web Service | Applicable to crate and JSON-RPC contracts | Migrate every public Rust caller or retain a fail-closed compatibility shim with a named removal owner; preserve stable RPC field meanings and typed shell failure behavior. | Plans 01–11; especially Plans 05–07, 09, 11 | Public-symbol caller inventory plus exact all-target workspace check in every Rust task; RPC unequal-value/time/error tests pass. |
| ASVS-130-V14 | V14 Configuration | Applicable | Capacity and fee configuration use distinct typed fields/defaults; operator evidence reports configured values without changing Phase 130 enforcement claims. | Plan 01 Task 2; Plan 02 Tasks 1–2; Plan 09 Task 1; Plan 12 Tasks 1–2; Plan 13 Task 1 | Config/default tests, exact RPC equations, parity docs, README checks, and checker mutations must pass. |

#### Rust Public-API Commit Boundary

Before every Rust task commit in Plans 130-01 through 130-11, the executor must inventory all workspace callers of every changed public type, field, constructor, method, enum, and return shape across library, binary, benchmark, integration-test, and doctest targets. Each caller must be migrated in that task or remain behind an explicit fail-closed compatibility shim whose rustdoc names the later removal owner plan. The task cannot commit unless this exact repository-local command exits 0 after the caller migration/shim work:

`bun run scripts/command-timings.ts run --key cargo-check-workspace-all-targets -- cargo check --manifest-path packages/Cargo.toml --workspace --all-targets`

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
| --- | --- | --- |
| V2 Authentication | No new authentication surface | Preserve existing authenticated direct-response boundary for identities. [VERIFIED: locked D-15] |
| V3 Session Management | No | No session behavior changes in this phase. [VERIFIED: phase boundary] |
| V4 Access Control | Indirectly | Detailed member identities remain in authenticated direct responses; shared evidence is aggregate. [VERIFIED: locked D-15] |
| V5 Input Validation | Yes | Parse numeric boundary inputs into resource, fee, time, origin, intent, and context types before pure policy. [VERIFIED: `standards/core/architecture.md:61-105`] |
| V6 Cryptography | No new cryptography | Do not add cryptographic dependencies or change transaction identity hashing. [VERIFIED: phase scope and manifests] |

### Known Threat Patterns

| Pattern | STRIDE | Standard Mitigation |
| --- | --- | --- |
| Under-accounting or arithmetic overflow permits resource exhaustion | Denial of Service / Tampering | Checked accounting math, cached-versus-oracle invariant, and no enforcement claim until Phase 131. [RECOMMENDED] |
| Peer/recovery input is mislabeled local and enters retry eligibility | Spoofing / Information Disclosure | Origin enum created only by trusted shell adapters; unknown recovery fails closed; retry requires local + requested + present. [VERIFIED constraint: locked D-09/D-10] |
| Wall-clock rollback or hidden RNG makes decisions irreproducible | Tampering / Repudiation | Immutable explicit time/jitter contexts; no clock/random access in pure crates. [VERIFIED constraint: locked D-11/D-12] |
| Removal cause is lost when descendants are removed | Tampering | Independent cause/role fields and deterministic deduplication. [RECOMMENDED] |
| Shared evidence leaks transaction/peer identities or creates cardinality growth | Information Disclosure / Denial of Service | Enum-derived fixed labels and aggregate counts only. [VERIFIED: locked D-15; existing relay evidence pattern] |
| Node adapter reclassifies the same delta differently per cache | Tampering | One cache-agnostic semantic delta; complete projection remains Phase 134. [VERIFIED: locked D-13/D-16] |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
| --- | --- | --- | --- |
| — | No training-only factual claims drive this research; design choices are marked `[RECOMMENDED]`, and implementation facts are verified from local primary sources. | All | None identified. |

## Open Questions (RESOLVED)

**Status: RESOLVED by the Phase 130 plan set.**

1. **Legacy snapshot metadata compatibility — resolved in Plan 130-08.**
   - Keep the repository-wide `SchemaVersion::CURRENT` unchanged.
   - Add optional `accepted_at_unix_seconds`, `origin`, and `relay_requested` DTO fields and decode them all-or-none.
   - All three present restore exact known metadata; all three absent become `LegacyUnknown` + `RecoveryUnknown` + `NotRequested`; partial or invalid metadata is typed mempool corruption.
   - Never substitute restart time or infer local origin. Phase 135 may add mempool-local versioning/checkpoint behavior but must preserve this classification.

2. **Raw rolling-floor evidence — resolved in Plan 130-09.**
   - Preserve Knots field meanings: `mempoolminfee` remains the derived effective `max(static, rolling)`, `minrelaytxfee` remains static, and `incrementalrelayfee` remains incremental.
   - Expose raw rolling and explicitly derived effective admission through Open Bitcoin extension fields `rollingmempoolfee` and `effectiveadmissionfee`; do not redefine a Knots field.

3. **Transitional vsize trim limit — resolved in Plan 130-01.**
   - Use the explicit `legacy_vsize_trim_limit: TransactionVirtualSize` field beside the distinct `mempool_capacity: MempoolCapacity`.
   - Phase 130 trimming reads only `legacy_vsize_trim_limit`; Phase 131 removes that seam when accounted-memory enforcement lands.

## Sources

### Primary (HIGH confidence)

- `.planning/phases/130-resource-time-and-fee-primitives/130-CONTEXT.md` — locked Phase 130 decisions and boundary.
- `.planning/REQUIREMENTS.md` — FEEP-01 through FEEP-05.
- `.planning/ROADMAP.md` — Phase 130 goal, success criteria, and Phase 131/134 separation.
- `.planning/research/{ARCHITECTURE,PITFALLS,SUMMARY,STACK}.md` — milestone sequencing and existing seam inventory.
- `packages/open-bitcoin-mempool/src/{types.rs,outcome.rs,pool.rs,pool/lifecycle.rs}` — current resources, fees, entries, outcomes, trimming, and lifecycle.
- `packages/open-bitcoin-node/src/network/{admission_bridge.rs,mempool_lifecycle.rs,relay_fanout.rs,runtime_authority.rs}` — current explicit timestamps, cache projection, fixed labels, and sole authority.
- `packages/open-bitcoin-node/src/storage/{mempool_snapshot.rs,snapshot_codec.rs}` — current metadata gap and replay contract.
- `packages/open-bitcoin-rpc/src/{dispatch/node.rs,method/node.rs}` — current `getmempoolinfo` schema/projection.
- `packages/bitcoin-knots/src/{txmempool.h,txmempool.cpp,rpc/mempool.cpp,validation.cpp}` — pinned resource, fee, admission, and RPC semantics.
- `packages/bitcoin-knots/src/kernel/{mempool_entry.h,mempool_removal_reason.h}` — pinned acceptance-time and removal-cause vocabulary.
- `packages/bitcoin-knots/src/node/mempool_persist.cpp` and `src/net_processing.cpp` — persisted acceptance time, unbroadcast recovery, and fresh-cycle retry jitter.
- `AGENTS.md`, `AGENTS.bright-builds.md`, relevant `standards/` pages, and `standards-overrides.md` — repository workflow and architecture constraints.

No secondary or tertiary web source materially drives the recommendations; local source at the pinned commit is the authoritative behavior baseline. [VERIFIED: repository policy and pinned submodule]

## Metadata

**Confidence breakdown:**

- Standard stack: **HIGH** — versions and targets were verified from manifests and installed tools.
- Architecture: **HIGH** — locked decisions and current first-party seams directly determine module ownership.
- Knots parity meanings: **HIGH** — verified against the pinned local implementation and RPC help.
- Accounted-memory formula components: **MEDIUM-HIGH** — the ownership boundary is locked and the recommended components match current Rust structures, but implementation should validate constants/formula with tests before freezing the contract.
- Legacy snapshot transition: **MEDIUM** — the missing metadata is verified, but the narrow compatibility treatment must avoid stealing Phase 135's complete recovery scope.

**Research date:** 2026-07-23
**Valid until:** 2026-08-22, or until Phase 130 code materially changes.
