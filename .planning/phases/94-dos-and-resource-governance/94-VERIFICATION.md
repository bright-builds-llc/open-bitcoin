---
phase: 94-dos-and-resource-governance
verified: "2026-06-27T01:20:21Z"
status: passed
score: "35/35 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: "94-2026-06-26T15-47-23"
generated_at: "2026-06-27T01:20:21Z"
lifecycle_validated: true
overrides_applied: 0
requirements_verified:
  - DOS-01
  - DOS-02
  - DOS-03
  - DOS-04
  - DOS-05
review_fixes_verified:
  - WR-01
  - WR-02
---

# Phase 94: DoS and Resource Governance Verification Report

**Phase Goal:** DoS and Resource Governance for bounded inbound envelope, queue/request/lifecycle policy, runtime listener enforcement, peer request caps, shared status/log/metrics evidence, CLI/support rendering, docs/parity, and deterministic verifier wiring while preserving no public-network/no-relay/no-production-readiness claims.
**Verified:** 2026-06-27T01:20:21Z
**Status:** passed
**Re-verification:** No - initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|---|---|---|
| R1 | Inbound parsing rejects invalid magic, malformed headers, oversized payloads, unsupported commands, and malformed payloads before unbounded allocation. | VERIFIED | `InboundEnvelopePolicy::evaluate_header` rejects malformed header, wrong magic, oversized payload, and unsupported commands before payload allocation in `packages/open-bitcoin-network/src/resource.rs`; runtime reader evaluates header before `vec!` allocation in `packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs`. |
| R2 | Per-peer and aggregate read/write queues, inventory/request bounds, header/block/transaction request caps, and backpressure behavior are enforced. | VERIFIED | `ResourceGovernancePolicy::decide_queue` and `decide_request` are wired into runtime reads/writes and peer inventory/getdata/getheaders flows; request-cap disconnects are recorded by managed network tests. |
| R3 | Slow handshakes, idle peers, connection churn, repeated failures, and banned/discouraged reconnect attempts have deterministic limits and tests. | VERIFIED | `decide_timeout`, `decide_churn`, `decide_repeated_failure`, and `decide_reconnect` are pure policy decisions with deterministic unit coverage and listener wiring. |
| R4 | Metrics, structured logs, support bundles, and status output expose resource pressure and next actions. | VERIFIED | Shared inbound status, fixed `MetricKind` variants, `inbound_resource_governance` structured logs, CLI status rendering, and support bundle rendering all consume the shared bounded event projection. |
| R5 | Default `bash scripts/verify.sh` remains public-network-free while proving inbound resource policy through synthetic and loopback-safe checks. | VERIFIED | `scripts/verify.sh` runs the Phase 94 checker/tests in the default contract; forbidden public-network listener/service-manager commands were absent from the default verifier path. |
| P01-1 | Inbound wire headers are classified before payload allocation. | VERIFIED | Header evaluation returns a bounded payload length before allocation in both pure policy and runtime reader paths. |
| P01-2 | Wrong magic, malformed headers, oversized payloads, unsupported commands, bad checksums, malformed payloads, and trailing payload bytes produce stable labels. | VERIFIED | Stable labels are defined and tested in `packages/open-bitcoin-network/src/resource.rs` and `resource/tests.rs`. |
| P01-3 | The new resource policy module is exported from open-bitcoin-network and registered in parity breadcrumbs. | VERIFIED | `packages/open-bitcoin-network/src/lib.rs` exports the resource module; `docs/parity/source-breadcrumbs.json` records resource and runtime anchors. |
| P02-1 | Read/write queue pressure is decided by pure data-in/data-out policy. | VERIFIED | `QueuePressureInput` flows through `ResourceGovernancePolicy::decide_queue` with deterministic tests. |
| P02-2 | Inventory, getdata, header, block, and transaction request caps have named constants and stable outcomes. | VERIFIED | Cap constants and `request_cap_reached` outcomes are defined in `resource.rs` and consumed by peer inventory state. |
| P02-3 | Slow handshakes, idle peers, churn, repeated failures, and banned/discouraged reconnect attempts are deterministic timestamp/counter decisions. | VERIFIED | Lifecycle inputs are plain counters/timestamps and tests cover cap boundary and over-cap outcomes without socket effects. |
| P03-1 | The opt-in inbound listener evaluates the header before allocating payload bytes. | VERIFIED | `read_wire_message_with_timeout_duration` calls `evaluate_header(&header)` before allocating the payload buffer. |
| P03-2 | The opt-in inbound listener consumes `ResourceGovernancePolicy::decide_queue` before reads and writes, and records resulting resource events. | VERIFIED | Runtime queue checks happen before socket reads and writes; tests assert read pressure precedes socket read and write pressure skips socket write. |
| P03-3 | Socket read/write waits are bounded by resource-policy timeouts so idle `read_exact` and `write_all` cannot wait indefinitely. | VERIFIED | `read_exact_until_deadline` uses an absolute deadline; write waits use policy timeout. WR-01 regression test covers timeout across partial header bytes. |
| P03-4 | Accept-loop churn, repeated-failure, and banned/discouraged reconnect suppression are evaluated before admission or peer work. | VERIFIED | `accept_loop` evaluates churn, repeated failure, and reconnect suppression before spawning peer work and records bounded events. |
| P03-5 | Runtime resource rejections are recorded as bounded listener evidence. | VERIFIED | `record_shared_resource_event` writes listener evidence and shared `ManagedRpcContext` status/log projections. |
| P03-6 | Slow handshakes and idle peers use policy output and remain testable without public-network traffic. | VERIFIED | Paused-time and loopback-safe listener tests cover timeout outcomes. |
| P04-1 | Peer request handling enforces request and inventory caps through the shared resource policy. | VERIFIED | `peer/inventory_state.rs` builds `RequestPressureInput` for inv/getdata/getheaders and returns `ResourceGovernanceDisconnect` on cap decisions. |
| P04-2 | Resource-limit disconnects have an explicit stable disconnect reason. | VERIFIED | `DisconnectReason::ResourceLimit` maps to `NetworkError::ResourceLimit` with stable display text. |
| P04-3 | `download`, `addr`, `noban`, and `forceinbound` permission effects are passed into resource-policy tests as bounded evidence without granting relay behavior. | VERIFIED | Permission-effect vectors flow into request pressure evidence; tests cover bounded evidence while relay/filter effects remain inactive. |
| P04-4 | Request caps do not enable transaction relay, mempool propagation, compact block relay, BIP37, or compact-filter serving. | VERIFIED | Peer tests keep deferred relay/filter labels inactive and docs/checker reject positive deferred-feature claims. |
| P05-1 | Resource pressure and abuse responses are projected through shared inbound status before renderers. | VERIFIED | `ManagedResourceGovernanceInfo::record_event` updates counters/latest decision; status and support renderers read the shared snapshot. |
| P05-2 | Metrics use fixed `MetricKind` variants with no dynamic labels. | VERIFIED | Resource metric kinds are fixed enum variants with low-cardinality tests. |
| P05-3 | Structured logs project the same bounded low-cardinality resource event fields as shared status. | VERIFIED | `inbound_resource_governance_log_record` emits allowlisted fields only; logging tests cover field allowlist and redaction. |
| P05-4 | Recording an inbound resource event in `ManagedRpcContext` appends an `inbound_resource_governance` JSONL structured log record when a log sink is configured. | VERIFIED | `context/resource_governance.rs` appends structured log records; `record_inbound_resource_event_appends_inbound_resource_governance_log_record` passed. |
| P05-5 | Shared resource evidence is bounded and omits raw peer identifiers, endpoints, payload bytes, permission strings, and credentials. | VERIFIED | Status/log/support projections use labels, counters, source, outcome, and next action; raw-evidence scans found no blocker in rendered shared evidence paths. |
| P06-1 | Operator status renders resource-governance counters and latest next action from shared status. | VERIFIED | `operator/status/render/inbound.rs` renders resource evidence and latest decision from `InboundStatus`. |
| P06-2 | Support bundles render redacted Phase 94 resource evidence and actionable next-action text. | VERIFIED | `operator/support/render/inbound.rs` renders counters, latest bounded decision, and Phase 94 next-action text. |
| P06-3 | Renderers do not invent local summaries or expose raw peer/payload/permission data. | VERIFIED | Renderer tests assert shared resource evidence output and no raw identifiers. |
| P07-1 | Operator docs explain bounded Phase 94 resource-governance behavior and next actions. | VERIFIED | `docs/operator/runtime-guide.md` documents labels, resource behavior, status/log/metric fields, and next actions. |
| P07-2 | UAT docs include repo-local Cargo and Bazel command forms. | VERIFIED | Runtime guide includes both `cargo run --manifest-path packages/Cargo.toml ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin ...` forms. |
| P07-3 | Parity docs record Knots anchors and preserve deferred relay/public-network/production no-claim boundaries. | VERIFIED | `docs/parity/catalog/p2p.md`, `checklist.md`, and `index.json` include Phase 94 anchors, DOS IDs, and explicit no-claim boundary. |
| P08-1 | Default verification includes deterministic Phase 94 resource-governance checks. | VERIFIED | `bash scripts/verify.sh` ran and completed successfully; Phase 94 checker tests and checker are in the default path. |
| P08-2 | The checker rejects missing labels, missing docs, dynamic metric omissions, missing structured-log emission wiring, public-network commands, and positive deferred-feature claims. | VERIFIED | `scripts/check-phase94-dos-resource-governance.test.ts` passed all failure-mode tests, including structured-log wiring and deferred-feature claim checks. |
| P08-3 | The checker is wired after Phase 93 in `bash scripts/verify.sh`. | VERIFIED | `scripts/verify.sh` runs Phase 93 then Phase 94 checker/tests in order, before pure-core dependency checks. |

