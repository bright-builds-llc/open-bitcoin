---
phase: 99-peer-policy-structured-log-emission
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 99-2026-06-29T02-03-43
generated_at: 2026-06-29T02:03:43Z
status: complete
---

# Phase 99 Research

## Findings

### Runtime Logging Pattern

`packages/open-bitcoin-rpc/src/context/resource_governance.rs` is the closest precedent. It records runtime resource-governance events through `ManagedRpcContext`, updates managed state, appends a sanitized structured log when a datadir log directory exists, and increments an internal write-failure counter instead of exposing filesystem append errors to operator workflows.

### Existing Peer-Policy Gap

`packages/open-bitcoin-rpc/src/context/peer_policy.rs` can append an explicit `InboundPeerPolicyEvent`, and tests prove the record shape. The helper that appends the latest status event is currently test-only, and peer-policy runtime mutation helpers in `ManagedRpcContext` are also test-only and cover only ban/discourage setup.

### Projection Subtlety

`ManagedPeerPolicyInfo::from_policy_decisions` builds an aggregate status projection from separate misbehavior, ban, and unban decision lists. That projection is stable for status evidence, but it is not a reliable chronological event stream after mixed mutations. Phase 99 should log the specific mutation decision at the context boundary.

### Checker Pattern

Phase 96 and Phase 98 checkers use fixed file lists, direct string assertions, and explicit verifier-order checks. Phase 99 should follow that pattern and avoid broad repository-wide prose scans.

## Implementation Guidance

- Keep side effects in `open-bitcoin-rpc`.
- Keep projection helpers side-effect-free.
- Reuse `inbound_peer_policy_log_record`.
- Add tests that read the actual structured log records from a temp datadir and assert raw markers are absent.
- Wire the checker after Phase 98 in both the visible command-order block and executable `run_step` list.

## Risks

| Risk | Mitigation |
| --- | --- |
| Logs duplicate status projection rather than specific decisions | Build/log `InboundPeerPolicyEvent` from the mutation result itself. |
| Sanitization weakens because tests check only happy-path labels | Include raw peer label, endpoint, credential, or cookie-like markers in fixture values and assert absence. |
| Verification adds expensive or public-network checks | Keep Phase 99 checker fixed-corpus and local-only. |
| Requirement traceability churn | Treat Phase 99 as optional cleanup with `requirements: []`. |
