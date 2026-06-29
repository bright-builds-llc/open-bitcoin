---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 99-2026-06-29T02-03-43
generated_at: 2026-06-29T02:03:43Z
---

# Phase 99: Peer Policy Structured Log Emission - Context

**Gathered:** 2026-06-29
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 99 closes the v1.9 audit tech debt for automatic sanitized `inbound_peer_policy` structured-log emission. The already completed Phase 96 runtime bridge projects peer-policy state into status, RPC, CLI, support, and reconnect-suppression evidence; this phase adds the missing production log-emission proof without remapping completed v1.9 requirements.

This phase is runtime-observability hardening. It must not expand inbound serving scope, transaction relay, compact block relay, mempool propagation, public inbound defaults, production service operation, or production full-node readiness claims.
</domain>

<decisions>
## Implementation Decisions

### Runtime Emission Boundary

- **D-01:** Emit peer-policy structured logs from `ManagedRpcContext`, the effectful adapter that owns the datadir-backed structured-log directory and can observe real peer-policy runtime mutations.
- **D-02:** Log the specific ban, discourage, unban, or misbehavior decision at the context mutation point instead of relying only on aggregate `latest_peer_policy_decision`; the aggregate projection is status-facing and may not be chronological when mixed decisions exist.
- **D-03:** Keep `record_latest_inbound_peer_policy_event_at` available in production code so reconnect-suppression paths and future runtime adapters are not forced through test-only callers.
- **D-04:** Preserve the pure network model. The pure `open-bitcoin-network` and `open-bitcoin-node` policy types may expose projection helpers, but they must not perform filesystem I/O.

### Sanitization And Cardinality

- **D-05:** Reuse `inbound_peer_policy_log_record` and the existing `redacted_peer_policy_field` sanitizer for all log records.
- **D-06:** Structured-log messages must remain low-cardinality: outcome, reason, label, source, and message only. They must not contain peer ids, raw endpoints, payload bytes, permission strings, credentials, cookies, raw config names, or long opaque identifiers.

### Verification

- **D-07:** Add focused Rust tests proving automatic context-level emission for ban, discourage, unban, and misbehavior decisions.
- **D-08:** Add a Phase 99 Bun checker and fixture test over a fixed corpus. The checker must prove production-callable emission, sanitizer reuse, no raw leakage markers, verifier wiring, and unchanged no-claim boundaries.
- **D-09:** Wire the Phase 99 checker immediately after Phase 98 in `scripts/verify.sh`, in both visible and executable order.
- **D-10:** Final verification must include targeted Bun tests/checker, relevant Rust tests, the required pre-commit Rust commands, and full `bash scripts/verify.sh`.

### Folded Todos

No pending todos matched this phase.

### Agent Discretion

The implementation may choose helper names and exact test boundaries, provided the final behavior logs real runtime decisions automatically, keeps status/RPC/CLI/support/metrics behavior unchanged, and leaves v1.9 requirement traceability intact.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Workflow Rules

- `AGENTS.md` - repo-local GSD workflow, Rust verification, parity breadcrumb, and generated-artifact rules.
- `AGENTS.bright-builds.md` - Bright Builds architecture, verification, testing, and code-shape rules.
- `standards/core/architecture.md` - functional core / imperative shell boundary.
- `standards/core/testing.md` - focused Arrange/Act/Assert tests.
- `standards/core/verification.md` - repo-native verification contract.
- `standards/languages/rust.md` - Rust style and pre-commit requirements.
- `standards/languages/typescript-javascript.md` - Bun checker guidance.

### Phase Scope And Audit Evidence

- `.planning/ROADMAP.md` - Phase 99 goal and success criteria.
- `.planning/v1.9-MILESTONE-AUDIT.md` - TD-01 peer-policy log-emission tech debt.
- `.planning/phases/96-peer-policy-runtime-bridge/96-VERIFICATION.md` - existing runtime bridge evidence and residual structured-log caveat.

### Runtime And Checker Surfaces

- `packages/open-bitcoin-rpc/src/context/peer_policy.rs` - peer-policy structured-log append adapter.
- `packages/open-bitcoin-rpc/src/context/network.rs` - context network mutation and reconnect-suppression surface.
- `packages/open-bitcoin-rpc/src/context/tests.rs` - managed context log/status tests.
- `packages/open-bitcoin-rpc/src/inbound_listener.rs` - production inbound listener reconnect/resource event pattern.
- `packages/open-bitcoin-node/src/logging.rs` - structured-log source and sanitizer.
- `packages/open-bitcoin-node/src/network/inbound.rs` - managed peer-policy status projection.
- `packages/open-bitcoin-node/src/network/peer_policy.rs` - pure managed network peer-policy mutation API.
- `scripts/check-phase96-peer-policy-runtime-bridge.ts` - existing Phase 96 checker pattern.
- `scripts/check-phase98-traceability-reconciliation.ts` - latest fixed-corpus checker pattern.
- `scripts/verify.sh` - default verifier wiring contract.
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `ManagedRpcContext::record_inbound_resource_event` already shows the effectful pattern: project runtime event into managed state, build a sanitized record, append to the datadir log directory, and swallow write failures into an internal counter.
- `inbound_peer_policy_log_record` already creates sanitized `inbound_peer_policy` records with `redacted_peer_policy_field`.
- `ManagedPeerNetwork::record_peer_policy_ban`, `record_peer_policy_unban`, `record_peer_policy_discouragement`, and `record_peer_policy_misbehavior` already own the pure runtime mutations.
- Phase 96 tests already prove direct appending and status projection, but the latest-event appender is currently test-only.

### Integration Points

- Make peer-policy decision logging production-callable from `ManagedRpcContext` mutation methods.
- Add focused tests in `packages/open-bitcoin-rpc/src/context/tests.rs`.
- Add `scripts/check-phase99-peer-policy-structured-log-emission.ts` and `.test.ts`.
- Wire Phase 99 after Phase 98 in `scripts/verify.sh`.
- Create Phase 99 summary and verification artifacts under `.planning/phases/99-peer-policy-structured-log-emission/`.
</code_context>

<specifics>
## Specific Ideas

- Prefer logging specific decision events over rereading aggregate latest status after every mutation.
- If a helper must project from `BanDecision`, `UnbanDecision`, or `MisbehaviorDecision`, keep it side-effect-free and reuse the exact labels already exposed through status where appropriate.
- For explicit discouragement, it is acceptable for the log projection to use a dedicated `discouragement_active` or `discouragement_expired` label because this only affects the new structured-log evidence, not existing status fields.
- Keep the Phase 99 checker narrow and literal so it catches removal of production wiring without scanning the whole repository.
</specifics>

<deferred>
## Deferred Ideas

Transaction relay, compact block relay, mempool propagation, full address relay, public inbound serving by default, public-network CI, service-manager verification, signed packaging, hosted dashboards, GUI work, migration apply mode, production service operation, production-funds wallet operation, automatic support-bundle upload, and production full-node readiness remain outside Phase 99 and outside v1.9.
</deferred>

*Phase: 99-peer-policy-structured-log-emission*
*Context gathered: 2026-06-29*
