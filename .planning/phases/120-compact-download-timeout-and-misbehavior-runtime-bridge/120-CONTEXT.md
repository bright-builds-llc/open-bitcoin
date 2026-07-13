---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 120-2026-07-13T20-01-34
generated_at: 2026-07-13T20:02:07.102Z
---

# Phase 120: Compact Download Timeout and Misbehavior Runtime Bridge - Context

**Gathered:** 2026-07-13
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 120 closes the v2.1 audit gaps for RCN-07, GOV-02, and GOV-03 (timeout tick + misbehavior bridge): schedule `expire_compact_download_timeouts` from the node/sync runtime on a deterministic tick so timeout expiration can emit full-block fallback or suppression `PeerAction`s on the live path, and escalate typed compact misbehavior beyond silent empty-action suppress into Knots-aligned disconnect, score, or suppression decisions.

Today `PeerManager::expire_compact_download_timeouts` exists and is unit-tested, but `open-bitcoin-node` never calls it. Typed `CompactBlockTxnMisbehavior` outcomes already exist, but `compact_block_txn_actions` maps `Misbehavior(_)` (and related unexpected paths) to an empty `Vec<PeerAction>` — silent suppress only.

This phase wires the timeout tick and misbehavior escalation only. It must not project block-relay metrics/logs through `DurableSyncRuntime` (Phase 121), enable package relay, bloom/filter or compact-filter serving, change public defaults, add public-network CI gates, claim archive-node behavior, claim production full-node readiness, or claim production-funds wallet safety.

</domain>

<decisions>
## Implementation Decisions

### Timeout Tick Scheduling Seam

- **D-01:** Call `PeerManager::expire_compact_download_timeouts(now_unix_seconds)` from the node shell on a live runtime path. Prefer a `ManagedPeerNetwork` forwarder that mirrors the existing `expire_transaction_requests` pattern (thin shell → PeerManager → translate/return `PeerAction`s), not a DurableSyncRuntime-only metrics hook.
- **D-02:** The tick must be deterministic and caller-clocked: pass explicit `now_unix_seconds` from the existing receive/drive/poll path that already owns wall-clock for relay timeouts. Do not invent a background thread or Tokio timer as the primary seam; reuse the same “operator/runtime supplies now” contract used for transaction request expiry.
- **D-03:** Timeout expiration must produce live-path `PeerAction`s for full-block fallback (and suppression when policy suppresses). Ensure returned actions are translated and sent the same way other compact download actions already are — not discarded after a pure call.

### Misbehavior Escalation Bridge

- **D-04:** Stop mapping `CompactBlockTxnHandleOutcome::Misbehavior(_)` to an empty `PeerAction` list. Typed compact misbehavior must escalate to Knots-aligned disconnect, score/discourage, or explicit suppression decisions via existing peer-policy / `PeerAction::Disconnect` / misbehavior recording surfaces.
- **D-05:** Cover GOV-02 cases called out by the audit and requirements: malformed compact blocks, invalid compact-block headers, duplicate `blocktxn`, unexpected `blocktxn`, and out-of-bounds indexes. Prefer mapping through existing `CompactBlockTxnMisbehavior` variants into `MisbehaviorKind` / disconnect reasons rather than inventing a parallel policy stack.
- **D-06:** Keep benign no-match paths suppressible when Knots would ignore them (e.g. true `NoMatchingInFlight` with no in-flight state). Do not treat every empty outcome as disconnect — only typed misbehavior and Knots-aligned unexpected/malformed cases escalate.

### Volatile Cleanup Contract

- **D-07:** Timeout expiration must clear only volatile compact-download in-flight state for expired entries (already the intent of `expire_stale_compact_downloads`). Disconnect, timeout, and reorg cleanup must continue to remove only volatile compact-relay state — never validated chainstate or durable block data (GOV-03).
- **D-08:** If `on_compact_download_block_connected` (or equivalent block-connect volatile cleanup) is still unwired from the node shell, wire it in this phase as part of GOV-03 completeness. Do not expand into mempool/package surfaces already closed by Phase 119.

### Verification And Scope Isolation

- **D-09:** Runtime/unit tests must prove: (1) node/shell tick calls `expire_compact_download_timeouts` and yields fallback/suppression actions on the live path, (2) typed misbehavior yields non-empty disconnect/score/suppression actions rather than silence-only, (3) disconnect/timeout/reorg cleanup still touches only volatile compact state, (4) Phase 121 DurableSyncRuntime block-relay metric/log projection, package/filter/public-default surfaces stay untouched.
- **D-10:** New or touched first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries unless an explicit `none` breadcrumb is defensible. Prefer Knots `net_processing.cpp` compact download timeout and misbehavior anchors.
- **D-11:** Verification remains deterministic and local through repo-native checks and `bash scripts/verify.sh`. Public-network compact-relay review stays opt-in UAT only.

### Claude's Discretion

The planner/researcher may choose exact tick call-site (ManagedPeerNetwork method invoked from receive loop vs sync drive helper), the precise `MisbehaviorKind` / disconnect-reason mapping table for each `CompactBlockTxnMisbehavior` variant within Knots alignment, whether escalation emits `PeerAction::Disconnect` alone or also records through `record_peer_policy_misbehavior`, and how tests advance `now_unix_seconds` to force expiry. Prefer early returns, the smallest seam that closes the audit gap, and reuse of existing action-translation / peer-policy bridges.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope

