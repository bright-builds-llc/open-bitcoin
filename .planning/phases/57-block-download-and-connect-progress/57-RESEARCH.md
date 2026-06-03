# Phase 57: Block Download and Connect Progress - Research

**Researched:** 2026-06-03
**Domain:** Rust Bitcoin P2P sync runtime, block download scheduling, durable progress evidence
**Confidence:** HIGH

<user_constraints>

## User Constraints (from CONTEXT.md)

Copied verbatim from `.planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md`. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)

- Same-datadir restart/resume proof remains Phase 58.
- Support bundle, threat-model update, release-boundary copy, and final operator
  evidence closeout remain Phase 59.
- Inbound serving, address relay, compact block relay, transaction relay,
  production-funds wallet use, migration apply mode, packaging, hosted
  dashboard, GUI work, and unattended production-node claims remain out of
  scope for v1.4.

</user_constraints>

<phase_requirements>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| BLK-01 | Daemon sync requests, tracks, and bounds in-flight block downloads for selected validated headers without exceeding documented v1.4 resource limits. [VERIFIED: .planning/REQUIREMENTS.md] | Existing `SyncRuntimeConfig` caps, `DurableSyncRuntime::inflight_blocks`, `block_reconcile::request_missing_blocks`, and `PeerManager::request_missing_blocks` are the implementation surface; deterministic tests should prove total and per-peer caps plus release on `block`, `notfound`, invalid data, and session cleanup. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-network/src/peer.rs] |
| BLK-02 | Daemon sync validates and connects the first non-genesis block or configured checkpoint-adjacent block in the opt-in live-smoke path when reachable peers provide the required data. [VERIFIED: .planning/REQUIREMENTS.md] | Use the existing first-party `ManagedPeerNetwork::connect_stored_block`, `block_reconcile::reconcile_best_chain`, and durable `connected_block_height` projection; no configured checkpoint-adjacent target exists in the current sync config, so the practical Phase 57 target is first non-genesis unless planning adds a scoped target field. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs; rg "target_block|checkpoint" packages/open-bitcoin-*] |
| BLK-03 | Live-smoke evidence records the first validated block connection with peer endpoint, block hash, height, timestamp, and before/after durable status, or records a typed diagnosis when block progress is not reached. [VERIFIED: .planning/REQUIREMENTS.md] | Extend `scripts/run-live-mainnet-smoke.ts` beside `firstHeaderProgress`; current snapshots parse `header_height` and `block_height` only, so Phase 57 should add downloaded/connected heights and a `firstBlockProgress` evidence shape. [VERIFIED: scripts/run-live-mainnet-smoke.ts; packages/open-bitcoin-node/src/status.rs] |
| BLK-04 | Missing, `notfound`, malformed, invalid, duplicate, or disconnected block responses are peer-attributed and do not advance active chainstate or create duplicate connect work. [VERIFIED: .planning/REQUIREMENTS.md] | Current network code clears requested inventory on `notfound` and block receipt, and sync maps invalid chainstate data to `PeerFailureReason::InvalidData`; Phase 57 should add explicit block-response disposition tests for no-credit paths because duplicate/non-extending blocks can currently return `Ok(None)` from `connect_stored_block`. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/network.rs] |

</phase_requirements>

## Summary

Phase 57 should stay on the existing first-party Rust sync stack and add no production dependency. [VERIFIED: AGENTS.md; .planning/STACK.md; cargo metadata --manifest-path packages/Cargo.toml --format-version 1 --no-deps] The current code already has most of the block-download skeleton: validated best-chain headers can be converted into `getdata`, per-peer and total in-flight state exists, received blocks are routed through managed network validation before durable save, and durable status already separates downloaded and connected heights. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs]

The plan should focus on tightening truth semantics, not inventing a new downloader. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md; packages/open-bitcoin-node/src/sync.rs] The highest-risk gap is distinguishing block dispositions: downloaded contiguous body, connected active-chain block, duplicate/disconnected/non-extending response, `notfound`, invalid body, and resource-limit stall need different peer outcomes and live-smoke diagnoses. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/sync/types.rs; scripts/run-live-mainnet-smoke.ts]