**Score:** 35/35 truths verified

### Required Artifacts

All eight plan artifact checks passed through `gsd-tools verify artifacts`. Manual artifact verification found the files substantive and wired.

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `packages/open-bitcoin-network/src/resource.rs` | Pure resource-governance policy, labels, envelope guards | VERIFIED | Defines constants, policies, labels, bounded events, request/queue/lifecycle decisions, and envelope header/payload checks. |
| `packages/open-bitcoin-network/src/resource/tests.rs` | Deterministic resource policy coverage | VERIFIED | Covers envelope rejects, request/queue caps, lifecycle decisions, labels, and valid bounded messages. |
| `packages/open-bitcoin-network/src/lib.rs` | Export resource governance API | VERIFIED | Exports policy/event/input types and Phase 94 constants for runtime and peer layers. |
| `packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs` | Runtime socket enforcement | VERIFIED | Enforces header-before-allocation, absolute read deadline, bounded write waits, queue pressure, and timeout event creation. |
| `packages/open-bitcoin-rpc/src/inbound_listener.rs` | Listener accept/read/write wiring | VERIFIED | Wires churn/reconnect/repeated-failure decisions before admission; records resource events into shared evidence. |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs` | Runtime enforcement tests | VERIFIED | Covers wrong magic, oversized payload, unsupported command, queue pressure, timeout, and WR-01 regression. |
| `packages/open-bitcoin-network/src/peer/inventory_state.rs` | Peer request cap enforcement | VERIFIED | Inv/getdata/getheaders paths call shared request policy before serving or mutating request state. |
| `packages/open-bitcoin-network/src/peer.rs` | Resource disconnect peer action | VERIFIED | Carries `InboundResourceEvent` in `ResourceGovernanceDisconnect` for downstream evidence. |
| `packages/open-bitcoin-node/src/network/inventory.rs` | Managed network disconnect/evidence path | VERIFIED | Records resource-governance event before disconnecting and returns stable resource-limit error. |
| `packages/open-bitcoin-node/src/network.rs` | Managed network action handling | VERIFIED | Handles resource-governance peer actions and exposes `resource_governance_info`. |
| `packages/open-bitcoin-rpc/src/context/resource_governance.rs` | Shared RPC status/log projection | VERIFIED | Records events into managed network and appends structured JSONL when configured. |
| `packages/open-bitcoin-node/src/network/inbound.rs` | Shared resource status projection | VERIFIED | Maintains counters and latest bounded resource decision for status/renderers. |
| `packages/open-bitcoin-node/src/status/inbound.rs` | Stable status schema | VERIFIED | Adds resource-governance counters/latest event and unavailable-state defaults. |
| `packages/open-bitcoin-node/src/metrics.rs` | Fixed metric variants | VERIFIED | Adds fixed resource pressure/cap metric kinds with tests. |
| `packages/open-bitcoin-node/src/logging.rs` | Structured log formatter | VERIFIED | Emits allowlisted low-cardinality `inbound_resource_governance` fields and appends JSONL. |
| `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` | CLI status renderer | VERIFIED | Renders resource evidence from shared status. |
| `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs` | Support bundle renderer | VERIFIED | Renders resource summary and next actions from shared status. |
| `docs/operator/runtime-guide.md` | Operator/UAT documentation | VERIFIED | Documents labels, evidence, next actions, and repo-local Cargo/Bazel UAT commands. |
| `docs/architecture/status-snapshot.md` | Status/log/metric architecture docs | VERIFIED | Documents resource status schema, log source/fields, and metric names. |
| `docs/architecture/operator-observability.md` | Operator observability docs | VERIFIED | Documents status/log/metric surfaces and bounded evidence. |
| `docs/parity/catalog/p2p.md` | Parity surface | VERIFIED | Maps Phase 94 surface to Knots anchors and no-claim boundary. |
| `docs/parity/checklist.md` | Requirement checklist | VERIFIED | Maps `v1-9-dos-resource-governance` to DOS-01 through DOS-05. |
| `docs/parity/index.json` | Machine-readable parity index | VERIFIED | Includes DOS-01 through DOS-05, anchors, evidence, and explicit no-claim text. |
| `docs/parity/source-breadcrumbs.json` | Source breadcrumb registry | VERIFIED | Includes resource policy, runtime, status/log/metric, renderer, docs, and checker breadcrumbs. |
| `scripts/check-phase94-dos-resource-governance.ts` | Deterministic Phase 94 checker | VERIFIED | Checks labels, wiring, structured log append path, metrics, docs, verifier wiring, and boundary claims. |
| `scripts/check-phase94-dos-resource-governance.test.ts` | Checker tests | VERIFIED | Covers complete evidence and failure modes. |
| `scripts/verify.sh` | Default verifier wiring | VERIFIED | Runs Phase 94 checker/tests in default public-network-free contract. |

### Key Link Verification

All eight plan key-link checks passed through `gsd-tools verify key-links`. Manual link tracing found no orphaned implementation paths.

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `network/resource.rs` | `open-bitcoin-network` public API | `pub use` in `lib.rs` | WIRED | Runtime, peer, and tests import the shared policy/events. |
| `InboundEnvelopePolicy` | Runtime reader | `evaluate_header` and `decode_payload` | WIRED | Header is evaluated before allocation; decode rejects malformed payload/checksum/trailing bytes. |
| `ResourceGovernancePolicy::decide_queue` | Runtime listener | `RuntimeQueuePressureState` | WIRED | Read and write pressure events are checked before socket operations. |
| `ResourceGovernancePolicy::decide_timeout` | Runtime listener | `resource_timeout_event` | WIRED | Slow-handshake and idle timeouts emit bounded labels/actions. |
| `ResourceGovernancePolicy::decide_churn` | Accept loop | `record_shared_resource_event` | WIRED | Churn rejection is evaluated before peer work. |
| `ResourceGovernancePolicy::decide_repeated_failure` | Accept loop | `record_shared_resource_event` | WIRED | Repeated failures are limited before admission. |
| `ResourceGovernancePolicy::decide_reconnect` | Accept loop | `record_shared_resource_event` | WIRED | Banned/discouraged reconnect attempts are suppressed before admission. |
| `ResourceGovernancePolicy::decide_request` | Peer inventory/getdata/getheaders | `resource_limit_disconnect_actions` | WIRED | Request cap decisions return `PeerAction::ResourceGovernanceDisconnect(event)`. |
| `PeerAction::ResourceGovernanceDisconnect` | Managed network | `disconnect_for_resource_governance` | WIRED | WR-02 fixed path records event, disconnects, and returns stable resource-limit error. |
| Runtime listener events | RPC shared context | `record_shared_resource_event` | WIRED | Listener evidence and managed context projection are both updated. |
| RPC shared context | Structured log file | `append_structured_log_record` | WIRED | Configured sink receives `inbound_resource_governance` JSONL records. |
| Managed network resource info | Status schema | `current_inbound_status` | WIRED | Counters/latest event flow to `InboundStatus`. |
| Managed network resource info | Metrics | fixed `MetricKind` variants | WIRED | Low-cardinality resource metrics are defined and tested. |
| Shared status | CLI status renderer | `render/inbound.rs` | WIRED | Resource counters and latest next action render in operator status. |
| Shared status | Support bundle renderer | `support/render/inbound.rs` | WIRED | Resource evidence and next actions render in support bundles. |
| Docs/parity artifacts | Phase 94 checker | file and content assertions | WIRED | Checker validates labels, UAT commands, parity anchors, and no-claim boundary. |
| Phase 94 checker/tests | Default verifier | `scripts/verify.sh` | WIRED | Default verification runs checker tests and checker after Phase 93. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| Runtime listener | `InboundResourceEvent` | Socket header/payload evaluation, queue state, timeout/churn/failure/reconnect policy | Yes | FLOWING |
| Runtime listener shared evidence | `ManagedRpcContext::record_inbound_resource_event` | `record_shared_resource_event` | Yes | FLOWING |
| Peer request caps | `InboundResourceEvent` | `RequestPressureInput` from inv/getdata/getheaders counts | Yes | FLOWING |
| Managed network | `ManagedResourceGovernanceInfo` | `PeerAction::ResourceGovernanceDisconnect(event)` and listener events | Yes | FLOWING |
| Status snapshot | `InboundResourceGovernanceEvent` and counters | `current_inbound_status` from managed network | Yes | FLOWING |
| Structured logs | `StructuredLogRecord` | `ManagedRpcContext::record_inbound_resource_event_at` | Yes | FLOWING |
| CLI status/support renderers | `InboundStatus.resource_governance` fields | Shared status snapshot | Yes | FLOWING |
| Phase 94 checker | File contents and verifier wiring | Source/docs/scripts scanned from repo | Yes | FLOWING |

### Code Review Fix Verification

| Review Item | Original Risk | Fix Evidence | Status |
|---|---|---|---|
| WR-01 | Runtime read timeout reset after every partial header byte, weakening slowloris protection. | `read_wire_message_with_timeout_duration` now computes one absolute deadline and `read_exact_until_deadline` uses `timeout_at`; regression test `read_wire_message_times_out_across_partial_header_bytes` passed under paused Tokio time. | FIXED |
| WR-02 | Request-cap governance events were dropped before shared status/log projection. | `resource_limit_disconnect_actions` now preserves `InboundResourceEvent` in `PeerAction::ResourceGovernanceDisconnect`; managed network records the event before disconnect; inv/getdata/getheaders managed-network regression tests passed. | FIXED |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Repo-native verification contract | `bash scripts/verify.sh` | Completed successfully in 6m 40.837s, including hooks, parity breadcrumbs, Phase 94 checker/tests, Rust tests, benchmark smoke, and Bazel smoke build. | PASS |
| Phase 94 checker tests | `bun test scripts/check-phase94-dos-resource-governance.test.ts` | 4 passed, 0 failed. | PASS |
| Phase 94 checker | `bun run scripts/check-phase94-dos-resource-governance.ts` | `validated Phase 94 DoS and resource-governance evidence`. | PASS |
| Pure resource policy | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network resource --no-fail-fast` | 36 passed. | PASS |
| Runtime listener enforcement | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_listener --no-fail-fast` | 27 library tests plus matching daemon runtime test passed. | PASS |
| Peer request caps | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer --no-fail-fast` | 56 peer-focused tests plus parity test passed. | PASS |
| Shared status/metrics/log projection | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node inbound --no-fail-fast`; `metrics`; `logging`; `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc context --no-fail-fast` | 20 inbound, 18 metrics, 15 logging, and 7 context tests passed. | PASS |
| CLI/support rendering | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status::render --no-fail-fast`; `support` | 15 status-render tests, 50 support library tests, and 7 support binary tests passed. | PASS |
| Boundary scan | `rg` for default public-network/service-manager verifier commands and positive relay/production claims | No blocking matches in `scripts/verify.sh` or Phase 94 parity docs. | PASS |

