# Phase 144: Operator Flush and Availability Evidence - Research

**Researched:** 2026-09-17
**Domain:** Sanitized operator evidence for flush, coins recovery, cache-size, and have-bytes versus do-not
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Add one dedicated evidence projection on
  `OpenBitcoinStatusSnapshot` (suggested name `chainstate_durability` or
  `flush_availability`) consumed by status, `openbitcoinnetworkstatus`, CLI
  JSON/human, dashboard rows, metrics, structured logs, and support bundles.
  Renderers must not re-derive flush, cache-size, recovery, or have-bytes
  truth from local heuristics.
- **D-02:** Preserve `FieldAvailability` semantics from Phases 72/105/116:
  unavailable when runtime has not projected evidence yet; never fabricate
  coins best-block, cache-size, flush reason, or payload-present facts.
- **D-03:** Do not fold this contract into `recovery_evidence` (sync-recovery
  taxonomy) or `block_relay` (serving/compact counters). Those surfaces stay
  as they are. CSOBS evidence is a distinct snapshot field.
- **D-04:** Do not invent a `getblock` RPC or CLI product. Phase 143 D-11
  still holds. Availability "per-request" evidence is classification labels
  and aggregate counters, not a new stored-block fetch API.
- **D-05:** Project existing `FlushDecisionFacts` without a second policy:
  `cache_size` labels `ok` / `large` / `critical`, and `last_flush_reason`
  labels `none` / `needed` / `periodic` / `always` / `failed_disk` (140 D-03).
  Do not recompute LARGE/CRITICAL thresholds in the operator crate.
- **D-06:** Also project last write-kind `none` / `flush` / `sync` /
  `refuse_disk_space` and CanFlush readiness `not_ready` / `ready_to_flush`
  from the Phase 142 manager. `FlushMode::None` remains observability-only.
- **D-07:** Occupancy may include aggregate `cache_bytes` and
  `cache_byte_limit` already known to the flush lifecycle. Do not treat Fjall
  item count as size, and do not dump per-coin occupancy.
- **D-08:** Interrupted-flush / replay evidence uses a distinct low-cardinality
  outcome vocabulary such as `consistent` / `replayed` / `interrupted` /
  `fail_closed`. Do not overload `SyncRecoveryCategory` (`clean_shutdown`,
  `store_corruption`, …) for coins-marker recovery.
- **D-09:** Coins best-block may appear as height plus hash, matching existing
  sync tip-hash fields. That is chain-authority evidence, not a coin dump.
  Outpoints, coin values, script bytes, and UTXO maps stay off every shared
  surface.
- **D-10:** Interrupted, replayed, and fail-closed outcomes must be
  distinguishable. Fail-closed evidence must not present a invented consistent
  tip. Peer identifiers stay off this surface.
- **D-11:** Project the Phase 143 HAVL-03 facts as sanitized labels:
  `payload_present`, `index_known`, `validated_on_active_chain`, plus serving
  status `available` / `unavailable`. Do not emit `pruned` or
  `block_status_pruned` on production paths (143 D-05/D-06).
- **D-12:** "Per-request availability labels" means last/aggregate
  classification labels and bounded counters (`available_count`,
  `unavailable_count`, `index_known_without_payload_count` or equivalent),
  not a hash-keyed request log. No block-hash tables, tx indexes, or peer
  ids on status/support/metrics/logs.
- **D-13:** Do-not is `unavailable` whenever `payload_present` is false.
  `index_known` or `validated_on_active_chain` cannot authorize Available
  or a serve. Operator copy must not read as prune-mode, archive-node, or
  public-default historical serving.
- **D-14:** Open Bitcoin-specific status carries the new field
  (`openbitcoinnetworkstatus` / shared snapshot). Baseline-compatible
  methods (`getblockchaininfo`, `getnetworkinfo`, and any existing stored-block
  RPC) stay unchanged. Do not expand Knots-shaped objects with ad hoc Open
  Bitcoin keys.
- **D-15:** Human CLI status adds compact `Label: value` lines from the
  shared contract (cache-size, last flush reason, coins best-block,
  recovery outcome, have-bytes vs do-not). JSON mode serializes the snapshot
  only.
- **D-16:** Dashboard adds or swaps **rows** from the same contract. Keep the
  existing four-band Ratatui layout and `MAX_DASHBOARD_CHARTS = 8`. Do not
  add a ninth chart, a new panel/section that breaks the 35/35/30 split, a
  hosted web dashboard, or shadcn/browser chrome. Follow the Phase 137 TUI
  contract: one label, one single-line value, `Unavailable: {reason}` for
  missing fields.
- **D-17:** Metrics are fixed `MetricKind` numeric series for flush outcomes,
  cache-size classification samples, recovery outcomes, and availability
  classifications. No dynamic labels, no per-peer/per-outpoint/per-block-hash
  series.
- **D-18:** Structured logs use an allowlisted source with stable
  `cause` / `outcome` / `label` fields. Sanitizers reject coin dumps, peer
  endpoints, credentials, and high-cardinality hashes except the existing
  tip/coins-best-block hash pattern already used on sync status.
- **D-19:** Support bundles consume the shared snapshot through existing
  recursive redaction. Include bounded flush/recovery/availability summaries
  only. Reject raw coin records, undo blobs, peer identifiers, dynamic
  labels, and raw hex.
- **D-20:** Update `docs/architecture/status-snapshot.md`,
  `docs/architecture/operator-observability.md`, and
  `docs/operator/runtime-guide.md` with the new field contract and
  copy-pasteable repo-local Cargo and Bazel inspection commands. Public-network
  review stays opt-in UAT only.
