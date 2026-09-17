---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 144-2026-09-17T11-23-17
generated_at: 2026-09-17T11:23:29.410Z
---

# Phase 144: Operator Flush and Availability Evidence - Context

**Gathered:** 2026-09-17
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Operators can see flush, recovery, cache-size, and have-bytes versus do-not
through sanitized surfaces. Status, RPC, CLI, dashboard, metrics, logs, and
support expose those facts as low-cardinality fields without peer identifiers
or raw coin dumps.

This phase delivers CSOBS-01 and CSOBS-02 only. It projects already-authoritative
Phase 140 `FlushDecision` / `CoinsCacheSizeState` / `LastFlushReason` facts,
Phase 142 manager readiness and interrupted-flush / replay outcomes, Phase 141
coins best-block, and Phase 143 `payload_present` / `index_known` /
`validated_on_active_chain` labels onto the existing operator-evidence family.

This phase must not invent a `getblock` product, emit `Pruned` on production
paths, add prune/archive modes, assumeutxo, compact-filter serving, public
defaults, or production-readiness claims. Phase 145 owns parity-root citations
and deterministic no-claim guardrails. Do not re-derive flush or availability
truth in renderers.

</domain>

<decisions>
## Implementation Decisions

### Shared evidence contract
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

### Flush and cache-size vocabulary
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

### Recovery and coins best-block
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

### Have-bytes versus do-not
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

### RPC, CLI, and dashboard
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

### Metrics, logs, support, and docs
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

### Folded Todos
None — `todo match-phase 144` returned no matches.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase and requirement
- `.planning/ROADMAP.md` — Phase 144 goal, CSOBS-01, CSOBS-02, success
  criteria, UI hint (terminal dashboard, not web)
- `.planning/REQUIREMENTS.md` — CSOBS-01 and CSOBS-02 wording; CSVFY-* stay
  Phase 145; FUT-18 prune deferred
- `.planning/PROJECT.md` — sanitized operator evidence, terminal-first
  surface, no hosted dashboard, v2.3 honest-availability already closed in
  Phase 143

### Locked prior decisions
- `.planning/phases/140-pure-flush-policy-and-typed-decisions/140-CONTEXT.md`
  — D-03: cache-size and last-flush-reason live on `FlushDecision`; Phase 144
  reports them; do not invent a second derivation
- `.planning/phases/141-durable-fjall-coins-adapter/141-CONTEXT.md` — coins
  `B` best-block and fail-closed disk reads (CSOBS-03 already owned)
- `.planning/phases/142-manager-flush-lifecycle-and-restart/142-CONTEXT.md`
  — CanFlush readiness, ordered flush, interrupted-flush replay vs fail-closed;
  operator surfaces deferred to this phase
- `.planning/phases/143-honest-stored-block-availability/143-CONTEXT.md` —
  HAVL facts, reserved `Pruned`, no new getblock, operator UI deferred here
- `.planning/phases/116-operator-evidence-metrics-logs-and-support-boundary/116-CONTEXT.md`
  — shared status contract, FieldAvailability, redaction, cross-surface checker
- `.planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-CONTEXT.md`
  — Open Bitcoin status vs baseline RPC, identifier-free shared evidence
- `.planning/phases/137-rpc-and-sanitized-operator-evidence/137-CONTEXT.md`
  — identifier-free snapshot; originating RPC vs shared evidence
- `.planning/phases/137-rpc-and-sanitized-operator-evidence/137-UI-SPEC.md`
  — TUI-only contract: `MAX_DASHBOARD_CHARTS = 8`, one-line rows, no web chrome

### Architecture and operator docs
- `docs/architecture/status-snapshot.md` — sole shared status model
- `docs/architecture/operator-observability.md` — low-cardinality metrics/logs
  and redaction
- `docs/operator/runtime-guide.md` — copy-pasteable Cargo/Bazel UAT commands
- `AGENTS.md` — UAT prefers repo-local Cargo and Bazel commands
- `docs/parity/source-breadcrumbs.json` — breadcrumb contract