### Requirements Coverage

| Requirement | Source Plan(s) | Description from `.planning/REQUIREMENTS.md` | Status | Evidence |
|---|---|---|---|---|
| DOS-01 | 94-01, 94-03, 94-07 | Inbound sessions enforce network magic, message header, payload size, malformed message, and unsupported command limits before allocating unbounded memory. | SATISFIED | Pure envelope policy and runtime header-before-allocation wiring reject all required cases with deterministic tests. |
| DOS-02 | 94-02, 94-03, 94-04, 94-07 | Inbound sessions enforce per-peer and aggregate read/write queues, inventory/request bounds, header/block/transaction request caps, and backpressure behavior. | SATISFIED | Runtime queue pressure and peer request cap flows enforce shared policy and record resource-limit disconnects. |
| DOS-03 | 94-02, 94-03, 94-07 | The node limits connection churn, slow handshakes, idle peers, repeated failures, and banned or discouraged reconnect attempts with deterministic synthetic tests. | SATISFIED | Pure lifecycle policy and accept-loop/runtime wiring are covered by deterministic unit and paused-time tests. |
| DOS-04 | 94-03, 94-04, 94-05, 94-06, 94-07 | Resource pressure and abuse responses appear in metrics, structured logs, support bundles, and operator status with clear next actions. | SATISFIED | Shared status/log/metric projection and CLI/support renderers are wired and tested, including structured JSONL append. |
| DOS-05 | 94-01 through 94-08 | Default verification covers inbound DoS/resource policy deterministically and keeps public-network listener exposure outside `bash scripts/verify.sh`. | SATISFIED | `scripts/verify.sh` includes Phase 94 checker/tests; full verifier passed; checker rejects public-network commands and positive deferred-feature claims. |

