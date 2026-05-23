---
phase: 41
phase_name: "Security Analysis Audit and Follow-Up"
generated_by: gsd-execute-phase-inline
lifecycle_mode: yolo
phase_lifecycle_id: "41-2026-05-23T02-51-11"
generated_at: "2026-05-23T02:52:44.465Z"
status: passed
threats_open: 0
needs_phase_count: 0
---

# Phase 41 Security Analysis Audit

## Result

Passed. The reviewed planning security corpus has no open registered threats
and no security follow-up that must become a new implementation phase before
v1.2 archive.

Phase 41 did find an artifact-shape gap: active v1.2 Phases 35 through 39 do
not each have standalone `*-SECURITY.md` files. That is not treated as an open
implementation threat because their security-relevant claims are covered by
phase verification, residual-risk documentation, and this consolidated audit.
The only active v1.2 plan with an explicit STRIDE register is Phase 39 plan
`39-02`, and its mitigations are covered by deterministic tests, local
live-shape daemon evidence, and user-rerun live UAT.

## Inventory

| Corpus | Count | Result |
|--------|------:|--------|
| Tracked `*-SECURITY.md` files across active and archived planning directories | 25 | All report `status: verified` and `threats_open: 0`. |
| Active v1.2 standalone `*-SECURITY.md` files | 1 | Phase 40 security closeout is verified with `threats_open: 0`. |
| Active v1.2 plan files with `<threat_model>` or `## STRIDE Threat Register` | 1 | Phase 39 plan `39-02` declares six sync-control threats. |
| Active v1.2 summaries with `## Threat Flags` | 0 | No additional summary-level threat flags found. |
| Follow-up items classified as `needs-phase` | 0 | No new security phase required before v1.2 archive. |

## Evidence Commands

```bash
find .planning/phases .planning/milestones/v1.1-phases -name '*-SECURITY.md' -type f | sort | wc -l
find .planning/phases .planning/milestones/v1.1-phases -name '*-SECURITY.md' -type f -print0 | xargs -0 awk 'FNR==1{file=FILENAME; phase=""; status=""; open=""} /^phase:/{phase=$0} /^status:/{status=$0} /^threats_open:/{open=$0} /^---$/ && FNR>1{printf "%s | %s | %s | %s\n", file, phase, status, open; nextfile}' | sort
find .planning/phases/35-daemon-mainnet-sync-activation .planning/phases/36-mainnet-peer-discovery-and-outbound-lifecycle .planning/phases/37-header-first-mainnet-sync-integration .planning/phases/38-block-download-connect-and-restart-recovery .planning/phases/39-operator-sync-observability-and-control .planning/phases/40-live-mainnet-smoke-docs-and-parity-closeout -name '*-PLAN.md' -type f -print0 | xargs -0 rg -l '<threat_model>|## STRIDE Threat Register' | sort
find .planning/phases/35-daemon-mainnet-sync-activation .planning/phases/36-mainnet-peer-discovery-and-outbound-lifecycle .planning/phases/37-header-first-mainnet-sync-integration .planning/phases/38-block-download-connect-and-restart-recovery .planning/phases/39-operator-sync-observability-and-control .planning/phases/40-live-mainnet-smoke-docs-and-parity-closeout -name '*-SUMMARY.md' -type f -print0 | xargs -0 rg -l '## Threat Flags' | sort
rg -n "T-39-02-0[1-6]|sync_control|auth|fallback|Fjall|Locked|rpcpassword|Authorization|timeout" .planning/phases/39-operator-sync-observability-and-control/39-VERIFICATION.md packages/open-bitcoin-cli/src/operator/runtime/support.rs packages/open-bitcoin-rpc/src/context.rs packages/open-bitcoin-rpc/src/dispatch/node.rs packages/open-bitcoin-cli/tests/operator_binary.rs
```

## Active v1.2 Phase Disposition

