# Requirements: Open Bitcoin v1.9 Inbound Peer Serving and Network Participation Boundary

**Defined:** 2026-06-25
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v1.9 Requirements

Requirements for the active v1.9 milestone. Each maps to exactly one roadmap phase.

### Inbound Admission

- [ ] **INB-01**: Operators can enable inbound peer serving only through explicit config or CLI controls, with inbound serving disabled by default unless a later release boundary says otherwise.
- [ ] **INB-02**: The daemon can bind and listen on configured interfaces with deterministic preflight and diagnostic errors when disabled, unavailable, unsafe, or already in use.
- [ ] **INB-03**: The node admits inbound peers through typed connection records, handshake lifecycle state, duplicate/self-connection protections, and inbound/outbound counters.
- [ ] **INB-04**: The node enforces configurable inbound connection caps, reserved slots, and protected peer handling without starving the existing outbound sync workflow.
- [ ] **INB-05**: Operator status, metrics, logs, RPC-facing status, and support evidence distinguish inbound serving from outbound sync and expose admission and handshake outcomes.

### Peer Permissions

- [x] **PERM-01**: Operators can define permissioned peer classes from config using Knots-aligned permission concepts, connection direction boundaries, and explicit validation errors.
- [x] **PERM-02**: Permission rules affect only v1.9 in-scope privileges: admission protection, eviction immunity, address response policy, download serving policy, and diagnostics.
- [x] **PERM-03**: Relay, mempool, force-relay, and compact-block-style permissions can be rejected, deferred, or parsed as inactive without enabling transaction relay, compact block relay, or mempool propagation.
- [x] **PERM-04**: Permission effects are visible in status/support evidence without leaking secrets or hiding why a peer was admitted, protected, disconnected, discouraged, or banned.

### Address Advertisement

- [x] **ADDR-01**: The node can derive local listen address candidates and advertise only configured, reachable, and privacy-safe addresses according to scoped Knots parity rules.
- [x] **ADDR-02**: The node can answer inbound address requests within bounded cache, count, age, and permission rules without claiming full address-relay network participation.
- [x] **ADDR-03**: Learned peer addresses enter a typed address-management contract with routability, source, freshness, and persistence boundaries that can be verified deterministically.
- [x] **ADDR-04**: Documentation and release checks distinguish local listener advertisement, inbound `getaddr` response behavior, peer discovery, and full address relay.

### Eviction And Ban Policy

- [ ] **EVICT-01**: The node scores inbound peers for eviction using deterministic, Knots-anchored criteria such as connection class, handshake progress, netgroup or diversity, activity, and permissions.
- [ ] **EVICT-02**: The node can disconnect or evict peers when admission caps or abuse policy require it, preserving stable reason codes and support evidence.
- [ ] **EVICT-03**: The node can discourage or ban peers through durable policy with expiry, address/subnet scope, manual unban, and no hidden broad-ban behavior.
- [ ] **EVICT-04**: Misbehavior accounting maps protocol violations to bounded responses without incorrectly banning or evicting permissioned peers.

### DoS And Resource Governance

- [ ] **DOS-01**: Inbound sessions enforce network magic, message header, payload size, malformed message, and unsupported command limits before allocating unbounded memory.
- [ ] **DOS-02**: Inbound sessions enforce per-peer and aggregate read/write queues, inventory/request bounds, header/block/transaction request caps, and backpressure behavior.
- [ ] **DOS-03**: The node limits connection churn, slow handshakes, idle peers, repeated failures, and banned or discouraged reconnect attempts with deterministic synthetic tests.
- [ ] **DOS-04**: Resource pressure and abuse responses appear in metrics, structured logs, support bundles, and operator status with clear next actions.
- [ ] **DOS-05**: Default verification covers inbound DoS/resource policy deterministically and keeps public-network listener exposure outside `bash scripts/verify.sh`.

### Release Boundary