**Primary recommendation:** Add a small typed block response/progress disposition in the sync shell, keep validation/connect decisions in existing first-party core/network/chainstate paths, then mirror Phase 56's `firstHeaderProgress` live-smoke pattern with `firstBlockProgress`. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs; scripts/run-live-mainnet-smoke.ts; .planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md]

## Project Constraints (from AGENTS.md)

- Follow Bitcoin Knots `29.3.knots20260210` for in-scope externally observable behavior and keep parity evidence auditable. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Preserve functional-core / imperative-shell boundaries: pure consensus, chainstate, network, and wallet logic stays in first-party core crates; storage, sockets, logs, metrics, daemon orchestration, and live-smoke reporting stay in shell layers. [VERIFIED: AGENTS.md; .planning/ARCHITECTURE.md; Bright Builds architecture standard at https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]
- Do not use existing Rust Bitcoin libraries in the production path; the project owns first-party Bitcoin primitives, codec, consensus, chainstate, network, and node implementations. [VERIFIED: AGENTS.md; .planning/PROJECT.md; cargo metadata --manifest-path packages/Cargo.toml --format-version 1 --no-deps]
- Use `bash scripts/verify.sh` as the repo-native verification contract; public-network checks remain opt-in and must not be added to default verification. [VERIFIED: AGENTS.md; .planning/REQUIREMENTS.md; docs/operator/runtime-guide.md]
- Use Bun for repo-owned TypeScript automation and keep Bash as thin orchestration. [VERIFIED: AGENTS.md; .planning/STACK.md; Bright Builds TypeScript standard at https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/typescript-javascript.md]
- When adding or touching Rust files under `packages/open-bitcoin-*/src` or tests, keep parity breadcrumb mappings fresh through `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json; scripts/check-parity-breadcrumbs.ts]
- Tests should cover one concern and use clear Arrange, Act, Assert sections unless the test is trivial. [VERIFIED: AGENTS.md; Bright Builds testing standard at https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md]
- Rust style should prefer `foo.rs` plus `foo/`, early-return/`let...else` control flow, `maybe_` names for optional internals, typed invariants, `?` propagation, and no `unwrap()` in production code. [VERIFIED: AGENTS.md; Bright Builds Rust standard at https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md]
- No project-local skills exist under `.claude/skills/` or `.agents/skills/`. [VERIFIED: AGENTS.md; ls -d .claude/skills .agents/skills]

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|-------------------|---------|---------|--------------|
| Rust toolchain | `rustc 1.94.1`, Cargo `1.94.1` | First-party implementation and tests for node, network, consensus, RPC, and CLI crates. | Pinned by repo toolchain and verified locally. [VERIFIED: rustc --version; cargo --version; AGENTS.md] |
| `open-bitcoin-node` | `0.1.0` | Durable sync runtime, storage/status/metrics/log shell, block reconciliation, and peer outcomes. | Phase 57 implementation surface already owns `DurableSyncRuntime`, `block_reconcile`, and durable status projection. [VERIFIED: cargo metadata; packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs] |
| `open-bitcoin-network` | `0.1.0` | First-party peer state, P2P message handling, header store, and block/request tracking. | Existing `PeerManager` tracks per-peer requested blocks and clears inventory on `notfound`/block receipt. [VERIFIED: cargo metadata; packages/open-bitcoin-network/src/peer.rs] |
| `open-bitcoin-chainstate` / `open-bitcoin-consensus` | `0.1.0` | Block validation, active-chain connection, PoW/header/block checks. | Production path must use first-party Bitcoin domain logic, not third-party Rust Bitcoin crates. [VERIFIED: cargo metadata; AGENTS.md] |
| Bitcoin Knots baseline | `29.3.knots20260210` at submodule commit `a9aee730466ac67d35a3c03ee24676be5e045878` | External behavior anchor for P2P block download and connection semantics. | Repo requires Knots parity evidence for in-scope behavior. [VERIFIED: git submodule status packages/bitcoin-knots; AGENTS.md] |

### Supporting

| Library / Surface | Version | Purpose | When to Use |
|-------------------|---------|---------|-------------|
| Bun | `1.3.9` | Run repo-owned TypeScript scripts, including live-smoke report tests. | Use for `scripts/run-live-mainnet-smoke.ts` and `scripts/test-run-live-mainnet-smoke.sh`; no `bun install` step exists. [VERIFIED: bun --version; .planning/STACK.md] |
| Bazel / Bazelisk | `8.6.0` | Top-level smoke build surface through Bzlmod/rules_rust. | Use through repo verification and operator-facing UAT command examples when docs need Cargo and Bazel forms. [VERIFIED: bazel --version; bazelisk --version; AGENTS.md] |
| `scripts/run-live-mainnet-smoke.ts` | repo-owned | Opt-in public-mainnet evidence runner. | Extend report schema, Markdown rendering, and deterministic script fixtures for `firstBlockProgress`. [VERIFIED: scripts/run-live-mainnet-smoke.ts; scripts/test-run-live-mainnet-smoke.sh] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| First-party block validation and P2P state | Third-party Rust Bitcoin libraries | Not allowed for production path by repo policy; would weaken auditable ownership of domain invariants. [VERIFIED: AGENTS.md; .planning/PROJECT.md] |
| Existing `DurableSyncRuntime`/`PeerManager` | A separate block downloader service | Would duplicate in-flight tracking, peer attribution, status projection, and parity breadcrumbs already present in the sync runtime. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-network/src/peer.rs; docs/parity/source-breadcrumbs.json] |
| Existing live-smoke runner | New public-network test in `scripts/verify.sh` | Violates the v1.4 requirement that public-network checks remain opt-in and outside default deterministic verification. [VERIFIED: .planning/REQUIREMENTS.md; docs/operator/runtime-guide.md] |

**Installation:** No new packages should be installed for Phase 57. [VERIFIED: .planning/STACK.md; .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]

```bash
# No npm/pnpm/bun/cargo dependency additions are recommended.
```

**Version verification:** Package and tool versions were verified with `rustc --version`, `cargo --version`, `bun --version`, `bazel --version`, `bazelisk --version`, `git submodule status packages/bitcoin-knots`, and `cargo metadata --manifest-path packages/Cargo.toml --format-version 1 --no-deps`. [VERIFIED: local command outputs]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-node/src/sync.rs                  # daemon sync shell and peer session orchestration [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
packages/open-bitcoin-node/src/sync/block_reconcile.rs  # best-chain block scheduling/connect reconciliation [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs]
packages/open-bitcoin-node/src/sync/runtime_state.rs    # durable progress/status/resource projection [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs]
packages/open-bitcoin-node/src/sync/types.rs            # typed outcomes, errors, config, summary [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs]
packages/open-bitcoin-node/src/network.rs               # managed adapter over first-party network/chainstate [VERIFIED: packages/open-bitcoin-node/src/network.rs]
packages/open-bitcoin-network/src/peer.rs               # pure-ish peer state and requested inventory tracking [VERIFIED: packages/open-bitcoin-network/src/peer.rs]
scripts/run-live-mainnet-smoke.ts                       # opt-in live evidence schema and report rendering [VERIFIED: scripts/run-live-mainnet-smoke.ts]
scripts/test-run-live-mainnet-smoke.sh                  # deterministic live-smoke report fixture checks [VERIFIED: scripts/test-run-live-mainnet-smoke.sh]
docs/operator/runtime-guide.md                          # operator guidance for opt-in UAT and status meaning [VERIFIED: docs/operator/runtime-guide.md]
docs/parity/catalog/p2p.md                              # P2P parity claim update surface [VERIFIED: docs/parity/catalog/p2p.md]
```

