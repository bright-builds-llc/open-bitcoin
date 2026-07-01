---
gsd_state_version: 1.0
milestone: v2.0
milestone_name: Transaction Relay and Mempool Participation Boundary
status: executing
stopped_at: Completed 105-01-PLAN.md
last_updated: "2026-07-01T23:35:49.363Z"
last_activity: 2026-07-01
progress:
  total_phases: 7
  completed_phases: 5
  total_plans: 22
  completed_plans: 19
  percent: 86
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-06-29)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 105 — Operator, RPC, Metrics, Logs, and Support Evidence

## Current Position

Milestone: v2.0 Transaction Relay and Mempool Participation Boundary
Phase: 105 (Operator, RPC, Metrics, Logs, and Support Evidence) — EXECUTING
Plan: 2 of 4
Status: Executing Phase 105
Last activity: 2026-07-01

Progress: [#########-] 86% milestone plan progress; Phase 105 plan 2 of 4 is ready to execute

## Performance Metrics

**Velocity:**

- Current milestone initialized with 7 planned phases and 32 scoped requirements.
- Prior milestone plans completed: 26 in v1.8.
- Prior milestone summary tasks counted: 49 in v1.8.

**By Phase:**

| Phase | Name | Requirements | Status |
|-------|------|--------------|--------|
| 100 | Relay Activation Boundary and Permission Semantics | ACT-01, ACT-02, ACT-03, ACT-04 | Complete |
| 101 | Transaction Inventory Identity and Download Scheduling | INV-01, INV-02, INV-03, INV-04, DL-01, DL-02 | Complete |
| 102 | Orphan Handling and Admission Outcome Bridge | DL-03, DL-04, DL-05, MEM-01, MEM-02 | Complete |
| 103 | Mempool Chainstate Lifecycle and Durable Recovery | MEM-03, MEM-04, MEM-05, MEM-06 | Complete |
| 104 | Relay Serving, Fanout, and Rebroadcast Policy | REL-01, REL-02, REL-03, REL-04 | Complete |
| 105 | Operator, RPC, Metrics, Logs, and Support Evidence | OBS-01, OBS-02, OBS-03, OBS-04 | Pending |
| 106 | Parity Traceability, UAT, and Release Boundary Guardrails | BOUND-01, BOUND-02, BOUND-03, BOUND-04, BOUND-05 | Pending |

**Recent Trend:**

- v1.6 completed explicit opt-in mainnet full-sync completion evidence.
- v1.7 hardened multi-day full-sync soak, bounded resources, recovery diagnosis, progress guarantees, and support-bundle forensics.
- v1.8 defined production-readiness claim gates, support/update/runbook/service policies, release-readiness evidence, and deterministic no-claim guardrails.
- v1.9 expanded toward opt-in inbound peer serving while keeping transaction relay, compact blocks, mempool propagation, production-funds wallet use, migration apply mode, packaging, hosted dashboard, GUI, public-network CI, and production full-node readiness deferred.
- Phase 90 remains historical implementation evidence for INB-01 through INB-04 while Phase 98 is canonical closure.
- Phase 98 canonical ownership remains archived for INB-01, INB-02, INB-03, INB-04, BOUND-06.
- Phase 98 preserves the no-claim boundary for transaction relay, compact block relay, mempool propagation, public inbound defaults, production service operation, and production full-node readiness.
- Phase 99 closed optional audit tech debt for automatic sanitized `inbound_peer_policy` structured-log emission without remapping completed v1.9 requirements.
- v2.0 is now planned as 7 phases and 32 requirements covering scoped transaction relay and mempool participation while keeping compact block relay, public relay defaults, production service operation, production-funds wallet use, public-network CI, and production full-node readiness deferred.
- Phase 100 Plan 100-01 added scoped relay permission-effect labels and a pure default-off relay activation eligibility policy without changing peer socket, mempool, or service-bit behavior.
- Phase 100 Plan 100-02 added default-off Open Bitcoin JSONC/CLI relay activation config and typed runtime wiring without changing peer socket, mempool, service-bit, or public status behavior.
- Phase 100 Plan 100-03 documented the relay activation boundary, registered `v2-0-relay-activation-boundary`, added the deterministic no-claim checker, wired it after Phase 99 in `bash scripts/verify.sh`, and recorded passed verification.
- Phase 101 completed typed txid/wtxid transaction relay identity, bounded download scheduling, PeerManager and managed-network integration, parity roots, deterministic checker coverage, and passed verification.
- Phase 102 completed bounded orphan handling, the shared mempool admission outcome bridge, clean code review, deterministic checker hardening, and passed verification.
- Phase 103 completed mempool pressure truth, block-connect cleanup, bounded reorg reconsideration, durable mempool snapshot recovery, parity roots, deterministic checker coverage, and passed verification.
- Phase 104 completed relay serving classification, managed `getdata` serving, fanout queues, local submission relay evidence, explicit `rebroadcast_deferred` evidence, parity roots, deterministic checker coverage, and passed verification.

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
| Phase 94 P06 | 22m 44s | 2 tasks | 5 files |
| Phase 94 P07 | 21m 14s | 2 tasks | 7 files |
| Phase 94 P08 | 30m 15s | 2 tasks | 5 files |
| Phase 95 P01 | 13m 04s | 2 tasks | 3 files |
| Phase 95 P02 | 17m 47s | 2 tasks | 5 files |
| Phase 95 P03 | 32m20s | 2 tasks | 5 files |
| Phase 95 P04 | 30m07s | 3 tasks | 5 files |
| Phase 98 P01 | 11m47s | 2 tasks | 4 files |
| Phase 98 P02 | 14m52s | 2 tasks | 10 files |
| Phase 98 P03 | 27m31s | 2 tasks | 8 files |
| Phase 100 P01 | 60m | 2 tasks | 11 files |
| Phase 100 P02 | 25m | 2 tasks | 6 files |
| Phase 100 P03 | 25m | 3 tasks | 13 files |
| Phase 102 P01 | 4626 | 2 tasks | 9 files |
| Phase 102 P02 | 1h 35m | 2 tasks | 14 files |
| Phase 102 P03 | 50m 54s | 2 tasks | 9 files |
| Phase 102 P04 | 40m 19s | 3 tasks | 9 files |
| Phase 105 P01 | 1h 16m | 3 tasks | 16 files |

## Accumulated Context

### Roadmap Evolution

- Phase 99 completed: Peer Policy Structured Log Emission.
- v1.9 archived to `.planning/milestones/v1.9-ROADMAP.md`, `.planning/milestones/v1.9-REQUIREMENTS.md`, and `.planning/milestones/v1.9-MILESTONE-AUDIT.md`.
- v2.0 started as Transaction Relay and Mempool Participation Boundary, continuing phase numbering after Phase 99.
- v2.0 requirements were defined in `.planning/REQUIREMENTS.md` and mapped to Phase 100 through Phase 106 in `.planning/ROADMAP.md`.

### Decisions

Decisions are logged in `PROJECT.md` Key Decisions table. Recent decisions:

- [v1.9]: Structure the milestone as six continuation phases, starting at Phase 90 after completed v1.8 Phase 89.
- [v1.9]: Scope inbound serving to explicit opt-in listener/admission, peer permissions, address boundaries, eviction/ban policy, DoS/resource governance, and release-boundary evidence.
- [v1.9]: Keep transaction relay, compact block relay, mempool propagation, public inbound defaults, and production-readiness claims out of scope until later milestones deliberately plan them.
- [v2.0]: Scope transaction relay and mempool participation to explicit activation, txid/wtxid inventory, bounded download/orphan handling, mempool lifecycle, relay serving/fanout, operator evidence, and release guardrails.
- [v2.0]: Keep compact block relay, bloom/filter serving, package relay, public relay defaults, public-network CI, production service operation, production full-node readiness, and production-funds wallet use deferred.
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
- [Phase 94]: Render Phase 94 resource-governance status and support output from shared InboundPeerServingStatus fields only.
- [Phase 94]: Keep Phase 94 support guidance bounded to evidence review and avoid public exposure, relay, raw peer, payload, permission, credential, or production-readiness claims.
- [Phase 94]: Document Phase 94 operator review as bounded loopback/regtest UAT with exact repo-local Cargo and Bazel command forms.
- [Phase 94]: Register v1-9-dos-resource-governance as the DOS-01 through DOS-05 parity surface with explicit Knots anchors.
- [Phase 94]: Preserve the Phase 94 no-claim boundary sentence across parity docs and index JSON.
- [Phase 94]: Validate actual ManagedRpcContext structured-log append wiring, not only helper projection strings.
- [Phase 94]: Keep Phase 94 verification local/static and wired immediately after Phase 93 in the default verifier.
- [Phase 95]: Redact only resource-governance decision fields that contain raw peer, endpoint, payload, permission, config, credential, cookie, or secret markers.
- [Phase 95]: Preserve safe Phase 94 labels such as invalid_checksum, payload_rejected, and source_inbound_resource_governance when they do not contain raw material.
- [Phase 95]: Keep resource-governance redaction in support_status_for_bundle so both support JSON and Markdown consume the sanitized status snapshot.
- [Phase 95]: Keep Phase 95 closeout evidence inside existing parity roots instead of introducing a separate release manifest.
- [Phase 95]: Document Phase 95 aggregate checker paths as the next deterministic gate owned by Plan 04.
- [Phase 95]: Bounded v1.9 inbound evidence is documented as opt-in UAT, not broad relay or public-default support.
- [Phase 95]: Legacy Phase 82/87 guardrail literals remain present while newer v1.9 wording points at the closeout roots.
- [Phase 95]: Runtime guide no-claim wording uses production-service vocabulary to satisfy service-lifecycle guardrails.
- [Phase 95]: Phase 95 verification is a static release-boundary checker over a fixed corpus, not a runtime public-network or service-manager gate.
- [Phase 95]: Validate both visible VERIFY_COMMAND_ORDER text and executable run_step order so documentation-only verifier wiring cannot pass.
- [Phase 95]: Check support redaction evidence through sanitizer, test, and safeguard identifiers rather than raw support-bundle material.
- [Phase 98]: Keep INB-01 through INB-04 and BOUND-06 pending until Phase 98 final verification exists.
- [Phase 98]: Preserve Phase 90 as historical implementation evidence while Phase 98 owns canonical closure for INB-01 through INB-04.
- [Phase 98]: Keep the Phase 98 checker unwired from default verification until Plan 98-03 creates 98-VERIFICATION.md.
- [Phase 98]: Preserve docs/parity/checklist.md and docs/parity/index.json as evidence roots because they do not make current canonical ownership claims.
- [Phase 98]: Record TDD RED locally but commit only passing task states because repo hooks run the full verifier.
- [Phase 98]: Wire Phase 98 immediately after Phase 97 in both visible and executable verifier order.
- [Phase 98]: Keep INT-04 and FLOW-04 as residual non-blocking observability caveats while closing INT-03 and FLOW-03.
- [Phase 98]: Preserve Phase 95 checker-compatible roadmap coverage wording while adding final 28/28 complete traceability wording.
- [Phase 102]: Preserved accept_transaction and added accept_transaction_outcome as the typed admission outcome bridge.
- [Phase 102]: Kept admission mutation snapshots test-only instead of widening production mempool internals.
- [Phase 102]: Bounded orphan staging remains pure network state and returns typed actions instead of mutating mempool or socket state.
- [Phase 102]: Orphan parent requests reuse Phase 101 transaction download scheduler caps, duplicate suppression, and local-fact suppression.
- [Phase 102]: PeerManager request routing lives behind the inventory-state extension so peer.rs stays under the repo file-length guard.
- [Phase 102]: Preserved ManagedPeerNetwork::submit_local_transaction -> AdmissionResult and added submit_local_transaction_outcome for outcome-aware local callers.
- [Phase 102]: Kept peer transaction mempool and orphan mutation in the managed node admission bridge after the Phase 101 download boundary.
- [Phase 102]: Translated missing-parent requests through the existing transaction download scheduler/action path instead of direct socket writes.
- [Phase 102]: Ran orphanage peer cleanup from managed disconnect cleanup alongside transaction request cleanup.
- [Phase 102]: Validated managed disconnect orphan cleanup through network/action_translation.rs because disconnect_peer_at is implemented in the split module.
- [Phase 102]: Included Phase 102 checker, checker tests, action translation, and orphanage case tests as parity evidence roots so docs match the exact guarded files.
- [Phase 102]: Wired the Phase 102 checker immediately after Phase 101 in the default verifier and before pure-core checks.
- [Phase 105]: Relay evidence is represented as typed implemented, unavailable, deferred, or intentionally_different fields with stable reasons for non-implemented states. — Downstream RPC, CLI, metrics, logs, and support surfaces need one truthful sanitized contract.
- [Phase 105]: Relay fanout, serving, and local submission records collapse to fixed counters before reaching RPC or operator-facing status. — This prevents transaction, peer, endpoint, permission, and free-form reason material from entering serialized support surfaces.
- [Phase 105]: Baseline-compatible RPC methods retain their existing response shapes; Open Bitcoin-specific network status is the truth surface for relay evidence. — Phase 105 must expose operator evidence without implying public relay readiness or changing Knots-compatible RPC surfaces.

### Pending Todos

- Discuss and plan Phase 105 for operator, RPC, metrics, logs, and support evidence over the implemented relay serving/fanout state.
- Keep v1.9 phase directories numbered 90+ under `.planning/phases/`; historical phase directories remain tracked because verifier scripts still depend on selected phase evidence.
- Keep repo-local Cargo and Bazel command forms in UAT guidance:
  - `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  - `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`
- Keep public-network relay exposure opt-in and outside default deterministic verification unless a future phase explicitly changes that contract.

### Blockers/Concerns

- No active milestone blockers are recorded.
- `.planning/phases/` still contains historical phase directories required by verifier scripts; new v2.0 phase directories should use Phase 100+ names to avoid collisions.
- Default local verification must remain deterministic; public-network relay review should be opt-in UAT evidence unless deliberately changed.
- Existing outbound sync, inbound serving, full-sync, soak, support-bundle, and release-boundary behavior must not regress while adding transaction relay and mempool participation.
- Local generated Rust test binaries hang at dyld start before test execution; Plan 91-01 used cargo test --no-run, cargo check, build, clippy, and breadcrumb checks as verification evidence.

## Session Continuity

Last session: 2026-07-01T23:35:49.360Z
Stopped at: Completed 105-01-PLAN.md
Resume file: None
