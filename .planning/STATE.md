---
gsd_state_version: 1.0
milestone: v1.9
milestone_name: Inbound Peer Serving and Network Participation Boundary
status: executing
stopped_at: Completed 94-05-PLAN.md
last_updated: "2026-06-26T21:49:48.880Z"
last_activity: 2026-06-26
progress:
  total_phases: 6
  completed_phases: 4
  total_plans: 40
  completed_plans: 37
  percent: 93
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-06-26)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 94 — DoS and Resource Governance

## Current Position

Milestone: v1.9 Inbound Peer Serving and Network Participation Boundary
Phase: 94 (DoS and Resource Governance) — EXECUTING
Plan: 6 of 8
Status: Ready to execute
Last activity: 2026-06-26

Progress: [#########-] 85%

## Performance Metrics

**Velocity:**

- New milestone initialized with 6 planned phases and 28 scoped requirements.
- Prior milestone plans completed: 26 in v1.8.
- Prior milestone summary tasks counted: 49 in v1.8.

**By Phase:**

| Phase | Name | Requirements | Status |
|-------|------|--------------|--------|
| 90 | Inbound Listener and Admission Policy | INB-01, INB-02, INB-03, INB-04, INB-05 | Complete |
| 91 | Peer Permissions and Connection Classes | PERM-01, PERM-02, PERM-03, PERM-04 | Complete |
| 92 | Address Advertisement and Discovery Boundaries | ADDR-01, ADDR-02, ADDR-03, ADDR-04 | Complete |
| 93 | Eviction, Ban, and Misbehavior Policy | EVICT-01, EVICT-02, EVICT-03, EVICT-04 | Complete |
| 94 | DoS and Resource Governance | DOS-01, DOS-02, DOS-03, DOS-04, DOS-05 | In Progress |
| 95 | Network Participation Evidence and Release Boundary | BOUND-01, BOUND-02, BOUND-03, BOUND-04, BOUND-05, BOUND-06 | Pending |

**Recent Trend:**

- v1.6 completed explicit opt-in mainnet full-sync completion evidence.
- v1.7 hardened multi-day full-sync soak, bounded resources, recovery diagnosis, progress guarantees, and support-bundle forensics.
- v1.8 defined production-readiness claim gates, support/update/runbook/service policies, release-readiness evidence, and deterministic no-claim guardrails.
- v1.9 now expands toward opt-in inbound peer serving while keeping transaction relay, compact blocks, mempool propagation, production-funds wallet use, migration apply mode, packaging, hosted dashboard, GUI, public-network CI, and production full-node readiness deferred.

| Phase 91-peer-permissions-and-connection-classes P01 | 27min | 2 tasks | 5 files |
| Phase 91 P02 | 34min | 2 tasks | 13 files |
| Phase 91 P03 | 16min | 2 tasks | 7 files |
| Phase 91 P04 | 14min | 2 tasks | 6 files |
| Phase 91 P05 | 13min | 2 tasks | 11 files |
| Phase 91 P06 | stalled verification | 2 tasks | 3 files |
| Phase 91 P07 | 25min | 2 tasks | 5 files |
| Phase 91 P08 | 11min | 2 tasks | 1 file |
| Phase 91 P09 | 4min | 3 tasks | 7 files |
| Phase 91 P10 | 27min | 3 tasks | 9 files |
| Phase 94 P01 | 31min | 2 tasks | 5 files |
| Phase 94 P02 | 29min | 2 tasks | 4 files |
| Phase 94 P03 | 71min | 3 tasks | 11 files |
| Phase 94 P04 | 27m | 2 tasks | 7 files |
| Phase 94 P05 | 65m 12s | 3 tasks | 20 files |

## Accumulated Context

### Decisions

Decisions are logged in `PROJECT.md` Key Decisions table. Recent decisions:

- [v1.9]: Structure the milestone as six continuation phases, starting at Phase 90 after completed v1.8 Phase 89.
- [v1.9]: Scope inbound serving to explicit opt-in listener/admission, peer permissions, address boundaries, eviction/ban policy, DoS/resource governance, and release-boundary evidence.
- [v1.9]: Keep transaction relay, compact block relay, mempool propagation, public inbound defaults, and production-readiness claims out of scope until later milestones deliberately plan them.
- [v1.8]: Keep production-readiness language guarded by evidence gates and deterministic no-claim checkers.
- [v1.7]: Keep multi-day public-network soak runs opt-in UAT evidence; default `bash scripts/verify.sh` must remain deterministic, public-network-free, service-manager-free, and free of wall-clock multi-day gates.
- [v1.6]: Continue keeping public-network full-sync and service checks opt-in UAT evidence unless a future phase deliberately changes the deterministic verification contract.
- [Phase 91-peer-permissions-and-connection-classes]: Treat relay, forcerelay, mempool, bloomfilter, and blockfilters as inactive effect labels in the Phase 91 network domain model.
- [Phase 91-peer-permissions-and-connection-classes]: Map only forceinbound-protected inbound classes to reserved admission capacity; ordinary and permissioned inbound stay ordinary slots.
- [Phase 91-peer-permissions-and-connection-classes]: Use literal IpAddr class matching and reject ranges, hostnames, and endpoint-shaped values at the class parser boundary.
- [Phase 91]: Use Open Bitcoin-owned JSONC and Open Bitcoin-prefixed CLI flags only; Knots whitelist and whitebind-style inputs remain rejected.
- [Phase 91]: CLI permission-class flags replace the JSONC class list as a complete override, preserving deterministic order.
- [Phase 91]: Carry the parsed PeerPermissionClassRegistry on InboundListenerConfig so later listener wiring can use the typed registry directly.
- [Phase 91]: Derive effective admission slot class from InboundPermissionDecision so protected inbound peers consume reserved capacity while ordinary and permissioned peers remain ordinary.
- [Phase 91]: Count permission effects as low-cardinality numeric observations on ManagedInboundAdmissionInfo, not by peer id, endpoint, or raw config name.
- [Phase 91]: Keep legacy add_inbound_peer compatibility records ordinary with empty permission evidence.
- [Phase 91]: Store the resolved PeerPermissionClassRegistry on ManagedRpcContext but omit it from Debug/status surfaces to avoid raw class-name leakage.
- [Phase 91]: Keep record_inbound_admission as an ordinary compatibility path and add record_inbound_admission_for_remote_addr for runtime listener use.
- [Phase 91]: Render operator status permission evidence from shared status fields only, with raw class names, raw permission strings, peer ids, and credentials kept out of output.
- [Phase 91]: Use listener remote_addr.ip() as the only runtime matching input for permission class resolution.
- [Phase 91]: Store only typed low-cardinality permission class/effect labels in managed admission evidence; do not store raw class names, endpoints, peer ids, or raw config strings.
- [Phase 91]: Expose permission evidence through openbitcoinnetworkstatus and the shared inbound status contract, while keeping getnetworkinfo free of permission fields.
- [Phase 91]: Add permission metrics as fixed MetricKind variants without dynamic labels or dimensions.
- [Phase 91]: Support bundles sanitize inbound permission evidence to bounded machine class/effect labels and redact raw class names, raw permission strings, peer ids, endpoints, and credential literals.
- [Phase 91]: Treat relay, forcerelay, mempool, bloomfilter, blockfilters, and all-expansion permission data as inactive labels that do not alter peer message handling, service bits, or compact-block behavior.
- [Phase 91]: Document permission-class UAT with repo-local Cargo and Bazel commands and register `v1-9-peer-permissions-connection-classes` as the PERM-01 through PERM-04 parity surface.
- [Phase 91]: Guard permission evidence, inactive/deferred labels, parity roots, source breadcrumbs, UAT commands, verifier order, support redaction, and no-claim language with a deterministic fixed-file checker.
- [Phase 92]: Scope local listener advertisement, legacy `getaddr` responses, and learned-address evidence without claiming full address relay or broader public-network discovery parity.
- [Phase 92]: Preserve aggregate learned-address rejection counts separately from bounded rejection samples so over-cap `addr` batches remain visible in managed status.
- [Phase 92]: Route empty-payload peer messages through shared trailing-payload validation so `verack`, `wtxidrelay`, `sendheaders`, and `getaddr` reject non-empty payloads consistently.
- [Phase 94]: Kept Phase 94 resource governance in a pure open-bitcoin-network module with no socket, runtime, or peer-manager side effects.
- [Phase 94]: Used existing codec and message decoding APIs rather than introducing a new wire parser.
- [Phase 94]: Preserved repo hook requirements by recording TDD RED locally and committing only verification-passing task states.
- [Phase 94]: Kept Phase 94 queue, request, timeout, churn, failure, and reconnect decisions as pure data-in/data-out policy.
- [Phase 94]: Treated inactive relay-like permission effects as evidence only, never as capacity multipliers.
- [Phase 94]: Exported the resource governance API from open-bitcoin-network so later runtime plans can consume the policy without private-module access.
- [Phase 94]: Preserved repo hook requirements by recording TDD RED locally and committing only verification-passing task states.
- [Phase 94]: Use existing Phase 93 managed peer-policy aggregate projection for reconnect suppression instead of adding listener-local ban maps.
- [Phase 94]: Place runtime resource accounting helpers in an inbound_listener child module to satisfy repo production file-length limits while keeping the root listener as the adapter orchestration surface.
- [Phase 94]: Preserve hook requirements by running TDD RED checks locally and committing only verification-passing task states.
- [Phase 94]: Map all request-policy non-accept decisions in PeerManager to DisconnectReason::ResourceLimit for stable peer-facing evidence.
- [Phase 94]: Keep request-cap logic in peer/inventory_state.rs so peer.rs stays below the production file-length gate.
- [Phase 94]: Project Phase 94 resource-governance events once through shared inbound status and reuse that contract for RPC/log evidence.
- [Phase 94]: Keep resource-governance metrics as fixed MetricKind variants with static CLI dashboard labels; do not add dynamic metric labels.
- [Phase 94]: Make managed RPC resource-event logging datadir-backed and bounded, with redaction for suspicious raw fields and a bounded write-failure count.

### Pending Todos

- Plan Phase 94 DoS and resource governance before execution.
- Keep v1.9 phase directories numbered 90+ under `.planning/phases/`; historical phase directories remain tracked because verifier scripts still depend on selected phase evidence.
- Keep repo-local Cargo and Bazel command forms in UAT guidance:
  - `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  - `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`
- Keep public-network listener exposure opt-in and outside default deterministic verification unless a future phase explicitly changes that contract.

### Blockers/Concerns

- No active milestone blockers are recorded.
- `.planning/phases/` still contains historical phase directories required by verifier scripts; new v1.9 phase directories should use Phase 90+ names to avoid collisions.
- Default local verification must remain deterministic; public-network inbound serving review should be opt-in UAT evidence unless deliberately changed.
- Existing outbound sync, full-sync, soak, support-bundle, and release-boundary behavior must not regress while adding inbound serving.
- Local generated Rust test binaries hang at dyld start before test execution; Plan 91-01 used cargo test --no-run, cargo check, build, clippy, and breadcrumb checks as verification evidence.

## Session Continuity

Last session: 2026-06-26T21:49:48.878Z
Stopped at: Completed 94-05-PLAN.md
Resume file: None
