---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 116-2026-07-06T03-46-36
generated_at: 2026-07-06T03:46:36.364Z
---

# Phase 116: Operator Evidence, Metrics, Logs, and Support Boundary - Context

**Gathered:** 2026-07-06
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 116 projects block-serving and compact-relay truth from Phases 110–115 through shared operator surfaces: RPC network status, CLI status, dashboard, bounded metrics, structured logs, and redacted support bundles. Operators must see activation, eligibility, compact negotiation, reconstruction, fallback, in-flight compact download, and cleanup outcomes as fixed low-cardinality labels without raw peer endpoints, permission strings, credentials, transaction payloads, or dynamic labels.

This phase consumes pure policy outcomes and runtime counters already produced by block serving, compact negotiation, reconstruction, missing-transaction download, and cleanup modules. It must not add new P2P behavior, change activation defaults, add parity/UAT release closeout (Phase 117), package relay, bloom/filter serving, public defaults, or production readiness claims.
</domain>

<decisions>
## Implementation Decisions

### Shared Status Contract

- **D-01:** Extend the shared status layer in `open-bitcoin-node` with a `BlockRelayEvidenceStatus` (or equivalent) that composes existing `BlockServingEvidenceStatus` plus compact-relay evidence fields: negotiation counters, announcement counters, reconstruction counters, missing-transaction counters, fallback counters, in-flight compact download counters, and cleanup counters.
- **D-02:** All operator surfaces consume this shared contract. RPC `open_bitcoin_network_status`, CLI status JSON/human renderers, dashboard models, metrics projections, structured logs, and support bundles must not re-derive block/compact truth from local heuristics.
- **D-03:** Preserve `FieldAvailability` semantics from Phase 72 and Phase 110: unavailable activation when runtime has not projected evidence yet; zeroed aggregate counters remain available where safe; never fabricate peer-level or block-hash detail.
- **D-04:** In-flight compact download evidence exposes aggregate counts only (`in_flight_count`, `getblocktxn_in_flight_count`, `peers_with_in_flight_count`) — never block hashes, tx indexes, or peer ids in status/support surfaces.

### RPC Projection (OBS-01)

- **D-05:** Add `block_relay` (or `block_serving`) to `OpenBitcoinNetworkStatusResponse` alongside existing `inbound`, `relay`, and `metrics`, serialized from the shared contract.
- **D-06:** Managed RPC context must project live runtime evidence from managed network state on each call; when runtime is unavailable, return the same default-unavailable contract used by CLI offline snapshots.
- **D-07:** Baseline-compatible RPC methods (`getnetworkinfo`, etc.) remain unchanged; Open Bitcoin-specific network status carries block/compact evidence.

### CLI And Dashboard (OBS-02)

- **D-08:** Human CLI status adds concise block-relay lines mirroring transaction relay evidence style: activation, eligibility/status counters, compact negotiation, reconstruction/fallback, in-flight, and cleanup summaries.
- **D-09:** Dashboard status model and render modules gain matching block-relay sections using the same shared contract fields as CLI JSON mode.
- **D-10:** No raw peer addresses, permission tokens, cookies, credentials, transaction hex, or compact block payloads in CLI or dashboard output.

### Metrics And Structured Logs (OBS-03)

- **D-11:** Add fixed-label counters/events for: full block served, block serving suppressed, compact announced, compact reconstructed, missing transaction requested, compact fallback, compact malformed, compact timeout, and compact cleanup — reusing existing label strings from Phases 110–115 where they already exist.
- **D-12:** Metrics remain bounded numeric samples; structured logs remain compact records with stable `cause`/`outcome`/`label` fields — no dynamic string labels or high-cardinality dimensions.
- **D-13:** Runtime adapters increment shared evidence counters when block/compact effects occur; metrics/log writers read projected status rather than parsing log text.

### Support Bundles And Redaction (OBS-04)

- **D-14:** Support bundle allowlist adds compact block-relay evidence summaries only through the shared status contract and existing redaction helpers — no raw `cmpctblock`, `blocktxn`, inventory lists, peer endpoints, or permission strings.
- **D-15:** Extend deterministic support redaction tests to cover new block-relay fields and reject dynamic labels, raw transaction lists, and peer-identifying material.
- **D-16:** Preserve Phase 59/72 allowlist posture: recursive redaction, schema-versioned support evidence, and summary-only live-smoke ingestion.

