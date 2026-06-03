---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 55-2026-06-02T22-36-24
generated_at: 2026-06-02T22:38:08.006Z
---

# Phase 55: Outbound Handshake Compatibility Fixes - Context

**Gathered:** 2026-06-02
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 55 makes daemon sync treat a completed outbound `version`/`verack`
handshake as a compatible connected peer even when that peer idles before
headers or blocks arrive. It also preserves existing protocol rejection
safeguards by converting disconnect decisions and malformed peer data into typed
sync outcomes, then moving on to replacement peers without useful-progress
credit.

Public-mainnet live smoke remains opt-in evidence outside `bash scripts/verify.sh`.

</domain>

<decisions>

## Implementation Decisions

### Handshake Completion

- **D-01:** Use existing peer-manager handshake state as the source of truth:
  local `version` sent, remote `version` received, local `verack` sent, and
  remote `verack` received.
- **D-02:** If the peer idles after that complete handshake, classify the sync
  outcome as `Connected`, not `Stalled`.
- **D-03:** If the peer idles before handshake completion, keep the existing
  `Stalled` outcome, warning signal, and retry backoff behavior.

### Incompatible Peer Outcomes

- **D-04:** Propagate peer-manager disconnect actions as sync failures instead
  of silently continuing after the peer is removed.
- **D-05:** Duplicate-version rejection should remain deterministic and should
  produce a typed failed peer outcome with no header or block contribution.
- **D-06:** Wrong-network and malformed-message behavior should remain covered
  by deterministic tests and should not receive useful-progress credit.
- **D-07:** Mixed compatible and incompatible peer runs should keep durable
  summary state coherent: compatible peers fill outbound slots, incompatible
  peers fail/back off, and only accepted headers or blocks advance durable
  progress.

### Scope Controls

- **D-08:** Do not add new public-network checks, CLI commands, peer discovery
  policy, inbound serving, relay, compact blocks, or production-node claims in
  this phase.
- **D-09:** Update parity docs only for the daemon-integrated behavior that
  materially changes the externally visible sync surface.

### the agent's Discretion

The agent may choose the exact helper names and test placement, provided the
change stays in the existing pure-core/network shell boundary and all evidence
is deterministic.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 55 goal and success criteria.
- `.planning/REQUIREMENTS.md` - COMPAT-03 and COMPAT-05.
- `.planning/PROJECT.md` - v1.4 scope boundaries and public-network evidence
  constraints.
- `.planning/STATE.md` - Carry-forward Phase 53 `handshake_failure` blocker and
  Phase 54 completion state.

### Prior Phase Evidence

- `.planning/phases/54-peer-compatibility-baseline-and-diagnostic-harness/54-CONTEXT.md` - Locked compatibility harness decisions.
- `.planning/phases/54-peer-compatibility-baseline-and-diagnostic-harness/54-VERIFICATION.md` - Deterministic compatibility diagnosis evidence.
- `.planning/phases/53-live-evidence-refresh/53-VERIFICATION.md` - Fresh
  diagnosed `handshake_failure` live-smoke blocker that Phase 55 addresses
  without making live smoke a default gate.

### Implementation Surfaces

- `packages/open-bitcoin-network/src/peer.rs` - Pure peer lifecycle state,
  outbound handshake actions, and disconnect decisions.
- `packages/open-bitcoin-network/src/compatibility.rs` - Phase 54 typed
  compatibility diagnosis harness.
- `packages/open-bitcoin-node/src/network.rs` - Managed network adapter that
  turns peer actions into sync messages and errors.
- `packages/open-bitcoin-node/src/sync.rs` - Durable sync runtime, peer
  selection, connected/stalled/failed outcome recording, and durable state
  persistence.
- `packages/open-bitcoin-node/src/sync/types.rs` - Sync outcome and failure
  reason taxonomy.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Deterministic scripted peer
  tests for manual peers, DNS peers, replacement behavior, contribution
  accounting, and durable state.
- `docs/parity/catalog/p2p.md` - Auditable P2P parity surface.

### Baseline Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` - Baseline peer-processing
  behavior anchor.
- `packages/bitcoin-knots/test/functional/p2p_handshake.py` - Handshake and
  rejection behavior anchor.
- `packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py` -
  Initial headers sync behavior anchor.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `PeerManager::add_outbound_peer` already records local `version` send state.
- `PeerManager::handle_message` already records remote `version`, local
  `verack`, and remote `verack` state.
- `ManagedPeerNetwork::peer_manager()` exposes the pure peer state to the sync
  shell without adding I/O to the network crate.
- `ScriptedTransport` and `ScriptedResolver` in sync tests already model manual,
  DNS, replacement, invalid, and stalled peers deterministically.

### Established Patterns

- Pure protocol decisions stay in `open-bitcoin-network`; durable peer
  selection, backoff, logs, and storage remain in `open-bitcoin-node`.
- Useful progress means accepted headers or blocks, not mere activity or failed
  compatibility checks.
- Tests use Arrange, Act, Assert comments for non-trivial cases.

### Integration Points

- Add a narrow helper in the sync runtime or managed network shell to check
  whether handshake state is complete.
- Change managed disconnect processing so duplicate-version and malformed
  disconnects become typed sync failures.
- Update existing sync tests around stalled peers and add replacement/durable
  state assertions for mixed compatible and incompatible peers.

</code_context>

<specifics>

## Specific Ideas

- Keep `version_verack_script(0)` as the deterministic successful handshake
  fixture.
- Add an explicit pre-handshake idle test so the previous stall warning behavior
  stays covered.
- Use existing manual and DNS peer constructors to prove both sources can
  produce connected outcomes without live network access.

</specifics>

<deferred>

## Deferred Ideas

- Multi-batch validated header convergence remains Phase 56.
- Block download/connect progress remains Phase 57.
- Restart/resume proof remains Phase 58.
- Support bundle, release boundary, and threat-model closeout remain Phase 59.

</deferred>

---

*Phase: 55-outbound-handshake-compatibility-fixes*
*Context gathered: 2026-06-02*