- **D-21:** Add a deterministic cross-surface checker verifying RPC/CLI JSON,
  dashboard projection, metrics/log label registry, and support redaction
  agree on core CSOBS fields or the same unavailable reasons.
- **D-22:** New or touched first-party Rust source/test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity
  breadcrumbs and `docs/parity/source-breadcrumbs.json` updates, using `none`
  only when no defensible Knots anchor exists.
- **D-23:** Verification remains `bash scripts/verify.sh`, deterministic, and
  public-network-free. Phase 145 still owns no-claim guardrails; this phase
  must not claim prune/archive, assumeutxo, compact filters, public defaults,
  or production readiness.

### Claude's Discretion
- Exact snapshot field, type, and module names (`chainstate_durability` vs
  `flush_availability`) as long as D-01/D-03 hold.
- Whether coins best-block hash reuses the existing sync hash renderer or a
  sibling field, provided D-09 holds.
- Exact counter names, CLI line placement (after block-relay vs a compact
  durability cluster), checker filename, and whether `as_str` helpers live
  beside `FlushDecision` or only on the status projection.
- How last classification vs aggregate counters are split, provided D-12
  does not become a request log.

### Deferred Ideas (OUT OF SCOPE)
- Parity-root citations for coins/flush/manager/serve-path and deterministic
  no-claim guardrails — Phase 145 (CSVFY-01, CSVFY-02).
- `getblock` product surface — out of v2.3 unless a later phase owns it.
- Prune-mode `Pruned` emission, archive-node claims, assumeutxo, compact
  filters, public serving defaults, production readiness — future requirements
  FUT-18 through FUT-23.
- Hosted web dashboard / GUI — project non-goal.

None — discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CSOBS-01 | Status, RPC, CLI, dashboard, metrics, logs, and support expose flush, recovery, and have-bytes vs do-not using sanitized low-cardinality fields. | One new `OpenBitcoinStatusSnapshot` field plus the Phase 116/137 projector family; retain last flush/recovery on `FlushLifecycle` and last/aggregate HAVL labels on a sibling serve accumulator; project through `openbitcoinnetworkstatus`, CLI, Ratatui rows, `MetricKind`, allowlisted logs, and support redaction. |
| CSOBS-02 | Operator evidence reports cache-size state (OK / LARGE / CRITICAL) and last flush reason. | Project `FlushDecisionFacts.cache_size` / `LastFlushReason` already carried on every `FlushDecision`. Current occupancy via `FlushMode::None` (observability-only). Do not recompute 90% / 10 MiB thresholds in CLI, RPC, or dashboard. |
</phase_requirements>

## Summary

Phase 144 is an operator-evidence rollout, not a new flush or serving policy. Phases 140–143 already own the truth: `FlushDecision` / `FlushDecisionFacts`, `ManagerReadiness`, interrupted-flush replay versus fail-closed, coins `B` best-block, and HAVL-03 presence facts on `ManagedBlockServeDecision.presence`. Those facts are not yet on `OpenBitcoinStatusSnapshot`, `openbitcoinnetworkstatus`, CLI, dashboard, metrics, logs, or support. [VERIFIED: packages/open-bitcoin-node/src/status.rs, packages/open-bitcoin-rpc/src/method/node.rs, packages/open-bitcoin-node/src/network/block_serving.rs]

The planner must copy the Phase 116/137 one-contract-many-surfaces playbook: add one dedicated snapshot field, keep `recovery_evidence` and `block_relay` unchanged, and teach every renderer to print that field or the same `Unavailable: {reason}`. Live CLI status reads `block_relay` from `OpenBitcoinNetworkStatusResponse`, not from a second collector, so the new field must be added to the snapshot, the managed operator snapshot, and the Open Bitcoin RPC response together. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]

Two retention gaps block a naive "just serialize FlushDecision" plan. `FlushLifecycle` currently discards `FlushExecution.decision` after `execute_flush` and discards `RecoveryDecision` after `initialize`. Serve-path presence facts exist on each decision but are not accumulated. The phase must retain last flush/recovery/occupancy on the manager and last/aggregate HAVL labels on a sibling accumulator, then project them. Do not have CLI or dashboard re-open Fjall and reclassify. [VERIFIED: packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs, packages/open-bitcoin-node/src/network/block_relay_evidence.rs]

**Primary recommendation:** Name the field `chainstate_durability`, put types in `status/chainstate_durability.rs`, retain facts in the manager/serve seam, and fan them out with the Phase 116 projector family. No `getblock`, no ninth chart, no `Pruned`, no Phase 145 no-claim closeout.

## Project Constraints (from .cursor/rules/)

No `.cursor/rules/` directory exists in this repo. [VERIFIED: glob `.cursor/rules`]

Applicable constraints instead come from `AGENTS.md`, `AGENTS.bright-builds.md`, and managed `standards/`:

- Functional core / imperative shell; no rust-bitcoin in production; Fjall stays in the node shell. [CITED: AGENTS.md]
- New modules use `foo.rs` plus `foo/`, not `foo/mod.rs`. [CITED: standards/languages/rust.md]
- Prefix nullable bindings with `maybe`. [CITED: AGENTS.bright-builds.md]
- Files near `floor(100 * tau)` (~628 lines) are refactor triggers; `status.rs` is already 618 lines. [VERIFIED: wc -l packages/open-bitcoin-node/src/status.rs]
- Verification is `bash scripts/verify.sh`. UAT commands must be repo-local Cargo and Bazel, not only the `open-bitcoin` alias. [CITED: AGENTS.md]
- New first-party Rust source/test files need parity breadcrumbs. [CITED: AGENTS.md]
- Terminal dashboard is the existing Ratatui surface; no hosted web UI. [CITED: 137-UI-SPEC.md]
- `standards-overrides.md` has no active local exceptions. [VERIFIED: standards-overrides.md]

