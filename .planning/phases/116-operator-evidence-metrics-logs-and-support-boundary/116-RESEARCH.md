---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 116-2026-07-06T03-46-36
generated_at: 2026-07-06T04:10:00Z
---

# Phase 116: Operator Evidence, Metrics, Logs, and Support Boundary - Research

**Researched:** 2026-07-06
**Domain:** block-serving and compact-relay operator evidence projection (RPC, CLI, dashboard, metrics, logs, support)
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** Extend the shared status layer in `open-bitcoin-node` with a `BlockRelayEvidenceStatus` (or equivalent) that composes existing `BlockServingEvidenceStatus` plus compact-relay evidence fields: negotiation counters, announcement counters, reconstruction counters, missing-transaction counters, fallback counters, in-flight compact download counters, and cleanup counters.
- **D-02:** All operator surfaces consume this shared contract. RPC `open_bitcoin_network_status`, CLI status JSON/human renderers, dashboard models, metrics projections, structured logs, and support bundles must not re-derive block/compact truth from local heuristics.
- **D-03:** Preserve `FieldAvailability` semantics from Phase 72 and Phase 110: unavailable activation when runtime has not projected evidence yet; zeroed aggregate counters remain available where safe; never fabricate peer-level or block-hash detail.
- **D-04:** In-flight compact download evidence exposes aggregate counts only (`in_flight_count`, `getblocktxn_in_flight_count`, `peers_with_in_flight_count`) — never block hashes, tx indexes, or peer ids in status/support surfaces.
- **D-05:** Add `block_relay` (or `block_serving`) to `OpenBitcoinNetworkStatusResponse` alongside existing `inbound`, `relay`, and `metrics`, serialized from the shared contract.
- **D-06:** Managed RPC context must project live runtime evidence from managed network state on each call; when runtime is unavailable, return the same default-unavailable contract used by CLI offline snapshots.
- **D-07:** Baseline-compatible RPC methods (`getnetworkinfo`, etc.) remain unchanged; Open Bitcoin-specific network status carries block/compact evidence.
- **D-08:** Human CLI status adds concise block-relay lines mirroring transaction relay evidence style: activation, eligibility/status counters, compact negotiation, reconstruction/fallback, in-flight, and cleanup summaries.
- **D-09:** Dashboard status model and render modules gain matching block-relay sections using the same shared contract fields as CLI JSON mode.
- **D-10:** No raw peer addresses, permission tokens, cookies, credentials, transaction hex, or compact block payloads in CLI or dashboard output.
- **D-11:** Add fixed-label counters/events for: full block served, block serving suppressed, compact announced, compact reconstructed, missing transaction requested, compact fallback, compact malformed, compact timeout, and compact cleanup — reusing existing label strings from Phases 110–115 where they already exist.
- **D-12:** Metrics remain bounded numeric samples; structured logs remain compact records with stable `cause`/`outcome`/`label` fields — no dynamic string labels or high-cardinality dimensions.
- **D-13:** Runtime adapters increment shared evidence counters when block/compact effects occur; metrics/log writers read projected status rather than parsing log text.
- **D-14:** Support bundle allowlist adds compact block-relay evidence summaries only through the shared status contract and existing redaction helpers — no raw `cmpctblock`, `blocktxn`, inventory lists, peer endpoints, or permission strings.
- **D-15:** Extend deterministic support redaction tests to cover new block-relay fields and reject dynamic labels, raw transaction lists, and peer-identifying material.
- **D-16:** Preserve Phase 59/72 allowlist posture: recursive redaction, schema-versioned support evidence, and summary-only live-smoke ingestion.
- **D-17:** Update operator/runtime docs with copy-pasteable repo-local Cargo and Bazel commands for inspecting block-serving and compact-relay status, metrics, logs, and support bundles — matching AGENTS.md UAT guidance.
- **D-18:** Public-network block-serving or compact-relay review remains opt-in UAT only; docs must not imply default CI or pre-commit requires public-network evidence.
- **D-19:** Add deterministic cross-surface checker(s) verifying RPC network status, CLI JSON snapshot, dashboard projection, metrics/log label registry, and support redaction agree on core block-relay fields or the same unavailable reasons.
- **D-20:** New or touched first-party Rust source/test files require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` updates unless explicit `none` is defensible.
- **D-21:** Verification remains `bash scripts/verify.sh` — deterministic, local, public-network-free.

### Claude's Discretion

The planner may choose exact type/field names, module split between `block_serving.rs` and a new `compact_relay_evidence.rs`, checker script names, and doc paths. Prefer extending existing relay-evidence patterns over inventing parallel renderer-local summaries.

### Deferred Ideas (OUT OF SCOPE)

Parity index closeout, Knots breadcrumb expansion, release-boundary checkers, README/release-note no-claim guardrails, and broad milestone UAT packaging remain Phase 117. Package relay, bloom/filter serving, public serving defaults, and production readiness claims remain out of scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| OBS-01 | RPC and shared network status report block-serving activation, serving eligibility, compact negotiation, reconstruction, fallback, and in-flight compact-block state truthfully. | Extend `OpenBitcoinNetworkStatusResponse` and `ManagedRpcContext` with `block_relay` field; add `ManagedPeerNetwork::block_relay_evidence_status()` mirroring `relay_evidence_status()`. [VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs; packages/open-bitcoin-node/src/network/relay_fanout.rs] |
| OBS-02 | CLI and dashboard surfaces render block-serving and compact-block relay state from the shared status contract without raw peer, permission, credential, or transaction payload leakage. | Add `block_relay` to status collection path and dedicated render modules under `operator/status/render/`, `operator/dashboard/model/`, mirroring Phase 105 relay modules. [VERIFIED: packages/open-bitcoin-cli/src/operator/status/render/relay.rs] |
| OBS-03 | Metrics and structured logs use fixed low-cardinality labels for served, suppressed, compact-announced, reconstructed, missing-requested, fallback, malformed, timeout, and cleanup outcomes. | Add `MetricKind` block/compact counter variants and `block_relay_log_record` helper mirroring `relay_metric_samples` / `relay_mempool_log_record`. Reuse Phase 110–115 `as_str()` labels. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-node/src/logging.rs] |
| OBS-04 | Support bundles sanitize block-serving and compact-relay evidence, including raw transaction lists, raw peer endpoints, permission strings, credentials, and dynamic labels. | Extend `support_status_for_bundle` and redaction tests with `redact_block_relay_evidence`; add support Markdown section mirroring `support/render/relay.rs`. [VERIFIED: packages/open-bitcoin-cli/src/operator/support/redaction.rs] |
| OBS-05 | Operator docs and UAT guidance provide copy-pasteable repo-local Cargo and Bazel commands for block-serving and compact-relay workflows. | Update `docs/operator/runtime-guide.md` and architecture docs with status/metrics/support commands; checker enforces command strings like Phase 105. [VERIFIED: scripts/check-phase105-operator-relay-evidence.ts] |
</phase_requirements>

## Summary

Phase 116 closes the observability gap between Phases 110–115 (pure policy, wire codecs, negotiation, reconstruction, download/fallback) and operator surfaces. The shared status contract `BlockServingEvidenceStatus` already exists with activation, eligibility, and status counter groups and unit tests, but it is **not referenced outside its own module** — no RPC field, no CLI/dashboard render, no metrics, no structured logs, no support redaction, and no managed-network projection. [VERIFIED: grep for `BlockServingEvidenceStatus` returns only `packages/open-bitcoin-node/src/status/block_serving.rs` and tests]

Transaction relay evidence from Phase 105 provides the exact wiring pattern: typed status in `open-bitcoin-node`, thin RPC projection via `ManagedRpcContext`, CLI collection from `openbitcoinnetworkstatus`, dedicated render modules, fixed `MetricKind` counters, sanitized structured logs, support redaction, and a deterministic Bun checker wired into `scripts/verify.sh`. [VERIFIED: 105-RESEARCH.md; packages/open-bitcoin-rpc/src/dispatch/node.rs; scripts/check-phase105-operator-relay-evidence.ts]

The cleanest plan introduces `BlockRelayEvidenceStatus` composing `BlockServingEvidenceStatus` plus compact-relay counter groups, adds `ManagedPeerNetwork::block_relay_evidence_status()` that aggregates runtime counters from block-serving and compact-relay effects, and threads that contract through every operator surface without duplicating label vocabulary. Runtime adapters must increment counters at existing effect sites (inventory serve, compact announce, reconstruction, getblocktxn schedule, fallback, cleanup) rather than re-parsing logs.

**Primary recommendation:** Mirror Phase 105 end-to-end — shared typed contract first, RPC projection second, CLI/dashboard/metrics/logs third, support redaction and checker fourth — reusing Phase 110–115 label strings and keeping baseline RPC methods unchanged.

## Current Gaps

| Surface | Phase 105 relay pattern (done) | Phase 116 block/compact (gap) |
| --- | --- | --- |
| Shared status | `RelayEvidenceStatus` in `status/relay_evidence.rs`, used by snapshot | `BlockServingEvidenceStatus` exists but not composed into operator snapshot or RPC [VERIFIED: packages/open-bitcoin-node/src/status/block_serving.rs; OpenBitcoinStatusSnapshot has no block_relay field] |
| RPC | `OpenBitcoinNetworkStatusResponse.relay` | No `block_relay` field; `open_bitcoin_network_status()` only projects inbound/relay/metrics [VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs; dispatch/node.rs] |
| Managed runtime | `ManagedPeerNetwork::relay_evidence_status()` | No `block_relay_evidence_status()`; block/compact effects occur in network adapters without aggregate counter projection [VERIFIED: packages/open-bitcoin-node/src/network/relay_fanout.rs vs absence in network.rs grep] |
| CLI human/JSON | `status/render/relay.rs` | Zero CLI references to block_serving/block_relay [VERIFIED: grep packages/open-bitcoin-cli] |
| Dashboard | `dashboard/model/relay.rs` | No block-relay rows |
| Metrics | `MetricKind::Relay*` + `relay_metric_samples` | No block/compact `MetricKind` variants [VERIFIED: packages/open-bitcoin-node/src/metrics.rs] |
| Structured logs | `relay_mempool_log_record` | No block-relay log source or record builder [VERIFIED: packages/open-bitcoin-node/src/logging.rs] |
| Support | `redact_relay_mempool_evidence` | No block-relay redaction or Markdown section |
| Checker | `check-phase105-operator-relay-evidence.ts` | No Phase 116 checker; verify.sh stops at Phase 105 for operator evidence [VERIFIED: scripts/verify.sh] |
| Docs | Runtime guide commands for relay status | Architecture docs describe `BlockServingEvidenceStatus` contract but surfaces not yet wired [VERIFIED: docs/architecture/status-snapshot.md; operator-observability.md] |

## Standard Stack

### Core

| Component | Version | Purpose | Why Standard |
| --- | --- | --- | --- |
| `open-bitcoin-node` status modules | repo-local | Shared evidence contracts | Existing functional-core boundary; Phase 105 and 110 established pattern [VERIFIED: packages/open-bitcoin-node/src/status/] |
| `serde` / `serde_json` | workspace-pinned | RPC and CLI JSON serialization | Already used by all status types [VERIFIED: packages/open-bitcoin-rpc/src/method/node.rs] |
| Bun | 1.3.9 (local) | Deterministic phase checker scripts | Repo canonical automation runtime [VERIFIED: AGENTS.md; `bun --version`] |
| Rust / Cargo | 1.94.1 (local) | Implementation and tests | Pinned by `rust-toolchain.toml` [VERIFIED: `cargo --version`] |

### Supporting

| Component | Purpose | When to Use |
| --- | --- | --- |
| Bazelisk / Bazel | Smoke build and UAT command forms | Doc examples and verify contract [VERIFIED: AGENTS.md] |
| `scripts/verify.sh` | Pre-commit and CI gate | Final closeout for all plans [VERIFIED: standards/core/verification.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
| --- | --- | --- |
| Extend `RelayEvidenceStatus` | Separate `BlockRelayEvidenceStatus` | Mixing transaction and block semantics violates Phase 110/116 boundary and confuses operators; CONTEXT locks separate contract [VERIFIED: 116-CONTEXT.md D-01] |
| Renderer-local summaries | Shared contract only | Causes schema drift and redaction gaps — Phase 105 explicitly rejected this [VERIFIED: 105-RESEARCH.md] |
| New telemetry crate | First-party metrics/logging | Conflicts with minimal-dependency and functional-core policy [VERIFIED: AGENTS.md] |

**No new dependencies recommended.**

## Label Inventory (Phases 110–115)

Reuse these fixed strings for counters, structured-log `label`/`cause` fields, and checker corpus validation. Do not introduce dynamic or peer-scoped labels.

### Phase 110 — Block serving activation, eligibility, status, resource, cleanup

**Eligibility (`BlockServingEligibilityReason::as_str`):** `eligible`, `disabled`, `activation_required`, `inbound_serving_required`, `permission_required`, `protected_not_serving`, `status_unavailable`, `permission_effect_inactive` [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs]

**Status (`BlockServingStatusLabel::as_str`):** `validated`, `available`, `stale`, `side_chain`, `pruned`, `unavailable`, `unvalidated`, `unknown`, `suppressed` [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs]

**Outcome / resource / cleanup (`BlockServingOutcomeLabel::as_str`):** `block_serving_disabled`, `block_serving_eligible`, `block_serving_suppressed`, `block_status_unavailable`, `block_status_pruned`, `block_status_unvalidated`, `block_request_cap_reached`, `block_inflight_cleanup_released`, `block_inflight_cleanup_peer_removed`, `block_inflight_cleanup_timeout`, `block_inflight_cleanup_restart`, `block_inflight_limit_still_reached` [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs]

**OBS-03 mapping (full block served / suppressed):** increment on `block_serving_eligible` serve success vs `block_serving_suppressed` and resource-gate suppressions.

### Phase 113 — Compact negotiation and announcement

**Announcement (`CompactAnnouncementReason::as_str`):** `compact_announced`, `compact_relay_disabled`, `compact_peer_not_negotiated`, `compact_unsupported_version`, `compact_high_bandwidth_not_requested`, `compact_header_continuity_missing`, `compact_peer_already_has_header`, `compact_block_unavailable`, `compact_resource_limited`, `compact_headers_fallback`, `compact_inventory_fallback` [VERIFIED: packages/open-bitcoin-network/src/peer/compact_relay.rs]

**Negotiation (`CompactRelayNegotiationReason`):** `Version2HighBandwidth`, `Version2LowBandwidth`, `UnsupportedVersion` — enum exists without `as_str()` today; planner should add snake_case projection (`version2_high_bandwidth`, etc.) or aggregate-only negotiation counters [VERIFIED: packages/open-bitcoin-network/src/peer/compact_relay.rs]

### Phase 114 — Reconstruction

**Invalid reasons (`CompactReconstructionInvalidReason`):** `NullHeader`, `EmptyCompactBlock`, `AlreadyInitialized`, `TransactionCountOutOfRange`, `NullPrefilledTransaction`, `PrefilledIndexOutOfBounds`, `MalformedPrefilledIndex`, `IncompleteTransactions` — no `as_str()` yet; map malformed bucket to fixed label `compact_malformed` or per-reason snake_case for aggregate counters only [VERIFIED: packages/open-bitcoin-network/src/compact_reconstruction.rs]

**Failure reasons:** `ShortIdCollision`, `ShortIdBucketOverload` — aggregate under reconstruction failure counters.

### Phase 115 — Download, fallback, cleanup

**Cleanup (`CompactDownloadCleanupCause::as_str`):** `compact_download_peer_disconnect`, `compact_download_timeout`, `compact_download_reorg`, `compact_download_restart`, `compact_download_block_connected` [VERIFIED: packages/open-bitcoin-network/src/compact_download.rs]

**Suppression / fallback (`CompactDownloadSuppressionReason::as_str`):** `compact_reconstruction_failed`, `compact_download_timeout`, `compact_peer_ineligible`, `compact_reconstruction_invalid`, `compact_block_already_in_flight` [VERIFIED: packages/open-bitcoin-network/src/compact_download.rs]

**Missing-tx schedule suppression (`ScheduleMissingTransactionSuppressionReason`):** variants exist without public `as_str()` — planner should add labels or fold into `missing_transaction_requested` vs suppressed counters [VERIFIED: packages/open-bitcoin-network/src/compact_download.rs]

**OBS-03 D-11 counter names (proposed fixed metric kinds):** `block_served_count`, `block_serving_suppressed_count`, `compact_announced_count`, `compact_reconstructed_count`, `compact_missing_tx_requested_count`, `compact_fallback_count`, `compact_malformed_count`, `compact_timeout_count`, `compact_cleanup_count` — names should end in `_count` like existing relay metrics [VERIFIED: packages/open-bitcoin-node/src/metrics.rs MetricKind naming]

## Architecture Patterns

### Recommended module layout

```
packages/open-bitcoin-node/src/status/
├── block_serving.rs              # existing BlockServingEvidenceStatus (extend or compose)
├── compact_relay_evidence.rs     # optional: compact counter groups (planner discretion)
└── block_relay_evidence.rs       # BlockRelayEvidenceStatus composing both