No orphaned Phase 94 requirement IDs were found: `.planning/REQUIREMENTS.md` maps only DOS-01 through DOS-05 to Phase 94, and all five are present in plan frontmatter.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---:|---|---|---|
| None | - | No blocking stub, orphan, forbidden public-network verifier command, positive relay/production claim, or raw-evidence leakage found. | - | - |

Notes:
- Internal `peer_id` variables remain in runtime listener implementation code, but the shared status/log/support evidence paths render bounded labels/counters/actions rather than raw peer IDs.
- Loopback/Tokio runtime tests exist where appropriate; pure policy code remains data-in/data-out and does not rely on sleeps, sockets, or wall-clock waits.
- Later Phase 95 concerns release-boundary closure. No Phase 94 gap was deferred to Phase 95.

### Human Verification Required

None. The phase goal is covered by deterministic source, checker, and test evidence. UAT commands are documented for operators but are not required to establish this phase's automated goal achievement.

### Lifecycle Validation

`94-CONTEXT.md`, all eight `94-*-PLAN.md` files, and all eight `94-*-SUMMARY.md` files share:

- `lifecycle_mode: yolo`
- `phase_lifecycle_id: 94-2026-06-26T15-47-23`

No direct-fallback provenance was found, so lifecycle validation is true.

### Gaps Summary

No gaps found. All DOS-01 through DOS-05 requirements, all roadmap success criteria, all plan must-haves, all key links, and both code review fixes are verified against the actual codebase. The implementation preserves the explicit boundary: no default public-network verification, no transaction relay/compact block relay/mempool propagation/BIP37/compact-filter serving claim, and no production full-node readiness claim.

---

_Verified: 2026-06-27T01:20:21Z_
_Verifier: the agent (gsd-verifier)_