### Pattern 1: Header-First Best-Chain Block Scheduling

**What:** Schedule block bodies only from validated best-chain header entries, skipping active-chain blocks, globally in-flight blocks, and already durable block bodies. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs]

**When to use:** Use this for BLK-01 and BLK-04; it matches the Phase 57 decision to avoid speculative side-chain/malformed/unvalidated inventory requests. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/sync/block_reconcile.rs
// Planning pattern: derive request candidates from best_chain_entries(),
// filter active/local/in-flight hashes, then let PeerManager enforce
// per-peer capacity before emitting getdata.
```

**Parity anchor:** Knots uses `MAX_BLOCKS_IN_TRANSIT_PER_PEER = 16`, tracks `mapBlocksInFlight`, chooses blocks through `FindNextBlocksToDownload`, and sends `GETDATA` for selected block indexes. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp] Bitcoin P2P `getdata` responses can be `block` or `notfound`, and `notfound` means the requested object is unavailable for relay. [CITED: https://developer.bitcoin.org/reference/p2p_networking.html]

### Pattern 2: Separate Downloaded And Connected Progress

**What:** Keep `downloaded_block_height` as the highest contiguous best-chain block body available in the durable store, and keep `connected_block_height`/`block_height` as active chainstate height. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs; docs/architecture/status-snapshot.md]

**When to use:** Use this for BLK-02 and BLK-03, especially when a block body is stored but active chainstate has not advanced. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/sync/runtime_state.rs
// Planning pattern: refresh Summary.best_block_height from chain tip,
// refresh Summary.downloaded_block_height from contiguous best-chain
// durable block bodies, then project both into SyncProgress.
```