packages/open-bitcoin-node/src/network/
├── block_relay_evidence.rs       # runtime projection from ManagedPeerNetwork state
└── (existing block_serving.rs, inventory.rs, relay_serving.rs effect sites)

packages/open-bitcoin-cli/src/operator/
├── status/render/block_relay.rs
├── dashboard/model/block_relay.rs
└── support/render/block_relay.rs
```

### Pattern 1: Shared contract with FieldAvailability (Phase 110)

**What:** Activation uses `FieldAvailability<T>` with explicit unavailable reason; eligibility/status counters default to zeroed available aggregates.

**When:** Block-serving activation before first runtime projection; same semantics for compact-relay activation subgroup.

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/status/block_serving.rs
pub struct BlockServingEvidenceStatus {
    pub activation: FieldAvailability<BlockServingActivationEvidence>,
    pub eligibility: FieldAvailability<BlockServingEligibilityCounters>,
    pub status: FieldAvailability<BlockServingStatusCounters>,
}
```

### Pattern 2: RPC thin projection (Phase 105)

**What:** `open_bitcoin_network_status()` reads managed context accessors; no dispatch-layer label translation.

**When:** Every RPC call for live evidence.

**Example:**

```rust
// Source: packages/open-bitcoin-rpc/src/dispatch/node.rs (extend similarly)
OpenBitcoinNetworkStatusResponse {
    inbound: context.current_inbound_status(),
    relay: context.relay_evidence_status(),
    block_relay: context.block_relay_evidence_status(), // Phase 116 addition
    metrics: context.metrics_status(),
}
```