- `AGENTS.md` — repo-local verification, submodule, parity breadcrumb, UAT command, and GSD workflow guidance.
- `AGENTS.bright-builds.md` — Bright Builds workflow, functional-core, verification, and testing rules.
- `standards/core/architecture.md` — functional core / imperative shell and domain-type rules.
- `standards/core/code-shape.md` — early-return, optional-name, and file/function shape rules.
- `standards/core/testing.md` — focused unit test and Arrange/Act/Assert expectations.
- `standards/core/verification.md` — repo-native verification and clean commit gate expectations.
- `standards/languages/rust.md` — Rust module, invariant, optional naming, and verification guidance.
- `.planning/PROJECT.md` — active v2.1 scope, parity value, architecture constraints, and deferred public/production claims.
- `.planning/REQUIREMENTS.md` — RCN-07, GOV-02, GOV-03 ownership for Phase 120.
- `.planning/ROADMAP.md` — Phase 120 goal, success criteria, and gap-closure framing.
- `.planning/STATE.md` — current milestone state and deterministic verification caveats.
- `.planning/v2.1-MILESTONE-AUDIT.md` — RCN-07/GOV-02/GOV-03 gap evidence: expire never called from node; misbehavior maps to empty PeerAction list.

### Prior Locked Decisions

- `.planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff/115-CONTEXT.md` — timeout/fallback/cleanup contracts already designed; this phase wires the runtime tick and escalates misbehavior beyond silence.
- `.planning/phases/119-compact-receive-mempool-candidate-injection/119-CONTEXT.md` — explicitly deferred timeout ticks and misbehavior escalation to Phase 120.
- `.planning/phases/114-compact-block-reconstruction-from-mempool-state/114-CONTEXT.md` — typed reconstruction outcomes and volatile partial state.
- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md` — negotiation independence from package/filter/public defaults.
- `.planning/phases/117-parity-traceability-uat-and-release-guardrails/117-CONTEXT.md` — no-claim and verifier-boundary posture to preserve.
- `.planning/phases/96-peer-policy-runtime-bridge/96-CONTEXT.md` — existing misbehavior/ban/discourage runtime bridge patterns to reuse.

### Existing Code Integration Points

- `packages/open-bitcoin-network/src/peer/compact_download_state.rs` — `expire_compact_download_timeouts`; `compact_block_txn_actions` currently maps `Misbehavior(_)` to empty Vec.
- `packages/open-bitcoin-network/src/compact_download.rs` — `CompactBlockTxnHandleOutcome`, timeout expiry helpers, fallback actions.
- `packages/open-bitcoin-node/src/network/action_translation.rs` — `expire_transaction_requests` forwarder pattern to mirror.
- `packages/open-bitcoin-node/src/network/peer_policy.rs` — `record_peer_policy_misbehavior` bridge.
- `packages/open-bitcoin-network/src/peer_policy.rs` — `MisbehaviorKind`, `MisbehaviorPolicy`, disconnect/discourage/ban responses.
- `packages/open-bitcoin-node/src/network.rs` — ManagedPeerNetwork receive/drive surfaces for tick placement.
- `docs/parity/source-breadcrumbs.json` — required breadcrumb registry for new first-party Rust source/test files.
- `scripts/verify.sh` — repo-native verification contract.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` — compact download timeout, fallback getdata, and compact misbehavior/disconnect handling.
- `packages/bitcoin-knots/src/blockencodings.cpp` — compact block / blocktxn validation failure paths.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` — timeout and malformed compact behavior expectations.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `PeerManager::expire_compact_download_timeouts` already returns full-block fetch `PeerAction`s for expired in-flight downloads.
- `ManagedPeerNetwork::expire_transaction_requests` is the proven shell forwarder + action-translation pattern for timeout ticks.
- Typed `CompactBlockTxnMisbehavior` variants and `MisbehaviorPolicy` / peer-policy runtime bridge already exist.
- Disconnect cleanup for compact download state is already wired; timeout tick and misbehavior escalation are the remaining gaps.

### Established Patterns

- Caller-supplied `now_unix_seconds` for deterministic expiry tests.
- Pure network outcomes → `PeerAction` translation in peer modules; node shell only applies effects.
- Low-cardinality typed outcomes and Knots-aligned misbehavior kinds.

### Integration Points

- Primary gap: no callers of `expire_compact_download_timeouts` under `packages/open-bitcoin-node/src`.
- Primary gap: `compact_block_txn_actions` returns `Vec::new()` for `Misbehavior(_)`, `UnexpectedBlockHash`, and `NoMatchingInFlight`.
- Optional gap: `on_compact_download_block_connected` may still lack a node-shell caller (confirm during research).

</code_context>

<specifics>
## Specific Ideas

- Mirror `expire_transaction_requests` for the timeout forwarder so operators and tests share one clock-injection style.
- Prefer mapping compact misbehavior into the existing peer-policy decision path rather than inventing a compact-only ban book.
- Keep Phase 121 (`persist_metrics` / `block_relay_log_record`) explicitly out of scope even if adjacent evidence counters already exist.

</specifics>

<deferred>
## Deferred Ideas

- Block-relay metrics and structured log runtime projection through `DurableSyncRuntime` — Phase 121 / OBS-03.
- Package relay, bloom/filter serving, compact filters, public defaults, public-network CI, production full-node readiness, production-funds wallet claims — out of v2.1 gap-closure scope.

</deferred>

<consensus>
## Consensus

Yolo mode auto-accepted recommended defaults: shell-forwarded deterministic timeout tick mirroring transaction expiry, Knots-aligned misbehavior escalation beyond silent suppress, volatile-only cleanup preservation, and strict Phase 121 / package / public-default isolation.

</consensus>