### Pattern 3: Typed Block Response Disposition

**What:** Prefer a small enum-like disposition over booleans for block response outcomes: `DownloadedContiguous`, `Connected`, `NotFound`, `Invalid`, `Duplicate`, `DisconnectedOrNonExtending`, and `ResourceLimited` are the planner-relevant cases. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md; packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/sync/types.rs]

**When to use:** Use this when adding tests and telemetry for BLK-03/BLK-04 so duplicate and disconnected responses cannot masquerade as connected-chain progress. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/sync/tests.rs]

**Example:**

```rust
// Source: existing Option<ChainPosition> return in packages/open-bitcoin-node/src/network.rs
// Recommendation: keep connect logic in ManagedPeerNetwork, but expose a
// typed disposition so the sync shell can record peer attribution and
// no-credit outcomes without re-validating block semantics.
```

### Pattern 4: Fresh Snapshot Evidence With Final Peer Correlation

**What:** Derive first progress evidence from fresh `openbitcoinsyncstatus` snapshots captured during the daemon run, then attach peer endpoint/source from final durable peer telemetry when available. [VERIFIED: scripts/run-live-mainnet-smoke.ts; .planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md]

**When to use:** Use this for `result.firstBlockProgress`; do not infer live progress only from final status because Phase 56 established before/after snapshot evidence as the report pattern. [VERIFIED: .planning/phases/56-header-ibd-convergence/56-CONTEXT.md; scripts/run-live-mainnet-smoke.ts]

**Example:**

```ts
// Source: scripts/run-live-mainnet-smoke.ts
// Planning pattern: mirror firstHeaderProgressEvidence(), but compare
// downloadedBlockHeight and connectedBlockHeight snapshots, and require
// connected progress for Phase 57 success.
```

### Anti-Patterns to Avoid

- **Counting any `block` message as connected progress:** `ManagedPeerNetwork::connect_stored_block` can return no connected position for duplicates or non-extending blocks, so the sync shell must not treat every received block body as active-chain advancement. [VERIFIED: packages/open-bitcoin-node/src/network.rs]
- **Adding public mainnet checks to default verification:** BLK-03 live smoke is opt-in UAT evidence and must not enter `bash scripts/verify.sh`. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]
- **Replacing current status truth with report-only fields:** Status, dashboard, metrics, logs, RPC, and live-smoke snapshots already share durable sync truth, so new evidence fields should be additive projections of durable state. [VERIFIED: docs/operator/runtime-guide.md; packages/open-bitcoin-node/src/sync/types/summary.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs]
- **Using raw strings for new no-progress causes:** Existing code uses typed `PeerFailureReason`, `SyncProgressSignal`, `SyncStopReason`, and `NoProgressCause`; Phase 57 should extend typed enums or typed report unions instead of sprinkling ad hoc messages. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; scripts/run-live-mainnet-smoke.ts]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Block hash, header, merkle, and block validation | Custom validators inside sync shell | First-party consensus/chainstate/network APIs | Repo policy requires first-party Bitcoin domain ownership, and existing validation paths already map invalid peer data to typed sync errors. [VERIFIED: AGENTS.md; packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/sync/types.rs] |
| P2P message parsing/encoding | Manual byte parsing in sync runtime or live-smoke script | `WireNetworkMessage`, `PeerManager`, `TcpPeerTransport` | The network crate already owns `getdata`, `notfound`, and `block` payload handling. [VERIFIED: packages/open-bitcoin-network/src/message.rs; packages/open-bitcoin-network/src/peer.rs] |
| Per-peer requested inventory tracking | A second per-peer request map in `DurableSyncRuntime` | `PeerManager::requested_blocks` plus runtime global `inflight_blocks` | Current design splits per-peer capacity from runtime total cap; duplicating both risks stale release paths. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs] |
| Durable progress projection | Report-specific height calculations | `SyncRunSummary`, `runtime_state::downloaded_block_height`, `SyncProgress` | Status surfaces already define downloaded/connected semantics and metrics/log projections. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs; packages/open-bitcoin-node/src/sync/types/summary.rs; docs/architecture/status-snapshot.md] |
| Live evidence rendering | A new smoke-report runner | Extend `scripts/run-live-mainnet-smoke.ts` and `scripts/test-run-live-mainnet-smoke.sh` | Existing runner already handles opt-in preflight, daemon lifecycle, fresh snapshots, final telemetry, JSON report, Markdown report, cancellation, and deterministic fixtures. [VERIFIED: scripts/run-live-mainnet-smoke.ts; scripts/test-run-live-mainnet-smoke.sh] |