| Phase | Security Inputs Reviewed | Disposition | Follow-Up |
|-------|--------------------------|-------------|-----------|
| 35 Daemon Mainnet Sync Activation | Plans, summaries, verification, docs residual risks | Closed by later v1.2 phases and bounded docs. Activation stays disabled by default and public-network behavior remains opt-in. | None. |
| 36 Mainnet Peer Discovery and Outbound Lifecycle | Plans, summaries, verification, resolver/peer lifecycle evidence | Closed. Resolver injection, typed peer outcomes, and deterministic peer failure coverage address the security-relevant peer lifecycle claims in scope. | None. |
| 37 Header-First Mainnet Sync Integration | Plans, summaries, verification, header validation evidence | Closed for v1.2 scope. Block download/connect residual risk was completed by Phase 38. Header-store chainwork fidelity remains a parity watch item, not an open security blocker for the current claim. | None. |
| 38 Block Download, Connect, and Restart Recovery | Plans, summaries, verification, restart/reorg evidence | Closed for v1.2 scope. Operator control residual risk was completed by Phase 39. Chainwork fidelity remains deferred parity evidence. | None. |
| 39 Operator Sync Observability and Control | Plans, explicit `39-02` STRIDE register, summaries, verification, UAT | Closed. The sync-control lock/auth threat model is mitigated or accepted as documented below. | None. |
| 40 Live Mainnet Smoke, Docs, and Parity Closeout | Plan, summary, verification, UAT, `40-SECURITY.md` | Closed. Verified `threats_open: 0`; live smoke remains opt-in and bounded. | None. |

## Phase 39 STRIDE Revisit

| Threat | Original Disposition | Audit Result | Evidence |
|--------|----------------------|--------------|----------|
| T-39-02-01 Spoofing: live RPC client | mitigate | Closed. Reachable daemon auth failures are terminal and do not fall back to direct-store mutation. | `sync_control_auth_failure_does_not_fallback_to_store`; `support.rs` keeps Authorization scoped to RPC calls. |
| T-39-02-02 Tampering: pause/resume RPC handlers | mitigate | Closed. Node-scoped RPC handlers route mutations through daemon sync control. | `open_bitcoin_sync_rpc_control_updates_daemon_runtime_metadata`; `dispatch/node.rs`; `context.rs`. |
| T-39-02-03 Repudiation: operator pause/resume control | accept | Accepted as planned. Phase 39 persists durable sync control state and reports it through status; no additional audit-log requirement was scoped. | Phase 39 verification and UAT. |
| T-39-02-04 Information Disclosure: CLI/RPC errors | mitigate | Closed. Audit found no evidence path requiring secrets in user-facing output; tests cover auth failure behavior. | `sync_control_auth_failure_does_not_fallback_to_store`; redacted operator error path. |
| T-39-02-05 Denial of Service: sync worker control channel | mitigate | Closed. Final implementation uses daemon store-backed control and finite RPC/client timeouts so sync control does not hang behind the busy worker path. | Phase 39 timeout follow-up verification; local live-shape daemon check. |
| T-39-02-06 Elevation of Privilege: offline fallback | mitigate | Closed. Fallback is limited to stopped/unreachable daemon cases; reachable daemon failures are terminal. | Locked-store and auth-failure operator-binary regressions. |

## Accepted And Deferred Risks

| Risk | Classification | Rationale |
|------|----------------|-----------|
| Phase 39 pause/resume audit logging beyond durable state/status | accepted | Explicitly accepted in `39-02-PLAN.md`; durable state, structured status, and verification evidence are sufficient for this v1.2 control surface. |
| Production-node readiness, production-funds wallet use, inbound serving, address relay, transaction relay, and packaged-service hardening | deferred | These remain outside the v1.2 shipped claim in `.planning/REQUIREMENTS.md`, README, operator docs, and parity closeout docs. |
| Header-chain cumulative work fidelity beyond the current header-store model | deferred parity watch | Mentioned in Phase 37/38 residual risks. It does not contradict the current operator-ready IBD review claim and should be scoped explicitly if later parity evidence requires it. |
| Per-phase standalone `*-SECURITY.md` files for Phases 35 through 39 | accepted artifact gap | This consolidated Phase 41 audit records the v1.2 security revisit. Future milestones may choose per-phase security files earlier, but no open threat is hidden by the current gap. |

## Follow-Up Scope Decision

No new security implementation phase is required before v1.2 archive.

If future work expands the shipped claim into production-node operation,
production-funds wallet use, inbound peer serving, address relay, mempool relay,
or packaged-service hardening, that future milestone must create a new threat
model for those surfaces instead of relying on this v1.2 closeout audit.
