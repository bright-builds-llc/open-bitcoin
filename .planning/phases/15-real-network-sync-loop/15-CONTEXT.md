---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 15-2026-04-26T21-05-56
generated_at: 2026-04-26T21:08:01.619Z
---

# Phase 15: Real Network Sync Loop - Context

**Gathered:** 2026-04-26
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Turn the existing peer/message/header primitives into a node-shell sync runtime that can connect to configured or DNS-seeded peers, drive handshake/header/block sync, persist progress through the Phase 14 store, and expose deterministic status/error evidence. This phase does not implement metrics history retention, log rotation, CLI status rendering, service management, dashboard UI, migration flows, or release benchmarks.

</domain>

<decisions>
## Implementation Decisions

### Runtime ownership
- **D-01:** Implement real-network sync in `open-bitcoin-node` as an imperative shell around existing `open-bitcoin-network`, `open-bitcoin-chainstate`, and Phase 14 storage contracts.
- **D-02:** Keep pure crates free of filesystem, socket, DNS, and database dependencies. Any TCP/DNS/live-network code belongs in the node shell and must be testable through deterministic adapters.
- **D-03:** Preserve the existing protocol primitives and peer state machine rather than replacing them with a new networking stack.

### Peer sources and live networking
- **D-04:** Support manual peer addresses and DNS seed hostnames in the sync config. Mainnet is the first-class default; testnet/signet/regtest constants may be modeled when they are useful and low-risk.
- **D-05:** Default verification must never contact the public Bitcoin network. Live-network smoke behavior must be opt-in through ignored tests, explicit config, or a non-default command path.
- **D-06:** The real TCP adapter should be small: connect with timeouts, encode/decode existing wire messages, validate network magic, and leave protocol decisions to `PeerManager`.

### Sync progress and persistence
- **D-07:** On startup, load persisted chainstate and headers from `FjallNodeStore` and seed the runtime before connecting to peers.
- **D-08:** Persist header entries, block-index/header metadata, accepted blocks, connected chainstate snapshots, runtime metadata, and sync metric samples as progress is made.
- **D-09:** Expose sync progress through the shared status model using header height, block height, progress ratio, peer counts, and health signals.

### Flow control and failures
- **D-10:** Bound in-flight block requests per peer. Do not allow a large `headers` response to produce unbounded `getdata` requests.
- **D-11:** Treat disconnects, timeouts, stalls, invalid magic, invalid payloads, missing ancestors, validation failures, and storage failures as typed runtime outcomes rather than panics.
- **D-12:** Retry behavior should be explicit and bounded. Default tests should assert retry/stall outcomes without sleeping or using real sockets.

### Tests and verification
- **D-13:** Deterministic simulated-network tests are required for handshake, headers sync, block download/connect, persistence/resume, bounded in-flight work, and failure paths.
- **D-14:** Optional live-network smoke tests must be ignored or otherwise explicitly opt-in and documented as non-default.
- **D-15:** Repo-native verification remains the commit gate.

### the agent's Discretion
- Exact type names, helper layout, and summary fields are discretionary if the implementation preserves the shell/core boundary, persists progress, keeps tests hermetic, and avoids broad dependency additions.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope
- `.planning/ROADMAP.md` - Phase 15 goal, dependencies, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` - SYNC-01 through SYNC-05 definitions and SYNC-06 downstream observability dependency.
- `.planning/PROJECT.md` - parity, functional-core, dependency, and verification constraints.
- `.planning/phases/13-operator-runtime-foundations/13-CONTEXT.md` - shared status, metrics, config, and shell ownership decisions.
- `.planning/phases/14-durable-storage-and-recovery/14-CONTEXT.md` - Phase 14 durable storage and recovery decisions.

### Existing code boundaries
- `packages/open-bitcoin-network/src/peer.rs` - handshake, getheaders, headers, getdata, inventory, and peer state machine.
- `packages/open-bitcoin-network/src/message.rs` - wire message encoding/decoding and network magic handling.
- `packages/open-bitcoin-network/src/header_store.rs` - header metadata and locator behavior.
- `packages/open-bitcoin-node/src/network.rs` - managed peer network integration with chainstate and mempool.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - durable store used for sync progress.
- `packages/open-bitcoin-node/src/status.rs` - shared status snapshot model.
- `docs/parity/source-breadcrumbs.json` - required source breadcrumb manifest for new Rust files.

### Standards
- `AGENTS.md` - Repo-local workflow, parity, verification, and GSD rules.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Local standards exceptions.
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/verification.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `PeerManager` already handles version/verack/wtxidrelay/sendheaders/getheaders/headers/getdata/tx/block protocol decisions.
- `ManagedPeerNetwork` already connects received blocks into chainstate and records mempool/network info.
- `FjallNodeStore` already persists headers, block index metadata, chainstate snapshots, runtime metadata, and metrics placeholders.
- `OpenBitcoinStatusSnapshot`, `SyncStatus`, `PeerStatus`, and `HealthSignal` already provide the status contract that later CLI/TUI work should consume.

### Established Patterns
- Node-shell modules own side effects while pure crates keep protocol/domain logic.
- New Rust files need parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries.
- Multi-file modules use `foo.rs` plus `foo/tests.rs`.
- Tests use isolated tempdirs and deterministic fake adapters for effectful behavior.

### Integration Points
- `open-bitcoin-node::lib` should export sync runtime/config/status types.
- `open-bitcoin-network::PeerManager` may need a small flow-control knob or helper for bounded block requests.
- `FjallNodeStore` may need block persistence helpers in addition to existing snapshot/header persistence.

</code_context>

<specifics>
## Specific Ideas

- Prefer a runtime that is useful for real operators but can be exercised entirely with fake peers in default tests.
- Treat breadcrumbs, status, and metrics as evidence for later CLI/dashboard work, not decorative output.

</specifics>

<deferred>
## Deferred Ideas

- Metrics history retention and log rotation belong to Phase 16.
- Rich `open-bitcoin status` output and onboarding belong to Phase 17.
- Service lifecycle integration belongs to Phase 18.
- Ratatui dashboard rendering belongs to Phase 19.
- Full real-sync benchmark reports belong to Phase 22.

</deferred>

---

*Phase: 15-real-network-sync-loop*
*Context gathered: 2026-04-26*
