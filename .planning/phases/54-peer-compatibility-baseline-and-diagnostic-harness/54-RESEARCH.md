---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 54-2026-06-02T20-31-26
generated_at: 2026-06-02T20:44:10Z
status: complete
---

# Phase 54: Peer Compatibility Baseline and Diagnostic Harness - Research

## Research Question

What needs to be known to plan a hermetic peer compatibility baseline and
diagnostic harness for Phase 54?

## Findings

### Existing Open Bitcoin Surfaces

- `packages/open-bitcoin-network/src/message.rs` already owns the pure wire
  message model for `version`, `verack`, `wtxidrelay`, `sendheaders`,
  `getheaders`, `headers`, `inv`, `getdata`, `notfound`, `tx`, and `block`.
- `WireNetworkMessage::command_name()` provides stable command names suitable
  for transcript steps and reports.
- `ParsedNetworkMessage::decode_wire()` already exposes malformed payload,
  bad checksum, unknown command, and envelope-size failures without requiring
  socket I/O.
- `packages/open-bitcoin-network/src/peer.rs` already owns pure peer lifecycle
  behavior. `PeerManager::add_outbound_peer()` emits outbound `version`;
  `handle_message()` responds to remote `version` with `wtxidrelay`, `verack`,
  and `sendheaders`; remote `verack` can trigger `getheaders`; accepted headers
  can trigger `getdata`.
- `packages/open-bitcoin-network/src/peer/tests.rs` already covers the happy
  outbound handshake path and inventory/header/block request paths. The phase
  should avoid duplicating those tests as the only deliverable.
- `packages/open-bitcoin-node/src/sync/tcp.rs` is the effectful TCP boundary.
  It should remain an adapter reference, not the default harness dependency.

### Knots Baseline Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` sends outbound `version`,
  handles remote `version`, sends `wtxidrelay` before `verack` when the common
  version supports it, records `verack`, rejects or disconnects some unsupported
  message-order cases, sends `getheaders`, and handles `getdata`.
- `packages/bitcoin-knots/test/functional/p2p_handshake.py` is the most direct
  functional baseline for `version` and `verack` handshake behavior.
- `packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py` is the
  most direct functional baseline for early `getheaders` behavior.
- `docs/parity/catalog/p2p.md` already claims hermetic encoded-message fixtures
  for handshake, initial sync, and relay. Phase 54 should update that entry only
  if the new harness changes the auditable P2P parity surface.

### Implementation Direction

- Add a small pure harness module in `open-bitcoin-network`, likely
  `src/compatibility.rs`, and export its report types from `lib.rs`.
- Represent a transcript as a sequence of scripted events such as local outbound
  connect, received message, received malformed wire bytes, timeout, and peer
  disconnect.
- Represent report output as deterministic data:
  - transcript id/name
  - ordered step results
  - observed inbound command or event
  - local outbound commands
  - diagnosis
  - useful progress flag
  - next action text
- Use an enum for COMPAT-04 outcomes:
  `VersionRejected`, `NetworkMismatch`, `ServiceBitMismatch`,
  `UnsupportedMessageOrder`, `Timeout`, `PeerDisconnect`,
  `MalformedPayload`, and `LocalConfigurationFailure`.
- Keep diagnosis strings and next-action text human-readable, but make tests
  assert enum variants and command sequences rather than only free-form text.
- Avoid adding CLI or daemon wiring unless planning shows it is necessary for
  the Phase 54 success criteria. A pure API plus tests and parity docs can
  satisfy the deterministic harness requirement with lower blast radius.

### Verification Architecture

- Unit tests in `open-bitcoin-network` should prove:
  - Knots-like outbound happy transcript produces `version`, then
    `wtxidrelay`, `verack`, `sendheaders`, then `getheaders`.
  - Duplicate remote `version` maps to a version rejection diagnosis.
  - Wrong network magic or malformed bytes map to distinct diagnostics.
  - Missing service bits map to `ServiceBitMismatch`.
  - `wtxidrelay` after `verack` maps to `UnsupportedMessageOrder`.
  - Timeout and peer disconnect scripted events map to their distinct outcomes.
  - Failed transcripts do not receive useful-progress credit.
- Existing repo verification remains `bash scripts/verify.sh`; do not add
  public-network checks to the default gate.
- Because a first-party Rust module is added under `packages/open-bitcoin-*`,
  update `docs/parity/source-breadcrumbs.json` so the new file has a valid
  breadcrumb group.

## Risks

- The current `PeerManager` records `wtxidrelay` without enforcing the Knots
  timing rule that it must arrive between `version` and `verack`; the harness
  can diagnose this without changing peer behavior yet.
- Adding too much daemon or CLI wiring in Phase 54 would blur into Phase 55.
- Free-form diagnosis strings alone would not satisfy COMPAT-04 because future
  phases need typed outcomes to skip/replace incompatible peers.

## Planning Recommendation

Use one implementation plan for a focused pure-core compatibility harness and
parity evidence update. Keep the file set small: `open-bitcoin-network` source
and tests, parity breadcrumbs, and P2P parity docs. Verification should run the
network crate tests first, then the full repo-native `bash scripts/verify.sh`.

## RESEARCH COMPLETE