## Standard Stack

This phase adds no libraries. Use the pinned first-party stack.

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust / Cargo | `1.94.1` (edition 2024) | First-party crates | Pinned by `rust-toolchain.toml`; local `rustc`/`cargo` match. [VERIFIED: rust-toolchain.toml, rustc --version] |
| `open-bitcoin-chainstate` | workspace `0.1.0` | `FlushDecision`, `FlushDecisionFacts`, `decide_flush`, `decide_recovery` | Sole flush/recovery policy; I/O-free. [VERIFIED: packages/open-bitcoin-chainstate/src/coins/flush.rs] |
| `open-bitcoin-node` | workspace `0.1.0` | Snapshot, flush lifecycle, serve facts, metrics, logs | Shared evidence owner. [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| `open-bitcoin-rpc` | workspace `0.1.0` | `openbitcoinnetworkstatus` | Open Bitcoin-only status RPC. [VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs] |
| `open-bitcoin-cli` | workspace `0.1.0` | Human/JSON status, Ratatui dashboard, support | Existing operator surfaces. [VERIFIED: packages/open-bitcoin-cli/src/operator/] |
| Fjall | `3.1.4` | Durable coins `B`/`H` (read, do not re-encode) | Existing coins adapter; do not treat item count as size. [VERIFIED: packages/open-bitcoin-node/Cargo.toml] |
| serde / serde_json | `1.0.228` / `1.0.149` | Snapshot and RPC JSON | Existing `FieldAvailability` tagged enum. [VERIFIED: packages/open-bitcoin-node/Cargo.toml] |
| Ratatui | `0.30` | Terminal dashboard rows | Existing TUI; no new widgets. [VERIFIED: packages/open-bitcoin-cli/Cargo.toml] |
| Bun | pin `.bun-version` `1.3.9`; local `1.4.2` | Deterministic TypeScript checker | Repo-owned checkers; no `package.json`. [VERIFIED: .bun-version, bun --version] |
| Bazel / Bazelisk | Bazel `8.6.0`, Bazelisk `1.28.1` | UAT `bazel run` commands | AGENTS.md UAT contract. [VERIFIED: bazel --version] |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| clap | existing CLI | Operator flags | Do not add new CLI products. |
| Tokio / Axum | existing RPC server | Serve `openbitcoinnetworkstatus` | Do not add a stored-block fetch route. |
| Crossterm | Ratatui backend | Existing dashboard | Do not restyle chrome. |
| `scripts/check-phase116-operator-block-relay-evidence.ts` | repo script | Cross-surface checker template | Copy structure for Phase 144. [VERIFIED: scripts/check-phase116-operator-block-relay-evidence.ts] |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Dedicated `chainstate_durability` field | Fold into `recovery_evidence` or `block_relay` | Forbidden by D-03. `SyncRecoveryCategory` is a different taxonomy. |
| `flush_availability` field name | `chainstate_durability` | `flush_availability` undersells coins best-block and recovery. Recommend `chainstate_durability`. |
| Recompute cache-size in CLI | `decide_flush(FlushMode::None)` | D-05/D-06: operator crate must not own 90% / 10 MiB math. |
| New `getblock` RPC | Last/aggregate HAVL labels | No `getblock` exists; D-04 forbids inventing one. [VERIFIED: packages/open-bitcoin-rpc/src/method.rs] |
| Ninth dashboard chart | Rows in existing bands | D-16 and `MAX_DASHBOARD_CHARTS = 8`. [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs] |
| Offline CLI Fjall coins scan | Manager-retained projection | Avoids coin dumps and a second derivation. D-01/D-02. |

**Installation:** none. Do not add crates.

**Version verification:** Rust `1.94.1` (2026-03-25), Fjall `3.1.4`, Ratatui `0.30`, serde `1.0.228`, Bun pin `1.3.9`. [VERIFIED: local toolchain and Cargo.toml files]

## Architecture Patterns

### Recommended Project Structure
```
packages/open-bitcoin-node/src/
├── status.rs                              # add one serde(default) field only
├── status/chainstate_durability.rs        # NEW shared contract + as_str labels
├── status/chainstate_durability/tests.rs  # NEW unit tests
├── chainstate/flush_lifecycle.rs          # retain last decision / recovery / occupancy
├── network/operator_snapshot.rs           # copy retained facts onto managed snapshot
├── network/block_relay_evidence.rs        # do not extend; sibling recorder instead
├── network/chainstate_durability_evidence.rs  # NEW last/aggregate HAVL accumulator
├── metrics/chainstate_durability.rs       # NEW MetricKind samples
└── logging.rs                             # NEW allowlisted log record helper

packages/open-bitcoin-rpc/src/
├── method/node.rs                         # add field to OpenBitcoinNetworkStatusResponse
└── dispatch/node.rs                       # project from authoritative snapshot

packages/open-bitcoin-cli/src/operator/
├── status.rs                              # live + stopped constructors
├── status/render/chainstate_durability.rs # NEW human lines
├── dashboard/model/chainstate_durability.rs # NEW rows
└── support/render/chainstate_durability.rs  # NEW support bullets + redaction

scripts/
├── check-phase144-operator-flush-availability-evidence.ts
└── check-phase144-operator-flush-availability-evidence.test.ts
```

### Pattern 1: One snapshot field, many thin renderers
**What:** `OpenBitcoinStatusSnapshot` is the sole shared model. RPC, CLI JSON, CLI human, dashboard, metrics, logs, and support clone or format that field. They do not reclassify.
**When to use:** Every CSOBS surface.
**Example:**

```rust
// Source: packages/open-bitcoin-node/src/status.rs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenBitcoinStatusSnapshot {
    pub node: NodeStatus,
    // ...
    #[serde(default)]
    pub block_relay: BlockRelayEvidenceStatus,
    #[serde(default)]
    pub recovery_evidence: FieldAvailability<RecoveryEvidenceSnapshot>,
    // ADD: #[serde(default)] pub chainstate_durability: FieldAvailability<ChainstateDurabilityEvidence>,
}
```

Live CLI already maps RPC `block_relay` into the snapshot. Repeat that for the new field:

```rust
// Source: packages/open-bitcoin-cli/src/operator/status.rs
let network_status = collect_open_bitcoin_network_status(rpc_client);
let block_relay = network_status.block_relay;
// ADD: let chainstate_durability = network_status.chainstate_durability;
```

Stopped / unreachable constructors must set `FieldAvailability::unavailable(reason)` with `#[serde(default)]` so old fixtures deserialize. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]