### Operator Docs And UAT (OBS-05)

- **D-17:** Update operator/runtime docs with copy-pasteable repo-local Cargo and Bazel commands for inspecting block-serving and compact-relay status, metrics, logs, and support bundles — matching AGENTS.md UAT guidance.
- **D-18:** Public-network block-serving or compact-relay review remains opt-in UAT only; docs must not imply default CI or pre-commit requires public-network evidence.

### Verification And Parity

- **D-19:** Add deterministic cross-surface checker(s) verifying RPC network status, CLI JSON snapshot, dashboard projection, metrics/log label registry, and support redaction agree on core block-relay fields or the same unavailable reasons.
- **D-20:** New or touched first-party Rust source/test files require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` updates unless explicit `none` is defensible.
- **D-21:** Verification remains `bash scripts/verify.sh` — deterministic, local, public-network-free.

### Claude's Discretion

The planner may choose exact type/field names, module split between `block_serving.rs` and a new `compact_relay_evidence.rs`, checker script names, and doc paths. Prefer extending existing relay-evidence patterns over inventing parallel renderer-local summaries.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope

- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/languages/rust.md`
- `.planning/PROJECT.md`, `.planning/REQUIREMENTS.md` (OBS-01 through OBS-05), `.planning/ROADMAP.md`, `.planning/STATE.md`

### Prior Locked Decisions

- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-CONTEXT.md`
- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md`
- `.planning/phases/114-compact-block-reconstruction-from-mempool-state/114-CONTEXT.md`
- `.planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff/115-CONTEXT.md`
- `.planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md`
- `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md`

### Existing Code Integration Points

- `packages/open-bitcoin-node/src/status/block_serving.rs` — existing activation/eligibility/status counters
- `packages/open-bitcoin-node/src/status/relay_evidence.rs` — relay evidence pattern to mirror
- `packages/open-bitcoin-rpc/src/method/node.rs` — `OpenBitcoinNetworkStatusResponse`
- `packages/open-bitcoin-cli/src/operator/status/` — CLI collection and render modules
- `packages/open-bitcoin-cli/src/operator/dashboard/` — dashboard model/render
- `packages/open-bitcoin-cli/src/operator/support/` — support bundle redaction
- `packages/open-bitcoin-network/src/block_serving.rs`, `compact_download.rs`, `peer/compact_relay.rs` — source label enums
- `docs/architecture/operator-observability.md`, `docs/operator/runtime-guide.md`
- `docs/parity/source-breadcrumbs.json`, `scripts/verify.sh`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `BlockServingEvidenceStatus` with activation, eligibility, and status counter fields already defined and tested.
- `RelayEvidenceStatus` pattern for RPC, CLI human lines, dashboard, metrics, and support projection.
- Compact relay/download modules already expose stable `as_str()` labels for negotiation, announcement, reconstruction, fallback, suppression, and cleanup causes.
- Phase 72 cross-surface comparison checker pattern and support redaction test harness.

### Established Patterns

- Shared status types in `open-bitcoin-node`, thin RPC/CLI/dashboard adapters, deterministic checker scripts under `scripts/`.
- Low-cardinality counter structs with serde defaults for backward-compatible JSON.

### Integration Points

- Managed network runtime must aggregate peer-level block/compact outcomes into runtime evidence projection consumed by RPC context.
- CLI `collect_open_bitcoin_network_status` and dashboard model builders need new block-relay fields from RPC or offline defaults.

</code_context>

<specifics>
## Specific Ideas

- Mirror transaction relay evidence layout: separate counter groups for negotiation, outcomes, recovery/cleanup, and in-flight state.
- Reuse Phase 115 cleanup cause strings (`compact_download_timeout`, `compact_download_peer_disconnect`, etc.) as metrics/log labels.
- Keep activation unavailable until managed network has projected at least one evidence snapshot.

</specifics>

<deferred>
## Deferred Ideas

Parity index closeout, Knots breadcrumb expansion, release-boundary checkers, README/release-note no-claim guardrails, and broad milestone UAT packaging remain Phase 117. Package relay, bloom/filter serving, public serving defaults, and production readiness claims remain out of scope.

</deferred>

---

*Phase: 116-operator-evidence-metrics-logs-and-support-boundary*
*Context gathered: 2026-07-06*
