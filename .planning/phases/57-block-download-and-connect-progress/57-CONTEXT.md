---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 57-2026-06-03T13-56-54
generated_at: 2026-06-03T13:56:54.625Z
---

# Phase 57: Block Download and Connect Progress - Context

**Gathered:** 2026-06-03
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 57 proves bounded daemon block download and the first validated block
connection for the scoped v1.4 IBD claim. It builds directly on Phase 56 header
convergence: accepted headers select candidate blocks, daemon sync requests and
tracks those block bodies within resource caps, accepted blocks update durable
download/connect status, and opt-in live-smoke evidence records first block
progress or a typed diagnosis.

This phase does not claim same-datadir restart/resume evidence, support bundle
closeout, release-readiness claims, inbound serving, relay behavior, unattended
production-node operation, or default public-network verification.

</domain>

<decisions>

## Implementation Decisions

### Block Download Runtime Contract

- **D-01:** Request blocks only for validated best-chain headers that are not
  already known locally. Do not request speculative side-chain, malformed, or
  unvalidated inventory as part of the v1.4 block-progress proof.
- **D-02:** Preserve the documented runtime resource caps:
  `sync.max_blocks_in_flight_per_peer` and
  `sync.max_blocks_in_flight_total`. Per-peer and total in-flight tracking must
  prevent duplicate requests and release entries on `block`, `notfound`,
  disconnect, invalid data, or end-of-peer-session cleanup.
- **D-03:** Keep block download scheduling additive to the existing header-first
  sync loop. Header acceptance can enqueue missing blocks; block download or
  failure must not weaken the Phase 54-56 handshake, compatibility, and header
  validation safeguards.

### Block Connect Evidence

- **D-04:** Treat the first connected non-genesis block, or configured
  checkpoint-adjacent block when that becomes the bounded target, as the
  Phase 57 success signal. A stored but disconnected block is useful durable
  download evidence, but it is not connected-chain progress.
- **D-05:** Persist and surface downloaded block height separately from
  connected block height. `downloaded_block_height` means a contiguous
  best-chain block body is available; `connected_block_height` means active
  chainstate advanced.
- **D-06:** Live-smoke `result.firstBlockProgress` should mirror Phase 56
  `firstHeaderProgress`: before/after fresh `openbitcoinsyncstatus` snapshots,
  observed timestamp, peer endpoint/source when available, block hash, height,
  and whether the evidence was download-only or connected-chain progress.

### Failure Attribution and No-Credit Paths

- **D-07:** Missing, `notfound`, malformed, invalid, duplicate, disconnected,
  or non-extending block responses remain peer-attributed through typed
  outcomes and health signals. They must not advance active chainstate or create
  duplicate connect work.
- **D-08:** If no block progress is reached, live-smoke evidence should produce
  a typed diagnosis and next operator action that distinguishes awaiting blocks,
  peer notfound/missing data, invalid block data, disconnected or duplicate
  blocks, network/compatibility failure, and local resource limits.
- **D-09:** Deterministic tests own the default verification surface. Public
  mainnet smoke remains opt-in UAT evidence and must not be added to
  `bash scripts/verify.sh`.

### Scope Controls

- **D-10:** Do not broaden this phase into restart/resume proof. Phase 58 owns
  same-datadir interruption and resume evidence after observed progress.
- **D-11:** Do not broaden this phase into support bundles, threat-model
  closeout, or release-boundary copy. Phase 59 owns those operator evidence and
  release claim surfaces.

### the agent's Discretion

- The planner may choose the smallest robust internal representation for block
  progress and first-block evidence as long as the externally observable fields
  stay additive, typed, and truth-aligned.
- The planner may split deterministic tests across sync runtime, managed
  network, RPC/config, and live-smoke script tests according to existing module
  boundaries.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 57 goal and success criteria.
- `.planning/REQUIREMENTS.md` - BLK-01 through BLK-04.
- `.planning/PROJECT.md` - v1.4 IBD convergence boundary and production-claim
  exclusions.
- `.planning/STATE.md` - Phase 56 completion context and Phase 57 readiness.

### Prior Phase Evidence

- `.planning/phases/56-header-ibd-convergence/56-CONTEXT.md` - Header progress
  decisions that Phase 57 builds on.
- `.planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md` - Completed
  header convergence, stop-reason, and first-header-progress behavior.
- `.planning/phases/55-outbound-handshake-compatibility-fixes/55-01-SUMMARY.md`
  - Connected outbound handshakes and typed compatibility failure behavior.

### Implementation Surfaces

