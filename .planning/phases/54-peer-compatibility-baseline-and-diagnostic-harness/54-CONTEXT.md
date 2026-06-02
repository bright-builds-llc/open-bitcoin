---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 54-2026-06-02T20-31-26
generated_at: 2026-06-02T20:32:15.521Z
---

# Phase 54: Peer Compatibility Baseline and Diagnostic Harness - Context

**Gathered:** 2026-06-02
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 54 makes outbound handshake and early-protocol failures reproducible,
typed, and comparable to the pinned Bitcoin Knots baseline. It covers the
`version`, `verack`, `sendheaders`, `wtxidrelay`, `getheaders`, and `getdata`
flow surfaces, but it does not yet change live peer selection or claim that
public peers complete a successful handshake. Phase 55 owns compatibility fixes
for reachable peers.

</domain>

<decisions>

## Implementation Decisions

### Harness Shape

- **D-01:** Build a hermetic deterministic compatibility harness around the
  existing pure network peer/message core instead of relying on public-network
  smoke behavior.
- **D-02:** The harness should model scripted peer transcripts and return a
  structured compatibility report that names each observed step and each local
  response.
- **D-03:** Keep live-mainnet checks outside `bash scripts/verify.sh`; default
  verification must stay deterministic.

### Baseline Comparison

- **D-04:** Capture the Knots baseline comparison as auditable data or docs that
  reviewers can inspect directly for `version`, `verack`, `sendheaders`,
  `wtxidrelay`, `getheaders`, and `getdata`.
- **D-05:** Compare externally observable message order and diagnostic outcomes,
  not line-by-line implementation internals.
- **D-06:** Treat the pinned Knots submodule as the source anchor for protocol
  behavior. Do targeted Knots/protocol comparison only; broad ecosystem research
  is intentionally out of scope for v1.4 planning.

### Diagnostics

- **D-07:** Diagnostics must use typed outcome variants for version rejection,
  network mismatch, service-bit mismatch, unsupported message order, timeout,
  peer disconnect, malformed payload, and local configuration failure.
- **D-08:** A failure report should include the failing step, peer endpoint or
  scripted peer identity when available, observed command, diagnosis, and next
  operator action.
- **D-09:** Unsupported or failed peers must not receive useful-progress credit
  in harness output.

### Integration and Scope

- **D-10:** Prefer a small first-party Rust surface in `open-bitcoin-network`
  for pure transcript evaluation, with shell/CLI surfaces added only if needed
  for operator ergonomics.
- **D-11:** Reuse live-smoke terminology where it already matches v1.3 outcomes,
  but avoid embedding raw live-smoke reports or public-network logs in the
  deterministic harness.
- **D-12:** Do not broaden scope into inbound serving, transaction relay,
  compact-block handling, production-node operation, migration apply mode, or
  public-network verification gates.

### the agent's Discretion

The agent may choose the exact Rust module layout and report schema as long as
it preserves the typed diagnostics, hermetic default behavior, and auditability
requirements above.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase and Milestone Scope

- `.planning/ROADMAP.md` - Phase 54 goal and success criteria.
- `.planning/REQUIREMENTS.md` - COMPAT-01, COMPAT-02, and COMPAT-04
  acceptance requirements.
- `.planning/PROJECT.md` - v1.4 scope boundary, Knots baseline, and
  functional-core/imperative-shell constraints.
- `.planning/STATE.md` - Current milestone state and carry-forward decisions.

### Existing Open Bitcoin Surfaces

- `packages/open-bitcoin-network/src/message.rs` - Wire message types,
  protocol constants, and encode/decode behavior for early P2P messages.
- `packages/open-bitcoin-network/src/peer.rs` - Pure peer lifecycle behavior,
  outbound handshake actions, `getheaders`, and `getdata` responses.
- `packages/open-bitcoin-network/src/error.rs` - Existing network error and
  disconnect reason taxonomy.
- `packages/open-bitcoin-node/src/sync/tcp.rs` - Effectful TCP peer transport
  boundary; useful only as an adapter reference.
- `scripts/run-live-mainnet-smoke.ts` - Existing v1.3 no-progress cause and
  operator action language for live evidence.
- `docs/operator/runtime-guide.md` - Operator-facing live-smoke and manual-peer
  command guidance that future docs must keep repo-local and copy-pasteable.

### Baseline and Parity Anchors

- `packages/bitcoin-knots/src/protocol.h` - Baseline protocol command and
  message-shape anchor.
- `packages/bitcoin-knots/src/net_processing.cpp` - Baseline peer-processing
  behavior anchor for early message ordering.
- `packages/bitcoin-knots/test/functional/p2p_handshake.py` - Baseline
  handshake behavior and rejection test anchor.
- `packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py` -
  Baseline initial `getheaders` behavior anchor.
- `docs/parity/catalog/p2p.md` - Existing P2P parity catalog.
- `docs/parity/index.json` - Auditable parity catalog data.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `PeerManager::add_outbound_peer` already emits the local outbound `version`
  message from pure state.
- `PeerManager::handle_message` already handles remote `version`, `verack`,
  `sendheaders`, `wtxidrelay`, `getheaders`, `headers`, `inv`, and `getdata`
  without direct I/O.
- `WireNetworkMessage::command_name` provides stable command names for report
  steps.
- `ParsedNetworkMessage::decode_wire` and `WireNetworkMessage::encode_wire`
  already expose malformed payload and network magic failures through pure
  decoding paths.

### Established Patterns

- Pure network behavior belongs in `packages/open-bitcoin-network`; TCP,
  resolver, runtime, and durable state effects stay in `open-bitcoin-node`.
- First-party source files require parity breadcrumb blocks, including explicit
  `none` only when no defensible Knots anchor exists.
- Tests should be deterministic and should use Arrange, Act, Assert comments
  for non-trivial unit behavior.

### Integration Points

- Add deterministic tests in or near `packages/open-bitcoin-network/src/peer`
  for the transcript/report core.
- Update parity docs if the phase adds new auditable compatibility evidence.
- Update operator docs only if a new operator-facing harness command is added.

</code_context>

<specifics>

## Specific Ideas

- Model failures as first-class compatibility outcomes rather than strings.
- Keep the report useful to both reviewers and future live-smoke integration:
  a single failed transcript should say what command failed, why it failed, and
  what an operator or Phase 55 implementation should do next.
- Prefer code paths that can later feed live smoke reports without making this
  phase depend on public network access.

</specifics>

<deferred>

## Deferred Ideas

- Public-peer handshake fixes and peer replacement behavior are Phase 55.
- Validated multi-batch header convergence is Phase 56.
- Block download/connect progress is Phase 57.
- Same-datadir restart and resume evidence is Phase 58.
- Support bundles, release boundaries, and v1.4 threat modeling are Phase 59.

</deferred>

---

*Phase: 54-peer-compatibility-baseline-and-diagnostic-harness*
*Context gathered: 2026-06-02*