### Pattern 3: CLI collection fallback (Phase 105)

**What:** On RPC failure, offline snapshot uses same default-unavailable contract as runtime default.

**When:** Stopped daemon or method-not-found.

**Example:**

```rust
// Source: packages/open-bitcoin-cli/src/operator/status.rs (extend fallback)
OpenBitcoinNetworkStatusResponse {
    inbound: FieldAvailability::unavailable(...),
    relay: RelayEvidenceStatus::default(),
    block_relay: BlockRelayEvidenceStatus::default_unavailable(), // Phase 116
    metrics: metrics_status(),
}
```

### Pattern 4: Cross-surface checker (Phase 105)

**What:** Fixed-file Bun checker validates symbols, counter field names, behavior test names, redaction needles, runtime-guide commands, breadcrumb groups, and verify.sh ordering.

**When:** Closeout of each plan touching OBS requirements.

### Anti-Patterns to Avoid

- **Renderer-local block/compact summaries:** Duplicates Phase 110 contract and breaks support redaction parity.
- **Peer or block-hash fields in status JSON:** Violates D-04 and Phase 59/72 redaction posture.
- **Parsing structured logs to populate metrics:** Violates D-13; counters must be incremented at effect sites.
- **Extending `getnetworkinfo`:** Violates D-07; keep Open Bitcoin-specific evidence on `openbitcoinnetworkstatus`.