### Pattern 2: Retain, then project — do not re-decide in renderers
**What:** `FlushLifecycle` already calls `decide_flush` and `decide_recovery`. It must remember the last outcomes so status collection is a copy.
**When to use:** Manager init, every `execute_flush`, and snapshot assembly.

Split the two clocks of CSOBS-02:

| Field | Authoritative source | Why |
|-------|----------------------|-----|
| `cache_size`, `cache_bytes`, `cache_byte_limit` | `decide_flush(FlushMode::None)` at projection time, using current `CoinsCache::estimated_cache_bytes()` and lifecycle limit | `None` is observability-only (140 D-17 / 142 D-16) and still classifies occupancy. [VERIFIED: packages/open-bitcoin-chainstate/src/coins/flush.rs] |
| `last_flush_reason`, `write_kind` | Last `FlushExecution.decision` from `execute_flush` | A later `None` classification must not overwrite `needed` / `periodic` / `always` / `failed_disk`. |
| `readiness` | `ManagerReadiness` | Already on `FlushLifecycle`. [VERIFIED: packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs] |
| `recovery_outcome` | Last `initialize` result, mapped to `consistent` / `replayed` / `interrupted` / `fail_closed` | `RecoveryDecision` today is discarded after `apply_recovery_decision`. |
| `coins_best_block` | `FjallCoinsView::best_block()` hash + `HeaderStore::entry(hash).height` | Hash-only on disk; height is a header lookup. [VERIFIED: packages/open-bitcoin-node/src/storage/coins_view.rs, packages/open-bitcoin-network/src/header_store.rs] |

Do not call `classify_cache_size` from CLI — it is `pub(crate)` in chainstate. The public hook is `decide_flush`. [VERIFIED: packages/open-bitcoin-chainstate/src/coins/flush.rs]

### Pattern 3: Sibling HAVL accumulator, not `block_relay` extension
**What:** `ManagedBlockServeDecision` already carries `presence: BlockServingPresenceFacts { payload_present, index_known, validated_on_active_chain }` and `status_label`. `record_block_serving` increments `BlockServingStatusCounters` including `pruned_count`. Leave that path alone. Add a sibling recorder that stores last labels plus bounded counters. [VERIFIED: packages/open-bitcoin-node/src/network/block_serving.rs, packages/open-bitcoin-node/src/network/block_relay_evidence.rs]

**When to use:** Every serve / inventory classification already recorded for block-relay.

Rules:

- Last labels: `payload_present`, `index_known`, `validated_on_active_chain`, `serving_status` (`available` / `unavailable`).
- Do-not: if `payload_present == false`, last serving status is `unavailable` even when index/validated are true (D-13).
- Counters: `available_count`, `unavailable_count`, `index_known_without_payload_count` (and optional `payload_present_count` if needed for metrics). No hash map.
- Never increment a `pruned` counter on this field. Existing `block_relay.block_serving.status.pruned_count` stays on the old surface only.

### Pattern 4: Phase 137 TUI contract
**What:** Four vertical bands (title 3, sections `Min(10)`, charts 8, actions 4). Horizontal split 35/35/30. Cyan/Bold titles, Gray labels, one-line rows. [CITED: 137-UI-SPEC.md]

**When to use:** Dashboard and human CLI.

Recommended human cluster after block-relay lines:

```text
Chainstate durability: cache_size=ok last_flush_reason=periodic write_kind=sync readiness=ready_to_flush
Coins best-block: height=840004 hash=1111…1111
Coins recovery: replayed
Have-bytes: available payload_present=true index_known=true validated_on_active_chain=true
Have-bytes counts: available_count=3 unavailable_count=1 index_known_without_payload_count=1
```

Missing fields render `Unavailable: {reason}` using the snapshot reason, not a second copy.