- `packages/open-bitcoin-node/src/sync.rs` - Durable sync loop, peer sessions,
  block persistence, and status updates.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs` - Block scheduling,
  in-flight limit enforcement, and best-chain reconciliation.
- `packages/open-bitcoin-node/src/sync/progress.rs` - Progress markers and
  peer failure/status mapping.
- `packages/open-bitcoin-node/src/sync/types.rs` - Runtime config, peer
  outcomes, errors, and block in-flight caps.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Summary projection,
  health signals, logs, metrics, and stop-reason output.
- `packages/open-bitcoin-node/src/sync/types/projection.rs` - Durable status
  projection for downloaded and connected block heights.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Deterministic sync tests and
  scripted peer transport.
- `packages/open-bitcoin-node/src/network.rs` - Managed network block
  request/connect primitives.
- `packages/open-bitcoin-node/src/network/tests.rs` - Managed network block
  request, notfound, duplicate, and chain-connect tests.
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` - JSONC runtime sync
  config fields.
- `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` -
  Mapping JSONC sync fields into `SyncRuntimeConfig`.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in live smoke command, status
  polling, report schema, and Markdown rendering.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke script
  report checks.
- `docs/operator/runtime-guide.md` - Operator-facing runtime, config, and
  live-smoke guidance.
- `docs/parity/catalog/p2p.md` - P2P parity catalog and block download status.

### Baseline Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` - Header-first block
  download, `getdata`, `notfound`, and block response attribution anchor.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` - Block storage and
  connection behavior anchor.
- `packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py` -
  Initial header sync behavior that precedes block download.
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py` - Inventory and
  request/response attribution patterns relevant to missing data behavior.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `DurableSyncRuntime::sync_connected_peer` already requests missing blocks,
  saves received block bodies, notes local block hashes, reconciles best-chain
  blocks, persists progress, and clears outstanding in-flight requests when a
  peer disconnects.
- `SyncRuntimeConfig` already has
  `max_blocks_in_flight_per_peer` and `max_blocks_in_flight_total` defaults,
  and operator docs already describe both config fields.
- `ManagedPeerNetwork` already exposes `request_missing_blocks`,
  `peer_requested_blocks`, `connect_stored_block`, and local block hash
  tracking.
- `ScriptedTransport`, `headers_script`, and deterministic sync tests can model
  header acceptance followed by `getdata`, `block`, `notfound`, malformed, and
  invalid block responses without public network access.
- The live smoke runner already computes `blockDelta`, polls fresh daemon
  status snapshots, derives typed no-progress causes, and renders peer
  contribution tables.

### Established Patterns

- Pure validation and chainstate decisions remain in core/network modules;
  daemon sync orchestration, durable persistence, logs, and live-smoke report
  projection live in shell layers.
- Operator-facing report/schema changes should be additive and tolerate older
  reports or unavailable final peer telemetry.
- Accepted headers and connected blocks are credited as useful progress; raw
  activity, bad data, and failed validation are visible but uncredited.
- Tests use Arrange, Act, Assert comments for non-trivial unit tests.

### Integration Points

- Tighten `block_reconcile::request_missing_blocks` and
  `release_inflight_for_message` around peer-attributed no-credit responses and
  total/per-peer cap behavior.
- Update sync summary/projection so downloaded and connected block heights are
  refreshed from durable block availability and chainstate connection state.
- Add first-block-progress report derivation beside the existing
  first-header-progress derivation in `scripts/run-live-mainnet-smoke.ts`.
- Update docs and parity catalog to describe Phase 57's bounded block progress
  claim without implying broader production-node operation.

</code_context>

<specifics>

## Specific Ideas

- Add a deterministic sync test where accepted headers trigger bounded
  `getdata`, a scripted block response is saved, and `connected_block_height`
  advances to one.
- Add tests for `notfound`, malformed block payload, invalid block data,
  duplicate block response, and disconnected non-extending block response that
  prove peer attribution plus no active-chain advancement.
- Add live-smoke `result.firstBlockProgress` with
  `kind: "downloaded" | "connected"`, `height`, `blockHash`,
  `observedAtUnixSeconds`, before/after status snapshots, and optional peer
  endpoint/source.
- Extend deterministic smoke-script checks to cover first-block-progress
  rendering and no-progress diagnosis.

</specifics>

<deferred>

## Deferred Ideas

- Same-datadir restart/resume proof remains Phase 58.
- Support bundle, threat-model update, release-boundary copy, and final operator
  evidence closeout remain Phase 59.
- Inbound serving, address relay, compact block relay, transaction relay,
  production-funds wallet use, migration apply mode, packaging, hosted
  dashboard, GUI work, and unattended production-node claims remain out of
  scope for v1.4.

</deferred>

---

*Phase: 57-block-download-and-connect-progress*
*Context gathered: 2026-06-03*
