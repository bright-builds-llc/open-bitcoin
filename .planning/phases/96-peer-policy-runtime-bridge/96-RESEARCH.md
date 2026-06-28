---
generated_by: gsd-phase-researcher
phase: 96-peer-policy-runtime-bridge
generated_at: 2026-06-28T02:43:32.273Z
status: complete
requirements: [EVICT-03, EVICT-04, DOS-03]
---

# Phase 96 Research: Peer Policy Runtime Bridge

## Research Complete

Phase 96 should close the v1.9 audit gap by wiring existing Phase 93 peer-policy domain types into live managed runtime state, then using that state for scoped reconnect suppression and shared bounded evidence.

The key finding is that the project already has most public evidence and pure policy shapes. The gap is not field design; it is runtime state and scoped lookup.

## Phase Goal

Connect durable ban, unban, discourage, and misbehavior policy decisions into:

- live managed runtime state,
- scoped reconnect suppression for connecting remotes,
- status/RPC/CLI/support/log evidence,
- deterministic local verification,
- no public banlist, relay, mempool, compact-block, public-default, or production-readiness claim expansion.

## Gap Evidence

`.planning/v1.9-MILESTONE-AUDIT.md` reports `INT-01-peer-policy-runtime-bridge` and `FLOW-01-peer-policy-to-runtime`.

The concrete code gaps are:

- `packages/open-bitcoin-node/src/network.rs` has `ManagedPeerNetwork::peer_policy_info()`, but it calls `ManagedPeerPolicyInfo::from_policy_decisions(eviction_count, Some(eviction), &[], &[], &[])`.
- `packages/open-bitcoin-rpc/src/context/network.rs` has `ManagedRpcContext::reconnect_suppression_input_for_remote_addr(remote_addr, now_unix_seconds)`, but it discards both inputs and derives `banned`/`discouraged` from aggregate peer-policy counters.
- `packages/open-bitcoin-network/src/peer_policy.rs` has typed `PeerBanBook`, `BanScope`, `BanDecision`, `UnbanDecision`, `MisbehaviorDecision`, and `MisbehaviorPolicy`, but no live runtime state API that can answer "does this remote IP match an active ban or discourage decision now?"
- `packages/open-bitcoin-node/src/status/inbound.rs` already exposes bounded peer-policy fields, so most renderer/schema work should be projection wiring rather than new public surface design.

## Recommended Architecture

Use a pure peer-policy state in `open-bitcoin-network`, reached through `PeerManager` or a sibling pure module. Keep `open-bitcoin-node` and `open-bitcoin-rpc` as shell adapters that inject time, call scoped pure queries, and project bounded events.

Suggested shape:

- Add a pure state type such as `PeerPolicyRuntimeState` or `PeerPolicyState`.
- Store or wrap:
  - `PeerBanBook`,
  - explicit discouraged address/subnet state,
  - bounded latest ban/unban/misbehavior decisions,
  - misbehavior scores or observations if needed for runtime decisions.
- Add pure query methods:
  - `maybe_ban_for_ip(ip: IpAddr, now_unix_seconds: i64) -> Option<BanDecision>`,
  - `maybe_discouragement_for_ip(ip: IpAddr, now_unix_seconds: i64) -> Option<...>`,
  - `reconnect_suppression_for_ip(ip: IpAddr, now_unix_seconds: i64) -> ReconnectSuppressionInput`,
  - a bounded projection method returning slices or summary inputs for `ManagedPeerPolicyInfo::from_policy_decisions`.
- Keep exact address and subnet matching explicit. Do not let an aggregate ban count suppress unrelated addresses.
- Keep expiry deterministic through injected timestamps.

Avoid these approaches for Phase 96:

- Event-history-only bridge: useful for evidence but not authoritative enough for scoped reconnect checks.
- Direct durable-store lookup in the accept loop: risks I/O and lock handling in runtime listener code before a pure scoped model exists.
- Admission-policy integration as the primary fix: mixes ban/discourage policy into Phase 90 admission cap concepts and can blur evidence semantics.
- Full durable event ledger/public banlist: too large and too easy to overclaim for this gap-closure phase.