### Existing code integration points
- `packages/open-bitcoin-chainstate/src/coins/flush.rs` —
  `FlushDecision`, `CoinsCacheSizeState`, `LastFlushReason`, `RecoveryDecision`
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` —
  `FlushLifecycle`, `ManagerReadiness`, `FlushExecution`
- `packages/open-bitcoin-node/src/network/block_serving.rs` —
  `payload_present` / `index_known` / `validated_on_active_chain`
- `packages/open-bitcoin-node/src/status.rs` — `OpenBitcoinStatusSnapshot`
- `packages/open-bitcoin-node/src/status/block_relay_evidence.rs` — pattern to
  mirror, not extend
- `packages/open-bitcoin-node/src/recovery.rs` — existing
  `RecoveryEvidenceSnapshot` (do not overload)
- `packages/open-bitcoin-rpc/src/method.rs` — `openbitcoinnetworkstatus`
- `packages/open-bitcoin-cli/src/operator/status/` — CLI collect/render
- `packages/open-bitcoin-cli/src/operator/dashboard/` — Ratatui model/render
- `packages/open-bitcoin-cli/src/operator/support/` — support redaction
- `scripts/check-phase116-operator-block-relay-evidence.ts` — checker pattern
- `scripts/verify.sh` — repo-native verification

### Knots anchors (projection only; no new policy)
- `packages/bitcoin-knots/src/validation.cpp` — `FlushStateToDisk` /
  `GetCoinsCacheSizeState`
- `packages/bitcoin-knots/src/node/chainstate.cpp` — manager flush / CanFlush
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — HaveBlockData vs missing

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `FlushDecision` / `FlushDecisionFacts` already carry cache-size and last-flush
  reason on every variant — project them, do not recompute.
- `ManagerReadiness` and `FlushExecution` in the node flush lifecycle already
  own CanFlush and last decision.
- Phase 143 presence facts already exist on the serve/inventory seam.
- `BlockRelayEvidenceStatus` plus CLI/dashboard/support projectors show the
  one-contract-many-surfaces pattern to copy.
- `FieldAvailability<T>` and support redaction helpers already reject
  peer/credential/hex material.

### Established Patterns
- Shared types in `open-bitcoin-node` status; thin RPC/CLI/dashboard adapters.
- Baseline Knots RPC stays narrow; Open Bitcoin evidence lives on
  `openbitcoinnetworkstatus` and the snapshot.
- Dashboard: four vertical bands, eight charts, one-line rows, Cyan/Bold titles,
  Gray labels. New evidence is rows, not charts.
- Deterministic Bun TypeScript checkers under `scripts/` plus `verify.sh`.

### Integration Points
- Snapshot struct in `packages/open-bitcoin-node/src/status.rs`.
- RPC projection beside existing `OpenBitcoinNetworkStatusResponse`.
- CLI human renderer under `packages/open-bitcoin-cli/src/operator/status/render.rs`.
- Dashboard rows beside block-relay projection in
  `packages/open-bitcoin-cli/src/operator/dashboard/model/`.
- Metrics kinds / structured-log allowlists / `support_status_for_bundle`.
- Docs: status-snapshot, operator-observability, runtime-guide.

</code_context>

<specifics>
## Specific Ideas

Reuse the Phase 116/137 operator-evidence playbook rather than inventing a
parallel renderer family. Cache-size labels should stay `ok` / `large` /
`critical` (snake_case machines, title-case humans). Have-bytes versus do-not
should be readable in one CLI line, for example
`have_bytes=true index_known=true validated_on_active_chain=false` or a compact
`available` / `unavailable` plus the three facts.

No getblock. No ninth dashboard chart. No web UI.

</specifics>

<deferred>
## Deferred Ideas

- Parity-root citations for coins/flush/manager/serve-path and deterministic
  no-claim guardrails — Phase 145 (CSVFY-01, CSVFY-02).
- `getblock` product surface — out of v2.3 unless a later phase owns it.
- Prune-mode `Pruned` emission, archive-node claims, assumeutxo, compact
  filters, public serving defaults, production readiness — future requirements
  FUT-18 through FUT-23.
- Hosted web dashboard / GUI — project non-goal.

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 144-operator-flush-and-availability-evidence*
*Context gathered: 2026-09-17*