## Integration Points

| Layer | File | Action |
| --- | --- | --- |
| Status contract | `packages/open-bitcoin-node/src/status/block_serving.rs` | Extend or compose into `BlockRelayEvidenceStatus` with compact counter structs |
| Status re-export | `packages/open-bitcoin-node/src/status.rs` | Export new types; optionally add `block_relay` to `OpenBitcoinStatusSnapshot` or `PeerStatus` (planner choice — RPC uses network status response today for relay) |
| Runtime projection | `packages/open-bitcoin-node/src/network.rs` + new `block_relay_evidence.rs` | Add counter state on `ManagedPeerNetwork`, increment at inventory serve / compact announce / download / cleanup sites |
| RPC context | `packages/open-bitcoin-rpc/src/context/network.rs` | Add `block_relay_evidence_status()` accessor |
| RPC types | `packages/open-bitcoin-rpc/src/method/node.rs` | Add `block_relay: BlockRelayEvidenceStatus` to response |
| RPC dispatch | `packages/open-bitcoin-rpc/src/dispatch/node.rs` | Wire field in `open_bitcoin_network_status()` |
| CLI collect | `packages/open-bitcoin-cli/src/operator/status.rs` | Map RPC `block_relay` into snapshot |
| CLI render | `packages/open-bitcoin-cli/src/operator/status/render.rs` + `render/block_relay.rs` | Human lines mirroring relay layout |
| Dashboard | `packages/open-bitcoin-cli/src/operator/dashboard/model/block_relay.rs` | Rows from shared snapshot |
| Metrics | `packages/open-bitcoin-node/src/metrics.rs` | New `MetricKind` variants + `block_relay_metric_samples()` |
| Logs | `packages/open-bitcoin-node/src/logging.rs` | `BLOCK_RELAY_LOG_SOURCE` + `block_relay_log_record()` |
| Support | `packages/open-bitcoin-cli/src/operator/support/redaction.rs`, `render/block_relay.rs` | Redaction + Markdown section |
| Parity | `docs/parity/source-breadcrumbs.json`, catalog docs | Breadcrumbs for new/touched files |
| Verify | `scripts/verify.sh` | Wire Phase 116 checker after Phase 105 |