**Key insight:** The hard part is preserving one truth model across peer outcomes, durable sync status, metrics/logs, and live-smoke evidence; duplicate local trackers make that harder and are not needed. [VERIFIED: docs/operator/runtime-guide.md; packages/open-bitcoin-node/src/sync/runtime_state.rs]

## Common Pitfalls

### Pitfall 1: Download-Only Evidence Masquerades As Connected Progress

**What goes wrong:** A stored block body or peer `blocks_received` counter is treated as Phase 57 success even when active chainstate did not advance. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md; packages/open-bitcoin-node/src/network.rs]

**Why it happens:** Current `connect_stored_block` returns `Ok(None)` for duplicate/non-extending cases, while the sync shell currently records accepted block contribution after a successful block message path. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/sync.rs]

**How to avoid:** Surface a typed block disposition and make `firstBlockProgress.kind = "connected"` the success signal; keep `"downloaded"` as useful evidence only. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]

**Warning signs:** `blocks_received > 0` while `connected_block_height` is unchanged, or live-smoke `status: passed` caused only by header progress. [VERIFIED: scripts/run-live-mainnet-smoke.ts; packages/open-bitcoin-node/src/status.rs]

### Pitfall 2: In-Flight Requests Leak Across No-Credit Paths

**What goes wrong:** A block hash remains in runtime or per-peer in-flight state after `notfound`, invalid data, disconnect, or end-of-session cleanup, causing duplicate suppression to block future legitimate requests. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md; packages/open-bitcoin-node/src/sync/block_reconcile.rs]

**Why it happens:** Runtime global in-flight state and peer-manager requested inventory are separate structures. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs; packages/open-bitcoin-network/src/peer.rs]

**How to avoid:** Add deterministic tests that exercise both structures for `block`, `notfound`, invalid block body, malformed receive error, and session disconnect cleanup. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs; packages/open-bitcoin-network/src/peer/tests.rs]

**Warning signs:** A second peer cannot request a missing best-chain block after the first peer fails, or `resource_pressure.blocks_in_flight` stays nonzero after a session ends. [VERIFIED: packages/open-bitcoin-node/src/sync/runtime_state.rs]

### Pitfall 3: Live Smoke Diagnosis Is Too Coarse For BLK-03

**What goes wrong:** The report says only `timeout`, `validation_failure`, or `tcp_connection_failure` when the actionable issue is awaiting blocks, peer `notfound`, duplicate/disconnected block, or resource-limit exhaustion. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md; scripts/run-live-mainnet-smoke.ts]

**Why it happens:** Current `NoProgressCause` was designed before Phase 57 and lacks block-specific causes. [VERIFIED: scripts/run-live-mainnet-smoke.ts]

**How to avoid:** Extend the report union and deterministic fixtures for `awaiting_blocks`, `peer_notfound`, `invalid_block`, `duplicate_or_disconnected_block`, and `resource_limit`. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md; scripts/test-run-live-mainnet-smoke.sh]

**Warning signs:** Final peer telemetry has `blocksReceived` or failure reasons, but `maybeNoProgressCause` remains generic. [VERIFIED: scripts/run-live-mainnet-smoke.ts]

### Pitfall 4: Side-Chain Or Unvalidated Inventory Requests Creep In