### Anti-Patterns to Avoid
- **Folding CSOBS into `recovery_evidence`:** that type is `SyncRecoveryCategory` (`clean_shutdown`, `store_corruption`, …). Coins-marker recovery is a different vocabulary (D-08). [VERIFIED: packages/open-bitcoin-node/src/status/recovery.rs]
- **Extending `BlockRelayEvidenceStatus`:** that is serving/compact counters. HAVL facts belong on the new field (D-03).
- **Recomputing LARGE/CRITICAL in CLI:** thresholds live in chainstate (`9/10` and `10 MiB`). [VERIFIED: packages/open-bitcoin-chainstate/src/coins/flush.rs]
- **Inventing `getblock`:** no such method is registered. [VERIFIED: packages/open-bitcoin-rpc/src/method.rs]
- **Emitting `pruned` on the new field:** Phase 143 reserved `Pruned` and forbids production emission (D-11).
- **Ninth chart or new dashboard band:** `MAX_DASHBOARD_CHARTS = 8`. [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs]
- **Appending types into `status.rs`:** 618 lines; add a sibling module. [VERIFIED: wc -l]
- **Using Fjall `len()` as cache size:** 141 D-07; occupancy is `estimated_cache_bytes`. [VERIFIED: packages/open-bitcoin-chainstate/src/coins/cache.rs]
- **Inventing a consistent tip on fail-closed:** if `B` is missing, coins best-block is unavailable; do not substitute interrupted `H` hashes (D-10).
- **Offline CLI opening the coins keyspace:** second derivation and coin-dump risk. Project from the running snapshot or mark unavailable (D-01/D-02).
- **Dynamic metric labels:** `MetricKind` is a closed enum; `ALL` is a fixed `[Self; 74]` today and must be updated by exact length. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs]
- **Putting 64-char hashes in structured-log field sanitizers:** `looks_like_hex_material` redacts hex length `>= 16`. Status/CLI/dashboard may show coins best-block hash like tip hash; logs should carry labels, height, and counts, not the hash, unless an explicit allowlist matching sync-status is added. [VERIFIED: packages/open-bitcoin-node/src/logging.rs]
- **Claiming prune/archive/public-default/production readiness in docs:** Phase 145 owns CSVFY-02 (D-23).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Flush / cache-size policy | Second LARGE/CRITICAL classifier | `decide_flush` / `FlushDecisionFacts` | 140 D-03; constants already pinned. |
| Recovery sketch | CLI `H` count parser | `decide_recovery` + retained initialize outcome | Count-only core already exists. |
| Coins best-block decode | Ad-hoc `B` key reader in CLI | `FjallCoinsView::best_block` via manager projector | Avoid coin dumps and schema drift. |
| Height for coins hash | Guess from sync tip | `HeaderStore::entry(&hash).height` | Header entry already has `height: u32`. [VERIFIED: packages/open-bitcoin-network/src/header_store.rs] |
| Presence classification | Renderer heuristics | `BlockServingPresenceFacts` already on the serve decision | Phase 143 wired facts; 144 only projects them. |
| Availability wrapper | `Option` / booleans | `FieldAvailability<T>` tagged `state`/`value` | Phases 72/105/116 contract. [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| Support redaction | New recursive walker | `support_status_for_bundle` + sibling `redact_chainstate_durability` | Existing recursive path. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs] |
| Cross-surface drift | Manual review only | Bun checker modeled on Phase 116 | D-21; `verify.sh` already runs those checkers. |
| Dashboard widgets | New chart / panel | Existing `row(label, value)` | 137 TUI contract. |
| Metric series names | Stringly labels | New `MetricKind` variants + `ALL` | Closed enum, snake_case serde. |

**Key insight:** The expensive work is already done in core and the serve seam. This phase is retention plus projection. Hand-rolling policy or a fetch API would recreate Phases 140–143 and violate locked decisions.

## Common Pitfalls

### Pitfall 1: Field added to snapshot but not to live RPC
**What goes wrong:** Dashboard and `status --format json` stay `Unavailable` on a running node.
**Why it happens:** Live collection copies `OpenBitcoinNetworkStatusResponse`, not the full in-process snapshot. [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]
**How to avoid:** Add the field to `OpenBitcoinStatusSnapshot`, `ManagedNetworkOperatorSnapshot`, and `OpenBitcoinNetworkStatusResponse` in the same plan. Update `dispatch/node.rs` and the exact-key test in `network_status_schema.rs` (currently `block_relay`, `inbound`, `mempool`, `metrics`, `relay`). [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/tests/network_status_schema.rs]
**Warning signs:** RPC JSON has the field, CLI JSON does not, or the reverse.

### Pitfall 2: Last `FlushDecision` used as current cache-size
**What goes wrong:** Operators see `ok` hours after the cache grew to `critical`.
**Why it happens:** `execute_flush` is IfNeeded / Periodic / Always; occupancy changes between writes.
**How to avoid:** Refresh `cache_size` and occupancy with `FlushMode::None` at projection time. Keep `last_flush_reason` and `write_kind` from the last real `execute_flush`.
**Warning signs:** Tests that only assert cache-size on the last write decision.

### Pitfall 3: Fail-closed presents `H[0]` as coins best-block
**What goes wrong:** Invented consistent tip (D-10).
**Why it happens:** Interrupted flush has `H = [new, old]` and missing `B`. `best_block()` is `None`.
**How to avoid:** `coins_best_block` is unavailable on fail-closed / interrupted-without-`B`. Recovery label is `fail_closed` or `interrupted`. Do not publish interrupted head hashes.
**Warning signs:** Hash present while recovery is `fail_closed`.