## Recommended Plan Shape

### 116-01: Shared block-relay status contract and RPC projection

- Define `BlockRelayEvidenceStatus` composing `BlockServingEvidenceStatus` + compact counter groups (negotiation, announcement, reconstruction, missing-tx, fallback, in-flight aggregates, cleanup).
- Add runtime counter storage and `ManagedPeerNetwork::block_relay_evidence_status()`.
- Extend RPC response, context accessor, dispatch, and dispatch tests.
- Keep `getnetworkinfo` unchanged.

### 116-02: CLI and dashboard rendering

- Extend status collection and JSON/human render modules.
- Add dashboard model rows from shared snapshot only.
- CLI/dashboard tests for unavailable/default-off activation and no sensitive needles.

### 116-03: Metrics and structured logs

- Add fixed `MetricKind` block/compact counters and projection helper.
- Add structured log record builder with fixed source name.
- Wire runtime increment hooks at effect sites (may span 116-01 if minimal hooks needed earlier).
- Metrics/logging tests for low cardinality and redaction.

### 116-04: Support redaction, checker, docs

- Extend support bundle redaction and tests (raw hex, endpoints, peer_id, dynamic labels).
- Add `scripts/check-phase116-operator-block-relay-evidence.ts` (+ test) mirroring Phase 105 fixed corpus.
- Update `docs/operator/runtime-guide.md`, architecture docs, parity index/checklist as needed.
- Wire checker into `scripts/verify.sh` after Phase 105.