## Implementation Seams

### Pure Network Policy

Primary files:

- `packages/open-bitcoin-network/src/peer_policy.rs`
- `packages/open-bitcoin-network/src/peer.rs`
- `packages/open-bitcoin-network/src/resource.rs`
- `packages/open-bitcoin-network/src/peer_policy/tests.rs`
- `packages/open-bitcoin-network/src/peer/tests.rs`

Planner should consider:

- Extend `BanScope` with IP matching helpers for address and subnet scope.
- Extend `PeerBanBook` or add a sibling state type to support scoped lookup by `IpAddr`.
- Add explicit discourage state if no current type preserves it separately from `MisbehaviorResponse::Discourage`.
- Preserve `MisbehaviorPolicy` as pure data-in/data-out.
- Expose only bounded state/projection APIs from `PeerManager`.

### Managed Runtime Projection

Primary files:

- `packages/open-bitcoin-node/src/network.rs`
- `packages/open-bitcoin-node/src/network/inbound.rs`
- `packages/open-bitcoin-node/src/status/inbound.rs`
- `packages/open-bitcoin-node/src/network/tests.rs`

Planner should consider:

- Add `ManagedPeerNetwork` methods that record ban, unban, discourage, and misbehavior decisions through pure policy state.
- Change `peer_policy_info()` so it projects real decisions instead of empty slices.
- Keep `ManagedPeerPolicyInfo::from_policy_decisions` if possible; it already maps decisions to low-cardinality counters and latest safe event labels.
- Extend the shared status contract only if a required low-cardinality field is truly missing.

### RPC Listener Runtime

Primary files:

- `packages/open-bitcoin-rpc/src/context/network.rs`
- `packages/open-bitcoin-rpc/src/inbound_listener.rs`
- `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs`
- `packages/open-bitcoin-rpc/src/context/tests.rs`

Planner should consider:

- Replace `_ = (remote_addr, now_unix_seconds)` with scoped lookup using `remote_addr.ip()` and `now_unix_seconds`.
- Preserve the existing Phase 94 `ResourceGovernancePolicy::decide_reconnect` labels:
  - `reconnect_suppressed_banned`,
  - `reconnect_suppressed_discouraged`.
- Add tests for:
  - matching active address ban suppresses reconnect,
  - matching subnet ban suppresses reconnect,
  - expired ban does not suppress reconnect,
  - non-matching active ban does not suppress reconnect,
  - discouraged remote suppresses reconnect with discouraged label,
  - protected/no-action evidence is visible but does not hide observations.

### Operator Evidence And Docs

Primary files:

- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs`
- `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs`
- `packages/open-bitcoin-cli/src/operator/support/redaction.rs`
- `docs/architecture/status-snapshot.md`
- `docs/architecture/operator-observability.md`
- `docs/operator/runtime-guide.md`
- `docs/parity/catalog/p2p.md`
- `docs/parity/index.json`
- `docs/parity/source-breadcrumbs.json`

Planner should consider:

- Prefer no renderer changes if existing shared status fields can carry the bridge evidence.
- If support/status text changes, keep it bounded to aggregate counters and latest safe events.
- Do not expose raw endpoint tables, raw ban scopes, peer IDs, permission class names, raw config strings, credentials, or payload material.
- If first-party Rust source/test files are added, add parity breadcrumb blocks and `docs/parity/source-breadcrumbs.json` entries.

### Deterministic Checker

Primary files:

- `scripts/verify.sh`
- `scripts/check-phase93-peer-policy.ts`
- `scripts/check-phase94-dos-resource-governance.ts`
- `scripts/check-phase95-network-boundary.ts`

Planner should include a Phase 96 checker if implementation/docs introduce a new closeout surface. Useful assertions:

- `ManagedPeerNetwork::peer_policy_info()` no longer passes empty slices for runtime ban/misbehavior/unban projection.
- `reconnect_suppression_input_for_remote_addr` uses `remote_addr.ip()` and `now_unix_seconds`.
- no aggregate-only `active_bans > 0` or `discouraged_peers > 0` suppression is present in reconnect input code.
- Phase 96 docs and parity roots use scoped runtime bridge wording, not public banlist or production participation wording.
- `scripts/verify.sh` runs any Phase 96 checker after Phase 95 if added, or in the phase-appropriate location agreed by existing verifier ordering.

## Verification Strategy

Minimum recommended verification for phase execution:

- Pure network tests:
  - `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer_policy`
  - or targeted module tests covering scoped ban/discourage lookup.
- Managed node tests:
  - `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node managed_peer_policy`
  - or targeted tests covering non-empty `ManagedPeerPolicyInfo` projection.
- RPC listener/context tests:
  - `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_listener reconnect`
  - or targeted tests covering scoped reconnect suppression.
- Checker tests if a Phase 96 checker is added:
  - `bun test scripts/check-phase96-*.test.ts`
  - `bun run scripts/check-phase96-*.ts`
- Full final verification:
  - `bash scripts/verify.sh`

If the known local Rust test binary dyld stall recurs, use the repo's established workaround evidence: compile/check/clippy plus focused non-running checks only when needed, and record the blocker explicitly in SUMMARY/VERIFICATION artifacts.

## Threat Model Notes

Plans must include `<threat_model>` blocks because security enforcement is enabled.

Likely threats:

- Hidden broad ban: one banned address suppresses unrelated inbound peers.
- Raw peer-policy leak: status/support/logs expose peer IDs, endpoints, ban scopes, config names, credentials, or payload material.
- Protected-peer bypass: `noban` or protected peer handling hides observations instead of producing protected-no-action evidence.
- Runtime split-brain: managed counters and reconnect suppression read different state.
- Claim creep: docs imply public banlist parity, public inbound default, transaction relay, compact block relay, mempool propagation, production service, or production full-node readiness.

Mitigations should be concrete in plan tasks and acceptance criteria.

## Planning Guidance

Recommended four-plan split:

1. **Pure peer-policy state and scoped matching**
   - Add scoped address/subnet ban lookup and explicit discourage state.
   - Unit-test matching, non-matching, expiry, unban, and misbehavior/protected paths.

2. **Managed runtime projection**
   - Wire pure policy state into `ManagedPeerNetwork`.
   - Replace empty decision-slice projection with actual bounded decisions.
   - Add node tests for counters/latest event.

3. **Reconnect suppression and operator evidence**
   - Replace aggregate-only reconnect suppression with scoped lookup by remote IP and timestamp.
   - Preserve shared status/support/log evidence and add only low-cardinality fields if necessary.
   - Add loopback-safe RPC/listener tests.

4. **Checker, docs, and final verification**
   - Add/update deterministic checker only where useful.
   - Update parity/docs/source breadcrumbs as needed.
   - Run targeted checks and full `bash scripts/verify.sh`.

## No-Claim Boundaries

Phase 96 must keep these surfaces out of scope:

- transaction relay,
- mempool propagation,
- compact block relay,
- full address relay,
- public inbound serving by default,
- public-network CI,
- production-service operation,
- production full-node readiness,
- public banlist parity,
- durable cross-restart ban ledger unless explicitly narrowed and documented.

## Open Risks For Planner

- File sizes around `peer.rs`, `network.rs`, `inbound.rs`, and renderers may be near local refactor triggers; prefer focused child modules for substantial new behavior.
- `phase_req_ids` was null in plan-phase init even though Phase 96 maps to EVICT-03, EVICT-04, and DOS-03. Plans should explicitly include these requirements in frontmatter.
- Existing status/support fields may already be sufficient. Avoid unnecessary schema churn unless tests prove a missing field.
- If checker additions touch TypeScript scripts, use Bun and keep logic in repo-owned `.ts` files rather than shell-embedded code.

## Conclusion

Plan Phase 96 as a runtime bridge, not a new public network feature. The robust path is pure scoped peer-policy state, thin managed projection, scoped reconnect suppression, shared sanitized evidence, and deterministic verification that rejects the two audit failures: empty policy projection and aggregate-only reconnect suppression.