### Pitfall 4: Initialize failure never reaches operator surfaces
**What goes wrong:** CSOBS-01 recovery evidence is missing for the most important outcome.
**Why it happens:** `initialize` returns `Err` and never sets `ReadyToFlush`. RPC may never start. [VERIFIED: packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs]
**How to avoid:** Map initialize errors to `fail_closed` on any snapshot the process still emits; emit the same label on the allowlisted log. If the process cannot serve RPC, stopped CLI must stay `Unavailable` (D-02) rather than scanning coins. Do not overload `recovery_evidence.category = store_corruption` as the CSOBS recovery label.
**Warning signs:** Only `SyncRecoveryCategory::StoreCorruption` appears, with no `fail_closed`.

### Pitfall 5: HAVL facts folded into `block_relay` or a request log
**What goes wrong:** D-03/D-12 violations; peer or hash cardinality leaks.
**Why it happens:** `record_block_serving` is the convenient increment site and already has `pruned_count`.
**How to avoid:** Sibling accumulator. Last labels + three counters. Checker rejects `pruned`, `peer_id`, and 64-char maps on the new field.
**Warning signs:** New keys under `block_relay` or a `by_hash` object.

### Pitfall 6: Structured logs leak hex or get fully redacted
**What goes wrong:** Either coin/block hashes appear, or the whole record becomes `redacted_*` and operators see nothing.
**Why it happens:** `looks_like_hex_material` treats `>= 16` hex chars as sensitive. [VERIFIED: packages/open-bitcoin-node/src/logging.rs]
**How to avoid:** Log `cause` / `outcome` / `label` plus numeric counts and maybe height. Put coins best-block hash only on status/CLI/dashboard/support, matching tip-hash practice (D-09/D-18).
**Warning signs:** Log tests that include a 64-char hash, or support Markdown that includes outpoints.

### Pitfall 7: Fixture / `MetricKind::ALL` compile breaks
**What goes wrong:** Dozens of `OpenBitcoinStatusSnapshot { ... }` literals and the `[Self; 74]` array fail to compile.
**Why it happens:** New struct fields and enum variants are exhaustive.
**How to avoid:** `#[serde(default)]` plus `Default` / `default_unavailable()` on the new type. Update every snapshot literal in node/cli/rpc tests. When adding `MetricKind` variants, update `ALL` length and `persist_metrics` tests that match on kind sets.
**Warning signs:** First `cargo test` wave is all missing-field / array-length errors.

### Pitfall 8: Docs or checker claim prune/archive/public defaults
**What goes wrong:** Phase 145 scope leak; D-23.
**Why it happens:** Honest-availability copy is easy to over-read as archive-node honesty.
**How to avoid:** Docs say have-bytes versus do-not. Checker needles include `archive-node`, `prune mode`, `public default`, `production ready` as forbidden on the new field docs.
**Warning signs:** Runtime-guide sentences that start from "historical serving is available when…".

## Code Examples

Verified patterns from this repo:

### FieldAvailability (do not replace)
```rust
// Source: packages/open-bitcoin-node/src/status.rs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state", content = "value")]
pub enum FieldAvailability<T> {
    Available(T),
    Unavailable { reason: String },
}
```

### FlushDecisionFacts to project (do not recompute)
```rust
// Source: packages/open-bitcoin-chainstate/src/coins/flush.rs
pub enum CoinsCacheSizeState { Ok = 0, Large = 1, Critical = 2 }
pub enum LastFlushReason { None, Needed, Periodic, Always, FailedDisk }
pub struct FlushDecisionFacts {
    pub cache_size: CoinsCacheSizeState,
    pub reason: LastFlushReason,
}
pub enum FlushDecision {
    None(FlushDecisionFacts),
    Flush(FlushDecisionFacts),
    Sync(FlushDecisionFacts),
    RefuseDiskSpace(FlushDecisionFacts),
}
```

Recommended operator labels (`as_str` on the status projection, not on core):

| Core value | Machine label | Human title |
|------------|---------------|-------------|
| `CoinsCacheSizeState::Ok` | `ok` | `OK` |
| `Large` | `large` | `LARGE` |
| `Critical` | `critical` | `CRITICAL` |
| `LastFlushReason::None` | `none` | `None` |
| `Needed` | `needed` | `Needed` |
| `Periodic` | `periodic` | `Periodic` |
| `Always` | `always` | `Always` |
| `FailedDisk` | `failed_disk` | `Failed disk` |
| `FlushDecision::None` | `none` | write-kind |
| `Flush` | `flush` | write-kind |
| `Sync` | `sync` | write-kind |
| `RefuseDiskSpace` | `refuse_disk_space` | write-kind |
| `ManagerReadiness::NotReady` | `not_ready` | CanFlush |
| `ReadyToFlush` | `ready_to_flush` | CanFlush |

### Recovery mapping (distinct from SyncRecoveryCategory)
```rust
// Source: packages/open-bitcoin-chainstate/src/coins/flush.rs
pub enum RecoveryDecision {
    ConsistentEmptyHeads, // + present B after init → consistent
    OneHead,              // treat as consistent-empty-adjacent; do not invent tip
    InterruptedTwoHeads,  // successful replay → replayed; missing bodies → fail_closed
    InconsistentOtherCount { count: usize }, // fail_closed
}
```

Do not serialize `count` on the operator field (cardinality / debug leak). Map to `fail_closed`.

### Serve-path facts already present
```rust
// Source: packages/open-bitcoin-node/src/network/block_serving.rs
pub(super) struct BlockServingPresenceFacts {
    pub payload_present: bool,
    pub index_known: bool,
    pub validated_on_active_chain: bool,
}
```