## Checker Strategy

Mirror `scripts/check-phase105-operator-relay-evidence.ts` structure: [VERIFIED: scripts/check-phase105-operator-relay-evidence.ts]

| Checker section | Phase 116 content |
| --- | --- |
| `SURFACE_ID` | New parity surface e.g. `v2-1-operator-block-relay-evidence` (Phase 117 may finalize index — 116 checker can validate file/symbol corpus first) |
| `REQUIRED_SYMBOLS` | `BlockRelayEvidenceStatus`, `BlockServingEvidenceStatus`, `block_relay_evidence_status`, `MetricKind::BlockServedCount` (exact names at planner discretion), `BLOCK_RELAY_LOG_SOURCE`, `openbitcoinnetworkstatus`, `redact_block_relay_evidence` |
| `REQUIRED_FIXED_COUNTERS` | OBS-03 counter field names + Phase 110–115 label strings in contract/tests |
| `REQUIRED_FILE_NEEDLES` | Cross-surface wiring needles (RPC context, CLI render, dashboard model, support redaction, metrics projection) |
| `REQUIRED_BEHAVIOR_TESTS` | RPC dispatch, CLI render from network status, dashboard rows, metric low-cardinality, log sanitization, support redaction |
| `REQUIRED_REDACTION_NEEDLES` | Reuse Phase 105 sensitive probes + block-hash-like hex, cmpctblock payloads, peer endpoints |
| `REQUIRED_RUNTIME_COMMANDS` | Cargo/Bazel `status --format human/json`, `support bundle` with block-relay doc strings |
| `REQUIRED_BREADCRUMB_FILES_BY_GROUP` | node-status-contract, rpc-surface, cli-operator-*, node-observability-contracts |
| `checkVerifierOrder` | Phase 116 test/check commands after Phase 105, before pure-core checks |
| `FORBIDDEN_CLAIMS` | Public block serving by default, production readiness, release validator — same guardrails as Phase 105 |

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
| --- | --- | --- | --- |
| Block/compact label vocabulary | Ad-hoc strings in renderers | Phase 110–115 `as_str()` enums | Drift breaks checker and parity docs |
| Support sanitization | Denylist-only regex | Allowlisted fixed fields + existing redaction helpers | Phase 59/72 threat model |
| Metric cardinality | Dynamic peer/block labels | Fixed `MetricKind` enum | Dashboard retention and operator safety |
| Evidence aggregation | Log scraping | Runtime counter increments at effect sites | D-13 and test determinism |

