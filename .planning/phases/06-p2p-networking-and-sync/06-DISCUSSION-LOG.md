---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 06-2026-04-15T00-28-26
generated_at: 2026-04-15T00:28:26Z
---

# Phase 6: P2P Networking and Sync - Discussion Log

**Mode:** Yolo  
**Completed:** 2026-04-14

## Auto-Selected Gray Areas

1. Crate and subsystem boundary
2. Message scope for the first networking slice
3. Header-sync and block-download behavior
4. Transaction or block relay semantics
5. Verification strategy for hermetic parity coverage

## Synthesized Decisions

### 1. Crate and subsystem boundary
- Auto-selected: new pure-core `open-bitcoin-network` crate
- Rationale: matches the repo's per-subsystem crate pattern and keeps peer
  state out of the imperative shell

### 2. Message scope for the first networking slice
- Auto-selected: `version`, `verack`, `wtxidrelay`, `sendheaders`, `ping`,
  `pong`, `getheaders`, `headers`, `inv`, `getdata`, `tx`, `block`, and
  `notfound`
- Rationale: this covers the handshake, sync, and relay surface promised by
  Phase 6 without dragging in discovery or bandwidth-optimization protocols

### 3. Header-sync and block-download behavior
- Auto-selected: header-first sync with deterministic locator construction and
  explicit block requests after accepted headers
- Rationale: aligns with the roadmap and Knots' modern sync direction while
  staying practical for in-memory fixtures

### 4. Transaction or block relay semantics
- Auto-selected: preserve txid or wtxid negotiation, request blocks through
  `getheaders` or `getdata`, and feed inbound transactions through the managed
  mempool
- Rationale: keeps relay behavior observable and compatible with the existing
  mempool boundary

### 5. Verification strategy for hermetic parity coverage
- Auto-selected: in-memory multi-node fixtures that exchange encoded messages
  between managed nodes
- Rationale: proves the wire layer and adapter integration without bringing in
  actual sockets or external services

## Deferred Ideas Captured

- address relay and discovery
- encrypted transport
- compact-block or filtered-block protocols
- peer-eviction and advanced resource-governance policy

---

*Phase: 06-p2p-networking-and-sync*  
*Discussion logged: 2026-04-14*