Hook the sibling accumulator from `record_block_serving_evidence`, which already receives `&ManagedBlockServeDecision`. [VERIFIED: packages/open-bitcoin-node/src/network/block_relay_evidence.rs]

### Structured log shape
```rust
// Source: packages/open-bitcoin-node/src/logging.rs
pub fn block_relay_log_record(...) -> StructuredLogRecord {
    let message = format!(
        "outcome=projected cause=status_projection label=block_relay ..."
    );
    StructuredLogRecord::new(
        StructuredLogLevel::Info,
        BLOCK_RELAY_LOG_SOURCE,
        message,
        timestamp_unix_seconds,
    )
}
```

Mirror as `CHAINSTATE_DURABILITY_LOG_SOURCE = "chainstate_durability"` with `outcome=` / `cause=` / `label=` and numeric counts only.

### Dashboard row helper
```rust
// Source: packages/open-bitcoin-cli/src/operator/dashboard/model/block_relay.rs
row("Block relay activation", activation_text(&status.block_serving.activation))
// Unavailable branch:
// format!("Unavailable: {reason}")
```

### Checker wiring
Copy `scripts/check-phase116-operator-block-relay-evidence.ts`: target file list, required symbols, required counters, required behavior-test names, redaction needles, and repo-local Cargo/Bazel commands. Add both `bun test` and `bun run` lines to `scripts/verify.sh` next to the Phase 116 steps. [VERIFIED: scripts/verify.sh]

Recommended forbidden needles on the new field: `pruned`, `block_status_pruned`, `peer_id=`, `127.0.0.1:`, outpoint-shaped `txid:vout`, `cmpctblock`, credential markers, `getblock`.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Renderer-local summaries | One `OpenBitcoinStatusSnapshot` field | Phases 72/105/116/137 | Phase 144 must not invent a parallel model. |
| Caller-supplied `durable_availability: bool` | Payload-byte probe + three HAVL facts | Phase 143 (2026-09-17) | Facts exist; surfaces do not. |
| Leftover snapshot blob as UTXO truth | Coins `B` / `H` + `FlushLifecycle` | Phases 141–142 | Project coins best-block and flush outcomes, not snapshot blobs. |
| `SyncRecoveryCategory` for all recovery | Separate coins-marker vocabulary | This phase (D-08) | Do not reuse `store_corruption` as CSOBS recovery. |
| `Pruned` for missing historical bodies | `Unavailable` unless prune mode deleted files | Phase 143 | New field never emits `pruned`. |

**Deprecated/outdated:**
- Treating `block_relay.pruned_count` as CSOBS have-bytes evidence. Leave the old counter; do not document it as prune-mode.
- Snapshot-blob occupancy as cache-size. Use `estimated_cache_bytes`.
- Hosted / shadcn dashboards. Terminal Ratatui only.

## Recommended Discretion Resolutions

These are planner defaults for CONTEXT.md discretion. They are not locked user decisions.

| Discretion | Recommendation | Why |
|------------|----------------|-----|
| Field / type name | `chainstate_durability` / `ChainstateDurabilityEvidence` | Covers flush, recovery, coins tip, and have-bytes. `flush_availability` reads as serving-only. |
| Module layout | `status/chainstate_durability.rs` + CLI/dashboard/support siblings | `status.rs` is 618 lines; Rust standard is `foo.rs` + `foo/`. |
| Hash renderer | Sibling fields `maybe_coins_best_block_height` / `maybe_coins_best_block_hash` using the same hex encoding as `maybe_validated_active_chain_hash` | D-09; do not reuse the sync progress object itself. |
| `as_str` home | Status-projection enums only | Keeps chainstate I/O-free and operator labels out of policy types. |
| CLI placement | Compact cluster immediately after block-relay lines | Matches "after block-relay vs durability cluster" option; keeps sync recovery lines distinct. |
| Last vs aggregate | `last_*` labels plus the three D-12 counters | No request log. |
| Checker filename | `scripts/check-phase144-operator-flush-availability-evidence.ts` | Matches Phase 116 naming. |
| Fail-closed offline | Unavailable on stopped CLI; log label if initialize failed | Avoids a second Fjall reader. |
| Metrics kinds | Fixed gauges/counters: cache-size class as `0/1/2`, flush-reason class as `0..4`, recovery class as `0..3`, plus availability counts | No dynamic labels. Cache-size sample is the current `None`-mode class, not last write. |

Suggested `ChainstateDurabilityEvidence` shape (serde snake_case):

```text
cache_size: ok|large|critical
last_flush_reason: none|needed|periodic|always|failed_disk
write_kind: none|flush|sync|refuse_disk_space
readiness: not_ready|ready_to_flush
cache_bytes: u64
cache_byte_limit: u64
recovery_outcome: consistent|replayed|interrupted|fail_closed
maybe_coins_best_block_height: Option<u64>
maybe_coins_best_block_hash: Option<String>  // 64 hex, tip-hash pattern
last_serving_status: available|unavailable
last_payload_present: bool
last_index_known: bool
last_validated_on_active_chain: bool
available_count: u64
unavailable_count: u64
index_known_without_payload_count: u64
```

Wrap the whole struct in `FieldAvailability` so a stopped node is one unavailable reason, not a fabricated zeroed object pretending to be live (D-02). Zeroed counters are acceptable only inside `Available` after the runtime has projected, matching Phase 116 counter posture.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Initialize failure usually prevents RPC from starting, so fail-closed may appear only on logs unless an error-path snapshot is added. | Pitfall 4 | Planner must include an explicit fail-closed projection task, not assume live RPC always exists. |
| A2 | Header-store height is the correct coins best-block height when the hash is present. | Pattern 2 | If coins `B` can precede header index, height must be unavailable rather than guessed from sync tip. |