## Common Pitfalls

### Pitfall 1: Schema drift across surfaces

**What goes wrong:** RPC exposes counters CLI does not collect; dashboard invents local summaries.

**Why it happens:** Parallel implementation without shared contract first.

**How to avoid:** Land 116-01 before renderers; checker file needles enforce cross-references.

**Warning signs:** Grep shows `block_serving_enabled` only in network policy, not in CLI tests.

### Pitfall 2: Sensitive material in aggregate counters

**What goes wrong:** In-flight evidence includes block hashes or peer ids.

**Why it happens:** Reusing internal `CompactDownloadInFlight` shapes directly.

**How to avoid:** D-04 aggregate-only fields; redaction tests with hash/endpoint needles.

### Pitfall 3: Activation shown as serving enabled

**What goes wrong:** Operators infer full block serving from activation flags alone.

**Why it happens:** Missing no-claim doc strings and suppressed/outcome counters.

**How to avoid:** Mirror Phase 105 deferred/unavailable semantics; docs and support next-action text.

### Pitfall 4: Counter increments without runtime projection

**What goes wrong:** Tests pass on synthetic status but live daemon always returns unavailable activation.

**Why it happens:** Status types added without `ManagedPeerNetwork` hookup.

**How to avoid:** Require managed-network integration tests like `relay_evidence_status_projects_*`.

## Code Examples

### Phase 105 relay metric projection (mirror for block relay)

```rust
// Source: packages/open-bitcoin-node/src/metrics.rs
pub fn relay_metric_samples(
    relay: &RelayEvidenceStatus,
    timestamp_unix_seconds: u64,
) -> Vec<MetricSample> { /* fixed MetricKind mapping */ }
```

### Phase 105 support redaction entry point

