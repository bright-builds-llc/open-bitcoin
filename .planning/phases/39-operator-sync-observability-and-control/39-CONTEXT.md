---
phase: 39
phase_name: "Operator Sync Observability and Control"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "39-2026-05-02T11-46-08"
generated_at: "2026-05-02T12:22:11Z"
---

# Phase 39 Context: Operator Sync Observability and Control

**Gathered:** 2026-05-02
**Status:** Ready for planning

<domain>
## Phase Boundary

Make daemon-owned mainnet sync understandable and controllable through the existing operator surfaces without reopening the sync mechanics that Phases 36 through 38 already settled. This phase owns truthful status, dashboard, metrics/log, RPC blockchain-info truth, and explicit operator pause/resume control. It does not own live-mainnet benchmark closeout, packaged-service hardening, inbound-serving expansion, or broader production-node claims.

</domain>

<decisions>
## Implementation Decisions

### Shared truth model
- **D-01:** Persist durable sync state in the node runtime metadata so CLI status, dashboard, `open-bitcoind`, and RPC can read the same lifecycle, lag, peer, and recovery truth.
- **D-02:** Extend the shared `SyncStatus` and `PeerStatus` contracts instead of creating a parallel daemon-sync-only snapshot type.
- **D-03:** Treat durable sync truth as authoritative over thin live RPC derivations whenever it is available for the selected datadir.

### Operator controls
- **D-04:** Add an explicit `open-bitcoin sync status|pause|resume` operator surface instead of asking operators to inspect or mutate internal store files manually.
- **D-05:** Represent pause as durable control state that the daemon sync loop polls between bounded rounds, not as an out-of-band process signal or a second config file.

### RPC and renderer truthfulness
- **D-06:** Make `getblockchaininfo` report header height, block height, verification progress, and IBD truth from durable sync state when it exists, rather than hard-coding `headers == blocks` or `initialblockdownload = false`.
- **D-07:** Keep dashboard and human status renderers additive: surface lifecycle, phase, lag, pressure, recovery guidance, and peer detail without regressing the existing quiet operator tone.

### the agent's Discretion
- Exact renderer wording for pressure and peer-detail lines.
- Whether the daemon writes lifecycle `active` versus `recovering` on a given round, as long as persisted status remains truthful and deterministic.
- The smallest README and runtime-guide changes needed to keep contributor-facing claims honest.

</decisions>

<specifics>
## Specific Ideas

- Reuse `RuntimeMetadata` as the durable home for sync status/control to avoid inventing another persistence path.
- Keep pause/resume local to the selected datadir so service-managed and shell-managed runs observe the same durable intent.
- Reconcile stale roadmap/state metadata for Phases 37 and 38 before closing Phase 39 so later phase selection is not anchored to outdated planning state.

</specifics>

<canonical_refs>
## Canonical References

- `.planning/REQUIREMENTS.md` — `PEERMAIN-04`, `CHAINMAIN-05`, `RESUME-04`, `OBSMAIN-01` through `OBSMAIN-04`, `VERMAIN-01`, and `VERMAIN-02`
- `.planning/ROADMAP.md` — Phase 39 goal, success criteria, and dependencies
- `.planning/STATE.md` — current milestone state and stale phase bookkeeping that this phase must reconcile
- `AGENTS.md` — repo-local verification, parity breadcrumb, and doc freshness expectations
- `AGENTS.bright-builds.md` — Bright Builds sync-before-edit and verification workflow
- Bright Builds `standards/core/architecture.md`
- Bright Builds `standards/core/code-shape.md`
- Bright Builds `standards/core/verification.md`
- Bright Builds `standards/core/testing.md`
- Bright Builds `standards/languages/rust.md`

</canonical_refs>

<code_context>
## Existing Code Insights

- `packages/open-bitcoin-node/src/status.rs` already owns the shared operator snapshot surface used by both status and dashboard.
- `packages/open-bitcoin-node/src/sync/types.rs` already owns the rich peer and sync summary truth that Phase 39 needs to project.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` still had a preflight-only daemon boundary before this phase.
- `packages/open-bitcoin-cli/src/operator/status.rs` and `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` were still deriving sync truth from thin RPC fields and peer counts only.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` was still reporting `headers == blocks` and `initialblockdownload = false` whenever any chain tip existed.

</code_context>

<deferred>
## Deferred Ideas

- Live mainnet smoke/benchmark closeout remains Phase 40.
- Production-node, production-funds, inbound-serving, addrman, and broader relay claims remain out of scope.
- Dashboard action-bar pause/resume affordances can follow later if the CLI control surface proves insufficient.

</deferred>

---

*Phase: 39-operator-sync-observability-and-control*
*Context gathered: 2026-05-02*