**What goes wrong:** The downloader starts requesting block hashes from announcements or malformed headers that are not validated best-chain candidates. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]

**Why it happens:** Bitcoin P2P `inv` and `getdata` semantics allow requesting advertised objects, but Phase 57 intentionally narrows the proof to validated best-chain headers. [CITED: https://developer.bitcoin.org/reference/p2p_networking.html] [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]

**How to avoid:** Keep scheduling in `block_reconcile::request_missing_blocks` over `network.best_chain_entries()` and do not add direct `inv`-to-block-body scheduling for this phase. [VERIFIED: packages/open-bitcoin-node/src/sync/block_reconcile.rs]

**Warning signs:** New tests request blocks without a preceding accepted header path. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

## Code Examples

Verified patterns from current sources:

### Bounded Request Scheduling

```rust
// Source: packages/open-bitcoin-node/src/sync/block_reconcile.rs
// Use existing structure:
// 1. available_global = max_blocks_in_flight_total - runtime.inflight_blocks.len()
// 2. scan network.best_chain_entries()
// 3. skip active-chain, runtime in-flight, and durable-local block hashes
// 4. call network.request_missing_blocks(peer_id, &requested)
// 5. insert only emitted GetData inventory into runtime.inflight_blocks
```

### Durable Download/Connect Projection

```rust
// Source: packages/open-bitcoin-node/src/sync/runtime_state.rs
// Use existing projection:
// - best_block_height comes from active chainstate tip
// - downloaded_block_height walks contiguous best-chain entries available
//   in durable block storage
// - SyncProgress exposes both downloaded_block_height and connected_block_height
```

### Live-Smoke Evidence Derivation

```ts
// Source: scripts/run-live-mainnet-smoke.ts
// Mirror firstHeaderProgressEvidence():
// - capture before/after fresh openbitcoinsyncstatus snapshots
// - compute downloaded and connected deltas
// - select final peer telemetry with blocksReceived > 0
// - record peer/source/endpoint/timestamp when available
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| v1.3 live evidence closed through diagnosed blocker/no-progress reports. [VERIFIED: .planning/STATE.md; .planning/PROJECT.md] | v1.4 Phase 56 added first validated header progress evidence from fresh daemon snapshots. [VERIFIED: .planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md] | 2026-06-03 [VERIFIED: .planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md] | Phase 57 should extend this pattern to block progress instead of inventing a separate evidence model. [VERIFIED: scripts/run-live-mainnet-smoke.ts] |
| Block progress surfaces could show one `block_height` compatibility alias. [VERIFIED: docs/operator/runtime-guide.md; docs/architecture/status-snapshot.md] | Status now exposes `downloaded_block_height` and `connected_block_height` separately. [VERIFIED: packages/open-bitcoin-node/src/status.rs; docs/architecture/status-snapshot.md] | Established before Phase 57. [VERIFIED: docs/operator/runtime-guide.md] | BLK-03 should report both, and BLK-02 success should require connected height. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md] |
| Knots-style block download allows direct/parallel requests with per-peer in-flight caps and block request removal after received blocks. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp] | Open Bitcoin's scoped proof uses a smaller bounded runtime: per-peer cap, total cap, validated best-chain scheduling, and opt-in live-smoke evidence. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs; .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md] | Active Phase 57 scope. [VERIFIED: .planning/ROADMAP.md] | Match externally relevant behavior without broadening into full production-node download policy. [VERIFIED: .planning/ROADMAP.md; .planning/PROJECT.md] |

**Deprecated/outdated for this phase:**

- Treating `blockDelta > 0 || headerDelta > 0` as sufficient Phase 57 success is outdated; block success requires first connected non-genesis or checkpoint-adjacent block evidence. [VERIFIED: scripts/run-live-mainnet-smoke.ts; .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]
- A report that exposes only `blockHeight` cannot distinguish downloaded-only from connected-chain progress; Phase 57 needs snapshot fields for downloaded and connected heights. [VERIFIED: scripts/run-live-mainnet-smoke.ts; packages/open-bitcoin-node/src/status.rs]

## Assumptions Log

All claims in this research were verified against provided project files, local command output, vendored Knots source, Bright Builds standards, OWASP/Bitcoin documentation, or codebase grep. [VERIFIED: local research session]

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| - | No `[ASSUMED]` claims recorded. | - | - |

## Open Questions (RESOLVED)

1. **Should Phase 57 introduce a configured checkpoint-adjacent block target, or use first non-genesis only?** [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]
   - What we know: Requirements allow first non-genesis or configured checkpoint-adjacent block. [VERIFIED: .planning/REQUIREMENTS.md]
   - What's unclear: Current sync config has `target_header_height` but no block/checkpoint target field. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs; rg "target_block|checkpoint" packages/open-bitcoin-*]
   - RESOLVED: Phase 57 implements the first non-genesis block as the practical success target. Checkpoint-adjacent block targeting remains future/configured scope only if a configured block/checkpoint target is already present; Phase 57 plans must not add a new target field solely to satisfy the live-smoke proof. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md; docs/operator/runtime-guide.md]

2. **Should duplicate/disconnected block responses count as `blocks_received` peer contribution?** [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/network.rs]
   - What we know: Phase 57 says duplicate/disconnected/non-extending responses must be peer-attributed no-credit paths. [VERIFIED: .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]
   - What's unclear: Current `blocks_received` naming has historically represented accepted block contribution, but duplicate/non-extending block bodies can be syntactically processed without active-chain advancement. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/network.rs]
   - RESOLVED: `blocks_received` means useful accepted block contribution only: a requested validated best-chain block body that is downloaded and/or connected. Duplicate, disconnected, non-extending, malformed, invalid, and `notfound` responses are no-credit outcomes and must use separate typed dispositions or peer failure reasons instead of incrementing useful contribution. [VERIFIED: packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/types.rs]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust / Cargo | Rust implementation and tests | yes | `rustc 1.94.1`, `cargo 1.94.1` | None needed. [VERIFIED: rustc --version; cargo --version] |
| Bun | Live-smoke TypeScript runner and deterministic script checks | yes | `1.3.9` | None needed. [VERIFIED: bun --version] |
| Bazel / Bazelisk | Repo-native smoke build and UAT command parity | yes | `8.6.0` | Cargo commands can verify targeted code, but repo-native `bash scripts/verify.sh` still expects Bazel smoke support. [VERIFIED: bazel --version; bazelisk --version; AGENTS.md] |
| Bitcoin Knots submodule | Parity source anchors | yes | `v29.3.knots20260210` commit `a9aee730466ac67d35a3c03ee24676be5e045878` | Run `git submodule update --init --recursive` if missing. [VERIFIED: git submodule status packages/bitcoin-knots; AGENTS.md] |
| Public Bitcoin network | Opt-in UAT live smoke only | not probed | - | Deterministic scripted tests remain default verification. [VERIFIED: .planning/REQUIREMENTS.md; docs/operator/runtime-guide.md] |

**Missing dependencies with no fallback:**
- None found for deterministic planning and implementation. [VERIFIED: local command outputs]

**Missing dependencies with fallback:**
- Public-network connectivity was intentionally not probed; live smoke remains opt-in and can produce typed no-progress evidence. [VERIFIED: .planning/REQUIREMENTS.md; scripts/run-live-mainnet-smoke.ts]

## Security Domain

ASVS categories are listed using the OWASP ASVS section model. [CITED: https://devguide.owasp.org/en/06-verification/01-guides/03-asvs/]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | Phase 57 does not change authentication surfaces. [VERIFIED: .planning/ROADMAP.md; .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md] |
| V3 Session Management | no | Phase 57 does not add web/session state. [VERIFIED: .planning/ROADMAP.md] |
| V4 Access Control | no | Phase 57 does not add user authorization decisions. [VERIFIED: .planning/ROADMAP.md] |
| V5 Validation, Sanitization and Encoding | yes | Use existing wire decoding, header validation, block validation, and typed invalid-data outcomes; do not bypass `WireNetworkMessage` or first-party validation APIs. [VERIFIED: packages/open-bitcoin-network/src/message.rs; packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/sync/types.rs] |
| V6 Stored Cryptography | limited | Do not alter cryptographic/hash/PoW validation paths; use first-party consensus and chainstate functions. [VERIFIED: packages/open-bitcoin-node/src/network.rs; packages/open-bitcoin-node/src/sync/tests.rs; AGENTS.md] |
| V7 Error Handling and Logging | yes | Preserve typed peer failures, health signals, structured logs, and non-sensitive operator guidance. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/types/summary.rs; docs/operator/runtime-guide.md] |

### Known Threat Patterns for P2P Block Sync

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Malformed or invalid block body from a public peer | Tampering / Denial of Service | Validate through managed network/chainstate before saving; attribute as `InvalidData`; do not advance active chainstate. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/tests.rs] |
| `notfound` or missing data after `getdata` | Denial of Service | Release in-flight inventory, attribute peer outcome, and retry other eligible peers within caps. [VERIFIED: packages/open-bitcoin-network/src/peer.rs; packages/open-bitcoin-node/src/sync/block_reconcile.rs] |
| Duplicate or non-extending block response | Tampering / Integrity | Use typed disposition; do not advance connected height or duplicate connect work. [VERIFIED: packages/open-bitcoin-node/src/network.rs; .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md] |
| Resource-limit exhaustion | Denial of Service | Enforce `max_blocks_in_flight_per_peer`, `max_blocks_in_flight_total`, `max_messages_per_peer`, and `max_rounds`; surface resource pressure in durable status. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/runtime_state.rs] |
| Misleading operator evidence | Repudiation / Integrity | Derive evidence from fresh daemon snapshots and durable peer telemetry, and preserve no-progress causes. [VERIFIED: scripts/run-live-mainnet-smoke.ts; .planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md` - locked Phase 57 decisions, scope, existing code insights, and deferred ideas.
- `.planning/REQUIREMENTS.md` - BLK-01 through BLK-04.
- `.planning/ROADMAP.md` and `.planning/STATE.md` - Phase 57 scope, v1.4 boundaries, and Phase 56 readiness.
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo constraints, verification contract, and Bright Builds routing.
- Bright Builds standards at commit `05f8d7a6c9c2e157ec4f922a05273e72dab97676` - architecture, code shape, verification, testing, Rust, and TypeScript guidance.
- `packages/open-bitcoin-node/src/sync.rs`, `sync/block_reconcile.rs`, `sync/runtime_state.rs`, `sync/types.rs`, `sync/types/summary.rs`, `sync/types/projection.rs`, `sync/tests.rs` - sync runtime implementation surface.
- `packages/open-bitcoin-node/src/network.rs`, `packages/open-bitcoin-network/src/peer.rs`, and their tests - managed peer/block request and connect behavior.
- `scripts/run-live-mainnet-smoke.ts` and `scripts/test-run-live-mainnet-smoke.sh` - live-smoke schema/report/test surface.
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/parity/catalog/p2p.md`, `docs/parity/source-breadcrumbs.json` - operator, status, parity, and breadcrumb surfaces.
- `packages/bitcoin-knots/src/net_processing.cpp` - pinned baseline block download, in-flight, `getdata`, and block processing anchor.

### Secondary (MEDIUM confidence)

- Bitcoin Developer Reference P2P Networking - `getdata`, `block`, `headers`, `notfound`, inventory, and protocol context. [CITED: https://developer.bitcoin.org/reference/p2p_networking.html]
- OWASP Developer Guide ASVS overview - ASVS category names and security verification framing. [CITED: https://devguide.owasp.org/en/06-verification/01-guides/03-asvs/]

### Tertiary (LOW confidence)

- None used for recommendations. [VERIFIED: source review]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - repo instructions, local tool versions, Cargo metadata, and submodule status were verified. [VERIFIED: AGENTS.md; local command outputs]
- Architecture: HIGH - implementation surfaces and repo architecture documents agree on functional-core / imperative-shell boundaries. [VERIFIED: .planning/ARCHITECTURE.md; packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/network.rs]
- Pitfalls: HIGH - risks are directly tied to current code paths and Phase 57 decisions. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/network.rs; .planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md]
- Live-smoke changes: HIGH - current script schema and Phase 56 first-header pattern were inspected. [VERIFIED: scripts/run-live-mainnet-smoke.ts; .planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md]

**Research date:** 2026-06-03
**Valid until:** Phase 58 planning starts or 2026-06-17, whichever comes first, because active v1.4 sync code is changing phase-by-phase. [VERIFIED: .planning/ROADMAP.md]