```rust
// Source: packages/open-bitcoin-cli/src/operator/support/redaction.rs
pub(crate) fn support_status_for_bundle(
    mut status: OpenBitcoinStatusSnapshot,
) -> OpenBitcoinStatusSnapshot {
    redact_relay_mempool_evidence(&mut status.mempool.relay);
    // Phase 116: redact_block_relay_evidence(&mut status....block_relay);
    status
}
```

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
| --- | --- | --- | --- | --- |
| Rust / Cargo | Build, test, UAT commands | ✓ | 1.94.1 | — |
| Bun | Phase checker scripts | ✓ | 1.3.9 | — |
| Bazelisk | UAT doc commands, verify smoke | ✓ | 8.6.0 | Cargo-only UAT text still required per AGENTS.md |
| Public Bitcoin network | OBS evidence | ✗ (intentional) | — | Deterministic local tests only; opt-in UAT per D-18 |

**Missing dependencies with no fallback:** None for deterministic implementation.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
| --- | --- | --- |
| V5 Input Validation | yes | Fixed-label allowlists; reject dynamic metric/log dimensions |
| V4 Access Control | partial | No permission strings or credentials in status/support |
| V6 Cryptography | no | No new crypto in observability phase |

### Known Threat Patterns

| Pattern | STRIDE | Standard Mitigation |
| --- | --- | --- |
| Peer endpoint leakage in status | Information disclosure | Aggregate counts only; redaction tests |
| Transaction payload in support bundles | Information disclosure | Allowlisted summary fields; no raw cmpctblock/blocktxn |
| Dynamic metric labels | Tampering / DoS (cardinality) | Fixed `MetricKind` enum |
| False production-readiness claims | Spoofing | Forbidden-claim checker patterns from Phase 105 |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
| --- | --- | --- | --- |
| A1 | `BlockRelayEvidenceStatus` is the preferred top-level name (per D-01) over nesting only under `PeerStatus` | Architecture | Low — planner may adjust field placement if snapshot shape fits better |
| A2 | Phase 116 checker can validate file/symbol corpus before parity index surface is marked `done` in Phase 117 | Checker Strategy | Medium — may need interim checker without full `index.json` surface until 117 |
| A3 | `CompactRelayNegotiationReason` and reconstruction invalid enums need new `as_str()` helpers for log labels | Label Inventory | Low — aggregate-only counters remain valid without per-reason strings |

## Open Questions

1. **Snapshot placement:** Should `block_relay` live on `OpenBitcoinStatusSnapshot` top-level, under `peers`, or only flow through `OpenBitcoinNetworkStatusResponse` into CLI merge logic?
   - What we know: Phase 105 puts relay under `MempoolStatus.relay`; block serving is network-scoped not mempool-scoped.
   - Recommendation: Top-level `block_relay` on snapshot **or** dedicated `NetworkEvidenceStatus` — avoid overloading `mempool`.

2. **Parity surface timing:** Register `v2-1-operator-block-relay-evidence` in Phase 116 or defer index closeout to 117?
   - Recommendation: Checker + docs in 116; index `done` status may wait for 117 per deferred scope.

## Dependency Guidance

No new crates. Extend existing first-party modules and Bun checkers only. [VERIFIED: 105-RESEARCH.md; AGENTS.md dependency policy]

## Sources

### Primary (HIGH confidence)

- `116-CONTEXT.md` — locked decisions D-01 through D-21
- `105-RESEARCH.md` — relay evidence wiring pattern
- `packages/open-bitcoin-node/src/status/block_serving.rs` — existing contract
- `packages/open-bitcoin-node/src/status/relay_evidence.rs` — mirror pattern
- `packages/open-bitcoin-rpc/src/method/node.rs` — RPC response shape
- `packages/open-bitcoin-network/src/block_serving.rs`, `peer/compact_relay.rs`, `compact_download.rs` — label inventories
- `scripts/check-phase105-operator-relay-evidence.ts` — checker template

### Secondary (MEDIUM confidence)

- `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md` — Phase 110 contract documentation (surfaces not yet wired)

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — same repo patterns as Phase 105, no new dependencies
- Architecture: HIGH — gaps verified by codebase grep and file reads
- Pitfalls: HIGH — explicit Phase 110/105 precedents and CONTEXT constraints

**Research date:** 2026-07-06
**Valid until:** 2026-08-06 (stable observability patterns)