- [ ] **BOUND-01**: Release docs, parity docs, and deterministic checkers prohibit transaction relay, compact block relay, mempool propagation, production-node readiness, production-service, and public inbound default claims for v1.9.
- [ ] **BOUND-02**: v1.9 parity breadcrumbs and documentation cite Knots anchors for `net.cpp`, `net_processing.cpp`, `addrman.cpp`, `banman.cpp`, and `net_permissions.cpp`, or record intentional deviations.
- [ ] **BOUND-03**: Existing outbound sync, full-sync, soak, support-bundle, release-boundary, and production no-claim behavior remains non-regressed while inbound serving is added.
- [ ] **BOUND-04**: Operator UAT guidance includes repo-local Cargo and Bazel command forms for loopback or synthetic inbound review, not only an installed `open-bitcoin` alias.
- [ ] **BOUND-05**: Support bundles redact inbound peer addresses where needed while preserving enough admission, permission, eviction, ban, and resource evidence for diagnosis.
- [ ] **BOUND-06**: Requirements, roadmap, phase summaries, verification reports, and milestone audit artifacts map every v1.9 requirement exactly once.

## Future Requirements

Deferred to future milestones. Tracked but not in the active v1.9 roadmap.

### Relay And Production Participation

- **RELAY-FUTURE-01**: Node can relay validated transactions and mempool inventory with Knots-compatible policy and privacy boundaries.
- **RELAY-FUTURE-02**: Node can participate in compact block relay and block-relay optimization with parity evidence.
- **RELAY-FUTURE-03**: Node can claim broader production full-node readiness after support, uptime, packaging, service, network, wallet, and release evidence gates are satisfied.
- **ADDR-FUTURE-01**: Node can claim full address-relay network participation beyond bounded local advertisement and scoped `getaddr` response behavior.

### Operator And Distribution

- **OPS-FUTURE-01**: Node can expose public inbound serving by default after release, support, firewall, packaging, abuse, and production-readiness evidence gates are deliberately planned.
- **OPS-FUTURE-02**: Node can ship signed packaging and service-manager integration for production-style public listener operation.
- **OPS-FUTURE-03**: Node can run public-network listener checks in CI only if a later milestone deliberately changes the deterministic verification policy.

## Out of Scope

Explicitly excluded from v1.9 to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Transaction relay and mempool propagation | Requires separate relay policy, privacy, orphan/package, fee, and DoS parity work; planned for v2.0+ candidate scope. |
| Compact block relay | Requires block-relay-specific protocol and reconciliation work beyond inbound admission and basic serving boundaries. |
| Production full-node readiness claim | v1.8 defined gates before this claim; v1.9 expands inbound capability but does not satisfy all production gates. |
| Public inbound serving by default | Listener exposure remains opt-in until support, release, firewall, packaging, and production evidence explicitly expand the default. |
| Production-funds wallet operation | Wallet safety remains separate from inbound network participation. |
| Migration apply mode or destructive repair | Existing datadir and wallet mutation remains dry-run-first and outside inbound serving scope. |
| GUI, hosted dashboards, signed packaging, and public-network CI | These are distribution and operations surfaces that need their own milestone plans. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| INB-01 | Phase 90 | Pending |
| INB-02 | Phase 90 | Pending |
| INB-03 | Phase 90 | Pending |
| INB-04 | Phase 90 | Pending |
| INB-05 | Phase 90 | Pending |
| PERM-01 | Phase 91 | Complete |
| PERM-02 | Phase 91 | Complete |
| PERM-03 | Phase 91 | Complete |
| PERM-04 | Phase 91 | Complete |
| ADDR-01 | Phase 92 | Complete |
| ADDR-02 | Phase 92 | Complete |
| ADDR-03 | Phase 92 | Complete |
| ADDR-04 | Phase 92 | Complete |
| EVICT-01 | Phase 93 | Pending |
| EVICT-02 | Phase 93 | Pending |
| EVICT-03 | Phase 93 | Pending |
| EVICT-04 | Phase 93 | Pending |
| DOS-01 | Phase 94 | Pending |
| DOS-02 | Phase 94 | Pending |
| DOS-03 | Phase 94 | Pending |
| DOS-04 | Phase 94 | Pending |
| DOS-05 | Phase 94 | Pending |
| BOUND-01 | Phase 95 | Pending |
| BOUND-02 | Phase 95 | Pending |
| BOUND-03 | Phase 95 | Pending |
| BOUND-04 | Phase 95 | Pending |
| BOUND-05 | Phase 95 | Pending |
| BOUND-06 | Phase 95 | Pending |

**Coverage:**
- v1.9 requirements: 28 total
- Mapped to phases: 28
- Unmapped: 0

---
*Requirements defined: 2026-06-25*
*Last updated: 2026-06-26 after Phase 92 completion*