No other `[ASSUMED]` product or compliance claims.

## Open Questions

1. **Fail-closed visibility when the daemon never binds RPC**
   - What we know: `initialize` returns `Err` on missing undo/bodies or inconsistent `H`. [VERIFIED: packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs]
   - What's unclear: whether any operator snapshot is produced on that path today.
   - Recommendation: add a log record and, if a snapshot exists, project `fail_closed` with unavailable coins best-block. Do not teach CLI to scan `H`.

2. **Whether `RecoveryDecision::OneHead` is `consistent` or `fail_closed`**
   - What we know: `apply_recovery_decision` treats `OneHead` like `ConsistentEmptyHeads` (no replay). [VERIFIED: packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs]
   - What's unclear: operator wording if `B` is also missing.
   - Recommendation: if `best_block()` is `Some`, label `consistent`; if `None`, `fail_closed` without a tip. Do not invent a height.

3. **Exact `MetricKind` cardinality**
   - What we know: `ALL` is `[Self; 74]` and must stay exact. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs]
   - What's unclear: how many new kinds reviewers will accept.
   - Recommendation: add a small closed set (cache-size class, last-reason class, write-kind class, recovery class, three availability counters). Prefer class-as-number over one series per enum value if `ALL` growth is painful, but one series per counter matches Phase 116.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| rustc / cargo | Rust implementation | ✓ | `1.94.1` | — |
| Bun | Checker + `verify.sh` | ✓ | local `1.4.2` (pin `1.3.9`) | Use repo scripts as-is; no install step |
| Bazelisk / Bazel | UAT command docs | ✓ | Bazelisk `1.28.1`, Bazel `8.6.0` | Cargo twin commands required anyway |
| Fjall | Coins best-block read | ✓ | `3.1.4` in tree | — |
| Public network | None | n/a | — | Forbidden in default verify (D-23) |

**Missing dependencies with no fallback:** none.

**Missing dependencies with fallback:** none. Local Bun is newer than `.bun-version`; not a blocker for TypeScript checkers.

Step 2.6: tooling present. Phase is code/docs/checker work on the existing stack.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | Existing RPC cookie/auth unchanged; no new auth surface |
| V3 Session Management | no | No session work |
| V4 Access Control | yes | Shared evidence stays identifier-free; originating RPC is not a coin/block dump |
| V5 Input Validation | yes | Parse RPC/status into domain enums; `FieldAvailability` + serde `rename_all = "snake_case"` |
| V6 Cryptography | no | No new crypto; do not hand-roll hashes |

### Known Threat Patterns for operator evidence

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Coin / UTXO dump on status or support | Information disclosure | Project height+hash only; reject outpoints, values, scripts, undo blobs (D-09/D-19) |
| Peer endpoint or peer id on CSOBS field | Information disclosure | Sibling accumulator stores labels/counters only (D-12) |
| High-cardinality metric labels | Information disclosure / DoS | Closed `MetricKind`; no per-hash series (D-17) |
| Hex / credential leak in logs | Information disclosure | Existing sanitizers; no hash in log fields (D-18) |
| Operator copy implying prune/archive/public serve | Spoofing / misuse | Forbidden `pruned` labels; docs stay have-bytes vs do-not (D-11/D-13/D-23) |
| Invented consistent tip after crash | Tampering / repudiation | Fail-closed without `B` keeps coins best-block unavailable (D-10) |

## Sources

### Primary (HIGH confidence)
- `.planning/phases/144-operator-flush-and-availability-evidence/144-CONTEXT.md` — locked D-01..D-23
- `packages/open-bitcoin-chainstate/src/coins/flush.rs` — `FlushDecision`, `FlushDecisionFacts`, `decide_flush`, `decide_recovery`
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` — `ManagerReadiness`, discarded last decision
- `packages/open-bitcoin-node/src/network/block_serving.rs` — HAVL presence facts
- `packages/open-bitcoin-node/src/status.rs` — snapshot and `FieldAvailability`
- `packages/open-bitcoin-rpc/src/method/node.rs` and `dispatch/node.rs` — `openbitcoinnetworkstatus`
- `packages/open-bitcoin-cli/src/operator/status.rs` — live RPC mapping
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` — four-band layout, `MAX_DASHBOARD_CHARTS`
- `scripts/check-phase116-operator-block-relay-evidence.ts` — checker template
- `137-UI-SPEC.md` — TUI contract
- `AGENTS.md`, `standards/languages/rust.md`, `standards/core/architecture.md`

### Secondary (MEDIUM confidence)
- `.planning/phases/140-CONTEXT.md` through `143-CONTEXT.md` — upstream locked vocabulary
- `docs/architecture/status-snapshot.md` and `docs/architecture/operator-observability.md` — field-ownership tables to extend
- `packages/open-bitcoin-node/src/logging.rs` — hex sanitizer behavior

### Tertiary (LOW confidence)
- A1/A2 in Assumptions Log — fail-closed process lifetime and header-height alignment

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — pinned repo versions verified locally; no new libraries
- Architecture: HIGH — snapshot/RPC/CLI/dashboard/checker paths read in tree; retention gap verified
- Pitfalls: HIGH — derived from locked decisions plus concrete compile/leak traps in existing fixtures

**Research date:** 2026-09-17
**Valid until:** 2026-10-17 (stable in-repo evidence; revisit if Phases 140–143 types move)
