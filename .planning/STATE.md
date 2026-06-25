---
gsd_state_version: 1.0
milestone: v1.9
milestone_name: Inbound Peer Serving and Network Participation Boundary
status: executing
stopped_at: Completed 91-05-PLAN.md
last_updated: "2026-06-25T17:42:38.711Z"
last_activity: 2026-06-25
progress:
  total_phases: 6
  completed_phases: 1
  total_plans: 20
  completed_plans: 15
  percent: 75
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-06-25)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 91 — peer-permissions-and-connection-classes

## Current Position

Milestone: v1.9 Inbound Peer Serving and Network Participation Boundary
Phase: 91 (peer-permissions-and-connection-classes) — PLANNED
Plan: 6 of 10
Status: Ready to execute
Last activity: 2026-06-25

Progress: [----------] 0%

## Performance Metrics

**Velocity:**

- New milestone initialized with 6 planned phases and 28 scoped requirements.
- Prior milestone plans completed: 26 in v1.8.
- Prior milestone summary tasks counted: 49 in v1.8.

**By Phase:**

| Phase | Name | Requirements | Status |
|-------|------|--------------|--------|
| 90 | Inbound Listener and Admission Policy | INB-01, INB-02, INB-03, INB-04, INB-05 | Complete |
| 91 | Peer Permissions and Connection Classes | PERM-01, PERM-02, PERM-03, PERM-04 | Planned |
| 92 | Address Advertisement and Discovery Boundaries | ADDR-01, ADDR-02, ADDR-03, ADDR-04 | Pending |
| 93 | Eviction, Ban, and Misbehavior Policy | EVICT-01, EVICT-02, EVICT-03, EVICT-04 | Pending |
| 94 | DoS and Resource Governance | DOS-01, DOS-02, DOS-03, DOS-04, DOS-05 | Pending |
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
- [Phase 91]: Use listener remote_addr.ip() as the only runtime matching input for permission class resolution.
- [Phase 91]: Store only typed low-cardinality permission class/effect labels in managed admission evidence; do not store raw class names, endpoints, peer ids, or raw config strings.
- [Phase 91]: Expose permission evidence through openbitcoinnetworkstatus and the shared inbound status contract, while keeping getnetworkinfo free of permission fields.
- [Phase 91]: Add permission metrics as fixed MetricKind variants without dynamic labels or dimensions.

### Pending Todos

- Execute Phase 91 with `/gsd-execute-phase 91`.
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

Last session: 2026-06-25T17:42:38.709Z
Stopped at: Completed 91-05-PLAN.md
Resume file: None
